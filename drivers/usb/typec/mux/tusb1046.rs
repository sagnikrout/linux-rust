//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/mux/tusb1046.c
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
// Driver for the TUSB1046-DCI USB Type-C crosspoint switch
//
// Copyright (C) 2024 Bootlin
//

pub const TUSB1046_REG_GENERAL: c_uint = 0xa;
// General register bits

// Mux modes
pub const TUSB1046_CTLSEL_DISABLED: c_uint = 0x0;
pub const TUSB1046_CTLSEL_USB3: c_uint = 0x1;
pub const TUSB1046_CTLSEL_4LANE_DP: c_uint = 0x2;
pub const TUSB1046_CTLSEL_USB3_AND_2LANE_DP: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tusb1046_priv {
    pub client: *mut i2c_client,
    pub sw: *mut typec_switch_dev,
    pub mux: *mut typec_mux_dev,
// Lock General register during accesses
    pub general_reg_lock: mutex,
}

    static int tusb1046_mux_set(struct typec_mux_dev *mux,
    struct typec_mux_state *state)
    {
    struct tusb1046_priv *priv = typec_mux_get_drvdata(mux);
    struct i2c_client *client = priv.client;
    struct device *dev = &client.dev;
    int mode, val, ret = 0;
    if (state.mode >= TYPEC_STATE_MODAL &&
    state.alt.svid != USB_TYPEC_DP_SID)
    return -EINVAL;
    dev_dbg(dev, "mux mode requested: %lu\n", state.mode);
    mutex_lock(&priv.general_reg_lock);
    val = i2c_smbus_read_byte_data(client, TUSB1046_REG_GENERAL);
    if (val < 0) {
    dev_err(dev, "failed to read ctlsel status, err %d\n", val);
    ret = val;
    goto out_unlock;
    }
    switch (state.mode) {
    case TYPEC_STATE_USB:
    mode = TUSB1046_CTLSEL_USB3;
    break;
    case TYPEC_DP_STATE_C:
    case TYPEC_DP_STATE_E:
    mode = TUSB1046_CTLSEL_4LANE_DP;
    break;
    case TYPEC_DP_STATE_D:
    mode = TUSB1046_CTLSEL_USB3_AND_2LANE_DP;
    break;
    case TYPEC_STATE_SAFE:
    default:
    mode = TUSB1046_CTLSEL_DISABLED;
    break;
    }
    val &= ~TUSB1046_GENERAL_CTLSEL;
    val |= mode;
    ret = i2c_smbus_write_byte_data(client, TUSB1046_REG_GENERAL, val);
    out_unlock:
    mutex_unlock(&priv.general_reg_lock);
    return ret;
    }
    static int tusb1046_switch_set(struct typec_switch_dev *sw,
    enum typec_orientation orientation)
    {
    struct tusb1046_priv *priv = typec_switch_get_drvdata(sw);
    struct i2c_client *client = priv.client;
    struct device *dev = &client.dev;
    int val, ret = 0;
    dev_dbg(dev, "setting USB3.0 lane flip for orientation %d\n", orientation);
    mutex_lock(&priv.general_reg_lock);
    val = i2c_smbus_read_byte_data(client, TUSB1046_REG_GENERAL);
    if (val < 0) {
    dev_err(dev, "failed to read flipsel status, err %d\n", val);
    ret = val;
    goto out_unlock;
    }
    if (orientation == TYPEC_ORIENTATION_REVERSE)
    val |= TUSB1046_GENERAL_FLIPSEL;
    else
    val &= ~TUSB1046_GENERAL_FLIPSEL;
    ret = i2c_smbus_write_byte_data(client, TUSB1046_REG_GENERAL, val);
    out_unlock:
    mutex_unlock(&priv.general_reg_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tusb1046_i2c_probe(client: *mut i2c_client) -> c_int {
    static int tusb1046_i2c_probe(struct i2c_client *client)
    {
    let mut sw_desc: typec_switch_desc = { };
    let mut mux_desc: typec_mux_desc = { };
    struct device *dev = &client.dev;
    struct tusb1046_priv *priv;
    let mut ret: c_int = 0;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.client = client;
    mutex_init(&priv.general_reg_lock);
    sw_desc.drvdata = priv;
    sw_desc.fwnode = dev_fwnode(dev);
    sw_desc.set = tusb1046_switch_set;
    priv.sw = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(priv.sw)) {
    ret = dev_err_probe(dev, PTR_ERR(priv.sw), "failed to register type-c switch\n");
    goto err_destroy_mutex;
    }
    mux_desc.drvdata = priv;
    mux_desc.fwnode = dev_fwnode(dev);
    mux_desc.set = tusb1046_mux_set;
    priv.mux = typec_mux_register(dev, &mux_desc);
    if (IS_ERR(priv.mux)) {
    ret = dev_err_probe(dev, PTR_ERR(priv.mux), "failed to register type-c mux\n");
    goto err_unregister_switch;
    }
    i2c_set_clientdata(client, priv);
    return 0;
    err_unregister_switch:
    typec_switch_unregister(priv.sw);
    err_destroy_mutex:
    mutex_destroy(&priv.general_reg_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tusb1046_i2c_remove(client: *mut i2c_client) {
    static void tusb1046_i2c_remove(struct i2c_client *client)
    {
    struct tusb1046_priv *priv = i2c_get_clientdata(client);
    typec_switch_unregister(priv.sw);
    typec_mux_unregister(priv.mux);
    mutex_destroy(&priv.general_reg_lock);
    }
    static const struct of_device_id tusb1046_match_table[] = {
    {.compatible = "ti,tusb1046"},
    {},
    };
    MODULE_DEVICE_TABLE(of, tusb1046_match_table);
    static struct i2c_driver tusb1046_driver = {
    .driver = {
    .name = "tusb1046",
    .of_match_table = tusb1046_match_table,
    },
    .probe = tusb1046_i2c_probe,
    .remove = tusb1046_i2c_remove,
    };
    module_i2c_driver(tusb1046_driver);
    MODULE_DESCRIPTION("TUSB1046 USB Type-C switch driver");
    MODULE_AUTHOR("Romain Gantois <romain.gantois@bootlin.com>");
    MODULE_LICENSE("GPL");
