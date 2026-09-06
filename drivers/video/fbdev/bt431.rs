//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/bt431.h
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


//
// linux/drivers/video/bt431.h
//
// Copyright 2003  Thiemo Seufer <seufer@csv.ica.uni-stuttgart.de>
// Copyright 2016  Maciej W. Rozycki <macro@linux-mips.org>
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file COPYING in the main directory of this
// archive for more details.
//

pub const BT431_CURSOR_SIZE: c_int = 64;
//
// Bt431 cursor generator registers, 32-bit aligned.
// Two twin Bt431 are used on the DECstation's PMAG-AA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt431_regs {
    pub addr_lo: volatile u16,
    pub pad0: u16,
    pub addr_hi: volatile u16,
    pub pad1: u16,
    pub addr_cmap: volatile u16,
    pub pad2: u16,
    pub addr_reg: volatile u16,
    pub pad3: u16,
}

//
// Additional registers addressed indirectly.
//
pub const BT431_REG_CMD: c_uint = 0x0000;
pub const BT431_REG_CXLO: c_uint = 0x0001;
pub const BT431_REG_CXHI: c_uint = 0x0002;
pub const BT431_REG_CYLO: c_uint = 0x0003;
pub const BT431_REG_CYHI: c_uint = 0x0004;
pub const BT431_REG_WXLO: c_uint = 0x0005;
pub const BT431_REG_WXHI: c_uint = 0x0006;
pub const BT431_REG_WYLO: c_uint = 0x0007;
pub const BT431_REG_WYHI: c_uint = 0x0008;
pub const BT431_REG_WWLO: c_uint = 0x0009;
pub const BT431_REG_WWHI: c_uint = 0x000a;
pub const BT431_REG_WHLO: c_uint = 0x000b;
pub const BT431_REG_WHHI: c_uint = 0x000c;
pub const BT431_REG_CRAM_BASE: c_uint = 0x0000;
pub const BT431_REG_CRAM_END: c_uint = 0x01ff;
//
// Command register.
//
pub const BT431_CMD_CURS_ENABLE: c_uint = 0x40;
pub const BT431_CMD_XHAIR_ENABLE: c_uint = 0x20;
pub const BT431_CMD_OR_CURSORS: c_uint = 0x10;
pub const BT431_CMD_XOR_CURSORS: c_uint = 0x00;
pub const BT431_CMD_1_1_MUX: c_uint = 0x00;
pub const BT431_CMD_4_1_MUX: c_uint = 0x04;
pub const BT431_CMD_5_1_MUX: c_uint = 0x08;
pub const BT431_CMD_xxx_MUX: c_uint = 0x0c;
pub const BT431_CMD_THICK_1: c_uint = 0x00;
pub const BT431_CMD_THICK_3: c_uint = 0x01;
pub const BT431_CMD_THICK_5: c_uint = 0x02;
pub const BT431_CMD_THICK_7: c_uint = 0x03;
//
// The compiler splits the write in two bytes without these
// helper variables.
//
// lo = bt431_set_value(ir & 0xff);
// hi = bt431_set_value((ir >> 8) & 0xff);
// Autoincrement read/write.
//
// The compiler splits the write in two bytes without the
// helper variable.
//
extern "C" {
    pub fn bt431_get_value(_arg: *mut r) -> return;
}
//
// The compiler splits the write in two bytes without the
// helper variable.
//
// r = bt431_set_value(value);
extern "C" {
    pub fn bt431_read_reg_inc(_arg: regs) -> return;
}
// Autoincremented read/write for the cursor map.
//
// The compiler splits the write in two bytes without the
// helper variable.
//
// The compiler splits the write in two bytes without the
// helper variable.
//
// r = value;
extern "C" {
    pub fn bt431_read_cmap_inc(_arg: regs) -> return;
}
//
// Magic from the MACH sources.
//
// Cx = x + D + H - P
// P = 37 if 1:1, 52 if 4:1, 57 if 5:1
// D = pixel skew between outdata and external data
// H = pixels between HSYNCH falling and active video
//
// Cy = y + V - 32
// V = scanlines between HSYNCH falling, two or more
// clocks after VSYNCH falling, and active video
//
// Use autoincrement.
// no crosshair window
