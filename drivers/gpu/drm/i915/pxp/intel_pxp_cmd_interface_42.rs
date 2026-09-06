//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/pxp/intel_pxp_cmd_interface_42.h
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
// Copyright(c) 2020, Intel Corporation. All rights reserved.
//

// PXP-Opcode for Init Session
pub const PXP42_CMDID_INIT_SESSION: c_uint = 0x1e;
// PXP-Opcode for Invalidate Stream Key
pub const PXP42_CMDID_INVALIDATE_STREAM_KEY: c_uint = 0x00000007;
// PXP-Input-Packet: Init Session (Arb-Session)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp42_create_arb_in {
    pub header: pxp_cmd_header,
    pub protection_mode: u32,
pub const PXP42_ARB_SESSION_MODE_HEAVY: c_uint = 0x2;
    pub session_id: u32,
    pub __packed: },
// PXP-Output-Packet: Init Session
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp42_create_arb_out {
    pub header: pxp_cmd_header,
    pub __packed: },
// PXP-Input-Packet: Invalidate Stream Key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp42_inv_stream_key_in {
    pub header: pxp_cmd_header,
    pub rsvd: [u32; 3],
    pub __packed: },
// PXP-Output-Packet: Invalidate Stream Key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp42_inv_stream_key_out {
    pub header: pxp_cmd_header,
    pub rsvd: u32,
    pub __packed: },
