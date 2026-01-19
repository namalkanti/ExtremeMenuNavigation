#[derive(Debug, Clone)]
pub struct GameSnapshot {
}

impl Default for GameSnapshot {
    fn default() -> Self {
        Self {
        }
    }
}

/// Events emitted by the game/menu layer and consumed by the chat/story layer.
///
/// These are "moments" that tell the story engine when to re-evaluate and what
/// just happened, without requiring the story to reconstruct full state from
/// an event history (it can read `GameSnapshot` for current truth).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event{
    Started,
    NetworkToggled,
}

/// Events emitted by the chat/story layer and consumed by the game/menu layer.
///
/// These are actions the story wants to trigger in the game. Keep them semantic
/// and minimal; add variants only when the story truly needs to cause an effect.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    SetFriendInLobby(bool),
    SetFriendReady(bool),
    ForceDisconnect,
}
