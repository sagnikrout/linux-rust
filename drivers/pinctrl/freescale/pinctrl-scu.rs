//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/freescale/pinctrl-scu.c
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
// Copyright 2017-2018 NXP
// Dong Aisheng <aisheng.dong@nxp.com>
//

pub const IMX_SC_PAD_FUNC_GET_WAKEUP: c_int = 9;
pub const IMX_SC_PAD_FUNC_SET_WAKEUP: c_int = 4;

    enum pad_func_e {
    IMX_SC_PAD_FUNC_SET = 15,
    IMX_SC_PAD_FUNC_GET = 16,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_req_pad_set {
    pub hdr: imx_sc_rpc_msg,
    pub val: u32,
    pub pad: u16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_req_pad_get {
    pub hdr: imx_sc_rpc_msg,
    pub pad: u16,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_resp_pad_get {
    pub hdr: imx_sc_rpc_msg,
    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_gpio_set_pad_wakeup {
    pub hdr: imx_sc_rpc_msg,
    pub pad: u16,
    pub wakeup: u8,
    pub __aligned(4): } __packed,
    pub pinctrl_ipc_handle: *mut static struct imx_sc_ipc,
#[no_mangle]
pub unsafe extern "C" fn imx_pinctrl_sc_ipc_init(pdev: *mut platform_device) -> c_int {
    int imx_pinctrl_sc_ipc_init(struct platform_device *pdev)
    {
    imx_scu_irq_group_enable(IMX_SC_IRQ_GROUP_WAKE,
    pub true): IMX_SC_IRQ_PAD,,
    pub imx_scu_get_handle(&pinctrl_ipc_handle): return,
    }
    int imx_pinconf_get_scu(struct pinctrl_dev *pctldev, unsigned pin_id,
    unsigned long *config)
    {
    pub msg: imx_sc_msg_req_pad_get,
    pub resp: *mut imx_sc_msg_resp_pad_get,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub ret: c_int,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_PAD: hdr->svc =,
    pub IMX_SC_PAD_FUNC_GET: hdr->func =,
    pub 2: hdr->size =,
    pub pin_id: msg.pad =,
    pub true): ret = imx_scu_call_rpc(pinctrl_ipc_handle, &msg,,
    if (ret)
    pub ret: return,
    pub )&msg: *mut resp = (struct imx_sc_msg_resp_pad_get,
// config = resp->val;
    pub 0: return,
    }
    int imx_pinconf_set_scu(struct pinctrl_dev *pctldev, unsigned pin_id,
    unsigned long *configs, unsigned num_configs)
    {
    pub pinctrl_dev_get_drvdata(pctldev): *mut *mut imx_pinctrl ipctl =,
    pub msg: imx_sc_msg_req_pad_set,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub configs: [unsigned int mux =; 0],
    pub conf: c_uint,
    pub val: c_uint,
    pub ret: c_int,
    if (num_configs == 1) {
    pub wmsg: imx_sc_msg_gpio_set_pad_wakeup,
    pub &wmsg.hdr: hdr =,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_PAD: hdr->svc =,
    pub IMX_SC_PAD_FUNC_SET_WAKEUP: hdr->func =,
    pub 2: hdr->size =,
    pub pin_id: wmsg.pad =,
    pub configs: *mut wmsg.wakeup =,
    pub true): ret = imx_scu_call_rpc(pinctrl_ipc_handle, &wmsg,,
    dev_dbg(ipctl.dev, "wakeup pin_id: %d type: %ld\n",
    pub configs): *mut pin_id,,
    pub ret: return,
    }
//
// Set mux and conf together in one IPC call
//
    pub 2): WARN_ON(num_configs !=,
    pub configs: [conf =; 1],
    pub BM_PAD_CTL_GP_ENABLE: val = conf | BM_PAD_CTL_IFMUX_ENABLE |,
    pub BP_PAD_CTL_IFMUX: val |= mux <<,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_PAD: hdr->svc =,
    pub IMX_SC_PAD_FUNC_SET: hdr->func =,
    pub 3: hdr->size =,
    pub pin_id: msg.pad =,
    pub val: msg.val =,
    pub true): ret = imx_scu_call_rpc(pinctrl_ipc_handle, &msg,,
    dev_dbg(ipctl.dev, "write: pin_id %u config 0x%x val 0x%x\n",
    pub val): pin_id, conf,,
    pub ret: return,
    }
    void imx_pinctrl_parse_pin_scu(struct imx_pinctrl *ipctl,
    unsigned int *pin_id, struct imx_pin *pin,
    const __be32 **list_p)
    {
    pub ipctl->info: *const *const imx_pinctrl_soc_info info =,
    pub &pin->conf.scu: *mut *mut imx_pin_scu pin_scu =,
    pub list_p: *const *const __be32 list =,
    pub be32_to_cpu(*list++): *mut pin->pin =,
// pin_id = pin->pin;
    pub be32_to_cpu(*list++): *mut pin_scu->mux_mode =,
    pub be32_to_cpu(*list++): *mut pin_scu->config =,
// list_p = list;
    dev_dbg(ipctl.dev, "%s: 0x%x 0x%08lx", info.pins[pin.pin].name,
    pub pin_scu->config): pin_scu->mux_mode,,
    }
    pub <aisheng.dong@nxp.com>"): MODULE_AUTHOR("Dong Aisheng,
    pub driver"): MODULE_DESCRIPTION("NXP i.MX SCU common pinctrl,
    pub v2"): MODULE_LICENSE("GPL,
