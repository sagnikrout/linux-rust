//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-msc313.c
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
// Copyright (C) 2020 Daniel Palmer<daniel@thingy.jp>

//
// These bits need to be saved to correctly restore the
// gpio state when resuming from suspend to memory.
//

// pad names for fuart, same for all SoCs so far

// pad names for sr, mercury5 is different

// pad names for sd, same for all SoCs so far

// pad names for i2c1, same for all SoCs so for

// pad names for spi0, same for all SoCs so far

    MSC313_PINNAME_FUART_RX,	\
    MSC313_PINNAME_FUART_TX,	\
    MSC313_PINNAME_FUART_CTS,	\
    MSC313_PINNAME_FUART_RTS
pub const OFF_FUART_RX: c_uint = 0x50;
pub const OFF_FUART_TX: c_uint = 0x54;
pub const OFF_FUART_CTS: c_uint = 0x58;
pub const OFF_FUART_RTS: c_uint = 0x5c;

    OFF_FUART_RX,	\
    OFF_FUART_TX,	\
    OFF_FUART_CTS,	\
    OFF_FUART_RTS

    MSC313_PINNAME_SR_IO2,	\
    MSC313_PINNAME_SR_IO3,	\
    MSC313_PINNAME_SR_IO4,	\
    MSC313_PINNAME_SR_IO5,	\
    MSC313_PINNAME_SR_IO6,	\
    MSC313_PINNAME_SR_IO7,	\
    MSC313_PINNAME_SR_IO8,	\
    MSC313_PINNAME_SR_IO9,	\
    MSC313_PINNAME_SR_IO10,	\
    MSC313_PINNAME_SR_IO11,	\
    MSC313_PINNAME_SR_IO12,	\
    MSC313_PINNAME_SR_IO13,	\
    MSC313_PINNAME_SR_IO14,	\
    MSC313_PINNAME_SR_IO15,	\
    MSC313_PINNAME_SR_IO16,	\
    MSC313_PINNAME_SR_IO17
pub const OFF_SR_IO2: c_uint = 0x88;
pub const OFF_SR_IO3: c_uint = 0x8c;
pub const OFF_SR_IO4: c_uint = 0x90;
pub const OFF_SR_IO5: c_uint = 0x94;
pub const OFF_SR_IO6: c_uint = 0x98;
pub const OFF_SR_IO7: c_uint = 0x9c;
pub const OFF_SR_IO8: c_uint = 0xa0;
pub const OFF_SR_IO9: c_uint = 0xa4;
pub const OFF_SR_IO10: c_uint = 0xa8;
pub const OFF_SR_IO11: c_uint = 0xac;
pub const OFF_SR_IO12: c_uint = 0xb0;
pub const OFF_SR_IO13: c_uint = 0xb4;
pub const OFF_SR_IO14: c_uint = 0xb8;
pub const OFF_SR_IO15: c_uint = 0xbc;
pub const OFF_SR_IO16: c_uint = 0xc0;
pub const OFF_SR_IO17: c_uint = 0xc4;

    OFF_SR_IO2,	\
    OFF_SR_IO3,	\
    OFF_SR_IO4,	\
    OFF_SR_IO5,	\
    OFF_SR_IO6,	\
    OFF_SR_IO7,	\
    OFF_SR_IO8,	\
    OFF_SR_IO9,	\
    OFF_SR_IO10,	\
    OFF_SR_IO11,	\
    OFF_SR_IO12,	\
    OFF_SR_IO13,	\
    OFF_SR_IO14,	\
    OFF_SR_IO15,	\
    OFF_SR_IO16,	\
    OFF_SR_IO17

    MSC313_PINNAME_SD_CLK,	\
    MSC313_PINNAME_SD_CMD,	\
    MSC313_PINNAME_SD_D0,	\
    MSC313_PINNAME_SD_D1,	\
    MSC313_PINNAME_SD_D2,	\
    MSC313_PINNAME_SD_D3
pub const OFF_SD_CLK: c_uint = 0x140;
pub const OFF_SD_CMD: c_uint = 0x144;
pub const OFF_SD_D0: c_uint = 0x148;
pub const OFF_SD_D1: c_uint = 0x14c;
pub const OFF_SD_D2: c_uint = 0x150;
pub const OFF_SD_D3: c_uint = 0x154;

    OFF_SD_CLK,	\
    OFF_SD_CMD,	\
    OFF_SD_D0,	\
    OFF_SD_D1,	\
    OFF_SD_D2,	\
    OFF_SD_D3

    MSC313_PINNAME_I2C1_SCL,	\
    MSC313_PINNAME_I2C1_SCA
