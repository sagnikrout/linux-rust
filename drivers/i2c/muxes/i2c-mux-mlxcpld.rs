//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/muxes/i2c-mux-mlxcpld.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Mellanox i2c mux driver
//
// Copyright (C) 2016-2020 Mellanox Technologies
//

// mlxcpld_mux - mux control structure:
// @last_val - last selected register value or -1 if mux deselected
// @client - I2C device client
// @pdata: platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxcpld_mux {
    pub last_val: c_int,
    pub client: *mut i2c_client,
    pub pdata: mlxcpld_mux_plat_data,
}

// MUX logic description.
// Driver can support different mux control logic, according to CPLD
// implementation.
//
// Connectivity schema.
//
// i2c-mlxcpld                                 Digital               Analog
// driver
// *--------*                                 * -> mux1 (virt bus2) -> mux -> |
// | I2CLPC | i2c physical                    * -> mux2 (virt bus3) -> mux -> |
// | bridge | bus 1                 *---------*                               |
// | logic  |---------------------> * mux reg *                               |
// | in CPLD|                       *---------*                               |
// *--------*   i2c-mux-mlxpcld          ^    * -> muxn (virt busn) -> mux -> |
// |        driver                   |                                    |
// |        *---------------*        |                              Devices
// |        * CPLD (i2c bus)* select |
// |        * registers for *--------
// |        * mux selection * deselect
// |        *---------------
// |                 |
// <-------->     <----------->
// i2c cntrl      Board cntrl reg
// reg space      space (mux select,
// IO, LED, WD, info)
//
// Write to mux register. Don't use i2c_transfer() and i2c_smbus_xfer()
// for this as they will try to lock adapter a second time.
//
    static int mlxcpld_mux_reg_write(struct i2c_adapter *adap,
    struct mlxcpld_mux *mux, u32 val)
    {
    struct i2c_client *client = mux.client;
    union i2c_smbus_data data;
    struct i2c_msg msg;
    u8 buf[3];
    switch (mux.pdata.reg_size) {
    case 1:
    data.byte = val;
    return __i2c_smbus_xfer(adap, client.addr, client.flags,
    I2C_SMBUS_WRITE, mux.pdata.sel_reg_addr,
    I2C_SMBUS_BYTE_DATA, &data);
    case 2:
    buf[0] = mux.pdata.sel_reg_addr >> 8;
    buf[1] = mux.pdata.sel_reg_addr;
    buf[2] = val;
    msg.addr = client.addr;
    msg.buf = buf;
    msg.len = mux.pdata.reg_size + 1;
    msg.flags = 0;
    return __i2c_transfer(adap, &msg, 1);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn mlxcpld_mux_select_chan(muxc: *mut i2c_mux_core, chan: u32) -> c_int {
    static int mlxcpld_mux_select_chan(struct i2c_mux_core *muxc, u32 chan)
    {
    struct mlxcpld_mux *mux = i2c_mux_priv(muxc);
    let mut regval: u32 = chan;
    let mut err: c_int = 0;
    if (mux.pdata.reg_size == 1)
    regval += 1;
// Only select the channel if its different from the last channel
    if (mux.last_val != regval) {
    err = mlxcpld_mux_reg_write(muxc.parent, mux, regval);
    mux.last_val = err < 0 ? -1 : regval;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxcpld_mux_deselect(muxc: *mut i2c_mux_core, chan: u32) -> c_int {
    static int mlxcpld_mux_deselect(struct i2c_mux_core *muxc, u32 chan)
    {
    struct mlxcpld_mux *mux = i2c_mux_priv(muxc);
// Deselect active channel
    mux.last_val = -1;
    return mlxcpld_mux_reg_write(muxc.parent, mux, 0);
    }
// Probe/reomove functions
#[no_mangle]
unsafe extern "C" fn mlxcpld_mux_probe(pdev: *mut platform_device) -> c_int {
    static int mlxcpld_mux_probe(struct platform_device *pdev)
    {
    struct mlxcpld_mux_plat_data *pdata = dev_get_platdata(&pdev.dev);
    struct i2c_client *client = to_i2c_client(pdev.dev.parent);
    struct i2c_mux_core *muxc;
    struct mlxcpld_mux *data;
    int num, err;
    u32 func;
    if (!pdata)
    return -EINVAL;
    switch (pdata.reg_size) {
    case 1:
    func = I2C_FUNC_SMBUS_WRITE_BYTE_DATA;
    break;
    case 2:
    func = I2C_FUNC_I2C;
    break;
    default:
    return -EINVAL;
    }
    if (!i2c_check_functionality(client.adapter, func))
    return -ENODEV;
    muxc = i2c_mux_alloc(client.adapter, &pdev.dev, pdata.num_adaps,
    sizeof(*data), 0, mlxcpld_mux_select_chan,
    mlxcpld_mux_deselect);
    if (!muxc)
    return -ENOMEM;
    platform_set_drvdata(pdev, muxc);
    data = i2c_mux_priv(muxc);
    data.client = client;
    memcpy(&data.pdata, pdata, sizeof(*pdata));
    data.last_val = -1; /* force the first selection */
// Create an adapter for each channel.
    for (num = 0; num < pdata.num_adaps; num++) {
    err = i2c_mux_add_adapter(muxc, 0, pdata.chan_ids[num]);
    if (err)
    goto virt_reg_failed;
    }
// Notify caller when all channels' adapters are created.
    if (pdata.completion_notify)
    pdata.completion_notify(pdata.handle, muxc.parent, muxc.adapter);
    return 0;
    virt_reg_failed:
    i2c_mux_del_adapters(muxc);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxcpld_mux_remove(pdev: *mut platform_device) {
    static void mlxcpld_mux_remove(struct platform_device *pdev)
    {
    struct i2c_mux_core *muxc = platform_get_drvdata(pdev);
    i2c_mux_del_adapters(muxc);
    }
    static struct platform_driver mlxcpld_mux_driver = {
    .driver = {
    .name = "i2c-mux-mlxcpld",
    },
    .probe = mlxcpld_mux_probe,
    .remove = mlxcpld_mux_remove,
    };
    module_platform_driver(mlxcpld_mux_driver);
    MODULE_AUTHOR("Michael Shych <michaels@mellanox.com>");
    MODULE_DESCRIPTION("Mellanox I2C-CPLD-MUX driver");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_ALIAS("platform:i2c-mux-mlxcpld");
