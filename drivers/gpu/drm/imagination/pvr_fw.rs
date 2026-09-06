//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_fw.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declarations from "pvr_device.h".
// Forward declaration from "pvr_vm.h".
pub const ROGUE_FWIF_FWCCB_NUMCMDS_LOG2: c_int = 5;
pub const ROGUE_FWIF_KCCB_NUMCMDS_LOG2_DEFAULT: c_int = 7;
//
// struct pvr_fw_object - container for firmware memory allocations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_object {
// @ref_count: FW object reference counter.
    pub ref_count: kref,
// @gem: GEM object backing the FW object.
    pub gem: *mut pvr_gem_object,
//
// @fw_mm_node: Node representing mapping in FW address space. @pvr_obj->lock must
// be held when writing.
//
    pub fw_mm_node: drm_mm_node,
//
// @fw_addr_offset: Virtual address offset of firmware mapping. Only
// valid if @flags has %PVR_GEM_OBJECT_FLAGS_FW_MAPPED
// set.
//
    pub fw_addr_offset: u32,
//
// @init: Initialisation callback. Will be called on object creation and FW hard reset.
// Object will have been zeroed before this is called.
//
    pub priv): *mut *mut *mut void (init)(void cpu_ptr, void,
// @init_priv: Private data for initialisation callback.
    pub init_priv: *mut c_void,
// @node: Node for firmware object list.
    pub node: list_head,
}

//
// struct pvr_fw_defs - FW processor function table and static definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_defs {
//
// @init:
//
// FW processor specific initialisation.
// @pvr_dev: Target PowerVR device.
//
// This function must call pvr_fw_heap_calculate() to initialise the firmware heap for this
// FW processor.
//
// This function is mandatory.
//
// Returns:
// * 0 on success, or
// * Any appropriate error on failure.
//
    pub pvr_dev): *mut *mut int (init)(struct pvr_device,
//
// @fini:
//
// FW processor specific finalisation.
// @pvr_dev: Target PowerVR device.
//
// This function is optional.
//
    pub pvr_dev): *mut *mut void (fini)(struct pvr_device,
//
// @fw_process:
//
// Load and process firmware image.
// @pvr_dev: Target PowerVR device.
// @fw: Pointer to firmware image.
// @fw_code_ptr: Pointer to firmware code section.
// @fw_data_ptr: Pointer to firmware data section.
// @fw_core_code_ptr: Pointer to firmware core code section. May be %NULL.
// @fw_core_data_ptr: Pointer to firmware core data section. May be %NULL.
// @core_code_alloc_size: Total allocation size of core code section.
//
// This function is mandatory.
//
// Returns:
// * 0 on success, or
// * Any appropriate error on failure.
//
    pub core_code_alloc_size): *mut *mut u8 fw_core_data_ptr, u32,
//
// @vm_map:
//
// Map FW object into FW processor address space.
// @pvr_dev: Target PowerVR device.
// @fw_obj: FW object to map.
//
// This function is mandatory.
//
// Returns:
// * 0 on success, or
// * Any appropriate error on failure.
//
    pub fw_obj): *mut *mut *mut int (vm_map)(struct pvr_device pvr_dev, struct pvr_fw_object,
//
// @vm_unmap:
//
// Unmap FW object from FW processor address space.
// @pvr_dev: Target PowerVR device.
// @fw_obj: FW object to map.
//
// This function is mandatory.
//
    pub fw_obj): *mut *mut *mut void (vm_unmap)(struct pvr_device pvr_dev, struct pvr_fw_object,
//
// @get_fw_addr_with_offset:
//
// Called to get address of object in firmware address space, with offset.
// @fw_obj: Pointer to object.
// @offset: Desired offset from start of object.
//
// This function is mandatory.
//
// Returns:
// * Address in firmware address space.
//
    pub offset): *mut *mut *mut u32 (get_fw_addr_with_offset)(struct pvr_fw_object fw_obj, u32,
//
// @wrapper_init:
//
// Called to initialise FW wrapper.
// @pvr_dev: Target PowerVR device.
//
// This function is mandatory.
//
// Returns:
// * 0 on success.
// * Any appropriate error on failure.
//
    pub pvr_dev): *mut *mut int (wrapper_init)(struct pvr_device,
//
// @irq_pending: Check interrupt status register for pending interrupts.
//
// @pvr_dev: Target PowerVR device.
//
// This function is mandatory.
//
    pub pvr_dev): *mut *mut bool (irq_pending)(struct pvr_device,
//
// @irq_clear: Clear pending interrupts.
//
// @pvr_dev: Target PowerVR device.
//
// This function is mandatory.
//
    pub pvr_dev): *mut *mut void (irq_clear)(struct pvr_device,
//
// @has_fixed_data_addr: Specify whether the firmware fixed data must be loaded at the
// address given by the firmware layout table.
//
// This value is mandatory.
//
    pub has_fixed_data_addr: bool,
}

