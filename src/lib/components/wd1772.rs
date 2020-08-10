use crate::lib::traits::component::*;

/*-----

1 : CS* (Chip Select)
2 : RW* (Read/Write)
3-4 : A0, A1 (Address)
5-12 : DAL0-DAL7 (Data lines)
13 : MR* (Master Reset)
14 : GND
15 : VCC
16 : STEP (Step)
17 : DIRC (Direction)
18 : CLK (Clock)
19 : RD* (Read Data)
20 : MO (Motor On)
21 : WF (Write Gate)

 -----*/

 enum WD1772_PINS {
     CS = 0,
     RW,
     A0,
     A1,
     DAL0,
     DAL1,
     DAL2,
     DAL3,
     DAL4,
     DAL5,
     DAL6,
     DAL7,
     MR,
     GND,
     VCC,
     STEP,
     DIRC,
     CLK,
     RD,
     MO,
     WG,
     WD,
     TR00,
     IP,
     WPRT,
     DDEN,
     DRQ,
     INTRQ
 }

struct WD1772 {
    pins: [Pin; 28],
    data_shift_register: u8,
    data_register: u8,
    track_register: u8,
    sector_register: u8,
    command_register: u8,
    status_register: u8,
}

impl WD1772 {
    pub fn new() -> WD1772 {
        WD1772 {
            data_shift_register: 0,
            data_register: 0,
            track_register: 0,
            sector_register: 0,
            command_register: 0,
            status_register: 0,
            pins: [
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::BOTH),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::OUTPUT),
                Pin::new(PinDirection::OUTPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::OUTPUT),
                Pin::new(PinDirection::OUTPUT),
                Pin::new(PinDirection::OUTPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::INPUT),
                Pin::new(PinDirection::OUTPUT),
                Pin::new(PinDirection::OUTPUT),
                Pin::new(PinDirection::OUTPUT),
            ],
        }
    }
}