//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/pvr_drm.h
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


// SPDX-License-Identifier: (GPL-2.0-only WITH Linux-syscall-note) OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// DOC: PowerVR UAPI
//
// The PowerVR IOCTL argument structs have a few limitations in place, in
// addition to the standard kernel restrictions:
//
// - All members must be type-aligned.
// - The overall struct must be padded to 64-bit alignment.
// - Explicit padding is almost always required. This takes the form of
// ``_padding_[x]`` members of sufficient size to pad to the next power-of-two
// alignment, where [x] is the offset into the struct in hexadecimal. Arrays
// are never used for alignment. Padding fields must be zeroed; this is
// always checked.
// - Unions may only appear as the last member of a struct.
// - Individual union members may grow in the future. The space between the
// end of a union member and the end of its containing union is considered
// "implicit padding" and must be zeroed. This is always checked.
//
// In addition to the IOCTL argument structs, the PowerVR UAPI makes use of
// DEV_QUERY argument structs. These are used to fetch information about the
// device and runtime. These structs are subject to the same rules set out
// above.
//
// struct drm_pvr_obj_array - Container used to pass arrays of objects
//
// It is not unusual to have to extend objects to pass new parameters, and the DRM
// ioctl infrastructure is supporting that by padding ioctl arguments with zeros
// when the data passed by userspace is smaller than the struct defined in the
// drm_ioctl_desc, thus keeping things backward compatible. This type is just
// applying the same concepts to indirect objects passed through arrays referenced
// from the main ioctl arguments structure: the stride basically defines the size
// of the object passed by userspace, which allows the kernel driver to pad with
// zeros when it's smaller than the size of the object it expects.
//
// Use ``DRM_PVR_OBJ_ARRAY()`` to fill object array fields, unless you
// have a very good reason not to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_obj_array {
// @stride: Stride of object struct. Used for versioning.
    pub stride: __u32,
// @count: Number of objects in the array.
    pub count: __u32,
// @array: User pointer to an array of objects.
    pub array: __u64,
}

//
// DRM_PVR_OBJ_ARRAY() - Helper macro for filling &struct drm_pvr_obj_array.
// @cnt: Number of elements pointed to py @ptr.
// @ptr: Pointer to start of a C array.
//
// Return: Literal of type &struct drm_pvr_obj_array.
//

//
// DOC: PowerVR IOCTL interface
//
// PVR_IOCTL() - Build a PowerVR IOCTL number
// @_ioctl: An incrementing id for this IOCTL. Added to %DRM_COMMAND_BASE.
// @_mode: Must be one of %DRM_IOR, %DRM_IOW or %DRM_IOWR.
// @_data: The type of the args struct passed by this IOCTL.
//
// The struct referred to by @_data must have a ``drm_pvr_ioctl_`` prefix and an
// ``_args suffix``. They are therefore omitted from @_data.
//
// This should only be used to build the constants described below; it should
// never be used to call an IOCTL directly.
//
// Return: An IOCTL number to be passed to ioctl() from userspace.
//

//
// DOC: PowerVR IOCTL DEV_QUERY interface
//
// struct drm_pvr_dev_query_gpu_info - Container used to fetch information about
// the graphics processor.
//
// When fetching this type &struct drm_pvr_ioctl_dev_query_args.type must be set
// to %DRM_PVR_DEV_QUERY_GPU_INFO_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_dev_query_gpu_info {
//
// @gpu_id: GPU identifier.
//
// For all currently supported GPUs this is the BVNC encoded as a 64-bit
// value as follows:
//
// +--------+--------+--------+-------+
// | 63..48 | 47..32 | 31..16 | 15..0 |
// +========+========+========+=======+
// | B      | V      | N      | C     |
// +--------+--------+--------+-------+
//
    pub gpu_id: __u64,
//
// @num_phantoms: Number of Phantoms present.
//
    pub num_phantoms: __u32,
// @_padding_c: Reserved. This field must be zeroed.
    pub _padding_c: __u32,
}

//
// struct drm_pvr_dev_query_runtime_info - Container used to fetch information
// about the graphics runtime.
//
// When fetching this type &struct drm_pvr_ioctl_dev_query_args.type must be set
// to %DRM_PVR_DEV_QUERY_RUNTIME_INFO_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_dev_query_runtime_info {
//
// @free_list_min_pages: Minimum allowed free list size,
// in PM physical pages.
//
    pub free_list_min_pages: __u64,
//
// @free_list_max_pages: Maximum allowed free list size,
// in PM physical pages.
//
    pub free_list_max_pages: __u64,
//
// @common_store_alloc_region_size: Size of the Allocation
// Region within the Common Store used for coefficient and shared
// registers, in dwords.
//
    pub common_store_alloc_region_size: __u32,
//
// @common_store_partition_space_size: Size of the
// Partition Space within the Common Store for output buffers, in
// dwords.
//
    pub common_store_partition_space_size: __u32,