//
// struct pvr_fw_mem - FW memory allocations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_mem {
// @code_obj: Object representing firmware code.
    pub code_obj: *mut pvr_fw_object,
// @data_obj: Object representing firmware data.
    pub data_obj: *mut pvr_fw_object,
//
// @core_code_obj: Object representing firmware core code. May be
// %NULL if firmware does not contain this section.
//
    pub core_code_obj: *mut pvr_fw_object,
//
// @core_data_obj: Object representing firmware core data. May be
// %NULL if firmware does not contain this section.
//
    pub core_data_obj: *mut pvr_fw_object,
// @code: Driver-side copy of firmware code.
    pub code: *mut u8,
// @data: Driver-side copy of firmware data.
    pub data: *mut u8,
//
// @core_code: Driver-side copy of firmware core code. May be %NULL if firmware does not
// contain this section.
//
    pub core_code: *mut u8,
//
// @core_data: Driver-side copy of firmware core data. May be %NULL if firmware does not
// contain this section.
//
    pub core_data: *mut u8,
// @code_alloc_size: Allocation size of firmware code section.
    pub code_alloc_size: u32,
// @data_alloc_size: Allocation size of firmware data section.
    pub data_alloc_size: u32,
// @core_code_alloc_size: Allocation size of firmware core code section.
    pub core_code_alloc_size: u32,
// @core_data_alloc_size: Allocation size of firmware core data section.
    pub core_data_alloc_size: u32,
//
// @fwif_connection_ctl_obj: Object representing FWIF connection control
// structure.
//
    pub fwif_connection_ctl_obj: *mut pvr_fw_object,
// @osinit_obj: Object representing FW OSINIT structure.
    pub osinit_obj: *mut pvr_fw_object,
// @sysinit_obj: Object representing FW SYSINIT structure.
    pub sysinit_obj: *mut pvr_fw_object,
// @osdata_obj: Object representing FW OSDATA structure.
    pub osdata_obj: *mut pvr_fw_object,
// @hwrinfobuf_obj: Object representing FW hwrinfobuf structure.
    pub hwrinfobuf_obj: *mut pvr_fw_object,
// @sysdata_obj: Object representing FW SYSDATA structure.
    pub sysdata_obj: *mut pvr_fw_object,
// @power_sync_obj: Object representing power sync state.
    pub power_sync_obj: *mut pvr_fw_object,
// @fault_page_obj: Object representing FW fault page.
    pub fault_page_obj: *mut pvr_fw_object,
// @gpu_util_fwcb_obj: Object representing FW GPU utilisation control structure.
    pub gpu_util_fwcb_obj: *mut pvr_fw_object,
// @runtime_cfg_obj: Object representing FW runtime config structure.
    pub runtime_cfg_obj: *mut pvr_fw_object,
// @mmucache_sync_obj: Object used as the sync parameter in an MMU cache operation.
    pub mmucache_sync_obj: *mut pvr_fw_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_device {
// @firmware: Handle to the firmware loaded into the device.
    pub firmware: *const firmware,
// @header: Pointer to firmware header.
    pub header: *const pvr_fw_info_header,
// @layout_entries: Pointer to firmware layout.
    pub layout_entries: *const pvr_fw_layout_entry,
// @mem: Structure containing objects representing firmware memory allocations.
    pub mem: pvr_fw_mem,
//
// @initialised: %true if the firmware has been successfully initialised,
// %false otherwise.
//
    pub initialised: bool,
//
// @processor_type: FW processor type for this device. Must be one of
// %PVR_FW_PROCESSOR_TYPE_*.
//
    pub processor_type: u16,
// @funcs: Function table for the FW processor used by this device.
    pub defs: *const pvr_fw_defs,
// @processor_data: Pointer to data specific to FW processor.
// @mips_data: Pointer to MIPS-specific data.
    pub mips_data: *mut pvr_fw_mips_data,
    pub processor_data: },
