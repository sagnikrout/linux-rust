//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/kconfig/lxdialog/dialog.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// dialog.h -- common declarations for all dialog modules
//
// AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
//

// Macro flag: #define CURS_MACROS

pub const KEY_ESC: c_int = 27;
pub const TAB: c_int = 9;
pub const MAX_LEN: c_int = 2048;

// error return codes

//
// Color definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dialog_color {
    pub /: *mut *mut chtype atr; / Color attribute,
    pub /: *mut *mut int fg; / foreground,
    pub /: *mut *mut int bg; / background,
    pub /: *mut *mut int hl; / highlight this item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subtitle_list {
    pub next: *mut subtitle_list,
    pub text: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dialog_info {
    pub backtitle: *const c_char,
    pub subtitles: *mut subtitle_list,
    pub screen: dialog_color,
    pub shadow: dialog_color,
    pub dialog: dialog_color,
    pub title: dialog_color,
    pub border: dialog_color,
    pub button_active: dialog_color,
    pub button_inactive: dialog_color,
    pub button_key_active: dialog_color,
    pub button_key_inactive: dialog_color,
    pub button_label_active: dialog_color,
    pub button_label_inactive: dialog_color,
    pub inputbox: dialog_color,
    pub position_indicator: dialog_color,
    pub menubox: dialog_color,
    pub menubox_border: dialog_color,
    pub item: dialog_color,
    pub item_selected: dialog_color,
    pub tag: dialog_color,
    pub tag_selected: dialog_color,
    pub tag_key: dialog_color,
    pub tag_key_selected: dialog_color,
    pub check: dialog_color,
    pub check_selected: dialog_color,
    pub uarrow: dialog_color,
    pub darrow: dialog_color,
}

//
// Global variables
//
// Function prototypes
//
// item list as used by checklist and menubox
extern "C" {
    pub fn item_reset();
}
extern "C" {
    pub fn item_make(fmt: *const c_char, ...);
}
extern "C" {
    pub fn item_add_str(fmt: *const c_char, ...);
}
extern "C" {
    pub fn item_set_tag(tag: c_char);
}
extern "C" {
    pub fn item_set_data(p: *mut c_void);
}
extern "C" {
    pub fn item_set_selected(val: c_int);
}
extern "C" {
    pub fn item_activate_selected() -> c_int;
}
extern "C" {
    pub fn item_tag() -> c_char;
}
// item list manipulation for lxdialog use
pub const MAXITEMSTR: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dialog_item {
    pub /: *mut *mut char str[MAXITEMSTR]; / prompt displayed,
    pub tag: c_char,
    pub /: *mut *mut *mut void data; / pointer to menu item - used by menubox+checklist,
    pub /: *mut *mut *mut int selected; / Set to 1 by dialog_() function if selected.,
}

// list of lialog_items
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dialog_list {
    pub node: dialog_item,
    pub next: *mut dialog_list,
}

extern "C" {
    pub fn item_count() -> c_int;
}
extern "C" {
    pub fn item_set(n: c_int);
}
extern "C" {
    pub fn item_n() -> c_int;
}
extern "C" {
    pub fn item_is_selected() -> c_int;
}
extern "C" {
    pub fn item_is_tag(tag: c_char) -> c_int;
}

// generic key handlers
extern "C" {
    pub fn on_key_esc(win: *mut WINDOW) -> c_int;
}
extern "C" {
    pub fn on_key_resize() -> c_int;
}
// minimum (re)size values

pub const CHECKLIST_WIDTH_MIN: c_int = 6;

pub const INPUTBOX_WIDTH_MIN: c_int = 2;

pub const MENUBOX_WIDTH_MIN: c_int = 65;

pub const TEXTBOX_WIDTH_MIN: c_int = 8;

pub const YESNO_WIDTH_MIN: c_int = 4;

pub const WINDOW_WIDTH_MIN: c_int = 80;
extern "C" {
    pub fn init_dialog(backtitle: *const c_char) -> c_int;
}
extern "C" {
    pub fn set_dialog_backtitle(backtitle: *const c_char);
}
extern "C" {
    pub fn set_dialog_subtitles(subtitles: *mut subtitle_list);
}
extern "C" {
    pub fn end_dialog(x: c_int, y: c_int);
}
extern "C" {
    pub fn attr_clear(win: *mut *mut WINDOW, height: c_int, width: c_int, attr: chtype);
}
extern "C" {
    pub fn dialog_clear();
}
extern "C" {
    pub fn print_autowrap(win: *mut *mut WINDOW, prompt: *const c_char, width: c_int, y: c_int, x: c_int);
}
extern "C" {
    pub fn print_button(win: *mut *mut WINDOW, label: *const c_char, y: c_int, x: c_int, selected: c_int);
}
extern "C" {
    pub fn print_title(dialog: *mut WINDOW, title: *const c_char, width: c_int);
}
extern "C" {
    pub fn draw_shadow(win: *mut *mut WINDOW, y: c_int, x: c_int, height: c_int, width: c_int);
}
extern "C" {
    pub fn first_alpha(string: *const c_char, exempt: *const c_char) -> c_int;
}
extern "C" {
    pub fn dialog_yesno(title: *const c_char, prompt: *const c_char, height: c_int, width: c_int) -> c_int;
}
