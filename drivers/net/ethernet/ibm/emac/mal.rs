//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/emac/mal.h
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
// drivers/net/ethernet/ibm/emac/mal.h
//
// Memory Access Layer (MAL) support
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//
// Based on the arch/ppc version of the driver:
//
// Copyright (c) 2004, 2005 Zultys Technologies.
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
//
// Based on original work by
// Armin Kuster <akuster@mvista.com>
// Copyright 2002 MontaVista Softare Inc.
//
// There are some variations on the MAL, we express them in this driver as
// MAL Version 1 and 2 though that doesn't match any IBM terminology.
//
// We call MAL 1 the version in 405GP, 405GPR, 405EP, 440EP, 440GR and
// NP405H.
//
// We call MAL 2 the version in 440GP, 440GX, 440SP, 440SPE and Axon
//
// The driver expects a "version" property in the emac node containing
// a number 1 or 2. New device-trees for EMAC capable platforms are thus
// required to include that when porting to arch/powerpc.
//
// MALx DCR registers
pub const MAL_CFG: c_uint = 0x00;
pub const MAL_CFG_SR: c_uint = 0x80000000;
pub const MAL_CFG_PLBB: c_uint = 0x00004000;
pub const MAL_CFG_OPBBL: c_uint = 0x00000080;
pub const MAL_CFG_EOPIE: c_uint = 0x00000004;
pub const MAL_CFG_LEA: c_uint = 0x00000002;
pub const MAL_CFG_SD: c_uint = 0x00000001;
// MAL V1 CFG bits
pub const MAL1_CFG_PLBP_MASK: c_uint = 0x00c00000;
pub const MAL1_CFG_PLBP_10: c_uint = 0x00800000;
pub const MAL1_CFG_GA: c_uint = 0x00200000;
pub const MAL1_CFG_OA: c_uint = 0x00100000;
pub const MAL1_CFG_PLBLE: c_uint = 0x00080000;
pub const MAL1_CFG_PLBT_MASK: c_uint = 0x00078000;

// MAL V2 CFG bits
pub const MAL2_CFG_RPP_MASK: c_uint = 0x00c00000;
pub const MAL2_CFG_RPP_10: c_uint = 0x00800000;
pub const MAL2_CFG_RMBS_MASK: c_uint = 0x00300000;
pub const MAL2_CFG_WPP_MASK: c_uint = 0x000c0000;
pub const MAL2_CFG_WPP_10: c_uint = 0x00080000;
pub const MAL2_CFG_WMBS_MASK: c_uint = 0x00030000;
pub const MAL2_CFG_PLBLE: c_uint = 0x00008000;

pub const MAL_ESR: c_uint = 0x01;
pub const MAL_ESR_EVB: c_uint = 0x80000000;
pub const MAL_ESR_CIDT: c_uint = 0x40000000;
pub const MAL_ESR_CID_MASK: c_uint = 0x3e000000;
pub const MAL_ESR_CID_SHIFT: c_int = 25;
pub const MAL_ESR_DE: c_uint = 0x00100000;
pub const MAL_ESR_OTE: c_uint = 0x00040000;
pub const MAL_ESR_OSE: c_uint = 0x00020000;
pub const MAL_ESR_PEIN: c_uint = 0x00010000;
pub const MAL_ESR_DEI: c_uint = 0x00000010;
pub const MAL_ESR_OTEI: c_uint = 0x00000004;
pub const MAL_ESR_OSEI: c_uint = 0x00000002;
pub const MAL_ESR_PBEI: c_uint = 0x00000001;
// MAL V1 ESR bits
pub const MAL1_ESR_ONE: c_uint = 0x00080000;
pub const MAL1_ESR_ONEI: c_uint = 0x00000008;
// MAL V2 ESR bits
pub const MAL2_ESR_PTE: c_uint = 0x00800000;
pub const MAL2_ESR_PRE: c_uint = 0x00400000;
pub const MAL2_ESR_PWE: c_uint = 0x00200000;
pub const MAL2_ESR_PTEI: c_uint = 0x00000080;
pub const MAL2_ESR_PREI: c_uint = 0x00000040;
pub const MAL2_ESR_PWEI: c_uint = 0x00000020;
pub const MAL_IER: c_uint = 0x02;
// MAL IER bits
pub const MAL_IER_DE: c_uint = 0x00000010;
pub const MAL_IER_OTE: c_uint = 0x00000004;
pub const MAL_IER_OE: c_uint = 0x00000002;
pub const MAL_IER_PE: c_uint = 0x00000001;
// PLB read/write/timeout errors
pub const MAL_IER_PTE: c_uint = 0x00000080;
pub const MAL_IER_PRE: c_uint = 0x00000040;
pub const MAL_IER_PWE: c_uint = 0x00000020;

