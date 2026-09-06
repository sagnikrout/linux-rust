//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/qxl_drm.h
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
// Copyright 2013 Red Hat
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Please note that modifications to all structs defined here are
// subject to backwards-compatibility constraints.
//
// Do not use pointers, use __u64 instead for 32 bit / 64 bit user/kernel
// compatibility Keep fields aligned to their size
//
pub const QXL_GEM_DOMAIN_CPU: c_int = 0;
pub const QXL_GEM_DOMAIN_VRAM: c_int = 1;
pub const QXL_GEM_DOMAIN_SURFACE: c_int = 2;
pub const DRM_QXL_ALLOC: c_uint = 0x00;
pub const DRM_QXL_MAP: c_uint = 0x01;
pub const DRM_QXL_EXECBUFFER: c_uint = 0x02;
pub const DRM_QXL_UPDATE_AREA: c_uint = 0x03;
pub const DRM_QXL_GETPARAM: c_uint = 0x04;
pub const DRM_QXL_CLIENTCAP: c_uint = 0x05;
pub const DRM_QXL_ALLOC_SURF: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_alloc {
    pub size: __u32,
    pub /: *mut *mut __u32 handle; / 0 is an invalid handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_map {
    pub /: *mut *mut __u64 offset; / use for mmap system call,
    pub handle: __u32,
    pub pad: __u32,
}

//
// dest is the bo we are writing the relocation into
// src is bo we are relocating.
// *(dest_handle.base_addr + dest_offset) = physical_address(src_handle.addr +
// src_offset)
//
pub const QXL_RELOC_TYPE_BO: c_int = 1;
pub const QXL_RELOC_TYPE_SURF: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_reloc {
    pub /: *mut *mut __u64 src_offset; / offset into src_handle or src buffer,
    pub /: *mut *mut __u64 dst_offset; / offset in dest handle,
    pub /: *mut *mut __u32 src_handle; / dest handle to compute address from,
    pub /: *mut *mut __u32 dst_handle; / 0 if to command buffer,
    pub reloc_type: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_command {
    pub /: *mut *mut *mut __u64 command; / void,
    pub /: *mut *mut *mut __u64 relocs; / struct drm_qxl_reloc,
    pub type: __u32,
    pub command_size: __u32,
    pub relocs_num: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_execbuffer {
    pub /: *mut *mut __u32 flags; / for future use,
    pub commands_num: __u32,
    pub /: *mut *mut *mut __u64 commands; / struct drm_qxl_command,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_update_area {
    pub handle: __u32,
    pub top: __u32,
    pub left: __u32,
    pub bottom: __u32,
    pub right: __u32,
    pub pad: __u32,
}

pub const QXL_PARAM_MAX_RELOCS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_getparam {
    pub param: __u64,
    pub value: __u64,
}

// these are one bit values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_clientcap {
    pub index: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_qxl_alloc_surf {
    pub format: __u32,
    pub width: __u32,
    pub height: __u32,
    pub stride: __s32,
    pub handle: __u32,
    pub pad: __u32,
}

