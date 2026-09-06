//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/bmi270/bmi270_spi.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)

//
// The following two functions are taken from the BMI323 spi driver code.
// In section 6.4 of the BMI270 data it specifies that after a read
// operation the first data byte from the device is a dummy byte
//
    static int bmi270_regmap_spi_read(void *spi, const void *reg_buf,
    size_t reg_size, void *val_buf,
    size_t val_size)
    {
    return spi_write_then_read(spi, reg_buf, reg_size, val_buf, val_size);
    }
    static int bmi270_regmap_spi_write(void *spi, const void *data,
    size_t count)
    {
    u8 *data_buff = (u8 *)data;
//
// Remove the extra pad byte since its only needed for the read
// operation
//
    data_buff[1] = data_buff[0];
    return spi_write_then_read(spi, data_buff + 1, count - 1, core::ptr::null_mut(), 0);
    }
    static const struct regmap_bus bmi270_regmap_bus = {
    .read = bmi270_regmap_spi_read,
    .write = bmi270_regmap_spi_write,
    };
    static const struct regmap_config bmi270_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .pad_bits = 8,
    .read_flag_mask = BIT(7),
    };
#[no_mangle]
unsafe extern "C" fn bmi270_spi_probe(spi: *mut spi_device) -> c_int {
    static int bmi270_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    struct device *dev = &spi.dev;
    const struct bmi270_chip_info *chip_info;
    chip_info = spi_get_device_match_data(spi);
    if (!chip_info)
    return -ENODEV;
    regmap = devm_regmap_init(dev, &bmi270_regmap_bus, dev,
    &bmi270_spi_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to init spi regmap\n");
    return bmi270_core_probe(dev, regmap, chip_info);
    }
    static const struct spi_device_id bmi270_spi_id[] = {
    { .name = "bmi260", .driver_data = (kernel_ulong_t)&bmi260_chip_info },
    { .name = "bmi270", .driver_data = (kernel_ulong_t)&bmi270_chip_info },
    { }
    };
    static const struct of_device_id bmi270_of_match[] = {
    { .compatible = "bosch,bmi260", .data = &bmi260_chip_info },
    { .compatible = "bosch,bmi270", .data = &bmi270_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmi270_of_match);
    static struct spi_driver bmi270_spi_driver = {
    .driver = {
    .name = "bmi270",
    .pm = pm_ptr(&bmi270_core_pm_ops),
    .of_match_table = bmi270_of_match,
    },
    .probe = bmi270_spi_probe,
    .id_table = bmi270_spi_id,
    };
    module_spi_driver(bmi270_spi_driver);
    MODULE_AUTHOR("Alex Lanzano");
    MODULE_DESCRIPTION("BMI270 driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BMI270");
