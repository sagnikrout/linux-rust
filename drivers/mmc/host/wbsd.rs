//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/wbsd.h
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
// linux/drivers/mmc/host/wbsd.h - Winbond W83L51xD SD/MMC driver
//
// Copyright (C) 2004-2007 Pierre Ossman, All Rights Reserved.
//
pub const LOCK_CODE: c_uint = 0xAA;
pub const WBSD_CONF_SWRST: c_uint = 0x02;
pub const WBSD_CONF_DEVICE: c_uint = 0x07;
pub const WBSD_CONF_ID_HI: c_uint = 0x20;
pub const WBSD_CONF_ID_LO: c_uint = 0x21;
pub const WBSD_CONF_POWER: c_uint = 0x22;
pub const WBSD_CONF_PME: c_uint = 0x23;
pub const WBSD_CONF_PMES: c_uint = 0x24;
pub const WBSD_CONF_ENABLE: c_uint = 0x30;
pub const WBSD_CONF_PORT_HI: c_uint = 0x60;
pub const WBSD_CONF_PORT_LO: c_uint = 0x61;
pub const WBSD_CONF_IRQ: c_uint = 0x70;
pub const WBSD_CONF_DRQ: c_uint = 0x74;
pub const WBSD_CONF_PINS: c_uint = 0xF0;
pub const DEVICE_SD: c_uint = 0x03;
pub const WBSD_PINS_DAT3_HI: c_uint = 0x20;
pub const WBSD_PINS_DAT3_OUT: c_uint = 0x10;
pub const WBSD_PINS_GP11_HI: c_uint = 0x04;
pub const WBSD_PINS_DETECT_GP11: c_uint = 0x02;
pub const WBSD_PINS_DETECT_DAT3: c_uint = 0x01;
pub const WBSD_CMDR: c_uint = 0x00;
pub const WBSD_DFR: c_uint = 0x01;
pub const WBSD_EIR: c_uint = 0x02;
pub const WBSD_ISR: c_uint = 0x03;
pub const WBSD_FSR: c_uint = 0x04;
pub const WBSD_IDXR: c_uint = 0x05;
pub const WBSD_DATAR: c_uint = 0x06;
pub const WBSD_CSR: c_uint = 0x07;
pub const WBSD_EINT_CARD: c_uint = 0x40;
pub const WBSD_EINT_FIFO_THRE: c_uint = 0x20;
pub const WBSD_EINT_CRC: c_uint = 0x10;
pub const WBSD_EINT_TIMEOUT: c_uint = 0x08;
pub const WBSD_EINT_PROGEND: c_uint = 0x04;
pub const WBSD_EINT_BUSYEND: c_uint = 0x02;
pub const WBSD_EINT_TC: c_uint = 0x01;
pub const WBSD_INT_PENDING: c_uint = 0x80;
pub const WBSD_INT_CARD: c_uint = 0x40;
pub const WBSD_INT_FIFO_THRE: c_uint = 0x20;
pub const WBSD_INT_CRC: c_uint = 0x10;
pub const WBSD_INT_TIMEOUT: c_uint = 0x08;
pub const WBSD_INT_PROGEND: c_uint = 0x04;
pub const WBSD_INT_BUSYEND: c_uint = 0x02;
pub const WBSD_INT_TC: c_uint = 0x01;
pub const WBSD_FIFO_EMPTY: c_uint = 0x80;
pub const WBSD_FIFO_FULL: c_uint = 0x40;
pub const WBSD_FIFO_EMTHRE: c_uint = 0x20;
pub const WBSD_FIFO_FUTHRE: c_uint = 0x10;
pub const WBSD_FIFO_SZMASK: c_uint = 0x0F;
pub const WBSD_MSLED: c_uint = 0x20;
pub const WBSD_POWER_N: c_uint = 0x10;
pub const WBSD_WRPT: c_uint = 0x04;
pub const WBSD_CARDPRESENT: c_uint = 0x01;
pub const WBSD_IDX_CLK: c_uint = 0x01;
pub const WBSD_IDX_PBSMSB: c_uint = 0x02;
pub const WBSD_IDX_TAAC: c_uint = 0x03;
pub const WBSD_IDX_NSAC: c_uint = 0x04;
pub const WBSD_IDX_PBSLSB: c_uint = 0x05;
pub const WBSD_IDX_SETUP: c_uint = 0x06;
pub const WBSD_IDX_DMA: c_uint = 0x07;
pub const WBSD_IDX_FIFOEN: c_uint = 0x08;
pub const WBSD_IDX_STATUS: c_uint = 0x10;
pub const WBSD_IDX_RSPLEN: c_uint = 0x1E;
pub const WBSD_IDX_RESP0: c_uint = 0x1F;
pub const WBSD_IDX_RESP1: c_uint = 0x20;
pub const WBSD_IDX_RESP2: c_uint = 0x21;
pub const WBSD_IDX_RESP3: c_uint = 0x22;
pub const WBSD_IDX_RESP4: c_uint = 0x23;
pub const WBSD_IDX_RESP5: c_uint = 0x24;
pub const WBSD_IDX_RESP6: c_uint = 0x25;
pub const WBSD_IDX_RESP7: c_uint = 0x26;
pub const WBSD_IDX_RESP8: c_uint = 0x27;
pub const WBSD_IDX_RESP9: c_uint = 0x28;
pub const WBSD_IDX_RESP10: c_uint = 0x29;
pub const WBSD_IDX_RESP11: c_uint = 0x2A;
pub const WBSD_IDX_RESP12: c_uint = 0x2B;
pub const WBSD_IDX_RESP13: c_uint = 0x2C;
pub const WBSD_IDX_RESP14: c_uint = 0x2D;
pub const WBSD_IDX_RESP15: c_uint = 0x2E;
pub const WBSD_IDX_RESP16: c_uint = 0x2F;
pub const WBSD_IDX_CRCSTATUS: c_uint = 0x30;
pub const WBSD_IDX_ISR: c_uint = 0x3F;
pub const WBSD_CLK_375K: c_uint = 0x00;
pub const WBSD_CLK_12M: c_uint = 0x01;
pub const WBSD_CLK_16M: c_uint = 0x02;
pub const WBSD_CLK_24M: c_uint = 0x03;
pub const WBSD_DATA_WIDTH: c_uint = 0x01;
pub const WBSD_DAT3_H: c_uint = 0x08;
pub const WBSD_FIFO_RESET: c_uint = 0x04;
pub const WBSD_SOFT_RESET: c_uint = 0x02;
pub const WBSD_INC_INDEX: c_uint = 0x01;
pub const WBSD_DMA_SINGLE: c_uint = 0x02;
pub const WBSD_DMA_ENABLE: c_uint = 0x01;
pub const WBSD_FIFOEN_EMPTY: c_uint = 0x20;
pub const WBSD_FIFOEN_FULL: c_uint = 0x10;
pub const WBSD_FIFO_THREMASK: c_uint = 0x0F;
pub const WBSD_BLOCK_READ: c_uint = 0x80;
pub const WBSD_BLOCK_WRITE: c_uint = 0x40;
pub const WBSD_BUSY: c_uint = 0x20;
pub const WBSD_CARDTRAFFIC: c_uint = 0x04;
pub const WBSD_SENDCMD: c_uint = 0x02;
pub const WBSD_RECVRES: c_uint = 0x01;
pub const WBSD_RSP_SHORT: c_uint = 0x00;
pub const WBSD_RSP_LONG: c_uint = 0x01;
pub const WBSD_CRC_MASK: c_uint = 0x1F;
pub const WBSD_CRC_OK: c_uint = 0x05 /* S010E (00101) */;
pub const WBSD_CRC_FAIL: c_uint = 0x0B /* S101E (01011) */;
pub const WBSD_DMA_SIZE: c_int = 65536;

