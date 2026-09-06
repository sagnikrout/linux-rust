//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bus/mhi/ep/internal.h
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
// Copyright (c) 2022, Linaro Ltd.
//

pub const MHI_REG_OFFSET: c_uint = 0x100;
pub const BHI_REG_OFFSET: c_uint = 0x200;
// MHI registers

// MHI BHI registers

// MHI Doorbell registers

pub const MHI_CTRL_INT_STATUS: c_uint = 0x4;

pub const MHI_CTRL_INT_CLEAR: c_uint = 0x4c;

//
// Unlike the usual "masking" convention, writing "1" to a bit in this register
// enables the interrupt and writing "0" will disable it..
//
pub const MHI_CTRL_INT_MASK: c_uint = 0x94;

pub const NR_OF_CMD_RINGS: c_int = 1;
pub const MHI_MASK_ROWS_CH_DB: c_int = 4;
pub const MHI_MASK_ROWS_EV_DB: c_int = 4;
pub const MHI_MASK_CH_LEN: c_int = 32;
pub const MHI_MASK_EV_LEN: c_int = 32;
// Generic context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_generic_ctx {
    pub reserved0: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub __aligned(4): __le64 rbase __packed,
    pub __aligned(4): __le64 rlen __packed,
    pub __aligned(4): __le64 rp __packed,
    pub __aligned(4): __le64 wp __packed,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_ep_ring_type {
    RING_TYPE_CMD,
    RING_TYPE_ER,
    RING_TYPE_CH,
}

// Ring element
#[repr(C)]
#[derive(Copy, Clone)]
pub union mhi_ep_ring_ctx {
    pub cmd: mhi_cmd_ctxt,
    pub ev: mhi_event_ctxt,
    pub ch: mhi_chan_ctxt,
    pub generic: mhi_generic_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_ring_item {
    pub node: list_head,
    pub ring: *mut mhi_ep_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_ring {
    pub mhi_cntrl: *mut mhi_ep_cntrl,
    pub ring_ctx: *mut mhi_ep_ring_ctx,
    pub ring_cache: *mut mhi_ring_element,
    pub type: mhi_ep_ring_type,
    pub intmodt_work: delayed_work,
    pub rbase: u64,
    pub rd_offset: usize,
    pub wr_offset: usize,
    pub ring_size: usize,
    pub db_offset_h: u32,
    pub db_offset_l: u32,
    pub ch_id: u32,
    pub er_index: u32,
    pub irq_vector: u32,
    pub intmodt: u32,
    pub started: bool,
    pub irq_pending: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_cmd {
    pub ring: mhi_ep_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_event {
    pub ring: mhi_ep_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_state_transition {
    pub node: list_head,
    pub state: mhi_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ep_chan {
    pub name: *mut c_char,
    pub mhi_dev: *mut mhi_ep_device,
    pub ring: mhi_ep_ring,
    pub lock: mutex,
    pub result): *mut *mut *mut void (xfer_cb)(struct mhi_ep_device mhi_dev, struct mhi_result,
    pub state: mhi_ch_state,
    pub dir: dma_data_direction,
    pub rd_offset: usize,
    pub tre_loc: u64,
    pub tre_size: u32,
    pub tre_bytes_left: u32,
    pub chan: u32,
    pub skip_td: bool,
}

// MHI Ring related functions
extern "C" {
    pub fn mhi_ep_ring_init(ring: *mut mhi_ep_ring, type: mhi_ep_ring_type, id: u32);
}
extern "C" {
    pub fn mhi_ep_ring_reset(mhi_cntrl: *mut mhi_ep_cntrl, ring: *mut mhi_ep_ring);
}
extern "C" {
    pub fn mhi_ep_ring_addr2offset(ring: *mut mhi_ep_ring, ptr: u64) -> usize;
}
extern "C" {
    pub fn mhi_ep_ring_add_element(ring: *mut mhi_ep_ring, element: *mut mhi_ring_element) -> c_int;
}
extern "C" {
    pub fn mhi_ep_ring_inc_index(ring: *mut mhi_ep_ring);
}
extern "C" {
    pub fn mhi_ep_update_wr_offset(ring: *mut mhi_ep_ring) -> c_int;
}
// MMIO related functions
extern "C" {
    pub fn mhi_ep_mmio_read(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32) -> u32;
}
extern "C" {
    pub fn mhi_ep_mmio_write(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32, val: u32);
}
extern "C" {
    pub fn mhi_ep_mmio_masked_write(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32, mask: u32, val: u32);
}
extern "C" {
    pub fn mhi_ep_mmio_masked_read(dev: *mut mhi_ep_cntrl, offset: u32, mask: u32) -> u32;
}
extern "C" {
    pub fn mhi_ep_mmio_enable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_disable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_enable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_disable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_enable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32);
}
extern "C" {
    pub fn mhi_ep_mmio_disable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32);
}
extern "C" {
    pub fn mhi_ep_mmio_enable_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_read_chdb_status_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) -> bool;
}
extern "C" {
    pub fn mhi_ep_mmio_mask_interrupts(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_get_chc_base(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_get_erc_base(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_get_crc_base(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_get_db(ring: *mut mhi_ep_ring) -> u64;
}
extern "C" {
    pub fn mhi_ep_mmio_set_env(mhi_cntrl: *mut mhi_ep_cntrl, value: u32);
}
extern "C" {
    pub fn mhi_ep_mmio_clear_reset(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_reset(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_init(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_mmio_update_ner(mhi_cntrl: *mut mhi_ep_cntrl);
}
// MHI EP core functions
extern "C" {
    pub fn mhi_ep_send_state_change_event(mhi_cntrl: *mut mhi_ep_cntrl, state: mhi_state) -> c_int;
}
extern "C" {
    pub fn mhi_ep_send_ee_event(mhi_cntrl: *mut mhi_ep_cntrl, exec_env: mhi_ee_type) -> c_int;
}
extern "C" {
    pub fn mhi_ep_set_mhi_state(mhi_cntrl: *mut mhi_ep_cntrl, mhi_state: mhi_state) -> c_int;
}
extern "C" {
    pub fn mhi_ep_set_m0_state(mhi_cntrl: *mut mhi_ep_cntrl) -> c_int;
}
extern "C" {
    pub fn mhi_ep_set_m3_state(mhi_cntrl: *mut mhi_ep_cntrl) -> c_int;
}
extern "C" {
    pub fn mhi_ep_set_ready_state(mhi_cntrl: *mut mhi_ep_cntrl) -> c_int;
}
extern "C" {
    pub fn mhi_ep_handle_syserr(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_resume_channels(mhi_cntrl: *mut mhi_ep_cntrl);
}
extern "C" {
    pub fn mhi_ep_suspend_channels(mhi_cntrl: *mut mhi_ep_cntrl);
}
