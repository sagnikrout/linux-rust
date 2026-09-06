//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-altera-platform.c
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
// Altera SPI driver
//
// Copyright (C) 2008 Thomas Chou <thomas@wytron.com.tw>
//
// Based on spi_s3c24xx.c, which is:
// Copyright (c) 2006 Ben Dooks
// Copyright (c) 2006 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//

    enum altera_spi_type {
    ALTERA_SPI_TYPE_UNKNOWN,
    ALTERA_SPI_TYPE_SUBDEV,
    };
    static const struct regmap_config spi_altera_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    };
#[no_mangle]
unsafe extern "C" fn altera_spi_probe(pdev: *mut platform_device) -> c_int {
    static int altera_spi_probe(struct platform_device *pdev)
    {
    const struct platform_device_id *platid = platform_get_device_id(pdev);
    struct altera_spi_platform_data *pdata = dev_get_platdata(&pdev.dev);
    let mut type: enum altera_spi_type = ALTERA_SPI_TYPE_UNKNOWN;
    struct altera_spi *hw;
    struct spi_controller *host;
    int err;
    u16 i;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(struct altera_spi));
    if (!host)
    return -ENOMEM;
// setup the host state.
    host.bus_num = -1;
    if (pdata) {
    if (pdata.num_chipselect > ALTERA_SPI_MAX_CS) {
    dev_err(&pdev.dev,
    "Invalid number of chipselect: %u\n",
    pdata.num_chipselect);
    return -EINVAL;
    }
    host.num_chipselect = pdata.num_chipselect;
    host.mode_bits = pdata.mode_bits;
    host.bits_per_word_mask = pdata.bits_per_word_mask;
    } else {
    host.num_chipselect = 16;
    host.mode_bits = SPI_CS_HIGH;
    host.bits_per_word_mask = SPI_BPW_RANGE_MASK(1, 16);
    }
    hw = spi_controller_get_devdata(host);
    hw.dev = &pdev.dev;
    if (platid)
    type = platid.driver_data;
// find and map our resources
    if (type == ALTERA_SPI_TYPE_SUBDEV) {
    struct resource *regoff;
    hw.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!hw.regmap)
    return dev_err_probe(&pdev.dev, -ENODEV, "get regmap failed\n");
    regoff = platform_get_resource(pdev, IORESOURCE_REG, 0);
    if (regoff)
    hw.regoff = regoff.start;
    } else {
    void __iomem *res;
    res = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(res))
    return PTR_ERR(res);
    hw.regmap = devm_regmap_init_mmio(&pdev.dev, res,
    &spi_altera_config);
    if (IS_ERR(hw.regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(hw.regmap),
    "regmap mmio init failed\n");
    }
    altera_spi_init_host(host);
// irq is optional
    hw.irq = platform_get_irq(pdev, 0);
    if (hw.irq >= 0) {
    err = devm_request_irq(&pdev.dev, hw.irq, altera_spi_irq, 0,
    pdev.name, host);
    if (err)
    return err;
    }
    err = devm_spi_register_controller(&pdev.dev, host);
    if (err)
    return err;
    if (pdata) {
    for (i = 0; i < pdata.num_devices; i++) {
    if (!spi_new_device(host, pdata.devices + i))
    dev_warn(&pdev.dev,
    "unable to create SPI device: %s\n",
    pdata.devices[i].modalias);
    }
    }
    dev_info(&pdev.dev, "regoff %u, irq %d\n", hw.regoff, hw.irq);
    return 0;
    }

    static const struct of_device_id altera_spi_match[] = {
    { .compatible = "ALTR,spi-1.0", },
    { .compatible = "altr,spi-1.0", },
    {},
    };
    MODULE_DEVICE_TABLE(of, altera_spi_match);

    static const struct platform_device_id altera_spi_ids[] = {
    { .name = DRV_NAME,		.driver_data = ALTERA_SPI_TYPE_UNKNOWN },
    { .name = "subdev_spi_altera",	.driver_data = ALTERA_SPI_TYPE_SUBDEV },
    { }
    };
    MODULE_DEVICE_TABLE(platform, altera_spi_ids);
    static struct platform_driver altera_spi_driver = {
    .probe = altera_spi_probe,
    .driver = {
    .name = DRV_NAME,
    .pm = core::ptr::null_mut(),
    .of_match_table = of_match_ptr(altera_spi_match),
    },
    .id_table	= altera_spi_ids,
    };
    module_platform_driver(altera_spi_driver);
    MODULE_DESCRIPTION("Altera SPI driver");
    MODULE_AUTHOR("Thomas Chou <thomas@wytron.com.tw>");
    MODULE_LICENSE("GPL");
