//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vhost/vhost.h
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

extern "C" {
    pub fn void(work: *mut *mut vhost_work_fn_t)(struct vhost_work) -> typedef;
}
pub const VHOST_WORK_QUEUED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_work {
    pub node: llist_node,
    pub fn: vhost_work_fn_t,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_worker_ops {
    pub name): *const c_char,
    pub worker): *mut *mut void (stop)(struct vhost_worker,
    pub worker): *mut *mut void (wakeup)(struct vhost_worker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_worker {
    pub kthread_task: *mut task_struct,
    pub vtsk: *mut vhost_task,
    pub dev: *mut vhost_dev,
// Used to serialize device wide flushing with worker swapping.
    pub mutex: mutex,
    pub work_list: llist_head,
    pub kcov_handle: kcov_common_handle_id,
    pub id: u32,
    pub attachment_cnt: c_int,
    pub killed: bool,
    pub ops: *const vhost_worker_ops,
}

// Poll a file (eventfd or socket)
// Note: there's nothing vhost specific about this structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_poll {
    pub table: poll_table,
    pub wqh: *mut wait_queue_head_t,
    pub wait: wait_queue_entry_t,
    pub work: vhost_work,
    pub mask: __poll_t,
    pub dev: *mut vhost_dev,
    pub vq: *mut vhost_virtqueue,
}

