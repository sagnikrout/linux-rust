//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/at91-usart.c
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
// Driver for AT91 USART
//
// Copyright (C) 2018 Microchip Technology
//
// Author: Radu Pirea <radu.pirea@microchip.com>
//

    static const struct mfd_cell at91_usart_spi_subdev =
    MFD_CELL_NAME("at91_usart_spi");
    static const struct mfd_cell at91_usart_serial_subdev =
    MFD_CELL_NAME("atmel_usart_serial");
#[no_mangle]
unsafe extern "C" fn at91_usart_mode_probe(pdev: *mut platform_device) -> c_int {
    static int at91_usart_mode_probe(struct platform_device *pdev)
    {
    const struct mfd_cell *cell;
    let mut opmode: u32 = AT91_USART_MODE_SERIAL;
    device_property_read_u32(&pdev.dev, "atmel,usart-mode", &opmode);
    switch (opmode) {
    case AT91_USART_MODE_SPI:
    cell = &at91_usart_spi_subdev;
    break;
    case AT91_USART_MODE_SERIAL:
    cell = &at91_usart_serial_subdev;
    break;
    default:
    dev_err(&pdev.dev, "atmel,usart-mode has an invalid value %u\n",
    opmode);
    return -EINVAL;
    }
    return devm_mfd_add_devices(&pdev.dev, PLATFORM_DEVID_AUTO, cell, 1,
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static const struct of_device_id at91_usart_mode_of_match[] = {
    { .compatible = "atmel,at91rm9200-usart" },
    { .compatible = "atmel,at91sam9260-usart" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, at91_usart_mode_of_match);
    static struct platform_driver at91_usart_mfd = {
    .probe	= at91_usart_mode_probe,
    .driver	= {
    .name		= "at91_usart_mode",
    .of_match_table	= at91_usart_mode_of_match,
    },
    };
    module_platform_driver(at91_usart_mfd);
    MODULE_AUTHOR("Radu Pirea <radu.pirea@microchip.com>");
    MODULE_DESCRIPTION("AT91 USART MFD driver");
    MODULE_LICENSE("GPL v2");