pub const OFF_I2C1_SCL: c_uint = 0x188;
pub const OFF_I2C1_SCA: c_uint = 0x18c;

    OFF_I2C1_SCL,	\
    OFF_I2C1_SCA

    MSC313_PINNAME_SPI0_CZ,	\
    MSC313_PINNAME_SPI0_CK,	\
    MSC313_PINNAME_SPI0_DI,	\
    MSC313_PINNAME_SPI0_DO
pub const OFF_SPI0_CZ: c_uint = 0x1c0;
pub const OFF_SPI0_CK: c_uint = 0x1c4;
pub const OFF_SPI0_DI: c_uint = 0x1c8;
pub const OFF_SPI0_DO: c_uint = 0x1cc;

    OFF_SPI0_CZ,	\
    OFF_SPI0_CK,	\
    OFF_SPI0_DI,	\
    OFF_SPI0_DO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msc313_gpio_data {
    pub names: *const *const c_char,
    pub offsets: *const c_uint,
    pub num: c_uint,
}

    static const struct msc313_gpio_data _chip##_data = { \
    .names = _chip##_names, \
    .offsets = _chip##_offsets, \
    .num = ARRAY_SIZE(_chip##_offsets), \
    }

    static const char * const msc313_names[] = {
    FUART_NAMES,
    SR_NAMES,
    SD_NAMES,
    I2C1_NAMES,
    SPI0_NAMES,
    };
    static const unsigned int msc313_offsets[] = {
    FUART_OFFSETS,
    SR_OFFSETS,
    SD_OFFSETS,
    I2C1_OFFSETS,
    SPI0_OFFSETS,
    };
    MSC313_GPIO_CHIPDATA(msc313);
//
// Unlike the msc313(e) the ssd20xd have a bunch of pins
// that are actually called gpio probably because they
// have no dedicated function.
//

    SSD20XD_PINNAME_GPIO1,  \
    SSD20XD_PINNAME_GPIO2,  \
    SSD20XD_PINNAME_GPIO3,  \
    SSD20XD_PINNAME_GPIO4,  \
    SSD20XD_PINNAME_GPIO5,  \
    SSD20XD_PINNAME_GPIO6,  \
    SSD20XD_PINNAME_GPIO7,  \
    SSD20XD_PINNAME_GPIO10, \
    SSD20XD_PINNAME_GPIO11, \
    SSD20XD_PINNAME_GPIO12, \
    SSD20XD_PINNAME_GPIO13, \
    SSD20XD_PINNAME_GPIO14, \
    SSD20XD_PINNAME_GPIO85, \
    SSD20XD_PINNAME_GPIO86, \
    SSD20XD_PINNAME_GPIO90
pub const SSD20XD_GPIO_OFF_GPIO0: c_uint = 0x0;
pub const SSD20XD_GPIO_OFF_GPIO1: c_uint = 0x4;
pub const SSD20XD_GPIO_OFF_GPIO2: c_uint = 0x8;
pub const SSD20XD_GPIO_OFF_GPIO3: c_uint = 0xc;
pub const SSD20XD_GPIO_OFF_GPIO4: c_uint = 0x10;
pub const SSD20XD_GPIO_OFF_GPIO5: c_uint = 0x14;
pub const SSD20XD_GPIO_OFF_GPIO6: c_uint = 0x18;
pub const SSD20XD_GPIO_OFF_GPIO7: c_uint = 0x1c;
pub const SSD20XD_GPIO_OFF_GPIO10: c_uint = 0x28;
pub const SSD20XD_GPIO_OFF_GPIO11: c_uint = 0x2c;
pub const SSD20XD_GPIO_OFF_GPIO12: c_uint = 0x30;
pub const SSD20XD_GPIO_OFF_GPIO13: c_uint = 0x34;
pub const SSD20XD_GPIO_OFF_GPIO14: c_uint = 0x38;
pub const SSD20XD_GPIO_OFF_GPIO85: c_uint = 0x100;
pub const SSD20XD_GPIO_OFF_GPIO86: c_uint = 0x104;
pub const SSD20XD_GPIO_OFF_GPIO90: c_uint = 0x114;

    SSD20XD_GPIO_OFF_GPIO1,  \
    SSD20XD_GPIO_OFF_GPIO2,  \
    SSD20XD_GPIO_OFF_GPIO3,  \
    SSD20XD_GPIO_OFF_GPIO4,  \
    SSD20XD_GPIO_OFF_GPIO5,  \
    SSD20XD_GPIO_OFF_GPIO6,  \
    SSD20XD_GPIO_OFF_GPIO7,  \
    SSD20XD_GPIO_OFF_GPIO10, \
    SSD20XD_GPIO_OFF_GPIO11, \
    SSD20XD_GPIO_OFF_GPIO12, \
    SSD20XD_GPIO_OFF_GPIO13, \
    SSD20XD_GPIO_OFF_GPIO14, \
    SSD20XD_GPIO_OFF_GPIO85, \
    SSD20XD_GPIO_OFF_GPIO86, \
    SSD20XD_GPIO_OFF_GPIO90
