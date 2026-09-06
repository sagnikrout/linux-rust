//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_fw.h
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_fw_mbx {
    pub tail: u8 ready, head,,
    pub msg: *mut fbnic_tlv_msg,
    pub addr: dma_addr_t,
    pub buf_info: [}; FBNIC_IPC_MBX_DESC_LEN],
}

// FW_VER_MAX_SIZE must match ETHTOOL_FWVERS_LEN
pub const FBNIC_FW_VER_MAX_SIZE: c_int = 32;
// Formatted version is in the format XX.YY.ZZ_RRR_COMMIT

pub const FBNIC_FW_LOG_VERSION: c_int = 1;
pub const FBNIC_FW_LOG_MAX_SIZE: c_int = 256;
//
// The max amount of logs which can fit in a single mailbox message. Firmware
// assumes each mailbox message is 4096B. The amount of messages supported is
// calculated as 4096 minus headers for message, arrays, and length minus the
// size of length divided by headers for each array plus the maximum LOG size,
// and the size of MSEC and INDEX. Put another way:
//
// MAX_LOG_HISTORY = ((4096 - TLV_HDR_SZ * 5 - LENGTH_SZ)
// / (FBNIC_FW_LOG_MAX_SIZE + TLV_HDR_SZ * 3 + MSEC_SZ
// + INDEX_SZ))
//
pub const FBNIC_FW_MAX_LOG_HISTORY: c_int = 14;
pub const FBNIC_MBX_RX_TO_SEC: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_fw_ver {
    pub version: u32,
    pub commit: [c_char; FBNIC_FW_CAP_RESP_COMMIT_MAX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_fw_cap {
    pub bootloader: fbnic_fw_ver mgmt,,
    pub running: },
    pub undi: fbnic_fw_ver mgmt, bootloader,,
    pub stored: },
    pub active_slot: u8,
    pub bmc_mac_addr: [u8; 4][ETH_ALEN],
    pub 1: u8 bmc_present :,
    pub 1: u8 need_bmc_tcam_reinit :,
    pub 1: u8 need_bmc_macda_sync :,
    pub 1: u8 all_multi :,
    pub link_speed: u8,
    pub link_fec: u8,
    pub anti_rollback_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_fw_completion {
    pub msg_type: u32,
    pub done: completion,
    pub ref_count: kref,
    pub result: c_int,
    pub size: u32,
    pub coredump_info: },
    pub size: u32,
    pub stride: u16,
    pub data: [*mut u8; ],
    pub coredump: },
    pub offset: u32,
    pub length: u32,
    pub fw_update: },
    pub length: u16,
    pub offset: u8,
    pub page: u8,
    pub bank: u8,
    pub __counted_by(length): u8 data[] __aligned(sizeof(u32)),
    pub qsfp: },
    pub millivolts: i32,
    pub millidegrees: i32,
    pub tsene: },
    pub u: },
}

extern "C" {
    pub fn __fbnic_mbx_rd_desc(fbd: *mut fbnic_dev, mbx_idx: c_int, desc_idx: c_int) -> u64;
}
extern "C" {
    pub fn fbnic_mbx_init(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_mbx_clean(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_mbx_poll(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_mbx_poll_tx_ready(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_mbx_flush_tx(fbd: *mut fbnic_dev);
}
//
// enum fbnic_fw_self_test_codes - return codes from self test routines
//
// These are the codes returned from the self test routines and
// stored in the test result array indexed by the specific
// test name.
//
// @FBNIC_TEST_FW_SUCCESS: test success
// @FBNIC_TEST_FW_NO_FIRMWARE: FW interface not present
// @FBNIC_TEST_FW_NO_CMPL: No completion available
// @FBNIC_TEST_FW_NO_XMIT: Could not xmit message
// @FBNIC_TEST_FW_NO_MSG: no message returned
// @FBNIC_TEST_FW_PARSE: returned message had parsing error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbnic_fw_self_test_codes {
    FBNIC_TEST_FW_SUCCESS = 0,
    FBNIC_TEST_FW_NO_FIRMWARE = 10,
    FBNIC_TEST_FW_NO_CMPL = 20,
    FBNIC_TEST_FW_NO_XMIT = 30,
    FBNIC_TEST_FW_NO_MSG = 40,
    FBNIC_TEST_FW_PARSE = 50,
}

extern "C" {
    pub fn fbnic_fw_mbx_self_test(fbd: *mut fbnic_dev) -> fbnic_fw_self_test_codes;
}
extern "C" {
    pub fn fbnic_fw_xmit_ownership_msg(fbd: *mut fbnic_dev, take_ownership: bool) -> c_int;
}
extern "C" {
    pub fn fbnic_fw_init_heartbeat(fbd: *mut fbnic_dev, poll: bool) -> c_int;
}
extern "C" {
    pub fn fbnic_fw_check_heartbeat(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_fw_xmit_rpc_macda_sync(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_fw_put_cmpl(cmpl_data: *mut fbnic_fw_completion);
}

pub const FW_RPC_MAC_SYNC_RX_FLAGS_PROMISC: c_int = 1;
pub const FW_RPC_MAC_SYNC_RX_FLAGS_ALLMULTI: c_int = 2;
pub const FW_RPC_MAC_SYNC_RX_FLAGS_BROADCAST: c_int = 4;
pub const FW_RPC_MAC_SYNC_UC_ARRAY_SIZE: c_int = 8;
pub const FW_RPC_MAC_SYNC_MC_ARRAY_SIZE: c_int = 8;
