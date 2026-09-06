//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/7990.h
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
// 7990.h -- LANCE ethernet IC generic routines.
// This is an attempt to separate out the bits of various ethernet
// drivers that are common because they all use the AMD 7990 LANCE
// (Local Area Network Controller for Ethernet) chip.
//
// Copyright (C) 05/1998 Peter Maydell <pmaydell@chiark.greenend.org.uk>
//
// Most of this stuff was obtained by looking at other LANCE drivers,
// in particular a2065.[ch]. The AMD C-LANCE datasheet was also helpful.
//
// The lance only has two register locations. We communicate mostly via memory.

// Transmit/receive ring definitions.
// We allow the specific drivers to override these defaults if they want to.
// NB: according to lance.c, increasing the number of buffers is a waste
// of space and reduces the chance that an upper layer will be able to
// reorder queued Tx packets based on priority. [Clearly there is a minimum
// limit too: too small and we drop rx packets and can't tx at full speed.]
// 4+4 seems to be the usual setting; the atarilance driver uses 3 and 5.
//
// Blast! This won't work. The problem is that we can't specify a default
// setting because that would cause the lance_init_block struct to be
// too long (and overflow the RAM on shared-memory cards like the HP LANCE.
//

pub const LANCE_LOG_TX_BUFFERS: c_int = 1;
pub const LANCE_LOG_RX_BUFFERS: c_int = 3;

// Each receive buffer is described by a receive message descriptor (RMD)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_rx_desc {
    pub /: *mut *mut volatile unsigned short rmd0; / low address of packet,
    pub /: *mut *mut volatile unsigned char rmd1_bits; / descriptor bits,
    pub /: *mut *mut volatile unsigned char rmd1_hadr; / high address of packet,
    pub (negative)!: *mut *mut volatile short length; / This length is 2s complement,
// Buffer length
    pub /: *mut *mut volatile unsigned short mblength; / Actual number of bytes received,
}

// Ditto for TMD:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_tx_desc {
    pub /: *mut *mut volatile unsigned short tmd0; / low address of packet,
    pub /: *mut *mut volatile unsigned char tmd1_bits; / descriptor bits,
    pub /: *mut *mut volatile unsigned char tmd1_hadr; / high address of packet,
    pub /: *mut *mut volatile short length; / Length is 2s complement (negative)!,
    pub misc: volatile unsigned short,
}

// There are three memory structures accessed by the LANCE:
// the initialization block, the receive and transmit descriptor rings,
// and the data buffers themselves. In fact we might as well put the
// init block,the Tx and Rx rings and the buffers together in memory:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_init_block {
    pub /: *mut *mut volatile unsigned short mode; / Pre-set mode (reg. 15),
    pub /: *mut *mut volatile unsigned char phys_addr[6]; / Physical ethernet address,
    pub /: *mut *mut volatile unsigned filter[2]; / Multicast filter (64 bits),
// Receive and transmit ring base, along with extra bits.
    pub /: *mut *mut volatile unsigned short rx_ptr; / receive descriptor addr,
    pub /: *mut *mut volatile unsigned short rx_len; / receive len and high addr,
    pub /: *mut *mut volatile unsigned short tx_ptr; / transmit descriptor addr,
    pub /: *mut *mut volatile unsigned short tx_len; / transmit len and high addr,
// The Tx and Rx ring entries must be aligned on 8-byte boundaries.
// This will be true if this whole struct is 8-byte aligned.
//
    pub btx_ring: [volatile struct lance_tx_desc; TX_RING_SIZE],
    pub brx_ring: [volatile struct lance_rx_desc; RX_RING_SIZE],
    pub tx_buf: [volatile char; TX_RING_SIZE][TX_BUFF_SIZE],
    pub rx_buf: [volatile char; RX_RING_SIZE][RX_BUFF_SIZE],
// we use this just to make the struct big enough that we can move its startaddr
// in order to force alignment to an eight byte boundary.
//
}

