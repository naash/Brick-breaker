// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

/// Marks an entity as a brick
#[derive(Debug, Clone)]
pub struct Brick {
    pub health: u32,
    pub points: u32,
}

impl Brick {
    pub fn new(brick_type: BrickType) -> Self {
        let (health, points) = match brick_type {
            BrickType::Normal => (1, 10),
            BrickType::Strong => (2, 20),
        };

        Self { health, points }
    }

    pub fn take_damage(&mut self) -> bool {
        if self.health > 0 {
            self.health -= 1;
        }
        self.health == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BrickType {
    Normal,
    Strong,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    WaitingToLaunch,
    Playing,
    GameOver,
    Won,
    Resetting,
}

/// Game state component (attached to a game manager entity)
#[derive(Debug, Clone)]
pub struct BrickBreakerState {
    pub score: u32,
    pub lives: u32,
    pub state: GameState,
}

impl Default for BrickBreakerState {
    fn default() -> Self {
        Self {
            score: 0,
            lives: 3,
            state : GameState::WaitingToLaunch
        }
    }
}