//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/m_can/m_can.h
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
// CAN bus driver for Bosch M_CAN controller
// Copyright (C) 2018 Texas Instruments Incorporated - http://www.ti.com
//

// m_can lec values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum m_can_lec_type {
    LEC_NO_ERROR = 0,
    LEC_STUFF_ERROR,
    LEC_FORM_ERROR,
    LEC_ACK_ERROR,
    LEC_BIT1_ERROR,
    LEC_BIT0_ERROR,
    LEC_CRC_ERROR,
    LEC_NO_CHANGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum m_can_mram_cfg {
    MRAM_SIDF = 0,
    MRAM_XIDF,
    MRAM_RXF0,
    MRAM_RXF1,
    MRAM_RXB,
    MRAM_TXE,
    MRAM_TXB,
    MRAM_CFG_NUM,
}

// address offset and element number for each FIFO/Buffer in the Message RAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mram_cfg {
    pub off: u16,
    pub num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m_can_ops {
// Device specific call backs
    pub cdev): *mut *mut int (clear_interrupts)(struct m_can_classdev,
    pub reg): *mut *mut *mut u32 (read_reg)(struct m_can_classdev cdev, int,
    pub val): *mut *mut *mut int (write_reg)(struct m_can_classdev cdev, int reg, int,
    pub val_count): *mut *mut *mut *mut int (read_fifo)(struct m_can_classdev cdev, int addr_offset, void val, size_t,
    pub val_count): *const *const void val, size_t,
    pub cdev): *mut *mut int (init)(struct m_can_classdev,
    pub cdev): *mut *mut int (deinit)(struct m_can_classdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m_can_tx_op {
    pub cdev: *mut m_can_classdev,
    pub work: work_struct,
    pub skb: *mut sk_buff,
    pub submit: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m_can_classdev {
    pub can: can_priv,
    pub offload: can_rx_offload,
    pub napi: napi_struct,
    pub net: *mut net_device,
    pub dev: *mut device,
    pub hclk: *mut clk,
    pub cclk: *mut clk,
    pub rst: *mut reset_control,
    pub tx_wq: *mut workqueue_struct,
    pub transceiver: *mut phy,
    pub irq_timer_wait: ktime_t,
    pub ops: *const m_can_ops,
    pub version: c_int,
    pub irqstatus: u32,
    pub pm_clock_support: c_int,
    pub pm_wake_source: c_int,
    pub is_peripheral: c_int,
    pub irq_edge_triggered: bool,
// Cached M_CAN_IE register content
    pub active_interrupts: u32,
    pub rx_max_coalesced_frames_irq: u32,
    pub rx_coalesce_usecs_irq: u32,
    pub tx_max_coalesced_frames: u32,
    pub tx_max_coalesced_frames_irq: u32,
    pub tx_coalesce_usecs_irq: u32,
// Store this internally to avoid fetch delays on peripheral chips
    pub tx_fifo_putidx: u32,
// Protects shared state between start_xmit and m_can_isr
    pub tx_handling_spinlock: spinlock_t,
    pub tx_fifo_in_flight: c_int,
    pub tx_ops: *mut m_can_tx_op,
    pub tx_fifo_size: c_int,
    pub next_tx_op: c_int,
    pub nr_txs_without_submit: c_int,
// bitfield of fifo elements that will be submitted together
    pub tx_peripheral_submit: u32,
    pub mcfg: [mram_cfg; MRAM_CFG_NUM],
    pub hrtimer: hrtimer,
    pub pinctrl: *mut pinctrl,
    pub pinctrl_state_wakeup: *mut pinctrl_state,
}

extern "C" {
    pub fn m_can_class_free_dev(net: *mut net_device);
}
extern "C" {
    pub fn m_can_class_register(cdev: *mut m_can_classdev) -> c_int;
}
extern "C" {
    pub fn m_can_class_unregister(cdev: *mut m_can_classdev);
}
extern "C" {
    pub fn m_can_class_get_clocks(cdev: *mut m_can_classdev) -> c_int;
}
extern "C" {
    pub fn m_can_check_mram_cfg(cdev: *mut m_can_classdev, mram_max_size: u32) -> c_int;
}
extern "C" {
    pub fn m_can_class_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn m_can_class_resume(dev: *mut device) -> c_int;
}
