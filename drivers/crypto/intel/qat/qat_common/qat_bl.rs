//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/qat_bl.h
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
// Copyright(c) 2014 - 2022 Intel Corporation

pub const QAT_MAX_BUFF_DESC: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_alg_buf {
    pub len: u32,
    pub resrvd: u32,
    pub addr: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_alg_buf_list {
// New members must be added within the __struct_group() macro below.
    pub resrvd: u64,
    pub num_bufs: u32,
    pub num_mapped_bufs: u32,
    pub buffers: [qat_alg_buf; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_alg_fixed_buf_list {
    pub sgl_hdr: qat_alg_buf_list_hdr,
    pub descriptors: [qat_alg_buf; QAT_MAX_BUFF_DESC],
    pub __aligned(64): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_request_buffs {
    pub bl: *mut qat_alg_buf_list,
    pub blp: dma_addr_t,
    pub blout: *mut qat_alg_buf_list,
    pub bloutp: dma_addr_t,
    pub sz: usize,
    pub sz_out: usize,
    pub sgl_src_valid: bool,
    pub sgl_dst_valid: bool,
    pub sgl_src: qat_alg_fixed_buf_list,
    pub sgl_dst: qat_alg_fixed_buf_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_sgl_to_bufl_params {
    pub extra_dst_buff: dma_addr_t,
    pub sz_extra_dst_buff: usize,
    pub sskip: c_uint,
    pub dskip: c_uint,
}
