//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/micrel/ks8851.h
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
// drivers/net/ethernet/micrel/ks8851.h
//
// Copyright 2009 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// KS8851 register definitions
//

pub const KS_CCR: c_uint = 0x08;

// MAC address registers

pub const KS_MARL: c_uint = 0x10;
pub const KS_MARM: c_uint = 0x12;
pub const KS_MARH: c_uint = 0x14;
pub const KS_OBCR: c_uint = 0x20;

pub const KS_EEPCR: c_uint = 0x22;

pub const KS_MBIR: c_uint = 0x24;

pub const KS_GRR: c_uint = 0x26;

pub const KS_WFCR: c_uint = 0x2A;

pub const KS_WF0CRC0: c_uint = 0x30;
pub const KS_WF0CRC1: c_uint = 0x32;
pub const KS_WF0BM0: c_uint = 0x34;
pub const KS_WF0BM1: c_uint = 0x36;
pub const KS_WF0BM2: c_uint = 0x38;
pub const KS_WF0BM3: c_uint = 0x3A;
pub const KS_WF1CRC0: c_uint = 0x40;
pub const KS_WF1CRC1: c_uint = 0x42;
pub const KS_WF1BM0: c_uint = 0x44;
pub const KS_WF1BM1: c_uint = 0x46;
pub const KS_WF1BM2: c_uint = 0x48;
pub const KS_WF1BM3: c_uint = 0x4A;
pub const KS_WF2CRC0: c_uint = 0x50;
pub const KS_WF2CRC1: c_uint = 0x52;
pub const KS_WF2BM0: c_uint = 0x54;
pub const KS_WF2BM1: c_uint = 0x56;
pub const KS_WF2BM2: c_uint = 0x58;
pub const KS_WF2BM3: c_uint = 0x5A;
pub const KS_WF3CRC0: c_uint = 0x60;
pub const KS_WF3CRC1: c_uint = 0x62;
pub const KS_WF3BM0: c_uint = 0x64;
pub const KS_WF3BM1: c_uint = 0x66;
pub const KS_WF3BM2: c_uint = 0x68;
pub const KS_WF3BM3: c_uint = 0x6A;
pub const KS_TXCR: c_uint = 0x70;

pub const KS_TXSR: c_uint = 0x72;

pub const KS_RXCR1: c_uint = 0x74;

pub const KS_RXCR2: c_uint = 0x76;

pub const KS_TXMIR: c_uint = 0x78;
pub const KS_RXFHSR: c_uint = 0x7C;

pub const KS_RXFHBCR: c_uint = 0x7E;

pub const KS_TXQCR: c_uint = 0x80;

pub const KS_RXQCR: c_uint = 0x82;

pub const KS_TXFDPR: c_uint = 0x84;

pub const KS_RXFDPR: c_uint = 0x86;

pub const KS_RXDTTR: c_uint = 0x8C;
pub const KS_RXDBCTR: c_uint = 0x8E;
pub const KS_IER: c_uint = 0x90;
pub const KS_ISR: c_uint = 0x92;

pub const KS_RXFCTR: c_uint = 0x9C;
pub const KS_RXFC: c_uint = 0x9D;

pub const KS_TXNTFSR: c_uint = 0x9E;
pub const KS_MAHTR0: c_uint = 0xA0;
pub const KS_MAHTR1: c_uint = 0xA2;
pub const KS_MAHTR2: c_uint = 0xA4;
pub const KS_MAHTR3: c_uint = 0xA6;
pub const KS_FCLWR: c_uint = 0xB0;
pub const KS_FCHWR: c_uint = 0xB2;
pub const KS_FCOWR: c_uint = 0xB4;
pub const KS_CIDER: c_uint = 0xC0;
pub const CIDER_ID: c_uint = 0x8870;

pub const KS_CGCR: c_uint = 0xC6;
pub const KS_IACR: c_uint = 0xC8;

pub const KS_IADLR: c_uint = 0xD0;
pub const KS_IAHDR: c_uint = 0xD2;
pub const KS_PMECR: c_uint = 0xD4;

// Standard MII PHY data
pub const KS_P1MBCR: c_uint = 0xE4;
pub const KS_P1MBSR: c_uint = 0xE6;
pub const KS_PHY1ILR: c_uint = 0xE8;
pub const KS_PHY1IHR: c_uint = 0xEA;
pub const KS_P1ANAR: c_uint = 0xEC;
pub const KS_P1ANLPR: c_uint = 0xEE;
pub const KS_P1SCLMD: c_uint = 0xF4;
pub const KS_P1CR: c_uint = 0xF6;

