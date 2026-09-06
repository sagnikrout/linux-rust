//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/starfive/pinctrl-starfive-jh7110-aon.c
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
// Pinctrl / GPIO driver for StarFive JH7110 SoC aon controller
//
// Copyright (C) 2022 StarFive Technology Co., Ltd.
//

pub const JH7110_AON_NGPIO: c_int = 4;
pub const JH7110_AON_REGS_NUM: c_int = 37;
// registers
pub const JH7110_AON_DOEN: c_uint = 0x0;
pub const JH7110_AON_DOUT: c_uint = 0x4;
pub const JH7110_AON_GPI: c_uint = 0x8;
pub const JH7110_AON_GPIOIN: c_uint = 0x2c;
pub const JH7110_AON_GPIOEN: c_uint = 0xc;
pub const JH7110_AON_GPIOIS: c_uint = 0x10;
pub const JH7110_AON_GPIOIC: c_uint = 0x14;
pub const JH7110_AON_GPIOIBE: c_uint = 0x18;
pub const JH7110_AON_GPIOIEV: c_uint = 0x1c;
pub const JH7110_AON_GPIOIE: c_uint = 0x20;
pub const JH7110_AON_GPIORIS: c_uint = 0x28;
pub const JH7110_AON_GPIOMIS: c_uint = 0x28;
pub const JH7110_AON_GPO_PDA_0_5_CFG: c_uint = 0x30;
    static const struct pinctrl_pin_desc jh7110_aon_pins[] = {
    PINCTRL_PIN(PAD_TESTEN,		"TESTEN"),
    PINCTRL_PIN(PAD_RGPIO0,		"RGPIO0"),
    PINCTRL_PIN(PAD_RGPIO1,		"RGPIO1"),
    PINCTRL_PIN(PAD_RGPIO2,		"RGPIO2"),
    PINCTRL_PIN(PAD_RGPIO3,		"RGPIO3"),
    PINCTRL_PIN(PAD_RSTN,		"RSTN"),
    PINCTRL_PIN(PAD_GMAC0_MDC,	"GMAC0_MDC"),
    PINCTRL_PIN(PAD_GMAC0_MDIO,	"GMAC0_MDIO"),
    PINCTRL_PIN(PAD_GMAC0_RXD0,	"GMAC0_RXD0"),
    PINCTRL_PIN(PAD_GMAC0_RXD1,	"GMAC0_RXD1"),
    PINCTRL_PIN(PAD_GMAC0_RXD2,	"GMAC0_RXD2"),
    PINCTRL_PIN(PAD_GMAC0_RXD3,	"GMAC0_RXD3"),
    PINCTRL_PIN(PAD_GMAC0_RXDV,	"GMAC0_RXDV"),
    PINCTRL_PIN(PAD_GMAC0_RXC,	"GMAC0_RXC"),
    PINCTRL_PIN(PAD_GMAC0_TXD0,	"GMAC0_TXD0"),
    PINCTRL_PIN(PAD_GMAC0_TXD1,	"GMAC0_TXD1"),
    PINCTRL_PIN(PAD_GMAC0_TXD2,	"GMAC0_TXD2"),
    PINCTRL_PIN(PAD_GMAC0_TXD3,	"GMAC0_TXD3"),
    PINCTRL_PIN(PAD_GMAC0_TXEN,	"GMAC0_TXEN"),
    PINCTRL_PIN(PAD_GMAC0_TXC,	"GMAC0_TXC"),
    };
    static int jh7110_aon_set_one_pin_mux(struct jh7110_pinctrl *sfp,
    unsigned int pin,
    unsigned int din, u32 dout,
    u32 doen, u32 func)
    {
    if (pin < sfp.gc.ngpio && func == 0)
    jh7110_set_gpiomux(sfp, pin, din, dout, doen);
    return 0;
    }
    static int jh7110_aon_get_padcfg_base(struct jh7110_pinctrl *sfp,
    unsigned int pin)
    {
    if (pin < PAD_GMAC0_MDC)
    return JH7110_AON_GPO_PDA_0_5_CFG;
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_aon_irq_handler(desc: *mut irq_desc) {
    static void jh7110_aon_irq_handler(struct irq_desc *desc)
    {
    struct jh7110_pinctrl *sfp = jh7110_from_irq_desc(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned long mis;
    unsigned int pin;
    chained_irq_enter(chip, desc);
    mis = readl_relaxed(sfp.base + JH7110_AON_GPIOMIS);
    for_each_set_bit(pin, &mis, JH7110_AON_NGPIO)
    generic_handle_domain_irq(sfp.gc.irq.domain, pin);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn jh7110_aon_init_hw(gc: *mut gpio_chip) -> c_int {
    static int jh7110_aon_init_hw(struct gpio_chip *gc)
    {
    struct jh7110_pinctrl *sfp = container_of(gc,
    struct jh7110_pinctrl, gc);
// mask all GPIO interrupts
    writel_relaxed(0, sfp.base + JH7110_AON_GPIOIE);
// clear edge interrupt flags
    writel_relaxed(0, sfp.base + JH7110_AON_GPIOIC);
    writel_relaxed(0x0f, sfp.base + JH7110_AON_GPIOIC);
// enable GPIO interrupts
    writel_relaxed(1, sfp.base + JH7110_AON_GPIOEN);
    return 0;
    }
    static const struct jh7110_gpio_irq_reg jh7110_aon_irq_reg = {
    .is_reg_base	= JH7110_AON_GPIOIS,
    .ic_reg_base	= JH7110_AON_GPIOIC,
    .ibe_reg_base	= JH7110_AON_GPIOIBE,
    .iev_reg_base	= JH7110_AON_GPIOIEV,
    .ie_reg_base	= JH7110_AON_GPIOIE,
    .ris_reg_base	= JH7110_AON_GPIORIS,
    .mis_reg_base	= JH7110_AON_GPIOMIS,
    };
    static const struct jh7110_pinctrl_soc_info jh7110_aon_pinctrl_info = {
    .pins		= jh7110_aon_pins,
    .npins		= ARRAY_SIZE(jh7110_aon_pins),
    .ngpios		= JH7110_AON_NGPIO,
    .dout_reg_base	= JH7110_AON_DOUT,
    .dout_mask	= GENMASK(3, 0),
    .doen_reg_base	= JH7110_AON_DOEN,
    .doen_mask	= GENMASK(2, 0),
    .gpi_reg_base	= JH7110_AON_GPI,
    .gpi_mask	= GENMASK(3, 0),
    .gpioin_reg_base	   = JH7110_AON_GPIOIN,
    .irq_reg		   = &jh7110_aon_irq_reg,
    .nsaved_regs		   = JH7110_AON_REGS_NUM,
    .jh7110_set_one_pin_mux  = jh7110_aon_set_one_pin_mux,
    .jh7110_get_padcfg_base  = jh7110_aon_get_padcfg_base,
    .jh7110_gpio_irq_handler = jh7110_aon_irq_handler,
    .jh7110_gpio_init_hw	 = jh7110_aon_init_hw,
    };
    static const struct of_device_id jh7110_aon_pinctrl_of_match[] = {
    {
    .compatible = "starfive,jh7110-aon-pinctrl",
    .data = &jh7110_aon_pinctrl_info,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, jh7110_aon_pinctrl_of_match);
    static struct platform_driver jh7110_aon_pinctrl_driver = {
    .probe = jh7110_pinctrl_probe,
    .driver = {
    .name = "starfive-jh7110-aon-pinctrl",
    .of_match_table = jh7110_aon_pinctrl_of_match,
    .pm = pm_sleep_ptr(&jh7110_pinctrl_pm_ops),
    },
    };
    module_platform_driver(jh7110_aon_pinctrl_driver);
    MODULE_DESCRIPTION("Pinctrl driver for the StarFive JH7110 SoC aon controller");
    MODULE_AUTHOR("Jianlong Huang <jianlong.huang@starfivetech.com>");
    MODULE_LICENSE("GPL");
