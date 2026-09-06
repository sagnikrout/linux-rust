//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/visl/visl-debugfs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Debugfs tracing for bitstream buffers. This is similar to VA-API's
// LIBVA_TRACE_BUFDATA in that the raw bitstream can be dumped as a debugging
// aid.
//
// Produces one file per OUTPUT buffer. Files are automatically cleared on
// STREAMOFF unless the module parameter "keep_bitstream_buffers" is set.
//

extern "C" {
    pub fn visl_debugfs_init(dev: *mut visl_dev) -> c_int;
}
extern "C" {
    pub fn visl_debugfs_bitstream_init(dev: *mut visl_dev) -> c_int;
}
extern "C" {
    pub fn visl_trace_bitstream(ctx: *mut visl_ctx, run: *mut visl_run);
}
extern "C" {
    pub fn visl_debugfs_clear_bitstream(dev: *mut visl_dev);
}
extern "C" {
    pub fn visl_debugfs_bitstream_deinit(dev: *mut visl_dev);
}
extern "C" {
    pub fn visl_debugfs_deinit(dev: *mut visl_dev);
}

