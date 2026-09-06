//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_mmu.h
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

// Forward declaration from "pvr_device.h"
// Forward declaration from "pvr_mmu.c"
// Forward declaration from "pvr_vm.c"
// Forward declaration from <linux/scatterlist.h>
//
// DOC: Public API (constants)
//
// .. c:macro:: PVR_DEVICE_PAGE_SIZE
//
// Fixed page size referenced by leaf nodes in the page table tree
// structure. In the current implementation, this value is pegged to the
// CPU page size (%PAGE_SIZE). It is therefore an error to specify a CPU
// page size which is not also a supported device page size. The supported
// device page sizes are: 4KiB, 16KiB, 64KiB, 256KiB, 1MiB and 2MiB.
//
// .. c:macro:: PVR_DEVICE_PAGE_SHIFT
//
// Shift value used to efficiently multiply or divide by
// %PVR_DEVICE_PAGE_SIZE.
//
// This value is derived from %PVR_DEVICE_PAGE_SIZE.
//
// .. c:macro:: PVR_DEVICE_PAGE_MASK
//
// Mask used to round a value down to the nearest multiple of
// %PVR_DEVICE_PAGE_SIZE. When bitwise negated, it will indicate whether a
// value is already a multiple of %PVR_DEVICE_PAGE_SIZE.
//
// This value is derived from %PVR_DEVICE_PAGE_SIZE.
//
// PVR_DEVICE_PAGE_SIZE determines the page size

//
// DOC: Page table index utilities (constants)
//
// .. c:macro:: PVR_PAGE_TABLE_ADDR_SPACE_SIZE
//
// Size of device-virtual address space which can be represented in the page
// table structure.
//
// This value is checked at runtime against
// &pvr_device_features.virtual_address_space_bits by
// pvr_vm_create_context(), which will return an error if the feature value
// does not match this constant.
//
// .. admonition:: Future work
//
// It should be possible to support other values of
// &pvr_device_features.virtual_address_space_bits, but so far no
// hardware has been created which advertises an unsupported value.
//
// .. c:macro:: PVR_PAGE_TABLE_ADDR_BITS
//
// Number of bits needed to represent any value less than
// %PVR_PAGE_TABLE_ADDR_SPACE_SIZE exactly.
//
// .. c:macro:: PVR_PAGE_TABLE_ADDR_MASK
//
// Bitmask of device-virtual addresses which are valid in the page table
// structure.
//
// This value is derived from %PVR_PAGE_TABLE_ADDR_SPACE_SIZE, so the same
// notes on that constant apply here.
//

extern "C" {
    pub fn pvr_mmu_flush_request_all(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_mmu_flush_exec(pvr_dev: *mut pvr_device, wait: bool) -> c_int;
}
extern "C" {
    pub fn pvr_mmu_context_destroy(ctx: *mut pvr_mmu_context);
}
extern "C" {
    pub fn pvr_mmu_get_root_table_dma_addr(ctx: *mut pvr_mmu_context) -> dma_addr_t;
}
extern "C" {
    pub fn pvr_mmu_op_context_destroy(op_ctx: *mut pvr_mmu_op_context);
}
extern "C" {
    pub fn pvr_mmu_unmap(op_ctx: *mut pvr_mmu_op_context, device_addr: u64, size: u64) -> c_int;
}
