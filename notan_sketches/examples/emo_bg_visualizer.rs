use notan::draw::*;
use notan::egui::{self, *};
use notan::extra::FpsLimit;
use notan::log;
use notan::math::{vec2, Vec2};
use notan::prelude::*;
use notan_sketches::emotion::*;
use notan_sketches::utils::{get_common_win_config, get_draw_setup, ScreenDimensions};
use palette::{FromColor, Hsl, Hsv, LinSrgb, Mix, RgbHue, Srgb};
use serde::{Deserialize, Serialize};
// use serde_json::{Result as JsonResult, Value};
use std::fs;
use FontFamily::{Monospace, Proportional};


// See details at https://stackoverflow.com/a/42764117
const EMOCAT_DOCS: [&'static str; 8] = [
    include_str!("assets/lb_bronte01.json"),
    include_str!("assets/lb_dickinson01.json"),
    // include_str!("assets/lb_dickinson02.json"),
    include_str!("assets/lb_howe01.json"),
    include_str!("assets/lb_hughes01.json"),
    include_str!("assets/lb_teasdale01.json"),
    include_str!("assets/wilde01.json"),
    include_str!("assets/lb_whitman01.json"),
    include_str!("assets/the_stagger.json"),
];


const CLEAR_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::BLACK;
const META_COLOR: Color = Color::GRAY;
const DYNAMIC_TEXT_COLOR: bool = false;
const STARTING_MIX_FACTOR: f32 = 0.0;
// const MIX_RATE: f32 = 0.001;
// const MIX_RATE: f32 = 0.0001;
const MIX_RATE: f32 = 0.00001;
// const MIX_RATE: f32 = 0.000001;
const COLOR_COMPARISON_PRECISION: f32 = 3.0;
const MAX_FPS: u8 = 240;


// #[derive(PartialEq)]
enum View {
    HOME,
    PUBLIC_DOMAIN_MENU,
    // MY_MENU,
    READ,
}


struct ReadingViewState {
    doc_index: usize,
    analysis: usize,
}

impl Default for ReadingViewState {
    fn default() -> Self {
        Self {
            doc_index: 0,
            analysis: 0,
        }
    }
}


#[derive(AppState)]
struct State {
    view: View,
    emodocs: Vec<EmocatOutputDoc>,
    reading: ReadingViewState,
    font: Font,
    title_font: Font,
    egui_fonts: FontDefinitions,
    egui_fonts_configured: bool,
    simple_color: Color,
    bg_color: Color,
    bg_color_mix_factor: f32,
    text_color: Color,
    dynamic_text_color: bool,
}

impl State {
    fn reset_colors(&mut self) {
        self.simple_color = CLEAR_COLOR;
        self.bg_color = CLEAR_COLOR;
        self.bg_color_mix_factor = STARTING_MIX_FACTOR;
        self.text_color = TITLE_COLOR;
        self.dynamic_text_color = DYNAMIC_TEXT_COLOR;
    }
}


fn configure_egui_fonts(title_font_bytes: &'static [u8]) -> FontDefinitions {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut egui_fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    let bytes = title_font_bytes.clone();
    egui_fonts
        .font_data
        .insert("my_font".to_owned(), egui::FontData::from_static(&bytes));

    // Put my font first (highest priority) for proportional text:
    egui_fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    egui_fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    egui_fonts
}


fn init(gfx: &mut Graphics, plugins: &mut Plugins) -> State {
    let font_bytes = include_bytes!(
        // "./assets/fonts/Ubuntu-B.ttf"
        // "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf"
        "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.spaced.ttf"
    );
    let font = gfx.create_font(font_bytes).unwrap();

    let title_font_bytes =
        include_bytes!("./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf");
    let title_font = gfx.create_font(title_font_bytes).unwrap();

    // Using the regular font for egui title menu items b/c I want the line spacing
    let egui_fonts = configure_egui_fonts(title_font_bytes);
    // let egui_fonts = configure_egui_fonts(font_bytes);


    let emodocs: Vec<EmocatOutputDoc> = EMOCAT_DOCS
        .iter()
        .map(|&doc| serde_json::from_str(doc).expect("Could not open emocat document"))
        .collect();


    let state = State {
        view: View::HOME,
        // view: View::READ,
        emodocs: emodocs,
        reading: ReadingViewState::default(),
        font: font,
        title_font: title_font,
        egui_fonts: egui_fonts,
        egui_fonts_configured: false,
        simple_color: CLEAR_COLOR,
        bg_color: CLEAR_COLOR,
        bg_color_mix_factor: STARTING_MIX_FACTOR,
        text_color: TITLE_COLOR,
        dynamic_text_color: DYNAMIC_TEXT_COLOR,
    };
    state
}


