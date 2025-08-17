use egui::{Color32, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    CatppuccinMocha,
    CatppuccinLatte,
    GruvboxDark,
    GruvboxLight,
    DraculaDark,
    SolarizedDark,
    SolarizedLight,
    TokyoNight,
    OneDark,
    MaterialDark,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeColors {
    pub primary: [u8; 3],
    pub secondary: [u8; 3],
    pub success: [u8; 3],
    pub warning: [u8; 3],
    pub error: [u8; 3],
    pub info: [u8; 3],
    pub text: [u8; 3],
    pub text_secondary: [u8; 3],
    pub text_muted: [u8; 3],
    pub background: [u8; 3],
    pub surface: [u8; 3],
    pub border: [u8; 3],
}

impl Default for CustomThemeColors {
    fn default() -> Self {
        // Default to Catppuccin Mocha colors
        Self {
            primary: [137, 180, 250],    // Blue
            secondary: [203, 166, 247],  // Mauve
            success: [166, 227, 161],    // Green
            warning: [249, 226, 175],    // Yellow
            error: [243, 139, 168],      // Red
            info: [137, 220, 235],       // Sky
            text: [205, 214, 244],       // Text
            text_secondary: [186, 194, 222], // Subtext1
            text_muted: [166, 173, 200], // Subtext0
            background: [30, 30, 46],    // Base
            surface: [49, 50, 68],       // Surface0
            border: [88, 91, 112],       // Overlay0
        }
    }
}

impl CustomThemeColors {
    pub fn to_theme_colors(&self) -> ThemeColors {
        ThemeColors {
            primary: Color32::from_rgb(self.primary[0], self.primary[1], self.primary[2]),
            secondary: Color32::from_rgb(self.secondary[0], self.secondary[1], self.secondary[2]),
            success: Color32::from_rgb(self.success[0], self.success[1], self.success[2]),
            warning: Color32::from_rgb(self.warning[0], self.warning[1], self.warning[2]),
            error: Color32::from_rgb(self.error[0], self.error[1], self.error[2]),
            info: Color32::from_rgb(self.info[0], self.info[1], self.info[2]),
            text: Color32::from_rgb(self.text[0], self.text[1], self.text[2]),
            text_secondary: Color32::from_rgb(self.text_secondary[0], self.text_secondary[1], self.text_secondary[2]),
            text_muted: Color32::from_rgb(self.text_muted[0], self.text_muted[1], self.text_muted[2]),
            background: Color32::from_rgb(self.background[0], self.background[1], self.background[2]),
            surface: Color32::from_rgb(self.surface[0], self.surface[1], self.surface[2]),
            border: Color32::from_rgb(self.border[0], self.border[1], self.border[2]),
        }
    }

    pub fn from_theme_colors(colors: &ThemeColors) -> Self {
        Self {
            primary: [colors.primary.r(), colors.primary.g(), colors.primary.b()],
            secondary: [colors.secondary.r(), colors.secondary.g(), colors.secondary.b()],
            success: [colors.success.r(), colors.success.g(), colors.success.b()],
            warning: [colors.warning.r(), colors.warning.g(), colors.warning.b()],
            error: [colors.error.r(), colors.error.g(), colors.error.b()],
            info: [colors.info.r(), colors.info.g(), colors.info.b()],
            text: [colors.text.r(), colors.text.g(), colors.text.b()],
            text_secondary: [colors.text_secondary.r(), colors.text_secondary.g(), colors.text_secondary.b()],
            text_muted: [colors.text_muted.r(), colors.text_muted.g(), colors.text_muted.b()],
            background: [colors.background.r(), colors.background.g(), colors.background.b()],
            surface: [colors.surface.r(), colors.surface.g(), colors.surface.b()],
            border: [colors.border.r(), colors.border.g(), colors.border.b()],
        }
    }
}

// Universal color scheme that works across all themes
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub primary: Color32,      // Main accent color
    pub secondary: Color32,    // Secondary accent color  
    pub success: Color32,      // Green for success states
    pub warning: Color32,      // Yellow/orange for warnings
    pub error: Color32,        // Red for errors
    pub info: Color32,         // Blue for info
    pub text: Color32,         // Primary text color
    pub text_secondary: Color32, // Secondary text color
    pub text_muted: Color32,   // Muted/disabled text
    pub background: Color32,   // Main background
    pub surface: Color32,      // Surface/panel background
    pub border: Color32,       // Border color
}

