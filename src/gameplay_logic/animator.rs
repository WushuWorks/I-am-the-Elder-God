use quicksilver::prelude::*;
//Std
use std::iter::Cycle;
use std::vec::IntoIter;

///Helps Animation by holding a series of key and returning the next one every time a delay passes
pub struct Animator {
    curr: String,
    keys: Cycle<IntoIter<String>>,
    delay: f64,
    frame_counter: f64,
}

impl Animator {
    /// Make a rudimentary animator. This holds an a s
    pub fn new(mut keys: Cycle<IntoIter<String>>, delay: f64) -> Result<Self> {
        Ok(Self {
            curr: keys.next().expect("cannot load first key Animator"),
            keys,
            delay,
            frame_counter: 0.0,
        })
    }

    /// Returns the next animation key if delay has passed
    pub fn next_if_not(&mut self, curr_fps: f64) -> Result<&String> {
        let frame_inc;

        //Prune edge case where fps is 0
        if curr_fps == 0.0 { frame_inc = 1.0 / 60.0; } else { frame_inc = 1.0 / curr_fps; }

        //increment counters
        if self.frame_counter >= self.delay {
            self.frame_counter = frame_inc; //reset and increment
            self.curr = self.keys.next().expect("Cannot find nect key in Animator.");
        } else {
            self.frame_counter += frame_inc; //increment
        }

        Ok(&self.curr)
    }
}