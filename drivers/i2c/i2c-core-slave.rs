//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/i2c-core-slave.c
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
// Linux I2C core slave support code
//
// Copyright (C) 2014 by Wolfram Sang <wsa@sang-engineering.com>
//

// Macro flag: #define CREATE_TRACE_POINTS

#[no_mangle]
pub unsafe extern "C" fn i2c_slave_register(client: *mut i2c_client, slave_cb: i2c_slave_cb_t) -> c_int {
    int i2c_slave_register(struct i2c_client *client, i2c_slave_cb_t slave_cb)
    {
    int ret;
    if (WARN(IS_ERR_OR_NULL(client) || !slave_cb, "insufficient data\n"))
    return -EINVAL;
    if (!(client.flags & I2C_CLIENT_SLAVE))
    dev_warn(&client.dev, "%s: client slave flag not set. You might see address collisions\n",
    __func__);
    if (!(client.flags & I2C_CLIENT_TEN)) {
// Enforce stricter address checking
    ret = i2c_check_7bit_addr_validity_strict(client.addr);
    if (ret) {
    dev_err(&client.dev, "%s: invalid address\n", __func__);
    return ret;
    }
    }
    if (!client.adapter.algo.reg_slave) {
    dev_err(&client.dev, "%s: not supported by adapter\n", __func__);
    return -EOPNOTSUPP;
    }
    client.slave_cb = slave_cb;
    i2c_lock_bus(client.adapter, I2C_LOCK_ROOT_ADAPTER);
    ret = client.adapter.algo.reg_slave(client);
    i2c_unlock_bus(client.adapter, I2C_LOCK_ROOT_ADAPTER);
    if (ret) {
    client.slave_cb = core::ptr::null_mut();
    dev_err(&client.dev, "%s: adapter returned error %d\n", __func__, ret);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(i2c_slave_register);
#[no_mangle]
pub unsafe extern "C" fn i2c_slave_unregister(client: *mut i2c_client) -> c_int {
    int i2c_slave_unregister(struct i2c_client *client)
    {
    int ret;
    if (IS_ERR_OR_NULL(client))
    return -EINVAL;
    if (!client.adapter.algo.unreg_slave) {
    dev_err(&client.dev, "%s: not supported by adapter\n", __func__);
    return -EOPNOTSUPP;
    }
    i2c_lock_bus(client.adapter, I2C_LOCK_ROOT_ADAPTER);
    ret = client.adapter.algo.unreg_slave(client);
    i2c_unlock_bus(client.adapter, I2C_LOCK_ROOT_ADAPTER);
    if (ret == 0)
    client.slave_cb = core::ptr::null_mut();
    else
    dev_err(&client.dev, "%s: adapter returned error %d\n", __func__, ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(i2c_slave_unregister);
    int i2c_slave_event(struct i2c_client *client,
    enum i2c_slave_event event, u8 *val)
    {
    let mut ret: c_int = client.slave_cb(client, event, val);
    if (trace_i2c_slave_enabled())
    trace_call__i2c_slave(client, event, val, ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(i2c_slave_event);
//
// i2c_detect_slave_mode - detect operation mode
// @dev: The device owning the bus
//
// This checks the device nodes for an I2C slave by checking the address
// used in the reg property. If the address match the I2C_OWN_SLAVE_ADDRESS
// flag this means the device is configured to act as a I2C slave and it will
// be listening at that address.
//
// Returns true if an I2C own slave address is detected, otherwise returns
// false.
//
#[no_mangle]
pub unsafe extern "C" fn i2c_detect_slave_mode(dev: *mut device) -> bool {
    bool i2c_detect_slave_mode(struct device *dev)
    {
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    if (is_of_node(fwnode)) {
    u32 reg;
    fwnode_for_each_child_node_scoped(fwnode, child) {
    fwnode_property_read_u32(child, "reg", &reg);
    if (reg & I2C_OWN_SLAVE_ADDRESS)
    return true;
    }
    } else if (is_acpi_device_node(fwnode)) {
    dev_dbg(dev, "ACPI slave is not supported yet\n");
    }
    return false;
    }
    EXPORT_SYMBOL_GPL(i2c_detect_slave_mode);
