//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/iommufd/iommufd_private.h
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
// Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_sw_msi_map {
    pub sw_msi_item: list_head,
    pub sw_msi_start: phys_addr_t,
    pub msi_addr: phys_addr_t,
    pub pgoff: c_uint,
    pub id: c_uint,
}

// Bitmap of struct iommufd_sw_msi_map::id
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_sw_msi_maps {
    pub 64): DECLARE_BITMAP(bitmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_ctx {
    pub file: *mut file,
    pub objects: xarray,
    pub groups: xarray,
    pub destroy_wait: wait_queue_head_t,
    pub ioas_creation_lock: rw_semaphore,
    pub mt_mmap: maple_tree,
    pub sw_msi_lock: mutex,
    pub sw_msi_list: list_head,
    pub sw_msi_id: c_uint,
    pub account_mode: u8,
// Compatibility with VFIO no iommu
    pub no_iommu_mode: u8,
    pub vfio_ioas: *mut iommufd_ioas,
}

// Entry for iommufd_ctx::mt_mmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_mmap {
    pub owner: *mut iommufd_object,
// Page-shifted start position in mt_mmap to validate vma->vm_pgoff
    pub vm_pgoff: c_ulong,
// Physical range for io_remap_pfn_range()
    pub mmio_addr: phys_addr_t,
    pub length: usize,
}

//
// The IOVA to PFN map. The map automatically copies the PFNs into multiple
// domains and permits sharing of PFNs between io_pagetable instances. This
// supports both a design where IOAS's are 1:1 with a domain (eg because the
// domain is HW customized), or where the IOAS is 1:N with multiple generic
// domains.  The io_pagetable holds an interval tree of iopt_areas which point
// to shared iopt_pages which hold the pfns mapped to the page table.
//
// The locking order is domains_rwsem -> iova_rwsem -> pages::mutex
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_pagetable {
    pub domains_rwsem: rw_semaphore,
    pub domains: xarray,
    pub access_list: xarray,
    pub next_domain_id: c_uint,
    pub iova_rwsem: rw_semaphore,
    pub area_itree: rb_root_cached,
// IOVA that cannot become reserved, struct iopt_allowed
    pub allowed_itree: rb_root_cached,
// IOVA that cannot be allocated, struct iopt_reserved
    pub reserved_itree: rb_root_cached,
    pub disable_large_pages: u8,
    pub iova_alignment: c_ulong,
}

extern "C" {
    pub fn iopt_init_table(iopt: *mut io_pagetable);
}
extern "C" {
    pub fn iopt_destroy_table(iopt: *mut io_pagetable);
}
extern "C" {
    pub fn iopt_free_pages_list(pages_list: *mut list_head);
}
extern "C" {
    pub fn iopt_unmap_all(iopt: *mut io_pagetable, unmapped: *mut c_ulong) -> c_int;
}

