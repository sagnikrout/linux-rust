//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/faraday/ftmac100.h
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
// Faraday FTMAC100 10/100 Ethernet
//
// (C) Copyright 2009-2011 Faraday Technology
// Po-Yu Chuang <ratbert@faraday-tech.com>
//
pub const FTMAC100_OFFSET_ISR: c_uint = 0x00;
pub const FTMAC100_OFFSET_IMR: c_uint = 0x04;
pub const FTMAC100_OFFSET_MAC_MADR: c_uint = 0x08;
pub const FTMAC100_OFFSET_MAC_LADR: c_uint = 0x0c;
pub const FTMAC100_OFFSET_MAHT0: c_uint = 0x10;
pub const FTMAC100_OFFSET_MAHT1: c_uint = 0x14;
pub const FTMAC100_OFFSET_TXPD: c_uint = 0x18;
pub const FTMAC100_OFFSET_RXPD: c_uint = 0x1c;
pub const FTMAC100_OFFSET_TXR_BADR: c_uint = 0x20;
pub const FTMAC100_OFFSET_RXR_BADR: c_uint = 0x24;
pub const FTMAC100_OFFSET_ITC: c_uint = 0x28;
pub const FTMAC100_OFFSET_APTC: c_uint = 0x2c;
pub const FTMAC100_OFFSET_DBLAC: c_uint = 0x30;
pub const FTMAC100_OFFSET_MACCR: c_uint = 0x88;
pub const FTMAC100_OFFSET_MACSR: c_uint = 0x8c;
pub const FTMAC100_OFFSET_PHYCR: c_uint = 0x90;
pub const FTMAC100_OFFSET_PHYWDATA: c_uint = 0x94;
pub const FTMAC100_OFFSET_FCR: c_uint = 0x98;
pub const FTMAC100_OFFSET_BPR: c_uint = 0x9c;
pub const FTMAC100_OFFSET_TS: c_uint = 0xc4;
pub const FTMAC100_OFFSET_DMAFIFOS: c_uint = 0xc8;
pub const FTMAC100_OFFSET_TM: c_uint = 0xcc;
pub const FTMAC100_OFFSET_TX_MCOL_SCOL: c_uint = 0xd4;
pub const FTMAC100_OFFSET_RPF_AEP: c_uint = 0xd8;
pub const FTMAC100_OFFSET_XM_PG: c_uint = 0xdc;
pub const FTMAC100_OFFSET_RUNT_TLCC: c_uint = 0xe0;
pub const FTMAC100_OFFSET_CRCER_FTL: c_uint = 0xe4;
pub const FTMAC100_OFFSET_RLC_RCC: c_uint = 0xe8;
pub const FTMAC100_OFFSET_BROC: c_uint = 0xec;
pub const FTMAC100_OFFSET_MULCA: c_uint = 0xf0;
pub const FTMAC100_OFFSET_RP: c_uint = 0xf4;
pub const FTMAC100_OFFSET_XP: c_uint = 0xf8;
//
// Interrupt status register & interrupt mask register
//

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
// MAC control register
//

//
// PHY control register
//
pub const FTMAC100_PHYCR_MIIRDATA: c_uint = 0xffff;

//
// PHY write data register
//

//
// Transmit descriptor, aligned to 16 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftmac100_txdes {
    pub txdes0: __le32,
    pub txdes1: __le32,
    pub /: *mut *mut __le32 txdes2; / TXBUF_BADR,
    pub /: *mut *mut unsigned int txdes3; / not used by HW,
// C attribute field omitted

//
// Receive descriptor, aligned to 16 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftmac100_rxdes {
    pub rxdes0: __le32,
    pub rxdes1: __le32,
    pub /: *mut *mut __le32 rxdes2; / RXBUF_BADR,
    pub /: *mut *mut unsigned int rxdes3; / not used by HW,
// C attribute field omitted
pub const FTMAC100_RXDES0_RFL: c_uint = 0x7ff;

