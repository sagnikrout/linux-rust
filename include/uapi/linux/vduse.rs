//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vduse.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)

pub const VDUSE_BASE: c_uint = 0x81;
// The ioctls for control device (/dev/vduse/control)
pub const VDUSE_API_VERSION: c_int = 0;
// VQ groups and ASID support
pub const VDUSE_API_VERSION_1: c_int = 1;
// The VDUSE instance expects a request for vq ready
pub const VDUSE_F_QUEUE_READY: c_int = 0;
// The VDUSE instance expects a request for suspend
pub const VDUSE_F_SUSPEND: c_int = 1;
//
// Get the version of VDUSE API that kernel supported (VDUSE_API_VERSION).
// This is used for future extension.
//

// Set the version of VDUSE API that userspace supported.

//
// struct vduse_dev_config - basic configuration of a VDUSE device
// @name: VDUSE device name, needs to be NUL terminated
// @vendor_id: virtio vendor id
// @device_id: virtio device id
// @features: virtio features
// @vq_num: the number of virtqueues
// @vq_align: the allocation alignment of virtqueue's metadata
// @ngroups: number of vq groups that VDUSE device declares
// @nas: number of address spaces that VDUSE device declares
// @reserved: for future use, needs to be initialized to zero
// @config_size: the size of the configuration space
// @config: the buffer of the configuration space
//
// Structure used by VDUSE_CREATE_DEV ioctl to create VDUSE device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_dev_config {
pub const VDUSE_NAME_MAX: c_int = 256;
    pub name: [c_char; VDUSE_NAME_MAX],
    pub vendor_id: __u32,
    pub device_id: __u32,
    pub features: __u64,
    pub vq_num: __u32,
    pub vq_align: __u32,
    pub /: *mut *mut __u32 ngroups; / if VDUSE_API_VERSION >= 1,
    pub /: *mut *mut __u32 nas; / if VDUSE_API_VERSION >= 1,
    pub reserved: [__u32; 11],
    pub config_size: __u32,
    pub config: [__u8; ],
}

// Create a VDUSE device which is represented by a char device (/dev/vduse/$NAME)

//
// Destroy a VDUSE device. Make sure there are no more references
// to the char device (/dev/vduse/$NAME).
//

// Get the VDUSE supported features

// Set the VDUSE features

// The ioctls for VDUSE device (/dev/vduse/$NAME)
//
// struct vduse_iotlb_entry - entry of IOTLB to describe one IOVA region [start, last]
// @offset: the mmap offset on returned file descriptor
// @start: start of the IOVA region
// @last: last of the IOVA region
// @perm: access permission of the IOVA region
//
// Structure used by VDUSE_IOTLB_GET_FD ioctl to find an overlapped IOVA region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_iotlb_entry {
    pub offset: __u64,
    pub start: __u64,
    pub last: __u64,
pub const VDUSE_ACCESS_RO: c_uint = 0x1;
pub const VDUSE_ACCESS_WO: c_uint = 0x2;
pub const VDUSE_ACCESS_RW: c_uint = 0x3;
    pub perm: __u8,
}

//
// Find the first IOVA region that overlaps with the range [start, last]
// and return the corresponding file descriptor. Return -EINVAL means the
// IOVA region doesn't exist. Caller should set start and last fields.
//

//
// Get the negotiated virtio features. It's a subset of the features in
// struct vduse_dev_config which can be accepted by virtio driver. It's
// only valid after FEATURES_OK status bit is set.
//

//
// struct vduse_config_data - data used to update configuration space
// @offset: the offset from the beginning of configuration space
// @length: the length to write to configuration space
// @buffer: the buffer used to write from
//
// Structure used by VDUSE_DEV_SET_CONFIG ioctl to update device
// configuration space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_config_data {
    pub offset: __u32,
    pub length: __u32,
    pub buffer: [__u8; ],
}

// Set device configuration space

//
// Inject a config interrupt. It's usually used to notify virtio driver
// that device configuration space has changed.
//