pub const MAL_TXCASR: c_uint = 0x04;
pub const MAL_TXCARR: c_uint = 0x05;
pub const MAL_TXEOBISR: c_uint = 0x06;
pub const MAL_TXDEIR: c_uint = 0x07;
pub const MAL_RXCASR: c_uint = 0x10;
pub const MAL_RXCARR: c_uint = 0x11;
pub const MAL_RXEOBISR: c_uint = 0x12;
pub const MAL_RXDEIR: c_uint = 0x13;

// In reality MAL can handle TX buffers up to 4095 bytes long,
// but this isn't a good round number :) 		 --ebs
//
pub const MAL_MAX_TX_SIZE: c_int = 4080;
pub const MAL_MAX_RX_SIZE: c_int = 4080;
extern "C" {
    pub fn DIV_ROUND_UP(_arg: len, _arg: MAL_MAX_TX_SIZE) -> return;
}

// MAL Buffer Descriptor structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mal_descriptor {
    pub /: *mut *mut u16 ctrl; / MAL / Commac status control bits,
    pub /: *mut *mut u16 data_len; / Max length is 4K-1 (12 bits),
    pub /: *mut *mut u32 data_ptr; / pointer to actual data buffer,
}

// the following defines are for the MadMAL status and control registers.
// MADMAL transmit and receive status/control bits
pub const MAL_RX_CTRL_EMPTY: c_uint = 0x8000;
pub const MAL_RX_CTRL_WRAP: c_uint = 0x4000;
pub const MAL_RX_CTRL_CM: c_uint = 0x2000;
pub const MAL_RX_CTRL_LAST: c_uint = 0x1000;
pub const MAL_RX_CTRL_FIRST: c_uint = 0x0800;
pub const MAL_RX_CTRL_INTR: c_uint = 0x0400;