/// Scale the font according to the current work size. Quite simple right now,
/// probably lots of room for improving this.
///
/// These return values were decided by comparing sizes on my own setup. Needs testing
/// across devices.
///
/// @TODO: What about portrait dimensions?
fn scale_font(default_size: f32, work_size: Vec2) -> f32 {
    // if work_size.x >= ScreenDimensions::RES_1080P.x && work_size.x < ScreenDimensions::RES_1440P.x
    // {
    //     log::debug!("1080p, x:{} y:{}", work_size.x, work_size.y);
    //     return default_size * 2.2;
    // }
    if work_size.x >= ScreenDimensions::RES_1080P.x && work_size.x < ScreenDimensions::RES_HDPLUS.x
    {
        log::debug!("1080p, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 2.2;
    }
    if work_size.x >= ScreenDimensions::RES_HDPLUS.x && work_size.x < ScreenDimensions::RES_1440P.x
    {
        log::debug!("HDPLus, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 2.5;
    }
    if work_size.x >= ScreenDimensions::RES_1440P.x && work_size.x < ScreenDimensions::RES_4K.x {
        log::debug!("1440p, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 3.0;
    }
    if work_size.x >= ScreenDimensions::RES_4K.x {
        log::debug!("4k, x:{} y:{}", work_size.x, work_size.y);
        return default_size * 4.4;
    }
    log::debug!("Default, x:{} y:{}", work_size.x, work_size.y);
    return default_size;
}


/// In this application, where font scaling is involved, a work size that matches
/// the window size results in nicer looking fonts. This comes at the expense of
/// not being able to use literal values for sizing shapes and such (not being able
/// to work against a known scale). Instead, one can use fractions of the work size
/// values.
fn get_work_size(gfx: &Graphics) -> Vec2 {
    // If we don't guard against a minimum like this, the app crashes if the window
    // is shrunk to a small size.
    if gfx.device.size().0 as f32 > ScreenDimensions::MINIMUM.x {
        return vec2(gfx.device.size().0 as f32, gfx.device.size().1 as f32);
    }
    ScreenDimensions::MINIMUM
}


/// Return black or white depending on the current background color
///
/// Based on this algorithm:
/// https://stackoverflow.com/a/1855903/4655636
///
fn get_text_color(state: &State) -> Color {
    let luminance: f32;
    if state.dynamic_text_color {
        luminance =
            0.299 * state.bg_color.r + 0.587 * state.bg_color.g + 0.114 * state.bg_color.b / 255.0;
    } else {
        luminance = 0.299 * state.simple_color.r
            + 0.587 * state.simple_color.g
            + 0.114 * state.simple_color.b / 255.0;
    }

    // log::debug!("Luminance {}", luminance);
    if luminance < 0.5 {
        return Color::WHITE;
    }
    Color::BLACK
}

fn round(val: f32, digits: f32) -> f32 {
    // log::debug!("{}, {}", val, (val * 100.0).round() / 100.0);
    // (val * 100.0).round() / 100.0

    let mut multiplier: f32 = 10.0;
    multiplier = multiplier.powf(digits);
    // log::debug!("{}, {}", val, (val * multiplier).round() / multiplier);
    (val * multiplier).round() / multiplier
}


fn update_bg_color(app: &App, state: &mut State) {
    // The mix function used to blend colors below doesn't always end up with the
    // exact floating point numbers of the end color, so comparing with rounded
    // color values instead of comparing the colors directly.
    let precision = COLOR_COMPARISON_PRECISION;
    // log::debug!(
    //     "{}::{}, {}::{}, {}::{}",
    //     round(state.bg_color.r, precision),
    //     round(state.simple_color.r, precision),
    //     round(state.bg_color.g, precision),
    //     round(state.simple_color.g, precision),
    //     round(state.bg_color.b, precision),
    //     round(state.simple_color.b, precision),
    // );
    if round(state.bg_color.r, precision) != round(state.simple_color.r, precision)
        || round(state.bg_color.g, precision) != round(state.simple_color.g, precision)
        || round(state.bg_color.b, precision) != round(state.simple_color.b, precision)
    {
        // log::debug!("Mix factor: {}", state.bg_color_mix_factor);
        let bg_color = Srgb::new(state.bg_color.r, state.bg_color.g, state.bg_color.b);
        let simple_color = Srgb::new(
            state.simple_color.r,
            state.simple_color.g,
            state.simple_color.b,
        );
        let mut bg_color = LinSrgb::from_color(bg_color);
        let simple_color = LinSrgb::from_color(simple_color);
        bg_color = bg_color.mix(&simple_color, state.bg_color_mix_factor);
        let bg_color = Srgb::from_color(bg_color);
        state.bg_color = Color::from_rgb(bg_color.red, bg_color.green, bg_color.blue);
        state.bg_color_mix_factor += MIX_RATE;
    } else {
        state.bg_color_mix_factor = STARTING_MIX_FACTOR;
    }
}


fn update_bg_color_simple(state: &mut State) {
    state.bg_color = state.simple_color.clone();
}


fn update_read_view(app: &mut App, state: &mut State) {
    let emodoc = &state.emodocs[state.reading.doc_index];

    if app.keyboard.was_pressed(KeyCode::Home) {
        log::debug!("home");
        state.reading.analysis = 0;
        state.simple_color = CLEAR_COLOR;
    }

    if app.keyboard.was_pressed(KeyCode::End) {
        log::debug!("end");
        state.reading.analysis = emodoc.analyses.len() - 1;
        state.simple_color = get_simple_color(&emodoc.analyses[state.reading.analysis - 1]);
    }


    if app.keyboard.was_pressed(KeyCode::Left) && state.reading.analysis > 0 {
        log::debug!("left");
        state.reading.analysis -= 1;
        if state.reading.analysis > 0 {
            state.simple_color = get_simple_color(&emodoc.analyses[state.reading.analysis - 1]);
        } else {
            state.simple_color = CLEAR_COLOR;
        }
    }

    if app.keyboard.was_pressed(KeyCode::Right) && state.reading.analysis < emodoc.analyses.len() {
        log::debug!("right");
        state.reading.analysis += 1;
        state.simple_color = get_simple_color(&emodoc.analyses[state.reading.analysis - 1]);
    }
    // update_bg_color_simple(state);
    update_bg_color(app, state);
    state.text_color = get_text_color(&state);
}


fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::M) {
        log::debug!("m");
        state.view = View::HOME;
        state.reading = ReadingViewState::default();
        state.reset_colors();
    }


    match state.view {
        View::READ => update_read_view(app, state),
        _ => (),
    }
}


fn draw_title(draw: &mut Draw, state: &State, work_size: Vec2) {
    let emodoc = &state.emodocs[state.reading.doc_index];
    let mut textbox_width = work_size.x * 0.75;

    draw.text(&state.title_font, &emodoc.title)
        .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
        .color(TITLE_COLOR)
        .size(scale_font(60.0, work_size))
        .max_width(textbox_width)
        .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.4)
        .h_align_left()
        .v_align_middle();

    // draw.text(&state.font, &state.emodoc.title)
    //     .alpha_mode(BlendMode::OVER) // Fixes some artifacting -- gonna be default in future Notan
    //     .color(Color::RED)
    //     .size(scale_font(60.0, work_size))
    //     .max_width(textbox_width)
    //     .position(
    //         work_size.x * 0.5 - textbox_width * 0.5 + 1.0,
    //         work_size.y * 0.4 - 1.0,
    //     )
    //     .h_align_left()
    //     .v_align_middle();


    let title_bounds = draw.last_text_bounds();

    textbox_width = textbox_width * 0.9;
    draw.text(&state.title_font, &format!("by {}", emodoc.author))
        .alpha_mode(BlendMode::OVER)
        .color(META_COLOR)
        .size(scale_font(30.0, work_size))
        .max_width(textbox_width)
        .position(
            work_size.x * 0.5 - textbox_width * 0.5,
            title_bounds.y + title_bounds.height + work_size.y * 0.1,
        )
        .h_align_left()
        .v_align_middle();
}


