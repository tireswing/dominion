//! Cards from the Cornucopia set

use std::collections::HashSet;

use super::prelude::*;

/// [Wiki link](http://wiki.dominionstrategy.com/index.php/Fairgrounds)
#[derive(Clone, Serialize, Deserialize)]
pub struct Fairgrounds;

#[typetag::serde]
impl Card for Fairgrounds {
    name!("Fairgrounds");
    cost!(6);
    types!(vec![Victory]);
    fn victory_points(&self, player: &Player) -> isize {
        let mut names = HashSet::new();
        for card in &player.hand {
            names.insert(card.name());
        }
        names.len() as isize / 5
    }
}