pub const MAL_TX_CTRL_READY: c_uint = 0x8000;
pub const MAL_TX_CTRL_WRAP: c_uint = 0x4000;
pub const MAL_TX_CTRL_CM: c_uint = 0x2000;
pub const MAL_TX_CTRL_LAST: c_uint = 0x1000;
pub const MAL_TX_CTRL_INTR: c_uint = 0x0400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mal_commac_ops {
    pub dev): *mut *mut void (poll_tx) (void,
    pub budget): *mut *mut *mut int (poll_rx) (void dev, int,
    pub dev): *mut *mut int (peek_rx) (void,
    pub dev): *mut *mut void (rxde) (void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mal_commac {
    pub ops: *mut mal_commac_ops,
    pub dev: *mut c_void,
    pub poll_list: list_head,
    pub flags: c_long,
pub const MAL_COMMAC_RX_STOPPED: c_int = 0;
pub const MAL_COMMAC_POLL_DISABLED: c_int = 1;
    pub tx_chan_mask: u32,
    pub rx_chan_mask: u32,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mal_instance {
    pub version: c_int,
    pub dcr_host: dcr_host_t,
    pub /: *mut *mut int num_tx_chans; / Number of TX channels,
    pub /: *mut *mut int num_rx_chans; / Number of RX channels,
    pub /: *mut *mut int txeob_irq; / TX End Of Buffer IRQ,
    pub /: *mut *mut int rxeob_irq; / RX End Of Buffer IRQ,
    pub /: *mut *mut int txde_irq; / TX Descriptor Error IRQ,
    pub /: *mut *mut int rxde_irq; / RX Descriptor Error IRQ,
    pub /: *mut *mut int serr_irq; / MAL System Error IRQ,
    pub poll_list: list_head,
    pub napi: napi_struct,
    pub list: list_head,
    pub tx_chan_mask: u32,
    pub rx_chan_mask: u32,
    pub bd_dma: dma_addr_t,
    pub bd_virt: *mut mal_descriptor,
    pub ofdev: *mut platform_device,
    pub index: c_int,
    pub lock: spinlock_t,
    pub dummy_dev: *mut net_device,
    pub features: c_uint,
}

extern "C" {
    pub fn dcr_read(_arg: mal->dcr_host, _arg: reg) -> return;
}
// Features of various MAL implementations
// Set if you have interrupt coalescing and you have to clear the SDR
// register for TXEOB and RXEOB interrupts to work
//
pub const MAL_FTR_CLEAR_ICINTSTAT: c_uint = 0x00000001;
// Set if your MAL has SERR, TXDE, and RXDE OR'd into a single UIC
// interrupt
//
pub const MAL_FTR_COMMON_ERR_INT: c_uint = 0x00000002;

// Register MAL devices
extern "C" {
    pub fn mal_init() -> c_int;
}
extern "C" {
    pub fn mal_exit();
}
extern "C" {
    pub fn mal_set_rcbs(mal: *mut mal_instance, channel: c_int, size: c_ulong) -> c_int;
}
// Returns BD ring offset for a particular channel
//
extern "C" {
    pub fn mal_tx_bd_offset(mal: *mut mal_instance, channel: c_int) -> c_int;
}
extern "C" {
    pub fn mal_rx_bd_offset(mal: *mut mal_instance, channel: c_int) -> c_int;
}
extern "C" {
    pub fn mal_enable_tx_channel(mal: *mut mal_instance, channel: c_int);
}
extern "C" {
    pub fn mal_disable_tx_channel(mal: *mut mal_instance, channel: c_int);
}
extern "C" {
    pub fn mal_enable_rx_channel(mal: *mut mal_instance, channel: c_int);
}
extern "C" {
    pub fn mal_disable_rx_channel(mal: *mut mal_instance, channel: c_int);
}
extern "C" {
    pub fn mal_poll_disable(mal: *mut mal_instance, commac: *mut mal_commac);
}
extern "C" {
    pub fn mal_poll_enable(mal: *mut mal_instance, commac: *mut mal_commac);
}
// Add/remove EMAC to/from MAL polling list
extern "C" {
    pub fn mal_poll_add(mal: *mut mal_instance, commac: *mut mal_commac);
}
extern "C" {
    pub fn mal_poll_del(mal: *mut mal_instance, commac: *mut mal_commac);
}
// Ethtool MAL registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mal_regs {
    pub tx_count: u32,
    pub rx_count: u32,
    pub cfg: u32,
    pub esr: u32,
    pub ier: u32,
    pub tx_casr: u32,
    pub tx_carr: u32,
    pub tx_eobisr: u32,
    pub tx_deir: u32,
    pub rx_casr: u32,
    pub rx_carr: u32,
    pub rx_eobisr: u32,
    pub rx_deir: u32,
    pub tx_ctpr: [u32; 32],
    pub rx_ctpr: [u32; 32],
    pub rcbs: [u32; 32],
}

extern "C" {
    pub fn mal_get_regs_len(mal: *mut mal_instance) -> c_int;
}
