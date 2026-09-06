//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/auxdisplay/charlcd.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Character LCD driver for Linux
//
// Copyright (C) 2000-2008, Willy Tarreau <w@1wt.eu>
// Copyright (C) 2016-2017 Glider bvba
//
pub const LCD_FLAG_B: c_uint = 0x0004	/* Blink on */;
pub const LCD_FLAG_C: c_uint = 0x0008	/* Cursor on */;
pub const LCD_FLAG_D: c_uint = 0x0010	/* Display on */;
pub const LCD_FLAG_F: c_uint = 0x0020	/* Large font mode */;
pub const LCD_FLAG_N: c_uint = 0x0040	/* 2-rows mode */;
pub const LCD_FLAG_L: c_uint = 0x0080	/* Backlight enabled */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_onoff {
    CHARLCD_OFF = 0,
    CHARLCD_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_shift_dir {
    CHARLCD_SHIFT_LEFT,
    CHARLCD_SHIFT_RIGHT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_fontsize {
    CHARLCD_FONTSIZE_SMALL,
    CHARLCD_FONTSIZE_LARGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charlcd_lines {
    CHARLCD_LINES_1,
    CHARLCD_LINES_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct charlcd {
    pub ops: *const charlcd_ops,
    pub /: *const *const *const unsigned char char_conv; / Optional,
    pub height: c_int,
    pub width: c_int,
// Contains the LCD X and Y offset
    pub x: c_ulong,
    pub y: c_ulong,
    pub addr: },
    pub /: *mut *mut *mut void drvdata; / Set by charlcd_alloc(),
}

//
// struct charlcd_ops - Functions used by charlcd. Drivers have to implement
// these.
// @backlight: Turn backlight on or off. Optional.
// @print: Print one character to the display at current cursor position.
// The buffered cursor position is advanced by charlcd. The cursor should not
// wrap to the next line at the end of a line.
// @gotoxy: Set cursor to x, y. The x and y values to set the cursor to are
// previously set in addr.x and addr.y by charlcd.
// @home: Set cursor to 0, 0. The values in addr.x and addr.y are set to 0, 0 by
// charlcd prior to calling this function.
// @clear_display: Clear the whole display and set the cursor to 0, 0. The
// values in addr.x and addr.y are set to 0, 0 by charlcd after to calling this
// function.
// @init_display: Initialize the display.
// @shift_cursor: Shift cursor left or right one position.
// @shift_display: Shift whole display content left or right.
// @display: Turn display on or off.
// @cursor: Turn cursor on or off.
// @blink: Turn cursor blink on or off.
// @lines: One or two lines.
// @redefine_char: Redefine the actual pixel matrix of character.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charlcd_ops {
    pub on): *mut *mut *mut void (backlight)(struct charlcd lcd, enum charlcd_onoff,
    pub c): *mut *mut *mut int (print)(struct charlcd lcd, int,
    pub y): *mut *mut *mut int (gotoxy)(struct charlcd lcd, unsigned int x, unsigned int,
    pub lcd): *mut *mut int (home)(struct charlcd,
    pub lcd): *mut *mut int (clear_display)(struct charlcd,
    pub lcd): *mut *mut int (init_display)(struct charlcd,
    pub dir): *mut *mut *mut int (shift_cursor)(struct charlcd lcd, enum charlcd_shift_dir,
    pub dir): *mut *mut *mut int (shift_display)(struct charlcd lcd, enum charlcd_shift_dir,
    pub on): *mut *mut *mut int (display)(struct charlcd lcd, enum charlcd_onoff,
    pub on): *mut *mut *mut int (cursor)(struct charlcd lcd, enum charlcd_onoff,
    pub on): *mut *mut *mut int (blink)(struct charlcd lcd, enum charlcd_onoff,
    pub size): *mut *mut *mut int (fontsize)(struct charlcd lcd, enum charlcd_fontsize,
    pub lines): *mut *mut *mut int (lines)(struct charlcd lcd, enum charlcd_lines,
    pub esc): *mut *mut *mut int (redefine_char)(struct charlcd lcd, char,
}

extern "C" {
    pub fn charlcd_backlight(lcd: *mut charlcd, on: charlcd_onoff);
}
extern "C" {
    pub fn charlcd_free(lcd: *mut charlcd);
}
extern "C" {
    pub fn charlcd_register(lcd: *mut charlcd) -> c_int;
}
extern "C" {
    pub fn charlcd_unregister(lcd: *mut charlcd) -> c_int;
}
extern "C" {
    pub fn charlcd_poke(lcd: *mut charlcd);
}
