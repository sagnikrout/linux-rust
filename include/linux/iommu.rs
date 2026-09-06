//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iommu.h
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
// Copyright (C) 2007-2008 Advanced Micro Devices, Inc.
// Author: Joerg Roedel <joerg.roedel@amd.com>
//

//
// Where the bus hardware includes a privilege level as part of its access type
// markings, and certain devices are capable of issuing transactions marked as
// either 'supervisor' or 'user', the IOMMU_PRIV flag requests that the other
// given permission flags only apply to accesses at the higher privilege level,
// and that unprivileged transactions should have as little access as possible.
// This would usually imply the same permissions as kernel mappings on the CPU,
// if the IOMMU page table format is equivalent.
//

// Generic fault types, can be expanded IRQ remapping fault
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iommu_fault_type {
    IOMMU_FAULT_PAGE_REQ = 1,	/* page request fault */
}

//
// struct iommu_fault_page_request - Page Request data
// @flags: encodes whether the corresponding fields are valid and whether this
// is the last page in group (IOMMU_FAULT_PAGE_REQUEST_* values).
// When IOMMU_FAULT_PAGE_RESPONSE_NEEDS_PASID is set, the page response
// must have the same PASID value as the page request. When it is clear,
// the page response should not have a PASID.
// @pasid: Process Address Space ID
// @grpid: Page Request Group Index
// @perm: requested page permissions (IOMMU_FAULT_PERM_* values)
// @addr: page address
// @private_data: device-specific private information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_fault_page_request {

    pub flags: u32,
    pub pasid: u32,
    pub grpid: u32,
    pub perm: u32,
    pub addr: u64,
    pub private_data: [u64; 2],
}

//
// struct iommu_fault - Generic fault data
// @type: fault type from &enum iommu_fault_type
// @prm: Page Request message, when @type is %IOMMU_FAULT_PAGE_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_fault {
    pub type: u32,
    pub prm: iommu_fault_page_request,
}

//
// enum iommu_page_response_code - Return status of fault handlers
// @IOMMU_PAGE_RESP_SUCCESS: Fault has been handled and the page tables
// populated, retry the access. This is "Success" in PCI PRI.
// @IOMMU_PAGE_RESP_FAILURE: General error. Drop all subsequent faults from
// this device if possible. This is "Response Failure" in PCI PRI.
// @IOMMU_PAGE_RESP_INVALID: Could not handle this fault, don't retry the
// access. This is "Invalid Request" in PCI PRI.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iommu_page_response_code {
    IOMMU_PAGE_RESP_SUCCESS = 0,
    IOMMU_PAGE_RESP_INVALID,
    IOMMU_PAGE_RESP_FAILURE,
}

//
// struct iommu_page_response - Generic page response information
// @pasid: Process Address Space ID
// @grpid: Page Request Group Index
// @code: response code from &enum iommu_page_response_code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_page_response {
    pub pasid: u32,
    pub grpid: u32,
    pub code: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopf_fault {
    pub fault: iommu_fault,
// node for pending lists
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopf_group {
    pub last_fault: iopf_fault,
    pub faults: list_head,
    pub fault_count: usize,
// list node for iommu_fault_param::faults
    pub pending_node: list_head,
    pub work: work_struct,
    pub attach_handle: *mut iommu_attach_handle,
// The device's fault data parameter.
    pub fault_param: *mut iommu_fault_param,
// Used by handler provider to hook the group on its own lists.
    pub node: list_head,
    pub cookie: u32,
}

//
// struct iopf_queue - IO Page Fault queue
// @wq: the fault workqueue
// @devices: devices attached to this queue
// @lock: protects the device list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iopf_queue {
    pub wq: *mut workqueue_struct,
    pub devices: list_head,
    pub lock: mutex,
}

