use notan::app::Event;
use notan::draw::*;
use notan::egui::{self, *};
use notan::extra::FpsLimit;
use notan::log;
use notan::math::Vec2;
use notan::prelude::*;
use notan_sketches::emotion::*;
use notan_sketches::utils::{
    get_common_win_config, get_draw_setup, scale_font, set_html_bgcolor, CommonHelpModal,
    ScreenDimensions,
};
use notan_touchy::{TouchGesture, TouchState};
// use serde_json::{Result as JsonResult, Value};
use notan_sketches::emotion_bg_visualizer::visualizers::color_transition::ColorTransitionVisualizer;
use notan_sketches::emotion_bg_visualizer::visualizers::tile::TilesVisualizer;
use notan_sketches::emotion_bg_visualizer::visualizers::VisualizerSelection;
use notan_sketches::emotion_bg_visualizer::{get_work_size, EmoVisualizerFull};
use FontFamily::{Monospace, Proportional};


// See details at https://stackoverflow.com/a/42764117
const EMOCAT_DOCS: [&'static str; 7] = [
    include_str!("assets/lb_bronte01.json"),
    include_str!("assets/lb_dickinson01.json"),
    // include_str!("assets/lb_dickinson02.json"),
    include_str!("assets/lb_howe01.json"),
    include_str!("assets/lb_hughes01.json"),
    include_str!("assets/lb_teasdale01.json"),
    include_str!("assets/wilde01.json"),
    include_str!("assets/lb_whitman01.json"),
    // include_str!("assets/the_stagger.json"),
];
const CLEAR_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::BLACK;
// const HELP_MODAL_TXTCOLOR: Color = Color::BLACK;
const HELP_MODAL_TXTCOLOR: Color = Color::GRAY;
const HELP_MODAL_BGCOLOR: Color = Color::from_rgb(0.95, 0.95, 0.9);
const ANALYSIS_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(128, 97, 0, 128);
const READ_VIEW_GUI_COLOR: egui::Color32 =
    egui::Color32::from_rgba_premultiplied(128, 128, 128, 128);
const DYNAMIC_TEXT_COLOR: bool = false;
const MAX_FPS: u8 = 240;
// const DEFAULT_VISUALIZER: VisualizerSelection = VisualizerSelection::ColorTransition;
const DEFAULT_VISUALIZER: VisualizerSelection = VisualizerSelection::Tiles;


#[derive(PartialEq)]
enum View {
    HOME,
    ABOUT,
    READ,
    SETTINGS,
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
    show_analysis: bool,
    emodocs: Vec<EmocatOutputDoc>,
    reading: ReadingViewState,
    font: Font,
    title_font: Font,
    egui_fonts: FontDefinitions,
    selected_visualizer: VisualizerSelection,
    visualizer: Box<dyn EmoVisualizerFull>,
    needs_handle_resize: bool,
    needs_egui_font_setup: bool,
    touch: TouchState,
    help_modal: CommonHelpModal,
    tile_texture: Texture,
}

impl State {
    fn goto_home_view(&mut self) {
        self.view = View::HOME;
        self.reading = ReadingViewState::default();
        self.visualizer
            .reset(CLEAR_COLOR, TITLE_COLOR, DYNAMIC_TEXT_COLOR);
    }

    fn goto_read_home(&mut self) {
        self.reading.analysis = 0;
        self.visualizer
            .gracefully_reset(CLEAR_COLOR, TITLE_COLOR, DYNAMIC_TEXT_COLOR);
    }

    fn goto_read_end(&mut self) {
        let emodoc = &self.emodocs[self.reading.doc_index];
        self.reading.analysis = emodoc.analyses.len();
        self.visualizer
            .update_model(&emodoc.analyses[self.reading.analysis - 1]);
    }

    fn goto_read_next(&mut self) {
        let emodoc = &self.emodocs[self.reading.doc_index];
        if self.reading.analysis < emodoc.analyses.len() {
            self.reading.analysis += 1;
            self.visualizer
                .update_model(&emodoc.analyses[self.reading.analysis - 1]);
        }
    }

