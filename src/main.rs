#[allow(unused_mut, unused_imports, dead_code)]
use tcod::colors::*;
use tcod::console::*;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 60; // normally this is set to 20 FPS max, but we want 60 cuz fuck it!

struct Tcod {
    root: Root,
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
    .font("assets/arial10x10.png", FontLayout::Tcod)
    .font_type(FontType::Greyscale)
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .title("Dr460n4ir3")
    .init();

    let mut tcod = Tcod { root }; // creates a variable called tcod that we can change later

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;
    
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(LIGHT_ORANGE); // change color of '@'
        tcod.root.clear();
        tcod.root
            .put_char(player_x, player_y, '@', BackgroundFlag::None); // spawns character 40,25 coordinates in center of screen
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        
        // handle keys and exit game if needed
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}

// handles key presses for player movement
fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    // need to finish this system
    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,

        _ => {}
    }

    return false;
}

