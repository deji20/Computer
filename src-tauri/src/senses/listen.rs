use std::time::SystemTime;

use hound::{SampleFormat, WavSpec, WavWriter};
use whisper_rs::{WhisperContext, FullParams, SamplingStrategy};
use rustpotter::{Rustpotter, RustpotterConfig, RustpotterDetection};
use portaudio::{ self as pa, DeviceIndex, InputStreamSettings };

pub struct Listener {
    context: WhisperContext,
    rustpotter: Rustpotter,
}

const SAMPLE_RATE: u32 = 16000;

impl Listener {
    pub fn new() -> Self {
        //configure rustpotter
        let mut rustpotter_config = RustpotterConfig::default();
        rustpotter_config.fmt = {
            let mut aud_fmt = rustpotter::AudioFmt::default();
            aud_fmt.sample_rate = SAMPLE_RATE as usize;
            aud_fmt.channels = 1;
            aud_fmt.sample_format = rustpotter::SampleFormat::F32;
            aud_fmt
        };
        rustpotter_config.detector = {
            let mut detector = rustpotter::DetectorConfig::default();
            detector.threshold = 0.9;
            detector.score_mode = rustpotter::ScoreMode::Max;
            detector.avg_threshold = 0.2;
            detector.eager = true;
            detector
        };

        // Instantiate rustpotter
        let mut rustpotter = Rustpotter::new(&rustpotter_config).unwrap();
        rustpotter.add_wakeword_from_file("hey_computer", "models/hey_computer.rpw").unwrap();
        println!("Wakeword added");

        Self {
            context: WhisperContext::new_with_params(
                "./models/base.en.bin",
                Default::default(),
            ).unwrap(),
            rustpotter: rustpotter
        }
    }

    // fn parse_file(&self, path: &Path) -> Vec<f32> {
    //     let reader = WavReader::open(path).expect("failed to read file");
    
    //     if reader.spec().channels != 1 {
    //         panic!("expected mono audio file");
    //     }
    //     if reader.spec().sample_format != SampleFormat::Int {
    //         panic!("expected integer audio file");
    //     }
    //     if reader.spec().sample_rate != 16000 {
    //         panic!("expected 16000 sample rate found {}", reader.spec().sample_rate);
    //     }
    //     if reader.spec().bits_per_sample != 16 {
    //         panic!("expected 16 bits per sample found {}", reader.spec().bits_per_sample);
    //     }
    
    //     let samples = reader
    //         .into_samples::<i16>()
    //         .map(|x| x.expect("sample"))
    //         .collect::<Vec<_>>();
    //     let mut output: Vec<f32> = vec!(0.0; samples.len());
    //     convert_integer_to_float_audio(&samples, &mut output).expect("Error converting audio to float"); 
    //     output
    // }

    fn wait_for_stream<F>(f: F, name: &str) -> u32
        where
            F: Fn() -> Result<pa::StreamAvailable, pa::error::Error>,
        {
            loop {
                match f() {
                    Ok(available) => match available {
                        pa::StreamAvailable::Frames(frames) => return frames as u32,
                        pa::StreamAvailable::InputOverflowed => println!("Input stream has overflowed"),
                        pa::StreamAvailable::OutputUnderflowed => {
                            println!("Output stream has underflowed")
                        }
                    },
                    Err(err) => panic!(
                        "An error occurred while waiting for the {} stream: {}",
                        name, err
                    ),
                }
            }
        }
    

    pub fn wake(&mut self) -> Result<RustpotterDetection, String> {
        Listener::record(self.rustpotter.get_samples_per_frame(), move |input_samples| {
            //clear console
            print!("\x1B[2J\x1B[1;1H");

            println!("Waiting to wake up...");
            println!("Press Ctrl+C to stop recording");

            self.rustpotter.process_samples(input_samples.to_vec())
        })
    }

    pub fn get_command<F>(mut f: F) -> Result<Vec<f32>, String> 
        where F: FnMut(Vec<f32>) -> () {
        //get unix time
        let mut capture: Option<u64> = None;
        let mut audio = Vec::new();
        let _ = Listener::record(200, |input_samples| {
            // clear console
            print!("\x1B[2J\x1B[1;1H");
        
            println!("Recording...");
            println!("Press Ctrl+C to stop recording");
            let avg = input_samples.iter().fold(0.0, |acc, x| acc + x.abs()) / input_samples.len() as f32; 
            
            f(input_samples.to_vec());

            if let Some(_) = capture { audio.extend(input_samples); }
            if avg > 0.050 { 
                println!("capturing");
                capture = Some(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
                None
            }
            else if let Some(capture) = capture {
                let elapsed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - capture;
                if elapsed > 2 { Some("this") }
                else { println!("since last audio: {}", elapsed.to_string()); None }
            } else { None }
        });
        Ok(audio)
    }

    pub fn record<T, F>(chunk_size: usize, mut f: F) -> Result<T, String> 
        where F: FnMut(Vec<f32>) -> Option<T> {
        // Configure portaudio
        let portaudio = pa::PortAudio::new().unwrap();

        //and default 
        let def_input = portaudio.default_input_device().unwrap();
        let input_info = portaudio.device_info(def_input).unwrap();
        let latency = input_info.default_high_input_latency;
        let input_params = pa::StreamParameters::<f32>::new(def_input, 1, true, latency);
        let settings = InputStreamSettings::new(input_params, SAMPLE_RATE as f64, chunk_size as u32);
        
        let mut stream = portaudio.open_blocking_stream(settings).unwrap();
        
        stream.start().unwrap();
        loop {
            // How many frames are available on the input stream?
            let in_frames = Listener::wait_for_stream(|| stream.read_available(), "Read");
            // println!("Frames available: {:?}", in_frames);
            if in_frames == 0 { continue; }

            // Read the available frames from the input stream.
            let input_samples = stream.read(chunk_size as u32).unwrap();
            //process the buffer
            if input_samples.len() >= chunk_size as usize {
                if let Some(result) = f(input_samples.to_vec()){
                    stream.stop().unwrap();
                    break Ok(result);
                }
            }  
        }
    }

    pub fn transcribe(&mut self, audio: &Vec<f32>) -> Result<String, String> {
        let mut state = self.context.create_state().expect("Error creating state");
        //create parameters for the full function
        let mut params = FullParams::new(SamplingStrategy::default());
        params.set_initial_prompt("experience");
        params.set_progress_callback_safe(|progress| println!("Progress callback: {}%", progress));
        //process and transcribe audio
        println!("Transcribing...");
        let _ = state.full(params, audio).expect("Error processing audio");
        
        // Iterate through the segments of the transcript.
        let Ok(num_segments) = state.full_n_segments() else {
            return Err("Error getting number of segments".to_string());
        };

        let mut transcript = Vec::new();
        for i in 0..num_segments {
            // Get the transcribed text and timestamps for the current segment.
            if let Ok(segment_text) = state.full_get_segment_text(i) {
                transcript.push(segment_text);
            }
        };
        Ok(transcript.join(""))
    }
}
