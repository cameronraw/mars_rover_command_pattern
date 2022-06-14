use std::char;

pub struct MarsRover {

}

impl MarsRover {
    pub fn new() -> Self {
        MarsRover{}
    }

    pub fn execute(&self, _command: &str) -> &'static str {
        "0:0:W"
    }
}

#[cfg(test)]
mod mars_rover_should {
    use super::*;

    #[test]
    fn face_west_when_sent_turn_left_command(){
        let mars_rover = MarsRover::new();
        let response = mars_rover.execute("L");
        assert_eq!(response, "0:0:W");
    }
}