// 2023-06-06
use std::io::{self, BufRead, BufReader};
use std::fs::File;

extern crate ncurses;

use ncurses::*;
use std::char;

const PLAYER_SYMBOL: char = '*';

struct Position {
    pub x: usize,
    pub y: usize,
}

struct Unit{
    pub position: Position,
    pub visibility_range: usize,
    pub speed: usize,
}

fn main() {
    let width: usize = 20;
    let height: usize = 20;

    // let mut player.position: Position = Position { x: 0, y: 0 };
    let mut player: Unit = Unit{
        position:  Position{ x: 0, y: 0 },
        visibility_range: 3,
        speed: 2
    };

    let characters_found: i8 = 0;
    let mut username = String::new();
    let characters_count: i8;
    let mut s_characters_count: String = String::new();
    let mut game_map: Vec<Vec<char>> = vec![vec![' '; width]; height];
    let mut actual_map: Vec<Vec<char>> = vec![vec![' '; width]; height];

    // loading map
    let f = BufReader::new(File::open("/usr/home/ishayahu/rust/dnd/map1.map").unwrap());
    //let mut line = String::new();

    for (y, line) in f.lines().enumerate(){
        for (x, char) in line.unwrap().chars().enumerate(){
            game_map[y][x] = char;
        }
        print!("\n");
    }

    println!("LaSil/IT D&D 5e");
    println!("What is your name?");
    io::stdin().read_line(&mut username).expect("no username supplied");
    /* match io::stdin().read_line(&mut username) {
        Ok() => {},
        Err(e) => {println!("Input error - {}", e)}
    } */
    println!("Wellcome, {}",username);

    println!("How many characters are in group?");

    io::stdin().read_line(&mut s_characters_count).expect("no input supplied");
    characters_count = s_characters_count.trim_end().to_owned().parse::<i8>().unwrap();
    // let characters_count: i8 = s_characters_count.trim().parse().unwrap()

    if characters_count>characters_found {
        println!("OOPS. I found onlye {} characters. You should create more characters", characters_found);
    } else {
        println!("Good. {} characters are ready", characters_count);
    }

    for i in  0..characters_count {
        println!("Character {} loaded.", i+1);
    }

    /* Setup ncurses. */
    initscr();
	start_color();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();
    clear();

    loop {
        make_actual_map(&mut actual_map, &game_map, &player);
        draw_map(&actual_map, &player);
        if !process_gamer_input(&game_map, &mut player){
            return;
        }
        refresh();
    }
}

fn process_gamer_input(game_map: &Vec<Vec<char>>, player: &mut Unit) -> bool {
    let key_pressed_str = char::from_u32(getch() as u32).expect("Invalid char");
    match key_pressed_str {
        '6' => {
            if game_map[player.position.y][player.position.x+1]!='@' {
                player.position.x += 1;
            }
        },
        '4' => {
            if game_map[player.position.y][player.position.x-1]!='@' {
                player.position.x -= 1;
            }
        },
        '8' => {
            if game_map[player.position.y-1][player.position.x]!='@' {
                player.position.y -= 1;
            }
        },
        '2' => {
            if game_map[player.position.y+1][player.position.x]!='@' {
                player.position.y += 1;
            }
        },
        'q' => return false,
        _ => (),
    }
    return true;
}

fn make_actual_map(actual_map: &mut Vec<Vec<char>>, game_map: &Vec<Vec<char>>, player: &Unit){
    // Сбрасываем карту
    for y in 0..game_map.len(){
        for x in 0..game_map[y].len(){
            actual_map[y][x] =  game_map[y][x];
        }
    }
    // Рисуем на ней персонажей
    actual_map[player.position.y][player.position.x] = PLAYER_SYMBOL;
}

fn draw_map(actual_map: &Vec<Vec<char>>, player: &Unit ){
    const UNIT_COLOR: i16 = 0;
    const FOREST_COLOR: i16 = 1;
    const MOUNT_COLOR: i16 = 2;
    const SAND_COLOR: i16 = 3;
    const ROAD_COLOR: i16 = 4;
    const DEFAULT_COLOR: i16 = 99;

    init_pair(FOREST_COLOR, COLOR_BLACK, COLOR_GREEN);
    init_pair(MOUNT_COLOR, COLOR_WHITE, COLOR_BLACK);
    init_pair(SAND_COLOR, COLOR_BLACK, COLOR_YELLOW);
    init_pair(ROAD_COLOR, COLOR_WHITE, COLOR_BLACK);
    init_pair(DEFAULT_COLOR, COLOR_WHITE, COLOR_BLACK);

    for y in 0..actual_map.len(){
        for x in 0..actual_map[y].len(){
            if (y as i8) < ((player.position.y as i8) - (player.visibility_range as i8))  ||
               (y as i8) > ((player.position.y as i8) + (player.visibility_range as i8)) ||
               (x as i8) < ((player.position.x as i8) - (player.visibility_range as i8))  ||
               (x as i8) > ((player.position.x as i8) + (player.visibility_range as i8)) {
                // вне зоны видимости
                mvaddch(y.try_into().unwrap(), x.try_into().unwrap(), '?' as u32);
            } else {
                match actual_map[y][x] {
                    '@' => {
                        attron(COLOR_PAIR(MOUNT_COLOR));
                        mvaddch(y.try_into().unwrap(), x.try_into().unwrap(), actual_map[y][x] as u32);
                        // mvaddch(y.try_into().unwrap(),x.try_into().unwrap(), ACS_BLOCK as u32);
                        attroff(COLOR_PAIR(MOUNT_COLOR));
                    },
                    'F' => {
                        attron(COLOR_PAIR(FOREST_COLOR));
                        // mvaddch(y.try_into().unwrap(),x.try_into().unwrap(), '╒' as u32);
                        mvaddch(y.try_into().unwrap(), x.try_into().unwrap(), actual_map[y][x] as u32);
                        attroff(COLOR_PAIR(FOREST_COLOR));
                    },
                    '0' => {
                        attron(COLOR_PAIR(SAND_COLOR));
                        mvaddch(y.try_into().unwrap(), x.try_into().unwrap(), actual_map[y][x] as u32);
                        // mvaddch(y.try_into().unwrap(),x.try_into().unwrap(), ACS_BOARD as u32);
                        attroff(COLOR_PAIR(SAND_COLOR));
                    },
                    PLAYER_SYMBOL => {
                        attron(COLOR_PAIR(UNIT_COLOR));
                        mvaddch(y.try_into().unwrap(), x.try_into().unwrap(), actual_map[y][x] as u32);
                        attroff(COLOR_PAIR(UNIT_COLOR));
                    },
                    _ => {
                        attron(COLOR_PAIR(DEFAULT_COLOR));
                        attroff(COLOR_PAIR(DEFAULT_COLOR));
                    },
                };
            };
        }
    }
}