//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/am65-cpsw-qos.h
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
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_est {
    pub buf: c_int,
// has to be the last one
    pub taprio: tc_taprio_qopt_offload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_mqprio {
    pub mqprio_hw: tc_mqprio_qopt_offload,
    pub max_rate_total: u64,
    pub shaper_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_iet {
    pub preemptible_tcs: u8,
    pub original_max_blks: u32,
    pub verify_time_ms: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_ale_ratelimit {
    pub cookie: c_ulong,
    pub rate_packet_ps: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_qos {
    pub est_admin: *mut am65_cpsw_est,
    pub est_oper: *mut am65_cpsw_est,
    pub link_down_time: ktime_t,
    pub link_speed: c_int,
    pub mqprio: am65_cpsw_mqprio,
    pub iet: am65_cpsw_iet,
    pub ale_bc_ratelimit: am65_cpsw_ale_ratelimit,
    pub ale_mc_ratelimit: am65_cpsw_ale_ratelimit,
}

pub const AM65_CPSW_REG_CTL: c_uint = 0x004;
pub const AM65_CPSW_PN_REG_CTL: c_uint = 0x004;
pub const AM65_CPSW_PN_REG_FIFO_STATUS: c_uint = 0x050;
pub const AM65_CPSW_PN_REG_EST_CTL: c_uint = 0x060;

pub const AM65_CPSW_PN_REG_CTL: c_uint = 0x004;
pub const AM65_CPSW_PN_REG_TX_PRI_MAP: c_uint = 0x018;
pub const AM65_CPSW_PN_REG_RX_PRI_MAP: c_uint = 0x020;
pub const AM65_CPSW_PN_REG_FIFO_STATUS: c_uint = 0x050;
pub const AM65_CPSW_PN_REG_EST_CTL: c_uint = 0x060;

// AM65_CPSW_REG_CTL register fields

// AM65_CPSW_PN_REG_CTL register fields

// AM65_CPSW_PN_REG_EST_CTL register fields

// AM65_CPSW_PN_REG_FIFO_STATUS register fields

// EST FETCH COMMAND RAM
pub const AM65_CPSW_FETCH_RAM_CMD_NUM: c_uint = 0x80;

pub const AM65_CPSW_FETCH_CNT_OFFSET: c_int = 8;

// number of priority queues per port FIFO
pub const AM65_CPSW_PN_FIFO_PRIO_NUM: c_int = 8;

extern "C" {
    pub fn am65_cpsw_qos_link_up(ndev: *mut net_device, link_speed: c_int);
}
extern "C" {
    pub fn am65_cpsw_qos_link_down(ndev: *mut net_device);
}
extern "C" {
    pub fn am65_cpsw_qos_ndo_tx_p0_set_maxrate(ndev: *mut net_device, queue: c_int, rate_mbps: u32) -> c_int;
}
extern "C" {
    pub fn am65_cpsw_qos_tx_p0_rate_init(common: *mut am65_cpsw_common);
}
extern "C" {
    pub fn am65_cpsw_iet_commit_preemptible_tcs(port: *mut am65_cpsw_port);
}
extern "C" {
    pub fn am65_cpsw_iet_common_enable(common: *mut am65_cpsw_common);
}

pub const AM65_CPSW_REG_CTL: c_uint = 0x004;
pub const AM65_CPSW_PN_REG_CTL: c_uint = 0x004;
pub const AM65_CPSW_PN_REG_MAX_BLKS: c_uint = 0x008;
pub const AM65_CPSW_PN_REG_TX_PRI_MAP: c_uint = 0x018;
pub const AM65_CPSW_PN_REG_RX_PRI_MAP: c_uint = 0x020;
pub const AM65_CPSW_PN_REG_IET_CTRL: c_uint = 0x040;
pub const AM65_CPSW_PN_REG_IET_STATUS: c_uint = 0x044;
pub const AM65_CPSW_PN_REG_IET_VERIFY: c_uint = 0x048;
pub const AM65_CPSW_PN_REG_FIFO_STATUS: c_uint = 0x050;
pub const AM65_CPSW_PN_REG_EST_CTL: c_uint = 0x060;

// AM65_CPSW_REG_CTL register fields

// AM65_CPSW_PN_REG_CTL register fields

// AM65_CPSW_PN_REG_EST_CTL register fields

// AM65_CPSW_PN_REG_IET_CTRL register fields

pub const AM65_CPSW_PN_IET_MAC_MAC_ADDFRAGSIZE_OFFSET: c_int = 8;

pub const AM65_CPSW_PN_IET_MAC_PREMPT_OFFSET: c_int = 16;

// AM65_CPSW_PN_REG_IET_STATUS register fields

// AM65_CPSW_PN_REG_IET_VERIFY register fields

// 10 msec converted to NSEC

// AM65_CPSW_PN_REG_FIFO_STATUS register fields

// EST FETCH COMMAND RAM
pub const AM65_CPSW_FETCH_RAM_CMD_NUM: c_uint = 0x80;

pub const AM65_CPSW_FETCH_CNT_OFFSET: c_int = 8;

// AM65_CPSW_PN_REG_MAX_BLKS fields for IET and No IET cases
// 7 blocks for pn_rx_max_blks, 13 for pn_tx_max_blks
pub const AM65_CPSW_PN_TX_RX_MAX_BLKS_IET: c_uint = 0xD07;
// Slave IET Stats. register offsets
pub const AM65_CPSW_STATN_IET_RX_ASSEMBLY_ERROR: c_uint = 0x140;
pub const AM65_CPSW_STATN_IET_RX_ASSEMBLY_OK: c_uint = 0x144;
pub const AM65_CPSW_STATN_IET_RX_SMD_ERROR: c_uint = 0x148;
pub const AM65_CPSW_STATN_IET_RX_FRAG: c_uint = 0x14c;
pub const AM65_CPSW_STATN_IET_TX_HOLD: c_uint = 0x150;
pub const AM65_CPSW_STATN_IET_TX_FRAG: c_uint = 0x154;
// number of priority queues per port FIFO
pub const AM65_CPSW_PN_FIFO_PRIO_NUM: c_int = 8;
