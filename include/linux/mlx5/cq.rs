//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/cq.h
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
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
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
pub struct mlx5_core_cq {
    pub cqn: u32,
    pub cqe_sz: c_int,
    pub set_ci_db: *mut __be32,
    pub arm_db: *mut __be32,
    pub refcount: refcount_t,
    pub free: completion,
    pub vector: unsigned,
    pub irqn: c_uint,
    pub eqe): *mut *mut *mut void (comp)(struct mlx5_core_cq cq, struct mlx5_eqe,
    pub mlx5_event): *mut *mut *mut void (event) (struct mlx5_core_cq , enum,
    pub cons_index: u32,
    pub arm_sn: unsigned,
    pub dbg: *mut mlx5_rsc_debug,
    pub pid: c_int,
    pub list: list_head,
    pub eqe): *mut *mut *mut void (comp)(struct mlx5_core_cq cq, struct mlx5_eqe,
    pub priv: *mut c_void,
    pub tasklet_ctx: },
    pub reset_notify_added: c_int,
    pub reset_notify: list_head,
    pub eq: *mut mlx5_eq_comp,
    pub uid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cq_modify_params {
    pub type: c_int,
    pub page_offset: u32,
    pub log_cq_size: u8,
    pub resize: },
    pub moder: },
    pub mapping: },
    pub params: },
}

// cq->set_ci_db = cpu_to_be32(cq->cons_index & 0xffffff);
// cq->arm_db = cpu_to_be32(sn << 28 | cmd | ci);
// Make sure that the doorbell record in host memory is
// written before ringing the doorbell via PCI MMIO.
//
extern "C" {
    pub fn mlx5_add_cq_to_tasklet(cq: *mut mlx5_core_cq, eqe: *mut mlx5_eqe);
}
extern "C" {
    pub fn mlx5_core_destroy_cq(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq) -> c_int;
}
extern "C" {
    pub fn mlx5_debug_cq_add(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq) -> c_int;
}
extern "C" {
    pub fn mlx5_debug_cq_remove(dev: *mut mlx5_core_dev, cq: *mut mlx5_core_cq);
}
