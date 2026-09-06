//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/cb710-mmc.h
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
// cb710/cb710-mmc.h
//
// Copyright by Michał Mirosław, 2008-2009
//

// per-MMC-reader structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb710_mmc_reader {
    pub finish_req_bh_work: work_struct,
    pub mrq: *mut mmc_request,
    pub irq_lock: spinlock_t,
    pub last_power_mode: c_uchar,
}

// some device struct walking
extern "C" {
    pub fn platform_get_drvdata(_arg: &slot->pdev) -> return;
}
extern "C" {
    pub fn cb710_pdev_to_slot(_arg: pdev) -> return;
}
// registers (this might be all wrong ;)
pub const CB710_MMC_DATA_PORT: c_uint = 0x00;
pub const CB710_MMC_CONFIG_PORT: c_uint = 0x04;
pub const CB710_MMC_CONFIG0_PORT: c_uint = 0x04;
pub const CB710_MMC_CONFIG1_PORT: c_uint = 0x05;
pub const CB710_MMC_C1_4BIT_DATA_BUS: c_uint = 0x40;
pub const CB710_MMC_CONFIG2_PORT: c_uint = 0x06;
pub const CB710_MMC_C2_READ_PIO_SIZE_MASK: c_uint = 0x0F	/* N-1 */;
pub const CB710_MMC_CONFIG3_PORT: c_uint = 0x07;
pub const CB710_MMC_CONFIGB_PORT: c_uint = 0x08;
pub const CB710_MMC_IRQ_ENABLE_PORT: c_uint = 0x0C;
pub const CB710_MMC_IE_TEST_MASK: c_uint = 0x00BF;
pub const CB710_MMC_IE_CARD_INSERTION_STATUS: c_uint = 0x1000;
pub const CB710_MMC_IE_IRQ_ENABLE: c_uint = 0x8000;

pub const CB710_MMC_STATUS_PORT: c_uint = 0x10;
pub const CB710_MMC_STATUS_ERROR_EVENTS: c_uint = 0x60FF;
pub const CB710_MMC_STATUS0_PORT: c_uint = 0x10;
pub const CB710_MMC_S0_FIFO_UNDERFLOW: c_uint = 0x40;
pub const CB710_MMC_STATUS1_PORT: c_uint = 0x11;
pub const CB710_MMC_S1_COMMAND_SENT: c_uint = 0x01;
pub const CB710_MMC_S1_DATA_TRANSFER_DONE: c_uint = 0x02;
pub const CB710_MMC_S1_PIO_TRANSFER_DONE: c_uint = 0x04;
pub const CB710_MMC_S1_CARD_CHANGED: c_uint = 0x10;
pub const CB710_MMC_S1_RESET: c_uint = 0x20;
pub const CB710_MMC_STATUS2_PORT: c_uint = 0x12;
pub const CB710_MMC_S2_FIFO_READY: c_uint = 0x01;
pub const CB710_MMC_S2_FIFO_EMPTY: c_uint = 0x02;
pub const CB710_MMC_S2_BUSY_10: c_uint = 0x10;
pub const CB710_MMC_S2_BUSY_20: c_uint = 0x20;
pub const CB710_MMC_STATUS3_PORT: c_uint = 0x13;
pub const CB710_MMC_S3_CARD_DETECTED: c_uint = 0x02;
pub const CB710_MMC_S3_WRITE_PROTECTED: c_uint = 0x04;
pub const CB710_MMC_CMD_TYPE_PORT: c_uint = 0x14;
pub const CB710_MMC_RSP_TYPE_MASK: c_uint = 0x0007;

pub const CB710_MMC_RSP_PRESENT_MASK: c_uint = 0x0018;

pub const CB710_MMC_CMD_TYPE_MASK: c_uint = 0x0060;

pub const CB710_MMC_DATA_READ: c_uint = 0x0080;
pub const CB710_MMC_CMD_CODE_MASK: c_uint = 0x3F00;
pub const CB710_MMC_CMD_CODE_SHIFT: c_int = 8;
pub const CB710_MMC_IS_APP_CMD: c_uint = 0x4000;
pub const CB710_MMC_RSP_BUSY: c_uint = 0x8000;
pub const CB710_MMC_CMD_PARAM_PORT: c_uint = 0x18;
pub const CB710_MMC_TRANSFER_SIZE_PORT: c_uint = 0x1C;
pub const CB710_MMC_RESPONSE0_PORT: c_uint = 0x20;
pub const CB710_MMC_RESPONSE1_PORT: c_uint = 0x24;
pub const CB710_MMC_RESPONSE2_PORT: c_uint = 0x28;
pub const CB710_MMC_RESPONSE3_PORT: c_uint = 0x2C;
