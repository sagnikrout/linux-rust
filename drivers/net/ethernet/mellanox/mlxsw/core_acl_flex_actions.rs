//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/core_acl_flex_actions.h
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
// Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_afa_ops {
    pub is_first): *mut *mut char enc_actions, bool,
    pub is_first): *mut *mut *mut void (kvdl_set_del)(void priv, u32 kvdl_index, bool,
    pub activity): *mut bool,
    pub local_port): *mut *mut *mut *mut int (kvdl_fwd_entry_add)(void priv, u32 p_kvdl_index, u16,
    pub kvdl_index): *mut *mut *mut void (kvdl_fwd_entry_del)(void priv, u32,
    pub p_counter_index): *mut *mut *mut int (counter_index_get)(void priv, unsigned int,
    pub counter_index): *mut *mut *mut void (counter_index_put)(void priv, unsigned int,
    pub p_span_id): *mut bool ingress, int,
    pub ingress): bool,
    pub extack): *mut netlink_ext_ack,
    pub policer_index): *mut *mut *mut void (policer_del)(void priv, u16,
    pub extack): *mut *mut int p_span_id, struct netlink_ext_ack,
    pub ingress): bool,
    pub dummy_first_set: bool,
}

extern "C" {
    pub fn mlxsw_afa_destroy(mlxsw_afa: *mut mlxsw_afa);
}
extern "C" {
    pub fn mlxsw_afa_block_destroy(block: *mut mlxsw_afa_block);
}
extern "C" {
    pub fn mlxsw_afa_block_commit(block: *mut mlxsw_afa_block) -> c_int;
}
extern "C" {
    pub fn mlxsw_afa_block_first_kvdl_index(block: *mut mlxsw_afa_block) -> u32;
}
extern "C" {
    pub fn mlxsw_afa_block_activity_get(block: *mut mlxsw_afa_block, activity: *mut bool) -> c_int;
}
extern "C" {
    pub fn mlxsw_afa_block_continue(block: *mut mlxsw_afa_block) -> c_int;
}
extern "C" {
    pub fn mlxsw_afa_block_jump(block: *mut mlxsw_afa_block, group_id: u16) -> c_int;
}
extern "C" {
    pub fn mlxsw_afa_block_terminate(block: *mut mlxsw_afa_block) -> c_int;
}
extern "C" {
    pub fn mlxsw_afa_block_append_trap(block: *mut mlxsw_afa_block, trap_id: u16) -> c_int;
}