// This is where we keep all the stuff the driver needs to know about.
// I'm definitely unhappy about the mechanism for allowing specific
// drivers to add things...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lance_private {
    pub name: *const c_char,
    pub base: c_ulong,
    pub /: *mut *mut *mut volatile struct lance_init_block init_block; / CPU address of RAM,
    pub /: *mut *mut *mut volatile struct lance_init_block lance_init_block; / LANCE address of RAM,
    pub tx_new: int rx_new,,
    pub tx_old: int rx_old,,
    pub lance_log_tx_bufs: int lance_log_rx_bufs,,
    pub tx_ring_mod_mask: int rx_ring_mod_mask,,
    pub /: *mut *mut int tpe; / TPE is selected,
    pub /: *mut *mut int auto_select; / cable-selection is by carrier,
    pub busmaster_regval: c_ushort,
    pub /: *mut *mut unsigned int irq; / IRQ to register,
// This is because the HP LANCE is disgusting and you have to check
// a DIO-specific register every time you read/write the LANCE regs :-<
// [could we get away with making these some sort of macro?]
//
    pub short): *mut *mut *mut void (writerap)(void , unsigned,
    pub short): *mut *mut *mut void (writerdp)(void , unsigned,
    pub ): *mut *mut unsigned short (readrdp)(void,
    pub devlock: spinlock_t,
    pub tx_full: c_char,
}

