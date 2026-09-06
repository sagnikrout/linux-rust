//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/sgx/defines.h
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
//
// Copyright(c) 2016-20 Intel Corporation.
//

pub const PAGE_SIZE: c_int = 4096;

// Macro flag: #define __section(x)__attribute__((__section__(x)))

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum encl_op_type {
    ENCL_OP_PUT_TO_BUFFER,
    ENCL_OP_GET_FROM_BUFFER,
    ENCL_OP_PUT_TO_ADDRESS,
    ENCL_OP_GET_FROM_ADDRESS,
    ENCL_OP_NOP,
    ENCL_OP_EACCEPT,
    ENCL_OP_EMODPE,
    ENCL_OP_INIT_TCS_PAGE,
    ENCL_OP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_header {
    pub type: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_put_to_buf {
    pub header: encl_op_header,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_get_from_buf {
    pub header: encl_op_header,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_put_to_addr {
    pub header: encl_op_header,
    pub value: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_get_from_addr {
    pub header: encl_op_header,
    pub value: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_eaccept {
    pub header: encl_op_header,
    pub epc_addr: u64,
    pub flags: u64,
    pub ret: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_emodpe {
    pub header: encl_op_header,
    pub epc_addr: u64,
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_op_init_tcs_page {
    pub header: encl_op_header,
    pub tcs_page: u64,
    pub ssa: u64,
    pub entry: u64,
}
