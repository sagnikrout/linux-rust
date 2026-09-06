//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/imx/imx-scu-soc.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2019 NXP.
//

    static struct imx_sc_ipc *imx_sc_soc_ipc_handle;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_misc_get_soc_id {
    pub hdr: imx_sc_rpc_msg,
    union {
    struct {
    pub control: u32,
    pub resource: u16,
    pub req: } __packed,
    struct {
    pub id: u32,
    pub resp: },
    pub data: },
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_misc_get_soc_uid {
    pub hdr: imx_sc_rpc_msg,
    pub uid_low: u32,
    pub uid_high: u32,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn imx_scu_soc_uid(soc_uid: *mut u64) -> c_int {
    static int imx_scu_soc_uid(u64 *soc_uid)
    {
    pub msg: imx_sc_msg_misc_get_soc_uid,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub ret: c_int,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_MISC: hdr->svc =,
    pub IMX_SC_MISC_FUNC_UNIQUE_ID: hdr->func =,
    pub 1: hdr->size =,
    pub true): ret = imx_scu_call_rpc(imx_sc_soc_ipc_handle, &msg,,
    if (ret) {
    pub ret): pr_err("%s: get soc uid failed, ret %d\n", __func__,,
    pub ret: return,
    }
// soc_uid = msg.uid_high;
// soc_uid <<= 32;
// soc_uid |= msg.uid_low;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn imx_scu_soc_id() -> c_int {
    static int imx_scu_soc_id(void)
    {
    pub msg: imx_sc_msg_misc_get_soc_id,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub ret: c_int,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_MISC: hdr->svc =,
    pub IMX_SC_MISC_FUNC_GET_CONTROL: hdr->func =,
    pub 3: hdr->size =,
    pub IMX_SC_C_ID: msg.data.req.control =,
    pub IMX_SC_R_SYSTEM: msg.data.req.resource =,
    pub true): ret = imx_scu_call_rpc(imx_sc_soc_ipc_handle, &msg,,
    if (ret) {
    pub ret): pr_err("%s: get soc info failed, ret %d\n", __func__,,
    pub ret: return,
    }
    pub msg.data.resp.id: return,
    }
    static const char *imx_scu_soc_name(u32 id)
    {
    switch (id) {
    case 0x1:
    pub "i.MX8QM": return,
    case 0x2:
    pub "i.MX8QXP": return,
    case 0xe:
    pub "i.MX8DXL": return,
    default:
    }
    pub "NULL": return,
    }
#[no_mangle]
pub unsafe extern "C" fn imx_scu_soc_init(dev: *mut device) -> c_int {
    int imx_scu_soc_init(struct device *dev)
    {
    pub soc_dev_attr: *mut soc_device_attribute,
    pub soc_dev: *mut soc_device,
    pub ret: int id,,
    pub 0: u64 uid =,
    pub val: u32,
    pub imx_scu_get_handle(&imx_sc_soc_ipc_handle): ret =,
    if (ret)
    pub ret: return,
    soc_dev_attr = devm_kzalloc(dev, sizeof(*soc_dev_attr),
    if (!soc_dev_attr)
    pub -ENOMEM: return,
    pub i.MX": soc_dev_attr->family = "Freescale,
    ret = of_property_read_string(of_root,
    "model",
    if (ret)
    pub ret: return,
    pub imx_scu_soc_id(): id =,
    if (id < 0)
    pub -EINVAL: return,
    pub imx_scu_soc_uid(&uid): ret =,
    if (ret < 0)
    pub -EINVAL: return,
// format soc_id value passed from SCU firmware
    pub 0x1f: val = id &,
    pub imx_scu_soc_name(val): soc_dev_attr->soc_id =,
// format revision value passed from SCU firmware
    pub 0xf: val = (id >> 5) &,
    pub 0x3): val = (((val >> 2) + 1) << 4) | (val &,
    soc_dev_attr.revision = devm_kasprintf(dev, GFP_KERNEL, "%d.%d",
    pub 0xf): (val >> 4) & 0xf, val &,
    if (!soc_dev_attr.revision)
    pub -ENOMEM: return,
    soc_dev_attr.serial_number = devm_kasprintf(dev, GFP_KERNEL,
    pub uid): "%016llX",,
    if (!soc_dev_attr.serial_number)
    pub -ENOMEM: return,
    pub soc_device_register(soc_dev_attr): soc_dev =,
    if (IS_ERR(soc_dev))
    pub PTR_ERR(soc_dev): return,
    pub 0: return,
    }