fn draw_paragraph(draw: &mut Draw, state: &State, work_size: Vec2) {
    let emodoc = &state.emodocs[state.reading.doc_index];
    let textbox_width = work_size.x * 0.75;

    draw.text(
        &state.font,
        &emodoc.analyses[state.reading.analysis - 1].text,
    )
    .alpha_mode(BlendMode::OVER)
    .color(state.text_color)
    .size(scale_font(32.0, work_size))
    .max_width(textbox_width)
    .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.5)
    .v_align_middle()
    // .position(work_size.x * 0.5 - textbox_width * 0.5, work_size.y * 0.3)
    // .v_align_top()
    .h_align_left();

    // let title_bounds = draw.last_text_bounds();
}


// fn draw_read_view(draw: &mut Draw, state: &State, work_size: Vec2) {
fn draw_read_view(gfx: &mut Graphics, state: &State, work_size: Vec2) {
    let mut draw = get_draw_setup(gfx, work_size, true, state.bg_color);

    if state.reading.analysis == 0 {
        draw_title(&mut draw, state, work_size);
    } else {
        draw_paragraph(&mut draw, state, work_size);
    }

    // draw to screen
    gfx.render(&draw);
}


#[inline]
fn title_button() -> TextStyle {
    TextStyle::Name("TitleButton".into())
}

