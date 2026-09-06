//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/imx/imx-scu-irq.c
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
// Copyright 2019,2023 NXP
//
// Implementation of the SCU IRQ functions using MU.
//

pub const IMX_SC_IRQ_FUNC_ENABLE: c_int = 1;
pub const IMX_SC_IRQ_FUNC_STATUS: c_int = 2;
pub const IMX_SC_IRQ_NUM_GROUP: c_int = 9;
    static u32 mu_resource_id;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_irq_get_status {
    pub hdr: imx_sc_rpc_msg,
    union {
    struct {
    pub resource: u16,
    pub group: u8,
    pub reserved: u8,
    pub req: } __packed,
    struct {
    pub status: u32,
    pub resp: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_irq_enable {
    pub hdr: imx_sc_rpc_msg,
    pub mask: u32,
    pub resource: u16,
    pub group: u8,
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_wakeup {
    pub mask: u32,
    pub wakeup_src: u32,
    pub valid: bool,
}

// Sysfs functions
    static struct kobject *wakeup_obj;
    static ssize_t wakeup_source_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf);
    static struct kobj_attribute wakeup_source_attr =
    __ATTR(wakeup_src, 0660, wakeup_source_show, core::ptr::null_mut());
    static struct scu_wakeup scu_irq_wakeup[IMX_SC_IRQ_NUM_GROUP];
    static struct imx_sc_ipc *imx_sc_irq_ipc_handle;
    static struct work_struct imx_sc_irq_work;
    static BLOCKING_NOTIFIER_HEAD(imx_scu_irq_notifier_chain);
#[no_mangle]
pub unsafe extern "C" fn imx_scu_irq_register_notifier(nb: *mut notifier_block) -> c_int {
    int imx_scu_irq_register_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(
    &imx_scu_irq_notifier_chain, nb);
    }
    EXPORT_SYMBOL(imx_scu_irq_register_notifier);
#[no_mangle]
pub unsafe extern "C" fn imx_scu_irq_unregister_notifier(nb: *mut notifier_block) -> c_int {
    int imx_scu_irq_unregister_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(
    &imx_scu_irq_notifier_chain, nb);
    }
    EXPORT_SYMBOL(imx_scu_irq_unregister_notifier);
#[no_mangle]
unsafe extern "C" fn imx_scu_irq_notifier_call_chain(status: c_ulong, group: *mut u8) -> c_int {
    static int imx_scu_irq_notifier_call_chain(unsigned long status, u8 *group)
    {
    return blocking_notifier_call_chain(&imx_scu_irq_notifier_chain,
    status, (void *)group);
    }
#[no_mangle]
unsafe extern "C" fn imx_scu_irq_work_handler(work: *mut work_struct) {
    static void imx_scu_irq_work_handler(struct work_struct *work)
    {
    u32 irq_status;
    int ret;
    u8 i;
    for (i = 0; i < IMX_SC_IRQ_NUM_GROUP; i++) {
    if (scu_irq_wakeup[i].mask) {
    scu_irq_wakeup[i].valid = false;
    scu_irq_wakeup[i].wakeup_src = 0;
    }
    ret = imx_scu_irq_get_status(i, &irq_status);
    if (ret) {
    pr_err("get irq group %d status failed, ret %d\n",
    i, ret);
    return;
    }
    if (!irq_status)
    continue;
    if (scu_irq_wakeup[i].mask & irq_status) {
    scu_irq_wakeup[i].valid = true;
    scu_irq_wakeup[i].wakeup_src = irq_status & scu_irq_wakeup[i].mask;
    } else {
    scu_irq_wakeup[i].wakeup_src = irq_status;
    }
    pm_system_wakeup();
    imx_scu_irq_notifier_call_chain(irq_status, &i);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn imx_scu_irq_get_status(group: u8, irq_status: *mut u32) -> c_int {
    int imx_scu_irq_get_status(u8 group, u32 *irq_status)
    {
    struct imx_sc_msg_irq_get_status msg;
    struct imx_sc_rpc_msg *hdr = &msg.hdr;
    int ret;
    hdr.ver = IMX_SC_RPC_VERSION;
    hdr.svc = IMX_SC_RPC_SVC_IRQ;
    hdr.func = IMX_SC_IRQ_FUNC_STATUS;
    hdr.size = 2;
    msg.data.req.resource = mu_resource_id;
    msg.data.req.group = group;
    ret = imx_scu_call_rpc(imx_sc_irq_ipc_handle, &msg, true);
    if (ret)
    return ret;
    if (irq_status)
// irq_status = msg.data.resp.status;
    return 0;
    }
    EXPORT_SYMBOL(imx_scu_irq_get_status);
#[no_mangle]
pub unsafe extern "C" fn imx_scu_irq_group_enable(group: u8, mask: u32, enable: u8) -> c_int {
    int imx_scu_irq_group_enable(u8 group, u32 mask, u8 enable)
    {
    struct imx_sc_msg_irq_enable msg;
    struct imx_sc_rpc_msg *hdr = &msg.hdr;
    int ret;
    if (!imx_sc_irq_ipc_handle)
    return -EPROBE_DEFER;
    hdr.ver = IMX_SC_RPC_VERSION;
    hdr.svc = IMX_SC_RPC_SVC_IRQ;
    hdr.func = IMX_SC_IRQ_FUNC_ENABLE;
    hdr.size = 3;
    msg.resource = mu_resource_id;
    msg.group = group;
    msg.mask = mask;
    msg.enable = enable;
    ret = imx_scu_call_rpc(imx_sc_irq_ipc_handle, &msg, true);
    if (ret)
    pr_err("enable irq failed, group %d, mask %d, ret %d\n",
    group, mask, ret);
    if (enable)
    scu_irq_wakeup[group].mask |= mask;
    else
    scu_irq_wakeup[group].mask &= ~mask;
    return ret;
    }
    EXPORT_SYMBOL(imx_scu_irq_group_enable);
#[no_mangle]
unsafe extern "C" fn imx_scu_irq_callback(c: *mut mbox_client, msg: *mut c_void) {
    static void imx_scu_irq_callback(struct mbox_client *c, void *msg)
    {
    schedule_work(&imx_sc_irq_work);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_source_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t wakeup_source_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    int i;
    for (i = 0; i < IMX_SC_IRQ_NUM_GROUP; i++) {
    if (!scu_irq_wakeup[i].wakeup_src)
    continue;
    if (scu_irq_wakeup[i].valid)
    sprintf(buf, "Wakeup source group = %d, irq = 0x%x\n",
    i, scu_irq_wakeup[i].wakeup_src);
    else
    sprintf(buf, "Spurious SCU wakeup, group = %d, irq = 0x%x\n",
    i, scu_irq_wakeup[i].wakeup_src);
    }
    return strlen(buf);
    }
#[no_mangle]
pub unsafe extern "C" fn imx_scu_enable_general_irq_channel(dev: *mut device) -> c_int {
    int imx_scu_enable_general_irq_channel(struct device *dev)
    {
    struct of_phandle_args spec;
    struct mbox_client *cl;
    struct mbox_chan *ch;
    let mut ret: c_int = 0, i = 0;
    if (!of_parse_phandle_with_args(dev.of_node, "mboxes",
    "#mbox-cells", 0, &spec)) {
    i = of_alias_get_id(spec.np, "mu");
    of_node_put(spec.np);
    }
// use mu1 as general mu irq channel if failed
    if (i < 0)
    i = 1;
    mu_resource_id = IMX_SC_R_MU_0A + i;
    ret = imx_scu_get_handle(&imx_sc_irq_ipc_handle);
    if (ret)
    return ret;
    cl = devm_kzalloc(dev, sizeof(*cl), GFP_KERNEL);
    if (!cl)
    return -ENOMEM;
    cl.dev = dev;
    cl.rx_callback = imx_scu_irq_callback;
    INIT_WORK(&imx_sc_irq_work, imx_scu_irq_work_handler);
// SCU general IRQ uses general interrupt channel 3
    ch = mbox_request_channel_byname(cl, "gip3");
    if (IS_ERR(ch)) {
    ret = PTR_ERR(ch);
    dev_err(dev, "failed to request mbox chan gip3, ret %d\n", ret);
    goto free_cl;
    }
// Create directory under /sysfs/firmware
    wakeup_obj = kobject_create_and_add("scu_wakeup_source", firmware_kobj);
    if (!wakeup_obj) {
    ret = -ENOMEM;
    goto free_ch;
    }
    ret = sysfs_create_file(wakeup_obj, &wakeup_source_attr.attr);
    if (ret) {
    dev_err(dev, "Cannot create wakeup source src file......\n");
    kobject_put(wakeup_obj);
    goto free_ch;
    }
    return 0;
    free_ch:
    mbox_free_channel(ch);
    free_cl:
    devm_kfree(dev, cl);
    return ret;
    }
