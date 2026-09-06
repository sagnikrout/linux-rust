//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mdp_reg_rdma.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//
pub const MDP_RDMA_EN: c_uint = 0x000;
pub const MDP_RDMA_RESET: c_uint = 0x008;
pub const MDP_RDMA_CON: c_uint = 0x020;
pub const MDP_RDMA_GMCIF_CON: c_uint = 0x028;
pub const MDP_RDMA_SRC_CON: c_uint = 0x030;
pub const MDP_RDMA_MF_BKGD_SIZE_IN_BYTE: c_uint = 0x060;
pub const MDP_RDMA_MF_BKGD_SIZE_IN_PXL: c_uint = 0x068;
pub const MDP_RDMA_MF_SRC_SIZE: c_uint = 0x070;
pub const MDP_RDMA_MF_CLIP_SIZE: c_uint = 0x078;
pub const MDP_RDMA_MF_OFFSET_1: c_uint = 0x080;
pub const MDP_RDMA_SF_BKGD_SIZE_IN_BYTE: c_uint = 0x090;
pub const MDP_RDMA_SRC_END_0: c_uint = 0x100;
pub const MDP_RDMA_SRC_END_1: c_uint = 0x108;
pub const MDP_RDMA_SRC_END_2: c_uint = 0x110;
pub const MDP_RDMA_SRC_OFFSET_0: c_uint = 0x118;
pub const MDP_RDMA_SRC_OFFSET_1: c_uint = 0x120;
pub const MDP_RDMA_SRC_OFFSET_2: c_uint = 0x128;
pub const MDP_RDMA_SRC_OFFSET_0_P: c_uint = 0x148;
pub const MDP_RDMA_TRANSFORM_0: c_uint = 0x200;
pub const MDP_RDMA_DMABUF_CON_0: c_uint = 0x240;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_0: c_uint = 0x248;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_0: c_uint = 0x250;
pub const MDP_RDMA_DMABUF_CON_1: c_uint = 0x258;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_1: c_uint = 0x260;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_1: c_uint = 0x268;
pub const MDP_RDMA_DMABUF_CON_2: c_uint = 0x270;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_2: c_uint = 0x278;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_2: c_uint = 0x280;
pub const MDP_RDMA_DMABUF_CON_3: c_uint = 0x288;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_3: c_uint = 0x290;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_3: c_uint = 0x298;
pub const MDP_RDMA_RESV_DUMMY_0: c_uint = 0x2a0;
pub const MDP_RDMA_MON_STA_1: c_uint = 0x408;
pub const MDP_RDMA_SRC_BASE_0: c_uint = 0xf00;
pub const MDP_RDMA_SRC_BASE_1: c_uint = 0xf08;
pub const MDP_RDMA_SRC_BASE_2: c_uint = 0xf10;
pub const MDP_RDMA_UFO_DEC_LENGTH_BASE_Y: c_uint = 0xf20;
pub const MDP_RDMA_UFO_DEC_LENGTH_BASE_C: c_uint = 0xf28;
// MASK
pub const MDP_RDMA_EN_MASK: c_uint = 0x00000001;
pub const MDP_RDMA_RESET_MASK: c_uint = 0x00000001;
pub const MDP_RDMA_CON_MASK: c_uint = 0x00001110;
pub const MDP_RDMA_GMCIF_CON_MASK: c_uint = 0xfffb3771;
pub const MDP_RDMA_SRC_CON_MASK: c_uint = 0xf3ffffff;
pub const MDP_RDMA_MF_BKGD_SIZE_IN_BYTE_MASK: c_uint = 0x001fffff;
pub const MDP_RDMA_MF_BKGD_SIZE_IN_PXL_MASK: c_uint = 0x001fffff;
pub const MDP_RDMA_MF_SRC_SIZE_MASK: c_uint = 0x1fff1fff;
pub const MDP_RDMA_MF_CLIP_SIZE_MASK: c_uint = 0x1fff1fff;
pub const MDP_RDMA_MF_OFFSET_1_MASK: c_uint = 0x003f001f;
pub const MDP_RDMA_SF_BKGD_SIZE_IN_BYTE_MASK: c_uint = 0x001fffff;
pub const MDP_RDMA_SRC_END_0_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_END_1_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_END_2_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_OFFSET_0_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_OFFSET_1_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_OFFSET_2_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_OFFSET_0_P_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_TRANSFORM_0_MASK: c_uint = 0xff110777;
pub const MDP_RDMA_DMABUF_CON_0_MASK: c_uint = 0x0fff00ff;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_0_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_0_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_DMABUF_CON_1_MASK: c_uint = 0x0f7f007f;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_1_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_1_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_DMABUF_CON_2_MASK: c_uint = 0x0f3f003f;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_2_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_2_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_DMABUF_CON_3_MASK: c_uint = 0x0f3f003f;
pub const MDP_RDMA_ULTRA_TH_HIGH_CON_3_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_ULTRA_TH_LOW_CON_3_MASK: c_uint = 0x3fffffff;
pub const MDP_RDMA_RESV_DUMMY_0_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_MON_STA_1_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_BASE_0_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_BASE_1_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_SRC_BASE_2_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_UFO_DEC_LENGTH_BASE_Y_MASK: c_uint = 0xffffffff;
pub const MDP_RDMA_UFO_DEC_LENGTH_BASE_C_MASK: c_uint = 0xffffffff;
