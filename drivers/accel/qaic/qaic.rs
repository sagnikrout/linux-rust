//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/qaic/qaic.h
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
// Copyright (c) 2019-2021, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic_families {
    FAMILY_AIC100,
    FAMILY_AIC200,
    FAMILY_MAX,
}

// Device is offline or will be very soon
// Device is booting, not clear if it's in a usable state
// Device is fully operational
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbc_states {
// DBC is free and can be activated
    DBC_STATE_IDLE,
// DBC is activated and a workload is running on device
    DBC_STATE_ASSIGNED,
// Sub-system associated with this workload has crashed and it will shutdown soon
    DBC_STATE_BEFORE_SHUTDOWN,
// Sub-system associated with this workload has crashed and it has shutdown
    DBC_STATE_AFTER_SHUTDOWN,
// Sub-system associated with this workload is shutdown and it will be powered up soon
    DBC_STATE_BEFORE_POWER_UP,
// Sub-system associated with this workload is now powered up
    DBC_STATE_AFTER_POWER_UP,
    DBC_STATE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_user {
// Uniquely identifies this user for the device
    pub handle: c_int,
    pub ref_count: kref,
// Char device opened by this user
    pub qddev: *mut qaic_drm_device,
// Node in list of users that opened this drm device
    pub node: list_head,
// SRCU used to synchronize this user during cleanup
    pub qddev_lock: srcu_struct,
    pub chunk_id: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_bridge_chan {
// Pointer to device strcut maintained by driver
    pub qdev: *mut qaic_device,
// ID of this DMA bridge channel(DBC)
    pub id: c_uint,
// Synchronizes access to xfer_list
    pub xfer_lock: spinlock_t,
// Base address of request queue
    pub req_q_base: *mut c_void,
// Base address of response queue
    pub rsp_q_base: *mut c_void,
//
// Base bus address of request queue. Response queue bus address can be
// calculated by adding request queue size to this variable
//
    pub dma_addr: dma_addr_t,
// Total size of request and response queue in byte
    pub total_size: u32,
// Capacity of request/response queue
    pub nelem: u32,
// The user that opened this DBC
    pub usr: *mut qaic_user,
//
// Request ID of next memory handle that goes in request queue. One
// memory handle can enqueue more than one request elements, all
// this requests that belong to same memory handle have same request ID
//
    pub next_req_id: u16,
// true: DBC is in use; false: DBC not in use
    pub in_use: bool,
//
// Base address of device registers. Used to read/write request and
// response queue's head and tail pointer of this DBC.
//
    pub dbc_base: *mut void __iomem,
// Synchronizes access to Request queue's head and tail pointer
    pub req_lock: mutex,
// Head of list where each node is a memory handle queued in request queue
    pub xfer_list: list_head,
// Synchronizes DBC readers during cleanup
    pub ch_lock: srcu_struct,
//
// When this DBC is released, any thread waiting on this wait queue is
// woken up
//
    pub dbc_release: wait_queue_head_t,
// Head of list where each node is a bo associated with this DBC
    pub bo_lists: list_head,
// The irq line for this DBC. Used for polling
    pub irq: c_uint,
// Polling work item to simulate interrupts
    pub poll_work: work_struct,
// Represents various states of this DBC from enum dbc_states
    pub state: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_device {
// Pointer to base PCI device struct of our physical device
    pub pdev: *mut pci_dev,
// Req. ID of request that will be queued next in MHI control device
    pub next_seq_num: u32,
// Base address of the MHI bar
    pub bar_mhi: *mut void __iomem,
// Base address of the DBCs bar
    pub bar_dbc: *mut void __iomem,
// Controller structure for MHI devices
    pub mhi_cntrl: *mut mhi_controller,
// MHI control channel device
    pub cntl_ch: *mut mhi_device,
// List of requests queued in MHI control device
    pub cntl_xfer_list: list_head,
// Synchronizes MHI control device transactions and its xfer list
    pub cntl_mutex: mutex,
// Work queue for tasks related to MHI control device
    pub cntl_wq: *mut workqueue_struct,
// Synchronizes all the users of device during cleanup
    pub dev_lock: srcu_struct,
// Track the state of the device during resets
    pub dev_state: dev_states,
// true: single MSI is used to operate device
    pub single_msi: bool,
//
// true: A tx MHI transaction has failed and a rx buffer is still queued
// in control device. Such a buffer is considered lost rx buffer
// false: No rx buffer is lost in control device
//
    pub cntl_lost_buf: bool,
// Maximum number of DBC supported by this device
    pub num_dbc: u32,
// Reference to the drm_device for this device when it is created
    pub qddev: *mut qaic_drm_device,
// Generate the CRC of a control message
    pub msg): *mut *mut u32 (gen_crc)(void,
// Validate the CRC of a control message
    pub msg): *mut *mut bool (valid_crc)(void,
// MHI "QAIC_TIMESYNC" channel device
    pub qts_ch: *mut mhi_device,
// Work queue for tasks related to MHI "QAIC_TIMESYNC" channel
    pub qts_wq: *mut workqueue_struct,
// MHI "QAIC_TIMESYNC_PERIODIC" channel device
    pub mqts_ch: *mut mhi_device,
// Head of list of page allocated by MHI bootlog device
    pub bootlog: list_head,
// MHI bootlog channel device
    pub bootlog_ch: *mut mhi_device,
// Work queue for tasks related to MHI bootlog device
    pub bootlog_wq: *mut workqueue_struct,
// Synchronizes access of pages in MHI bootlog device
    pub bootlog_mutex: mutex,
// MHI RAS channel device
    pub ras_ch: *mut mhi_device,
// Correctable error count
    pub ce_count: c_uint,
// Un-correctable error count
    pub ue_count: c_uint,
// Un-correctable non-fatal error count
    pub ue_nf_count: c_uint,
// MHI SSR channel device
    pub ssr_ch: *mut mhi_device,
// Work queue for tasks related to MHI SSR device
    pub ssr_wq: *mut workqueue_struct,
// Buffer to collect SSR crashdump via SSR MHI channel
    pub ssr_mhi_buf: *mut c_void,
// DBC which is under SSR. Sentinel U32_MAX would mean that no SSR in progress
    pub ssr_dbc: u32,
// Array of DBC struct of this device
    pub __counted_by(num_dbc): dma_bridge_chan dbc[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_drm_device {
// The drm device struct of this drm device
    pub drm: drm_device,
// Pointer to the root device struct driven by this driver
    pub qdev: *mut qaic_device,
//
// The physical device can be partition in number of logical devices.
// And each logical device is given a partition id. This member stores
// that id. QAIC_NO_PARTITION is a sentinel used to mark that this drm
// device is the actual physical device
//
    pub partition_id: i32,
// Head in list of users who have opened this drm device
    pub users: list_head,
// Synchronizes access to users list
    pub users_mutex: mutex,
// Pointer to array of DBC sysfs attributes
    pub sysfs_attrs: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_bo {
    pub base: drm_gem_object,
// Scatter/gather table for allocate/imported BO
    pub sgt: *mut sg_table,
// Head in list of slices of this BO
    pub slices: list_head,
// Total nents, for all slices of this BO
    pub total_slice_nents: c_int,
//
// Direction of transfer. It can assume only two value DMA_TO_DEVICE and
// DMA_FROM_DEVICE.
//
    pub dir: c_int,
// The pointer of the DBC which operates on this BO
    pub dbc: *mut dma_bridge_chan,
// Number of slice that belongs to this buffer
    pub nr_slice: u32,
// Number of slice that have been transferred by DMA engine
    pub nr_slice_xfer_done: u32,
//
// If true then user has attached slicing information to this BO by
// calling DRM_IOCTL_QAIC_ATTACH_SLICE_BO ioctl.
//
    pub sliced: bool,
// Request ID of this BO if it is queued for execution
    pub req_id: u16,
// Wait on this for completion of DMA transfer of this BO
    pub xfer_done: completion,
//
// Node in linked list where head is dbc->xfer_list.
// This link list contain BO's that are queued for DMA transfer.
//
    pub xfer_list: list_head,
//
// Node in linked list where head is dbc->bo_lists.
// This link list contain BO's that are associated with the DBC it is
// linked to.
//
    pub bo_list: list_head,
//
// Latest timestamp(ns) at which kernel received a request to
// execute this BO
//
    pub req_received_ts: u64,
//
// Latest timestamp(ns) at which kernel enqueued requests of
// this BO for execution in DMA queue
//
    pub req_submit_ts: u64,
//
// Latest timestamp(ns) at which kernel received a completion
// interrupt for requests of this BO
//
    pub req_processed_ts: u64,
//
// Number of elements already enqueued in DMA queue before
// enqueuing requests of this BO
//
    pub queue_level_before: u32,
    pub perf_stats: },
// Synchronizes BO operations
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bo_slice {
// Mapped pages
    pub sgt: *mut sg_table,
// Number of requests required to queue in DMA queue
    pub nents: c_int,
// See enum dma_data_direction
    pub dir: c_int,
// Actual requests that will be copied in DMA queue
    pub reqs: *mut dbc_req,
    pub ref_count: kref,
// true: No DMA transfer required
    pub no_xfer: bool,
// Pointer to the parent BO handle
    pub bo: *mut qaic_bo,
// Node in list of slices maintained by parent BO
    pub slice: list_head,
// Size of this slice in bytes
    pub size: u64,
// Offset of this slice in buffer
    pub offset: u64,
}

extern "C" {
    pub fn get_dbc_req_elem_size() -> c_int;
}
extern "C" {
    pub fn get_dbc_rsp_elem_size() -> c_int;
}
extern "C" {
    pub fn get_cntl_version(qdev: *mut qaic_device, usr: *mut qaic_user, major: *mut u16, minor: *mut u16) -> c_int;
}
extern "C" {
    pub fn qaic_manage_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_mhi_ul_xfer_cb(mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result);
}
extern "C" {
    pub fn qaic_mhi_dl_xfer_cb(mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result);
}
extern "C" {
    pub fn qaic_control_open(qdev: *mut qaic_device) -> c_int;
}
extern "C" {
    pub fn qaic_control_close(qdev: *mut qaic_device);
}
extern "C" {
    pub fn qaic_release_usr(qdev: *mut qaic_device, usr: *mut qaic_user);
}
extern "C" {
    pub fn dbc_irq_threaded_fn(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn dbc_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn disable_dbc(qdev: *mut qaic_device, dbc_id: u32, usr: *mut qaic_user) -> c_int;
}
extern "C" {
    pub fn enable_dbc(qdev: *mut qaic_device, dbc_id: u32, usr: *mut qaic_user);
}
extern "C" {
    pub fn wakeup_dbc(qdev: *mut qaic_device, dbc_id: u32);
}
extern "C" {
    pub fn release_dbc(qdev: *mut qaic_device, dbc_id: u32);
}
extern "C" {
    pub fn qaic_data_get_fifo_info(dbc: *mut dma_bridge_chan, head: *mut u32, tail: *mut u32);
}
extern "C" {
    pub fn wake_all_cntl(qdev: *mut qaic_device);
}
extern "C" {
    pub fn qaic_dev_reset_clean_local_state(qdev: *mut qaic_device);
}
extern "C" {
    pub fn qaic_create_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_mmap_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_attach_slice_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_execute_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_partial_execute_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_wait_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_perf_stats_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_detach_slice_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn qaic_irq_polling_work(work: *mut work_struct);
}
extern "C" {
    pub fn qaic_dbc_enter_ssr(qdev: *mut qaic_device, dbc_id: u32);
}
extern "C" {
    pub fn qaic_dbc_exit_ssr(qdev: *mut qaic_device);
}
// qaic_sysfs.c
extern "C" {
    pub fn qaic_sysfs_init(qddev: *mut qaic_drm_device) -> c_int;
}
extern "C" {
    pub fn qaic_sysfs_remove(qddev: *mut qaic_drm_device);
}
extern "C" {
    pub fn set_dbc_state(qdev: *mut qaic_device, dbc_id: u32, state: c_uint);
}
