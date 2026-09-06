//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/wq.h
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


//
// Copyright (c) 2013-2015, Mellanox Technologies, Ltd.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wq_param {
    pub buf_numa_node: c_int,
    pub db_numa_node: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wq_ctrl {
    pub mdev: *mut mlx5_core_dev,
    pub buf: mlx5_frag_buf,
    pub db: mlx5_db,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wq_cyc {
    pub fbc: mlx5_frag_buf_ctrl,
    pub db: *mut __be32,
    pub sz: u16,
    pub wqe_ctr: u16,
    pub cur_sz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wq_qp {
    pub rq: mlx5_wq_cyc,
    pub sq: mlx5_wq_cyc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cqwq {
    pub fbc: mlx5_frag_buf_ctrl,
    pub db: *mut __be32,
    pub /: *mut *mut u32 cc; / consumer counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wq_ll {
    pub fbc: mlx5_frag_buf_ctrl,
    pub db: *mut __be32,
    pub tail_next: *mut __be16,
    pub head: u16,
    pub wqe_ctr: u16,
    pub cur_sz: u16,
}

extern "C" {
    pub fn mlx5_wq_cyc_wqe_dump(wq: *mut mlx5_wq_cyc, ix: u16, nstrides: u8);
}
extern "C" {
    pub fn mlx5_wq_cyc_reset(wq: *mut mlx5_wq_cyc);
}
extern "C" {
    pub fn mlx5_wq_ll_reset(wq: *mut mlx5_wq_ll);
}
extern "C" {
    pub fn mlx5_wq_destroy(wq_ctrl: *mut mlx5_wq_ctrl);
}
// wq->db = cpu_to_be32(wq->wqe_ctr);
extern "C" {
    pub fn mlx5_wq_cyc_ctr2ix(_arg: wq, _arg: wq->wqe_ctr) -> return;
}
extern "C" {
    pub fn mlx5_wq_cyc_ctr2ix(_arg: wq, wq->cur_sz: wq->wqe_ctr -) -> return;
}
extern "C" {
    pub fn mlx5_frag_buf_get_wqe(_arg: &wq->fbc, _arg: ix) -> return;
}
extern "C" {
    pub fn mlx5_cqwq_ctr2ix(_arg: wq, _arg: wq->cc) -> return;
}
// For 128B CQEs the data is in the last 64B
extern "C" {
    pub fn mlx5_cqwq_get_ctr_wrap_cnt(_arg: wq, _arg: wq->cc) -> return;
}
// wq->db = cpu_to_be32(wq->cc & 0xffffff);
// ensure cqe content is read after cqe ownership bit
// ensure cqe content is read after cqe ownership bit/validity byte
extern "C" {
    pub fn mlx5_frag_buf_get_wqe(_arg: &wq->fbc, _arg: ix) -> return;
}
extern "C" {
    pub fn be16_to_cpu(_arg: wqe->next_wqe_index) -> return;
}
// wq->tail_next = ix;
// wq->db = cpu_to_be32(wq->wqe_ctr);
