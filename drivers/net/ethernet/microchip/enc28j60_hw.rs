//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/enc28j60_hw.h
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
// enc28j60_hw.h: EDTP FrameThrower style enc28j60 registers
//
// $Id: enc28j60_hw.h,v 1.9 2007/12/14 11:59:16 claudio Exp $
//
// ENC28J60 Control Registers
// Control register definitions are a combination of address,
// bank number, and Ethernet/MAC/PHY indicator bits.
// - Register address	(bits 0-4)
// - Bank number	(bits 5-6)
// - MAC/MII indicator	(bit 7)
//
pub const ADDR_MASK: c_uint = 0x1F;
pub const BANK_MASK: c_uint = 0x60;
pub const SPRD_MASK: c_uint = 0x80;
// All-bank registers
pub const EIE: c_uint = 0x1B;
pub const EIR: c_uint = 0x1C;
pub const ESTAT: c_uint = 0x1D;
pub const ECON2: c_uint = 0x1E;
pub const ECON1: c_uint = 0x1F;
// Bank 0 registers

// Bank 1 registers

// Bank 2 registers

// #define MACON2	(0x01|0x40|SPRD_MASK)

// Bank 3 registers

// PHY registers
pub const PHCON1: c_uint = 0x00;
pub const PHSTAT1: c_uint = 0x01;
pub const PHHID1: c_uint = 0x02;
pub const PHHID2: c_uint = 0x03;
pub const PHCON2: c_uint = 0x10;
pub const PHSTAT2: c_uint = 0x11;
pub const PHIE: c_uint = 0x12;
pub const PHIR: c_uint = 0x13;
pub const PHLCON: c_uint = 0x14;
// ENC28J60 EIE Register Bit Definitions
pub const EIE_INTIE: c_uint = 0x80;
pub const EIE_PKTIE: c_uint = 0x40;
pub const EIE_DMAIE: c_uint = 0x20;
pub const EIE_LINKIE: c_uint = 0x10;
pub const EIE_TXIE: c_uint = 0x08;
// #define EIE_WOLIE	0x04 (reserved)
pub const EIE_TXERIE: c_uint = 0x02;
pub const EIE_RXERIE: c_uint = 0x01;
// ENC28J60 EIR Register Bit Definitions
pub const EIR_PKTIF: c_uint = 0x40;
pub const EIR_DMAIF: c_uint = 0x20;
pub const EIR_LINKIF: c_uint = 0x10;
pub const EIR_TXIF: c_uint = 0x08;
// #define EIR_WOLIF	0x04 (reserved)
pub const EIR_TXERIF: c_uint = 0x02;
pub const EIR_RXERIF: c_uint = 0x01;
// ENC28J60 ESTAT Register Bit Definitions
pub const ESTAT_INT: c_uint = 0x80;
pub const ESTAT_LATECOL: c_uint = 0x10;
pub const ESTAT_RXBUSY: c_uint = 0x04;
pub const ESTAT_TXABRT: c_uint = 0x02;
pub const ESTAT_CLKRDY: c_uint = 0x01;
// ENC28J60 ECON2 Register Bit Definitions
pub const ECON2_AUTOINC: c_uint = 0x80;
pub const ECON2_PKTDEC: c_uint = 0x40;
pub const ECON2_PWRSV: c_uint = 0x20;
pub const ECON2_VRPS: c_uint = 0x08;
// ENC28J60 ECON1 Register Bit Definitions
pub const ECON1_TXRST: c_uint = 0x80;
pub const ECON1_RXRST: c_uint = 0x40;
pub const ECON1_DMAST: c_uint = 0x20;
pub const ECON1_CSUMEN: c_uint = 0x10;
pub const ECON1_TXRTS: c_uint = 0x08;
pub const ECON1_RXEN: c_uint = 0x04;
pub const ECON1_BSEL1: c_uint = 0x02;
pub const ECON1_BSEL0: c_uint = 0x01;
// ENC28J60 MACON1 Register Bit Definitions
pub const MACON1_LOOPBK: c_uint = 0x10;
pub const MACON1_TXPAUS: c_uint = 0x08;
pub const MACON1_RXPAUS: c_uint = 0x04;
pub const MACON1_PASSALL: c_uint = 0x02;
pub const MACON1_MARXEN: c_uint = 0x01;
// ENC28J60 MACON2 Register Bit Definitions
pub const MACON2_MARST: c_uint = 0x80;
pub const MACON2_RNDRST: c_uint = 0x40;
pub const MACON2_MARXRST: c_uint = 0x08;
pub const MACON2_RFUNRST: c_uint = 0x04;
pub const MACON2_MATXRST: c_uint = 0x02;
pub const MACON2_TFUNRST: c_uint = 0x01;
// ENC28J60 MACON3 Register Bit Definitions
pub const MACON3_PADCFG2: c_uint = 0x80;
pub const MACON3_PADCFG1: c_uint = 0x40;
pub const MACON3_PADCFG0: c_uint = 0x20;
pub const MACON3_TXCRCEN: c_uint = 0x10;
pub const MACON3_PHDRLEN: c_uint = 0x08;
pub const MACON3_HFRMLEN: c_uint = 0x04;
pub const MACON3_FRMLNEN: c_uint = 0x02;
pub const MACON3_FULDPX: c_uint = 0x01;
// ENC28J60 MICMD Register Bit Definitions
pub const MICMD_MIISCAN: c_uint = 0x02;
pub const MICMD_MIIRD: c_uint = 0x01;
// ENC28J60 MISTAT Register Bit Definitions
pub const MISTAT_NVALID: c_uint = 0x04;
pub const MISTAT_SCAN: c_uint = 0x02;
pub const MISTAT_BUSY: c_uint = 0x01;
// ENC28J60 ERXFCON Register Bit Definitions
pub const ERXFCON_UCEN: c_uint = 0x80;
pub const ERXFCON_ANDOR: c_uint = 0x40;
pub const ERXFCON_CRCEN: c_uint = 0x20;
pub const ERXFCON_PMEN: c_uint = 0x10;
pub const ERXFCON_MPEN: c_uint = 0x08;
pub const ERXFCON_HTEN: c_uint = 0x04;
pub const ERXFCON_MCEN: c_uint = 0x02;
pub const ERXFCON_BCEN: c_uint = 0x01;
// ENC28J60 PHY PHCON1 Register Bit Definitions
pub const PHCON1_PRST: c_uint = 0x8000;
pub const PHCON1_PLOOPBK: c_uint = 0x4000;
pub const PHCON1_PPWRSV: c_uint = 0x0800;
pub const PHCON1_PDPXMD: c_uint = 0x0100;
// ENC28J60 PHY PHSTAT1 Register Bit Definitions
pub const PHSTAT1_PFDPX: c_uint = 0x1000;
pub const PHSTAT1_PHDPX: c_uint = 0x0800;
pub const PHSTAT1_LLSTAT: c_uint = 0x0004;
pub const PHSTAT1_JBSTAT: c_uint = 0x0002;
// ENC28J60 PHY PHSTAT2 Register Bit Definitions