//
// @max_coeffs: Maximum coefficients, in dwords.
//
    pub max_coeffs: __u32,
//
// @cdm_max_local_mem_size_regs: Maximum amount of local
// memory available to a compute kernel, in dwords.
//
    pub cdm_max_local_mem_size_regs: __u32,
}

//
// struct drm_pvr_dev_query_quirks - Container used to fetch information about
// hardware fixes for which the device may require support in the user mode
// driver.
//
// When fetching this type &struct drm_pvr_ioctl_dev_query_args.type must be set
// to %DRM_PVR_DEV_QUERY_QUIRKS_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_dev_query_quirks {
//
// @quirks: A userspace address for the hardware quirks __u32 array.
//
// The first @musthave_count items in the list are quirks that the
// client must support for this device. If userspace does not support
// all these quirks then functionality is not guaranteed and client
// initialisation must fail.
// The remaining quirks in the list affect userspace and the kernel or
// firmware. They are disabled by default and require userspace to
// opt-in. The opt-in mechanism depends on the quirk.
//
    pub quirks: __u64,
// @count: Length of @quirks (number of __u32).
    pub count: __u16,
//
// @musthave_count: The number of entries in @quirks that are
// mandatory, starting at index 0.
//
    pub musthave_count: __u16,
// @_padding_c: Reserved. This field must be zeroed.
    pub _padding_c: __u32,
}

//
// struct drm_pvr_dev_query_enhancements - Container used to fetch information
// about optional enhancements supported by the device that require support in
// the user mode driver.
//
// When fetching this type &struct drm_pvr_ioctl_dev_query_args.type must be set
// to %DRM_PVR_DEV_ENHANCEMENTS_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_dev_query_enhancements {
//
// @enhancements: A userspace address for the hardware enhancements
// __u32 array.
//
// These enhancements affect userspace and the kernel or firmware. They
// are disabled by default and require userspace to opt-in. The opt-in
// mechanism depends on the enhancement.
//
    pub enhancements: __u64,
// @count: Length of @enhancements (number of __u32).
    pub count: __u16,
// @_padding_a: Reserved. This field must be zeroed.
    pub _padding_a: __u16,
// @_padding_c: Reserved. This field must be zeroed.
    pub _padding_c: __u32,
}

//
// enum drm_pvr_heap_id - Array index for heap info data returned by
// %DRM_PVR_DEV_QUERY_HEAP_INFO_GET.
//
// For compatibility reasons all indices will be present in the returned array,
// however some heaps may not be present. These are indicated where
// &struct drm_pvr_heap.size is set to zero.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_pvr_heap_id {
// @DRM_PVR_HEAP_GENERAL: General purpose heap.
    DRM_PVR_HEAP_GENERAL = 0,
// @DRM_PVR_HEAP_PDS_CODE_DATA: PDS code and data heap.
    DRM_PVR_HEAP_PDS_CODE_DATA,
// @DRM_PVR_HEAP_USC_CODE: USC code heap.
    DRM_PVR_HEAP_USC_CODE,
// @DRM_PVR_HEAP_RGNHDR: Region header heap. Only used if GPU has BRN63142.
    DRM_PVR_HEAP_RGNHDR,
// @DRM_PVR_HEAP_VIS_TEST: Visibility test heap.
    DRM_PVR_HEAP_VIS_TEST,
// @DRM_PVR_HEAP_TRANSFER_FRAG: Transfer fragment heap.
    DRM_PVR_HEAP_TRANSFER_FRAG,

//
// @DRM_PVR_HEAP_COUNT: The number of heaps returned by
// %DRM_PVR_DEV_QUERY_HEAP_INFO_GET.
//
// More heaps may be added, so this also serves as the copy limit when
// sent by the caller.
//
    DRM_PVR_HEAP_COUNT
// Please only add additional heaps above DRM_PVR_HEAP_COUNT!
}

//
// struct drm_pvr_heap - Container holding information about a single heap.
//
// This will always be fetched as an array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_heap {
// @base: Base address of heap.
    pub base: __u64,
// @size: Size of heap, in bytes. Will be 0 if the heap is not present.
    pub size: __u64,
// @flags: Flags for this heap. Currently always 0.
    pub flags: __u32,
// @page_size_log2: Log2 of page size.
    pub page_size_log2: __u32,
}

//
// struct drm_pvr_dev_query_heap_info - Container used to fetch information
// about heaps supported by the device driver.
//
// Please note all driver-supported heaps will be returned up to &heaps.count.
// Some heaps will not be present in all devices, which will be indicated by
// &struct drm_pvr_heap.size being set to zero.
//
// When fetching this type &struct drm_pvr_ioctl_dev_query_args.type must be set
// to %DRM_PVR_DEV_QUERY_HEAP_INFO_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_dev_query_heap_info {
//
// @heaps: Array of &struct drm_pvr_heap. If pointer is NULL, the count
// and stride will be updated with those known to the driver version, to
// facilitate allocation by the caller.
//
    pub heaps: drm_pvr_obj_array,
}

