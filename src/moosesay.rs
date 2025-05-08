use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use crossterm::{
    terminal::{self, Clear, ClearType},
    cursor::{Hide, Show, MoveTo},
    queue,
    execute,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    ExecutableCommand,
};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use crate::quotes;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn random() -> Self {
        match rand::random::<u8>() % 8 {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            4 => Direction::UpLeft,
            5 => Direction::UpRight,
            6 => Direction::DownLeft,
            _ => Direction::DownRight,
        }
    }

    fn get_dx_dy(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Tree {
    x: i32,
    y: i32,
    growth_stage: u8,
    max_stage: u8,
    growth_timer: f32,
    is_falling: bool,
    fall_progress: f32,
    branches: Vec<(i32, i32)>,
    age: f32,
    is_dead: bool,
    decay_stage: u8,
    decay_timer: f32,
    color: Color,
    is_growing: bool,
    is_multiplying: bool,
    is_stomping: bool,
    event_timer: f32,
}

#[derive(Debug, Clone)]
struct Seed {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    age: f32,
    is_planted: bool,
    target_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx * dx + dy * dy) as f32).sqrt()
    }

    fn neighbors(&self, width: i32, height: i32) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let new_x = self.x + dx;
                let new_y = self.y + dy;
                if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    neighbors.push(Point::new(new_x, new_y));
                }
            }
        }
        neighbors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    point: Point,
    f: i32,
    g: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Animal {
    x: i32,
    y: i32,
    is_rabbit: bool,
    animation_frame: u8,
    collected_nuts: Vec<(i32, i32)>,
    target_nut: Option<(i32, i32)>,
    move_timer: f32,
    nuts_collected: u32,
    path: Vec<Point>,
    target_sprout: Option<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MooseStyle {
    Default,
    VK2001,
    JGSVK,
    DaveBird,
    Bulldog,
    Unknown,
    DanFunky,
    KeithAmmann,
}

#[derive(Debug, Clone)]
struct Moose {
    x: i32,
    y: i32,
    direction: Direction,
    is_edge_walker: bool,
    animation_frame: u8,
    speech_bubble: Option<String>,
    speech_timer: f32,
    center_x: i32,
    center_y: i32,
    style: MooseStyle,
    quote_timer: f32,
    quote_db: quotes::QuoteDatabase,
    path: Vec<Point>,
    target_tree: Option<Point>,
    stomp_cooldown: f32,
    current_quote: Option<String>,
    typed_text: String,
    typing_buffer: String,
}

impl Moose {
    pub fn new(width: u16, height: u16) -> Self {
        let center_x = width as i32 / 2;
        let center_y = height as i32 / 2;
        let style = MooseStyle::random();
        let mut quote_db = quotes::QuoteDatabase::new();
        let initial_quote = quote_db.next_random();
        Self {
            x: center_x,
            y: center_y,
            direction: Direction::Right,
            is_edge_walker: false,
            animation_frame: 0,
            speech_bubble: None,
            speech_timer: 0.0,
            center_x,
            center_y,
            style,
            quote_timer: 0.0,
            quote_db,
            path: Vec::new(),
            target_tree: None,
            stomp_cooldown: 0.0,
            current_quote: Some(initial_quote.text),
            typed_text: String::new(),
            typing_buffer: String::new(),
        }
    }

    pub fn update(&mut self, width: u16, height: u16, trees: &[Tree]) {
        // Update animation frame
        self.animation_frame = (self.animation_frame + 1) % 3;

        // Update stomp cooldown
        if self.stomp_cooldown > 0.0 {
            self.stomp_cooldown -= 0.1;
        }

        // Find nearest tree if we don't have a target
        if self.target_tree.is_none() && self.stomp_cooldown <= 0.0 {
            let mut nearest_dist = f32::MAX;
            for tree in trees {
                if !tree.is_dead && !tree.is_falling {
                    let dx = tree.x - self.x;
                    let dy = tree.y - self.y;
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                    if dist < nearest_dist {
                        nearest_dist = dist;
                        self.target_tree = Some(Point::new(tree.x, tree.y));
                    }
                }
            }
        }

        // If we have a target tree, use A* to find path
        if let Some(target) = self.target_tree {
            if self.path.is_empty() {
                if let Some(path) = astar(
                    Point::new(self.x, self.y),
                    target,
                    width as i32,
                    height as i32,
                ) {
                    self.path = path;
                }
            }

            // Follow path
            if let Some(next_point) = self.path.get(1) {
                // Update direction based on movement
                let dx = next_point.x - self.x;
                let dy = next_point.y - self.y;
                self.direction = match (dx, dy) {
                    (0, -1) => Direction::Up,
                    (0, 1) => Direction::Down,
                    (-1, 0) => Direction::Left,
                    (1, 0) => Direction::Right,
                    (-1, -1) => Direction::UpLeft,
                    (1, -1) => Direction::UpRight,
                    (-1, 1) => Direction::DownLeft,
                    (1, 1) => Direction::DownRight,
                    _ => self.direction,
                };

                self.x = next_point.x;
                self.y = next_point.y;
                self.path.remove(0);

                // Check if we reached the tree
                if self.x == target.x && self.y == target.y {
                    self.target_tree = None;
                    self.path.clear();
                    self.stomp_cooldown = 50.0; // Set cooldown after stomping
                }
            }
        } else {
            // Random movement when no trees are available
            if rand::random::<f64>() < 0.1 {
                self.direction = Direction::random();
            }

            // Move based on direction with screen wrapping
            let (dx, dy) = self.direction.get_dx_dy();
            self.x = (self.x + dx).rem_euclid(width as i32);
            self.y = (self.y + dy).rem_euclid(height as i32);
        }

        // Update quote timer and ensure only one quote at a time
        if self.current_quote.is_none() {
            self.quote_timer += 0.1;
            if self.quote_timer >= 300.0 { // 30 seconds (300 frames at 10fps)
                self.quote_timer = 0.0;
                let quote = self.quote_db.next_random();
                self.current_quote = Some(quote.text);
                self.typed_text.clear();
            }
        }

        // Update speech bubble timer
        if self.speech_timer > 0.0 {
            self.speech_timer -= 1.0;
            if self.speech_timer <= 0.0 {
                self.speech_bubble = None;
            }
        }
    }

    pub fn handle_input(&mut self, input: char) {
        if let Some(quote) = &self.current_quote {
            if input == '\x08' { // Backspace
                self.typed_text.pop();
            } else if input.is_ascii() && !input.is_control() {
                self.typed_text.push(input);
                
                // Check if the typed text matches the quote
                if self.typed_text == *quote {
                    self.current_quote = None;
                    self.quote_timer = 0.0;
                }
            }
        }
    }

    pub fn draw(&self) -> Vec<String> {
        let mut moose = match self.direction {
            Direction::Left | Direction::UpLeft | Direction::DownLeft => self.draw_moose_left(),
            Direction::Right | Direction::UpRight | Direction::DownRight => self.draw_moose_right(),
            Direction::Up => self.draw_moose_up(),
            Direction::Down => self.draw_moose_down(),
        };

        // Add speech bubble if present
        if let Some(text) = &self.speech_bubble {
            let bubble_width = text.len() + 4;
            let bubble_x = match self.direction {
                Direction::Left | Direction::UpLeft | Direction::DownLeft => -2 - bubble_width as i32,
                _ => 14, // Right side of moose
            };
            let bubble_y = -2; // Above moose's head

            // Create speech bubble
            let mut bubble = vec![
                format!(" {} ", "_".repeat(bubble_width)),
                format!("/ {} \\", " ".repeat(bubble_width)),
                format!("| {} |", text),
                format!("\\ {} /", "_".repeat(bubble_width)),
            ];

            // Add connector to moose's head
            match self.direction {
                Direction::Left | Direction::UpLeft | Direction::DownLeft => {
                    bubble.push("      \\".to_string());
                },
                _ => {
                    bubble.push("    /".to_string());
                }
            }

            // Insert bubble above moose
            for (i, line) in bubble.into_iter().enumerate() {
                if i as i32 + bubble_y >= 0 && i as i32 + bubble_y < moose.len() as i32 {
                    let mut new_line = " ".repeat(bubble_x.max(0) as usize);
                    new_line.push_str(&line);
                    moose[i as usize] = new_line;
                }
            }
        }

        moose
    }

    fn draw_moose_left(&self) -> Vec<String> {
        match self.style {
            MooseStyle::Default => vec![
                r"          \`-'.'.   /`.    |\",
                r"           \     `-'   \  / '-./\       .'\",
                r"            `-.         '.       `-.)\.'  /",
                r"               `._       ''-. `:.      _.'",
                r"                  `-._...__.::::.__.--'",
                r"                       _.-..'''''.",
                r"               _.---.__`._.       `-.",
                r"        ___..-'             `o>      `-.",
                r"   .-```                           <)   )",
                r" .'                         `._.-.`-._.'",
                r"/                           /     `-'",
                r"|            '             /",
                r"|     .       '          .'",
                r" \     \       \       .'|",
                r"  '    / -..__.|     /   |",
                r"  |   /|   |   \    '\   /",
                r"  \   | \  |    |  | |  |",
                r"   \ /   . |    |  /  \ |",
                r"   | |    \ \   | |   | |",
                r"   | \     | `. | |   | |",
                r"   |  \    /   `' |  /_  `.",
                r"   /'  \   `---/_  `.  `.\.'",
                r"    `.\.' LGB    `.\.'",
            ],
            MooseStyle::VK2001 => vec![
                r"                   __,)____",
                r"                   `,  `__,`",
                r"                   /  /((",
                r"         ______,--'  (",
                r"        (            | ",
                r"         \  )    (   |",
                r"        /| /`-----` /|",
                r"        \ \        / |",
                r"ejm     |\|\      /| |\\",
            ],
            MooseStyle::JGSVK => vec![
                r"      _/\_       __/\__",
                r"      ) . (_    _) .' (",
                r"      `) '.(   ) .'  (`",
                r"       `-._\(_ )/__(~`",
                r"           (ovo)-.__.--._",
                r"           )             `-.______",
                r"  jgs/VK  /                       `---._",
                r"         ( ,// )                        \\",
                r"          `''\/-.                        |",
                r"                 \\                       |",
                r"                 |                       |",
            ],
            MooseStyle::DaveBird => vec![
                r"    ___            ___",
                r"   /   \\          /   \\",
                r"   \\_   \\        /  __/",
                r"    _\\   \\      /  /__",
                r"    \\___  \\____/   __/",
                r"        \\_       _/",
                r"          | @ @  \\_",
                r"          |",
                r"        _/     /\\   -Dave Bird-",
                r"       /o)  (o/\\ \\_",
                r"       \\_____/ /",
                r"         \\____/",
            ],
            MooseStyle::Bulldog => vec![
                r"          W_W_-__/",
                r"           Uv.   )",
                r"             ||-|\\",
                r"             || | |",
            ],
            MooseStyle::Unknown => vec![
                r" /^\                                         /^\",
                r"|   \_/^\                               /^\_/   |",
                r"|        \_/^\                     /^\_/        |",
                r" \            \_/^\           /^\_/            /",
                r"  \__              \___---___/              __/",
                r"     ---___         /       \         ___---",
                r"           ---___  |         |  ___---",
                r"                 --|  _   _  |--",
                r"                  |  | | | |  |",
                r"                  |  |o| |o|  |",
                r"                 /    -   -    \\",
                r"                |      ___      |",
                r"               /     --   --     \\",
                r"              |                   |",
                r"             /                     \\",
                r"            |                       |",
                r"            |                       |",
                r"            |       /\     /\       |",
                r"             \ \   |  |   |  |   / /",
                r"             /\ \   --     --   / /\\",
                r"            /  \ \_____   _____/ /  \\",
                r"           /    \__    ---    __/    \\",
                r"          /\       --__---__--       /\\",
                r"         /  \/                3    \/  \\",
                r"        /   /    _S___W___F____     \   \\",
                r"       /   |             _______-----|   \\",
                r"      |    |________-----            |    |",
                r"      |    |                         |    |",
                r"      |    |                         |    |",
                r"       \    \                       /    /",
                r"        \\   \                     /   //",
                r"         \\ \_\                   /_/ //",
                r"           --  \                 /  --",
                r"                |  ---_____---  |",
                r"                |     |   |     |",
                r"                |     |   |     |",
                r"                |     |   |     |",
                r"                |     |   |     |",
                r"               / V     \ /    V  \\",
                r"               |_|_____| |____|__|",
            ],
            MooseStyle::DanFunky => vec![
                r"      \     \ /            \/    ___//",
                r"    \_ /    //             \]   //~~~",
                r"      \\    ]]            //   //",
                r"     \__\ _]_\_          _\\ __/\//",
                r"         __ _____\        /_\//  _",
                r"     __ _/     \/~~~~~~\/ \__ //",
                r"      _/       [        ]    \/",
                r"              /[  /  \  ]",
                r"             /  [(0  0)]",
                r"            /   [      ]",
                r"  _________~   [        ]",
                r"               \ <    > /",
                r"              / \______/",
                r"              ]     (_)",
                r"             ]",
            ],
            MooseStyle::KeithAmmann => vec![
                r" _  _  _  _      *      _  _  _  _",
                r"8\\_\\_\\_\\    / \    //_//_//_//8",
                r"8 \_        \  /===\  /        _/ 8",
                r"8   \____    \wwwwwww/    ____/   8",
                r"8        \___ WWWWWWW ___/        8",
                r"8           /  o   o  \           8           M O O S E",
                r"8          /_/|     |\_\          8",
                r"8             |     |             8",
                r"8       _____/|     |\===___      8  O N   P O R C H   S W I N G",
                r"8      /     \_\_T_/_<_  \  \     8",
                r"8     |        \ovo/ <___/\  |    8       W I T H   B E E R",
                r"8     |         | |   ===  \ |    8",
                r"8     |   \       |     \   \|    8",
                r"8     |    \_________    \   |    8",
                r"8      \             \    \__/    8",
                r"8       \___________  \__ /       8",
                r"8         /   \     \_\\\         8",
                r"8        |     |   |     |        8     -Keith Ammann-",
                r"O        |     |   |     |        O",
                r"=========|     |===|     |=========",
                r"=========|     |===|     |=========",
                r"         |     |   |     |",
            ],
        }.into_iter().map(|s| s.to_string()).collect()
    }

    fn draw_moose_right(&self) -> Vec<String> {
        match self.style {
            MooseStyle::Default => vec![
                r"          /`.'/.-'`/    /|",
                r"         /'.-'`-'     /  /\`-.'       /'.",
                r"        /  .'       .'       `-.)/.'  \\",
                r"       `.'      _.-''`  .:`      `._'",
                r"      `--.__.:::.`...-._'",
                r"       `.''''''..-._",
                r"       `-.       `._`.__---._",
                r"       `-.      `o<             `-..___",
                r"   (   (>                           ```-.",
                r"   `'._`-._.'                         `.",
                r"   `-'     \\                           \\",
                r"            \\             '            |",
                r"             `.          '       .     |",
                r"             |`.       /       /     /",
                r"             |   \\    |     .-..\\    '",
                r"             /   /    /    /   |\\   |",
                r"            |  | |  |    /  | / |   /",
                r"            | /  \\  |    | .   \\ /",
                r"            | |   | |   / /    | |",
                r"            | |   | | .' |     / |",
                r"           .'  \\_  | '`   \\    /  |",
                r"          .'/.  `.  \\_---'`   /  '\\",
                r"          .'/.    `.GBL    .'/.`",
            ],
            MooseStyle::VK2001 => vec![
                r"                   _       _",
                r"                  ) \\     /_(",
                r"                   )_`-)-_(_",
                r"                    `,' `__,)",
                r"                   _/   ((",
                r"          ______,-'    )",
                r"         (            ,",
                r"          \\  )    (   |",
                r"         /| /`-----` /|",
                r"         \\ \\        / |",
                r" ejm     |\\|\\      /| |\\",
            ],
            MooseStyle::JGSVK => vec![
                r"      _/\\_       __/\\__",
                r"      ) . (_    _) .' (",
                r"      `) '.(   ) .'  (`",
                r"       `-._\\(_ )/__(~`",
                r"           (ovo)-.__.--._",
                r"           )             `-.______",
                r"  jgs/VK  /                       `---._",
                r"         ( ,// )                        \\",
                r"          `''\\/-.                        |",
                r"                 \\                       |",
                r"                 |                       |",
            ],
            MooseStyle::DaveBird => vec![
                r"    ___            ___",
                r"   /   \\          /   \\",
                r"   \\_   \\        /  __/",
                r"    _\\   \\      /  /__",
                r"    \\___  \\____/   __/",
                r"        \\_       _/",
                r"          | @ @  \\_",
                r"          |",
                r"        _/     /\\   -Dave Bird-",
                r"       /o)  (o/\\ \\_",
                r"       \\_____/ /",
                r"         \\____/",
            ],
            MooseStyle::Bulldog => vec![
                r"          W_W_-__/",
                r"           Uv.   )",
                r"             ||-|\\",
                r"             || | |",
            ],
            MooseStyle::Unknown => vec![
                r" /^\                                         /^\",
                r"|   \_/^\                               /^\_/   |",
                r"|        \_/^\                     /^\_/        |",
                r" \            \_/^\           /^\_/            /",
                r"  \__              \___---___/              __/",
                r"     ---___         /       \         ___---",
                r"           ---___  |         |  ___---",
                r"                 --|  _   _  |--",
                r"                  |  | | | |  |",
                r"                  |  |o| |o|  |",
                r"                 /    -   -    \\",
                r"                |      ___      |",
                r"               /     --   --     \\",
                r"              |                   |",
                r"             /                     \\",
                r"            |                       |",
                r"            |                       |",
                r"            |       /\     /\       |",
                r"             \ \   |  |   |  |   / /",
                r"             /\ \   --     --   / /\\",
                r"            /  \ \_____   _____/ /  \\",
                r"           /    \__    ---    __/    \\",
                r"          /\       --__---__--       /\\",
                r"         /  \/                3    \/  \\",
                r"        /   /    _S___W___F____     \   \\",
                r"       /   |             _______-----|   \\",
                r"      |    |________-----            |    |",
                r"      |    |                         |    |",
                r"      |    |                         |    |",
                r"       \    \                       /    /",
                r"        \\   \                     /   //",
                r"         \\ \_\                   /_/ //",
                r"           --  \                 /  --",
                r"                |  ---_____---  |",
                r"                |     |   |     |",
                r"                |     |   |     |",
                r"                |     |   |     |",
                r"                |     |   |     |",
                r"               / V     \ /    V  \\",
                r"               |_|_____| |____|__|",
            ],
            MooseStyle::DanFunky => vec![
                r"      \     \ /            \/    ___//",
                r"    \_ /    //             \]   //~~~",
                r"      \\    ]]            //   //",
                r"     \__\ _]_\_          _\\ __/\//",
                r"         __ _____\        /_\//  _",
                r"     __ _/     \/~~~~~~\/ \__ //",
                r"      _/       [        ]    \/",
                r"              /[  /  \  ]",
                r"             /  [(0  0)]",
                r"            /   [      ]",
                r"  _________~   [        ]",
                r"               \ <    > /",
                r"              / \______/",
                r"              ]     (_)",
                r"             ]",
            ],
            MooseStyle::KeithAmmann => vec![
                r" _  _  _  _      *      _  _  _  _",
                r"8\\_\\_\\_\\    / \    //_//_//_//8",
                r"8 \_        \  /===\  /        _/ 8",
                r"8   \____    \wwwwwww/    ____/   8",
                r"8        \___ WWWWWWW ___/        8",
                r"8           /  o   o  \           8           M O O S E",
                r"8          /_/|     |\_\          8",
                r"8             |     |             8",
                r"8       _____/|     |\===___      8  O N   P O R C H   S W I N G",
                r"8      /     \_\_T_/_<_  \  \     8",
                r"8     |        \ovo/ <___/\  |    8       W I T H   B E E R",
                r"8     |         | |   ===  \ |    8",
                r"8     |   \       |     \   \|    8",
                r"8     |    \_________    \   |    8",
                r"8      \             \    \__/    8",
                r"8       \___________  \__ /       8",
                r"8         /   \     \_\\\         8",
                r"8        |     |   |     |        8     -Keith Ammann-",
                r"O        |     |   |     |        O",
                r"=========|     |===|     |=========",
                r"=========|     |===|     |=========",
                r"         |     |   |     |",
            ],
        }.into_iter().map(|s| s.to_string()).collect()
    }

    fn draw_moose_up(&self) -> Vec<String> {
        self.draw_moose_left()
    }

    fn draw_moose_down(&self) -> Vec<String> {
        self.draw_moose_left()
    }
}

impl MooseStyle {
    fn random() -> Self {
        match rand::random::<u8>() % 8 {
            0 => MooseStyle::Default,
            1 => MooseStyle::VK2001,
            2 => MooseStyle::JGSVK,
            3 => MooseStyle::DaveBird,
            4 => MooseStyle::Bulldog,
            5 => MooseStyle::Unknown,
            6 => MooseStyle::DanFunky,
            _ => MooseStyle::KeithAmmann,
        }
    }
}

impl Tree {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width.max(1) as i32),
            y: rng.gen_range(0..height.max(1) as i32),
            growth_stage: 0,
            max_stage: rng.gen_range(4..=8),
            growth_timer: 0.0,
            is_falling: false,
            fall_progress: 0.0,
            branches: Vec::new(),
            age: 0.0,
            is_dead: false,
            decay_stage: 0,
            decay_timer: 0.0,
            color: Color::Green,
            is_growing: false,
            is_multiplying: false,
            is_stomping: false,
            event_timer: 0.0,
        }
    }

    fn update(&mut self, width: u16, height: u16) {
        // Update event timer
        if self.event_timer > 0.0 {
            self.event_timer -= 0.1;
            if self.event_timer <= 0.0 {
                self.is_growing = false;
                self.is_multiplying = false;
                self.is_stomping = false;
            }
        }

        if !self.is_dead {
            self.age += 0.1;
            
            // Growth stages
            if self.growth_stage < self.max_stage {
                self.growth_timer += 0.1;
                if self.growth_timer >= 1.0 {
                    self.growth_timer = 0.0;
                    self.growth_stage += 1;
                    self.is_growing = true;
                    self.event_timer = 20.0; // Show growing animation for 2 seconds
                    
                    // Add new branches
                    if self.growth_stage > 1 {
                        let dx = if rand::random::<bool>() { 1 } else { -1 };
                        self.branches.push((dx, -(self.growth_stage as i32)));
                    }
                }
            }
            
            // Aging and death
            if self.age > 20.0 && !self.is_falling {
                self.is_falling = true;
                self.fall_progress = 0.0;
            }
        } else if self.is_falling {
            // Falling animation
            self.fall_progress += 0.1;
            if self.fall_progress >= 1.0 {
                self.is_falling = false;
                self.decay_stage = 1; // Start decay process
            }
        } else if self.decay_stage > 0 {
            // Decay process
            self.decay_timer += 0.1;
            if self.decay_timer >= 2.0 {
                self.decay_timer = 0.0;
                self.decay_stage += 1;
                
                // Update color based on decay stage
                self.color = match self.decay_stage {
                    1..=2 => Color::DarkGrey,
                    3..=4 => Color::Grey,
                    _ => Color::DarkGrey,
                };
                
                if self.decay_stage > 5 {
                    self.is_dead = true;
                }
            }
        }
    }

    fn draw(&mut self, stdout: &mut impl Write, width: u16, height: u16) -> io::Result<()> {
        if self.is_dead && !self.is_falling && self.decay_stage > 5 {
            return Ok(());
        }

        // Handle special event animations
        if self.is_growing {
            let emoji = match self.growth_stage {
                0 => "🌱",
                1 => "🌿",
                2 => "🌳",
                _ => "🌲",
            };
            queue!(
                stdout,
                MoveTo(self.x.clamp(0, width as i32 - 1) as u16, self.y.clamp(0, height as i32 - 1) as u16),
                SetForegroundColor(self.color),
                Print(emoji),
                ResetColor
            )?;
            return Ok(());
        }

        if self.is_multiplying {
            queue!(
                stdout,
                MoveTo(self.x.clamp(0, width as i32 - 1) as u16, self.y.clamp(0, height as i32 - 1) as u16),
                SetForegroundColor(self.color),
                Print("🌱"),
                ResetColor
            )?;
            return Ok(());
        }

        if self.is_stomping {
            queue!(
                stdout,
                MoveTo(self.x.clamp(0, width as i32 - 1) as u16, self.y.clamp(0, height as i32 - 1) as u16),
                SetForegroundColor(Color::DarkGrey),
                Print("💥"),
                ResetColor
            )?;
            return Ok(());
        }

        // Regular tree drawing
        let tree_art = if self.is_falling {
            match (self.fall_progress * 3.0) as u8 {
                0 => vec![
                    r"  /\  ",
                    r" /~~\ ",
                    r"/____\",
                    r"  ||  ",
                ],
                1 => vec![
                    r"   /\ ",
                    r"  /~~\",
                    r" /____",
                    r"   || ",
                ],
                _ => vec![
                    r"    /",
                    r"   /~",
                    r"  /__",
                    r"   | ",
                ],
            }
        } else if self.is_dead {
            // Dead tree appearance
            match self.decay_stage {
                0 | 1 => vec![
                    r"  XX  ",
                    r" X~~X ",
                    r"X____X",
                    r"  ||  ",
                ],
                2 | 3 => vec![
                    r"  xx  ",
                    r" x~~x ",
                    r"x____x",
                    r"  ||  ",
                ],
                _ => vec![
                    r"  ..  ",
                    r" .~~. ",
                    r".____.",
                    r"  ||  ",
                ],
            }
        } else {
            match self.growth_stage {
                0 => vec![r"·"],
                1 => vec![r"🌱"],
                2 => vec![
                    r" /^\",
                    r"|   \",
                    r" \__/",
                ],
                3 => vec![
                    r"  /\  ",
                    r" /~~\ ",
                    r"/____\",
                    r"  ||  ",
                ],
                4 => vec![
                    r"   /\   ",
                    r"  /~~\  ",
                    r" /____\ ",
                    r"/______\",
                    r"   ||   ",
                ],
                5 => vec![
                    r"    /\    ",
                    r"   /~~\   ",
                    r"  /____\  ",
                    r" /______\ ",
                    r"/________\",
                    r"    ||    ",
                ],
                _ => vec![
                    r"     /\     ",
                    r"    /~~\    ",
                    r"   /____\   ",
                    r"  /______\  ",
                    r" /________\ ",
                    r"/__________\",
                    r"     ||     ",
                ],
            }
        };

        // Calculate position to center the tree
        let tree_width = tree_art[0].len() as i32;
        let tree_height = tree_art.len() as i32;
        let x = self.x - tree_width / 2;
        let y = self.y - tree_height;

        // Draw the tree
        for (i, line) in tree_art.iter().enumerate() {
            queue!(
                stdout,
                MoveTo(x.clamp(0, width as i32 - tree_width) as u16, (y + i as i32).clamp(0, height as i32 - 1) as u16),
                SetForegroundColor(self.color),
                Print(line),
                ResetColor
            )?;
        }

        Ok(())
    }

    fn start_falling(&mut self) {
        self.is_falling = true;
        self.fall_progress = 0.0;
        self.is_stomping = true;
        self.event_timer = 20.0; // Show stomping animation for 2 seconds
    }

    fn spawn_seed(&mut self) -> Option<Seed> {
        if self.growth_stage >= 3 && !self.is_dead && !self.is_falling && rand::random::<f64>() < 0.1 {
            self.is_multiplying = true;
            self.event_timer = 20.0; // Show multiplying animation for 2 seconds
            let mut rng = rand::thread_rng();
            Some(Seed {
                x: self.x,
                y: self.y,
                dx: rng.gen_range(-1..=1),
                dy: 1,
                age: 0.0,
                is_planted: false,
                target_y: rng.gen_range(0..(self.y / 2)), // Random distance from top up to half current height
            })
        } else {
            None
        }
    }
}

