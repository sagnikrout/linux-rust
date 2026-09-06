//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iommufd.h
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
//
// Copyright (C) 2021 Intel Corporation
// Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iommufd_object_type {
    IOMMUFD_OBJ_NONE,
    IOMMUFD_OBJ_ANY = IOMMUFD_OBJ_NONE,
    IOMMUFD_OBJ_DEVICE,
    IOMMUFD_OBJ_HWPT_PAGING,
    IOMMUFD_OBJ_HWPT_NESTED,
    IOMMUFD_OBJ_IOAS,
    IOMMUFD_OBJ_ACCESS,
    IOMMUFD_OBJ_FAULT,
    IOMMUFD_OBJ_VIOMMU,
    IOMMUFD_OBJ_VDEVICE,
    IOMMUFD_OBJ_VEVENTQ,
    IOMMUFD_OBJ_HW_QUEUE,

    IOMMUFD_OBJ_SELFTEST,

    IOMMUFD_OBJ_MAX,
}

// Base struct for all objects with a userspace ID handle.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_object {
//
// Destroy will sleep and wait for wait_cnt to go to zero. This allows
// concurrent users of the ID to reliably avoid causing a spurious
// destroy failure. Incrementing this count should either be short
// lived or be revoked and blocked during pre_destroy().
//
    pub wait_cnt: refcount_t,
    pub users: refcount_t,
    pub type: iommufd_object_type,
    pub id: c_uint,
}

