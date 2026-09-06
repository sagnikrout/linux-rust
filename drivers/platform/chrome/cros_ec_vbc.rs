//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/cros_ec_vbc.c
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
// Expose the vboot context nvram to userspace
//
// Copyright (C) 2012 Google, Inc.
// Copyright (C) 2015 Collabora Ltd.

    static ssize_t vboot_context_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *att, char *buf,
    loff_t pos, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct cros_ec_dev *ec = to_cros_ec_dev(dev);
    struct cros_ec_device *ecdev = ec.ec_dev;
    struct cros_ec_command *msg;
//
// This should be a pointer to the same type as op field in
// struct ec_params_vbnvcontext.
//
    uint32_t *params_op;
    int err;
    let mut para_sz: usize = sizeof(*params_op);
    let mut resp_sz: usize = sizeof(struct ec_response_vbnvcontext);
    let mut payload: usize = max(para_sz, resp_sz);
    msg = kmalloc(sizeof(*msg) + payload, GFP_KERNEL);
    if (!msg)
    return -ENOMEM;
// NB: we only kmalloc()ated enough space for the op field
    params_op = (uint32_t *)msg.data;
// params_op = EC_VBNV_CONTEXT_OP_READ;
    msg.version = EC_VER_VBNV_CONTEXT;
    msg.command = EC_CMD_VBNV_CONTEXT;
    msg.outsize = para_sz;
    msg.insize = resp_sz;
    err = cros_ec_cmd_xfer_status(ecdev, msg);
    if (err < 0) {
    dev_err(dev, "Error sending read request: %d\n", err);
    kfree(msg);
    return err;
    }
    memcpy(buf, msg.data, resp_sz);
    kfree(msg);
    return resp_sz;
    }
    static ssize_t vboot_context_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf,
    loff_t pos, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct cros_ec_dev *ec = to_cros_ec_dev(dev);
    struct cros_ec_device *ecdev = ec.ec_dev;
    struct ec_params_vbnvcontext *params;
    struct cros_ec_command *msg;
    int err;
    let mut para_sz: usize = sizeof(*params);
    let mut data_sz: usize = sizeof(params.block);
// Only write full values
    if (count != data_sz)
    return -EINVAL;
    msg = kmalloc(sizeof(*msg) + para_sz, GFP_KERNEL);
    if (!msg)
    return -ENOMEM;
    params = (struct ec_params_vbnvcontext *)msg.data;
    params.op = EC_VBNV_CONTEXT_OP_WRITE;
    memcpy(params.block, buf, data_sz);
    msg.version = EC_VER_VBNV_CONTEXT;
    msg.command = EC_CMD_VBNV_CONTEXT;
    msg.outsize = para_sz;
    msg.insize = 0;
    err = cros_ec_cmd_xfer_status(ecdev, msg);
    if (err < 0) {
    dev_err(dev, "Error sending write request: %d\n", err);
    kfree(msg);
    return err;
    }
    kfree(msg);
    return data_sz;
    }
    static const BIN_ATTR_RW(vboot_context, 16);
    static const struct bin_attribute *const cros_ec_vbc_bin_attrs[] = {
    &bin_attr_vboot_context,
    core::ptr::null_mut()
    };
    static const struct attribute_group cros_ec_vbc_attr_group = {
    .name = "vbc",
    .bin_attrs = cros_ec_vbc_bin_attrs,
    };
#[no_mangle]
unsafe extern "C" fn cros_ec_vbc_probe(pd: *mut platform_device) -> c_int {
    static int cros_ec_vbc_probe(struct platform_device *pd)
    {
    struct cros_ec_dev *ec_dev = dev_get_drvdata(pd.dev.parent);
    struct device *dev = &pd.dev;
    int ret;
    ret = sysfs_create_group(&ec_dev.class_dev.kobj,
    &cros_ec_vbc_attr_group);
    if (ret < 0)
    dev_err(dev, "failed to create %s attributes. err=%d\n",
    cros_ec_vbc_attr_group.name, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_vbc_remove(pd: *mut platform_device) {
    static void cros_ec_vbc_remove(struct platform_device *pd)
    {
    struct cros_ec_dev *ec_dev = dev_get_drvdata(pd.dev.parent);
    sysfs_remove_group(&ec_dev.class_dev.kobj,
    &cros_ec_vbc_attr_group);
    }
    static const struct platform_device_id cros_ec_vbc_id[] = {
    { .name = DRV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(platform, cros_ec_vbc_id);
    static struct platform_driver cros_ec_vbc_driver = {
    .driver = {
    .name = DRV_NAME,
    },
    .probe = cros_ec_vbc_probe,
    .remove = cros_ec_vbc_remove,
    .id_table = cros_ec_vbc_id,
    };
    module_platform_driver(cros_ec_vbc_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Expose the vboot context nvram to userspace");
