//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bus/omap_l3_smx.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// OMAP3XXX L3 Interconnect Driver header
//
// Copyright (C) 2011 Texas Corporation
// Felipe Balbi <balbi@ti.com>
// Santosh Shilimkar <santosh.shilimkar@ti.com>
// sricharan <r.sricharan@ti.com>
//
// Register definitions. All 64-bit wide
pub const L3_COMPONENT: c_uint = 0x000;
pub const L3_CORE: c_uint = 0x018;
pub const L3_AGENT_CONTROL: c_uint = 0x020;
pub const L3_AGENT_STATUS: c_uint = 0x028;
pub const L3_ERROR_LOG: c_uint = 0x058;

pub const L3_ERROR_LOG_ADDR: c_uint = 0x060;
// Register definitions for Sideband Interconnect
pub const L3_SI_CONTROL: c_uint = 0x020;
pub const L3_SI_FLAG_STATUS_0: c_uint = 0x510;

pub const L3_SI_FLAG_STATUS_1: c_uint = 0x530;

pub const L3_PM_ERROR_LOG: c_uint = 0x020;
pub const L3_PM_CONTROL: c_uint = 0x028;
pub const L3_PM_ERROR_CLEAR_SINGLE: c_uint = 0x030;
pub const L3_PM_ERROR_CLEAR_MULTI: c_uint = 0x038;

// L3 error log bit fields. Common for IA and TA
pub const L3_ERROR_LOG_CODE: c_int = 24;
pub const L3_ERROR_LOG_INITID: c_int = 8;
pub const L3_ERROR_LOG_CMD: c_int = 0;
// L3 agent status bit fields.
pub const L3_AGENT_STATUS_CLEAR_IA: c_uint = 0x10000000;
pub const L3_AGENT_STATUS_CLEAR_TA: c_uint = 0x01000000;
pub const OMAP34xx_IRQ_L3_APP: c_int = 10;
pub const L3_APPLICATION_ERROR: c_uint = 0x0;
pub const L3_DEBUG_ERROR: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap3_l3_initiator_id {
// LCD has 1 ID
    OMAP_L3_LCD = 29,
// SAD2D has 1 ID
    OMAP_L3_SAD2D = 28,
// MPU has 5 IDs
    OMAP_L3_IA_MPU_SS_1 = 27,
    OMAP_L3_IA_MPU_SS_2 = 26,
    OMAP_L3_IA_MPU_SS_3 = 25,
    OMAP_L3_IA_MPU_SS_4 = 24,
    OMAP_L3_IA_MPU_SS_5 = 23,
// IVA2.2 SS has 3 IDs
    OMAP_L3_IA_IVA_SS_1 = 22,
    OMAP_L3_IA_IVA_SS_2 = 21,
    OMAP_L3_IA_IVA_SS_3 = 20,
// IVA 2.2 SS DMA has 6 IDS
    OMAP_L3_IA_IVA_SS_DMA_1 = 19,
    OMAP_L3_IA_IVA_SS_DMA_2 = 18,
    OMAP_L3_IA_IVA_SS_DMA_3 = 17,
    OMAP_L3_IA_IVA_SS_DMA_4 = 16,
    OMAP_L3_IA_IVA_SS_DMA_5 = 15,
    OMAP_L3_IA_IVA_SS_DMA_6 = 14,
// SGX has 1 ID
    OMAP_L3_IA_SGX = 13,
// CAM has 3 ID
    OMAP_L3_IA_CAM_1 = 12,
    OMAP_L3_IA_CAM_2 = 11,
    OMAP_L3_IA_CAM_3 = 10,
// DAP has 1 ID
    OMAP_L3_IA_DAP = 9,
// SDMA WR has 2 IDs
    OMAP_L3_SDMA_WR_1 = 8,
    OMAP_L3_SDMA_WR_2 = 7,
// SDMA RD has 4 IDs
    OMAP_L3_SDMA_RD_1 = 6,
    OMAP_L3_SDMA_RD_2 = 5,
    OMAP_L3_SDMA_RD_3 = 4,
    OMAP_L3_SDMA_RD_4 = 3,
// HSUSB OTG has 1 ID
    OMAP_L3_USBOTG = 2,
// HSUSB HOST has 1 ID
    OMAP_L3_USBHOST = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap3_l3_code {
    OMAP_L3_CODE_NOERROR = 0,
    OMAP_L3_CODE_UNSUP_CMD = 1,
    OMAP_L3_CODE_ADDR_HOLE = 2,
    OMAP_L3_CODE_PROTECT_VIOLATION = 3,
    OMAP_L3_CODE_IN_BAND_ERR = 4,
// codes 5 and 6 are reserved
    OMAP_L3_CODE_REQ_TOUT_NOT_ACCEPT = 7,
    OMAP_L3_CODE_REQ_TOUT_NO_RESP = 8,
// codes 9 - 15 are also reserved
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3_l3 {
    pub dev: *mut device,
    pub ick: *mut clk,
// memory base
    pub rt: *mut void __iomem,
    pub debug_irq: c_int,
    pub app_irq: c_int,
// true when and inband functional error occurs
    pub inband:1: unsigned,
}

// offsets for l3 agents in order with the Flag status register
// MPU IA
// RESERVED
// IVA 2.2 IA
// SGX IA
// RESERVED
// CAMERA IA
// DISPLAY IA
// RESERVED
// SDMA RD IA
// RESERVED
// SDMA WR IA
// RESERVED
// USB OTG IA
// USB HOST IA
// RESERVED
// SAD2D IA
// RESERVED
// SMA TA
// GPMC TA
// OCM RAM TA
// OCM ROM TA
// L4 CORE TA
// L4 PER TA
// IVA 2.2 TA
// SGX TA
// L4 EMU TA
// GPMC TA
// L4 CORE TA
// L4 PER TA
// L4 EMU TA
// MAD2D TA
// RESERVED
// MPU DATA IA
// RESERVED
// DAP IA
// RESERVED
// IVA 2.2 IA
// REST RESERVED
//
// REVISIT define __raw_readll/__raw_writell here, but move them to
// <asm/io.h> at some point
//

// (volatile u64  *)(a) = (v))

// (volatile u64  *)(a))
