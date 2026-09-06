//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvdimm/virtio_pmem.h
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
// virtio_pmem.h: virtio pmem Driver
//
// Discovers persistent memory range information
// from host and provides a virtio based flushing
// interface.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pmem_request {
    pub kref: kref,
// Wait queue to process deferred work after ack from host
    pub host_acked: wait_queue_head_t,
    pub done: bool,
// Wait queue to process deferred work after virt queue buffer avail
    pub wq_buf: wait_queue_head_t,
    pub wq_buf_avail: bool,
    pub list: list_head,
    pub req: virtio_pmem_req,
    pub resp: virtio_pmem_resp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pmem {
    pub vdev: *mut virtio_device,
// Virtio pmem request queue
    pub req_vq: *mut virtqueue,
// Serialize flush requests to the device.
    pub flush_lock: mutex,
// Complete asynchronous FUA flushes outside the submit path.
    pub flush_wq: *mut workqueue_struct,
// nvdimm bus registers virtio pmem device
    pub nvdimm_bus: *mut nvdimm_bus,
    pub nd_desc: nvdimm_bus_descriptor,
// List to store deferred work if virtqueue is full
    pub req_list: list_head,
// Request currently owned by the virtqueue.
    pub req_inflight: *mut virtio_pmem_request,
// Fail fast and wake waiters if the request virtqueue is broken.
    pub broken: bool,
// Synchronize virtqueue data
    pub pmem_lock: spinlock_t,
// Memory region information
    pub start: __u64,
    pub size: __u64,
}

extern "C" {
    pub fn virtio_pmem_host_ack(vq: *mut virtqueue);
}
extern "C" {
    pub fn virtio_pmem_mark_broken(vpmem: *mut virtio_pmem);
}
extern "C" {
    pub fn virtio_pmem_drain(vpmem: *mut virtio_pmem);
}
extern "C" {
    pub fn async_pmem_flush(nd_region: *mut nd_region, bio: *mut bio) -> c_int;
}
