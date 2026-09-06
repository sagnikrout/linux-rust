//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/kxsd9-spi.c
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

#[no_mangle]
unsafe extern "C" fn kxsd9_spi_probe(spi: *mut spi_device) -> c_int {
    static int kxsd9_spi_probe(struct spi_device *spi)
    {
    static const struct regmap_config config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x0e,
    };
    struct regmap *regmap;
    spi.mode = SPI_MODE_0;
    regmap = devm_regmap_init_spi(spi, &config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "%s: regmap allocation failed: %ld\n",
    __func__, PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    return kxsd9_common_probe(&spi.dev,
    regmap,
    spi_get_device_id(spi).name);
    }
#[no_mangle]
unsafe extern "C" fn kxsd9_spi_remove(spi: *mut spi_device) {
    static void kxsd9_spi_remove(struct spi_device *spi)
    {
    kxsd9_common_remove(&spi.dev);
    }
    static const struct spi_device_id kxsd9_spi_id[] = {
    { .name = "kxsd9" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, kxsd9_spi_id);
    static const struct of_device_id kxsd9_of_match[] = {
    { .compatible = "kionix,kxsd9" },
    { }
    };
    MODULE_DEVICE_TABLE(of, kxsd9_of_match);
    static struct spi_driver kxsd9_spi_driver = {
    .driver = {
    .name = "kxsd9",
    .pm = pm_ptr(&kxsd9_dev_pm_ops),
    .of_match_table = kxsd9_of_match,
    },
    .probe = kxsd9_spi_probe,
    .remove = kxsd9_spi_remove,
    .id_table = kxsd9_spi_id,
    };
    module_spi_driver(kxsd9_spi_driver);
    MODULE_AUTHOR("Jonathan Cameron <jic23@kernel.org>");
    MODULE_DESCRIPTION("Kionix KXSD9 SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_KXSD9");
