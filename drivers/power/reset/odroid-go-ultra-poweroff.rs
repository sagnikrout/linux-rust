//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/odroid-go-ultra-poweroff.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2023 Neil Armstrong <neil.armstrong@linaro.org>
//

//
// The Odroid Go Ultra has 2 PMICs:
// - RK818 (manages the battery and USB-C power supply)
// - RK817
// Both PMICs feeds power to the S922X SoC, so they must be powered-off in sequence.
// Vendor does power-off the RK817 first, then the RK818 so here we follow this sequence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct odroid_go_ultra_poweroff_data {
    pub dev: *mut device,
    pub rk817: *mut device,
    pub rk818: *mut device,
}

#[no_mangle]
unsafe extern "C" fn odroid_go_ultra_poweroff_prepare(data: *mut sys_off_data) -> c_int {
    static int odroid_go_ultra_poweroff_prepare(struct sys_off_data *data)
    {
    struct odroid_go_ultra_poweroff_data *poweroff_data = data.cb_data;
    struct regmap *rk817, *rk818;
    int ret;
// RK817 Regmap
    rk817 = dev_get_regmap(poweroff_data.rk817, core::ptr::null_mut());
    if (!rk817) {
    dev_err(poweroff_data.dev, "failed to get rk817 regmap\n");
    return notifier_from_errno(-EINVAL);
    }
// RK818 Regmap
    rk818 = dev_get_regmap(poweroff_data.rk818, core::ptr::null_mut());
    if (!rk818) {
    dev_err(poweroff_data.dev, "failed to get rk818 regmap\n");
    return notifier_from_errno(-EINVAL);
    }
    dev_info(poweroff_data.dev, "Setting PMICs for power off");
// RK817
    ret = regmap_update_bits(rk817, RK817_SYS_CFG(3), DEV_OFF, DEV_OFF);
    if (ret) {
    dev_err(poweroff_data.dev, "failed to poweroff rk817\n");
    return notifier_from_errno(ret);
    }
// RK818
    ret = regmap_update_bits(rk818, RK818_DEVCTRL_REG, DEV_OFF, DEV_OFF);
    if (ret) {
    dev_err(poweroff_data.dev, "failed to poweroff rk818\n");
    return notifier_from_errno(ret);
    }
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn odroid_go_ultra_poweroff_put_pmic_device(data: *mut c_void) {
    static void odroid_go_ultra_poweroff_put_pmic_device(void *data)
    {
    struct device *dev = data;
    put_device(dev);
    }
    static int odroid_go_ultra_poweroff_get_pmic_device(struct device *dev, const char *compatible,
    struct device **pmic)
    {
    struct device_node *pmic_node;
    struct i2c_client *pmic_client;
    pmic_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), compatible);
    if (!pmic_node)
    return -ENODEV;
    pmic_client = of_find_i2c_device_by_node(pmic_node);
    of_node_put(pmic_node);
    if (!pmic_client)
    return -EPROBE_DEFER;
// pmic = &pmic_client->dev;
    return devm_add_action_or_reset(dev, odroid_go_ultra_poweroff_put_pmic_device, *pmic);
    }
#[no_mangle]
unsafe extern "C" fn odroid_go_ultra_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int odroid_go_ultra_poweroff_probe(struct platform_device *pdev)
    {
    struct odroid_go_ultra_poweroff_data *poweroff_data;
    int ret;
    poweroff_data = devm_kzalloc(&pdev.dev, sizeof(*poweroff_data), GFP_KERNEL);
    if (!poweroff_data)
    return -ENOMEM;
    dev_set_drvdata(&pdev.dev, poweroff_data);
// RK818 PMIC Device
    ret = odroid_go_ultra_poweroff_get_pmic_device(&pdev.dev, "rockchip,rk818",
    &poweroff_data.rk818);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to get rk818 mfd data\n");
// RK817 PMIC Device
    ret = odroid_go_ultra_poweroff_get_pmic_device(&pdev.dev, "rockchip,rk817",
    &poweroff_data.rk817);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to get rk817 mfd data\n");
// Register as SYS_OFF_MODE_POWER_OFF_PREPARE because regmap_update_bits may sleep
    ret = devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_POWER_OFF_PREPARE,
    SYS_OFF_PRIO_DEFAULT,
    odroid_go_ultra_poweroff_prepare,
    poweroff_data);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to register sys-off handler\n");
    dev_info(&pdev.dev, "Registered Power-Off handler\n");
    return 0;
    }
    static struct platform_device *pdev;
    static struct platform_driver odroid_go_ultra_poweroff_driver = {
    .driver = {
    .name	= "odroid-go-ultra-poweroff",
    },
    .probe = odroid_go_ultra_poweroff_probe,
    };
#[no_mangle]
unsafe extern "C" fn odroid_go_ultra_poweroff_init() -> int __init {
    static int __init odroid_go_ultra_poweroff_init(void)
    {
    int ret;
// Only create when running on the Odroid Go Ultra device
    if (!of_device_is_compatible(of_root, "hardkernel,odroid-go-ultra"))
    return -ENODEV;
    ret = platform_driver_register(&odroid_go_ultra_poweroff_driver);
    if (ret)
    return ret;
    pdev = platform_device_register_resndata(core::ptr::null_mut(), "odroid-go-ultra-poweroff", -1,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    platform_driver_unregister(&odroid_go_ultra_poweroff_driver);
    return PTR_ERR(pdev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn odroid_go_ultra_poweroff_exit() -> void __exit {
    static void __exit odroid_go_ultra_poweroff_exit(void)
    {
// Only delete when running on the Odroid Go Ultra device
    if (!of_device_is_compatible(of_root, "hardkernel,odroid-go-ultra"))
    return;
    platform_device_unregister(pdev);
    platform_driver_unregister(&odroid_go_ultra_poweroff_driver);
    }
    module_init(odroid_go_ultra_poweroff_init);
    module_exit(odroid_go_ultra_poweroff_exit);
    MODULE_AUTHOR("Neil Armstrong <neil.armstrong@linaro.org>");
    MODULE_DESCRIPTION("Odroid Go Ultra poweroff driver");
    MODULE_LICENSE("GPL");