impl Seed {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width.max(1) as i32),
            y: rng.gen_range(0..height.max(1) as i32),
            dx: 0,
            dy: 0,
            age: 0.0,
            is_planted: false,
            target_y: rng.gen_range(0..(height.max(2) as i32) / 2),
        }
    }

    fn update(&mut self, width: u16, height: u16) {
        if !self.is_planted {
            self.age += 0.1;

            // If we're not at our target y position, move towards it
            if self.y != self.target_y {
                self.dy = if self.y > self.target_y { -1 } else { 1 };
            } else {
                self.dy = 0;
            }

            // Random horizontal movement
            if rand::random::<f64>() < 0.1 {
                self.dx = rand::thread_rng().gen_range(-1..=1);
            }

            // Move seed
            self.x = (self.x + self.dx).clamp(0, width as i32 - 1);
            self.y = (self.y + self.dy).clamp(0, height as i32 - 1);

            // Plant when reaching target y or after some time
            if (self.y == self.target_y && self.age > 1.0) || self.age > 5.0 {
                self.is_planted = true;
                // Keep within bounds
                self.x = self.x.clamp(0, width as i32 - 1);
                self.y = self.y.clamp(0, height as i32 - 1);
            }
        }
    }

    fn draw(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        queue!(
            stdout,
            MoveTo(self.x as u16, self.y as u16),
            SetForegroundColor(if self.is_planted { Color::Green } else { Color::Rgb { r: 139, g: 69, b: 19 } }),
            Print(if self.is_planted { '🌱' } else { '🌰' })
        )?;
        Ok(())
    }
}

