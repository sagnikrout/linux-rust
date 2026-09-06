//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aic79xx_inline.h
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
// Inline routines shareable across OS platforms.
//
// Copyright (c) 1994-2001 Justin T. Gibbs.
// Copyright (c) 2000-2003 Adaptec Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//
// $Id: //depot/aic7xxx/aic7xxx/aic79xx_inline.h#59 $
//
// $FreeBSD$
//
// Debugging
// Sequencer Execution Control
extern "C" {
    pub fn ahd_save_modes(ahd: *mut ahd_softc) -> ahd_mode_state;
}
extern "C" {
    pub fn ahd_is_paused(ahd: *mut ahd_softc) -> c_int;
}
extern "C" {
    pub fn ahd_pause(ahd: *mut ahd_softc);
}
extern "C" {
    pub fn ahd_unpause(ahd: *mut ahd_softc);
}
// src = (state & SRC_MODE) >> SRC_MODE_SHIFT;
// dst = (state & DST_MODE) >> DST_MODE_SHIFT;
// Scatter Gather List Handling
// Memory mapping routines
extern "C" {
    pub fn ahd_sg_size(ahd: *mut ahd_softc) -> usize;
}
// Miscellaneous Support Functions
extern "C" {
    pub fn ahd_get_scbptr(ahd: *mut ahd_softc) -> u_int;
}
extern "C" {
    pub fn ahd_set_scbptr(ahd: *mut ahd_softc, scbptr: u_int);
}
extern "C" {
    pub fn ahd_inb_scbram(ahd: *mut ahd_softc, offset: u_int) -> u_int;
}
extern "C" {
    pub fn ahd_inw_scbram(ahd: *mut ahd_softc, offset: u_int) -> u_int;
}
extern "C" {
    pub fn ahd_queue_scb(ahd: *mut ahd_softc, scb: *mut scb);
}

// Interrupt Processing
extern "C" {
    pub fn ahd_intr(ahd: *mut ahd_softc) -> c_int;
}
