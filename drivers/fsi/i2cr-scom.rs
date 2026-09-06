//! Automatically rewritten from C to Rust
//! Source: drivers/fsi/i2cr-scom.c
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
// Copyright (C) IBM Corporation 2023

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2cr_scom {
    pub dev: device,
    pub cdev: cdev,
    pub i2cr: *mut fsi_master_i2cr,
}

#[no_mangle]
unsafe extern "C" fn i2cr_scom_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    static loff_t i2cr_scom_llseek(struct file *file, loff_t offset, int whence)
    {
    switch (whence) {
    case SEEK_CUR:
    break;
    case SEEK_SET:
    file.f_pos = offset;
    break;
    default:
    return -EINVAL;
    }
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn i2cr_scom_read(filep: *mut file, buf: *mut char __user, len: usize, offset: *mut loff_t) -> isize {
    static ssize_t i2cr_scom_read(struct file *filep, char __user *buf, size_t len, loff_t *offset)
    {
    struct i2cr_scom *scom = filep.private_data;
    u64 data;
    int ret;
    if (len != sizeof(data))
    return -EINVAL;
    ret = fsi_master_i2cr_read(scom.i2cr, (u32)*offset, &data);
    if (ret)
    return ret;
    ret = copy_to_user(buf, &data, len);
    if (ret)
    return ret;
    return len;
    }
    static ssize_t i2cr_scom_write(struct file *filep, const char __user *buf, size_t len,
    loff_t *offset)
    {
    struct i2cr_scom *scom = filep.private_data;
    u64 data;
    int ret;
    if (len != sizeof(data))
    return -EINVAL;
    ret = copy_from_user(&data, buf, len);
    if (ret)
    return ret;
    ret = fsi_master_i2cr_write(scom.i2cr, (u32)*offset, data);
    if (ret)
    return ret;
    return len;
    }
    static const struct file_operations i2cr_scom_fops = {
    .owner		= THIS_MODULE,
    .open		= simple_open,
    .llseek		= i2cr_scom_llseek,
    .read		= i2cr_scom_read,
    .write		= i2cr_scom_write,
    };
#[no_mangle]
unsafe extern "C" fn i2cr_scom_probe(fsi_dev: *mut fsi_device) -> c_int {
    static int i2cr_scom_probe(struct fsi_device *fsi_dev)
    {
    struct device *dev = &fsi_dev.dev;
    struct i2cr_scom *scom;
    int didx;
    int ret;
    if (!is_fsi_master_i2cr(fsi_dev.slave.master))
    return -ENODEV;
    scom = devm_kzalloc(dev, sizeof(*scom), GFP_KERNEL);
    if (!scom)
    return -ENOMEM;
    scom.i2cr = to_fsi_master_i2cr(fsi_dev.slave.master);
    dev_set_drvdata(dev, scom);
    scom.dev.type = &fsi_cdev_type;
    scom.dev.parent = dev;
    device_initialize(&scom.dev);
    ret = fsi_get_new_minor(fsi_dev, fsi_dev_scom, &scom.dev.devt, &didx);
    if (ret)
    return ret;
    dev_set_name(&scom.dev, "scom%d", didx);
    cdev_init(&scom.cdev, &i2cr_scom_fops);
    ret = cdev_device_add(&scom.cdev, &scom.dev);
    if (ret)
    fsi_free_minor(scom.dev.devt);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2cr_scom_remove(fsi_dev: *mut fsi_device) {
    static void i2cr_scom_remove(struct fsi_device *fsi_dev)
    {
    struct i2cr_scom *scom = dev_get_drvdata(&fsi_dev.dev);
    cdev_device_del(&scom.cdev, &scom.dev);
    fsi_free_minor(scom.dev.devt);
    }
    static const struct of_device_id i2cr_scom_of_ids[] = {
    { .compatible = "ibm,i2cr-scom" },
    { }
    };
    MODULE_DEVICE_TABLE(of, i2cr_scom_of_ids);
    static const struct fsi_device_id i2cr_scom_ids[] = {
    { 0x5, FSI_VERSION_ANY },
    { }
    };
    static struct fsi_driver i2cr_scom_driver = {
    .probe = i2cr_scom_probe,
    .remove = i2cr_scom_remove,
    .id_table = i2cr_scom_ids,
    .drv = {
    .name = "i2cr_scom",
    .of_match_table = i2cr_scom_of_ids,
    }
    };
    module_fsi_driver(i2cr_scom_driver);
    MODULE_AUTHOR("Eddie James <eajames@linux.ibm.com>");
    MODULE_DESCRIPTION("IBM I2C Responder SCOM driver");
    MODULE_LICENSE("GPL");
