//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/sf/vhca_event.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2020 Mellanox Technologies Ltd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vhca_state_event {
    pub function_id: u16,
    pub sw_function_id: u16,
    pub new_vhca_state: u8,
}

extern "C" {
    pub fn MLX5_CAP_GEN_MAX(_arg: dev, _arg: vhca_state) -> return;
}
extern "C" {
    pub fn mlx5_vhca_state_cap_handle(dev: *mut mlx5_core_dev, set_hca_cap: *mut c_void);
}
extern "C" {
    pub fn mlx5_vhca_state_notifier_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_vhca_event_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_vhca_event_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_vhca_event_start(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_vhca_event_stop(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_vhca_event_notifier_register(dev: *mut mlx5_core_dev, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mlx5_vhca_event_notifier_unregister(dev: *mut mlx5_core_dev, nb: *mut notifier_block);
}
extern "C" {
    pub fn mlx5_modify_vhca_sw_id(dev: *mut mlx5_core_dev, function_id: u16, sw_fn_id: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_vhca_event_arm(dev: *mut mlx5_core_dev, function_id: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_vhca_events_work_enqueue(dev: *mut mlx5_core_dev, idx: c_int, work: *mut work_struct);
}
extern "C" {
    pub fn mlx5_vhca_event_work_queues_flush(dev: *mut mlx5_core_dev);
}

