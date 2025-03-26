use std::io::{Read, Write};
use k_board::termio::{setup_raw_mode, termios, restore};

fn main() {

    let mut state = State {
        acceleration: 0,
        speed: 0,
        position: 0
    };

    loop {
        let env: termios = setup_raw_mode().unwrap();
        std::io::stdout().flush().unwrap();

        let mut buffer: [u8; 3] = [0; 3];
        std::io::stdin().read(&mut buffer).unwrap();

        match buffer[2]{
            0x41 => state = update(state, 1),
            0x42 => state = update(state, -1),
            _ => state = update(state, 0)
        }

        println!("accel: {0}; speed: {1}; position: {2}", state.acceleration, state.speed, state.position);

        std::io::stdout().flush().unwrap();
        restore(&env).unwrap();
    }
}

struct State {
    acceleration: i32,
    speed: i32,
    position: i32
}
fn update(current_state: State, throttle: i32) -> State{
    State {
        acceleration: current_state.acceleration + throttle,
        speed: current_state.speed + throttle + current_state.acceleration,
        position: current_state.position + current_state.speed
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn throttle_should_set_acceleration_when_initially_zero() {
        let current_state = State {
            acceleration: 0,
            speed: 0,
            position: 0
        };
        let new_state = update(current_state, 1);
        assert_eq!(1, new_state.acceleration);
    }

    #[test]
    fn throttle_should_add_acceleration_to_current_acceleration() {
        let current_state = State {
            acceleration: 1,
            speed: 0,
            position: 0
        };
        let new_state = update(current_state, 1);
        assert_eq!(2, new_state.acceleration);
    }

    #[test]
    fn throttle_should_update_speed_when_initially_zero() {
        let current_state = State {
            acceleration: 0,
            speed: 0,
            position: 0
        };
        let new_state = update(current_state, 1);
        assert_eq!(1, new_state.speed);
    }

    #[test]
    fn throttle_should_add_speed_to_current_speed() {
        let current_state = State {
            acceleration: 0,
            speed: 1,
            position: 0
        };
        let new_state = update(current_state, 1);
        assert_eq!(2, new_state.speed);
    }

    #[test]
    fn no_throttle_should_update_speed_with_current_acceleration(){

        let current_state = State {
            acceleration: 1,
            speed: 0,
            position: 0
        };
        let new_state = update(current_state, 0);
        assert_eq!(1, new_state.speed);
    }

    #[test]
    fn throttle_should_add_speed_from_current_acceleration_and_throttle_input () {
        let current_state = State {
            acceleration: 1,
            speed: 0,
            position: 0
        };
        let new_state = update(current_state, 1);
        assert_eq!(2, new_state.speed);
    }

    #[test]
    fn no_throttle_should_update_position_from_speed (){
        let current_state = State {
            acceleration: 0,
            speed: 1,
            position: 0
        };

        let new_state = update(current_state, 0);
        assert_eq!(1, new_state.position);
    }

    #[test]
    fn no_throttle_should_add_current_speed_to_current_position () {
        let current_state = State {
            acceleration: 0,
            speed: 1,
            position: 1
        };

        let new_state = update(current_state, 0);
        assert_eq!(2, new_state.position);
    }

    #[test]
    fn throttle_input_should_add_up_to_position_with_current_speed (){
        let current_state = State {
            acceleration: 0,
            speed: 1,
            position: 1
        };

        let new_state = update(current_state, 1);
        assert_eq!(2, new_state.speed);
    }
}
