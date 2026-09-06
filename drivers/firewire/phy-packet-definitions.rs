//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firewire/phy-packet-definitions.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// phy-packet-definitions.h - The definitions of phy packet for IEEE 1394.
//
// Copyright (c) 2024 Takashi Sakamoto
pub const PACKET_IDENTIFIER_MASK: c_uint = 0xc0000000;
pub const PACKET_IDENTIFIER_SHIFT: c_int = 30;
// quadlet &= ~PACKET_IDENTIFIER_MASK;
// quadlet |= (packet_identifier << PACKET_IDENTIFIER_SHIFT) & PACKET_IDENTIFIER_MASK;
pub const PHY_PACKET_PACKET_IDENTIFIER_PHY_CONFIG: c_int = 0;
pub const PHY_CONFIG_ROOT_ID_MASK: c_uint = 0x3f000000;
pub const PHY_CONFIG_ROOT_ID_SHIFT: c_int = 24;
pub const PHY_CONFIG_FORCE_ROOT_NODE_MASK: c_uint = 0x00800000;
pub const PHY_CONFIG_FORCE_ROOT_NODE_SHIFT: c_int = 23;
pub const PHY_CONFIG_GAP_COUNT_OPTIMIZATION_MASK: c_uint = 0x00400000;
pub const PHY_CONFIG_GAP_COUNT_OPTIMIZATION_SHIFT: c_int = 22;
pub const PHY_CONFIG_GAP_COUNT_MASK: c_uint = 0x003f0000;
pub const PHY_CONFIG_GAP_COUNT_SHIFT: c_int = 16;
// quadlet &= ~PHY_CONFIG_ROOT_ID_MASK;
// quadlet |= (root_id << PHY_CONFIG_ROOT_ID_SHIFT) & PHY_CONFIG_ROOT_ID_MASK;
// quadlet &= ~PHY_CONFIG_FORCE_ROOT_NODE_MASK;
// quadlet |= (has_force_root_node << PHY_CONFIG_FORCE_ROOT_NODE_SHIFT) & PHY_CONFIG_FORCE_ROOT_NODE_MASK;
// quadlet &= ~PHY_CONFIG_GAP_COUNT_OPTIMIZATION_MASK;
// quadlet |= (has_gap_count_optimization << PHY_CONFIG_GAP_COUNT_OPTIMIZATION_SHIFT) & PHY_CONFIG_GAP_COUNT_OPTIMIZATION_MASK;
// quadlet &= ~PHY_CONFIG_GAP_COUNT_MASK;
// quadlet |= (gap_count << PHY_CONFIG_GAP_COUNT_SHIFT) & PHY_CONFIG_GAP_COUNT_MASK;
pub const PHY_PACKET_PACKET_IDENTIFIER_SELF_ID: c_int = 2;
pub const SELF_ID_PHY_ID_MASK: c_uint = 0x3f000000;
pub const SELF_ID_PHY_ID_SHIFT: c_int = 24;
pub const SELF_ID_EXTENDED_MASK: c_uint = 0x00800000;
pub const SELF_ID_EXTENDED_SHIFT: c_int = 23;
pub const SELF_ID_MORE_PACKETS_MASK: c_uint = 0x00000001;
pub const SELF_ID_MORE_PACKETS_SHIFT: c_int = 0;
pub const SELF_ID_ZERO_LINK_ACTIVE_MASK: c_uint = 0x00400000;
pub const SELF_ID_ZERO_LINK_ACTIVE_SHIFT: c_int = 22;
pub const SELF_ID_ZERO_GAP_COUNT_MASK: c_uint = 0x003f0000;
pub const SELF_ID_ZERO_GAP_COUNT_SHIFT: c_int = 16;
pub const SELF_ID_ZERO_SCODE_MASK: c_uint = 0x0000c000;
pub const SELF_ID_ZERO_SCODE_SHIFT: c_int = 14;
pub const SELF_ID_ZERO_CONTENDER_MASK: c_uint = 0x00000800;
pub const SELF_ID_ZERO_CONTENDER_SHIFT: c_int = 11;
pub const SELF_ID_ZERO_POWER_CLASS_MASK: c_uint = 0x00000700;
pub const SELF_ID_ZERO_POWER_CLASS_SHIFT: c_int = 8;
pub const SELF_ID_ZERO_INITIATED_RESET_MASK: c_uint = 0x00000002;
pub const SELF_ID_ZERO_INITIATED_RESET_SHIFT: c_int = 1;
pub const SELF_ID_EXTENDED_SEQUENCE_MASK: c_uint = 0x00700000;
pub const SELF_ID_EXTENDED_SEQUENCE_SHIFT: c_int = 20;
pub const SELF_ID_PORT_STATUS_MASK: c_uint = 0x3;
pub const SELF_ID_SEQUENCE_MAXIMUM_QUADLET_COUNT: c_int = 4;
// quadlet &= ~SELF_ID_PHY_ID_MASK;
// quadlet |= (phy_id << SELF_ID_PHY_ID_SHIFT) & SELF_ID_PHY_ID_MASK;
// quadlet &= ~SELF_ID_EXTENDED_MASK;
// quadlet |= (extended << SELF_ID_EXTENDED_SHIFT) & SELF_ID_EXTENDED_MASK;
// quadlet &= ~SELF_ID_ZERO_LINK_ACTIVE_MASK;
// quadlet |= (is_active << SELF_ID_ZERO_LINK_ACTIVE_SHIFT) & SELF_ID_ZERO_LINK_ACTIVE_MASK;
// quadlet &= ~SELF_ID_ZERO_GAP_COUNT_MASK;
// quadlet |= (gap_count << SELF_ID_ZERO_GAP_COUNT_SHIFT) & SELF_ID_ZERO_GAP_COUNT_MASK;
// quadlet &= ~SELF_ID_ZERO_SCODE_MASK;
// quadlet |= (speed << SELF_ID_ZERO_SCODE_SHIFT) & SELF_ID_ZERO_SCODE_MASK;
// quadlet &= ~SELF_ID_ZERO_CONTENDER_MASK;
// quadlet |= (is_contender << SELF_ID_ZERO_CONTENDER_SHIFT) & SELF_ID_ZERO_CONTENDER_MASK;
// quadlet &= ~SELF_ID_ZERO_POWER_CLASS_MASK;
// quadlet |= (power_class << SELF_ID_ZERO_POWER_CLASS_SHIFT) & SELF_ID_ZERO_POWER_CLASS_MASK;
// quadlet &= ~SELF_ID_ZERO_INITIATED_RESET_MASK;
// quadlet |= (is_initiated_reset << SELF_ID_ZERO_INITIATED_RESET_SHIFT) & SELF_ID_ZERO_INITIATED_RESET_MASK;
// quadlet &= ~SELF_ID_MORE_PACKETS_MASK;
// quadlet |= (is_more_packets << SELF_ID_MORE_PACKETS_SHIFT) & SELF_ID_MORE_PACKETS_MASK;
// quadlet &= ~SELF_ID_EXTENDED_SEQUENCE_MASK;
// quadlet |= (sequence << SELF_ID_EXTENDED_SHIFT) & SELF_ID_EXTENDED_SEQUENCE_MASK;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct self_id_sequence_enumerator {
    pub cursor: *const u32,
    pub quadlet_count: c_uint,
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODATA) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EPROTO) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EPROTO) -> return;
}
// quadlet_count = count;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_packet_self_id_port_status {
    PHY_PACKET_SELF_ID_PORT_STATUS_NONE = 0,
    PHY_PACKET_SELF_ID_PORT_STATUS_NCONN = 1,
    PHY_PACKET_SELF_ID_PORT_STATUS_PARENT = 2,
    PHY_PACKET_SELF_ID_PORT_STATUS_CHILD = 3,
}
