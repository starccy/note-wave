#[allow(unused)]
mod freq {
    pub const NONE: (f32, &str) = (0.0, "0");

    pub const C2: (f32, &str) = (65.41, "1..");
    pub const C2S: (f32, &str) = (69.30, "1..#");
    pub const D2B: (f32, &str) = (69.30, "2..b");
    pub const D2: (f32, &str) = (73.42, "2..");
    pub const D2S: (f32, &str) = (77.78, "2..#");
    pub const E2B: (f32, &str) = (77.78, "3..b");
    pub const E2: (f32, &str) = (82.41, "3..");
    pub const F2: (f32, &str) = (87.31, "4..");
    pub const F2S: (f32, &str) = (92.50, "4..#");
    pub const G2B: (f32, &str) = (92.50, "5..b");
    pub const G2: (f32, &str) = (98.00, "5..");
    pub const G2S: (f32, &str) = (103.83, "5..#");
    pub const A2B: (f32, &str) = (103.83, "6..b");
    pub const A2: (f32, &str) = (110.00, "6..");
    pub const A2S: (f32, &str) = (116.54, "6..#");
    pub const B2B: (f32, &str) = (116.54, "7..b");
    pub const B2: (f32, &str) = (123.47, "7..");

    pub const C3: (f32, &str) = (130.81, "1.");
    pub const C3S: (f32, &str) = (138.59, "1.#");
    pub const D3B: (f32, &str) = (138.59, "2.b");
    pub const D3: (f32, &str) = (146.83, "2.");
    pub const D3S: (f32, &str) = (155.56, "2.#");
    pub const E3B: (f32, &str) = (155.56, "3.b");
    pub const E3: (f32, &str) = (164.81, "3.");
    pub const F3: (f32, &str) = (174.61, "4.");
    pub const F3S: (f32, &str) = (185.00, "4.#");
    pub const G3B: (f32, &str) = (185.00, "5.b");
    pub const G3: (f32, &str) = (196.00, "5.");
    pub const G3S: (f32, &str) = (207.65, "5.#");
    pub const A3B: (f32, &str) = (207.65, "6.b");
    pub const A3: (f32, &str) = (220.00, "6.");
    pub const A3S: (f32, &str) = (233.08, "6.#");
    pub const B3B: (f32, &str) = (233.08, "7.b");
    pub const B3: (f32, &str) = (246.94, "7.");

    pub const C4: (f32, &str) = (261.63, "1");
    pub const C4S: (f32, &str) = (277.18, "1#");
    pub const D4B: (f32, &str) = (277.18, "2b");
    pub const D4: (f32, &str) = (293.66, "2");
    pub const D4S: (f32, &str) = (311.13, "2#");
    pub const E4B: (f32, &str) = (311.13, "3b");
    pub const E4: (f32, &str) = (329.63, "3");
    pub const F4: (f32, &str) = (349.23, "4");
    pub const F4S: (f32, &str) = (369.99, "4#");
    pub const G4B: (f32, &str) = (369.99, "5b");
    pub const G4: (f32, &str) = (392.00, "5");
    pub const G4S: (f32, &str) = (415.30, "5#");
    pub const A4B: (f32, &str) = (415.30, "6b");
    pub const A4: (f32, &str) = (440.00, "6");
    pub const A4S: (f32, &str) = (466.16, "6#");
    pub const B4B: (f32, &str) = (466.16, "7b");
    pub const B4: (f32, &str) = (493.88, "7");

    pub const C5: (f32, &str) = (523.25, "1^");
    pub const C5S: (f32, &str) = (554.37, "1^#");
    pub const D5B: (f32, &str) = (554.37, "2^b");
    pub const D5: (f32, &str) = (587.33, "2^");
    pub const D5S: (f32, &str) = (622.25, "2^#");
    pub const E5B: (f32, &str) = (622.25, "3^b");
    pub const E5: (f32, &str) = (659.26, "3^");
    pub const F5: (f32, &str) = (698.46, "4^");
    pub const F5S: (f32, &str) = (739.99, "4^#");
    pub const G5B: (f32, &str) = (739.99, "5^b");
    pub const G5: (f32, &str) = (783.99, "5^");
    pub const G5S: (f32, &str) = (830.61, "5^#");
    pub const A5B: (f32, &str) = (830.61, "6^b");
    pub const A5: (f32, &str) = (880.00, "6^");
    pub const A5S: (f32, &str) = (932.33, "6^#");
    pub const B5B: (f32, &str) = (932.33, "7^b");
    pub const B5: (f32, &str) = (987.77, "7^");
}


macro_rules! impl_note {
    ($($note:ident),+) => {
    #[allow(unused)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Note {
            $($note),+
        }

        impl Note {
            pub fn to_freq(&self) -> f32 {
                match self {
                    $(Self::$note => freq::$note.0,)+
                }
            }

            pub fn from_str(note_str: &str) -> Self {
                match note_str {
                    $(x if x == freq::$note.1 => Self::$note,)+
                    _ => panic!("this pattern: \"{}\" cannot map to Note", note_str),
                }
            }
        }
    };
}

impl_note!(NONE,
    C2, C2S, D2B, D2, D2S, E2B, E2, F2, F2S, G2B, G2, G2S, A2B, A2, A2S, B2B, B2,
    C3, C3S, D3B, D3, D3S, E3B, E3, F3, F3S, G3B, G3, G3S, A3B, A3, A3S, B3B, B3,
    C4, C4S, D4B, D4, D4S, E4B, E4, F4, F4S, G4B, G4, G4S, A4B, A4, A4S, B4B, B4,
    C5, C5S, D5B, D5, D5S, E5B, E5, F5, F5S, G5B, G5, G5S, A5B, A5, A5S, B5B, B5
);
