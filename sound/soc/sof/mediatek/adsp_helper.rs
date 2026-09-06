//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/mediatek/adsp_helper.h
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
// Copyright (c) 2021 MediaTek Corporation. All rights reserved.
//

//
// Global important adsp data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_adsp_chip_info {
    pub pa_sram: phys_addr_t,
    pub /: *mut *mut phys_addr_t pa_dram; / adsp dram physical base,
    pub pa_cfgreg: phys_addr_t,
    pub sramsize: u32,
    pub dramsize: u32,
    pub cfgregsize: u32,
    pub /: *mut *mut *mut void __iomem va_sram; / corresponding to pa_sram,
    pub /: *mut *mut *mut void __iomem va_dram; / corresponding to pa_dram,
    pub va_cfgreg: *mut void __iomem,
    pub adsp_bootup_addr: phys_addr_t,
    pub view*/: *mut *mut int dram_offset; /dram offset between system and dsp,
    pub pa_secreg: phys_addr_t,
    pub secregsize: u32,
    pub va_secreg: *mut void __iomem,
    pub pa_busreg: phys_addr_t,
    pub busregsize: u32,
    pub va_busreg: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adsp_priv {
    pub dev: *mut device,
    pub sdev: *mut snd_sof_dev,
    pub dsp_ipc: *mut mtk_adsp_ipc,
    pub ipc_dev: *mut platform_device,
    pub adsp: *mut mtk_adsp_chip_info,
    pub clk: *mut clk,
    pub data): *mut *mut u32 (ap2adsp_addr)(u32 addr, void,
    pub data): *mut *mut u32 (adsp2ap_addr)(u32 addr, void,
    pub private_data: *mut c_void,
}
