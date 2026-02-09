mod filtered_io;
mod image_viewer;
mod phases;
mod resources;
mod types;

use resources::Resources;
use types::{Phase, GameState};

fn main() {
    println!("\nHello World!\n");

    let mut current_phase = Phase::Selection;
    
    let mut game_state = GameState{
        filepath: String::new(),
        resources: Resources::new(),
        translations: Vec::new(),
        settings: (String::new(), String::new()),
        results: (0, 0),
    };
    
    filtered_io::new_page();
    println!("\n\
        \x20 _      __            ______       _   \n\
        \x20| | /| / /__  _______/ / __ L__ __(_)__\n\
        \x20| |/ |/ / _ V  __/ _  / /_/ / // / /_ /\n\
        \x20|__/|__/L___/_/  L___/L__L__L___/_//__/\n\
        \x20                          Let's goooooo\n");
                                        

    phases::help();

    loop {
      
        match current_phase {
            
            Phase::Selection => {
                filtered_io::header("SELECTION");
                current_phase = phases::selection(
                    &mut game_state
                );
            },
            Phase::Game => {
                filtered_io::header("GAME");
                current_phase = phases::game(
                    &mut game_state
                );
            },
            Phase::Results => {
                filtered_io::header("RESULTS");
                current_phase = phases::results(
                    &game_state
                );
            },
            Phase::Quit => {
                println!("Bye!");
                break;
            },
            _ => unreachable!(),
        }
        
        filtered_io::new_page();
        
    }
    
}
