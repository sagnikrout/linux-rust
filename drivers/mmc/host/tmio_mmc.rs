//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/tmio_mmc.h
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
// Driver for the MMC / SD / SDIO cell found in:
//
// TC6393XB TC6391XB TC6387XB T7L66XB ASIC3
//
// Copyright (C) 2015-19 Renesas Electronics Corporation
// Copyright (C) 2016-19 Sang Engineering, Wolfram Sang
// Copyright (C) 2016-17 Horms Solutions, Simon Horman
// Copyright (C) 2007 Ian Molton
// Copyright (C) 2004 Ian Molton
//

pub const CTL_SD_CMD: c_uint = 0x00;
pub const CTL_ARG_REG: c_uint = 0x04;
pub const CTL_STOP_INTERNAL_ACTION: c_uint = 0x08;
pub const CTL_XFER_BLK_COUNT: c_uint = 0xa;
pub const CTL_RESPONSE: c_uint = 0x0c;
// driver merges STATUS and following STATUS2
pub const CTL_STATUS: c_uint = 0x1c;
// driver merges IRQ_MASK and following IRQ_MASK2
pub const CTL_IRQ_MASK: c_uint = 0x20;
pub const CTL_SD_CARD_CLK_CTL: c_uint = 0x24;
pub const CTL_SD_XFER_LEN: c_uint = 0x26;
pub const CTL_SD_MEM_CARD_OPT: c_uint = 0x28;
pub const CTL_SD_ERROR_DETAIL_STATUS: c_uint = 0x2c;
pub const CTL_SD_DATA_PORT: c_uint = 0x30;
pub const CTL_TRANSACTION_CTL: c_uint = 0x34;
pub const CTL_SDIO_STATUS: c_uint = 0x36;
pub const CTL_SDIO_IRQ_MASK: c_uint = 0x38;
pub const CTL_DMA_ENABLE: c_uint = 0xd8;
pub const CTL_RESET_SD: c_uint = 0xe0;
pub const CTL_VERSION: c_uint = 0xe2;
pub const CTL_SDIF_MODE: c_uint = 0xe6 /* only known on R-Car 2+ */;
pub const CTL_SD_STATUS: c_uint = 0xf2 /* only known on RZ/{G2L,G3E,V2H} */;
// Definitions for values the CTL_STOP_INTERNAL_ACTION register can take

// Definitions for values the CTL_STATUS register can take

// These belong technically to CTL_STATUS2, but the driver merges them

// Definitions for values the CTL_SD_CARD_CLK_CTL register can take
pub const CLK_CTL_DIV_MASK: c_uint = 0xff;

// Definitions for values the CTL_SD_MEM_CARD_OPT register can take
pub const CARD_OPT_TOP_MASK: c_uint = 0xf0;
pub const CARD_OPT_TOP_SHIFT: c_int = 4;

// Definitions for values the CTL_SDIO_STATUS register can take
pub const TMIO_SDIO_STAT_IOIRQ: c_uint = 0x0001;
pub const TMIO_SDIO_STAT_EXPUB52: c_uint = 0x4000;
pub const TMIO_SDIO_STAT_EXWT: c_uint = 0x8000;
pub const TMIO_SDIO_MASK_ALL: c_uint = 0xc007;
pub const TMIO_SDIO_SETBITS_MASK: c_uint = 0x0006;
// Definitions for values the CTL_DMA_ENABLE register can take

// Definitions for values the CTL_SDIF_MODE register can take

// Definitions for values the CTL_SD_STATUS register can take

// Define some IRQ masks
// This is the mask used at reset by the chip
pub const TMIO_MASK_ALL: c_uint = 0x837f031d;
pub const TMIO_MASK_ALL_RCAR2: c_uint = 0x8b7f031d;

