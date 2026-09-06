//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_bo_types.h
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

pub const XE_BO_MAX_PLACEMENTS: c_int = 3;
// TODO: To be selected with VM_MADVISE
pub const XE_BO_PRIORITY_NORMAL: c_int = 1;
//
// struct xe_bo - Xe buffer object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_bo {
// @ttm: TTM base buffer object
    pub ttm: ttm_buffer_object,
// @backup_obj: The backup object when pinned and suspended (vram only)
    pub backup_obj: *mut xe_bo,
// @parent_obj: Ref to parent bo if this a backup_obj
    pub parent_obj: *mut xe_bo,
// @dma_buf: Imported dma-buf ref to keep its resv alive.
    pub dma_buf: *mut dma_buf,
// @flags: flags for this buffer object
    pub flags: u32,
// @vm: VM this BO is attached to, for extobj this will be NULL
    pub vm: *mut xe_vm,
// @tile: Tile this BO is attached to (kernel BO only)
    pub tile: *mut xe_tile,
// @placements: valid placements for this BO
    pub placements: [ttm_place; XE_BO_MAX_PLACEMENTS],
// @placement: current placement for this BO
    pub placement: ttm_placement,
// @ggtt_node: Array of GGTT nodes if this BO is mapped in the GGTTs
    pub ggtt_node: [*mut xe_ggtt_node; XE_MAX_TILES_PER_DEVICE],
// @vmap: iosys map of this buffer
    pub vmap: iosys_map,
// @kmap: TTM bo kmap object for internal use only. Keep off.
    pub kmap: ttm_bo_kmap_obj,
// @pinned_link: link to present / evicted list of pinned BO
    pub pinned_link: list_head,

//
// @client: @xe_drm_client which created the bo
//
    pub client: *mut xe_drm_client,
//
// @client_link: Link into @xe_drm_client.objects_list
//
    pub client_link: list_head,

// @attr: User controlled attributes for bo
//
// @attr.atomic_access: type of atomic access bo needs
// protected by bo dma-resv lock
//
    pub atomic_access: u32,
    pub attr: },
//
// @pxp_key_instance: PXP key instance this BO was created against. A
// 0 in this variable indicates that the BO does not use PXP encryption.
//
    pub pxp_key_instance: u32,
// @freed: List node for delayed put.
    pub freed: llist_node,
// @update_index: Update index if PT BO
    pub update_index: c_int,
// @created: Whether the bo has passed initial creation
    pub created: bool,
// @ccs_cleared: true means that CCS region of BO is already cleared
    pub ccs_cleared: bool,
// @bb_ccs: BB instructions of CCS read/write. Valid only for VF
    pub bb_ccs: [*mut xe_mem_pool_node; XE_SRIOV_VF_CCS_CTX_COUNT],
//
// @cpu_caching: CPU caching mode. Currently only used for userspace
// objects. Exceptions are system memory on DGFX, which is always
// WB.
//
    pub cpu_caching: u16,
// @devmem_allocation: SVM device memory allocation
    pub devmem_allocation: drm_pagemap_devmem,
// @vram_userfault_link: Link into @mem_access.vram_userfault.list
    pub vram_userfault_link: list_head,
//
// @min_align: minimum alignment needed for this BO if different
// from default
//
    pub min_align: u64,
//
// @purgeable: Purgeability state and accounting.
//
// All fields are protected by the BO's dma-resv lock.
//
// @purgeable.state: BO purgeability state
// (WILLNEED/DONTNEED/PURGED).
//
    pub state: u32,
//
// @purgeable.vma_count: Number of VMAs currently mapping this BO.
//
    pub vma_count: u32,
//
// @purgeable.willneed_count: Number of active WILLNEED holders.
//
// Counts WILLNEED VMAs plus active dma-buf exports for
// non-imported BOs. The BO flips to DONTNEED on a 1->0
// transition only when VMAs still exist; if the last VMA is
// removed, the previous BO state is preserved.
//
    pub willneed_count: u32,
    pub purgeable: },
}
