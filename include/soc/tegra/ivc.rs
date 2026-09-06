//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/ivc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2016, NVIDIA CORPORATION.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ivc {
    pub peer: *mut device,
    pub map: iosys_map,
    pub position: c_uint,
    pub phys: dma_addr_t,
    pub tx: } rx,,
    pub data): *mut *mut *mut void (notify)(struct tegra_ivc ivc, void,
    pub notify_data: *mut c_void,
    pub num_frames: c_uint,
    pub frame_size: usize,
}

//
// tegra_ivc_read_get_next_frame - Peek at the next frame to receive
// @ivc		pointer of the IVC channel
//
// Peek at the next frame to be received, without removing it from
// the queue.
//
// Returns a pointer to the frame, or an error encoded pointer.
//
extern "C" {
    pub fn tegra_ivc_read_get_next_frame(ivc: *mut tegra_ivc, map: *mut iosys_map) -> c_int;
}
//
// tegra_ivc_read_advance - Advance the read queue
// @ivc		pointer of the IVC channel
//
// Advance the read queue
//
// Returns 0, or a negative error value if failed.
//
extern "C" {
    pub fn tegra_ivc_read_advance(ivc: *mut tegra_ivc) -> c_int;
}
//
// tegra_ivc_write_get_next_frame - Poke at the next frame to transmit
// @ivc		pointer of the IVC channel
//
// Get access to the next frame.
//
// Returns a pointer to the frame, or an error encoded pointer.
//
extern "C" {
    pub fn tegra_ivc_write_get_next_frame(ivc: *mut tegra_ivc, map: *mut iosys_map) -> c_int;
}
//
// tegra_ivc_write_advance - Advance the write queue
// @ivc		pointer of the IVC channel
//
// Advance the write queue
//
// Returns 0, or a negative error value if failed.
//
extern "C" {
    pub fn tegra_ivc_write_advance(ivc: *mut tegra_ivc) -> c_int;
}
//
// tegra_ivc_notified - handle internal messages
// @ivc		pointer of the IVC channel
//
// This function must be called following every notification.
//
// Returns 0 if the channel is ready for communication, or -EAGAIN if a channel
// reset is in progress.
//
extern "C" {
    pub fn tegra_ivc_notified(ivc: *mut tegra_ivc) -> c_int;
}
//
// tegra_ivc_reset - initiates a reset of the shared memory state
// @ivc		pointer of the IVC channel
//
// This function must be called after a channel is reserved before it is used
// for communication. The channel will be ready for use when a subsequent call
// to notify the remote of the channel reset.
//
extern "C" {
    pub fn tegra_ivc_reset(ivc: *mut tegra_ivc);
}
extern "C" {
    pub fn tegra_ivc_align(size: usize) -> usize;
}
extern "C" {
    pub fn tegra_ivc_total_queue_size(queue_size: unsigned) -> unsigned;
}
extern "C" {
    pub fn tegra_ivc_cleanup(ivc: *mut tegra_ivc);
}
