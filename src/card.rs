//! Traits for card types.

use crate::State;

/// Card type that can be judged.
pub trait Judge<Role, Deck>: Sized {
    /// Command type that is returned when the card is judged.
    type Command;

    /// Symbol type that is used to judge the card.
    type Symbol;

    /// Judge the card.
    fn judge(self, symbol: Self::Symbol, state: &State<Role, Self, Deck>) -> Option<Self::Command>;
}

/// Card type that can modify health attributes.
pub trait HealthModifier {
    /// Get max health modifier of the player, based on the default health.
    #[inline]
    fn max_health_modifier(&self, default: u16) -> i16 {
        0
    }
}