impl Default for Theme {
    fn default() -> Self {
        Theme::CatppuccinMocha
    }
}

impl Theme {
    pub fn name(&self) -> &'static str {
        match self {
            Theme::CatppuccinMocha => "Catppuccin Mocha",
            Theme::CatppuccinLatte => "Catppuccin Latte",
            Theme::GruvboxDark => "Gruvbox Dark",
            Theme::GruvboxLight => "Gruvbox Light",
            Theme::DraculaDark => "Dracula",
            Theme::SolarizedDark => "Solarized Dark",
            Theme::SolarizedLight => "Solarized Light",
            Theme::TokyoNight => "Tokyo Night",
            Theme::OneDark => "One Dark",
            Theme::MaterialDark => "Material Dark",
            Theme::Custom => "Custom",
        }
    }

    pub fn colors(&self, custom_colors: Option<&CustomThemeColors>) -> ThemeColors {
        match self {
            Theme::Custom => {
                if let Some(custom) = custom_colors {
                    custom.to_theme_colors()
                } else {
                    CustomThemeColors::default().to_theme_colors()
                }
            }
            _ => self.predefined_colors(),
        }
    }

    fn predefined_colors(&self) -> ThemeColors {
        match self {
            Theme::CatppuccinMocha => ThemeColors {
                primary: Color32::from_rgb(137, 180, 250),      // Blue
                secondary: Color32::from_rgb(203, 166, 247),    // Mauve
                success: Color32::from_rgb(166, 227, 161),      // Green
                warning: Color32::from_rgb(249, 226, 175),      // Yellow
                error: Color32::from_rgb(243, 139, 168),        // Red
                info: Color32::from_rgb(137, 220, 235),         // Sky
                text: Color32::from_rgb(205, 214, 244),         // Text
                text_secondary: Color32::from_rgb(186, 194, 222), // Subtext1
                text_muted: Color32::from_rgb(166, 173, 200),   // Subtext0
                background: Color32::from_rgb(30, 30, 46),      // Base
                surface: Color32::from_rgb(49, 50, 68),         // Surface0
                border: Color32::from_rgb(108, 112, 134),       // Overlay0
            },
            Theme::CatppuccinLatte => ThemeColors {
                primary: Color32::from_rgb(30, 102, 245),       // Blue
                secondary: Color32::from_rgb(136, 57, 239),     // Mauve
                success: Color32::from_rgb(64, 160, 43),        // Green
                warning: Color32::from_rgb(223, 142, 29),       // Yellow
                error: Color32::from_rgb(210, 15, 57),          // Red
                info: Color32::from_rgb(4, 165, 229),           // Sky
                text: Color32::from_rgb(76, 79, 105),           // Text
                text_secondary: Color32::from_rgb(108, 111, 133), // Subtext1
                text_muted: Color32::from_rgb(140, 143, 161),   // Subtext0
                background: Color32::from_rgb(239, 241, 245),   // Base
                surface: Color32::from_rgb(220, 224, 232),      // Surface0
                border: Color32::from_rgb(156, 160, 176),       // Overlay0
            },
            Theme::GruvboxDark => ThemeColors {
                primary: Color32::from_rgb(131, 165, 152),      // Aqua
                secondary: Color32::from_rgb(211, 134, 155),    // Purple
                success: Color32::from_rgb(184, 187, 38),       // Green
                warning: Color32::from_rgb(250, 189, 47),       // Yellow
                error: Color32::from_rgb(251, 73, 52),          // Red
                info: Color32::from_rgb(69, 133, 136),          // Blue
                text: Color32::from_rgb(235, 219, 178),         // Fg
                text_secondary: Color32::from_rgb(189, 174, 147), // Fg2
                text_muted: Color32::from_rgb(146, 131, 116),   // Fg4
                background: Color32::from_rgb(40, 40, 40),      // Bg
                surface: Color32::from_rgb(60, 56, 54),         // Bg1
                border: Color32::from_rgb(102, 92, 84),         // Bg3
            },
            Theme::GruvboxLight => ThemeColors {
                primary: Color32::from_rgb(66, 123, 88),        // Aqua
                secondary: Color32::from_rgb(143, 63, 113),     // Purple
                success: Color32::from_rgb(121, 116, 14),       // Green
                warning: Color32::from_rgb(181, 118, 20),       // Yellow
                error: Color32::from_rgb(157, 0, 6),            // Red
                info: Color32::from_rgb(7, 102, 120),           // Blue
                text: Color32::from_rgb(60, 56, 54),            // Fg
                text_secondary: Color32::from_rgb(80, 73, 69),  // Fg2
                text_muted: Color32::from_rgb(124, 111, 100),   // Fg4
                background: Color32::from_rgb(251, 241, 199),   // Bg
                surface: Color32::from_rgb(235, 219, 178),      // Bg1
                border: Color32::from_rgb(189, 174, 147),       // Bg3
            },
            Theme::DraculaDark => ThemeColors {
                primary: Color32::from_rgb(189, 147, 249),      // Purple
                secondary: Color32::from_rgb(255, 121, 198),    // Pink
                success: Color32::from_rgb(80, 250, 123),       // Green
                warning: Color32::from_rgb(241, 250, 140),      // Yellow
                error: Color32::from_rgb(255, 85, 85),          // Red
                info: Color32::from_rgb(139, 233, 253),         // Cyan
                text: Color32::from_rgb(248, 248, 242),         // Foreground
                text_secondary: Color32::from_rgb(198, 208, 245), // Comment
                text_muted: Color32::from_rgb(98, 114, 164),    // Comment dark
                background: Color32::from_rgb(40, 42, 54),      // Background
                surface: Color32::from_rgb(68, 71, 90),         // Current line
                border: Color32::from_rgb(98, 114, 164),        // Comment
            },
            Theme::SolarizedDark => ThemeColors {
                primary: Color32::from_rgb(38, 139, 210),       // Blue
                secondary: Color32::from_rgb(211, 54, 130),     // Magenta
                success: Color32::from_rgb(133, 153, 0),        // Green
                warning: Color32::from_rgb(181, 137, 0),        // Yellow
                error: Color32::from_rgb(220, 50, 47),          // Red
                info: Color32::from_rgb(42, 161, 152),          // Cyan
                text: Color32::from_rgb(131, 148, 150),         // Base0
                text_secondary: Color32::from_rgb(147, 161, 161), // Base1
                text_muted: Color32::from_rgb(88, 110, 117),    // Base01
                background: Color32::from_rgb(0, 43, 54),       // Base03
                surface: Color32::from_rgb(7, 54, 66),          // Base02
                border: Color32::from_rgb(88, 110, 117),        // Base01
            },
            Theme::SolarizedLight => ThemeColors {
                primary: Color32::from_rgb(38, 139, 210),       // Blue
                secondary: Color32::from_rgb(211, 54, 130),     // Magenta
                success: Color32::from_rgb(133, 153, 0),        // Green
                warning: Color32::from_rgb(181, 137, 0),        // Yellow
                error: Color32::from_rgb(220, 50, 47),          // Red
                info: Color32::from_rgb(42, 161, 152),          // Cyan
                text: Color32::from_rgb(101, 123, 131),         // Base00
                text_secondary: Color32::from_rgb(88, 110, 117), // Base01
                text_muted: Color32::from_rgb(147, 161, 161),   // Base1
                background: Color32::from_rgb(253, 246, 227),   // Base3
                surface: Color32::from_rgb(238, 232, 213),      // Base2
                border: Color32::from_rgb(147, 161, 161),       // Base1
            },
            Theme::TokyoNight => ThemeColors {
                primary: Color32::from_rgb(125, 207, 255),      // Blue
                secondary: Color32::from_rgb(187, 154, 247),    // Purple
                success: Color32::from_rgb(158, 206, 106),      // Green
                warning: Color32::from_rgb(224, 175, 104),      // Yellow
                error: Color32::from_rgb(247, 118, 142),        // Red
                info: Color32::from_rgb(125, 207, 255),         // Blue
                text: Color32::from_rgb(192, 202, 245),         // Fg
                text_secondary: Color32::from_rgb(169, 177, 214), // Fg_dark
                text_muted: Color32::from_rgb(86, 95, 137),     // Comment
                background: Color32::from_rgb(26, 27, 38),      // Bg
                surface: Color32::from_rgb(37, 40, 56),         // Bg_highlight
                border: Color32::from_rgb(65, 72, 104),         // Border
            },
            Theme::OneDark => ThemeColors {
                primary: Color32::from_rgb(97, 175, 239),       // Blue
                secondary: Color32::from_rgb(198, 120, 221),    // Purple
                success: Color32::from_rgb(152, 195, 121),      // Green
                warning: Color32::from_rgb(229, 192, 123),      // Yellow
                error: Color32::from_rgb(224, 108, 117),        // Red
                info: Color32::from_rgb(86, 182, 194),          // Cyan
                text: Color32::from_rgb(171, 178, 191),         // Fg
                text_secondary: Color32::from_rgb(92, 99, 112), // Comment
                text_muted: Color32::from_rgb(73, 80, 93),      // Gutter_fg_grey
                background: Color32::from_rgb(40, 44, 52),      // Black
                surface: Color32::from_rgb(33, 37, 43),         // Visual_grey
                border: Color32::from_rgb(92, 99, 112),         // Comment
            },
            Theme::MaterialDark => ThemeColors {
                primary: Color32::from_rgb(130, 170, 255),      // Blue
                secondary: Color32::from_rgb(199, 146, 234),    // Purple
                success: Color32::from_rgb(195, 232, 141),      // Green
                warning: Color32::from_rgb(255, 203, 107),      // Yellow
                error: Color32::from_rgb(240, 113, 120),        // Red
                info: Color32::from_rgb(137, 221, 255),         // Cyan
                text: Color32::from_rgb(233, 237, 241),         // Text
                text_secondary: Color32::from_rgb(176, 190, 197), // Text_secondary
                text_muted: Color32::from_rgb(84, 110, 122),    // Disabled
                background: Color32::from_rgb(38, 50, 56),      // Background
                surface: Color32::from_rgb(46, 60, 67),         // Surface
                border: Color32::from_rgb(84, 110, 122),        // Border
            },
            Theme::Custom => {
                // This should never be called since Custom uses custom_colors
                CustomThemeColors::default().to_theme_colors()
            }
        }
    }

    pub fn all_themes() -> Vec<Theme> {
        vec![
            Theme::CatppuccinMocha,
            Theme::CatppuccinLatte,
            Theme::GruvboxDark,
            Theme::GruvboxLight,
            Theme::DraculaDark,
            Theme::SolarizedDark,
            Theme::SolarizedLight,
            Theme::TokyoNight,
            Theme::OneDark,
            Theme::MaterialDark,
            Theme::Custom,
        ]
    }

    pub fn apply(&self, ctx: &egui::Context) {
        match self {
            Theme::CatppuccinMocha => apply_catppuccin_mocha(ctx),
            Theme::CatppuccinLatte => apply_catppuccin_latte(ctx),
            Theme::GruvboxDark => apply_gruvbox_dark(ctx),
            Theme::GruvboxLight => apply_gruvbox_light(ctx),
            Theme::DraculaDark => apply_dracula_dark(ctx),
            Theme::SolarizedDark => apply_solarized_dark(ctx),
            Theme::SolarizedLight => apply_solarized_light(ctx),
            Theme::TokyoNight => apply_tokyo_night(ctx),
            Theme::OneDark => apply_one_dark(ctx),
            Theme::MaterialDark => apply_material_dark(ctx),
            Theme::Custom => apply_catppuccin_mocha(ctx), // Default base for custom themes
        }
    }
}

