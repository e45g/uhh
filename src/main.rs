mod prompts;
use prompts::fuzzy_select::FuzzySelect;
use prompts::input::Input;

fn main() {
    let mut items: Vec<&str> = vec![
        "The Shawshank Redemption",
        "The Godfather",
        "The Dark Knight",
        "Pulp Fiction",
        "The Lord of the Rings: The Return of the King",
        "Forrest Gump",
        "Inception",
        "The Matrix",
        "Fight Club",
        "The Empire Strikes Back",
        "The Silence of the Lambs",
        "Schindler's List",
        "The Godfather Part II",
        "Interstellar",
        "Gladiator",
        "The Lion King",
        "Jurassic Park",
        "The Green Mile",
        "Braveheart",
        "Saving Private Ryan",
        "Back to the Future",
        "Se7en",
        "The Prestige",
        "Avatar",
        "The Departed",
        "Titanic",
        "The Avengers",
        "Django Unchained",
        "The Pianist",
        "Whiplash",
        "The Social Network",
        "Parasite",
        "Inglourious Basterds",
        "The Wolf of Wall Street",
        "Toy Story",
        "Joker",
        "The Sixth Sense",
        "Black Panther",
        "Mad Max: Fury Road",
        "No Country for Old Men",
        "Spirited Away",
        "The Truman Show",
        "The Usual Suspects",
        "Goodfellas",
        "American Beauty",
        "Blade Runner",
        "The Shining",
        "Casablanca",
        "The Great Gatsby",
        "The Grand Budapest Hotel",
        "Her"
    ]; 
    items.push("New Task");
    let selection = FuzzySelect::new()
        .items(&items)
        .interact()
        .unwrap()
        .unwrap();

    if items[selection] == "New Task" {
        let name = Input::new()
            .prompt("Task Name")
            .allow_empty(false)
            .interact()
            .unwrap();


        println!("{}", name);
    }

}

