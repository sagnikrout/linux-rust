//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ipack/devices/scc2698.h
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
// scc2698.h
//
// driver for the IPOCTAL boards
//
// Copyright (C) 2009-2012 CERN (www.cern.ch)
// Author: Nicolas Serafini, EIC2 SA
// Author: Samuel Iglesias Gonsalvez <siglesias@igalia.com>
//
// union scc2698_channel - Channel access to scc2698 IO
//
// dn value are only spacer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union scc2698_channel {
    pub 1/2*/: *mut *mut u8 d0, mr; / Mode register,
    pub /: *mut *mut u8 d1, sr; / Status register,
    pub /: *mut *mut u8 d2, r1; / reserved,
    pub /: *mut *mut u8 d3, rhr; / Receive holding register (R),
    pub /: *mut *mut u8 junk[8]; / other crap for block control,
    pub /: *mut *mut } __packed r; / Read access,
    pub /: *mut *mut u8 d0, mr; / Mode register 1/2,
    pub /: *mut *mut u8 d1, csr; / Clock select register,
    pub /: *mut *mut u8 d2, cr; / Command register,
    pub /: *mut *mut u8 d3, thr; / Transmit holding register,
    pub /: *mut *mut u8 junk[8]; / other crap for block control,
    pub /: *mut *mut } __packed w; / Write access,
}

//
// union scc2698_block - Block access to scc2698 IO
//
// The scc2698 contain 4 block.
// Each block containt two channel a and b.
// dn value are only spacer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union scc2698_block {
    pub /: *mut *mut u8 d0, mra; / Mode register 1/2 (a),
    pub /: *mut *mut u8 d1, sra; / Status register (a),
    pub /: *mut *mut u8 d2, r1; / reserved,
    pub /: *mut *mut u8 d3, rhra; / Receive holding register (a),
    pub /: *mut *mut u8 d4, ipcr; / Input port change register of block,
    pub /: *mut *mut u8 d5, isr; / Interrupt status register of block,
    pub /: *mut *mut u8 d6, ctur; / Counter timer upper register of block,
    pub /: *mut *mut u8 d7, ctlr; / Counter timer lower register of block,
    pub /: *mut *mut u8 d8, mrb; / Mode register 1/2 (b),
    pub /: *mut *mut u8 d9, srb; / Status register (b),
    pub /: *mut *mut u8 da, r2; / reserved,
    pub /: *mut *mut u8 db, rhrb; / Receive holding register (b),
    pub /: *mut *mut u8 dc, r3; / reserved,
    pub /: *mut *mut u8 dd, ip; / Input port register of block,
    pub /: *mut *mut u8 de, ctg; / Start counter timer of block,
    pub /: *mut *mut u8 df, cts; / Stop counter timer of block,
    pub /: *mut *mut } __packed r; / Read access,
    pub /: *mut *mut u8 d0, mra; / Mode register 1/2 (a),
    pub /: *mut *mut u8 d1, csra; / Clock select register (a),
    pub /: *mut *mut u8 d2, cra; / Command register (a),
    pub /: *mut *mut u8 d3, thra; / Transmit holding register (a),
    pub /: *mut *mut u8 d4, acr; / Auxiliary control register of block,
    pub /: *mut *mut u8 d5, imr; / Interrupt mask register of block,
    pub /: *mut *mut u8 d6, ctu; / Counter timer upper register of block,
    pub /: *mut *mut u8 d7, ctl; / Counter timer lower register of block,
    pub /: *mut *mut u8 d8, mrb; / Mode register 1/2 (b),
    pub /: *mut *mut u8 d9, csrb; / Clock select register (a),
    pub /: *mut *mut u8 da, crb; / Command register (b),
    pub /: *mut *mut u8 db, thrb; / Transmit holding register (b),
    pub /: *mut *mut u8 dc, r1; / reserved,
    pub /: *mut *mut u8 dd, opcr; / Output port configuration register of block,
    pub /: *mut *mut u8 de, r2; / reserved,
    pub /: *mut *mut u8 df, r3; / reserved,
    pub /: *mut *mut } __packed w; / Write access,
}

pub const ACK_INT_REQ0: c_int = 0;
pub const ACK_INT_REQ1: c_int = 2;
