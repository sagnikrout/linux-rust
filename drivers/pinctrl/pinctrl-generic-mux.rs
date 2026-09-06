//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/pinctrl-generic-mux.c
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
// Generic Pin Control Driver for Board-Level Mux Chips
// Copyright 2026 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_pin_function {
    pub mux_state: *mut mux_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_pinctrl {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
// mutex protect [pinctrl|pinmux]_generic functions
    pub lock: mutex,
}

    static int
    mux_pinmux_dt_node_to_map(struct pinctrl_dev *pctldev,
    struct device_node *np_config,
    struct pinctrl_map **maps, unsigned int *num_maps)
    {
    let mut num_reserved_maps: c_uint = 0;
    struct mux_pin_function *function;
    const char **group_names;
    int ret;
    function = devm_kzalloc(pctldev.dev, sizeof(*function), GFP_KERNEL);
    if (!function)
    return -ENOMEM;
    group_names = devm_kcalloc(pctldev.dev, 1, sizeof(*group_names), GFP_KERNEL);
    if (!group_names)
    return -ENOMEM;
    function.mux_state = devm_mux_state_get_from_np(pctldev.dev, core::ptr::null_mut(), np_config);
    if (IS_ERR(function.mux_state))
    return PTR_ERR(function.mux_state);
    ret = pinctrl_generic_to_map(pctldev, np_config, np_config, maps,
    num_maps, &num_reserved_maps, group_names,
    0, &np_config.name, core::ptr::null_mut(), 0);
    if (ret)
    return ret;
    ret = pinmux_generic_add_function(pctldev, np_config.name, group_names,
    1, function);
    if (ret < 0) {
    pinctrl_utils_free_map(pctldev, *maps, *num_maps);
    return ret;
    }
    return 0;
    }
    static const struct pinctrl_ops mux_pinctrl_ops = {
    .get_groups_count = pinctrl_generic_get_group_count,
    .get_group_name = pinctrl_generic_get_group_name,
    .get_group_pins = pinctrl_generic_get_group_pins,
    .dt_node_to_map = mux_pinmux_dt_node_to_map,
    .dt_free_map = pinctrl_utils_free_map,
    };
    static int mux_pinmux_set_mux(struct pinctrl_dev *pctldev,
    unsigned int func_selector,
    unsigned int group_selector)
    {
    struct mux_pinctrl *mpctl = pinctrl_dev_get_drvdata(pctldev);
    const struct function_desc *function;
    struct mux_pin_function *func;
    int ret;
    guard(mutex)(&mpctl.lock);
    function = pinmux_generic_get_function(pctldev, func_selector);
    func = function.data;
    ret = mux_state_try_select(func.mux_state);
    if (ret)
    return ret;
    return 0;
    }
    static void mux_pinmux_release_mux(struct pinctrl_dev *pctldev,
    unsigned int func_selector,
    unsigned int group_selector)
    {
    struct mux_pinctrl *mpctl = pinctrl_dev_get_drvdata(pctldev);
    const struct function_desc *function;
    struct mux_pin_function *func;
    guard(mutex)(&mpctl.lock);
    function = pinmux_generic_get_function(pctldev, func_selector);
    func = function.data;
    mux_state_deselect(func.mux_state);
    }
    static const struct pinmux_ops mux_pinmux_ops = {
    .get_functions_count = pinmux_generic_get_function_count,
    .get_function_name = pinmux_generic_get_function_name,
    .get_function_groups = pinmux_generic_get_function_groups,
    .set_mux = mux_pinmux_set_mux,
    .release_mux = mux_pinmux_release_mux,
    };
#[no_mangle]
unsafe extern "C" fn mux_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int mux_pinctrl_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mux_pinctrl *mpctl;
    struct pinctrl_desc *pctl_desc;
    int ret;
    mpctl = devm_kzalloc(dev, sizeof(*mpctl), GFP_KERNEL);
    if (!mpctl)
    return -ENOMEM;
    mpctl.dev = dev;
    platform_set_drvdata(pdev, mpctl);
    pctl_desc = devm_kzalloc(dev, sizeof(*pctl_desc), GFP_KERNEL);
    if (!pctl_desc)
    return -ENOMEM;
    ret = devm_mutex_init(dev, &mpctl.lock);
    if (ret)
    return ret;
    pctl_desc.name = dev_name(dev);
    pctl_desc.owner = THIS_MODULE;
    pctl_desc.pctlops = &mux_pinctrl_ops;
    pctl_desc.pmxops = &mux_pinmux_ops;
    ret = devm_pinctrl_register_and_init(dev, pctl_desc, mpctl,
    &mpctl.pctl);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register pinctrl.\n");
    ret = pinctrl_enable(mpctl.pctl);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable pinctrl.\n");
    return 0;
    }
    static const struct of_device_id mux_pinctrl_of_match[] = {
    { .compatible = "pinctrl-multiplexer" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mux_pinctrl_of_match);
    static struct platform_driver mux_pinctrl_driver = {
    .driver = {
    .name = "generic-pinctrl-mux",
    .of_match_table = mux_pinctrl_of_match,
    },
    .probe = mux_pinctrl_probe,
    };
    module_platform_driver(mux_pinctrl_driver);
    MODULE_AUTHOR("Frank Li <Frank.Li@nxp.com>");
    MODULE_DESCRIPTION("Generic Pin Control Driver for Board-Level Mux Chips");
    MODULE_LICENSE("GPL");
