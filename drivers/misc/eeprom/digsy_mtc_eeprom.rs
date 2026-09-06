//! Automatically rewritten from C to Rust
//! Source: drivers/misc/eeprom/digsy_mtc_eeprom.c
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
// EEPROMs access control driver for display configuration EEPROMs
// on DigsyMTC board.
//
// (C) 2011 DENX Software Engineering, Anatolij Gustschin <agust@denx.de>
//
// FIXME: this driver is used on a device-tree probed platform: it
// should be defined as a bit-banged SPI device and probed from the device
// tree and not like this with static grabbing of a few numbered GPIO
// lines at random.
//
// Add proper SPI and EEPROM in arch/powerpc/boot/dts/digsy_mtc.dts
// and delete this driver.
//

pub const GPIO_EEPROM_CLK: c_int = 216;
pub const GPIO_EEPROM_CS: c_int = 210;
pub const GPIO_EEPROM_DI: c_int = 217;
pub const GPIO_EEPROM_DO: c_int = 249;
pub const GPIO_EEPROM_OE: c_int = 255;
pub const EE_SPI_BUS_NUM: c_int = 1;
    static const struct property_entry digsy_mtc_spi_properties[] = {
    PROPERTY_ENTRY_U32("data-size", 8),
    { }
    };
    static const struct software_node digsy_mtc_spi_node = {
    .properties = digsy_mtc_spi_properties,
    };
    static struct spi_gpio_platform_data eeprom_spi_gpio_data = {
    .num_chipselect	= 1,
    };
    static struct platform_device digsy_mtc_eeprom = {
    .name	= "spi_gpio",
    .id	= EE_SPI_BUS_NUM,
    .dev	= {
    .platform_data	= &eeprom_spi_gpio_data,
    },
    };
    static struct gpiod_lookup_table eeprom_spi_gpiod_table = {
    .dev_id         = "spi_gpio.1",
    .table          = {
    GPIO_LOOKUP("gpio@b00", GPIO_EEPROM_CLK,
    "sck", GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP("gpio@b00", GPIO_EEPROM_DI,
    "mosi", GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP("gpio@b00", GPIO_EEPROM_DO,
    "miso", GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP("gpio@b00", GPIO_EEPROM_CS,
    "cs", GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP("gpio@b00", GPIO_EEPROM_OE,
    "select", GPIO_ACTIVE_LOW),
    { },
    },
    };
    static struct spi_board_info digsy_mtc_eeprom_info[] __initdata = {
    {
    .modalias		= "eeprom-93xx46",
    .max_speed_hz		= 1000000,
    .bus_num		= EE_SPI_BUS_NUM,
    .chip_select		= 0,
    .mode			= SPI_MODE_0,
    },
    };
#[no_mangle]
unsafe extern "C" fn digsy_mtc_eeprom_devices_init() -> int __init {
    static int __init digsy_mtc_eeprom_devices_init(void)
    {
    int ret;
    gpiod_add_lookup_table(&eeprom_spi_gpiod_table);
    spi_register_board_info(digsy_mtc_eeprom_info,
    ARRAY_SIZE(digsy_mtc_eeprom_info));
    ret = device_add_software_node(&digsy_mtc_eeprom.dev, &digsy_mtc_spi_node);
    if (ret)
    return ret;
    ret = platform_device_register(&digsy_mtc_eeprom);
    if (ret)
    device_remove_software_node(&digsy_mtc_eeprom.dev);
    return ret;
    }
    device_initcall(digsy_mtc_eeprom_devices_init);
