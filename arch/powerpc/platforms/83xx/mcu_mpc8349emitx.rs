//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/mcu_mpc8349emitx.c
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
// Power Management and GPIO expander driver for MPC8349E-mITX-compatible MCU
//
// Copyright (c) 2008  MontaVista Software, Inc.
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

//
// I don't have specifications for the MCU firmware, I found this register
// and bits positions by the trial&error method.
//
pub const MCU_REG_CTRL: c_uint = 0x20;
pub const MCU_CTRL_POFF: c_uint = 0x40;
pub const MCU_CTRL_BTN: c_uint = 0x80;
pub const MCU_NUM_GPIO: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu {
    pub lock: mutex,
    pub client: *mut i2c_client,
    pub gc: gpio_chip,
    pub reg_ctrl: u8,
}

    static struct mcu *glob_mcu;
    struct task_struct *shutdown_thread;
#[no_mangle]
unsafe extern "C" fn shutdown_thread_fn(data: *mut c_void) -> c_int {
    static int shutdown_thread_fn(void *data)
    {
    int ret;
    struct mcu *mcu = glob_mcu;
    while (!kthread_should_stop()) {
    ret = i2c_smbus_read_byte_data(mcu.client, MCU_REG_CTRL);
    if (ret < 0)
    pr_err("MCU status reg read failed.\n");
    mcu.reg_ctrl = ret;
    if (mcu.reg_ctrl & MCU_CTRL_BTN) {
    i2c_smbus_write_byte_data(mcu.client, MCU_REG_CTRL,
    mcu.reg_ctrl & ~MCU_CTRL_BTN);
    ctrl_alt_del();
    }
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(HZ);
    }
    return 0;
    }
    static ssize_t show_status(struct device *d,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    struct mcu *mcu = glob_mcu;
    ret = i2c_smbus_read_byte_data(mcu.client, MCU_REG_CTRL);
    if (ret < 0)
    return -ENODEV;
    mcu.reg_ctrl = ret;
    return sysfs_emit(buf, "%02x\n", ret);
    }
    static DEVICE_ATTR(status, 0444, show_status, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn mcu_power_off() {
    static void mcu_power_off(void)
    {
    struct mcu *mcu = glob_mcu;
    pr_info("Sending power-off request to the MCU...\n");
    mutex_lock(&mcu.lock);
    i2c_smbus_write_byte_data(mcu.client, MCU_REG_CTRL,
    mcu.reg_ctrl | MCU_CTRL_POFF);
    mutex_unlock(&mcu.lock);
    }
#[no_mangle]
unsafe extern "C" fn mcu_gpio_set(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    static int mcu_gpio_set(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct mcu *mcu = gpiochip_get_data(gc);
    let mut bit: u8 = 1 << (4 + gpio);
    int ret;
    mutex_lock(&mcu.lock);
    if (val)
    mcu.reg_ctrl &= ~bit;
    else
    mcu.reg_ctrl |= bit;
    ret = i2c_smbus_write_byte_data(mcu.client, MCU_REG_CTRL,
    mcu.reg_ctrl);
    mutex_unlock(&mcu.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcu_gpio_dir_out(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    static int mcu_gpio_dir_out(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    return mcu_gpio_set(gc, gpio, val);
    }
#[no_mangle]
unsafe extern "C" fn mcu_gpiochip_add(mcu: *mut mcu) -> c_int {
    static int mcu_gpiochip_add(struct mcu *mcu)
    {
    struct device *dev = &mcu.client.dev;
    struct gpio_chip *gc = &mcu.gc;
    gc.owner = THIS_MODULE;
    gc.label = kasprintf(GFP_KERNEL, "%pfw", dev_fwnode(dev));
    if (!gc.label)
    return -ENOMEM;
    gc.can_sleep = 1;
    gc.ngpio = MCU_NUM_GPIO;
    gc.base = -1;
    gc.set = mcu_gpio_set;
    gc.direction_output = mcu_gpio_dir_out;
    gc.parent = dev;
    return gpiochip_add_data(gc, mcu);
    }
#[no_mangle]
unsafe extern "C" fn mcu_gpiochip_remove(mcu: *mut mcu) {
    static void mcu_gpiochip_remove(struct mcu *mcu)
    {
    kfree(mcu.gc.label);
    gpiochip_remove(&mcu.gc);
    }
#[no_mangle]
unsafe extern "C" fn mcu_probe(client: *mut i2c_client) -> c_int {
    static int mcu_probe(struct i2c_client *client)
    {
    struct mcu *mcu;
    int ret;
    mcu = kzalloc_obj(*mcu);
    if (!mcu)
    return -ENOMEM;
    mutex_init(&mcu.lock);
    mcu.client = client;
    i2c_set_clientdata(client, mcu);
    ret = i2c_smbus_read_byte_data(mcu.client, MCU_REG_CTRL);
    if (ret < 0)
    goto err;
    mcu.reg_ctrl = ret;
    ret = mcu_gpiochip_add(mcu);
    if (ret)
    goto err;
// XXX: this is potentially racy, but there is no lock for pm_power_off
    if (!pm_power_off) {
    glob_mcu = mcu;
    pm_power_off = mcu_power_off;
    dev_info(&client.dev, "will provide power-off service\n");
    }
    if (device_create_file(&client.dev, &dev_attr_status))
    dev_err(&client.dev,
    "couldn't create device file for status\n");
    shutdown_thread = kthread_run(shutdown_thread_fn, core::ptr::null_mut(),
    "mcu-i2c-shdn");
    return 0;
    err:
    kfree(mcu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcu_remove(client: *mut i2c_client) {
    static void mcu_remove(struct i2c_client *client)
    {
    struct mcu *mcu = i2c_get_clientdata(client);
    kthread_stop(shutdown_thread);
    device_remove_file(&client.dev, &dev_attr_status);
    if (glob_mcu == mcu) {
    pm_power_off = core::ptr::null_mut();
    glob_mcu = core::ptr::null_mut();
    }
    mcu_gpiochip_remove(mcu);
    kfree(mcu);
    }
    static const struct i2c_device_id mcu_ids[] = {
    { "mcu-mpc8349emitx", },
    {},
    };
    MODULE_DEVICE_TABLE(i2c, mcu_ids);
    static const struct of_device_id mcu_of_match_table[] = {
    { .compatible = "fsl,mcu-mpc8349emitx", },
    { },
    };
    static struct i2c_driver mcu_driver = {
    .driver = {
    .name = "mcu-mpc8349emitx",
    .of_match_table = mcu_of_match_table,
    },
    .probe = mcu_probe,
    .remove	= mcu_remove,
    .id_table = mcu_ids,
    };
    module_i2c_driver(mcu_driver);
    MODULE_DESCRIPTION("Power Management and GPIO expander driver for "
    "MPC8349E-mITX-compatible MCU");
    MODULE_AUTHOR("Anton Vorontsov <avorontsov@ru.mvista.com>");
    MODULE_LICENSE("GPL");