extern "C" {
    pub fn vhost_poll_start(poll: *mut vhost_poll, file: *mut file) -> c_int;
}
extern "C" {
    pub fn vhost_poll_stop(poll: *mut vhost_poll);
}
extern "C" {
    pub fn vhost_poll_queue(poll: *mut vhost_poll);
}
extern "C" {
    pub fn vhost_work_init(work: *mut vhost_work, fn: vhost_work_fn_t);
}
extern "C" {
    pub fn vhost_dev_flush(dev: *mut vhost_dev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_log {
    pub addr: u64,
    pub len: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vhost_uaddr_type {
    VHOST_ADDR_DESC = 0,
    VHOST_ADDR_AVAIL = 1,
    VHOST_ADDR_USED = 2,
    VHOST_NUM_ADDRS = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vring_call {
    pub ctx: *mut eventfd_ctx,
    pub producer: irq_bypass_producer,
}

// The virtqueue structure describes a queue attached to a device.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_virtqueue {
    pub dev: *mut vhost_dev,
    pub worker: *mut vhost_worker __rcu,
// The actual ring of buffers.
    pub mutex: mutex,
    pub num: c_uint,
    pub desc: *mut vring_desc_t __user,
    pub avail: *mut vring_avail_t __user,
    pub used: *mut vring_used_t __user,
    pub meta_iotlb: [*const vhost_iotlb_map; VHOST_NUM_ADDRS],
    pub kick: *mut file,
    pub call_ctx: vhost_vring_call,
    pub error_ctx: *mut eventfd_ctx,
    pub log_ctx: *mut eventfd_ctx,
    pub poll: vhost_poll,
// The routine to call when the Guest pings us, or timeout.
    pub handle_kick: vhost_work_fn_t,
// Last available index we saw.
// Values are limited to 0x7fff, and the high bit is used as
// a wrap counter when using VIRTIO_F_RING_PACKED.
    pub last_avail_idx: u16,
// Next avail ring head when VIRTIO_F_IN_ORDER is negoitated
    pub next_avail_head: u16,
// Caches available index value from user.
    pub avail_idx: u16,
// Last index we used.
// Values are limited to 0x7fff, and the high bit is used as
// a wrap counter when using VIRTIO_F_RING_PACKED.
    pub last_used_idx: u16,
// Used flags
    pub used_flags: u16,
// Last used index value we have signalled on
    pub signalled_used: u16,
// Last used index value we have signalled on
    pub signalled_used_valid: bool,
// Log writes to used structure.
    pub log_used: bool,
    pub log_addr: u64,
    pub iov: [iovec; UIO_MAXIOV],
    pub iotlb_iov: [iovec; 64],
    pub indirect: *mut iovec,
    pub heads: *mut vring_used_elem,
    pub nheads: *mut u16,
// Protected by virtqueue mutex.
    pub umem: *mut vhost_iotlb,
    pub iotlb: *mut vhost_iotlb,
    pub private_data: *mut c_void,
    pub acked_backend_features: u64,
// Log write descriptors
    pub log_base: *mut void __user,
    pub log: *mut vhost_log,
    pub log_iov: [iovec; 64],
// Ring endianness. Defaults to legacy native endianness.
// Set to true when starting a modern virtio device.
    pub is_le: bool,

// Ring endianness requested by userspace for cross-endian support.
    pub user_be: bool,

    pub busyloop_timeout: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_msg_node {
    pub msg: vhost_msg,
    pub msg_v2: vhost_msg_v2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_dev {
    pub mm: *mut mm_struct,
    pub mutex: mutex,
    pub vqs: *mut vhost_virtqueue,
    pub nvqs: c_int,
    pub log_ctx: *mut eventfd_ctx,
    pub umem: *mut vhost_iotlb,
    pub iotlb: *mut vhost_iotlb,
    pub iotlb_lock: spinlock_t,
    pub read_list: list_head,
    pub pending_list: list_head,
    pub wait: wait_queue_head_t,
    pub iov_limit: c_int,
    pub weight: c_int,
    pub byte_weight: c_int,
    pub worker_xa: xarray,
    pub use_worker: bool,
//
// If fork_owner is true we use vhost_tasks to create
// the worker so all settings/limits like cgroups, NPROC,
// scheduler, etc are inherited from the owner. If false,
// we use kthreads and only attach to the same cgroups
// as the owner for compat with older kernels.
// here we use true as default value.
// The default value is set by fork_from_owner_default
//
    pub fork_owner: bool,
    pub msg): *mut vhost_iotlb_msg,
}

extern "C" {
    pub fn vhost_exceeds_weight(vq: *mut vhost_virtqueue, pkts: c_int, total_len: c_int) -> bool;
}
extern "C" {
    pub fn vhost_dev_set_owner(dev: *mut vhost_dev) -> c_long;
}
extern "C" {
    pub fn vhost_dev_has_owner(dev: *mut vhost_dev) -> bool;
}
extern "C" {
    pub fn vhost_dev_check_owner(: *mut vhost_dev) -> c_long;
}
extern "C" {
    pub fn vhost_dev_reset_owner(dev: *mut vhost_dev, iotlb: *mut vhost_iotlb);
}
extern "C" {
    pub fn vhost_dev_cleanup(: *mut vhost_dev);
}
extern "C" {
    pub fn vhost_dev_stop(: *mut vhost_dev);
}
extern "C" {
    pub fn vhost_dev_ioctl(: *mut vhost_dev, ioctl: c_uint, argp: *mut void __user) -> c_long;
}
extern "C" {
    pub fn vhost_vring_ioctl(d: *mut vhost_dev, ioctl: c_uint, argp: *mut void __user) -> c_long;
}
extern "C" {
    pub fn vhost_vq_access_ok(vq: *mut vhost_virtqueue) -> bool;
}
extern "C" {
    pub fn vhost_log_access_ok(: *mut vhost_dev) -> bool;
}
extern "C" {
    pub fn vhost_clear_msg(dev: *mut vhost_dev);
}
extern "C" {
    pub fn vhost_vq_work_queue(vq: *mut vhost_virtqueue, work: *mut vhost_work) -> bool;
}
extern "C" {
    pub fn vhost_vq_has_work(vq: *mut vhost_virtqueue) -> bool;
}
extern "C" {
    pub fn vhost_vq_is_setup(vq: *mut vhost_virtqueue) -> bool;
}
extern "C" {
    pub fn vhost_vq_init_access(: *mut vhost_virtqueue) -> c_int;
}
extern "C" {
    pub fn vhost_add_used(: *mut vhost_virtqueue, head: c_uint, len: c_int) -> c_int;
}
extern "C" {
    pub fn vhost_signal(: *mut vhost_dev, : *mut vhost_virtqueue);
}
extern "C" {
    pub fn vhost_disable_notify(: *mut vhost_dev, : *mut vhost_virtqueue);
}
extern "C" {
    pub fn vhost_vq_avail_empty(: *mut vhost_dev, : *mut vhost_virtqueue) -> bool;
}
extern "C" {
    pub fn vhost_enable_notify(: *mut vhost_dev, : *mut vhost_virtqueue) -> bool;
}
extern "C" {
    pub fn vq_meta_prefetch(vq: *mut vhost_virtqueue) -> c_int;
}
extern "C" {
    pub fn vhost_set_backend_features(dev: *mut vhost_dev, features: u64);
}
extern "C" {
    pub fn vhost_init_device_iotlb(d: *mut vhost_dev) -> c_int;
}

//
// vhost_vq_set_backend - Set backend.
//
// @vq            Virtqueue.
// @private_data  The private data.
//
// Context: Need to call with vq->mutex acquired.
//
// vhost_vq_get_backend - Get backend.
//
// @vq            Virtqueue.
//
// Context: Need to call with vq->mutex acquired.
// Return: Private data previously set with vhost_vq_set_backend.
//
extern "C" {
    pub fn virtio_features_test_bit(_arg: vq->acked_features_array, _arg: bit) -> return;
}

// Memory accessors
extern "C" {
    pub fn __virtio16_to_cpu(_arg: vhost_is_little_endian(vq), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio16(_arg: vhost_is_little_endian(vq), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio32_to_cpu(_arg: vhost_is_little_endian(vq), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio32(_arg: vhost_is_little_endian(vq), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio64_to_cpu(_arg: vhost_is_little_endian(vq), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio64(_arg: vhost_is_little_endian(vq), _arg: val) -> return;
}
