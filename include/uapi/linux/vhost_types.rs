//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vhost_types.h
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
// Userspace interface for in-kernel virtio accelerators.
// vhost is used to reduce the number of system calls involved in virtio.
//
// Existing virtio net code is used in the guest without modification.
//
// This header includes interface used by userspace hypervisor for
// device configuration.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vring_state {
    pub index: c_uint,
    pub num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vring_file {
    pub index: c_uint,
    pub /: *mut *mut int fd; / Pass -1 to unbind from file.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vring_addr {
    pub index: c_uint,
// Option flags.
    pub flags: c_uint,
// Flag values:
// Whether log address is valid. If set enables logging.
pub const VHOST_VRING_F_LOG: c_int = 0;
// Start of array of descriptors (virtually contiguous)
    pub desc_user_addr: __u64,
// Used structure address. Must be 32 bit aligned
    pub used_user_addr: __u64,
// Available structure address. Must be 16 bit aligned
    pub avail_user_addr: __u64,
// Logging support.
// Log writes to used structure, at offset calculated from specified
// address. Address must be 32 bit aligned.
    pub log_guest_addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_worker_state {
//
// For VHOST_NEW_WORKER the kernel will return the new vhost_worker id.
// For VHOST_FREE_WORKER this must be set to the id of the vhost_worker
// to free.
//
    pub worker_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vring_worker {
// vring index
    pub index: c_uint,
// The id of the vhost_worker returned from VHOST_NEW_WORKER
    pub worker_id: c_uint,
}

// no alignment requirement
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_iotlb_msg {
    pub iova: __u64,
    pub size: __u64,
    pub uaddr: __u64,
pub const VHOST_ACCESS_RO: c_uint = 0x1;
pub const VHOST_ACCESS_WO: c_uint = 0x2;
pub const VHOST_ACCESS_RW: c_uint = 0x3;
    pub perm: __u8,
pub const VHOST_IOTLB_MISS: c_int = 1;
pub const VHOST_IOTLB_UPDATE: c_int = 2;
pub const VHOST_IOTLB_INVALIDATE: c_int = 3;
pub const VHOST_IOTLB_ACCESS_FAIL: c_int = 4;
//
// VHOST_IOTLB_BATCH_BEGIN and VHOST_IOTLB_BATCH_END allow modifying
// multiple mappings in one go: beginning with
// VHOST_IOTLB_BATCH_BEGIN, followed by any number of
// VHOST_IOTLB_UPDATE messages, and ending with VHOST_IOTLB_BATCH_END.
// When one of these two values is used as the message type, the rest
// of the fields in the message are ignored. There's no guarantee that
// these changes take place automatically in the device.
//
pub const VHOST_IOTLB_BATCH_BEGIN: c_int = 5;
pub const VHOST_IOTLB_BATCH_END: c_int = 6;
    pub type: __u8,
}

pub const VHOST_IOTLB_MSG: c_uint = 0x1;
pub const VHOST_IOTLB_MSG_V2: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_msg {
    pub type: c_int,
    pub iotlb: vhost_iotlb_msg,
    pub padding: [__u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_msg_v2 {
    pub type: __u32,
    pub asid: __u32,
    pub iotlb: vhost_iotlb_msg,
    pub padding: [__u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_features_array {
    pub /: *mut *mut __u64 count; / number of entries present in features array,
    pub __counted_by(count): __u64 features[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_memory_region {
    pub guest_phys_addr: __u64,
    pub /: *mut *mut __u64 memory_size; / bytes,
    pub userspace_addr: __u64,
    pub /: *mut *mut __u64 flags_padding; / No flags are currently specified.,
}

// All region addresses and sizes must be 4K aligned.
pub const VHOST_PAGE_SIZE: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_memory {
    pub nregions: __u32,
    pub padding: __u32,
    pub regions: [vhost_memory_region; ],
}

// VHOST_SCSI specific definitions
//
// Used by QEMU userspace to ensure a consistent vhost-scsi ABI.
//
// ABI Rev 0: July 2012 version starting point for v3.6-rc merge candidate +
// RFC-v2 vhost-scsi userspace.  Add GET_ABI_VERSION ioctl usage
// ABI Rev 1: January 2013. Ignore vhost_tpgt field in struct vhost_scsi_target.
// All the targets under vhost_wwpn can be seen and used by guset.
//
pub const VHOST_SCSI_ABI_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_scsi_target {
    pub abi_version: c_int,
    pub /: *mut *mut char vhost_wwpn[224]; / TRANSPORT_IQN_LEN,
    pub vhost_tpgt: c_ushort,
    pub reserved: c_ushort,
}

// VHOST_VDPA specific definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vdpa_config {
    pub off: __u32,
    pub len: __u32,
    pub buf: [__u8; ],
}

// vhost vdpa IOVA range
// @first: First address that can be mapped by vhost-vDPA
// @last: Last address that can be mapped by vhost-vDPA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_vdpa_iova_range {
    pub first: __u64,
    pub last: __u64,
}

// Feature bits
// Log all write descriptors. Can be changed while device is active.
pub const VHOST_F_LOG_ALL: c_int = 26;
// vhost-net should add virtio_net_hdr for RX, and strip for TX packets.
pub const VHOST_NET_F_VIRTIO_NET_HDR: c_int = 27;
// Use message type V2
pub const VHOST_BACKEND_F_IOTLB_MSG_V2: c_uint = 0x1;
// IOTLB can accept batching hints
pub const VHOST_BACKEND_F_IOTLB_BATCH: c_uint = 0x2;
// IOTLB can accept address space identifier through V2 type of IOTLB
// message
//
pub const VHOST_BACKEND_F_IOTLB_ASID: c_uint = 0x3;
// Device can be suspended
pub const VHOST_BACKEND_F_SUSPEND: c_uint = 0x4;
// Device can be resumed
pub const VHOST_BACKEND_F_RESUME: c_uint = 0x5;
// Device supports the driver enabling virtqueues both before and after
// DRIVER_OK
//
pub const VHOST_BACKEND_F_ENABLE_AFTER_DRIVER_OK: c_uint = 0x6;
// Device may expose the virtqueue's descriptor area, driver area and
// device area to a different group for ASID binding than where its
// buffers may reside. Requires VHOST_BACKEND_F_IOTLB_ASID.
//
pub const VHOST_BACKEND_F_DESC_ASID: c_uint = 0x7;
// IOTLB don't flush memory mapping across device reset
pub const VHOST_BACKEND_F_IOTLB_PERSIST: c_uint = 0x8;
