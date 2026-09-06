//! Automatically rewritten from C to Rust
//! Source: drivers/rpmsg/qcom_glink_ssr.c
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
// Copyright (c) 2014-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2017, Linaro Ltd.
//

//
// struct do_cleanup_msg - The data structure for an SSR do_cleanup message
// @version:	The G-Link SSR protocol version
// @command:	The G-Link SSR command - do_cleanup
// @seq_num:	Sequence number
// @name_len:	Length of the name of the subsystem being restarted
// @name:	G-Link edge name of the subsystem being restarted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct do_cleanup_msg {
    pub version: __le32,
    pub command: __le32,
    pub seq_num: __le32,
    pub name_len: __le32,
    pub name: [c_char; 32],
}

//
// struct cleanup_done_msg - The data structure for an SSR cleanup_done message
// @version:	The G-Link SSR protocol version
// @response:	The G-Link SSR response to a do_cleanup command, cleanup_done
// @seq_num:	Sequence number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cleanup_done_msg {
    pub version: __le32,
    pub response: __le32,
    pub seq_num: __le32,
}

//
// G-Link SSR protocol commands
//
pub const GLINK_SSR_DO_CLEANUP: c_int = 0;
pub const GLINK_SSR_CLEANUP_DONE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glink_ssr {
    pub dev: *mut device,
    pub ept: *mut rpmsg_endpoint,
    pub nb: notifier_block,
    pub seq_num: u32,
    pub completion: completion,
}

// Notifier list for all registered glink_ssr instances
    static BLOCKING_NOTIFIER_HEAD(ssr_notifiers);
//
// qcom_glink_ssr_notify() - notify GLINK SSR about stopped remoteproc
// @ssr_name:	name of the remoteproc that has been stopped
//
#[no_mangle]
pub unsafe extern "C" fn qcom_glink_ssr_notify(ssr_name: *const c_char) {
    void qcom_glink_ssr_notify(const char *ssr_name)
    {
    blocking_notifier_call_chain(&ssr_notifiers, 0, (void *)ssr_name);
    }
    EXPORT_SYMBOL_GPL(qcom_glink_ssr_notify);
    static int qcom_glink_ssr_callback(struct rpmsg_device *rpdev,
    void *data, int len, void *priv, u32 addr)
    {
    struct cleanup_done_msg *msg = data;
    struct glink_ssr *ssr = dev_get_drvdata(&rpdev.dev);
    if (len < sizeof(*msg)) {
    dev_err(ssr.dev, "message too short\n");
    return -EINVAL;
    }
    if (le32_to_cpu(msg.version) != 0)
    return -EINVAL;
    if (le32_to_cpu(msg.response) != GLINK_SSR_CLEANUP_DONE)
    return 0;
    if (le32_to_cpu(msg.seq_num) != ssr.seq_num) {
    dev_err(ssr.dev, "invalid sequence number of response\n");
    return -EINVAL;
    }
    complete(&ssr.completion);
    return 0;
    }
    static int qcom_glink_ssr_notifier_call(struct notifier_block *nb,
    unsigned long event,
    void *data)
    {
    struct glink_ssr *ssr = container_of(nb, struct glink_ssr, nb);
    struct do_cleanup_msg msg;
    char *ssr_name = data;
    int ret;
    ssr.seq_num++;
    reinit_completion(&ssr.completion);
    memset(&msg, 0, sizeof(msg));
    msg.command = cpu_to_le32(GLINK_SSR_DO_CLEANUP);
    msg.seq_num = cpu_to_le32(ssr.seq_num);
    msg.name_len = cpu_to_le32(strlen(ssr_name));
    strscpy(msg.name, ssr_name, sizeof(msg.name));
    ret = rpmsg_send(ssr.ept, &msg, sizeof(msg));
    if (ret < 0)
    dev_err(ssr.dev, "failed to send cleanup message\n");
    ret = wait_for_completion_timeout(&ssr.completion, HZ);
    if (!ret)
    dev_err(ssr.dev, "timeout waiting for cleanup done message\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn qcom_glink_ssr_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int qcom_glink_ssr_probe(struct rpmsg_device *rpdev)
    {
    struct glink_ssr *ssr;
    ssr = devm_kzalloc(&rpdev.dev, sizeof(*ssr), GFP_KERNEL);
    if (!ssr)
    return -ENOMEM;
    init_completion(&ssr.completion);
    ssr.dev = &rpdev.dev;
    ssr.ept = rpdev.ept;
    ssr.nb.notifier_call = qcom_glink_ssr_notifier_call;
    dev_set_drvdata(&rpdev.dev, ssr);
    return blocking_notifier_chain_register(&ssr_notifiers, &ssr.nb);
    }
#[no_mangle]
unsafe extern "C" fn qcom_glink_ssr_remove(rpdev: *mut rpmsg_device) {
    static void qcom_glink_ssr_remove(struct rpmsg_device *rpdev)
    {
    struct glink_ssr *ssr = dev_get_drvdata(&rpdev.dev);
    blocking_notifier_chain_unregister(&ssr_notifiers, &ssr.nb);
    }
    static const struct rpmsg_device_id qcom_glink_ssr_match[] = {
    { "glink_ssr" },
    {}
    };
    MODULE_DEVICE_TABLE(rpmsg, qcom_glink_ssr_match);
    static struct rpmsg_driver qcom_glink_ssr_driver = {
    .probe = qcom_glink_ssr_probe,
    .remove = qcom_glink_ssr_remove,
    .callback = qcom_glink_ssr_callback,
    .id_table = qcom_glink_ssr_match,
    .drv = {
    .name = "qcom_glink_ssr",
    },
    };
    module_rpmsg_driver(qcom_glink_ssr_driver);
