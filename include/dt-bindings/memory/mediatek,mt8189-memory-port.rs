//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/mediatek,mt8189-memory-port.h
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
// Copyright (c) 2025 MediaTek Inc.
// Author: Zhengnan chen <zhengnan.chen@mediatek.com>
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
// modules		dma-address-region	larbs-ports
// disp/mdp		0 ~ 4G			larb0/1/2
// vcodec		4G ~ 8G                 larb4/7
// imgsys/cam/ipesys	8G ~ 12G                the other larbs.
// N/A			12G ~ 16G
//
// Larb0 -- disp

// Larb1 -- disp

// Larb2 -- mmlsys(mdp)

// Larb3: null
// Larb4 -- vdec

// Larb5: null
// Larb6: null
// Larb7 -- venc

// Larb8: null
// Larb9 --imgsys

// Larb10: null
// Larb11 -- imgsys

// Larb12: null
// Larb13 -- cam

// Larb14 -- cam

// Larb15: null
// Larb16 -- cam

// Larb17 -- cam

// Larb19 -- ipesys

// Larb20 -- ipesys

// fake larb21 for gce

// fake larb & port for svp and dual svp and wfd

// fake larb0 for apu

// infra/peri

