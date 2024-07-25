use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;

struct World {
    player_row: u16,
    player_column: u16,
}
struct Food {
    f_row: u16,
    f_col: u16,
}

fn init_game(screen: &mut Stdout, row: u16, col: u16) -> std::io::Result<()> {
    screen.queue(MoveTo(0, 0))?;
    screen.queue(Clear(ClearType::All))?;
    screen.queue(Print("Starting..."))?;
    for i in 0..row * 10 {
        screen.queue(Print("-"))?;
        screen.queue(MoveTo(i, row))?;
    }
    for i in 0..row * 10 {
        screen.queue(Print("-"))?;
        screen.queue(MoveTo(i, 0))?;
    }
    for i in 0..col {
        screen.queue(Print("|"))?;
        screen.queue(MoveTo(0, i))?;
    }
    for i in 0..row * 10 {
        screen.queue(Print("|"))?;
        screen.queue(MoveTo(col, i))?;
    }
    Ok(())
}

fn draw_screen(screen: &mut Stdout, world: &World, player: String) -> std::io::Result<()> {
    screen.queue(MoveTo(world.player_column, world.player_row))?;
    screen.queue(Print(player))?;
    screen.flush()?;
    Ok(())
}

fn clear_at_line(screen: &mut Stdout, row: u16, col: u16) -> std::io::Result<()> {
    screen.queue(MoveTo(col, row))?;
    screen.queue(Print(" "))?;
    // screen.queue(Clear(ClearType::UntilNewLine))?;
    Ok(())
}

fn spawn_food(screen: &mut Stdout, row: u16, col: u16, food: &mut Food) -> std::io::Result<()> {
    let r_cor = rand::thread_rng().gen_range(3..row - 1);
    let c_cor = rand::thread_rng().gen_range(3..col - 1);
    screen.queue(MoveTo(c_cor, r_cor))?;
    screen.queue(Print("*"))?;
    screen.flush()?;
    food.f_col = c_cor;
    food.f_row = r_cor;
    Ok(())
}

fn eat(
    screen: &mut Stdout,
    world: &World,
    row: u16,
    col: u16,
    food: &mut Food,
    user_score: &mut u16,
) -> std::io::Result<()> {
    if world.player_column == food.f_col && world.player_row == food.f_row {
        spawn_food(screen, row, col, food)?;
        *user_score += 1;
        screen.flush()?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // setup screen
    let mut screen = stdout();

    // hide cursor
    screen.execute(Hide)?;

    // read about this shit
    enable_raw_mode()?;

    // getting the screensize
    let (columns, rows) = size().unwrap();

    // setup world
    let mut world = World {
        player_column: 1,
        player_row: 1,
    };

    // player string
    let player = String::from("8");

    // food cor
    let mut food = Food { f_col: 0, f_row: 0 };

    // clear terminal first
    init_game(&mut screen, rows, columns)?;

    // spawn food
    spawn_food(&mut screen, rows, columns, &mut food).unwrap();

    // score shit
    let mut player_score = 0;
    screen.queue(MoveTo(columns - 11, 1))?;
    screen.queue(Print("Score:"))?;
    screen.queue(Print(player_score))?;

    // game loop
    'game: loop {
        // read input
        if poll(Duration::from_millis(500))? {
            let reading = read().unwrap();
            match reading {
                crossterm::event::Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => {
                        screen.queue(Print("\nThank you.Game Over.\n"))?;
                        screen.queue(MoveTo(0, 0))?;
                        break 'game;
                    }
                    KeyCode::Char('w') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        if world.player_row > 1 {
                            world.player_row -= 1;
                        } else {
                            world.player_row = rows - 2;
                        }
                    }
                    KeyCode::Char('s') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        if world.player_row < rows - 2 {
                            world.player_row += 1;
                        } else {
                            world.player_row = 1;
                        }
                    }
                    KeyCode::Char('a') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        if world.player_column > 1 {
                            world.player_column -= 1;
                        } else {
                            world.player_column = columns - 2;
                        }
                    }
                    KeyCode::Char('d') => {
                        clear_at_line(&mut screen, world.player_row, world.player_column)?;
                        if world.player_column < columns - 2 {
                            world.player_column += 1;
                        } else {
                            world.player_column = 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // do physics / calculation
        screen.queue(MoveTo(columns - 11, 1))?;
        screen.queue(Print("Score:"))?;
        screen.queue(Print(player_score))?;

        eat(
            &mut screen,
            &world,
            rows,
            columns,
            &mut food,
            &mut player_score,
        )?;

        // draw screen
        draw_screen(&mut screen, &world, player.clone())?;
    }
    screen.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
