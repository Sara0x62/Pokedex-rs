// Local imports
use pokedex::menus;
use pokedex::structs::Pokemon;

fn read_csv_file(file: &str) -> Result<Vec<Pokemon>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(file)?;
    let mut pokemons: Vec<Pokemon> = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(record) => {
                pokemons.push(record);
            },
            Err(e) => panic!("Something went wrong reading the data!\nErr: {}", e),
        }
    }

    Ok(pokemons)
}


fn main() {
    println!("Pokedex starting...");
    println!("Loading data...");

    let data_file = "data/pokedex.csv";
    let pokemons = match read_csv_file(data_file) {
        Ok(pokemons) => pokemons,
        Err(e) => panic!("Something went wrong reading the data!\nErr: {}", e),
    };

    println!("Pokedex loaded! - There are {} Pokémon(s) in our database!", pokemons.len());
    println!();

    menus::main_menu(&pokemons);
}