    fn goto_read_prev(&mut self) {
        if self.reading.analysis > 0 {
            let emodoc = &self.emodocs[self.reading.doc_index];
            self.reading.analysis -= 1;
            if self.reading.analysis > 0 {
                self.visualizer
                    .update_model(&emodoc.analyses[self.reading.analysis - 1]);
            } else {
                self.visualizer
                    .gracefully_reset(CLEAR_COLOR, TITLE_COLOR, DYNAMIC_TEXT_COLOR);
            }
        }
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


fn init(gfx: &mut Graphics) -> State {
    let font_bytes = include_bytes!(
        // "./assets/fonts/Ubuntu-B.ttf"
        // "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf"
        "./assets/fonts/libre_baskerville/LibreBaskerville-Regular.spaced.ttf"
    );
    let font = gfx.create_font(font_bytes).unwrap();

    let title_font_bytes =
        include_bytes!("./assets/fonts/libre_baskerville/LibreBaskerville-Regular.ttf");
    let title_font = gfx.create_font(title_font_bytes).unwrap();

    let egui_fonts = configure_egui_fonts(title_font_bytes);


    let emodocs: Vec<EmocatOutputDoc> = EMOCAT_DOCS
        .iter()
        .map(|&doc| serde_json::from_str(doc).expect("Could not open emocat document"))
        .collect();

    let help_text = concat!(
        // "Use \u{00AB} left or right \u{00BB} arrow keys to read poem\n\n",
        "Use left or right arrow keys to read through the piece\n\n",
        "Press 'a' to toggle Analysis panel\n\n",
        "Press 'm' to return to poem listing\n\n",
        "Click mouse to close this help",
    );

    let touch_help_text = concat!(
        "Swipe left or right to read through the piece\n\n",
        "Swipe down to toggle Analysis panel\n\n",
        "Swipe up to return to poem listing\n\n",
        "Tap to close this help",
    );

    let tile_texture = gfx
        .create_texture()
        .from_image(include_bytes!("../examples/assets/tiles/tile3_4k.png"))
        .build()
        .unwrap();


    let state = State {
        view: View::HOME,
        // view: View::READ,
        show_analysis: false,
        emodocs,
        reading: ReadingViewState::default(),
        font,
        title_font,
        egui_fonts,
        selected_visualizer: DEFAULT_VISUALIZER,
        visualizer: match DEFAULT_VISUALIZER {
            // "TiledShaderVisualizer" => Box::new(TilesVisualizer::new(
            //     gfx,
            //     CLEAR_COLOR,
            //     TITLE_COLOR,
            //     DYNAMIC_TEXT_COLOR,
            // )),
            VisualizerSelection::Tiles => Box::new(TilesVisualizer::new(
                CLEAR_COLOR,
                TITLE_COLOR,
                DYNAMIC_TEXT_COLOR,
                tile_texture.clone(),
            )),
            _ => Box::new(ColorTransitionVisualizer::new(
                CLEAR_COLOR,
                TITLE_COLOR,
                DYNAMIC_TEXT_COLOR,
            )),
        },
        needs_handle_resize: true,
        needs_egui_font_setup: true,
        touch: TouchState::default(),
        help_modal: CommonHelpModal::new(
            gfx,
            help_text.to_string(),
            touch_help_text.to_string(),
            None,
        ),
        tile_texture,
    };
    state
}


fn update_read_view(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Home) {
        log::debug!("home");
        state.goto_read_home();
    }

    if app.keyboard.was_pressed(KeyCode::End) {
        log::debug!("end");
        state.goto_read_end();
    }

    if app.keyboard.was_pressed(KeyCode::Left) {
        log::debug!("left");
        state.goto_read_prev();
    }

    if app.keyboard.was_pressed(KeyCode::Right) {
        log::debug!("right");
        state.goto_read_next();
    }

    if app.keyboard.was_pressed(KeyCode::A) {
        log::debug!("a");
        state.show_analysis = !state.show_analysis;
    }

    state.visualizer.update_visualization();
}


fn handle_read_view_touch_events(app: &mut App, state: &mut State, evt: Event) {
    let gesture = state.touch.get_gesture(&app.timer.time_since_init(), &evt);

    if gesture.is_some() {
        // if !state.help_modal.handle_first_touch_with_help() {
        match gesture {
            Some(TouchGesture::SwipeLeft) => state.goto_read_next(),
            Some(TouchGesture::SwipeRight) => state.goto_read_prev(),
            Some(TouchGesture::SwipeUp) => state.goto_home_view(),
            Some(TouchGesture::SwipeDown) => state.show_analysis = !state.show_analysis,
            Some(TouchGesture::Tap) => state.help_modal.toggle_touch_help(),
            _ => {}
        }
        // }
    } else if state.touch.touch_interface_detected == false {
        match evt {
            Event::MouseUp { .. } => {
                log::debug!("mouse up in read_view_events!");
                // if state.events_focus.has_focus() {
                state.help_modal.handle_mouse_up()
                // }
            }
            _ => {}
        }
    }
}


fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::M) {
        log::debug!("m");
        state.goto_home_view();
    }

    match state.view {
        View::READ => update_read_view(app, state),
        _ => (),
    }
}