// iommu fault flags
pub const IOMMU_FAULT_READ: c_uint = 0x0;
pub const IOMMU_FAULT_WRITE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_domain_geometry {
    pub /: *mut *mut dma_addr_t aperture_start; / First address that can be mapped,
    pub /: *mut *mut dma_addr_t aperture_end; / Last address that can be mapped,
    pub /: *mut *mut bool force_aperture; / DMA only allowed in mappable range?,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iommu_domain_cookie_type {
    IOMMU_COOKIE_NONE,
    IOMMU_COOKIE_DMA_IOVA,
    IOMMU_COOKIE_DMA_MSI,
    IOMMU_COOKIE_FAULT_HANDLER,
    IOMMU_COOKIE_SVA,
    IOMMU_COOKIE_IOMMUFD,
}

// Domain feature flags

//
// This are the possible domain-types
//
// IOMMU_DOMAIN_BLOCKED	- All DMA is blocked, can be used to isolate
// devices
// IOMMU_DOMAIN_IDENTITY	- DMA addresses are system physical addresses
// IOMMU_DOMAIN_UNMANAGED	- DMA mappings managed by IOMMU-API user, used
// for VMs
// IOMMU_DOMAIN_DMA	- Internally used for DMA-API implementations.
// This flag allows IOMMU drivers to implement
// certain optimizations for these domains
// IOMMU_DOMAIN_DMA_FQ	- As above, but definitely using batched TLB
// invalidation.
// IOMMU_DOMAIN_SVA	- DMA addresses are shared process addresses
// represented by mm_struct's.
// IOMMU_DOMAIN_PLATFORM	- Legacy domain for drivers that do their own
// dma_api stuff. Do not use in new drivers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_domain {
    pub type: unsigned,
    pub cookie_type: iommu_domain_cookie_type,
    pub is_iommupt: bool,
    pub ops: *const iommu_domain_ops,
    pub dirty_ops: *const iommu_dirty_ops,
    pub /: *const *const *const iommu_ops owner; / Whose domain_alloc we came from,
    pub /: *mut *mut unsigned long pgsize_bitmap; / Bitmap of page sizes in use,
    pub geometry: iommu_domain_geometry,
    pub group): *mut *mut int (iopf_handler)(struct iopf_group,
    pub iova_cookie: *mut iommu_dma_cookie,
    pub msi_cookie: *mut iommu_dma_msi_cookie,
    pub iommufd_hwpt: *mut iommufd_hw_pagetable,
    pub handler: iommu_fault_handler_t,
    pub handler_token: *mut c_void,
}

//
// Next iommu_domain in mm->iommu_mm->sva-domains list
// protected by iommu_sva_lock.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iommu_cap {
    IOMMU_CAP_CACHE_COHERENCY,	/* IOMMU_CACHE is supported */
    IOMMU_CAP_NOEXEC,		/* IOMMU_NOEXEC flag */
    IOMMU_CAP_PRE_BOOT_PROTECTION,	/* Firmware says it used the IOMMU for
    DMA protection and we should too */
//
// Per-device flag indicating if enforce_cache_coherency() will work on
// this device.
//
    IOMMU_CAP_ENFORCE_CACHE_COHERENCY,
//
// IOMMU driver does not issue TLB maintenance during .unmap, so can
// usefully support the non-strict DMA flush queue.
//
    IOMMU_CAP_DEFERRED_FLUSH,
    IOMMU_CAP_DIRTY_TRACKING,	/* IOMMU supports dirty tracking */
// ATS is supported and may be enabled for this device
    IOMMU_CAP_PCI_ATS_SUPPORTED,
}

// These are the possible reserved region types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iommu_resv_type {
// Memory regions which must be mapped 1:1 at all times
    IOMMU_RESV_DIRECT,
//
// Memory regions which are advertised to be 1:1 but are
// commonly considered relaxable in some conditions,
// for instance in device assignment use case (USB, Graphics)
//
    IOMMU_RESV_DIRECT_RELAXABLE,
// Arbitrary "never map this or give it to a device" address ranges
    IOMMU_RESV_RESERVED,
// Hardware MSI region (untranslated)
    IOMMU_RESV_MSI,
// Software-managed MSI translation window
    IOMMU_RESV_SW_MSI,
}

//
// struct iommu_resv_region - descriptor for a reserved memory region
// @list: Linked list pointers
// @start: System physical start address of the region
// @length: Length of the region in bytes
// @prot: IOMMU Protection flags (READ/WRITE/...)
// @type: Type of the reserved region
// @free: Callback to free associated memory allocations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_resv_region {
    pub list: list_head,
    pub start: phys_addr_t,
    pub length: usize,
    pub prot: c_int,
    pub type: iommu_resv_type,
    pub region): *mut *mut *mut void (free)(struct device dev, struct iommu_resv_region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_iort_rmr_data {
    pub rr: iommu_resv_region,
// Stream IDs associated with IORT RMR entry
    pub sids: *const u32,
    pub num_sids: u32,
}

pub type ioasid_t = c_uint;
// Read but do not clear any dirty bits

//
// Pages allocated through iommu_alloc_pages_node_sz() can be placed on this
// list using iommu_pages_list_add(). Note: ONLY pages from
// iommu_alloc_pages_node_sz() can be used this way!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_pages_list {
    pub pages: list_head,
}

//
// struct iommu_iotlb_gather - Range information for a pending IOTLB flush
//
// This structure is intended to be updated by multiple calls to the
// ->unmap() function in struct iommu_ops before eventually being passed
// into ->iotlb_sync(). Drivers can add pages to @freelist to be freed after
// ->iotlb_sync() or ->iotlb_flush_all() have cleared all cached references to
// them. @queued is set to indicate when ->iotlb_flush_all() will be called
// later instead of ->iotlb_sync(), so drivers may optimise accordingly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_iotlb_gather {
// @start: IOVA representing the start of the range to be flushed
    pub start: c_ulong,
