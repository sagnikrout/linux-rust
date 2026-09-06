//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/host1x.h
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
// Copyright (c) 2009-2013, NVIDIA Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host1x_class {
    HOST1X_CLASS_HOST1X = 0x1,
    HOST1X_CLASS_NVJPG1 = 0x7,
    HOST1X_CLASS_NVENC = 0x21,
    HOST1X_CLASS_NVENC1 = 0x22,
    HOST1X_CLASS_GR2D = 0x51,
    HOST1X_CLASS_GR2D_SB = 0x52,
    HOST1X_CLASS_VIC = 0x5D,
    HOST1X_CLASS_GR3D = 0x60,
    HOST1X_CLASS_NVJPG = 0xC0,
    HOST1X_CLASS_NVDEC = 0xF0,
    HOST1X_CLASS_NVDEC1 = 0xF5,
    HOST1X_CLASS_OFA = 0xF8,
}

extern "C" {
    pub fn host1x_get_dma_mask(host1x: *mut host1x) -> u64;
}
//
// struct host1x_bo_cache - host1x buffer object cache
// @mappings: list of mappings
// @lock: synchronizes accesses to the list of mappings
//
// Note that entries are not periodically evicted from this cache and instead need to be
// explicitly released. This is used primarily for DRM/KMS where the cache's reference is
// released when the last reference to a buffer object represented by a mapping in this
// cache is dropped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_bo_cache {
    pub mappings: list_head,
    pub lock: mutex,
}

// XXX warn if not empty?
//
// struct host1x_client_ops - host1x client operations
// @early_init: host1x client early initialization code
// @init: host1x client initialization code
// @exit: host1x client tear down code
// @late_exit: host1x client late tear down code
// @suspend: host1x client suspend code
// @resume: host1x client resume code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_client_ops {
    pub client): *mut *mut int (early_init)(struct host1x_client,
    pub client): *mut *mut int (init)(struct host1x_client,
    pub client): *mut *mut int (exit)(struct host1x_client,
    pub client): *mut *mut int (late_exit)(struct host1x_client,
    pub client): *mut *mut int (suspend)(struct host1x_client,
    pub client): *mut *mut int (resume)(struct host1x_client,
}

//
// struct host1x_client - host1x client structure
// @list: list node for the host1x client
// @host: pointer to struct device representing the host1x controller
// @dev: pointer to struct device backing this host1x client
// @group: IOMMU group that this client is a member of
// @ops: host1x client operations
// @class: host1x class represented by this client
// @channel: host1x channel associated with this client
// @syncpts: array of syncpoints requested for this client
// @num_syncpts: number of syncpoints requested for this client
// @parent: pointer to parent structure
// @usecount: reference count for this structure
// @lock: mutex for mutually exclusive concurrency
// @cache: host1x buffer object cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_client {
    pub list: list_head,
    pub host: *mut device,
    pub dev: *mut device,
    pub group: *mut iommu_group,
    pub ops: *const host1x_client_ops,
    pub class: host1x_class,
    pub channel: *mut host1x_channel,
    pub syncpts: *mut host1x_syncpt,
    pub num_syncpts: c_uint,
    pub parent: *mut host1x_client,
    pub usecount: c_uint,
    pub lock: mutex,
    pub cache: host1x_bo_cache,
}

//
// host1x buffer objects
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_bo_mapping {
    pub ref: kref,
    pub attach: *mut dma_buf_attachment,
    pub direction: dma_data_direction,
    pub list: list_head,
    pub bo: *mut host1x_bo,
    pub sgt: *mut sg_table,
    pub chunks: c_uint,
    pub dev: *mut device,
    pub phys: dma_addr_t,
    pub size: usize,
    pub cache: *mut host1x_bo_cache,
    pub entry: list_head,
}

