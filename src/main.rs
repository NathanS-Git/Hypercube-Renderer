use nannou::prelude::*;
use nannou::geom::pt2;
use nannou::math::map_range;
use ndarray::prelude::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Hypercube dimension (e.g. 3, 4, 5, ...).
const DEFAULT_DIM: usize = 4;
/// Frames per second for captured output.
const DEFAULT_FPS: f64 = 60.0;
/// Length of the seamless loop in seconds.
const DEFAULT_LOOP_SECONDS: f64 = 6.0;

struct Model {
    dim: usize,
    vertices: Array2<f32>,
    perspective_project: bool,
    capture: bool,
    output_dir: PathBuf,
    fps: f64,
    total_frames: usize,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    let capture = args.len() > 1 && args[1] == "capture";

    let dim = env::var("DIM")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_DIM);

    let fps = env::var("FPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_FPS);

    let loop_seconds = env::var("LOOP_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_LOOP_SECONDS);

    let base_dim = env::var("BASE_DIMENSION")
        .ok()
        .and_then(|s| s.parse::<usize>().ok());

    let loop_seconds = if let Some(base_dim) = base_dim {
        assert!(base_dim >= 2, "BASE_DIMENSION must be at least 2");
        let base_planes = (base_dim * (base_dim - 1)) / 2;
        let dim_planes = (dim * (dim - 1)) / 2;
        // Scale duration so the perceived visual speed (RMS vertex motion) is the
        // same as the base dimension. Higher dimensions have more rotation planes,
        // so they need more time for one full, seamless period.
        loop_seconds * f64::sqrt(dim_planes as f64 / base_planes as f64)
    } else {
        loop_seconds
    };

    let perspective_project = env::var("PERSPECTIVE")
        .ok()
        .map(|s| s.to_lowercase() != "false")
        .unwrap_or(true);

    if perspective_project {
        assert!(dim >= 3, "Perspective projection requires dimension >= 3");
    }

    let total_frames = (fps * loop_seconds).round().max(1.0) as usize;

    let output_dir = PathBuf::from("frames");
    if capture {
        std::fs::create_dir_all(&output_dir).unwrap();
        // Empty the frames directory so old frames don't leak into the new video.
        for entry in std::fs::read_dir(&output_dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                std::fs::remove_file(entry.path()).unwrap();
            }
        }
    }

    // Use physical pixels so the captured frames have a predictable, even size
    // that ffmpeg's yuv420p encoder accepts. In capture mode we also hide the
    // window and make it non-resizable so tiling window managers can't distort
    // the 1:1 aspect ratio.
    let mut builder = app
        .new_window()
        .size_pixels(800, 800)
        .resizable(false)
        .view(view);
    if capture {
        builder = builder.visible(false);
    }
    let _window = builder.build().unwrap();

    app.set_loop_mode(LoopMode::rate_fps(fps));

    Model {
        dim,
        vertices: generate_hypercube_vertices(dim),
        perspective_project,
        capture,
        output_dir,
        fps,
        total_frames,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.capture && app.elapsed_frames() >= model.total_frames as u64 {
        // Make sure every PNG has been flushed to disk before we hand the frames
        // directory to ffmpeg.
        if let Err(e) = app.main_window().await_capture_frame_jobs() {
            eprintln!("Timed out waiting for frame capture jobs: {:?}", e);
        }
        encode_video(model);
        app.quit();
    }
}

fn encode_video(model: &Model) {
    let crf = env::var("CRF")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(18);

    let filename = format!(
        "hypercube{}D_{}fps_{:.1}s_loop.mp4",
        model.dim,
        model.fps as u32,
        model.total_frames as f64 / model.fps
    );

    println!("Encoding {} frames into {} ...", model.total_frames, filename);

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg(format!("{}", model.fps))
        .arg("-i")
        .arg(model.output_dir.join("frame_%05d.png"))
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-crf")
        .arg(format!("{}", crf))
        .arg("-movflags")
        .arg("+faststart")
        .arg(&filename)
        .status();

    match status {
        Ok(s) if s.success() => println!("Perfect-loop video saved to: {}", filename),
        Ok(s) => eprintln!("ffmpeg exited with status: {}", s),
        Err(e) => eprintln!(
            "Failed to run ffmpeg (is it installed?). Frames are still in {:?}: {}",
            model.output_dir, e
        ),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let frame_idx = app.elapsed_frames();

    if model.capture && frame_idx >= model.total_frames as u64 {
        return;
    }

    // In capture mode, advance time so that exactly one full rotation period is
    // spread evenly across `total_frames`. The full rotation matrix only returns
    // to the identity (with matching velocity) at t = 2π, so we use that as the
    // loop boundary for every dimension. Frame 0 is at angle 0 and the last frame
    // is the step before the hypercube returns to its exact starting orientation,
    // giving a C^1 seamless loop.
    let time = if model.capture {
        (frame_idx as f32) * (2.0 * PI / model.total_frames as f32)
    } else {
        app.time as f32 * 0.5
    };

    let rotation_matrix = build_rotation_matrix(model.dim, time);
    let vertices = rotation_matrix.dot(&model.vertices);

    let draw = app.draw();
    draw.background().color(WHITE);

    let z_depth = if model.perspective_project { 1.5 } else { 1.0 };
    let zoom = 250.0;
    let n = 1 << model.dim;

    for i in 0..n {
        for j in 0..model.dim {
            let neighbor = i ^ (1 << j);

            let denominator_1 = if model.perspective_project {
                z_depth - vertices[[2, i]]
            } else {
                1.0
            };
            let denominator_2 = if model.perspective_project {
                z_depth - vertices[[2, neighbor]]
            } else {
                1.0
            };

            let x_1 = map_range(
                (vertices[[0, i]] * z_depth) / denominator_1,
                -1.0,
                1.0,
                -zoom,
                zoom,
            );
            let x_2 = map_range(
                (vertices[[0, neighbor]] * z_depth) / denominator_2,
                -1.0,
                1.0,
                -zoom,
                zoom,
            );
            let y_1 = map_range(
                (vertices[[1, i]] * z_depth) / denominator_1,
                -1.0,
                1.0,
                -zoom,
                zoom,
            );
            let y_2 = map_range(
                (vertices[[1, neighbor]] * z_depth) / denominator_2,
                -1.0,
                1.0,
                -zoom,
                zoom,
            );

            draw.line()
                .start(pt2(x_1, y_1))
                .end(pt2(x_2, y_2))
                .weight(2.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();

    if model.capture {
        let path = model.output_dir.join(format!("frame_{:05}.png", frame_idx));
        app.main_window().capture_frame(&path);
    }
}

fn generate_hypercube_vertices(dim: usize) -> Array2<f32> {
    let mut vertices: Array2<f32> = Array2::zeros((dim, 1 << dim));

    for i in 0..dim {
        for j in 0..(1 << dim) {
            vertices[[i, j]] = if (j % (1 << (i + 1))) >= (1 << i) {
                -0.5
            } else {
                0.5
            };
        }
    }

    vertices
}

fn build_rotation_matrix(dim: usize, angle: f32) -> Array2<f32> {
    let mut rotation_matrix = Array2::eye(dim);

    for i in 0..dim {
        for j in (i + 1)..dim {
            rotation_matrix = rotation_matrix.dot(&construct_rotation_matrix(dim, i, j, angle));
        }
    }

    rotation_matrix
}

fn construct_rotation_matrix(dim: usize, axis_1: usize, axis_2: usize, angle: f32) -> Array2<f32> {
    let mut rotation_matrix: Array2<f32> = Array2::eye(dim);

    // Make the rotation flip every other time.
    let angle = if axis_1 % 2 == 0 { angle } else { -angle };

    rotation_matrix[[axis_1, axis_2]] = f32::sin(angle);
    rotation_matrix[[axis_2, axis_1]] = -f32::sin(angle);
    rotation_matrix[[axis_1, axis_1]] = f32::cos(angle);
    rotation_matrix[[axis_2, axis_2]] = f32::cos(angle);

    rotation_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_matrix_returns_to_identity_at_loop_boundary() {
        // The full rotation matrix returns to identity at t = 2π for every
        // dimension. Using π works for odd dimensions position-wise, but the
        // velocity does not match, so the seamless loop uses 2π.
        for dim in 2..=10 {
            let m = build_rotation_matrix(dim, 2.0 * PI);
            let _identity: Array2<f32> = Array2::eye(dim);
            for i in 0..dim {
                for j in 0..dim {
                    let expected = if i == j { 1.0 } else { 0.0 };
                    assert!(
                        (m[[i, j]] - expected).abs() < 1e-5,
                        "dim {} element [{},{}] = {}, expected {}",
                        dim, i, j, m[[i, j]], expected
                    );
                }
            }
        }
    }


}