fn draw_title(draw: &mut Draw, state: &mut State, work_size: Vec2) {
    let emodoc = &state.emodocs[state.reading.doc_index];
    state.visualizer.draw_title(
        draw,
        &state.title_font,
        &emodoc.title,
        &emodoc.author,
        work_size,
    );
}


fn draw_paragraph(draw: &mut Draw, state: &mut State, work_size: Vec2) {
    let emodoc = &state.emodocs[state.reading.doc_index];
    state.visualizer.draw_paragraph(
        draw,
        &state.font,
        &emodoc.analyses[state.reading.analysis - 1].text,
        work_size,
    );
}

fn draw_read_help(draw: &mut Draw, state: &mut State, work_size: Vec2) {
    // Using a custom help popup instead of state.help_modal.draw()
    if state.help_modal.show_help || state.help_modal.show_touch_help {
        let mut help_txt = &state.help_modal.help_text;
        if state.help_modal.show_touch_help {
            help_txt = &state.help_modal.touch_help_text;
        }
        state.visualizer.draw_read_help(
            draw,
            &state.font,
            help_txt,
            work_size,
            HELP_MODAL_TXTCOLOR,
            HELP_MODAL_BGCOLOR,
        );
    }
}


fn draw_read_view(
    app: &mut App,
    gfx: &mut Graphics,
    plugins: &mut Plugins,
    state: &mut State,
    work_size: Vec2,
) {
    let draw = &mut get_draw_setup(gfx, work_size, true, CLEAR_COLOR);

    // NOTE: If the egui ui seems to be "blocking" the draw, it may be because the visualizer
    // draw() method is not calling `draw.clear()`. If this isn't done, the egui background
    // will block the draw. For an example, see impl method of ColorTransitionVisualizer::draw().
    state.visualizer.draw(app, gfx, draw);

    if state.reading.analysis == 0 {
        draw_title(draw, state, work_size);
    } else {
        draw_paragraph(draw, state, work_size);
    }
    draw_read_help(draw, state, work_size);
    gfx.render(draw);

    let output = plugins.egui(|ctx| {
        draw_analysis_panel(ctx, state, work_size);
    });
    // Not checking if output needs repaint because then the analysis panel
    // is erased by the draw above
    gfx.render(&output);
}

#[inline]
fn logo() -> TextStyle {
    TextStyle::Name("Logo".into())
}


#[inline]
fn analysis_panel_title() -> TextStyle {
    TextStyle::Name("AnalysisPanelTitle".into())
}

#[inline]
fn title_button() -> TextStyle {
    TextStyle::Name("TitleButton".into())
}

#[inline]
fn small_button() -> TextStyle {
    TextStyle::Name("SmallButton".into())
}

#[inline]
fn author_menu_text() -> TextStyle {
    TextStyle::Name("AuthorMenuText".into())
}


// Based on: https://github.com/emilk/egui/blob/master/examples/custom_font_style/src/main.rs
//
// NOTE: These font sizes and styles only affect egui UI items -- they don't apply to
// the draw.text() items used in the reading view.
fn configure_text_styles(ctx: &egui::Context, work_size: Vec2) {
    let portrait = work_size.x < work_size.y;
    // log::debug!("{} < {} = {}", work_size.x, work_size.y, portrait);
    let small_button_size: f32;
    let small_text_size: f32;
    if portrait {
        small_button_size = 12.0;
        small_text_size = 10.0;
    } else {
        small_button_size = 8.0;
        small_text_size = 8.0;
    }

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(scale_font(14.0, work_size), Monospace),
        ),
        (logo(), FontId::new(scale_font(26.0, work_size), Monospace)),
        (
            analysis_panel_title(),
            FontId::new(scale_font(10.0, work_size), Monospace),
        ),
        (
            TextStyle::Body,
            FontId::new(scale_font(12.0, work_size), Monospace),
        ),
        (
            author_menu_text(),
            FontId::new(scale_font(10.0, work_size), Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(scale_font(10.0, work_size), Monospace),
        ),
        (
            title_button(),
            FontId::new(scale_font(15.0, work_size), Proportional),
        ),
        (
            small_button(),
            FontId::new(scale_font(small_button_size, work_size), Monospace),
        ),
        (
            TextStyle::Small,
            FontId::new(scale_font(small_text_size, work_size), Monospace),
        ),
    ]
    .into();
    ctx.set_style(style);
    // Must request a repaint after changing styles, otherwise the display will go awry
    // if the draw method is checking egui Output's `needs_repaint()`, especially
    // for wasm target
    ctx.request_repaint();
}


