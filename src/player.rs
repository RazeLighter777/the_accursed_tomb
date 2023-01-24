pub struct Player {
    name : String,
}

impl Player {
    pub fn new(name : String) -> Player {
        Player {
            name,
        }
    }
}

pub fn get_comma_seperated_player_names(players : &Vec<Player>) -> String {
    let mut player_names = String::new();
    for player in players {
        player_names.push_str(&player.name);
        player_names.push_str(", ");
    }
    player_names.pop();
    player_names.pop();
    player_names
}

pub fn advance_player_order(players : &mut Vec<Player>) {
    let first_player = players.remove(0);
    players.push(first_player);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn player_test() {
        let player = Player::new("Vormithrax".to_string());
        assert_eq!(player.name, "Vormithrax");
    }
    #[test]
    fn get_comma_seperated_player_names_test() {
        
    }
    #[test]
    fn advance_player_order_test() {
        let mut players = vec![Player::new("Vormithrax".to_string()), Player::new("Biblo".to_string()), Player::new("Gandalf".to_string()), Player::new("Sauron".to_string())];
        advance_player_order(&mut players);
        assert_eq!(players[0].name, "Biblo");
        assert_eq!(players[1].name, "Gandalf");
        assert_eq!(players[2].name, "Sauron");
        assert_eq!(players[3].name, "Vormithrax");
    }
}