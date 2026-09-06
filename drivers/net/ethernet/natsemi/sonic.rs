//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/natsemi/sonic.h
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
// Header file for sonic.c
//
// (C) Waldorf Electronics, Germany
// Written by Andreas Busse
//
// NOTE: most of the structure definitions here are endian dependent.
// If you want to use this driver on big endian machines, the data
// and pad structure members must be exchanged. Also, the structures
// need to be changed accordingly to the bus size.
//
// 981229 MSch:	did just that for the 68k Mac port (32 bit, big endian)
//
// 990611 David Huggins-Daines <dhd@debian.org>: This machine abstraction
// does not cope with 16-bit bus sizes very well.  Therefore I have
// rewritten it with ugly macros and evil inlines.
//
// 050625 Finn Thain: introduced more 32-bit cards and dhd's support
// for 16-bit cards (from the mac68k project).
//
// SONIC register offsets
//
pub const SONIC_CMD: c_uint = 0x00;
pub const SONIC_DCR: c_uint = 0x01;
pub const SONIC_RCR: c_uint = 0x02;
pub const SONIC_TCR: c_uint = 0x03;
pub const SONIC_IMR: c_uint = 0x04;
pub const SONIC_ISR: c_uint = 0x05;
pub const SONIC_UTDA: c_uint = 0x06;
pub const SONIC_CTDA: c_uint = 0x07;
pub const SONIC_URDA: c_uint = 0x0d;
pub const SONIC_CRDA: c_uint = 0x0e;
pub const SONIC_EOBC: c_uint = 0x13;
pub const SONIC_URRA: c_uint = 0x14;
pub const SONIC_RSA: c_uint = 0x15;
pub const SONIC_REA: c_uint = 0x16;
pub const SONIC_RRP: c_uint = 0x17;
pub const SONIC_RWP: c_uint = 0x18;
pub const SONIC_RSC: c_uint = 0x2b;
pub const SONIC_CEP: c_uint = 0x21;
pub const SONIC_CAP2: c_uint = 0x22;
pub const SONIC_CAP1: c_uint = 0x23;
pub const SONIC_CAP0: c_uint = 0x24;
pub const SONIC_CE: c_uint = 0x25;
pub const SONIC_CDP: c_uint = 0x26;
pub const SONIC_CDC: c_uint = 0x27;
pub const SONIC_WT0: c_uint = 0x29;
pub const SONIC_WT1: c_uint = 0x2a;
pub const SONIC_SR: c_uint = 0x28;
// test-only registers
pub const SONIC_TPS: c_uint = 0x08;
pub const SONIC_TFC: c_uint = 0x09;
pub const SONIC_TSA0: c_uint = 0x0a;
pub const SONIC_TSA1: c_uint = 0x0b;
pub const SONIC_TFS: c_uint = 0x0c;
pub const SONIC_CRBA0: c_uint = 0x0f;
pub const SONIC_CRBA1: c_uint = 0x10;
pub const SONIC_RBWC0: c_uint = 0x11;
pub const SONIC_RBWC1: c_uint = 0x12;
pub const SONIC_TTDA: c_uint = 0x20;
pub const SONIC_MDT: c_uint = 0x2f;
pub const SONIC_TRBA0: c_uint = 0x19;
pub const SONIC_TRBA1: c_uint = 0x1a;
pub const SONIC_TBWC0: c_uint = 0x1b;
pub const SONIC_TBWC1: c_uint = 0x1c;
pub const SONIC_LLFA: c_uint = 0x1f;
pub const SONIC_ADDR0: c_uint = 0x1d;
pub const SONIC_ADDR1: c_uint = 0x1e;
//
// Error counters
//
pub const SONIC_CRCT: c_uint = 0x2c;
pub const SONIC_FAET: c_uint = 0x2d;
pub const SONIC_MPT: c_uint = 0x2e;
pub const SONIC_DCR2: c_uint = 0x3f;
//
// SONIC command bits
//
pub const SONIC_CR_LCAM: c_uint = 0x0200;
pub const SONIC_CR_RRRA: c_uint = 0x0100;
pub const SONIC_CR_RST: c_uint = 0x0080;
pub const SONIC_CR_ST: c_uint = 0x0020;
pub const SONIC_CR_STP: c_uint = 0x0010;
pub const SONIC_CR_RXEN: c_uint = 0x0008;
pub const SONIC_CR_RXDIS: c_uint = 0x0004;
pub const SONIC_CR_TXP: c_uint = 0x0002;
pub const SONIC_CR_HTX: c_uint = 0x0001;

