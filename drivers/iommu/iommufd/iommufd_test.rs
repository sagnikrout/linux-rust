//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/iommufd/iommufd_test.h
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
// Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES.
//

// These values are true for MOCK_IOMMUPT_DEFAULT
// Reserved for special pasid replace test
pub const IOMMU_TEST_PASID_RESERVED: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_test_cmd {
    pub size: __u32,
    pub op: __u32,
    pub id: __u32,
    pub __reserved: __u32,
    pub start: __aligned_u64,
    pub length: __aligned_u64,
    pub add_reserved: },
    pub out_stdev_id: __u32,
    pub out_hwpt_id: __u32,
// out_idev_id is the standard iommufd_bind object
    pub out_idev_id: __u32,
    pub mock_domain: },
    pub out_stdev_id: __u32,
    pub out_hwpt_id: __u32,
    pub out_idev_id: __u32,
// Expand mock_domain to set mock device flags
    pub dev_flags: __u32,
    pub mock_domain_flags: },
    pub pt_id: __u32,
    pub mock_domain_replace: },
    pub iova: __aligned_u64,
    pub length: __aligned_u64,
    pub uptr: __aligned_u64,
    pub check_map: },
    pub length: __aligned_u64,
    pub uptr: __aligned_u64,
    pub refs: __u32,
    pub check_refs: },
    pub out_access_fd: __u32,
    pub flags: __u32,
    pub create_access: },
    pub access_pages_id: __u32,
    pub destroy_access_pages: },
    pub flags: __u32,
    pub out_access_pages_id: __u32,
    pub iova: __aligned_u64,
    pub length: __aligned_u64,
    pub uptr: __aligned_u64,
    pub access_pages: },
    pub iova: __aligned_u64,
    pub length: __aligned_u64,
    pub uptr: __aligned_u64,
    pub flags: __u32,
    pub access_rw: },
    pub limit: __u32,
    pub memory_limit: },
    pub ioas_id: __u32,
    pub access_replace_ioas: },
    pub flags: __u32,
    pub iova: __aligned_u64,
    pub length: __aligned_u64,
    pub page_size: __aligned_u64,
    pub uptr: __aligned_u64,
    pub out_nr_dirty: __aligned_u64,
    pub dirty: },
    pub id: __u32,
    pub iotlb: __u32,
    pub check_iotlb: },
    pub dev_id: __u32,
    pub pasid: __u32,
    pub grpid: __u32,
    pub perm: __u32,
    pub addr: __u64,
    pub trigger_iopf: },
    pub id: __u32,
    pub cache: __u32,
    pub check_dev_cache: },
    pub dev_id: __u32,
    pub trigger_vevent: },
    pub pasid: __u32,
    pub pt_id: __u32,
// @id is stdev_id
    pub pasid_attach: },
    pub pasid: __u32,
    pub pt_id: __u32,
// @id is stdev_id
    pub pasid_replace: },
    pub pasid: __u32,
// @id is stdev_id
    pub pasid_detach: },
    pub pasid: __u32,
    pub hwpt_id: __u32,
// @id is stdev_id
    pub pasid_check: },
    pub length: __u32,
    pub open_flags: __u32,
    pub dmabuf_get: },
    pub dmabuf_fd: __s32,
    pub revoked: __u32,
    pub dmabuf_revoke: },
}

// Mock device/iommu PASID width
pub const MOCK_PASID_WIDTH: c_int = 20;
// Mock structs for IOMMU_DEVICE_GET_HW_INFO ioctl
pub const IOMMU_HW_INFO_TYPE_SELFTEST: c_uint = 0xfeedbeef;
pub const IOMMU_HW_INFO_SELFTEST_REGVAL: c_uint = 0xdeadbeef;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_test_hw_info {
    pub flags: __u32,
    pub test_reg: __u32,
}

// Should not be equal to any defined value in enum iommu_hwpt_data_type
pub const IOMMU_HWPT_DATA_SELFTEST: c_uint = 0xdead;
pub const IOMMU_TEST_IOTLB_DEFAULT: c_uint = 0xbadbeef;
pub const IOMMU_TEST_DEV_CACHE_DEFAULT: c_uint = 0xbaddad;
//
// struct iommu_hwpt_selftest
//
// @iotlb: default mock iotlb value, IOMMU_TEST_IOTLB_DEFAULT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_selftest {
    pub iotlb: __u32,
    pub pagetable_type: __u32,
}

// Should not be equal to any defined value in enum iommu_hwpt_invalidate_data_type
pub const IOMMU_HWPT_INVALIDATE_DATA_SELFTEST: c_uint = 0xdeadbeef;
pub const IOMMU_HWPT_INVALIDATE_DATA_SELFTEST_INVALID: c_uint = 0xdadbeef;
//
// struct iommu_hwpt_invalidate_selftest - Invalidation data for Mock driver
// (IOMMU_HWPT_INVALIDATE_DATA_SELFTEST)
// @flags: Invalidate flags
// @iotlb_id: Invalidate iotlb entry index
//
// If IOMMU_TEST_INVALIDATE_ALL is set in @flags, @iotlb_id will be ignored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_invalidate_selftest {

    pub flags: __u32,
    pub iotlb_id: __u32,
}

pub const IOMMU_VIOMMU_TYPE_SELFTEST: c_uint = 0xdeadbeef;
//
// struct iommu_viommu_selftest - vIOMMU data for Mock driver
// (IOMMU_VIOMMU_TYPE_SELFTEST)
// @in_data: Input random data from user space
// @out_data: Output data (matching @in_data) to user space
// @out_mmap_offset: The offset argument for mmap syscall
// @out_mmap_length: The length argument for mmap syscall
//
// Simply set @out_data=@in_data for a loopback test
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_viommu_selftest {
    pub in_data: __u32,
    pub out_data: __u32,
    pub out_mmap_offset: __aligned_u64,
    pub out_mmap_length: __aligned_u64,
}

// Should not be equal to any defined value in enum iommu_viommu_invalidate_data_type
pub const IOMMU_VIOMMU_INVALIDATE_DATA_SELFTEST: c_uint = 0xdeadbeef;
pub const IOMMU_VIOMMU_INVALIDATE_DATA_SELFTEST_INVALID: c_uint = 0xdadbeef;
//
// struct iommu_viommu_invalidate_selftest - Invalidation data for Mock VIOMMU
// (IOMMU_VIOMMU_INVALIDATE_DATA_SELFTEST)
// @flags: Invalidate flags
// @cache_id: Invalidate cache entry index
//
// If IOMMU_TEST_INVALIDATE_ALL is set in @flags, @cache_id will be ignored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_viommu_invalidate_selftest {

    pub flags: __u32,
    pub vdev_id: __u32,
    pub cache_id: __u32,
}

pub const IOMMU_VEVENTQ_TYPE_SELFTEST: c_uint = 0xbeefbeef;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_viommu_event_selftest {
    pub virt_id: __u32,
}

pub const IOMMU_HW_QUEUE_TYPE_SELFTEST: c_uint = 0xdeadbeef;
pub const IOMMU_TEST_HW_QUEUE_MAX: c_int = 2;
