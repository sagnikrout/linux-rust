//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/smd-rpm.c
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
// Copyright (c) 2015, Sony Mobile Communications AB.
// Copyright (c) 2012-2013, The Linux Foundation. All rights reserved.
//

//
// struct qcom_smd_rpm - state of the rpm device driver
// @rpm_channel:	reference to the smd channel
// @dev:		rpm device
// @ack:		completion for acks
// @lock:		mutual exclusion around the send/complete pair
// @ack_status:		result of the rpm request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_smd_rpm {
    pub rpm_channel: *mut rpmsg_endpoint,
    pub dev: *mut device,
    pub ack: completion,
    pub lock: mutex,
    pub ack_status: c_int,
}

//
// struct qcom_rpm_header - header for all rpm requests and responses
// @service_type:	identifier of the service
// @length:		length of the payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rpm_header {
    pub service_type: __le32,
    pub length: __le32,
}

//
// struct qcom_rpm_request - request message to the rpm
// @msg_id:	identifier of the outgoing message
// @flags:	active/sleep state flags
// @type:	resource type
// @id:		resource id
// @data_len:	length of the payload following this header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rpm_request {
    pub msg_id: __le32,
    pub flags: __le32,
    pub type: __le32,
    pub id: __le32,
    pub data_len: __le32,
}

//
// struct qcom_rpm_message - response message from the rpm
// @msg_type:	indicator of the type of message
// @length:	the size of this message, including the message header
// @msg_id:	message id
// @message:	textual message from the rpm
//
// Multiple of these messages can be stacked in an rpm message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rpm_message {
    pub msg_type: __le32,
    pub length: __le32,
    union {
    pub msg_id: __le32,
    pub message): DECLARE_FLEX_ARRAY(u8,,
}

    };
