//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/esd/esdacc.h
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
// Copyright (C) 2015 - 2016 Thomas Körper, esd electronic system design gmbh
// Copyright (C) 2017 - 2023 Stefan Mätje, esd electronics gmbh
//

pub const ACC_I2C_ADDON_DETECT_DELAY_MS: c_int = 10;
// esdACC Overview Module
pub const ACC_OV_OF_PROBE: c_uint = 0x0000;
pub const ACC_OV_OF_VERSION: c_uint = 0x0004;
pub const ACC_OV_OF_INFO: c_uint = 0x0008;
pub const ACC_OV_OF_CANCORE_FREQ: c_uint = 0x000c;
pub const ACC_OV_OF_TS_FREQ_LO: c_uint = 0x0010;
pub const ACC_OV_OF_TS_FREQ_HI: c_uint = 0x0014;
pub const ACC_OV_OF_IRQ_STATUS_CORES: c_uint = 0x0018;
pub const ACC_OV_OF_TS_CURR_LO: c_uint = 0x001c;
pub const ACC_OV_OF_TS_CURR_HI: c_uint = 0x0020;
pub const ACC_OV_OF_IRQ_STATUS: c_uint = 0x0028;
pub const ACC_OV_OF_MODE: c_uint = 0x002c;
pub const ACC_OV_OF_BM_IRQ_COUNTER: c_uint = 0x0070;
pub const ACC_OV_OF_BM_IRQ_MASK: c_uint = 0x0074;
pub const ACC_OV_OF_MSI_DATA: c_uint = 0x0080;
pub const ACC_OV_OF_MSI_ADDRESSOFFSET: c_uint = 0x0084;
// Feature flags are contained in the upper 16 bit of the version
// register at ACC_OV_OF_VERSION but only used with these masks after
// extraction into an extra variable => (xx - 16).
//

// esdACC CAN Core Module
pub const ACC_CORE_OF_CTRL: c_uint = 0x0000;
pub const ACC_CORE_OF_STATUS_IRQ: c_uint = 0x0008;
pub const ACC_CORE_OF_BRP: c_uint = 0x000c;
pub const ACC_CORE_OF_BTR: c_uint = 0x0010;
pub const ACC_CORE_OF_FBTR: c_uint = 0x0014;
pub const ACC_CORE_OF_STATUS: c_uint = 0x0030;
pub const ACC_CORE_OF_TXFIFO_CONFIG: c_uint = 0x0048;
pub const ACC_CORE_OF_TXFIFO_STATUS: c_uint = 0x004c;
pub const ACC_CORE_OF_TX_STATUS_IRQ: c_uint = 0x0050;
pub const ACC_CORE_OF_TX_ABORT_MASK: c_uint = 0x0054;
pub const ACC_CORE_OF_BM_IRQ_COUNTER: c_uint = 0x0070;
pub const ACC_CORE_OF_TXFIFO_ID: c_uint = 0x00c0;
pub const ACC_CORE_OF_TXFIFO_DLC: c_uint = 0x00c4;
pub const ACC_CORE_OF_TXFIFO_DATA_0: c_uint = 0x00c8;
pub const ACC_CORE_OF_TXFIFO_DATA_1: c_uint = 0x00cc;
// CTRL register layout

// BRP and BTR register layout for CAN-Classic version

// BRP and BTR register layout for CAN-FD version

// 256 BM_MSGs of 32 byte size

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acc_bmmsg_id {
    BM_MSG_ID_RXTXDONE = 0x01,
    BM_MSG_ID_TXABORT = 0x02,
    BM_MSG_ID_OVERRUN = 0x03,
    BM_MSG_ID_BUSERR = 0x04,
    BM_MSG_ID_ERRPASSIVE = 0x05,
    BM_MSG_ID_ERRWARN = 0x06,
    BM_MSG_ID_TIMESLICE = 0x07,
    BM_MSG_ID_HWTIMER = 0x08,
    BM_MSG_ID_HOTPLUG = 0x09,
}

// The struct acc_bmmsg_* structure declarations that follow here provide
// access to the ring buffer of bus master messages maintained by the FPGA
// bus master engine. All bus master messages have the same size of
// ACC_CORE_DMAMSG_SIZE and a minimum alignment of ACC_CORE_DMAMSG_SIZE in
// memory.
//
// All structure members are natural aligned. Therefore we should not need
// a __packed attribute. All struct acc_bmmsg_* declarations have at least
// reserved* members to fill the structure to the full ACC_CORE_DMAMSG_SIZE.
//
// A failure of this property due padding will be detected at compile time
// by static_assert(sizeof(union acc_bmmsg) == ACC_CORE_DMAMSG_SIZE).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_rxtxdone {
    pub msg_id: u8,
    pub txfifo_level: u8,
    pub reserved1: [u8; 2],
    pub txtsfifo_level: u8,
    pub reserved2: [u8; 3],
    pub id: u32,
    pub len: u8,
    pub txdfifo_idx: u8,
    pub zeroes8: u8,
    pub reserved: u8,
    pub acc_dlc: },
    pub data: [u8; CAN_MAX_DLEN],
