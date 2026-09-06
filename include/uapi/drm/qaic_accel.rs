//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/qaic_accel.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2021-2023 Qualcomm Innovation Center, Inc. All rights reserved.
//

// The length(4K) includes len and count fields of qaic_manage_msg

// semaphore flags
pub const QAIC_SEM_INSYNCFENCE: c_int = 2;
pub const QAIC_SEM_OUTSYNCFENCE: c_int = 1;
// Semaphore commands
pub const QAIC_SEM_NOP: c_int = 0;
pub const QAIC_SEM_INIT: c_int = 1;
pub const QAIC_SEM_INC: c_int = 2;
pub const QAIC_SEM_DEC: c_int = 3;
pub const QAIC_SEM_WAIT_EQUAL: c_int = 4;

pub const QAIC_TRANS_UNDEFINED: c_int = 0;
pub const QAIC_TRANS_PASSTHROUGH_FROM_USR: c_int = 1;
pub const QAIC_TRANS_PASSTHROUGH_TO_USR: c_int = 2;
pub const QAIC_TRANS_PASSTHROUGH_FROM_DEV: c_int = 3;
pub const QAIC_TRANS_PASSTHROUGH_TO_DEV: c_int = 4;
pub const QAIC_TRANS_DMA_XFER_FROM_USR: c_int = 5;
pub const QAIC_TRANS_DMA_XFER_TO_DEV: c_int = 6;
pub const QAIC_TRANS_ACTIVATE_FROM_USR: c_int = 7;
pub const QAIC_TRANS_ACTIVATE_FROM_DEV: c_int = 8;
pub const QAIC_TRANS_ACTIVATE_TO_DEV: c_int = 9;
pub const QAIC_TRANS_DEACTIVATE_FROM_USR: c_int = 10;
pub const QAIC_TRANS_DEACTIVATE_FROM_DEV: c_int = 11;
pub const QAIC_TRANS_STATUS_FROM_USR: c_int = 12;
pub const QAIC_TRANS_STATUS_TO_USR: c_int = 13;
pub const QAIC_TRANS_STATUS_FROM_DEV: c_int = 14;
pub const QAIC_TRANS_STATUS_TO_DEV: c_int = 15;
pub const QAIC_TRANS_TERMINATE_FROM_DEV: c_int = 16;
pub const QAIC_TRANS_TERMINATE_TO_DEV: c_int = 17;
pub const QAIC_TRANS_DMA_XFER_CONT: c_int = 18;
pub const QAIC_TRANS_VALIDATE_PARTITION_FROM_DEV: c_int = 19;
pub const QAIC_TRANS_VALIDATE_PARTITION_TO_DEV: c_int = 20;
//
// struct qaic_manage_trans_hdr - Header for a transaction in a manage message.
// @type: In. Identifies this transaction. See QAIC_TRANS_* defines.
// @len: In. Length of this transaction, including this header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_hdr {
    pub type: __u32,
    pub len: __u32,
}

//
// struct qaic_manage_trans_passthrough - Defines a passthrough transaction.
// @hdr: In. Header to identify this transaction.
// @data: In. Payload of this transaction. Opaque to the driver. Userspace must
// encode in little endian and align/pad to 64-bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_passthrough {
    pub hdr: qaic_manage_trans_hdr,
    pub data: [__u8; ],
}

//
// struct qaic_manage_trans_dma_xfer - Defines a DMA transfer transaction.
// @hdr: In. Header to identify this transaction.
// @tag: In. Identified this transfer in other transactions. Opaque to the
// driver.
// @pad: Structure padding.
// @addr: In. Address of the data to DMA to the device.
// @size: In. Length of the data to DMA to the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_dma_xfer {
    pub hdr: qaic_manage_trans_hdr,
    pub tag: __u32,
    pub pad: __u32,
    pub addr: __u64,
    pub size: __u64,
}

//
// struct qaic_manage_trans_activate_to_dev - Defines an activate request.
// @hdr: In. Header to identify this transaction.
// @queue_size: In. Number of elements for DBC request and response queues.
// @eventfd: Unused.
// @options: In. Device specific options for this activate.
// @pad: Structure padding.  Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_activate_to_dev {
    pub hdr: qaic_manage_trans_hdr,
    pub queue_size: __u32,
    pub eventfd: __u32,
    pub options: __u32,
    pub pad: __u32,
}

