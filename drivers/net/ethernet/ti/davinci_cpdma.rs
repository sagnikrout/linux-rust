//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/davinci_cpdma.h
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
// Texas Instruments CPDMA Driver
//
// Copyright (C) 2010 Texas Instruments
//

pub const CPDMA_EOI_RX_THRESH: c_uint = 0x0;
pub const CPDMA_EOI_RX: c_uint = 0x1;
pub const CPDMA_EOI_TX: c_uint = 0x2;
pub const CPDMA_EOI_MISC: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpdma_params {
    pub dev: *mut device,
    pub dmaregs: *mut void __iomem,
    pub rxcp: *mut *mut *mut *mut void __iomem txhdp, rxhdp, txcp,,
    pub rxfree: *mut *mut void __iomem rxthresh,,
    pub num_chan: c_int,
    pub has_soft_reset: bool,
    pub min_packet_size: c_int,
    pub desc_mem_phys: dma_addr_t,
    pub desc_hw_addr: dma_addr_t,
    pub desc_mem_size: c_int,
    pub desc_align: c_int,
    pub bus_freq_mhz: u32,
    pub descs_pool_size: u32,
//
// Some instances of embedded cpdma controllers have extra control and
// status registers.  The following flag enables access to these
// "extended" registers.
//
    pub has_ext_regs: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpdma_chan_stats {
    pub head_enqueue: u32,
    pub tail_enqueue: u32,
    pub pad_enqueue: u32,
    pub misqueued: u32,
    pub desc_alloc_fail: u32,
    pub pad_alloc_fail: u32,
    pub runt_receive_buff: u32,
    pub runt_transmit_buff: u32,
    pub empty_dequeue: u32,
    pub busy_dequeue: u32,
    pub good_dequeue: u32,
    pub requeue: u32,
    pub teardown_dequeue: u32,
}

extern "C" {
    pub fn void(token: *mut *mut cpdma_handler_fn)(void, len: c_int, status: c_int) -> typedef;
}
extern "C" {
    pub fn cpdma_ctlr_destroy(ctlr: *mut cpdma_ctlr) -> c_int;
}
extern "C" {
    pub fn cpdma_ctlr_start(ctlr: *mut cpdma_ctlr) -> c_int;
}
extern "C" {
    pub fn cpdma_ctlr_stop(ctlr: *mut cpdma_ctlr) -> c_int;
}
extern "C" {
    pub fn cpdma_chan_get_rx_buf_num(chan: *mut cpdma_chan) -> c_int;
}
extern "C" {
    pub fn cpdma_chan_destroy(chan: *mut cpdma_chan) -> c_int;
}
extern "C" {
    pub fn cpdma_chan_start(chan: *mut cpdma_chan) -> c_int;
}
extern "C" {
    pub fn cpdma_chan_stop(chan: *mut cpdma_chan) -> c_int;
}
extern "C" {
    pub fn cpdma_chan_process(chan: *mut cpdma_chan, quota: c_int) -> c_int;
}
extern "C" {
    pub fn cpdma_ctlr_int_ctrl(ctlr: *mut cpdma_ctlr, enable: bool) -> c_int;
}
extern "C" {
    pub fn cpdma_ctlr_eoi(ctlr: *mut cpdma_ctlr, value: u32);
}
extern "C" {
    pub fn cpdma_chan_int_ctrl(chan: *mut cpdma_chan, enable: bool) -> c_int;
}
extern "C" {
    pub fn cpdma_ctrl_rxchs_state(ctlr: *mut cpdma_ctlr) -> u32;
}
extern "C" {
    pub fn cpdma_ctrl_txchs_state(ctlr: *mut cpdma_ctlr) -> u32;
}
extern "C" {
    pub fn cpdma_check_free_tx_desc(chan: *mut cpdma_chan) -> bool;
}
extern "C" {
    pub fn cpdma_chan_set_weight(ch: *mut cpdma_chan, weight: c_int) -> c_int;
}
extern "C" {
    pub fn cpdma_chan_set_rate(ch: *mut cpdma_chan, rate: u32) -> c_int;
}
extern "C" {
    pub fn cpdma_chan_get_rate(ch: *mut cpdma_chan) -> u32;
}
extern "C" {
    pub fn cpdma_chan_get_min_rate(ctlr: *mut cpdma_ctlr) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpdma_control {
    CPDMA_TX_RLIM,			/* read-write */
    CPDMA_CMD_IDLE,			/* write-only */
    CPDMA_COPY_ERROR_FRAMES,	/* read-write */
    CPDMA_RX_OFF_LEN_UPDATE,	/* read-write */
    CPDMA_RX_OWNERSHIP_FLIP,	/* read-write */
    CPDMA_TX_PRIO_FIXED,		/* read-write */
    CPDMA_STAT_IDLE,		/* read-only */
    CPDMA_STAT_TX_ERR_CHAN,		/* read-only */
    CPDMA_STAT_TX_ERR_CODE,		/* read-only */
    CPDMA_STAT_RX_ERR_CHAN,		/* read-only */
    CPDMA_STAT_RX_ERR_CODE,		/* read-only */
    CPDMA_RX_BUFFER_OFFSET,		/* read-write */
}

extern "C" {
    pub fn cpdma_control_get(ctlr: *mut cpdma_ctlr, control: c_int) -> c_int;
}
extern "C" {
    pub fn cpdma_control_set(ctlr: *mut cpdma_ctlr, control: c_int, value: c_int) -> c_int;
}
extern "C" {
    pub fn cpdma_get_num_rx_descs(ctlr: *mut cpdma_ctlr) -> c_int;
}
extern "C" {
    pub fn cpdma_set_num_rx_descs(ctlr: *mut cpdma_ctlr, num_rx_desc: c_int) -> c_int;
}
extern "C" {
    pub fn cpdma_get_num_tx_descs(ctlr: *mut cpdma_ctlr) -> c_int;
}
