//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/swap_ops.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_iocb {
    pub iocb: kiocb,
    pub bio: bio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_io_ctx {
    pub sio: *mut swap_iocb,
    pub sis: *mut swap_info_struct,
}

//
// SWAP_OPS_F_REQUIRE_NOFS:
// When set, all reclaim operations must operated as GFS_NOFS and not
// just GFP_NOIO, as GFP_NOIO allocations could recourse into the
// file system backing this swap file.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_ops {
    pub flags: c_uint,
    pub rw): size_t prev_folio_size, int,
    pub ctx): *mut *mut void (submit_write)(struct swap_io_ctx,
    pub ctx): *mut *mut void (submit_read)(struct swap_io_ctx,
}

extern "C" {
    pub fn swap_fs_prepare_rw(ctx: *mut swap_io_ctx, rw: c_int, iter: *mut iov_iter);
}
extern "C" {
    pub fn swap_fs_activate(sis: *mut swap_info_struct, ops: *const swap_ops) -> c_int;
}
