//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/txheader.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2015-2018 Mellanox Technologies. All rights reserved
// tx_hdr_version
// Tx header version.
// Must be set to 1.
//
// tx_hdr_ctl
// Packet control type.
// 0 - Ethernet control (e.g. EMADs, LACP)
// 1 - Ethernet data
//
// tx_hdr_proto
// Packet protocol type. Must be set to 1 (Ethernet).
//
// tx_hdr_rx_is_router
// Packet is sent from the router. Valid for data packets only.
//
// tx_hdr_fid_valid
// Indicates if the 'fid' field is valid and should be used for
// forwarding lookup. Valid for data packets only.
//
// tx_hdr_swid
// Switch partition ID. Must be set to 0.
//
// tx_hdr_control_tclass
// Indicates if the packet should use the control TClass and not one
// of the data TClasses.
//
// tx_hdr_port_mid
// Destination local port for unicast packets.
// Destination multicast ID for multicast packets.
//
// Control packets are directed to a specific egress port, while data
// packets are transmitted through the CPU port (0) into the switch partition,
// where forwarding rules are applied.
//
// tx_hdr_fid
// Forwarding ID used for L2 forwarding lookup. Valid only if 'fid_valid' is
// set, otherwise calculated based on the packet's VID using VID to FID mapping.
// Valid for data packets only.
//
// tx_hdr_type
// 0 - Data packets
// 6 - Control packets
//
pub const MLXSW_TXHDR_LEN: c_uint = 0x10;
pub const MLXSW_TXHDR_VERSION_0: c_int = 0;
pub const MLXSW_TXHDR_VERSION_1: c_int = 1;
pub const MLXSW_TXHDR_PROTO_ETH: c_int = 1;
pub const MLXSW_TXHDR_CTCLASS3: c_int = 0;
pub const MLXSW_TXHDR_CPU_SIG: c_int = 0;
pub const MLXSW_TXHDR_SIG: c_uint = 0xE0E0;
pub const MLXSW_TXHDR_STCLASS_NONE: c_int = 0;