// ENC28J60 PHY PHCON2 Register Bit Definitions
pub const PHCON2_FRCLINK: c_uint = 0x4000;
pub const PHCON2_TXDIS: c_uint = 0x2000;
pub const PHCON2_JABBER: c_uint = 0x0400;
pub const PHCON2_HDLDIS: c_uint = 0x0100;
// ENC28J60 PHY PHIE Register Bit Definitions

// ENC28J60 PHY PHIR Register Bit Definitions

// ENC28J60 Packet Control Byte Bit Definitions
pub const PKTCTRL_PHUGEEN: c_uint = 0x08;
pub const PKTCTRL_PPADEN: c_uint = 0x04;
pub const PKTCTRL_PCRCEN: c_uint = 0x02;
pub const PKTCTRL_POVERRIDE: c_uint = 0x01;
// ENC28J60 Transmit Status Vector
pub const TSV_TXBYTECNT: c_int = 0;
pub const TSV_TXCOLLISIONCNT: c_int = 16;
pub const TSV_TXCRCERROR: c_int = 20;
pub const TSV_TXLENCHKERROR: c_int = 21;
pub const TSV_TXLENOUTOFRANGE: c_int = 22;
pub const TSV_TXDONE: c_int = 23;
pub const TSV_TXMULTICAST: c_int = 24;
pub const TSV_TXBROADCAST: c_int = 25;
pub const TSV_TXPACKETDEFER: c_int = 26;
pub const TSV_TXEXDEFER: c_int = 27;
pub const TSV_TXEXCOLLISION: c_int = 28;
pub const TSV_TXLATECOLLISION: c_int = 29;
pub const TSV_TXGIANT: c_int = 30;
pub const TSV_TXUNDERRUN: c_int = 31;
pub const TSV_TOTBYTETXONWIRE: c_int = 32;
pub const TSV_TXCONTROLFRAME: c_int = 48;
pub const TSV_TXPAUSEFRAME: c_int = 49;
pub const TSV_BACKPRESSUREAPP: c_int = 50;
pub const TSV_TXVLANTAGFRAME: c_int = 51;
pub const TSV_SIZE: c_int = 7;

// ENC28J60 Receive Status Vector
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
pub const RSV_SIZE: c_int = 6;

// SPI operation codes
pub const ENC28J60_READ_CTRL_REG: c_uint = 0x00;
pub const ENC28J60_READ_BUF_MEM: c_uint = 0x3A;
pub const ENC28J60_WRITE_CTRL_REG: c_uint = 0x40;
pub const ENC28J60_WRITE_BUF_MEM: c_uint = 0x7A;
pub const ENC28J60_BIT_FIELD_SET: c_uint = 0x80;
pub const ENC28J60_BIT_FIELD_CLR: c_uint = 0xA0;
pub const ENC28J60_SOFT_RESET: c_uint = 0xFF;
// buffer boundaries applied to internal 8K ram
// entire available packet buffer space is allocated.
// Give TX buffer space for one full ethernet frame (~1500 bytes)
// receive buffer gets the rest
pub const TXSTART_INIT: c_uint = 0x1A00;
pub const TXEND_INIT: c_uint = 0x1FFF;
// Put RX buffer at 0 as suggested by the Errata datasheet
pub const RXSTART_INIT: c_uint = 0x0000;
pub const RXEND_INIT: c_uint = 0x19FF;
// maximum ethernet frame length
pub const MAX_FRAMELEN: c_int = 1518;
// Preferred half duplex: LEDA: Link status LEDB: Rx/Tx activity
pub const ENC28J60_LAMPS_MODE: c_uint = 0x3476;
