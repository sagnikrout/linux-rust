//! Automatically rewritten from C Header to Rust Module
//! Source: fs/fuse/args.h
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

// One input argument of a request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_in_arg {
    pub size: unsigned,
    pub value: *const c_void,
}

// One output argument of a request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_arg {
    pub size: unsigned,
    pub value: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_args {
    pub nodeid: u64,
    pub opcode: u32,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32,
    pub in_numargs: u8,
    pub out_numargs: u8,
    pub ext_idx: u8,
    pub force:1: bool,
    pub noreply:1: bool,
    pub nocreds:1: bool,
    pub in_pages:1: bool,
    pub out_pages:1: bool,
    pub user_pages:1: bool,
    pub out_argvar:1: bool,
    pub page_zeroing:1: bool,
    pub page_replace:1: bool,
    pub may_block:1: bool,
    pub is_ext:1: bool,
    pub is_pinned:1: bool,
    pub invalidate_vmap:1: bool,
    pub abort_on_kill:1: bool,
// server requested io-uring zero-copy for this op
    pub zero_copy:1: bool,
    pub in_args: [fuse_in_arg; 4],
    pub out_args: [fuse_arg; 2],
    pub error): *mut *mut *mut void (end)(struct fuse_args args, int,
// Used for kvec iter backed by vmalloc address
    pub vmap_base: *mut c_void,
}

// FUSE folio descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_folio_desc {
    pub length: c_uint,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_args_pages {
    pub args: fuse_args,
    pub folios: *mut folio,
    pub descs: *mut fuse_folio_desc,
    pub num_folios: c_uint,
}
