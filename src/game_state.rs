use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use rand::Rng;
use std::f64;

//use crate::geometry::{Position, Size};
use crate::models::World;

use super::Actions;
//use crate::game_state::GameState;
use crate::geometry::{Advance, Point, Position, Size};
use crate::models::{Bullet, Enemy, Particle, Vector};
use crate::util;


// Constants related to time
const BULLETS_PER_SECOND: f64 = 100.0;
const BULLET_RATE: f64 = 1.0 / BULLETS_PER_SECOND;

const ENEMY_SPAWNS_PER_SECOND: f64 = 1.0;
const ENEMY_SPAWN_RATE: f64 = 1.0 / ENEMY_SPAWNS_PER_SECOND;

const TRAIL_PARTICLES_PER_SECOND: f64 = 20.0;
const TRAIL_PARTICLE_RATE: f64 = 1.0 / TRAIL_PARTICLES_PER_SECOND;

// Constants related to movement
// Speed is measured in pixels per second
// Rotation speed is measured in radians per second
const ADVANCE_SPEED: f64 = 200.0;
const BULLET_SPEED: f64 = 500.0;
const ENEMY_SPEED: f64 = 100.0;
const ROTATE_SPEED: f64 = 2.0 * f64::consts::PI;

const PLAYER_GRACE_AREA: f64 = 200.0;

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current score of the player
    pub score: u32,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size) -> GameState {
        let mut rng = Pcg32Basic::from_seed([42, 42]);
        GameState {
            world: World::new(&mut rng, size),
            score: 0,
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {
        let mut rng = Pcg32Basic::from_seed([42, 42]);

        // Reset player position
        for player in &mut self.world.player{
            *player.x_mut() = self.world.size.random_x(&mut rng);
            *player.y_mut() = self.world.size.random_y(&mut rng);
        }

        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
//        self.world.bullets.clear();
//        self.world.enemies.clear();
    }
}

/// Timers to handle creation of bullets, enemies and particles
pub struct TimeController<T: Rng> {
    /// A random number generator
    rng: T,
    current_time: f64,
    last_tail_particle: f64,
    last_shoot: f64,
    last_spawned_enemy: f64,
}

impl<T: Rng> TimeController<T> {
    pub fn new(rng: T) -> TimeController<T> {
        TimeController {
            rng,
            current_time: 0.0,
            last_tail_particle: 0.0,
            last_shoot: 0.0,
            last_spawned_enemy: 0.0,
        }
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update_seconds(&mut self, dt: f64, actions: &Actions, state: &mut GameState) {
        self.current_time += dt;

        // Update rocket rotation
        for player in &mut state.world.player {
            if actions.up {
    //            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
                *player.direction_mut() = 1.5 * f64::consts::PI;
            }
            if actions.down {
    //            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
                *player.direction_mut() = 0.5 * f64::consts::PI;
            }
            if actions.left {
    //            *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
                *player.direction_mut() = f64::consts::PI;
            }
            if actions.right {
    //            *state.world.player.direction_mut() += ROTATE_SPEED * dt;
                *player.direction_mut() = 0.0;
            };

            // Set speed and advance the player with wrap around
            let speed = if actions.up || actions.down || actions.left || actions.right {
                player.speed
            } else {
                0.0
            };
                player.advance_wrapping(dt * speed, state.world.size);

            // Update particles
            for particle in &mut state.world.particles {
                particle.update(dt);
            }

            // Remove old particles
            util::fast_retain(&mut state.world.particles, |p| p.ttl > 0.0);

            // Add new particles at the player's position, to leave a trail
            if self.current_time - self.last_tail_particle > TRAIL_PARTICLE_RATE {
                self.last_tail_particle = self.current_time;
                state.world.particles.push(Particle::new(
                    player.vector.clone().invert(),
                    0.5,
                ));
            }

            // Add bullets
    /*
            if actions.shoot && self.current_time - self.last_shoot > BULLET_RATE {
                self.last_shoot = self.current_time;
                state.world.bullets.push(Bullet::new(Vector::new(
                    state.world.player.front(),
                    state.world.player.direction(),
                )));
            }

            // Advance bullets
            for bullet in &mut state.world.bullets {
                bullet.update(dt * BULLET_SPEED);
            }

            // Remove bullets outside the viewport
            {
                // Shorten the lifetime of size
                let size = &state.world.size;
                util::fast_retain(&mut state.world.bullets, |b| size.contains(b.position()));
            }
    */
            // Spawn enemies at random locations
    /*
            if self.current_time - self.last_spawned_enemy > ENEMY_SPAWN_RATE {
                self.last_spawned_enemy = self.current_time;

                let player_pos: &Vector = &state.world.player.vector;
                let mut enemy_pos;
                // We loop here, just in case the new enemy random position is exactly equal
                // to the players current position, this would break our calculations below
                loop {
                    enemy_pos = Vector::random(&mut self.rng, state.world.size);
                    if enemy_pos.position != player_pos.position {
                        break;
                    }
                }
                // Check if the newly spawned enemy is inside the player's grace area,
                // if so, we push its spawn point to the edge of the area
                if enemy_pos
                    .position
                    .intersect_circle(&player_pos.position, PLAYER_GRACE_AREA)
                {
                    let length: f64 = enemy_pos
                        .position
                        .squared_distance_to(&player_pos.position)
                        .sqrt();
                    let dp: Point = enemy_pos.position - player_pos.position;
                    enemy_pos.position = player_pos.position + dp / length * PLAYER_GRACE_AREA;
                }

                let new_enemy = Enemy::new(enemy_pos);
                state.world.enemies.push(new_enemy);
            }

            // Move enemies in the player's direction
            for enemy in &mut state.world.enemies {
                enemy.update(dt * ENEMY_SPEED, state.world.player.position());
            }
    */
        }
    }
}
