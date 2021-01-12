use std::io::Write;
use crate::parser::SheetMusic;

const SAMPLE_RATE: u32 = 108000;
const NUM_CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;
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
        for tune in tunes.iter() {
            let duration = 60.0 / speed * tune.beat * 1000.0;
            music.add_note(tune.note.to_freq(), duration);
        }
        music
    }

    fn add_note(&mut self, freq: f32, duration: f32) {
        let a = self.header.bytes_per_sec as f32 / freq;

        for i in 0..(self.header.bytes_per_sec as f32 * duration / 1000.0) as usize {
            self.buf.push((64.0 * f32::sin(std::f32::consts::PI * 2.0 / a * i as f32)) as u8);
        }

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