impl Animal {
    fn new(width: u16, height: u16, is_rabbit: bool) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width.max(1) as i32),
            y: rng.gen_range(0..height.max(1) as i32),
            is_rabbit,
            animation_frame: 0,
            collected_nuts: Vec::new(),
            target_nut: None,
            move_timer: 0.0,
            nuts_collected: 0,
            path: Vec::new(),
            target_sprout: None,
        }
    }

    fn update(&mut self, width: u16, height: u16, seeds: &[Seed]) -> Option<Animal> {
        self.move_timer += 0.1;
        if self.move_timer < 0.5 {
            return None;
        }
        self.move_timer = 0.0;

        if self.is_rabbit {
            // Find nearest sprout
            if self.target_sprout.is_none() {
                let mut nearest_dist = f32::MAX;
                for seed in seeds {
                    if seed.is_planted {
                        let dx = seed.x - self.x;
                        let dy = seed.y - self.y;
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        if dist < nearest_dist {
                            nearest_dist = dist;
                            self.target_sprout = Some(Point::new(seed.x, seed.y));
                        }
                    }
                }
            }

            // If we have a target sprout, use A* to find path
            if let Some(target) = self.target_sprout {
                if self.path.is_empty() {
                    if let Some(path) = astar(
                        Point::new(self.x, self.y),
                        target,
                        width as i32,
                        height as i32,
                    ) {
                        self.path = path;
                    }
                }

                // Follow path
                if let Some(next_point) = self.path.get(1) {
                    self.x = next_point.x;
                    self.y = next_point.y;
                    self.path.remove(0);

                    // Check if we reached the sprout
                    if self.x == target.x && self.y == target.y {
                        self.target_sprout = None;
                        self.path.clear();
                        
                        // Spawn new rabbit immediately after eating a sprout
                        let mut rng = rand::thread_rng();
                        return Some(Animal {
                            x: self.x + rng.gen_range(-2..=2),
                            y: self.y + rng.gen_range(-2..=2),
                            is_rabbit: true,
                            animation_frame: 0,
                            collected_nuts: Vec::new(),
                            target_nut: None,
                            move_timer: 0.0,
                            nuts_collected: 0,
                            path: Vec::new(),
                            target_sprout: None,
                        });
                    }
                }
            } else {
                // Random movement when no sprouts are available
                let mut rng = rand::thread_rng();
                self.x += rng.gen_range(-1..=1);
                self.y += rng.gen_range(-1..=1);
                self.x = self.x.clamp(0, width as i32 - 1);
                self.y = self.y.clamp(0, height as i32 - 1);
            }
        } else {
            // Original fox behavior
            if self.target_nut.is_none() {
                let mut nearest_dist = f32::MAX;
                for seed in seeds {
                    if !seed.is_planted {
                        let dx = seed.x - self.x;
                        let dy = seed.y - self.y;
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        if dist < nearest_dist {
                            nearest_dist = dist;
                            self.target_nut = Some((seed.x, seed.y));
                        }
                    }
                }
            }

            if let Some((target_x, target_y)) = self.target_nut {
                let dx = (target_x - self.x).signum();
                let dy = (target_y - self.y).signum();
                
                self.x += dx;
                self.y += dy;

                self.x = self.x.clamp(0, width as i32 - 1);
                self.y = self.y.clamp(0, height as i32 - 1);

                if self.x == target_x && self.y == target_y {
                    self.collected_nuts.push((target_x, target_y));
                    self.target_nut = None;
                }
            } else {
                let mut rng = rand::thread_rng();
                self.x += rng.gen_range(-1..=1);
                self.y += rng.gen_range(-1..=1);
                self.x = self.x.clamp(0, width as i32 - 1);
                self.y = self.y.clamp(0, height as i32 - 1);
            }
        }

        // Update animation frame
        self.animation_frame = (self.animation_frame + 1) % 4;

        // Move collected nuts
        for i in (1..self.collected_nuts.len()).rev() {
            self.collected_nuts[i] = self.collected_nuts[i - 1];
        }
        if !self.collected_nuts.is_empty() {
            self.collected_nuts[0] = (self.x, self.y);
        }

        None
    }

    fn draw(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        // Draw collected nuts trail
        for (i, &(x, y)) in self.collected_nuts.iter().enumerate() {
            let color = if i == 0 {
                Color::Rgb { r: 139, g: 69, b: 19 } // Darker brown for first nut
            } else {
                Color::Rgb { r: 160, g: 82, b: 45 } // Lighter brown for trail
            };
            queue!(
                stdout,
                MoveTo(x as u16, y as u16),
                SetForegroundColor(color),
                Print('🌰')
            )?;
        }

        // Draw animal
        let animal_char = if self.is_rabbit {
            match self.animation_frame {
                0 => '🐰',
                1 => '🐇',
                2 => '🐰',
                _ => '🐇',
            }
        } else {
            match self.animation_frame {
                0 => '🦊',
                1 => '🦊',
                2 => '🦊',
                _ => '🦊',
            }
        };

        queue!(
            stdout,
            MoveTo(self.x as u16, self.y as u16),
            SetForegroundColor(if self.is_rabbit { Color::White } else { Color::Rgb { r: 139, g: 69, b: 19 } }),
            Print(animal_char)
        )?;

        Ok(())
    }
}

