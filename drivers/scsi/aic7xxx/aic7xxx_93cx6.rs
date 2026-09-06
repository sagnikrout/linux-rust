//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aic7xxx_93cx6.h
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
// Interface to the 93C46/56 serial EEPROM that is used to store BIOS
// settings for the aic7xxx based adaptec SCSI controllers.  It can
// also be used for 93C26 and 93C06 serial EEPROMS.
//
// Copyright (c) 1994, 1995, 2000 Justin T. Gibbs.
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
// $Id: //depot/aic7xxx/aic7xxx/aic7xxx_93cx6.h#12 $
//
// $FreeBSD$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seeprom_descriptor {
    pub sd_ahc: *mut ahc_softc,
    pub sd_control_offset: u_int,
    pub sd_status_offset: u_int,
    pub sd_dataout_offset: u_int,
    pub sd_chip: seeprom_chip_t,
    pub sd_MS: u16,
    pub sd_RDY: u16,
    pub sd_CS: u16,
    pub sd_CK: u16,
    pub sd_DO: u16,
    pub sd_DI: u16,
}

//
// This function will read count 16-bit words from the serial EEPROM and
// return their value in buf.  The port address of the aic7xxx serial EEPROM
// control register is passed in as offset.  The following parameters are
// also passed in:
//
// CS  - Chip select
// CK  - Clock
// DO  - Data out
// DI  - Data in
// RDY - SEEPROM ready
// MS  - Memory port mode select
//
// A failed read attempt returns 0, and a successful read returns 1.
//

extern "C" {
    pub fn ahc_verify_cksum(sc: *mut seeprom_config) -> c_int;
}
