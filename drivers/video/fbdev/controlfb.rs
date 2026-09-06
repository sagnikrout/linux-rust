//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/controlfb.h
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
// controlfb_hw.h: Constants of all sorts for controlfb
//
// Copyright (C) 1998 Daniel Jacobowitz <dan@debian.org>
//
// Based on an awful lot of code, including:
//
// control.c: Console support for PowerMac "control" display adaptor.
// Copyright (C) 1996 Paul Mackerras.
//
// The so far unpublished platinumfb.c
// Copyright (C) 1998 Jon Howell
//
// Structure of the registers for the RADACAL colormap device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmap_regs {
    pub /: *mut *mut unsigned char addr; / index for both cmap and misc registers,
    pub pad1: [c_char; 15],
    pub /: *mut *mut unsigned char crsr; / cursor palette,
    pub pad2: [c_char; 15],
    pub /: *mut *mut unsigned char dat; / RADACAL misc register data,
    pub pad3: [c_char; 15],
    pub /: *mut *mut unsigned char lut; / cmap data,
    pub pad4: [c_char; 15],
}

//
// Structure of the registers for the "control" display adaptor.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct preg {
    pub r: unsigned,
    pub pad: [c_char; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_regs {
    pub /: *mut *mut preg vcount; / vertical counter,
// Vertical parameters are in units of 1/2 scan line
    pub /: *mut *mut preg vswin; / between vsblank and vssync,
    pub /: *mut *mut preg vsblank; / vert start blank,
    pub /: *mut *mut preg veblank; / vert end blank (display start),
    pub /: *mut *mut preg vewin; / between vesync and veblank,
    pub /: *mut *mut preg vesync; / vert end sync,
    pub /: *mut *mut preg vssync; / vert start sync,
    pub /: *mut *mut preg vperiod; / vert period,
    pub /: *mut *mut preg piped; / pipe delay hardware cursor,
// Horizontal params are in units of 2 pixels
    pub /: *mut *mut preg hperiod; / horiz period - 2,
    pub /: *mut *mut preg hsblank; / horiz start blank,
    pub /: *mut *mut preg heblank; / horiz end blank,
    pub /: *mut *mut preg hesync; / horiz end sync,
    pub /: *mut *mut preg hssync; / horiz start sync,
    pub /: *mut *mut preg heq; / half horiz sync len,
    pub /: *mut *mut preg hlfln; / half horiz period,
    pub /: *mut *mut preg hserr; / horiz period - horiz sync len,
    pub cnttst: preg,
    pub /: *mut *mut preg ctrl; / display control,
    pub /: *mut *mut preg start_addr; / start address: 5 lsbs zero,
    pub /: *mut *mut preg pitch; / addrs diff between scan lines,
    pub /: *mut *mut preg mon_sense; / monitor sense bits,
    pub /: *mut *mut preg vram_attr; / enable vram banks,
    pub mode: preg,
    pub /: *mut *mut preg rfrcnt; / refresh count,
    pub /: *mut *mut preg intr_ena; / interrupt enable,
    pub /: *mut *mut preg intr_stat; / interrupt status,
    pub res: [preg; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_regints {
// Vertical parameters are in units of 1/2 scan line
    pub /: *mut *mut unsigned vswin; / between vsblank and vssync,
    pub /: *mut *mut unsigned vsblank; / vert start blank,
    pub /: *mut *mut unsigned veblank; / vert end blank (display start),
    pub /: *mut *mut unsigned vewin; / between vesync and veblank,
    pub /: *mut *mut unsigned vesync; / vert end sync,
    pub /: *mut *mut unsigned vssync; / vert start sync,
    pub /: *mut *mut unsigned vperiod; / vert period,
    pub /: *mut *mut unsigned piped; / pipe delay hardware cursor,
// Horizontal params are in units of 2 pixels
// Except, apparently, for hres > 1024 (or == 1280?)
    pub /: *mut *mut unsigned hperiod; / horiz period - 2,
    pub /: *mut *mut unsigned hsblank; / horiz start blank,
    pub /: *mut *mut unsigned heblank; / horiz end blank,
    pub /: *mut *mut unsigned hesync; / horiz end sync,
    pub /: *mut *mut unsigned hssync; / horiz start sync,
    pub /: *mut *mut unsigned heq; / half horiz sync len,
    pub /: *mut *mut unsigned hlfln; / half horiz period,
    pub /: *mut *mut unsigned hserr; / horiz period - horiz sync len,
}

//
// Dot clock rate is
// 3.9064MHz * 2**clock_params[2] * clock_params[1] / clock_params[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_regvals {
    pub /: *mut *mut unsigned regs[16]; / for vswin .. hserr,
    pub mode: c_uchar,
    pub radacal_ctrl: c_uchar,
    pub clock_params: [c_uchar; 3],
}

//
// Best cmode supported by control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max_cmodes {
    pub /: *mut *mut int m[2]; / 0: 2MB vram, 1: 4MB vram,
}

//
// Video modes supported by macmodes.c
//