//
// enum drm_pvr_static_data_area_usage - Array index for static data area info
// returned by %DRM_PVR_DEV_QUERY_STATIC_DATA_AREAS_GET.
//
// For compatibility reasons all indices will be present in the returned array,
// however some areas may not be present. These are indicated where
// &struct drm_pvr_static_data_area.size is set to zero.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_pvr_static_data_area_usage {
//
// @DRM_PVR_STATIC_DATA_AREA_EOT: End of Tile PDS program code segment.
//
// The End of Tile PDS task runs at completion of a tile during a fragment job, and is
// responsible for emitting the tile to the Pixel Back End.
//
    DRM_PVR_STATIC_DATA_AREA_EOT = 0,

//
// @DRM_PVR_STATIC_DATA_AREA_FENCE: MCU fence area, used during cache flush and
// invalidation.
//
// This must point to valid physical memory but the contents otherwise are not used.
//
    DRM_PVR_STATIC_DATA_AREA_FENCE,

//
// @DRM_PVR_STATIC_DATA_AREA_VDM_SYNC: VDM sync program.
//
// The VDM sync program is used to synchronise multiple areas of the GPU hardware.
//
    DRM_PVR_STATIC_DATA_AREA_VDM_SYNC,

//
// @DRM_PVR_STATIC_DATA_AREA_YUV_CSC: YUV coefficients.
//
// Area contains up to 16 slots with stride of 64 bytes. Each is a 3x4 matrix of u16 fixed
// point numbers, with 1 sign bit, 2 integer bits and 13 fractional bits.
//
// The slots are :
// 0 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR
// 1 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR (full range)
// 2 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR (conformant range)
// 3 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR (full range)
// 4 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR (conformant range)
// 5 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR (full range)
// 6 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR (conformant range)
// 7 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR (full range)
// 8 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR (conformant range)
// 9 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR (conformant range, 10 bit)
// 10 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR (conformant range, 10 bit)
// 11 = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR (conformant range, 10 bit)
// 14 = Identity (biased)
// 15 = Identity
//
    DRM_PVR_STATIC_DATA_AREA_YUV_CSC,
}

//
// struct drm_pvr_static_data_area - Container holding information about a
// single static data area.
//
// This will always be fetched as an array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_static_data_area {
//
// @area_usage: Usage of static data area.
// See &enum drm_pvr_static_data_area_usage.
//
    pub area_usage: __u16,
//
// @location_heap_id: Array index of heap where this of static data
// area is located. This array is fetched using
// %DRM_PVR_DEV_QUERY_HEAP_INFO_GET.
//
    pub location_heap_id: __u16,
// @size: Size of static data area. Not present if set to zero.
    pub size: __u32,
// @offset: Offset of static data area from start of heap.
    pub offset: __u64,
}

//
// struct drm_pvr_dev_query_static_data_areas - Container used to fetch
// information about the static data areas in heaps supported by the device
// driver.
//
// Please note all driver-supported static data areas will be returned up to
// &static_data_areas.count. Some will not be present for all devices which,
// will be indicated by &struct drm_pvr_static_data_area.size being set to zero.
//
// Further, some heaps will not be present either. See &struct
// drm_pvr_dev_query_heap_info.
//
// When fetching this type &struct drm_pvr_ioctl_dev_query_args.type must be set
// to %DRM_PVR_DEV_QUERY_STATIC_DATA_AREAS_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_dev_query_static_data_areas {
//
// @static_data_areas: Array of &struct drm_pvr_static_data_area. If
// pointer is NULL, the count and stride will be updated with those
// known to the driver version, to facilitate allocation by the caller.
//
    pub static_data_areas: drm_pvr_obj_array,
}

//
// enum drm_pvr_dev_query - For use with &drm_pvr_ioctl_dev_query_args.type to
// indicate the type of the receiving container.
//
// Append only. Do not reorder.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_pvr_dev_query {
//
// @DRM_PVR_DEV_QUERY_GPU_INFO_GET: The dev query args contain a pointer
// to &struct drm_pvr_dev_query_gpu_info.
//
    DRM_PVR_DEV_QUERY_GPU_INFO_GET = 0,

//
// @DRM_PVR_DEV_QUERY_RUNTIME_INFO_GET: The dev query args contain a
// pointer to &struct drm_pvr_dev_query_runtime_info.
//
    DRM_PVR_DEV_QUERY_RUNTIME_INFO_GET,

//
// @DRM_PVR_DEV_QUERY_QUIRKS_GET: The dev query args contain a pointer
// to &struct drm_pvr_dev_query_quirks.
//
    DRM_PVR_DEV_QUERY_QUIRKS_GET,

//
// @DRM_PVR_DEV_QUERY_ENHANCEMENTS_GET: The dev query args contain a
// pointer to &struct drm_pvr_dev_query_enhancements.
//
    DRM_PVR_DEV_QUERY_ENHANCEMENTS_GET,

