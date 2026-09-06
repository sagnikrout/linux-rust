//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hibmcge/hbg_reg.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2024 Hisilicon Limited.
// DEV SPEC
pub const HBG_REG_SPEC_VALID_ADDR: c_uint = 0x0000;
pub const HBG_REG_EVENT_REQ_ADDR: c_uint = 0x0004;
pub const HBG_REG_MAC_ID_ADDR: c_uint = 0x0008;
pub const HBG_REG_PHY_ID_ADDR: c_uint = 0x000C;
pub const HBG_REG_MAC_ADDR_ADDR: c_uint = 0x0010;
pub const HBG_REG_MAC_ADDR_HIGH_ADDR: c_uint = 0x0014;
pub const HBG_REG_UC_MAC_NUM_ADDR: c_uint = 0x0018;
pub const HBG_REG_MDIO_FREQ_ADDR: c_uint = 0x0024;
pub const HBG_REG_MAX_MTU_ADDR: c_uint = 0x0028;
pub const HBG_REG_MIN_MTU_ADDR: c_uint = 0x002C;
pub const HBG_REG_TX_FIFO_NUM_ADDR: c_uint = 0x0030;
pub const HBG_REG_RX_FIFO_NUM_ADDR: c_uint = 0x0034;
pub const HBG_REG_VLAN_LAYERS_ADDR: c_uint = 0x0038;
pub const HBG_REG_PUSH_REQ_ADDR: c_uint = 0x00F0;
pub const HBG_REG_MSG_HEADER_ADDR: c_uint = 0x00F4;

pub const HBG_REG_MSG_DATA_BASE_ADDR: c_uint = 0x0100;
// MDIO
pub const HBG_REG_MDIO_BASE: c_uint = 0x8000;

// GMAC
pub const HBG_REG_SGMII_BASE: c_uint = 0x10000;

// PCU

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_port_mode {
// 0x0 ~ 0x5 are reserved
    HBG_PORT_MODE_SGMII_10M = 0x6,
    HBG_PORT_MODE_SGMII_100M = 0x7,
    HBG_PORT_MODE_SGMII_1000M = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_tx_desc {
    pub word0: u32,
    pub word1: u32,
    pub /: *mut *mut u32 word2; / pkt_addr,
    pub /: *mut *mut u32 word3; / clear_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_rx_desc {
    pub word0: u32,
    pub /: *mut *mut u32 word1; / tag,
    pub word2: u32,
    pub word3: u32,
    pub word4: u32,
    pub word5: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_l3_err_code {
    HBG_L3_OK = 0,
    HBG_L3_WRONG_HEAD,
    HBG_L3_CSUM_ERR,
    HBG_L3_LEN_ERR,
    HBG_L3_ZERO_TTL,
    HBG_L3_RSVD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_l4_err_code {
    HBG_L4_OK = 0,
    HBG_L4_WRONG_HEAD,
    HBG_L4_LEN_ERR,
    HBG_L4_CSUM_ERR,
    HBG_L4_ZERO_PORT_NUM,
    HBG_L4_RSVD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_pkt_type_code {
    HBG_NO_IP_PKT = 0,
    HBG_IP_PKT,
    HBG_TCP_PKT,
    HBG_UDP_PKT,
}