// Time stamps in struct acc_ov::timestamp_frequency ticks.
    pub ts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_txabort {
    pub msg_id: u8,
    pub txfifo_level: u8,
    pub abort_mask: u16,
    pub txtsfifo_level: u8,
    pub reserved2: [u8; 1],
    pub abort_mask_txts: u16,
    pub ts: u64,
    pub reserved3: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_overrun {
    pub msg_id: u8,
    pub txfifo_level: u8,
    pub lost_cnt: u8,
    pub reserved1: u8,
    pub txtsfifo_level: u8,
    pub reserved2: [u8; 3],
    pub ts: u64,
    pub reserved3: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_buserr {
    pub msg_id: u8,
    pub txfifo_level: u8,
    pub ecc: u8,
    pub reserved1: u8,
    pub txtsfifo_level: u8,
    pub reserved2: [u8; 3],
    pub ts: u64,
    pub reg_status: u32,
    pub reg_btr: u32,
    pub reserved3: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_errstatechange {
    pub msg_id: u8,
    pub txfifo_level: u8,
    pub reserved1: [u8; 2],
    pub txtsfifo_level: u8,
    pub reserved2: [u8; 3],
    pub ts: u64,
    pub reg_status: u32,
    pub reserved3: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_timeslice {
    pub msg_id: u8,
    pub txfifo_level: u8,
    pub reserved1: [u8; 2],
    pub txtsfifo_level: u8,
    pub reserved2: [u8; 3],
    pub ts: u64,
    pub reserved3: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_hwtimer {
    pub msg_id: u8,
    pub reserved1: [u8; 3],
    pub reserved2: [u32; 1],
    pub timer: u64,
    pub reserved3: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmmsg_hotplug {
    pub msg_id: u8,
    pub reserved1: [u8; 3],
    pub reserved2: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acc_bmmsg {
    pub msg_id: u8,
    pub rxtxdone: acc_bmmsg_rxtxdone,
    pub txabort: acc_bmmsg_txabort,
    pub overrun: acc_bmmsg_overrun,
    pub buserr: acc_bmmsg_buserr,
    pub errstatechange: acc_bmmsg_errstatechange,
    pub timeslice: acc_bmmsg_timeslice,
    pub hwtimer: acc_bmmsg_hwtimer,
}

// Check size of union acc_bmmsg to be of expected size.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_bmfifo {
    pub messages: *const acc_bmmsg,
// irq_cnt points to an u32 value where the esdACC FPGA deposits
// the bm_fifo head index in coherent DMA memory. Only bits 7..0
// are valid. Use READ_ONCE() to access this memory location.
//
    pub irq_cnt: *const u32,
    pub local_irq_cnt: u32,
    pub msg_fifo_tail: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_core {
    pub addr: *mut void __iomem,
    pub netdev: *mut net_device,
    pub bmfifo: acc_bmfifo,
    pub tx_fifo_size: u8,
    pub tx_fifo_head: u8,
    pub tx_fifo_tail: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_ov {
    pub addr: *mut void __iomem,
    pub bmfifo: acc_bmfifo,
    pub timestamp_frequency: u32,
    pub core_frequency: u32,
    pub version: u16,
    pub features: u16,
    pub total_cores: u8,
    pub active_cores: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_net_priv {
    pub /: *mut *mut can_priv can; / must be the first member!,
    pub core: *mut acc_core,
    pub ov: *mut acc_ov,
}

extern "C" {
    pub fn ioread32be(offs: core->addr +) -> return;
}
extern "C" {
    pub fn ioread32be(offs: ov->addr +) -> return;
}
// (Re-)start and wait for completion of addon detection on the I^2C bus
extern "C" {
    pub fn acc_init_ov(ov: *mut acc_ov, dev: *mut device);
}
extern "C" {
    pub fn acc_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn acc_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn acc_start_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn acc_set_mode(netdev: *mut net_device, mode: can_mode) -> c_int;
}
extern "C" {
    pub fn acc_set_bittiming(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn acc_card_interrupt(ov: *mut acc_ov, cores: *mut acc_core) -> irqreturn_t;
}
