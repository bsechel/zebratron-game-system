use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use hound;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.wav> <output_name>", args[0]);
        eprintln!("Example: {} laugh.wav laugh_sample", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_name = &args[2];
    
    // Read WAV file
    let mut reader = match hound::WavReader::open(input_path) {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error opening WAV file: {}", e);
            std::process::exit(1);
        }
    };

    let spec = reader.spec();
    println!("Original format: {} Hz, {} channels, {} bits", 
             spec.sample_rate, spec.channels, spec.bits_per_sample);

    // Convert to 8-bit samples at original sample rate (we'll downsample if needed)
    let mut samples: Vec<i16> = Vec::new();
    
    match spec.sample_format {
        hound::SampleFormat::Int => {
            match spec.bits_per_sample {
                16 => {
                    for sample in reader.samples::<i16>() {
                        samples.push(sample.unwrap());
                    }
                },
                8 => {
                    for sample in reader.samples::<i8>() {
                        // Convert 8-bit to 16-bit for processing
                        samples.push((sample.unwrap() as i16) << 8);
                    }
                },
                _ => {
                    eprintln!("Unsupported bit depth: {}", spec.bits_per_sample);
                    std::process::exit(1);
                }
            }
        },
        hound::SampleFormat::Float => {
            for sample in reader.samples::<f32>() {
                // Convert float to 16-bit int
                let sample_f32 = sample.unwrap();
                let sample_i16 = (sample_f32 * 32767.0) as i16;
                samples.push(sample_i16);
            }
        }
    }

    // Convert stereo to mono if needed
    if spec.channels == 2 {
        let mut mono_samples = Vec::new();
        for chunk in samples.chunks(2) {
            if chunk.len() == 2 {
                let mono = ((chunk[0] as i32 + chunk[1] as i32) / 2) as i16;
                mono_samples.push(mono);
            }
        }
        samples = mono_samples;
    }

    // Downsample to 5.5kHz for authentic retro crunch
    let target_sample_rate = 5512;
    if spec.sample_rate > target_sample_rate {
        let downsample_ratio = spec.sample_rate as f32 / target_sample_rate as f32;
        let mut downsampled = Vec::new();
        
        let mut index = 0.0;
        while (index as usize) < samples.len() {
            downsampled.push(samples[index as usize]);
            index += downsample_ratio;
        }
        samples = downsampled;
        println!("Downsampled to {} Hz ({} samples)", target_sample_rate, samples.len());
    }

    // Convert to 8-bit unsigned (0-255 range for retro feel)
    let mut sample_data: Vec<u8> = Vec::new();
    for sample in samples {
        // Convert from signed 16-bit (-32768 to 32767) to unsigned 8-bit (0 to 255)
        let unsigned_sample = ((sample as i32 + 32768) / 256) as u8;
        sample_data.push(unsigned_sample);
    }

    // Limit sample length for memory efficiency (max 2 seconds at 5.5kHz = 11024 samples)
    let max_samples = 11024;
    if sample_data.len() > max_samples {
        sample_data.truncate(max_samples);
        println!("Truncated to {} samples (2 seconds max)", max_samples);
    }

    // Generate Rust code
    let output_file = format!("{}_sample.rs", output_name);
    let mut file = File::create(&output_file).expect("Failed to create output file");
    
    writeln!(file, "// {} sample data ({} samples at {} Hz)", 
             output_name, sample_data.len(), target_sample_rate).unwrap();
    writeln!(file, "pub const {}_SAMPLE_RATE: u32 = {};", 
             output_name.to_uppercase(), target_sample_rate).unwrap();
    writeln!(file, "pub const {}_SAMPLE_DATA: &[u8] = &[", 
             output_name.to_uppercase()).unwrap();
    
    // Write sample data in rows of 16 values
    for (i, chunk) in sample_data.chunks(16).enumerate() {
        write!(file, "    ").unwrap();
        for (j, &sample) in chunk.iter().enumerate() {
            write!(file, "{}", sample).unwrap();
            if i * 16 + j < sample_data.len() - 1 {
                write!(file, ",").unwrap();
            }
            if j < chunk.len() - 1 {
                write!(file, " ").unwrap();
            }
        }
        writeln!(file).unwrap();
    }
    
    writeln!(file, "];").unwrap();
    writeln!(file).unwrap();
    writeln!(file, "// Sample info:").unwrap();
    writeln!(file, "// - Length: {} samples ({:.2} seconds)", 
             sample_data.len(), sample_data.len() as f32 / target_sample_rate as f32).unwrap();
    writeln!(file, "// - Sample rate: {} Hz", target_sample_rate).unwrap();
    writeln!(file, "// - Format: 8-bit unsigned (0-255)").unwrap();

    println!("Generated {} with {} samples", output_file, sample_data.len());
    println!("Sample length: {:.2} seconds", sample_data.len() as f32 / target_sample_rate as f32);
}