// Based on: https://github.com/emilk/egui/blob/master/examples/custom_font/src/main.rs
fn configure_custom_fonts(ctx: &egui::Context, state: &mut State) {
    // Kind of a hack right now because I don't know a better way to avoid setting up
    // font on every draw(). Still have to clone the whole fonts setup here, but at
    // least we only do it once by setting the egui_fonts_configured flag.

    // Tell egui to use these fonts:
    ctx.set_fonts(state.egui_fonts.clone());
}


// Based on https://github.com/Nazariglez/notan/blob/main/examples/input_mouse_events.rs
fn event(app: &mut App, state: &mut State, evt: Event) {
    match evt {
        Event::WindowResize { width, height } => {
            // state.text = "resize...".to_string();
            log::debug!("Window resized to: w {}, h {}", width, height);
            state.needs_handle_resize = true
        }
        _ => {}
    }

    match state.view {
        View::READ => handle_read_view_touch_events(app, state, evt),
        _ => (),
    }
}


fn ui_common_setup(ctx: &Context, state: &mut State, work_size: Vec2) -> egui::Color32 {
    // Switch to light mode
    ctx.set_visuals(egui::Visuals::light());

    // Custom font setup whenever resized:
    if state.needs_egui_font_setup {
        configure_custom_fonts(ctx, state);
        state.needs_egui_font_setup = false;
    }
    // Reconfigure styles if window resized
    if state.needs_handle_resize {
        configure_text_styles(ctx, work_size);
        state.needs_handle_resize = false;
    }
    let clear_color_u8 = CLEAR_COLOR.rgba_u8();
    egui::Color32::from_rgb(clear_color_u8[0], clear_color_u8[1], clear_color_u8[2])
}


fn draw_main_nav(ui: &mut Ui, state: &mut State) {
    fn _make_small_button(
        text: &str,
        fill: egui::Color32,
        text_color: egui::Color32,
    ) -> egui::Button {
        let richtext = RichText::new(text)
            .color(text_color)
            .text_style(small_button());
        egui::Button::new(richtext).wrap(true).fill(fill)
    }

    fn make_small_button(text: &str) -> egui::Button {
        _make_small_button(text, egui::Color32::GRAY, egui::Color32::WHITE)
    }

    fn make_small_analysis_button(text: &str) -> egui::Button {
        _make_small_button(text, ANALYSIS_COLOR, egui::Color32::BLACK)
    }


    ui.horizontal(|ui| {
        if state.view != View::HOME {
            let about_button = make_small_button("Main Menu");
            if ui.add(about_button).clicked() {
                state.view = View::HOME;
            }
        }

        if state.view != View::ABOUT {
            let about_button = make_small_button("About");
            if ui.add(about_button).clicked() {
                state.view = View::ABOUT;
            }
        }

        if state.view != View::SETTINGS {
            let settings_button = make_small_button("Options");
            if ui.add(settings_button).clicked() {
                log::debug!("clicked settings");
                state.view = View::SETTINGS;
            }
        }

        let button: Button;
        if state.show_analysis {
            button = make_small_analysis_button("Close Analysis");
        } else {
            button = make_small_button("Show Analysis");
        }
        if ui.add(button).clicked() {
            log::debug!("clicked settings");
            state.show_analysis = !state.show_analysis;
        }
    });
}


