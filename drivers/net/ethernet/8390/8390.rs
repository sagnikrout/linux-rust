//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/8390/8390.h
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


// SPDX-License-Identifier: GPL-1.0+
// Generic NS8390 register definitions.
// This file is part of Donald Becker's 8390 drivers, and is distributed
// under the same license. Auto-loading of 8390.o only in v2.2 - Paul G.
// Some of these names and comments originated from the Crynwr
// packet drivers, which are distributed under the GPL.
//

// Macro flag: #define _8390_h

// The 8390 specific per-packet-header format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e8390_pkt_hdr {
    pub /: *mut *mut unsigned char status; / status,
    pub /: *mut *mut unsigned char next; / pointer to next packet.,
    pub /: *mut *mut unsigned short count; / header + packet length in bytes,
}

extern "C" {
    pub fn ei_poll(dev: *mut net_device);
}
extern "C" {
    pub fn eip_poll(dev: *mut net_device);
}

// Without I/O delay - non ISA or later chips
extern "C" {
    pub fn NS8390_init(dev: *mut net_device, startp: c_int);
}
extern "C" {
    pub fn ei_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ei_close(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ei_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ei_tx_timeout(dev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn ei_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn ei_set_multicast_list(dev: *mut net_device);
}
extern "C" {
    pub fn __alloc_ei_netdev(_arg: 0) -> return;
}
// With I/O delay form
extern "C" {
    pub fn NS8390p_init(dev: *mut net_device, startp: c_int);
}
extern "C" {
    pub fn eip_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn eip_close(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn eip_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn eip_tx_timeout(dev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn eip_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn eip_set_multicast_list(dev: *mut net_device);
}
extern "C" {
    pub fn __alloc_eip_netdev(_arg: 0) -> return;
}
// You have one of these per-board
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ei_device {
    pub name: *const c_char,
    pub dev): *mut *mut void (reset_8390)(struct net_device,
    pub ring_page): *mut *mut e8390_pkt_hdr hdr, int,
    pub start_page): *const *const unsigned char buf, int,
    pub ring_offset): *mut *mut sk_buff skb, int,
    pub rmem_start: c_ulong,
    pub rmem_end: c_ulong,
    pub mem: *mut void __iomem,
    pub mcfilter: [c_uchar; 8],
    pub open:1: unsigned,
    pub 8-bit): *mut *mut unsigned word16:1; / We have the 16-bit (vs,
// version of the card.
//
    pub NOT: *mut *mut unsigned bigendian:1; / 16-bit big endian mode. Do,
// set this on random 8390 clones!
//
    pub /: *mut *mut unsigned txing:1; / Transmit Active,
    pub /: *mut *mut unsigned irqlock:1; / 8390's intrs disabled when '1'.,
    pub /: *mut *mut unsigned dmaing:1; / Remote DMA Active,
    pub stop_page: unsigned char tx_start_page, rx_start_page,,
    pub /: *mut *mut unsigned char current_page; / Read pointer in buffer,
    pub /: *mut *mut unsigned char interface_num; / Net port (AUI, 10bT.) to use.,
    pub /: *mut *mut unsigned char txqueue; / Tx Packet buffer queue length.,
    pub /: *mut *mut short tx1, tx2; / Packet lengths for ping-pong tx.,
    pub /: *mut *mut short lasttx; / Alpha version consistency check.,
    pub /: *mut *mut unsigned char reg0; / Register '0' in a WD8013,
    pub /: *mut *mut unsigned char reg5; / Register '5' in a WD8013,
    pub /: *mut *mut unsigned char saved_irq; / Original dev->irq value.,
    pub /: *mut *mut *mut u32 reg_offset; / Register mapping table,
    pub /: *mut *mut spinlock_t page_lock; / Page register locks,
    pub /: *mut *mut unsigned long priv; / Private field to store bus IDs etc.,
    pub /: *mut *mut u32 msg_enable; / debug message level,

    pub /: *mut *mut unsigned char rxcr_base; / default value for RXCR,

}

// The maximum number of 8390 interrupt service routines called per IRQ.
pub const MAX_SERVICE: c_int = 12;
// The maximum time waited (in jiffies) before assuming a Tx failed. (20ms)

// Some generic ethernet register configurations.
pub const E8390_TX_IRQ_MASK: c_uint = 0xa	/* For register EN0_ISR */;
pub const E8390_RX_IRQ_MASK: c_uint = 0x5;

// EN0_RXCR: broadcasts, no multicast,errors
pub const E8390_RXCONFIG: c_uint = 0x4;
// EN0_RXCR: Accept no packets
pub const E8390_RXOFF: c_uint = 0x20;

// EN0_TXCR: Normal transmit mode
pub const E8390_TXCONFIG: c_uint = 0x00;
// EN0_TXCR: Transmitter off
pub const E8390_TXOFF: c_uint = 0x02;
// Register accessed at EN_CMD, the 8390 base addr.
pub const E8390_STOP: c_uint = 0x01	/* Stop and reset the chip */;
pub const E8390_START: c_uint = 0x02	/* Start the chip, clear reset */;
pub const E8390_TRANS: c_uint = 0x04	/* Transmit a frame */;
pub const E8390_RREAD: c_uint = 0x08	/* Remote read */;
pub const E8390_RWRITE: c_uint = 0x10	/* Remote write  */;
pub const E8390_NODMA: c_uint = 0x20	/* Remote DMA */;
pub const E8390_PAGE0: c_uint = 0x00	/* Select page chip registers */;
pub const E8390_PAGE1: c_uint = 0x40	/* using the two high-order bits */;
pub const E8390_PAGE2: c_uint = 0x80	/* Page 3 is invalid. */;
// Only generate indirect loads given a machine that needs them.
// - removed AMIGA_PCMCIA from this list, handled as ISA io now
// - the _p for generates no delay by default 8390p.c overrides this.
//

// Page 0 register offsets.

// Bits in EN0_ISR - Interrupt status register
pub const ENISR_RX: c_uint = 0x01	/* Receiver, no error */;
pub const ENISR_TX: c_uint = 0x02	/* Transmitter, no error */;
pub const ENISR_RX_ERR: c_uint = 0x04	/* Receiver, with error */;
pub const ENISR_TX_ERR: c_uint = 0x08	/* Transmitter, with error */;
pub const ENISR_OVER: c_uint = 0x10	/* Receiver overwrote the ring */;
pub const ENISR_COUNTERS: c_uint = 0x20	/* Counters need emptying */;
pub const ENISR_RDC: c_uint = 0x40	/* remote dma complete */;
pub const ENISR_RESET: c_uint = 0x80	/* Reset completed */;
pub const ENISR_ALL: c_uint = 0x3f	/* Interrupts we will enable */;
// Bits in EN0_DCFG - Data config register
pub const ENDCFG_WTS: c_uint = 0x01	/* word transfer mode selection */;
pub const ENDCFG_BOS: c_uint = 0x02	/* byte order selection */;
// Page 1 register offsets.

// Bits in received packet status byte and EN0_RSR
pub const ENRSR_RXOK: c_uint = 0x01	/* Received a good packet */;
pub const ENRSR_CRC: c_uint = 0x02	/* CRC error */;
pub const ENRSR_FAE: c_uint = 0x04	/* frame alignment error */;
pub const ENRSR_FO: c_uint = 0x08	/* FIFO overrun */;
pub const ENRSR_MPA: c_uint = 0x10	/* missed pkt */;
pub const ENRSR_PHY: c_uint = 0x20	/* physical/multicast address */;
pub const ENRSR_DIS: c_uint = 0x40	/* receiver disable. set in monitor mode */;
pub const ENRSR_DEF: c_uint = 0x80	/* deferring */;
// Transmitted packet status, EN0_TSR.
pub const ENTSR_PTX: c_uint = 0x01	/* Packet transmitted without error */;
pub const ENTSR_ND: c_uint = 0x02	/* The transmit wasn't deferred. */;
pub const ENTSR_COL: c_uint = 0x04	/* The transmit collided at least once. */;
pub const ENTSR_ABT: c_uint = 0x08  /* The transmit collided 16 times, and was deferred. */;
pub const ENTSR_CRS: c_uint = 0x10	/* The carrier sense was lost. */;
pub const ENTSR_FU: c_uint = 0x20  /* A "FIFO underrun" occurred during transmit. */;
pub const ENTSR_CDH: c_uint = 0x40	/* The collision detect "heartbeat" signal was lost. */;
pub const ENTSR_OWC: c_uint = 0x80  /* There was an out-of-window collision. */;
