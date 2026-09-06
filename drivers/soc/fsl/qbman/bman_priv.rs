//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/fsl/qbman/bman_priv.h
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


// Copyright 2008 - 2016 Freescale Semiconductor, Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// Portal processing (interrupt) sources
pub const BM_PIRQ_RCRI: c_uint = 0x00000002	/* RCR Ring (below threshold) */;
// Revision info (for errata and feature handling)
pub const BMAN_REV10: c_uint = 0x0100;
pub const BMAN_REV20: c_uint = 0x0200;
pub const BMAN_REV21: c_uint = 0x0201;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_portal_config {
// Portal addresses
    pub addr_virt_ce: *mut c_void,
    pub addr_virt_ci: *mut void __iomem,
// Allow these to be joined in lists
    pub list: list_head,
    pub dev: *mut device,
// User-visible portal configuration settings
// portal is affined to this cpu
    pub cpu: c_int,
// portal interrupt line
    pub irq: c_int,
}

//
// The below bman_p_***() variant might be called in a situation that the cpu
// which the portal affine to is not online yet.
// @bman_portal specifies which portal the API will use.
//
extern "C" {
    pub fn bman_p_irqsource_add(p: *mut bman_portal, bits: u32) -> c_int;
}
//
// Used by all portal interrupt registers except 'inhibit'
// This mask contains all the "irqsource" bits visible to API users
//

extern "C" {
    pub fn bman_requires_cleanup() -> c_int;
}
extern "C" {
    pub fn bman_done_cleanup();
}
extern "C" {
    pub fn bm_shutdown_pool(bpid: u32) -> c_int;
}
