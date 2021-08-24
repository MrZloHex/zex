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
            ascii: Color::Rgb(79, 141, 195),
            space_line: Color::Rgb(240, 157, 48),
            service_symbols: Color::Rgb(123, 230, 126),
            zero: Color::Rgb(200, 200, 200)
        }
    }

    pub fn get_byte_color(&self, byte: u8) -> Color {
        if *byte > 32 && *byte < 127 {
            self.ascii.clone()
        } else if *byte == 32 || *byte == 10 {
            self.space_line.clone()
        } else if (*byte > 0 && *byte < 32) || *byte == 127 {
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
            background: Color::Rgb(20, 20, 20),
            border_style: Color::Rgb(150, 150, 150),
            title: Color::Rgb(200, 200, 200),
            selected: Color::Rgb(100, 100, 100),
            text: Color::Rgb(200, 200, 200),
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

