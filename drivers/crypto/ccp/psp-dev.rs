//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/psp-dev.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AMD Platform Security Processor (PSP) interface driver
//
// Copyright (C) 2017-2019 Advanced Micro Devices, Inc.
//
// Author: Brijesh Singh <brijesh.singh@amd.com>
//

pub const MAX_PSP_NAME_LEN: c_int = 16;
extern "C" {
    pub fn void(_arg: *mut psp_irq_handler_t)(int, : *mut c_void, int: unsigned) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union psp_cap_register {
    pub raw: c_uint,
    pub :12: rsvd5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_device {
    pub entry: list_head,
    pub vdata: *mut psp_vdata,
    pub name: [c_char; MAX_PSP_NAME_LEN],
    pub dev: *mut device,
    pub sp: *mut sp_device,
    pub io_regs: *mut void __iomem,
    pub mailbox_mutex: mutex,
    pub sev_irq_handler: psp_irq_handler_t,
    pub sev_irq_data: *mut c_void,
    pub sev_data: *mut c_void,
    pub tee_data: *mut c_void,
    pub platform_access_data: *mut c_void,
    pub dbc_data: *mut c_void,
    pub sfs_data: *mut c_void,
    pub capability: psp_cap_register,
}

extern "C" {
    pub fn psp_clear_sev_irq_handler(psp: *mut psp_device);
}
//
// enum psp_cmd - PSP mailbox commands
// @PSP_CMD_TEE_RING_INIT:	Initialize TEE ring buffer
// @PSP_CMD_TEE_RING_DESTROY:	Destroy TEE ring buffer
// @PSP_CMD_TEE_EXTENDED_CMD:	Extended command
// @PSP_CMD_MAX:		Maximum command id
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_cmd {
    PSP_CMD_TEE_RING_INIT		= 1,
    PSP_CMD_TEE_RING_DESTROY	= 2,
    PSP_CMD_TEE_EXTENDED_CMD	= 14,
    PSP_CMD_MAX			= 15,
}

//
// struct psp_ext_req_buffer_hdr - Structure of the extended command header
// @payload_size: total payload size
// @sub_cmd_id: extended command ID
// @status: status of command execution (out)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_ext_req_buffer_hdr {
    pub payload_size: u32,
    pub sub_cmd_id: u32,
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_ext_request {
    pub header: psp_ext_req_buffer_hdr,
    pub buf: *mut c_void,
    pub __packed: },
//
// enum psp_sub_cmd - PSP mailbox sub commands
// @PSP_SUB_CMD_DBC_GET_NONCE:		Get nonce from DBC
// @PSP_SUB_CMD_DBC_SET_UID:		Set UID for DBC
// @PSP_SUB_CMD_DBC_GET_PARAMETER:	Get parameter from DBC
// @PSP_SUB_CMD_DBC_SET_PARAMETER:	Set parameter for DBC
// @PSP_SUB_CMD_SFS_GET_FW_VERS:	Get firmware versions for ASP and other MP
// @PSP_SUB_CMD_SFS_UPDATE:		Command to load, verify and execute SFS package
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_sub_cmd {
    PSP_SUB_CMD_DBC_GET_NONCE	= PSP_DYNAMIC_BOOST_GET_NONCE,
    PSP_SUB_CMD_DBC_SET_UID		= PSP_DYNAMIC_BOOST_SET_UID,
    PSP_SUB_CMD_DBC_GET_PARAMETER	= PSP_DYNAMIC_BOOST_GET_PARAMETER,
    PSP_SUB_CMD_DBC_SET_PARAMETER	= PSP_DYNAMIC_BOOST_SET_PARAMETER,
    PSP_SUB_CMD_SFS_GET_FW_VERS	= PSP_SFS_GET_FW_VERSIONS,
    PSP_SUB_CMD_SFS_UPDATE		= PSP_SFS_UPDATE,
}

    pub req): *mut psp_ext_request,
