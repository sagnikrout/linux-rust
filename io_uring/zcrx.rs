//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/zcrx.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_zcrx_mem {
    pub size: c_ulong,
    pub is_dmabuf: bool,
    pub pages: *mut page,
    pub nr_folios: c_ulong,
    pub page_sg_table: sg_table,
    pub account_pages: c_ulong,
    pub sgt: *mut sg_table,
    pub attach: *mut dma_buf_attachment,
    pub dmabuf: *mut dma_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_zcrx_area {
    pub nia: net_iov_area,
    pub ifq: *mut io_zcrx_ifq,
    pub user_refs: *mut core::sync::atomic::AtomicI32,
    pub is_mapped: bool,
    pub area_id: u16,
// freelist
    pub free_count: u32,
    pub freelist: *mut u32,
    pub mem: io_zcrx_mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_rq_hdr {
    pub ____cacheline_aligned_in_smp: u32 head,
    pub ____cacheline_aligned_in_smp: u32 tail,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_rq {
    pub lock: spinlock_t,
    pub ring: *mut zcrx_rq_hdr,
    pub rqes: *mut io_uring_zcrx_rqe,
    pub cached_head: u32,
    pub cached_tail: u32,
    pub nr_entries: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_zcrx_ifq {
// read-protected by any of: ->pp_lock, ->alloc_lock, ->rq.lock
    pub areas: *mut io_zcrx_area,
    pub nr_areas: unsigned,
    pub niov_shift: unsigned,
    pub user: *mut user_struct,
    pub mm_account: *mut mm_struct,
    pub kern_readable: bool,
    pub ____cacheline_aligned_in_smp: zcrx_rq rq,
    pub ____cacheline_aligned_in_smp: spinlock_t alloc_lock,
    pub if_rxq: u32,
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub netdev_tracker: netdevice_tracker,
    pub refs: refcount_t,
// counts userspace facing users like io_uring
    pub user_refs: refcount_t,
//
// Page pool and net configuration lock, can be taken deeper in the
// net stack.
//
    pub pp_lock: mutex,
    pub rq_region: io_mapped_region,
    pub ctx_lock: spinlock_t,
    pub master_ctx: *mut io_ring_ctx,
    pub allowed_notif_mask: u32,
    pub fired_notifs: u32,
    pub notif_data: u64,
    pub notif_stats: *mut zcrx_stats,
}

extern "C" {
    pub fn io_zcrx_ctrl(ctx: *mut io_ring_ctx, arg: *mut void __user, nr_arg: unsigned) -> c_int;
}
extern "C" {
    pub fn io_unregister_zcrx(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_terminate_zcrx(ctx: *mut io_ring_ctx);
}

extern "C" {
    pub fn io_recvzc(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_recvzc_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
