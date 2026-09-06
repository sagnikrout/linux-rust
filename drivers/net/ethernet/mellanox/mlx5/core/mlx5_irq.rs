//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/mlx5_irq.h
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
// Copyright (c) 2021 Mellanox Technologies.

pub const MLX5_COMP_EQS_PER_SF: c_int = 8;
extern "C" {
    pub fn mlx5_irq_table_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_irq_table_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_irq_table_create(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_irq_table_destroy(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_irq_table_free_irqs(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_irq_table_get_num_comp(table: *mut mlx5_irq_table) -> c_int;
}
extern "C" {
    pub fn mlx5_irq_table_get_sfs_vec(table: *mut mlx5_irq_table) -> c_int;
}
extern "C" {
    pub fn mlx5_get_default_msix_vec_count(dev: *mut mlx5_core_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_ctrl_irq_release(dev: *mut mlx5_core_dev, ctrl_irq: *mut mlx5_irq);
}
extern "C" {
    pub fn mlx5_irq_release_vector(irq: *mut mlx5_irq);
}
extern "C" {
    pub fn mlx5_irq_attach_nb(irq: *mut mlx5_irq, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mlx5_irq_detach_nb(irq: *mut mlx5_irq, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mlx5_irq_get_index(irq: *mut mlx5_irq) -> c_int;
}
extern "C" {
    pub fn mlx5_irq_get_irq(irq: *const mlx5_irq) -> c_int;
}

extern "C" {
    pub fn mlx5_irq_affinity_irq_release(dev: *mut mlx5_core_dev, irq: *mut mlx5_irq);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