//
// @end: IOVA representing the end of the range to be
// flushed (inclusive)
//
    pub end: c_ulong,
//
// @pgsize: The interval at which to perform the flush, only
// used by arm-smmu-v3
//
    pub pgsize: usize,
//
// @pt.leaf_levels_bitmap: Bitmap of generic_pt
// levels where leaf entries were unmapped. Bit 0
// means the leaf only level. If 0 no leafs
// were unmapped.
//
    pub leaf_levels_bitmap: u8,
//
// @pt.table_levels_bitmap: Bitmap of generic_pt levels
// of table entries that were removed. Bit 0 is never
// set, bit 1 means a table of all leafs was removed.
// When freelist is empty this must be 0.
//
    pub table_levels_bitmap: u8,
    pub pt: },
}

//
// @freelist: Removed pages to free after sync, only used by
// iommupt
//
// @queued: True if the gather will be completed with a flush all
//
// struct iommu_dirty_bitmap - Dirty IOVA bitmap state
// @bitmap: IOVA bitmap
// @gather: Range information for a pending IOTLB flush
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_dirty_bitmap {
    pub bitmap: *mut iova_bitmap,
    pub gather: *mut iommu_iotlb_gather,
}

//
// struct iommu_dirty_ops - domain specific dirty tracking operations
// @set_dirty_tracking: Enable or Disable dirty tracking on the iommu domain
// @read_and_clear_dirty: Walk IOMMU page tables for dirtied PTEs marshalled
// into a bitmap, with a bit represented as a page.
// Reads the dirty PTE bits and clears it from IO
// pagetables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_dirty_ops {
    pub enabled): *mut *mut *mut int (set_dirty_tracking)(struct iommu_domain domain, bool,
    pub dirty): *mut iommu_dirty_bitmap,
}

//
// struct iommu_user_data - iommu driver specific user space data info
// @type: The data type of the user buffer
// @uptr: Pointer to the user buffer for copy_from_user()
// @len: The length of the user buffer in bytes
//
// A user space data is an uAPI that is defined in include/uapi/linux/iommufd.h
// @type, @uptr and @len should be just copied from an iommufd core uAPI struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_user_data {
    pub type: c_uint,
    pub uptr: *mut void __user,
    pub len: usize,
}

//
// struct iommu_user_data_array - iommu driver specific user space data array
// @type: The data type of all the entries in the user buffer array
// @uptr: Pointer to the user buffer array
// @entry_len: The fixed-width length of an entry in the array, in bytes
// @entry_num: The number of total entries in the array
//
// The user buffer includes an array of requests with format defined in
// include/uapi/linux/iommufd.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_user_data_array {
    pub type: c_uint,
    pub uptr: *mut void __user,
    pub entry_len: usize,
    pub entry_num: u32,
}

//
// __iommu_copy_struct_from_user - Copy iommu driver specific user space data
// @dst_data: Pointer to an iommu driver specific user data that is defined in
// include/uapi/linux/iommufd.h
// @src_data: Pointer to a struct iommu_user_data for user space data info
// @data_type: The data type of the @dst_data. Must match with @src_data.type
// @data_len: Length of current user data structure, i.e. sizeof(struct _dst)
// @min_len: Initial length of user data structure for backward compatibility.
// This should be offsetofend using the last member in the user data
// struct that was initially added to include/uapi/linux/iommufd.h
//
// iommu_copy_struct_from_user - Copy iommu driver specific user space data
// @kdst: Pointer to an iommu driver specific user data that is defined in
// include/uapi/linux/iommufd.h
// @user_data: Pointer to a struct iommu_user_data for user space data info
// @data_type: The data type of the @kdst. Must match with @user_data->type
// @min_last: The last member of the data structure @kdst points in the initial
// version.
// Return 0 for success, otherwise -error.
//

//
// __iommu_copy_struct_from_user_array - Copy iommu driver specific user space
// data from an iommu_user_data_array
// @dst_data: Pointer to an iommu driver specific user data that is defined in
// include/uapi/linux/iommufd.h
// @src_array: Pointer to a struct iommu_user_data_array for a user space array
// @data_type: The data type of the @dst_data. Must match with @src_array.type
// @index: Index to the location in the array to copy user data from
// @data_len: Length of current user data structure, i.e. sizeof(struct _dst)
// @min_len: Initial length of user data structure for backward compatibility.
// This should be offsetofend using the last member in the user data
// struct that was initially added to include/uapi/linux/iommufd.h
//
// iommu_copy_struct_from_user_array - Copy iommu driver specific user space
// data from an iommu_user_data_array
// @kdst: Pointer to an iommu driver specific user data that is defined in
// include/uapi/linux/iommufd.h
// @user_array: Pointer to a struct iommu_user_data_array for a user space
// array
// @data_type: The data type of the @kdst. Must match with @user_array->type
// @index: Index to the location in the array to copy user data from
// @min_last: The last member of the data structure @kdst points in the
// initial version.
//
// Copy a single entry from a user array. Return 0 for success, otherwise
// -error.
//

