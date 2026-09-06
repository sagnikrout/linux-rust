//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/davinci_asp.h
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
// TI DaVinci Audio Serial Port support
//
// Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_mcasp_pdata {
    pub tx_dma_offset: u32,
    pub rx_dma_offset: u32,
    pub /: *mut *mut int asp_chan_q; / event queue number for ASP channel,
    pub /: *mut *mut int ram_chan_q; / event queue number for RAM channel,
//
// Allowing this is more efficient and eliminates left and right swaps
// caused by underruns, but will swap the left and right channels
// when compared to previous behavior.
//
    pub enable_channel_combine:1: unsigned,
    pub sram_size_playback: unsigned,
    pub sram_size_capture: unsigned,
    pub sram_pool: *mut gen_pool,
//
// This flag works when both clock and FS are outputs for the cpu
// and makes clock more accurate (FS is not symmetrical and the
// clock is very fast.
// The clock becoming faster is named
// i2s continuous serial clock (I2S_SCK) and it is an externally
// visible bit clock.
//
// first line : WordSelect
// second line : ContinuousSerialClock
// third line: SerialData
//
// SYMMETRICAL APPROACH:
// _______________________          LEFT
// _|         RIGHT         |______________________|
// _   _         _   _   _   _         _   _
// _| |_| |_ x16 _| |_| |_| |_| |_ x16 _| |_| |_
// _   _         _   _   _   _         _   _
// _/ \_/ \_ ... _/ \_/ \_/ \_/ \_ ... _/ \_/ \_
// \_/ \_/       \_/ \_/ \_/ \_/       \_/ \_
//
// ACCURATE CLOCK APPROACH:
// ______________          LEFT
// _|     RIGHT    |_______________________________|
// _         _   _         _   _   _   _   _   _
// _| |_ x16 _| |_| |_ x16 _| |_| |_| |_| |_| |_| |
// _         _   _          _      dummy cycles
// _/ \_ ... _/ \_/ \_  ... _/ \__________________
// \_/       \_/ \_/        \_
//
    pub i2s_accurate_sck: bool,
// McASP specific fields
    pub tdm_slots_tx: c_int,
    pub tdm_slots_rx: c_int,
    pub op_mode: u8,
    pub dismod: u8,
    pub num_serializer: u8,
    pub serial_dir: *mut u8,
    pub version: u8,
    pub txnumevt: u8,
    pub rxnumevt: u8,
    pub tx_dma_channel: c_int,
    pub rx_dma_channel: c_int,
}

// TODO: Fix arch/arm/mach-davinci/ users and remove this define

pub const INACTIVE_MODE: c_int = 0;
pub const TX_MODE: c_int = 1;
pub const RX_MODE: c_int = 2;
pub const DAVINCI_MCASP_IIS_MODE: c_int = 0;
pub const DAVINCI_MCASP_DIT_MODE: c_int = 1;