//
// SONIC data configuration bits
//
pub const SONIC_DCR_EXBUS: c_uint = 0x8000;
pub const SONIC_DCR_LBR: c_uint = 0x2000;
pub const SONIC_DCR_PO1: c_uint = 0x1000;
pub const SONIC_DCR_PO0: c_uint = 0x0800;
pub const SONIC_DCR_SBUS: c_uint = 0x0400;
pub const SONIC_DCR_USR1: c_uint = 0x0200;
pub const SONIC_DCR_USR0: c_uint = 0x0100;
pub const SONIC_DCR_WC1: c_uint = 0x0080;
pub const SONIC_DCR_WC0: c_uint = 0x0040;
pub const SONIC_DCR_DW: c_uint = 0x0020;
pub const SONIC_DCR_BMS: c_uint = 0x0010;
pub const SONIC_DCR_RFT1: c_uint = 0x0008;
pub const SONIC_DCR_RFT0: c_uint = 0x0004;
pub const SONIC_DCR_TFT1: c_uint = 0x0002;
pub const SONIC_DCR_TFT0: c_uint = 0x0001;
//
// Constants for the SONIC receive control register.
//
pub const SONIC_RCR_ERR: c_uint = 0x8000;
pub const SONIC_RCR_RNT: c_uint = 0x4000;
pub const SONIC_RCR_BRD: c_uint = 0x2000;
pub const SONIC_RCR_PRO: c_uint = 0x1000;
pub const SONIC_RCR_AMC: c_uint = 0x0800;
pub const SONIC_RCR_LB1: c_uint = 0x0400;
pub const SONIC_RCR_LB0: c_uint = 0x0200;
pub const SONIC_RCR_MC: c_uint = 0x0100;
pub const SONIC_RCR_BC: c_uint = 0x0080;
pub const SONIC_RCR_LPKT: c_uint = 0x0040;
pub const SONIC_RCR_CRS: c_uint = 0x0020;
pub const SONIC_RCR_COL: c_uint = 0x0010;
pub const SONIC_RCR_CRCR: c_uint = 0x0008;
pub const SONIC_RCR_FAER: c_uint = 0x0004;
pub const SONIC_RCR_LBK: c_uint = 0x0002;
pub const SONIC_RCR_PRX: c_uint = 0x0001;
pub const SONIC_RCR_LB_OFF: c_int = 0;

// default RCR setup

