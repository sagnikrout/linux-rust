//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio_ring.h
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

//
// Barriers in virtio are tricky.  Non-SMP virtio guests can't assume
// they're not on an SMP host system, so they need to assume real
// barriers.  Non-SMP virtio hosts could skip the barriers, but does
// anyone care?
//
// For virtio_pci on SMP, we don't need to order with respect to MMIO
// accesses through relaxed memory I/O windows, so virt_mb() et al are
// sufficient.
//
// For using virtio to talk to real devices (eg. other heterogeneous
// CPUs) we do need real barriers.  In theory, we could be using both
// kinds of virtio, so it's a runtime decision, and the branch is
// actually quite cheap.
//

//
// Creates a virtqueue and allocates the descriptor ring.  If
// may_reduce_num is set, then this may allocate a smaller ring than
// expected.  The caller should query virtqueue_get_vring_size to learn
// the actual size of the ring.
//
// Creates a virtqueue and allocates the descriptor ring with per
// virtqueue mapping operations.
//
// Creates a virtqueue with a standard layout but a caller-allocated
// ring.
//
// Destroys a virtqueue.  If created with vring_create_virtqueue, this
// also frees the ring.
//
extern "C" {
    pub fn vring_del_virtqueue(vq: *mut virtqueue);
}
// Filter out transport-specific feature bits.
extern "C" {
    pub fn vring_transport_features(vdev: *mut virtio_device);
}
extern "C" {
    pub fn vring_interrupt(irq: c_int, _vq: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn vring_notification_data(_vq: *mut virtqueue) -> u32;
}
