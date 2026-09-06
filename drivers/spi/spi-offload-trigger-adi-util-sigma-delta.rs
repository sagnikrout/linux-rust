//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-offload-trigger-adi-util-sigma-delta.c
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
// Copyright (C) 2025 Analog Devices Inc.
// Copyright (C) 2025 BayLibre, SAS
//

    static bool adi_util_sigma_delta_match(struct spi_offload_trigger *trigger,
    enum spi_offload_trigger_type type,
    u64 *args, u32 nargs)
    {
    let mut type: return = = SPI_OFFLOAD_TRIGGER_DATA_READY && nargs == 0;
    }
    static const struct spi_offload_trigger_ops adi_util_sigma_delta_ops = {
    .match = adi_util_sigma_delta_match,
    };
#[no_mangle]
unsafe extern "C" fn adi_util_sigma_delta_probe(pdev: *mut platform_device) -> c_int {
    static int adi_util_sigma_delta_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_offload_trigger_info info = {
    .fwnode = dev_fwnode(dev),
    .ops = &adi_util_sigma_delta_ops,
    };
    struct clk *clk;
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "Failed to get clock\n");
    return devm_spi_offload_trigger_register(dev, &info);
    }
    static const struct of_device_id adi_util_sigma_delta_of_match_table[] = {
    { .compatible = "adi,util-sigma-delta-spi", },
    { }
    };
    MODULE_DEVICE_TABLE(of, adi_util_sigma_delta_of_match_table);
    static struct platform_driver adi_util_sigma_delta_driver = {
    .probe  = adi_util_sigma_delta_probe,
    .driver = {
    .name = "adi-util-sigma-delta-spi",
    .of_match_table = adi_util_sigma_delta_of_match_table,
    },
    };
    module_platform_driver(adi_util_sigma_delta_driver);
    MODULE_AUTHOR("David Lechner <dlechner@baylibre.com>");
    MODULE_DESCRIPTION("ADI Sigma-Delta SPI offload trigger utility driver");
    MODULE_LICENSE("GPL");
