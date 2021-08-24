use tui::style::Color;

#[derive(Clone)]
pub struct ByteColors {
    ascii: Color,
    space_line: Color,
    buttons: Color,
    sep_ctrl: Color,
    service_symbols: Color,
    zero: Color
}

impl ByteColors {
    pub fn new() -> ByteColors {
        ByteColors {
            ascii: Color::Rgb(184, 187, 38),
            space_line: Color::Rgb(67, 133, 136),
            buttons: Color::Rgb(250, 189, 47),
            sep_ctrl: Color::Rgb(177, 98, 134),
            service_symbols: Color::Rgb(254, 128, 25),
            zero: Color::Rgb(146, 131, 116)
        }
    }

    pub fn get_byte_color(&self, byte: u8) -> Color {
        if byte > 32 && byte < 127 {
            self.ascii.clone()
        } else if byte == 32 || byte == 10 {
            self.space_line.clone()
        } else if (byte > 0 && byte < 32) || byte == 127 {
            match byte {
                8 | 9 | 13 | 14 | 15 | 27 | 127 => self.buttons.clone(),
                16..=20 | 28..=31 => self.sep_ctrl.clone(),
                _ => self.service_symbols.clone()
            }
        } else {
            self.zero.clone()
        }
    }
}


#[derive(Clone)]
pub struct ColorPallete {
    background: Color,
    border_style: Color,
    title: Color,
    selected: Color,
    text: Color,
    byte_colors: ByteColors
}

impl ColorPallete {
    pub fn new() -> Self {
        ColorPallete {
            background: Color::Rgb(40, 40, 40),
            border_style: Color::Rgb(80, 73, 69),
            title: Color::Rgb(235, 219, 178),
            selected: Color::Rgb(124, 111, 100),
            text: Color::Rgb(168, 153, 132),
            byte_colors: ByteColors::new()
        }
    }
    pub fn bg(&self) -> Color {
        self.background.clone()
    }
    pub fn bs(&self) -> Color {
        self.border_style.clone()
    }
    pub fn tl(&self) -> Color {
        self.title.clone()
    }
    pub fn slct(&self) -> Color {
        self.selected.clone()
    }
    pub fn text(&self) -> Color {
        self.text.clone()
    }
    pub fn bc(&self) -> ByteColors {
        self.byte_colors.clone()
    }
}

