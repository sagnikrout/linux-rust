//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/sdio.h
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
// include/linux/mmc/sdio.h
//
// Copyright 2006-2007 Pierre Ossman
//
// SDIO commands                         type  argument     response

//
// SD_IO_RW_DIRECT argument format:
//
// [31] R/W flag
// [30:28] Function number
// [27] RAW flag
// [25:9] Register address
// [7:0] Data
//
// SD_IO_RW_EXTENDED argument format:
//
// [31] R/W flag
// [30:28] Function number
// [27] Block mode
// [26] Increment address
// [25:9] Register address
// [8:0] Byte/block count
//

//

//
// Card Common Control Registers (CCCR)
//
pub const SDIO_CCCR_CCCR: c_uint = 0x00;

pub const SDIO_CCCR_SD: c_uint = 0x01;

pub const SDIO_CCCR_IOEx: c_uint = 0x02;
pub const SDIO_CCCR_IORx: c_uint = 0x03;
pub const SDIO_CCCR_IENx: c_uint = 0x04	/* Function/Master Interrupt Enable */;
pub const SDIO_CCCR_INTx: c_uint = 0x05	/* Function Interrupt Pending */;
pub const SDIO_CCCR_ABORT: c_uint = 0x06	/* function abort/card reset */;
pub const SDIO_CCCR_IF: c_uint = 0x07	/* bus interface controls */;
pub const SDIO_BUS_WIDTH_MASK: c_uint = 0x03	/* data bus width setting */;
pub const SDIO_BUS_WIDTH_1BIT: c_uint = 0x00;
pub const SDIO_BUS_WIDTH_RESERVED: c_uint = 0x01;
pub const SDIO_BUS_WIDTH_4BIT: c_uint = 0x02;
pub const SDIO_BUS_ECSI: c_uint = 0x20	/* Enable continuous SPI interrupt */;
pub const SDIO_BUS_SCSI: c_uint = 0x40	/* Support continuous SPI interrupt */;
pub const SDIO_BUS_ASYNC_INT: c_uint = 0x20;
pub const SDIO_BUS_CD_DISABLE: c_uint = 0x80	/* disable pull-up on DAT3 (pin 1) */;
pub const SDIO_CCCR_CAPS: c_uint = 0x08;
pub const SDIO_CCCR_CAP_SDC: c_uint = 0x01	/* can do CMD52 while data transfer */;
pub const SDIO_CCCR_CAP_SMB: c_uint = 0x02	/* can do multi-block xfers (CMD53) */;
pub const SDIO_CCCR_CAP_SRW: c_uint = 0x04	/* supports read-wait protocol */;
pub const SDIO_CCCR_CAP_SBS: c_uint = 0x08	/* supports suspend/resume */;
pub const SDIO_CCCR_CAP_S4MI: c_uint = 0x10	/* interrupt during 4-bit CMD53 */;
pub const SDIO_CCCR_CAP_E4MI: c_uint = 0x20	/* enable ints during 4-bit CMD53 */;
pub const SDIO_CCCR_CAP_LSC: c_uint = 0x40	/* low speed card */;
pub const SDIO_CCCR_CAP_4BLS: c_uint = 0x80	/* 4 bit low speed card */;
pub const SDIO_CCCR_CIS: c_uint = 0x09	/* common CIS pointer (3 bytes) */;
// Following 4 regs are valid only if SBS is set
pub const SDIO_CCCR_SUSPEND: c_uint = 0x0c;
pub const SDIO_CCCR_SELx: c_uint = 0x0d;
pub const SDIO_CCCR_EXECx: c_uint = 0x0e;
pub const SDIO_CCCR_READYx: c_uint = 0x0f;
pub const SDIO_CCCR_BLKSIZE: c_uint = 0x10;
pub const SDIO_CCCR_POWER: c_uint = 0x12;
pub const SDIO_POWER_SMPC: c_uint = 0x01	/* Supports Master Power Control */;
pub const SDIO_POWER_EMPC: c_uint = 0x02	/* Enable Master Power Control */;
pub const SDIO_CCCR_SPEED: c_uint = 0x13;
pub const SDIO_SPEED_SHS: c_uint = 0x01	/* Supports High-Speed mode */;
pub const SDIO_SPEED_BSS_SHIFT: c_int = 1;

pub const SDIO_CCCR_UHS: c_uint = 0x14;
pub const SDIO_UHS_SDR50: c_uint = 0x01;
pub const SDIO_UHS_SDR104: c_uint = 0x02;
pub const SDIO_UHS_DDR50: c_uint = 0x04;
pub const SDIO_CCCR_DRIVE_STRENGTH: c_uint = 0x15;
pub const SDIO_SDTx_MASK: c_uint = 0x07;

pub const SDIO_DRIVE_DTSx_MASK: c_uint = 0x03;
pub const SDIO_DRIVE_DTSx_SHIFT: c_int = 4;

pub const SDIO_CCCR_INTERRUPT_EXT: c_uint = 0x16;

//
// Function Basic Registers (FBR)
//

pub const SDIO_FBR_STD_IF: c_uint = 0x00;
pub const SDIO_FBR_SUPPORTS_CSA: c_uint = 0x40	/* supports Code Storage Area */;
pub const SDIO_FBR_ENABLE_CSA: c_uint = 0x80	/* enable Code Storage Area */;
pub const SDIO_FBR_STD_IF_EXT: c_uint = 0x01;
pub const SDIO_FBR_POWER: c_uint = 0x02;
pub const SDIO_FBR_POWER_SPS: c_uint = 0x01	/* Supports Power Selection */;
pub const SDIO_FBR_POWER_EPS: c_uint = 0x02	/* Enable (low) Power Selection */;
pub const SDIO_FBR_CIS: c_uint = 0x09	/* CIS pointer (3 bytes) */;
pub const SDIO_FBR_CSA: c_uint = 0x0C	/* CSA pointer (3 bytes) */;
pub const SDIO_FBR_CSA_DATA: c_uint = 0x0F;
pub const SDIO_FBR_BLKSIZE: c_uint = 0x10	/* block size (2 bytes) */;
