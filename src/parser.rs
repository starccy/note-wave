use crate::note::Note;

#[derive(Debug, Copy, Clone)]
pub struct Tune {
    pub note: Note,
    pub beat: f32,
}

pub struct Parser<'a> {
    content:&'a str,
    idx: usize,
}

#[derive(Debug)]
pub struct SheetMusic {
    pub speed: f32,
    pub tunes: Vec<Tune>,
}

impl SheetMusic {
    pub fn add_into_blank_note(&mut self, beat: f32) {
        let blank_note = Tune {
            note: Note::NONE,
            beat,
        };
        let mut i = 1;
        while i < self.tunes.len() {
            if self.tunes[i].note == Note::NONE {
                i += 1;
                continue;
            }
            if self.tunes[i].note == self.tunes[i - 1].note {
                self.tunes.insert(i, blank_note);
            }
            i += 1;
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            idx: 0,
        }
    }

    fn parse_speed(line: &str) -> Option<f32> {
        let mut start = 0;
        let mut result = String::new();
        let line_u8 = line.as_bytes();
        while line_u8[start] == ' ' as u8 {
            start += 1;
        }
        let speed = "speed";
        let end = start + speed.len().min(line.len());
        if &line[start..end] != speed {
            return None;
        }
        start = end;
        while line_u8[start] == ' ' as u8 {
            start += 1;
        }
        if line_u8[start] != '=' as u8 {
            return None;
        }
        start += 1;
        while line_u8[start] == ' ' as u8 {
            start += 1;
        }
        while line_u8[start] != '\n' as u8 {
            result.push(line_u8[start] as char);
            start += 1;
        }
        Some(result.parse::<f32>().unwrap())
    }

    pub fn parse(&mut self) -> SheetMusic {
        let mut speed = None;
        let mut tunes = Vec::new();
        let mut has_speed = false;
        while let Some(line) = self.read_line() {
            if let Some(sp) = Self::parse_speed(line) {
                has_speed = true;
                speed.replace(sp);
                continue;
            }
            if !has_speed {
                continue;
            }
            let split_notes = Self::split_notes(line);
            let line_tunes = split_notes.into_iter().filter_map(Self::parse_note)
                .collect::<Vec<_>>();
            tunes.extend(line_tunes);
        }
        SheetMusic {
            tunes,
            speed: speed.unwrap(),
        }
    }

    fn split_notes(line: &str) -> Vec<&str> {
        let line_u8 = line.as_bytes();
        let len = line_u8.len();
        let mut result = Vec::new();
        let mut start = 0;
        while start < len && line_u8[start] != '\n' as u8 {
            while line_u8[start] == ' ' as u8 {
                start += 1;
            }
            let cur_start = start;
            if start >= len {
                let trim_content = &line[cur_start..len].trim();
                if *trim_content != "" {
                    result.push(*trim_content);
                    break;
                }
            }
            while start < len && line_u8[start] != ' ' as u8 {
                start += 1;
            }
            let cur_end = start;
            result.push(&line[cur_start..cur_end]);
        }
        result
    }

    fn read_line(&mut self) -> Option<&str> {
        let content = self.content.as_bytes();
        let start = self.idx;
        while self.idx < content.len() - 1 {
            if content[self.idx] == '\n' as u8 {
                self.idx += 1;
                break;
            }
            self.idx += 1;
        }
        if self.idx == start {
            None
        } else {
            Some(&self.content[start..self.idx])
        }
    }

    fn parse_note(note: &str) -> Option<Tune> {
        let mut start = 0;
        let len = note.len();
        let note_u8 = note.as_bytes();

        let cur_note = note_u8[start] as char;

        // may helpful for read but useless in parse, skip
        if cur_note == '|' {
            return None;
        }

        start += 1;

        // parse tune low / high
        let height_start = start;
        while start < len && (note_u8[start] == '^' as u8
            || note_u8[start] == '.' as u8)
        {
            start += 1;
        }
        let height_end = start;
        let note_height = &note[height_start..height_end];

        // parse mark
        let mark_start = start;
        if start < len && (note_u8[start] == '#' as u8
            || note_u8[start] == 'b' as u8)
        {
            start += 1;
        }
        let mark_end = start;
        let note_mark = &note[mark_start..mark_end];

        // parse beats duration
        let beat_start = start;
        while start < len && (note_u8[start] == '_' as u8
            || note_u8[start] == '-' as u8
            || note_u8[start] == '*' as u8)
        {
            start += 1;
        }
        let beat_end = start;
        let note_beat = &note[beat_start..beat_end];

        // skip rest white char
        while start < len && (note_u8[start].is_ascii_whitespace()) {
            start += 1;
        }
        if start < len {
            panic!("cannot parse \"{}\", got unknown piece: \"{}\", cur pos: {}", note, &note[start..len], start);
        }

        let note_str = format!("{note}{height}{mark}", note=cur_note, height=note_height, mark=note_mark);
        let note = Note::from_str(&note_str);
        let beat = Self::calc_beats(note_beat);
        Some(Tune {
            note,
            beat,
        })
    }

    fn calc_beats(beats: &str) -> f32 {
        if beats.is_empty() {
            return 1.0;
        }
        let mut result = 0.0;
        for beat in beats.chars() {
            let duration_rate = match beat {
                '_' => if result != 0.0 { -result * 0.5 } else { 0.5 },
                '-' => if result != 0.0 { 1.0 } else { 2.0 },
                '*' => if result != 0.0 { result * 0.5 } else { 1.5 },
                _ => panic!("cannot parse beat seq: \"{}\", got unexpected beat: '{}'", beats, beat),
            };
            result += duration_rate;
        }
        result
    }
}

