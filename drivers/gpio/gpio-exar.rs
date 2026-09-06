//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-exar.c
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
// GPIO driver for Exar XR17V35X chip
//
// Copyright (C) 2015 Sudip Mukherjee <sudip.mukherjee@codethink.co.uk>
//

pub const EXAR_OFFSET_MPIOLVL_LO: c_uint = 0x90;
pub const EXAR_OFFSET_MPIOSEL_LO: c_uint = 0x93;
pub const EXAR_OFFSET_MPIOLVL_HI: c_uint = 0x96;
pub const EXAR_OFFSET_MPIOSEL_HI: c_uint = 0x99;
//
// The Device Configuration and UART Configuration Registers
// for each UART channel take 1KB of memory address space.
//
pub const EXAR_UART_CHANNEL_SIZE: c_uint = 0x400;

    static DEFINE_IDA(ida_index);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exar_gpio_chip {
    pub gpio_chip: gpio_chip,
    pub regmap: *mut regmap,
    pub index: c_int,
    pub name: [c_char; 20],
    pub first_pin: c_uint,
//
// The offset to the cascaded device's (if existing)
// Device Configuration Registers.
//
    pub cascaded_offset: c_uint,
}

    static unsigned int
    exar_offset_to_sel_addr(struct exar_gpio_chip *exar_gpio, unsigned int offset)
    {
    let mut pin: c_uint = exar_gpio.first_pin + (offset % 16);
    let mut cascaded: c_uint = offset / 16;
    let mut addr: c_uint = pin / 8 ? EXAR_OFFSET_MPIOSEL_HI : EXAR_OFFSET_MPIOSEL_LO;
    return addr + (cascaded ? exar_gpio.cascaded_offset : 0);
    }
    static unsigned int
    exar_offset_to_lvl_addr(struct exar_gpio_chip *exar_gpio, unsigned int offset)
    {
    let mut pin: c_uint = exar_gpio.first_pin + (offset % 16);
    let mut cascaded: c_uint = offset / 16;
    let mut addr: c_uint = pin / 8 ? EXAR_OFFSET_MPIOLVL_HI : EXAR_OFFSET_MPIOLVL_LO;
    return addr + (cascaded ? exar_gpio.cascaded_offset : 0);
    }
    static unsigned int
    exar_offset_to_bit(struct exar_gpio_chip *exar_gpio, unsigned int offset)
    {
    let mut pin: c_uint = exar_gpio.first_pin + (offset % 16);
    return pin % 8;
    }
