//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/timb_dma.h
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
// timb_dma.h timberdale FPGA DMA driver defines
// Copyright (c) 2010 Intel Corporation
//
// Supports:
// Timberdale FPGA DMA engine
//
// struct timb_dma_platform_data_channel - Description of each individual
// DMA channel for the timberdale DMA driver
// @rx:			true if this channel handles data in the direction to
// the CPU.
// @bytes_per_line:	Number of bytes per line, this is specific for channels
// handling video data. For other channels this shall be left to 0.
// @descriptors:	Number of descriptors to allocate for this channel.
// @descriptor_elements: Number of elements in each descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timb_dma_platform_data_channel {
    pub rx: bool,
    pub bytes_per_line: c_uint,
    pub descriptors: c_uint,
    pub descriptor_elements: c_uint,
}

//
// struct timb_dma_platform_data - Platform data of the timberdale DMA driver
// @nr_channels:	Number of defined channels in the channels array.
// @channels:		Definition of the each channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timb_dma_platform_data {
    pub nr_channels: unsigned,
    pub channels: [timb_dma_platform_data_channel; 32],
}
