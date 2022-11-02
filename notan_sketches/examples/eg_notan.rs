use notan::draw::*;
use notan::log;
use notan::prelude::*;


#[notan_main]
fn main() -> Result<(), String> {
    notan::init()
        .add_config(log::LogConfig::debug())
        .add_config(DrawConfig)
        .initialize(starting_logs)
        .draw(draw)
        .build()
}


fn starting_logs() {
    log::debug!("Hello, this is a debug log...");
    log::info!("And this is a info log!");
    log::warn!("I'm warning you");
    log::error!("I'm an error, I told you...");
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);
}
