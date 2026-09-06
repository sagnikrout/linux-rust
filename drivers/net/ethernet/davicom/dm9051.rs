//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/davicom/dm9051.h
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
// Copyright (c) 2022 Davicom Semiconductor,Inc.
// Davicom DM9051 SPI Fast Ethernet Linux driver
//

pub const DM9051_ID: c_uint = 0x9051;
pub const DM9051_NCR: c_uint = 0x00;
pub const DM9051_NSR: c_uint = 0x01;
pub const DM9051_TCR: c_uint = 0x02;
pub const DM9051_RCR: c_uint = 0x05;
pub const DM9051_BPTR: c_uint = 0x08;
pub const DM9051_FCR: c_uint = 0x0A;
pub const DM9051_EPCR: c_uint = 0x0B;
pub const DM9051_EPAR: c_uint = 0x0C;
pub const DM9051_EPDRL: c_uint = 0x0D;
pub const DM9051_EPDRH: c_uint = 0x0E;
pub const DM9051_PAR: c_uint = 0x10;
pub const DM9051_MAR: c_uint = 0x16;
pub const DM9051_GPCR: c_uint = 0x1E;
pub const DM9051_GPR: c_uint = 0x1F;
pub const DM9051_VIDL: c_uint = 0x28;
pub const DM9051_VIDH: c_uint = 0x29;
pub const DM9051_PIDL: c_uint = 0x2A;
pub const DM9051_PIDH: c_uint = 0x2B;
pub const DM9051_SMCR: c_uint = 0x2F;
pub const DM9051_ATCR: c_uint = 0x30;
pub const DM9051_SPIBCR: c_uint = 0x38;
pub const DM9051_INTCR: c_uint = 0x39;
pub const DM9051_PPCR: c_uint = 0x3D;
pub const DM9051_MPCR: c_uint = 0x55;
pub const DM9051_LMCR: c_uint = 0x57;
pub const DM9051_MBNDRY: c_uint = 0x5E;
pub const DM9051_MRRL: c_uint = 0x74;
pub const DM9051_MRRH: c_uint = 0x75;
pub const DM9051_MWRL: c_uint = 0x7A;
pub const DM9051_MWRH: c_uint = 0x7B;
pub const DM9051_TXPLL: c_uint = 0x7C;
pub const DM9051_TXPLH: c_uint = 0x7D;
pub const DM9051_ISR: c_uint = 0x7E;
pub const DM9051_IMR: c_uint = 0x7F;
pub const DM_SPI_MRCMDX: c_uint = 0x70;
pub const DM_SPI_MRCMD: c_uint = 0x72;
pub const DM_SPI_MWCMD: c_uint = 0x78;
pub const DM_SPI_WR: c_uint = 0x80;
// dm9051 Ethernet controller registers bits
//
// 0x00

// 0x01

// 0x02

// 0x05

// 0x06

// 0x0A

// 0x0B

// 0x1E

// 0x1F

// 0x30

// 0x39

// 0x3D
// Pause Packet Control Register - default = 1
pub const PPCR_PAUSE_COUNT: c_uint = 0x08;
// 0x55

// 0x57
// LEDMode Control Register - LEDMode1
// Value 0x81 : bit[7] = 1, bit[2] = 0, bit[1:0] = 01b

// 0x5E

// 0xFE

// 0xFF

// Const
//

pub const DM9051_PHY: c_uint = 0x40	/* PHY address 0x01 */;
pub const DM9051_PKT_RDY: c_uint = 0x01	/* Packet ready to receive */;

pub const DM9051_TX_QUE_HI_WATER: c_int = 50;
pub const DM9051_TX_QUE_LO_WATER: c_int = 25;
pub const DM_EEPROM_MAGIC: c_uint = 0x9051;

extern "C" {
    pub fn netdev_priv(_arg: ndev) -> return;
}