//
// struct vduse_vq_config - basic configuration of a virtqueue
// @index: virtqueue index
// @max_size: the max size of virtqueue
// @reserved1: for future use, needs to be initialized to zero
// @group: virtqueue group
// @reserved2: for future use, needs to be initialized to zero
//
// Structure used by VDUSE_VQ_SETUP ioctl to setup a virtqueue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_config {
    pub index: __u32,
    pub max_size: __u16,
    pub reserved1: __u16,
    pub group: __u32,
    pub reserved2: [__u16; 10],
}

//
// Setup the specified virtqueue. Make sure all virtqueues have been
// configured before the device is attached to vDPA bus.
//

//
// struct vduse_vq_state_split - split virtqueue state
// @avail_index: available index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_state_split {
    pub avail_index: __u16,
}

//
// struct vduse_vq_state_packed - packed virtqueue state
// @last_avail_counter: last driver ring wrap counter observed by device
// @last_avail_idx: device available index
// @last_used_counter: device ring wrap counter
// @last_used_idx: used index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_state_packed {
    pub last_avail_counter: __u16,
    pub last_avail_idx: __u16,
    pub last_used_counter: __u16,
    pub last_used_idx: __u16,
}

//
// struct vduse_vq_group_asid - virtqueue group ASID
// @group: Index of the virtqueue group
// @asid: Address space ID of the group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_group_asid {
    pub group: __u32,
    pub asid: __u32,
}

//
// struct vduse_vq_info - information of a virtqueue
// @index: virtqueue index
// @num: the size of virtqueue
// @desc_addr: address of desc area
// @driver_addr: address of driver area
// @device_addr: address of device area
// @split: split virtqueue state
// @packed: packed virtqueue state
// @ready: ready status of virtqueue
//
// Structure used by VDUSE_VQ_GET_INFO ioctl to get virtqueue's information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_info {
    pub index: __u32,
    pub num: __u32,
    pub desc_addr: __u64,
    pub driver_addr: __u64,
    pub device_addr: __u64,
    pub split: vduse_vq_state_split,
    pub packed: vduse_vq_state_packed,
}

// Get the specified virtqueue's information. Caller should set index field.

//
// struct vduse_vq_eventfd - eventfd configuration for a virtqueue
// @index: virtqueue index
// @fd: eventfd, -1 means de-assigning the eventfd
//
// Structure used by VDUSE_VQ_SETUP_KICKFD ioctl to setup kick eventfd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_eventfd {
    pub index: __u32,

    pub fd: c_int,
}

//
// Setup kick eventfd for specified virtqueue. The kick eventfd is used
// by VDUSE kernel module to notify userspace to consume the avail vring.
//

//
// Inject an interrupt for specific virtqueue. It's used to notify virtio driver
// to consume the used vring.
//

//
// struct vduse_iova_umem - userspace memory configuration for one IOVA region
// @uaddr: start address of userspace memory, it must be aligned to page size
// @iova: start of the IOVA region
// @size: size of the IOVA region
// @asid: Address space ID of the IOVA region
// @reserved: for future use, needs to be initialized to zero
//
// Structure used by VDUSE_IOTLB_REG_UMEM and VDUSE_IOTLB_DEREG_UMEM
// ioctls to register/de-register userspace memory for IOVA regions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_iova_umem {
    pub uaddr: __u64,
    pub iova: __u64,
    pub size: __u64,
    pub asid: __u32,
    pub reserved: [__u32; 5],
}

// Register userspace memory for IOVA regions

// De-register the userspace memory. Caller should set iova and size field.

//
// struct vduse_iova_info - information of one IOVA region
// @start: start of the IOVA region
// @last: last of the IOVA region
// @capability: capability of the IOVA region
// @asid: Address space ID of the IOVA region, only if device API version >= 1
// @reserved: for future use, needs to be initialized to zero
//
// Structure used by VDUSE_IOTLB_GET_INFO ioctl to get information of
// one IOVA region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_iova_info {
    pub start: __u64,
    pub last: __u64,

    pub capability: __u64,
    pub /: *mut *mut __u32 asid; / Only if device API version >= 1,
    pub reserved: [__u32; 5],
}

//
// Find the first IOVA region that overlaps with the range [start, last]
// and return some information on it. Caller should set start and last fields.
//