pub struct CatppuccinMocha;

impl CatppuccinMocha {
    // Catppuccin Mocha color palette
    pub const BASE: Color32 = Color32::from_rgb(30, 30, 46);      // #1e1e2e
    pub const MANTLE: Color32 = Color32::from_rgb(24, 24, 37);    // #181825
    pub const CRUST: Color32 = Color32::from_rgb(17, 17, 27);     // #11111b
    pub const TEXT: Color32 = Color32::from_rgb(205, 214, 244);   // #cdd6f4
    pub const SUBTEXT1: Color32 = Color32::from_rgb(186, 194, 222); // #bac2de
    pub const SUBTEXT0: Color32 = Color32::from_rgb(166, 173, 200); // #a6adc8
    pub const OVERLAY2: Color32 = Color32::from_rgb(147, 153, 178); // #9399b2
    pub const OVERLAY1: Color32 = Color32::from_rgb(127, 132, 156); // #7f849c
    pub const OVERLAY0: Color32 = Color32::from_rgb(108, 112, 134); // #6c7086
    pub const SURFACE2: Color32 = Color32::from_rgb(88, 91, 112);   // #585b70
    pub const SURFACE1: Color32 = Color32::from_rgb(69, 71, 90);    // #45475a
    pub const SURFACE0: Color32 = Color32::from_rgb(49, 50, 68);    // #313244
    
