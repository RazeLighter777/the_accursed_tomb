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
    #[test]
    pub fn print_out_emoji_table() {
        let mut table = Table::new();
        for i in 0..7 {
            for j in 0..7 {
                let card = table.draw_or_view_card_at(i, j);
                if let Some(card) = card {
                    print!("{}\t", card.get_card_meaning().emoji());
                } else {
                    print!("{} ", "🂠");
                }
            }
            println!();
        }
    }
}