//
// iommu_copy_struct_from_full_user_array - Copy iommu driver specific user
// space data from an iommu_user_data_array
// @kdst: Pointer to an iommu driver specific user data that is defined in
// include/uapi/linux/iommufd.h
// @kdst_entry_size: sizeof(*kdst)
// @user_array: Pointer to a struct iommu_user_data_array for a user space
// array
// @data_type: The data type of the @kdst. Must match with @user_array->type
//
// Copy the entire user array. kdst must have room for kdst_entry_size
// user_array->entry_num bytes. Return 0 for success, otherwise -error.
//
// Copy item by item
//
// __iommu_copy_struct_to_user - Report iommu driver specific user space data
// @dst_data: Pointer to a struct iommu_user_data for user space data location
// @src_data: Pointer to an iommu driver specific user data that is defined in
// include/uapi/linux/iommufd.h
// @data_type: The data type of the @src_data. Must match with @dst_data.type
// @data_len: Length of current user data structure, i.e. sizeof(struct _src)
// @min_len: Initial length of user data structure for backward compatibility.
// This should be offsetofend using the last member in the user data
// struct that was initially added to include/uapi/linux/iommufd.h
//
// iommu_copy_struct_to_user - Report iommu driver specific user space data
// @user_data: Pointer to a struct iommu_user_data for user space data location
// @ksrc: Pointer to an iommu driver specific user data that is defined in
// include/uapi/linux/iommufd.h
// @data_type: The data type of the @ksrc. Must match with @user_data->type
// @min_last: The last member of the data structure @ksrc points in the initial
// version.
// Return 0 for success, otherwise -error.
//

//
// struct iommu_ops - iommu ops and capabilities
// @capable: check capability
// @hw_info: report iommu hardware information. The data buffer returned by this
// op is allocated in the iommu driver and freed by the caller after
// use. @type can input a requested type and output a supported type.
// Driver should reject an unsupported data @type input
// @domain_alloc: Do not use in new drivers
// @domain_alloc_identity: allocate an IDENTITY domain. Drivers should prefer to
// use identity_domain instead. This should only be used
// if dynamic logic is necessary.
// @domain_alloc_paging_flags: Allocate an iommu domain corresponding to the
// input parameters as defined in
// include/uapi/linux/iommufd.h. The @user_data can be
// optionally provided, the new domain must support
// __IOMMU_DOMAIN_PAGING. Upon failure, ERR_PTR must be
// returned.
// @domain_alloc_paging: Allocate an iommu_domain that can be used for
// UNMANAGED, DMA, and DMA_FQ domain types. This is the
// same as invoking domain_alloc_paging_flags() with
// @flags=0, @user_data=NULL. A driver should implement
// only one of the two ops.
// @domain_alloc_sva: Allocate an iommu_domain for Shared Virtual Addressing.
// @domain_alloc_nested: Allocate an iommu_domain for nested translation.
// @probe_device: Add device to iommu driver handling
// @release_device: Remove device from iommu driver handling
// @probe_finalize: Do final setup work after the device is added to an IOMMU
// group and attached to the groups domain
// @device_group: find iommu group for a particular device
// @get_resv_regions: Request list of reserved regions for a device
// @of_xlate: add OF master IDs to iommu grouping
// @is_attach_deferred: Check if domain attach should be deferred from iommu
// driver init to device driver init (default no)
// @page_response: handle page request response
// @def_domain_type: device default domain type, return value:
// - IOMMU_DOMAIN_IDENTITY: must use an identity domain
// - IOMMU_DOMAIN_DMA: must use a dma domain
// - 0: use the default setting
// @default_domain_ops: the default ops for domains
// @get_viommu_size: Get the size of a driver-level vIOMMU structure for a given
// @dev corresponding to @viommu_type. Driver should return 0
// if vIOMMU isn't supported accordingly. It is required for
// driver to use the VIOMMU_STRUCT_SIZE macro to sanitize the
// driver-level vIOMMU structure related to the core one
// @viommu_init: Init the driver-level struct of an iommufd_viommu on a physical
// IOMMU instance @viommu->iommu_dev, as the set of virtualization
// resources shared/passed to user space IOMMU instance. Associate
// it with a nesting @parent_domain. It is required for driver to
// set @viommu->ops pointing to its own viommu_ops
// @owner: Driver module providing these ops
// @identity_domain: An always available, always attachable identity
// translation.
// @blocked_domain: An always available, always attachable blocking
// translation.
// @default_domain: If not NULL this will always be set as the default domain.
// This should be an IDENTITY/BLOCKED/PLATFORM domain.
// Do not use in new drivers.
// @user_pasid_table: IOMMU driver supports user-managed PASID table. There is
// no user domain for each PASID and the I/O page faults are
// forwarded through the user domain attached to the device
// RID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ops {
    pub iommu_cap): *mut *mut *mut bool (capable)(struct device dev, enum,
    pub type): *mut iommu_hw_info_type,