    // Accent colors
    pub const LAVENDER: Color32 = Color32::from_rgb(180, 190, 254); // #b4befe
    pub const BLUE: Color32 = Color32::from_rgb(137, 180, 250);     // #89b4fa
    pub const SAPPHIRE: Color32 = Color32::from_rgb(116, 199, 236); // #74c7ec
    pub const SKY: Color32 = Color32::from_rgb(137, 220, 235);      // #89dceb
    pub const TEAL: Color32 = Color32::from_rgb(148, 226, 213);     // #94e2d5
    pub const GREEN: Color32 = Color32::from_rgb(166, 227, 161);    // #a6e3a1
    pub const YELLOW: Color32 = Color32::from_rgb(249, 226, 175);   // #f9e2af
    pub const PEACH: Color32 = Color32::from_rgb(250, 179, 135);    // #fab387
    pub const MAROON: Color32 = Color32::from_rgb(238, 153, 160);   // #ee99a0
    pub const RED: Color32 = Color32::from_rgb(243, 139, 168);      // #f38ba8
    pub const MAUVE: Color32 = Color32::from_rgb(203, 166, 247);    // #cba6f7
    pub const PINK: Color32 = Color32::from_rgb(245, 194, 231);     // #f5c2e7
    pub const FLAMINGO: Color32 = Color32::from_rgb(242, 205, 205); // #f2cdcd
    pub const ROSEWATER: Color32 = Color32::from_rgb(245, 224, 220); // #f5e0dc
}

