//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/toshsd.h
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
// Toshiba PCI Secure Digital Host Controller Interface driver
//
// Copyright (C) 2014 Ondrej Zary
// Copyright (C) 2007 Richard Betts, All Rights Reserved.
//
// Based on asic3_mmc.c Copyright (c) 2005 SDG Systems, LLC
//

pub const SD_PCICFG_CLKSTOP: c_uint = 0x40	/* 0x1f = clock controller, 0 = stop */;
pub const SD_PCICFG_GATEDCLK: c_uint = 0x41	/* Gated clock */;
pub const SD_PCICFG_CLKMODE: c_uint = 0x42	/* Control clock of SD controller */;
pub const SD_PCICFG_PINSTATUS: c_uint = 0x44	/* R/O: read status of SD pins */;
pub const SD_PCICFG_POWER1: c_uint = 0x48;
pub const SD_PCICFG_POWER2: c_uint = 0x49;
pub const SD_PCICFG_POWER3: c_uint = 0x4a;
pub const SD_PCICFG_CARDDETECT: c_uint = 0x4c;
pub const SD_PCICFG_SLOTS: c_uint = 0x50	/* R/O: define support slot number */;
pub const SD_PCICFG_EXTGATECLK1: c_uint = 0xf0	/* Could be used for gated clock */;
pub const SD_PCICFG_EXTGATECLK2: c_uint = 0xf1	/* Could be used for gated clock */;
pub const SD_PCICFG_EXTGATECLK3: c_uint = 0xf9	/* Bit 1: double buffer/single buffer */;
pub const SD_PCICFG_SDLED_ENABLE1: c_uint = 0xfa;
pub const SD_PCICFG_SDLED_ENABLE2: c_uint = 0xfe;

pub const SD_PCICFG_CLKSTOP_ENABLE_ALL: c_uint = 0x1f;
pub const SD_PCICFG_LED_ENABLE1_START: c_uint = 0x12;
pub const SD_PCICFG_LED_ENABLE2_START: c_uint = 0x80;
pub const SD_PCICFG_PWR1_33V: c_uint = 0x08	/* Set for 3.3 volts */;
pub const SD_PCICFG_PWR1_OFF: c_uint = 0x00	/* Turn off power */;
pub const SD_PCICFG_PWR2_AUTO: c_uint = 0x02;
pub const SD_CMD: c_uint = 0x00	/* also for SDIO */;
pub const SD_ARG0: c_uint = 0x04	/* also for SDIO */;
pub const SD_ARG1: c_uint = 0x06	/* also for SDIO */;
pub const SD_STOPINTERNAL: c_uint = 0x08;
pub const SD_BLOCKCOUNT: c_uint = 0x0a	/* also for SDIO */;
pub const SD_RESPONSE0: c_uint = 0x0c	/* also for SDIO */;
pub const SD_RESPONSE1: c_uint = 0x0e	/* also for SDIO */;
pub const SD_RESPONSE2: c_uint = 0x10	/* also for SDIO */;
pub const SD_RESPONSE3: c_uint = 0x12	/* also for SDIO */;
pub const SD_RESPONSE4: c_uint = 0x14	/* also for SDIO */;
pub const SD_RESPONSE5: c_uint = 0x16	/* also for SDIO */;
pub const SD_RESPONSE6: c_uint = 0x18	/* also for SDIO */;
pub const SD_RESPONSE7: c_uint = 0x1a	/* also for SDIO */;
pub const SD_CARDSTATUS: c_uint = 0x1c	/* also for SDIO */;
pub const SD_BUFFERCTRL: c_uint = 0x1e	/* also for SDIO */;
pub const SD_INTMASKCARD: c_uint = 0x20	/* also for SDIO */;
pub const SD_INTMASKBUFFER: c_uint = 0x22	/* also for SDIO */;
pub const SD_CARDCLOCKCTRL: c_uint = 0x24;
pub const SD_CARDXFERDATALEN: c_uint = 0x26	/* also for SDIO */;
pub const SD_CARDOPTIONSETUP: c_uint = 0x28	/* also for SDIO */;
pub const SD_ERRORSTATUS0: c_uint = 0x2c	/* also for SDIO */;
pub const SD_ERRORSTATUS1: c_uint = 0x2e	/* also for SDIO */;
pub const SD_DATAPORT: c_uint = 0x30	/* also for SDIO */;
pub const SD_TRANSACTIONCTRL: c_uint = 0x34	/* also for SDIO */;
pub const SD_SOFTWARERESET: c_uint = 0xe0	/* also for SDIO */;
// registers above marked "also for SDIO" and all SDIO registers below can be
// accessed at SDIO_BASE + reg address
pub const SDIO_BASE: c_uint = 0x100;
pub const SDIO_CARDPORTSEL: c_uint = 0x02;
pub const SDIO_CARDINTCTRL: c_uint = 0x36;
pub const SDIO_CLOCKNWAITCTRL: c_uint = 0x38;
pub const SDIO_HOSTINFORMATION: c_uint = 0x3a;
pub const SDIO_ERRORCTRL: c_uint = 0x3c;
pub const SDIO_LEDCTRL: c_uint = 0x3e;

pub const SD_CARDCLK_CLK_DIV_2: c_int = 0;
pub const SD_CARDOPT_REQUIRED: c_uint = 0x000e;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct toshsd_host {
    pub pdev: *mut pci_dev,
    pub mmc: *mut mmc_host,
    pub lock: spinlock_t,
    pub /: *mut *mut *mut mmc_request mrq;/ Current request,
    pub /: *mut *mut *mut mmc_command cmd;/ Current command,
    pub /: *mut *mut *mut mmc_data data; / Current data request,
    pub /: *mut *mut sg_mapping_iter sg_miter; / for PIO,
    pub /: *mut *mut *mut void __iomem ioaddr; / mapped address,
}
