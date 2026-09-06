//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/zram/zcomp.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct deflate_params {
    pub winbits: i32,
}

//
// Immutable driver (backend) parameters. The driver may attach private
// data to it (e.g. driver representation of the dictionary, etc.).
//
// This data is kept per-comp and is shared among execution contexts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcomp_params {
    pub dict: *mut c_void,
    pub dict_sz: usize,
    pub level: i32,
    pub deflate: deflate_params,
}

//
// Run-time driver context - scratch buffers, etc. It is modified during
// request execution (compression/decompression), cannot be shared, so
// it's in per-CPU area.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcomp_ctx {
    pub context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcomp_strm {
    pub lock: mutex,
// compression buffer
    pub buffer: *mut c_void,
// local copy of handle memory
    pub local_copy: *mut c_void,
    pub ctx: zcomp_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcomp_req {
    pub src: *const c_uchar,
    pub src_len: usize,
    pub dst: *mut c_uchar,
    pub dst_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcomp_ops {
    pub req): *mut zcomp_req,
    pub req): *mut zcomp_req,
    pub ctx): *mut *mut *mut int (create_ctx)(struct zcomp_params params, struct zcomp_ctx,
    pub ctx): *mut *mut void (destroy_ctx)(struct zcomp_ctx,
    pub params): *mut *mut int (setup_params)(struct zcomp_params,
    pub params): *mut *mut void (release_params)(struct zcomp_params,
    pub name: *const c_char,
}

// dynamic per-device compression frontend
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcomp {
    pub stream: *mut zcomp_strm __percpu,
    pub ops: *const zcomp_ops,
    pub params: *mut zcomp_params,
    pub node: hlist_node,
}

extern "C" {
    pub fn zcomp_cpu_up_prepare(cpu: c_uint, node: *mut hlist_node) -> c_int;
}
extern "C" {
    pub fn zcomp_cpu_dead(cpu: c_uint, node: *mut hlist_node) -> c_int;
}
extern "C" {
    pub fn zcomp_available_show(comp: *const c_char, buf: *mut c_char, at: isize) -> isize;
}
extern "C" {
    pub fn zcomp_destroy(comp: *mut zcomp);
}
extern "C" {
    pub fn zcomp_stream_put(zstrm: *mut zcomp_strm);
}
