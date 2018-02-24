use std;

use wiimote;

fn check_flag<T: std::ops::BitAnd<Output=T> + std::cmp::PartialEq + Copy>(src: T, flag: T) -> bool {
    (src & flag) == flag
}

pub fn WIIMOTE_ID(wm: &wiimote) -> i32 {
    wm.unid
}

pub fn WIIMOTE_IS_SET(wm: &wiimote, s: i32) -> bool {
    check_flag(wm.state, s)
}

pub fn WIIMOTE_IS_CONNECTED(wm: &wiimote) -> bool {
    WIIMOTE_IS_SET(wm, 0x8)
}

pub fn IS_PRESSED(dev: &wiimote, button: u16) -> bool {
    check_flag(dev.btns, button)
}

pub fn IS_HELD(dev: &wiimote, button: u16) -> bool {
    check_flag(dev.btns_held, button)
}

pub fn IS_RELEASED(dev: &wiimote, button: u16) -> bool {
    check_flag(dev.btns_released, button)
}

pub fn IS_JUST_PRESSED(dev: &wiimote, button: u16) -> bool {
    IS_PRESSED(dev, button) && !IS_HELD(dev, button)
}

/**
 * This method diverges from the original source slightly.
 * I hope that's okay.
 */
pub fn WIIUSE_GET_IR_SENSITIVITY(wm: &wiimote) -> u8 {
    if check_flag(wm.state, 0x0200) { return 1; }
    else if check_flag(wm.state, 0x0400) { return 2; }
    else if check_flag(wm.state, 0x0800) { return 3; }
    else if check_flag(wm.state, 0x1000) { return 4; }
    else if check_flag(wm.state, 0x2000) { return 5; }
    0
}

pub fn WIIUSE_USING_ACC(wm: &wiimote) -> bool {
    check_flag(wm.state, 0x020)
}

pub fn WIIUSE_USING_EXP(wm: &wiimote) -> bool {
    check_flag(wm.state, 0x040)
}

pub fn WIIUSE_USING_IR(wm: &wiimote) -> bool {
    check_flag(wm.state, 0x080)
}

pub fn WIIUSE_USING_SPEAKER(wm: &wiimote) -> bool {
    check_flag(wm.state, 0x100)
}

pub fn WIIUSE_IS_LED_SET(wm: &wiimote, num: u8) -> bool {
    use WIIMOTE_LED_1;
    use WIIMOTE_LED_2;
    use WIIMOTE_LED_3;
    use WIIMOTE_LED_4;

    check_flag(i64::from(wm.state), match num {
        1 => WIIMOTE_LED_1,
        2 => WIIMOTE_LED_2,
        3 => WIIMOTE_LED_3,
        4 => WIIMOTE_LED_4,
        _ => panic!("no such wiimote led")
    }.into())
}
