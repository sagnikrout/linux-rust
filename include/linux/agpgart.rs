//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/agpgart.h
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
// AGPGART module version 0.99
// Copyright (C) 1999 Jeff Hartmann
// Copyright (C) 1999 Precision Insight, Inc.
// Copyright (C) 1999 Xi Graphics, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// JEFF HARTMANN, OR ANY OTHER CONTRIBUTORS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
pub const _AGP_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_info {
    pub /: *mut *mut agp_version version; / version of the driver,
    pub /: *mut *mut u32 bridge_id; / bridge vendor/device,
    pub /: *mut *mut u32 agp_mode; / mode info of bridge,
    pub /: *mut *mut unsigned long aper_base;/ base of aperture,
    pub /: *mut *mut size_t aper_size; / size of aperture,
    pub /: *mut *mut size_t pg_total; / max pages (swap + system),
    pub /: *mut *mut size_t pg_system; / max pages (system),
    pub /: *mut *mut size_t pg_used; / current pages used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_setup {
    pub /: *mut *mut u32 agp_mode; / mode info of bridge,
}

//
// The "prot" down below needs still a "sleep" flag somehow ...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_segment {
    pub /: *mut *mut off_t pg_start; / starting page to populate,
    pub /: *mut *mut size_t pg_count; / number of pages,
    pub /: *mut *mut int prot; / prot flags for mmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_segment_priv {
    pub pg_start: off_t,
    pub pg_count: usize,
    pub prot: pgprot_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_region {
    pub /: *mut *mut pid_t pid; / pid of process,
    pub /: *mut *mut size_t seg_count; / number of segments,
    pub seg_list: *mut agp_segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_allocate {
    pub /: *mut *mut int key; / tag of allocation,
    pub /: *mut *mut size_t pg_count; / number of pages,
    pub /: *mut *mut u32 type; / 0 == normal, other devspec,
    pub devices: *mut *mut u32 physical; / device specific (some,
// need a phys address of the
// actual page behind the gatt
// table)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_bind {
    pub /: *mut *mut int key; / tag of allocation,
    pub /: *mut *mut off_t pg_start; / starting page to populate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_unbind {
    pub /: *mut *mut int key; / tag of allocation,
    pub /: *mut *mut u32 priority; / priority for paging out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_client {
    pub next: *mut agp_client,
    pub prev: *mut agp_client,
    pub pid: pid_t,
    pub num_segments: c_int,
    pub segments: *mut agp_segment_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_controller {
    pub next: *mut agp_controller,
    pub prev: *mut agp_controller,
    pub pid: pid_t,
    pub num_clients: c_int,
    pub pool: *mut agp_memory,
    pub clients: *mut agp_client,
}

pub const AGP_FF_ALLOW_CLIENT: c_int = 0;
pub const AGP_FF_ALLOW_CONTROLLER: c_int = 1;
pub const AGP_FF_IS_CLIENT: c_int = 2;
pub const AGP_FF_IS_CONTROLLER: c_int = 3;
pub const AGP_FF_IS_VALID: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_file_private {
    pub next: *mut agp_file_private,
    pub prev: *mut agp_file_private,
    pub my_pid: pid_t,
    pub /: *mut *mut unsigned long access_flags; / long req'd for set_bit --RR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_front_data {
    pub agp_mutex: mutex,
    pub current_controller: *mut agp_controller,
    pub controllers: *mut agp_controller,
    pub file_priv_list: *mut agp_file_private,
    pub used_by_controller: bool,
    pub backend_acquired: bool,
}
