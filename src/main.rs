use std::{error::Error, io, time::{Duration, Instant}, sync::mpsc, thread};

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}};
use rusty_snake::{frame::{self, new_frame, Drawable}, render, snake::{Snake, Direction}, pickup::Pickup};

fn main() -> Result <(), Box<dyn Error>> {

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render lopp in a separated thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn( move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    // Game Loop
    let mut snake = Snake::new();
    let mut pickup = Pickup::new();
    let mut instant = Instant::now();

    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => snake.change_direction(Direction::Left),
                    KeyCode::Right =>snake.change_direction(Direction::Right),
                    KeyCode::Up => snake.change_direction(Direction::Up),
                    KeyCode::Down => snake.change_direction(Direction::Down),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        snake.update(delta);
        if snake.eat_pickup(&mut pickup) {
            snake.grow()
        }
        if snake.dead {
            break 'gameloop;
        }
        // Draw & Render
        let drawables: Vec<&dyn Drawable> = vec![&snake, &pickup];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Clean-up
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