// Domain allocation and freeing by the iommu driver

    pub iommu_domain_type): *mut *mut *mut iommu_domain (domain_alloc)(unsigned,

    pub dev): *mut *mut *mut iommu_domain (domain_alloc_identity)(device,
    pub user_data): *const iommu_user_data,
    pub dev): *mut *mut *mut iommu_domain (domain_alloc_paging)(device,
    pub mm): *mut mm_struct,
    pub user_data): *const iommu_user_data,
    pub dev): *mut *mut *mut iommu_device (probe_device)(device,
    pub dev): *mut *mut void (release_device)(struct device,
    pub dev): *mut *mut void (probe_finalize)(struct device,
    pub dev): *mut *mut *mut iommu_group (device_group)(device,
// Request/Free a list of reserved regions for a device
    pub list): *mut *mut *mut void (get_resv_regions)(struct device dev, struct list_head,
    pub args): *const *const *const int (of_xlate)(struct device dev, struct of_phandle_args,
    pub dev): *mut *mut bool (is_attach_deferred)(struct device,
// Per device IOMMU features
    pub msg): *mut iommu_page_response,
    pub dev): *mut *mut int (def_domain_type)(struct device,
    pub viommu_type): iommu_viommu_type,
    pub user_data): *const iommu_user_data,
    pub default_domain_ops: *const iommu_domain_ops,
    pub owner: *mut module,
    pub identity_domain: *mut iommu_domain,
    pub blocked_domain: *mut iommu_domain,
    pub release_domain: *mut iommu_domain,
    pub default_domain: *mut iommu_domain,
    pub user_pasid_table:1: u8,
}

//
// struct iommu_domain_ops - domain specific operations
// @attach_dev: attach an iommu domain to a device
// Return:
// * 0		- success
// * EINVAL	- can indicate that device and domain are incompatible due to
// some previous configuration of the domain, in which case the
// driver shouldn't log an error, since it is legitimate for a
// caller to test reuse of existing domains. Otherwise, it may
// still represent some other fundamental problem
// * ENOMEM	- out of memory
// * ENOSPC	- non-ENOMEM type of resource allocation failures
// * EBUSY	- device is attached to a domain and cannot be changed
// * ENODEV	- device specific errors, not able to be attached
// * <others>	- treated as ENODEV by the caller. Use is discouraged
// @set_dev_pasid: set or replace an iommu domain to a pasid of device. The pasid of
// the device should be left in the old config in error case.
// @map_pages: map a physically contiguous set of pages of the same size to
// an iommu domain.
// @unmap_pages: unmap a number of pages of the same size from an iommu domain
// @flush_iotlb_all: Synchronously flush all hardware TLBs for this domain
// @iotlb_sync_map: Sync mappings created recently using @map to the hardware
// @iotlb_sync: Flush all queued ranges from the hardware TLBs and empty flush
// queue
// @cache_invalidate_user: Flush hardware cache for user space IO page table.
// The @domain must be IOMMU_DOMAIN_NESTED. The @array
// passes in the cache invalidation requests, in form
// of a driver data structure. The driver must update
// array->entry_num to report the number of handled
// invalidation requests. The driver data structure
// must be defined in include/uapi/linux/iommufd.h
// @iova_to_phys: translate iova to physical address
// @enforce_cache_coherency: Prevent any kind of DMA from bypassing IOMMU_CACHE,
// including no-snoop TLPs on PCIe or other platform
// specific mechanisms.
// @set_pgtable_quirks: Set io page table quirks (IO_PGTABLE_QUIRK_*)
// @free: Release the domain after use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_domain_ops {
    pub old): *mut iommu_domain,
    pub old): *mut ioasid_t pasid, struct iommu_domain,
    pub mapped): *mut int prot, gfp_t gfp, size_t,
    pub iotlb_gather): *mut iommu_iotlb_gather,
    pub domain): *mut *mut void (flush_iotlb_all)(struct iommu_domain,
    pub size): usize,
    pub iotlb_gather): *mut iommu_iotlb_gather,
    pub array): *mut iommu_user_data_array,
    pub iova): dma_addr_t,
    pub domain): *mut *mut bool (enforce_cache_coherency)(struct iommu_domain,
    pub quirks): c_ulong,
    pub domain): *mut *mut void (free)(struct iommu_domain,
}

