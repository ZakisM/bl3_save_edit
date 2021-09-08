use iced::Font;

pub const JETBRAINS_MONO: Font = Font::External {
    name: "Jetbrains Mono",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Regular.ttf"),
};

pub const JETBRAINS_MONO_BOLD: Font = Font::External {
    name: "Jetbrains Mono Bold",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Bold.ttf"),
};

pub const JETBRAINS_MONO_NL_EXTRA_BOLD: Font = Font::External {
    name: "Jetbrains Mono NL Extra Bold",
    bytes: include_bytes!("../../resources/font/JetBrainsMonoNL-ExtraBold.ttf"),
};

pub const JETBRAINS_MONO_LIGHT_ITALIC: Font = Font::External {
    name: "Jetbrains Mono Light Italic",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-LightItalic.ttf"),
};