fn draw_analysis_panel(ctx: &egui::Context, state: &mut State, work_size: Vec2) {
    if state.show_analysis {
        let title_text = RichText::new("Analysis Details")
            .color(egui::Color32::BLACK)
            .text_style(analysis_panel_title());

        let panel_color = READ_VIEW_GUI_COLOR;
        let panel_inner_margin_x = work_size.x * 0.02;
        let panel_inner_margin_y = work_size.y * 0.02;
        egui::Window::new(title_text)
            // .default_pos(egui::Pos2::new(work_size.x, work_size.y))
            .default_pos(egui::Pos2::new(work_size.x, 0.0))
            .default_width(work_size.x * 0.2)
            .collapsible(false)
            .frame(egui::Frame::none().fill(panel_color).inner_margin(
                egui::style::Margin::symmetric(panel_inner_margin_x, panel_inner_margin_y),
            ))
            .show(ctx, |ui| {
                state.visualizer.egui_metrics(ui, &analysis_panel_title);
            });
    }
}


// First time creating a fn that uses a a closure. Useful resources around closures
// and passing them as fn params:
//
// https://www.programiz.com/rust/closure
// https://doc.rust-lang.org/rust-by-example/fn/closures.html
// https://doc.rust-lang.org/rust-by-example/fn/closures.html
fn draw_with_main_panel<F>(
    ctx: &egui::Context,
    state: &mut State,
    work_size: Vec2,
    ui_fill: Color32,
    // view_fn: &dyn Fn(&egui::Context, &mut Ui, &mut State, Vec2),
    view_fn: F,
) where
    F: Fn(&egui::Context, &mut Ui, &mut State, Vec2),
{
    let panel_inner_margin_w;
    if work_size.x > work_size.y {
        panel_inner_margin_w = work_size.y * 0.1;
    } else {
        panel_inner_margin_w = work_size.y * 0.02;
    }
    let panel_inner_margin_h = work_size.y * 0.02;
    let panel_frame = egui::Frame::none()
        // .fill(ui_fill)
        .inner_margin(egui::style::Margin {
            left: panel_inner_margin_w,
            right: panel_inner_margin_w,
            top: panel_inner_margin_h,
            bottom: panel_inner_margin_h,
        });
    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(ctx, |ui| {
            // ui.vertical_centered(|ui| {
            ui.vertical(|ui| {
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
                    let logo_text = RichText::new("emo bg visualizer").text_style(logo());

                    ui.label(logo_text);
                    ui.small("A background visualizer for emotions found in text");
                });
                heading_frame.show(ui, |ui| {
                    draw_main_nav(ui, state);
                });
                view_fn(ctx, ui, state, work_size);
                draw_analysis_panel(ctx, state, work_size);
            });
        });
}


fn draw_about_view(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State, work_size: Vec2) {
    let mut output = plugins.egui(|ctx| {
        let ui_fill = ui_common_setup(ctx, state, work_size);
        draw_with_main_panel(
            ctx,
            state,
            work_size,
            ui_fill,
            // |ctx, ui, state, work_size| {
            |_, ui, _, _| {
                ui.label("This app explores the presentation of written works with background visualizations driven by emotion analysis of the text.\n");
                ui.label("(Please note that this is a work in progress -- further details will be added soon.\n");
                // Add bit about pre-preparing analysis to JSON files to the para below
                // ui.label("For each work, analysis is performed per paragraph (or stanza, in the case of poems) allowing the analysis-driven visualization to change as the reader progresses through the work.\n");
                // ui.label("Describe the analyzer & caveats.\n");
                // ui.label("Currently there is just one visualization model: a \"Simple Color\" model which uses emotion to color associations (based on some different color psychology models) to transition the background color as one goes through  the written piece. The plan is to develop further visualization models in the future.\n");

            },
        );
    });

    output.clear_color(CLEAR_COLOR);
    if output.needs_repaint() {
        gfx.render(&output);
    }
}


