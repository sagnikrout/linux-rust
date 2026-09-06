//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/imx/ipc.h
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
// Copyright 2018 NXP
//
// Header file for the IPC implementation.
//

pub const IMX_SC_RPC_VERSION: c_int = 1;
pub const IMX_SC_RPC_MAX_MSG: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_sc_rpc_svc {
    IMX_SC_RPC_SVC_UNKNOWN = 0,
    IMX_SC_RPC_SVC_RETURN = 1,
    IMX_SC_RPC_SVC_PM = 2,
    IMX_SC_RPC_SVC_RM = 3,
    IMX_SC_RPC_SVC_TIMER = 5,
    IMX_SC_RPC_SVC_PAD = 6,
    IMX_SC_RPC_SVC_MISC = 7,
    IMX_SC_RPC_SVC_IRQ = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_rpc_msg {
    pub ver: u8,
    pub size: u8,
    pub svc: u8,
    pub func: u8,
}

//
// This is an function to send an RPC message over an IPC channel.
// It is called by client-side SCFW API function shims.
//
// @param[in]     ipc         IPC handle
// @param[in,out] msg         handle to a message
// @param[in]     have_resp   response flag
//
// If have_resp is true then this function waits for a response
// and returns the result in msg.
//
extern "C" {
    pub fn imx_scu_call_rpc(ipc: *mut imx_sc_ipc, msg: *mut c_void, have_resp: bool) -> c_int;
}
//
// This function gets the default ipc handle used by SCU
//
// @param[out]	ipc	sc ipc handle
//
// @return Returns an error code (0 = success, failed if < 0)
//
extern "C" {
    pub fn imx_scu_get_handle(ipc: *mut imx_sc_ipc) -> c_int;
}

