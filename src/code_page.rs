#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CodePage {
    #[default]
    Ascii,
    Eastern,
    Cyrillic,
    ShiftJIS,
    Big5,
}

pub const ALL: &[CodePage] = &[
    CodePage::Ascii,
    CodePage::Big5,
    CodePage::Cyrillic,
    CodePage::Eastern,
    CodePage::ShiftJIS,
];

impl CodePage {
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Ascii => "West European (ASCII)",
            Self::Eastern => "East European (1250)",
            Self::Cyrillic => "Cyrillic (1251)",
            Self::ShiftJIS => "Japanese (Shift JIS)",
            Self::Big5 => "Chinese (Big5)",
        }
    }

    pub const fn to_encoding(self) -> &'static encoding_rs::Encoding {
        match self {
            Self::Ascii => encoding_rs::WINDOWS_1252,
            Self::Eastern => encoding_rs::WINDOWS_1250,
            Self::Cyrillic => encoding_rs::WINDOWS_1251,
            Self::ShiftJIS => encoding_rs::SHIFT_JIS,
            Self::Big5 => encoding_rs::BIG5,
        }
    }
}
