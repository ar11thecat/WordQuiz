use crate::types::{Phase, Command, GameState};
use crate::filtered_io;
use crate::image_viewer;

use std::env;
use std::fs;
use rand::rng;
use rand::seq::SliceRandom;

fn process_input(prompt: &str) -> Result<String, Phase> {
    
    filtered_io::question(prompt);
    
    match filtered_io::filtered_input() {
        Command::ChangePhase(input) => Err(
            match input.as_str() {
                "-help" => {
                    help();
                    return process_input(prompt)
                },
                "-skip" => Phase::Skip,
                "-quit" => Phase::Quit,
                _ => unreachable!(),
            }
        ),
        Command::Normal(input) => Ok(input),
    }
}

pub fn help() -> () {

    let path = env::current_dir().unwrap();
    
    filtered_io::print_cyan(&format!("\n\
        \x20Commands:\n\
        \x20'-help' : show help\n\
        \x20'-skip' : skip to the next part\n\
        \x20'-quit' : to quit the game\n\
        \n\
        \x20File format:\n\
        \x20[original] = [translation] or\n\
        \x20[original] = [translation] = [score]\n\
        \n\
        \x20Make sure the file path is relative to the current directory:\n\
        \x20{}\n",
        path.display())
     );
}

pub fn selection(state: &mut GameState) -> Phase {

    // select file
    loop {
        
        state.filepath = match process_input("Enter path to the file to practice:") {
            Ok(input) => input,
            Err(phase) => match phase {
                Phase::Skip => break,
                _ => return phase,
            },
        };

        // load lines from file
        match fs::read_to_string(&state.filepath) {
            Ok(lines) => {

                // empty existing translations
                state.translations.clear();
            
                // load lines to translations
                for line in lines.lines() {
                    let parts: Vec<&str> = line.split('=').collect();

                    if parts.len() >= 2 {
                        let og: String = parts[0].trim().to_string();
                        let trans: String = parts[1].trim().to_string();
                        let word_score = parts.get(2)
                            .and_then(|p| p.trim().parse::<f64>().ok())
                            .unwrap_or(2.0);

                        state.translations.push((og, trans, word_score));
                    }
                }
            },

            Err(e) => {
                filtered_io::warning(&format!("Oops! Can't find the file '{}': {}", &state.filepath, e));
                continue
            },
        };

        if state.translations.is_empty() {
            filtered_io::warning(&format!("Oops! The file '{}' is empty and/or a bad format", &state.filepath));
            continue
        } else {
            break
        }
    }

    // select mode
    loop {
        match process_input("Ask the original or the translation?") {
            Ok(mode) => match mode.as_str() {
                "original" | "translation" => {
                    state.settings.0 = mode;
                    break
                },
                _ => {
                    filtered_io::warning(&format!("Oops! '{}' is not an option", mode));
                    continue
                },
            },
            Err(phase) => match phase {
                Phase::Skip => break,
                _ => return phase,
            },
        }
    }

    // select order
    loop {
        match process_input("Sort with shuffle, by alphabet or by difficulty?") {
            Ok(order) => match order.as_str() {
                "shuffle" | "alphabet" | "difficulty" => {
                    state.settings.1 = order;
                    break
                },
                _ => {
                    filtered_io::warning(&format!("Oops! '{}' is not a valid option", order));
                    continue
                },
            },
            Err(phase) => match phase {
                Phase::Skip => break,
                _ => return phase,
            },
        }
    }
            
    return Phase::Game
}

