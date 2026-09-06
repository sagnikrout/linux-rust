//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/parport/parport_gsc.h
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
// Low-level parallel-support for PC-style hardware integrated in the
// LASI-Controller (on GSC-Bus) for HP-PARISC Workstations
//
// (C) 1999-2001 by Helge Deller <deller@gmx.de>
//
// based on parport_pc.c by
// Grant Guenther <grant@torque.net>
// Phil Blundell <Philip.Blundell@pobox.com>
// Tim Waugh <tim@cyberelk.demon.co.uk>
// Jose Renau <renau@acm.org>
// David Campbell
// Andrea Arcangeli
//

pub const DELAY_TIME: c_int = 0;

extern "C" {
    pub fn gsc_readb(_arg: port) -> return;
}

// --- register definitions -------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_gsc_private {
// Contents of CTR.
    pub ctr: c_uchar,
// Bitmask of writable CTR bits.
    pub ctr_writable: c_uchar,
// Number of bytes per portword.
    pub pword: c_int,
// Not used yet.
    pub readIntrThreshold: c_int,
    pub writeIntrThreshold: c_int,
// buffer suitable for DMA, if DMA enabled
    pub dev: *mut pci_dev,
}

// __parport_gsc_frob_control differs from parport_gsc_frob_control in that
// it doesn't do any extra masking.

// Take this out when drivers have adapted to newer interface.
// Restrict mask and val to control lines.
extern "C" {
    pub fn __parport_gsc_frob_control(_arg: p, _arg: mask, _arg: val) -> return;
}
extern "C" {
    pub fn parport_readb(_arg: STATUS(p)) -> return;
}
extern "C" {
    pub fn parport_gsc_release_resources(p: *mut parport);
}
extern "C" {
    pub fn parport_gsc_claim_resources(p: *mut parport) -> c_int;
}
extern "C" {
    pub fn parport_gsc_init_state(: *mut pardevice, s: *mut parport_state);
}
extern "C" {
    pub fn parport_gsc_save_state(p: *mut parport, s: *mut parport_state);
}
extern "C" {
    pub fn parport_gsc_restore_state(p: *mut parport, s: *mut parport_state);
}
extern "C" {
    pub fn parport_gsc_inc_use_count();
}
extern "C" {
    pub fn parport_gsc_dec_use_count();
}
