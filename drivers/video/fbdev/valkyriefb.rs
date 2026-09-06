//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/valkyriefb.h
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
// valkyriefb.h: Constants of all sorts for valkyriefb
//
// Created 8 August 1998 by
// Martin Costabel <costabel@wanadoo.fr> and Kevin Schoedel
//
// Vmode-switching changes and vmode 15/17 modifications created 29 August
// 1998 by Barry K. Nathan <barryn@pobox.com>.
//
// vmode 10 changed by Steven Borley <sjb@salix.demon.co.uk>, 14 mai 2000
//
// Ported to 68k Macintosh by David Huggins-Daines <dhd@debian.org>
//
// Based directly on:
//
// controlfb.h: Constants of all sorts for controlfb
// Copyright (C) 1998 Daniel Jacobowitz <dan@debian.org>
//
// pmc-valkyrie.h: Console support for PowerMac "control" display adaptor.
// Copyright (C) 1997 Paul Mackerras.
//
// pmc-valkyrie.c: Console support for PowerMac "control" display adaptor.
// Copyright (C) 1997 Paul Mackerras.
//
// and indirectly from:
//
// pmc-control.h: Console support for PowerMac "control" display adaptor.
// Copyright (C) 1997 Paul Mackerras.
//
// pmc-control.c: Console support for PowerMac "control" display adaptor.
// Copyright (C) 1996 Paul Mackerras.
//
// platinumfb.c: Console support for PowerMac "platinum" display adaptor.
// Copyright (C) 1998 Jon Howell
//

// Valkyrie registers are word-aligned on m68k
pub const VALKYRIE_REG_PADSIZE: c_int = 3;

pub const VALKYRIE_REG_PADSIZE: c_int = 7;

//
// Structure of the registers for the Valkyrie colormap registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmap_regs {
    pub addr: c_uchar,
    pub pad1: [c_char; VALKYRIE_REG_PADSIZE],
    pub lut: c_uchar,
}

//
// Structure of the registers for the "valkyrie" display adaptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpreg {
    pub r: c_uchar,
    pub pad: [c_char; VALKYRIE_REG_PADSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct valkyrie_regs {
    pub mode: vpreg,
    pub depth: vpreg,
    pub status: vpreg,
    pub reg3: vpreg,
    pub intr: vpreg,
    pub reg5: vpreg,
    pub intr_enb: vpreg,
    pub msense: vpreg,
}

//
// Register initialization tables for the valkyrie display.
//
// Dot clock rate is
// 3.9064MHz * 2**clock_params[2] * clock_params[1] / clock_params[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct valkyrie_regvals {
    pub mode: c_uchar,
    pub clock_params: [c_uchar; 3],
    pub /: *mut *mut int pitch[2]; / bytes/line, indexed by color_mode,
    pub hres: c_int,
    pub vres: c_int,
}

// Register values for 1024x768, 75Hz mode (17)
// I'm not sure which mode this is (16 or 17), so I'm defining it as 17,
// since the equivalent mode in controlfb (which I adapted this from) is
// also 17. Just because MacOS can't do this on Valkyrie doesn't mean we
// can't! :)
//
// I was going to use 12, 31, 3, which I found by myself, but instead I'm
// using 11, 28, 3 like controlfb, for consistency's sake.
//
// Register values for 1024x768, 72Hz mode (15)
// This used to be 12, 30, 3 for pixel clock = 78.12MHz for V=72.12Hz, but
// that didn't match MacOS in the same video mode on this chip, and it also
// caused the 15" Apple Studio Display to not work in this mode. While this
// mode still doesn't match MacOS exactly (as far as I can tell), it's a lot
// closer now, and it works with the Apple Studio Display.
//
// Yes, even though MacOS calls it "72Hz", in reality it's about 70Hz.
//
// I interpolated the V=69.71 from the vmode 14 and old 15
// numbers. Is this result correct?
//
// Register values for 1024x768, 60Hz mode (14)

// Register values for 832x624, 75Hz mode (13)
// Register values for 800x600, 72Hz mode (11)
// Register values for 800x600, 60Hz mode (10)
// Register values for 640x480, 67Hz mode (6)
// Register values for 640x480, 60Hz mode (5)

