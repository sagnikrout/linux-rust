//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/imx/misc.c
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
// Copyright 2017~2018 NXP
// Author: Dong Aisheng <aisheng.dong@nxp.com>
//
// File containing client-side RPC functions for the MISC service. These
// function are ported to clients that communicate to the SC.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_req_misc_set_ctrl {
    pub hdr: imx_sc_rpc_msg,
    pub ctrl: u32,
    pub val: u32,
    pub resource: u16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_req_cpu_start {
    pub hdr: imx_sc_rpc_msg,
    pub address_hi: u32,
    pub address_lo: u32,
    pub resource: u16,
    pub enable: u8,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_req_misc_get_ctrl {
    pub hdr: imx_sc_rpc_msg,
    pub ctrl: u32,
    pub resource: u16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_resp_misc_get_ctrl {
    pub hdr: imx_sc_rpc_msg,
    pub val: u32,
    pub __aligned(4): } __packed,
//
// This function sets a miscellaneous control value.
//
// @param[in]     ipc         IPC handle
// @param[in]     resource    resource the control is associated with
// @param[in]     ctrl        control to change
// @param[in]     val         value to apply to the control
//
// @return Returns 0 for success and < 0 for errors.
//
    int imx_sc_misc_set_control(struct imx_sc_ipc *ipc, u32 resource,
    u8 ctrl, u32 val)
    {
    pub msg: imx_sc_msg_req_misc_set_ctrl,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub (uint8_t)IMX_SC_RPC_SVC_MISC: hdr->svc =,
    pub (uint8_t)IMX_SC_MISC_FUNC_SET_CONTROL: hdr->func =,
    pub 4: hdr->size =,
    pub ctrl: msg.ctrl =,
    pub val: msg.val =,
    pub resource: msg.resource =,
    pub true): return imx_scu_call_rpc(ipc, &msg,,
    }
//
// This function gets a miscellaneous control value.
//
// @param[in]     ipc         IPC handle
// @param[in]     resource    resource the control is associated with
// @param[in]     ctrl        control to get
// @param[out]    val         pointer to return the control value
//
// @return Returns 0 for success and < 0 for errors.
//
    int imx_sc_misc_get_control(struct imx_sc_ipc *ipc, u32 resource,
    u8 ctrl, u32 *val)
    {
    pub msg: imx_sc_msg_req_misc_get_ctrl,
    pub resp: *mut imx_sc_msg_resp_misc_get_ctrl,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub ret: c_int,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub (uint8_t)IMX_SC_RPC_SVC_MISC: hdr->svc =,
    pub (uint8_t)IMX_SC_MISC_FUNC_GET_CONTROL: hdr->func =,
    pub 3: hdr->size =,
    pub ctrl: msg.ctrl =,
    pub resource: msg.resource =,
    pub true): ret = imx_scu_call_rpc(ipc, &msg,,
    if (ret)
    pub ret: return,
    pub )&msg: *mut resp = (struct imx_sc_msg_resp_misc_get_ctrl,
    if (val != core::ptr::null_mut())
// val = resp->val;
    pub 0: return,
    }
//
// This function starts/stops a CPU identified by @resource
//
// @param[in]     ipc         IPC handle
// @param[in]     resource    resource the control is associated with
// @param[in]     enable      true for start, false for stop
// @param[in]     phys_addr   initial instruction address to be executed
//
// @return Returns 0 for success and < 0 for errors.
//
    int imx_sc_pm_cpu_start(struct imx_sc_ipc *ipc, u32 resource,
    bool enable, u64 phys_addr)
    {
    pub msg: imx_sc_msg_req_cpu_start,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_PM: hdr->svc =,
    pub IMX_SC_PM_FUNC_CPU_START: hdr->func =,
    pub 4: hdr->size =,
    pub 32: msg.address_hi = phys_addr >>,
    pub phys_addr: msg.address_lo =,
    pub resource: msg.resource =,
    pub enable: msg.enable =,
    pub true): return imx_scu_call_rpc(ipc, &msg,,
    }
