//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mvsas/mv_chips.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Marvell 88SE64xx/88SE94xx register IO interface
//
// Copyright 2007 Red Hat, Inc.
// Copyright 2008 Marvell. <kewei@marvell.com>
// Copyright 2009-2011 Marvell. <yuxiangl@marvell.com>
//

extern "C" {
    pub fn mr32(_arg: MVS_CMD_DATA) -> return;
}
// enable retry 127 times
// extend open frame timeout to max
// not to halt for different port op during wideport link change
extern "C" {
    pub fn mr32(_arg: MVS_RX_CONS_IDX) -> return;
}
extern "C" {
    pub fn sizeof(mvs_prd: struct) -> return;
}