//
// @DRM_PVR_DEV_QUERY_HEAP_INFO_GET: The dev query args contain a
// pointer to &struct drm_pvr_dev_query_heap_info.
//
    DRM_PVR_DEV_QUERY_HEAP_INFO_GET,

//
// @DRM_PVR_DEV_QUERY_STATIC_DATA_AREAS_GET: The dev query args contain
// a pointer to &struct drm_pvr_dev_query_static_data_areas.
//
    DRM_PVR_DEV_QUERY_STATIC_DATA_AREAS_GET,
}

//
// struct drm_pvr_ioctl_dev_query_args - Arguments for %DRM_IOCTL_PVR_DEV_QUERY.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_dev_query_args {
//
// @type: Type of query and output struct. See &enum drm_pvr_dev_query.
//
    pub type: __u32,
//
// @size: Size of the receiving struct, see @type.
//
// After a successful call this will be updated to the written byte
// length.
// Can also be used to get the minimum byte length (see @pointer).
// This allows additional fields to be appended to the structs in
// future.
//
    pub size: __u32,
//
// @pointer: Pointer to struct @type.
//
// Must be large enough to contain @size bytes.
// If pointer is NULL, the expected size will be returned in the @size
// field, but no other data will be written.
//
    pub pointer: __u64,
}

//
// DOC: PowerVR IOCTL CREATE_BO interface
//
// DOC: Flags for CREATE_BO
//
// We use "device" to refer to the GPU here because of the ambiguity between CPU and GPU in some
// fonts.
//
// Device mapping options
// :DRM_PVR_BO_BYPASS_DEVICE_CACHE: Specify that device accesses to this memory will bypass the
// cache. This is used for buffers that will either be regularly updated by the CPU (eg free
// lists) or will be accessed only once and therefore isn't worth caching (eg partial render
// buffers).
// By default, the device flushes its memory caches after every job, so this is not normally
// required for coherency.
// :DRM_PVR_BO_PM_FW_PROTECT: Specify that only the Parameter Manager (PM) and/or firmware
// processor should be allowed to access this memory when mapped to the device. It is not
// valid to specify this flag with DRM_PVR_BO_ALLOW_CPU_USERSPACE_ACCESS.
//
// CPU mapping options
// :DRM_PVR_BO_ALLOW_CPU_USERSPACE_ACCESS: Allow userspace to map and access the contents of this
// memory. It is not valid to specify this flag with DRM_PVR_BO_PM_FW_PROTECT.
//

// Bits 3..63 are reserved.

//
// struct drm_pvr_ioctl_create_bo_args - Arguments for %DRM_IOCTL_PVR_CREATE_BO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_create_bo_args {
//
// @size: [IN] Size of buffer object to create. This must be page size
// aligned.
//
    pub size: __u64,
//
// @handle: [OUT] GEM handle of the new buffer object for use in
// userspace.
//
    pub handle: __u32,
// @_padding_c: Reserved. This field must be zeroed.
    pub _padding_c: __u32,
//
// @flags: [IN] Options which will affect the behaviour of this
// creation operation and future mapping operations on the created
// object. This field must be a valid combination of ``DRM_PVR_BO_*``
// values, with all bits marked as reserved set to zero.
//
    pub flags: __u64,
}

//
// DOC: PowerVR IOCTL GET_BO_MMAP_OFFSET interface
//
// struct drm_pvr_ioctl_get_bo_mmap_offset_args - Arguments for
// %DRM_IOCTL_PVR_GET_BO_MMAP_OFFSET
//
// Like other DRM drivers, the "mmap" IOCTL doesn't actually map any memory.
// Instead, it allocates a fake offset which refers to the specified buffer
// object. This offset can be used with a real mmap call on the DRM device
// itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_get_bo_mmap_offset_args {
// @handle: [IN] GEM handle of the buffer object to be mapped.
    pub handle: __u32,
// @_padding_4: Reserved. This field must be zeroed.
    pub _padding_4: __u32,
// @offset: [OUT] Fake offset to use in the real mmap call.
    pub offset: __u64,
}

//
// DOC: PowerVR IOCTL CREATE_VM_CONTEXT and DESTROY_VM_CONTEXT interfaces
//
// struct drm_pvr_ioctl_create_vm_context_args - Arguments for
// %DRM_IOCTL_PVR_CREATE_VM_CONTEXT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_create_vm_context_args {
// @handle: [OUT] Handle for new VM context.
    pub handle: __u32,
// @_padding_4: Reserved. This field must be zeroed.
    pub _padding_4: __u32,
}

//
// struct drm_pvr_ioctl_destroy_vm_context_args - Arguments for
// %DRM_IOCTL_PVR_DESTROY_VM_CONTEXT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_destroy_vm_context_args {
//
// @handle: [IN] Handle for VM context to be destroyed.
//
    pub handle: __u32,
// @_padding_4: Reserved. This field must be zeroed.
    pub _padding_4: __u32,
}

