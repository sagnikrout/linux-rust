//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/pxp/intel_pxp_cmd_interface_43.h
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
// Copyright(c) 2022, Intel Corporation. All rights reserved.
//

// PXP-Cmd-Op definitions
pub const PXP43_CMDID_START_HUC_AUTH: c_uint = 0x0000003A;
pub const PXP43_CMDID_NEW_HUC_AUTH: c_uint = 0x0000003F /* MTL+ */;
pub const PXP43_CMDID_INIT_SESSION: c_uint = 0x00000036;
// PXP-Packet sizes for MTL's GSCCS-HECI instruction is spec'd at 65K before page alignment

// PXP-Packet size for MTL's NEW_HUC_AUTH instruction

// PXP-Input-Packet: HUC Load and Authentication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp43_start_huc_auth_in {
    pub header: pxp_cmd_header,
    pub huc_base_address: __le64,
    pub __packed: },
// PXP-Input-Packet: HUC Auth-only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp43_new_huc_auth_in {
    pub header: pxp_cmd_header,
    pub huc_base_address: u64,
    pub huc_size: u32,
    pub __packed: },
// PXP-Output-Packet: HUC Load and Authentication or Auth-only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp43_huc_auth_out {
    pub header: pxp_cmd_header,
    pub __packed: },
// PXP-Input-Packet: Init PXP session
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp43_create_arb_in {
    pub header: pxp_cmd_header,
// header.stream_id fields for version 4.3 of Init PXP session:

    pub protection_mode: u32,
pub const PXP43_INIT_SESSION_PROTECTION_ARB: c_uint = 0x2;
    pub sub_session_id: u32,
    pub init_flags: u32,
    pub rsvd: [u32; 12],
    pub __packed: },
// PXP-Input-Packet: Init PXP session
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp43_create_arb_out {
    pub header: pxp_cmd_header,
    pub rsvd: [u32; 8],
    pub __packed: },
