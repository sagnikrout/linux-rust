//! Automatically rewritten from C to Rust
//! Source: sound/aoa/soundbus/i2sbus/control.c
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
// i2sbus driver -- bus control routines
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

#[no_mangle]
pub unsafe extern "C" fn i2sbus_control_init(dev: *mut *mut macio_dev, c: *mut i2sbus_control) -> c_int {
    int i2sbus_control_init(struct macio_dev* dev, struct i2sbus_control **c)
    {
// c = kzalloc_obj(struct i2sbus_control);
    if (!*c)
    return -ENOMEM;
    INIT_LIST_HEAD(&(*c).list);
    (*c).macio = dev.bus.chip;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn i2sbus_control_destroy(c: *mut i2sbus_control) {
    void i2sbus_control_destroy(struct i2sbus_control *c)
    {
    kfree(c);
    }
// this is serialised externally
    int i2sbus_control_add_dev(struct i2sbus_control *c,
    struct i2sbus_dev *i2sdev)
    {
    struct device_node *np;
    np = i2sdev.sound.ofdev.dev.of_node;
    i2sdev.enable = pmf_find_function(np, "enable");
    i2sdev.cell_enable = pmf_find_function(np, "cell-enable");
    i2sdev.clock_enable = pmf_find_function(np, "clock-enable");
    i2sdev.cell_disable = pmf_find_function(np, "cell-disable");
    i2sdev.clock_disable = pmf_find_function(np, "clock-disable");
// if the bus number is not 0 or 1 we absolutely need to use
// the platform functions -- there's nothing in Darwin that
// would allow seeing a system behind what the FCRs are then,
// and I don't want to go parsing a bunch of platform functions
// by hand to try finding a system...
    if (i2sdev.bus_number != 0 && i2sdev.bus_number != 1 &&
    (!i2sdev.enable ||
    !i2sdev.cell_enable || !i2sdev.clock_enable ||
    !i2sdev.cell_disable || !i2sdev.clock_disable)) {
    pmf_put_function(i2sdev.enable);
    pmf_put_function(i2sdev.cell_enable);
    pmf_put_function(i2sdev.clock_enable);
    pmf_put_function(i2sdev.cell_disable);
    pmf_put_function(i2sdev.clock_disable);
    return -ENODEV;
    }
    list_add(&i2sdev.item, &c.list);
    return 0;
    }
    void i2sbus_control_remove_dev(struct i2sbus_control *c,
    struct i2sbus_dev *i2sdev)
    {
// this is serialised externally
    list_del(&i2sdev.item);
    if (list_empty(&c.list))
    i2sbus_control_destroy(c);
    }
    int i2sbus_control_enable(struct i2sbus_control *c,
    struct i2sbus_dev *i2sdev)
    {
    let mut args: pmf_args = { .count = 0 };
    struct macio_chip *macio = c.macio;
    if (i2sdev.enable)
    return pmf_call_one(i2sdev.enable, &args);
    if (macio == core::ptr::null_mut() || macio.base == core::ptr::null_mut())
    return -ENODEV;
    switch (i2sdev.bus_number) {
    case 0:
// these need to be locked or done through
// newly created feature calls!
    MACIO_BIS(KEYLARGO_FCR1, KL1_I2S0_ENABLE);
    break;
    case 1:
    MACIO_BIS(KEYLARGO_FCR1, KL1_I2S1_ENABLE);
    break;
    default:
    return -ENODEV;
    }
    return 0;
    }
    int i2sbus_control_cell(struct i2sbus_control *c,
    struct i2sbus_dev *i2sdev,
    int enable)
    {
    let mut args: pmf_args = { .count = 0 };
    struct macio_chip *macio = c.macio;
    switch (enable) {
    case 0:
    if (i2sdev.cell_disable)
    return pmf_call_one(i2sdev.cell_disable, &args);
    break;
    case 1:
    if (i2sdev.cell_enable)
    return pmf_call_one(i2sdev.cell_enable, &args);
    break;
    default:
    printk(KERN_ERR "i2sbus: INVALID CELL ENABLE VALUE\n");
    return -ENODEV;
    }
    if (macio == core::ptr::null_mut() || macio.base == core::ptr::null_mut())
    return -ENODEV;
    switch (i2sdev.bus_number) {
    case 0:
    if (enable)
    MACIO_BIS(KEYLARGO_FCR1, KL1_I2S0_CELL_ENABLE);
    else
    MACIO_BIC(KEYLARGO_FCR1, KL1_I2S0_CELL_ENABLE);
    break;
    case 1:
    if (enable)
    MACIO_BIS(KEYLARGO_FCR1, KL1_I2S1_CELL_ENABLE);
    else
    MACIO_BIC(KEYLARGO_FCR1, KL1_I2S1_CELL_ENABLE);
    break;
    default:
    return -ENODEV;
    }
    return 0;
    }
    int i2sbus_control_clock(struct i2sbus_control *c,
    struct i2sbus_dev *i2sdev,
    int enable)
    {
    let mut args: pmf_args = { .count = 0 };
    struct macio_chip *macio = c.macio;
    switch (enable) {
    case 0:
    if (i2sdev.clock_disable)
    return pmf_call_one(i2sdev.clock_disable, &args);
    break;
    case 1:
    if (i2sdev.clock_enable)
    return pmf_call_one(i2sdev.clock_enable, &args);
    break;
    default:
    printk(KERN_ERR "i2sbus: INVALID CLOCK ENABLE VALUE\n");
    return -ENODEV;
    }
    if (macio == core::ptr::null_mut() || macio.base == core::ptr::null_mut())
    return -ENODEV;
    switch (i2sdev.bus_number) {
    case 0:
    if (enable)
    MACIO_BIS(KEYLARGO_FCR1, KL1_I2S0_CLK_ENABLE_BIT);
    else
    MACIO_BIC(KEYLARGO_FCR1, KL1_I2S0_CLK_ENABLE_BIT);
    break;
    case 1:
    if (enable)
    MACIO_BIS(KEYLARGO_FCR1, KL1_I2S1_CLK_ENABLE_BIT);
    else
    MACIO_BIC(KEYLARGO_FCR1, KL1_I2S1_CLK_ENABLE_BIT);
    break;
    default:
    return -ENODEV;
    }
    return 0;
    }
