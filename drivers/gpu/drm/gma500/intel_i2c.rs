//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/gma500/intel_i2c.c
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
// Copyright © 2006-2007 Intel Corporation
//
// Authors:
// Eric Anholt <eric@anholt.net>
//

//
// Intel GPIO access functions
//
pub const I2C_RISEFALL_TIME: c_int = 20;
#[no_mangle]
unsafe extern "C" fn get_clock(data: *mut c_void) -> c_int {
    static int get_clock(void *data)
    {
    struct gma_i2c_chan *chan = data;
    struct drm_device *dev = chan.drm_dev;
    u32 val;
    val = REG_READ(chan.reg);
    return (val & GPIO_CLOCK_VAL_IN) != 0;
    }
#[no_mangle]
unsafe extern "C" fn get_data(data: *mut c_void) -> c_int {
    static int get_data(void *data)
    {
    struct gma_i2c_chan *chan = data;
    struct drm_device *dev = chan.drm_dev;
    u32 val;
    val = REG_READ(chan.reg);
    return (val & GPIO_DATA_VAL_IN) != 0;
    }
#[no_mangle]
unsafe extern "C" fn set_clock(data: *mut c_void, state_high: c_int) {
    static void set_clock(void *data, int state_high)
    {
    struct gma_i2c_chan *chan = data;
    struct drm_device *dev = chan.drm_dev;
    let mut reserved: u32 = 0, clock_bits;
// On most chips, these bits must be preserved in software.
    reserved =
    REG_READ(chan.reg) & (GPIO_DATA_PULLUP_DISABLE |
    GPIO_CLOCK_PULLUP_DISABLE);
    if (state_high)
    clock_bits = GPIO_CLOCK_DIR_IN | GPIO_CLOCK_DIR_MASK;
    else
    clock_bits = GPIO_CLOCK_DIR_OUT | GPIO_CLOCK_DIR_MASK |
    GPIO_CLOCK_VAL_MASK;
    REG_WRITE(chan.reg, reserved | clock_bits);
    udelay(I2C_RISEFALL_TIME);	/* wait for the line to change state */
    }
#[no_mangle]
unsafe extern "C" fn set_data(data: *mut c_void, state_high: c_int) {
    static void set_data(void *data, int state_high)
    {
    struct gma_i2c_chan *chan = data;
    struct drm_device *dev = chan.drm_dev;
    let mut reserved: u32 = 0, data_bits;
// On most chips, these bits must be preserved in software.
    reserved =
    REG_READ(chan.reg) & (GPIO_DATA_PULLUP_DISABLE |
    GPIO_CLOCK_PULLUP_DISABLE);
    if (state_high)
    data_bits = GPIO_DATA_DIR_IN | GPIO_DATA_DIR_MASK;
    else
    data_bits =
    GPIO_DATA_DIR_OUT | GPIO_DATA_DIR_MASK |
    GPIO_DATA_VAL_MASK;
    REG_WRITE(chan.reg, reserved | data_bits);
    udelay(I2C_RISEFALL_TIME);	/* wait for the line to change state */
    }
//
// gma_i2c_create - instantiate an Intel i2c bus using the specified GPIO reg
// @dev: DRM device
// @reg: GPIO reg to use
// @name: name for this bus
//
// Creates and registers a new i2c bus with the Linux i2c layer, for use
// in output probing and control (e.g. DDC or SDVO control functions).
//
// Possible values for @reg include:
// %GPIOA
// %GPIOB
// %GPIOC
// %GPIOD
// %GPIOE
// %GPIOF
// %GPIOG
// %GPIOH
// see PRM for details on how these different busses are used.
//
    struct gma_i2c_chan *gma_i2c_create(struct drm_device *dev, const u32 reg,
    const char *name)
    {
    struct gma_i2c_chan *chan;
    chan = kzalloc_obj(struct gma_i2c_chan);
    if (!chan)
    goto out_free;
    chan.drm_dev = dev;
    chan.reg = reg;
    snprintf(chan.base.name, I2C_NAME_SIZE, "intel drm %s", name);
    chan.base.owner = THIS_MODULE;
    chan.base.algo_data = &chan.algo;
    chan.base.dev.parent = dev.dev;
    chan.algo.setsda = set_data;
    chan.algo.setscl = set_clock;
    chan.algo.getsda = get_data;
    chan.algo.getscl = get_clock;
    chan.algo.udelay = 20;
    chan.algo.timeout = usecs_to_jiffies(2200);
    chan.algo.data = chan;
    i2c_set_adapdata(&chan.base, chan);
    if (i2c_bit_add_bus(&chan.base))
    goto out_free;
// JJJ:  raise SCL and SDA?
    set_data(chan, 1);
    set_clock(chan, 1);
    udelay(20);
    return chan;
    out_free:
    kfree(chan);
    return core::ptr::null_mut();
    }
//
// gma_i2c_destroy - unregister and free i2c bus resources
// @chan: channel to free
//
// Unregister the adapter from the i2c layer, then free the structure.
//
#[no_mangle]
pub unsafe extern "C" fn gma_i2c_destroy(chan: *mut gma_i2c_chan) {
    void gma_i2c_destroy(struct gma_i2c_chan *chan)
    {
    if (!chan)
    return;
    i2c_del_adapter(&chan.base);
    kfree(chan);
    }
