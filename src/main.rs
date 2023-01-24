mod card_deck;
mod table;
mod player;
mod dice;


pub struct Game {
    table : table::Table,
    players : Vec<player::Player>,
    dice : dice::Dice,
}


impl Game {
    pub fn new() -> Game {
        Game {
            table : table::Table::new(),
            players : vec![player::Player::new("Vormithrax".to_string()), player::Player::new("Biblo".to_string()), player::Player::new("Gandalf".to_string()), player::Player::new("Sauron".to_string())],
            dice : dice::Dice::new(6),
        }
    }
    pub fn get_game_board_string(&self) -> String {
        let mut game_board_string = String::new();
        game_board_string.push_str(&self.table.get_emoji_table_string());
        game_board_string.push_str(&player::get_comma_seperated_player_names(&self.players));
        game_board_string
    }
}
fn main() {
    /**
     * Your game simulation will include a menu that initially has three commands:

SHOW GAME AREA - displays everything necessary to observe and understand the progress of the game (e.g., a text-based representation of the game board, player hands, the score board, etc.)
SHOW PLAYER ORDER - shows player names in the order they will play
ADVANCE ORDER - moves the player who is currently "up" to the end of the playing order, and advances the "next" player to be "up"

     */
    let mut game = Game::new();
    loop {
        println!("Please enter a command: ");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim();
        match command {
            "SHOW GAME AREA" => println!("{}", game.get_game_board_string()),
            "SHOW PLAYER ORDER" => println!("{}", player::get_comma_seperated_player_names(&game.players)),
            "ADVANCE ORDER" => {
                game.players.rotate_left(1);
                println!("Player order has been advanced");
            },
            _ => println!("Invalid command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn game_board_string_test() {
        let game_board_expected = "++++++++\n++++++++\n++++++++\n++++++++\n++++++++\n++++++++\n++++++++\n++++++++\nVormithrax, Biblo, Gandalf, Sauron";
        let game = Game::new();
        assert_eq!(game.get_game_board_string(), game_board_expected);
    }
    #[test]
    fn show_player_order_test() {
        let game = Game::new();
        let expected_player_order = "Vormithrax, Biblo, Gandalf, Sauron";
        assert_eq!(player::get_comma_seperated_player_names(&game.players), expected_player_order);
    }
    #[test]
    fn rotate_player_order_test() {
        let mut game = Game::new();
        let expected_player_order = "Biblo, Gandalf, Sauron, Vormithrax";
        game.players.rotate_left(1);
        assert_eq!(player::get_comma_seperated_player_names(&game.players), expected_player_order);
    }
}