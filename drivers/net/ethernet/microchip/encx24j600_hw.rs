//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/encx24j600_hw.h
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
// encx24j600_hw.h: Register definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encx24j600_context {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub phymap: *mut regmap,
    pub /: *mut *mut mutex mutex; / mutex to protect access to regmap,
    pub bank: c_int,
}

// Single-byte instructions

pub const B0SEL: c_uint = 0xC0		/* Bank 0 Select */;
pub const B1SEL: c_uint = 0xC2		/* Bank 1 Select */;
pub const B2SEL: c_uint = 0xC4		/* Bank 2 Select */;
pub const B3SEL: c_uint = 0xC6		/* Bank 3 Select */;
pub const SETETHRST: c_uint = 0xCA		/* System Reset */;
pub const FCDISABLE: c_uint = 0xE0		/* Flow Control Disable */;
pub const FCSINGLE: c_uint = 0xE2		/* Flow Control Single */;
pub const FCMULTIPLE: c_uint = 0xE4		/* Flow Control Multiple */;
pub const FCCLEAR: c_uint = 0xE6		/* Flow Control Clear */;
pub const SETPKTDEC: c_uint = 0xCC		/* Decrement Packet Counter */;
pub const DMASTOP: c_uint = 0xD2		/* DMA Stop */;
pub const DMACKSUM: c_uint = 0xD8		/* DMA Start Checksum */;
pub const DMACKSUMS: c_uint = 0xDA		/* DMA Start Checksum with Seed */;
pub const DMACOPY: c_uint = 0xDC		/* DMA Start Copy */;
pub const DMACOPYS: c_uint = 0xDE		/* DMA Start Copy and Checksum with Seed */;
pub const SETTXRTS: c_uint = 0xD4		/* Request Packet Transmission */;
pub const ENABLERX: c_uint = 0xE8		/* Enable RX */;
pub const DISABLERX: c_uint = 0xEA		/* Disable RX */;
pub const SETEIE: c_uint = 0xEC		/* Enable Interrupts */;
pub const CLREIE: c_uint = 0xEE		/* Disable Interrupts */;
// Two byte instructions
pub const RBSEL: c_uint = 0xC8		/* Read Bank Select */;
// Three byte instructions
pub const WGPRDPT: c_uint = 0x60		/* Write EGPRDPT */;
pub const RGPRDPT: c_uint = 0x62		/* Read EGPRDPT */;
pub const WRXRDPT: c_uint = 0x64		/* Write ERXRDPT */;
pub const RRXRDPT: c_uint = 0x66		/* Read ERXRDPT */;
pub const WUDARDPT: c_uint = 0x68		/* Write EUDARDPT */;
pub const RUDARDPT: c_uint = 0x6A		/* Read EUDARDPT */;
pub const WGPWRPT: c_uint = 0x6C		/* Write EGPWRPT */;
pub const RGPWRPT: c_uint = 0x6E		/* Read EGPWRPT */;
pub const WRXWRPT: c_uint = 0x70		/* Write ERXWRPT */;
pub const RRXWRPT: c_uint = 0x72		/* Read ERXWRPT */;
pub const WUDAWRPT: c_uint = 0x74		/* Write EUDAWRPT */;
pub const RUDAWRPT: c_uint = 0x76		/* Read EUDAWRPT */;
// n byte instructions
pub const RCRCODE: c_uint = 0x00;
pub const WCRCODE: c_uint = 0x40;
pub const BFSCODE: c_uint = 0x80;
pub const BFCCODE: c_uint = 0xA0;

pub const RCRU: c_uint = 0x20		/* Read Control Register Unbanked */;
pub const WCRU: c_uint = 0x22		/* Write Control Register Unbanked */;

pub const BFSU: c_uint = 0x24		/* Bit Field Set Unbanked */;
pub const BFCU: c_uint = 0x26		/* Bit Field Clear Unbanked */;
pub const RGPDATA: c_uint = 0x28		/* Read EGPDATA */;
pub const WGPDATA: c_uint = 0x2A		/* Write EGPDATA */;
pub const RRXDATA: c_uint = 0x2C		/* Read ERXDATA */;
pub const WRXDATA: c_uint = 0x2E		/* Write ERXDATA */;
pub const RUDADATA: c_uint = 0x30		/* Read EUDADATA */;
pub const WUDADATA: c_uint = 0x32		/* Write EUDADATA */;
pub const SFR_REG_COUNT: c_uint = 0xA0;
// ENC424J600 Control Registers
// Control register definitions are a combination of address
// and bank number
// - Register address (bits 0-4)
// - Bank number (bits 5-6)
//
pub const ADDR_MASK: c_uint = 0x1F;
pub const BANK_MASK: c_uint = 0x60;
pub const BANK_SHIFT: c_int = 5;
// All-bank registers
pub const EUDAST: c_uint = 0x16;
pub const EUDAND: c_uint = 0x18;
pub const ESTAT: c_uint = 0x1A;
pub const EIR: c_uint = 0x1C;
pub const ECON1: c_uint = 0x1E;
// Bank 0 registers