fn draw_settings_view(
    gfx: &mut Graphics,
    plugins: &mut Plugins,
    state: &mut State,
    work_size: Vec2,
) {
    let mut output = plugins.egui(|ctx| {
        let ui_fill = ui_common_setup(ctx, state, work_size);
        draw_with_main_panel(
            ctx,
            state,
            work_size,
            ui_fill,
            // |ctx, ui, state, work_size| {
            |_, ui, state, _| {
                let margin = work_size.y * 0.02;
                let mut heading_frame =
                    egui::Frame::none()
                        .fill(ui_fill)
                        .inner_margin(egui::style::Margin {
                            left: 0.0,
                            right: 0.0,
                            top: 0.0,
                            bottom: margin,
                        });

                heading_frame.show(ui, |ui| {
                    ui.heading("General Options");
                });

                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("Select Visualizer");
                    let visualizer_name = match state.selected_visualizer {
                        // Annoying that padding is necessary here, or else it wraps the longer
                        // options uglily
                        VisualizerSelection::Tiles => "Tiles",
                        _ => "Color Transition",
                    };
                    egui::ComboBox::from_id_source("selected-visualizer")
                        .selected_text(visualizer_name)
                        .wrap(false)
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.selectable_value(
                                &mut state.selected_visualizer,
                                VisualizerSelection::ColorTransition,
                                "Color Transition",
                            );
                            ui.selectable_value(
                                &mut state.selected_visualizer,
                                VisualizerSelection::Tiles,
                                "Tiles",
                            );
                        });
                });

                heading_frame.inner_margin.top = margin * 2.0;
                heading_frame.show(ui, |ui| {
                    ui.heading("Visualizer Specific");
                });
                state.visualizer.egui_settings(ui);
            },
        );
    });

    output.clear_color(CLEAR_COLOR);
    if output.needs_repaint() {
        gfx.render(&output);
    }
}


fn draw_home_view(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State, work_size: Vec2) {
    let mut output = plugins.egui(|ctx| {
        let ui_fill = ui_common_setup(ctx, state, work_size);
        draw_with_main_panel(
            ctx,
            state,
            work_size,
            ui_fill,
            // &draw_public_domain_menu_items,
            |ctx, ui, state, work_size| {
                let heading_frame_margin = work_size.y * 0.01;
                let heading_frame =
                    egui::Frame::none()
                        .fill(ui_fill)
                        .inner_margin(egui::style::Margin {
                            left: 0.0,
                            right: 0.0,
                            top: 0.0,
                            bottom: heading_frame_margin,
                        });

                // ui.separator();
                heading_frame.show(ui, |ui| {
                    ui.heading("Read select poems and prose from the public domain:");
                });

                egui::ScrollArea::vertical()
                    // .max_width(500.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            let mut style = (*ctx.style()).clone();
                            // let button_top_margin = work_size.y * 0.0075;
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
                            style.spacing.button_padding =
                                egui::Vec2::new(button_padding_x, button_padding_y);
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
                                    let author_text = RichText::new(&emodoc.author)
                                        .text_style(author_menu_text());
                                    ui.label(author_text);
                                });
                            }
                        });
                    });
            },
        );
    });

    output.clear_color(CLEAR_COLOR);
    if output.needs_repaint() {
        gfx.render(&output);
    }
}


fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let work_size = get_work_size(gfx);

    if state.selected_visualizer != state.visualizer.get_enum() {
        match state.selected_visualizer {
            VisualizerSelection::Tiles => {
                log::debug!("swap to TilesVisualizer");
                state.visualizer = Box::new(TilesVisualizer::new(
                    CLEAR_COLOR,
                    TITLE_COLOR,
                    DYNAMIC_TEXT_COLOR,
                    state.tile_texture.clone(),
                ));
            }
            _ => {
                log::debug!("swap to ColorTransitionVisualizer");
                state.visualizer = Box::new(ColorTransitionVisualizer::new(
                    CLEAR_COLOR,
                    TITLE_COLOR,
                    DYNAMIC_TEXT_COLOR,
                ));
            }
        }
    }


    match state.view {
        View::READ => draw_read_view(app, gfx, plugins, state, work_size),
        View::ABOUT => draw_about_view(gfx, plugins, state, work_size),
        View::SETTINGS => draw_settings_view(gfx, plugins, state, work_size),
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
        // ScreenDimensions::RES_HDPLUS.x as i32,
        // ScreenDimensions::RES_HDPLUS.y as i32,
        ScreenDimensions::RES_1080P.x as i32,
        ScreenDimensions::RES_1080P.y as i32,
        // ScreenDimensions::DEFAULT.x as i32,
        // ScreenDimensions::DEFAULT.y as i32,
    );

    #[cfg(target_arch = "wasm32")]
    let win_config = get_common_win_config().high_dpi(true);

    set_html_bgcolor(CLEAR_COLOR);

    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(win_config)
        .add_config(EguiConfig)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .add_plugin(FpsLimit::new(MAX_FPS))
        // .touch_as_mouse(false)
        .event(event)
        .draw(draw)
        .update(update)
        .build()
}