//
// DOC: PowerVR IOCTL VM_MAP and VM_UNMAP interfaces
//
// The VM UAPI allows userspace to create buffer object mappings in GPU virtual address space.
//
// The client is responsible for managing GPU address space. It should allocate mappings within
// the heaps returned by %DRM_PVR_DEV_QUERY_HEAP_INFO_GET.
//
// %DRM_IOCTL_PVR_VM_MAP creates a new mapping. The client provides the target virtual address for
// the mapping. Size and offset within the mapped buffer object can be specified, so the client can
// partially map a buffer.
//
// %DRM_IOCTL_PVR_VM_UNMAP removes a mapping. The entire mapping will be removed from GPU address
// space only if the size of the mapping matches that known to the driver.
//
// struct drm_pvr_ioctl_vm_map_args - Arguments for %DRM_IOCTL_PVR_VM_MAP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_vm_map_args {
//
// @vm_context_handle: [IN] Handle for VM context for this mapping to
// exist in.
//
    pub vm_context_handle: __u32,
// @flags: [IN] Flags which affect this mapping. Currently always 0.
    pub flags: __u32,
//
// @device_addr: [IN] Requested device-virtual address for the mapping.
// This must be non-zero and aligned to the device page size for the
// heap containing the requested address. It is an error to specify an
// address which is not contained within one of the heaps returned by
// %DRM_PVR_DEV_QUERY_HEAP_INFO_GET.
//
    pub device_addr: __u64,
//
// @handle: [IN] Handle of the target buffer object. This must be a
// valid handle returned by %DRM_IOCTL_PVR_CREATE_BO.
//
    pub handle: __u32,
// @_padding_14: Reserved. This field must be zeroed.
    pub _padding_14: __u32,
//
// @offset: [IN] Offset into the target bo from which to begin the
// mapping.
//
    pub offset: __u64,
//
// @size: [IN] Size of the requested mapping. Must be aligned to
// the device page size for the heap containing the requested address,
// as well as the host page size. When added to @device_addr, the
// result must not overflow the heap which contains @device_addr (i.e.
// the range specified by @device_addr and @size must be completely
// contained within a single heap specified by
// %DRM_PVR_DEV_QUERY_HEAP_INFO_GET).
//
    pub size: __u64,
}

//
// struct drm_pvr_ioctl_vm_unmap_args - Arguments for %DRM_IOCTL_PVR_VM_UNMAP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_vm_unmap_args {
//
// @vm_context_handle: [IN] Handle for VM context that this mapping
// exists in.
//
    pub vm_context_handle: __u32,
// @_padding_4: Reserved. This field must be zeroed.
    pub _padding_4: __u32,
//
// @device_addr: [IN] Device-virtual address at the start of the target
// mapping. This must be non-zero.
//
    pub device_addr: __u64,
//
// @size: Size in bytes of the target mapping. This must be non-zero.
//
    pub size: __u64,
}

//
// DOC: PowerVR IOCTL CREATE_CONTEXT and DESTROY_CONTEXT interfaces
//
// enum drm_pvr_ctx_priority - Arguments for
// &drm_pvr_ioctl_create_context_args.priority
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_pvr_ctx_priority {
// @DRM_PVR_CTX_PRIORITY_LOW: Priority below normal.
    DRM_PVR_CTX_PRIORITY_LOW = -512,

// @DRM_PVR_CTX_PRIORITY_NORMAL: Normal priority.
    DRM_PVR_CTX_PRIORITY_NORMAL = 0,

//
// @DRM_PVR_CTX_PRIORITY_HIGH: Priority above normal.
// Note this requires ``CAP_SYS_NICE`` or ``DRM_MASTER``.
//
    DRM_PVR_CTX_PRIORITY_HIGH = 512,
}

//
// enum drm_pvr_ctx_type - Arguments for
// &struct drm_pvr_ioctl_create_context_args.type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_pvr_ctx_type {
//
// @DRM_PVR_CTX_TYPE_RENDER: Render context.
//
    DRM_PVR_CTX_TYPE_RENDER = 0,

//
// @DRM_PVR_CTX_TYPE_COMPUTE: Compute context.
//
    DRM_PVR_CTX_TYPE_COMPUTE,

//
// @DRM_PVR_CTX_TYPE_TRANSFER_FRAG: Transfer context for fragment data
// master.
//
    DRM_PVR_CTX_TYPE_TRANSFER_FRAG,
}

//
// struct drm_pvr_ioctl_create_context_args - Arguments for
// %DRM_IOCTL_PVR_CREATE_CONTEXT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_create_context_args {
//
// @type: [IN] Type of context to create.
//
// This must be one of the values defined by &enum drm_pvr_ctx_type.
//
    pub type: __u32,
// @flags: [IN] Flags for context.
    pub flags: __u32,
//
// @priority: [IN] Priority of new context.
//
// This must be one of the values defined by &enum drm_pvr_ctx_priority.
//
    pub priority: __s32,
