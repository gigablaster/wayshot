use std::{
    cmp, env,
    error::Error,
    fs::File,
    io::{stdout, BufWriter},
    process::exit,
    time::{SystemTime, UNIX_EPOCH},
};

use wayland_client::{protocol::wl_output::WlOutput, Display};

mod backend;
mod clap;
mod convert;
mod output;

// TODO: Create a xdg-shell surface, check for the enter event, grab the output from it.
//
// TODO: Patch multiple output bug via multiple images composited into 1.

fn main() -> Result<(), Box<dyn Error>> {
    let args = clap::set_flags().get_matches();
    env::set_var("RUST_LOG", "wayshot=info");

    if args.is_present("debug") {
        env::set_var("RUST_LOG", "wayshot=trace");
    }

    env_logger::init();
    log::trace!("Logger initialized.");

    let display = Display::connect_to_env()?;

    let mut cursor_overlay: i32 = 0;
    if args.is_present("cursor") {
        cursor_overlay = 1;
    }

    if args.is_present("listoutputs") {
        let valid_outputs = output::get_all_outputs(display);
        for output in valid_outputs {
            log::info!("{:#?}", output.name);
        }
        exit(1);
    }

    let output: WlOutput = if args.is_present("output") {
        output::get_wloutput(
            args.value_of("output").unwrap().trim().to_string(),
            output::get_all_outputs(display.clone()),
        )
    } else {
        output::get_all_outputs(display.clone())
            .first()
            .unwrap()
            .wl_output
            .clone()
    };

    let frame_copy: backend::FrameCopy = if args.is_present("slurp") {
        if args.value_of("slurp").unwrap() == "" {
            log::error!("Failed to recieve geometry.");
            exit(1);
        }
        let slurp: Vec<_> = args.value_of("slurp").unwrap().trim().split(' ').collect();
        let slurp: Vec<i32> = slurp.iter().map(|i| i.parse::<i32>().unwrap()).collect();
        let x_coordinate = slurp[0];
        let y_coordinate = slurp[1];
        let width = slurp[2];
        let height = slurp[3];

        let outputs = output::get_all_outputs(display.clone());
        let mut intersecting_outputs: Vec<output::OutputInfo> = Vec::new();
        for output in outputs {
            let x1: i32 = cmp::max(output.dimensions.x, x_coordinate);
            let y1: i32 = cmp::max(output.dimensions.y, y_coordinate);
            let x2: i32 = cmp::min(
                output.dimensions.x + output.dimensions.width,
                x_coordinate + width,
            );
            let y2: i32 = cmp::min(
                output.dimensions.y + output.dimensions.height,
                y_coordinate + height,
            );

            let width = x2 - x1;
            let height = y2 - y1;

            if !(width <= 0 || height <= 0) {
                intersecting_outputs.push(output);
            }
        }
        if intersecting_outputs.is_empty() {
            log::error!("Provided capture region doesn't intersect with any outputs!");
            exit(1);
        }
        // NOTE: Figure out box bounds for multi monitor screenshot.

        backend::capture_output_frame(
            display,
            cursor_overlay,
            output,
            Some(backend::CaptureRegion {
                x_coordinate,
                y_coordinate,
                width,
                height,
            }),
        )?
    } else {
        backend::capture_output_frame(display, cursor_overlay, output, None)?
    };

    let extension = if args.is_present("extension") {
        let ext: &str = &args.value_of("extension").unwrap().trim().to_lowercase();
        match ext {
            "jpeg" | "jpg" => backend::EncodingFormat::Jpg,
            "png" => backend::EncodingFormat::Png,
            "ppm" => backend::EncodingFormat::Ppm,
            _ => {
                log::error!("Invalid extension provided.\nValid extensions:\n1) jpeg\n2) jpg\n3) png\n4) ppm");
                exit(1);
            }
        }
    } else {
        backend::EncodingFormat::Png
    };

    if extension != backend::EncodingFormat::Png {
        log::debug!("Using custom extension: {:#?}", extension);
    }

    if args.is_present("stdout") {
        let stdout = stdout();
        let writer = BufWriter::new(stdout.lock());
        backend::write_to_file(writer, extension, frame_copy)?;
    } else {
        let path = if args.is_present("file") {
            args.value_of("file").unwrap().trim().to_string()
        } else {
            let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs().to_string(),
                Err(_) => {
                    log::error!("SystemTime before UNIX EPOCH!");
                    exit(1);
                }
            };

            time + match extension {
                backend::EncodingFormat::Png => "-wayshot.png",
                backend::EncodingFormat::Jpg => "-wayshot.jpg",
                backend::EncodingFormat::Ppm => "-wayshot.ppm",
            }
        };

        backend::write_to_file(File::create(path)?, extension, frame_copy)?;
    }

    Ok(())
}
