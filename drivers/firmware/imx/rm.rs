//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/imx/rm.c
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
// Copyright 2020 NXP
//
// File containing client-side RPC functions for the RM service. These
// function are ported to clients that communicate to the SC.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_rm_rsrc_owned {
    pub hdr: imx_sc_rpc_msg,
    pub resource: u16,
    pub __aligned(4): } __packed,
//
// This function check @resource is owned by current partition or not
//
// @param[in]     ipc         IPC handle
// @param[in]     resource    resource the control is associated with
//
// @return Returns 0 for not owned and 1 for owned.
//
#[no_mangle]
pub unsafe extern "C" fn imx_sc_rm_is_resource_owned(ipc: *mut imx_sc_ipc, resource: u16) -> bool {
    bool imx_sc_rm_is_resource_owned(struct imx_sc_ipc *ipc, u16 resource)
    {
    pub msg: imx_sc_msg_rm_rsrc_owned,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_RM: hdr->svc =,
    pub IMX_SC_RM_FUNC_IS_RESOURCE_OWNED: hdr->func =,
    pub 2: hdr->size =,
    pub resource: msg.resource =,
//
// SCU firmware only returns value 0 or 1
// for resource owned check which means not owned or owned.
// So it is always successful.
//
    pub true): imx_scu_call_rpc(ipc, &msg,,
    pub hdr->func: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_rm_get_resource_owner {
    pub hdr: imx_sc_rpc_msg,
    union {
    struct {
    pub resource: u16,
    pub req: },
    struct {
    pub val: u8,
    pub resp: },
    pub data: },
    pub __aligned(4): } __packed,
//
// This function get @resource partition number
//
// @param[in]     ipc         IPC handle
// @param[in]     resource    resource the control is associated with
// @param[out]    pt          pointer to return the partition number
//
// @return Returns 0 for success and < 0 for errors.
//
#[no_mangle]
pub unsafe extern "C" fn imx_sc_rm_get_resource_owner(ipc: *mut imx_sc_ipc, resource: u16, pt: *mut u8) -> c_int {
    int imx_sc_rm_get_resource_owner(struct imx_sc_ipc *ipc, u16 resource, u8 *pt)
    {
    pub msg: imx_sc_msg_rm_get_resource_owner,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub ret: c_int,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_RM: hdr->svc =,
    pub IMX_SC_RM_FUNC_GET_RESOURCE_OWNER: hdr->func =,
    pub 2: hdr->size =,
    pub resource: msg.data.req.resource =,
    pub true): ret = imx_scu_call_rpc(ipc, &msg,,
    if (ret)
    pub ret: return,
    if (pt)
// pt = msg.data.resp.val;
    pub 0: return,
    }