fn draw_tree(tree: &Tree, stdout: &mut io::Stdout) -> io::Result<()> {
    let base_x = tree.x as u16;
    let base_y = tree.y as u16;
    
    // Draw each branch
    for &(dx, dy) in &tree.branches {
        let x = (base_x as i32 + dx) as u16;
        let y = (base_y as i32 + dy) as u16;
        
        let symbol = if dx == 0 {
            '|' // Trunk
        } else if dy == -(tree.growth_stage as i32) {
            '^' // Top
        } else {
            '/' // Branches
        };
        
        queue!(
            stdout,
            MoveTo(x, y),
            SetForegroundColor(tree.color),
            Print(symbol),
            ResetColor
        )?;
    }
    
    Ok(())
}

fn draw_seed(seed: &Seed, stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(
        stdout,
        MoveTo(seed.x.clamp(0, 18) as u16, seed.y.clamp(0, 6) as u16),
        SetForegroundColor(Color::Yellow),
        Print('*')
    )
}

fn draw_animal(animal: &Animal, stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(
        stdout,
        MoveTo(animal.x.clamp(0, 18) as u16, animal.y.clamp(0, 6) as u16),
        SetForegroundColor(if animal.is_rabbit { Color::White } else { Color::Rgb { r: 139, g: 69, b: 19 } }),
        Print(if animal.is_rabbit { 'r' } else { 's' })
    )
}

