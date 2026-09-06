//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mthca/mthca_wqe.h
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
// Copyright (c) 2005 Cisco Systems. All rights reserved.
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
pub struct mthca_next_seg {
    pub /: *mut *mut __be32 nda_op; / [31:6] next WQE [4:0] next opcode,
    pub /: *mut *mut __be32 ee_nds; / [31:8] next EE [7] DBD [6] F [5:0] next WQE size,
    pub /: *mut *mut __be32 flags; / [3] CQ [2] Event [1] Solicit,
    pub /: *mut *mut __be32 imm; / immediate data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_tavor_ud_seg {
    pub reserved1: u32,
    pub lkey: __be32,
    pub av_addr: __be64,
    pub reserved2: [u32; 4],
    pub dqpn: __be32,
    pub qkey: __be32,
    pub reserved3: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_arbel_ud_seg {
    pub av: [__be32; 8],
    pub dqpn: __be32,
    pub qkey: __be32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_bind_seg {
    pub /: *mut *mut __be32 flags; / [31] Atomic [30] rem write [29] rem read,
    pub reserved: u32,
    pub new_rkey: __be32,
    pub lkey: __be32,
    pub addr: __be64,
    pub length: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_raddr_seg {
    pub raddr: __be64,
    pub rkey: __be32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_atomic_seg {
    pub swap_add: __be64,
    pub compare: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_data_seg {
    pub byte_count: __be32,
    pub lkey: __be32,
    pub addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_mlx_seg {
    pub nda_op: __be32,
    pub nds: __be32,
    pub rate: *mut *mut __be32 flags; / [17] VL15 [16] SLR [14:12] static,
    pub rlid: __be16,
    pub vcrc: __be16,
}
