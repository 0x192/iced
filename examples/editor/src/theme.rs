use iced::overlay::menu;
use iced::widget::{
    button, checkbox, container, pick_list, radio, rule, scrollable, text, text_input, text_editor
};
use iced::{color, application, Background, Color, Border};

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub fn palette(self) -> ColorPalette {
        match self {
            Self::Dark => ColorPalette {
                base: BaseColors {
                    background: color!(0x111111),
                    foreground: color!(0x1C1C1C),
                },
                normal: NormalColors {
                    primary: color!(0x5E4266),
                    secondary: color!(0x386e50),
                    surface: color!(0x828282),
                    error: color!(0x992B2B),
                },
                bright: BrightColors {
                    primary: color!(0xBA84FC),
                    secondary: color!(0x49eb7a),
                    surface: color!(0xE0E0E0),
                    error: color!(0xC13047),
                },
            },
            Self::Light => ColorPalette {
                base: BaseColors {
                    background: color!(0xEEEEEE),
                    foreground: color!(0xE0E0E0),
                },
                normal: NormalColors {
                    primary: color!(0x230F08),
                    secondary: color!(0xF9D659),
                    surface: color!(0x818181),
                    error: color!(0x992B2B),
                },
                bright: BrightColors {
                    primary: color!(0x673AB7),
                    secondary: color!(0x3797A4),
                    surface: color!(0x000000),
                    error: color!(0xC13047),
                },
            },
        }
    }
}


#[derive(Default, Debug, Clone, Copy)]
pub enum Application {
    #[default]
    Default,
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().base.background,
            text_color: self.palette().bright.surface,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    #[default]
    Invisible,
    Frame,
    BorderedFrame,
    Tooltip,
    Background,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Invisible => container::Appearance::default(),
            Container::Frame => container::Appearance {
                background: Some(Background::Color(self.palette().base.foreground)),
                text_color: Some(self.palette().bright.surface),
                border: Border {
                    radius: 5.0.into(),
                    ..Border::default()
                },
                ..container::Appearance::default()
            },
            Container::BorderedFrame => container::Appearance {
                background: Some(Background::Color(self.palette().base.foreground)),
                text_color: Some(self.palette().bright.surface),
                border: Border {
                    width: 1.0,
                    radius: 5.0.into(),
                    color: self.palette().normal.error,
                    ..Border::default()
                },
                ..container::Appearance::default()
            },
            Container::Tooltip => container::Appearance {
                background: Some(Background::Color(self.palette().base.foreground)),
                text_color: Some(self.palette().bright.surface),
                border: Border {
                    width: 1.0,
                    radius: 8.0.into(),
                    color: self.palette().normal.primary,
                    ..Border::default()
                },
                ..container::Appearance::default()
            },

