//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/rx.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024-2026 Intel Corporation
//

// Macro flag: #define __iwl_mld_rx_h__

//
// enum iwl_mld_internal_rxq_notif_type - RX queue sync notif types
//
// @IWL_MLD_RXQ_EMPTY: empty sync notification
// @IWL_MLD_RXQ_NOTIF_DEL_BA: notify RSS queues of delBA
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_internal_rxq_notif_type {
    IWL_MLD_RXQ_EMPTY,
    IWL_MLD_RXQ_NOTIF_DEL_BA,
}

//
// struct iwl_mld_internal_rxq_notif - @iwl_rxq_sync_cmd internal data.
// This data is echoed by the firmware to all RSS queues and should be DWORD
// aligned. FW is agnostic to the data, so there are no endianness requirements
//
// @type: one of &iwl_mld_internal_rxq_notif_type
// @cookie: unique internal cookie to identify old notifications
// @reserved: reserved for alignment
// @payload: data to send to RX queues based on the type (may be empty)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_internal_rxq_notif {
    pub type: u8,
    pub reserved: [u8; 3],
    pub cookie: u32,
    pub payload: [u8; ],
    pub __packed: },
//
// struct iwl_mld_rx_queues_sync - RX queues sync data
//
// @waitq: wait queue for RX queues sync completion
// @cookie: unique id to correlate sync requests with responses
// @state: bitmask representing the sync state of RX queues
// all RX queues bits are set before sending the command, and the
// corresponding queue bit cleared upon handling the notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_rx_queues_sync {
    pub waitq: wait_queue_head_t,
    pub cookie: u32,
    pub state: c_ulong,
}

