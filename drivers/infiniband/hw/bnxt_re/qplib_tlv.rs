//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/qplib_tlv.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_tlv {
    pub tlv: tlv,
    pub chunks: u8 total_size; // in units of 16 byte,
    pub alignment: u8 unused[7]; // for 16 byte,
}

pub const CHUNK_SIZE: c_int = 16;

//
// TLV size in units of 16 byte chunks
//

//
// TLV length in bytes
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_tlv_modify_cc_req {
    pub tlv_hdr: roce_tlv,
    pub base_req: cmdq_modify_roce_cc,
    pub tlvpad: __le64,
    pub ext_req: cmdq_modify_roce_cc_gen1_tlv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_tlv_query_rcc_sb {
    pub tlv_hdr: roce_tlv,
    pub base_sb: creq_query_roce_cc_resp_sb,
    pub gen1_sb: creq_query_roce_cc_gen1_resp_sb_tlv,
}
