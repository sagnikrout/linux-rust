//! Automatically rewritten from C to Rust
//! Source: drivers/soc/renesas/pwc-rzv2m.c
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
// Copyright (C) 2023 Renesas Electronics Corporation
//

pub const PWC_PWCRST: c_uint = 0x00;
pub const PWC_PWCCKEN: c_uint = 0x04;
pub const PWC_PWCCTL: c_uint = 0x50;
pub const PWC_GPIO: c_uint = 0x80;
pub const PWC_PWCRST_RSTSOFTAX: c_uint = 0x1;
pub const PWC_PWCCKEN_ENGCKMAIN: c_uint = 0x1;
pub const PWC_PWCCTL_PWOFF: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2m_pwc_priv {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub gp: gpio_chip,
    pub 2): DECLARE_BITMAP(ch_en_bits,,
}

    static int rzv2m_pwc_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct rzv2m_pwc_priv *priv = gpiochip_get_data(chip);
    u32 reg;
// BIT 16 enables write to BIT 0, and BIT 17 enables write to BIT 1
    reg = BIT(offset + 16);
    if (value)
    reg |= BIT(offset);
    writel(reg, priv.base + PWC_GPIO);
    assign_bit(offset, priv.ch_en_bits, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_pwc_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int rzv2m_pwc_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct rzv2m_pwc_priv *priv = gpiochip_get_data(chip);
    return test_bit(offset, priv.ch_en_bits);
    }
    static int rzv2m_pwc_gpio_direction_output(struct gpio_chip *gc,
    unsigned int nr, int value)
    {
    if (nr > 1)
    return -EINVAL;
    rzv2m_pwc_gpio_set(gc, nr, value);
    return 0;
    }
    static const struct gpio_chip rzv2m_pwc_gc = {
    .label = "gpio_rzv2m_pwc",
    .owner = THIS_MODULE,
    .get = rzv2m_pwc_gpio_get,
    .set = rzv2m_pwc_gpio_set,
    .direction_output = rzv2m_pwc_gpio_direction_output,
    .can_sleep = false,
    .ngpio = 2,
    .base = -1,
    };
#[no_mangle]
unsafe extern "C" fn rzv2m_pwc_poweroff(data: *mut sys_off_data) -> c_int {
    static int rzv2m_pwc_poweroff(struct sys_off_data *data)
    {
    struct rzv2m_pwc_priv *priv = data.cb_data;
    writel(PWC_PWCRST_RSTSOFTAX, priv.base + PWC_PWCRST);
    writel(PWC_PWCCKEN_ENGCKMAIN, priv.base + PWC_PWCCKEN);
    writel(PWC_PWCCTL_PWOFF, priv.base + PWC_PWCCTL);
    mdelay(150);
    dev_err(priv.dev, "Failed to power off the system");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn rzv2m_pwc_probe(pdev: *mut platform_device) -> c_int {
    static int rzv2m_pwc_probe(struct platform_device *pdev)
    {
    struct rzv2m_pwc_priv *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
//
// The register used by this driver cannot be read, therefore set the
// outputs to their default values and initialize priv->ch_en_bits
// accordingly. BIT 16 enables write to BIT 0, BIT 17 enables write to
// BIT 1, and the default value of both BIT 0 and BIT 1 is 0.
//
    writel(BIT(17) | BIT(16), priv.base + PWC_GPIO);
    bitmap_zero(priv.ch_en_bits, 2);
    priv.gp = rzv2m_pwc_gc;
    priv.gp.parent = pdev.dev.parent;
    priv.gp.fwnode = dev_fwnode(&pdev.dev);
    ret = devm_gpiochip_add_data(&pdev.dev, &priv.gp, priv);
    if (ret)
    return ret;
    if (device_property_read_bool(&pdev.dev, "renesas,rzv2m-pwc-power"))
    ret = devm_register_power_off_handler(&pdev.dev,
    rzv2m_pwc_poweroff, priv);
    return ret;
    }
    static const struct of_device_id rzv2m_pwc_of_match[] = {
    { .compatible = "renesas,rzv2m-pwc" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzv2m_pwc_of_match);
    static struct platform_driver rzv2m_pwc_driver = {
    .probe = rzv2m_pwc_probe,
    .driver = {
    .name = "rzv2m_pwc",
    .of_match_table = rzv2m_pwc_of_match,
    },
    };
    module_platform_driver(rzv2m_pwc_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Fabrizio Castro <castro.fabrizio.jz@renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/V2M PWC driver");
