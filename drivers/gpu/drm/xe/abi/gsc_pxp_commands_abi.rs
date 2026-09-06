//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/gsc_pxp_commands_abi.h
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

// Heci client ID for PXP commands
pub const HECI_MEADDRESS_PXP: c_int = 17;

//
// A PXP sub-section in an HECI packet can be up to 64K big in each direction.
// This does not include the top-level GSC header.
//

//
// there are a lot of status codes for PXP, but we only define the cross-API
// common ones that we actually can handle in the kernel driver. Other failure
// codes should be printed to error msg for debug.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pxp_status {
    PXP_STATUS_SUCCESS = 0x0,
    PXP_STATUS_ERROR_API_VERSION = 0x1002,
    PXP_STATUS_NOT_READY = 0x100e,
    PXP_STATUS_PLATFCONFIG_KF1_NOVERIF = 0x101a,
    PXP_STATUS_PLATFCONFIG_KF1_BAD = 0x101f,
    PXP_STATUS_PLATFCONFIG_FIXED_KF1_NOT_SUPPORTED = 0x1037,
    PXP_STATUS_OP_NOT_PERMITTED = 0x4013
}

// Common PXP FW message header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp_cmd_header {
    pub api_version: u32,
    pub command_id: u32,
    pub /: *mut *mut u32 status; / out,
    pub /: *mut *mut u32 stream_id; / in,

}

// Length of the message (excluding the header)
pub const PXP43_CMDID_INVALIDATE_STREAM_KEY: c_uint = 0x00000007;
pub const PXP43_CMDID_INIT_SESSION: c_uint = 0x00000036;
pub const PXP43_CMDID_NEW_HUC_AUTH: c_uint = 0x0000003F /* MTL+ */;
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
// header.stream_id fields for vesion 4.3 of Init PXP session:

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
// PXP-Input-Packet: Invalidate Stream Key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp43_inv_stream_key_in {
    pub header: pxp_cmd_header,
    pub rsvd: [u32; 3],
    pub __packed: },
// PXP-Output-Packet: Invalidate Stream Key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp43_inv_stream_key_out {
    pub header: pxp_cmd_header,
    pub rsvd: u32,
    pub __packed: },
