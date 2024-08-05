use std::path::Path;

use whisper_rs::{WhisperContext, FullParams, SamplingStrategy, convert_integer_to_float_audio};
use hound::{SampleFormat, WavReader};
use rustpotter::{Rustpotter, RustpotterConfig, Wakeword};

pub struct Listener {
    context: WhisperContext
}

impl Listener {
    pub fn new() -> Self {
        Self {
            context: WhisperContext::new_with_params(
                "./models/base.en.bin",
                Default::default(),
            ).unwrap(),
        }
    }

    fn parse_file(&self, path: &Path) -> Vec<f32> {
        let reader = WavReader::open(path).expect("failed to read file");
    
        if reader.spec().channels != 1 {
            panic!("expected mono audio file");
        }
        if reader.spec().sample_format != SampleFormat::Int {
            panic!("expected integer audio file");
        }
        if reader.spec().sample_rate != 16000 {
            panic!("expected 16000 sample rate found {}", reader.spec().sample_rate);
        }
        if reader.spec().bits_per_sample != 16 {
            panic!("expected 16 bits per sample found {}", reader.spec().bits_per_sample);
        }
    
        let samples = reader
            .into_samples::<i16>()
            .map(|x| x.expect("sample"))
            .collect::<Vec<_>>();
        let mut output: Vec<f32> = vec!(0.0; samples.len());
        convert_integer_to_float_audio(&samples, &mut output).expect("Error converting audio to float"); 
        output
    }

    pub fn wake(&mut self) -> (){
        // assuming the audio input format match the rustpotter defaults
        let mut rustpotter_config = RustpotterConfig::default();
        // Instantiate rustpotter
        let mut rustpotter = Rustpotter::new(&rustpotter_config).unwrap();
        // load a wakeword
        rustpotter.add_wakeword_from_file("computer", "./tests/resources/hey_home.rpw").unwrap();
        
        // You need a buffer of size 
        // `rustpotter.get_samples_per_frame()` when using samples or `rustpotter.get_bytes_per_frame()` when using bytes.
        let sample_per_frame = rustpotter.get_samples_per_frame();
        let samples = hound::WavReader::open("./tests/resources/hey_home.wav")
            .unwrap().into_samples::<i16>()
            .map(|x| x.unwrap())
            .collect::<Vec<i16>>()
            .chunks(sample_per_frame);

        samples.for_each(|samples_buffer| {
            // Process the audio buffer
            let detection = rustpotter.process(samples_buffer);
            if let Some(detection) = detection {
                println!("{:?}", detection);
            }
        });
    }

    pub fn listen(&mut self, file_path: &str) -> Result<String, String> {
        let mut state = self.context.create_state().expect("Error creating state");
        //create parameters for the full function
        let mut params = FullParams::new(SamplingStrategy::default());
        params.set_initial_prompt("experience");
        params.set_progress_callback_safe(|progress| println!("Progress callback: {}%", progress));

        //read file as 16 bit integer samples
        let samples = &self.parse_file(Path::new(file_path));

        //process and transcribe audio
        let _ = state.full(params, samples).expect("Error processing audio");
        
        // Iterate through the segments of the transcript.
        let Ok(num_segments) = state.full_n_segments() else {
            return Err("Error getting number of segments".to_string());
        };

        println!("Number of segments: {}", num_segments);

        let mut transcript = Vec::new();
        for i in 0..num_segments {
            println!("Segment {}", i);
            // Get the transcribed text and timestamps for the current segment.
            if let Ok(segment_text) = state.full_get_segment_text(i) {
                println!("{}", segment_text);
                transcript.push(segment_text);
            }
        };
        Ok(transcript.join(""))
    }
}
