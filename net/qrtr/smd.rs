//! Automatically rewritten from C to Rust
//! Source: net/qrtr/smd.c
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
// Copyright (c) 2015, Sony Mobile Communications Inc.
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qrtr_smd_dev {
    pub ep: qrtr_endpoint,
    pub channel: *mut rpmsg_endpoint,
    pub dev: *mut device,
}

// from smd to qrtr
    static int qcom_smd_qrtr_callback(struct rpmsg_device *rpdev,
    void *data, int len, void *priv, u32 addr)
    {
    struct qrtr_smd_dev *qdev = dev_get_drvdata(&rpdev.dev);
    int rc;
    if (!qdev)
    return -EAGAIN;
    rc = qrtr_endpoint_post(&qdev.ep, data, len);
    if (rc == -EINVAL) {
    dev_err(qdev.dev, "invalid ipcrouter packet\n");
// return 0 to let smd drop the packet
    rc = 0;
    }
    return rc;
    }
// from qrtr to smd
#[no_mangle]
unsafe extern "C" fn qcom_smd_qrtr_send(ep: *mut qrtr_endpoint, skb: *mut sk_buff) -> c_int {
    static int qcom_smd_qrtr_send(struct qrtr_endpoint *ep, struct sk_buff *skb)
    {
    struct qrtr_smd_dev *qdev = container_of(ep, struct qrtr_smd_dev, ep);
    int rc;
    rc = skb_linearize(skb);
    if (rc)
    goto out;
    rc = rpmsg_send(qdev.channel, skb.data, skb.len);
    out:
    if (rc)
    kfree_skb(skb);
    else
    consume_skb(skb);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn qcom_smd_qrtr_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int qcom_smd_qrtr_probe(struct rpmsg_device *rpdev)
    {
    struct qrtr_smd_dev *qdev;
    int rc;
    qdev = devm_kzalloc(&rpdev.dev, sizeof(*qdev), GFP_KERNEL);
    if (!qdev)
    return -ENOMEM;
    qdev.channel = rpdev.ept;
    qdev.dev = &rpdev.dev;
    qdev.ep.xmit = qcom_smd_qrtr_send;
    rc = qrtr_endpoint_register(&qdev.ep, QRTR_EP_NID_AUTO);
    if (rc)
    return rc;
    dev_set_drvdata(&rpdev.dev, qdev);
    dev_dbg(&rpdev.dev, "Qualcomm SMD QRTR driver probed\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_smd_qrtr_remove(rpdev: *mut rpmsg_device) {
    static void qcom_smd_qrtr_remove(struct rpmsg_device *rpdev)
    {
    struct qrtr_smd_dev *qdev = dev_get_drvdata(&rpdev.dev);
    qrtr_endpoint_unregister(&qdev.ep);
    dev_set_drvdata(&rpdev.dev, core::ptr::null_mut());
    }
    static const struct rpmsg_device_id qcom_smd_qrtr_smd_match[] = {
    { "IPCRTR" },
    {}
    };
    static struct rpmsg_driver qcom_smd_qrtr_driver = {
    .probe = qcom_smd_qrtr_probe,
    .remove = qcom_smd_qrtr_remove,
    .callback = qcom_smd_qrtr_callback,
    .id_table = qcom_smd_qrtr_smd_match,
    .drv = {
    .name = "qcom_smd_qrtr",
    },
    };
    module_rpmsg_driver(qcom_smd_qrtr_driver);
    MODULE_ALIAS("rpmsg:IPCRTR");
    MODULE_DESCRIPTION("Qualcomm IPC-Router SMD interface driver");
    MODULE_LICENSE("GPL v2");
