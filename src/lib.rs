#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Plugin, Info, CanDo};
use vst2::event::Event;
use vst2::api::Supported;

mod note;

struct SawWave {
    sample_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,
    level: f32,
}

impl SawWave {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => ()
        }
    }

    fn note_on(&mut self, note: u8) {
        self.note_duration = 0.0;
        self.note = Some(note)
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
    }
}

impl Default for SawWave {
    fn default() -> SawWave {
        SawWave {
            sample_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: None,
            level: 1.0,
        }
    }
}

impl Plugin for SawWave {
    fn get_info(&self) -> Info {
        Info {
            name: "SawWave".to_string(),
            vendor: "SpectralFlux".to_string(),
            unique_id: 6667,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.level,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            // We don't want to divide by zero, so we'll clamp the value
            0 => self.level = value.max(0.01),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Level".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.level * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    #[allow(unused_variables)]
    fn process_events(&mut self, events: Vec<Event>) {
        for event in events {
            match event {
                Event::Midi { data, ..  } => self.process_midi_event(data),
                // More events can be handled here.
                _ => {}
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate as f64;
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();

        let samples = inputs
            .first()
            .map(|channel| channel.len())
            .unwrap_or(0);

        let per_sample = self.time_per_sample();

        for (input_buffer, output_buffer) in inputs.iter().zip(outputs) {
            let mut t = self.time;

            for (_, output_sample) in input_buffer.iter().zip(output_buffer) {
                if let Some(current_note) = self.note {

                    // build saw wave
                    let note_hz =  note::midi_note_to_hz(current_note);
                    let full_period_time = 1.0 / note_hz;
                    let local_time = t % full_period_time;

                    let signal = (self.level as f64) * (local_time / full_period_time) * 2.0 - 1.0;

                    *output_sample = signal as f32;

                    t += per_sample;
                } else {
                    *output_sample = 0.0;
                }
            }
        }

        self.time += samples as f64 * per_sample;
        self.note_duration += samples as f64 * per_sample;
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe
        }
    }
}

plugin_main!(SawWave);