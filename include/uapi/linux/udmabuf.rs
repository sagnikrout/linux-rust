//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/udmabuf.h
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

pub const UDMABUF_FLAGS_CLOEXEC: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udmabuf_create {
    pub memfd: __u32,
    pub flags: __u32,
    pub offset: __u64,
    pub size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udmabuf_create_item {
    pub memfd: __u32,
    pub __pad: __u32,
    pub offset: __u64,
    pub size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udmabuf_create_list {
    pub flags: __u32,
    pub count: __u32,
    pub list: [udmabuf_create_item; ],
}

