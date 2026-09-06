//! Automatically rewritten from C to Rust
//! Source: drivers/phy/phy-nxp-ptn3222.c
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
// Copyright (c) 2024, Linaro Limited
//

pub const NUM_SUPPLIES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptn3222 {
    pub client: *mut i2c_client,
    pub phy: *mut phy,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: *mut regulator_bulk_data,
}

#[no_mangle]
unsafe extern "C" fn ptn3222_init(phy: *mut phy) -> c_int {
    static int ptn3222_init(struct phy *phy)
    {
    struct ptn3222 *ptn3222 = phy_get_drvdata(phy);
    int ret;
    ret = regulator_bulk_enable(NUM_SUPPLIES, ptn3222.supplies);
    if (ret)
    return ret;
    gpiod_set_value_cansleep(ptn3222.reset_gpio, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptn3222_exit(phy: *mut phy) -> c_int {
    static int ptn3222_exit(struct phy *phy)
    {
    struct ptn3222 *ptn3222 = phy_get_drvdata(phy);
    gpiod_set_value_cansleep(ptn3222.reset_gpio, 1);
    return regulator_bulk_disable(NUM_SUPPLIES, ptn3222.supplies);
    }
    static const struct phy_ops ptn3222_ops = {
    .init		= ptn3222_init,
    .exit		= ptn3222_exit,
    .owner		= THIS_MODULE,
    };
    static const struct regulator_bulk_data ptn3222_supplies[NUM_SUPPLIES] = {
    {
    .supply = "vdd3v3",
    .init_load_uA = 11000,
    }, {
    .supply = "vdd1v8",
    .init_load_uA = 55000,
    }
    };
#[no_mangle]
unsafe extern "C" fn ptn3222_probe(client: *mut i2c_client) -> c_int {
    static int ptn3222_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct phy_provider *phy_provider;
    struct ptn3222 *ptn3222;
    int ret;
    ptn3222 = devm_kzalloc(dev, sizeof(*ptn3222), GFP_KERNEL);
    if (!ptn3222)
    return -ENOMEM;
    ptn3222.client = client;
    ptn3222.reset_gpio = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ptn3222.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ptn3222.reset_gpio),
    "unable to acquire reset gpio\n");
    ret = devm_regulator_bulk_get_const(dev, NUM_SUPPLIES, ptn3222_supplies,
    &ptn3222.supplies);
    if (ret)
    return ret;
    ptn3222.phy = devm_phy_create(dev, dev.of_node, &ptn3222_ops);
    if (IS_ERR(ptn3222.phy)) {
    dev_err(dev, "failed to create PHY: %d\n", ret);
    return PTR_ERR(ptn3222.phy);
    }
    phy_set_drvdata(ptn3222.phy, ptn3222);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct i2c_device_id ptn3222_table[] = {
    { .name = "ptn3222" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ptn3222_table);
    static const struct of_device_id ptn3222_of_table[] = {
    { .compatible = "nxp,ptn3222" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ptn3222_of_table);
    static struct i2c_driver ptn3222_driver = {
    .driver = {
    .name = "ptn3222",
    .of_match_table = ptn3222_of_table,
    },
    .probe = ptn3222_probe,
    .id_table = ptn3222_table,
    };
    module_i2c_driver(ptn3222_driver);
    MODULE_DESCRIPTION("NXP PTN3222 eUSB2 Redriver driver");
    MODULE_LICENSE("GPL");
