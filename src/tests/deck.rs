use crate::deck::{document::Document, Content, Deck};

#[test]
fn closure_gen() {
    let deck = Deck::from(Document::parse("<12345678 (A)[a] | (B)[b]>\n").unwrap());
    assert_eq!(
        deck.cards().get("12345678").unwrap().rems(),
        &vec![(
            0,
            vec![
                Content::Closure(("12345678".into(), "A".into()), "a".into()),
                Content::Text(" | ".into()),
                Content::Closure(("12345678".into(), "B".into()), "b".into())
            ]
        )]
    )
}
