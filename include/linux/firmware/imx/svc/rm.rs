//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/imx/svc/rm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Copyright 2017-2020 NXP
//
// Header file containing the public API for the System Controller (SC)
// Resource Management (RM) function. This includes functions for
// partitioning resources, pads, and memory regions.
//
// RM_SVC (SVC) Resource Management Service
//
// Module for the Resource Management (RM) service.
//

//
// This type is used to indicate RPC RM function calls.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_sc_rm_func {
    IMX_SC_RM_FUNC_UNKNOWN = 0,
    IMX_SC_RM_FUNC_PARTITION_ALLOC = 1,
    IMX_SC_RM_FUNC_SET_CONFIDENTIAL = 31,
    IMX_SC_RM_FUNC_PARTITION_FREE = 2,
    IMX_SC_RM_FUNC_GET_DID = 26,
    IMX_SC_RM_FUNC_PARTITION_STATIC = 3,
    IMX_SC_RM_FUNC_PARTITION_LOCK = 4,
    IMX_SC_RM_FUNC_GET_PARTITION = 5,
    IMX_SC_RM_FUNC_SET_PARENT = 6,
    IMX_SC_RM_FUNC_MOVE_ALL = 7,
    IMX_SC_RM_FUNC_ASSIGN_RESOURCE = 8,
    IMX_SC_RM_FUNC_SET_RESOURCE_MOVABLE = 9,
    IMX_SC_RM_FUNC_SET_SUBSYS_RSRC_MOVABLE = 28,
    IMX_SC_RM_FUNC_SET_MASTER_ATTRIBUTES = 10,
    IMX_SC_RM_FUNC_SET_MASTER_SID = 11,
    IMX_SC_RM_FUNC_SET_PERIPHERAL_PERMISSIONS = 12,
    IMX_SC_RM_FUNC_IS_RESOURCE_OWNED = 13,
    IMX_SC_RM_FUNC_GET_RESOURCE_OWNER = 33,
    IMX_SC_RM_FUNC_IS_RESOURCE_MASTER = 14,
    IMX_SC_RM_FUNC_IS_RESOURCE_PERIPHERAL = 15,
    IMX_SC_RM_FUNC_GET_RESOURCE_INFO = 16,
    IMX_SC_RM_FUNC_MEMREG_ALLOC = 17,
    IMX_SC_RM_FUNC_MEMREG_SPLIT = 29,
    IMX_SC_RM_FUNC_MEMREG_FRAG = 32,
    IMX_SC_RM_FUNC_MEMREG_FREE = 18,
    IMX_SC_RM_FUNC_FIND_MEMREG = 30,
    IMX_SC_RM_FUNC_ASSIGN_MEMREG = 19,
    IMX_SC_RM_FUNC_SET_MEMREG_PERMISSIONS = 20,
    IMX_SC_RM_FUNC_IS_MEMREG_OWNED = 21,
    IMX_SC_RM_FUNC_GET_MEMREG_INFO = 22,
    IMX_SC_RM_FUNC_ASSIGN_PAD = 23,
    IMX_SC_RM_FUNC_SET_PAD_MOVABLE = 24,
    IMX_SC_RM_FUNC_IS_PAD_OWNED = 25,
    IMX_SC_RM_FUNC_DUMP = 27,
}

extern "C" {
    pub fn imx_sc_rm_is_resource_owned(ipc: *mut imx_sc_ipc, resource: u16) -> bool;
}
extern "C" {
    pub fn imx_sc_rm_get_resource_owner(ipc: *mut imx_sc_ipc, resource: u16, pt: *mut u8) -> c_int;
}