pub fn animate_moose_quote(duration: u64, quiet_mode: bool, verbose_mode: bool) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (width, height) = terminal::size()?;

    // Initialize game state
    let mut trees = Vec::new();
    let mut seeds = Vec::new();
    let mut animals = Vec::new();
    let mut moose = Moose::new(width, height);

    // Create initial trees
    for _ in 0..5 {
        trees.push(Tree::new(width, height));
    }

    // Create initial animals
    for _ in 0..3 {
        animals.push(Animal::new(width, height, true));
    }

    // Create initial seeds
    for _ in 0..10 {
        seeds.push(Seed::new(width, height));
    }

    // Get start time
    let start = Instant::now();

    // Clear screen and hide cursor
    queue!(stdout, terminal::Clear(terminal::ClearType::All), Hide)?;
    stdout.flush()?;

    // Main animation loop
    while start.elapsed() < Duration::from_secs(duration) {
        // Check for input
        if poll(Duration::from_millis(100))? {
            if let Ok(Event::Key(event)) = read() {
                match event.code {
                    KeyCode::Esc | KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Char(c) => {
                        moose.handle_input(c);
                    }
                    KeyCode::Backspace => {
                        moose.handle_input('\x08');
                    }
                    _ => {}
                }
            }
        }

        // Clear screen
        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

        // Update and draw game state
        update_and_draw_trees(&mut stdout, &mut trees, &mut seeds, width, height)?;
        update_and_draw_animals(&mut stdout, &mut animals, &seeds, width, height)?;
        
        // Draw quote before moose so moose appears in front
        if !quiet_mode {
            if let Some(quote) = &moose.current_quote {
                draw_quote(&mut stdout, quote, &moose.typed_text, width, height, &moose)?;
            }
        }
        
        update_and_draw_moose(&mut stdout, &mut moose, &mut trees, width, height)?;

        // Flush output
        stdout.flush()?;

        // Sleep for a bit
        thread::sleep(Duration::from_millis(100));
    }

    // Clear screen and show cursor
    queue!(stdout, terminal::Clear(terminal::ClearType::All), Show)?;
    stdout.flush()?;

    Ok(())
}

