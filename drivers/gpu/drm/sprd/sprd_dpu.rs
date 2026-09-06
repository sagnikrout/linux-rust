//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sprd/sprd_dpu.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2020 Unisoc Inc.
//

// DPU Layer registers offset
pub const DPU_LAY_REG_OFFSET: c_uint = 0x30;
//
// Sprd DPU context structure
//
// @base: DPU controller base address
// @irq: IRQ number to install the handler for
// @if_type: The type of DPI interface, default is DPI mode.
// @vm: videomode structure to use for DPU and DPI initialization
// @stopped: indicates whether DPU are stopped
// @wait_queue: wait queue, used to wait for DPU shadow register update done and
// DPU stop register done interrupt signal.
// @evt_update: wait queue condition for DPU shadow register
// @evt_stop: wait queue condition for DPU stop register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_context {
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub if_type: u8,
    pub vm: videomode,
    pub stopped: bool,
    pub wait_queue: wait_queue_head_t,
    pub evt_update: bool,
    pub evt_stop: bool,
}

//
// Sprd DPU device structure
//
// @crtc: crtc object
// @drm: A point to drm device
// @ctx: DPU's implementation specific context object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_dpu {
    pub base: drm_crtc,
    pub drm: *mut drm_device,
    pub ctx: dpu_context,
}

extern "C" {
    pub fn container_of(_arg: crtc, sprd_dpu: struct, _arg: base) -> return;
}
extern "C" {
    pub fn readl(layer_offset: ctx->base +) -> return;
}
extern "C" {
    pub fn sprd_dpu_run(dpu: *mut sprd_dpu);
}
extern "C" {
    pub fn sprd_dpu_stop(dpu: *mut sprd_dpu);
}