//
// struct qaic_manage_trans_activate_from_dev - Defines an activate response.
// @hdr: Out. Header to identify this transaction.
// @status: Out. Return code of the request from the device.
// @dbc_id: Out. Id of the assigned DBC for successful request.
// @options: Out. Device specific options for this activate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_activate_from_dev {
    pub hdr: qaic_manage_trans_hdr,
    pub status: __u32,
    pub dbc_id: __u32,
    pub options: __u64,
}

//
// struct qaic_manage_trans_deactivate - Defines a deactivate request.
// @hdr: In. Header to identify this transaction.
// @dbc_id: In. Id of assigned DBC.
// @pad: Structure padding.  Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_deactivate {
    pub hdr: qaic_manage_trans_hdr,
    pub dbc_id: __u32,
    pub pad: __u32,
}

//
// struct qaic_manage_trans_status_to_dev - Defines a status request.
// @hdr: In. Header to identify this transaction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_status_to_dev {
    pub hdr: qaic_manage_trans_hdr,
}

//
// struct qaic_manage_trans_status_from_dev - Defines a status response.
// @hdr: Out. Header to identify this transaction.
// @major: Out. NNC protocol version major number.
// @minor: Out. NNC protocol version minor number.
// @status: Out. Return code from device.
// @status_flags: Out. Flags from device.  Bit 0 indicates if CRCs are required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_trans_status_from_dev {
    pub hdr: qaic_manage_trans_hdr,
    pub major: __u16,
    pub minor: __u16,
    pub status: __u32,
    pub status_flags: __u64,
}

//
// struct qaic_manage_msg - Defines a message to the device.
// @len: In. Length of all the transactions contained within this message.
// @count: In. Number of transactions in this message.
// @data: In. Address to an array where the transactions can be found.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_manage_msg {
    pub len: __u32,
    pub count: __u32,
    pub data: __u64,
}

//
// struct qaic_create_bo - Defines a request to create a buffer object.
// @size: In.  Size of the buffer in bytes.
// @handle: Out. GEM handle for the BO.
// @pad: Structure padding. Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_create_bo {
    pub size: __u64,
    pub handle: __u32,
    pub pad: __u32,
}

//
// struct qaic_mmap_bo - Defines a request to prepare a BO for mmap().
// @handle: In.  Handle of the GEM BO to prepare for mmap().
// @pad: Structure padding. Must be 0.
// @offset: Out. Offset value to provide to mmap().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_mmap_bo {
    pub handle: __u32,
    pub pad: __u32,
    pub offset: __u64,
}

//
// struct qaic_sem - Defines a semaphore command for a BO slice.
// @val: In. Only lower 12 bits are valid.
// @index: In. Only lower 5 bits are valid.
// @presync: In. 1 if presync operation, 0 if postsync.
// @cmd: In. One of QAIC_SEM_*.
// @flags: In. Bitfield. See QAIC_SEM_INSYNCFENCE and QAIC_SEM_OUTSYNCFENCE
// @pad: Structure padding.  Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_sem {
    pub val: __u16,
    pub index: __u8,
    pub presync: __u8,
    pub cmd: __u8,
    pub flags: __u8,
    pub pad: __u16,
}

//
// struct qaic_attach_slice_entry - Defines a single BO slice.
// @size: In. Size of this slice in bytes.
// @sem0: In. Semaphore command 0. Must be 0 is not valid.
// @sem1: In. Semaphore command 1. Must be 0 is not valid.
// @sem2: In. Semaphore command 2. Must be 0 is not valid.
// @sem3: In. Semaphore command 3. Must be 0 is not valid.
// @dev_addr: In. Device address this slice pushes to or pulls from.
// @db_addr: In. Address of the doorbell to ring.
// @db_data: In. Data to write to the doorbell.
// @db_len: In. Size of the doorbell data in bits - 32, 16, or 8.  0 is for
// inactive doorbells.
// @offset: In. Start of this slice as an offset from the start of the BO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_attach_slice_entry {
    pub size: __u64,
    pub sem0: qaic_sem,
    pub sem1: qaic_sem,
    pub sem2: qaic_sem,
    pub sem3: qaic_sem,
    pub dev_addr: __u64,
    pub db_addr: __u64,
    pub db_data: __u32,
    pub db_len: __u32,
    pub offset: __u64,
}

//
// struct qaic_attach_slice_hdr - Defines metadata for a set of BO slices.
// @count: In. Number of slices for this BO.
// @dbc_id: In. Associate the sliced BO with this DBC.
// @handle: In. GEM handle of the BO to slice.
// @dir: In. Direction of data flow. 1 = DMA_TO_DEVICE, 2 = DMA_FROM_DEVICE
// @size: Deprecated. This value is ignored and size of @handle is used instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_attach_slice_hdr {
    pub count: __u32,
    pub dbc_id: __u32,
    pub handle: __u32,
    pub dir: __u32,
    pub size: __u64,
}

