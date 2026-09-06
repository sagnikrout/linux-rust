//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accessibility/speakup/spk_types.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
// This file includes all of the typedefs and structs used in speakup.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum var_type_t {
    VAR_NUM = 0,
    VAR_TIME,
    VAR_STRING,
    VAR_PROC
}

//
// Note: add new members at the end, speakupmap.h depends on the values of the
// enum starting from SPELL_DELAY (see inc_dec_var)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum var_id_t {
    VERSION = 0, SYNTH, SILENT, SYNTH_DIRECT,
    KEYMAP, CHARS,
    PUNC_SOME, PUNC_MOST, PUNC_ALL,
    DELIM, REPEATS, EXNUMBER,
    DELAY, TRIGGER, JIFFY, FULL, /* all timers must be together */
    BLEEP_TIME, CURSOR_TIME, BELL_POS,
    SAY_CONTROL, SAY_WORD_CTL, NO_INTERRUPT, KEY_ECHO,
    SPELL_DELAY, PUNC_LEVEL, READING_PUNC,
    ATTRIB_BLEEP, BLEEPS,
    RATE, PITCH, VOL, TONE, PUNCT, VOICE, FREQUENCY, LANG,
    DIRECT, PAUSE,
    CAPS_START, CAPS_STOP, CHARTAB, INFLECTION, FLUSH,
    CUR_PHONETIC, MAXVARS
}

pub const COLOR_BUFFER_SIZE: c_int = 160;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spk_highlight_color_track {
// Count of each background color
    pub bgcount: [c_uint; 8],
// Buffer for characters drawn with each background color
    pub highbuf: [u16; 8][COLOR_BUFFER_SIZE],
// Current index into highbuf
    pub highsize: [c_uint; 8],
// Reading Position for each color
    pub ry: [u_long rpos[8], rx[8],; 8],
// Real Cursor Y Position
    pub cy: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_spk_t {
    pub cursor_x: u_long reading_x,,
    pub cursor_y: u_long reading_y,,
    pub cursor_pos: u_long reading_pos,,
    pub go_pos: u_long go_x,,
    pub w_right: u_long w_top, w_bottom, w_left,,
    pub w_enabled: u_char w_start,,
    pub old_attr: u_char reading_attr,,
    pub shut_up: char parked,,
    pub ht: spk_highlight_color_track,
    pub tty_stopped: c_int,
}

// now some defines to make these easier to use.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_var_header {
    pub name: *mut c_char,
    pub var_id: var_id_t,
    pub var_type: var_type_t,
    pub /: *mut *mut *mut void p_val; / ptr to programs variable to store value,
    pub /: *mut *mut *mut void data; / ptr to the vars data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct num_var_t {
    pub synth_fmt: *mut c_char,
    pub default_val: c_int,
    pub low: c_int,
    pub high: c_int,
    pub /: *mut *mut short offset, multiplier; / for fiddling rates etc.,
    pub /: *mut *mut *mut char out_str; / if synth needs char representation of number,
    pub /: *mut *mut int value; / current value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct punc_var_t {
    pub var_id: var_id_t,
    pub value: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct string_var_t {
    pub default_val: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_t {
    pub var_id: var_id_t,
    pub n: num_var_t,
    pub s: string_var_t,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_bits_data {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub mask: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_indexing {
    pub command: *mut c_char,
    pub lowindex: c_uchar,
    pub highindex: c_uchar,
    pub currindex: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spk_io_ops {
    pub ch): *const *const *const int (synth_out)(struct spk_synth synth, char,
    pub ch): *mut *mut *mut int (synth_out_unicode)(struct spk_synth synth, u16,
    pub ch): *mut *mut *mut void (send_xchar)(struct spk_synth synth, char,
    pub clear): *mut *mut *mut void (tiocmset)(struct spk_synth synth, unsigned int set, unsigned int,
    pub synth): *mut *mut unsigned char (synth_in)(struct spk_synth,
    pub synth): *mut *mut unsigned char (synth_in_nowait)(struct spk_synth,
    pub synth): *mut *mut void (flush_buffer)(struct spk_synth,
    pub synth): *mut *mut int (wait_for_xmitr)(struct spk_synth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spk_synth {
    pub node: list_head,
    pub name: *const c_char,
    pub version: *const c_char,
    pub long_name: *const c_char,
    pub init: *const c_char,
    pub procspeech: c_char,
    pub clear: c_char,
    pub delay: c_int,
    pub trigger: c_int,
    pub jiffies: c_int,
    pub full: c_int,
    pub flush_time: c_int,
    pub ser: c_int,
    pub dev_name: *mut c_char,
    pub flags: c_short,
    pub startup: c_short,
    pub /: *const *const int checkval; / for validating a proper synth module,
    pub vars: *mut var_t,
    pub default_pitch: *mut c_int,
    pub default_vol: *mut c_int,
    pub io_ops: *mut spk_io_ops,
    pub synth): *mut *mut int (probe)(struct spk_synth,
    pub synth): *mut *mut void (release)(struct spk_synth,
    pub buff): *const c_char,
    pub synth): *mut *mut void (catch_up)(struct spk_synth,
    pub synth): *mut *mut void (flush)(struct spk_synth,
    pub synth): *mut *mut int (is_alive)(struct spk_synth,
    pub var): *mut *mut *mut int (synth_adjust)(struct spk_synth synth, struct st_var_header,
    pub c): *mut *mut void (read_buff_add)(u_char,
    pub synth): *mut *mut unsigned char (get_index)(struct spk_synth,
    pub indexing: synth_indexing,
    pub alive: c_int,
    pub attributes: attribute_group,
    pub dev: *mut c_void,
}

//
// module_spk_synth() - Helper macro for registering a speakup driver
// @__spk_synth: spk_synth struct
// Helper macro for speakup drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct speakup_info_t {
    pub spinlock: spinlock_t,
    pub port_tts: c_int,
    pub flushing: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bleep {
    pub freq: c_short,
    pub jiffies: c_ulong,
    pub active: c_int,
}
