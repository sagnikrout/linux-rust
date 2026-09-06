//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intersil/p54/p54spi.h
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
// Copyright (C) 2008 Christian Lamparter <chunkeey@web.de>
//
// This driver is a port from stlc45xx:
// Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies).
//

// Bit 15 is read/write bit; ON = READ, OFF = WRITE
pub const SPI_ADRS_READ_BIT_15: c_uint = 0x8000;
pub const SPI_ADRS_ARM_INTERRUPTS: c_uint = 0x00;
pub const SPI_ADRS_ARM_INT_EN: c_uint = 0x04;
pub const SPI_ADRS_HOST_INTERRUPTS: c_uint = 0x08;
pub const SPI_ADRS_HOST_INT_EN: c_uint = 0x0c;
pub const SPI_ADRS_HOST_INT_ACK: c_uint = 0x10;
pub const SPI_ADRS_GEN_PURP_1: c_uint = 0x14;
pub const SPI_ADRS_GEN_PURP_2: c_uint = 0x18;
pub const SPI_ADRS_DEV_CTRL_STAT: c_uint = 0x26    /* high word */;
pub const SPI_ADRS_DMA_DATA: c_uint = 0x28;
pub const SPI_ADRS_DMA_WRITE_CTRL: c_uint = 0x2c;
pub const SPI_ADRS_DMA_WRITE_LEN: c_uint = 0x2e;
pub const SPI_ADRS_DMA_WRITE_BASE: c_uint = 0x30;
pub const SPI_ADRS_DMA_READ_CTRL: c_uint = 0x34;
pub const SPI_ADRS_DMA_READ_LEN: c_uint = 0x36;
pub const SPI_ADRS_DMA_READ_BASE: c_uint = 0x38;
pub const SPI_CTRL_STAT_HOST_OVERRIDE: c_uint = 0x8000;
pub const SPI_CTRL_STAT_START_HALTED: c_uint = 0x4000;
pub const SPI_CTRL_STAT_RAM_BOOT: c_uint = 0x2000;
pub const SPI_CTRL_STAT_HOST_RESET: c_uint = 0x1000;
pub const SPI_CTRL_STAT_HOST_CPU_EN: c_uint = 0x0800;
pub const SPI_DMA_WRITE_CTRL_ENABLE: c_uint = 0x0001;
pub const SPI_DMA_READ_CTRL_ENABLE: c_uint = 0x0001;

pub const SPI_MAX_TX_PACKETS: c_int = 32;
pub const SPI_MAX_PACKET_SIZE: c_int = 32767;
pub const SPI_TARGET_INT_WAKEUP: c_uint = 0x00000001;
pub const SPI_TARGET_INT_SLEEP: c_uint = 0x00000002;
pub const SPI_TARGET_INT_RDDONE: c_uint = 0x00000004;
pub const SPI_TARGET_INT_CTS: c_uint = 0x00004000;
pub const SPI_TARGET_INT_DR: c_uint = 0x00008000;
pub const SPI_HOST_INT_READY: c_uint = 0x00000001;
pub const SPI_HOST_INT_WR_READY: c_uint = 0x00000002;
pub const SPI_HOST_INT_SW_UPDATE: c_uint = 0x00000004;
pub const SPI_HOST_INT_UPDATE: c_uint = 0x10000000;
// clear to send
pub const SPI_HOST_INT_CR: c_uint = 0x00004000;
// data ready
pub const SPI_HOST_INT_DR: c_uint = 0x00008000;

pub const TARGET_BOOT_SLEEP: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54s_dma_regs {
    pub cmd: __le16,
    pub len: __le16,
    pub addr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54s_tx_info {
    pub tx_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54s_priv {
// p54_common has to be the first entry
    pub common: p54_common,
    pub hw: *mut ieee80211_hw,
    pub spi: *mut spi_device,
    pub work: work_struct,
    pub mutex: mutex,
    pub fw_comp: completion,
    pub tx_lock: spinlock_t,
// protected by tx_lock
    pub tx_pending: list_head,
    pub fw_state: fw_state,
    pub firmware: *const firmware,
    pub gpio_powerdown: *mut gpio_desc,
    pub irq: c_int,
}
