//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/port.h
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

pub const MLXSW_PORT_DEFAULT_VID: c_int = 1;
pub const MLXSW_PORT_SWID_DISABLED_PORT: c_int = 255;
pub const MLXSW_PORT_SWID_ALL_SWIDS: c_int = 254;
pub const MLXSW_PORT_SWID_TYPE_IB: c_int = 1;
pub const MLXSW_PORT_SWID_TYPE_ETH: c_int = 2;
pub const MLXSW_PORT_MAX_IB_PHY_PORTS: c_int = 36;

pub const MLXSW_PORT_CPU_PORT: c_uint = 0x0;
pub const MLXSW_PORT_DONT_CARE: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_port_admin_status {
    MLXSW_PORT_ADMIN_STATUS_UP = 1,
    MLXSW_PORT_ADMIN_STATUS_DOWN = 2,
    MLXSW_PORT_ADMIN_STATUS_UP_ONCE = 3,
    MLXSW_PORT_ADMIN_STATUS_DISABLED = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_reg_pude_oper_status {
    MLXSW_PORT_OPER_STATUS_UP = 1,
    MLXSW_PORT_OPER_STATUS_DOWN = 2,
    MLXSW_PORT_OPER_STATUS_FAILURE = 4,	/* Can be set to up again. */
}
