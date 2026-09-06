//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aic79xx_pci.h
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
// Adaptec AIC79xx device driver for Linux.
//
// Copyright (c) 2000-2001 Adaptec Inc.
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
// $Id$
//
pub const ID_ALL_MASK: c_uint = 0xFFFFFFFFFFFFFFFFull;
pub const ID_ALL_IROC_MASK: c_uint = 0xFF7FFFFFFFFFFFFFull;
pub const ID_DEV_VENDOR_MASK: c_uint = 0xFFFFFFFF00000000ull;
pub const ID_9005_GENERIC_MASK: c_uint = 0xFFF0FFFF00000000ull;
pub const ID_9005_GENERIC_IROC_MASK: c_uint = 0xFF70FFFF00000000ull;
pub const ID_AIC7901: c_uint = 0x800F9005FFFF9005ull;
pub const ID_AHA_29320A: c_uint = 0x8000900500609005ull;
pub const ID_AHA_29320ALP: c_uint = 0x8017900500449005ull;
pub const ID_AHA_29320LPE: c_uint = 0x8017900500459005ull;
pub const ID_AIC7901A: c_uint = 0x801E9005FFFF9005ull;
pub const ID_AHA_29320LP: c_uint = 0x8014900500449005ull;
pub const ID_AIC7902: c_uint = 0x801F9005FFFF9005ull;
pub const ID_AIC7902_B: c_uint = 0x801D9005FFFF9005ull;
pub const ID_AHA_39320: c_uint = 0x8010900500409005ull;
pub const ID_AHA_29320: c_uint = 0x8012900500429005ull;
pub const ID_AHA_29320B: c_uint = 0x8013900500439005ull;
pub const ID_AHA_39320_B: c_uint = 0x8015900500409005ull;
pub const ID_AHA_39320_B_DELL: c_uint = 0x8015900501681028ull;
pub const ID_AHA_39320A: c_uint = 0x8016900500409005ull;
pub const ID_AHA_39320D: c_uint = 0x8011900500419005ull;
pub const ID_AHA_39320D_B: c_uint = 0x801C900500419005ull;
pub const ID_AHA_39320D_HP: c_uint = 0x8011900500AC0E11ull;
pub const ID_AHA_39320D_B_HP: c_uint = 0x801C900500AC0E11ull;
