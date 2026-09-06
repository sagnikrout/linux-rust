//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/inv_mpu6050/inv_mpu_spi.c
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
// Copyright (C) 2015 Intel Corporation Inc.
//

    static const struct regmap_config inv_mpu_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn inv_mpu_i2c_disable(indio_dev: *mut iio_dev) -> c_int {
    static int inv_mpu_i2c_disable(struct iio_dev *indio_dev)
    {
    struct inv_mpu6050_state *st = iio_priv(indio_dev);
    let mut ret: c_int = 0;
    if (st.reg.i2c_if) {
    ret = regmap_write(st.map, st.reg.i2c_if,
    INV_ICM20602_BIT_I2C_IF_DIS);
    } else {
    st.chip_config.user_ctrl |= INV_MPU6050_BIT_I2C_IF_DIS;
    ret = regmap_write(st.map, st.reg.user_ctrl,
    st.chip_config.user_ctrl);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn inv_mpu_probe(spi: *mut spi_device) -> c_int {
    static int inv_mpu_probe(struct spi_device *spi)
    {
    const void *match;
    struct regmap *regmap;
    const struct spi_device_id *spi_id;
    const char *name = core::ptr::null_mut();
    enum inv_devices chip_type;
    if ((spi_id = spi_get_device_id(spi))) {
    chip_type = (enum inv_devices)spi_id.driver_data;
    name = spi_id.name;
    } else if ((match = device_get_match_data(&spi.dev))) {
    chip_type = (uintptr_t)match;
    name = dev_name(&spi.dev);
    } else {
    return -ENODEV;
    }
    regmap = devm_regmap_init_spi(spi, &inv_mpu_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap: %pe\n",
    regmap);
    return PTR_ERR(regmap);
    }
    return inv_mpu_core_probe(regmap, spi.irq, name,
    inv_mpu_i2c_disable, chip_type);
    }
//
// device id table is used to identify what device can be
// supported by this driver
//
    static const struct spi_device_id inv_mpu_id[] = {
    { .name = "mpu6000", .driver_data = INV_MPU6000 },
    { .name = "mpu6500", .driver_data = INV_MPU6500 },
    { .name = "mpu6515", .driver_data = INV_MPU6515 },
    { .name = "mpu6880", .driver_data = INV_MPU6880 },
    { .name = "mpu9250", .driver_data = INV_MPU9250 },
    { .name = "mpu9255", .driver_data = INV_MPU9255 },
    { .name = "icm20608", .driver_data = INV_ICM20608 },
    { .name = "icm20608d", .driver_data = INV_ICM20608D },
    { .name = "icm20609", .driver_data = INV_ICM20609 },
    { .name = "icm20689", .driver_data = INV_ICM20689 },
    { .name = "icm20600", .driver_data = INV_ICM20600 },
    { .name = "icm20602", .driver_data = INV_ICM20602 },
    { .name = "icm20690", .driver_data = INV_ICM20690 },
    { .name = "iam20380", .driver_data = INV_IAM20380 },
    { .name = "iam20680", .driver_data = INV_IAM20680 },
    { .name = "iam20680hp", .driver_data = INV_IAM20680HP },
    { .name = "iam20680ht", .driver_data = INV_IAM20680HT },
    { }
    };
    MODULE_DEVICE_TABLE(spi, inv_mpu_id);
    static const struct of_device_id inv_of_match[] = {
    {
    .compatible = "invensense,mpu6000",
    .data = (void *)INV_MPU6000
    },
    {
    .compatible = "invensense,mpu6500",
    .data = (void *)INV_MPU6500
    },
    {
    .compatible = "invensense,mpu6515",
    .data = (void *)INV_MPU6515
    },
    {
    .compatible = "invensense,mpu6880",
    .data = (void *)INV_MPU6880
    },
    {
    .compatible = "invensense,mpu9250",
    .data = (void *)INV_MPU9250
    },
    {
    .compatible = "invensense,mpu9255",
    .data = (void *)INV_MPU9255
    },
    {
    .compatible = "invensense,icm20608",
    .data = (void *)INV_ICM20608
    },
    {
    .compatible = "invensense,icm20608d",
    .data = (void *)INV_ICM20608D
    },
    {
    .compatible = "invensense,icm20609",
    .data = (void *)INV_ICM20609
    },
    {
    .compatible = "invensense,icm20689",
    .data = (void *)INV_ICM20689
    },
    {
    .compatible = "invensense,icm20600",
    .data = (void *)INV_ICM20600
    },
    {
    .compatible = "invensense,icm20602",
    .data = (void *)INV_ICM20602
    },
    {
    .compatible = "invensense,icm20690",
    .data = (void *)INV_ICM20690
    },
    {
    .compatible = "invensense,iam20380",
    .data = (void *)INV_IAM20380
    },
    {
    .compatible = "invensense,iam20680",
    .data = (void *)INV_IAM20680
    },
    {
    .compatible = "invensense,iam20680hp",
    .data = (void *)INV_IAM20680HP
    },
    {
    .compatible = "invensense,iam20680ht",
    .data = (void *)INV_IAM20680HT
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, inv_of_match);
    static const struct acpi_device_id inv_acpi_match[] = {
    {"INVN6000", INV_MPU6000},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, inv_acpi_match);
    static struct spi_driver inv_mpu_driver = {
    .probe		=	inv_mpu_probe,
    .id_table	=	inv_mpu_id,
    .driver = {
    .of_match_table = inv_of_match,
    .acpi_match_table = inv_acpi_match,
    .name	=	"inv-mpu6000-spi",
    .pm     =       pm_ptr(&inv_mpu_pmops),
    },
    };
    module_spi_driver(inv_mpu_driver);
    MODULE_AUTHOR("Adriana Reus <adriana.reus@intel.com>");
    MODULE_DESCRIPTION("Invensense device MPU6000 driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_MPU6050");
