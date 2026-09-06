//! Automatically rewritten from C to Rust
//! Source: drivers/platform/arm64/lenovo-yoga-c630.c
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
// Copyright (c) 2022-2024, Linaro Ltd
// Authors:
// Bjorn Andersson
// Dmitry Baryshkov
//

pub const LENOVO_EC_RESPONSE_REG: c_uint = 0x01;
pub const LENOVO_EC_REQUEST_REG: c_uint = 0x02;
pub const LENOVO_EC_UCSI_WRITE: c_uint = 0x20;
pub const LENOVO_EC_UCSI_READ: c_uint = 0x21;
pub const LENOVO_EC_READ_REG: c_uint = 0xb0;
pub const LENOVO_EC_REQUEST_NEXT_EVENT: c_uint = 0x84;
pub const LENOVO_EC_UCSI_VERSION: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yoga_c630_ec {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub notifier_list: blocking_notifier_head,
}

    static int yoga_c630_ec_request(struct yoga_c630_ec *ec, u8 *req, size_t req_len,
    u8 *resp, size_t resp_len)
    {
    int ret;
    lockdep_assert_held(&ec.lock);
    ret = i2c_smbus_write_i2c_block_data(ec.client, LENOVO_EC_REQUEST_REG,
    req_len, req);
    if (ret < 0)
    return ret;
    return i2c_smbus_read_i2c_block_data(ec.client, LENOVO_EC_RESPONSE_REG,
    resp_len, resp);
    }
