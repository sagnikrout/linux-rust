//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/faraday/ftgmac100.h
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
// Faraday FTGMAC100 Gigabit Ethernet
//
// (C) Copyright 2009-2011 Faraday Technology
// Po-Yu Chuang <ratbert@faraday-tech.com>
//
pub const FTGMAC100_OFFSET_ISR: c_uint = 0x00;
pub const FTGMAC100_OFFSET_IER: c_uint = 0x04;
pub const FTGMAC100_OFFSET_MAC_MADR: c_uint = 0x08;
pub const FTGMAC100_OFFSET_MAC_LADR: c_uint = 0x0c;
pub const FTGMAC100_OFFSET_MAHT0: c_uint = 0x10;
pub const FTGMAC100_OFFSET_MAHT1: c_uint = 0x14;
pub const FTGMAC100_OFFSET_NPTXPD: c_uint = 0x18;
pub const FTGMAC100_OFFSET_RXPD: c_uint = 0x1c;
pub const FTGMAC100_OFFSET_NPTXR_BADR: c_uint = 0x20;
pub const FTGMAC100_OFFSET_RXR_BADR: c_uint = 0x24;
pub const FTGMAC100_OFFSET_HPTXPD: c_uint = 0x28;
pub const FTGMAC100_OFFSET_HPTXR_BADR: c_uint = 0x2c;
pub const FTGMAC100_OFFSET_ITC: c_uint = 0x30;
pub const FTGMAC100_OFFSET_APTC: c_uint = 0x34;
pub const FTGMAC100_OFFSET_DBLAC: c_uint = 0x38;
pub const FTGMAC100_OFFSET_DMAFIFOS: c_uint = 0x3c;
pub const FTGMAC100_OFFSET_REVR: c_uint = 0x40;
pub const FTGMAC100_OFFSET_FEAR: c_uint = 0x44;
pub const FTGMAC100_OFFSET_TPAFCR: c_uint = 0x48;
pub const FTGMAC100_OFFSET_RBSR: c_uint = 0x4c;
pub const FTGMAC100_OFFSET_MACCR: c_uint = 0x50;
pub const FTGMAC100_OFFSET_MACSR: c_uint = 0x54;
pub const FTGMAC100_OFFSET_TM: c_uint = 0x58;
pub const FTGMAC100_OFFSET_PHYCR: c_uint = 0x60;
pub const FTGMAC100_OFFSET_PHYDATA: c_uint = 0x64;
pub const FTGMAC100_OFFSET_FCR: c_uint = 0x68;
pub const FTGMAC100_OFFSET_BPR: c_uint = 0x6c;
pub const FTGMAC100_OFFSET_WOLCR: c_uint = 0x70;
pub const FTGMAC100_OFFSET_WOLSR: c_uint = 0x74;
pub const FTGMAC100_OFFSET_WFCRC: c_uint = 0x78;
pub const FTGMAC100_OFFSET_WFBM1: c_uint = 0x80;
pub const FTGMAC100_OFFSET_WFBM2: c_uint = 0x84;
pub const FTGMAC100_OFFSET_WFBM3: c_uint = 0x88;
pub const FTGMAC100_OFFSET_WFBM4: c_uint = 0x8c;
pub const FTGMAC100_OFFSET_NPTXR_PTR: c_uint = 0x90;
pub const FTGMAC100_OFFSET_HPTXR_PTR: c_uint = 0x94;
pub const FTGMAC100_OFFSET_RXR_PTR: c_uint = 0x98;
pub const FTGMAC100_OFFSET_TX: c_uint = 0xa0;
pub const FTGMAC100_OFFSET_TX_MCOL_SCOL: c_uint = 0xa4;
pub const FTGMAC100_OFFSET_TX_ECOL_FAIL: c_uint = 0xa8;
pub const FTGMAC100_OFFSET_TX_LCOL_UND: c_uint = 0xac;
pub const FTGMAC100_OFFSET_RX: c_uint = 0xb0;
pub const FTGMAC100_OFFSET_RX_BC: c_uint = 0xb4;
pub const FTGMAC100_OFFSET_RX_MC: c_uint = 0xb8;
pub const FTGMAC100_OFFSET_RX_PF_AEP: c_uint = 0xbc;
pub const FTGMAC100_OFFSET_RX_RUNT: c_uint = 0xc0;
pub const FTGMAC100_OFFSET_RX_CRCER_FTL: c_uint = 0xc4;
pub const FTGMAC100_OFFSET_RX_COL_LOST: c_uint = 0xc8;
//
// Interrupt status register & interrupt enable register
//

// Interrupts we care about in NAPI mode

// Normal RX/TX interrupts, enabled when NAPI off

// All the interrupts we care about

//
// Interrupt timer control register
//

//
// Automatic polling timer control register
//

//
// DMA burst length and arbitration control register
//

//
// DMA FIFO status register
//

//
// Feature Register
//

//
// Receive buffer size register
//

//
// MAC control register
//

//
// test mode control register
//

//
// PHY control register
//
pub const FTGMAC100_PHYCR_MDC_CYCTHR_MASK: c_uint = 0x3f;

//
// PHY data register
//

//
// Flow control register
//

//
// Transmit descriptor, aligned to 16 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftgmac100_txdes {
    pub /: *mut *mut __le32 txdes0; / Control & status bits,
    pub /: *mut *mut __le32 txdes1; / Irq, checksum and vlan control,
    pub /: *mut *mut __le32 txdes2; / Reserved,
    pub /: *mut *mut __le32 txdes3; / DMA buffer address,
// C attribute field omitted

//
// Receive descriptor, aligned to 16 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftgmac100_rxdes {
    pub /: *mut *mut __le32 rxdes0; / Control & status bits,
    pub /: *mut *mut __le32 rxdes1; / Checksum and vlan status,
    pub /: *mut *mut __le32 rxdes2; / length/type on AST2500,
    pub /: *mut *mut __le32 rxdes3; / DMA buffer address,
// C attribute field omitted
pub const FTGMAC100_RXDES0_VDBC: c_uint = 0x3fff;

// Errors we care about for dropping packets

pub const FTGMAC100_RXDES1_VLANTAG_CI: c_uint = 0xffff;

