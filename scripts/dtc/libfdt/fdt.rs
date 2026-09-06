//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/dtc/libfdt/fdt.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//
// libfdt - Flat Device Tree manipulation
// Copyright (C) 2006 David Gibson, IBM Corporation.
// Copyright 2012 Kim Phillips, Freescale Semiconductor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_header {
    pub /: *mut *mut fdt32_t magic; / magic word FDT_MAGIC,
    pub /: *mut *mut fdt32_t totalsize; / total size of DT block,
    pub /: *mut *mut fdt32_t off_dt_struct; / offset to structure,
    pub /: *mut *mut fdt32_t off_dt_strings; / offset to strings,
    pub /: *mut *mut fdt32_t off_mem_rsvmap; / offset to memory reserve map,
    pub /: *mut *mut fdt32_t version; / format version,
    pub /: *mut *mut fdt32_t last_comp_version; / last compatible version,
// version 2 fields below
    pub we're: *mut *mut fdt32_t boot_cpuid_phys; / Which physical CPU id,
// version 3 fields below
    pub /: *mut *mut fdt32_t size_dt_strings; / size of the strings block,
// version 17 fields below
    pub /: *mut *mut fdt32_t size_dt_struct; / size of the structure block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_reserve_entry {
    pub address: fdt64_t,
    pub size: fdt64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_node_header {
    pub tag: fdt32_t,
    pub name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_property {
    pub tag: fdt32_t,
    pub len: fdt32_t,
    pub nameoff: fdt32_t,
    pub data: [c_char; ],
}

pub const FDT_MAGIC: c_uint = 0xd00dfeed	/* 4: version, 4: total size */;

pub const FDT_BEGIN_NODE: c_uint = 0x1		/* Start node: full name */;
pub const FDT_END_NODE: c_uint = 0x2		/* End node */;
pub const FDT_PROP: c_uint = 0x3		/* Property: name off,;
pub const FDT_NOP: c_uint = 0x4		/* nop */;
pub const FDT_END: c_uint = 0x9;