#[no_mangle]
pub unsafe extern "C" fn yoga_c630_ec_read8(ec: *mut yoga_c630_ec, addr: u8) -> c_int {
    int yoga_c630_ec_read8(struct yoga_c630_ec *ec, u8 addr)
    {
    u8 req[2] = { LENOVO_EC_READ_REG, };
    int ret;
    u8 val;
    guard(mutex)(&ec.lock);
    req[1] = addr;
    ret = yoga_c630_ec_request(ec, req, sizeof(req), &val, 1);
    if (ret < 0)
    return ret;
    return val;
    }
    EXPORT_SYMBOL_GPL(yoga_c630_ec_read8);
#[no_mangle]
pub unsafe extern "C" fn yoga_c630_ec_read16(ec: *mut yoga_c630_ec, addr: u8) -> c_int {
    int yoga_c630_ec_read16(struct yoga_c630_ec *ec, u8 addr)
    {
    u8 req[2] = { LENOVO_EC_READ_REG, };
    int ret;
    u8 msb;
    u8 lsb;
// don't overflow the address
    if (addr == 0xff)
    return -EINVAL;
    guard(mutex)(&ec.lock);
    req[1] = addr;
    ret = yoga_c630_ec_request(ec, req, sizeof(req), &lsb, 1);
    if (ret < 0)
    return ret;
    req[1] = addr + 1;
    ret = yoga_c630_ec_request(ec, req, sizeof(req), &msb, 1);
    if (ret < 0)
    return ret;
    return msb << 8 | lsb;
    }
    EXPORT_SYMBOL_GPL(yoga_c630_ec_read16);
#[no_mangle]
pub unsafe extern "C" fn yoga_c630_ec_ucsi_get_version(ec: *mut yoga_c630_ec) -> u16 {
    u16 yoga_c630_ec_ucsi_get_version(struct yoga_c630_ec *ec)
    {
    u8 req[3] = { 0xb3, 0xf2, };
    int ret;
    u8 msb;
    u8 lsb;
    guard(mutex)(&ec.lock);
    req[2] = LENOVO_EC_UCSI_VERSION;
    ret = yoga_c630_ec_request(ec, req, sizeof(req), &lsb, 1);
    if (ret < 0)
    return ret;
    req[2] = LENOVO_EC_UCSI_VERSION + 1;
    ret = yoga_c630_ec_request(ec, req, sizeof(req), &msb, 1);
    if (ret < 0)
    return ret;
    return msb << 8 | lsb;
    }
    EXPORT_SYMBOL_GPL(yoga_c630_ec_ucsi_get_version);
    int yoga_c630_ec_ucsi_write(struct yoga_c630_ec *ec,
    const u8 req[YOGA_C630_UCSI_WRITE_SIZE])
    {
    int ret;
    mutex_lock(&ec.lock);
    ret = i2c_smbus_write_i2c_block_data(ec.client, LENOVO_EC_UCSI_WRITE,
    YOGA_C630_UCSI_WRITE_SIZE, req);
    mutex_unlock(&ec.lock);
    return ret < 0 ? ret : 0;
    }
    EXPORT_SYMBOL_GPL(yoga_c630_ec_ucsi_write);
    int yoga_c630_ec_ucsi_read(struct yoga_c630_ec *ec,
    u8 resp[YOGA_C630_UCSI_READ_SIZE])
    {
    int ret;
    mutex_lock(&ec.lock);
    ret = i2c_smbus_read_i2c_block_data(ec.client, LENOVO_EC_UCSI_READ,
    YOGA_C630_UCSI_READ_SIZE, resp);
    mutex_unlock(&ec.lock);
    return ret < 0 ? ret : 0;
    }
    EXPORT_SYMBOL_GPL(yoga_c630_ec_ucsi_read);
#[no_mangle]
unsafe extern "C" fn yoga_c630_ec_thread_intr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t yoga_c630_ec_thread_intr(int irq, void *data)
    {
    u8 req[] = { LENOVO_EC_REQUEST_NEXT_EVENT };
    struct yoga_c630_ec *ec = data;
    u8 event;
    int ret;
    mutex_lock(&ec.lock);
    ret = yoga_c630_ec_request(ec, req, sizeof(req), &event, 1);
    mutex_unlock(&ec.lock);
    if (ret < 0)
    return IRQ_HANDLED;
    blocking_notifier_call_chain(&ec.notifier_list, event, ec);
    return IRQ_HANDLED;
    }
//
// yoga_c630_ec_register_notify - Register a notifier callback for EC events.
// @ec: Yoga C630 EC
// @nb: Notifier block pointer to register
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn yoga_c630_ec_register_notify(ec: *mut yoga_c630_ec, nb: *mut notifier_block) -> c_int {
    int yoga_c630_ec_register_notify(struct yoga_c630_ec *ec, struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&ec.notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(yoga_c630_ec_register_notify);
//
// yoga_c630_ec_unregister_notify - Unregister notifier callback for EC events.
// @ec: Yoga C630 EC
// @nb: Notifier block pointer to unregister
//
// Unregister a notifier callback that was previously registered with
// yoga_c630_ec_register_notify().
//
#[no_mangle]
pub unsafe extern "C" fn yoga_c630_ec_unregister_notify(ec: *mut yoga_c630_ec, nb: *mut notifier_block) {
    void yoga_c630_ec_unregister_notify(struct yoga_c630_ec *ec, struct notifier_block *nb)
    {
    blocking_notifier_chain_unregister(&ec.notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(yoga_c630_ec_unregister_notify);
    static int yoga_c630_aux_init(struct device *parent, const char *name,
    struct yoga_c630_ec *ec)
    {
    struct auxiliary_device *adev;
    adev = devm_auxiliary_device_create(parent, name, ec);
    if (!adev)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn yoga_c630_ec_probe(client: *mut i2c_client) -> c_int {
    static int yoga_c630_ec_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct yoga_c630_ec *ec;
    int ret;
    ec = devm_kzalloc(dev, sizeof(*ec), GFP_KERNEL);
    if (!ec)
    return -ENOMEM;
    mutex_init(&ec.lock);
    ec.client = client;
    BLOCKING_INIT_NOTIFIER_HEAD(&ec.notifier_list);
    ret = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), yoga_c630_ec_thread_intr,
    IRQF_ONESHOT, "yoga_c630_ec", ec);
    if (ret < 0)
    return dev_err_probe(dev, ret, "unable to request irq\n");
    ret = yoga_c630_aux_init(dev, YOGA_C630_DEV_PSY, ec);
    if (ret)
    return ret;
    return yoga_c630_aux_init(dev, YOGA_C630_DEV_UCSI, ec);
    }
    static const struct of_device_id yoga_c630_ec_of_match[] = {
    { .compatible = "lenovo,yoga-c630-ec" },
    {}
    };
    MODULE_DEVICE_TABLE(of, yoga_c630_ec_of_match);
    static const struct i2c_device_id yoga_c630_ec_i2c_id_table[] = {
    { .name = "yoga-c630-ec" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, yoga_c630_ec_i2c_id_table);
    static struct i2c_driver yoga_c630_ec_i2c_driver = {
    .driver = {
    .name = "yoga-c630-ec",
    .of_match_table = yoga_c630_ec_of_match
    },
    .probe = yoga_c630_ec_probe,
    .id_table = yoga_c630_ec_i2c_id_table,
    };
    module_i2c_driver(yoga_c630_ec_i2c_driver);
    MODULE_DESCRIPTION("Lenovo Yoga C630 Embedded Controller");
    MODULE_LICENSE("GPL");
