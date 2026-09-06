//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/eq.h
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
// Copyright (c) 2018-2021, Mellanox Technologies inc.  All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eq_tasklet {
    pub list: list_head,
    pub process_list: list_head,
    pub task: tasklet_struct,
    pub /: *mut *mut spinlock_t lock; / lock completion tasklet list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cq_table {
    pub /: *mut *mut spinlock_t lock; / protect radix tree,
    pub tree: radix_tree_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eq {
    pub fbc: mlx5_frag_buf_ctrl,
    pub frag_buf: mlx5_frag_buf,
    pub dev: *mut mlx5_core_dev,
    pub cq_table: mlx5_cq_table,
    pub doorbell: *mut __be32 __iomem,
    pub cons_index: u32,
    pub vecidx: c_uint,
    pub irqn: c_uint,
    pub eqn: u8,
    pub dbg: *mut mlx5_rsc_debug,
    pub irq: *mut mlx5_irq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eq_async {
    pub core: mlx5_eq,
    pub irq_nb: notifier_block,
    pub /: *mut *mut spinlock_t lock; / To avoid irq EQ handle races with resiliency flows,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eq_comp {
    pub core: mlx5_eq,
    pub irq_nb: notifier_block,
    pub tasklet_ctx: mlx5_eq_tasklet,
    pub list: list_head,
}

extern "C" {
    pub fn mlx5_frag_buf_get_wqe(_arg: &eq->fbc, _arg: entry) -> return;
}
// We still want ordering, just not swabbing, so add a barrier
extern "C" {
    pub fn mlx5_eq_table_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_eq_table_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_eq_table_create(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_eq_table_destroy(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_eq_add_cq(eq: *mut mlx5_eq, cq: *mut mlx5_core_cq) -> c_int;
}
extern "C" {
    pub fn mlx5_eq_del_cq(eq: *mut mlx5_eq, cq: *mut mlx5_core_cq);
}
extern "C" {
    pub fn mlx5_cq_tasklet_cb(t: *mut tasklet_struct);
}
extern "C" {
    pub fn mlx5_eq_poll_irq_disabled(eq: *mut mlx5_eq_comp) -> u32;
}
extern "C" {
    pub fn mlx5_cmd_eq_recover(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_eq_synchronize_async_irq(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_eq_synchronize_cmd_irq(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_debug_eq_add(dev: *mut mlx5_core_dev, eq: *mut mlx5_eq) -> c_int;
}
extern "C" {
    pub fn mlx5_debug_eq_remove(dev: *mut mlx5_core_dev, eq: *mut mlx5_eq);
}
extern "C" {
    pub fn mlx5_eq_debugfs_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_eq_debugfs_cleanup(dev: *mut mlx5_core_dev);
}
// This function should only be called after mlx5_cmd_force_teardown_hca
extern "C" {
    pub fn mlx5_core_eq_free_irqs(dev: *mut mlx5_core_dev);
}

extern "C" {
    pub fn mlx5_comp_irqn_get(dev: *mut mlx5_core_dev, vector: c_int, irqn: *mut c_uint) -> c_int;
}