pub const TMIO_MAX_BLK_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmio_mmc_dma_ops {
    pub data): *mut *mut *mut void (start)(struct tmio_mmc_host host, struct mmc_data,
    pub enable): *mut *mut *mut void (enable)(struct tmio_mmc_host host, bool,
    pub pdata): *mut tmio_mmc_data,
    pub host): *mut *mut void (release)(struct tmio_mmc_host,
    pub host): *mut *mut void (abort)(struct tmio_mmc_host,
    pub host): *mut *mut void (dataend)(struct tmio_mmc_host,
// optional
    pub /: *mut *mut *mut *mut void (end)(struct tmio_mmc_host host); / held host->lock,
    pub host): *mut *mut bool (dma_irq)(struct tmio_mmc_host,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmio_mmc_host {
    pub ctl: *mut void __iomem,
    pub cmd: *mut mmc_command,
    pub mrq: *mut mmc_request,
    pub data: *mut mmc_data,
    pub mmc: *mut mmc_host,
    pub ops: mmc_host_ops,
// pio related stuff
    pub sg_ptr: *mut scatterlist,
    pub sg_orig: *mut scatterlist,
    pub sg_len: c_uint,
    pub sg_off: c_uint,
    pub bus_shift: c_uint,
    pub pdev: *mut platform_device,
    pub pdata: *mut tmio_mmc_data,
// DMA support
    pub dma_on: bool,
    pub chan_rx: *mut dma_chan,
    pub chan_tx: *mut dma_chan,
    pub dma_issue: work_struct,
    pub bounce_sg: scatterlist,
    pub bounce_buf: *mut u8,
// Track lost interrupts
    pub delayed_reset_work: delayed_work,
    pub done: work_struct,
// Cache
    pub sdcard_irq_mask: u32,
    pub sdio_irq_mask: u32,
    pub clk_cache: c_uint,
    pub sdcard_irq_setbit_mask: u32,
    pub sdcard_irq_mask_all: u32,
    pub /: *mut *mut spinlock_t lock; / protect host private data,
    pub last_req_ts: c_ulong,
    pub /: *mut *mut mutex ios_lock; / protect set_ios() context,
    pub native_hotplug: bool,
    pub sdio_irq_enabled: bool,
// Mandatory callback
    pub host): *mut *mut int (clk_enable)(struct tmio_mmc_host,
    pub clock): *mut *mut *mut void (set_clock)(struct tmio_mmc_host host, unsigned int,
// Optional callbacks
    pub host): *mut *mut void (clk_disable)(struct tmio_mmc_host,
    pub blk_size): unsigned int direction, int,
    pub addr): *mut *mut *mut int (write16_hook)(struct tmio_mmc_host host, int,
    pub preserve): *mut *mut *mut void (reset)(struct tmio_mmc_host host, bool,
    pub mrq): *mut *mut *mut bool (check_retune)(struct tmio_mmc_host host, struct mmc_request,
    pub mrq): *mut *mut *mut void (fixup_request)(struct tmio_mmc_host host, struct mmc_request,
    pub host): *mut *mut unsigned int (get_timeout_cycles)(struct tmio_mmc_host,
    pub host): *mut *mut void (sdio_irq)(struct tmio_mmc_host,
    pub dma_ops: *const tmio_mmc_dma_ops,
}

extern "C" {
    pub fn tmio_mmc_host_probe(host: *mut tmio_mmc_host) -> c_int;
}
extern "C" {
    pub fn tmio_mmc_host_remove(host: *mut tmio_mmc_host);
}
extern "C" {
    pub fn tmio_mmc_do_data_irq(host: *mut tmio_mmc_host);
}
extern "C" {
    pub fn tmio_mmc_enable_mmc_irqs(host: *mut tmio_mmc_host, i: u32);
}
extern "C" {
    pub fn tmio_mmc_disable_mmc_irqs(host: *mut tmio_mmc_host, i: u32);
}
extern "C" {
    pub fn tmio_mmc_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn tmio_mmc_host_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tmio_mmc_host_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ioread16(host->bus_shift): host->ctl + (addr <<) -> return;
}
extern "C" {
    pub fn ioread32(host->bus_shift): host->ctl + (addr <<) -> return;
}

// If there is a hook and it returns non-zero then there
// is an error and the write should be skipped
//
