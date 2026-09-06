//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/adc/stm32-dfsdm.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This file is part of STM32 DFSDM driver
//
// Copyright (C) 2017, STMicroelectronics - All Rights Reserved
// Author(s): Arnaud Pouliquen <arnaud.pouliquen@st.com>.
//

//
// STM32 DFSDM - global register map
// __________________________________________________________
// | Offset    |             Registers block                |
// ----------------------------------------------------------
// | 0x000     |      CHANNEL 0 + COMMON CHANNEL FIELDS     |
// ----------------------------------------------------------
// | 0x020     |                CHANNEL 1                   |
// ----------------------------------------------------------
// | ...       |                 .....                      |
// ----------------------------------------------------------
// | 0x20 x n  |                CHANNEL n                   |
// ----------------------------------------------------------
// | 0x100     |      FILTER  0 + COMMON FILTER FIELDs      |
// ----------------------------------------------------------
// | 0x200     |                FILTER  1                   |
// ----------------------------------------------------------
// |           |                 .....                      |
// ----------------------------------------------------------
// | 0x100 x m |                FILTER  m                   |
// ----------------------------------------------------------
// |           |                 .....                      |
// ----------------------------------------------------------
// | 0x7F0-7FC |         Identification registers           |
// ----------------------------------------------------------
//
// Channels register definitions
//

// CHCFGR1: Channel configuration register 1

// CHCFGR2: Channel configuration register 2

// AWSCDR: Channel analog watchdog and short circuit detector

//
// Filters register definitions
//
pub const DFSDM_FILTER_BASE_ADR: c_uint = 0x100;
pub const DFSDM_FILTER_REG_MASK: c_uint = 0x7F;

// CR1 Control register 1

// CR2: Control register 2

// ISR: Interrupt status register

// ICR: Interrupt flag clear register

// FCR: Filter control register

// RDATAR: Filter data register for regular channel

pub const DFSDM_DATAR_DATA_OFFSET: c_int = 8;

// AWLTR: Filter analog watchdog low threshold register

// AWHTR: Filter analog watchdog low threshold register

// AWSR: Filter watchdog status register

// AWCFR: Filter watchdog status register

//
// Identification register definitions
//
pub const DFSDM_HWCFGR: c_uint = 0x7F0;
pub const DFSDM_VERR: c_uint = 0x7F4;
pub const DFSDM_IPIDR: c_uint = 0x7F8;
pub const DFSDM_SIDR: c_uint = 0x7FC;
// HWCFGR: Hardware configuration register

// VERR: Version register

pub const STM32MP15_IPIDR_NUMBER: c_uint = 0x00110031;
// DFSDM filter order
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm32_dfsdm_sinc_order {
    DFSDM_FASTSINC_ORDER, /* FastSinc filter type */
    DFSDM_SINC1_ORDER,    /* Sinc 1 filter type */
    DFSDM_SINC2_ORDER,    /* Sinc 2 filter type */
    DFSDM_SINC3_ORDER,    /* Sinc 3 filter type */
    DFSDM_SINC4_ORDER,    /* Sinc 4 filter type (N.A. for watchdog) */
    DFSDM_SINC5_ORDER,    /* Sinc 5 filter type (N.A. for watchdog) */
    DFSDM_NB_SINC_ORDER,
}

//
// struct stm32_dfsdm_filter_osr - DFSDM filter settings linked to oversampling
// @iosr: integrator oversampling
// @fosr: filter oversampling
// @rshift: output sample right shift (hardware shift)
// @lshift: output sample left shift (software shift)
// @res: output sample resolution
// @bits: output sample resolution in bits
// @max: output sample maximum positive value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_dfsdm_filter_osr {
    pub iosr: c_uint,
    pub fosr: c_uint,
    pub rshift: c_uint,
    pub lshift: c_uint,
    pub res: u64,
    pub bits: u32,
    pub max: i32,
}

//
// struct stm32_dfsdm_filter - structure relative to stm32 FDSDM filter
// @ford: filter order
// @flo: filter oversampling data table indexed by fast mode flag
// @sync_mode: filter synchronized with filter 0
// @fast: filter fast mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_dfsdm_filter {
    pub ford: stm32_dfsdm_sinc_order,
    pub flo: [stm32_dfsdm_filter_osr; 2],
    pub sync_mode: c_uint,
    pub fast: c_uint,
}

//
// struct stm32_dfsdm_channel - structure relative to stm32 FDSDM channel
// @id: id of the channel
// @type: interface type linked to stm32_dfsdm_chan_type
// @src: interface type linked to stm32_dfsdm_chan_src
// @alt_si: alternative serial input interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_dfsdm_channel {
    pub id: c_uint,
    pub type: c_uint,
    pub src: c_uint,
    pub alt_si: c_uint,
}

//
// struct stm32_dfsdm - stm32 FDSDM driver common data (for all instances)
// @base:	control registers base cpu addr
// @phys_base:	DFSDM IP register physical address
// @regmap:	regmap for register read/write
// @fl_list:	filter resources list
// @num_fls:	number of filter resources available
// @ch_list:	channel resources list
// @num_chs:	number of channel resources available
// @spi_master_freq: SPI clock out frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_dfsdm {
    pub base: *mut void __iomem,
    pub phys_base: phys_addr_t,
    pub regmap: *mut regmap,
    pub fl_list: *mut stm32_dfsdm_filter,
    pub num_fls: c_uint,
    pub ch_list: *mut stm32_dfsdm_channel,
    pub num_chs: c_uint,
    pub spi_master_freq: c_uint,
}

// DFSDM channel serial spi clock source
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm32_dfsdm_spi_clk_src {
    DFSDM_CHANNEL_SPI_CLOCK_EXTERNAL,
    DFSDM_CHANNEL_SPI_CLOCK_INTERNAL,
    DFSDM_CHANNEL_SPI_CLOCK_INTERNAL_DIV2_FALLING,
    DFSDM_CHANNEL_SPI_CLOCK_INTERNAL_DIV2_RISING
}

extern "C" {
    pub fn stm32_dfsdm_start_dfsdm(dfsdm: *mut stm32_dfsdm) -> c_int;
}
extern "C" {
    pub fn stm32_dfsdm_stop_dfsdm(dfsdm: *mut stm32_dfsdm) -> c_int;
}