// "ttl" pins lcd interface pins

    SSD20XD_PINNAME_TTL1,  \
    SSD20XD_PINNAME_TTL2,  \
    SSD20XD_PINNAME_TTL3,  \
    SSD20XD_PINNAME_TTL4,  \
    SSD20XD_PINNAME_TTL5,  \
    SSD20XD_PINNAME_TTL6,  \
    SSD20XD_PINNAME_TTL7,  \
    SSD20XD_PINNAME_TTL8,  \
    SSD20XD_PINNAME_TTL9,  \
    SSD20XD_PINNAME_TTL10, \
    SSD20XD_PINNAME_TTL11, \
    SSD20XD_PINNAME_TTL12, \
    SSD20XD_PINNAME_TTL13, \
    SSD20XD_PINNAME_TTL14, \
    SSD20XD_PINNAME_TTL15, \
    SSD20XD_PINNAME_TTL16, \
    SSD20XD_PINNAME_TTL17, \
    SSD20XD_PINNAME_TTL18, \
    SSD20XD_PINNAME_TTL19, \
    SSD20XD_PINNAME_TTL20, \
    SSD20XD_PINNAME_TTL21, \
    SSD20XD_PINNAME_TTL22, \
    SSD20XD_PINNAME_TTL23, \
    SSD20XD_PINNAME_TTL24, \
    SSD20XD_PINNAME_TTL25, \
    SSD20XD_PINNAME_TTL26, \
    SSD20XD_PINNAME_TTL27
