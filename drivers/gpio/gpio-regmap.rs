//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-regmap.c
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
// regmap based generic GPIO driver
//
// Copyright 2020 Michael Walle <michael@walle.cc>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_regmap {
    pub parent: *mut device,
    pub regmap: *mut regmap,
    pub gpio_chip: gpio_chip,
    pub reg_stride: c_int,
    pub ngpio_per_reg: c_int,
    pub reg_dat_base: c_uint,
    pub reg_set_base: c_uint,
    pub reg_clr_base: c_uint,
    pub reg_dir_in_base: c_uint,
    pub reg_dir_out_base: c_uint,
    pub fixed_direction_mask: *mut c_ulong,
    pub fixed_direction_output: *mut c_ulong,

    pub regmap_irq_line: c_int,
    pub irq_chip_data: *mut regmap_irq_chip_data,

    int (*reg_mask_xlate)(struct gpio_regmap *gpio,
    enum gpio_regmap_operation op,
    unsigned int base, unsigned int offset,
    pub mask): *mut *mut unsigned int reg, unsigned int,
    int (*value_xlate)(struct gpio_regmap *gpio,
    enum gpio_regmap_operation op,
    unsigned int base, unsigned int offset,
    unsigned int reg, unsigned int *mask,
    pub val): *mut c_uint,
    int (*set_config)(struct gpio_regmap *gpio, struct gpio_chip *chip,
    pub config): unsigned int offset, unsigned long,
    pub driver_data: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn gpio_regmap_addr(addr: c_uint) -> c_uint {
    static unsigned int gpio_regmap_addr(unsigned int addr)
    {
    if (addr == GPIO_REGMAP_ADDR_ZERO)
    return 0;
    return addr;
    }
    static int gpio_regmap_simple_xlate(struct gpio_regmap *gpio,
    enum gpio_regmap_operation __maybe_unused op,
    unsigned int base, unsigned int offset,
    unsigned int *reg, unsigned int *mask)
    {
    let mut line: c_uint = offset % gpio.ngpio_per_reg;
    let mut stride: c_uint = offset / gpio.ngpio_per_reg;
// reg = base + stride * gpio->reg_stride;
// mask = BIT(line);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_regmap_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int gpio_regmap_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct gpio_regmap *gpio = gpiochip_get_data(chip);
    unsigned int base, val, reg, mask;
    int ret;
// we might not have an output register if we are input only
    if (gpio.reg_dat_base)
    base = gpio_regmap_addr(gpio.reg_dat_base);
    else
    base = gpio_regmap_addr(gpio.reg_set_base);
    ret = gpio.reg_mask_xlate(gpio, GPIO_REGMAP_GET_OP, base, offset, &reg, &mask);
    if (ret)
    return ret;
// ensure we don't spoil any register cache with pin input values
    if (gpio.reg_dat_base == gpio.reg_set_base)
    ret = regmap_read_bypassed(gpio.regmap, reg, &val);
    else
    ret = regmap_read(gpio.regmap, reg, &val);
    if (ret)
    return ret;
    return !!(val & mask);
    }
    static int gpio_regmap_set(struct gpio_chip *chip, unsigned int offset,
    int val)
    {
    struct gpio_regmap *gpio = gpiochip_get_data(chip);
    let mut base: c_uint = gpio_regmap_addr(gpio.reg_set_base);
    unsigned int reg, mask, mask_val;
    int ret;
    ret = gpio.reg_mask_xlate(gpio, GPIO_REGMAP_SET_OP, base, offset, &reg, &mask);
    if (ret)
    return ret;
    if (val)
    mask_val = mask;
    else
    mask_val = 0;
    if (gpio.value_xlate) {
    ret = gpio.value_xlate(gpio, GPIO_REGMAP_SET_OP, base, offset,
    reg, &mask, &mask_val);
    if (ret)
    return ret;
    }
// ignore input values which shadow the old output value
    if (gpio.reg_dat_base == gpio.reg_set_base)
    ret = regmap_write_bits(gpio.regmap, reg, mask, mask_val);
    else
    ret = regmap_update_bits(gpio.regmap, reg, mask, mask_val);
    return ret;
    }
    static int gpio_regmap_set_with_clear(struct gpio_chip *chip,
    unsigned int offset, int val)
    {
    struct gpio_regmap *gpio = gpiochip_get_data(chip);
    unsigned int base, reg, mask, value = 0;
    int ret;
    if (val)
    base = gpio_regmap_addr(gpio.reg_set_base);
    else
    base = gpio_regmap_addr(gpio.reg_clr_base);
    ret = gpio.reg_mask_xlate(gpio, GPIO_REGMAP_SET_OP, base, offset, &reg, &mask);
    if (ret)
    return ret;
    if (gpio.value_xlate) {
    ret = gpio.value_xlate(gpio, GPIO_REGMAP_SET_OP, base, offset,
    reg, &mask, &value);
    if (ret)
    return ret;
    }
    return regmap_write(gpio.regmap, reg, mask);
    }
    static bool gpio_regmap_fixed_direction(struct gpio_regmap *gpio,
    unsigned int offset)
    {
    if (!gpio.fixed_direction_output)
    return false;
// In this case only some GPIOs are fixed as input/output
    if (gpio.fixed_direction_mask &&
    !test_bit(offset, gpio.fixed_direction_mask))
    return false;
    return true;
    }
    static int gpio_regmap_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct gpio_regmap *gpio = gpiochip_get_data(chip);
    unsigned int base, val, reg, mask;
    int invert, ret;
    if (gpio_regmap_fixed_direction(gpio, offset)) {
    if (test_bit(offset, gpio.fixed_direction_output))
    return GPIO_LINE_DIRECTION_OUT;
    else
    return GPIO_LINE_DIRECTION_IN;
    }
    if (gpio.reg_dat_base && !gpio.reg_set_base)
    return GPIO_LINE_DIRECTION_IN;
    if (gpio.reg_set_base && !gpio.reg_dat_base)
    return GPIO_LINE_DIRECTION_OUT;
    if (gpio.reg_dir_out_base) {
    base = gpio_regmap_addr(gpio.reg_dir_out_base);
    invert = 0;
    } else if (gpio.reg_dir_in_base) {
    base = gpio_regmap_addr(gpio.reg_dir_in_base);
    invert = 1;
    } else {
    return -ENOTSUPP;
    }
    ret = gpio.reg_mask_xlate(gpio, GPIO_REGMAP_GET_DIR_OP, base, offset, &reg, &mask);
    if (ret)
    return ret;
    ret = regmap_read(gpio.regmap, reg, &val);
    if (ret)
    return ret;
    if (!!(val & mask) ^ invert)
    return GPIO_LINE_DIRECTION_OUT;
    else
    return GPIO_LINE_DIRECTION_IN;
    }
    static int gpio_regmap_try_direction_fixed(struct gpio_regmap *gpio,
    unsigned int offset, bool output)
    {
    if (test_bit(offset, gpio.fixed_direction_output)) {
    if (output)
    return 0;
    else
    return -EINVAL;
    } else {
    if (output)
    return -EINVAL;
    else
    return 0;
    }
    }
    static int gpio_regmap_set_direction(struct gpio_chip *chip,
    unsigned int offset, bool output)
    {
    struct gpio_regmap *gpio = gpiochip_get_data(chip);
    unsigned int base, val, reg, mask;
    int invert, ret;
//
// If the direction is fixed, only accept the fixed
// direction in this call.
//
    if (gpio_regmap_fixed_direction(gpio, offset))
    return gpio_regmap_try_direction_fixed(gpio, offset, output);
    if (gpio.reg_dir_out_base) {
    base = gpio_regmap_addr(gpio.reg_dir_out_base);
    invert = 0;
    } else if (gpio.reg_dir_in_base) {
    base = gpio_regmap_addr(gpio.reg_dir_in_base);
    invert = 1;
    } else {
    return -ENOTSUPP;
    }
    ret = gpio.reg_mask_xlate(gpio, GPIO_REGMAP_SET_DIR_OP, base, offset, &reg, &mask);
    if (ret)
    return ret;
    if (invert)
    val = output ? 0 : mask;
    else
    val = output ? mask : 0;
    if (gpio.value_xlate) {
    ret = gpio.value_xlate(gpio, GPIO_REGMAP_SET_DIR_OP, base, offset,
    reg, &mask, &val);
    if (ret)
    return ret;
    }
    return regmap_update_bits(gpio.regmap, reg, mask, val);
    }
    static int gpio_regmap_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    return gpio_regmap_set_direction(chip, offset, false);
    }
    static int gpio_regmap_direction_output(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    struct gpio_regmap *gpio = gpiochip_get_data(chip);
    int ret;
//
// First check if this is gonna work on a fixed direction line,
// if it doesn't (i.e. this is a fixed input line), then do not
// attempt to set the output value either and just bail out.
//
    if (gpio_regmap_fixed_direction(gpio, offset)) {
    ret = gpio_regmap_try_direction_fixed(gpio, offset, true);
    if (ret)
    return ret;
    }
    gpio_regmap_set(chip, offset, value);
    return gpio_regmap_set_direction(chip, offset, true);
    }
    static int gpio_regmap_set_config(struct gpio_chip *chip,
    unsigned int offset,
    unsigned long cfg)
    {
    struct gpio_regmap *gpio = gpiochip_get_data(chip);
    return gpio.set_config(gpio, chip, offset, cfg);
    }
