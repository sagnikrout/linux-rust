//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/mpu3050-i2c.c
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

    static const struct regmap_config mpu3050_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn mpu3050_i2c_bypass_select(mux: *mut i2c_mux_core, chan_id: u32) -> c_int {
    static int mpu3050_i2c_bypass_select(struct i2c_mux_core *mux, u32 chan_id)
    {
    struct mpu3050 *mpu3050 = i2c_mux_priv(mux);
// Just power up the device, that is all that is needed
    return pm_runtime_resume_and_get(mpu3050.dev);
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_i2c_bypass_deselect(mux: *mut i2c_mux_core, chan_id: u32) -> c_int {
    static int mpu3050_i2c_bypass_deselect(struct i2c_mux_core *mux, u32 chan_id)
    {
    struct mpu3050 *mpu3050 = i2c_mux_priv(mux);
    pm_runtime_put_autosuspend(mpu3050.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_i2c_probe(client: *mut i2c_client) -> c_int {
    static int mpu3050_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regmap *regmap;
    const char *name;
    struct mpu3050 *mpu3050;
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_I2C_BLOCK))
    return -EOPNOTSUPP;
    if (id)
    name = id.name;
    else
    return -ENODEV;
    regmap = devm_regmap_init_i2c(client, &mpu3050_i2c_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Failed to register i2c regmap: %pe\n",
    regmap);
    return PTR_ERR(regmap);
    }
    ret = mpu3050_common_probe(&client.dev, regmap, client.irq, name);
    if (ret)
    return ret;
// The main driver is up, now register the I2C mux
    mpu3050 = iio_priv(dev_get_drvdata(&client.dev));
    mpu3050.i2cmux = i2c_mux_alloc(client.adapter, &client.dev,
    1, 0, I2C_MUX_LOCKED | I2C_MUX_GATE,
    mpu3050_i2c_bypass_select,
    mpu3050_i2c_bypass_deselect);
// Just fail the mux, there is no point in killing the driver
    if (!mpu3050.i2cmux)
    dev_err(&client.dev, "failed to allocate I2C mux\n");
    else {
    mpu3050.i2cmux.priv = mpu3050;
// Ignore failure, not critical
    i2c_mux_add_adapter(mpu3050.i2cmux, 0, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_i2c_remove(client: *mut i2c_client) {
    static void mpu3050_i2c_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(&client.dev);
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    if (mpu3050.i2cmux)
    i2c_mux_del_adapters(mpu3050.i2cmux);
    mpu3050_common_remove(&client.dev);
    }
//
// device id table is used to identify what device can be
// supported by this driver
//
    static const struct i2c_device_id mpu3050_i2c_id[] = {
    { .name = "mpu3050" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mpu3050_i2c_id);
    static const struct of_device_id mpu3050_i2c_of_match[] = {
    { .compatible = "invensense,mpu3050", .data = "mpu3050" },
// Deprecated vendor ID from the Input driver
    { .compatible = "invn,mpu3050", .data = "mpu3050" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mpu3050_i2c_of_match);
    static struct i2c_driver mpu3050_i2c_driver = {
    .probe = mpu3050_i2c_probe,
    .remove = mpu3050_i2c_remove,
    .id_table = mpu3050_i2c_id,
    .driver = {
    .of_match_table = mpu3050_i2c_of_match,
    .name = "mpu3050-i2c",
    .pm = pm_ptr(&mpu3050_dev_pm_ops),
    },
    };
    module_i2c_driver(mpu3050_i2c_driver);
    MODULE_AUTHOR("Linus Walleij");
    MODULE_DESCRIPTION("Invensense MPU3050 gyroscope driver");
    MODULE_LICENSE("GPL");
