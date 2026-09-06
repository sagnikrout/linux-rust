//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/microchip_rds_ptp.h
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
// Copyright (C) 2024 Microchip Technology
//

pub const MCHP_RDS_PTP_CMD_CTL: c_uint = 0x0;

pub const MCHP_RDS_PTP_REF_CLK_CFG: c_uint = 0x2;
pub const MCHP_RDS_PTP_REF_CLK_SRC_250MHZ: c_uint = 0x0;

pub const MCHP_RDS_PTP_REF_CLK_PERIOD: c_int = 4;

pub const MCHP_RDS_PTP_LTC_SEC_HI: c_uint = 0x5;
pub const MCHP_RDS_PTP_LTC_SEC_MID: c_uint = 0x6;
pub const MCHP_RDS_PTP_LTC_SEC_LO: c_uint = 0x7;
pub const MCHP_RDS_PTP_LTC_NS_HI: c_uint = 0x8;
pub const MCHP_RDS_PTP_LTC_NS_LO: c_uint = 0x9;
pub const MCHP_RDS_PTP_LTC_RATE_ADJ_HI: c_uint = 0xc;

pub const MCHP_RDS_PTP_LTC_RATE_ADJ_LO: c_uint = 0xd;
pub const MCHP_RDS_PTP_STEP_ADJ_HI: c_uint = 0x12;

pub const MCHP_RDS_PTP_STEP_ADJ_LO: c_uint = 0x13;
pub const MCHP_RDS_PTP_LTC_READ_SEC_HI: c_uint = 0x29;
pub const MCHP_RDS_PTP_LTC_READ_SEC_MID: c_uint = 0x2a;
pub const MCHP_RDS_PTP_LTC_READ_SEC_LO: c_uint = 0x2b;
pub const MCHP_RDS_PTP_LTC_READ_NS_HI: c_uint = 0x2c;
pub const MCHP_RDS_PTP_LTC_READ_NS_LO: c_uint = 0x2d;
pub const MCHP_RDS_PTP_OP_MODE: c_uint = 0x41;
pub const MCHP_RDS_PTP_OP_MODE_DIS: c_int = 0;
pub const MCHP_RDS_PTP_OP_MODE_STANDALONE: c_int = 1;
pub const MCHP_RDS_PTP_LATENCY_CORRECTION_CTL: c_uint = 0x44;

pub const MCHP_RDS_PTP_INT_EN: c_uint = 0x0;
pub const MCHP_RDS_PTP_INT_STS: c_uint = 0x01;

pub const MCHP_RDS_PTP_CAP_INFO: c_uint = 0x2e;

pub const MCHP_RDS_PTP_RX_PARSE_CONFIG: c_uint = 0x42;
pub const MCHP_RDS_PTP_RX_PARSE_L2_ADDR_EN: c_uint = 0x44;
pub const MCHP_RDS_PTP_RX_PARSE_IPV4_ADDR_EN: c_uint = 0x45;
pub const MCHP_RDS_PTP_RX_TIMESTAMP_CONFIG: c_uint = 0x4e;

pub const MCHP_RDS_PTP_RX_VERSION: c_uint = 0x48;
pub const MCHP_RDS_PTP_RX_TIMESTAMP_EN: c_uint = 0x4d;
pub const MCHP_RDS_PTP_RX_INGRESS_NS_HI: c_uint = 0x54;

pub const MCHP_RDS_PTP_RX_INGRESS_NS_LO: c_uint = 0x55;
pub const MCHP_RDS_PTP_RX_INGRESS_SEC_HI: c_uint = 0x56;
pub const MCHP_RDS_PTP_RX_INGRESS_SEC_LO: c_uint = 0x57;
pub const MCHP_RDS_PTP_RX_MSG_HDR2: c_uint = 0x59;
pub const MCHP_RDS_PTP_TX_PARSE_CONFIG: c_uint = 0x82;

pub const MCHP_RDS_PTP_TX_PARSE_L2_ADDR_EN: c_uint = 0x84;
pub const MCHP_RDS_PTP_TX_PARSE_IPV4_ADDR_EN: c_uint = 0x85;
pub const MCHP_RDS_PTP_TX_VERSION: c_uint = 0x88;

