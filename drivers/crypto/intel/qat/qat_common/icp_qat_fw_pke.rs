//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_fw_pke.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_req_hdr_pke_cd_pars {
    pub content_desc_addr: __u64,
    pub content_desc_resrvd: __u32,
    pub func_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_req_pke_mid {
    pub opaque: __u64,
    pub src_data_addr: __u64,
    pub dest_data_addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_req_pke_hdr {
    pub resrvd1: __u8,
    pub resrvd2: __u8,
    pub service_type: __u8,
    pub hdr_flags: __u8,
    pub comn_req_flags: __u16,
    pub resrvd4: __u16,
    pub cd_pars: icp_qat_fw_req_hdr_pke_cd_pars,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_pke_request {
    pub pke_hdr: icp_qat_fw_req_pke_hdr,
    pub pke_mid: icp_qat_fw_req_pke_mid,
    pub output_param_count: __u8,
    pub input_param_count: __u8,
    pub resrvd1: __u16,
    pub resrvd2: __u32,
    pub next_req_adr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_resp_pke_hdr {
    pub resrvd1: __u8,
    pub resrvd2: __u8,
    pub response_type: __u8,
    pub hdr_flags: __u8,
    pub comn_resp_flags: __u16,
    pub resrvd4: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_pke_resp {
    pub pke_resp_hdr: icp_qat_fw_resp_pke_hdr,
    pub opaque: __u64,
    pub src_data_addr: __u64,
    pub dest_data_addr: __u64,
}

pub const ICP_QAT_FW_PKE_HDR_VALID_FLAG_BITPOS: c_int = 7;
pub const ICP_QAT_FW_PKE_HDR_VALID_FLAG_MASK: c_uint = 0x1;

