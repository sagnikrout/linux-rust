//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/rmnet/rmnet_map.h
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
// Copyright (c) 2013-2018, 2021, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_map_control_command {
    pub command_name: u8,
    pub cmd_type:2: u8,
    pub reserved:6: u8,
    pub reserved2: u16,
    pub transaction_id: u32,
    pub ip_family:2: u16,
    pub reserved:14: u16,
    pub flow_control_seq_num: __be16,
    pub qos_id: __be32,
    pub flow_control: },
    pub data): DECLARE_FLEX_ARRAY(u8,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rmnet_map_commands {
    RMNET_MAP_COMMAND_NONE,
    RMNET_MAP_COMMAND_FLOW_DISABLE,
    RMNET_MAP_COMMAND_FLOW_ENABLE,
// These should always be the last 2 elements
    RMNET_MAP_COMMAND_UNKNOWN,
    RMNET_MAP_COMMAND_ENUM_LENGTH
}

pub const RMNET_MAP_COMMAND_REQUEST: c_int = 0;
pub const RMNET_MAP_COMMAND_ACK: c_int = 1;
pub const RMNET_MAP_COMMAND_UNSUPPORTED: c_int = 2;
pub const RMNET_MAP_COMMAND_INVALID: c_int = 3;
pub const RMNET_MAP_NO_PAD_BYTES: c_int = 0;
pub const RMNET_MAP_ADD_PAD_BYTES: c_int = 1;
extern "C" {
    pub fn rmnet_map_command(skb: *mut sk_buff, port: *mut rmnet_port);
}
extern "C" {
    pub fn rmnet_map_checksum_downlink_packet(skb: *mut sk_buff, len: u16) -> c_int;
}
extern "C" {
    pub fn rmnet_map_process_next_hdr_packet(skb: *mut sk_buff, len: u16) -> c_int;
}
extern "C" {
    pub fn rmnet_map_tx_aggregate_init(port: *mut rmnet_port);
}
extern "C" {
    pub fn rmnet_map_tx_aggregate_exit(port: *mut rmnet_port);
}
extern "C" {
    pub fn rmnet_map_validate_packet_len(skb: *mut sk_buff, port: *mut rmnet_port) -> u32;
}