extern "C" {
    pub fn iopt_remove_reserved_iova(iopt: *mut io_pagetable, owner: *mut c_void);
}
extern "C" {
    pub fn iopt_enable_large_pages(iopt: *mut io_pagetable);
}
extern "C" {
    pub fn iopt_disable_large_pages(iopt: *mut io_pagetable) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_ucmd {
    pub ictx: *mut iommufd_ctx,
    pub ubuffer: *mut void __user,
    pub user_size: u32,
    pub cmd: *mut c_void,
    pub new_obj: *mut iommufd_object,
}

// Copy the response in ucmd->cmd back to userspace.
//
// If the caller doesn't already have a ref on obj this must be
// called under the xa_lock. Otherwise the caller is holding a
// ref on users. Thus it cannot be one before this decrement.
//
extern "C" {
    pub fn iommufd_try_inc_users(ictx: *mut iommufd_ctx, obj: *mut iommufd_object) -> c_int;
}
//
// Users first, then wait_cnt so that REMOVE_WAIT never sees a spurious
// !0 users with a 0 wait_cnt.
//
extern "C" {
    pub fn iommufd_object_abort(ictx: *mut iommufd_ctx, obj: *mut iommufd_object);
}
//
// The caller holds a users refcount and wants to destroy the object. At this
// point the caller has no wait_cnt reference and at least the xarray will be
// holding one.
//
// If there is a bug and we couldn't destroy the object then we did put
// back the caller's users refcount and will eventually try to free it
// again during close.
//
// Similar to iommufd_object_destroy_user(), except that the object ID is left
// reserved/tombstoned.
//
// If there is a bug and we couldn't destroy the object then we did put
// back the caller's users refcount and will eventually try to free it
// again during close.
//
// The HWPT allocated by autodomains is used in possibly many devices and
// is automatically destroyed when its refcount reaches zero.
//
// If userspace uses the HWPT manually, even for a short term, then it will
// disrupt this refcounting and the auto-free in the kernel will not work.
// Userspace that tries to use the automatically allocated HWPT must be careful
// to ensure that it is consistently destroyed, eg by not racing accesses
// and by not attaching an automatic HWPT to a device manually.
//
// Callers of these normal object allocators must call iommufd_object_finalize()
// to finalize the object, or call iommufd_object_abort_and_destroy() to revert
// the allocation.
//

//
// Callers of these _ucmd allocators should not call iommufd_object_finalize()
// or iommufd_object_abort_and_destroy(), as the core automatically does that.
//

//
// The IO Address Space (IOAS) pagetable is a virtual page table backed by the
// io_pagetable object. It is a user controlled mapping of IOVA -> PFNs. The
// mapping is copied into all of the associated domains and made available to
// in-kernel users.
//
// Every iommu_domain that is created is wrapped in a iommufd_hw_pagetable
// object. When we go to attach a device to an IOAS we need to get an
// iommu_domain and wrapping iommufd_hw_pagetable for it.
//
// An iommu_domain & iommfd_hw_pagetable will be automatically selected
// for a device based on the hwpt_list. If no suitable iommu_domain
// is found a new iommu_domain will be created.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_ioas {
    pub obj: iommufd_object,
    pub iopt: io_pagetable,
    pub mutex: mutex,
    pub hwpt_list: list_head,
}

