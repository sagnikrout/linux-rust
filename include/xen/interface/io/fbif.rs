//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/fbif.h
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


// SPDX-License-Identifier: MIT
//
// fbif.h -- Xen virtual frame buffer device
//
// Copyright (C) 2005 Anthony Liguori <aliguori@us.ibm.com>
// Copyright (C) 2006 Red Hat, Inc., Markus Armbruster <armbru@redhat.com>
//
// Out events (frontend -> backend)
//
// Out events may be sent only when requested by backend, and receipt
// of an unknown out event is an error.
//
// Event type 1 currently not used
//
// Framebuffer update notification event
// Capable frontend sets feature-update in xenstore.
// Backend requests it by setting request-update in xenstore.
//
pub const XENFB_TYPE_UPDATE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenfb_update {
    pub /: *mut *mut uint8_t type; / XENFB_TYPE_UPDATE,
    pub /: *mut *mut int32_t x; / source x,
    pub /: *mut *mut int32_t y; / source y,
    pub /: *mut *mut int32_t width; / rect width,
    pub /: *mut *mut int32_t height; / rect height,
}

//
// Framebuffer resize notification event
// Capable backend sets feature-resize in xenstore.
//
pub const XENFB_TYPE_RESIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenfb_resize {
    pub /: *mut *mut uint8_t type; / XENFB_TYPE_RESIZE,
    pub /: *mut *mut int32_t width; / width in pixels,
    pub /: *mut *mut int32_t height; / height in pixels,
    pub /: *mut *mut int32_t stride; / stride in bytes,
    pub /: *mut *mut int32_t depth; / depth in bits,
    pub /: *mut *mut int32_t offset; / start offset within framebuffer,
}

pub const XENFB_OUT_EVENT_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub union xenfb_out_event {
    pub type: u8,
    pub update: xenfb_update,
    pub resize: xenfb_resize,
    pub pad: [c_char; XENFB_OUT_EVENT_SIZE],
}

// In events (backend -> frontend)
//
// Frontends should ignore unknown in events.
// No in events currently defined.
//
pub const XENFB_IN_EVENT_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub union xenfb_in_event {
    pub type: u8,
    pub pad: [c_char; XENFB_IN_EVENT_SIZE],
}

// shared page
pub const XENFB_IN_RING_SIZE: c_int = 1024;

pub const XENFB_IN_RING_OFFS: c_int = 1024;

pub const XENFB_OUT_RING_SIZE: c_int = 2048;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenfb_page {
    pub in_prod: uint32_t in_cons,,
    pub out_prod: uint32_t out_cons,,
    pub /: *mut *mut int32_t width; / width of the framebuffer (in pixels),
    pub /: *mut *mut int32_t height; / height of the framebuffer (in pixels),
    pub /: *mut *mut uint32_t line_length; / length of a row of pixels (in bytes),
    pub /: *mut *mut uint32_t mem_length; / length of the framebuffer (in bytes),
    pub /: *mut *mut uint8_t depth; / depth of a pixel (in bits),
//
// Framebuffer page directory
//
// Each directory page holds PAGE_SIZE / sizeof(*pd)
// framebuffer pages, and can thus map up to PAGE_SIZE
// PAGE_SIZE / sizeof(*pd) bytes.  With PAGE_SIZE == 4096 and
// sizeof(unsigned long) == 4/8, that's 4 Megs 32 bit and 2
// Megs 64 bit.  256 directories give enough room for a 512
// Meg framebuffer with a max resolution of 12,800x10,240.
// Should be enough for a while with room leftover for
// expansion.
//
    pub pd: [c_ulong; 256],
}

//
// Wart: xenkbd needs to know default resolution.  Put it here until a
// better solution is found, but don't leak it to the backend.
//

pub const XENFB_WIDTH: c_int = 800;
pub const XENFB_HEIGHT: c_int = 600;
pub const XENFB_DEPTH: c_int = 32;

