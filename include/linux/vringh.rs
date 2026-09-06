//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vringh.h
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
//
// Linux host-side vring helpers; for when the kernel needs to access
// someone else's vring.
//
// Copyright IBM Corporation, 2013.
// Parts taken from drivers/vhost/vhost.c Copyright 2009 Red Hat, Inc.
//
// Written by: Rusty Russell <rusty@rustcorp.com.au>
//

// virtio_ring with information needed for host access.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vringh {
// Everything is little endian
    pub little_endian: bool,
// Guest publishes used event idx (note: we always do).
    pub event_indices: bool,
// Can we get away with weak barriers?
    pub weak_barriers: bool,
// Use user's VA
    pub use_va: bool,
// Last available index we saw (ie. where we're up to).
    pub last_avail_idx: u16,
// Last index we used.
    pub last_used_idx: u16,
// How many descriptors we've completed since last need_notify().
    pub completed: u32,
// The vring (note: it may contain user pointers!)
    pub vring: vring,
// IOTLB for this vring
    pub iotlb: *mut vhost_iotlb,
// spinlock to synchronize IOTLB accesses
    pub iotlb_lock: *mut spinlock_t,
// The function to call to notify the guest about added buffers
    pub ): *mut *mut void (notify)(struct vringh,
}

extern "C" {
    pub fn vrh_callback_t(: *mut virtio_device, : *mut vringh) -> typedef void;
}
//
// struct vringh_config_ops - ops for creating a host vring from a virtio driver
// @find_vrhs: find the host vrings and instantiate them
// vdev: the virtio_device
// nhvrs: the number of host vrings to find
// hvrs: on success, includes new host vrings
// callbacks: array of driver callbacks, for each host vring
// include a NULL entry for vqs that do not need a callback
// Returns 0 on success or error status
// @del_vrhs: free the host vrings found by find_vrhs().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vringh_config_ops {
    pub callbacks[]): *mut *mut vringh vrhs[], vrh_callback_t,
    pub vdev): *mut *mut void (del_vrhs)(struct virtio_device,
}

// The memory the vring can access, and what offset to apply.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vringh_range {
    pub end_incl: u64 start,,
    pub offset: u64,
}

//
// struct vringh_iov - iovec mangler.
// @iov: array of iovecs to operate on
// @consumed: number of bytes consumed within iov[i]
// @i: index of current iovec
// @used: number of iovecs present in @iov
// @max_num: maximum number of iovecs.
// corresponds to allocated memory of @iov
//
// Mangles iovec in place, and restores it.
// Remaining data is iov + i, of used - i elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vringh_iov {
    pub iov: *mut iovec,
    pub /: *mut *mut size_t consumed; / Within iov[i],
    pub max_num: unsigned i, used,,
}

//
// struct vringh_kiov - kvec mangler.
// @iov: array of iovecs to operate on
// @consumed: number of bytes consumed within iov[i]
// @i: index of current iovec
// @used: number of iovecs present in @iov
// @max_num: maximum number of iovecs.
// corresponds to allocated memory of @iov
//
// Mangles kvec in place, and restores it.
// Remaining data is iov + i, of used - i elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vringh_kiov {
    pub iov: *mut kvec,
    pub /: *mut *mut size_t consumed; / Within iov[i],
    pub max_num: unsigned i, used,,
}

// Flag on max_num to indicate we're kmalloced.
pub const VRINGH_IOV_ALLOCATED: c_uint = 0x8000000;
// Helpers for userspace vrings.
// Convert a descriptor into iovecs.
// Copy bytes from readable vsg, consuming it (and incrementing wiov->i).
extern "C" {
    pub fn vringh_iov_pull_user(riov: *mut vringh_iov, dst: *mut c_void, len: usize) -> isize;
}
// Copy bytes into writable vsg, consuming it (and incrementing wiov->i).
// Mark a descriptor as used.
extern "C" {
    pub fn vringh_complete_user(vrh: *mut vringh, head: u16, len: u32) -> c_int;
}
// Do we need to fire the eventfd to notify the other side?
extern "C" {
    pub fn vringh_need_notify_user(vrh: *mut vringh) -> c_int;
}
extern "C" {
    pub fn vringh_notify_enable_user(vrh: *mut vringh) -> bool;
}
extern "C" {
    pub fn vringh_notify_disable_user(vrh: *mut vringh);
}
// Helpers for kernelspace vrings.
extern "C" {
    pub fn vringh_kiov_advance(kiov: *mut vringh_kiov, len: usize);
}
extern "C" {
    pub fn vringh_complete_kern(vrh: *mut vringh, head: u16, len: u32) -> c_int;
}
extern "C" {
    pub fn vringh_notify_enable_kern(vrh: *mut vringh) -> bool;
}
extern "C" {
    pub fn vringh_notify_disable_kern(vrh: *mut vringh);
}
extern "C" {
    pub fn vringh_need_notify_kern(vrh: *mut vringh) -> c_int;
}
// Notify the guest about buffers added to the used ring
extern "C" {
    pub fn __virtio16_to_cpu(_arg: vringh_is_little_endian(vrh), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio16(_arg: vringh_is_little_endian(vrh), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio32_to_cpu(_arg: vringh_is_little_endian(vrh), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio32(_arg: vringh_is_little_endian(vrh), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio64_to_cpu(_arg: vringh_is_little_endian(vrh), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio64(_arg: vringh_is_little_endian(vrh), _arg: val) -> return;
}

extern "C" {
    pub fn vringh_complete_iotlb(vrh: *mut vringh, head: u16, len: u32) -> c_int;
}
extern "C" {
    pub fn vringh_need_notify_iotlb(vrh: *mut vringh) -> c_int;
}

