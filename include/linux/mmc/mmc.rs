//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/mmc.h
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


//
// Header for MultiMediaCard (MMC)
//
// Copyright 2002 Hewlett-Packard Company
//
// Use consistent with the GNU GPL is permitted,
// provided that this copyright notice is
// preserved in its entirety in all copies and derived works.
//
// HEWLETT-PACKARD COMPANY MAKES NO WARRANTIES, EXPRESSED OR IMPLIED,
// AS TO THE USEFULNESS OR CORRECTNESS OF THIS CODE OR ITS
// FITNESS FOR ANY PARTICULAR PURPOSE.
//
// Many thanks to Alessandro Rubini and Jonathan Corbet!
//
// Based strongly on code by:
//
// Author: Yong-iL Joh <tolkien@mizi.com>
//
// Author:  Andrew Christian
// 15 May 2002
//

// Standard MMC commands (4.1)           type  argument     response
// class 1

// class 2

// class 3

// class 4

// class 6

// class 5

// class 9

// class 7

// class 8

// class 11

//
// MMC_SWITCH argument format:
//
// [31:26] Always 0
// [25:24] Access Mode
// [23:16] Location of target Byte in EXT_CSD
// [15:08] Value Byte
// [07:03] Always 0
// [02:00] Command Set
//

pub const R1_STATE_IDLE: c_int = 0;
pub const R1_STATE_READY: c_int = 1;
pub const R1_STATE_IDENT: c_int = 2;
pub const R1_STATE_STBY: c_int = 3;
pub const R1_STATE_TRAN: c_int = 4;
pub const R1_STATE_DATA: c_int = 5;
pub const R1_STATE_RCV: c_int = 6;
pub const R1_STATE_PRG: c_int = 7;
pub const R1_STATE_DIS: c_int = 8;
//
// Some cards mishandle the status bits, so make sure to check both the
// busy indication and the card state.
//
// MMC/SD in SPI mode reports R1 status always, and R2 for SEND_STATUS
// R1 is the low order byte; R2 is the next highest byte, when present.
//

// R1 bit 7 is always zero

//
// OCR bits are mostly in host.h
//
pub const MMC_CARD_BUSY: c_uint = 0x80000000	/* Card Power up status bit */;
//
// Card Command Classes (CCC)
//

// (CMD0,1,2,3,4,7,9,10,12,13,15)
// (and for SPI, CMD58,59)

// (CMD11)

// (CMD16,17,18)

// (CMD20)

// (CMD16,24,25,26,27)

// (CMD32,33,34,35,36,37,38,39)

// (CMD28,29,30)

// (CMD16,CMD42)

// (CMD55,56,57,ACMD*)

// (CMD5,39,40,52,53)

// (CMD6,34,35,36,37,50)
// (11) Reserved
// (CMD?)
//
// CSD field definitions
//

//
// EXT_CSD fields
//

//
// EXT_CSD field definitions
//

// DDR mode @1.8V or 3V I/O

// DDR mode @1.2V I/O

// SDR mode @1.2V I/O

pub const EXT_CSD_RST_N_EN_MASK: c_uint = 0x3;

pub const EXT_CSD_NO_POWER_NOTIFICATION: c_int = 0;
pub const EXT_CSD_POWER_ON: c_int = 1;
pub const EXT_CSD_POWER_OFF_SHORT: c_int = 2;
pub const EXT_CSD_POWER_OFF_LONG: c_int = 3;
pub const EXT_CSD_PWR_CL_8BIT_MASK: c_uint = 0xF0	/* 8 bit PWR CLS */;
pub const EXT_CSD_PWR_CL_4BIT_MASK: c_uint = 0x0F	/* 8 bit PWR CLS */;
pub const EXT_CSD_PWR_CL_8BIT_SHIFT: c_int = 4;
pub const EXT_CSD_PWR_CL_4BIT_SHIFT: c_int = 0;
//
// EXCEPTION_EVENT_STATUS field
//

//
// BKOPS status level
//
pub const EXT_CSD_BKOPS_LEVEL_2: c_uint = 0x2;
//
// BKOPS modes
//
pub const EXT_CSD_MANUAL_BKOPS_MASK: c_uint = 0x01;
pub const EXT_CSD_AUTO_BKOPS_MASK: c_uint = 0x02;
//
// Command Queue
//

//
// MMC_SWITCH access modes
//
pub const MMC_SWITCH_MODE_CMD_SET: c_uint = 0x00	/* Change the command set */;
pub const MMC_SWITCH_MODE_SET_BITS: c_uint = 0x01	/* Set bits which are 1 in value */;
pub const MMC_SWITCH_MODE_CLEAR_BITS: c_uint = 0x02	/* Clear bits which are 1 in value */;
pub const MMC_SWITCH_MODE_WRITE_BYTE: c_uint = 0x03	/* Set target to value */;
//
// Erase/trim/discard
//
pub const MMC_ERASE_ARG: c_uint = 0x00000000;
pub const MMC_SECURE_ERASE_ARG: c_uint = 0x80000000;
pub const MMC_TRIM_ARG: c_uint = 0x00000001;
pub const MMC_DISCARD_ARG: c_uint = 0x00000003;
pub const MMC_SECURE_TRIM1_ARG: c_uint = 0x80000001;
pub const MMC_SECURE_TRIM2_ARG: c_uint = 0x80008000;
pub const MMC_SECURE_ARGS: c_uint = 0x80000000;
pub const MMC_TRIM_OR_DISCARD_ARGS: c_uint = 0x00008003;

