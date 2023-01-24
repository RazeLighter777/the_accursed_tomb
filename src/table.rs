use crate::card_deck;
pub struct Table {
    //8x8 table of cards, starts empty
    cards : [[Option<card_deck::Card>; 8]; 8],
    deck : card_deck::Deck,
}



impl Table {
    pub fn new() -> Table {
        Table {
            cards : Default::default(),
            deck : card_deck::Deck::shuffled_full_deck(),
        }
    }
    pub fn view_card_at(&self, x: usize, y: usize) -> Option<&card_deck::Card> {
        self.cards[x][y].as_ref()
    }
    pub fn draw_or_view_card_at(&mut self, x : usize, y : usize) -> Option<&card_deck::Card> {
        if let Some(card) = self.cards[x][y].clone() {
            self.cards[x][y].as_ref()
        } else {
            //draw a card and put it in the slot
            let card = self.deck.draw();
            self.cards[x][y] = card;
            self.cards[x][y].as_ref()
        }
    }
    pub fn get_emoji_table_string(&self) -> String {
        let mut table_string = String::new();
        for row in self.cards.iter() {
            for card in row.iter() {
                if let Some(card) = card {
                    table_string.push_str(&card.get_card_meaning().emoji());
                } else {
                    table_string.push_str("+");
                }
            }
            table_string.push_str("\n");
        }
        table_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn table_test() {
        let mut table = Table::new();
        let card = table.draw_or_view_card_at(0, 0);
        assert!(card.is_some());
        let card = table.draw_or_view_card_at(0, 0);
        assert!(card.is_some());
    }
}