#[no_mangle]
unsafe extern "C" fn exar_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int exar_get_direction(struct gpio_chip *chip, unsigned int offset)
    {
    struct exar_gpio_chip *exar_gpio = gpiochip_get_data(chip);
    let mut addr: c_uint = exar_offset_to_sel_addr(exar_gpio, offset);
    let mut bit: c_uint = exar_offset_to_bit(exar_gpio, offset);
    if (regmap_test_bits(exar_gpio.regmap, addr, BIT(bit)))
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn exar_get_value(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int exar_get_value(struct gpio_chip *chip, unsigned int offset)
    {
    struct exar_gpio_chip *exar_gpio = gpiochip_get_data(chip);
    let mut addr: c_uint = exar_offset_to_lvl_addr(exar_gpio, offset);
    let mut bit: c_uint = exar_offset_to_bit(exar_gpio, offset);
    return !!(regmap_test_bits(exar_gpio.regmap, addr, BIT(bit)));
    }
    static int exar_set_value(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct exar_gpio_chip *exar_gpio = gpiochip_get_data(chip);
    let mut addr: c_uint = exar_offset_to_lvl_addr(exar_gpio, offset);
    let mut bit: c_uint = exar_offset_to_bit(exar_gpio, offset);
    let mut bit_value: c_uint = value ? BIT(bit) : 0;
//
// regmap_write_bits() forces value to be written when an external
// pull up/down might otherwise indicate value was already set.
//
    return regmap_write_bits(exar_gpio.regmap, addr, BIT(bit), bit_value);
    }
    static int exar_direction_output(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct exar_gpio_chip *exar_gpio = gpiochip_get_data(chip);
    let mut addr: c_uint = exar_offset_to_sel_addr(exar_gpio, offset);
    let mut bit: c_uint = exar_offset_to_bit(exar_gpio, offset);
    int ret;
    ret = exar_set_value(chip, offset, value);
    if (ret)
    return ret;
    return regmap_clear_bits(exar_gpio.regmap, addr, BIT(bit));
    }
#[no_mangle]
unsafe extern "C" fn exar_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int exar_direction_input(struct gpio_chip *chip, unsigned int offset)
    {
    struct exar_gpio_chip *exar_gpio = gpiochip_get_data(chip);
    let mut addr: c_uint = exar_offset_to_sel_addr(exar_gpio, offset);
    let mut bit: c_uint = exar_offset_to_bit(exar_gpio, offset);
    regmap_set_bits(exar_gpio.regmap, addr, BIT(bit));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exar_devm_ida_free(data: *mut c_void) {
    static void exar_devm_ida_free(void *data)
    {
    struct exar_gpio_chip *exar_gpio = data;
    ida_free(&ida_index, exar_gpio.index);
    }
    static const struct regmap_config exar_regmap_config = {
    .name		= "exar-gpio",
    .reg_bits	= 16,
    .val_bits	= 8,
    .io_port	= true,
    };
#[no_mangle]
unsafe extern "C" fn gpio_exar_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_exar_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pci_dev *pcidev = to_pci_dev(dev.parent);
    struct exar_gpio_chip *exar_gpio;
    u32 first_pin, ngpios;
    void __iomem *p;
    int index, ret;
//
// The UART driver must have mapped region 0 prior to registering this
// device - use it.
//
    p = pcim_iomap_table(pcidev)[0];
    if (!p)
    return -ENOMEM;
    ret = device_property_read_u32(dev, "exar,first-pin", &first_pin);
    if (ret)
    return ret;
    ret = device_property_read_u32(dev, "ngpios", &ngpios);
    if (ret)
    return ret;
    exar_gpio = devm_kzalloc(dev, sizeof(*exar_gpio), GFP_KERNEL);
    if (!exar_gpio)
    return -ENOMEM;
//
// If cascaded, secondary xr17v354 or xr17v358 have the same amount
// of MPIOs as their primaries and the last 4 bits of the primary's
// PCI Device ID is the number of its UART channels.
//
    if (pcidev.device & GENMASK(15, 12)) {
    ngpios += ngpios;
    exar_gpio.cascaded_offset = (pcidev.device & GENMASK(3, 0)) *
    EXAR_UART_CHANNEL_SIZE;
    }
//
// We don't need to check the return values of mmio regmap operations (unless
// the regmap has a clock attached which is not the case here).
//
    exar_gpio.regmap = devm_regmap_init_mmio(dev, p, &exar_regmap_config);
    if (IS_ERR(exar_gpio.regmap))
    return PTR_ERR(exar_gpio.regmap);
    index = ida_alloc(&ida_index, GFP_KERNEL);
    if (index < 0)
    return index;
    ret = devm_add_action_or_reset(dev, exar_devm_ida_free, exar_gpio);
    if (ret)
    return ret;
    sprintf(exar_gpio.name, "exar_gpio%d", index);
    exar_gpio.gpio_chip.label = exar_gpio.name;
    exar_gpio.gpio_chip.parent = dev;
    exar_gpio.gpio_chip.direction_output = exar_direction_output;
    exar_gpio.gpio_chip.direction_input = exar_direction_input;
    exar_gpio.gpio_chip.get_direction = exar_get_direction;
    exar_gpio.gpio_chip.get = exar_get_value;
    exar_gpio.gpio_chip.set = exar_set_value;
    exar_gpio.gpio_chip.base = -1;
    exar_gpio.gpio_chip.ngpio = ngpios;
    exar_gpio.index = index;
    exar_gpio.first_pin = first_pin;
    ret = devm_gpiochip_add_data(dev, &exar_gpio.gpio_chip, exar_gpio);
    if (ret)
    return ret;
    return 0;
    }
    static struct platform_driver gpio_exar_driver = {
    .probe	= gpio_exar_probe,
    .driver	= {
    .name = DRIVER_NAME,
    },
    };
    module_platform_driver(gpio_exar_driver);
    MODULE_ALIAS("platform:" DRIVER_NAME);
    MODULE_DESCRIPTION("Exar GPIO driver");
    MODULE_AUTHOR("Sudip Mukherjee <sudip.mukherjee@codethink.co.uk>");
    MODULE_LICENSE("GPL");