// @fw_heap_info: Firmware heap information.
// @gpu_addr: Base address of firmware heap in GPU address space.
    pub gpu_addr: u64,
// @size: Size of main area of heap.
    pub size: u32,
// @offset_mask: Mask for offsets within FW heap.
    pub offset_mask: u32,
// @raw_size: Raw size of heap, including reserved areas.
    pub raw_size: u32,
// @log2_size: Log2 of raw size of heap.
    pub log2_size: u32,
// @config_offset: Offset of config area within heap.
    pub config_offset: u32,
// @reserved_size: Size of reserved area in heap.
    pub reserved_size: u32,
    pub fw_heap_info: },
// @fw_mm: Firmware address space allocator.
    pub fw_mm: drm_mm,
// @fw_mm_lock: Lock protecting access to &fw_mm.
    pub fw_mm_lock: spinlock_t,
// @fw_mm_base: Base address of address space managed by @fw_mm.
    pub fw_mm_base: u64,
//
// @fwif_connection_ctl: Pointer to CPU mapping of FWIF connection
// control structure.
//
    pub fwif_connection_ctl: *mut rogue_fwif_connection_ctl,
// @fwif_sysinit: Pointer to CPU mapping of FW SYSINIT structure.
    pub fwif_sysinit: *mut rogue_fwif_sysinit,
// @fwif_sysdata: Pointer to CPU mapping of FW SYSDATA structure.
    pub fwif_sysdata: *mut rogue_fwif_sysdata,
// @fwif_osinit: Pointer to CPU mapping of FW OSINIT structure.
    pub fwif_osinit: *mut rogue_fwif_osinit,
// @fwif_osdata: Pointer to CPU mapping of FW OSDATA structure.
    pub fwif_osdata: *mut rogue_fwif_osdata,
// @power_sync: Pointer to CPU mapping of power sync state.
    pub power_sync: *mut u32,
// @hwrinfobuf: Pointer to CPU mapping of FW HWR info buffer.
    pub hwrinfobuf: *mut rogue_fwif_hwrinfobuf,
// @fw_trace: Device firmware trace buffer state.
    pub fw_trace: pvr_fw_trace,
// @fw_objs: Structure tracking FW objects.
// @list: Head of FW object list.
    pub list: list_head,
// @lock: Lock protecting access to FW object list.
    pub lock: mutex,
    pub fw_objs: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr_fw_processor_type {
    PVR_FW_PROCESSOR_TYPE_META = 0,
    PVR_FW_PROCESSOR_TYPE_MIPS,
    PVR_FW_PROCESSOR_TYPE_RISCV,
    PVR_FW_PROCESSOR_TYPE_COUNT,
}

extern "C" {
    pub fn pvr_fw_validate_init_device_info(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_fw_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_fw_fini(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_wait_for_fw_boot(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_fw_mts_schedule(pvr_dev: *mut pvr_device, val: u32);
}
extern "C" {
    pub fn pvr_gem_object_vmap(_arg: fw_obj->gem) -> return;
}
extern "C" {
    pub fn pvr_fw_object_destroy(fw_obj: *mut pvr_fw_object);
}
//
// pvr_fw_object_get_dma_addr() - Get DMA address for given offset in firmware
// object.
// @fw_obj: Pointer to object to lookup address in.
// @offset: Offset within object to lookup address at.
// @dma_addr_out: Pointer to location to store DMA address.
//
// Returns:
// * 0 on success, or
// * -%EINVAL if object is not currently backed, or if @offset is out of valid
// range for this object.
//
extern "C" {
    pub fn pvr_gem_get_dma_addr(_arg: fw_obj->gem, _arg: offset, _arg: dma_addr_out) -> return;
}
extern "C" {
    pub fn pvr_fw_object_get_fw_addr_offset(fw_obj: *mut pvr_fw_object, offset: u32, fw_addr_out: *mut u32);
}
extern "C" {
    pub fn pvr_gem_object_size(_arg: fw_obj->gem) -> return;
}
// Util functions defined in pvr_fw_util.c. These are intended for use in pvr_fw_<arch>.c files.
