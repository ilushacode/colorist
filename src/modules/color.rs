use std::f32;
use colored::Colorize;
use serde::Serialize;

#[derive(Serialize)]
pub struct ColorOutput {
    pub hex: String,
    pub rgb: [u8; 3],
}

#[derive(Serialize)]
pub struct PaletteOutput {
    pub colors: Vec<ColorOutput>,
}

pub fn kmeans(pixels: &Vec<[u8; 3]>, k: usize) -> Vec<[u8; 3]> {
    if pixels.is_empty() || k == 0 {
        return vec![];
    }

    let step = pixels.len() / k;
    let mut centroids: Vec<[f32; 3]> = (0..k)
        .map(|i| {
            let p = pixels[i * step];
            [p[0] as f32, p[1] as f32, p[2] as f32]
        })
        .collect();

    let max_iterations = 15;

    for _ in 0..max_iterations {
        let mut sums = vec![[0.0f32; 3]; k];
        let mut counts = vec![0usize; k];

        for pixel in pixels {
            let p_f32 = [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32];
            
            let mut min_dist = f32::MAX;
            let mut closest_centroid_idx = 0;

            for (idx, centroid) in centroids.iter().enumerate() {
                let dist = ((p_f32[0] - centroid[0]).powi(2)
                    + (p_f32[1] - centroid[1]).powi(2)
                    + (p_f32[2] - centroid[2]).powi(2))
                .sqrt();

                if dist < min_dist {
                    min_dist = dist;
                    closest_centroid_idx = idx;
                }
            }

            sums[closest_centroid_idx][0] += p_f32[0];
            sums[closest_centroid_idx][1] += p_f32[1];
            sums[closest_centroid_idx][2] += p_f32[2];
            counts[closest_centroid_idx] += 1;
        }

        for idx in 0..k {
            if counts[idx] > 0 {
                centroids[idx][0] = sums[idx][0] / counts[idx] as f32;
                centroids[idx][1] = sums[idx][1] / counts[idx] as f32;
                centroids[idx][2] = sums[idx][2] / counts[idx] as f32;
            }
        }
    }

    centroids
        .into_iter()
        .map(|c| [c[0].round() as u8, c[1].round() as u8, c[2].round() as u8])
        .collect()
}

pub fn print_color_row(r: u8, g: u8, b: u8) {
    let hex_code = format!("#{:02X}{:02X}{:02X}", r, g, b);
    println!("  {}  {}", "●".truecolor(r, g, b), hex_code);
}