pub fn apply_catppuccin_mocha(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    // Background colors
    visuals.window_fill = CatppuccinMocha::BASE;
    visuals.panel_fill = CatppuccinMocha::MANTLE;
    visuals.faint_bg_color = CatppuccinMocha::SURFACE0;
    visuals.extreme_bg_color = CatppuccinMocha::CRUST;
    
    // Text colors - Don't override text color to allow RichText colors to work
    visuals.warn_fg_color = CatppuccinMocha::YELLOW;
    
    // Widget colors
    visuals.widgets.noninteractive.bg_fill = CatppuccinMocha::SURFACE0;
    visuals.widgets.noninteractive.weak_bg_fill = CatppuccinMocha::SURFACE1;
    visuals.widgets.noninteractive.bg_stroke.color = CatppuccinMocha::OVERLAY0;
    visuals.widgets.noninteractive.fg_stroke.color = CatppuccinMocha::TEXT;
    
    visuals.widgets.inactive.bg_fill = CatppuccinMocha::SURFACE1;
    visuals.widgets.inactive.weak_bg_fill = CatppuccinMocha::SURFACE0;
    visuals.widgets.inactive.bg_stroke.color = CatppuccinMocha::OVERLAY1;
    visuals.widgets.inactive.fg_stroke.color = CatppuccinMocha::SUBTEXT1;
    
    visuals.widgets.hovered.bg_fill = CatppuccinMocha::SURFACE2;
    visuals.widgets.hovered.weak_bg_fill = CatppuccinMocha::SURFACE1;
    visuals.widgets.hovered.bg_stroke.color = CatppuccinMocha::OVERLAY2;
    visuals.widgets.hovered.fg_stroke.color = CatppuccinMocha::TEXT;
    
    visuals.widgets.active.bg_fill = CatppuccinMocha::BLUE;
    visuals.widgets.active.weak_bg_fill = CatppuccinMocha::SURFACE2;
    visuals.widgets.active.bg_stroke.color = CatppuccinMocha::BLUE;
    visuals.widgets.active.fg_stroke.color = CatppuccinMocha::BASE;
    
    visuals.widgets.open.bg_fill = CatppuccinMocha::SURFACE1;
    visuals.widgets.open.weak_bg_fill = CatppuccinMocha::SURFACE0;
    visuals.widgets.open.bg_stroke.color = CatppuccinMocha::OVERLAY2;
    visuals.widgets.open.fg_stroke.color = CatppuccinMocha::TEXT;
    
    // Selection colors
    visuals.selection.bg_fill = CatppuccinMocha::BLUE.linear_multiply(0.3);
    visuals.selection.stroke.color = CatppuccinMocha::BLUE;
    
    // Hyperlink color
    visuals.hyperlink_color = CatppuccinMocha::BLUE;
    
    // Error colors
    visuals.error_fg_color = CatppuccinMocha::RED;
    visuals.warn_fg_color = CatppuccinMocha::YELLOW;
    
    ctx.set_visuals(visuals);
}

