//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_vport.h
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
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2022 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2004-2006 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
// Portions Copyright (C) 2004-2005 Christoph Hellwig
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//
// API version values (each will be an individual bit)
pub const VPORT_API_VERSION_1: c_uint = 0x01;
// Values returned via lpfc_vport_getinfo()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_info {
    pub api_versions: u32,
    pub linktype: u8,
pub const VPORT_TYPE_PHYSICAL: c_int = 0;
pub const VPORT_TYPE_VIRTUAL: c_int = 1;
    pub state: u8,
pub const VPORT_STATE_OFFLINE: c_int = 0;
pub const VPORT_STATE_ACTIVE: c_int = 1;
pub const VPORT_STATE_FAILED: c_int = 2;
    pub fail_reason: u8,
    pub prev_fail_reason: u8,
pub const VPORT_FAIL_UNKNOWN: c_int = 0;
pub const VPORT_FAIL_LINKDOWN: c_int = 1;
pub const VPORT_FAIL_FAB_UNSUPPORTED: c_int = 2;
pub const VPORT_FAIL_FAB_NORESOURCES: c_int = 3;
pub const VPORT_FAIL_FAB_LOGOUT: c_int = 4;
pub const VPORT_FAIL_ADAP_NORESOURCES: c_int = 5;
    pub /: *mut *mut uint8_t node_name[8]; / WWNN,
    pub /: *mut *mut uint8_t port_name[8]; / WWPN,
    pub shost: *mut Scsi_Host,
// Following values are valid only on physical links
    pub vports_max: u32,
    pub vports_inuse: u32,
    pub rpi_max: u32,
    pub rpi_inuse: u32,
pub const VPORT_CNT_INVALID: c_uint = 0xFFFFFFFF;
}

// data used  in link creation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_data {
    pub api_version: u32,
    pub options: u32,
pub const VPORT_OPT_AUTORETRY: c_uint = 0x01;
    pub /: *mut *mut uint8_t node_name[8]; / WWNN,
    pub /: *mut *mut uint8_t port_name[8]; / WWPN,
//
// Upon successful creation, vport_shost will point to the new Scsi_Host
// structure for the new virtual link.
//
    pub vport_shost: *mut Scsi_Host,
}

// API function return codes
pub const VPORT_OK: c_int = 0;

extern "C" {
    pub fn lpfc_vport_create(: *mut fc_vport, _arg: bool) -> c_int;
}
extern "C" {
    pub fn lpfc_vport_delete(: *mut fc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_vport_getinfo(: *mut Scsi_Host, : *mut vport_info) -> c_int;
}
extern "C" {
    pub fn lpfc_vport_tgt_remove(: *mut Scsi_Host, _arg: c_uint, _arg: c_uint) -> c_int;
}
extern "C" {
    pub fn lpfc_destroy_vport_work_array(: *mut lpfc_hba, : *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_alloc_vpi(phba: *mut lpfc_hba) -> c_int;
}
//
// queuecommand  VPORT-specific return codes. Specified in  the host byte code.
// Returned when the virtual link has failed or is not active.
//
pub const DID_VPORT_ERROR: c_uint = 0x0f;
pub const VPORT_INFO: c_uint = 0x1;
pub const VPORT_CREATE: c_uint = 0x2;
pub const VPORT_DELETE: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_cmd_tag {
    pub cmd: u32,
    pub cdata: vport_data,
    pub cinfo: vport_info,
    pub vport: *mut c_void,
    pub vport_num: c_int,
}
