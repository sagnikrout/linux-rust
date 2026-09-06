//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-lpc32xx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// GPIO driver for LPC32xx SoC
//
// Author: Kevin Wells <kevin.wells@nxp.com>
//
// Copyright (C) 2010 NXP Semiconductors
//

pub const LPC32XX_GPIO_P0_MAX: c_int = 8;
pub const LPC32XX_GPIO_P1_MAX: c_int = 24;
pub const LPC32XX_GPIO_P2_MAX: c_int = 13;
pub const LPC32XX_GPIO_P3_MAX: c_int = 6;
pub const LPC32XX_GPI_P3_MAX: c_int = 29;
pub const LPC32XX_GPO_P3_MAX: c_int = 24;
pub const LPC32XX_GPIO_P0_GRP: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_regs {
    pub inp_state: c_ulong,
    pub outp_state: c_ulong,
    pub outp_set: c_ulong,
    pub outp_clr: c_ulong,
    pub dir_set: c_ulong,
    pub dir_clr: c_ulong,
}

//
// GPIO names
//
    static const char *gpio_p0_names[LPC32XX_GPIO_P0_MAX] = {
    "p0.0", "p0.1", "p0.2", "p0.3",
    "p0.4", "p0.5", "p0.6", "p0.7"
    };
    static const char *gpio_p1_names[LPC32XX_GPIO_P1_MAX] = {
    "p1.0", "p1.1", "p1.2", "p1.3",
    "p1.4", "p1.5", "p1.6", "p1.7",
    "p1.8", "p1.9", "p1.10", "p1.11",
    "p1.12", "p1.13", "p1.14", "p1.15",
    "p1.16", "p1.17", "p1.18", "p1.19",
    "p1.20", "p1.21", "p1.22", "p1.23",
    };
    static const char *gpio_p2_names[LPC32XX_GPIO_P2_MAX] = {
    "p2.0", "p2.1", "p2.2", "p2.3",
    "p2.4", "p2.5", "p2.6", "p2.7",
    "p2.8", "p2.9", "p2.10", "p2.11",
    "p2.12"
    };
    static const char *gpio_p3_names[LPC32XX_GPIO_P3_MAX] = {
    "gpio00", "gpio01", "gpio02", "gpio03",
    "gpio04", "gpio05"
    };
    static const char *gpi_p3_names[LPC32XX_GPI_P3_MAX] = {
    "gpi00", "gpi01", "gpi02", "gpi03",
    "gpi04", "gpi05", "gpi06", "gpi07",
    "gpi08", "gpi09",  core::ptr::null_mut(),    core::ptr::null_mut(),
    core::ptr::null_mut(),    core::ptr::null_mut(),    core::ptr::null_mut(),   "gpi15",
    "gpi16", "gpi17", "gpi18", "gpi19",
    "gpi20", "gpi21", "gpi22", "gpi23",
    "gpi24", "gpi25", "gpi26", "gpi27",
    "gpi28"
    };
    static const char *gpo_p3_names[LPC32XX_GPO_P3_MAX] = {
    "gpo00", "gpo01", "gpo02", "gpo03",
    "gpo04", "gpo05", "gpo06", "gpo07",
    "gpo08", "gpo09", "gpo10", "gpo11",
    "gpo12", "gpo13", "gpo14", "gpo15",
    "gpo16", "gpo17", "gpo18", "gpo19",
    "gpo20", "gpo21", "gpo22", "gpo23"
    };
    static struct gpio_regs gpio_grp_regs_p0 = {
    .inp_state	= LPC32XX_GPIO_P0_INP_STATE,
    .outp_set	= LPC32XX_GPIO_P0_OUTP_SET,
    .outp_clr	= LPC32XX_GPIO_P0_OUTP_CLR,
    .dir_set	= LPC32XX_GPIO_P0_DIR_SET,
    .dir_clr	= LPC32XX_GPIO_P0_DIR_CLR,
    };
    static struct gpio_regs gpio_grp_regs_p1 = {
    .inp_state	= LPC32XX_GPIO_P1_INP_STATE,
    .outp_set	= LPC32XX_GPIO_P1_OUTP_SET,
    .outp_clr	= LPC32XX_GPIO_P1_OUTP_CLR,
    .dir_set	= LPC32XX_GPIO_P1_DIR_SET,
    .dir_clr	= LPC32XX_GPIO_P1_DIR_CLR,
    };
    static struct gpio_regs gpio_grp_regs_p2 = {
    .inp_state	= LPC32XX_GPIO_P2_INP_STATE,
    .outp_set	= LPC32XX_GPIO_P2_OUTP_SET,
    .outp_clr	= LPC32XX_GPIO_P2_OUTP_CLR,
    .dir_set	= LPC32XX_GPIO_P2_DIR_SET,
    .dir_clr	= LPC32XX_GPIO_P2_DIR_CLR,
    };
    static struct gpio_regs gpio_grp_regs_p3 = {
    .inp_state	= LPC32XX_GPIO_P3_INP_STATE,
    .outp_state	= LPC32XX_GPIO_P3_OUTP_STATE,
    .outp_set	= LPC32XX_GPIO_P3_OUTP_SET,
    .outp_clr	= LPC32XX_GPIO_P3_OUTP_CLR,
    .dir_set	= LPC32XX_GPIO_P2_DIR_SET,
    .dir_clr	= LPC32XX_GPIO_P2_DIR_CLR,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_gpio_chip {
    pub chip: gpio_chip,
    pub gpio_grp: *mut gpio_regs,
    pub reg_base: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn gpreg_read(group: *mut lpc32xx_gpio_chip, offset: c_ulong) -> u32 {
    static inline u32 gpreg_read(struct lpc32xx_gpio_chip *group, unsigned long offset)
    {
    return __raw_readl(group.reg_base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn gpreg_write(group: *mut lpc32xx_gpio_chip, val: u32, offset: c_ulong) {
    static inline void gpreg_write(struct lpc32xx_gpio_chip *group, u32 val, unsigned long offset)
    {
    __raw_writel(val, group.reg_base + offset);
    }
    static void __set_gpio_dir_p012(struct lpc32xx_gpio_chip *group,
    unsigned pin, int input)
    {
    if (input)
    gpreg_write(group, GPIO012_PIN_TO_BIT(pin),
    group.gpio_grp.dir_clr);
    else
    gpreg_write(group, GPIO012_PIN_TO_BIT(pin),
    group.gpio_grp.dir_set);
    }
    static void __set_gpio_dir_p3(struct lpc32xx_gpio_chip *group,
    unsigned pin, int input)
    {
    let mut u: u32 = GPIO3_PIN_TO_BIT(pin);
    if (input)
    gpreg_write(group, u, group.gpio_grp.dir_clr);
    else
    gpreg_write(group, u, group.gpio_grp.dir_set);
    }
    static void __set_gpio_level_p012(struct lpc32xx_gpio_chip *group,
    unsigned pin, int high)
    {
    if (high)
    gpreg_write(group, GPIO012_PIN_TO_BIT(pin),
    group.gpio_grp.outp_set);
    else
    gpreg_write(group, GPIO012_PIN_TO_BIT(pin),
    group.gpio_grp.outp_clr);
    }
    static void __set_gpio_level_p3(struct lpc32xx_gpio_chip *group,
    unsigned pin, int high)
    {
    let mut u: u32 = GPIO3_PIN_TO_BIT(pin);
    if (high)
    gpreg_write(group, u, group.gpio_grp.outp_set);
    else
    gpreg_write(group, u, group.gpio_grp.outp_clr);
    }
    static void __set_gpo_level_p3(struct lpc32xx_gpio_chip *group,
    unsigned pin, int high)
    {
    if (high)
    gpreg_write(group, GPO3_PIN_TO_BIT(pin), group.gpio_grp.outp_set);
    else
    gpreg_write(group, GPO3_PIN_TO_BIT(pin), group.gpio_grp.outp_clr);
    }
    static int __get_gpio_state_p012(struct lpc32xx_gpio_chip *group,
    unsigned pin)
    {
    return GPIO012_PIN_IN_SEL(gpreg_read(group, group.gpio_grp.inp_state),
    pin);
    }
    static int __get_gpio_state_p3(struct lpc32xx_gpio_chip *group,
    unsigned pin)
    {
    let mut state: c_int = gpreg_read(group, group.gpio_grp.inp_state);
//
// P3 GPIO pin input mapping is not contiguous, GPIOP3-0..4 is mapped
// to bits 10..14, while GPIOP3-5 is mapped to bit 24.
//
    return GPIO3_PIN_IN_SEL(state, pin);
    }
    static int __get_gpi_state_p3(struct lpc32xx_gpio_chip *group,
    unsigned pin)
    {
    return GPI3_PIN_IN_SEL(gpreg_read(group, group.gpio_grp.inp_state), pin);
    }
    static int __get_gpo_state_p3(struct lpc32xx_gpio_chip *group,
    unsigned pin)
    {
    return GPO3_PIN_IN_SEL(gpreg_read(group, group.gpio_grp.outp_state), pin);
    }
//
// GPIO primitives.
//
    static int lpc32xx_gpio_dir_input_p012(struct gpio_chip *chip,
    unsigned pin)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpio_dir_p012(group, pin, 1);
    return 0;
    }
    static int lpc32xx_gpio_dir_input_p3(struct gpio_chip *chip,
    unsigned pin)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpio_dir_p3(group, pin, 1);
    return 0;
    }
    static int lpc32xx_gpio_dir_in_always(struct gpio_chip *chip,
    unsigned pin)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpio_get_value_p012(chip: *mut gpio_chip, pin: unsigned) -> c_int {
    static int lpc32xx_gpio_get_value_p012(struct gpio_chip *chip, unsigned pin)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    return !!__get_gpio_state_p012(group, pin);
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpio_get_value_p3(chip: *mut gpio_chip, pin: unsigned) -> c_int {
    static int lpc32xx_gpio_get_value_p3(struct gpio_chip *chip, unsigned pin)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    return !!__get_gpio_state_p3(group, pin);
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpi_get_value(chip: *mut gpio_chip, pin: unsigned) -> c_int {
    static int lpc32xx_gpi_get_value(struct gpio_chip *chip, unsigned pin)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    return !!__get_gpi_state_p3(group, pin);
    }
    static int lpc32xx_gpio_dir_output_p012(struct gpio_chip *chip, unsigned pin,
    int value)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpio_level_p012(group, pin, value);
    __set_gpio_dir_p012(group, pin, 0);
    return 0;
    }
    static int lpc32xx_gpio_dir_output_p3(struct gpio_chip *chip, unsigned pin,
    int value)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpio_level_p3(group, pin, value);
    __set_gpio_dir_p3(group, pin, 0);
    return 0;
    }
    static int lpc32xx_gpio_dir_out_always(struct gpio_chip *chip, unsigned pin,
    int value)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpo_level_p3(group, pin, value);
    return 0;
    }
    static int lpc32xx_gpio_set_value_p012(struct gpio_chip *chip,
    unsigned int pin, int value)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpio_level_p012(group, pin, value);
    return 0;
    }
    static int lpc32xx_gpio_set_value_p3(struct gpio_chip *chip,
    unsigned int pin, int value)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpio_level_p3(group, pin, value);
    return 0;
    }
    static int lpc32xx_gpo_set_value(struct gpio_chip *chip, unsigned int pin,
    int value)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    __set_gpo_level_p3(group, pin, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpo_get_value(chip: *mut gpio_chip, pin: unsigned) -> c_int {
    static int lpc32xx_gpo_get_value(struct gpio_chip *chip, unsigned pin)
    {
    struct lpc32xx_gpio_chip *group = gpiochip_get_data(chip);
    return !!__get_gpo_state_p3(group, pin);
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpio_request(chip: *mut gpio_chip, pin: unsigned) -> c_int {
    static int lpc32xx_gpio_request(struct gpio_chip *chip, unsigned pin)
    {
    if (pin < chip.ngpio)
    return 0;
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpio_to_irq_p01(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int lpc32xx_gpio_to_irq_p01(struct gpio_chip *chip, unsigned offset)
    {
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpio_to_irq_gpio_p3(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int lpc32xx_gpio_to_irq_gpio_p3(struct gpio_chip *chip, unsigned offset)
    {
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpio_to_irq_gpi_p3(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int lpc32xx_gpio_to_irq_gpi_p3(struct gpio_chip *chip, unsigned offset)
    {
    return -ENXIO;
    }
    static struct lpc32xx_gpio_chip lpc32xx_gpiochip[] = {
    {
    .chip = {
    .label			= "gpio_p0",
    .direction_input	= lpc32xx_gpio_dir_input_p012,
    .get			= lpc32xx_gpio_get_value_p012,
    .direction_output	= lpc32xx_gpio_dir_output_p012,
    .set			= lpc32xx_gpio_set_value_p012,
    .request		= lpc32xx_gpio_request,
    .to_irq			= lpc32xx_gpio_to_irq_p01,
    .base			= LPC32XX_GPIO_P0_GRP,
    .ngpio			= LPC32XX_GPIO_P0_MAX,
    .names			= gpio_p0_names,
    .can_sleep		= false,
    },
    .gpio_grp = &gpio_grp_regs_p0,
    },
    {
    .chip = {
    .label			= "gpio_p1",
    .direction_input	= lpc32xx_gpio_dir_input_p012,
    .get			= lpc32xx_gpio_get_value_p012,
    .direction_output	= lpc32xx_gpio_dir_output_p012,
    .set			= lpc32xx_gpio_set_value_p012,
    .request		= lpc32xx_gpio_request,
    .to_irq			= lpc32xx_gpio_to_irq_p01,
    .base			= LPC32XX_GPIO_P1_GRP,
    .ngpio			= LPC32XX_GPIO_P1_MAX,
    .names			= gpio_p1_names,
    .can_sleep		= false,
    },
    .gpio_grp = &gpio_grp_regs_p1,
    },
    {
    .chip = {
    .label			= "gpio_p2",
    .direction_input	= lpc32xx_gpio_dir_input_p012,
    .get			= lpc32xx_gpio_get_value_p012,
    .direction_output	= lpc32xx_gpio_dir_output_p012,
    .set			= lpc32xx_gpio_set_value_p012,
    .request		= lpc32xx_gpio_request,
    .base			= LPC32XX_GPIO_P2_GRP,
    .ngpio			= LPC32XX_GPIO_P2_MAX,
    .names			= gpio_p2_names,
    .can_sleep		= false,
    },
    .gpio_grp = &gpio_grp_regs_p2,
    },
    {
    .chip = {
    .label			= "gpio_p3",
    .direction_input	= lpc32xx_gpio_dir_input_p3,
    .get			= lpc32xx_gpio_get_value_p3,
    .direction_output	= lpc32xx_gpio_dir_output_p3,
    .set			= lpc32xx_gpio_set_value_p3,
    .request		= lpc32xx_gpio_request,
    .to_irq			= lpc32xx_gpio_to_irq_gpio_p3,
    .base			= LPC32XX_GPIO_P3_GRP,
    .ngpio			= LPC32XX_GPIO_P3_MAX,
    .names			= gpio_p3_names,
    .can_sleep		= false,
    },
    .gpio_grp = &gpio_grp_regs_p3,
    },
    {
    .chip = {
    .label			= "gpi_p3",
    .direction_input	= lpc32xx_gpio_dir_in_always,
    .get			= lpc32xx_gpi_get_value,
    .request		= lpc32xx_gpio_request,
    .to_irq			= lpc32xx_gpio_to_irq_gpi_p3,
    .base			= LPC32XX_GPI_P3_GRP,
    .ngpio			= LPC32XX_GPI_P3_MAX,
    .names			= gpi_p3_names,
    .can_sleep		= false,
    },
    .gpio_grp = &gpio_grp_regs_p3,
    },
    {
    .chip = {
    .label			= "gpo_p3",
    .direction_output	= lpc32xx_gpio_dir_out_always,
    .set			= lpc32xx_gpo_set_value,
    .get			= lpc32xx_gpo_get_value,
    .request		= lpc32xx_gpio_request,
    .base			= LPC32XX_GPO_P3_GRP,
    .ngpio			= LPC32XX_GPO_P3_MAX,
    .names			= gpo_p3_names,
    .can_sleep		= false,
    },
    .gpio_grp = &gpio_grp_regs_p3,
    },
    };
    static int lpc32xx_of_xlate(struct gpio_chip *gc,
    const struct of_phandle_args *gpiospec, u32 *flags)
    {
// Is this the correct bank?
    let mut bank: u32 = gpiospec.args[0];
    if ((bank >= ARRAY_SIZE(lpc32xx_gpiochip) ||
    (gc != &lpc32xx_gpiochip[bank].chip)))
    return -EINVAL;
    if (flags)
// flags = gpiospec->args[2];
    return gpiospec.args[1];
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int lpc32xx_gpio_probe(struct platform_device *pdev)
    {
    int i;
    void __iomem *reg_base;
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    for (i = 0; i < ARRAY_SIZE(lpc32xx_gpiochip); i++) {
    lpc32xx_gpiochip[i].chip.parent = &pdev.dev;
    if (pdev.dev.of_node) {
    lpc32xx_gpiochip[i].chip.of_xlate = lpc32xx_of_xlate;
    lpc32xx_gpiochip[i].chip.of_gpio_n_cells = 3;
    lpc32xx_gpiochip[i].reg_base = reg_base;
    }
    devm_gpiochip_add_data(&pdev.dev, &lpc32xx_gpiochip[i].chip,
    &lpc32xx_gpiochip[i]);
    }
    return 0;
    }
    static const struct of_device_id lpc32xx_gpio_of_match[] = {
    { .compatible = "nxp,lpc3220-gpio", },
    { },
    };
    MODULE_DEVICE_TABLE(of, lpc32xx_gpio_of_match);
    static struct platform_driver lpc32xx_gpio_driver = {
    .driver		= {
    .name	= "lpc32xx-gpio",
    .of_match_table = lpc32xx_gpio_of_match,
    },
    .probe		= lpc32xx_gpio_probe,
    };
    module_platform_driver(lpc32xx_gpio_driver);
    MODULE_AUTHOR("Kevin Wells <kevin.wells@nxp.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("GPIO driver for LPC32xx SoC");