fn print_final_screen(stdout: &mut impl Write, trees: &[Tree], seeds: &[Seed], animals: &[Animal], moose: &Moose, width: u16, height: u16) -> io::Result<()> {
    // Create a 2D buffer to store the final screen
    let mut screen = vec![vec![' '; width as usize]; height as usize];

    // Draw trees
    for tree in trees {
        let x = tree.x.clamp(0, width as i32 - 1) as usize;
        let y = tree.y.clamp(0, height as i32 - 1) as usize;
        if x < width as usize && y < height as usize {
            screen[y][x] = '🌲';
        }
    }

    // Draw seeds
    for seed in seeds {
        let x = seed.x.clamp(0, width as i32 - 1) as usize;
        let y = seed.y.clamp(0, height as i32 - 1) as usize;
        if x < width as usize && y < height as usize {
            screen[y][x] = '.';
        }
    }

    // Draw animals
    for animal in animals {
        let x = animal.x.clamp(0, width as i32 - 1) as usize;
        let y = animal.y.clamp(0, height as i32 - 1) as usize;
        if x < width as usize && y < height as usize {
            screen[y][x] = if animal.is_rabbit { '🐰' } else { '🦊' };
        }
    }

    // Draw moose
    let moose_x = moose.x.clamp(0, width as i32 - 1) as usize;
    let moose_y = moose.y.clamp(0, height as i32 - 1) as usize;
    if moose_x < width as usize && moose_y < height as usize {
        screen[moose_y][moose_x] = '🦌';
    }

    // Draw speech bubble if present
    if let Some(text) = &moose.speech_bubble {
        let bubble_x = moose_x;
        let bubble_y = moose_y.saturating_sub(2);
        if bubble_y < height as usize {
            let text = format!("💭 {}", text);
            for (i, c) in text.chars().enumerate() {
                if bubble_x + i < width as usize {
                    screen[bubble_y][bubble_x + i] = c;
                }
            }
        }
    }

    // Print the final screen
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    for row in screen {
        queue!(stdout, Print(row.iter().collect::<String>()), Print("\n"))?;
    }
    stdout.flush()?;

    Ok(())
}

