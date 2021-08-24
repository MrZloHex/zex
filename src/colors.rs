use tui::style::Color;

#[derive(Clone)]
pub struct ByteColors {
    ascii: Color,
    space_line: Color,
    service_symbols: Color,
    zero: Color
}

impl ByteColors {
    pub fn new() -> ByteColors {
        ByteColors {
            ascii: Color::Rgb(184, 187, 38),
            space_line: Color::Rgb(67, 133, 136),
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
            self.service_symbols.clone()
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

