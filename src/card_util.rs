use poker::Card;

pub fn deal_without(n: usize, dealt: &Vec<Card>) -> Vec<Card>{
    //TODO: make this actually efficient
    let mut cards = Vec::new();
    let deck = Vec::from(Card::generate_shuffled_deck());

    for card in deck {
        if !dealt.contains(&card) {
            cards.push(card); 
            if cards.len() >= n {
                break;
            }
        }
    }

    cards
}