// @handle: [OUT] Handle for new context.
    pub handle: __u32,
//
// @static_context_state: [IN] Pointer to static context state stream.
//
    pub static_context_state: __u64,
//
// @static_context_state_len: [IN] Length of static context state, in bytes.
//
    pub static_context_state_len: __u32,
//
// @vm_context_handle: [IN] Handle for VM context that this context is
// associated with.
//
    pub vm_context_handle: __u32,
//
// @callstack_addr: [IN] Address for initial call stack pointer. Only valid
// if @type is %DRM_PVR_CTX_TYPE_RENDER, otherwise must be 0.
//
    pub callstack_addr: __u64,
}

//
// struct drm_pvr_ioctl_destroy_context_args - Arguments for
// %DRM_IOCTL_PVR_DESTROY_CONTEXT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_destroy_context_args {
//
// @handle: [IN] Handle for context to be destroyed.
//
    pub handle: __u32,
// @_padding_4: Reserved. This field must be zeroed.
    pub _padding_4: __u32,
}

//
// DOC: PowerVR IOCTL CREATE_FREE_LIST and DESTROY_FREE_LIST interfaces
//
// struct drm_pvr_ioctl_create_free_list_args - Arguments for
// %DRM_IOCTL_PVR_CREATE_FREE_LIST
//
// Free list arguments have the following constraints :
//
// - @max_num_pages must be greater than zero.
// - @grow_threshold must be between 0 and 100.
// - @grow_num_pages must be less than or equal to &max_num_pages.
// - @initial_num_pages, @max_num_pages and @grow_num_pages must be multiples
// of 4.
// - When &grow_num_pages is 0, @initial_num_pages must be equal to
// @max_num_pages.
// - When &grow_num_pages is non-zero, @initial_num_pages must be less than
// @max_num_pages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_create_free_list_args {
//
// @free_list_gpu_addr: [IN] Address of GPU mapping of buffer object
// containing memory to be used by free list.
//
// The mapped region of the buffer object must be at least
// @max_num_pages * ``sizeof(__u32)``.
//
// The buffer object must have been created with
// %DRM_PVR_BO_DEVICE_PM_FW_PROTECT set and
// %DRM_PVR_BO_CPU_ALLOW_USERSPACE_ACCESS not set.
//
    pub free_list_gpu_addr: __u64,
// @initial_num_pages: [IN] Pages initially allocated to free list.
    pub initial_num_pages: __u32,
// @max_num_pages: [IN] Maximum number of pages in free list.
    pub max_num_pages: __u32,
// @grow_num_pages: [IN] Pages to grow free list by per request.
    pub grow_num_pages: __u32,
//
// @grow_threshold: [IN] Percentage of FL memory used that should
// trigger a new grow request.
//
    pub grow_threshold: __u32,
//
// @vm_context_handle: [IN] Handle for VM context that the free list buffer
// object is mapped in.
//
    pub vm_context_handle: __u32,
//
// @handle: [OUT] Handle for created free list.
//
    pub handle: __u32,
}

//
// struct drm_pvr_ioctl_destroy_free_list_args - Arguments for
// %DRM_IOCTL_PVR_DESTROY_FREE_LIST
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_destroy_free_list_args {
//
// @handle: [IN] Handle for free list to be destroyed.
//
    pub handle: __u32,
// @_padding_4: Reserved. This field must be zeroed.
    pub _padding_4: __u32,
}

//
// DOC: PowerVR IOCTL CREATE_HWRT_DATASET and DESTROY_HWRT_DATASET interfaces
//
// struct drm_pvr_create_hwrt_geom_data_args - Geometry data arguments used for
// &struct drm_pvr_ioctl_create_hwrt_dataset_args.geom_data_args.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_create_hwrt_geom_data_args {
// @tpc_dev_addr: [IN] Tail pointer cache GPU virtual address.
    pub tpc_dev_addr: __u64,
// @tpc_size: [IN] Size of TPC, in bytes.
    pub tpc_size: __u32,
// @tpc_stride: [IN] Stride between layers in TPC, in pages
    pub tpc_stride: __u32,
// @vheap_table_dev_addr: [IN] VHEAP table GPU virtual address.
    pub vheap_table_dev_addr: __u64,
// @rtc_dev_addr: [IN] Render Target Cache virtual address.
    pub rtc_dev_addr: __u64,
}

//
// struct drm_pvr_create_hwrt_rt_data_args - Render target arguments used for
// &struct drm_pvr_ioctl_create_hwrt_dataset_args.rt_data_args.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_create_hwrt_rt_data_args {
// @pm_mlist_dev_addr: [IN] PM MLIST GPU virtual address.
    pub pm_mlist_dev_addr: __u64,
// @macrotile_array_dev_addr: [IN] Macrotile array GPU virtual address.
    pub macrotile_array_dev_addr: __u64,
// @region_header_dev_addr: [IN] Region header array GPU virtual address.
    pub region_header_dev_addr: __u64,
}

