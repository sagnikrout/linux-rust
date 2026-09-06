//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pci_types.h
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
// Copyright © 2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_subplatform_desc {
    pub subplatform: xe_subplatform,
    pub name: *const c_char,
    pub pciidlist: *const u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_device_desc {
// Should only ever be set for platforms without GMD_ID
    pub pre_gmdid_graphics_ip: *const xe_ip,
// Should only ever be set for platforms without GMD_ID
    pub pre_gmdid_media_ip: *const xe_ip,
    pub platform_name: *const c_char,
    pub subplatforms: *const xe_subplatform_desc,
    pub platform: xe_platform,
    pub dma_mask_size: u8,
    pub max_remote_tiles:2: u8,
    pub max_gt_per_tile:2: u8,
    pub multi_lrc_mask: u8,
    pub va_bits: u8,
    pub vm_max_level: u8,
    pub vram_flags: u8,
    pub require_force_probe:1: u8,
    pub is_dgfx:1: u8,
    pub has_cached_pt:1: u8,
    pub has_display:1: u8,
    pub has_drm_ras:1: u8,
    pub has_fan_control:1: u8,
    pub has_flat_ccs:1: u8,
    pub has_gsc_nvm:1: u8,
    pub has_heci_gscfi:1: u8,
    pub has_heci_cscfi:1: u8,
    pub has_i2c:1: u8,
    pub has_late_bind:1: u8,
    pub has_llc:1: u8,
    pub has_mbx_power_limits:1: u8,
    pub has_mbx_thermal_info:1: u8,
    pub has_mert:1: u8,
    pub has_pre_prod_wa:1: u8,
    pub has_page_reclaim_hw_assist:1: u8,
    pub has_pxp:1: u8,
    pub has_soc_remapper_sysctrl:1: u8,
    pub has_soc_remapper_telem:1: u8,
    pub has_sriov:1: u8,
    pub has_sysctrl:1: u8,
    pub needs_scratch:1: u8,
    pub skip_guc_pc:1: u8,
    pub skip_pcode:1: u8,
    pub needs_shared_vf_gt_wq:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_graphics_desc {
    pub /: *mut *mut u64 hw_engine_mask; / hardware engines provided by graphics IP,
    pub /: *mut *mut u16 multi_queue_engine_class_mask; / bitmask of engine classes which support multi queue,
    pub num_geometry_xecore_fuse_regs: u8,
    pub num_compute_xecore_fuse_regs: u8,
    pub has_access_counter:1: u8,
    pub has_asid:1: u8,
    pub has_atomic_enable_pte_bit:1: u8,
    pub has_indirect_ring_state:1: u8,
    pub has_range_tlb_inval:1: u8,
    pub has_ctx_tlb_inval:1: u8,
    pub has_usm:1: u8,
    pub has_64bit_timestamp:1: u8,
    pub has_uncorrectable_error_reporting:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_media_desc {
    pub /: *mut *mut u64 hw_engine_mask; / hardware engines provided by media IP,
    pub has_indirect_ring_state:1: u8,
    pub has_uncorrectable_error_reporting:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ip {
    pub verx100: c_uint,
    pub name: *const c_char,
    pub desc: *const c_void,
}