pub const SSD20XD_TTL_OFFSET_TTL0: c_uint = 0x80;
pub const SSD20XD_TTL_OFFSET_TTL1: c_uint = 0x84;
pub const SSD20XD_TTL_OFFSET_TTL2: c_uint = 0x88;
pub const SSD20XD_TTL_OFFSET_TTL3: c_uint = 0x8c;
pub const SSD20XD_TTL_OFFSET_TTL4: c_uint = 0x90;
pub const SSD20XD_TTL_OFFSET_TTL5: c_uint = 0x94;
pub const SSD20XD_TTL_OFFSET_TTL6: c_uint = 0x98;
pub const SSD20XD_TTL_OFFSET_TTL7: c_uint = 0x9c;
pub const SSD20XD_TTL_OFFSET_TTL8: c_uint = 0xa0;
pub const SSD20XD_TTL_OFFSET_TTL9: c_uint = 0xa4;
pub const SSD20XD_TTL_OFFSET_TTL10: c_uint = 0xa8;
pub const SSD20XD_TTL_OFFSET_TTL11: c_uint = 0xac;
pub const SSD20XD_TTL_OFFSET_TTL12: c_uint = 0xb0;
pub const SSD20XD_TTL_OFFSET_TTL13: c_uint = 0xb4;
pub const SSD20XD_TTL_OFFSET_TTL14: c_uint = 0xb8;
pub const SSD20XD_TTL_OFFSET_TTL15: c_uint = 0xbc;
pub const SSD20XD_TTL_OFFSET_TTL16: c_uint = 0xc0;
pub const SSD20XD_TTL_OFFSET_TTL17: c_uint = 0xc4;
pub const SSD20XD_TTL_OFFSET_TTL18: c_uint = 0xc8;
pub const SSD20XD_TTL_OFFSET_TTL19: c_uint = 0xcc;
pub const SSD20XD_TTL_OFFSET_TTL20: c_uint = 0xd0;
pub const SSD20XD_TTL_OFFSET_TTL21: c_uint = 0xd4;
pub const SSD20XD_TTL_OFFSET_TTL22: c_uint = 0xd8;
pub const SSD20XD_TTL_OFFSET_TTL23: c_uint = 0xdc;
pub const SSD20XD_TTL_OFFSET_TTL24: c_uint = 0xe0;
pub const SSD20XD_TTL_OFFSET_TTL25: c_uint = 0xe4;
pub const SSD20XD_TTL_OFFSET_TTL26: c_uint = 0xe8;
pub const SSD20XD_TTL_OFFSET_TTL27: c_uint = 0xec;

    SSD20XD_TTL_OFFSET_TTL1,  \
    SSD20XD_TTL_OFFSET_TTL2,  \
    SSD20XD_TTL_OFFSET_TTL3,  \
    SSD20XD_TTL_OFFSET_TTL4,  \
    SSD20XD_TTL_OFFSET_TTL5,  \
    SSD20XD_TTL_OFFSET_TTL6,  \
    SSD20XD_TTL_OFFSET_TTL7,  \
    SSD20XD_TTL_OFFSET_TTL8,  \
    SSD20XD_TTL_OFFSET_TTL9,  \
    SSD20XD_TTL_OFFSET_TTL10, \
    SSD20XD_TTL_OFFSET_TTL11, \
    SSD20XD_TTL_OFFSET_TTL12, \
    SSD20XD_TTL_OFFSET_TTL13, \
    SSD20XD_TTL_OFFSET_TTL14, \
    SSD20XD_TTL_OFFSET_TTL15, \
    SSD20XD_TTL_OFFSET_TTL16, \
    SSD20XD_TTL_OFFSET_TTL17, \
    SSD20XD_TTL_OFFSET_TTL18, \
    SSD20XD_TTL_OFFSET_TTL19, \
    SSD20XD_TTL_OFFSET_TTL20, \
    SSD20XD_TTL_OFFSET_TTL21, \
    SSD20XD_TTL_OFFSET_TTL22, \
    SSD20XD_TTL_OFFSET_TTL23, \
    SSD20XD_TTL_OFFSET_TTL24, \
    SSD20XD_TTL_OFFSET_TTL25, \
    SSD20XD_TTL_OFFSET_TTL26, \
    SSD20XD_TTL_OFFSET_TTL27
// On the ssd20xd the two normal uarts have dedicated pins

    SSD20XD_PINNAME_UART0_RX, \
    SSD20XD_PINNAME_UART0_TX

    SSD20XD_PINNAME_UART1_RX, \
    SSD20XD_PINNAME_UART1_TX
pub const SSD20XD_OFF_UART0_RX: c_uint = 0x60;
pub const SSD20XD_OFF_UART0_TX: c_uint = 0x64;

    SSD20XD_OFF_UART0_RX, \
    SSD20XD_OFF_UART0_TX
pub const SSD20XD_OFF_UART1_RX: c_uint = 0x68;
pub const SSD20XD_OFF_UART1_TX: c_uint = 0x6c;

    SSD20XD_OFF_UART1_RX, \
    SSD20XD_OFF_UART1_TX
//
// ssd20x has the same pin names but different ordering
// of the registers that control the gpio.
//
pub const SSD20XD_OFF_SD_D0: c_uint = 0x140;
pub const SSD20XD_OFF_SD_D1: c_uint = 0x144;
pub const SSD20XD_OFF_SD_D2: c_uint = 0x148;
pub const SSD20XD_OFF_SD_D3: c_uint = 0x14c;
pub const SSD20XD_OFF_SD_CMD: c_uint = 0x150;
pub const SSD20XD_OFF_SD_CLK: c_uint = 0x154;

    SSD20XD_OFF_SD_CMD, \
    SSD20XD_OFF_SD_D0,  \
    SSD20XD_OFF_SD_D1,  \
    SSD20XD_OFF_SD_D2,  \
    SSD20XD_OFF_SD_D3
    static const char * const ssd20xd_names[] = {
    FUART_NAMES,
    SD_NAMES,
    SSD20XD_UART0_NAMES,
    SSD20XD_UART1_NAMES,
    SSD20XD_TTL_PINNAMES,
    SSD20XD_GPIO_NAMES,
    };
    static const unsigned int ssd20xd_offsets[] = {
    FUART_OFFSETS,
    SSD20XD_SD_OFFSETS,
    SSD20XD_UART0_OFFSETS,
    SSD20XD_UART1_OFFSETS,
    SSD20XD_TTL_OFFSETS,
    SSD20XD_GPIO_OFFSETS,
    };
    MSC313_GPIO_CHIPDATA(ssd20xd);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msc313_gpio {
    pub base: *mut void __iomem,
    pub gpio_data: *const msc313_gpio_data,
    pub saved: *mut u8,
}

