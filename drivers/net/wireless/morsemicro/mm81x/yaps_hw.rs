//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/yaps_hw.h
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
//
// Copyright (c) 2017-2026 Morse Micro
//

pub const MM81X_INT_YAPS_FC_PKT_WAITING_IRQN: c_int = 0;
pub const MM81X_INT_YAPS_FC_PACKET_FREED_UP_IRQN: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_yaps_hw_table {
// NOTE: We need these padding bytes for yaps to work
    pub padding: [u8; 4],
    pub ysl_addr: __le32,
    pub yds_addr: __le32,
    pub status_regs_addr: __le32,
// Alloc pool sizes
    pub tc_tx_pool_size: __le16,
    pub fc_rx_pool_size: __le16,
    pub tc_cmd_pool_size: u8,
    pub tc_beacon_pool_size: u8,
    pub tc_mgmt_pool_size: u8,
    pub fc_resp_pool_size: u8,
    pub fc_tx_sts_pool_size: u8,
    pub fc_aux_pool_size: u8,
// To chip/from chip queue sizes
    pub tc_tx_q_size: u8,
    pub tc_cmd_q_size: u8,
    pub tc_beacon_q_size: u8,
    pub tc_mgmt_q_size: u8,
    pub fc_q_size: u8,
    pub fc_done_q_size: u8,
    pub yaps_reserved_page_size: __le16,
    pub reserved_unused: __le16,
    pub __packed: },
    pub mm81x: struct,
    pub enable): *mut *mut void mm81x_yaps_hw_enable_irqs(struct mm81x mors, bool,
    pub mors): *mut int mm81x_yaps_hw_init(struct mm81x,
    pub mors): *mut void mm81x_yaps_hw_finish(struct mm81x,
    pub tbl_ptr): *mut mm81x_yaps_hw_table,