pub const PVR_DRM_HWRT_FREE_LIST_LOCAL: c_int = 0;

//
// struct drm_pvr_ioctl_create_hwrt_dataset_args - Arguments for
// %DRM_IOCTL_PVR_CREATE_HWRT_DATASET
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_create_hwrt_dataset_args {
// @geom_data_args: [IN] Geometry data arguments.
    pub geom_data_args: drm_pvr_create_hwrt_geom_data_args,
//
// @rt_data_args: [IN] Array of render target arguments.
//
// Each entry in this array represents a render target in a double buffered
// setup.
//
    pub rt_data_args: [drm_pvr_create_hwrt_rt_data_args; 2],
//
// @free_list_handles: [IN] Array of free list handles.
//
// free_list_handles[PVR_DRM_HWRT_FREE_LIST_LOCAL] must have initial
// size of at least that reported by
// &drm_pvr_dev_query_runtime_info.free_list_min_pages.
//
    pub free_list_handles: [__u32; 2],
// @width: [IN] Width in pixels.
    pub width: __u32,
// @height: [IN] Height in pixels.
    pub height: __u32,
// @samples: [IN] Number of samples.
    pub samples: __u32,
// @layers: [IN] Number of layers.
    pub layers: __u32,
// @isp_merge_lower_x: [IN] Lower X coefficient for triangle merging.
    pub isp_merge_lower_x: __u32,
// @isp_merge_lower_y: [IN] Lower Y coefficient for triangle merging.
    pub isp_merge_lower_y: __u32,
// @isp_merge_scale_x: [IN] Scale X coefficient for triangle merging.
    pub isp_merge_scale_x: __u32,
// @isp_merge_scale_y: [IN] Scale Y coefficient for triangle merging.
    pub isp_merge_scale_y: __u32,
// @isp_merge_upper_x: [IN] Upper X coefficient for triangle merging.
    pub isp_merge_upper_x: __u32,
// @isp_merge_upper_y: [IN] Upper Y coefficient for triangle merging.
    pub isp_merge_upper_y: __u32,
//
// @region_header_size: [IN] Size of region header array. This common field is used by
// both render targets in this data set.
//
// The units for this field differ depending on what version of the simple internal
// parameter format the device uses. If format 2 is in use then this is interpreted as the
// number of region headers. For other formats it is interpreted as the size in dwords.
//
    pub region_header_size: __u32,
//
// @handle: [OUT] Handle for created HWRT dataset.
//
    pub handle: __u32,
}

//
// struct drm_pvr_ioctl_destroy_hwrt_dataset_args - Arguments for
// %DRM_IOCTL_PVR_DESTROY_HWRT_DATASET
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_destroy_hwrt_dataset_args {
//
// @handle: [IN] Handle for HWRT dataset to be destroyed.
//
    pub handle: __u32,
// @_padding_4: Reserved. This field must be zeroed.
    pub _padding_4: __u32,
}

//
// DOC: PowerVR IOCTL SUBMIT_JOBS interface
//
// DOC: Flags for the drm_pvr_sync_op object.
//
// .. c:macro:: DRM_PVR_SYNC_OP_HANDLE_TYPE_MASK
//
// Handle type mask for the drm_pvr_sync_op::flags field.
//
// .. c:macro:: DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_SYNCOBJ
//
// Indicates the handle passed in drm_pvr_sync_op::handle is a syncobj handle.
// This is the default type.
//
// .. c:macro:: DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_TIMELINE_SYNCOBJ
//
// Indicates the handle passed in drm_pvr_sync_op::handle is a timeline syncobj handle.
//
// .. c:macro:: DRM_PVR_SYNC_OP_FLAG_SIGNAL
//
// Signal operation requested. The out-fence bound to the job will be attached to
// the syncobj whose handle is passed in drm_pvr_sync_op::handle.
//
// .. c:macro:: DRM_PVR_SYNC_OP_FLAG_WAIT
//
// Wait operation requested. The job will wait for this particular syncobj or syncobj
// point to be signaled before being started.
// This is the default operation.
//
pub const DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_MASK: c_uint = 0xf;
pub const DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_SYNCOBJ: c_int = 0;
pub const DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_TIMELINE_SYNCOBJ: c_int = 1;

pub const DRM_PVR_SYNC_OP_FLAG_WAIT: c_int = 0;

//
// struct drm_pvr_sync_op - Object describing a sync operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_sync_op {
// @handle: Handle of sync object.
    pub handle: __u32,
// @flags: Combination of ``DRM_PVR_SYNC_OP_FLAG_`` flags.
    pub flags: __u32,
// @value: Timeline value for this drm_syncobj. MBZ for a binary syncobj.
    pub value: __u64,
}

//
// DOC: Flags for SUBMIT_JOB ioctl geometry command.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_GEOM_CMD_FIRST
//
// Indicates if this the first command to be issued for a render.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_GEOM_CMD_LAST
//
// Indicates if this the last command to be issued for a render.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_GEOM_CMD_SINGLE_CORE
//
// Forces to use single core in a multi core device.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_GEOM_CMD_FLAGS_MASK
//
// Logical OR of all the geometry cmd flags.
//

