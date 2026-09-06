//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/iidc_rdma_ice.h
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
// Copyright (C) 2021-2025, Intel Corporation.

pub const IIDC_MAX_USER_PRIORITY: c_int = 8;
pub const IIDC_DSCP_PFC_MODE: c_uint = 0x1;
//
// struct iidc_rdma_qset_params - Struct to hold per RDMA Qset info
// @teid: TEID of the Qset node
// @qs_handle: SW index of the Qset, RDMA provides this
// @vport_id: VSI index
// @tc: Traffic Class branch the QSet should belong to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_qset_params {
// Qset TEID returned to the RDMA driver in
// ice_add_rdma_qset and used by RDMA driver
// for calls to ice_del_rdma_qset
//
    pub teid: u32,
    pub qs_handle: u16,
    pub vport_id: u16,
    pub tc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_qos_info {
    pub tc_ctx: u64,
    pub rel_bw: u8,
    pub prio_type: u8,
    pub egress_virt_up: u8,
    pub ingress_virt_up: u8,
}

// Struct to pass QoS info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_qos_params {
    pub tc_info: [iidc_rdma_qos_info; IEEE_8021QAZ_MAX_TCS],
    pub up2tc: [u8; IIDC_MAX_USER_PRIORITY],
    pub vport_relative_bw: u8,
    pub vport_priority_type: u8,
    pub num_tc: u8,
    pub pfc_mode: u8,
    pub dscp_map: [u8; DSCP_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_priv_dev_info {
    pub pf_id: u8,
    pub vport_id: u16,
    pub netdev: *mut net_device,
    pub qos_info: iidc_rdma_qos_params,
    pub hw_addr: *mut u8 __iomem,
}