//
// SONIC Transmit Control register bits
//
pub const SONIC_TCR_PINTR: c_uint = 0x8000;
pub const SONIC_TCR_POWC: c_uint = 0x4000;
pub const SONIC_TCR_CRCI: c_uint = 0x2000;
pub const SONIC_TCR_EXDIS: c_uint = 0x1000;
pub const SONIC_TCR_EXD: c_uint = 0x0400;
pub const SONIC_TCR_DEF: c_uint = 0x0200;
pub const SONIC_TCR_NCRS: c_uint = 0x0100;
pub const SONIC_TCR_CRLS: c_uint = 0x0080;
pub const SONIC_TCR_EXC: c_uint = 0x0040;
pub const SONIC_TCR_OWC: c_uint = 0x0020;
pub const SONIC_TCR_PMB: c_uint = 0x0008;
pub const SONIC_TCR_FU: c_uint = 0x0004;
pub const SONIC_TCR_BCM: c_uint = 0x0002;
pub const SONIC_TCR_PTX: c_uint = 0x0001;
pub const SONIC_TCR_DEFAULT: c_uint = 0x0000;
//
// Constants for the SONIC_INTERRUPT_MASK and
// SONIC_INTERRUPT_STATUS registers.
//
pub const SONIC_INT_BR: c_uint = 0x4000;
pub const SONIC_INT_HBL: c_uint = 0x2000;
pub const SONIC_INT_LCD: c_uint = 0x1000;
pub const SONIC_INT_PINT: c_uint = 0x0800;
pub const SONIC_INT_PKTRX: c_uint = 0x0400;
pub const SONIC_INT_TXDN: c_uint = 0x0200;
pub const SONIC_INT_TXER: c_uint = 0x0100;
pub const SONIC_INT_TC: c_uint = 0x0080;
pub const SONIC_INT_RDE: c_uint = 0x0040;
pub const SONIC_INT_RBE: c_uint = 0x0020;
pub const SONIC_INT_RBAE: c_uint = 0x0010;
pub const SONIC_INT_CRC: c_uint = 0x0008;
pub const SONIC_INT_FAE: c_uint = 0x0004;
pub const SONIC_INT_MP: c_uint = 0x0002;
pub const SONIC_INT_RFO: c_uint = 0x0001;
//
// The interrupts we allow.
//

pub const SONIC_EOL: c_uint = 0x0001;
pub const CAM_DESCRIPTORS: c_int = 16;
// Offsets in the various DMA buffers accessed by the SONIC
pub const SONIC_BITMODE16: c_int = 0;
pub const SONIC_BITMODE32: c_int = 1;

// Note!  These are all measured in bus-size units, so use SONIC_BUS_SCALE
pub const SIZEOF_SONIC_RR: c_int = 4;
pub const SONIC_RR_BUFADR_L: c_int = 0;
pub const SONIC_RR_BUFADR_H: c_int = 1;
pub const SONIC_RR_BUFSIZE_L: c_int = 2;
pub const SONIC_RR_BUFSIZE_H: c_int = 3;
pub const SIZEOF_SONIC_RD: c_int = 7;
pub const SONIC_RD_STATUS: c_int = 0;
pub const SONIC_RD_PKTLEN: c_int = 1;
pub const SONIC_RD_PKTPTR_L: c_int = 2;
pub const SONIC_RD_PKTPTR_H: c_int = 3;
pub const SONIC_RD_SEQNO: c_int = 4;
pub const SONIC_RD_LINK: c_int = 5;
pub const SONIC_RD_IN_USE: c_int = 6;
pub const SIZEOF_SONIC_TD: c_int = 8;
pub const SONIC_TD_STATUS: c_int = 0;
pub const SONIC_TD_CONFIG: c_int = 1;
pub const SONIC_TD_PKTSIZE: c_int = 2;
pub const SONIC_TD_FRAG_COUNT: c_int = 3;
pub const SONIC_TD_FRAG_PTR_L: c_int = 4;
pub const SONIC_TD_FRAG_PTR_H: c_int = 5;
pub const SONIC_TD_FRAG_SIZE: c_int = 6;
pub const SONIC_TD_LINK: c_int = 7;
pub const SIZEOF_SONIC_CD: c_int = 4;
pub const SONIC_CD_ENTRY_POINTER: c_int = 0;
pub const SONIC_CD_CAP0: c_int = 1;
pub const SONIC_CD_CAP1: c_int = 2;
pub const SONIC_CD_CAP2: c_int = 3;

//
// Some tunables for the buffer areas. Power of 2 is required
// the current driver uses one receive buffer for each descriptor.
//
// MSch: use more buffer space for the slow m68k Macs!
//

// Again, measured in bus size units!

// Information that need to be kept for each board.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sonic_local {
// Bus size.  0 == 16 bits, 1 == 32 bits.
    pub dma_bitmode: c_int,
// Register offset within the longword (independent of endianness,
    pub reg_offset: c_int,
    pub descriptors: *mut c_void,
