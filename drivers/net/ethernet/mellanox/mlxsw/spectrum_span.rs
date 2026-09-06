//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_span.h
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
// Copyright (c) 2018 Mellanox Technologies. All rights reserved

// SPAN session identifiers that correspond to MLXSW_TRAP_ID_MIRROR_SESSION<i>
// trap identifiers. The session identifier is an attribute of the SPAN agent,
// which determines the trap identifier of packets that are mirrored to the
// CPU. Packets that are trapped to the CPU for the same logical reason (e.g.,
// buffer drops) should use the same session identifier.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_span_session_id {
    MLXSW_SP_SPAN_SESSION_ID_BUFFER,
    MLXSW_SP_SPAN_SESSION_ID_SAMPLING,

    __MLXSW_SP_SPAN_SESSION_ID_MAX = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_span_parms {
    pub /: *mut *mut *mut mlxsw_sp_port dest_port; / NULL for unoffloaded SPAN.,
    pub ttl: c_uint,
    pub dmac: [c_uchar; ETH_ALEN],
    pub smac: [c_uchar; ETH_ALEN],
    pub daddr: mlxsw_sp_l3addr,
    pub saddr: mlxsw_sp_l3addr,
    pub vid: u16,
    pub policer_id: u16,
    pub policer_enable: bool,
    pub session_id: mlxsw_sp_span_session_id,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_span_trigger {
    MLXSW_SP_SPAN_TRIGGER_INGRESS,
    MLXSW_SP_SPAN_TRIGGER_EGRESS,
    MLXSW_SP_SPAN_TRIGGER_TAIL_DROP,
    MLXSW_SP_SPAN_TRIGGER_EARLY_DROP,
    MLXSW_SP_SPAN_TRIGGER_ECN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_span_trigger_parms {
    pub span_id: c_int,
    pub probability_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_span_agent_parms {
    pub to_dev: *const net_device,
    pub policer_id: u16,
    pub policer_enable: bool,
    pub session_id: mlxsw_sp_span_session_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_span_ops {
    pub mlxsw_sp): *mut *mut int (init)(struct mlxsw_sp,
    pub policer_id_base): u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_span_entry {
    pub to_dev: *const net_device,
    pub ops: *const mlxsw_sp_span_entry_ops,
    pub parms: mlxsw_sp_span_parms,
    pub ref_count: refcount_t,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_span_entry_ops {
    pub is_static: bool,
    pub to_dev): *const *const bool (can_handle)(struct net_device,
    pub sparmsp): *mut mlxsw_sp_span_parms,
    pub sparms): mlxsw_sp_span_parms,
    pub span_entry): *mut *mut void (deconfigure)(struct mlxsw_sp_span_entry,
}

extern "C" {
    pub fn mlxsw_sp_span_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_span_fini(mlxsw_sp: *mut mlxsw_sp);
}
extern "C" {
    pub fn mlxsw_sp_span_respin(mlxsw_sp: *mut mlxsw_sp);
}
extern "C" {
    pub fn mlxsw_sp_span_agent_put(mlxsw_sp: *mut mlxsw_sp, span_id: c_int);
}
extern "C" {
    pub fn mlxsw_sp_span_trigger_is_ingress(trigger: mlxsw_sp_span_trigger) -> bool;
}