pub const MCHP_RDS_PTP_TX_TIMESTAMP_EN: c_uint = 0x8d;

pub const MCHP_RDS_PTP_TX_TIMESTAMP_CONFIG: c_uint = 0x8e;

pub const MCHP_RDS_PTP_TX_MOD: c_uint = 0x8f;

pub const MCHP_RDS_PTP_TX_EGRESS_NS_HI: c_uint = 0x94;

pub const MCHP_RDS_PTP_TX_EGRESS_NS_LO: c_uint = 0x95;
pub const MCHP_RDS_PTP_TX_EGRESS_SEC_HI: c_uint = 0x96;
pub const MCHP_RDS_PTP_TX_EGRESS_SEC_LO: c_uint = 0x97;
pub const MCHP_RDS_PTP_TX_MSG_HDR2: c_uint = 0x99;
pub const MCHP_RDS_PTP_TSU_GEN_CONFIG: c_uint = 0xc0;

pub const MCHP_RDS_PTP_TSU_HARD_RESET: c_uint = 0xc1;

pub const MCHP_RDS_PTP_CLK_TRGT_SEC_HI: c_uint = 0x15;
pub const MCHP_RDS_PTP_CLK_TRGT_SEC_LO: c_uint = 0x16;
pub const MCHP_RDS_PTP_CLK_TRGT_NS_HI: c_uint = 0x17;
pub const MCHP_RDS_PTP_CLK_TRGT_NS_LO: c_uint = 0x18;
pub const MCHP_RDS_PTP_CLK_TRGT_RELOAD_SEC_HI: c_uint = 0x19;
pub const MCHP_RDS_PTP_CLK_TRGT_RELOAD_SEC_LO: c_uint = 0x1a;
pub const MCHP_RDS_PTP_CLK_TRGT_RELOAD_NS_HI: c_uint = 0x1b;
pub const MCHP_RDS_PTP_CLK_TRGT_RELOAD_NS_LO: c_uint = 0x1c;
pub const MCHP_RDS_PTP_GEN_CFG: c_uint = 0x01;

// Represents 1ppm adjustment in 2^32 format with
// each nsec contains 4 clock cycles in 250MHz.
// The value is calculated as following: (1/1000000)/((2^-32)/4)
//
pub const MCHP_RDS_PTP_1PPM_FORMAT: c_int = 17179;
pub const MCHP_RDS_PTP_FIFO_SIZE: c_int = 8;
pub const MCHP_RDS_PTP_MAX_ADJ: c_int = 31249999;
pub const MCHP_RDS_PTP_BUFFER_TIME: c_int = 2;
pub const MCHP_RDS_PTP_N_PIN: c_int = 4;
pub const MCHP_RDS_PTP_N_PEROUT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mchp_rds_ptp_base {
    MCHP_RDS_PTP_PORT,
    MCHP_RDS_PTP_CLOCK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mchp_rds_ptp_fifo_dir {
    MCHP_RDS_PTP_INGRESS_FIFO,
    MCHP_RDS_PTP_EGRESS_FIFO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_rds_ptp_clock {
    pub mii_ts: mii_timestamper,
    pub phydev: *mut phy_device,
    pub ptp_clock: *mut ptp_clock,
    pub tx_queue: sk_buff_head,
    pub rx_queue: sk_buff_head,
    pub rx_ts_list: list_head,
    pub caps: ptp_clock_info,
// Lock for Rx ts fifo
    pub rx_ts_lock: spinlock_t,
    pub hwts_tx_type: c_int,
    pub rx_filter: hwtstamp_rx_filters,
    pub layer: c_int,
    pub version: c_int,
    pub port_base_addr: u16,
    pub clk_base_addr: u16,
// Lock for phc
    pub ptp_lock: mutex,
    pub mmd: u8,
    pub mchp_rds_ptp_event: c_int,
    pub event_pin: c_int,
    pub pin_config: *mut ptp_pin_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_rds_ptp_rx_ts {
    pub list: list_head,
    pub seconds: u32,
    pub nsec: u32,
    pub seq_id: u16,
}

extern "C" {
    pub fn mchp_rds_ptp_handle_interrupt(clock: *mut mchp_rds_ptp_clock) -> irqreturn_t;
}

// phydev, u8 mmd,
// clock)

