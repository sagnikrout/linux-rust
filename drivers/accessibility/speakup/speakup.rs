//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accessibility/speakup/speakup.h
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

pub const KEY_MAP_VER: c_int = 119;
pub const SHIFT_TBL_SIZE: c_int = 64;
pub const MAX_DESC_LEN: c_int = 72;

pub const MAXVARLEN: c_int = 15;
pub const SYNTH_OK: c_uint = 0x0001;
pub const B_ALPHA: c_uint = 0x0002;
pub const ALPHA: c_uint = 0x0003;
pub const B_CAP: c_uint = 0x0004;
pub const A_CAP: c_uint = 0x0007;
pub const B_NUM: c_uint = 0x0008;
pub const NUM: c_uint = 0x0009;

pub const SOME: c_uint = 0x0010;
pub const MOST: c_uint = 0x0020;
pub const PUNC: c_uint = 0x0040;
pub const A_PUNC: c_uint = 0x0041;
pub const B_WDLM: c_uint = 0x0080;
pub const WDLM: c_uint = 0x0081;
pub const B_EXNUM: c_uint = 0x0100;
pub const CH_RPT: c_uint = 0x0200;
pub const B_CTL: c_uint = 0x0400;

pub const B_SYM: c_uint = 0x0800;

// FIXME: u16

extern "C" {
    pub fn speakup_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn spk_reset_default_chars();
}
extern "C" {
    pub fn spk_reset_default_chartab();
}
extern "C" {
    pub fn synth_start();
}
extern "C" {
    pub fn synth_insert_next_index(sent_num: c_int);
}
extern "C" {
    pub fn spk_reset_index_count(sc: c_int);
}
extern "C" {
    pub fn spk_get_index_count(linecount: *mut c_int, sentcount: *mut c_int);
}
extern "C" {
    pub fn spk_set_key_info(key_info: *const u_char, k_buffer: *mut u_char) -> c_int;
}
extern "C" {
    pub fn speakup_kobj_init() -> c_int;
}
extern "C" {
    pub fn speakup_kobj_exit();
}
extern "C" {
    pub fn spk_chartab_get_value(keyword: *mut c_char) -> c_int;
}
extern "C" {
    pub fn speakup_register_var(var: *mut var_t);
}
extern "C" {
    pub fn speakup_unregister_var(var_id: var_id_t);
}
extern "C" {
    pub fn spk_set_num_var(val: c_int, var: *mut st_var_header, how: c_int) -> c_int;
}
extern "C" {
    pub fn spk_set_string_var(page: *const c_char, var: *mut st_var_header, len: c_int) -> c_int;
}
extern "C" {
    pub fn spk_set_mask_bits(input: *const c_char, which: c_int, how: c_int) -> c_int;
}
extern "C" {
    pub fn spk_handle_help(vc: *mut vc_data, type: u_char, ch: u_char, key: u16) -> c_int;
}
extern "C" {
    pub fn synth_init(name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn synth_release();
}
extern "C" {
    pub fn spk_do_flush();
}
extern "C" {
    pub fn speakup_start_ttys();
}
extern "C" {
    pub fn synth_buffer_add(ch: u16);
}
extern "C" {
    pub fn synth_buffer_clear();
}
extern "C" {
    pub fn speakup_set_selection(tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn speakup_cancel_selection();
}
extern "C" {
    pub fn speakup_paste_selection(tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn speakup_cancel_paste();
}
extern "C" {
    pub fn speakup_register_devsynth();
}
extern "C" {
    pub fn speakup_unregister_devsynth();
}
extern "C" {
    pub fn synth_utf8_get(buf: *const c_char, count: usize, consumed: *mut usize, want: *mut usize) -> i32;
}
extern "C" {
    pub fn synth_write(buf: *const c_char, count: usize);
}
extern "C" {
    pub fn synth_writeu(buf: *const c_char, count: usize);
}
extern "C" {
    pub fn synth_supports_indexing() -> c_int;
}
// Protect speakup synthesizer list
// Prototypes from fakekey.c.
extern "C" {
    pub fn speakup_add_virtual_keyboard() -> c_int;
}
extern "C" {
    pub fn speakup_remove_virtual_keyboard();
}
extern "C" {
    pub fn speakup_fake_down_arrow();
}
extern "C" {
    pub fn speakup_fake_key_pressed() -> bool;
}
