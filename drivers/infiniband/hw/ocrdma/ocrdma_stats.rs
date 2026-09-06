//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ocrdma/ocrdma_stats.h
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


// This file is part of the Emulex RoCE Device Driver for
// RoCE (RDMA over Converged Ethernet) adapters.
// Copyright (C) 2012-2015 Emulex. All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.emulex.com
//
// This software is available to you under a choice of one of two licenses.
// You may choose to be licensed under the terms of the GNU General Public
// License (GPL) Version 2, available from the file COPYING in the main
// directory of this source tree, or the BSD license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//

pub const OCRDMA_MAX_DBGFS_MEM: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OCRDMA_STATS_TYPE {
    OCRDMA_RSRC_STATS,
    OCRDMA_RXSTATS,
    OCRDMA_WQESTATS,
    OCRDMA_TXSTATS,
    OCRDMA_DB_ERRSTATS,
    OCRDMA_RXQP_ERRSTATS,
    OCRDMA_TXQP_ERRSTATS,
    OCRDMA_TX_DBG_STATS,
    OCRDMA_RX_DBG_STATS,
    OCRDMA_DRV_STATS,
    OCRDMA_RESET_STATS
}

extern "C" {
    pub fn ocrdma_rem_debugfs();
}
extern "C" {
    pub fn ocrdma_init_debugfs();
}
extern "C" {
    pub fn ocrdma_alloc_stats_resources(dev: *mut ocrdma_dev) -> bool;
}
extern "C" {
    pub fn ocrdma_release_stats_resources(dev: *mut ocrdma_dev);
}
extern "C" {
    pub fn ocrdma_rem_port_stats(dev: *mut ocrdma_dev);
}
extern "C" {
    pub fn ocrdma_add_port_stats(dev: *mut ocrdma_dev);
}
extern "C" {
    pub fn ocrdma_pma_counters(dev: *mut ocrdma_dev, out_mad: *mut ib_mad);
}