//
// struct iommu_device - IOMMU core representation of one IOMMU hardware
// instance
// @list: Used by the iommu-core to keep a list of registered iommus
// @ops: iommu-ops for talking to this iommu
// @dev: struct device for sysfs handling
// @singleton_group: Used internally for drivers that have only one group
// @max_pasids: number of supported PASIDs
// @ready: set once iommu_device_register() has completed successfully
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_device {
    pub list: list_head,
    pub ops: *const iommu_ops,
    pub fwnode: *mut fwnode_handle,
    pub dev: *mut device,
    pub singleton_group: *mut iommu_group,
    pub max_pasids: u32,
    pub ready: bool,
}

//
// struct iommu_fault_param - per-device IOMMU fault data
// @lock: protect pending faults list
// @users: user counter to manage the lifetime of the data
// @rcu: rcu head for kfree_rcu()
// @dev: the device that owns this param
// @queue: IOPF queue
// @queue_list: index into queue->devices
// @partial: faults that are part of a Page Request Group for which the last
// request hasn't been submitted yet.
// @faults: holds the pending faults which need response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_fault_param {
    pub lock: mutex,
    pub users: refcount_t,
    pub rcu: rcu_head,
    pub dev: *mut device,
    pub queue: *mut iopf_queue,
    pub queue_list: list_head,
    pub partial: list_head,
    pub faults: list_head,
}

//
// struct dev_iommu - Collection of per-device IOMMU data
//
// @fault_param: IOMMU detected device fault reporting data
// @fwspec:	 IOMMU fwspec data
// @iommu_dev:	 IOMMU device this device is linked to
// @priv:	 IOMMU Driver private data
// @max_pasids:  number of PASIDs this device can consume
// @attach_deferred: the dma domain attachment is deferred
// @pci_32bit_workaround: Limit DMA allocations to 32-bit IOVAs
// @require_direct: device requires IOMMU_RESV_DIRECT regions
// @shadow_on_flush: IOTLB flushes are used to sync shadow tables
//
// TODO: migrate other per device data pointers under iommu_dev_data, e.g.
// struct iommu_group	*iommu_group;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_iommu {
    pub lock: mutex,
    pub fault_param: *mut iommu_fault_param __rcu,
    pub fwspec: *mut iommu_fwspec,
    pub iommu_dev: *mut iommu_device,
    pub priv: *mut c_void,
    pub max_pasids: u32,
    pub attach_deferred:1: u32,
    pub pci_32bit_workaround:1: u32,
    pub require_direct:1: u32,
    pub shadow_on_flush:1: u32,
}

extern "C" {
    pub fn iommu_device_unregister(iommu: *mut iommu_device);
}
extern "C" {
    pub fn iommu_device_sysfs_remove(iommu: *mut iommu_device);
}
extern "C" {
    pub fn iommu_device_link(iommu: *mut iommu_device, link: *mut device) -> c_int;
}
extern "C" {
    pub fn iommu_device_unlink(iommu: *mut iommu_device, link: *mut device);
}
extern "C" {
    pub fn iommu_deferred_attach(dev: *mut device, domain: *mut iommu_domain) -> c_int;
}
//
// iommu_get_iommu_dev - Get iommu_device for a device
// @dev: an end-point device
//
// Note that this function must be called from the iommu_ops
// to retrieve the iommu_device for a device, which the core code
// guarentees it will not invoke the op without an attached iommu.
//

