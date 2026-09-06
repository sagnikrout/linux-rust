//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc_hwconfig.c
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

//
// GuC has a blob containing hardware configuration information (HWConfig).
// This is formatted as a simple and flexible KLV (Key/Length/Value) table.
//
// For example, a minimal version could be:
// enum device_attr {
// ATTR_SOME_VALUE = 0,
// ATTR_SOME_MASK  = 1,
// };
//
// static const u32 hwconfig[] = {
// ATTR_SOME_VALUE,
// 1,		// Value Length in DWords
// 8,		// Value
//
// ATTR_SOME_MASK,
// 3,
// 0x00FFFFFFFF, 0xFFFFFFFF, 0xFF000000,
// };
//
// The attribute ids are defined in a hardware spec.
//
    static int __guc_action_get_hwconfig(struct intel_guc *guc,
    u32 ggtt_offset, u32 ggtt_size)
    {
    u32 action[] = {
    INTEL_GUC_ACTION_GET_HWCONFIG,
    lower_32_bits(ggtt_offset),
    upper_32_bits(ggtt_offset),
    ggtt_size,
    };
    int ret;
    guc_dbg(guc, "Querying HW config table: size = %d, offset = 0x%08X\n",
    ggtt_size, ggtt_offset);
    ret = intel_guc_send_mmio(guc, action, ARRAY_SIZE(action), core::ptr::null_mut(), 0);
    if (ret == -ENXIO)
    return -ENOENT;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn guc_hwconfig_discover_size(guc: *mut intel_guc, hwconfig: *mut intel_hwconfig) -> c_int {
    static int guc_hwconfig_discover_size(struct intel_guc *guc, struct intel_hwconfig *hwconfig)
    {
    int ret;
//
// Sending a query with zero offset and size will return the
// size of the blob.
//
    ret = __guc_action_get_hwconfig(guc, 0, 0);
    if (ret < 0)
    return ret;
    if (ret == 0)
    return -EINVAL;
    hwconfig.size = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn guc_hwconfig_fill_buffer(guc: *mut intel_guc, hwconfig: *mut intel_hwconfig) -> c_int {
    static int guc_hwconfig_fill_buffer(struct intel_guc *guc, struct intel_hwconfig *hwconfig)
    {
    struct i915_vma *vma;
    u32 ggtt_offset;
    void *vaddr;
    int ret;
    GEM_BUG_ON(!hwconfig.size);
    ret = intel_guc_allocate_and_map_vma(guc, hwconfig.size, &vma, &vaddr);
    if (ret)
    return ret;
    ggtt_offset = intel_guc_ggtt_offset(guc, vma);
    ret = __guc_action_get_hwconfig(guc, ggtt_offset, hwconfig.size);
    if (ret >= 0)
    memcpy(hwconfig.ptr, vaddr, hwconfig.size);
    i915_vma_unpin_and_release(&vma, I915_VMA_RELEASE_MAP);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn has_table(i915: *mut drm_i915_private) -> bool {
    static bool has_table(struct drm_i915_private *i915)
    {
    if (IS_ALDERLAKE_P(i915) && !IS_ALDERLAKE_P_N(i915))
    return true;
    if (GRAPHICS_VER_FULL(i915) >= IP_VER(12, 55))
    return true;
    return false;
    }
//
// intel_guc_hwconfig_init - Initialize the HWConfig
//
// Retrieve the HWConfig table from the GuC and save it locally.
// It can then be queried on demand by other users later on.
//
#[no_mangle]
unsafe extern "C" fn guc_hwconfig_init(gt: *mut intel_gt) -> c_int {
    static int guc_hwconfig_init(struct intel_gt *gt)
    {
    struct intel_hwconfig *hwconfig = &gt.info.hwconfig;
    struct intel_guc *guc = gt_to_guc(gt);
    int ret;
    if (!has_table(gt.i915))
    return 0;
    ret = guc_hwconfig_discover_size(guc, hwconfig);
    if (ret)
    return ret;
    hwconfig.ptr = kmalloc(hwconfig.size, GFP_KERNEL);
    if (!hwconfig.ptr) {
    hwconfig.size = 0;
    return -ENOMEM;
    }
    ret = guc_hwconfig_fill_buffer(guc, hwconfig);
    if (ret < 0) {
    intel_gt_fini_hwconfig(gt);
    return ret;
    }
    return 0;
    }
//
// intel_gt_init_hwconfig - Initialize the HWConfig if available
//
// Retrieve the HWConfig table if available on the current platform.
//
#[no_mangle]
pub unsafe extern "C" fn intel_gt_init_hwconfig(gt: *mut intel_gt) -> c_int {
    int intel_gt_init_hwconfig(struct intel_gt *gt)
    {
    if (!intel_uc_uses_guc(&gt.uc))
    return 0;
    return guc_hwconfig_init(gt);
    }
//
// intel_gt_fini_hwconfig - Finalize the HWConfig
//
// Free up the memory allocation holding the table.
//
#[no_mangle]
pub unsafe extern "C" fn intel_gt_fini_hwconfig(gt: *mut intel_gt) {
    void intel_gt_fini_hwconfig(struct intel_gt *gt)
    {
    struct intel_hwconfig *hwconfig = &gt.info.hwconfig;
    kfree(hwconfig.ptr);
    hwconfig.size = 0;
    hwconfig.ptr = core::ptr::null_mut();
    }
