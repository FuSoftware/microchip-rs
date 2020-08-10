#[derive(PartialEq)]
pub enum PinDirection {
    INPUT,
    OUTPUT,
    BOTH
}

pub struct Pin {
    direction: PinDirection,
    value: bool
}

impl Pin {
    pub fn new(direction: PinDirection) -> Pin {
        Pin {
            direction: direction,
            value: false,
        }
    }

    pub fn set_value(&mut self, value: bool, direction: PinDirection) {
        if self.direction == PinDirection::BOTH {
            
        } else {
            if self.direction == direction {
                self.value = value;
            } else {
                println!("Wrong writing direction")
            }
        }
    }

    pub fn get_value(&self) -> bool {
        return self.value;
    }
}

trait Component {

}