use clap::Parser;
use image::ImageReader;

// Declare our internal modules
mod args;
mod modules;

// Import specific items from modules for cleaner code
use args::{Args, AnswerType};
use modules::color::{kmeans, print_color_row, ColorOutput, PaletteOutput};

fn main() {
    let args = Args::parse();

    // Debug logs will only appear if --debug or -d flag is provided
    if args.debug {
        println!("[DEBUG] Initializing Colorist CLI...");
        println!("[DEBUG] Target Image: {:?}", args.image);
        println!("[DEBUG] Requesting {} dominant colors", args.count);
        println!("[DEBUG] Output format mode: {:?}", args.answer_type);
    }

    // Load and decode the image
    if args.debug { println!("[DEBUG] Opening and decoding image file..."); }
    let img = ImageReader::open(&args.image)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    // Downscale for performance optimization
    if args.debug { println!("[DEBUG] Downscaling image to 100x100 thumbnail for analysis..."); }
    let small_img = img.thumbnail(100, 100);
    
    if args.debug { 
        println!("[DEBUG] Saving cache thumbnail to disk...");
        small_img.save("thumbnail.jpg").expect("Failed to save thumbnail"); 
    }

    // Convert pixels to RGB buffer vector
    let rgb_buffer = small_img.to_rgb8();
    let pixels: Vec<[u8; 3]> = rgb_buffer
        .pixels()
        .map(|p| p.0)
        .collect();

    if args.debug { println!("[DEBUG] Total pixels processed for K-Means: {}", pixels.len()); }

    // Run K-Means algorithm
    if args.debug { println!("[DEBUG] Processing K-Means clustering..."); }
    let mut dominant_colors = kmeans(&pixels, args.count);

    // Sort colors by relative human-eye luminance (dark to light)
    if args.debug { println!("[DEBUG] Sorting extracted palette by luminance..."); }
    dominant_colors.sort_by(|a, b| {
        let brightness_a = 0.2126 * a[0] as f32 + 0.7152 * a[1] as f32 + 0.0722 * a[2] as f32;
        let brightness_b = 0.2126 * b[0] as f32 + 0.7152 * b[1] as f32 + 0.0722 * b[2] as f32;
        brightness_a.partial_cmp(&brightness_b).unwrap()
    });

    // Handle user selected output format
    match args.answer_type {
        AnswerType::Json => {
            let colors_for_json: Vec<ColorOutput> = dominant_colors
                .iter()
                .map(|c| ColorOutput {
                    hex: format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2]),
                    rgb: *c,
                })
                .collect();

            let output = PaletteOutput { colors: colors_for_json };
            let json_string = serde_json::to_string_pretty(&output)
                .expect("Failed to serialize data to JSON");
            
            println!("{}", json_string);
        }
        AnswerType::Raw => {
            for color in &dominant_colors {
                println!("R: {}, G: {}, B: {}", color[0], color[1], color[2]);
            }
        }
        AnswerType::Pretied => {
            for color in &dominant_colors {
                print_color_row(color[0], color[1], color[2]);
            }
        }
    }
}