//! Traits for role types.

/// Role type that presents a character in the game.
pub trait Character {
    /// Get the maximum health of the character.
    fn max_health(&self) -> u16;
}
