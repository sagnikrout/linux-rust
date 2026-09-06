//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-en7523.c
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

pub const AIROHA_GPIO_MAX: c_int = 32;
//
// struct airoha_gpio_ctrl - Airoha GPIO driver data
// @gen_gc: Associated gpio_generic_chip instance.
// @data: The data register.
// @dir: [0] The direction register for the lower 16 pins.
// [1]: The direction register for the higher 16 pins.
// @output: The output enable register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_gpio_ctrl {
    pub gen_gc: gpio_generic_chip,
    pub data: *mut void __iomem,
    pub dir: [*mut void __iomem; 2],
    pub output: *mut void __iomem,
}

    static int airoha_dir_set(struct gpio_chip *gc, unsigned int gpio,
    int val, int out)
    {
    struct airoha_gpio_ctrl *ctrl = gpiochip_get_data(gc);
    let mut dir: u32 = ioread32(ctrl.dir[gpio / 16]);
    let mut output: u32 = ioread32(ctrl.output);
    let mut mask: u32 = BIT((gpio % 16) * 2);
    if (out) {
    dir |= mask;
    output |= BIT(gpio);
    } else {
    dir &= ~mask;
    output &= ~BIT(gpio);
    }
    iowrite32(dir, ctrl.dir[gpio / 16]);
    if (out)
    gpio_generic_chip_set(&ctrl.gen_gc, gpio, val);
    iowrite32(output, ctrl.output);
    return 0;
    }
    static int airoha_dir_out(struct gpio_chip *gc, unsigned int gpio,
    int val)
    {
    return airoha_dir_set(gc, gpio, val, 1);
    }
#[no_mangle]
unsafe extern "C" fn airoha_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int airoha_dir_in(struct gpio_chip *gc, unsigned int gpio)
    {
    return airoha_dir_set(gc, gpio, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn airoha_get_dir(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int airoha_get_dir(struct gpio_chip *gc, unsigned int gpio)
    {
    struct airoha_gpio_ctrl *ctrl = gpiochip_get_data(gc);
    let mut dir: u32 = ioread32(ctrl.dir[gpio / 16]);
    let mut mask: u32 = BIT((gpio % 16) * 2);
    return (dir & mask) ? GPIO_LINE_DIRECTION_OUT : GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn airoha_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int airoha_gpio_probe(struct platform_device *pdev)
    {
    let mut config: gpio_generic_chip_config = { };
    struct device *dev = &pdev.dev;
    struct airoha_gpio_ctrl *ctrl;
    int err;
    ctrl = devm_kzalloc(dev, sizeof(*ctrl), GFP_KERNEL);
    if (!ctrl)
    return -ENOMEM;
    ctrl.data = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctrl.data))
    return PTR_ERR(ctrl.data);
    ctrl.dir[0] = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(ctrl.dir[0]))
    return PTR_ERR(ctrl.dir[0]);
    ctrl.dir[1] = devm_platform_ioremap_resource(pdev, 2);
    if (IS_ERR(ctrl.dir[1]))
    return PTR_ERR(ctrl.dir[1]);
    ctrl.output = devm_platform_ioremap_resource(pdev, 3);
    if (IS_ERR(ctrl.output))
    return PTR_ERR(ctrl.output);
    config.dev = dev;
    config.sz = 4;
    config.dat = ctrl.data;
    err = gpio_generic_chip_init(&ctrl.gen_gc, &config);
    if (err)
    return dev_err_probe(dev, err, "unable to init generic GPIO");
    ctrl.gen_gc.gc.ngpio = AIROHA_GPIO_MAX;
    ctrl.gen_gc.gc.owner = THIS_MODULE;
    ctrl.gen_gc.gc.direction_output = airoha_dir_out;
    ctrl.gen_gc.gc.direction_input = airoha_dir_in;
    ctrl.gen_gc.gc.get_direction = airoha_get_dir;
    return devm_gpiochip_add_data(dev, &ctrl.gen_gc.gc, ctrl);
    }
    static const struct of_device_id airoha_gpio_of_match[] = {
    { .compatible = "airoha,en7523-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, airoha_gpio_of_match);
    static struct platform_driver airoha_gpio_driver = {
    .driver = {
    .name = "airoha-gpio",
    .of_match_table	= airoha_gpio_of_match,
    },
    .probe = airoha_gpio_probe,
    };
    module_platform_driver(airoha_gpio_driver);
    MODULE_DESCRIPTION("Airoha GPIO support");
    MODULE_AUTHOR("John Crispin <john@phrozen.org>");
    MODULE_LICENSE("GPL v2");
