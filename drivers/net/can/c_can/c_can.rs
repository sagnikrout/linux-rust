//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/c_can/c_can.h
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


//
// CAN bus driver for Bosch C_CAN controller
//
// Copyright (C) 2010 ST Microelectronics
// Bhupesh Sharma <bhupesh.sharma@st.com>
//
// Borrowed heavily from the C_CAN driver originally written by:
// Copyright (C) 2007
// - Sascha Hauer, Marc Kleine-Budde, Pengutronix <s.hauer@pengutronix.de>
// - Simon Kallweit, intefo AG <simon.kallweit@intefo.ch>
//
// Bosch C_CAN controller is compliant to CAN protocol version 2.0 part A and B.
// Bosch C_CAN user manual can be obtained from:
// http://www.semiconductors.bosch.de/media/en/pdf/ipmodules_1/c_can
// users_manual_c_can.pdf
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg {
    C_CAN_CTRL_REG = 0,
    C_CAN_CTRL_EX_REG,
    C_CAN_STS_REG,
    C_CAN_ERR_CNT_REG,
    C_CAN_BTR_REG,
    C_CAN_INT_REG,
    C_CAN_TEST_REG,
    C_CAN_BRPEXT_REG,
    C_CAN_IF1_COMREQ_REG,
    C_CAN_IF1_COMMSK_REG,
    C_CAN_IF1_MASK1_REG,
    C_CAN_IF1_MASK2_REG,
    C_CAN_IF1_ARB1_REG,
    C_CAN_IF1_ARB2_REG,
    C_CAN_IF1_MSGCTRL_REG,
    C_CAN_IF1_DATA1_REG,
    C_CAN_IF1_DATA2_REG,
    C_CAN_IF1_DATA3_REG,
    C_CAN_IF1_DATA4_REG,
    C_CAN_IF2_COMREQ_REG,
    C_CAN_IF2_COMMSK_REG,
    C_CAN_IF2_MASK1_REG,
    C_CAN_IF2_MASK2_REG,
    C_CAN_IF2_ARB1_REG,
    C_CAN_IF2_ARB2_REG,
    C_CAN_IF2_MSGCTRL_REG,
    C_CAN_IF2_DATA1_REG,
    C_CAN_IF2_DATA2_REG,
    C_CAN_IF2_DATA3_REG,
    C_CAN_IF2_DATA4_REG,
    C_CAN_TXRQST1_REG,
    C_CAN_TXRQST2_REG,
    C_CAN_NEWDAT1_REG,
    C_CAN_NEWDAT2_REG,
    C_CAN_INTPND1_REG,
    C_CAN_INTPND2_REG,
    C_CAN_INTPND3_REG,
    C_CAN_MSGVAL1_REG,
    C_CAN_MSGVAL2_REG,
    C_CAN_FUNCTION_REG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c_can_dev_id {
    BOSCH_C_CAN,
    BOSCH_D_CAN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raminit_bits {
    pub start: u8,
    pub done: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c_can_driver_data {
    pub id: c_can_dev_id,
    pub msg_obj_num: c_uint,
// RAMINIT register description. Optional.
    pub /: *const *const *const raminit_bits raminit_bits; / Array of START/DONE bit positions,
    pub /: *mut *mut u8 raminit_num; / Number of CAN instances on the SoC,
    pub /: *mut *mut bool raminit_pulse; / If set, sets and clears START bit (pulse),
}

// Out of band RAMINIT register access via syscon regmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c_can_raminit {
    pub /: *mut *mut *mut regmap syscon; / for raminit ctrl. reg. access,
    pub /: *mut *mut unsigned int reg; / register index within syscon,
    pub bits: raminit_bits,
    pub needs_pulse: bool,
}

// c_can tx ring structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c_can_tx_ring {
    pub head: c_uint,
    pub tail: c_uint,
    pub obj_num: c_uint,
}

// c_can private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c_can_priv {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub napi: napi_struct,
    pub dev: *mut net_device,
    pub device: *mut device,
    pub msg_obj_num: c_uint,
    pub msg_obj_rx_num: c_uint,
    pub msg_obj_tx_num: c_uint,
    pub msg_obj_rx_first: c_uint,
    pub msg_obj_rx_last: c_uint,
    pub msg_obj_tx_first: c_uint,
    pub msg_obj_tx_last: c_uint,
    pub msg_obj_rx_mask: u32,
    pub sie_pending: core::sync::atomic::AtomicI32,
    pub tx_dir: c_ulong,
    pub last_status: c_int,
    pub tx: c_can_tx_ring,
    pub index): *const *const *const u16 (read_reg)(struct c_can_priv priv, enum reg,
    pub val): *const *const *const void (write_reg)(struct c_can_priv priv, enum reg index, u16,
    pub index): *const *const *const u32 (read_reg32)(struct c_can_priv priv, enum reg,
    pub val): *const *const *const void (write_reg32)(struct c_can_priv priv, enum reg index, u32,
    pub base: *mut void __iomem,
    pub regs: *const u16,
    pub type: c_can_dev_id,
    pub /: *mut *mut c_can_raminit raminit_sys; / RAMINIT via syscon regmap,
    pub enable): *const *const *const void (raminit)(struct c_can_priv priv, bool,
    pub comm_rcv_high: u32,
}

extern "C" {
    pub fn free_c_can_dev(dev: *mut net_device);
}
extern "C" {
    pub fn register_c_can_dev(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn unregister_c_can_dev(dev: *mut net_device);
}

extern "C" {
    pub fn c_can_power_up(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn c_can_power_down(dev: *mut net_device) -> c_int;
}

// This is not a FIFO. C/D_CAN sends out the buffers
// prioritized. The lowest buffer number wins.
//