// Bank 1 registers

// Bank 2 registers

// Bank 3 registers

// Unbanked registers

// Register bit definitions
// ESTAT

// EIR

// ECON1

// ETXSTAT

pub const COLCNT_MASK: c_uint = 0xF;
// ERXFCON

// MACON1

// MACON2

// MAIPG
// value of the high byte is given by the reserved bits,
// value of the low byte is recommended setting of the
// IPG parameter.
//
pub const MAIPGH_VAL: c_uint = 0x0C;
pub const MAIPGL_VAL: c_uint = 0x12;
// MIREGADRH

// MIREGADRL
pub const PHREG_MASK: c_uint = 0x1F;
// MICMD

// MISTAT

// ECON2

// EIE

// EIDLED

pub const DEVID_SHIFT: c_int = 5;

pub const REVID_SHIFT: c_int = 0;

// PHY registers
pub const PHCON1: c_uint = 0x00;
pub const PHSTAT1: c_uint = 0x01;
pub const PHANA: c_uint = 0x04;
pub const PHANLPA: c_uint = 0x05;
pub const PHANE: c_uint = 0x06;
pub const PHCON2: c_uint = 0x11;
pub const PHSTAT2: c_uint = 0x1B;
pub const PHSTAT3: c_uint = 0x1F;
// PHCON1

// PHSTAT1

// PHSTAT2

// PHSTAT3

pub const SPDDPX_SHIFT: c_int = 2;

// PHANA
// Default value for PHY initialization
pub const PHANA_DEFAULT: c_uint = 0x05E1;
// PHANE

pub const EUDAST_TEST_VAL: c_uint = 0x1234;
pub const TSV_SIZE: c_int = 7;
pub const ENCX24J600_DEV_ID: c_uint = 0x1;
// Configuration
// Led is on when the link is present and driven low
// temporarily when packet is TX'd or RX'd
//
pub const LED_A_SETTINGS: c_uint = 0xC;
// Led is on if the link is in 100 Mbps mode
pub const LED_B_SETTINGS: c_uint = 0x8;
// maximum ethernet frame length
// Currently not used as a limit anywhere
// (we're using the "huge frame enable" feature of
// enc424j600).
//
pub const MAX_FRAMELEN: c_int = 1518;
// Size in bytes of the receive buffer in enc424j600.
// Must be word aligned (even).
//

// Start of the general purpose area in sram
pub const SRAM_GP_START: c_uint = 0x0;
// SRAM size
pub const SRAM_SIZE: c_uint = 0x6000;
// Start of the receive buffer

pub const RSV_RXLONGEVDROPEV: c_int = 16;
pub const RSV_CARRIEREV: c_int = 18;
pub const RSV_CRCERROR: c_int = 20;
pub const RSV_LENCHECKERR: c_int = 21;
pub const RSV_LENOUTOFRANGE: c_int = 22;
pub const RSV_RXOK: c_int = 23;
pub const RSV_RXMULTICAST: c_int = 24;
pub const RSV_RXBROADCAST: c_int = 25;
pub const RSV_DRIBBLENIBBLE: c_int = 26;
pub const RSV_RXCONTROLFRAME: c_int = 27;
pub const RSV_RXPAUSEFRAME: c_int = 28;
pub const RSV_RXUNKNOWNOPCODE: c_int = 29;
pub const RSV_RXTYPEVLAN: c_int = 30;
pub const RSV_RUNTFILTERMATCH: c_int = 31;
pub const RSV_NOTMEFILTERMATCH: c_int = 32;
pub const RSV_HASHFILTERMATCH: c_int = 33;
pub const RSV_MAGICPKTFILTERMATCH: c_int = 34;
pub const RSV_PTRNMTCHFILTERMATCH: c_int = 35;
pub const RSV_UNICASTFILTERMATCH: c_int = 36;
pub const RSV_SIZE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsv {
    pub next_packet: u16,
    pub len: u16,
    pub rxstat: u32,
}

// Put RX buffer at 0 as suggested by the Errata datasheet

pub const RXEND_INIT: c_uint = 0x5FFF;
extern "C" {
    pub fn regmap_encx24j600_spi_read(context: *mut c_void, reg: u8, data: *mut u8, count: usize) -> c_int;
}
