fn main() {}
fn throttle(current_acceleration: i32, acceleration: i32, current_speed: i32) -> (i32, i32){
    (current_acceleration + acceleration, current_speed + acceleration)
}

fn update(current_speed: i32, current_acceleration: i32) -> (i32) {
    current_speed + current_acceleration
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn throttle_should_set_acceleration_when_initially_zero() {
        let acceleration = 0;
        let (new_acceleration, new_speed) = throttle(acceleration, 1, 0);
        assert_eq!(1, new_acceleration);
    }

    #[test]
    fn throttle_should_add_acceleration_to_current_acceleration() {
        let current_acceleration = 1;
        let (new_acceleration, new_speed) = throttle(current_acceleration, 1, 0);
        assert_eq!(2, new_acceleration);
    }

    #[test]
    fn throttle_should_update_speed_when_initially_zero() {
        let current_speed = 0;
        let (new_acceleration, new_speed) = throttle(0, 1, current_speed);
        assert_eq!(1, new_speed);
    }

    #[test]
    fn throttle_should_add_speed_to_current_speed() {
        let current_speed = 1;
        let (new_acceleration, new_speed) = throttle(0, 1, current_speed);
        assert_eq!(2, new_speed);
    }

    #[test]
    fn update_should_update_speed_with_current_acceleration(){
        let current_speed = 0;
        let current_acceleration = 1;
        let new_speed = update(current_speed, current_acceleration);
        assert_eq!(1, new_speed);
    }
}