extern "C" {
    pub fn container_of(_arg: ref, host1x_bo_mapping: struct, _arg: ref) -> return;
}
//
// struct host1x_bo_ops - operations implemented by a host1x_bo provider
//
// @pin: create a DMA mapping. Implementation must not touch the bo's refcount.
// @unpin: destroy a DMA mapping. Implementation must not touch the bo's refcount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_bo_ops {
    pub bo): *mut *mut *mut host1x_bo (get)(host1x_bo,
    pub bo): *mut *mut void (put)(struct host1x_bo,
    pub dir): dma_data_direction,
    pub map): *mut *mut void (unpin)(struct host1x_bo_mapping,
    pub bo): *mut *mut *mut void (mmap)(struct host1x_bo,
    pub addr): *mut *mut *mut void (munmap)(struct host1x_bo bo, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_bo {
    pub ops: *const host1x_bo_ops,
    pub mappings: list_head,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn host1x_bo_unpin(map: *mut host1x_bo_mapping);
}
extern "C" {
    pub fn host1x_bo_clear_cached_mappings(bo: *mut host1x_bo);
}
//
// host1x syncpoints
//

extern "C" {
    pub fn host1x_syncpt_id(sp: *mut host1x_syncpt) -> u32;
}
extern "C" {
    pub fn host1x_syncpt_read_min(sp: *mut host1x_syncpt) -> u32;
}
extern "C" {
    pub fn host1x_syncpt_read_max(sp: *mut host1x_syncpt) -> u32;
}
extern "C" {
    pub fn host1x_syncpt_read(sp: *mut host1x_syncpt) -> u32;
}
extern "C" {
    pub fn host1x_syncpt_incr(sp: *mut host1x_syncpt) -> c_int;
}
extern "C" {
    pub fn host1x_syncpt_incr_max(sp: *mut host1x_syncpt, incrs: u32) -> u32;
}
extern "C" {
    pub fn host1x_syncpt_put(sp: *mut host1x_syncpt);
}
extern "C" {
    pub fn host1x_syncpt_base_id(base: *mut host1x_syncpt_base) -> u32;
}
extern "C" {
    pub fn host1x_fence_cancel(fence: *mut dma_fence);
}
//
// host1x channel
//
extern "C" {
    pub fn host1x_channel_stop(channel: *mut host1x_channel);
}
extern "C" {
    pub fn host1x_channel_put(channel: *mut host1x_channel);
}
extern "C" {
    pub fn host1x_job_submit(job: *mut host1x_job) -> c_int;
}
//
// host1x job
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_reloc {
    pub bo: *mut host1x_bo,
    pub offset: c_ulong,
    pub cmdbuf: },
    pub bo: *mut host1x_bo,
    pub offset: c_ulong,
    pub target: },
    pub shift: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_job {
// When refcount goes to zero, job can be freed
    pub ref: kref,
// List entry
    pub list: list_head,
// Channel where job is submitted to
    pub channel: *mut host1x_channel,
// client where the job originated
    pub client: *mut host1x_client,
// Gathers and their memory
    pub cmds: *mut host1x_job_cmd,
    pub num_cmds: c_uint,
// Array of handles to be pinned & unpinned
    pub relocs: *mut host1x_reloc,
    pub num_relocs: c_uint,
    pub unpins: *mut host1x_job_unpin_data,
    pub num_unpins: c_uint,
    pub addr_phys: *mut dma_addr_t,
    pub gather_addr_phys: *mut dma_addr_t,
    pub reloc_addr_phys: *mut dma_addr_t,
// Sync point id, number of increments and end related to the submit
    pub syncpt: *mut host1x_syncpt,
    pub syncpt_incrs: u32,
    pub syncpt_end: u32,
// Completion fence for job tracking
    pub fence: *mut dma_fence,
    pub fence_cb: dma_fence_cb,
// Maximum time to wait for this job
    pub timeout: c_uint,
// Job has timed out and should be released
    pub cancelled: bool,
// Index and number of slots used in the push buffer
    pub first_get: c_uint,
    pub num_slots: c_uint,
// Copy of gathers
    pub gather_copy_size: usize,
    pub gather_copy: dma_addr_t,
    pub gather_copy_mapped: *mut u8,
// Check if register is marked as an address reg
    pub reg): *mut *mut *mut int (is_addr_reg)(struct device dev, u32 class, u32,
// Check if class belongs to the unit
    pub class): *mut *mut int (is_valid_class)(u32,
// Request a SETCLASS to this class
    pub class: u32,
// Add a channel wait for previous ops to complete
    pub serialize: bool,
// Fast-forward syncpoint increments on job timeout
    pub syncpt_recovery: bool,
// Callback called when job is freed
    pub job): *mut *mut void (release)(struct host1x_job,
    pub user_data: *mut c_void,
// Whether host1x-side firewall should be ran for this job or not
    pub enable_firewall: bool,
// Options for configuring engine data stream ID
// Context device to use for job
    pub memory_context: *mut host1x_memory_context,
// Stream ID to use if context isolation is disabled (!memory_context)
    pub engine_fallback_streamid: u32,
// Engine offset to program stream ID to
    pub engine_streamid_offset: u32,
}

