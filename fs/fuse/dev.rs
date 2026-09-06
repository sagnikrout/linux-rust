//! Automatically rewritten from C Header to Rust Module
//! Source: fs/fuse/dev.h
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

// Maximum number of outstanding background requests
pub const FUSE_DEFAULT_MAX_BACKGROUND: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_chan_param {
    pub minor: c_uint,
    pub max_write: c_uint,
    pub max_pages: c_uint,
    pub io_uring_enabled: bool,
}

extern "C" {
    pub fn fuse_chan_release(fch: *mut fuse_chan);
}
extern "C" {
    pub fn fuse_chan_free(fch: *mut fuse_chan);
}
extern "C" {
    pub fn fuse_chan_num_background(fch: *mut fuse_chan) -> c_uint;
}
extern "C" {
    pub fn fuse_chan_max_background(fch: *mut fuse_chan) -> c_uint;
}
extern "C" {
    pub fn fuse_chan_max_background_set(fch: *mut fuse_chan, val: c_uint);
}
extern "C" {
    pub fn fuse_chan_num_waiting(fch: *mut fuse_chan) -> c_uint;
}
extern "C" {
    pub fn fuse_chan_set_fc(fch: *mut fuse_chan, fc: *mut fuse_conn);
}
extern "C" {
    pub fn fuse_chan_set_initialized(fch: *mut fuse_chan, param: *mut fuse_chan_param);
}
extern "C" {
    pub fn fuse_chan_send(fch: *mut fuse_chan, args: *mut fuse_args) -> isize;
}
extern "C" {
    pub fn fuse_chan_send_bg(fch: *mut fuse_chan, args: *mut fuse_args, gfp_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn fuse_chan_send_notify_reply(fch: *mut fuse_chan, args: *mut fuse_args, unique: u64) -> c_int;
}
extern "C" {
    pub fn fuse_chan_resend(fch: *mut fuse_chan);
}
//
// Initialize the client device
//
extern "C" {
    pub fn fuse_dev_init() -> c_int;
}
//
// Cleanup the client device
//
extern "C" {
    pub fn fuse_dev_cleanup();
}
extern "C" {
    pub fn fuse_dev_install(fud: *mut fuse_dev, fch: *mut fuse_chan);
}
extern "C" {
    pub fn fuse_dev_verify(fud: *mut fuse_dev, fch: *mut fuse_chan) -> bool;
}
extern "C" {
    pub fn fuse_dev_put(fud: *mut fuse_dev);
}
extern "C" {
    pub fn fuse_dev_is_installed(fud: *mut fuse_dev) -> bool;
}
extern "C" {
    pub fn fuse_dev_is_sync_init(fud: *mut fuse_dev) -> bool;
}
extern "C" {
    pub fn fuse_init_server_timeout(fch: *mut fuse_chan, timeout: c_uint);
}
// Abort all requests
extern "C" {
    pub fn fuse_chan_abort(fch: *mut fuse_chan, abort_with_err: bool);
}
extern "C" {
    pub fn fuse_chan_wait_aborted(fch: *mut fuse_chan);
}
//
// Acquire reference to fuse_conn
//
// Release reference to fuse_conn
//
extern "C" {
    pub fn fuse_conn_put(fc: *mut fuse_conn);
}
extern "C" {
    pub fn fuse_conn_get_id(fc: *mut fuse_conn) -> dev_t;
}
extern "C" {
    pub fn fuse_end_polls(fc: *mut fuse_conn);
}
extern "C" {
    pub fn fuse_backing_open(fc: *mut fuse_conn, map: *mut fuse_backing_map) -> c_int;
}
extern "C" {
    pub fn fuse_backing_close(fc: *mut fuse_conn, backing_id: c_int) -> c_int;
}
extern "C" {
    pub fn fuse_copy_one(cs: *mut fuse_copy_state, val: *mut c_void, size: unsigned) -> c_int;
}
extern "C" {
    pub fn fuse_copy_finish(cs: *mut fuse_copy_state);
}

extern "C" {
    pub fn fuse_uring_enabled() -> bool;
}
extern "C" {
    pub fn fuse_uring_destruct(fch: *mut fuse_chan);
}

