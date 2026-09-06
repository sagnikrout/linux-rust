//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/omap_dmm_priv.h
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
// Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com
// Author: Rob Clark <rob@ti.com>
// Andy Gross <andy.gross@ti.com>
//
pub const DMM_REVISION: c_uint = 0x000;
pub const DMM_HWINFO: c_uint = 0x004;
pub const DMM_LISA_HWINFO: c_uint = 0x008;
pub const DMM_DMM_SYSCONFIG: c_uint = 0x010;
pub const DMM_LISA_LOCK: c_uint = 0x01C;
pub const DMM_LISA_MAP__0: c_uint = 0x040;
pub const DMM_LISA_MAP__1: c_uint = 0x044;
pub const DMM_TILER_HWINFO: c_uint = 0x208;
pub const DMM_TILER_OR__0: c_uint = 0x220;
pub const DMM_TILER_OR__1: c_uint = 0x224;
pub const DMM_PAT_HWINFO: c_uint = 0x408;
pub const DMM_PAT_GEOMETRY: c_uint = 0x40C;
pub const DMM_PAT_CONFIG: c_uint = 0x410;
pub const DMM_PAT_VIEW__0: c_uint = 0x420;
pub const DMM_PAT_VIEW__1: c_uint = 0x424;
pub const DMM_PAT_VIEW_MAP__0: c_uint = 0x440;
pub const DMM_PAT_VIEW_MAP_BASE: c_uint = 0x460;
pub const DMM_PAT_IRQ_EOI: c_uint = 0x478;
pub const DMM_PAT_IRQSTATUS_RAW: c_uint = 0x480;
pub const DMM_PAT_IRQSTATUS: c_uint = 0x490;
pub const DMM_PAT_IRQENABLE_SET: c_uint = 0x4A0;
pub const DMM_PAT_IRQENABLE_CLR: c_uint = 0x4B0;
pub const DMM_PAT_STATUS__0: c_uint = 0x4C0;
pub const DMM_PAT_STATUS__1: c_uint = 0x4C4;
pub const DMM_PAT_STATUS__2: c_uint = 0x4C8;
pub const DMM_PAT_STATUS__3: c_uint = 0x4CC;
pub const DMM_PAT_DESCR__0: c_uint = 0x500;
pub const DMM_PAT_DESCR__1: c_uint = 0x510;
pub const DMM_PAT_DESCR__2: c_uint = 0x520;
pub const DMM_PAT_DESCR__3: c_uint = 0x530;
pub const DMM_PEG_HWINFO: c_uint = 0x608;
pub const DMM_PEG_PRIO: c_uint = 0x620;
pub const DMM_PEG_PRIO_PAT: c_uint = 0x640;

// note: don't treat DMM_PATSTATUS_ERR_ACCESS as an error

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pat_ctrl {
    pub start:4: u32,
    pub dir:4: u32,
    pub lut_id:8: u32,
    pub sync:12: u32,
    pub ini:4: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pat {
    pub next_pa: u32,
    pub area: pat_area,
    pub ctrl: pat_ctrl,
    pub data_pa: u32,
}

pub const DMM_FIXED_RETRY_COUNT: c_int = 1000;
// create refill buffer big enough to refill all slots, plus 3 descriptors..
// 3 descriptors is probably the worst-case for # of 2d-slices in a 1d area,
// but I guess you don't hit that worst case at the same time as full area
// refill
//
pub const DESCR_SIZE: c_int = 128;

// For OMAP5, a fixed offset is added to all Y coordinates for 1D buffers.
// This is used in programming to address the upper portion of the LUT
//
pub const OMAP5_LUT_OFFSET: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmm_txn {
    pub engine_handle: *mut c_void,
    pub tcm: *mut tcm,
    pub current_va: *mut u8,
    pub current_pa: dma_addr_t,
    pub last_pat: *mut pat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refill_engine {
    pub id: c_int,
    pub dmm: *mut dmm,
    pub tcm: *mut tcm,
    pub refill_va: *mut u8,
    pub refill_pa: dma_addr_t,
// only one trans per engine for now
    pub txn: dmm_txn,
    pub async: bool,
    pub compl: completion,
    pub idle_node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmm_platform_data {
    pub cpu_cache_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmm {
    pub dev: *mut device,
    pub phys_base: dma_addr_t,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub dummy_page: *mut page,
    pub dummy_pa: dma_addr_t,
    pub refill_va: *mut c_void,
    pub refill_pa: dma_addr_t,
// refill engines
    pub engine_queue: wait_queue_head_t,
    pub idle_head: list_head,
    pub engines: *mut refill_engine,
    pub num_engines: c_int,
    pub engine_counter: core::sync::atomic::AtomicI32,
// container information
    pub container_width: c_int,
    pub container_height: c_int,
    pub lut_width: c_int,
    pub lut_height: c_int,
    pub num_lut: c_int,
// array of LUT - TCM containers
    pub tcm: *mut tcm,
// allocation list and lock
    pub alloc_head: list_head,
    pub plat_data: *const dmm_platform_data,
    pub dmm_workaround: bool,
    pub wa_lock: spinlock_t,
    pub wa_dma_data: *mut u32,
    pub wa_dma_handle: dma_addr_t,
    pub wa_dma_chan: *mut dma_chan,
}
