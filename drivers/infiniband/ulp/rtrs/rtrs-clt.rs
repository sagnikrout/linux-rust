//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/rtrs/rtrs-clt.h
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
// enum rtrs_clt_state - Client states.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_clt_state {
    RTRS_CLT_CONNECTING,
    RTRS_CLT_CONNECTING_ERR,
    RTRS_CLT_RECONNECTING,
    RTRS_CLT_CONNECTED,
    RTRS_CLT_CLOSING,
    RTRS_CLT_CLOSED,
    RTRS_CLT_DEAD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtrs_mp_policy {
    MP_POLICY_RR,
    MP_POLICY_MIN_INFLIGHT,
    MP_POLICY_MIN_LATENCY,
}

// see Documentation/ABI/testing/sysfs-class-rtrs-client for details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_stats_reconnects {
    pub successful_cnt: c_int,
    pub fail_cnt: c_int,
}

// see Documentation/ABI/testing/sysfs-class-rtrs-client for details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_stats_cpu_migr {
    pub from: core::sync::atomic::AtomicI32,
    pub to: c_int,
}

// stats for Read and write operation.
// see Documentation/ABI/testing/sysfs-class-rtrs-client for details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_stats_rdma {
    pub cnt: u64,
    pub size_total: u64,
    pub dir: [}; 2],
    pub failover_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_stats_pcpu {
    pub cpu_migr: rtrs_clt_stats_cpu_migr,
    pub rdma: rtrs_clt_stats_rdma,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_stats {
    pub kobj_stats: kobject,
    pub pcpu_stats: *mut rtrs_clt_stats_pcpu __percpu,
    pub reconnects: rtrs_clt_stats_reconnects,
    pub inflight: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_con {
    pub c: rtrs_con,
    pub rsp_ius: *mut rtrs_iu,
    pub queue_num: u32,
    pub cpu: c_uint,
    pub con_mutex: mutex,
    pub cm_err: c_int,
}

//
// rtrs_permit - permits the memory allocation for future RDMA operation.
// Combine with irq pinning to keep IO on same CPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_permit {
    pub con_type: rtrs_clt_con_type,
    pub cpu_id: c_uint,
    pub mem_id: c_uint,
    pub mem_off: c_uint,
}

//
// rtrs_clt_io_req - describes one inflight IO request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_io_req {
    pub iu: *mut rtrs_iu,
    pub /: *mut *mut *mut scatterlist sglist; / list holding user data,
    pub sg_cnt: c_uint,
    pub sg_size: c_uint,
    pub data_len: c_uint,
    pub usr_len: c_uint,
    pub priv: *mut c_void,
    pub in_use: bool,
    pub mp_policy: rtrs_mp_policy,
    pub con: *mut rtrs_clt_con,
    pub sge: *mut ib_sge,
    pub permit: *mut rtrs_permit,
    pub dir: dma_data_direction,
    pub errno): *mut *mut *mut void (conf)(void priv, int,
    pub mr: *mut ib_mr,
    pub inv_cqe: ib_cqe,
    pub inv_comp: completion,
    pub inv_errno: c_int,
    pub need_inv_comp: bool,
    pub ref: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_rbuf {
    pub addr: u64,
    pub rkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_path {
    pub s: rtrs_path,
    pub clt: *mut rtrs_clt_sess,
    pub state_wq: wait_queue_head_t,
    pub state: rtrs_clt_state,
    pub connected_cnt: core::sync::atomic::AtomicI32,
    pub init_mutex: mutex,
    pub reqs: *mut rtrs_clt_io_req,
    pub reconnect_dwork: delayed_work,
    pub close_work: work_struct,
    pub err_recovery_work: work_struct,
    pub reconnect_attempts: c_uint,
    pub established: bool,
    pub rbufs: *mut rtrs_rbuf,
    pub max_io_size: usize,
    pub max_hdr_size: u32,
    pub chunk_size: u32,
    pub queue_depth: usize,
    pub max_pages_per_mr: u32,
    pub flags: u32,
    pub kobj: kobject,
    pub for_new_clt: u8,
// cache hca_port and hca_name to display in sysfs
    pub hca_port: u8,
    pub hca_name: [c_char; IB_DEVICE_NAME_MAX],
// mp_skip_entry;
    pub stats: [rtrs_clt_stats; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrs_clt_sess {
    pub /: *mut *mut list_head paths_list; / rcu protected list,
    pub paths_num: usize,
    pub pcpu_path: *mut *mut __rcu  __percpu,
    pub paths_uuid: uuid_t,
    pub paths_up: c_int,
    pub paths_mutex: mutex,
    pub paths_ev_mutex: mutex,
    pub sessname: [c_char; NAME_MAX],
    pub port: u16,
    pub max_reconnect_attempts: c_uint,
    pub reconnect_delay_sec: c_uint,
    pub max_segments: c_uint,
    pub permits: *mut c_void,
    pub permits_map: *mut c_ulong,
    pub queue_depth: usize,
    pub max_io_size: usize,
    pub permits_wait: wait_queue_head_t,
    pub pdu_sz: usize,
    pub priv: *mut c_void,
    pub ev): rtrs_clt_link_ev,
    pub dev: device,
    pub kobj_paths: *mut kobject,
    pub mp_policy: rtrs_mp_policy,
}

extern "C" {
    pub fn container_of(_arg: c, rtrs_clt_con: struct, _arg: c) -> return;
}
extern "C" {
    pub fn container_of(_arg: s, rtrs_clt_path: struct, _arg: s) -> return;
}
extern "C" {
    pub fn rtrs_clt_reconnect_from_sysfs(path: *mut rtrs_clt_path) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_close_conns(clt_path: *mut rtrs_clt_path, wait: bool);
}
extern "C" {
    pub fn rtrs_clt_set_max_reconnect_attempts(clt: *mut rtrs_clt_sess, value: c_int);
}
extern "C" {
    pub fn rtrs_clt_get_max_reconnect_attempts(clt: *const rtrs_clt_sess) -> c_int;
}
extern "C" {
    pub fn free_path(clt_path: *mut rtrs_clt_path);
}
// rtrs-clt-stats.c
extern "C" {
    pub fn rtrs_clt_init_stats(stats: *mut rtrs_clt_stats) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_inc_failover_cnt(s: *mut rtrs_clt_stats);
}
extern "C" {
    pub fn rtrs_clt_update_wc_stats(con: *mut rtrs_clt_con);
}
extern "C" {
    pub fn rtrs_clt_update_all_stats(req: *mut rtrs_clt_io_req, dir: c_int);
}
extern "C" {
    pub fn rtrs_clt_reset_cpu_migr_stats(stats: *mut rtrs_clt_stats, enable: bool) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_stats_migration_from_cnt_to_str(stats: *mut rtrs_clt_stats, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_stats_migration_to_cnt_to_str(stats: *mut rtrs_clt_stats, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_reset_reconnects_stat(stats: *mut rtrs_clt_stats, enable: bool) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_stats_reconnects_to_str(stats: *mut rtrs_clt_stats, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_reset_rdma_stats(stats: *mut rtrs_clt_stats, enable: bool) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_reset_all_stats(stats: *mut rtrs_clt_stats, enable: bool) -> c_int;
}
// rtrs-clt-sysfs.c
extern "C" {
    pub fn rtrs_clt_create_sysfs_root_files(clt: *mut rtrs_clt_sess) -> c_int;
}
extern "C" {
    pub fn rtrs_clt_destroy_sysfs_root(clt: *mut rtrs_clt_sess);
}
extern "C" {
    pub fn rtrs_clt_create_path_files(clt_path: *mut rtrs_clt_path) -> c_int;
}
