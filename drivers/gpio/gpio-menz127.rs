//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-menz127.c
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
// MEN 16Z127 GPIO driver
//
// Copyright (C) 2016 MEN Mikroelektronik GmbH (www.men.de)
//

pub const MEN_Z127_CTRL: c_uint = 0x00;
pub const MEN_Z127_PSR: c_uint = 0x04;
pub const MEN_Z127_IRQR: c_uint = 0x08;
pub const MEN_Z127_GPIODR: c_uint = 0x0c;
pub const MEN_Z127_IER1: c_uint = 0x10;
pub const MEN_Z127_IER2: c_uint = 0x14;
pub const MEN_Z127_DBER: c_uint = 0x18;
pub const MEN_Z127_ODER: c_uint = 0x1C;

// MEN Z127 supported model ids
pub const MEN_Z127_ID: c_uint = 0x7f;
pub const MEN_Z034_ID: c_uint = 0x22;
pub const MEN_Z037_ID: c_uint = 0x25;
pub const MEN_Z127_DB_MIN_US: c_int = 50;
// 16 bit compare register. Each bit represents 50us

    (db <= MEN_Z127_DB_MAX_US))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct men_z127_gpio {
    pub chip: gpio_generic_chip,
    pub reg_base: *mut void __iomem,
    pub mem: *mut resource,
}

    static int men_z127_debounce(struct gpio_chip *gc, unsigned gpio,
    unsigned debounce)
    {
    struct men_z127_gpio *priv = gpiochip_get_data(gc);
    struct device *dev = gc.parent;
    unsigned int rnd;
    u32 db_en, db_cnt;
    if (!MEN_Z127_DB_IN_RANGE(debounce)) {
    dev_err(dev, "debounce value %u out of range", debounce);
    return -EINVAL;
    }
    if (debounce > 0) {
// round up or down depending on MSB-1
    rnd = fls(debounce) - 1;
    if (rnd && (debounce & BIT(rnd - 1)))
    debounce = roundup(debounce, MEN_Z127_DB_MIN_US);
    else
    debounce = rounddown(debounce, MEN_Z127_DB_MIN_US);
    if (debounce > MEN_Z127_DB_MAX_US)
    debounce = MEN_Z127_DB_MAX_US;
// 50us per register unit
    debounce /= 50;
    }
    guard(gpio_generic_lock)(&priv.chip);
    db_en = readl(priv.reg_base + MEN_Z127_DBER);
    if (debounce == 0) {
    db_en &= ~BIT(gpio);
    db_cnt = 0;
    } else {
    db_en |= BIT(gpio);
    db_cnt = debounce;
    }
    writel(db_en, priv.reg_base + MEN_Z127_DBER);
    writel(db_cnt, priv.reg_base + GPIO_TO_DBCNT_REG(gpio));
    return 0;
    }
    static int men_z127_set_single_ended(struct gpio_chip *gc,
    unsigned offset,
    enum pin_config_param param)
    {
    struct men_z127_gpio *priv = gpiochip_get_data(gc);
    u32 od_en;
    guard(gpio_generic_lock)(&priv.chip);
    od_en = readl(priv.reg_base + MEN_Z127_ODER);
    if (param == PIN_CONFIG_DRIVE_OPEN_DRAIN)
    od_en |= BIT(offset);
    else
// Implicitly PIN_CONFIG_DRIVE_PUSH_PULL
    od_en &= ~BIT(offset);
    writel(od_en, priv.reg_base + MEN_Z127_ODER);
    return 0;
    }
    static int men_z127_set_config(struct gpio_chip *gc, unsigned offset,
    unsigned long config)
    {
    let mut param: enum pin_config_param = pinconf_to_config_param(config);
    switch (param) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return men_z127_set_single_ended(gc, offset, param);
    case PIN_CONFIG_INPUT_DEBOUNCE:
    return men_z127_debounce(gc, offset,
    pinconf_to_config_argument(config));
    default:
    break;
    }
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn men_z127_release_mem(data: *mut c_void) {
    static void men_z127_release_mem(void *data)
    {
    struct resource *res = data;
    mcb_release_mem(res);
    }
    static int men_z127_probe(struct mcb_device *mdev,
    const struct mcb_device_id *id)
    {
    struct gpio_generic_chip_config config;
    struct men_z127_gpio *men_z127_gpio;
    struct device *dev = &mdev.dev;
    int ret;
    unsigned long sz;
    men_z127_gpio = devm_kzalloc(dev, sizeof(struct men_z127_gpio),
    GFP_KERNEL);
    if (!men_z127_gpio)
    return -ENOMEM;
    men_z127_gpio.mem = mcb_request_mem(mdev, dev_name(dev));
    if (IS_ERR(men_z127_gpio.mem))
    return dev_err_probe(dev, PTR_ERR(men_z127_gpio.mem),
    "failed to request device memory");
    ret = devm_add_action_or_reset(dev, men_z127_release_mem,
    men_z127_gpio.mem);
    if (ret)
    return ret;
    men_z127_gpio.reg_base = devm_ioremap(dev, men_z127_gpio.mem.start,
    resource_size(men_z127_gpio.mem));
    if (men_z127_gpio.reg_base == core::ptr::null_mut())
    return -ENXIO;
    mcb_set_drvdata(mdev, men_z127_gpio);
    switch (mdev.id) {
    case MEN_Z127_ID:
    sz = 4;
    break;
    case MEN_Z034_ID:
    case MEN_Z037_ID:
    sz = 1;
    break;
    default:
    return dev_err_probe(&mdev.dev, -EINVAL, "no size found for id %d", mdev.id);
    }
    config = (struct gpio_generic_chip_config) {
    .dev = &mdev.dev,
    .sz = sz,
    .dat = men_z127_gpio.reg_base + MEN_Z127_PSR,
    .set = men_z127_gpio.reg_base + MEN_Z127_CTRL,
    .dirout = men_z127_gpio.reg_base + MEN_Z127_GPIODR,
    };
    ret = gpio_generic_chip_init(&men_z127_gpio.chip, &config);
    if (ret)
    return ret;
    men_z127_gpio.chip.gc.set_config = men_z127_set_config;
    ret = devm_gpiochip_add_data(dev, &men_z127_gpio.chip.gc, men_z127_gpio);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to register MEN 16Z127 GPIO controller");
    return 0;
    }
    static const struct mcb_device_id men_z127_ids[] = {
    { .device = MEN_Z127_ID },
    { .device = MEN_Z034_ID },
    { .device = MEN_Z037_ID },
    { }
    };
    MODULE_DEVICE_TABLE(mcb, men_z127_ids);
    static struct mcb_driver men_z127_driver = {
    .driver = {
    .name = "z127-gpio",
    },
    .probe = men_z127_probe,
    .id_table = men_z127_ids,
    };
    module_mcb_driver(men_z127_driver);
    MODULE_AUTHOR("Andreas Werner <andreas.werner@men.de>");
    MODULE_DESCRIPTION("MEN GPIO Controller");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MCB");
