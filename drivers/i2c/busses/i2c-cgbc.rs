//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-cgbc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Congatec Board Controller I2C busses driver
//
// Copyright (C) 2024 Bootlin
// Author: Thomas Richard <thomas.richard@bootlin.com>
//

pub const CGBC_I2C_PRIMARY_BUS_ID: c_int = 0;
pub const CGBC_I2C_PM_BUS_ID: c_int = 4;
pub const CGBC_I2C_CMD_START: c_uint = 0x40;
pub const CGBC_I2C_CMD_STAT: c_uint = 0x48;
pub const CGBC_I2C_CMD_DATA: c_uint = 0x50;
pub const CGBC_I2C_CMD_SPEED: c_uint = 0x58;
pub const CGBC_I2C_STAT_IDL: c_uint = 0x00;
pub const CGBC_I2C_STAT_DAT: c_uint = 0x01;
pub const CGBC_I2C_STAT_BUSY: c_uint = 0x02;
pub const CGBC_I2C_START: c_uint = 0x80;
pub const CGBC_I2C_STOP: c_uint = 0x40;
pub const CGBC_I2C_LAST_ACK: c_uint = 0x80    /* send ACK on last read byte */;
//
// Reference code defines 1kHz as min freq and 6.1MHz as max freq.
// But in practice, the board controller limits the frequency to 1MHz, and the
// 1kHz is not functional (minimal working freq is 50kHz).
// So use these values as limits.
//

