use super::structs::Pokemon;
use super::statics;
use inquire::Select;

pub fn main_menu(pokemons: &Vec<Pokemon>) {
    loop {
        let options: Vec<&str> = vec![
            "List Legendary pokemons",
            "List by type",
            "List by Generation",
            "Search all",
            "Exit"
            ];
        let message = "Select an option.";

        println!("========================");

        if let Ok(choice) = Select::new(message, options).prompt() {
            match choice {
                "List Legendary pokemons" => list_legendary_pokemons(pokemons),
                "List by type" => list_by_type(pokemons),
                "List by Generation" => list_by_generation(pokemons),
                "Search all" => search_all(pokemons),
                "Exit" => break,
                &_ => println!("Invalid option!"),
            }
        } else {
            println!("Error ");
        }

    }
}

fn list_legendary_pokemons(pokemons: &Vec<Pokemon>) {
    println!("Legendary pokemons:");

    let mut options: Vec<&Pokemon> = Vec::new();

    for pokemon in pokemons {
        if pokemon.legendary {
            options.push(pokemon);
        }
    }

    let selected = Select::new("Legendary pokemons", options)
        .prompt();

    match selected {
        Ok(choice) => selected_pokemon_info(choice),
        Err(_) => println!("Error"),
    }
}


fn list_by_type(pokemons: &Vec<Pokemon>) {
    let selected_type = Select::new("Type", statics::POKEMON_TYPES.to_vec()).prompt();
    let mut selected_pokemons: Vec<&Pokemon> = Vec::new();

    match selected_type {
        Ok(choice) => {
            for pokemon in pokemons {
                if pokemon.type_1.contains(choice) || pokemon.type_2.contains(choice) {
                    selected_pokemons.push(pokemon);
                }
            }
            list_by(format!("All pokemons of type: {}", choice), selected_pokemons)
        },
        Err(_) => println!("Error"),
    }
}

fn list_by_generation(pokemons: &Vec<Pokemon>) {
    let selected_generation = Select::new("Generation", statics::POKEMON_GENERATIONS.to_vec()).prompt();
    let mut selectable: Vec<&Pokemon> = Vec::new();

    match selected_generation {
        Ok(choice) => {
            for pokemon in pokemons {
                if pokemon.generation == choice {
                    selectable.push(pokemon);
                }
            }
            list_by(format!("All pokemons from generation: {}", choice), selectable);
        }
        Err(_) => println!("Error"),
    }
}

fn list_by(msg: String, selectable: Vec<&Pokemon>) {
    let selected = Select::new(&msg, selectable).prompt();
    match selected {
        Ok(choice) => selected_pokemon_info(&choice),
        Err(_) => println!("Error"),
    }
}

fn search_all(pokemons: &Vec<Pokemon>) {
    // I doubt .to_vec() is the best but idk how else to do this rn
    let select = Select::new("Pokemon list", pokemons.to_vec()).prompt();

    match select {
        Ok(choice) => selected_pokemon_info(&choice),
        Err(_) => println!("Error"),
    }
}

fn selected_pokemon_info(poke: &Pokemon) {
    println!("Detailed info...");

    println!(
        "{} | {}\n\tType(s): {} {}\n\tTotal: {}",
        poke.id,
        poke.name,
        poke.type_1,
        poke.type_2,
        poke.total,
    );
        
    println!("\t\tAttack: {} | Special Attack: {}\n\t\tDefense: {} | Special Defense: {}\n\t\tSpeed: {}",
        poke.attack,
        poke.sp_attack,
        poke.defense,
        poke.sp_defense,
        poke.speed,
    );

    println!("\tGeneration: {}\n\tLegendary: {}", 
        poke.generation,
        poke.legendary
    );
}