#[no_mangle]
unsafe extern "C" fn msc313_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int msc313_gpio_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct msc313_gpio *gpio = gpiochip_get_data(chip);
    let mut gpioreg: u8 = readb_relaxed(gpio.base + gpio.gpio_data.offsets[offset]);
    if (value)
    gpioreg |= MSC313_GPIO_OUT;
    else
    gpioreg &= ~MSC313_GPIO_OUT;
    writeb_relaxed(gpioreg, gpio.base + gpio.gpio_data.offsets[offset]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int msc313_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct msc313_gpio *gpio = gpiochip_get_data(chip);
    return readb_relaxed(gpio.base + gpio.gpio_data.offsets[offset]) & MSC313_GPIO_IN;
    }
#[no_mangle]
unsafe extern "C" fn msc313_gpio_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int msc313_gpio_direction_input(struct gpio_chip *chip, unsigned int offset)
    {
    struct msc313_gpio *gpio = gpiochip_get_data(chip);
    let mut gpioreg: u8 = readb_relaxed(gpio.base + gpio.gpio_data.offsets[offset]);
    gpioreg |= MSC313_GPIO_OEN;
    writeb_relaxed(gpioreg, gpio.base + gpio.gpio_data.offsets[offset]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_gpio_direction_output(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int msc313_gpio_direction_output(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct msc313_gpio *gpio = gpiochip_get_data(chip);
    let mut gpioreg: u8 = readb_relaxed(gpio.base + gpio.gpio_data.offsets[offset]);
    gpioreg &= ~MSC313_GPIO_OEN;
    if (value)
    gpioreg |= MSC313_GPIO_OUT;
    else
    gpioreg &= ~MSC313_GPIO_OUT;
    writeb_relaxed(gpioreg, gpio.base + gpio.gpio_data.offsets[offset]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_gpio_irq_mask(d: *mut irq_data) {
    static void msc313_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    irq_chip_mask_parent(d);
    gpiochip_disable_irq(gc, d.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn msc313_gpio_irq_unmask(d: *mut irq_data) {
    static void msc313_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    gpiochip_enable_irq(gc, d.hwirq);
    irq_chip_unmask_parent(d);
    }
//
// The interrupt handling happens in the parent interrupt controller,
// we don't do anything here.
//
    static const struct irq_chip msc313_gpio_irqchip = {
    .name = "GPIO",
    .irq_eoi = irq_chip_eoi_parent,
    .irq_mask = msc313_gpio_irq_mask,
    .irq_unmask = msc313_gpio_irq_unmask,
    .irq_set_type = irq_chip_set_type_parent,
    .irq_set_affinity = irq_chip_set_affinity_parent,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
//
// The parent interrupt controller needs the GIC interrupt type set to GIC_SPI
// so we need to provide the fwspec. Essentially gpiochip_populate_parent_fwspec_twocell
// that puts GIC_SPI into the first cell.
//
    static int msc313_gpio_populate_parent_fwspec(struct gpio_chip *gc,
    union gpio_irq_fwspec *gfwspec,
    unsigned int parent_hwirq,
    unsigned int parent_type)
    {
    struct irq_fwspec *fwspec = &gfwspec.fwspec;
    fwspec.fwnode = gc.irq.parent_domain.fwnode;
    fwspec.param_count = 3;
    fwspec.param[0] = GIC_SPI;
    fwspec.param[1] = parent_hwirq;
    fwspec.param[2] = parent_type;
    return 0;
    }
    static int msc313e_gpio_child_to_parent_hwirq(struct gpio_chip *chip,
    unsigned int child,
    unsigned int child_type,
    unsigned int *parent,
    unsigned int *parent_type)
    {
    struct msc313_gpio *priv = gpiochip_get_data(chip);
    let mut offset: c_uint = priv.gpio_data.offsets[child];
//
// only the spi0 pins have interrupts on the parent
// on all of the known chips and so far they are all
// mapped to the same place
//
    if (offset >= OFF_SPI0_CZ && offset <= OFF_SPI0_DO) {
// parent_type = child_type;
// parent = ((offset - OFF_SPI0_CZ) >> 2) + 28;
    return 0;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn msc313_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int msc313_gpio_probe(struct platform_device *pdev)
    {
    const struct msc313_gpio_data *match_data;
    struct msc313_gpio *gpio;
    struct gpio_chip *gpiochip;
    struct gpio_irq_chip *gpioirqchip;
    struct irq_domain *parent_domain;
    struct device_node *parent_node;
    struct device *dev = &pdev.dev;
    match_data = of_device_get_match_data(dev);
    if (!match_data)
    return -EINVAL;
    parent_node = of_irq_find_parent(dev.of_node);
    if (!parent_node)
    return -ENODEV;
    parent_domain = irq_find_host(parent_node);
    if (!parent_domain)
    return -ENODEV;
    gpio = devm_kzalloc(dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.gpio_data = match_data;
    gpio.saved = devm_kcalloc(dev, gpio.gpio_data.num, sizeof(*gpio.saved), GFP_KERNEL);
    if (!gpio.saved)
    return -ENOMEM;
    gpio.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gpio.base))
    return PTR_ERR(gpio.base);
    platform_set_drvdata(pdev, gpio);
    gpiochip = devm_kzalloc(dev, sizeof(*gpiochip), GFP_KERNEL);
    if (!gpiochip)
    return -ENOMEM;
    gpiochip.label = DRIVER_NAME;
    gpiochip.parent = dev;
    gpiochip.request = gpiochip_generic_request;
    gpiochip.free = gpiochip_generic_free;
    gpiochip.direction_input = msc313_gpio_direction_input;
    gpiochip.direction_output = msc313_gpio_direction_output;
    gpiochip.get = msc313_gpio_get;
    gpiochip.set = msc313_gpio_set;
    gpiochip.base = -1;
    gpiochip.ngpio = gpio.gpio_data.num;
    gpiochip.names = gpio.gpio_data.names;
    gpioirqchip = &gpiochip.irq;
    gpio_irq_chip_set_chip(gpioirqchip, &msc313_gpio_irqchip);
    gpioirqchip.fwnode = dev_fwnode(dev);
    gpioirqchip.parent_domain = parent_domain;
    gpioirqchip.child_to_parent_hwirq = msc313e_gpio_child_to_parent_hwirq;
    gpioirqchip.populate_parent_alloc_arg = msc313_gpio_populate_parent_fwspec;
    gpioirqchip.handler = handle_bad_irq;
    gpioirqchip.default_type = IRQ_TYPE_NONE;
    return devm_gpiochip_add_data(dev, gpiochip, gpio);
    }
    static const struct of_device_id msc313_gpio_of_match[] = {

    {
    .compatible = "mstar,msc313-gpio",
    .data = &msc313_data,
    },
    {
    .compatible = "sstar,ssd20xd-gpio",
    .data = &ssd20xd_data,
    },

    { }
    };
//
// The GPIO controller loses the state of the registers when the
// SoC goes into suspend to memory mode so we need to save some
// of the register bits before suspending and put it back when resuming
//
#[no_mangle]
unsafe extern "C" fn msc313_gpio_suspend(dev: *mut device) -> c_int {
    static int msc313_gpio_suspend(struct device *dev)
    {
    struct msc313_gpio *gpio = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < gpio.gpio_data.num; i++)
    gpio.saved[i] = readb_relaxed(gpio.base + gpio.gpio_data.offsets[i]) & MSC313_GPIO_BITSTOSAVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_gpio_resume(dev: *mut device) -> c_int {
    static int msc313_gpio_resume(struct device *dev)
    {
    struct msc313_gpio *gpio = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < gpio.gpio_data.num; i++)
    writeb_relaxed(gpio.saved[i], gpio.base + gpio.gpio_data.offsets[i]);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(msc313_gpio_ops, msc313_gpio_suspend, msc313_gpio_resume);
    static struct platform_driver msc313_gpio_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = msc313_gpio_of_match,
    .pm = pm_sleep_ptr(&msc313_gpio_ops),
    },
    .probe = msc313_gpio_probe,
    };
    builtin_platform_driver(msc313_gpio_driver);
