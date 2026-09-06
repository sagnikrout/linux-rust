//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/devcom.h
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
// Copyright (c) 2018 Mellanox Technologies

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_devom_match_flags {
    MLX5_DEVCOM_MATCH_FLAGS_NS = BIT(0),
}

pub const MLX5_DEVCOM_MATCH_KEY_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_devcom_match_key {
    pub val: u64,
    pub buf: [u8; MLX5_DEVCOM_MATCH_KEY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_devcom_match_attr {
    pub flags: u32,
    pub key: mlx5_devcom_match_key,
    pub net: *mut net,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_devcom_component {
    MLX5_DEVCOM_ESW_OFFLOADS,
    MLX5_DEVCOM_MPV,
    MLX5_DEVCOM_HCA_PORTS,
    MLX5_DEVCOM_SD_GROUP,
    MLX5_DEVCOM_SHARED_CLOCK,
    MLX5_DEVCOM_NUM_COMPONENTS,
}

extern "C" {
    pub fn mlx5_devcom_unregister_device(devc: *mut mlx5_devcom_dev);
}
extern "C" {
    pub fn mlx5_devcom_unregister_component(devcom: *mut mlx5_devcom_comp_dev);
}

extern "C" {
    pub fn mlx5_devcom_comp_get_size(devcom: *mut mlx5_devcom_comp_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_devcom_comp_set_ready(devcom: *mut mlx5_devcom_comp_dev, ready: bool);
}
extern "C" {
    pub fn mlx5_devcom_comp_is_ready(devcom: *mut mlx5_devcom_comp_dev) -> bool;
}
extern "C" {
    pub fn mlx5_devcom_for_each_peer_begin(devcom: *mut mlx5_devcom_comp_dev) -> bool;
}
extern "C" {
    pub fn mlx5_devcom_for_each_peer_end(devcom: *mut mlx5_devcom_comp_dev);
}

extern "C" {
    pub fn mlx5_devcom_comp_lock(devcom: *mut mlx5_devcom_comp_dev);
}
extern "C" {
    pub fn mlx5_devcom_comp_unlock(devcom: *mut mlx5_devcom_comp_dev);
}
extern "C" {
    pub fn mlx5_devcom_comp_trylock(devcom: *mut mlx5_devcom_comp_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_devcom_comp_assert_locked(devcom: *mut mlx5_devcom_comp_dev);
}
