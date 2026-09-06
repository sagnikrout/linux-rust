//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/mana-abi.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note)
//
// Copyright (c) 2022, Microsoft Corporation. All rights reserved.
//

//
// Increment this value if any changes that break userspace ABI
// compatibility are made.
//
pub const MANA_IB_UVERBS_ABI_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_create_cq_flags {
// Reserved for backward compatibility. Legacy
// kernel versions use it to create CQs in RNIC
//
    MANA_IB_CREATE_RNIC_CQ	= 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_cq {
    pub buf_addr: __aligned_u64,
    pub comp_mask: __u16,
    pub reserved0: __u16,
    pub reserved1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_cq_resp {
    pub cqid: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_qp {
    pub sq_buf_addr: __aligned_u64,
    pub sq_buf_size: __u32,
    pub port: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_qp_resp {
    pub sqid: __u32,
    pub cqid: __u32,
    pub tx_vp_offset: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_rc_qp {
    pub queue_buf: [__aligned_u64; 4],
    pub queue_size: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_rc_qp_resp {
    pub queue_id: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_uc_qp {
    pub queue_buf: [__aligned_u64; 3],
    pub queue_size: [__u32; 3],
    pub comp_mask: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_uc_qp_resp {
    pub queue_id: [__u32; 3],
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_wq {
    pub wq_buf_addr: __aligned_u64,
    pub wq_buf_size: __u32,
    pub reserved: __u32,
}

// RX Hash function flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_rx_hash_function_flags {
    MANA_IB_RX_HASH_FUNC_TOEPLITZ = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_qp_rss {
    pub rx_hash_fields_mask: __aligned_u64,
    pub rx_hash_function: __u8,
    pub reserved: [__u8; 7],
    pub rx_hash_key_len: __u32,
    pub rx_hash_key: [__u8; 40],
    pub port: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rss_resp_entry {
    pub cqid: __u32,
    pub wqid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_create_qp_rss_resp {
    pub num_entries: __aligned_u64,
    pub entries: [rss_resp_entry; 64],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_ucontext_support {
    MANA_IB_UCNTX_ALLOC_PDN_SUPPORT = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_alloc_ucontext_resp {
    pub comp_mask: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_create_pd_flags {
    MANA_IB_PD_SHORT_PDN = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_alloc_pd {
    pub comp_mask: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_alloc_pd_resp {
    pub pdn: __u32,
    pub reserved: __u32,
}