// Add helper functions for updating and drawing elements
fn update_and_draw_trees(stdout: &mut impl Write, trees: &mut Vec<Tree>, seeds: &mut Vec<Seed>, width: u16, height: u16) -> io::Result<()> {
    // Update and draw trees
    for tree in trees.iter_mut() {
        tree.update(width, height);
        tree.draw(stdout, width, height)?;
    }

    // Update and draw seeds
    let mut i = 0;
    while i < seeds.len() {
        let seed = &mut seeds[i];
        seed.update(width as u16, height as u16);
        seed.draw(stdout)?;
        i += 1;
    }

    Ok(())
}

fn update_and_draw_animals(stdout: &mut impl Write, animals: &mut Vec<Animal>, seeds: &[Seed], width: u16, height: u16) -> io::Result<()> {
    let mut new_animals = Vec::new();
    
    for animal in animals.iter_mut() {
        if let Some(new_rabbit) = animal.update(width, height, seeds) {
            new_animals.push(new_rabbit);
        }
        animal.draw(stdout)?;
    }

    // Add new rabbits to the population
    animals.extend(new_animals);
    
    Ok(())
}

fn update_and_draw_moose(stdout: &mut impl Write, moose: &mut Moose, trees: &mut Vec<Tree>, width: u16, height: u16) -> io::Result<()> {
    moose.update(width, height, trees);

    // Check for collisions with trees
    for tree in trees.iter_mut() {
        // Calculate distance between moose and tree
        let dx = moose.x - tree.x;
        let dy = moose.y - tree.y;
        let distance = ((dx * dx + dy * dy) as f32).sqrt();

        // If moose is within 4 units of a tree (creating a 9x9 killbox) and the tree isn't already dead
        if distance < 4.5 && !tree.is_dead {
            // Kill the tree
            tree.is_dead = true;
            tree.is_falling = true;
            tree.fall_progress = 0.0;
            tree.color = Color::DarkGrey;
            tree.is_stomping = true;
            tree.event_timer = 20.0;
            
            // Make the moose say something about stomping the tree
            let stomp_quotes = [
                "Take that, tree!",
                "Timber!",
                "Sorry, not sorry!",
                "That's what you get for being in my way!",
                "Oops, did I do that?",
                "Another one bites the dust!",
                "I am the destroyer of forests!",
                "Moose: 1, Tree: 0",
            ];
            let quote = stomp_quotes[rand::thread_rng().gen_range(0..stomp_quotes.len())];
            moose.speech_bubble = Some(quote.to_string());
            moose.speech_timer = 50.0;
        }
    }

    let moose_art = moose.draw();
    for (i, line) in moose_art.iter().enumerate() {
        let y = (moose.y - 4 + i as i32).clamp(0, height as i32 - 1);
        if y >= 0 && y < height as i32 {
            queue!(
                stdout,
                MoveTo(moose.x.clamp(0, width as i32 - 1) as u16, y as u16),
                Print(line)
            )?;
        }
    }
    Ok(())
}

