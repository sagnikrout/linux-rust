//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-line-mux.c
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
// GPIO line mux which acts as virtual gpiochip and provides a 1-to-many
// mapping between virtual GPIOs and a real GPIO + multiplexer.
//
// Copyright (c) 2025 Jonas Jelonek <jelonek.jonas@gmail.com>
//

pub const MUX_SELECT_DELAY_US: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_lmux {
    pub gc: gpio_chip,
    pub mux: *mut mux_control,
    pub muxed_gpio: *mut gpio_desc,
    pub num_gpio_mux_states: u32,
    pub __counted_by(num_gpio_mux_states): unsigned int gpio_mux_states[],
}

#[no_mangle]
unsafe extern "C" fn gpio_lmux_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int gpio_lmux_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct gpio_lmux *glm = gpiochip_get_data(gc);
    int ret;
    ret = mux_control_select_delay(glm.mux, glm.gpio_mux_states[offset],
    MUX_SELECT_DELAY_US);
    if (ret < 0)
    return ret;
    ret = gpiod_get_raw_value_cansleep(glm.muxed_gpio);
    mux_control_deselect(glm.mux);
    return ret;
    }
    static int gpio_lmux_gpio_get_direction(struct gpio_chip *gc,
    unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn gpio_lmux_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_lmux_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gpio_lmux *glm;
    unsigned int ngpio;
    size_t size;
    int ret;
    ngpio = device_property_count_u32(dev, "gpio-line-mux-states");
    if (!ngpio)
    return -EINVAL;
    size = struct_size(glm, gpio_mux_states, ngpio);
    glm = devm_kzalloc(dev, size, GFP_KERNEL);
    if (!glm)
    return -ENOMEM;
    glm.gc.base = -1;
    glm.gc.can_sleep = true;
    glm.gc.fwnode = dev_fwnode(dev);
    glm.gc.label = dev_name(dev);
    glm.gc.ngpio = ngpio;
    glm.gc.owner = THIS_MODULE;
    glm.gc.parent = dev;
    glm.gc.get = gpio_lmux_gpio_get;
    glm.gc.get_direction = gpio_lmux_gpio_get_direction;
    glm.mux = devm_mux_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(glm.mux))
    return dev_err_probe(dev, PTR_ERR(glm.mux),
    "could not get mux controller\n");
    glm.muxed_gpio = devm_gpiod_get(dev, "muxed", GPIOD_IN);
    if (IS_ERR(glm.muxed_gpio))
    return dev_err_probe(dev, PTR_ERR(glm.muxed_gpio),
    "could not get muxed-gpio\n");
    glm.num_gpio_mux_states = ngpio;
    ret = device_property_read_u32_array(dev, "gpio-line-mux-states",
    &glm.gpio_mux_states[0], ngpio);
    if (ret)
    return dev_err_probe(dev, ret, "could not get mux states\n");
    ret = devm_gpiochip_add_data(dev, &glm.gc, glm);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add gpiochip\n");
    return 0;
    }
    static const struct of_device_id gpio_lmux_of_match[] = {
    { .compatible = "gpio-line-mux" },
    { }
    };
    MODULE_DEVICE_TABLE(of, gpio_lmux_of_match);
    static struct platform_driver gpio_lmux_driver = {
    .driver = {
    .name = "gpio-line-mux",
    .of_match_table = gpio_lmux_of_match,
    },
    .probe = gpio_lmux_probe,
    };
    module_platform_driver(gpio_lmux_driver);
    MODULE_AUTHOR("Jonas Jelonek <jelonek.jonas@gmail.com>");
    MODULE_DESCRIPTION("GPIO line mux driver");
    MODULE_LICENSE("GPL");
