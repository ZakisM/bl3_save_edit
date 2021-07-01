use iced::Font;

pub const COMPACTA: Font = Font::External {
    name: "Compacta",
    bytes: include_bytes!("../resources/font/Compacta.otf"),
};

pub const CABIN: Font = Font::External {
    name: "Cabin",
    bytes: include_bytes!("../resources/font/Cabin-Regular.ttf"),
};

pub const CABIN_BOLD: Font = Font::External {
    name: "Cabin Bold",
    bytes: include_bytes!("../resources/font/Cabin-Bold.ttf"),
};
