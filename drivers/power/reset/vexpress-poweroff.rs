//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/vexpress-poweroff.c
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
// Copyright (C) 2012 ARM Limited
//

#[no_mangle]
unsafe extern "C" fn vexpress_reset_do(dev: *mut device, what: *const c_char) {
    static void vexpress_reset_do(struct device *dev, const char *what)
    {
    let mut err: c_int = -ENOENT;
    struct regmap *reg = dev_get_drvdata(dev);
    if (reg) {
    err = regmap_write(reg, 0, 0);
    if (!err)
    mdelay(1000);
    }
    dev_emerg(dev, "Unable to %s (%d)\n", what, err);
    }
    static struct device *vexpress_power_off_device;
    let mut vexpress_restart_nb_refcnt: static atomic_t = ATOMIC_INIT(0);
#[no_mangle]
unsafe extern "C" fn vexpress_power_off() {
    static void vexpress_power_off(void)
    {
    vexpress_reset_do(vexpress_power_off_device, "power off");
    }
    static struct device *vexpress_restart_device;
    static int vexpress_restart(struct notifier_block *this, unsigned long mode,
    void *cmd)
    {
    vexpress_reset_do(vexpress_restart_device, "restart");
    return NOTIFY_DONE;
    }
    static struct notifier_block vexpress_restart_nb = {
    .notifier_call = vexpress_restart,
    .priority = 128,
    };
    static ssize_t vexpress_reset_active_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "%d\n", vexpress_restart_device == dev);
    }
    static ssize_t vexpress_reset_active_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    long value;
    let mut err: c_int = kstrtol(buf, 0, &value);
    if (!err && value)
    vexpress_restart_device = dev;
    return err ? err : count;
    }
    static DEVICE_ATTR(active, S_IRUGO | S_IWUSR, vexpress_reset_active_show,
    vexpress_reset_active_store);
    enum vexpress_reset_func { FUNC_RESET, FUNC_SHUTDOWN, FUNC_REBOOT };
    static const struct of_device_id vexpress_reset_of_match[] = {
    {
    .compatible = "arm,vexpress-reset",
    .data = (void *)FUNC_RESET,
    }, {
    .compatible = "arm,vexpress-shutdown",
    .data = (void *)FUNC_SHUTDOWN
    }, {
    .compatible = "arm,vexpress-reboot",
    .data = (void *)FUNC_REBOOT
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn _vexpress_register_restart_handler(dev: *mut device) -> c_int {
    static int _vexpress_register_restart_handler(struct device *dev)
    {
    int err;
    vexpress_restart_device = dev;
    if (atomic_inc_return(&vexpress_restart_nb_refcnt) == 1) {
    err = register_restart_handler(&vexpress_restart_nb);
    if (err) {
    dev_err(dev, "cannot register restart handler (err=%d)\n", err);
    atomic_dec(&vexpress_restart_nb_refcnt);
    return err;
    }
    }
    device_create_file(dev, &dev_attr_active);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vexpress_reset_probe(pdev: *mut platform_device) -> c_int {
    static int vexpress_reset_probe(struct platform_device *pdev)
    {
    enum vexpress_reset_func func;
    struct regmap *regmap;
    let mut ret: c_int = 0;
    regmap = devm_regmap_init_vexpress_config(&pdev.dev);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    dev_set_drvdata(&pdev.dev, regmap);
    func = (uintptr_t)device_get_match_data(&pdev.dev);
    switch (func) {
    case FUNC_SHUTDOWN:
    vexpress_power_off_device = &pdev.dev;
    pm_power_off = vexpress_power_off;
    break;
    case FUNC_RESET:
    if (!vexpress_restart_device)
    ret = _vexpress_register_restart_handler(&pdev.dev);
    break;
    case FUNC_REBOOT:
    ret = _vexpress_register_restart_handler(&pdev.dev);
    break;
    }
    return ret;
    }
    static struct platform_driver vexpress_reset_driver = {
    .probe = vexpress_reset_probe,
    .driver = {
    .name = "vexpress-reset",
    .of_match_table = vexpress_reset_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(vexpress_reset_driver);