// gather = (struct iommu_iotlb_gather) {
extern "C" {
    pub fn device_iommu_capable(dev: *mut device, cap: iommu_cap) -> bool;
}
extern "C" {
    pub fn iommu_group_has_isolated_msi(group: *mut iommu_group) -> bool;
}
extern "C" {
    pub fn iommu_paging_domain_alloc_flags(_arg: dev, _arg: 0) -> return;
}
extern "C" {
    pub fn iommu_domain_free(domain: *mut iommu_domain);
}
extern "C" {
    pub fn iommu_iova_to_phys(domain: *mut iommu_domain, iova: dma_addr_t) -> phys_addr_t;
}
extern "C" {
    pub fn iommu_get_resv_regions(dev: *mut device, list: *mut list_head);
}
extern "C" {
    pub fn iommu_put_resv_regions(dev: *mut device, list: *mut list_head);
}
extern "C" {
    pub fn iommu_set_default_passthrough(cmd_line: bool);
}
extern "C" {
    pub fn iommu_set_default_translated(cmd_line: bool);
}
extern "C" {
    pub fn iommu_default_passthrough() -> bool;
}
extern "C" {
    pub fn iommu_group_set_name(group: *mut iommu_group, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn iommu_group_remove_device(dev: *mut device);
}
extern "C" {
    pub fn iommu_group_put(group: *mut iommu_group);
}
extern "C" {
    pub fn iommu_group_id(group: *mut iommu_group) -> c_int;
}
extern "C" {
    pub fn iommu_set_dma_strict();
}
//
// iommu_iotlb_gather_is_disjoint - Checks whether a new range is disjoint
//
// @gather: TLB gather data
// @iova: start of page to invalidate
// @size: size of page to invalidate
//
// Helper for IOMMU drivers to check whether a new range and the gathered range
// are disjoint. For many IOMMUs, flushing the IOMMU in this case is better
// than merging the two, which might lead to unnecessary invalidations.
//
// iommu_iotlb_gather_add_range - Gather for address-based TLB invalidation
// @gather: TLB gather data
// @iova: start of page to invalidate
// @size: size of page to invalidate
//
// Helper for IOMMU drivers to build arbitrarily-sized invalidation commands
// where only the address range matters, and simply minimising intermediate
// syncs is preferred.
//
// iommu_iotlb_gather_add_page - Gather for page-based TLB invalidation
// @domain: IOMMU domain to be invalidated
// @gather: TLB gather data
// @iova: start of page to invalidate
// @size: size of page to invalidate
//
// Helper for IOMMU drivers to build invalidation commands based on individual
// pages, or with page size/table level hints which cannot be gathered if they
// differ.
//
// If the new page is disjoint from the current range or is mapped at
// a different granularity, then sync the TLB so that the gather
// structure can be rewritten.
//
// PCI device grouping function
// Generic device grouping function
// FSL-MC device grouping function
//
// struct iommu_fwspec - per-device IOMMU instance data
// @iommu_fwnode: firmware handle for this device's IOMMU
// @flags: IOMMU_FWSPEC_* flags
// @num_ids: number of associated device IDs
// @ids: IDs which this device may present to the IOMMU
//
// Note that the IDs (and any other information, really) stored in this structure should be
// considered private to the IOMMU device driver and are not to be used directly by IOMMU
// consumers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_fwspec {
    pub iommu_fwnode: *mut fwnode_handle,
    pub flags: u32,
    pub num_ids: c_uint,
    pub ids: [u32; ],
}

// ATS is supported

// CANWBS is supported

//
// An iommu attach handle represents a relationship between an iommu domain
// and a PASID or RID of a device. It is allocated and managed by the component
// that manages the domain and is stored in the iommu group during the time the
// domain is attached.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_attach_handle {
    pub domain: *mut iommu_domain,
}

