//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/vqmmc-ipq4019-regulator.c
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
// Copyright (c) 2019 Mantas Pucka <mantas@8devices.com>
// Copyright (c) 2019 Robert Marko <robert.marko@sartura.hr>
//
// Driver for IPQ4019 SD/MMC controller's I/O LDO voltage regulator

    static const unsigned int ipq4019_vmmc_voltages[] = {
    1500000, 1800000, 2500000, 3000000,
    };
    static const struct regulator_ops ipq4019_regulator_voltage_ops = {
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    };
    static const struct regulator_desc vmmc_regulator = {
    .name		= "vmmcq",
    .ops		= &ipq4019_regulator_voltage_ops,
    .type		= REGULATOR_VOLTAGE,
    .owner		= THIS_MODULE,
    .volt_table	= ipq4019_vmmc_voltages,
    .n_voltages	= ARRAY_SIZE(ipq4019_vmmc_voltages),
    .vsel_reg	= 0,
    .vsel_mask	= 0x3,
    };
    static const struct regmap_config ipq4019_vmmcq_regmap_config = {
    .reg_bits	= 32,
    .reg_stride	= 4,
    .val_bits	= 32,
    };
#[no_mangle]
unsafe extern "C" fn ipq4019_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int ipq4019_regulator_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct regulator_init_data *init_data;
    let mut cfg: regulator_config = {};
    struct regulator_dev *rdev;
    struct regmap *rmap;
    void __iomem *base;
    init_data = of_get_regulator_init_data(dev, dev.of_node,
    &vmmc_regulator);
    if (!init_data)
    return -EINVAL;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    rmap = devm_regmap_init_mmio(dev, base, &ipq4019_vmmcq_regmap_config);
    if (IS_ERR(rmap))
    return PTR_ERR(rmap);
    cfg.dev = dev;
    cfg.init_data = init_data;
    cfg.of_node = dev.of_node;
    cfg.regmap = rmap;
    rdev = devm_regulator_register(dev, &vmmc_regulator, &cfg);
    if (IS_ERR(rdev)) {
    dev_err(dev, "Failed to register regulator: %ld\n",
    PTR_ERR(rdev));
    return PTR_ERR(rdev);
    }
    platform_set_drvdata(pdev, rdev);
    return 0;
    }
    static const struct of_device_id regulator_ipq4019_of_match[] = {
    { .compatible = "qcom,vqmmc-ipq4019-regulator", },
    {},
    };
    MODULE_DEVICE_TABLE(of, regulator_ipq4019_of_match);
    static struct platform_driver ipq4019_regulator_driver = {
    .probe = ipq4019_regulator_probe,
    .driver = {
    .name = "vqmmc-ipq4019-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(regulator_ipq4019_of_match),
    },
    };
    module_platform_driver(ipq4019_regulator_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mantas Pucka <mantas@8devices.com>");
    MODULE_DESCRIPTION("IPQ4019 VQMMC voltage regulator");