pub const CGBC_I2C_FREQ_UNIT_1KHZ: c_uint = 0x40;
pub const CGBC_I2C_FREQ_UNIT_10KHZ: c_uint = 0x80;
pub const CGBC_I2C_FREQ_UNIT_100KHZ: c_uint = 0xC0;
pub const CGBC_I2C_FREQ_UNIT_MASK: c_uint = 0xC0;
pub const CGBC_I2C_FREQ_VALUE_MASK: c_uint = 0x3F;
pub const CGBC_I2C_READ_MAX_LEN: c_int = 31;
pub const CGBC_I2C_WRITE_MAX_LEN: c_int = 32;
pub const CGBC_I2C_CMD_HEADER_SIZE: c_int = 4;

    enum cgbc_i2c_state {
    CGBC_I2C_STATE_DONE = 0,
    CGBC_I2C_STATE_INIT,
    CGBC_I2C_STATE_START,
    CGBC_I2C_STATE_READ,
    CGBC_I2C_STATE_WRITE,
    CGBC_I2C_STATE_ERROR,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_algo_cgbc_data {
    pub bus_id: u8,
    pub read_maxtime_us: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_i2c_data {
    pub dev: *mut device,
    pub cgbc: *mut cgbc_device_data,
    pub adap: i2c_adapter,
    pub msg: *mut i2c_msg,
    pub nmsgs: c_int,
    pub pos: c_int,
    pub state: enum cgbc_i2c_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_i2c_transfer {
    pub bus_id: u8,
    pub start: bool,
    pub stop: bool,
    pub last_ack: bool,
    pub read: u8,
    pub write: u8,
    pub addr: u8,
    pub data: [u8; CGBC_I2C_WRITE_MAX_LEN],
}

#[no_mangle]
unsafe extern "C" fn cgbc_i2c_freq_to_reg(bus_frequency: c_uint) -> u8 {
    static u8 cgbc_i2c_freq_to_reg(unsigned int bus_frequency)
    {
    u8 reg;
    if (bus_frequency <= 10000)
    reg = CGBC_I2C_FREQ_UNIT_1KHZ | (bus_frequency / 1000);
#[no_mangle]
pub unsafe extern "C" fn if(100000: bus_frequency <=) -> else {
    else if (bus_frequency <= 100000)
    reg = CGBC_I2C_FREQ_UNIT_10KHZ | (bus_frequency / 10000);
    else
    reg = CGBC_I2C_FREQ_UNIT_100KHZ | (bus_frequency / 100000);
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_i2c_reg_to_freq(reg: u8) -> c_uint {
    static unsigned int cgbc_i2c_reg_to_freq(u8 reg)
    {
    let mut freq: c_uint = reg & CGBC_I2C_FREQ_VALUE_MASK;
    let mut unit: u8 = reg & CGBC_I2C_FREQ_UNIT_MASK;
    if (unit == CGBC_I2C_FREQ_UNIT_100KHZ)
    return freq * 100000;
#[no_mangle]
pub unsafe extern "C" fn if(CGBC_I2C_FREQ_UNIT_10KHZ: unit ==) -> else {
    else if (unit == CGBC_I2C_FREQ_UNIT_10KHZ)
    return freq * 10000;
    else
    return freq * 1000;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_i2c_get_status(adap: *mut i2c_adapter) -> c_int {
    static int cgbc_i2c_get_status(struct i2c_adapter *adap)
    {
    struct i2c_algo_cgbc_data *algo_data = adap.algo_data;
    struct cgbc_i2c_data *i2c = i2c_get_adapdata(adap);
    struct cgbc_device_data *cgbc = i2c.cgbc;
    let mut cmd: u8 = CGBC_I2C_CMD_STAT | algo_data.bus_id;
    u8 status;
    int ret;
    ret = cgbc_command(cgbc, &cmd, sizeof(cmd), core::ptr::null_mut(), 0, &status);
    if (ret)
    return ret;
    return status;
    }
    static int cgbc_i2c_set_frequency(struct i2c_adapter *adap,
    unsigned int bus_frequency)
    {
    struct i2c_algo_cgbc_data *algo_data = adap.algo_data;
    struct cgbc_i2c_data *i2c = i2c_get_adapdata(adap);
    struct cgbc_device_data *cgbc = i2c.cgbc;
    u8 cmd[2], data;
    int ret;
    if (bus_frequency > CGBC_I2C_FREQ_MAX_HZ ||
    bus_frequency < CGBC_I2C_FREQ_MIN_HZ) {
    dev_info(i2c.dev, "invalid frequency %u, using default\n", bus_frequency);
    bus_frequency = I2C_MAX_STANDARD_MODE_FREQ;
    }
    cmd[0] = CGBC_I2C_CMD_SPEED | algo_data.bus_id;
    cmd[1] = cgbc_i2c_freq_to_reg(bus_frequency);
    ret = cgbc_command(cgbc, &cmd, sizeof(cmd), &data, 1, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(i2c.dev, ret,
    "Failed to initialize I2C bus %s",
    adap.name);
    cmd[1] = 0x00;
    ret = cgbc_command(cgbc, &cmd, sizeof(cmd), &data, 1, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(i2c.dev, ret,
    "Failed to get I2C bus frequency");
    bus_frequency = cgbc_i2c_reg_to_freq(data);
    dev_dbg(i2c.dev, "%s is running at %d Hz\n", adap.name, bus_frequency);
//
// The read_maxtime_us variable represents the maximum time to wait
// for data during a read operation. The maximum amount of data that
// can be read by a command is CGBC_I2C_READ_MAX_LEN.
// Therefore, calculate the max time to properly size the timeout.
//
    algo_data.read_maxtime_us = (BITS_PER_BYTE + 1) * CGBC_I2C_READ_MAX_LEN
// USEC_PER_SEC / bus_frequency;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_i2c_xfer_to_cmd(xfer: cgbc_i2c_transfer, cmd: *mut u8) -> c_uint {
    static unsigned int cgbc_i2c_xfer_to_cmd(struct cgbc_i2c_transfer xfer, u8 *cmd)
    {
    let mut i: c_int = 0;
    cmd[i++] = CGBC_I2C_CMD_START | xfer.bus_id;
    cmd[i] = (xfer.start) ? CGBC_I2C_START : 0x00;
    if (xfer.stop)
    cmd[i] |= CGBC_I2C_STOP;
    cmd[i++] |= (xfer.start) ? xfer.write + 1 : xfer.write;
    cmd[i++] = (xfer.last_ack) ? (xfer.read | CGBC_I2C_LAST_ACK) : xfer.read;
    if (xfer.start)
    cmd[i++] = xfer.addr;
    if (xfer.write > 0)
    memcpy(&cmd[i], &xfer.data, xfer.write);
    return i + xfer.write;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_i2c_xfer_msg(adap: *mut i2c_adapter) -> c_int {
    static int cgbc_i2c_xfer_msg(struct i2c_adapter *adap)
    {
    struct i2c_algo_cgbc_data *algo_data = adap.algo_data;
    struct cgbc_i2c_data *i2c = i2c_get_adapdata(adap);
    struct cgbc_device_data *cgbc = i2c.cgbc;
    struct i2c_msg *msg = i2c.msg;
    u8 cmd[CGBC_I2C_CMD_SIZE];
    int ret, max_len, len, i;
    unsigned int cmd_len;
    u8 cmd_data;
    struct cgbc_i2c_transfer xfer = {
    .bus_id = algo_data.bus_id,
    .addr = i2c_8bit_addr_from_msg(msg),
    };
    if (i2c.state == CGBC_I2C_STATE_DONE)
    return 0;
    ret = cgbc_i2c_get_status(adap);
    if (ret == CGBC_I2C_STAT_BUSY)
    return -EBUSY;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    goto err;
    if (i2c.state == CGBC_I2C_STATE_INIT ||
    (i2c.state == CGBC_I2C_STATE_WRITE && msg.flags & I2C_M_RD))
    xfer.start = true;
    i2c.state = (msg.flags & I2C_M_RD) ? CGBC_I2C_STATE_READ : CGBC_I2C_STATE_WRITE;
    max_len = (i2c.state == CGBC_I2C_STATE_READ) ?
    CGBC_I2C_READ_MAX_LEN : CGBC_I2C_WRITE_MAX_LEN;
    if (msg.len - i2c.pos > max_len) {
    len = max_len;
    } else {
    len = msg.len - i2c.pos;
    if (i2c.nmsgs == 1)
    xfer.stop = true;
    }
    if (i2c.state == CGBC_I2C_STATE_WRITE) {
    xfer.write = len;
    xfer.read = 0;
    for (i = 0; i < len; i++)
    xfer.data[i] = msg.buf[i2c.pos + i];
    cmd_len = cgbc_i2c_xfer_to_cmd(xfer, &cmd[0]);
    ret = cgbc_command(cgbc, &cmd, cmd_len, core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    goto err;
    } else if (i2c.state == CGBC_I2C_STATE_READ) {
    xfer.write = 0;
    xfer.read = len;
    if (i2c.nmsgs > 1 || msg.len - i2c.pos > max_len)
    xfer.read |= CGBC_I2C_LAST_ACK;
    cmd_len = cgbc_i2c_xfer_to_cmd(xfer, &cmd[0]);
    ret = cgbc_command(cgbc, &cmd, cmd_len, core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    goto err;
    ret = read_poll_timeout(cgbc_i2c_get_status, ret,
    ret != CGBC_I2C_STAT_BUSY, 0,
    2 * algo_data.read_maxtime_us, false, adap);
    if (ret < 0)
    goto err;
    cmd_data = CGBC_I2C_CMD_DATA | algo_data.bus_id;
    ret = cgbc_command(cgbc, &cmd_data, sizeof(cmd_data),
    msg.buf + i2c.pos, len, core::ptr::null_mut());
    if (ret)
    goto err;
    }
    if (len == (msg.len - i2c.pos)) {
    i2c.msg++;
    i2c.nmsgs--;
    i2c.pos = 0;
    } else {
    i2c.pos += len;
    }
    if (i2c.nmsgs == 0)
    i2c.state = CGBC_I2C_STATE_DONE;
    return 0;
    err:
    i2c.state = CGBC_I2C_STATE_ERROR;
    return ret;
    }
    static int cgbc_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    struct cgbc_i2c_data *i2c = i2c_get_adapdata(adap);
    let mut timeout: c_ulong = jiffies + HZ;
    int ret;
    i2c.state = CGBC_I2C_STATE_INIT;
    i2c.msg = msgs;
    i2c.nmsgs = num;
    i2c.pos = 0;
    while (time_before(jiffies, timeout)) {
    ret = cgbc_i2c_xfer_msg(adap);
    if (i2c.state == CGBC_I2C_STATE_DONE)
    return num;
    if (i2c.state == CGBC_I2C_STATE_ERROR)
    return ret;
    if (ret == 0)
    timeout = jiffies + HZ;
    }
    i2c.state = CGBC_I2C_STATE_ERROR;
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 cgbc_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | (I2C_FUNC_SMBUS_EMUL & ~(I2C_FUNC_SMBUS_QUICK));
    }
    static const struct i2c_algorithm cgbc_i2c_algorithm = {
    .xfer = cgbc_i2c_xfer,
    .functionality = cgbc_i2c_func,
    };
    static struct i2c_algo_cgbc_data cgbc_i2c_algo_data[] = {
    { .bus_id = CGBC_I2C_PRIMARY_BUS_ID },
    { .bus_id = CGBC_I2C_PM_BUS_ID },
    };
    static const struct i2c_adapter cgbc_i2c_adapter[] = {
    {
    .owner		= THIS_MODULE,
    .name		= "Congatec General Purpose I2C adapter",
    .class		= I2C_CLASS_DEPRECATED,
    .algo		= &cgbc_i2c_algorithm,
    .algo_data	= &cgbc_i2c_algo_data[0],
    .nr		= -1,
    },
    {
    .owner		= THIS_MODULE,
    .name		= "Congatec Power Management I2C adapter",
    .class		= I2C_CLASS_DEPRECATED,
    .algo		= &cgbc_i2c_algorithm,
    .algo_data	= &cgbc_i2c_algo_data[1],
    .nr		= -1,
    },
    };
#[no_mangle]
unsafe extern "C" fn cgbc_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int cgbc_i2c_probe(struct platform_device *pdev)
    {
    struct cgbc_device_data *cgbc = dev_get_drvdata(pdev.dev.parent);
    struct cgbc_i2c_data *i2c;
    int ret;
    i2c = devm_kzalloc(&pdev.dev, sizeof(*i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    i2c.cgbc = cgbc;
    i2c.dev = &pdev.dev;
    i2c.adap = cgbc_i2c_adapter[pdev.id];
    i2c.adap.dev.parent = i2c.dev;
    i2c_set_adapdata(&i2c.adap, i2c);
    platform_set_drvdata(pdev, i2c);
    ret = cgbc_i2c_set_frequency(&i2c.adap, I2C_MAX_STANDARD_MODE_FREQ);
    if (ret)
    return ret;
    return i2c_add_numbered_adapter(&i2c.adap);
    }
#[no_mangle]
unsafe extern "C" fn cgbc_i2c_remove(pdev: *mut platform_device) {
    static void cgbc_i2c_remove(struct platform_device *pdev)
    {
    struct cgbc_i2c_data *i2c = platform_get_drvdata(pdev);
    i2c_del_adapter(&i2c.adap);
    }
    static struct platform_driver cgbc_i2c_driver = {
    .driver = {
    .name = "cgbc-i2c",
    },
    .probe		= cgbc_i2c_probe,
    .remove		= cgbc_i2c_remove,
    };
    module_platform_driver(cgbc_i2c_driver);
    MODULE_DESCRIPTION("Congatec Board Controller I2C Driver");
    MODULE_AUTHOR("Thomas Richard <thomas.richard@bootlin.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:cgbc_i2c");
