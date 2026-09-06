//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/mediatek/mtk-mmsys.h
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
// Copyright (c) 2015 MediaTek Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_dpi_out_format_con {
    MTK_DPI_RGB888_SDR_CON,
    MTK_DPI_RGB888_DDR_CON,
    MTK_DPI_RGB565_SDR_CON,
    MTK_DPI_RGB565_DDR_CON
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_ddp_comp_id {
    DDP_COMPONENT_AAL0,
    DDP_COMPONENT_AAL1,
    DDP_COMPONENT_BLS,
    DDP_COMPONENT_CCORR,
    DDP_COMPONENT_COLOR0,
    DDP_COMPONENT_COLOR1,
    DDP_COMPONENT_DITHER0,
    DDP_COMPONENT_DITHER1,
    DDP_COMPONENT_DP_INTF0,
    DDP_COMPONENT_DP_INTF1,
    DDP_COMPONENT_DPI0,
    DDP_COMPONENT_DPI1,
    DDP_COMPONENT_DSC0,
    DDP_COMPONENT_DSC1,
    DDP_COMPONENT_DSI0,
    DDP_COMPONENT_DSI1,
    DDP_COMPONENT_DSI2,
    DDP_COMPONENT_DSI3,
    DDP_COMPONENT_ETHDR_MIXER,
    DDP_COMPONENT_GAMMA,
    DDP_COMPONENT_MDP_RDMA0,
    DDP_COMPONENT_MDP_RDMA1,
    DDP_COMPONENT_MDP_RDMA2,
    DDP_COMPONENT_MDP_RDMA3,
    DDP_COMPONENT_MDP_RDMA4,
    DDP_COMPONENT_MDP_RDMA5,
    DDP_COMPONENT_MDP_RDMA6,
    DDP_COMPONENT_MDP_RDMA7,
    DDP_COMPONENT_MERGE0,
    DDP_COMPONENT_MERGE1,
    DDP_COMPONENT_MERGE2,
    DDP_COMPONENT_MERGE3,
    DDP_COMPONENT_MERGE4,
    DDP_COMPONENT_MERGE5,
    DDP_COMPONENT_OD0,
    DDP_COMPONENT_OD1,
    DDP_COMPONENT_OVL0,
    DDP_COMPONENT_OVL_2L0,
    DDP_COMPONENT_OVL_2L1,
    DDP_COMPONENT_OVL_2L2,
    DDP_COMPONENT_OVL1,
    DDP_COMPONENT_PADDING0,
    DDP_COMPONENT_PADDING1,
    DDP_COMPONENT_PADDING2,
    DDP_COMPONENT_PADDING3,
    DDP_COMPONENT_PADDING4,
    DDP_COMPONENT_PADDING5,
    DDP_COMPONENT_PADDING6,
    DDP_COMPONENT_PADDING7,
    DDP_COMPONENT_POSTMASK0,
    DDP_COMPONENT_PWM0,
    DDP_COMPONENT_PWM1,
    DDP_COMPONENT_PWM2,
    DDP_COMPONENT_RDMA0,
    DDP_COMPONENT_RDMA1,
    DDP_COMPONENT_RDMA2,
    DDP_COMPONENT_RDMA4,
    DDP_COMPONENT_UFOE,
    DDP_COMPONENT_WDMA0,
    DDP_COMPONENT_WDMA1,
    DDP_COMPONENT_ID_MAX,
}

extern "C" {
    pub fn mtk_mmsys_ddp_dpi_fmt_config(dev: *mut device, val: u32);
}
