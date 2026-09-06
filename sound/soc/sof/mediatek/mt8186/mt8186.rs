//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/mediatek/mt8186/mt8186.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright (c) 2022 MediaTek Corporation. All rights reserved.
//
// Header file for the mt8186 DSP register definition
//
pub const DSP_REG_BAR: c_int = 4;
pub const DSP_SECREG_BAR: c_int = 5;
pub const DSP_BUSREG_BAR: c_int = 6;
//
// R E G I S T E R       TABLE
//
// dsp cfg
pub const ADSP_CFGREG_SW_RSTN: c_uint = 0x0000;

pub const ADSP_HIFI_IO_CONFIG: c_uint = 0x000C;

pub const ADSP_IRQ_MASK: c_uint = 0x0030;
pub const ADSP_DVFSRC_REQ: c_uint = 0x0040;
pub const ADSP_DDREN_REQ_0: c_uint = 0x0044;
pub const ADSP_SEMAPHORE: c_uint = 0x0064;
pub const ADSP_WDT_CON_C0: c_uint = 0x007C;
pub const ADSP_MBOX_IRQ_EN: c_uint = 0x009C;

pub const DSP_PDEBUGPC: c_uint = 0x013C;
pub const DSP_PDEBUGDATA: c_uint = 0x0140;
pub const DSP_PDEBUGINST: c_uint = 0x0144;
pub const DSP_PDEBUGLS0STAT: c_uint = 0x0148;
pub const DSP_PDEBUGSTATUS: c_uint = 0x014C;
pub const DSP_PFAULTINFO: c_uint = 0x0150;
pub const ADSP_CK_EN: c_uint = 0x1000;

pub const ADSP_UART_CTRL: c_uint = 0x1010;

// dsp sec
pub const ADSP_PRID: c_uint = 0x0;
pub const ADSP_ALTVEC_C0: c_uint = 0x04;
pub const ADSP_ALTVECSEL: c_uint = 0x0C;

//
// On MT8188, BIT(1) is not evaluated and on MT8186 BIT(0) is not evaluated:
// We can simplify the driver by safely setting both bits regardless of the SoC.
//

// dsp bus
pub const ADSP_SRAM_POOL_CON: c_uint = 0x190;
pub const DSP_SRAM_POOL_PD_MASK: c_uint = 0xF00F /* [0:3] and [12:15] */;
pub const DSP_C0_EMI_MAP_ADDR: c_uint = 0xA00  /* ADSP Core0 To EMI Address Remap */;
pub const DSP_C0_DMAEMI_MAP_ADDR: c_uint = 0xA08  /* DMA0 To EMI Address Remap */;
// DSP memories
pub const MBOX_OFFSET: c_uint = 0x500000 /* DRAM */;
pub const MBOX_SIZE: c_uint = 0x1000   /* consistent with which in memory.h of sof fw */;
pub const DSP_DRAM_SIZE: c_uint = 0xA00000 /* 16M */;
// remap dram between AP and DSP view, 4KB aligned
pub const SRAM_PHYS_BASE_FROM_DSP_VIEW: c_uint = 0x4E100000 /* MT8186 DSP view */;
pub const DRAM_PHYS_BASE_FROM_DSP_VIEW: c_uint = 0x60000000 /* MT8186 DSP view */;
pub const DRAM_REMAP_SHIFT: c_int = 12;
pub const DRAM_REMAP_MASK: c_uint = 0xFFF;
pub const SIZE_SHARED_DRAM_DL: c_uint = 0x40000 /*Shared buffer for Downlink*/;
pub const SIZE_SHARED_DRAM_UL: c_uint = 0x40000 /*Shared buffer for Uplink*/;

extern "C" {
    pub fn mt8186_sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32);
}
extern "C" {
    pub fn mt8186_sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev);
}
