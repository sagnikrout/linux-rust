//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/atmel/pmecc.h
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
//
// © Copyright 2016 ATMEL
// © Copyright 2016 Free Electrons
//
// Author: Boris Brezillon <boris.brezillon@free-electrons.com>
//
// Derived from the atmel_nand.c driver which contained the following
// copyrights:
//
// Copyright © 2003 Rick Bronson
//
// Derived from drivers/mtd/nand/autcpu12.c (removed in v3.8)
// Copyright © 2001 Thomas Gleixner (gleixner@autronix.de)
//
// Derived from drivers/mtd/spia.c (removed in v3.8)
// Copyright © 2000 Steven J. Hill (sjhill@cotw.com)
//
// Add Hardware ECC support for AT91SAM9260 / AT91SAM9263
// Richard Genoud (richard.genoud@gmail.com), Adeneo Copyright © 2007
//
// Derived from Das U-Boot source code
// (u-boot-1.1.5/board/atmel/at91sam9263ek/nand.c)
// © Copyright 2006 ATMEL Rousset, Lacressonniere Nicolas
//
// Add Programmable Multibit ECC support for various AT91 SoC
// © Copyright 2012 ATMEL, Hong Xu
//
// Add Nand Flash Controller support for SAMA5 SoC
// © Copyright 2013 ATMEL, Josh Wu (josh.wu@atmel.com)
//
pub const ATMEL_PMECC_MAXIMIZE_ECC_STRENGTH: c_int = 0;
pub const ATMEL_PMECC_SECTOR_SIZE_AUTO: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_pmecc_user_req {
    pub pagesize: c_int,
    pub oobsize: c_int,
    pub strength: c_int,
    pub bytes: c_int,
    pub sectorsize: c_int,
    pub nsectors: c_int,
    pub ooboffset: c_int,
    pub ecc: },
}

extern "C" {
    pub fn atmel_pmecc_reset(pmecc: *mut atmel_pmecc);
}
extern "C" {
    pub fn atmel_pmecc_enable(user: *mut atmel_pmecc_user, op: c_int) -> c_int;
}
extern "C" {
    pub fn atmel_pmecc_disable(user: *mut atmel_pmecc_user);
}
extern "C" {
    pub fn atmel_pmecc_wait_rdy(user: *mut atmel_pmecc_user) -> c_int;
}
extern "C" {
    pub fn atmel_pmecc_correct_erased_chunks(user: *mut atmel_pmecc_user) -> bool;
}