// Catppuccin Latte (Light theme)
pub fn apply_catppuccin_latte(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    
    visuals.window_fill = Color32::from_rgb(239, 241, 245);      // #eff1f5
    visuals.panel_fill = Color32::from_rgb(230, 233, 239);       // #e6e9ef
    visuals.faint_bg_color = Color32::from_rgb(220, 224, 232);   // #dce0e8
    visuals.extreme_bg_color = Color32::from_rgb(204, 208, 218); // #ccd0da
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(30, 102, 245);   // #1e66f5
    visuals.hyperlink_color = Color32::from_rgb(30, 102, 245);
    
    ctx.set_visuals(visuals);
}

// Gruvbox Dark
pub fn apply_gruvbox_dark(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    visuals.window_fill = Color32::from_rgb(40, 40, 40);         // #282828
    visuals.panel_fill = Color32::from_rgb(50, 48, 47);          // #32302f
    visuals.faint_bg_color = Color32::from_rgb(60, 56, 54);      // #3c3836
    visuals.extreme_bg_color = Color32::from_rgb(29, 32, 33);    // #1d2021
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(254, 128, 25);     // #fe8019
    visuals.hyperlink_color = Color32::from_rgb(131, 165, 152);           // #83a598
    
    ctx.set_visuals(visuals);
}

// Gruvbox Light
pub fn apply_gruvbox_light(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    
    visuals.window_fill = Color32::from_rgb(251, 241, 199);      // #fbf1c7
    visuals.panel_fill = Color32::from_rgb(242, 229, 188);       // #f2e5bc
    visuals.faint_bg_color = Color32::from_rgb(235, 219, 178);   // #ebdbb2
    visuals.extreme_bg_color = Color32::from_rgb(213, 196, 161); // #d5c4a1
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(175, 58, 3);    // #af3a03
    visuals.hyperlink_color = Color32::from_rgb(69, 133, 136);         // #458588
    
    ctx.set_visuals(visuals);
}

