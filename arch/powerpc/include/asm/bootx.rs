//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/bootx.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This file describes the structure passed from the BootX application
// (for MacOS) when it is used to boot Linux.
//
// Written by Benjamin Herrenschmidt.
//

// (*) The format of the colormap is 256 * 3 * 2 bytes. Each color index
// is represented by 3 short words containing a 16 bits (unsigned) color
// component. Later versions may contain the gamma table for direct-color
// devices here.
//

// BootX passes the device-tree using a format that comes from earlier
// ppc32 kernels. This used to match what is in prom.h, but not anymore
// so we now define it here
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootx_dt_prop {
    pub name: u32,
    pub length: c_int,
    pub value: u32,
    pub next: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootx_dt_node {
    pub unused0: u32,
    pub unused1: u32,
    pub /: *mut *mut u32 phandle; / not really available,
    pub unused2: u32,
    pub unused3: u32,
    pub unused4: u32,
    pub unused5: u32,
    pub full_name: u32,
    pub properties: u32,
    pub parent: u32,
    pub child: u32,
    pub sibling: u32,
    pub next: u32,
    pub allnext: u32,
}

extern "C" {
    pub fn bootx_init(r4: c_ulong, phys: c_ulong);
}
