use std::os::raw::{
    c_char,
    c_int,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawCell {
    pub ch: u32,
    pub fg: u16,
    pub bg: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawEvent {
    pub etype: u8,
    pub emod: u8,
    pub key: u16,
    pub ch: u32,
    pub w: i32,
    pub h: i32,
    pub x: i32,
    pub y: i32,
}

extern "C" {
    pub fn tb_init() -> c_int;
    pub fn tb_shutdown();
    pub fn tb_width() -> c_int;
    pub fn tb_height() -> c_int;
    pub fn tb_clear();
    pub fn tb_set_clear_attributes(fg: u16, bg: u16);
    pub fn tb_present();
    pub fn tb_set_cursor(cx: c_int, cy: c_int);
    pub fn tb_put_cell(x: c_int, y: c_int, cell: *const RawCell);
    pub fn tb_change_cell(x: c_int, y: c_int, ch: u32, fg: u16, bg: u16);
    pub fn tb_blit(x: c_int, y: c_int, w: c_int, h: c_int, cells: *const RawCell);
    pub fn tb_cell_buffer() -> *mut RawCell;
    pub fn tb_select_input_mode(mode: c_int) -> c_int;
    pub fn tb_select_output_mode(mode: c_int) -> c_int;
    pub fn tb_peek_event(ev: *mut RawEvent, timeout: c_int) -> c_int;
    pub fn tb_poll_event(ev: *mut RawEvent) -> c_int;
    pub fn tb_utf8_char_length(c: c_char) -> c_int;
    pub fn tb_utf8_char_to_unicode(out: *mut u32, c: *const c_char) -> c_int;
    pub fn tb_utf8_unicode_to_char(out: *mut c_char, c: u32) -> c_int;
}

pub const TB_KEY_F1: u16 = 0xffff - 0;
pub const TB_KEY_F2: u16 = 0xffff - 1;
pub const TB_KEY_F3: u16 = 0xffff - 2;
pub const TB_KEY_F4: u16 = 0xffff - 3;
pub const TB_KEY_F5: u16 = 0xffff - 4;
pub const TB_KEY_F6: u16 = 0xffff - 5;
pub const TB_KEY_F7: u16 = 0xffff - 6;
pub const TB_KEY_F8: u16 = 0xffff - 7;
pub const TB_KEY_F9: u16 = 0xffff - 8;
pub const TB_KEY_F10: u16 = 0xffff - 9;
pub const TB_KEY_F11: u16 = 0xffff - 10;
pub const TB_KEY_F12: u16 = 0xffff - 11;
pub const TB_KEY_INSERT: u16 = 0xffff - 12;
pub const TB_KEY_DELETE: u16 = 0xffff - 13;
pub const TB_KEY_HOME: u16 = 0xffff - 14;
pub const TB_KEY_END: u16 = 0xffff - 15;
pub const TB_KEY_PGUP: u16 = 0xffff - 16;
pub const TB_KEY_PGDN: u16 = 0xffff - 17;
pub const TB_KEY_ARROW_UP: u16 = 0xffff - 18;
pub const TB_KEY_ARROW_DOWN: u16 = 0xffff - 19;
pub const TB_KEY_ARROW_LEFT: u16 = 0xffff - 20;
pub const TB_KEY_ARROW_RIGHT: u16 = 0xffff - 21;
pub const TB_KEY_MOUSE_LEFT: u16 = 0xffff - 22;
pub const TB_KEY_MOUSE_RIGHT: u16 = 0xffff - 23;
pub const TB_KEY_MOUSE_MIDDLE: u16 = 0xffff - 24;
pub const TB_KEY_MOUSE_RELEASE: u16 = 0xffff - 25;
pub const TB_KEY_MOUSE_WHEEL_UP: u16 = 0xffff - 26;
pub const TB_KEY_MOUSE_WHEEL_DOWN: u16 = 0xffff - 27;

pub const TB_KEY_CTRL_TILDE: u16 = 0x00;
pub const TB_KEY_CTRL_2: u16 = 0x00;
pub const TB_KEY_CTRL_A: u16 = 0x01;
pub const TB_KEY_CTRL_B: u16 = 0x02;
pub const TB_KEY_CTRL_C: u16 = 0x03;
pub const TB_KEY_CTRL_D: u16 = 0x04;
pub const TB_KEY_CTRL_E: u16 = 0x05;
pub const TB_KEY_CTRL_F: u16 = 0x06;
pub const TB_KEY_CTRL_G: u16 = 0x07;
pub const TB_KEY_BACKSPACE: u16 = 0x08;
pub const TB_KEY_CTRL_H: u16 = 0x08;
pub const TB_KEY_TAB: u16 = 0x09;
pub const TB_KEY_CTRL_I: u16 = 0x09;
pub const TB_KEY_CTRL_J: u16 = 0x0a;
pub const TB_KEY_CTRL_K: u16 = 0x0b;
pub const TB_KEY_CTRL_L: u16 = 0x0c;
pub const TB_KEY_ENTER: u16 = 0x0d;
pub const TB_KEY_CTRL_M: u16 = 0x0d;
pub const TB_KEY_CTRL_N: u16 = 0x0e;
pub const TB_KEY_CTRL_O: u16 = 0x0f;
pub const TB_KEY_CTRL_P: u16 = 0x10;
pub const TB_KEY_CTRL_Q: u16 = 0x11;
pub const TB_KEY_CTRL_R: u16 = 0x12;
pub const TB_KEY_CTRL_S: u16 = 0x13;
pub const TB_KEY_CTRL_T: u16 = 0x14;
pub const TB_KEY_CTRL_U: u16 = 0x15;
pub const TB_KEY_CTRL_V: u16 = 0x16;
pub const TB_KEY_CTRL_W: u16 = 0x17;
pub const TB_KEY_CTRL_X: u16 = 0x18;
pub const TB_KEY_CTRL_Y: u16 = 0x19;
pub const TB_KEY_CTRL_Z: u16 = 0x1a;
pub const TB_KEY_ESC: u16 = 0x1b;
pub const TB_KEY_CTRL_LSQ_BRACKET: u16 = 0x1b;
pub const TB_KEY_CTRL_3: u16 = 0x1b;
pub const TB_KEY_CTRL_4: u16 = 0x1c;
pub const TB_KEY_CTRL_BACKSLASH: u16 = 0x1c;
pub const TB_KEY_CTRL_5: u16 = 0x1d;
pub const TB_KEY_CTRL_RSQ_BRACKET: u16 = 0x1d;
pub const TB_KEY_CTRL_6: u16 = 0x1e;
pub const TB_KEY_CTRL_7: u16 = 0x1f;
pub const TB_KEY_CTRL_SLASH: u16 = 0x1f;
pub const TB_KEY_CTRL_UNDERSCORE: u16 = 0x1f;
pub const TB_KEY_SPACE: u16 = 0x20;
pub const TB_KEY_BACKSPACE2: u16 = 0x7f;
pub const TB_KEY_CTRL_8: u16 = 0x7f;

pub const TB_MOD_ALT: u8 = 0x01;

pub const TB_DEFAULT: u16 = 0x00;
pub const TB_BLACK: u16 = 0x01;
pub const TB_RED: u16 = 0x02;
pub const TB_GREEN: u16 = 0x03;
pub const TB_YELLOW: u16 = 0x04;
pub const TB_BLUE: u16 = 0x05;
pub const TB_MAGENTA: u16 = 0x06;
pub const TB_CYAN: u16 = 0x07;
pub const TB_WHITE: u16 = 0x08;

pub const TB_BOLD: u16 = 0x0100;
pub const TB_UNDERLINE: u16 = 0x0200;
pub const TB_REVERSE: u16 = 0x0400;

pub const TB_EVENT_KEY: u8 = 1;
pub const TB_EVENT_RESIZE: u8 = 2;
pub const TB_EVENT_MOUSE: u8 = 3;

pub const TB_EUNSUPPORTED_TERMINAL: c_int = -1;
pub const TB_EFAILED_TO_OPEN_TTY: c_int = -2;
pub const TB_EPIPE_TRAP_ERROR: c_int = -3;

pub const TB_HIDE_CURSOR: c_int = -1;

pub const TB_INPUT_CURRENT: c_int = 0;
pub const TB_INPUT_ESC: c_int = 1;
pub const TB_INPUT_ALT: c_int = 2;
pub const TB_INPUT_MOUSE: c_int = 4;

pub const TB_OUTPUT_CURRENT: c_int = 0;
pub const TB_OUTPUT_NORMAL: c_int = 1;
pub const TB_OUTPUT_256: c_int = 2;
pub const TB_OUTPUT_216: c_int = 3;
pub const TB_OUTPUT_GRAYSCALE: c_int = 4;

pub const TB_EOF: c_int = -1;
