//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/intel_soc_pmic_mrfld.c
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
// Device access for Basin Cove PMIC
//
// Copyright (c) 2019, Intel Corporation.
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

//
// Level 2 IRQs
//
// Firmware on the systems with Basin Cove PMIC services Level 1 IRQs
// without an assistance. Thus, each of the Level 1 IRQ is represented
// as a separate RTE in IOAPIC.
//
    static struct resource irq_level2_resources[] = {
    DEFINE_RES_IRQ(0), /* power button */
    DEFINE_RES_IRQ(0), /* TMU */
    DEFINE_RES_IRQ(0), /* thermal */
    DEFINE_RES_IRQ(0), /* BCU */
    DEFINE_RES_IRQ(0), /* ADC */
    DEFINE_RES_IRQ(0), /* charger */
    DEFINE_RES_IRQ(0), /* GPIO */
    };
    static const struct mfd_cell bcove_dev[] = {
    {
    .name = "mrfld_bcove_pwrbtn",
    .num_resources = 1,
    .resources = &irq_level2_resources[0],
    }, {
    .name = "mrfld_bcove_tmu",
    .num_resources = 1,
    .resources = &irq_level2_resources[1],
    }, {
    .name = "mrfld_bcove_thermal",
    .num_resources = 1,
    .resources = &irq_level2_resources[2],
    }, {
    .name = "mrfld_bcove_bcu",
    .num_resources = 1,
    .resources = &irq_level2_resources[3],
    }, {
    .name = "mrfld_bcove_adc",
    .num_resources = 1,
    .resources = &irq_level2_resources[4],
    }, {
    .name = "mrfld_bcove_charger",
    .num_resources = 1,
    .resources = &irq_level2_resources[5],
    }, {
    .name = "mrfld_bcove_pwrsrc",
    .num_resources = 1,
    .resources = &irq_level2_resources[5],
    }, {
    .name = "mrfld_bcove_gpio",
    .num_resources = 1,
    .resources = &irq_level2_resources[6],
    },
    {	.name = "mrfld_bcove_region", },
    };
    static int bcove_ipc_byte_reg_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct intel_soc_pmic *pmic = context;
    u8 ipc_out;
    int ret;
    ret = intel_scu_ipc_dev_ioread8(pmic.scu, reg, &ipc_out);
    if (ret)
    return ret;
// val = ipc_out;
    return 0;
    }
    static int bcove_ipc_byte_reg_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct intel_soc_pmic *pmic = context;
    let mut ipc_in: u8 = val;
    return intel_scu_ipc_dev_iowrite8(pmic.scu, reg, ipc_in);
    }
    static const struct regmap_config bcove_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .max_register = 0xff,
    .reg_write = bcove_ipc_byte_reg_write,
    .reg_read = bcove_ipc_byte_reg_read,
    };
#[no_mangle]
unsafe extern "C" fn bcove_probe(pdev: *mut platform_device) -> c_int {
    static int bcove_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct intel_soc_pmic *pmic;
    unsigned int i;
    int ret;
    pmic = devm_kzalloc(dev, sizeof(*pmic), GFP_KERNEL);
    if (!pmic)
    return -ENOMEM;
    pmic.scu = devm_intel_scu_ipc_dev_get(dev);
    if (!pmic.scu)
    return -ENOMEM;
    platform_set_drvdata(pdev, pmic);
    pmic.dev = &pdev.dev;
    pmic.regmap = devm_regmap_init(dev, core::ptr::null_mut(), pmic, &bcove_regmap_config);
    if (IS_ERR(pmic.regmap))
    return PTR_ERR(pmic.regmap);
    for (i = 0; i < ARRAY_SIZE(irq_level2_resources); i++) {
    ret = platform_get_irq(pdev, i);
    if (ret < 0)
    return ret;
    irq_level2_resources[i].start = ret;
    irq_level2_resources[i].end = ret;
    }
    return devm_mfd_add_devices(dev, PLATFORM_DEVID_NONE,
    bcove_dev, ARRAY_SIZE(bcove_dev),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static const struct acpi_device_id bcove_acpi_ids[] = {
    { "INTC100E" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, bcove_acpi_ids);
    static struct platform_driver bcove_driver = {
    .driver = {
    .name = "intel_soc_pmic_mrfld",
    .acpi_match_table = bcove_acpi_ids,
    },
    .probe = bcove_probe,
    };
    module_platform_driver(bcove_driver);
    MODULE_DESCRIPTION("IPC driver for Intel SoC Basin Cove PMIC");
    MODULE_LICENSE("GPL v2");
