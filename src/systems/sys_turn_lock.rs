use crate::components::{NavAgent, Player, Target, WorldTurn};
use crate::resources::{TurnLockMode, TurnLocker};
use specs::prelude::*;

pub struct SysTurnLock;

impl<'a> System<'a> for SysTurnLock {
    type SystemData = (
        Write<'a, TurnLocker>,
        ReadStorage<'a, NavAgent>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Target>,
        ReadStorage<'a, WorldTurn>,
    );

    fn run(&mut self, (mut turn_lock, nav, play, target, world): Self::SystemData) {
        // Locks turn while Player is playing
        if turn_lock.lock == TurnLockMode::PlayerTurn {
            for (play, nav, target) in (&play, &nav, &target).join() {
                if target.reached {
                    turn_lock.end_turn = true;
                    if !play.can_still_move() || nav.reached(target) {
                        turn_lock.next_sys = true;
                    }
                    return;
                }
            }
        }

        // Locks turn while World is playing
        if turn_lock.lock == TurnLockMode::WorldTurn {
            for (_, target) in (&world, &target).join() {
                if !target.reached {
                    return;
                }
            }
            turn_lock.end_turn = true;
            turn_lock.next_sys = true;
        }

        // Locks turn while Playing is inputting
        // in this particular case, next_sys will come from other systems
        if turn_lock.lock == TurnLockMode::PlayerInput {
            turn_lock.end_turn = true;
        }
    }
}
