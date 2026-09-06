//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/idpf_virtchnl.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2024 Intel Corporation

extern "C" {
    pub fn idpf_init_dflt_mbx(adapter: *mut idpf_adapter) -> c_int;
}
extern "C" {
    pub fn idpf_deinit_dflt_mbx(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_vc_core_init(adapter: *mut idpf_adapter) -> c_int;
}
extern "C" {
    pub fn idpf_vc_core_deinit(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_vport_is_cap_ena(vport: *mut idpf_vport, flag: u16) -> bool;
}
extern "C" {
    pub fn idpf_sideband_flow_type_ena(vport: *mut idpf_vport, flow_type: u32) -> bool;
}
extern "C" {
    pub fn idpf_fsteer_max_rules(vport: *mut idpf_vport) -> c_uint;
}
extern "C" {
    pub fn idpf_send_vf_reset_msg(adapter: *mut idpf_adapter);
}
//
// idpf_send_mb_msg_stack - send a mailbox message from an on-stack buffer
// @adapter: driver specific private structure
// @xn_params: Xn send parameters to fill
// @ptr: pointer to the on-stack message object to send
//
// Send size is deduced based on the pointer type.
//
// Return: %0 on success, -%errno on failure.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_queue_ptr {
    pub type: virtchnl2_queue_type,
    pub rxq: *mut idpf_rx_queue,
    pub txq: *mut idpf_tx_queue,
    pub bufq: *mut idpf_buf_queue,
    pub complq: *mut idpf_compl_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_queue_set {
    pub adapter: *mut idpf_adapter,
    pub qv_rsrc: *mut idpf_q_vec_rsrc,
    pub vport_id: u32,
    pub num: u32,
    pub __counted_by(num): idpf_queue_ptr qs[],
}

extern "C" {
    pub fn idpf_send_enable_queue_set_msg(qs: *const idpf_queue_set) -> c_int;
}
extern "C" {
    pub fn idpf_send_disable_queue_set_msg(qs: *const idpf_queue_set) -> c_int;
}
extern "C" {
    pub fn idpf_send_config_queue_set_msg(qs: *const idpf_queue_set) -> c_int;
}
extern "C" {
    pub fn idpf_send_disable_queues_msg(vport: *mut idpf_vport) -> c_int;
}
extern "C" {
    pub fn idpf_send_enable_queues_msg(vport: *mut idpf_vport) -> c_int;
}
extern "C" {
    pub fn idpf_vport_init(vport: *mut idpf_vport, max_q: *mut idpf_vport_max_q) -> c_int;
}
extern "C" {
    pub fn idpf_get_vport_id(vport: *mut idpf_vport) -> u32;
}
extern "C" {
    pub fn idpf_send_destroy_vport_msg(adapter: *mut idpf_adapter, vport_id: u32) -> c_int;
}
extern "C" {
    pub fn idpf_send_enable_vport_msg(adapter: *mut idpf_adapter, vport_id: u32) -> c_int;
}
extern "C" {
    pub fn idpf_send_disable_vport_msg(adapter: *mut idpf_adapter, vport_id: u32) -> c_int;
}
extern "C" {
    pub fn idpf_send_alloc_vectors_msg(adapter: *mut idpf_adapter, num_vectors: u16) -> c_int;
}
extern "C" {
    pub fn idpf_send_dealloc_vectors_msg(adapter: *mut idpf_adapter) -> c_int;
}
extern "C" {
    pub fn idpf_check_supported_desc_ids(vport: *mut idpf_vport) -> c_int;
}
extern "C" {
    pub fn idpf_send_set_sriov_vfs_msg(adapter: *mut idpf_adapter, num_vfs: u16) -> c_int;
}