//
// struct iommu_sva - handle to a device-mm bond
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_sva {
    pub handle: iommu_attach_handle,
    pub dev: *mut device,
    pub users: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_mm_data {
    pub pasid: u32,
    pub mm: *mut mm_struct,
    pub sva_domains: list_head,
    pub mm_list_elm: list_head,
}

extern "C" {
    pub fn iommu_fwspec_init(dev: *mut device, iommu_fwnode: *mut fwnode_handle) -> c_int;
}
extern "C" {
    pub fn iommu_fwspec_add_ids(dev: *mut device, ids: *const u32, num_ids: c_int) -> c_int;
}
extern "C" {
    pub fn dev_iommu_priv_set(dev: *mut device, priv: *mut c_void);
}
extern "C" {
    pub fn iommu_probe_device(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn iommu_device_use_default_domain(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn iommu_device_unuse_default_domain(dev: *mut device);
}
extern "C" {
    pub fn iommu_group_claim_dma_owner(group: *mut iommu_group, owner: *mut c_void) -> c_int;
}
extern "C" {
    pub fn iommu_group_release_dma_owner(group: *mut iommu_group);
}
extern "C" {
    pub fn iommu_group_dma_owner_claimed(group: *mut iommu_group) -> bool;
}
extern "C" {
    pub fn iommu_device_claim_dma_owner(dev: *mut device, owner: *mut c_void) -> c_int;
}
extern "C" {
    pub fn iommu_device_release_dma_owner(dev: *mut device);
}
extern "C" {
    pub fn iommu_alloc_global_pasid(dev: *mut device) -> ioasid_t;
}
extern "C" {
    pub fn iommu_free_global_pasid(pasid: ioasid_t);
}
// PCI device reset functions
extern "C" {
    pub fn pci_dev_reset_iommu_prepare(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_dev_reset_iommu_done(pdev: *mut pci_dev);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ops {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_group {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_fwspec {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_device {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_fault_param {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_iotlb_gather {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_dirty_bitmap {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_dirty_ops {
    pub false: return,
    pub ERR_PTR(-ENODEV): return,
    pub ERR_PTR(-ENODEV): return,
    pub -ENODEV: return,
    pub NULL: return,
    pub -ENODEV: return,
    pub 0: return,
    pub 0: return,
    pub -ENODEV: return,
    pub 0: return,
    pub -ENODEV: return,
    pub true: return,
    pub -ENODEV: return,
    pub ERR_PTR(-ENODEV): return,
    pub NULL: return,
    pub -ENODEV: return,
    pub -ENODEV: return,
    pub -ENODEV: return,
    pub NULL: return,
    pub -ENODEV: return,
    pub 0: return,
    pub -ENODEV: return,
    pub NULL: return,
    pub false: return,
    pub -ENODEV: return,
    pub -EINVAL: return,
    pub -ENODEV: return,
    pub -ENODEV: return,
    pub NULL: return,
    pub 0: return,
    pub -ENODEV: return,
    pub false: return,
    pub -ENODEV: return,
    pub -ENODEV: return,
    pub IOMMU_PASID_INVALID: return,
    pub 0: return,

    pub msi_addr): *mut *mut int iommu_dma_prepare_msi(struct msi_desc desc, phys_addr_t,

    pub 0: return,

    pub dev): *mut void iommu_group_mutex_assert(struct device,

//
// iommu_map_sgtable - Map the given buffer to the IOMMU domain
// @domain:	The IOMMU domain to perform the mapping
// @iova:	The start address to map the buffer
// @sgt:	The sg_table object describing the buffer
// @prot:	IOMMU protection bits
//
// Creates a mapping at @iova for the buffer described by a scatterlist
// stored in the given sg_table object in the provided IOMMU domain.
//

    pub iommu_debugfs_dir: *mut extern struct dentry,
    pub iommu_debugfs_setup(void): c_void,

    pub base): *mut *mut int iommu_get_msi_cookie(struct iommu_domain domain, dma_addr_t,

    pub -ENODEV: return,

//
// Newer generations of Tegra SoCs require devices' stream IDs to be directly programmed into
// some registers. These are always paired with a Tegra SMMU or ARM SMMU, for which the contents
// of the struct iommu_fwspec are known. Use this helper to formalize access to these internals.
//
pub const TEGRA_STREAM_ID_BYPASS: c_uint = 0x7f;

    pub dev_iommu_fwspec_get(dev): *mut *mut iommu_fwspec fwspec =,
// stream_id = fwspec->ids[0] & 0xffff;
    pub true: return,

    pub false: return,

//
// During dup_mm(), a new mm will be memcpy'd from an old one and that makes
// the new mm and the old one point to a same iommu_mm instance. When either
// one of the two mms gets released, the iommu_mm instance is freed, leaving
// the other mm running into a use-after-free/double-free problem. To avoid
// the problem, zeroing the iommu_mm pointer of a new mm is needed here.
//
    pub NULL: mm->iommu_mm =,
    pub READ_ONCE(mm->iommu_mm): return,
    pub READ_ONCE(mm->iommu_mm): *mut *mut iommu_mm_data iommu_mm =,
    pub IOMMU_PASID_INVALID: return,
    pub iommu_mm->pasid: return,
    pub mm): *mut void mm_pasid_drop(struct mm_struct,
    pub mm): *mut mm_struct,
    pub handle): *mut void iommu_sva_unbind_device(struct iommu_sva,
    pub handle): *mut u32 iommu_sva_get_pasid(struct iommu_sva,
    pub end): void iommu_sva_invalidate_kva_range(unsigned long start, unsigned long,

    pub ERR_PTR(-ENODEV): return,
    pub IOMMU_PASID_INVALID: return,
    pub }: *mut *mut static inline bool mm_valid_pasid(struct mm_struct mm) { return false;,
    pub IOMMU_PASID_INVALID: return,

    pub dev): *mut *mut int iopf_queue_add_device(struct iopf_queue queue, struct device,
    pub dev): *mut *mut void iopf_queue_remove_device(struct iopf_queue queue, struct device,
    pub dev): *mut int iopf_queue_flush_dev(struct device,
    pub name): *const *const iopf_queue iopf_queue_alloc(char,
    pub queue): *mut void iopf_queue_free(struct iopf_queue,
    pub queue): *mut int iopf_queue_discard_partial(struct iopf_queue,
    pub group): *mut void iopf_free_group(struct iopf_group,
    pub evt): *mut *mut int iommu_report_device_fault(struct device dev, struct iopf_fault,
    pub status): iommu_page_response_code,
    pub group): *mut void iopf_group_dequeue(struct iopf_group,

    pub -ENODEV: return,
    pub -ENODEV: return,
    pub NULL: return,
    pub -ENODEV: return,
    pub -ENODEV: return,