            Container::Background => container::Appearance {
                background: Some(Background::Color(self.palette().base.background)),
                text_color: Some(self.palette().bright.surface),
                border: Border {
                    radius: 5.0.into(),
                    ..Border::default()
                },
                ..container::Appearance::default()
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    #[default]
    Primary,
    Unavailable,
    SelfUpdate,
    Refresh,
    UninstallPackage,
    RestorePackage,
    NormalPackage,
    SelectedPackage,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let p = self.palette();

        let appearance = button::Appearance {
            border: Border {
                width: 1.0,
                radius: 2.0.into(),
                ..Border::default()
            },
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>, mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.base.foreground))),
            border: Border {
                color: Color { a: 0.5, ..mc },
                radius: 2.0.into(),
                ..Border::default()
            },
            text_color: mc,
            ..appearance
        };

        match style {
            Button::Primary | Button::SelfUpdate | Button::Refresh => {
                active_appearance(None, p.bright.primary)
            }
            Button::RestorePackage => active_appearance(None, p.bright.secondary),
            Button::NormalPackage => button::Appearance {
                background: Some(Background::Color(p.base.foreground)),
                text_color: p.bright.surface,
                border: Border {
                    color: p.base.background,
                    width: 0.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
                ..appearance
            },
            Button::SelectedPackage => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..p.normal.primary
                })),
                text_color: p.bright.primary,
                border: Border {
                    color: p.normal.primary,
                    width: 0.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
                ..appearance
            },
            Button::Unavailable | Button::UninstallPackage => {
                active_appearance(None, p.bright.error)
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette();

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.25, ..bg })),
            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Primary | Button::SelfUpdate | Button::Refresh => {
                hover_appearance(p.bright.primary, None)
            }
            Button::NormalPackage => hover_appearance(p.normal.primary, Some(p.bright.surface)),
            Button::SelectedPackage => hover_appearance(p.normal.primary, None),
            Button::RestorePackage => hover_appearance(p.bright.secondary, None),
            Button::Unavailable | Button::UninstallPackage => {
                hover_appearance(p.bright.error, None)
            }
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette();

        let disabled_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.05, ..bg })),
            text_color: Color {
                a: 0.50,
                ..tc.unwrap_or(bg)
            },
            ..active
        };

        match style {
            Button::RestorePackage => disabled_appearance(p.normal.primary, Some(p.bright.primary)),
            Button::UninstallPackage => disabled_appearance(p.bright.error, None),
            Button::Primary => disabled_appearance(p.bright.primary, Some(p.bright.primary)),
            _ => active,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Description,
    Packages,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let from_appearance = |c: Color| scrollable::Scrollbar {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                color:  Color::TRANSPARENT,
                width: 0.0,
                radius: 5.0.into(),
                ..Border::default()
            },
            scroller: scrollable::Scroller {
                color: c,
                border: Border {
                    color:  Color::TRANSPARENT,
                    width: 1.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
            },
        };

        match style {
            Scrollable::Description => from_appearance(self.palette().normal.surface),
            Scrollable::Packages => from_appearance(self.palette().base.foreground),
        }
    }

    fn hovered(&self, style: &Self::Style, _mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            scroller: self.active(style).scroller,
            ..self.active(style)
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style, true);
        scrollable::Scrollbar {
            scroller: hovered.scroller,
            ..hovered
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    PackageEnabled,
    PackageDisabled,
    SettingsEnabled,
    SettingsDisabled,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckBox::PackageEnabled => checkbox::Appearance {
                background: Background::Color(self.palette().base.background),
                icon_color: self.palette().bright.primary,
                border: Border {
                    color:  self.palette().base.background,
                    width: 1.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
                text_color: Some(self.palette().bright.surface),
            },
            CheckBox::PackageDisabled => checkbox::Appearance {
                background: Background::Color(Color {
                    a: 0.55,
                    ..self.palette().base.background
                }),
                icon_color: self.palette().bright.primary,
                border: Border {
                    color:  self.palette().normal.primary,
                    width: 1.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
                text_color: Some(self.palette().normal.primary),
            },
            CheckBox::SettingsEnabled => checkbox::Appearance {
                background: Background::Color(self.palette().base.background),
                icon_color: self.palette().bright.primary,
                border: Border {
                    color:  self.palette().bright.primary,
                    width: 1.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
                text_color: Some(self.palette().bright.surface),
            },
            CheckBox::SettingsDisabled => checkbox::Appearance {
                background: Background::Color(self.palette().base.foreground),
                icon_color: self.palette().bright.primary,
                border: Border {
                    color:  self.palette().normal.primary,
                    width: 1.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
                text_color: Some(self.palette().bright.surface),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let from_appearance = || checkbox::Appearance {
            background: Background::Color(self.palette().base.foreground),
            icon_color: self.palette().bright.primary,
            border: Border {
                    color:  self.palette().bright.primary,
                    width: 2.0,
                    radius: 5.0.into(),
                    ..Border::default()
                },
            text_color: Some(self.palette().bright.surface),
        };

        match style {
            CheckBox::PackageEnabled | CheckBox::SettingsEnabled => from_appearance(),
            CheckBox::PackageDisabled | CheckBox::SettingsDisabled => {
                self.active(style, is_checked)
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border: Border {
                color:  self.palette().base.foreground,
                width: 0.0,
                radius: 5.0.into(),
                ..Border::default()
            },
            icon_color: self.palette().bright.surface
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border: Border {
                color:  self.palette().normal.primary,
                width: 1.0,
                radius: 2.0.into(),
                ..Border::default()
            },
            icon_color:self.palette().bright.surface
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border: Border {
                color:  self.palette().normal.primary,
                width: 1.0,
                radius: 2.0.into(),
                ..Border::default()
            },
            icon_color: self.palette().bright.surface
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette().normal.surface
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.palette().bright.primary
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.palette().normal.primary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum TextEditor {
    #[default]
    Default,
    Test,
}

impl text_editor::StyleSheet for Theme {
    type Style = TextEditor;

    fn active(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border: Border {
                color:  self.palette().base.foreground,
                width: 0.0,
                radius: 2.0.into(),
                ..Border::default()
            },
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border: Border {
                color:  Color {
                    a: 0.5,
                    ..self.palette().normal.primary
                },
                width: 1.0,
                radius: 2.0.into(),
                ..Border::default()
            },
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border: Border {
                color:  self.palette().normal.primary,
                width: 1.0,
                radius: 2.0.into(),
                ..Border::default()
            },
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette().normal.surface
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.palette().bright.primary
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.palette().normal.primary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> text_editor::Appearance {
        self.focused(style)
    }
}


#[derive(Default, Debug, Clone, Copy)]
pub enum PickList {
    #[default]
    Default,
}

impl menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        let p = self.palette();

        menu::Appearance {
            text_color: p.bright.surface,
            background: p.base.background.into(),
            border: Border {
                color:  p.base.background,
                width: 1.0,
                radius: 2.0.into(),
                ..Border::default()
            },
            selected_text_color: p.bright.surface,
            selected_background: p.normal.primary.into(),
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.palette().bright.surface,
            background: self.palette().base.background.into(),
            border: Border {
                color:  Color {
                    a: 0.5,
                    ..self.palette().normal.primary
                },
                width: 1.0,
                radius: 2.0.into(),
                ..Border::default()
            },
            handle_color: self.palette().bright.surface,
            placeholder_color: self.palette().bright.surface,
        }
    }

    fn hovered(&self, style: &()) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            border: Border {
                color:  self.palette().normal.primary,
                ..Border::default()
            },
            ..active
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Ok,
    Danger,
    Commentary,
    Color(Color),
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => text::Appearance::default(),
            Text::Ok => text::Appearance {
                color: Some(self.palette().bright.secondary),
            },
            Text::Danger => text::Appearance {
                color: Some(self.palette().bright.error),
            },
            Text::Commentary => text::Appearance {
                color: Some(self.palette().normal.surface),
            },
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: Color::TRANSPARENT.into(),
            dot_color: self.palette().bright.primary,
            border_width: 1.0,
            border_color: self.palette().bright.primary,
            text_color: None,
        }
    }

    fn hovered(&self, style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        let active = self.active(style, true);

        radio::Appearance {
            dot_color: self.palette().bright.primary,
            border_color: self.palette().bright.primary,
            border_width: 2.0,
            ..active
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Rule {
    #[default]
    Default,
}

impl rule::StyleSheet for Theme {
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        match style {
            Rule::Default => rule::Appearance {
                color: self.palette().bright.surface,
                width: 2,
                radius: 2.0.into(),
                fill_mode: rule::FillMode::Full,
            },
        }
    }
}
