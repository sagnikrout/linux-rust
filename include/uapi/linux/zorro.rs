//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/zorro.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// linux/zorro.h -- Amiga AutoConfig (Zorro) Bus Definitions
//
// Copyright (C) 1995--2003 Geert Uytterhoeven
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

//
// Each Zorro board has a 32-bit ID of the form
//
// mmmmmmmmmmmmmmmmppppppppeeeeeeee
//
// with
//
// mmmmmmmmmmmmmmmm	16-bit Manufacturer ID (assigned by CBM (sigh))
// pppppppp		8-bit Product ID (assigned by manufacturer)
// eeeeeeee		8-bit Extended Product ID (currently only used
// for some GVP boards)
//

pub type zorro_id = __u32;
// Include the ID list

//
// GVP identifies most of its products through the 'extended product code'
// (epc). The epc has to be ANDed with the GVP_PRODMASK before the
// identification.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GVP_flags {
    GVP_IO			= 0x01,
    GVP_ACCEL		= 0x02,
    GVP_SCSI		= 0x04,
    GVP_24BITDMA		= 0x08,
    GVP_25BITDMA		= 0x10,
    GVP_NOBANK		= 0x20,
    GVP_14MHZ		= 0x40,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Node {
    pub /: *mut *mut __be32 ln_Succ; / Pointer to next (successor),
    pub /: *mut *mut __be32 ln_Pred; / Pointer to previous (predecessor),
    pub ln_Type: __u8,
    pub /: *mut *mut __s8 ln_Pri; / Priority, for sorting,
    pub /: *mut *mut __be32 ln_Name; / ID string, null terminated,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExpansionRom {
// -First 16 bytes of the expansion ROM
    pub /: *mut *mut __u8 er_Type; / Board type, size and flags,
    pub /: *mut *mut __u8 er_Product; / Product number, assigned by manufacturer,
    pub /: *mut *mut __u8 er_Flags; / Flags,
    pub /: *mut *mut __u8 er_Reserved03; / Must be zero ($ff inverted),
    pub /: *mut *mut __be16 er_Manufacturer; / Unique ID, ASSIGNED BY COMMODORE-AMIGA!,
    pub /: *mut *mut __be32 er_SerialNumber; / Available for use by manufacturer,
    pub /: *mut *mut __be16 er_InitDiagVec; / Offset to optional "DiagArea" structure,
    pub er_Reserved0c: __u8,
    pub er_Reserved0d: __u8,
    pub er_Reserved0e: __u8,
    pub er_Reserved0f: __u8,
    pub __packed: },
// er_Type board type bits
pub const ERT_TYPEMASK: c_uint = 0xc0;
pub const ERT_ZORROII: c_uint = 0xc0;
pub const ERT_ZORROIII: c_uint = 0x80;
// other bits defined in er_Type

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigDev {
    pub cd_Node: Node,
    pub /: *mut *mut __u8 cd_Flags; / (read/write),
    pub /: *mut *mut __u8 cd_Pad; / reserved,
    pub /: *mut *mut ExpansionRom cd_Rom; / copy of board's expansion ROM,
    pub /: *mut *mut __be32 cd_BoardAddr; / where in memory the board was placed,
    pub /: *mut *mut __be32 cd_BoardSize; / size of board in bytes,
    pub /: *mut *mut __be16 cd_SlotAddr; / which slot number (PRIVATE),
    pub /: *mut *mut __be16 cd_SlotSize; / number of slots (PRIVATE),
    pub /: *mut *mut __be32 cd_Driver; / pointer to node of driver,
    pub /: *mut *mut __be32 cd_NextCD; / linked list of drivers to config,
    pub /: *mut *mut __be32 cd_Unused[4]; / for whatever the driver wants,
    pub __packed: },
pub const ZORRO_NUM_AUTO: c_int = 16;
