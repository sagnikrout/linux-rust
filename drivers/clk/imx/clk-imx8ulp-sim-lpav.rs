//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-imx8ulp-sim-lpav.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2025 NXP
//

pub const SYSCTRL0: c_uint = 0x8;

    {							\
    .name = gname "_cg",				\
    .id = IMX8ULP_CLK_SIM_LPAV_HIFI_##cname,	\
    .parent = { .fw_name = pname },			\
    .bit = bidx,					\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_imx8ulp_sim_lpav_data {
    pub /: *mut *mut spinlock_t lock; / shared by MUX, clock gate and reset,
    pub /: *mut *mut unsigned long flags; / for spinlock usage,
    pub /: *mut *mut clk_hw_onecell_data clk_data; / keep last,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_imx8ulp_sim_lpav_gate {
    pub name: *const c_char,
    pub id: c_int,
    pub parent: clk_parent_data,
    pub bit: u8,
}

    static struct clk_imx8ulp_sim_lpav_gate gates[] = {
    IMX8ULP_HIFI_CLK_GATE("hifi_core", CORE, "core", 17),
    IMX8ULP_HIFI_CLK_GATE("hifi_pbclk", PBCLK, "bus", 18),
    IMX8ULP_HIFI_CLK_GATE("hifi_plat", PLAT, "plat", 19)
    };
#[no_mangle]
unsafe extern "C" fn clk_imx8ulp_sim_lpav_lock(__acquires(&data->lock: *mut *mut void arg)) {
    static void clk_imx8ulp_sim_lpav_lock(void *arg) __acquires(&data.lock)
    {
    struct clk_imx8ulp_sim_lpav_data *data = dev_get_drvdata(arg);
    spin_lock_irqsave(&data.lock, data.flags);
    }
#[no_mangle]
unsafe extern "C" fn clk_imx8ulp_sim_lpav_unlock(__releases(&data->lock: *mut *mut void arg)) {
    static void clk_imx8ulp_sim_lpav_unlock(void *arg) __releases(&data.lock)
    {
    struct clk_imx8ulp_sim_lpav_data *data = dev_get_drvdata(arg);
    spin_unlock_irqrestore(&data.lock, data.flags);
    }
#[no_mangle]
unsafe extern "C" fn clk_imx8ulp_sim_lpav_probe(pdev: *mut platform_device) -> c_int {
    static int clk_imx8ulp_sim_lpav_probe(struct platform_device *pdev)
    {
    const struct regmap_config regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .lock = clk_imx8ulp_sim_lpav_lock,
    .unlock = clk_imx8ulp_sim_lpav_unlock,
    .lock_arg = &pdev.dev,
    };
    struct clk_imx8ulp_sim_lpav_data *data;
    struct auxiliary_device *adev;
    struct regmap *regmap;
    void __iomem *base;
    struct clk_hw *hw;
    int i, ret;
    data = devm_kzalloc(&pdev.dev,
    struct_size(data, clk_data.hws, ARRAY_SIZE(gates)),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    dev_set_drvdata(&pdev.dev, data);
//
// this lock is used directly by the clock gate and indirectly
// by the reset and mux controller via the regmap API
//
    spin_lock_init(&data.lock);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return dev_err_probe(&pdev.dev, PTR_ERR(base),
    "failed to ioremap base\n");
//
// although the clock gate doesn't use the regmap API to modify the
// registers, we still need the regmap because of the reset auxiliary
// driver and the MUX drivers, which use the parent device's regmap
//
    regmap = devm_regmap_init_mmio(&pdev.dev, base, &regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(regmap),
    "failed to initialize regmap\n");
    data.clk_data.num = ARRAY_SIZE(gates);
    for (i = 0; i < ARRAY_SIZE(gates); i++) {
    hw = devm_clk_hw_register_gate_parent_data(&pdev.dev,
    gates[i].name,
    &gates[i].parent,
    CLK_SET_RATE_PARENT,
    base + SYSCTRL0,
    gates[i].bit,
    0x0, &data.lock);
    if (IS_ERR(hw))
    return dev_err_probe(&pdev.dev, PTR_ERR(hw),
    "failed to register %s gate\n",
    gates[i].name);
    data.clk_data.hws[i] = hw;
    }
    adev = devm_auxiliary_device_create(&pdev.dev, "reset", core::ptr::null_mut());
    if (!adev)
    return dev_err_probe(&pdev.dev, -ENODEV,
    "failed to register aux reset\n");
    ret = devm_of_clk_add_hw_provider(&pdev.dev,
    of_clk_hw_onecell_get,
    &data.clk_data);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to register clk hw provider\n");
// used to probe MUX child device
    return devm_of_platform_populate(&pdev.dev);
    }
    static const struct of_device_id clk_imx8ulp_sim_lpav_of_match[] = {
    { .compatible = "fsl,imx8ulp-sim-lpav" },
    { }
    };
    MODULE_DEVICE_TABLE(of, clk_imx8ulp_sim_lpav_of_match);
    static struct platform_driver clk_imx8ulp_sim_lpav_driver = {
    .probe = clk_imx8ulp_sim_lpav_probe,
    .driver = {
    .name = "clk-imx8ulp-sim-lpav",
    .of_match_table = clk_imx8ulp_sim_lpav_of_match,
    },
    };
    module_platform_driver(clk_imx8ulp_sim_lpav_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("i.MX8ULP LPAV System Integration Module (SIM) clock driver");
    MODULE_AUTHOR("Laurentiu Mihalcea <laurentiu.mihalcea@nxp.com>");