#[inline]
fn author_menu_text() -> TextStyle {
    TextStyle::Name("AuthorMenuText".into())
}

///
/// Based on: https://github.com/emilk/egui/blob/master/examples/custom_font_style/src/main.rs
///
fn configure_text_styles(ctx: &egui::Context, work_size: Vec2) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(scale_font(25.0, work_size), Monospace),
        ),
        (
            TextStyle::Body,
            FontId::new(scale_font(16.0, work_size), Monospace),
        ),
        (
            author_menu_text(),
            FontId::new(scale_font(10.0, work_size), Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(scale_font(12.0, work_size), Monospace),
        ),
        (
            title_button(),
            // FontId::new(scale_font(22.0, work_size), Proportional),
            FontId::new(scale_font(16.0, work_size), Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(scale_font(8.0, work_size), Monospace),
        ),
    ]
    .into();
    ctx.set_style(style);
}


///
/// Based on: https://github.com/emilk/egui/blob/master/examples/custom_font/src/main.rs
///
fn configure_custom_fonts(ctx: &egui::Context, state: &mut State) {
    // Kind of a hack right now because I don't know a better way to avoid setting up
    // font on every draw(). Still have to clone the whole fonts setup here, but at
    // least we only do it once by setting the egui_fonts_configured flag.

    // Tell egui to use these fonts:
    if !state.egui_fonts_configured {
        ctx.set_fonts(state.egui_fonts.clone());
        state.egui_fonts_configured = true;
    }
}

fn ui_common_setup(ctx: &Context, state: &mut State, work_size: Vec2) -> egui::Color32 {
    // Switch to light mode
    ctx.set_visuals(egui::Visuals::light());
    configure_custom_fonts(ctx, state);
    configure_text_styles(ctx, work_size);

    let clear_color_u8 = CLEAR_COLOR.rgba_u8();
    egui::Color32::from_rgb(clear_color_u8[0], clear_color_u8[1], clear_color_u8[2])
}

fn draw_public_domain_menu_items(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    state: &mut State,
    work_size: Vec2,
) {
    let mut style = (*ctx.style()).clone();
    let button_top_margin = work_size.y * 0.01;
    let title_frame = egui::Frame::none()
        // .fill(egui::Color32::RED)
        .inner_margin(egui::style::Margin {
            left: 0.0,
            right: 0.0,
            top: button_top_margin,
            bottom: button_top_margin,
        });
    let button_padding_x = work_size.x * 0.005;
    let button_padding_y = work_size.y * 0.005;
    style.spacing.button_padding = egui::Vec2::new(button_padding_x, button_padding_y);
    ctx.set_style(style);


    for (doc_index, emodoc) in state.emodocs.iter().enumerate() {
        // ui.heading(&emodoc.title);
        let title_text = RichText::new(&emodoc.title)
            .color(egui::Color32::WHITE)
            .text_style(title_button());
        // if ui.add(egui::Button::new(&emodoc.title)).clicked() {
        let title_button = egui::Button::new(title_text)
            .wrap(true)
            .fill(egui::Color32::GRAY);
        title_frame.show(ui, |ui| {
            if ui.add(title_button).clicked() {
                state.reading.doc_index = doc_index;
                state.view = View::READ;
                log::debug!("{}", &emodoc.title);
            }
            let author_text = RichText::new(&emodoc.author).text_style(author_menu_text());
            ui.label(author_text);
        });
        // ui.separator();
    }
}

