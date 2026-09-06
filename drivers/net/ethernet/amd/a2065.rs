//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/a2065.h
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
// Amiga Linux/68k A2065 Ethernet Driver
//
// (C) Copyright 1995 by Geert Uytterhoeven <geert@linux-m68k.org>
//
// ---------------------------------------------------------------------------
//
// This program is based on
//
// ariadne.?:	Amiga Linux/68k Ariadne Ethernet Driver
// (C) Copyright 1995 by Geert Uytterhoeven,
// Peter De Schrijver
//
// lance.c:	An AMD LANCE ethernet driver for linux.
// Written 1993-94 by Donald Becker.
//
// Am79C960:	PCnet(tm)-ISA Single-Chip Ethernet Controller
// Advanced Micro Devices
// Publication #16907, Rev. B, Amendment/0, May 1994
//
// ---------------------------------------------------------------------------
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of the Linux
// distribution for more details.
//
// ---------------------------------------------------------------------------
//
// The A2065 is a Zorro-II board made by Commodore/Ameristar. It contains:
//
// - an Am7990 Local Area Network Controller for Ethernet (LANCE) with
// both 10BASE-2 (thin coax) and AUI (DB-15) connectors
//
// Am7990 Local Area Network Controller for Ethernet (LANCE)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_regs {
    pub /: *mut *mut unsigned short rdp; / Register Data Port,
    pub /: *mut *mut unsigned short rap; / Register Address Port,
}

//
// Am7990 Control and Status Registers
//
pub const LE_CSR0: c_uint = 0x0000		/* LANCE Controller Status */;
pub const LE_CSR1: c_uint = 0x0001		/* IADR[15:0] */;
pub const LE_CSR2: c_uint = 0x0002		/* IADR[23:16] */;
pub const LE_CSR3: c_uint = 0x0003		/* Misc */;
//
// Bit definitions for CSR0 (LANCE Controller Status)
//
pub const LE_C0_ERR: c_uint = 0x8000		/* Error */;
pub const LE_C0_BABL: c_uint = 0x4000		/* Babble: Transmitted too many bits */;
pub const LE_C0_CERR: c_uint = 0x2000		/* No Heartbeat (10BASE-T) */;
pub const LE_C0_MISS: c_uint = 0x1000		/* Missed Frame */;
pub const LE_C0_MERR: c_uint = 0x0800		/* Memory Error */;
pub const LE_C0_RINT: c_uint = 0x0400		/* Receive Interrupt */;
pub const LE_C0_TINT: c_uint = 0x0200		/* Transmit Interrupt */;
pub const LE_C0_IDON: c_uint = 0x0100		/* Initialization Done */;
pub const LE_C0_INTR: c_uint = 0x0080		/* Interrupt Flag */;
pub const LE_C0_INEA: c_uint = 0x0040		/* Interrupt Enable */;
pub const LE_C0_RXON: c_uint = 0x0020		/* Receive On */;
pub const LE_C0_TXON: c_uint = 0x0010		/* Transmit On */;
pub const LE_C0_TDMD: c_uint = 0x0008		/* Transmit Demand */;
pub const LE_C0_STOP: c_uint = 0x0004		/* Stop */;
pub const LE_C0_STRT: c_uint = 0x0002		/* Start */;
pub const LE_C0_INIT: c_uint = 0x0001		/* Initialize */;
//
// Bit definitions for CSR3
//
pub const LE_C3_BSWP: c_uint = 0x0004		/* Byte Swap;
pub const LE_C3_ACON: c_uint = 0x0002		/* ALE Control;
pub const LE_C3_BCON: c_uint = 0x0001		/* Byte Control */;
//
// Mode Flags
//
pub const LE_MO_PROM: c_uint = 0x8000		/* Promiscuous Mode */;
pub const LE_MO_INTL: c_uint = 0x0040		/* Internal Loopback */;
pub const LE_MO_DRTY: c_uint = 0x0020		/* Disable Retry */;
pub const LE_MO_FCOLL: c_uint = 0x0010		/* Force Collision */;
pub const LE_MO_DXMTFCS: c_uint = 0x0008		/* Disable Transmit CRC */;
pub const LE_MO_LOOP: c_uint = 0x0004		/* Loopback Enable */;
pub const LE_MO_DTX: c_uint = 0x0002		/* Disable Transmitter */;
pub const LE_MO_DRX: c_uint = 0x0001		/* Disable Receiver */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_rx_desc {
    pub /: *mut *mut unsigned short rmd0; / low address of packet,
    pub /: *mut *mut unsigned char rmd1_bits; / descriptor bits,
    pub /: *mut *mut unsigned char rmd1_hadr; / high address of packet,
    pub (negative)!: *mut *mut short length; / This length is 2s complement,
// Buffer length
//
    pub /: *mut *mut unsigned short mblength; / Aactual number of bytes received,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_tx_desc {
    pub /: *mut *mut unsigned short tmd0; / low address of packet,
    pub /: *mut *mut unsigned char tmd1_bits; / descriptor bits,
    pub /: *mut *mut unsigned char tmd1_hadr; / high address of packet,
    pub /: *mut *mut short length; / Length is 2s complement (negative)!,
    pub misc: c_ushort,
}

//
// Receive Flags
//
pub const LE_R1_OWN: c_uint = 0x80		/* LANCE owns the descriptor */;
pub const LE_R1_ERR: c_uint = 0x40		/* Error */;
pub const LE_R1_FRA: c_uint = 0x20		/* Framing Error */;
pub const LE_R1_OFL: c_uint = 0x10		/* Overflow Error */;
pub const LE_R1_CRC: c_uint = 0x08		/* CRC Error */;
pub const LE_R1_BUF: c_uint = 0x04		/* Buffer Error */;
pub const LE_R1_SOP: c_uint = 0x02		/* Start of Packet */;
pub const LE_R1_EOP: c_uint = 0x01		/* End of Packet */;
pub const LE_R1_POK: c_uint = 0x03		/* Packet is complete: SOP + EOP */;
//
// Transmit Flags
//
pub const LE_T1_OWN: c_uint = 0x80		/* LANCE owns the descriptor */;
pub const LE_T1_ERR: c_uint = 0x40		/* Error */;
pub const LE_T1_RES: c_uint = 0x20		/* Reserved,;
pub const LE_T1_EMORE: c_uint = 0x10		/* More than one retry needed */;
pub const LE_T1_EONE: c_uint = 0x08		/* One retry needed */;
pub const LE_T1_EDEF: c_uint = 0x04		/* Deferred */;
pub const LE_T1_SOP: c_uint = 0x02		/* Start of Packet */;
pub const LE_T1_EOP: c_uint = 0x01		/* End of Packet */;
pub const LE_T1_POK: c_uint = 0x03		/* Packet is complete: SOP + EOP */;
//
// Error Flags
//
pub const LE_T3_BUF: c_uint = 0x8000		/* Buffer Error */;
pub const LE_T3_UFL: c_uint = 0x4000		/* Underflow Error */;
pub const LE_T3_LCOL: c_uint = 0x1000		/* Late Collision */;
pub const LE_T3_CLOS: c_uint = 0x0800		/* Loss of Carrier */;
pub const LE_T3_RTY: c_uint = 0x0400		/* Retry Error */;
pub const LE_T3_TDR: c_uint = 0x03ff		/* Time Domain Reflectometry */;
//
// A2065 Expansion Board Structure
//
pub const A2065_LANCE: c_uint = 0x4000;
pub const A2065_RAM: c_uint = 0x8000;
pub const A2065_RAM_SIZE: c_uint = 0x8000;
