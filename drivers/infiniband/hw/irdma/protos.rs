//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/protos.h
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
// Copyright (c) 2016 - 2021 Intel Corporation
pub const PAUSE_TIMER_VAL: c_uint = 0xffff;
pub const REFRESH_THRESHOLD: c_uint = 0x7fff;
pub const HIGH_THRESHOLD: c_uint = 0x800;
pub const LOW_THRESHOLD: c_uint = 0x200;
pub const ALL_TC2PFC: c_uint = 0xff;
pub const CQP_COMPL_WAIT_TIME_MS: c_int = 10;
pub const CQP_TIMEOUT_THRESHOLD: c_int = 500;
pub const CQP_DEF_CMPL_TIMEOUT_THRESHOLD: c_int = 2500;
// init operations
extern "C" {
    pub fn irdma_sc_rt_init(dev: *mut irdma_sc_dev);
}
extern "C" {
    pub fn irdma_sc_cqp_post_sq(cqp: *mut irdma_sc_cqp);
}
// HMC/FPM functions
extern "C" {
    pub fn irdma_sc_init_iw_hmc(dev: *mut irdma_sc_dev, hmc_fn_id: u8) -> c_int;
}
// stats misc
extern "C" {
    pub fn irdma_alloc_ws_node_id(dev: *mut irdma_sc_dev) -> u16;
}
extern "C" {
    pub fn irdma_free_ws_node_id(dev: *mut irdma_sc_dev, node_id: u16);
}
// vsi functions
extern "C" {
    pub fn irdma_vsi_stats_free(vsi: *mut irdma_sc_vsi);
}
extern "C" {
    pub fn irdma_sc_add_cq_ctx(ceq: *mut irdma_sc_ceq, cq: *mut irdma_sc_cq) -> c_int;
}
extern "C" {
    pub fn irdma_sc_remove_cq_ctx(ceq: *mut irdma_sc_ceq, cq: *mut irdma_sc_cq);
}
// misc L2 param change functions
extern "C" {
    pub fn irdma_sc_suspend_resume_qps(vsi: *mut irdma_sc_vsi, suspend: u8);
}
extern "C" {
    pub fn irdma_cqp_qp_suspend_resume(qp: *mut irdma_sc_qp, cmd: u8) -> c_int;
}
extern "C" {
    pub fn irdma_qp_add_qos(qp: *mut irdma_sc_qp);
}
extern "C" {
    pub fn irdma_qp_rem_qos(qp: *mut irdma_sc_qp);
}
extern "C" {
    pub fn irdma_reinitialize_ieq(vsi: *mut irdma_sc_vsi);
}
// terminate functions
extern "C" {
    pub fn irdma_terminate_send_fin(qp: *mut irdma_sc_qp);
}
// dynamic memory allocation
// misc
extern "C" {
    pub fn irdma_get_encoded_wqe_size(wqsize: u32, queue_type: irdma_queue_type) -> u8;
}
extern "C" {
    pub fn irdma_modify_qp_to_err(sc_qp: *mut irdma_sc_qp);
}
extern "C" {
    pub fn irdma_cfg_fpm_val(dev: *mut irdma_sc_dev, qp_count: u32) -> c_int;
}
extern "C" {
    pub fn irdma_get_rdma_features(dev: *mut irdma_sc_dev) -> c_int;
}
extern "C" {
    pub fn free_sd_mem(dev: *mut irdma_sc_dev);
}
extern "C" {
    pub fn irdma_process_bh(dev: *mut irdma_sc_dev) -> c_int;
}
extern "C" {
    pub fn irdma_add_dev_ref(dev: *mut irdma_sc_dev);
}
extern "C" {
    pub fn irdma_put_dev_ref(dev: *mut irdma_sc_dev);
}