//
// Am7990 Control and Status Registers
//
pub const LE_CSR0: c_uint = 0x0000	/* LANCE Controller Status */;
pub const LE_CSR1: c_uint = 0x0001	/* IADR[15:0] (bit0==0 ie word aligned) */;
pub const LE_CSR2: c_uint = 0x0002	/* IADR[23:16] (high bits reserved) */;
pub const LE_CSR3: c_uint = 0x0003	/* Misc */;
//
// Bit definitions for CSR0 (LANCE Controller Status)
//
pub const LE_C0_ERR: c_uint = 0x8000	/* Error = BABL | CERR | MISS | MERR */;
pub const LE_C0_BABL: c_uint = 0x4000	/* Babble: Transmitted too many bits */;
pub const LE_C0_CERR: c_uint = 0x2000	/* No Heartbeat (10BASE-T) */;
pub const LE_C0_MISS: c_uint = 0x1000	/* Missed Frame (no rx buffer to put it in) */;
pub const LE_C0_MERR: c_uint = 0x0800	/* Memory Error */;
pub const LE_C0_RINT: c_uint = 0x0400	/* Receive Interrupt */;
pub const LE_C0_TINT: c_uint = 0x0200	/* Transmit Interrupt */;
pub const LE_C0_IDON: c_uint = 0x0100	/* Initialization Done */;
pub const LE_C0_INTR: c_uint = 0x0080	/* Interrupt Flag;
pub const LE_C0_INEA: c_uint = 0x0040	/* Interrupt Enable */;
pub const LE_C0_RXON: c_uint = 0x0020	/* Receive On */;
pub const LE_C0_TXON: c_uint = 0x0010	/* Transmit On */;
pub const LE_C0_TDMD: c_uint = 0x0008	/* Transmit Demand */;
pub const LE_C0_STOP: c_uint = 0x0004	/* Stop */;
pub const LE_C0_STRT: c_uint = 0x0002	/* Start */;
pub const LE_C0_INIT: c_uint = 0x0001	/* Initialize */;
//
// Bit definitions for CSR3
//
pub const LE_C3_BSWP: c_uint = 0x0004	/* Byte Swap (on for big endian byte order) */;
pub const LE_C3_ACON: c_uint = 0x0002	/* ALE Control (on for active low ALE) */;
pub const LE_C3_BCON: c_uint = 0x0001	/* Byte Control */;
//
// Mode Flags
//
pub const LE_MO_PROM: c_uint = 0x8000	/* Promiscuous Mode */;
// these next ones 0x4000 -- 0x0080 are not available on the LANCE 7990,
// but they are in NetBSD's am7990.h, presumably for backwards-compatible chips
//
pub const LE_MO_DRCVBC: c_uint = 0x4000	/* disable receive broadcast */;
pub const LE_MO_DRCVPA: c_uint = 0x2000	/* disable physical address detection */;
pub const LE_MO_DLNKTST: c_uint = 0x1000	/* disable link status */;
pub const LE_MO_DAPC: c_uint = 0x0800	/* disable automatic polarity correction */;
pub const LE_MO_MENDECL: c_uint = 0x0400	/* MENDEC loopback mode */;
pub const LE_MO_LRTTSEL: c_uint = 0x0200	/* lower RX threshold / TX mode selection */;
pub const LE_MO_PSEL1: c_uint = 0x0100	/* port selection bit1 */;
pub const LE_MO_PSEL0: c_uint = 0x0080	/* port selection bit0 */;
// and this one is from the C-LANCE data sheet...
pub const LE_MO_EMBA: c_uint = 0x0080	/* Enable Modified Backoff Algorithm;
pub const LE_MO_INTL: c_uint = 0x0040	/* Internal Loopback */;
pub const LE_MO_DRTY: c_uint = 0x0020	/* Disable Retry */;
pub const LE_MO_FCOLL: c_uint = 0x0010	/* Force Collision */;
pub const LE_MO_DXMTFCS: c_uint = 0x0008	/* Disable Transmit CRC */;
pub const LE_MO_LOOP: c_uint = 0x0004	/* Loopback Enable */;
pub const LE_MO_DTX: c_uint = 0x0002	/* Disable Transmitter */;
pub const LE_MO_DRX: c_uint = 0x0001	/* Disable Receiver */;
//
// Receive Flags
//
pub const LE_R1_OWN: c_uint = 0x80	/* LANCE owns the descriptor */;
pub const LE_R1_ERR: c_uint = 0x40	/* Error */;
pub const LE_R1_FRA: c_uint = 0x20	/* Framing Error */;
pub const LE_R1_OFL: c_uint = 0x10	/* Overflow Error */;
pub const LE_R1_CRC: c_uint = 0x08	/* CRC Error */;
pub const LE_R1_BUF: c_uint = 0x04	/* Buffer Error */;
pub const LE_R1_SOP: c_uint = 0x02	/* Start of Packet */;
pub const LE_R1_EOP: c_uint = 0x01	/* End of Packet */;
pub const LE_R1_POK: c_uint = 0x03	/* Packet is complete: SOP + EOP */;
//
// Transmit Flags
//
pub const LE_T1_OWN: c_uint = 0x80	/* LANCE owns the descriptor */;
pub const LE_T1_ERR: c_uint = 0x40	/* Error */;
pub const LE_T1_RES: c_uint = 0x20	/* Reserved, LANCE writes this with a zero */;
pub const LE_T1_EMORE: c_uint = 0x10	/* More than one retry needed */;
pub const LE_T1_EONE: c_uint = 0x08	/* One retry needed */;
pub const LE_T1_EDEF: c_uint = 0x04	/* Deferred */;
pub const LE_T1_SOP: c_uint = 0x02	/* Start of Packet */;
pub const LE_T1_EOP: c_uint = 0x01	/* End of Packet */;
pub const LE_T1_POK: c_uint = 0x03	/* Packet is complete: SOP + EOP */;
//
// Error Flags
//
pub const LE_T3_BUF: c_uint = 0x8000	/* Buffer Error */;
pub const LE_T3_UFL: c_uint = 0x4000	/* Underflow Error */;
pub const LE_T3_LCOL: c_uint = 0x1000	/* Late Collision */;
pub const LE_T3_CLOS: c_uint = 0x0800	/* Loss of Carrier */;
pub const LE_T3_RTY: c_uint = 0x0400	/* Retry Error */;
pub const LE_T3_TDR: c_uint = 0x03ff	/* Time Domain Reflectometry */;
// Miscellaneous useful macros

// The LANCE only uses 24 bit addresses. This does the obvious thing.

// Now the prototypes we export
extern "C" {
    pub fn lance_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn lance_close(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn lance_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn lance_set_multicast(dev: *mut net_device);
}
extern "C" {
    pub fn lance_tx_timeout(dev: *mut net_device, txqueue: c_uint);
}

extern "C" {
    pub fn lance_poll(dev: *mut net_device);
}

