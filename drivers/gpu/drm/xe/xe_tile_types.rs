//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_tile_types.h
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
// Copyright © 2022-2026 Intel Corporation
//

//
// struct xe_tile - hardware tile structure
//
// From a driver perspective, a "tile" is effectively a complete GPU, containing
// an SGunit, 1-2 GTs, and (for discrete platforms) VRAM.
//
// Multi-tile platforms effectively bundle multiple GPUs behind a single PCI
// device and designate one "root" tile as being responsible for external PCI
// communication.  PCI BAR0 exposes the GGTT and MMIO register space for each
// tile in a stacked layout, and PCI BAR2 exposes the local memory associated
// with each tile similarly.  Device-wide interrupts can be enabled/disabled
// at the root tile, and the MSTR_TILE_INTR register will report which tiles
// have interrupts that need servicing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_tile {
// @xe: Backpointer to tile's PCI device
    pub xe: *mut xe_device,
// @id: ID of the tile
    pub id: u8,
//
// @primary_gt: Primary GT
//
    pub primary_gt: *mut xe_gt,
//
// @media_gt: Media GT
//
// Only present on devices with media version >= 13.
//
    pub media_gt: *mut xe_gt,
//
// @mmio: MMIO info for a tile.
//
// Each tile has its own 16MB space in BAR0, laid out as:
// * 0-4MB: registers
// * 4MB-8MB: reserved
// * 8MB-16MB: global GTT
//
    pub mmio: xe_mmio,
// @mem: memory management info for tile
//
// @mem.kernel_vram: kernel-dedicated VRAM info for tile.
//
// Although VRAM is associated with a specific tile, it can
// still be accessed by all tiles' GTs.
//
    pub kernel_vram: *mut xe_vram_region,
//
// @mem.vram: general purpose VRAM info for tile.
//
// Although VRAM is associated with a specific tile, it can
// still be accessed by all tiles' GTs.
//
    pub vram: *mut xe_vram_region,
// @mem.ggtt: Global graphics translation table
    pub ggtt: *mut xe_ggtt,
//
// @mem.kernel_bb_pool: Pool from which batchbuffers are allocated.
//
// Media GT shares a pool with its primary GT.
//
    pub kernel_bb_pool: *mut xe_sa_manager,
//
// @mem.reclaim_pool: Pool for PRLs allocated.
//
// Only main GT has page reclaim list allocations.
//
    pub reclaim_pool: *mut xe_sa_manager,
    pub mem: },
// @sriov: tile level virtualization data
// @sriov.pf.lmtt: Local Memory Translation Table.
    pub lmtt: xe_lmtt,
    pub pf: },
// @sriov.vf.self_config: VF configuration data
    pub self_config: xe_tile_sriov_vf_selfconfig,
    pub vf: },
    pub sriov: },
// @memirq: Memory Based Interrupts.
    pub memirq: xe_memirq,
// @csc_hw_error_work: worker to report CSC HW errors
    pub csc_hw_error_work: work_struct,
// @pcode: tile's PCODE
// @pcode.lock: protecting tile's PCODE mailbox data
    pub lock: mutex,
    pub pcode: },
// @migrate: Migration helper for vram blits and clearing
    pub migrate: *mut xe_migrate,
// @sysfs: sysfs' kobj used by xe_tile_sysfs
    pub sysfs: *mut kobject,
// @debugfs: debugfs directory associated with this tile
    pub debugfs: *mut dentry,
// @mert: MERT-related data
    pub mert: xe_mert,
}