extern "C" {
    pub fn host1x_job_put(job: *mut host1x_job);
}
extern "C" {
    pub fn host1x_job_pin(job: *mut host1x_job, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn host1x_job_unpin(job: *mut host1x_job);
}
//
// subdevice probe infrastructure
//
// struct host1x_driver - host1x logical device driver
// @driver: core driver
// @subdevs: table of OF device IDs matching subdevices for this driver
// @list: list node for the driver
// @probe: called when the host1x logical device is probed
// @remove: called when the host1x logical device is removed
// @shutdown: called when the host1x logical device is shut down
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_driver {
    pub driver: device_driver,
    pub subdevs: *const of_device_id,
    pub list: list_head,
    pub device): *mut *mut int (probe)(struct host1x_device,
    pub device): *mut *mut void (remove)(struct host1x_device,
    pub device): *mut *mut void (shutdown)(struct host1x_device,
}

extern "C" {
    pub fn container_of(_arg: driver, host1x_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn host1x_driver_unregister(driver: *mut host1x_driver);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_device {
    pub driver: *mut host1x_driver,
    pub list: list_head,
    pub dev: device,
    pub subdevs_lock: mutex,
    pub subdevs: list_head,
    pub active: list_head,
    pub clients_lock: mutex,
    pub clients: list_head,
    pub registered: bool,
    pub dma_parms: device_dma_parameters,
}

extern "C" {
    pub fn container_of(_arg: dev, host1x_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn host1x_device_init(device: *mut host1x_device) -> c_int;
}
extern "C" {
    pub fn host1x_device_exit(device: *mut host1x_device) -> c_int;
}
extern "C" {
    pub fn __host1x_client_init(client: *mut host1x_client, key: *mut lock_class_key);
}
extern "C" {
    pub fn host1x_client_exit(client: *mut host1x_client);
}

extern "C" {
    pub fn __host1x_client_register(client: *mut host1x_client) -> c_int;
}
//
// Note that this wrapper calls __host1x_client_init() for compatibility
// with existing callers. Callers that want to separately initialize and
// register a host1x client must first initialize using either of the
// __host1x_client_init() or host1x_client_init() functions and then use
// the low-level __host1x_client_register() function to avoid the client
// getting reinitialized.
//

extern "C" {
    pub fn host1x_client_unregister(client: *mut host1x_client);
}
extern "C" {
    pub fn host1x_client_suspend(client: *mut host1x_client) -> c_int;
}
extern "C" {
    pub fn host1x_client_resume(client: *mut host1x_client) -> c_int;
}
// host1x memory contexts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_memory_context {
    pub host: *mut host1x,
    pub ref: refcount_t,
    pub owner: *mut pid,
    pub dma_parms: device_dma_parameters,
    pub dev: device,
    pub dma_mask: u64,
    pub stream_id: u32,
}

extern "C" {
    pub fn host1x_memory_context_get(cd: *mut host1x_memory_context);
}
extern "C" {
    pub fn host1x_memory_context_put(cd: *mut host1x_memory_context);
}