// Crud.  These areas have to be within the same 64K.  Therefore
    pub /: *mut *mut *mut void cda; / CAM descriptor area,
    pub /: *mut *mut *mut void tda; / Transmit descriptor area,
    pub /: *mut *mut *mut void rra; / Receive resource area,
    pub /: *mut *mut *mut void rda; / Receive descriptor area,
    pub /: *mut *mut *mut sk_buff volatile rx_skb[SONIC_NUM_RRS]; / packets to be received,
    pub /: *mut *mut *mut sk_buff volatile tx_skb[SONIC_NUM_TDS]; / packets to be transmitted,
    pub /: *mut *mut unsigned int tx_len[SONIC_NUM_TDS]; / lengths of tx DMA mappings,
// Logical DMA addresses on MIPS, bus addresses on m68k
// (so "laddr" is a bit misleading)
    pub descriptors_laddr: dma_addr_t,
    pub /: *mut *mut u32 cda_laddr; / logical DMA address of CDA,
    pub /: *mut *mut u32 tda_laddr; / logical DMA address of TDA,
    pub /: *mut *mut u32 rra_laddr; / logical DMA address of RRA,
    pub /: *mut *mut u32 rda_laddr; / logical DMA address of RDA,
    pub /: *mut *mut dma_addr_t rx_laddr[SONIC_NUM_RRS]; / logical DMA addresses of rx skbuffs,
    pub /: *mut *mut dma_addr_t tx_laddr[SONIC_NUM_TDS]; / logical DMA addresses of tx skbuffs,
    pub cur_rx: c_uint,
    pub /: *mut *mut unsigned int cur_tx; / first unacked transmit packet,
    pub eol_rx: c_uint,
    pub /: *mut *mut unsigned int eol_tx; / last unacked transmit packet,
    pub msg_enable: c_int,
    pub /: *mut *mut *mut device device; / generic device,
    pub stats: net_device_stats,
    pub lock: spinlock_t,
}

// Index to functions, as function prototypes.
extern "C" {
    pub fn sonic_open(dev: *mut net_device) -> static int;
}
extern "C" {
    pub fn sonic_send_packet(skb: *mut sk_buff, dev: *mut net_device) -> static int;
}
extern "C" {
    pub fn sonic_interrupt(irq: c_int, dev_id: *mut c_void) -> static irqreturn_t;
}
extern "C" {
    pub fn sonic_rx(dev: *mut net_device) -> static void;
}
extern "C" {
    pub fn sonic_close(dev: *mut net_device) -> static int;
}
extern "C" {
    pub fn sonic_multicast_list(dev: *mut net_device) -> static void;
}
extern "C" {
    pub fn sonic_init(dev: *mut net_device, may_sleep: bool) -> static int;
}
extern "C" {
    pub fn sonic_tx_timeout(dev: *mut net_device, txqueue: c_uint) -> static void;
}
extern "C" {
    pub fn sonic_msg_init(dev: *mut net_device) -> static void;
}
extern "C" {
    pub fn sonic_alloc_descriptors(dev: *mut net_device) -> static int;
}
// Internal inlines for reading/writing DMA buffers.  Note that bus
// OpenBSD calls this "SWO".  I'd like to think that sonic_buf_put()

extern "C" {
    pub fn __raw_readw(1: *mut *mut base + (offset  2) +) -> return;
}

extern "C" {
    pub fn __raw_readw(0: *mut *mut base + (offset  2) +) -> return;
}

extern "C" {
    pub fn __raw_readw(0: *mut *mut base + (offset  1) +) -> return;
}
// Inlines that you should actually use for reading/writing DMA buffers
extern "C" {
    pub fn sonic_buf_get(_arg: lp->cda, _arg: lp->dma_bitmode, _arg: SONIC_CDA_CAM_ENABLE) -> return;
}
extern "C" {
    pub fn SONIC_BUS_SCALE(_arg: lp->dma_bitmode) -> *mut *mut entry  SIZEOF_SONIC_RR;
}
