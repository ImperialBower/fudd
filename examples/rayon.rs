use fudd::games::holdem::case_evals::CaseEvals;
use fudd::games::holdem::table::Table;
use fudd::types::card_slot::CardSlot;
use fudd::types::playing_cards::PlayingCards;

fn main() {
    let player_count: usize = 2;
    let table = Table::seat(player_count);
    let index = "Q♠ 4♦ 8♥ 5♦ 2♠ 4♠ J♦ 7♠ 9♣ 8♠ 6♦ 2♦ J♥ Q♥ 6♠ 4♥ T♣ 4♣ 6♣ 8♦ A♣ 3♣ A♥ 8♣ J♠ 9♠ 7♣ T♠ Q♣ T♦ 9♦ 5♠ 7♥ 2♣ 3♥ 5♣ 7♦ 3♦ 3♠ Q♦ K♥ A♠ K♦ 9♥ K♠ 5♥ K♣ 2♥ T♥ 6♥ A♦ J♣";
    let mut cards = PlayingCards::try_from(index).unwrap();

    for _ in 0..(player_count * 2) + 5 {
        table.take(cards.draw_one());
    }

    let mut _evals = CaseEvals::default();
    let _combos = table.remaining_at_deal().combinations(5);
}