extern "C" {
    pub fn iommufd_ioas_alloc_ioctl(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_ioas_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_ioas_iova_ranges(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_ioas_allow_iovas(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_ioas_map(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_ioas_map_file(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_ioas_change_process(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_ioas_copy(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_ioas_unmap(ucmd: *mut iommufd_ucmd) -> c_int;
}

extern "C" {
    pub fn iommufd_ioas_noiommu_get_pa(ucmd: *mut iommufd_ucmd) -> c_int;
}

extern "C" {
    pub fn iommufd_ioas_option(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_vfio_ioas(ucmd: *mut iommufd_ucmd) -> c_int;
}
//
// A HW pagetable is called an iommu_domain inside the kernel. This user object
// allows directly creating and inspecting the domains. Domains that have kernel
// owned page tables will be associated with an iommufd_ioas that provides the
// IOVA to PFN map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_hw_pagetable {
    pub obj: iommufd_object,
    pub domain: *mut iommu_domain,
    pub fault: *mut iommufd_fault,
    pub 1: bool pasid_compat :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_hwpt_paging {
    pub common: iommufd_hw_pagetable,
    pub ioas: *mut iommufd_ioas,
    pub 1: bool auto_domain :,
    pub 1: bool enforce_cache_coherency :,
    pub 1: bool nest_parent :,
// Head at iommufd_ioas::hwpt_list
    pub hwpt_item: list_head,
    pub present_sw_msi: iommufd_sw_msi_maps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_hwpt_nested {
    pub common: iommufd_hw_pagetable,
    pub parent: *mut iommufd_hwpt_paging,
    pub viommu: *mut iommufd_viommu,
}

extern "C" {
    pub fn container_of(_arg: hwpt, iommufd_hwpt_paging: struct, _arg: common) -> return;
}
extern "C" {
    pub fn container_of(_arg: hwpt, iommufd_hwpt_nested: struct, _arg: common) -> return;
}
extern "C" {
    pub fn to_hwpt_paging(_arg: hwpt) -> return;
}
extern "C" {
    pub fn iommufd_hwpt_set_dirty_tracking(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_hwpt_get_dirty_bitmap(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_hwpt_paging_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_hwpt_paging_abort(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_hwpt_nested_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_hwpt_nested_abort(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_hwpt_alloc(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_hwpt_invalidate(ucmd: *mut iommufd_ucmd) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_group {
    pub ref: kref,
    pub lock: mutex,
    pub ictx: *mut iommufd_ctx,
    pub group: *mut iommu_group,
    pub pasid_attach: xarray,
    pub required_sw_msi: iommufd_sw_msi_maps,
    pub sw_msi_start: phys_addr_t,
}

//
// A iommufd_device object represents the binding relationship between a
// consuming driver and the iommufd. These objects are created/destroyed by
// external drivers, not by userspace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_device {
    pub obj: iommufd_object,
    pub ictx: *mut iommufd_ctx,
    pub igroup: *mut iommufd_group,
    pub group_item: list_head,
// always the physical device
    pub dev: *mut device,
    pub enforce_cache_coherency: bool,
    pub vdev: *mut iommufd_vdevice,
    pub destroying: bool,
}

extern "C" {
    pub fn __iommu_get_iommu_dev(_arg: idev->dev) -> return;
}
extern "C" {
    pub fn iommufd_device_pre_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_device_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_get_hw_info(ucmd: *mut iommufd_ucmd) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_access {
    pub obj: iommufd_object,
    pub ictx: *mut iommufd_ctx,
    pub ioas: *mut iommufd_ioas,
    pub ioas_unpin: *mut iommufd_ioas,
    pub ioas_lock: mutex,
    pub ops: *const iommufd_access_ops,
    pub data: *mut c_void,
    pub iova_alignment: c_ulong,
    pub iopt_access_list_id: u32,
}

extern "C" {
    pub fn iopt_add_access(iopt: *mut io_pagetable, access: *mut iommufd_access) -> c_int;
}
extern "C" {
    pub fn iommufd_access_destroy_object(obj: *mut iommufd_object);
}
// iommufd_access for internal use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_eventq {
    pub obj: iommufd_object,
    pub ictx: *mut iommufd_ctx,
    pub filep: *mut file,
    pub /: *mut *mut spinlock_t lock; / protects the deliver list,
    pub deliver: list_head,
    pub wait_queue: wait_queue_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_attach_handle {
    pub handle: iommu_attach_handle,
    pub idev: *mut iommufd_device,
}

// Convert an iommu attach handle to iommufd handle.

//
// An iommufd_fault object represents an interface to deliver I/O page faults
// to the user space. These objects are created/destroyed by the user space and
// associated with hardware page table objects during page-table allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_fault {
    pub common: iommufd_eventq,
    pub /: *mut *mut mutex mutex; / serializes response flows,
    pub response: xarray,
}

extern "C" {
    pub fn container_of(_arg: eventq, iommufd_fault: struct, _arg: common) -> return;
}
extern "C" {
    pub fn iommufd_fault_alloc(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_fault_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_fault_iopf_handler(group: *mut iopf_group) -> c_int;
}
// An iommufd_vevent represents a vIOMMU event in an iommufd_veventq
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_vevent {
    pub header: iommufd_vevent_header,
    pub /: *mut *mut list_head node; / for iommufd_eventq::deliver,
    pub data_len: isize,
    pub __counted_by(data_len): u8 event_data[],
}

//
// An iommufd_veventq object represents an interface to deliver vIOMMU events to
// the user space. It is created/destroyed by the user space and associated with
// a vIOMMU object during the allocations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_veventq {
    pub common: iommufd_eventq,
    pub viommu: *mut iommufd_viommu,
    pub /: *mut *mut list_head node; / for iommufd_viommu::veventqs,
    pub type: iommu_veventq_type,
    pub depth: c_uint,
// Use common.lock for protection
    pub num_events: u32,
    pub sequence: u32,
// Must be last as it ends in a flexible-array member.
    pub lost_events_header: iommufd_vevent,
}

extern "C" {
    pub fn container_of(_arg: eventq, iommufd_veventq: struct, _arg: common) -> return;
}
extern "C" {
    pub fn iommufd_veventq_alloc(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_veventq_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_veventq_abort(obj: *mut iommufd_object);
}
//
// Remove the lost_events_header and add the new node at the same time.
// Note the new node can be lost_events_header, for a sequence update.
//
extern "C" {
    pub fn iommufd_viommu_alloc_ioctl(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_viommu_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_vdevice_alloc_ioctl(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_vdevice_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_vdevice_abort(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_hw_queue_alloc_ioctl(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_hw_queue_destroy(obj: *mut iommufd_object);
}

extern "C" {
    pub fn iommufd_test(ucmd: *mut iommufd_ucmd) -> c_int;
}
extern "C" {
    pub fn iommufd_selftest_destroy(obj: *mut iommufd_object);
}
extern "C" {
    pub fn iommufd_should_fail() -> bool;
}
extern "C" {
    pub fn iommufd_test_init() -> int __init;
}
extern "C" {
    pub fn iommufd_test_exit();
}
extern "C" {
    pub fn iommufd_selftest_is_mock_dev(dev: *mut device) -> bool;
}

