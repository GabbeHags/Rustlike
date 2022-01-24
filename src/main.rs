

mod game;
mod view;
mod controller;

fn main() -> crossterm::Result<()>{
    // let (x, y) = (20, 10);
    let (x, y) = crossterm::terminal::size().unwrap();
    let x = x as usize;
    let y= y as usize-1;
    let mut model = game::Model::new(x, y);
    let mut screen = view::Screen::new();
    screen.start()?;
    loop {
        model.update();
        screen.clear_screen()?;
        screen.render(&model);
        match controller::read() {
            game::Events::Quit => break,
            e => {model.do_action(e)}
        }
    }
    screen.end()?;
    Ok(())
}
