//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3pf/hclge_ptp.h
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
// Copyright (c) 2021 Hisilicon Limited.

pub const HCLGE_PTP_REG_OFFSET: c_uint = 0x29000;
pub const HCLGE_PTP_TX_TS_SEQID_REG: c_uint = 0x0;
pub const HCLGE_PTP_TX_TS_NSEC_REG: c_uint = 0x4;

pub const HCLGE_PTP_TX_TS_SEC_L_REG: c_uint = 0x8;
pub const HCLGE_PTP_TX_TS_SEC_H_REG: c_uint = 0xC;

pub const HCLGE_PTP_TX_TS_CNT_REG: c_uint = 0x30;
pub const HCLGE_PTP_TIME_SEC_H_REG: c_uint = 0x50;

pub const HCLGE_PTP_TIME_SEC_L_REG: c_uint = 0x54;
pub const HCLGE_PTP_TIME_NSEC_REG: c_uint = 0x58;
pub const HCLGE_PTP_TIME_NSEC_MASK: c_uint = 0x3fffffffLL;

pub const HCLGE_PTP_TIME_SYNC_REG: c_uint = 0x5C;

pub const HCLGE_PTP_TIME_ADJ_REG: c_uint = 0x60;

pub const HCLGE_PTP_CYCLE_QUO_REG: c_uint = 0x64;

pub const HCLGE_PTP_CYCLE_DEN_REG: c_uint = 0x68;
pub const HCLGE_PTP_CYCLE_NUM_REG: c_uint = 0x6C;
pub const HCLGE_PTP_CYCLE_CFG_REG: c_uint = 0x70;

pub const HCLGE_PTP_CUR_TIME_SEC_H_REG: c_uint = 0x74;
pub const HCLGE_PTP_CUR_TIME_SEC_L_REG: c_uint = 0x78;
pub const HCLGE_PTP_CUR_TIME_NSEC_REG: c_uint = 0x7C;
pub const HCLGE_PTP_CYCLE_ADJ_MAX: c_int = 500000000;

pub const HCLGE_PTP_FLAG_EN: c_int = 0;
pub const HCLGE_PTP_FLAG_TX_EN: c_int = 1;
pub const HCLGE_PTP_FLAG_RX_EN: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_ptp_cycle {
    pub quo: u32,
    pub numer: u32,
    pub den: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_ptp {
    pub hdev: *mut hclge_dev,
    pub clock: *mut ptp_clock,
    pub tx_skb: *mut sk_buff,
    pub flags: c_ulong,
    pub io_base: *mut void __iomem,
    pub info: ptp_clock_info,
    pub ts_cfg: kernel_hwtstamp_config,
    pub /: *mut *mut spinlock_t lock; / protects ptp registers,
    pub ptp_cfg: u32,
    pub last_tx_seqid: u32,
    pub cycle: hclge_ptp_cycle,
    pub tx_start: c_ulong,
    pub tx_cnt: c_ulong,
    pub tx_skipped: c_ulong,
    pub tx_cleaned: c_ulong,
    pub last_rx: c_ulong,
    pub rx_cnt: c_ulong,
    pub tx_timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_ptp_int_cmd {

    pub int_en: u8,
    pub rsvd: [u8; 23],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_ptp_udp_type {
    HCLGE_PTP_UDP_NOT_TYPE,
    HCLGE_PTP_UDP_P13F_TYPE,
    HCLGE_PTP_UDP_P140_TYPE,
    HCLGE_PTP_UDP_FULL_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_ptp_msg_type {
    HCLGE_PTP_MSG_TYPE_V2_L2,
    HCLGE_PTP_MSG_TYPE_V2,
    HCLGE_PTP_MSG_TYPE_V2_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_ptp_msg0_type {
    HCLGE_PTP_MSG0_V2_DELAY_REQ = 1,
    HCLGE_PTP_MSG0_V2_PDELAY_REQ,
    HCLGE_PTP_MSG0_V2_DELAY_RESP,
    HCLGE_PTP_MSG0_V2_EVENT = 0xF,
}

pub const HCLGE_PTP_MSG1_V2_DEFAULT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_ptp_cfg_cmd {

pub const HCLGE_PTP_UDP_EN_SHIFT: c_int = 3;

pub const HCLGE_PTP_MSG_TYPE_SHIFT: c_int = 8;

pub const HCLGE_PTP_MSG1_SHIFT: c_int = 16;

pub const HCLGE_PTP_MSG0_SHIFT: c_int = 24;

    pub cfg: __le32,
    pub rsvd: [u8; 20],
}

extern "C" {
    pub fn hclge_ptp_set_tx_info(handle: *mut hnae3_handle, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn hclge_ptp_clean_tx_hwts(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_ptp_init(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_ptp_uninit(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_ptp_cfg_qry(hdev: *mut hclge_dev, cfg: *mut u32) -> c_int;
}
