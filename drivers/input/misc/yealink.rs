//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/misc/yealink.h
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
// drivers/usb/input/yealink.h
//
// Copyright (c) 2005 Henk Vergonet <Henk.Vergonet@gmail.com>
//
// Using the control channel on interface 3 various aspects of the phone
// can be controlled like LCD, LED, dialtone and the ringtone.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yld_ctl_packet {
    pub /: *mut *mut u8 cmd; / command code, see below,
    pub /: *mut *mut u8 size; / 1-11, size of used data bytes.,
    pub /: *mut *mut __be16 offset; / internal packet offset,
    pub data: [u8; 11],
    pub /: *mut *mut s8 sum; / negative sum of 15 preceding bytes,
// C attribute field omitted

// The following yld_ctl_packet's are available:
// Init registers
//
// cmd		0x8e
// size		10
// offset	0
// data		0,0,0,0....
//
pub const CMD_INIT: c_uint = 0x8e;
// Request key scan
//
// cmd		0x80
// size		1
// offset	0
// data[0]	on return returns the key number, if it changes there's a new
// key pressed.
//
pub const CMD_KEYPRESS: c_uint = 0x80;
// Request scancode
//
// cmd		0x81
// size		1
// offset	key number [0-1f]
// data[0]	on return returns the scancode
//
pub const CMD_SCANCODE: c_uint = 0x81;
// Set LCD
//
// cmd		0x04
// size		1-11
// offset	0-23
// data		segment bits
//
pub const CMD_LCD: c_uint = 0x04;
// Set led
//
// cmd		0x05
// size		1
// offset	0
// data[0]	0 OFF / 1 ON
//
pub const CMD_LED: c_uint = 0x05;
// Set ringtone volume
//
// cmd		0x11
// size		1
// offset	0
// data[0]	0-0xff  volume
//
pub const CMD_RING_VOLUME: c_uint = 0x11;
// Set ringtone notes
//
// cmd		0x02
// size		1-11
// offset	0->
// data		binary representation LE16(-freq), LE16(duration) ....
//
pub const CMD_RING_NOTE: c_uint = 0x02;
// Sound ringtone via the speaker on the back
//
// cmd		0x03
// size		1
// offset	0
// data[0]	0 OFF / 0x24 ON
//
pub const CMD_RINGTONE: c_uint = 0x03;
// Sound dial tone via the ear speaker
//
// cmd		0x09
// size		1
// offset	0
// data[0]	0 OFF / 1 ON
//
pub const CMD_DIALTONE: c_uint = 0x09;

// This table maps the LCD segments onto individual bit positions in the
// yld_status struct.
//
// LCD, each segment must be driven separately.
//
// Layout:
//
// |[]   [][]   [][]   [][]   in   |[][]
// |[] M [][] D [][] : [][]   out  |[][]
// store
//
// NEW REP         SU MO TU WE TH FR SA
//
// [] [] [] [] [] [] [] [] [] [] [] []
//
// Line 1
// Format		: 18.e8.M8.88...188
// Icon names	: M D : IN OUT STORE
//
pub const LCD_LINE1_OFFSET: c_int = 0;
pub const LCD_LINE1_SIZE: c_int = 17;
// Note: first g then f =>			       !      !
// _SEG(    type    a      b      c      d      e      g      f   )
// Line 2
// Format		: .........
// Pict. name	: NEW REP SU MO TU WE TH FR SA
//

pub const LCD_LINE2_SIZE: c_int = 9;
// Line 3
// Format		: 888888888888
//

pub const LCD_LINE3_SIZE: c_int = 12;
// Line 4
//
// The LED, DIALTONE and RINGTONE are implemented as icons and use the same
// sysfs interface.
//

