//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/omap_drm.h
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
// include/uapi/drm/omap_drm.h
//
// Copyright (C) 2011 Texas Instruments
// Author: Rob Clark <rob@ti.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <http://www.gnu.org/licenses/>.
//

// Please note that modifications to all structs defined here are
// subject to backwards-compatibility constraints.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_omap_param {
    pub /: *mut *mut __u64 param; / in,
    pub /: *mut *mut __u64 value; / in (set_param), out (get_param),
}

// Scanout buffer, consumable by DSS
pub const OMAP_BO_SCANOUT: c_uint = 0x00000001;
// Buffer CPU caching mode: cached, write-combining or uncached.
pub const OMAP_BO_CACHED: c_uint = 0x00000000;
pub const OMAP_BO_WC: c_uint = 0x00000002;
pub const OMAP_BO_UNCACHED: c_uint = 0x00000004;
pub const OMAP_BO_CACHE_MASK: c_uint = 0x00000006;
// Use TILER for the buffer. The TILER container unit can be 8, 16 or 32 bits.
pub const OMAP_BO_TILED_8: c_uint = 0x00000100;
pub const OMAP_BO_TILED_16: c_uint = 0x00000200;
pub const OMAP_BO_TILED_32: c_uint = 0x00000300;
pub const OMAP_BO_TILED_MASK: c_uint = 0x00000f00;
#[repr(C)]
#[derive(Copy, Clone)]
pub union omap_gem_size {
    pub /: *mut *mut __u32 bytes; / (for non-tiled formats),
    pub width: __u16,
    pub height: __u16,
    pub /: *mut *mut } tiled; / (for tiled formats),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_omap_gem_new {
    pub /: *mut *mut omap_gem_size size; / in,
    pub /: *mut *mut __u32 flags; / in,
    pub /: *mut *mut __u32 handle; / out,
    pub __pad: __u32,
}

// mask of operations:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_gem_op {
    OMAP_GEM_READ = 0x01,
    OMAP_GEM_WRITE = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_omap_gem_cpu_prep {
    pub /: *mut *mut __u32 handle; / buffer handle (in),
    pub /: *mut *mut __u32 op; / mask of omap_gem_op (in),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_omap_gem_cpu_fini {
    pub /: *mut *mut __u32 handle; / buffer handle (in),
    pub /: *mut *mut __u32 op; / mask of omap_gem_op (in),
// TODO maybe here we pass down info about what regions are touched
// by sw so we can be clever about cache ops?  For now a placeholder,
// set to zero and we just do full buffer flush..
//
    pub nregions: __u32,
    pub __pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_omap_gem_info {
    pub /: *mut *mut __u32 handle; / buffer handle (in),
    pub pad: __u32,
    pub /: *mut *mut __u64 offset; / mmap offset (out),
// note: in case of tiled buffers, the user virtual size can be
// different from the physical size (ie. how many pages are needed
// to back the object) which is returned in DRM_IOCTL_GEM_OPEN..
// This size here is the one that should be used if you want to
// mmap() the buffer:
//
    pub /: *mut *mut __u32 size; / virtual size for mmap'ing (out),
    pub __pad: __u32,
}

pub const DRM_OMAP_GET_PARAM: c_uint = 0x00;
pub const DRM_OMAP_SET_PARAM: c_uint = 0x01;
pub const DRM_OMAP_GEM_NEW: c_uint = 0x03;
pub const DRM_OMAP_GEM_CPU_PREP: c_uint = 0x04	/* Deprecated, to be removed */;
pub const DRM_OMAP_GEM_CPU_FINI: c_uint = 0x05	/* Deprecated, to be removed */;
pub const DRM_OMAP_GEM_INFO: c_uint = 0x06;
pub const DRM_OMAP_NUM_IOCTLS: c_uint = 0x07;