extern "C" {
    pub fn iommufd_device_unbind(idev: *mut iommufd_device);
}
extern "C" {
    pub fn iommufd_device_detach(idev: *mut iommufd_device, pasid: ioasid_t);
}
extern "C" {
    pub fn iommufd_device_to_id(idev: *mut iommufd_device) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_access_ops {
    pub 1: u8 needs_pin_pages :,
    pub length): *mut *mut *mut void (unmap)(void data, unsigned long iova, unsigned long,
}

// Set if the caller is in a kthread then rw will use kthread_use_mm()
// Only for use by selftest
extern "C" {
    pub fn iommufd_access_destroy(access: *mut iommufd_access);
}
extern "C" {
    pub fn iommufd_access_attach(access: *mut iommufd_access, ioas_id: u32) -> c_int;
}
extern "C" {
    pub fn iommufd_access_replace(access: *mut iommufd_access, ioas_id: u32) -> c_int;
}
extern "C" {
    pub fn iommufd_access_detach(access: *mut iommufd_access);
}
extern "C" {
    pub fn iommufd_ctx_get(ictx: *mut iommufd_ctx);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_viommu {
    pub obj: iommufd_object,
    pub ictx: *mut iommufd_ctx,
    pub iommu_dev: *mut iommu_device,
    pub hwpt: *mut iommufd_hwpt_paging,
    pub ops: *const iommufd_viommu_ops,
    pub vdevs: xarray,
    pub veventqs: list_head,
    pub veventqs_rwsem: rw_semaphore,
    pub type: iommu_viommu_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_vdevice {
    pub obj: iommufd_object,
    pub viommu: *mut iommufd_viommu,
    pub idev: *mut iommufd_device,
//
// Virtual device ID per vIOMMU, e.g. vSID of ARM SMMUv3, vDeviceID of
// AMD IOMMU, and vRID of Intel VT-d
//
    pub virt_id: u64,
// Clean up all driver-specific parts of an iommufd_vdevice
    pub vdev): *mut *mut void (destroy)(struct iommufd_vdevice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_hw_queue {
    pub obj: iommufd_object,
    pub viommu: *mut iommufd_viommu,
    pub access: *mut iommufd_access,
    pub /: *mut *mut u64 base_addr; / in guest physical address space,
    pub length: usize,
    pub type: iommu_hw_queue_type,
// Clean up all driver-specific parts of an iommufd_hw_queue
    pub hw_queue): *mut *mut void (destroy)(struct iommufd_hw_queue,
}

//
// struct iommufd_viommu_ops - vIOMMU specific operations
// @destroy: Clean up all driver-specific parts of an iommufd_viommu. The memory
// of the vIOMMU will be free-ed by iommufd core after calling this op
// @alloc_domain_nested: Allocate a IOMMU_DOMAIN_NESTED on a vIOMMU that holds a
// nesting parent domain (IOMMU_DOMAIN_PAGING). @user_data
// must be defined in include/uapi/linux/iommufd.h.
// It must fully initialize the new iommu_domain before
// returning. Upon failure, ERR_PTR must be returned.
// @cache_invalidate: Flush hardware cache used by a vIOMMU. It can be used for
// any IOMMU hardware specific cache: TLB and device cache.
// The @array passes in the cache invalidation requests, in
// form of a driver data structure. A driver must update the
// array->entry_num to report the number of handled requests.
// The data structure of the array entry must be defined in
// include/uapi/linux/iommufd.h
// @vdevice_size: Size of the driver-defined vDEVICE structure per this vIOMMU
// @vdevice_init: Initialize the driver-level structure of a vDEVICE object, or
// related HW procedure. @vdev is already initialized by iommufd
// core: vdev->dev and vdev->viommu pointers; vdev->id carries a
// per-vIOMMU virtual ID (refer to struct iommu_vdevice_alloc in
// include/uapi/linux/iommufd.h)
// If driver has a deinit function to revert what vdevice_init op
// does, it should set it to the @vdev->destroy function pointer
// @get_hw_queue_size: Get the size of a driver-defined HW queue structure for a
// given @viommu corresponding to @queue_type. Driver should
// return 0 if HW queue aren't supported accordingly. It is
// required for driver to use the HW_QUEUE_STRUCT_SIZE macro
// to sanitize the driver-level HW queue structure related
// to the core one
// @hw_queue_init_phys: Initialize the driver-level structure of a HW queue that
// is initialized with its core-level structure that holds
// all the info about a guest queue memory.
// Driver providing this op indicates that HW accesses the
// guest queue memory via physical addresses.
// @index carries the logical HW QUEUE ID per vIOMMU in a
// guest VM, for a multi-queue model. @base_addr_pa carries
// the physical location of the guest queue
// If driver has a deinit function to revert what this op
// does, it should set it to the @hw_queue->destroy pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_viommu_ops {
    pub viommu): *mut *mut void (destroy)(struct iommufd_viommu,
    pub user_data): *const iommu_user_data,
    pub array): *mut iommu_user_data_array,
    pub vdevice_size: usize,
    pub vdev): *mut *mut int (vdevice_init)(struct iommufd_vdevice,
    pub queue_type): iommu_hw_queue_type,
// AMD's HW will add hw_queue_init simply using @hw_queue->base_addr
    pub base_addr_pa): phys_addr_t,
}

extern "C" {
    pub fn iommufd_ctx_put(ictx: *mut iommufd_ctx);
}
extern "C" {
    pub fn iommufd_ctx_has_group(ictx: *mut iommufd_ctx, group: *mut iommu_group) -> bool;
}
extern "C" {
    pub fn iommufd_vfio_compat_ioas_get_id(ictx: *mut iommufd_ctx, out_ioas_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn iommufd_vfio_compat_ioas_create(ictx: *mut iommufd_ctx) -> c_int;
}
extern "C" {
    pub fn iommufd_vfio_compat_set_no_iommu(ictx: *mut iommufd_ctx) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

//
// Helpers for IOMMU driver to build/destroy a dependency between two sibling
// structures created by one of the allocators above
//

//
// Helpers for IOMMU driver to alloc/destroy an mmapable area for a structure.
//
// To support an mmappable MMIO region, kernel driver must first register it to
// iommufd core to allocate an @offset, during a driver-structure initialization
// (e.g. viommu_init op). Then, it should report to user space this @offset and
// the @length of the MMIO region for mmap syscall.
//
