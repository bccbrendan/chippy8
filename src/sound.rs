use sdl2;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = match self.phase {
                p if (0.0..0.5).contains(&p) => self.volume,
                _ => -self.volume,
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Sound {
    device_opt: Option<AudioDevice<SquareWave>>,
    playing: bool,
}

impl Sound {
    pub fn new(sdl: &sdl2::Sdl) -> Self {
        println!("Getting audio subsystem");
        let audio_subsystem = sdl.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device_opt = match audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }) {
            Err(e) => {
                println!("Unable to initiate audio: {}", e);
                None
            }
            Ok(device) => {
                Some(device)
            }
        };
        Sound {
            device_opt: device_opt,
            playing: false,
        }
    }

    pub fn beep(&mut self, to_beep_or_not_to_beep: bool) {
        match &self.device_opt {
            Some(device) => {
                if !self.playing && to_beep_or_not_to_beep {
                    device.resume()
                } else if self.playing && !to_beep_or_not_to_beep {
                    device.pause()
                }
            },
            None => {},
        }
        self.playing = to_beep_or_not_to_beep;
    }
}
