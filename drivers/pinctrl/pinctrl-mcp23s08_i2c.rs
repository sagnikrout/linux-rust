//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/pinctrl-mcp23s08_i2c.c
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
// MCP23S08 I2C GPIO driver

#[no_mangle]
unsafe extern "C" fn mcp230xx_probe(client: *mut i2c_client) -> c_int {
    static int mcp230xx_probe(struct i2c_client *client)
    {
    const struct mcp23s08_info *info;
    struct device *dev = &client.dev;
    struct mcp23s08 *mcp;
    int ret;
    mcp = devm_kzalloc(dev, sizeof(*mcp), GFP_KERNEL);
    if (!mcp)
    return -ENOMEM;
    info = i2c_get_match_data(client);
    if (!info)
    return dev_err_probe(dev, -EINVAL, "invalid device type\n");
    mcp.reg_shift = info.reg_shift;
    mcp.chip.ngpio = info.ngpio;
    mcp.chip.label = info.label;
    mcp.regmap = devm_regmap_init_i2c(client, info.regmap);
    if (IS_ERR(mcp.regmap))
    return PTR_ERR(mcp.regmap);
    mcp.irq = client.irq;
    mcp.pinctrl_desc.name = "mcp23xxx-pinctrl";
    ret = mcp23s08_probe_one(mcp, dev, client.addr, info.type, -1);
    if (ret)
    return ret;
    i2c_set_clientdata(client, mcp);
    return 0;
    }
    static const struct mcp23s08_info mcp23008_i2c = {
    .regmap = &mcp23x08_regmap,
    .label = "mcp23008",
    .type = MCP_TYPE_008,
    .ngpio = 8,
    .reg_shift = 0,
    };
    static const struct mcp23s08_info mcp23017_i2c = {
    .regmap = &mcp23x17_regmap,
    .label = "mcp23017",
    .type = MCP_TYPE_017,
    .ngpio = 16,
    .reg_shift = 1,
    };
    static const struct mcp23s08_info  mcp23018_i2c = {
    .regmap = &mcp23x17_regmap,
    .label = "mcp23018",
    .type = MCP_TYPE_018,
    .ngpio = 16,
    .reg_shift = 1,
    };
    static const struct i2c_device_id mcp230xx_id[] = {
    { .name = "mcp23008", .driver_data = (kernel_ulong_t)&mcp23008_i2c },
    { .name = "mcp23017", .driver_data = (kernel_ulong_t)&mcp23017_i2c },
    { .name = "mcp23018", .driver_data = (kernel_ulong_t)&mcp23018_i2c },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mcp230xx_id);
    static const struct of_device_id mcp23s08_i2c_of_match[] = {
    { .compatible = "microchip,mcp23008", .data = &mcp23008_i2c },
    { .compatible = "microchip,mcp23017", .data = &mcp23017_i2c },
    { .compatible = "microchip,mcp23018", .data = &mcp23018_i2c },
// NOTE: The use of the mcp prefix is deprecated and will be removed.
    { .compatible = "mcp,mcp23008", .data = &mcp23008_i2c },
    { .compatible = "mcp,mcp23017", .data = &mcp23017_i2c },
    { }
    };
    MODULE_DEVICE_TABLE(of, mcp23s08_i2c_of_match);
    static struct i2c_driver mcp230xx_driver = {
    .driver = {
    .name	= "mcp230xx",
    .of_match_table = mcp23s08_i2c_of_match,
    },
    .probe		= mcp230xx_probe,
    .id_table	= mcp230xx_id,
    };
#[no_mangle]
unsafe extern "C" fn mcp23s08_i2c_init() -> int __init {
    static int __init mcp23s08_i2c_init(void)
    {
    return i2c_add_driver(&mcp230xx_driver);
    }
//
// Register after I²C postcore initcall and before
// subsys initcalls that may rely on these GPIOs.
//
    subsys_initcall(mcp23s08_i2c_init);
#[no_mangle]
unsafe extern "C" fn mcp23s08_i2c_exit() {
    static void mcp23s08_i2c_exit(void)
    {
    i2c_del_driver(&mcp230xx_driver);
    }
    module_exit(mcp23s08_i2c_exit);
    MODULE_DESCRIPTION("MCP23S08 I2C GPIO driver");
    MODULE_LICENSE("GPL");