//
// struct qaic_attach_slice - Defines a set of BO slices.
// @hdr: In. Metadata of the set of slices.
// @data: In. Pointer to an array containing the slice definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_attach_slice {
    pub hdr: qaic_attach_slice_hdr,
    pub data: __u64,
}

//
// struct qaic_execute_entry - Defines a BO to submit to the device.
// @handle: In. GEM handle of the BO to commit to the device.
// @dir: In. Direction of data. 1 = to device, 2 = from device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_execute_entry {
    pub handle: __u32,
    pub dir: __u32,
}

//
// struct qaic_partial_execute_entry - Defines a BO to resize and submit.
// @handle: In. GEM handle of the BO to commit to the device.
// @dir: In. Direction of data. 1 = to device, 2 = from device.
// @resize: In. New size of the BO.  Must be <= the original BO size.
// @resize as 0 would be interpreted as no DMA transfer is
// involved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_partial_execute_entry {
    pub handle: __u32,
    pub dir: __u32,
    pub resize: __u64,
}

//
// struct qaic_execute_hdr - Defines metadata for BO submission.
// @count: In. Number of BOs to submit.
// @dbc_id: In. DBC to submit the BOs on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_execute_hdr {
    pub count: __u32,
    pub dbc_id: __u32,
}

//
// struct qaic_execute - Defines a list of BOs to submit to the device.
// @hdr: In. BO list metadata.
// @data: In. Pointer to an array of BOs to submit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_execute {
    pub hdr: qaic_execute_hdr,
    pub data: __u64,
}

//
// struct qaic_wait - Defines a blocking wait for BO execution.
// @handle: In. GEM handle of the BO to wait on.
// @timeout: In. Maximum time in ms to wait for the BO.
// @dbc_id: In. DBC the BO is submitted to.
// @pad: Structure padding. Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_wait {
    pub handle: __u32,
    pub timeout: __u32,
    pub dbc_id: __u32,
    pub pad: __u32,
}

//
// struct qaic_perf_stats_hdr - Defines metadata for getting BO perf info.
// @count: In. Number of BOs requested.
// @pad: Structure padding. Must be 0.
// @dbc_id: In. DBC the BO are associated with.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_perf_stats_hdr {
    pub count: __u16,
    pub pad: __u16,
    pub dbc_id: __u32,
}

//
// struct qaic_perf_stats - Defines a request for getting BO perf info.
// @hdr: In. Request metadata
// @data: In. Pointer to array of stats structures that will receive the data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_perf_stats {
    pub hdr: qaic_perf_stats_hdr,
    pub data: __u64,
}

//
// struct qaic_perf_stats_entry - Defines a BO perf info.
// @handle: In. GEM handle of the BO to get perf stats for.
// @queue_level_before: Out. Number of elements in the queue before this BO
// was submitted.
// @num_queue_element: Out. Number of elements added to the queue to submit
// this BO.
// @submit_latency_us: Out. Time taken by the driver to submit this BO.
// @device_latency_us: Out. Time taken by the device to execute this BO.
// @pad: Structure padding. Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_perf_stats_entry {
    pub handle: __u32,
    pub queue_level_before: __u32,
    pub num_queue_element: __u32,
    pub submit_latency_us: __u32,
    pub device_latency_us: __u32,
    pub pad: __u32,
}

//
// struct qaic_detach_slice - Detaches slicing configuration from BO.
// @handle: In. GEM handle of the BO to detach slicing configuration.
// @pad: Structure padding. Must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qaic_detach_slice {
    pub handle: __u32,
    pub pad: __u32,
}

pub const DRM_QAIC_MANAGE: c_uint = 0x00;
pub const DRM_QAIC_CREATE_BO: c_uint = 0x01;
pub const DRM_QAIC_MMAP_BO: c_uint = 0x02;
pub const DRM_QAIC_ATTACH_SLICE_BO: c_uint = 0x03;
pub const DRM_QAIC_EXECUTE_BO: c_uint = 0x04;
pub const DRM_QAIC_PARTIAL_EXECUTE_BO: c_uint = 0x05;
pub const DRM_QAIC_WAIT_BO: c_uint = 0x06;
pub const DRM_QAIC_PERF_STATS_BO: c_uint = 0x07;
pub const DRM_QAIC_DETACH_SLICE_BO: c_uint = 0x08;

