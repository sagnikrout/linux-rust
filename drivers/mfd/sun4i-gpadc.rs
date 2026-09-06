//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/sun4i-gpadc.c
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
// ADC MFD core driver for sunxi platforms
//
// Copyright (c) 2016 Quentin Schulz <quentin.schulz@free-electrons.com>
//

pub const ARCH_SUN4I_A10: c_int = 0;
pub const ARCH_SUN5I_A13: c_int = 1;
pub const ARCH_SUN6I_A31: c_int = 2;
    static const struct resource adc_resources[] = {
    DEFINE_RES_IRQ_NAMED(SUN4I_GPADC_IRQ_FIFO_DATA, "FIFO_DATA_PENDING"),
    DEFINE_RES_IRQ_NAMED(SUN4I_GPADC_IRQ_TEMP_DATA, "TEMP_DATA_PENDING"),
    };
    static const struct regmap_irq sun4i_gpadc_regmap_irq[] = {
    REGMAP_IRQ_REG(SUN4I_GPADC_IRQ_FIFO_DATA, 0,
    SUN4I_GPADC_INT_FIFOC_TP_DATA_IRQ_EN),
    REGMAP_IRQ_REG(SUN4I_GPADC_IRQ_TEMP_DATA, 0,
    SUN4I_GPADC_INT_FIFOC_TEMP_IRQ_EN),
    };
    static const struct regmap_irq_chip sun4i_gpadc_regmap_irq_chip = {
    .name = "sun4i_gpadc_irq_chip",
    .status_base = SUN4I_GPADC_INT_FIFOS,
    .ack_base = SUN4I_GPADC_INT_FIFOS,
    .unmask_base = SUN4I_GPADC_INT_FIFOC,
    .init_ack_masked = true,
    .irqs = sun4i_gpadc_regmap_irq,
    .num_irqs = ARRAY_SIZE(sun4i_gpadc_regmap_irq),
    .num_regs = 1,
    };
    static struct mfd_cell sun4i_gpadc_cells[] = {
    {
    .name	= "sun4i-a10-gpadc-iio",
    .resources = adc_resources,
    .num_resources = ARRAY_SIZE(adc_resources),
    },
    { .name = "iio_hwmon" }
    };
    static struct mfd_cell sun5i_gpadc_cells[] = {
    {
    .name	= "sun5i-a13-gpadc-iio",
    .resources = adc_resources,
    .num_resources = ARRAY_SIZE(adc_resources),
    },
    { .name = "iio_hwmon" },
    };
    static struct mfd_cell sun6i_gpadc_cells[] = {
    {
    .name	= "sun6i-a31-gpadc-iio",
    .resources = adc_resources,
    .num_resources = ARRAY_SIZE(adc_resources),
    },
    { .name = "iio_hwmon" },
    };
    static const struct regmap_config sun4i_gpadc_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
    static const struct of_device_id sun4i_gpadc_of_match[] = {
    {
    .compatible = "allwinner,sun4i-a10-ts",
    .data = (void *)ARCH_SUN4I_A10,
    }, {
    .compatible = "allwinner,sun5i-a13-ts",
    .data = (void *)ARCH_SUN5I_A13,
    }, {
    .compatible = "allwinner,sun6i-a31-ts",
    .data = (void *)ARCH_SUN6I_A31,
    }, { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sun4i_gpadc_of_match);
#[no_mangle]
unsafe extern "C" fn sun4i_gpadc_probe(pdev: *mut platform_device) -> c_int {
    static int sun4i_gpadc_probe(struct platform_device *pdev)
    {
    struct sun4i_gpadc_dev *dev;
    const struct of_device_id *of_id;
    const struct mfd_cell *cells;
    unsigned int irq, size;
    int ret;
    of_id = of_match_node(sun4i_gpadc_of_match, pdev.dev.of_node);
    if (!of_id)
    return -EINVAL;
    switch ((long)of_id.data) {
    case ARCH_SUN4I_A10:
    cells = sun4i_gpadc_cells;
    size = ARRAY_SIZE(sun4i_gpadc_cells);
    break;
    case ARCH_SUN5I_A13:
    cells = sun5i_gpadc_cells;
    size = ARRAY_SIZE(sun5i_gpadc_cells);
    break;
    case ARCH_SUN6I_A31:
    cells = sun6i_gpadc_cells;
    size = ARRAY_SIZE(sun6i_gpadc_cells);
    break;
    default:
    return -EINVAL;
    }
    dev = devm_kzalloc(&pdev.dev, sizeof(*dev), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    dev.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(dev.base))
    return PTR_ERR(dev.base);
    dev.dev = &pdev.dev;
    dev_set_drvdata(dev.dev, dev);
    dev.regmap = devm_regmap_init_mmio(dev.dev, dev.base,
    &sun4i_gpadc_regmap_config);
    if (IS_ERR(dev.regmap)) {
    ret = PTR_ERR(dev.regmap);
    dev_err(&pdev.dev, "failed to init regmap: %d\n", ret);
    return ret;
    }
// Disable all interrupts
    regmap_write(dev.regmap, SUN4I_GPADC_INT_FIFOC, 0);
    irq = platform_get_irq(pdev, 0);
    ret = devm_regmap_add_irq_chip(&pdev.dev, dev.regmap, irq,
    IRQF_ONESHOT, 0,
    &sun4i_gpadc_regmap_irq_chip,
    &dev.regmap_irqc);
    if (ret) {
    dev_err(&pdev.dev, "failed to add irq chip: %d\n", ret);
    return ret;
    }
    ret = devm_mfd_add_devices(dev.dev, 0, cells, size, core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret) {
    dev_err(&pdev.dev, "failed to add MFD devices: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static struct platform_driver sun4i_gpadc_driver = {
    .driver = {
    .name = "sun4i-gpadc",
    .of_match_table = sun4i_gpadc_of_match,
    },
    .probe = sun4i_gpadc_probe,
    };
    module_platform_driver(sun4i_gpadc_driver);
    MODULE_DESCRIPTION("Allwinner sunxi platforms' GPADC MFD core driver");
    MODULE_AUTHOR("Quentin Schulz <quentin.schulz@free-electrons.com>");
    MODULE_LICENSE("GPL v2");
