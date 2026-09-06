//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/mediatek,mt8188-memory-port.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chengci Xu <chengci.xu@mediatek.com>
//

//
// MM IOMMU larbs:
// From below, for example larb11 has larb11a/larb11b/larb11c,
// the index of larb is not in order. So we reindexed these larbs from a
// software view.
//
pub const SMI_L0_ID: c_int = 0;
pub const SMI_L1_ID: c_int = 1;
pub const SMI_L2_ID: c_int = 2;
pub const SMI_L3_ID: c_int = 3;
pub const SMI_L4_ID: c_int = 4;
pub const SMI_L5_ID: c_int = 5;
pub const SMI_L6_ID: c_int = 6;
pub const SMI_L7_ID: c_int = 7;
pub const SMI_L9_ID: c_int = 8;
pub const SMI_L10_ID: c_int = 9;
pub const SMI_L11A_ID: c_int = 10;
pub const SMI_L11B_ID: c_int = 11;
pub const SMI_L11C_ID: c_int = 12;
pub const SMI_L12_ID: c_int = 13;
pub const SMI_L13_ID: c_int = 14;
pub const SMI_L14_ID: c_int = 15;
pub const SMI_L15_ID: c_int = 16;
pub const SMI_L16A_ID: c_int = 17;
pub const SMI_L16B_ID: c_int = 18;
pub const SMI_L17A_ID: c_int = 19;
pub const SMI_L17B_ID: c_int = 20;
pub const SMI_L19_ID: c_int = 21;
pub const SMI_L21_ID: c_int = 22;
pub const SMI_L23_ID: c_int = 23;
pub const SMI_L27_ID: c_int = 24;
pub const SMI_L28_ID: c_int = 25;
//
// MM IOMMU supports 16GB dma address. We separate it to four ranges:
// 0 ~ 4G; 4G ~ 8G; 8G ~ 12G; 12G ~ 16G, we could adjust these masters
// locate in anyone region. BUT:
// a) Make sure all the ports inside a larb are in one range.
// b) The iova of any master can NOT cross the 4G/8G/12G boundary.
//
// This is the suggested mapping in this SoC:
//
// modules    dma-address-region	larbs-ports
// disp         0 ~ 4G                  larb0/1/2/3
// vcodec      4G ~ 8G                  larb19(21)[1]/21(22)/23
// cam/mdp     8G ~ 12G                 the other larbs.
// N/A         12G ~ 16G
// CCU0   0x24000_0000 ~ 0x243ff_ffff   larb27(24): port 0/1
// CCU1   0x24400_0000 ~ 0x247ff_ffff   larb27(24): port 2/3
//
// This SoC have two MM IOMMU HWs, this is the connected information:
// iommu-vdo: larb0/2/5/9/10/11A/11C/13/16B/17B/19/21
// iommu-vpp: larb1/3/4/6/7/11B/12/14/15/16A/17A/23/27
//
// [1]: This is larb19, but the index is 21 from the SW view.
//
// MM IOMMU ports
// LARB 0 -- VDO-0

// LARB 1 -- VD0-0

// LARB 2 -- VDO-1

// LARB 3 -- VDO-1

// LARB 4 -- VPP-0

// LARB 5 -- VPP-1

// LARB 6 -- VPP-1

// LARB 7 -- WPE

// LARB 9 -- IMG-M

// LARB 10 -- IMG-D

// LARB 11A -- IMG-D

// LARB 11B -- IMG-D

// LARB 11C -- IMG-D

// LARB 12 -- IPE

// LARB 13 -- CAM-1

// LARB 14 -- CAM-1

// LARB 15 -- IMG-D

// LARB 16A -- CAM

// LARB 16B -- CAM

// LARB 17A -- CAM

// LARB 17B -- CAM

// LARB 19 -- VENC

// LARB 21 -- VDEC-CORE0

// LARB 23 -- VDEC-SOC

// LARB 27 -- CCU

// LARB 28 -- AXI-CCU

// infra/peri

