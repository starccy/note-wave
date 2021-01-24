use std::io::Write;
use crate::parser::{SheetMusic, Tune};

const SAMPLE_RATE: u32 = 108000;
const NUM_CHANNELS: u16 = 2;
const BITS_PER_SAMPLE: u16 = 24;
const BYTE_RATE: u32 = SAMPLE_RATE * (NUM_CHANNELS * BITS_PER_SAMPLE / 8) as u32;

#[repr(C)]
#[derive(Clone)]
struct WavHeader {
    riff: [u8; 4],
    chunk_size: u32,
    wave: [u8; 4],
    fmt: [u8; 4],
    sub_chunk_size: u32,
    audio_format: u16,
    num_of_chan: u16,
    samples_per_sec: u32,
    bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
    sub_chunk_2_id: [u8; 4],
    sub_chunk_2_size: u32,
}

impl Default for WavHeader {
    fn default() -> Self {
        Self {
            riff: *b"RIFF",
            chunk_size: 0,
            wave: *b"WAVE",
            fmt: *b"fmt ",
            sub_chunk_size: 16,
            audio_format: 1,
            num_of_chan: NUM_CHANNELS,
            samples_per_sec: SAMPLE_RATE,
            bytes_per_sec: BYTE_RATE,
            block_align: 2,
            bits_per_sample: BITS_PER_SAMPLE,
            sub_chunk_2_id: *b"data",
            sub_chunk_2_size: 0,
        }
    }
}

impl WavHeader {
    fn set_size(&mut self, data_size: u32) {
        let self_size = std::mem::size_of::<Self>() as u32;
        self.chunk_size = data_size + self_size - 8;
        self.sub_chunk_2_size = data_size + self_size - 44;
    }
}

pub struct Music {
    header: WavHeader,
    buf: Vec<u8>,
}

impl Music {
    fn new() -> Self {
        Self {
            header: WavHeader::default(),
            buf: Vec::new()
        }
    }

    pub fn from_sheet_music(sheet_music: SheetMusic) -> Self {
        let mut music = Music::new();
        let speed = sheet_music.speed;
        let tunes = sheet_music.tunes;
        let non_channel_tunes = tunes.iter().filter(|tune| tune.channel == 0).collect::<Vec<_>>();
        // if not empty, means the whole sheet music is single channel
        if !non_channel_tunes.is_empty() {
            for tune in tunes.iter() {
                music.add_note(tune.note.to_freq(), calc_duration(speed, tune.beat));
            }
            return music;
        }
        let left_channel_tunes = tunes.iter().filter(|tune| tune.channel == 1).collect::<Vec<_>>();
        let right_channel_tunes = tunes.iter().filter(|tune| tune.channel == 2).collect::<Vec<_>>();
        music.add_multi_channel_notes(&left_channel_tunes, &right_channel_tunes, speed);
        music
    }

    fn add_multi_channel_notes(&mut self, left_tunes: &Vec<&Tune>, right_tunes: &Vec<&Tune>, speed: f32) {
        let l_byte_len = left_tunes.iter().fold(0, |acc, &x| acc + self.duration_len(calc_duration(speed, x.beat)));
        let r_byte_len = right_tunes.iter().fold(0, |acc, &x| acc + self.duration_len(calc_duration(speed, x.beat)));
        self.buf.resize(std::cmp::max(l_byte_len, r_byte_len) * 2, 0);
        // let mut l_tune_iter = left_tunes.iter();
        // let mut r_tune_iter = right_tunes.iter();
        // let mut l_tune = l_tune_iter.next();
        // let mut r_tune = r_tune_iter.next();
        // let mut l_finished = false;
        // let mut r_finished = false;
        // while l_tune.is_some() || r_tune.is_some() {
        //     l_finished = l_tune.is_none();
        //     r_finished = r_tune.is_none();
        //     let (l_freq, l_duration) = Self::freq_duration_or_default(l_tune, speed);
        //     let (r_freq, r_duration) = Self::freq_duration_or_default(r_tune, speed);
        //
        // }
        let mut idx = 0;
        for l_tune in left_tunes.iter() {
            for i in 0..self.duration_len(calc_duration(speed, l_tune.beat)) {
                self.buf[idx] = self.to_byte_data(i, l_tune.note.to_freq());
                idx += 2;
                // self.buf.push(self.to_byte_data(i, l_tune.note.to_freq()));
                // self.buf.push(self.to_byte_data(i, 0.0));
            }
        }
        let mut idx = 1;
        for r_tune in right_tunes.iter() {
            for i in 0..self.duration_len(calc_duration(speed, r_tune.beat)) {
                self.buf[idx] = self.to_byte_data(i, r_tune.note.to_freq());
                idx += 2;
            }
        }
    }

    fn freq_duration_or_default(tune: Option<&&Tune>, speed: f32) -> (f32, f32) {
        if let Some(tune) = tune {
            (tune.note.to_freq(), calc_duration(speed, tune.beat))
        } else {
            (0.0, 0.0)
        }
    }

    fn add_note(&mut self, freq: f32, duration: f32) {
        // 多声道 循环 除以 num_of_chan
        // 循环里面 push 的频率 a 要 乘以 num_of_chan
        for i in 0..self.duration_len(duration) {
            for _ in 0..NUM_CHANNELS {
                self.buf.push(self.to_byte_data(i, freq));
            }
        }
    }

    fn to_byte_data(&self, n: usize, freq: f32) -> u8 {
        let a = self.header.bytes_per_sec as f32 / freq;
        (64.0 * f32::sin(std::f32::consts::PI * 2.0 / a * NUM_CHANNELS as f32 * n as f32)) as u8
    }

    fn duration_len(&self, duration: f32) -> usize {
        (self.header.bytes_per_sec as f32 * duration / 1000.0 / NUM_CHANNELS as f32) as usize
    }

    pub fn write_to_file(&mut self, filename: &str) {
        let mut file = std::fs::File::create(filename).unwrap();

        self.header.set_size(self.buf.len() as u32);
        let slice = unsafe { std::mem::transmute::<WavHeader, [u8; std::mem::size_of::<WavHeader>()]>(self.header.clone())
        };

        file.write(&slice).unwrap();
        file.write(&self.buf).unwrap();
    }
}

fn calc_duration(speed: f32, beat: f32) -> f32 {
    60.0 / speed * beat * 1000.0
}
