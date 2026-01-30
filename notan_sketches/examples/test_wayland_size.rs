use notan::draw::*;
use notan::log;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    frame_count: u32,
    size_detected: bool,
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    log::info!("=== INIT ===");
    log::info!("Screen size: {:?}", app.window().screen_size());
    log::info!("Container size: {:?}", app.window().container_size());
    log::info!("Graphics size: {:?}", gfx.size());
    log::info!("DPI: {}", app.window().dpi());

    State {
        frame_count: 0,
        size_detected: false,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.frame_count += 1;

    let screen_size = app.window().screen_size();
    let container_size = app.window().container_size();

    // Only log for first 100 frames or till size changes from (0,0)
    if state.frame_count <= 100
        && (!state.size_detected && (screen_size.0 != 0 || container_size.0 != 0))
    {
        log::info!(
            "Frame {}: Screen size: {:?}, Container size: {:?}",
            state.frame_count,
            screen_size,
            container_size
        );

        if !state.size_detected && (screen_size.0 != 0 || container_size.0 != 0) {
            state.size_detected = true;
            log::info!("✓ Size detected at frame {}", state.frame_count);
        }
    }
}

fn draw(_app: &mut App, gfx: &mut Graphics, _state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::from_rgb(0.1, 0.2, 0.3));

    gfx.render(&draw);
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(log::LogConfig::debug())
        .add_config(WindowConfig::default().set_size(800, 600))
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}
