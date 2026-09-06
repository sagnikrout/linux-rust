//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uv/uv_geo.h
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
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2020 Hewlett Packard Enterprise Development LP. All rights reserved.
//
// Type declarations
// Size of a geoid_s structure (must be before decl. of geoid_u)
pub const GEOID_SIZE: c_int = 8;
// Fields common to all substructures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_common_s {
    pub /: *mut *mut unsigned char type; / What type of h/w is named by this geoid_s,
    pub blade: c_uchar,
    pub /: *mut *mut unsigned char slot; / slot is IRU,
    pub upos: c_uchar,
    pub rack: c_uchar,
}

// Additional fields for particular types of hardware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_node_s {
    pub /: *mut *mut geo_common_s common; / No additional fields needed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_rtr_s {
    pub /: *mut *mut geo_common_s common; / No additional fields needed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_iocntl_s {
    pub /: *mut *mut geo_common_s common; / No additional fields needed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_pcicard_s {
    pub common: geo_iocntl_s,
    pub /: *mut *mut char bus; / Bus/widget number,
    pub /: *mut *mut char slot; / PCI slot number,
}

// Subcomponents of a node
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_cpu_s {
    pub node: geo_node_s,
    pub core: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geo_mem_s {
    pub node: geo_node_s,
    pub /: *mut *mut char membus; / The memory bus on the node,
    pub /: *mut *mut char memslot; / The memory slot on the bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union geoid_u {
    pub common: geo_common_s,
    pub node: geo_node_s,
    pub iocntl: geo_iocntl_s,
    pub pcicard: geo_pcicard_s,
    pub rtr: geo_rtr_s,
    pub cpu: geo_cpu_s,
    pub mem: geo_mem_s,
    pub padsize: [c_char; GEOID_SIZE],
}

// Defined constants
pub const GEO_MAX_LEN: c_int = 48;
pub const GEO_TYPE_INVALID: c_int = 0;
pub const GEO_TYPE_MODULE: c_int = 1;
pub const GEO_TYPE_NODE: c_int = 2;
pub const GEO_TYPE_RTR: c_int = 3;
pub const GEO_TYPE_IOCNTL: c_int = 4;
pub const GEO_TYPE_IOCARD: c_int = 5;
pub const GEO_TYPE_CPU: c_int = 6;
pub const GEO_TYPE_MEM: c_int = 7;