pub const RPM_SERVICE_TYPE_REQUEST: c_uint = 0x00716572 /* "req\0" */;
pub const RPM_MSG_TYPE_ERR: c_uint = 0x00727265 /* "err\0" */;
pub const RPM_MSG_TYPE_MSG_ID: c_uint = 0x2367736d /* "msg#" */;
//
// qcom_rpm_smd_write - write @buf to @type:@id
// @rpm:	rpm handle
// @state:	active/sleep state flags
// @type:	resource type
// @id:		resource identifier
// @buf:	the data to be written
// @count:	number of bytes in @buf
//
    int qcom_rpm_smd_write(struct qcom_smd_rpm *rpm,
    int state,
    u32 type, u32 id,
    void *buf,
    size_t count)
    {
    let mut msg_id: static unsigned = 1;
    int left;
    int ret;
    struct {
    struct qcom_rpm_header hdr;
    struct qcom_rpm_request req;
    u8 payload[];
    } *pkt;
    let mut size: usize = sizeof(*pkt) + count;
// SMD packets to the RPM may not exceed 256 bytes
    if (WARN_ON(size >= 256))
    return -EINVAL;
    pkt = kmalloc(size, GFP_ATOMIC);
    if (!pkt)
    return -ENOMEM;
    mutex_lock(&rpm.lock);
    pkt.hdr.service_type = cpu_to_le32(RPM_SERVICE_TYPE_REQUEST);
    pkt.hdr.length = cpu_to_le32(sizeof(struct qcom_rpm_request) + count);
    pkt.req.msg_id = cpu_to_le32(msg_id++);
    pkt.req.flags = cpu_to_le32(state);
    pkt.req.type = cpu_to_le32(type);
    pkt.req.id = cpu_to_le32(id);
    pkt.req.data_len = cpu_to_le32(count);
    memcpy(pkt.payload, buf, count);
    ret = rpmsg_send(rpm.rpm_channel, pkt, size);
    if (ret)
    goto out;
    left = wait_for_completion_timeout(&rpm.ack, RPM_REQUEST_TIMEOUT);
    if (!left)
    ret = -ETIMEDOUT;
    else
    ret = rpm.ack_status;
    out:
    kfree(pkt);
    mutex_unlock(&rpm.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(qcom_rpm_smd_write);
    static int qcom_smd_rpm_callback(struct rpmsg_device *rpdev,
    void *data,
    int count,
    void *priv,
    u32 addr)
    {
    const struct qcom_rpm_header *hdr = data;
    let mut hdr_length: usize = le32_to_cpu(hdr.length);
    const struct qcom_rpm_message *msg;
    struct qcom_smd_rpm *rpm = dev_get_drvdata(&rpdev.dev);
    const u8 *buf = data + sizeof(struct qcom_rpm_header);
    const u8 *end = buf + hdr_length;
    char msgbuf[32];
    let mut status: c_int = 0;
    u32 len, msg_length;
    if (le32_to_cpu(hdr.service_type) != RPM_SERVICE_TYPE_REQUEST ||
    hdr_length < sizeof(struct qcom_rpm_message)) {
    dev_err(rpm.dev, "invalid request\n");
    return 0;
    }
    while (buf < end) {
    msg = (struct qcom_rpm_message *)buf;
    msg_length = le32_to_cpu(msg.length);
    switch (le32_to_cpu(msg.msg_type)) {
    case RPM_MSG_TYPE_MSG_ID:
    break;
    case RPM_MSG_TYPE_ERR:
    len = min_t(u32, ALIGN(msg_length, 4), sizeof(msgbuf));
    memcpy_fromio(msgbuf, msg.message, len);
    msgbuf[len - 1] = 0;
    if (!strcmp(msgbuf, "resource does not exist"))
    status = -ENXIO;
    else
    status = -EINVAL;
    break;
    }
    buf = PTR_ALIGN(buf + 2 * sizeof(u32) + msg_length, 4);
    }
    rpm.ack_status = status;
    complete(&rpm.ack);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_smd_rpm_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int qcom_smd_rpm_probe(struct rpmsg_device *rpdev)
    {
    struct qcom_smd_rpm *rpm;
    rpm = devm_kzalloc(&rpdev.dev, sizeof(*rpm), GFP_KERNEL);
    if (!rpm)
    return -ENOMEM;
    mutex_init(&rpm.lock);
    init_completion(&rpm.ack);
    rpm.dev = &rpdev.dev;
    rpm.rpm_channel = rpdev.ept;
    dev_set_drvdata(&rpdev.dev, rpm);
    return of_platform_populate(rpdev.dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), &rpdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn qcom_smd_rpm_remove(rpdev: *mut rpmsg_device) {
    static void qcom_smd_rpm_remove(struct rpmsg_device *rpdev)
    {
    of_platform_depopulate(&rpdev.dev);
    }
    static const struct of_device_id qcom_smd_rpm_of_match[] = {
    { .compatible = "qcom,glink-smd-rpm" },
    { .compatible = "qcom,smd-rpm" },
//
// Don't add any more compatibles to the list, two previous entryes
// should match all defined devices.
//
    { .compatible = "qcom,rpm-apq8084" },
    { .compatible = "qcom,rpm-ipq6018" },
    { .compatible = "qcom,rpm-ipq9574" },
    { .compatible = "qcom,rpm-msm8226" },
    { .compatible = "qcom,rpm-msm8909" },
    { .compatible = "qcom,rpm-msm8916" },
    { .compatible = "qcom,rpm-msm8936" },
    { .compatible = "qcom,rpm-msm8953" },
    { .compatible = "qcom,rpm-msm8974" },
    { .compatible = "qcom,rpm-msm8976" },
    { .compatible = "qcom,rpm-msm8994" },
    { .compatible = "qcom,rpm-msm8996" },
    { .compatible = "qcom,rpm-msm8998" },
    { .compatible = "qcom,rpm-sdm660" },
    { .compatible = "qcom,rpm-sm6115" },
    { .compatible = "qcom,rpm-sm6125" },
    { .compatible = "qcom,rpm-sm6375" },
    { .compatible = "qcom,rpm-qcm2290" },
    { .compatible = "qcom,rpm-qcs404" },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_smd_rpm_of_match);
    static struct rpmsg_driver qcom_smd_rpm_driver = {
    .probe = qcom_smd_rpm_probe,
    .remove = qcom_smd_rpm_remove,
    .callback = qcom_smd_rpm_callback,
    .drv  = {
    .name  = "qcom_smd_rpm",
    .of_match_table = qcom_smd_rpm_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn qcom_smd_rpm_init() -> int __init {
    static int __init qcom_smd_rpm_init(void)
    {
    return register_rpmsg_driver(&qcom_smd_rpm_driver);
    }
    arch_initcall(qcom_smd_rpm_init);
#[no_mangle]
unsafe extern "C" fn qcom_smd_rpm_exit() -> void __exit {
    static void __exit qcom_smd_rpm_exit(void)
    {
    unregister_rpmsg_driver(&qcom_smd_rpm_driver);
    }
    module_exit(qcom_smd_rpm_exit);
    MODULE_AUTHOR("Bjorn Andersson <bjorn.andersson@sonymobile.com>");
    MODULE_DESCRIPTION("Qualcomm SMD backed RPM driver");
    MODULE_LICENSE("GPL v2");