pub fn game(state: &mut GameState) -> Phase {

    state.results.0 = 0;
    state.results.1 = 0;

    // sort according to order
    let mut indices: Vec<usize> = (0..state.translations.len()).collect();
    
    match state.settings.1.as_str() {
        "shuffle" => {
            let mut rng = rng();
            indices.shuffle(&mut rng);
        },
        "alphabet" => {
            indices.sort_by(|&a, &b| state.translations[a].0.to_lowercase()
                .cmp(&state.translations[b].0.to_lowercase()));
        },
        "difficulty" => {
            indices.sort_by(|&a, &b| state.translations[a].2
                .partial_cmp(&state.translations[b].2).unwrap());
        },
        _ => {
            filtered_io::warning("Oops! It looks like some essential settings are missing!");
            match process_input("Press any key to go back to SELECTION") {
                Ok(_input) => return Phase::Selection,
                Err(phase) => match phase {
                    Phase::Skip => (),
                    _ => return phase,
                },
            };
        },
    }

    // game loop
    for &i in &indices {
        let (og, trans, word_score): &mut (String, String, f64) = &mut state.translations[i];
        
        let (question, answer): (&str, &str) = match state.settings.0.as_str() {
            "original" => (og, trans),
            "translation" => (trans, og),
            _ => {
                filtered_io::warning("Oops! It looks like some essential settings are missing!");
                match process_input("Press any key to go back to SELECTION") {
                    Ok(_input) => return Phase::Selection,
                    Err(phase) => match phase {
                        Phase::Skip => break,
                        _ => return phase,
                    },
                };
            },
        };

        state.results.0 += 1;

        match process_input(&format!("{}:", question)) {
            Ok(input) => {
                
                let answer_vec: Vec<&str> = answer.split(", ").collect();
                if answer_vec.contains(&input.as_str()) {
                    
                    filtered_io::easy_print(" Nice job!");
                    *word_score += 2.0 / (*word_score + 1.0);
                    state.results.1 += 1;
                    
                    if answer_vec.len() > 1 {
                        println!("\x20All answers: {}", answer);
                    } else {
                        println!();
                    }
                    
                } else {
                    
                    match image_viewer::show_image(state.resources.fail(), "meow meow ", 3.0, 1) {
                        Ok(()) => (),
                        Err(e) => {
                            filtered_io::warning(&format!("Failed to show the fail reaction: {}", e));
                        }
                    }
                    println!("\x20The answer should be: {}", answer);
                    *word_score = (*word_score + 1.0) / 2.0;
                }
            },
            Err(phase) => match phase {
                Phase::Skip => break,
                _ => return phase,
            },
        };
    }

    return Phase::Results
}

pub fn results(state: &GameState) -> Phase {

    println!("\
        \x20Words tested: {}\n\
        \x20Words guessed correctly: {}\n\
        ", state.results.0, state.results.1);

    // react
    let guessed_ratio: i32 = state.results.1 * 100 / state.results.0;
    if guessed_ratio > 70 {
        match image_viewer::show_image(state.resources.cheer(), "meow meow ", 1.2, 2) {
            Ok(()) => (),
            Err(e) => {
                filtered_io::warning(&format!("Failed to show the fail reaction: {}", e));
            },
        }
    } else if guessed_ratio < 30 {
        match image_viewer::show_image(state.resources.cringe(), "meow meow ", 0.8, 2) {
            Ok(()) => (),
            Err(e) => {
                filtered_io::warning(&format!("Failed to show the fail reaction: {}", e));
            },
        }
    } else {
        match image_viewer::show_image(state.resources.mid(), "meow meow ", 2.0, 2) {
            Ok(()) => (),
            Err(e) => {
                filtered_io::warning(&format!("Failed to show the fail reaction: {}", e));
            },
        }
    }
           
    // save difficulty
    match process_input("Save the updates to difficulty? [yes/else]") {
        Ok(input) => match input.as_str() {
            "yes" => {
                let mut lines: Vec<String> = Vec::new();
                for (og, trans, word_score) in &state.translations {
                    lines.push(format!("{} = {} = {}", og, trans, word_score.to_string()));
                }
    
                match fs::write(&state.filepath, lines.join("\n")) {
                    Ok(()) => (),
                    Err(e) => {
                        filtered_io::warning(&format!("Oops! Can't write to the file '{}': {}", &state.filepath, e));
                    },
                };
            },
            _ => (),
        },
        Err(phase) => match phase {
            Phase::Skip => (),
            _ => return phase,
        },
    };

    // Ask to repeat with same settings
    match process_input("Play another game with the same settings? [yes/else]") {
        Ok(input) => match input.as_str() {
            "yes" => return Phase::Game,
            _ => return Phase::Selection,
        },
        Err(phase) => match phase {
            Phase::Skip => (),
            _ => return phase,
        }
    }
    
    return Phase::Selection
}
