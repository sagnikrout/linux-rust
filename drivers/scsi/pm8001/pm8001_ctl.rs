//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/pm8001/pm8001_ctl.h
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
// PMC-Sierra SPC 8001 SAS/SATA based host adapters driver
//
// Copyright (c) 2008-2009 USI Co., Ltd.
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

// Macro flag: #define PM8001_CTL_H_INCLUDED
pub const IOCTL_BUF_SIZE: c_int = 4096;
pub const HEADER_LEN: c_int = 28;
pub const SIZE_OFFSET: c_int = 16;
pub const BIOSOFFSET: c_int = 56;
pub const BIOS_OFFSET_LIMIT: c_int = 61;
pub const FLASH_OK: c_uint = 0x000000;
pub const FAIL_OPEN_BIOS_FILE: c_uint = 0x000100;
pub const FAIL_FILE_SIZE: c_uint = 0x000a00;
pub const FAIL_PARAMETERS: c_uint = 0x000b00;
pub const FAIL_OUT_MEMORY: c_uint = 0x000c00;
pub const FLASH_IN_PROGRESS: c_uint = 0x001000;
pub const IB_OB_READ_TIMES: c_int = 256;
pub const SYSFS_OFFSET: c_int = 1024;

