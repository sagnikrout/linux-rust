//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/mt8186-memory-port.h
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
//
// Author: Anan Sun <anan.sun@mediatek.com>
// Author: Yong Wu <yong.wu@mediatek.com>
//

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
// disp         0 ~ 4G                  larb0/1/2
// vcodec      4G ~ 8G                  larb4/7
// cam/mdp     8G ~ 12G                 the other larbs.
// N/A         12G ~ 16G
// CCU0   0x24000_0000 ~ 0x243ff_ffff   larb13: port 9/10
// CCU1   0x24400_0000 ~ 0x247ff_ffff   larb14: port 4/5
//
// MM IOMMU ports
// LARB 0 -- MMSYS

// LARB 1 -- MMSYS

// LARB 2 -- MMSYS

// LARB 4 -- VDEC

// LARB 7 -- VENC

// LARB 8 -- WPE

// LARB 9 -- IMG-1

// LARB 11 -- IMG-2

// LARB 13 -- CAM

// LARB 14 -- CAM

// LARB 16 -- RAW-A

// LARB 17 -- RAW-B

// LARB 19 -- IPE

// LARB 20 -- IPE