pub const KS_P1SR: c_uint = 0xF8;

// TX Frame control

//
// struct ks8851_rxctrl - KS8851 driver rx control
// @mchash: Multicast hash-table data.
// @rxcr1: KS_RXCR1 register setting
// @rxcr2: KS_RXCR2 register setting
//
// Representation of the settings needs to control the receive filtering
// such as the multicast hash-filter and the receive register settings. This
// is used to make the job of working out if the receive settings change and
// then issuing the new settings to the worker that will send the necessary
// commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ks8851_rxctrl {
    pub mchash: [u16; 4],
    pub rxcr1: u16,
    pub rxcr2: u16,
}

//
// union ks8851_tx_hdr - tx header data
// @txb: The header as bytes
// @txw: The header as 16bit, little-endian words
//
// A dual representation of the tx header data to allow
// access to individual bytes, and to allow 16bit accesses
// with 16bit alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ks8851_tx_hdr {
    pub txb: [u8; 6],
    pub txw: [__le16; 3],
}

//
// struct ks8851_net - KS8851 driver private data
// @netdev: The network device we're bound to
// @statelock: Lock on this structure for tx list.
// @mii: The MII state information for the mii calls.
// @rxctrl: RX settings for @rxctrl_work.
// @rxctrl_work: Work queue for updating RX mode and multicast lists
// @txq: Queue of packets for transmission.
// @txh: Space for generating packet TX header in DMA-able data
// @rxd: Space for receiving SPI data, in DMA-able space.
// @txd: Space for transmitting SPI data, in DMA-able space.
// @msg_enable: The message flags controlling driver output (see ethtool).
// @tx_space: Free space in the hardware TX buffer (cached copy of KS_TXMIR).
// @queued_len: Space required in hardware TX buffer for queued packets in txq.
// @fid: Incrementing frame id tag.
// @rc_ier: Cached copy of KS_IER.
// @rc_ccr: Cached copy of KS_CCR.
// @rc_rxqcr: Cached copy of KS_RXQCR.
// @eeprom: 93CX6 EEPROM state for accessing on-board EEPROM.
// @vdd_reg:	Optional regulator supplying the chip
// @vdd_io: Optional digital power supply for IO
// @gpio: Optional reset_n gpio
// @mii_bus: Pointer to MII bus structure
// @lock: Bus access lock callback
// @unlock: Bus access unlock callback
// @rdreg16: 16bit register read callback
// @wrreg16: 16bit register write callback
// @rdfifo: FIFO read callback
// @wrfifo: FIFO write callback
// @start_xmit: start_xmit() implementation callback
// @flush_tx_work: flush_tx_work() implementation callback
//
// The @statelock is used to protect information in the structure which may
// need to be accessed via several sources, such as the network driver layer
// or one of the work queues.
//
// We align the buffers we may use for rx/tx to ensure that if the SPI driver
// wants to DMA map them, it will not have any problems with data the driver
// modifies.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ks8851_net {
    pub netdev: *mut net_device,
    pub statelock: spinlock_t,
    pub ____cacheline_aligned: ks8851_tx_hdr txh,
    pub rxd: [u8; 8],
    pub txd: [u8; 8],
    pub ____cacheline_aligned: u32 msg_enable,
    pub tx_space: u16,
    pub fid: u8,
    pub rc_ier: u16,
    pub rc_rxqcr: u16,
    pub rc_ccr: u16,
    pub mii: mii_if_info,
    pub rxctrl: ks8851_rxctrl,
    pub rxctrl_work: work_struct,
    pub txq: sk_buff_head,
    pub queued_len: c_uint,
    pub eeprom: eeprom_93cx6,
    pub vdd_reg: *mut regulator,
    pub vdd_io: *mut regulator,
    pub gpio: *mut gpio_desc,
    pub mii_bus: *mut mii_bus,
    pub ks): *mut *mut void (lock)(struct ks8851_net,
    pub ks): *mut *mut void (unlock)(struct ks8851_net,
    pub reg): c_uint,
    pub val): unsigned int reg, unsigned int,
    pub len): c_uint,
    pub irq): *mut *mut sk_buff txp, bool,
    pub dev): *mut net_device,
    pub ks): *mut *mut void (flush_tx_work)(struct ks8851_net,
}

extern "C" {
    pub fn ks8851_remove_common(dev: *mut device);
}
extern "C" {
    pub fn ks8851_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ks8851_resume(dev: *mut device) -> c_int;
}
//
// ks8851_done_tx - update and then free skbuff after transmitting
// @ks: The device state
// @txb: The buffer transmitted
//
