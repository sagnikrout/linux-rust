//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/mlx5/core/mlx5_vdpa.h
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
// Copyright (c) 2020 Mellanox Technologies Ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_direct_mr {
    pub start: u64,
    pub end: u64,
    pub perm: u32,
    pub mr: u32,
    pub sg_head: sg_table,
    pub log_size: c_int,
    pub nsg: c_int,
    pub nent: c_int,
    pub list: list_head,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_mr {
    pub mkey: u32,
// list of direct MRs descendants of this indirect mr
    pub head: list_head,
    pub num_directs: c_ulong,
    pub num_klms: c_ulong,
    pub iotlb: *mut vhost_iotlb,
    pub user_mr: bool,
    pub refcount: refcount_t,
    pub mr_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_resources {
    pub pdn: u32,
    pub uar: *mut mlx5_uars_page,
    pub kick_addr: *mut void __iomem,
    pub phys_kick_addr: u64,
    pub uid: u16,
    pub null_mkey: u32,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_control_vq {
    pub iotlb: *mut vhost_iotlb,
// spinlock to synchronize iommu table
    pub iommu_lock: spinlock_t,
    pub vring: vringh,
    pub ready: bool,
    pub desc_addr: u64,
    pub device_addr: u64,
    pub driver_addr: u64,
    pub event_cb: vdpa_callback,
    pub riov: vringh_kiov,
    pub wiov: vringh_kiov,
    pub head: c_ushort,
    pub received_desc: c_uint,
    pub completed_desc: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_wq_ent {
    pub work: work_struct,
    pub mvdev: *mut mlx5_vdpa_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_mr_resources {
    pub mr: [*mut mlx5_vdpa_mr; MLX5_VDPA_NUM_AS],
    pub group2asid: [c_uint; MLX5_VDPA_NUMVQ_GROUPS],
// Pre-deletion mr list
    pub mr_list_head: list_head,
// Deferred mr list
    pub mr_gc_list_head: list_head,
    pub wq_gc: *mut workqueue_struct,
    pub gc_dwork_ent: delayed_work,
    pub lock: mutex,
    pub shutdown: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_dev {
    pub vdev: vdpa_device,
    pub mdev: *mut mlx5_core_dev,
    pub res: mlx5_vdpa_resources,
    pub mres: mlx5_vdpa_mr_resources,
    pub mlx_features: u64,
    pub actual_features: u64,
    pub status: u8,
    pub max_vqs: u32,
    pub max_idx: u16,
    pub generation: u32,
    pub cvq: mlx5_control_vq,
    pub wq: *mut workqueue_struct,
    pub suspended: bool,
    pub async_ctx: mlx5_async_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_async_cmd {
    pub err: c_int,
    pub cb_work: mlx5_async_work,
    pub cmd_done: completion,
    pub in: *mut c_void,
    pub inlen: usize,
    pub out: *mut c_void,
    pub outlen: usize,
}

extern "C" {
    pub fn mlx5_vdpa_create_tis(mvdev: *mut mlx5_vdpa_dev, in: *mut c_void, tisn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_destroy_tis(mvdev: *mut mlx5_vdpa_dev, tisn: u32);
}
extern "C" {
    pub fn mlx5_vdpa_create_rqt(mvdev: *mut mlx5_vdpa_dev, in: *mut c_void, inlen: c_int, rqtn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_modify_rqt(mvdev: *mut mlx5_vdpa_dev, in: *mut c_void, inlen: c_int, rqtn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_destroy_rqt(mvdev: *mut mlx5_vdpa_dev, rqtn: u32);
}
extern "C" {
    pub fn mlx5_vdpa_create_tir(mvdev: *mut mlx5_vdpa_dev, in: *mut c_void, tirn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_destroy_tir(mvdev: *mut mlx5_vdpa_dev, tirn: u32);
}
extern "C" {
    pub fn mlx5_vdpa_alloc_transport_domain(mvdev: *mut mlx5_vdpa_dev, tdn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_dealloc_transport_domain(mvdev: *mut mlx5_vdpa_dev, tdn: u32);
}
extern "C" {
    pub fn mlx5_vdpa_alloc_resources(mvdev: *mut mlx5_vdpa_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_free_resources(mvdev: *mut mlx5_vdpa_dev);
}
extern "C" {
    pub fn mlx5_vdpa_destroy_mkey(mvdev: *mut mlx5_vdpa_dev, mkey: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_init_mr_resources(mvdev: *mut mlx5_vdpa_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_destroy_mr_resources(mvdev: *mut mlx5_vdpa_dev);
}
extern "C" {
    pub fn mlx5_vdpa_clean_mrs(mvdev: *mut mlx5_vdpa_dev);
}
extern "C" {
    pub fn mlx5_vdpa_create_dma_mr(mvdev: *mut mlx5_vdpa_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_vdpa_reset_mr(mvdev: *mut mlx5_vdpa_dev, asid: c_uint) -> c_int;
}

