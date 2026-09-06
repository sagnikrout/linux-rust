//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/rtrs/rtrs-srv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// RDMA Transport Layer
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

//
// enum rtrs_srv_state - Server states.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_srv_state {
    RTRS_SRV_CONNECTING,
    RTRS_SRV_CONNECTED,
    RTRS_SRV_CLOSING,
    RTRS_SRV_CLOSED,
}

// stats for Read and write operation.
// see Documentation/ABI/testing/sysfs-class-rtrs-server for details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_stats_rdma_stats {
    pub cnt: u64,
    pub size_total: u64,
    pub dir: [}; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_stats {
    pub kobj_stats: kobject,
    pub rdma_stats: *mut rtrs_srv_stats_rdma_stats __percpu,
    pub srv_path: *mut rtrs_srv_path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_con {
    pub c: rtrs_con,
    pub rsp_wr_wait_list: list_head,
    pub rsp_wr_wait_lock: spinlock_t,
}

// IO context in rtrs_srv, each io has one
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_op {
    pub con: *mut rtrs_srv_con,
    pub msg_id: u32,
    pub dir: u8,
    pub rd_msg: *mut rtrs_msg_rdma_read,
    pub tx_wr: ib_rdma_wr,
    pub tx_sg: ib_sge,
    pub wait_list: list_head,
    pub status: c_int,
}

//
// server side memory region context, when always_invalidate=Y, we need
// queue_depth of memory region to invalidate each memory region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_mr {
    pub mr: *mut ib_mr,
    pub sgt: sg_table,
    pub /: *mut *mut ib_cqe inv_cqe; / only for always_invalidate=true,
    pub /: *mut *mut u32 msg_id; / only for always_invalidate=true,
    pub /: *mut *mut u32 msg_off; / only for always_invalidate=true,
    pub /: *mut *mut *mut rtrs_iu iu; / send buffer for new rkey msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_path {
    pub s: rtrs_path,
    pub srv: *mut rtrs_srv_sess,
    pub close_work: work_struct,
    pub state: rtrs_srv_state,
    pub state_lock: spinlock_t,
    pub cur_cq_vector: c_int,
    pub ops_ids: *mut rtrs_srv_op,
    pub ids_inflight_ref: percpu_ref,
    pub complete_done: completion,
    pub mrs: *mut rtrs_srv_mr,
    pub mrs_num: c_uint,
    pub dma_addr: *mut dma_addr_t,
    pub established: bool,
    pub mem_bits: c_uint,
    pub kobj: kobject,
    pub stats: *mut rtrs_srv_stats,
    pub connection_timeout: c_ulong,
}

extern "C" {
    pub fn container_of(_arg: s, rtrs_srv_path: struct, _arg: s) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_sess {
    pub paths_list: list_head,
    pub paths_up: c_int,
    pub paths_ev_mutex: mutex,
    pub paths_num: usize,
    pub paths_mutex: mutex,
    pub paths_uuid: uuid_t,
    pub refcount: refcount_t,
    pub ctx: *mut rtrs_srv_ctx,
    pub ctx_list: list_head,
    pub priv: *mut c_void,
    pub queue_depth: usize,
    pub chunks: *mut page,
    pub dev: device,
    pub dev_ref: c_uint,
    pub kobj_paths: *mut kobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_ctx {
    pub ops: rtrs_srv_ops,
    pub cm_id_ip: *mut rdma_cm_id,
    pub cm_id_ib: *mut rdma_cm_id,
    pub srv_mutex: mutex,
    pub srv_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_srv_ib_ctx {
    pub srv_ctx: *mut rtrs_srv_ctx,
    pub port: u16,
    pub ib_dev_mutex: mutex,
    pub ib_dev_count: c_int,
}

extern "C" {
    pub fn close_path(srv_path: *mut rtrs_srv_path);
}
// functions which are implemented in rtrs-srv-stats.c
extern "C" {
    pub fn rtrs_srv_reset_rdma_stats(stats: *mut rtrs_srv_stats, enable: bool) -> c_int;
}
extern "C" {
    pub fn rtrs_srv_stats_rdma_to_str(stats: *mut rtrs_srv_stats, page: *mut c_char) -> isize;
}
extern "C" {
    pub fn rtrs_srv_reset_all_stats(stats: *mut rtrs_srv_stats, enable: bool) -> c_int;
}
// functions which are implemented in rtrs-srv-sysfs.c
extern "C" {
    pub fn rtrs_srv_create_path_files(srv_path: *mut rtrs_srv_path) -> c_int;
}
extern "C" {
    pub fn rtrs_srv_destroy_path_files(srv_path: *mut rtrs_srv_path);
}