fn draw_quote(stdout: &mut impl Write, quote: &str, typed_text: &str, width: u16, height: u16, moose: &Moose) -> io::Result<()> {
    // Calculate bubble dimensions
    let quote_width = quote.len() + 4;
    let bubble_height = 7; // Total height including borders and connector
    
    // Ensure we have enough space
    if width < quote_width as u16 + 2 || height < bubble_height {
        return Ok(());
    }
    
    // Calculate safe bounds for bubble position
    let max_x = width as i32 - quote_width as i32;
    let min_x = 0;
    
    // Position bubble higher above moose (10 lines above the ASCII art top)
    let mut bubble_y = moose.y - 14;
    
    // Wrap bubble_y to bottom of screen if it goes above top
    if bubble_y < 0 {
        bubble_y = height as i32 - bubble_height as i32;
    }
    
    // Center bubble horizontally relative to moose with wrapping
    let mut bubble_x = moose.x - (quote_width as i32 / 2);
    if bubble_x < min_x {
        bubble_x = max_x - (min_x - bubble_x);
    } else if bubble_x > max_x {
        bubble_x = min_x + (bubble_x - max_x);
    }
    
    // Create bubble borders
    let top_border = format!(" {} ", "_".repeat(quote_width));
    let bottom_border = format!(" {} ", "‾".repeat(quote_width));
    
    // Draw top of bubble
    queue!(
        stdout,
        MoveTo(bubble_x as u16, bubble_y as u16),
        Print(&top_border)
    )?;
    
    // Draw middle of bubble with quote
    queue!(
        stdout,
        MoveTo(bubble_x as u16, bubble_y as u16 + 1),
        Print(format!("/ {} \\", " ".repeat(quote_width))),
        MoveTo(bubble_x as u16, bubble_y as u16 + 2),
        Print(format!("| {} |", quote)),
        MoveTo(bubble_x as u16, bubble_y as u16 + 3),
        Print(format!("\\ {} /", " ".repeat(quote_width)))
    )?;
    
    // Draw bottom of bubble
    queue!(
        stdout,
        MoveTo(bubble_x as u16, bubble_y as u16 + 4),
        Print(&bottom_border)
    )?;
    
    // Draw typing input at bottom of screen
    let typing_y = height - 2;
    queue!(
        stdout,
        MoveTo(0, typing_y),
        Print(format!("Type this: {}", typed_text))
    )?;
    
    // Draw connector
    let connector_start_y = bubble_y + 5;
    let mut connector_end_y = moose.y - 4;
    let mut connector_length = connector_end_y - connector_start_y;
    
    if connector_length < 0 {
        connector_length = (height as i32 - connector_start_y) + connector_end_y;
    }
    
    let mut current_y = connector_start_y;
    for _ in 0..connector_length {
        let wrapped_y = current_y % height as i32;
        queue!(
            stdout,
            MoveTo(bubble_x as u16 + quote_width as u16 / 2, wrapped_y as u16),
            Print("|")
        )?;
        current_y += 1;
    }
    
    queue!(
        stdout,
        MoveTo(bubble_x as u16 + quote_width as u16 / 2, (moose.y - 5).rem_euclid(height as i32) as u16),
        Print("\\"),
        MoveTo(bubble_x as u16 + quote_width as u16 / 2 + 1, (moose.y - 4).rem_euclid(height as i32) as u16),
        Print("\\")
    )?;
    
    Ok(())
}

fn astar(start: Point, goal: Point, width: i32, height: i32) -> Option<Vec<Point>> {
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use std::cmp::Ordering;

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: i32,
        position: Point,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut closed_set = HashSet::new();

    g_score.insert(start, 0);
    open_set.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost: _, position }) = open_set.pop() {
        if position == goal {
            let mut path = Vec::new();
            let mut current = position;
            while current != start {
                path.push(current);
                current = came_from[&current];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        if closed_set.contains(&position) {
            continue;
        }
        closed_set.insert(position);

        // Check all 8 adjacent cells
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let next = Point::new(
                    (position.x + dx).rem_euclid(width),
                    (position.y + dy).rem_euclid(height),
                );

                let tentative_g_score = g_score[&position] + 1;

                if !g_score.contains_key(&next) || tentative_g_score < g_score[&next] {
                    came_from.insert(next, position);
                    g_score.insert(next, tentative_g_score);
                    let f_score = tentative_g_score + (next.distance(&goal) * 10.0) as i32;
                    open_set.push(State {
                        cost: f_score,
                        position: next,
                    });
                }
            }
        }
    }

    None
} 