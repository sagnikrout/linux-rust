//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-nct6694.c
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
//
// Nuvoton NCT6694 I2C adapter driver based on USB interface.
//
// Copyright (C) 2025 Nuvoton Technology Corp.
//

//
// USB command module type for NCT6694 I2C controller.
// This defines the module type used for communication with the NCT6694
// I2C controller over the USB interface.
//
pub const NCT6694_I2C_MOD: c_uint = 0x03;
// Command 00h - I2C Deliver
pub const NCT6694_I2C_DELIVER: c_uint = 0x00;
pub const NCT6694_I2C_DELIVER_SEL: c_uint = 0x00;
pub const NCT6694_I2C_MAX_XFER_SIZE: c_int = 64;
pub const NCT6694_I2C_MAX_DEVS: c_int = 6;
    static unsigned char br_reg[NCT6694_I2C_MAX_DEVS] = {[0 ... (NCT6694_I2C_MAX_DEVS - 1)] = 0xFF};
    module_param_array(br_reg, byte, core::ptr::null_mut(), 0644);
    MODULE_PARM_DESC(br_reg,
    "I2C Baudrate register per adapter: (0=25K, 1=50K, 2=100K, 3=200K, 4=400K, 5=800K, 6=1M), default=2");
    enum nct6694_i2c_baudrate {
    NCT6694_I2C_BR_25K = 0,
    NCT6694_I2C_BR_50K,
    NCT6694_I2C_BR_100K,
    NCT6694_I2C_BR_200K,
    NCT6694_I2C_BR_400K,
    NCT6694_I2C_BR_800K,
    NCT6694_I2C_BR_1M
    };
    struct __packed nct6694_i2c_deliver {
    u8 port;
    u8 br;
    u8 addr;
    u8 w_cnt;
    u8 r_cnt;
    u8 rsv[11];
    u8 write_data[NCT6694_I2C_MAX_XFER_SIZE];
    u8 read_data[NCT6694_I2C_MAX_XFER_SIZE];
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nct6694_i2c_data {
    pub dev: *mut device,
    pub nct6694: *mut nct6694,
    pub adapter: i2c_adapter,
    pub deliver: nct6694_i2c_deliver,
    pub port: c_uchar,
    pub br: c_uchar,
}

#[no_mangle]
unsafe extern "C" fn nct6694_i2c_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int nct6694_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    struct nct6694_i2c_data *data = adap.algo_data;
    struct nct6694_i2c_deliver *deliver = &data.deliver;
    static const struct nct6694_cmd_header cmd_hd = {
    .mod = NCT6694_I2C_MOD,
    .cmd = NCT6694_I2C_DELIVER,
    .sel = NCT6694_I2C_DELIVER_SEL,
    .len = cpu_to_le16(sizeof(*deliver))
    };
    int ret, i;
    for (i = 0; i < num; i++) {
    struct i2c_msg *msg_temp = &msgs[i];
    memset(deliver, 0, sizeof(*deliver));
    deliver.port = data.port;
    deliver.br = data.br;
    deliver.addr = i2c_8bit_addr_from_msg(msg_temp);
    if (msg_temp.flags & I2C_M_RD) {
    deliver.r_cnt = msg_temp.len;
    ret = nct6694_write_msg(data.nct6694, &cmd_hd, deliver);
    if (ret < 0)
    return ret;
    memcpy(msg_temp.buf, deliver.read_data, msg_temp.len);
    } else {
    deliver.w_cnt = msg_temp.len;
    memcpy(deliver.write_data, msg_temp.buf, msg_temp.len);
    ret = nct6694_write_msg(data.nct6694, &cmd_hd, deliver);
    if (ret < 0)
    return ret;
    }
    }
    return num;
    }
#[no_mangle]
unsafe extern "C" fn nct6694_i2c_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 nct6694_i2c_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_adapter_quirks nct6694_i2c_quirks = {
    .max_read_len = NCT6694_I2C_MAX_XFER_SIZE,
    .max_write_len = NCT6694_I2C_MAX_XFER_SIZE,
    };
    static const struct i2c_algorithm nct6694_i2c_algo = {
    .xfer = nct6694_i2c_xfer,
    .functionality = nct6694_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn nct6694_i2c_set_baudrate(data: *mut nct6694_i2c_data) -> c_int {
    static int nct6694_i2c_set_baudrate(struct nct6694_i2c_data *data)
    {
    if (data.port >= NCT6694_I2C_MAX_DEVS) {
    dev_err(data.dev, "Invalid I2C port index %d\n", data.port);
    return -EINVAL;
    }
    if (br_reg[data.port] > NCT6694_I2C_BR_1M) {
    dev_warn(data.dev, "Invalid baudrate %d for I2C%d, using 100K\n",
    br_reg[data.port], data.port);
    br_reg[data.port] = NCT6694_I2C_BR_100K;
    }
    data.br = br_reg[data.port];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nct6694_i2c_ida_free(d: *mut c_void) {
    static void nct6694_i2c_ida_free(void *d)
    {
    struct nct6694_i2c_data *data = d;
    struct nct6694 *nct6694 = data.nct6694;
    ida_free(&nct6694.i2c_ida, data.port);
    }
#[no_mangle]
unsafe extern "C" fn nct6694_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int nct6694_i2c_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct nct6694 *nct6694 = dev_get_drvdata(dev.parent);
    struct nct6694_i2c_data *data;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = dev;
    data.nct6694 = nct6694;
    ret = ida_alloc(&nct6694.i2c_ida, GFP_KERNEL);
    if (ret < 0)
    return ret;
    data.port = ret;
    ret = devm_add_action_or_reset(dev, nct6694_i2c_ida_free, data);
    if (ret)
    return ret;
    ret = nct6694_i2c_set_baudrate(data);
    if (ret)
    return ret;
    sprintf(data.adapter.name, "NCT6694 I2C Adapter %d", data.port);
    data.adapter.owner = THIS_MODULE;
    data.adapter.algo = &nct6694_i2c_algo;
    data.adapter.quirks = &nct6694_i2c_quirks;
    data.adapter.dev.parent = dev;
    data.adapter.algo_data = data;
    platform_set_drvdata(pdev, data);
    return devm_i2c_add_adapter(dev, &data.adapter);
    }
    static struct platform_driver nct6694_i2c_driver = {
    .driver = {
    .name	= "nct6694-i2c",
    },
    .probe		= nct6694_i2c_probe,
    };
    module_platform_driver(nct6694_i2c_driver);
    MODULE_DESCRIPTION("USB-I2C adapter driver for NCT6694");
    MODULE_AUTHOR("Ming Yu <tmyu0@nuvoton.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:nct6694-i2c");
