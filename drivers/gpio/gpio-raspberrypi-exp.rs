//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-raspberrypi-exp.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Raspberry Pi 3 expander GPIO driver
//
// Uses the firmware mailbox service to communicate with the
// GPIO expander on the VPU.
//
// Copyright (C) 2017 Raspberry Pi Trading Ltd.
//

pub const NUM_GPIO: c_int = 8;
pub const RPI_EXP_GPIO_BASE: c_int = 128;
pub const RPI_EXP_GPIO_DIR_IN: c_int = 0;
pub const RPI_EXP_GPIO_DIR_OUT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_exp_gpio {
    pub gc: gpio_chip,
    pub fw: *mut rpi_firmware,
}

// VC4 firmware mailbox interface data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_set_config {
    pub gpio: u32,
    pub direction: u32,
    pub polarity: u32,
    pub term_en: u32,
    pub term_pull_up: u32,
    pub state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_get_config {
    pub gpio: u32,
    pub direction: u32,
    pub polarity: u32,
    pub term_en: u32,
    pub term_pull_up: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_get_set_state {
    pub gpio: u32,
    pub state: u32,
}

#[no_mangle]
unsafe extern "C" fn rpi_exp_gpio_get_polarity(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int rpi_exp_gpio_get_polarity(struct gpio_chip *gc, unsigned int off)
    {
    struct rpi_exp_gpio *gpio;
    struct gpio_get_config get;
    int ret;
    gpio = gpiochip_get_data(gc);
    get.gpio = off + RPI_EXP_GPIO_BASE;	/* GPIO to update */
    ret = rpi_firmware_property(gpio.fw, RPI_FIRMWARE_GET_GPIO_CONFIG,
    &get, sizeof(get));
    if (ret || get.gpio != 0) {
    dev_err(gc.parent, "Failed to get GPIO %u config (%d %x)\n",
    off, ret, get.gpio);
    return ret ? ret : -EIO;
    }
    return get.polarity;
    }
#[no_mangle]
unsafe extern "C" fn rpi_exp_gpio_dir_in(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int rpi_exp_gpio_dir_in(struct gpio_chip *gc, unsigned int off)
    {
    struct rpi_exp_gpio *gpio;
    struct gpio_set_config set_in;
    int ret;
    gpio = gpiochip_get_data(gc);
    set_in.gpio = off + RPI_EXP_GPIO_BASE;	/* GPIO to update */
    set_in.direction = RPI_EXP_GPIO_DIR_IN;
    set_in.term_en = 0;		/* termination disabled */
    set_in.term_pull_up = 0;	/* n/a as termination disabled */
    set_in.state = 0;		/* n/a as configured as an input */
    ret = rpi_exp_gpio_get_polarity(gc, off);
    if (ret < 0)
    return ret;
    set_in.polarity = ret;		/* Retain existing setting */
    ret = rpi_firmware_property(gpio.fw, RPI_FIRMWARE_SET_GPIO_CONFIG,
    &set_in, sizeof(set_in));
    if (ret || set_in.gpio != 0) {
    dev_err(gc.parent, "Failed to set GPIO %u to input (%d %x)\n",
    off, ret, set_in.gpio);
    return ret ? ret : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_exp_gpio_dir_out(gc: *mut gpio_chip, off: c_uint, val: c_int) -> c_int {
    static int rpi_exp_gpio_dir_out(struct gpio_chip *gc, unsigned int off, int val)
    {
    struct rpi_exp_gpio *gpio;
    struct gpio_set_config set_out;
    int ret;
    gpio = gpiochip_get_data(gc);
    set_out.gpio = off + RPI_EXP_GPIO_BASE;	/* GPIO to update */
    set_out.direction = RPI_EXP_GPIO_DIR_OUT;
    set_out.term_en = 0;		/* n/a as an output */
    set_out.term_pull_up = 0;	/* n/a as termination disabled */
    set_out.state = val;		/* Output state */
    ret = rpi_exp_gpio_get_polarity(gc, off);
    if (ret < 0)
    return ret;
    set_out.polarity = ret;		/* Retain existing setting */
    ret = rpi_firmware_property(gpio.fw, RPI_FIRMWARE_SET_GPIO_CONFIG,
    &set_out, sizeof(set_out));
    if (ret || set_out.gpio != 0) {
    dev_err(gc.parent, "Failed to set GPIO %u to output (%d %x)\n",
    off, ret, set_out.gpio);
    return ret ? ret : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_exp_gpio_get_direction(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int rpi_exp_gpio_get_direction(struct gpio_chip *gc, unsigned int off)
    {
    struct rpi_exp_gpio *gpio;
    struct gpio_get_config get;
    int ret;
    gpio = gpiochip_get_data(gc);
    get.gpio = off + RPI_EXP_GPIO_BASE;	/* GPIO to update */
    ret = rpi_firmware_property(gpio.fw, RPI_FIRMWARE_GET_GPIO_CONFIG,
    &get, sizeof(get));
    if (ret || get.gpio != 0) {
    dev_err(gc.parent,
    "Failed to get GPIO %u config (%d %x)\n", off, ret,
    get.gpio);
    return ret ? ret : -EIO;
    }
    if (get.direction)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn rpi_exp_gpio_get(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int rpi_exp_gpio_get(struct gpio_chip *gc, unsigned int off)
    {
    struct rpi_exp_gpio *gpio;
    struct gpio_get_set_state get;
    int ret;
    gpio = gpiochip_get_data(gc);
    get.gpio = off + RPI_EXP_GPIO_BASE;	/* GPIO to update */
    get.state = 0;		/* storage for returned value */
    ret = rpi_firmware_property(gpio.fw, RPI_FIRMWARE_GET_GPIO_STATE,
    &get, sizeof(get));
    if (ret || get.gpio != 0) {
    dev_err(gc.parent,
    "Failed to get GPIO %u state (%d %x)\n", off, ret,
    get.gpio);
    return ret ? ret : -EIO;
    }
    return !!get.state;
    }
#[no_mangle]
unsafe extern "C" fn rpi_exp_gpio_set(gc: *mut gpio_chip, off: c_uint, val: c_int) -> c_int {
    static int rpi_exp_gpio_set(struct gpio_chip *gc, unsigned int off, int val)
    {
    struct rpi_exp_gpio *gpio;
    struct gpio_get_set_state set;
    int ret;
    gpio = gpiochip_get_data(gc);
    set.gpio = off + RPI_EXP_GPIO_BASE;	/* GPIO to update */
    set.state = val;	/* Output state */
    ret = rpi_firmware_property(gpio.fw, RPI_FIRMWARE_SET_GPIO_STATE,
    &set, sizeof(set));
    if (ret || set.gpio != 0) {
    dev_err(gc.parent,
    "Failed to set GPIO %u state (%d %x)\n", off, ret,
    set.gpio);
    return ret ? ret : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_exp_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int rpi_exp_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *fw_node;
    struct rpi_firmware *fw;
    struct rpi_exp_gpio *rpi_gpio;
    fw_node = of_get_parent(np);
    if (!fw_node) {
    dev_err(dev, "Missing firmware node\n");
    return -ENOENT;
    }
    fw = devm_rpi_firmware_get(&pdev.dev, fw_node);
    of_node_put(fw_node);
    if (!fw)
    return -EPROBE_DEFER;
    rpi_gpio = devm_kzalloc(dev, sizeof(*rpi_gpio), GFP_KERNEL);
    if (!rpi_gpio)
    return -ENOMEM;
    rpi_gpio.fw = fw;
    rpi_gpio.gc.parent = dev;
    rpi_gpio.gc.label = MODULE_NAME;
    rpi_gpio.gc.owner = THIS_MODULE;
    rpi_gpio.gc.base = -1;
    rpi_gpio.gc.ngpio = NUM_GPIO;
    rpi_gpio.gc.direction_input = rpi_exp_gpio_dir_in;
    rpi_gpio.gc.direction_output = rpi_exp_gpio_dir_out;
    rpi_gpio.gc.get_direction = rpi_exp_gpio_get_direction;
    rpi_gpio.gc.get = rpi_exp_gpio_get;
    rpi_gpio.gc.set = rpi_exp_gpio_set;
    rpi_gpio.gc.can_sleep = true;
    return devm_gpiochip_add_data(dev, &rpi_gpio.gc, rpi_gpio);
    }
    static const struct of_device_id rpi_exp_gpio_ids[] = {
    { .compatible = "raspberrypi,firmware-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rpi_exp_gpio_ids);
    static struct platform_driver rpi_exp_gpio_driver = {
    .driver	= {
    .name		= MODULE_NAME,
    .of_match_table	= rpi_exp_gpio_ids,
    },
    .probe	= rpi_exp_gpio_probe,
    };
    module_platform_driver(rpi_exp_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Dave Stevenson <dave.stevenson@raspberrypi.org>");
    MODULE_DESCRIPTION("Raspberry Pi 3 expander GPIO driver");
    MODULE_ALIAS("platform:rpi-exp-gpio");
