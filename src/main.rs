use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, KeyCode},
    style::{Color, Print, Stylize},
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

#[derive(Clone)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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

fn draw_screen(
    screen: &mut Stdout,
    world: &mut World,
    player: String,
    rows: u16,
    columns: u16,
    to_where: DIRECTION,
) -> std::io::Result<()> {
    clear_at_line(screen, world.player_row, world.player_column)?;
    match to_where {
        DIRECTION::UP => {
            // go up shit
            if world.player_row > 1 {
                world.player_row -= 1;
            } else {
                world.player_row = rows - 2;
            }
            clear_at_line(screen, world.player_row, world.player_column)?;
        }
        DIRECTION::DOWN => {
            // go down shit
            if world.player_row < rows - 2 {
                world.player_row += 1;
            } else {
                world.player_row = 1;
            }
        }
        DIRECTION::RIGHT => {
            // go right shit
            if world.player_column < columns - 2 {
                world.player_column += 1;
            } else {
                world.player_column = 1;
            }
        }
        DIRECTION::LEFT => {
            // go left shit
            if world.player_column > 2 {
                world.player_column -= 1;
            } else {
                world.player_column = columns - 2;
            }
        }
    }
    screen.queue(MoveTo(world.player_column, world.player_row))?;
    screen.queue(Print(player.with(Color::Red)))?;

    screen.flush()?;
    Ok(())
}

fn clear_at_line(screen: &mut Stdout, row: u16, col: u16) -> std::io::Result<()> {
    screen.queue(MoveTo(col, row))?;
    screen.queue(Print(" "))?;
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
    let player_speed = 50;

    // player string
    let player = String::from("8");
    let mut player_direction = DIRECTION::RIGHT;

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
        if poll(Duration::from_millis(player_speed))? {
            let reading = read().unwrap();
            match reading {
                crossterm::event::Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => {
                        screen.queue(Print("\nThank you.Game Over.\n"))?;
                        screen.queue(MoveTo(0, 0))?;
                        break 'game;
                    }
                    KeyCode::Char('w') => {
                        player_direction = DIRECTION::UP;
                    }
                    KeyCode::Char('s') => {
                        player_direction = DIRECTION::DOWN;
                    }
                    KeyCode::Char('a') => {
                        player_direction = DIRECTION::LEFT;
                    }
                    KeyCode::Char('d') => {
                        player_direction = DIRECTION::RIGHT;
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
        draw_screen(
            &mut screen,
            &mut world,
            player.clone(),
            rows,
            columns,
            player_direction.clone(),
        )?;
    }

    // exit shit
    screen.execute(Show)?;
    disable_raw_mode()?;

    Ok(())
}
