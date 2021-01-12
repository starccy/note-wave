#[allow(unused)]
mod freq {
    pub const NONE: (f32, &str) = (0.0, "0");

    pub const C3: (f32, &str) = (130.81, "1.");
    pub const D3: (f32, &str) = (146.83, "2.");
    pub const E3: (f32, &str) = (164.81, "3.");
    pub const F3: (f32, &str) = (174.61, "4.");
    pub const G3: (f32, &str) = (196.00, "5.");
    pub const A3: (f32, &str) = (220.00, "6.");
    pub const B3: (f32, &str) = (246.94, "7.");
    pub const C4: (f32, &str) = (261.63, "1");
    pub const C4S: (f32, &str) = (277.18, "1#");
    pub const D4: (f32, &str) = (293.66, "2");
    pub const D4S: (f32, &str) = (311.13, "2#");
    pub const E4: (f32, &str) = (329.63, "3");
    pub const F4: (f32, &str) = (349.23, "4");
    pub const F4S: (f32, &str) = (369.99, "4#");
    pub const G4: (f32, &str) = (392.00, "5");
    pub const G4S: (f32, &str) = (415.30, "5#");
    pub const A4: (f32, &str) = (440.00, "6");
    pub const A4S: (f32, &str) = (466.16, "6#");
    pub const B4: (f32, &str) = (493.88, "7");
    pub const C5: (f32, &str) = (523.25, "1^");
    pub const D5: (f32, &str) = (587.33, "2^");
    pub const E5: (f32, &str) = (659.26, "3^");
    pub const F5: (f32, &str) = (698.46, "4^");
    pub const G5: (f32, &str) = (783.99, "5^");
    pub const A5: (f32, &str) = (880.00, "6^");
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

impl_note!(NONE, C3, D3, E3, F3, G3, A3, B3,
    C4, C4S, D4, D4S, E4, F4, F4S, G4, G4S, A4, A4S, B4,
    C5, D5, E5, F5, G5, A5, B5
);
