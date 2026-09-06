//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-elkhartlake.c
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
// Intel Elkhart Lake PSE GPIO driver
//
// Copyright (c) 2023, 2025 Intel Corporation.
//
// Authors: Pandith N <pandith.n@intel.com>
// Raag Jadav <raag.jadav@intel.com>
//

// Each Intel EHL PSE GPIO Controller has 30 GPIO pins
pub const EHL_PSE_NGPIO: c_int = 30;
#[no_mangle]
unsafe extern "C" fn ehl_gpio_probe(adev: *mut auxiliary_device, id: *const auxiliary_device_id) -> c_int {
    static int ehl_gpio_probe(struct auxiliary_device *adev, const struct auxiliary_device_id *id)
    {
    struct device *dev = &adev.dev;
    struct ehl_pse_io_data *data;
    struct tng_gpio *priv;
    int ret;
    data = dev_get_platdata(dev);
    if (!data)
    return -ENODATA;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.reg_base = devm_ioremap_resource(dev, &data.mem);
    if (IS_ERR(priv.reg_base))
    return PTR_ERR(priv.reg_base);
    priv.dev = dev;
    priv.irq = data.irq;
    priv.info.base = -1;
    priv.info.ngpio = EHL_PSE_NGPIO;
    priv.wake_regs.gwmr = GWMR_EHL;
    priv.wake_regs.gwsr = GWSR_EHL;
    priv.wake_regs.gsir = GSIR_EHL;
    ret = devm_tng_gpio_probe(dev, priv);
    if (ret)
    return dev_err_probe(dev, ret, "tng_gpio_probe error\n");
    auxiliary_set_drvdata(adev, priv);
    return 0;
    }
    static const struct auxiliary_device_id ehl_gpio_ids[] = {
    { EHL_PSE_IO_NAME "." EHL_PSE_GPIO_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(auxiliary, ehl_gpio_ids);
    static struct auxiliary_driver ehl_gpio_driver = {
    .driver	= {
    .pm	= pm_sleep_ptr(&tng_gpio_pm_ops),
    },
    .probe		= ehl_gpio_probe,
    .id_table	= ehl_gpio_ids,
    };
    module_auxiliary_driver(ehl_gpio_driver);
    MODULE_AUTHOR("Pandith N <pandith.n@intel.com>");
    MODULE_AUTHOR("Raag Jadav <raag.jadav@intel.com>");
    MODULE_DESCRIPTION("Intel Elkhart Lake PSE GPIO driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("GPIO_TANGIER");
