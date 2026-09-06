//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/mt8192-larb-port.h
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
// Copyright (c) 2020 MediaTek Inc.
//
// Author: Chao Hao <chao.hao@mediatek.com>
// Author: Yong Wu <yong.wu@mediatek.com>
//

//
// MM IOMMU supports 16GB dma address.
//
// The address will preassign like this:
//
// modules    dma-address-region	larbs-ports
// disp         0 ~ 4G                   larb0/1
// vcodec      4G ~ 8G                  larb4/5/7
// cam/mdp     8G ~ 12G             larb2/9/11/13/14/16/17/18/19/20
// CCU0    0x4000_0000 ~ 0x43ff_ffff     larb13: port 9/10
// CCU1    0x4400_0000 ~ 0x47ff_ffff     larb14: port 4/5
//
// larb3/6/8/10/12/15 is null.
//
// larb0

// larb1

// larb2

// larb3: null
// larb4

// larb5

// larb6: null
// larb7

// larb8: null
// larb9

// larb10: null
// larb11

// larb12: null
// larb13

// larb14

// larb15: null
// larb16

// larb17

// larb18

// larb19

// larb20

