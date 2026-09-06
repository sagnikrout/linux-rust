//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/raa215300.c
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
// Renesas RAA215300 PMIC driver
//
// Copyright (C) 2023 Renesas Electronics Corporation
//

pub const RAA215300_FAULT_LATCHED_STATUS_1: c_uint = 0x59;
pub const RAA215300_FAULT_LATCHED_STATUS_2: c_uint = 0x5a;
pub const RAA215300_FAULT_LATCHED_STATUS_3: c_uint = 0x5b;
pub const RAA215300_FAULT_LATCHED_STATUS_4: c_uint = 0x5c;
pub const RAA215300_FAULT_LATCHED_STATUS_6: c_uint = 0x5e;
pub const RAA215300_INT_MASK_1: c_uint = 0x64;
pub const RAA215300_INT_MASK_2: c_uint = 0x65;
pub const RAA215300_INT_MASK_3: c_uint = 0x66;
pub const RAA215300_INT_MASK_4: c_uint = 0x67;
pub const RAA215300_INT_MASK_6: c_uint = 0x68;
pub const RAA215300_REG_BLOCK_EN: c_uint = 0x6c;
pub const RAA215300_HW_REV: c_uint = 0xf8;

pub const RAA215300_RTC_DEFAULT_ADDR: c_uint = 0x6f;
    static const struct regmap_config raa215300_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xff,
    };
#[no_mangle]
unsafe extern "C" fn raa215300_rtc_unregister_device(data: *mut c_void) {
    static void raa215300_rtc_unregister_device(void *data)
    {
    i2c_unregister_device(data);
    }
#[no_mangle]
unsafe extern "C" fn raa215300_clk_present(client: *mut i2c_client, name: *const c_char) -> c_int {
    static int raa215300_clk_present(struct i2c_client *client, const char *name)
    {
    struct clk *clk;
    clk = devm_clk_get_optional(&client.dev, name);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    return !!clk;
    }
#[no_mangle]
unsafe extern "C" fn raa215300_i2c_probe(client: *mut i2c_client) -> c_int {
    static int raa215300_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    const char *clkin_name = "clkin";
    unsigned int pmic_version, val;
    const char *xin_name = "xin";
    const char *clk_name = core::ptr::null_mut();
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_i2c(client, &raa215300_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "regmap i2c init failed\n");
    ret = regmap_read(regmap, RAA215300_HW_REV, &pmic_version);
    if (ret < 0)
    return dev_err_probe(dev, ret, "HW rev read failed\n");
    dev_dbg(dev, "RAA215300 PMIC version 0x%04x\n", pmic_version);
// Clear all blocks except RTC, if enabled
    regmap_read(regmap, RAA215300_REG_BLOCK_EN, &val);
    val &= RAA215300_REG_BLOCK_EN_RTC_EN;
    regmap_write(regmap, RAA215300_REG_BLOCK_EN, val);
// Clear the latched registers
    regmap_read(regmap, RAA215300_FAULT_LATCHED_STATUS_1, &val);
    regmap_write(regmap, RAA215300_FAULT_LATCHED_STATUS_1, val);
    regmap_read(regmap, RAA215300_FAULT_LATCHED_STATUS_2, &val);
    regmap_write(regmap, RAA215300_FAULT_LATCHED_STATUS_2, val);
    regmap_read(regmap, RAA215300_FAULT_LATCHED_STATUS_3, &val);
    regmap_write(regmap, RAA215300_FAULT_LATCHED_STATUS_3, val);
    regmap_read(regmap, RAA215300_FAULT_LATCHED_STATUS_4, &val);
    regmap_write(regmap, RAA215300_FAULT_LATCHED_STATUS_4, val);
    regmap_read(regmap, RAA215300_FAULT_LATCHED_STATUS_6, &val);
    regmap_write(regmap, RAA215300_FAULT_LATCHED_STATUS_6, val);
// Mask all the PMIC interrupts
    regmap_write(regmap, RAA215300_INT_MASK_1, RAA215300_INT_MASK_1_ALL);
    regmap_write(regmap, RAA215300_INT_MASK_2, RAA215300_INT_MASK_2_ALL);
    regmap_write(regmap, RAA215300_INT_MASK_3, RAA215300_INT_MASK_3_ALL);
    regmap_write(regmap, RAA215300_INT_MASK_4, RAA215300_INT_MASK_4_ALL);
    regmap_write(regmap, RAA215300_INT_MASK_6, RAA215300_INT_MASK_6_ALL);
    ret = raa215300_clk_present(client, xin_name);
    if (ret < 0) {
    return ret;
    } else if (ret) {
    clk_name = xin_name;
    } else {
    ret = raa215300_clk_present(client, clkin_name);
    if (ret < 0)
    return ret;
    if (ret)
    clk_name = clkin_name;
    }
    if (clk_name) {
    const char *name = pmic_version >= 0x12 ? "isl1208" : "raa215300_a0";
    struct device_node *np = client.dev.of_node;
    let mut addr: u32 = RAA215300_RTC_DEFAULT_ADDR;
    let mut info: i2c_board_info = {};
    struct i2c_client *rtc_client;
    struct clk_hw *hw;
    ssize_t size;
    hw = devm_clk_hw_register_fixed_rate(dev, clk_name, core::ptr::null_mut(), 0, 32768);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    ret = devm_clk_hw_register_clkdev(dev, hw, clk_name, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(dev, ret, "Failed to initialize clkdev\n");
    if (np) {
    int i;
    i = of_property_match_string(np, "reg-names", "rtc");
    if (i >= 0)
    of_property_read_u32_index(np, "reg", i, &addr);
    }
    info.addr = addr;
    if (client.irq > 0)
    info.irq = client.irq;
    size = strscpy(info.type, name, sizeof(info.type));
    if (size < 0)
    return dev_err_probe(dev, size,
    "Invalid device name: %s\n", name);
// Enable RTC block
    regmap_update_bits(regmap, RAA215300_REG_BLOCK_EN,
    RAA215300_REG_BLOCK_EN_RTC_EN,
    RAA215300_REG_BLOCK_EN_RTC_EN);
    rtc_client = i2c_new_client_device(client.adapter, &info);
    if (IS_ERR(rtc_client))
    return PTR_ERR(rtc_client);
    ret = devm_add_action_or_reset(dev,
    raa215300_rtc_unregister_device,
    rtc_client);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static const struct of_device_id raa215300_dt_match[] = {
    { .compatible = "renesas,raa215300" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, raa215300_dt_match);
    static struct i2c_driver raa215300_i2c_driver = {
    .driver = {
    .name = "raa215300",
    .of_match_table = raa215300_dt_match,
    },
    .probe = raa215300_i2c_probe,
    };
    module_i2c_driver(raa215300_i2c_driver);
    MODULE_DESCRIPTION("Renesas RAA215300 PMIC driver");
    MODULE_AUTHOR("Fabrizio Castro <fabrizio.castro.jz@renesas.com>");
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
    MODULE_LICENSE("GPL");