//
// struct vduse_iotlb_entry_v2 - entry of IOTLB to describe one IOVA region
//
// @v1: the original vduse_iotlb_entry
// @asid: address space ID of the IOVA region
// @reserved: for future use, needs to be initialized to zero
//
// Structure used by VDUSE_IOTLB_GET_FD2 ioctl to find an overlapped IOVA region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_iotlb_entry_v2 {
    pub offset: __u64,
    pub start: __u64,
    pub last: __u64,
    pub perm: __u8,
    pub padding: [__u8; 7],
    pub asid: __u32,
    pub reserved: [__u32; 11],
}

//
// Same as VDUSE_IOTLB_GET_FD but with vduse_iotlb_entry_v2 argument that
// support extra fields.
//

// The control messages definition for read(2)/write(2) on /dev/vduse/$NAME
//
// enum vduse_req_type - request type
// @VDUSE_GET_VQ_STATE: get the state for specified virtqueue from userspace
// @VDUSE_SET_STATUS: set the device status
// @VDUSE_UPDATE_IOTLB: Notify userspace to update the memory mapping for
// specified IOVA range via VDUSE_IOTLB_GET_FD ioctl
// @VDUSE_SET_VQ_GROUP_ASID: Notify userspace to update the address space of a
// virtqueue group.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vduse_req_type {
    VDUSE_GET_VQ_STATE,
    VDUSE_SET_STATUS,
    VDUSE_UPDATE_IOTLB,
    VDUSE_SET_VQ_GROUP_ASID,
    VDUSE_SET_VQ_READY,
    VDUSE_SUSPEND,
}

//
// struct vduse_vq_state - virtqueue state
// @index: virtqueue index
// @split: split virtqueue state
// @packed: packed virtqueue state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_state {
    pub index: __u32,
    pub split: vduse_vq_state_split,
    pub packed: vduse_vq_state_packed,
}

//
// struct vduse_dev_status - device status
// @status: device status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_dev_status {
    pub status: __u8,
}

//
// struct vduse_iova_range - IOVA range [start, last]
// @start: start of the IOVA range
// @last: last of the IOVA range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_iova_range {
    pub start: __u64,
    pub last: __u64,
}

//
// struct vduse_iova_range_v2 - IOVA range [start, last] if API_VERSION >= 1
// @start: start of the IOVA range
// @last: last of the IOVA range
// @asid: address space ID of the IOVA range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_iova_range_v2 {
    pub start: __u64,
    pub last: __u64,
    pub asid: __u32,
    pub padding: __u32,
}

//
// struct vduse_vq_ready - Virtqueue ready request message
// @num: Virtqueue number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_vq_ready {
    pub num: __u32,
    pub ready: __u32,
}

//
// struct vduse_dev_request - control request
// @type: request type
// @request_id: request id
// @reserved: for future use
// @vq_state: virtqueue state, only index field is available
// @s: device status
// @iova: IOVA range for updating
// @iova_v2: IOVA range for updating if API_VERSION >= 1
// @vq_group_asid: ASID of a virtqueue group
// @vq_ready: Virtqueue ready request
// @padding: padding
//
// Structure used by read(2) on /dev/vduse/$NAME.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_dev_request {
    pub type: __u32,
    pub request_id: __u32,
    pub reserved: [__u32; 4],
    pub vq_state: vduse_vq_state,
    pub s: vduse_dev_status,
    pub iova: vduse_iova_range,
// Following members but padding exist only if vduse api
// version >= 1
//
    pub iova_v2: vduse_iova_range_v2,
    pub vq_group_asid: vduse_vq_group_asid,
// Only if VDUSE_F_QUEUE_READY is negotiated
    pub vq_ready: vduse_vq_ready,
    pub padding: [__u32; 32],
}

//
// struct vduse_dev_response - response to control request
// @request_id: corresponding request id
// @result: the result of request
// @reserved: for future use, needs to be initialized to zero
// @vq_state: virtqueue state
// @padding: padding
//
// Structure used by write(2) on /dev/vduse/$NAME.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_dev_response {
    pub request_id: __u32,
pub const VDUSE_REQ_RESULT_OK: c_uint = 0x00;
pub const VDUSE_REQ_RESULT_FAILED: c_uint = 0x01;
    pub result: __u32,
    pub reserved: [__u32; 4],
    pub vq_state: vduse_vq_state,
    pub padding: [__u32; 32],
}
