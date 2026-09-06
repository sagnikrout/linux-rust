//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/muxes/i2c-mux-gpmux.c
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
// General Purpose I2C multiplexer
//
// Copyright (C) 2017 Axentia Technologies AB
//
// Author: Peter Rosin <peda@axentia.se>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux {
    pub control: *mut mux_control,
    pub do_not_deselect: bool,
}

#[no_mangle]
unsafe extern "C" fn i2c_mux_select(muxc: *mut i2c_mux_core, chan: u32) -> c_int {
    static int i2c_mux_select(struct i2c_mux_core *muxc, u32 chan)
    {
    struct mux *mux = i2c_mux_priv(muxc);
    int ret;
    ret = mux_control_select(mux.control, chan);
    mux.do_not_deselect = ret < 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_mux_deselect(muxc: *mut i2c_mux_core, chan: u32) -> c_int {
    static int i2c_mux_deselect(struct i2c_mux_core *muxc, u32 chan)
    {
    struct mux *mux = i2c_mux_priv(muxc);
    if (mux.do_not_deselect)
    return 0;
    return mux_control_deselect(mux.control);
    }
    static struct i2c_adapter *mux_parent_adapter(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct device_node *parent_np;
    struct i2c_adapter *parent;
    parent_np = of_parse_phandle(np, "i2c-parent", 0);
    if (!parent_np) {
    dev_err(dev, "Cannot parse i2c-parent\n");
    return ERR_PTR(-ENODEV);
    }
    parent = of_get_i2c_adapter_by_node(parent_np);
    of_node_put(parent_np);
    if (!parent)
    return ERR_PTR(-EPROBE_DEFER);
    return parent;
    }
    static const struct of_device_id i2c_mux_of_match[] = {
    { .compatible = "i2c-mux", },
    {},
    };
    MODULE_DEVICE_TABLE(of, i2c_mux_of_match);
#[no_mangle]
unsafe extern "C" fn i2c_mux_probe(pdev: *mut platform_device) -> c_int {
    static int i2c_mux_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *child;
    struct i2c_mux_core *muxc;
    struct mux *mux;
    struct i2c_adapter *parent;
    int children;
    int ret;
    if (!np)
    return -ENODEV;
    mux = devm_kzalloc(dev, sizeof(*mux), GFP_KERNEL);
    if (!mux)
    return -ENOMEM;
    mux.control = devm_mux_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(mux.control))
    return dev_err_probe(dev, PTR_ERR(mux.control),
    "failed to get control-mux\n");
    parent = mux_parent_adapter(dev);
    if (IS_ERR(parent))
    return dev_err_probe(dev, PTR_ERR(parent),
    "failed to get i2c-parent adapter\n");
    children = of_get_child_count(np);
    muxc = i2c_mux_alloc(parent, dev, children, 0, 0,
    i2c_mux_select, i2c_mux_deselect);
    if (!muxc) {
    ret = -ENOMEM;
    goto err_parent;
    }
    muxc.priv = mux;
    platform_set_drvdata(pdev, muxc);
    muxc.mux_locked = of_property_read_bool(np, "mux-locked");
    for_each_child_of_node(np, child) {
    u32 chan;
    ret = of_property_read_u32(child, "reg", &chan);
    if (ret < 0) {
    dev_err(dev, "no reg property for node '%pOFn'\n",
    child);
    goto err_children;
    }
    if (chan >= mux_control_states(mux.control)) {
    dev_err(dev, "invalid reg %u\n", chan);
    ret = -EINVAL;
    goto err_children;
    }
    ret = i2c_mux_add_adapter(muxc, 0, chan);
    if (ret)
    goto err_children;
    }
    dev_info(dev, "%d-port mux on %s adapter\n", children, parent.name);
    return 0;
    err_children:
    of_node_put(child);
    i2c_mux_del_adapters(muxc);
    err_parent:
    i2c_put_adapter(parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_mux_remove(pdev: *mut platform_device) {
    static void i2c_mux_remove(struct platform_device *pdev)
    {
    struct i2c_mux_core *muxc = platform_get_drvdata(pdev);
    i2c_mux_del_adapters(muxc);
    i2c_put_adapter(muxc.parent);
    }
    static struct platform_driver i2c_mux_driver = {
    .probe	= i2c_mux_probe,
    .remove = i2c_mux_remove,
    .driver	= {
    .name	= "i2c-mux-gpmux",
    .of_match_table = i2c_mux_of_match,
    },
    };
    module_platform_driver(i2c_mux_driver);
    MODULE_DESCRIPTION("General Purpose I2C multiplexer driver");
    MODULE_AUTHOR("Peter Rosin <peda@axentia.se>");
    MODULE_LICENSE("GPL v2");
