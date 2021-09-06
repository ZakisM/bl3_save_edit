use iced::Font;

pub const OSWALD_MEDIUM: Font = Font::External {
    name: "Oswald Medium",
    bytes: include_bytes!("../../resources/font/Oswald-Medium.ttf"),
};

pub const JETBRAINS_MONO: Font = Font::External {
    name: "Jetbrains Mono",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Regular.ttf"),
};

pub const JETBRAINS_MONO_BOLD: Font = Font::External {
    name: "Jetbrains Mono Bold",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Bold.ttf"),
};

pub const JETBRAINS_MONO_LIGHT_ITALIC: Font = Font::External {
    name: "Jetbrains Mono Light Italic",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-LightItalic.ttf"),
};