#[no_mangle]
pub unsafe extern "C" fn gpio_regmap_reqres_irq(gpio: *mut gpio_regmap, offset: c_uint) -> c_int {
    int gpio_regmap_reqres_irq(struct gpio_regmap *gpio, unsigned int offset)
    {
    return gpiochip_reqres_irq(&gpio.gpio_chip, offset);
    }
    EXPORT_SYMBOL_GPL(gpio_regmap_reqres_irq);
#[no_mangle]
pub unsafe extern "C" fn gpio_regmap_relres_irq(gpio: *mut gpio_regmap, offset: c_uint) {
    void gpio_regmap_relres_irq(struct gpio_regmap *gpio, unsigned int offset)
    {
    gpiochip_relres_irq(&gpio.gpio_chip, offset);
    }
    EXPORT_SYMBOL_GPL(gpio_regmap_relres_irq);
#[no_mangle]
unsafe extern "C" fn gpio_regmap_irq_reqres(irq_drv_data: *mut c_void, hwirq: irq_hw_number_t) -> int __maybe_unused {
    static int __maybe_unused gpio_regmap_irq_reqres(void *irq_drv_data, irq_hw_number_t hwirq)
    {
    return gpio_regmap_reqres_irq(irq_drv_data, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn gpio_regmap_irq_relres(irq_drv_data: *mut c_void, hwirq: irq_hw_number_t) -> void __maybe_unused {
    static void __maybe_unused gpio_regmap_irq_relres(void *irq_drv_data, irq_hw_number_t hwirq)
    {
    gpio_regmap_relres_irq(irq_drv_data, hwirq);
    }
    void *gpio_regmap_get_drvdata(struct gpio_regmap *gpio)
    {
    return gpio.driver_data;
    }
    EXPORT_SYMBOL_GPL(gpio_regmap_get_drvdata);
#[no_mangle]
pub unsafe extern "C" fn gpio_regmap_enable_irq(gpio: *mut gpio_regmap, hwirq: irq_hw_number_t) {
    void gpio_regmap_enable_irq(struct gpio_regmap *gpio, irq_hw_number_t hwirq)
    {
    gpiochip_enable_irq(&gpio.gpio_chip, hwirq);
    }
    EXPORT_SYMBOL_GPL(gpio_regmap_enable_irq);
#[no_mangle]
pub unsafe extern "C" fn gpio_regmap_disable_irq(gpio: *mut gpio_regmap, hwirq: irq_hw_number_t) {
    void gpio_regmap_disable_irq(struct gpio_regmap *gpio, irq_hw_number_t hwirq)
    {
    gpiochip_disable_irq(&gpio.gpio_chip, hwirq);
    }
    EXPORT_SYMBOL_GPL(gpio_regmap_disable_irq);
//
// gpio_regmap_register() - Register a generic regmap GPIO controller
// @config: configuration for gpio_regmap
//
// Return: A pointer to the registered gpio_regmap or ERR_PTR error value.
//
    struct gpio_regmap *gpio_regmap_register(const struct gpio_regmap_config *config)
    {
    struct irq_domain *irq_domain;
    struct gpio_regmap *gpio;
    struct gpio_chip *chip;
    int ret;
    if (!config.parent)
    return ERR_PTR(-EINVAL);
// we need at least one
    if (!config.reg_dat_base && !config.reg_set_base)
    return ERR_PTR(-EINVAL);
// if we have a direction register we need both input and output
    if ((config.reg_dir_out_base || config.reg_dir_in_base) &&
    (!config.reg_dat_base || !config.reg_set_base))
    return ERR_PTR(-EINVAL);
// we don't support having both registers simultaneously for now
    if (config.reg_dir_out_base && config.reg_dir_in_base)
    return ERR_PTR(-EINVAL);
    gpio = kzalloc_obj(*gpio);
    if (!gpio)
    return ERR_PTR(-ENOMEM);
    gpio.parent = config.parent;
    gpio.driver_data = config.drvdata;
    gpio.regmap = config.regmap;
    gpio.reg_dat_base = config.reg_dat_base;
    gpio.reg_set_base = config.reg_set_base;
    gpio.reg_clr_base = config.reg_clr_base;
    gpio.reg_dir_in_base = config.reg_dir_in_base;
    gpio.reg_dir_out_base = config.reg_dir_out_base;
    chip = &gpio.gpio_chip;
    chip.parent = config.parent;
    chip.fwnode = config.fwnode;
    chip.base = -1;
    chip.names = config.names;
    chip.label = config.label ?: dev_name(config.parent);
    chip.can_sleep = regmap_might_sleep(config.regmap);
    chip.init_valid_mask = config.init_valid_mask;
    chip.request = gpiochip_generic_request;
    chip.free = gpiochip_generic_free;
    chip.get = gpio_regmap_get;
    if (gpio.reg_set_base && gpio.reg_clr_base)
    chip.set = gpio_regmap_set_with_clear;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: gpio->reg_set_base) -> else {
    else if (gpio.reg_set_base)
    chip.set = gpio_regmap_set;
    chip.get_direction = gpio_regmap_get_direction;
    if (gpio.reg_dir_in_base || gpio.reg_dir_out_base) {
    chip.direction_input = gpio_regmap_direction_input;
    chip.direction_output = gpio_regmap_direction_output;
    }
    chip.ngpio = config.ngpio;
    if (!chip.ngpio) {
    ret = gpiochip_get_ngpios(chip, chip.parent);
    if (ret)
    goto err_free_gpio;
    }
    if (config.fixed_direction_mask) {
    gpio.fixed_direction_mask = bitmap_alloc(chip.ngpio,
    GFP_KERNEL);
    if (!gpio.fixed_direction_mask) {
    ret = -ENOMEM;
    goto err_free_gpio;
    }
    bitmap_copy(gpio.fixed_direction_mask,
    config.fixed_direction_mask, chip.ngpio);
    }
    if (config.fixed_direction_output) {
    gpio.fixed_direction_output = bitmap_alloc(chip.ngpio,
    GFP_KERNEL);
    if (!gpio.fixed_direction_output) {
    ret = -ENOMEM;
    goto err_free_bitmap_dirmask;
    }
    bitmap_copy(gpio.fixed_direction_output,
    config.fixed_direction_output, chip.ngpio);
    }
// if not set, assume there is only one register
    gpio.ngpio_per_reg = config.ngpio_per_reg;
    if (!gpio.ngpio_per_reg)
    gpio.ngpio_per_reg = config.ngpio;
// if not set, assume they are consecutive
    gpio.reg_stride = config.reg_stride;
    if (!gpio.reg_stride)
    gpio.reg_stride = 1;
    gpio.reg_mask_xlate = config.reg_mask_xlate;
    if (!gpio.reg_mask_xlate)
    gpio.reg_mask_xlate = gpio_regmap_simple_xlate;
    gpio.value_xlate = config.value_xlate;
    if (config.set_config) {
    gpio.set_config = config.set_config;
    chip.set_config = gpio_regmap_set_config;
    }
    ret = gpiochip_add_data(chip, gpio);
    if (ret < 0)
    goto err_free_bitmap_output;

    if (config.regmap_irq_chip) {
    gpio.regmap_irq_line = config.regmap_irq_line;
    config.regmap_irq_chip.irq_reqres = gpio_regmap_irq_reqres;
    config.regmap_irq_chip.irq_relres = gpio_regmap_irq_relres;
    config.regmap_irq_chip.irq_drv_data = gpio;
    ret = regmap_add_irq_chip_fwnode(dev_fwnode(config.parent), config.regmap,
    config.regmap_irq_line, config.regmap_irq_flags,
    0, config.regmap_irq_chip, &gpio.irq_chip_data);
    if (ret)
    goto err_remove_gpiochip;
    irq_domain = regmap_irq_get_domain(gpio.irq_chip_data);
    } else

    irq_domain = config.irq_domain;
    if (irq_domain) {
    ret = gpiochip_irqchip_add_domain(chip, irq_domain);
    if (ret)
    goto err_remove_gpiochip;
    }
    return gpio;
    err_remove_gpiochip:
    gpiochip_remove(chip);
    err_free_bitmap_output:
    bitmap_free(gpio.fixed_direction_output);
    err_free_bitmap_dirmask:
    bitmap_free(gpio.fixed_direction_mask);
    err_free_gpio:
    kfree(gpio);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(gpio_regmap_register);
//
// gpio_regmap_unregister() - Unregister a generic regmap GPIO controller
// @gpio: gpio_regmap device to unregister
//
#[no_mangle]
pub unsafe extern "C" fn gpio_regmap_unregister(gpio: *mut gpio_regmap) {
    void gpio_regmap_unregister(struct gpio_regmap *gpio)
    {

    if (gpio.irq_chip_data)
    regmap_del_irq_chip(gpio.regmap_irq_line, gpio.irq_chip_data);

    gpiochip_remove(&gpio.gpio_chip);
    bitmap_free(gpio.fixed_direction_output);
    bitmap_free(gpio.fixed_direction_mask);
    kfree(gpio);
    }
    EXPORT_SYMBOL_GPL(gpio_regmap_unregister);
#[no_mangle]
unsafe extern "C" fn devm_gpio_regmap_unregister(res: *mut c_void) {
    static void devm_gpio_regmap_unregister(void *res)
    {
    gpio_regmap_unregister(res);
    }
//
// devm_gpio_regmap_register() - resource managed gpio_regmap_register()
// @dev: device that is registering this GPIO device
// @config: configuration for gpio_regmap
//
// Managed gpio_regmap_register(). For generic regmap GPIO device registered by
// this function, gpio_regmap_unregister() is automatically called on driver
// detach. See gpio_regmap_register() for more information.
//
// Return: A pointer to the registered gpio_regmap or ERR_PTR error value.
//
    struct gpio_regmap *devm_gpio_regmap_register(struct device *dev,
    const struct gpio_regmap_config *config)
    {
    struct gpio_regmap *gpio;
    int ret;
    gpio = gpio_regmap_register(config);
    if (IS_ERR(gpio))
    return gpio;
    ret = devm_add_action_or_reset(dev, devm_gpio_regmap_unregister, gpio);
    if (ret)
    return ERR_PTR(ret);
    return gpio;
    }
    EXPORT_SYMBOL_GPL(devm_gpio_regmap_register);
    MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
    MODULE_DESCRIPTION("GPIO generic regmap driver core");
    MODULE_LICENSE("GPL");