fn draw_home_view(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State, work_size: Vec2) {
    let mut output = plugins.egui(|ctx| {
        let ui_fill = ui_common_setup(ctx, state, work_size);

        let panel_inner_margin = work_size.y * 0.02;
        let panel_frame = egui::Frame::none()
            // .fill(ui_fill)
            .inner_margin(egui::style::Margin {
                left: panel_inner_margin,
                right: panel_inner_margin,
                // left: 200.0,
                // right: 200.0,
                top: panel_inner_margin,
                bottom: panel_inner_margin,
            });
        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let heading_frame_margin = work_size.y * 0.02;
                    let heading_frame =
                        egui::Frame::none()
                            .fill(ui_fill)
                            .inner_margin(egui::style::Margin {
                                left: 0.0,
                                right: 0.0,
                                top: 0.0,
                                bottom: heading_frame_margin,
                            });
                    heading_frame.show(ui, |ui| {
                        ui.heading("emo bg visualizer");
                        ui.small("A background visualizer for emotions found in text");
                    });
                    let mut navi_frame = heading_frame.clone();
                    // navi_frame.inner_margin.bottom *= 2.0;
                    navi_frame.show(ui, |ui| {
                        ui.small("Settings |  About");
                    });
                    heading_frame.show(ui, |ui| {
                        ui.label("Read select poems and prose from the public domain:");
                    });

                    egui::ScrollArea::vertical()
                        // .max_width(500.0)
                        .show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                draw_public_domain_menu_items(ctx, ui, state, work_size);
                            });
                        });
                });
            });
    });

    output.clear_color(CLEAR_COLOR);
    if output.needs_repaint() {
        gfx.render(&output);
    }
}


fn draw_public_domain_menu(
    gfx: &mut Graphics,
    plugins: &mut Plugins,
    state: &mut State,
    work_size: Vec2,
) {
    let menu_width = work_size.x * 0.75;

    let mut output = plugins.egui(|ctx| {
        let ui_fill = ui_common_setup(ctx, state, work_size);

        let menu_inner_margin = work_size.y * 0.1;
        let menu_frame = egui::Frame::none()
            .fill(ui_fill)
            .inner_margin(egui::style::Margin {
                left: menu_inner_margin,
                right: menu_inner_margin,
                top: menu_inner_margin,
                bottom: menu_inner_margin,
            });

        let button_top_margin = work_size.y * 0.01;
        let title_frame = egui::Frame::none()
            // .fill(egui::Color32::RED)
            .inner_margin(egui::style::Margin {
                left: 0.0,
                right: 0.0,
                top: button_top_margin,
                bottom: button_top_margin,
            });
        egui::SidePanel::right("Menu")
            .resizable(false)
            .default_width(menu_width)
            // Should experiment more with how these min/max settings behave when shrinking
            // the app window
            // .max_width(menu_width)
            .min_width(menu_width)
            .frame(menu_frame)
            .show(&ctx, |ui| {
                let mut style = (*ctx.style()).clone();
                let button_padding_x = work_size.x * 0.005;
                let button_padding_y = work_size.y * 0.005;
                style.spacing.button_padding = egui::Vec2::new(button_padding_x, button_padding_y);
                ctx.set_style(style);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        for (doc_index, emodoc) in state.emodocs.iter().enumerate() {
                            // ui.heading(&emodoc.title);
                            let title_text = RichText::new(&emodoc.title)
                                .color(egui::Color32::WHITE)
                                .text_style(title_button());
                            // if ui.add(egui::Button::new(&emodoc.title)).clicked() {
                            let title_button = egui::Button::new(title_text)
                                .wrap(true)
                                .fill(egui::Color32::GRAY);
                            title_frame.show(ui, |ui| {
                                if ui.add(title_button).clicked() {
                                    state.reading.doc_index = doc_index;
                                    state.view = View::READ;
                                    log::debug!("{}", &emodoc.title);
                                }
                                let author_text =
                                    RichText::new(&emodoc.author).text_style(author_menu_text());
                                ui.label(author_text);
                            });
                            // ui.separator();
                        }
                    });
                });
            });
    });

    output.clear_color(CLEAR_COLOR);
    if output.needs_repaint() {
        gfx.render(&output);
    }
}


fn draw(
    // app: &mut App,
    gfx: &mut Graphics,
    plugins: &mut Plugins,
    state: &mut State,
) {
    let work_size = get_work_size(gfx);

    match state.view {
        View::READ => draw_read_view(gfx, state, work_size),
        View::PUBLIC_DOMAIN_MENU => draw_public_domain_menu(gfx, plugins, state, work_size),
        _ => draw_home_view(gfx, plugins, state, work_size),
        // _ => draw_public_domain_menu(gfx, plugins, state, work_size),
    }

    // log::debug!("fps: {}", app.timer.fps().round());
}


#[notan_main]
fn main() -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    // let win_config = get_common_win_config().high_dpi(true).vsync(true).size(
    let win_config = get_common_win_config().high_dpi(true).size(
        ScreenDimensions::RES_1080P.x as i32,
        ScreenDimensions::RES_1080P.y as i32,
        // ScreenDimensions::DEFAULT.x as i32,
        // ScreenDimensions::DEFAULT.y as i32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);


    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(EguiConfig)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .add_plugin(FpsLimit::new(MAX_FPS))
        .draw(draw)
        .update(update)
        .build()
}
