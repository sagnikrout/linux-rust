//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/atmel/atmel-isi.h
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
// Register definitions for the Atmel Image Sensor Interface.
//
// Copyright (C) 2011 Atmel Corporation
// Josh Wu, <josh.wu@atmel.com>
//
// Based on previous work by Lars Haring, <lars.haring@atmel.com>
// and Sedji Gaouaou
//

// ISI_V2 register offsets
pub const ISI_CFG1: c_uint = 0x0000;
pub const ISI_CFG2: c_uint = 0x0004;
pub const ISI_PSIZE: c_uint = 0x0008;
pub const ISI_PDECF: c_uint = 0x000c;
pub const ISI_Y2R_SET0: c_uint = 0x0010;
pub const ISI_Y2R_SET1: c_uint = 0x0014;
pub const ISI_R2Y_SET0: c_uint = 0x0018;
pub const ISI_R2Y_SET1: c_uint = 0x001C;
pub const ISI_R2Y_SET2: c_uint = 0x0020;
pub const ISI_CTRL: c_uint = 0x0024;
pub const ISI_STATUS: c_uint = 0x0028;
pub const ISI_INTEN: c_uint = 0x002C;
pub const ISI_INTDIS: c_uint = 0x0030;
pub const ISI_INTMASK: c_uint = 0x0034;
pub const ISI_DMA_CHER: c_uint = 0x0038;
pub const ISI_DMA_CHDR: c_uint = 0x003C;
pub const ISI_DMA_CHSR: c_uint = 0x0040;
pub const ISI_DMA_P_ADDR: c_uint = 0x0044;
pub const ISI_DMA_P_CTRL: c_uint = 0x0048;
pub const ISI_DMA_P_DSCR: c_uint = 0x004C;
pub const ISI_DMA_C_ADDR: c_uint = 0x0050;
pub const ISI_DMA_C_CTRL: c_uint = 0x0054;
pub const ISI_DMA_C_DSCR: c_uint = 0x0058;
// Bitfields in CFG1

// Constants for FRATE(ISI_V2)

// Definition for THMASK(ISI_V2)

// Bitfields in CFG2

// Constants for YCC_SWAP(ISI_V2)

pub const ISI_CFG2_IM_VSIZE_OFFSET: c_int = 0;
pub const ISI_CFG2_IM_HSIZE_OFFSET: c_int = 16;

// Bitfields in PSIZE
pub const ISI_PSIZE_PREV_VSIZE_OFFSET: c_int = 0;
pub const ISI_PSIZE_PREV_HSIZE_OFFSET: c_int = 16;

// Bitfields in PDECF

// Bitfields in CTRL
// Also using in SR(ISI_V2)

// Also using in SR/IER/IDR/IMR(ISI_V2)

// Bitfields in SR

// Also using in SR/IER/IDR/IMR

// Bitfields in DMA_C_CTRL & in DMA_P_CTRL

// Bitfields in DMA_CHSR/CHER/CHDR

// Definition for isi_platform_data
pub const ISI_DATAWIDTH_8: c_uint = 0x01;
pub const ISI_DATAWIDTH_10: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isi_platform_data {
    pub has_emb_sync: u8,
    pub hsync_act_low: u8,
    pub vsync_act_low: u8,
    pub pclk_act_falling: u8,
    pub full_mode: u8,
    pub data_width_flags: u32,
// Using for ISI_CFG1
    pub frate: u32,
}
