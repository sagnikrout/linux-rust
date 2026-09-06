//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_iommu.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Virtio-iommu definition v0.12
//
// Copyright (C) 2019 Arm Ltd.
//

// Feature bits
pub const VIRTIO_IOMMU_F_INPUT_RANGE: c_int = 0;
pub const VIRTIO_IOMMU_F_DOMAIN_RANGE: c_int = 1;
pub const VIRTIO_IOMMU_F_MAP_UNMAP: c_int = 2;
pub const VIRTIO_IOMMU_F_BYPASS: c_int = 3;
pub const VIRTIO_IOMMU_F_PROBE: c_int = 4;
pub const VIRTIO_IOMMU_F_MMIO: c_int = 5;
pub const VIRTIO_IOMMU_F_BYPASS_CONFIG: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_range_64 {
    pub start: __le64,
    pub end: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_range_32 {
    pub start: __le32,
    pub end: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_config {
// Supported page sizes
    pub page_size_mask: __le64,
// Supported IOVA range
    pub input_range: virtio_iommu_range_64,
// Max domain ID size
    pub domain_range: virtio_iommu_range_32,
// Probe buffer size
    pub probe_size: __le32,
    pub bypass: __u8,
    pub reserved: [__u8; 3],
}

// Request types
pub const VIRTIO_IOMMU_T_ATTACH: c_uint = 0x01;
pub const VIRTIO_IOMMU_T_DETACH: c_uint = 0x02;
pub const VIRTIO_IOMMU_T_MAP: c_uint = 0x03;
pub const VIRTIO_IOMMU_T_UNMAP: c_uint = 0x04;
pub const VIRTIO_IOMMU_T_PROBE: c_uint = 0x05;
// Status types
pub const VIRTIO_IOMMU_S_OK: c_uint = 0x00;
pub const VIRTIO_IOMMU_S_IOERR: c_uint = 0x01;
pub const VIRTIO_IOMMU_S_UNSUPP: c_uint = 0x02;
pub const VIRTIO_IOMMU_S_DEVERR: c_uint = 0x03;
pub const VIRTIO_IOMMU_S_INVAL: c_uint = 0x04;
pub const VIRTIO_IOMMU_S_RANGE: c_uint = 0x05;
pub const VIRTIO_IOMMU_S_NOENT: c_uint = 0x06;
pub const VIRTIO_IOMMU_S_FAULT: c_uint = 0x07;
pub const VIRTIO_IOMMU_S_NOMEM: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_req_head {
    pub type: __u8,
    pub reserved: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_req_tail {
    pub status: __u8,
    pub reserved: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_req_attach {
    pub head: virtio_iommu_req_head,
    pub domain: __le32,
    pub endpoint: __le32,
    pub flags: __le32,
    pub reserved: [__u8; 4],
    pub tail: virtio_iommu_req_tail,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_req_detach {
    pub head: virtio_iommu_req_head,
    pub domain: __le32,
    pub endpoint: __le32,
    pub reserved: [__u8; 8],
    pub tail: virtio_iommu_req_tail,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_req_map {
    pub head: virtio_iommu_req_head,
    pub domain: __le32,
    pub virt_start: __le64,
    pub virt_end: __le64,
    pub phys_start: __le64,
    pub flags: __le32,
    pub tail: virtio_iommu_req_tail,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_req_unmap {
    pub head: virtio_iommu_req_head,
    pub domain: __le32,
    pub virt_start: __le64,
    pub virt_end: __le64,
    pub reserved: [__u8; 4],
    pub tail: virtio_iommu_req_tail,
}

pub const VIRTIO_IOMMU_PROBE_T_NONE: c_int = 0;
pub const VIRTIO_IOMMU_PROBE_T_RESV_MEM: c_int = 1;
pub const VIRTIO_IOMMU_PROBE_T_MASK: c_uint = 0xfff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_probe_property {
    pub type: __le16,
    pub length: __le16,
}

pub const VIRTIO_IOMMU_RESV_MEM_T_RESERVED: c_int = 0;
pub const VIRTIO_IOMMU_RESV_MEM_T_MSI: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_probe_resv_mem {
    pub head: virtio_iommu_probe_property,
    pub subtype: __u8,
    pub reserved: [__u8; 3],
    pub start: __le64,
    pub end: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_req_probe {
    pub head: virtio_iommu_req_head,
    pub endpoint: __le32,
    pub reserved: [__u8; 64],
    pub properties: [__u8; ],
//
// Tail follows the variable-length properties array. No padding,
// property lengths are all aligned on 8 bytes.
//
}

// Fault types
pub const VIRTIO_IOMMU_FAULT_R_UNKNOWN: c_int = 0;
pub const VIRTIO_IOMMU_FAULT_R_DOMAIN: c_int = 1;
pub const VIRTIO_IOMMU_FAULT_R_MAPPING: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_iommu_fault {
    pub reason: __u8,
    pub reserved: [__u8; 3],
    pub flags: __le32,
    pub endpoint: __le32,
    pub reserved2: [__u8; 4],
    pub address: __le64,
}
