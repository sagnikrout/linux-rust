//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_gsc_uc_heci_cmd_submit.h
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
// Copyright © 2023 Intel Corporation
//

pub const GSC_HECI_REPLY_LATENCY_MS: c_int = 500;
//
// Max FW response time is 500ms, but this should be counted from the time the
// command has hit the GSC-CS hardware, not the preceding handoff to GuC CTB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_mtl_header {
    pub validity_marker: u32,
pub const GSC_HECI_VALIDITY_MARKER: c_uint = 0xA578875A;
    pub heci_client_id: u8,
pub const HECI_MEADDRESS_MKHI: c_int = 7;
pub const HECI_MEADDRESS_PROXY: c_int = 10;
pub const HECI_MEADDRESS_PXP: c_int = 17;
pub const HECI_MEADDRESS_HDCP: c_int = 18;
    pub reserved1: u8,
    pub header_version: u16,
pub const MTL_GSC_HEADER_VERSION: c_int = 1;
//
// FW allows host to decide host_session handle
// as it sees fit.
// For intertracebility reserving select bits(60-63)
// to differentiate caller-target subsystem
// 0000 - HDCP
// 0001 - PXP Single Session
//
    pub host_session_handle: u64,

    pub gsc_message_handle: u64,
    pub /: *mut *mut u32 message_size; / lower 20 bits only, upper 12 are reserved,
//
// Flags mask:
// Bit 0: Pending
// Bit 1: Session Cleanup;
// Bits 2-15: Flags
// Bits 16-31: Extension Size
// According to internal spec flags are either input or output
// we distinguish the flags using OUTFLAG or INFLAG
//
    pub flags: u32,

    pub status: u32,
    pub __packed: },
    pub size_out): u64 addr_out, u32,
    pub host_session_id): u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_heci_non_priv_pkt {
    pub addr_in: u64,
    pub size_in: u32,
    pub addr_out: u64,
    pub size_out: u32,
    pub heci_pkt_vma: *mut i915_vma,
    pub bb_vma: *mut i915_vma,
}
