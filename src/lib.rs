#![no_std]

//! # wac-icore
//!
//! This is the core library for the server implementations of the game.

extern crate alloc;

use alloc::vec::Vec;
use card::{HealthModifier, Judge};
use rand::{
    distributions::{Distribution as RandDist, Standard as RandStd},
    Rng,
};
use role::Character;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use stack::Stack;

pub mod stack;

pub mod card;
pub mod role;

/// Unique identifier of a player.
///
/// The inner number is unique for each player, and is ordered by the order
/// of card playing.
///
/// # Serde
///
/// This type can be serialized and deserialized using Serde as a unsigned 16-bit
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[repr(transparent)]
pub struct PlayerId(pub u16);

/// A player in the game.
#[derive(Debug)]
pub struct Player<Role, Card> {
    health: u16,
    role: Role,
    holding: Stack<Card>,
    judge: Stack<Card>,

    cached_max_health_modifier: i16,
}

impl<Role, Card> Player<Role, Card> {
    /// Whether the player is alive.
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl<Role, Card> Player<Role, Card>
where
    Role: Character,
{
    /// Get the maximum health of the player.
    ///
    /// The maximum health is calculated by adding the default maximum health
    /// of the role and the cached maximum health modifier.
    #[inline]
    pub fn max_health(&self) -> u16 {
        (self.role.max_health() as i16 + self.cached_max_health_modifier) as u16
    }

    /// Update the cached maximum health modifier of the player.
    pub fn update_max_health_modifier(&mut self)
    where
        Card: HealthModifier,
    {
        let def = self.role.max_health();
        self.cached_max_health_modifier = self
            .holding
            .iter()
            .map(|c| c.max_health_modifier(def))
            .sum();
    }

    /// Get the health of the player.
    #[inline]
    pub fn health(&self) -> u16 {
        self.health.min(self.max_health())
    }
}

/// In-game state of the game.
#[derive(Debug)]
pub struct State<Role, Card, Deck> {
    players: Vec<Player<Role, Card>>,
    deck: Deck,

    current: PlayerId,
}

impl<Role, Card, Deck> State<Role, Card, Deck> {
    /// Create a new game state.
    #[inline]
    pub fn new(players: Vec<Player<Role, Card>>, deck: Deck) -> Self {
        Self {
            players,
            deck,
            current: PlayerId(0),
        }
    }
}

/// Information about a turn in the game.
#[derive(Debug)]
pub struct Turn<Cmd> {
    player: PlayerId,

    judge: Option<Cmd>,
}

impl<Role, Card, Deck, Cmd> Iterator for State<Role, Card, Deck>
where
    Card: Judge<Role, Deck, Command = Cmd>,
    for<'a> &'a Cmd: Command<Role, Card, Deck>,
    Deck: Pool<<Card as Judge<Role, Deck>>::Symbol>,
{
    type Item = Turn<Cmd>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.players[self.current.0 as usize + 1..]
            .iter()
            .position(|p| p.is_alive())
            .or_else(|| {
                self.players[..self.current.0 as usize]
                    .iter()
                    .position(|p| p.is_alive())
            })?;
        self.current = PlayerId(i as u16);

        // Judgement
        let judge_cmd = if let Some(judge) = self.players[i].judge.pop() {
            let symbol = self.deck.pop();
            let cmd = judge.judge(symbol, self);
            let mut ok = false;
            if let Some(cmd) = &cmd {
                ok = cmd.execute(self);
                // All judge cards should be cleared here, but this action should be declared in
                // the command.
            }
            cmd.filter(|_| ok)
        } else {
            None
        };

        todo!()
    }
}

/// Pool of cards.
pub trait Pool<T> {
    /// Pops a card from the pool.
    fn pop(&mut self) -> T;
}

impl<T, R> Pool<T> for R
where
    R: Rng,
    RandStd: RandDist<T>,
{
    /// Generates a card randomly.
    #[inline]
    fn pop(&mut self) -> T {
        self.gen()
    }
}

/// Command that can be applied on a [`State`].
pub trait Command<Role, Card, Deck> {
    /// Execute the command, and returns whether the execution is successful.
    fn execute(self, state: &mut State<Role, Card, Deck>) -> bool;
}
