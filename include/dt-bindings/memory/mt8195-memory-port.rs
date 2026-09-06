//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/mt8195-memory-port.h
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
// disp         0 ~ 4G                  larb0/1/2/3
// vcodec      4G ~ 8G                  larb19/20/21/22/23/24
// cam/mdp     8G ~ 12G                 the other larbs.
// N/A         12G ~ 16G
// CCU0   0x24000_0000 ~ 0x243ff_ffff   larb18: port 0/1
// CCU1   0x24400_0000 ~ 0x247ff_ffff   larb18: port 2/3
//
// This SoC have two IOMMU HWs, this is the detailed connected information:
// iommu-vdo: larb0/2/5/7/9/10/11/13/17/19/21/24/25/28
// iommu-vpp: larb1/3/4/6/8/12/14/16/18/20/22/23/26/27
//
// MM IOMMU ports
// larb0

// larb1

// larb2

// larb3

// larb4

// larb5

// larb6

// larb7

// larb8

// larb9

// larb10

// larb11

// larb12

// larb13

// larb14

// larb15: null
// larb16

// larb17

// larb18

// larb19

// larb20

// larb21

// larb22

// larb23

// larb24

// larb25

// larb26

// larb27

// larb28

// Infra iommu ports
// PCIe1: read: BIT16; write BIT17.

// PCIe0: read: BIT18; write BIT19.

