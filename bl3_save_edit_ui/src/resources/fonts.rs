use iced::Font;

pub const COMPACTA: Font = Font::External {
    name: "Compacta",
    bytes: include_bytes!("../../resources/font/Compacta.otf"),
};

pub const JETBRAINS_MONO: Font = Font::External {
    name: "Jetbrains Mono",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Regular.ttf"),
};

pub const JETBRAINS_MONO_BOLD: Font = Font::External {
    name: "Jetbrains Mono Bold",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Bold.ttf"),
};