// Dracula
pub fn apply_dracula_dark(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    visuals.window_fill = Color32::from_rgb(40, 42, 54);         // #282a36
    visuals.panel_fill = Color32::from_rgb(68, 71, 90);          // #44475a
    visuals.faint_bg_color = Color32::from_rgb(98, 114, 164);    // #6272a4
    visuals.extreme_bg_color = Color32::from_rgb(33, 34, 44);    // #21222c
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(189, 147, 249);    // #bd93f9
    visuals.hyperlink_color = Color32::from_rgb(139, 233, 253);           // #8be9fd
    
    ctx.set_visuals(visuals);
}

// Solarized Dark
pub fn apply_solarized_dark(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    visuals.window_fill = Color32::from_rgb(0, 43, 54);          // #002b36
    visuals.panel_fill = Color32::from_rgb(7, 54, 66);           // #073642
    visuals.faint_bg_color = Color32::from_rgb(88, 110, 117);    // #586e75
    visuals.extreme_bg_color = Color32::from_rgb(0, 30, 38);     // #001e26
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(38, 139, 210);     // #268bd2
    visuals.hyperlink_color = Color32::from_rgb(42, 161, 152);            // #2aa198
    
    ctx.set_visuals(visuals);
}

// Solarized Light
pub fn apply_solarized_light(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    
    visuals.window_fill = Color32::from_rgb(253, 246, 227);      // #fdf6e3
    visuals.panel_fill = Color32::from_rgb(238, 232, 213);       // #eee8d5
    visuals.faint_bg_color = Color32::from_rgb(147, 161, 161);   // #93a1a1
    visuals.extreme_bg_color = Color32::from_rgb(220, 215, 201); // #dcd7c9
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(38, 139, 210);     // #268bd2
    visuals.hyperlink_color = Color32::from_rgb(42, 161, 152);            // #2aa198
    
    ctx.set_visuals(visuals);
}

// Tokyo Night
pub fn apply_tokyo_night(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    visuals.window_fill = Color32::from_rgb(36, 40, 59);         // #24283b
    visuals.panel_fill = Color32::from_rgb(26, 27, 38);          // #1a1b26
    visuals.faint_bg_color = Color32::from_rgb(65, 72, 104);     // #414868
    visuals.extreme_bg_color = Color32::from_rgb(22, 22, 30);    // #16161e
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(125, 207, 255);    // #7dcfff
    visuals.hyperlink_color = Color32::from_rgb(187, 154, 247);           // #bb9af7
    
    ctx.set_visuals(visuals);
}

// One Dark
pub fn apply_one_dark(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    visuals.window_fill = Color32::from_rgb(40, 44, 52);         // #282c34
    visuals.panel_fill = Color32::from_rgb(33, 37, 43);          // #21252b
    visuals.faint_bg_color = Color32::from_rgb(60, 64, 73);      // #3c4049
    visuals.extreme_bg_color = Color32::from_rgb(28, 31, 36);    // #1c1f24
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(97, 175, 239);     // #61afef
    visuals.hyperlink_color = Color32::from_rgb(224, 108, 117);           // #e06c75
    
    ctx.set_visuals(visuals);
}

// Material Dark
pub fn apply_material_dark(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    visuals.window_fill = Color32::from_rgb(33, 33, 33);         // #212121
    visuals.panel_fill = Color32::from_rgb(48, 48, 48);          // #303030
    visuals.faint_bg_color = Color32::from_rgb(66, 66, 66);      // #424242
    visuals.extreme_bg_color = Color32::from_rgb(18, 18, 18);    // #121212
    
    // Don't override text color to allow RichText colors to work
    visuals.widgets.active.bg_fill = Color32::from_rgb(33, 150, 243);     // #2196f3
    visuals.hyperlink_color = Color32::from_rgb(3, 169, 244);             // #03a9f4
    
    ctx.set_visuals(visuals);
}

pub fn get_accent_color(index: usize) -> Color32 {
    let colors = [
        CatppuccinMocha::BLUE,
        CatppuccinMocha::MAUVE,
        CatppuccinMocha::GREEN,
        CatppuccinMocha::YELLOW,
        CatppuccinMocha::PEACH,
        CatppuccinMocha::PINK,
        CatppuccinMocha::TEAL,
        CatppuccinMocha::SKY,
    ];
    colors[index % colors.len()]
}
