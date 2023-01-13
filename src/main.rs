
/**
 * This method adds two to the number given.
 * @param a The number to which two will be added.
 * @return The result of the addition.
 */
mod card_deck;
mod table;
use egui;
mod dice;
pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn main() {
    let ctx = egui::Context::default();
    let input = egui::RawInput::default();
    ctx.begin_frame(input);
    let ui = egui::CentralPanel::default().show(&ctx, |ui| {
        ui.label("Hello World!");
    });

}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test] 
    fn add_two_test() {
        assert_eq!(4, add_two(2))
    }
}