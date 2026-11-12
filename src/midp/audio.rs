//! MMAPI `Player` stand-in for MIDI playback using rustysynth.
//! If no SoundFont is available, players still advance in time silently so
//! game logic that waits for a tune to finish keeps working.

use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};
use std::sync::Arc;

pub const CLOSED: i32 = 0;
pub const UNREALIZED: i32 = 100;
pub const REALIZED: i32 = 200;
pub const PREFETCHED: i32 = 300;
pub const STARTED: i32 = 400;

pub const RATE: i32 = 44100;

struct Player {
    state: i32,
    midi: Arc<MidiFile>,
    seq: Option<MidiFileSequencer>,
    looping: bool,
    played: bool,
    elapsed: f64,
    length: f64,
    ended: bool,
}

pub struct Audio {
    sf: Option<Arc<SoundFont>>,
    players: Vec<Option<Player>>,
    ended: Vec<usize>,
    pub volume: f32,
    pub muted: bool,
    l: Vec<f32>,
    r: Vec<f32>,
}

impl Audio {
    pub fn new(sf: Option<Arc<SoundFont>>) -> Audio {
        Audio { sf, players: Vec::new(), ended: Vec::new(), volume: 2.0, muted: false, l: Vec::new(), r: Vec::new() }
    }

    pub fn load_soundfont(path: &std::path::Path) -> Option<Arc<SoundFont>> {
        let mut f = std::fs::File::open(path).ok()?;
        SoundFont::new(&mut f).ok().map(Arc::new)
    }

    /// `Manager.createPlayer(stream, "audio/midi")`
    pub fn create(&mut self, data: &[u8]) -> Option<usize> {
        let mut cur = std::io::Cursor::new(data);
        let midi = Arc::new(MidiFile::new(&mut cur).ok()?);
        let length = midi.get_length();
        let p = Player { state: UNREALIZED, midi, seq: None, looping: false, played: false, elapsed: 0.0, length, ended: false };
        if let Some(i) = self.players.iter().position(|p| p.is_none()) {
            self.players[i] = Some(p);
            Some(i)
        } else {
            self.players.push(Some(p));
            Some(self.players.len() - 1)
        }
    }

    fn p(&mut self, id: usize) -> Option<&mut Player> {
        self.players.get_mut(id).and_then(|p| p.as_mut())
    }

    pub fn state(&self, id: usize) -> i32 {
        self.players.get(id).and_then(|p| p.as_ref()).map(|p| p.state).unwrap_or(CLOSED)
    }

    pub fn realize(&mut self, id: usize) {
        if let Some(p) = self.p(id) {
            if p.state < REALIZED {
                p.state = REALIZED;
            }
        }
    }

    pub fn prefetch(&mut self, id: usize) {
        let sf = self.sf.clone();
        if let Some(p) = self.p(id) {
            if p.state < PREFETCHED {
                p.state = PREFETCHED;
            }
            if p.seq.is_none() {
                if let Some(sf) = sf {
                    let settings = SynthesizerSettings::new(RATE);
                    if let Ok(syn) = Synthesizer::new(&sf, &settings) {
                        p.seq = Some(MidiFileSequencer::new(syn));
                    }
                }
            }
        }
    }

    /// `setLoopCount(-1)` => loop forever
    pub fn set_loop_count(&mut self, id: usize, n: i32) {
        if let Some(p) = self.p(id) {
            p.looping = n == -1;
        }
    }

    pub fn start(&mut self, id: usize) {
        self.prefetch(id);
        if let Some(p) = self.p(id) {
            if !p.played {
                p.played = true;
                let midi = p.midi.clone();
                let looping = p.looping;
                if let Some(seq) = p.seq.as_mut() {
                    seq.play(&midi, looping);
                }
            }
            p.ended = false;
            p.state = STARTED;
        }
    }

    pub fn stop(&mut self, id: usize) {
        if let Some(p) = self.p(id) {
            if p.state == STARTED {
                p.state = PREFETCHED;
            }
        }
    }

    pub fn close(&mut self, id: usize) {
        if let Some(slot) = self.players.get_mut(id) {
            *slot = None;
        }
        self.ended.retain(|&e| e != id);
    }

    /// Players that reached END_OF_MEDIA since the last call.
    pub fn take_ended(&mut self) -> Vec<usize> {
        std::mem::take(&mut self.ended)
    }

    /// Advance all started players by `frames` samples and mix them into an
    /// interleaved stereo buffer.
    pub fn render(&mut self, frames: usize, out: &mut Vec<f32>) {
        out.clear();
        out.resize(frames * 2, 0.0);
        self.l.resize(frames, 0.0);
        self.r.resize(frames, 0.0);
        let dt = frames as f64 / RATE as f64;
        let gain = if self.muted { 0.0 } else { self.volume };
        for (id, slot) in self.players.iter_mut().enumerate() {
            let Some(p) = slot.as_mut() else { continue };
            if p.state != STARTED {
                continue;
            }
            p.elapsed += dt;
            if let Some(seq) = p.seq.as_mut() {
                seq.render(&mut self.l[..frames], &mut self.r[..frames]);
                for i in 0..frames {
                    out[i * 2] += self.l[i] * gain;
                    out[i * 2 + 1] += self.r[i] * gain;
                }
                if !p.looping && seq.end_of_sequence() && !p.ended {
                    p.ended = true;
                }
            } else if !p.looping && p.elapsed >= p.length && !p.ended {
                p.ended = true;
            }
            if p.ended && p.state == STARTED {
                p.state = PREFETCHED;
                self.ended.push(id);
            }
        }
        for s in out.iter_mut() {
            *s = s.clamp(-1.0, 1.0);
        }
    }
}
