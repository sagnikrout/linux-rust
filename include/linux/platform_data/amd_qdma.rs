//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/amd_qdma.h
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
// Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
//

//
// struct qdma_queue_info - DMA queue information. This information is used to
// match queue when DMA channel is requested
// @dir: Channel transfer direction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_queue_info {
    pub dir: dma_transfer_direction,
}

//
// struct qdma_platdata - Platform specific data for QDMA engine
// @max_mm_channels: Maximum number of MM DMA channels in each direction
// @device_map: DMA slave map
// @irq_index: The index of first IRQ
// @dma_dev: The device pointer for dma operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_platdata {
    pub max_mm_channels: u32,
    pub irq_index: u32,
    pub device_map: *mut dma_slave_map,
    pub dma_dev: *mut device,
}
