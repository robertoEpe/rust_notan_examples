use notan::prelude::*;
use notan::draw::*;
#[derive(AppState)]
struct State {
    clr : Color,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
    .add_config(DrawConfig)
    .draw(draw).build()
}

fn setup(_fx: &mut Graphics) -> State {
    State { clr: Color::BLUE }
}

fn draw(_app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(state.clr.clone());
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);
}