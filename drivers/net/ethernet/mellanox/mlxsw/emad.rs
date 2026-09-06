//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/emad.h
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

pub const MLXSW_EMAD_MAX_RETRY: c_int = 5;
// EMAD Ethernet header
pub const MLXSW_EMAD_ETH_HDR_LEN: c_uint = 0x10	/* Length in u8 */;

pub const MLXSW_EMAD_EH_ETHERTYPE: c_uint = 0x8932;
pub const MLXSW_EMAD_EH_MLX_PROTO: c_int = 0;
pub const MLXSW_EMAD_EH_PROTO_VERSION: c_int = 0;
// EMAD TLV Types
// OP TLV

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_emad_op_tlv_status {
    MLXSW_EMAD_OP_TLV_STATUS_SUCCESS,
    MLXSW_EMAD_OP_TLV_STATUS_BUSY,
    MLXSW_EMAD_OP_TLV_STATUS_VERSION_NOT_SUPPORTED,
    MLXSW_EMAD_OP_TLV_STATUS_UNKNOWN_TLV,
    MLXSW_EMAD_OP_TLV_STATUS_REGISTER_NOT_SUPPORTED,
    MLXSW_EMAD_OP_TLV_STATUS_CLASS_NOT_SUPPORTED,
    MLXSW_EMAD_OP_TLV_STATUS_METHOD_NOT_SUPPORTED,
    MLXSW_EMAD_OP_TLV_STATUS_BAD_PARAMETER,
    MLXSW_EMAD_OP_TLV_STATUS_RESOURCE_NOT_AVAILABLE,
    MLXSW_EMAD_OP_TLV_STATUS_MESSAGE_RECEIPT_ACK,
    MLXSW_EMAD_OP_TLV_STATUS_INTERNAL_ERROR = 0x70,
}

// STRING TLV

// LATENCY TLV

// END TLV

