use quicksilver::sound::*;
use quicksilver::prelude::*;

pub struct MusicPlayer {
    music: Asset<Sound>,
    music_length: f64,
    frame_counter: f64,
    started: bool,
}

impl MusicPlayer {
    /// Set up a rudimentary sound loop by passing a file name, the length in seconds, and a volume.
    /// Volume is set here and is multiplicative, I.E. 0 is silent
    /// 1 is identity, and 2 is double volume
    pub fn new(path: &'static str, length_sec: f64, volume: f32) -> Result<Self> {
        let mut music_loop = Asset::new(Sound::load(path));
        music_loop.execute(|sound| {
            sound.set_volume(volume);
            Ok(())
        })?;

        Ok(Self {
            music: music_loop,
            music_length: length_sec,
            frame_counter: 0.0,
            started: false,
        })
    }

    /// Replays a sound if it it finished playing.
    /// This uses the principle that draw calls happen at a certain fps to guarantee accurate replays
    /// Very experimental, use at own risk
    pub fn play_if_not(&mut self, curr_fps: f64) -> Result<()> {
        let frame_inc;

        //Prune edge case where fps is 0
        if curr_fps == 0.0 { frame_inc = 1.0 / 60.0; } else { frame_inc = 1.0 / curr_fps; }

        //increment counters
        if self.frame_counter >= self.music_length {
            self.frame_counter = frame_inc; //reset and increment
            self.started = false; //we need to restart music
        } else {
            self.frame_counter += frame_inc; //increment
        }

        //Play music if needed
        if self.started == false {
            self.started = true;
            self.music.execute(|music| { music.play() })?;
        }

        Ok(())
    }
}