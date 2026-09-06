//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/ui/browser.h
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
pub const _PERF_UI_BROWSER_H_: c_int = 1;

pub const HE_COLORSET_TOP: c_int = 50;
pub const HE_COLORSET_MEDIUM: c_int = 51;
pub const HE_COLORSET_NORMAL: c_int = 52;
pub const HE_COLORSET_SELECTED: c_int = 53;
pub const HE_COLORSET_JUMP_ARROWS: c_int = 54;
pub const HE_COLORSET_ADDR: c_int = 55;
pub const HE_COLORSET_ROOT: c_int = 56;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ui_browser {
    pub top_idx: u64 index,,
    pub entries: *mut *mut void top,,
    pub horiz_scroll: u16 y, x, width, height, rows, columns,,
    pub extra_title_lines: u8,
    pub current_color: c_int,
    pub priv: *mut c_void,
    pub title: *mut c_char,
    pub helpline: *mut c_char,
    pub no_samples_msg: *const c_char,
    pub browser): *mut *mut void (refresh_dimensions)(struct ui_browser,
    pub browser): *mut *mut unsigned int (refresh)(struct ui_browser,
    pub row): *mut *mut *mut *mut void (write)(struct ui_browser browser, void entry, int,
    pub whence): *mut *mut *mut void (seek)(struct ui_browser browser, off_t offset, int,
    pub entry): *mut *mut *mut bool (filter)(struct ui_browser browser, void,
    pub nr_entries: u32,
    pub navkeypressed: bool,
    pub use_navkeypressed: bool,
}

extern "C" {
    pub fn ui_browser__set_color(browser: *mut ui_browser, color: c_int) -> c_int;
}
extern "C" {
    pub fn ui_browser__is_current_entry(browser: *mut ui_browser, row: unsigned) -> bool;
}
extern "C" {
    pub fn ui_browser__refresh_dimensions(browser: *mut ui_browser);
}
extern "C" {
    pub fn ui_browser__reset_index(browser: *mut ui_browser);
}
extern "C" {
    pub fn ui_browser__gotorc_title(browser: *mut ui_browser, y: c_int, x: c_int);
}
extern "C" {
    pub fn ui_browser__gotorc(browser: *mut ui_browser, y: c_int, x: c_int);
}
extern "C" {
    pub fn ui_browser__vprintf(browser: *mut ui_browser, fmt: *const c_char, args: va_list);
}
extern "C" {
    pub fn ui_browser__printf(browser: *mut ui_browser, fmt: *const c_char, ...);
}
extern "C" {
    pub fn ui_browser__write_graph(browser: *mut ui_browser, graph: c_int);
}
extern "C" {
    pub fn __ui_browser__show_title(browser: *mut ui_browser, title: *const c_char);
}
extern "C" {
    pub fn ui_browser__show_title(browser: *mut ui_browser, title: *const c_char);
}
extern "C" {
    pub fn ui_browser__hide(browser: *mut ui_browser);
}
extern "C" {
    pub fn ui_browser__refresh(browser: *mut ui_browser) -> c_int;
}
extern "C" {
    pub fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int;
}
extern "C" {
    pub fn ui_browser__update_nr_entries(browser: *mut ui_browser, nr_entries: u32);
}
extern "C" {
    pub fn ui_browser__handle_resize(browser: *mut ui_browser);
}
extern "C" {
    pub fn ui_browser__warn_unhandled_hotkey(browser: *mut ui_browser, key: c_int, timeout: c_int, help: *const c_char) -> c_int;
}
extern "C" {
    pub fn ui_browser__help_window(browser: *mut ui_browser, text: *const c_char) -> c_int;
}
extern "C" {
    pub fn ui_browser__dialog_yesno(browser: *mut ui_browser, text: *const c_char) -> bool;
}
extern "C" {
    pub fn tui__header_window(session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn ui_browser__argv_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);
}
extern "C" {
    pub fn ui_browser__argv_refresh(browser: *mut ui_browser) -> c_uint;
}
extern "C" {
    pub fn ui_browser__rb_tree_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);
}
extern "C" {
    pub fn ui_browser__rb_tree_refresh(browser: *mut ui_browser) -> c_uint;
}
extern "C" {
    pub fn ui_browser__list_head_seek(browser: *mut ui_browser, offset: off_t, whence: c_int);
}
extern "C" {
    pub fn ui_browser__list_head_refresh(browser: *mut ui_browser) -> c_uint;
}
extern "C" {
    pub fn ui_browser__init();
}