//
// DOC: Flags for SUBMIT_JOB ioctl fragment command.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_SINGLE_CORE
//
// Use single core in a multi core setup.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_DEPTHBUFFER
//
// Indicates whether a depth buffer is present.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_STENCILBUFFER
//
// Indicates whether a stencil buffer is present.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_PREVENT_CDM_OVERLAP
//
// Disallow compute overlapped with this render.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_GET_VIS_RESULTS
//
// Indicates whether this render produces visibility results.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_SCRATCHBUFFER
//
// Indicates whether partial renders write to a scratch buffer instead of
// the final surface. It also forces the full screen copy expected to be
// present on the last render after all partial renders have completed.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_DISABLE_PIXELMERGE
//
// Disable pixel merging for this render.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_FRAG_CMD_FLAGS_MASK
//
// Logical OR of all the fragment cmd flags.
//

//
// DOC: Flags for SUBMIT_JOB ioctl compute command.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_COMPUTE_CMD_PREVENT_ALL_OVERLAP
//
// Disallow other jobs overlapped with this compute.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_COMPUTE_CMD_SINGLE_CORE
//
// Forces to use single core in a multi core device.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_COMPUTE_CMD_FLAGS_MASK
//
// Logical OR of all the compute cmd flags.
//

//
// DOC: Flags for SUBMIT_JOB ioctl transfer command.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_TRANSFER_CMD_SINGLE_CORE
//
// Forces job to use a single core in a multi core device.
//
// .. c:macro:: DRM_PVR_SUBMIT_JOB_TRANSFER_CMD_FLAGS_MASK
//
// Logical OR of all the transfer cmd flags.
//

//
// enum drm_pvr_job_type - Arguments for &struct drm_pvr_job.job_type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_pvr_job_type {
// @DRM_PVR_JOB_TYPE_GEOMETRY: Job type is geometry.
    DRM_PVR_JOB_TYPE_GEOMETRY = 0,

// @DRM_PVR_JOB_TYPE_FRAGMENT: Job type is fragment.
    DRM_PVR_JOB_TYPE_FRAGMENT,

// @DRM_PVR_JOB_TYPE_COMPUTE: Job type is compute.
    DRM_PVR_JOB_TYPE_COMPUTE,

// @DRM_PVR_JOB_TYPE_TRANSFER_FRAG: Job type is a fragment transfer.
    DRM_PVR_JOB_TYPE_TRANSFER_FRAG,
}

//
// struct drm_pvr_hwrt_data_ref - Reference HWRT data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_hwrt_data_ref {
// @set_handle: HWRT data set handle.
    pub set_handle: __u32,
// @data_index: Index of the HWRT data inside the data set.
    pub data_index: __u32,
}

//
// struct drm_pvr_job - Job arguments passed to the %DRM_IOCTL_PVR_SUBMIT_JOBS ioctl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_job {
//
// @type: [IN] Type of job being submitted
//
// This must be one of the values defined by &enum drm_pvr_job_type.
//
    pub type: __u32,
//
// @context_handle: [IN] Context handle.
//
// When @job_type is %DRM_PVR_JOB_TYPE_RENDER, %DRM_PVR_JOB_TYPE_COMPUTE or
// %DRM_PVR_JOB_TYPE_TRANSFER_FRAG, this must be a valid handle returned by
// %DRM_IOCTL_PVR_CREATE_CONTEXT. The type of context must be compatible
// with the type of job being submitted.
//
// When @job_type is %DRM_PVR_JOB_TYPE_NULL, this must be zero.
//
    pub context_handle: __u32,
//
// @flags: [IN] Flags for command.
//
// Those are job-dependent. See all ``DRM_PVR_SUBMIT_JOB_*``.
//
    pub flags: __u32,
//
// @cmd_stream_len: [IN] Length of command stream, in bytes.
//
    pub cmd_stream_len: __u32,
//
// @cmd_stream: [IN] Pointer to command stream for command.
//
// The command stream must be u64-aligned.
//
    pub cmd_stream: __u64,
// @sync_ops: [IN] Fragment sync operations.
    pub sync_ops: drm_pvr_obj_array,
//
// @hwrt: [IN] HWRT data used by render jobs (geometry or fragment).
//
// Must be zero for non-render jobs.
//
    pub hwrt: drm_pvr_hwrt_data_ref,
}

//
// struct drm_pvr_ioctl_submit_jobs_args - Arguments for %DRM_IOCTL_PVR_SUBMIT_JOB
//
// If the syscall returns an error it is important to check the value of
// @jobs.count. This indicates the index into @jobs.array where the
// error occurred.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_ioctl_submit_jobs_args {
// @jobs: [IN] Array of jobs to submit.
    pub jobs: drm_pvr_obj_array,
}

