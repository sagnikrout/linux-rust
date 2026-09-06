//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/pg/dcn35/dcn35_pg_cntl.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2023 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

// Macro flag: #define PG_CNTL_REG_LIST_DCN35()\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_cntl_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_cntl_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_cntl_registers {
    pub LONO_STATE: u32,
    pub DC_IP_REQUEST_CNTL: u32,
    pub DOMAIN0_PG_CONFIG: u32,
    pub DOMAIN1_PG_CONFIG: u32,
    pub DOMAIN2_PG_CONFIG: u32,
    pub DOMAIN3_PG_CONFIG: u32,
    pub DOMAIN16_PG_CONFIG: u32,
    pub DOMAIN17_PG_CONFIG: u32,
    pub DOMAIN18_PG_CONFIG: u32,
    pub DOMAIN19_PG_CONFIG: u32,
    pub DOMAIN22_PG_CONFIG: u32,
    pub DOMAIN23_PG_CONFIG: u32,
    pub DOMAIN24_PG_CONFIG: u32,
    pub DOMAIN25_PG_CONFIG: u32,
    pub DOMAIN0_PG_STATUS: u32,
    pub DOMAIN1_PG_STATUS: u32,
    pub DOMAIN2_PG_STATUS: u32,
    pub DOMAIN3_PG_STATUS: u32,
    pub DOMAIN16_PG_STATUS: u32,
    pub DOMAIN17_PG_STATUS: u32,
    pub DOMAIN18_PG_STATUS: u32,
    pub DOMAIN19_PG_STATUS: u32,
    pub DOMAIN22_PG_STATUS: u32,
    pub DOMAIN23_PG_STATUS: u32,
    pub DOMAIN24_PG_STATUS: u32,
    pub DOMAIN25_PG_STATUS: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_pg_cntl {
    pub base: pg_cntl,
    pub regs: *const pg_cntl_registers,
    pub pg_cntl_shift: *const pg_cntl_shift,
    pub pg_cntl_mask: *const pg_cntl_mask,
}

extern "C" {
    pub fn pg_cntl35_dsc_pg_control(pg_cntl: *mut pg_cntl, dsc_inst: c_uint, power_on: bool);
}
extern "C" {
    pub fn pg_cntl35_hpo_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl35_io_clk_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl35_plane_otg_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl35_dwb_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl35_init_pg_status(pg_cntl: *mut pg_cntl);
}
extern "C" {
    pub fn dcn_pg_cntl_destroy(pg_cntl: *mut pg_cntl);
}
