use egui::{Color32, Visuals};

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

pub fn apply_catppuccin_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    // Background colors
    visuals.window_fill = CatppuccinMocha::BASE;
    visuals.panel_fill = CatppuccinMocha::MANTLE;
    visuals.faint_bg_color = CatppuccinMocha::SURFACE0;
    visuals.extreme_bg_color = CatppuccinMocha::CRUST;
    
    // Text colors
    visuals.override_text_color = Some(CatppuccinMocha::TEXT);
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
