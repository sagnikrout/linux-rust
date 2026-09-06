//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/i2c-slave-testunit.c
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
// I2C slave mode testunit
//
// Copyright (C) 2020 by Wolfram Sang, Sang Engineering <wsa@sang-engineering.com>
// Copyright (C) 2020 by Renesas Electronics Corporation
//

pub const TU_VERSION_MAX_LENGTH: c_int = 128;
    enum testunit_cmds {
    TU_CMD_READ_BYTES = 1,	/* save 0 for ABORT, RESET or similar */
    TU_CMD_SMBUS_HOST_NOTIFY,
    TU_CMD_SMBUS_BLOCK_PROC_CALL,
    TU_CMD_GET_VERSION_WITH_REP_START,
    TU_CMD_SMBUS_ALERT_REQUEST,
    TU_NUM_CMDS
    };
    enum testunit_regs {
    TU_REG_CMD,
    TU_REG_DATAL,
    TU_REG_DATAH,
    TU_REG_DELAY,
    TU_NUM_REGS
    };
    enum testunit_flags {
    TU_FLAG_IN_PROCESS,
    TU_FLAG_NACK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct testunit_data {
    pub flags: c_ulong,
    pub regs: [u8; TU_NUM_REGS],
    pub reg_idx: u8,
    pub read_idx: u8,
    pub client: *mut i2c_client,
    pub worker: delayed_work,
    pub gpio: *mut gpio_desc,
    pub alert_done: completion,
}

    static char tu_version_info[] = "v" UTS_RELEASE "\n\0";
    static int i2c_slave_testunit_smbalert_cb(struct i2c_client *client,
    enum i2c_slave_event event, u8 *val)
    {
    struct testunit_data *tu = i2c_get_clientdata(client);
    switch (event) {
    case I2C_SLAVE_READ_PROCESSED:
    gpiod_set_value(tu.gpio, 0);
    fallthrough;
    case I2C_SLAVE_READ_REQUESTED:
// val = tu->regs[TU_REG_DATAL];
    break;
    case I2C_SLAVE_STOP:
    complete(&tu.alert_done);
    break;
    case I2C_SLAVE_WRITE_REQUESTED:
    case I2C_SLAVE_WRITE_RECEIVED:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int i2c_slave_testunit_slave_cb(struct i2c_client *client,
    enum i2c_slave_event event, u8 *val)
    {
    struct testunit_data *tu = i2c_get_clientdata(client);
    bool is_proc_call = tu.reg_idx == 3 && tu.regs[TU_REG_DATAL] == 1 &&
    tu.regs[TU_REG_CMD] == TU_CMD_SMBUS_BLOCK_PROC_CALL;
    bool is_get_version = tu.reg_idx == 3 &&
    tu.regs[TU_REG_CMD] == TU_CMD_GET_VERSION_WITH_REP_START;
    let mut ret: c_int = 0;
    switch (event) {
    case I2C_SLAVE_WRITE_REQUESTED:
    if (test_bit(TU_FLAG_IN_PROCESS | TU_FLAG_NACK, &tu.flags)) {
    ret = -EBUSY;
    break;
    }
    memset(tu.regs, 0, TU_NUM_REGS);
    tu.reg_idx = 0;
    tu.read_idx = 0;
    break;
    case I2C_SLAVE_WRITE_RECEIVED:
    if (test_bit(TU_FLAG_IN_PROCESS | TU_FLAG_NACK, &tu.flags)) {
    ret = -EBUSY;
    break;
    }
    if (tu.reg_idx < TU_NUM_REGS)
    tu.regs[tu.reg_idx] = *val;
    else
    ret = -EMSGSIZE;
    if (tu.reg_idx <= TU_NUM_REGS)
    tu.reg_idx++;
// TU_REG_CMD always written at this point
    if (tu.regs[TU_REG_CMD] >= TU_NUM_CMDS)
    ret = -EINVAL;
    break;
    case I2C_SLAVE_STOP:
    if (tu.reg_idx == TU_NUM_REGS) {
    set_bit(TU_FLAG_IN_PROCESS, &tu.flags);
    queue_delayed_work(system_dfl_long_wq, &tu.worker,
    msecs_to_jiffies(10 * tu.regs[TU_REG_DELAY]));
    }
//
// Reset reg_idx to avoid that work gets queued again in case of
// STOP after a following read message. But do not clear TU regs
// here because we still need them in the workqueue!
//
    tu.reg_idx = 0;
    clear_bit(TU_FLAG_NACK, &tu.flags);
    break;
    case I2C_SLAVE_READ_PROCESSED:
// Advance until we reach the NUL character
    if (is_get_version && tu_version_info[tu.read_idx] != 0)
    tu.read_idx++;
#[no_mangle]
pub unsafe extern "C" fn if(tu->regs[TU_REG_DATAH]: is_proc_call &&) -> else {
    else if (is_proc_call && tu.regs[TU_REG_DATAH])
    tu.regs[TU_REG_DATAH]--;
    fallthrough;
    case I2C_SLAVE_READ_REQUESTED:
    if (is_get_version)
// val = tu_version_info[tu->read_idx];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_proc_call) -> else {
    else if (is_proc_call)
// val = tu->regs[TU_REG_DATAH];
    else
// val = test_bit(TU_FLAG_IN_PROCESS, &tu->flags) ?
    tu.regs[TU_REG_CMD] : 0;
    break;
    }
// If an error occurred somewhen, we NACK everything until next STOP
    if (ret)
    set_bit(TU_FLAG_NACK, &tu.flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_slave_testunit_work(work: *mut work_struct) {
    static void i2c_slave_testunit_work(struct work_struct *work)
    {
    struct testunit_data *tu = container_of(work, struct testunit_data, worker.work);
    unsigned long time_left;
    struct i2c_msg msg;
    u8 msgbuf[256];
    u16 orig_addr;
    let mut ret: c_int = 0;
    msg.addr = I2C_CLIENT_END;
    msg.buf = msgbuf;
    switch (tu.regs[TU_REG_CMD]) {
    case TU_CMD_READ_BYTES:
    msg.addr = tu.regs[TU_REG_DATAL];
    msg.flags = I2C_M_RD;
    msg.len = tu.regs[TU_REG_DATAH];
    break;
    case TU_CMD_SMBUS_HOST_NOTIFY:
    msg.addr = 0x08;
    msg.flags = 0;
    msg.len = 3;
    msgbuf[0] = tu.client.addr;
    msgbuf[1] = tu.regs[TU_REG_DATAL];
    msgbuf[2] = tu.regs[TU_REG_DATAH];
    break;
    case TU_CMD_SMBUS_ALERT_REQUEST:
    if (!tu.gpio) {
    ret = -ENOENT;
    break;
    }
    i2c_slave_unregister(tu.client);
    orig_addr = tu.client.addr;
    tu.client.addr = 0x0c;
    ret = i2c_slave_register(tu.client, i2c_slave_testunit_smbalert_cb);
    if (ret)
    goto out_smbalert;
    reinit_completion(&tu.alert_done);
    gpiod_set_value(tu.gpio, 1);
    time_left = wait_for_completion_timeout(&tu.alert_done, HZ);
    if (!time_left)
    ret = -ETIMEDOUT;
    i2c_slave_unregister(tu.client);
    out_smbalert:
    tu.client.addr = orig_addr;
    i2c_slave_register(tu.client, i2c_slave_testunit_slave_cb);
    break;
    default:
    break;
    }
    if (msg.addr != I2C_CLIENT_END) {
    ret = i2c_transfer(tu.client.adapter, &msg, 1);
// convert '0 msgs transferred' to errno
    ret = (ret == 0) ? -EIO : ret;
    }
    if (ret < 0)
    dev_err(&tu.client.dev, "CMD%02X failed (%d)\n", tu.regs[TU_REG_CMD], ret);
    clear_bit(TU_FLAG_IN_PROCESS, &tu.flags);
    }
#[no_mangle]
unsafe extern "C" fn i2c_slave_testunit_probe(client: *mut i2c_client) -> c_int {
    static int i2c_slave_testunit_probe(struct i2c_client *client)
    {
    struct testunit_data *tu;
    tu = devm_kzalloc(&client.dev, sizeof(struct testunit_data), GFP_KERNEL);
    if (!tu)
    return -ENOMEM;
    tu.client = client;
    i2c_set_clientdata(client, tu);
    init_completion(&tu.alert_done);
    INIT_DELAYED_WORK(&tu.worker, i2c_slave_testunit_work);
    tu.gpio = devm_gpiod_get_index_optional(&client.dev, core::ptr::null_mut(), 0, GPIOD_OUT_LOW);
    if (IS_ERR(tu.gpio))
    return PTR_ERR(tu.gpio);
    if (gpiod_cansleep(tu.gpio)) {
    dev_err(&client.dev, "GPIO access which may sleep is not allowed\n");
    return -EDEADLK;
    }
    if (sizeof(tu_version_info) > TU_VERSION_MAX_LENGTH)
    tu_version_info[TU_VERSION_MAX_LENGTH - 1] = 0;
    return i2c_slave_register(client, i2c_slave_testunit_slave_cb);
    };
#[no_mangle]
unsafe extern "C" fn i2c_slave_testunit_remove(client: *mut i2c_client) {
    static void i2c_slave_testunit_remove(struct i2c_client *client)
    {
    struct testunit_data *tu = i2c_get_clientdata(client);
    cancel_delayed_work_sync(&tu.worker);
    i2c_slave_unregister(client);
    }
    static const struct i2c_device_id i2c_slave_testunit_id[] = {
    { .name = "slave-testunit" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, i2c_slave_testunit_id);
    static struct i2c_driver i2c_slave_testunit_driver = {
    .driver = {
    .name = "i2c-slave-testunit",
    },
    .probe = i2c_slave_testunit_probe,
    .remove = i2c_slave_testunit_remove,
    .id_table = i2c_slave_testunit_id,
    };
    module_i2c_driver(i2c_slave_testunit_driver);
    MODULE_AUTHOR("Wolfram Sang <wsa@sang-engineering.com>");
    MODULE_DESCRIPTION("I2C slave mode test unit");
    MODULE_LICENSE("GPL v2");
