//! Automatically rewritten from C to Rust
//! Source: drivers/ras/amd/atl/prm.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMD Address Translation Library
//
// prm.c : Plumbing code for ACPI Platform Runtime Mechanism (PRM)
//
// Information on AMD PRM modules and handlers including the GUIDs and buffer
// structures used here are defined in the AMD ACPI Porting Guide in the
// chapter "Platform Runtime Mechanism Table (PRMT)"
//
// Copyright (c) 2024, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: John Allen <john.allen@amd.com>
//

//
// PRM parameter buffer - normalized to system physical address, as described
// in the "PRM Parameter Buffer" section of the AMD ACPI Porting Guide.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct norm_to_sys_param_buf {
    pub norm_addr: u64,
    pub socket: u8,
    pub bank_id: u64,
    pub out_buf: *mut c_void,
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn prm_umc_norm_to_sys_addr(socket_id: u8, bank_id: u64, addr: c_ulong) -> c_ulong {
    unsigned long prm_umc_norm_to_sys_addr(u8 socket_id, u64 bank_id, unsigned long addr)
    {
    pub p_buf: norm_to_sys_param_buf,
    pub ret_addr: c_ulong,
    pub ret: c_int,
    pub addr: p_buf.norm_addr =,
    pub socket_id: p_buf.socket =,
    pub bank_id: p_buf.bank_id =,
    pub &ret_addr: p_buf.out_buf =,
    pub &p_buf): ret = acpi_call_prm_handler(norm_to_sys_guid,,
    if (!ret)
    pub ret_addr: return,
    if (ret == -ENODEV)
    pub available\n"): pr_debug("PRM module/handler not,
    else
    pub failed\n"): pr_notice_once("PRM address translation,
    pub ret: return,
    }
