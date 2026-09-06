//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mpc5200.c
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
// MPC52xx gpio driver
//
// Copyright (c) 2008 Sascha Hauer <s.hauer@pengutronix.de>, Pengutronix
//

    static DEFINE_SPINLOCK(gpio_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_gpiochip {
    pub gc: gpio_chip,
    pub regs: *mut void __iomem,
    pub shadow_dvo: c_uint,
    pub shadow_gpioe: c_uint,
    pub shadow_ddr: c_uint,
}

//
// GPIO LIB API implementation for wakeup GPIOs.
//
// There's a maximum of 8 wakeup GPIOs. Which of these are available
// for use depends on your board setup.
//
// 0 -> GPIO_WKUP_7
// 1 -> GPIO_WKUP_6
// 2 -> PSC6_1
// 3 -> PSC6_0
// 4 -> ETH_17
// 5 -> PSC3_9
// 6 -> PSC2_4
// 7 -> PSC1_4
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_wkup_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int mpc52xx_wkup_gpio_get(struct gpio_chip *gc, unsigned int gpio)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio_wkup __iomem *regs = chip.regs;
    unsigned int ret;
    ret = (in_8(&regs.wkup_ival) >> (7 - gpio)) & 1;
    pr_debug("%s: gpio: %d ret: %d\n", __func__, gpio, ret);
    return ret;
    }
    static inline void
    __mpc52xx_wkup_gpio_set(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio_wkup __iomem *regs = chip.regs;
    if (val)
    chip.shadow_dvo |= 1 << (7 - gpio);
    else
    chip.shadow_dvo &= ~(1 << (7 - gpio));
    out_8(&regs.wkup_dvo, chip.shadow_dvo);
    }
    static int
    mpc52xx_wkup_gpio_set(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    unsigned long flags;
    spin_lock_irqsave(&gpio_lock, flags);
    __mpc52xx_wkup_gpio_set(gc, gpio, val);
    spin_unlock_irqrestore(&gpio_lock, flags);
    pr_debug("%s: gpio: %d val: %d\n", __func__, gpio, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_wkup_gpio_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int mpc52xx_wkup_gpio_dir_in(struct gpio_chip *gc, unsigned int gpio)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio_wkup __iomem *regs = chip.regs;
    unsigned long flags;
    spin_lock_irqsave(&gpio_lock, flags);
// set the direction
    chip.shadow_ddr &= ~(1 << (7 - gpio));
    out_8(&regs.wkup_ddr, chip.shadow_ddr);
// and enable the pin
    chip.shadow_gpioe |= 1 << (7 - gpio);
    out_8(&regs.wkup_gpioe, chip.shadow_gpioe);
    spin_unlock_irqrestore(&gpio_lock, flags);
    return 0;
    }
    static int
    mpc52xx_wkup_gpio_dir_out(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio_wkup __iomem *regs = chip.regs;
    unsigned long flags;
    spin_lock_irqsave(&gpio_lock, flags);
    __mpc52xx_wkup_gpio_set(gc, gpio, val);
// Then set direction
    chip.shadow_ddr |= 1 << (7 - gpio);
    out_8(&regs.wkup_ddr, chip.shadow_ddr);
// Finally enable the pin
    chip.shadow_gpioe |= 1 << (7 - gpio);
    out_8(&regs.wkup_gpioe, chip.shadow_gpioe);
    spin_unlock_irqrestore(&gpio_lock, flags);
    pr_debug("%s: gpio: %d val: %d\n", __func__, gpio, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_wkup_gpiochip_probe(ofdev: *mut platform_device) -> c_int {
    static int mpc52xx_wkup_gpiochip_probe(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    struct device_node *np = dev.of_node;
    struct mpc52xx_gpiochip *chip;
    struct mpc52xx_gpio_wkup __iomem *regs;
    struct gpio_chip *gc;
    int ret;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    platform_set_drvdata(ofdev, chip);
    gc = &chip.gc;
    gc.base             = -1;
    gc.ngpio            = 8;
    gc.direction_input  = mpc52xx_wkup_gpio_dir_in;
    gc.direction_output = mpc52xx_wkup_gpio_dir_out;
    gc.get              = mpc52xx_wkup_gpio_get;
    gc.set              = mpc52xx_wkup_gpio_set;
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if (!gc.label)
    return -ENOMEM;
    chip.regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if (IS_ERR(chip.regs))
    return PTR_ERR(chip.regs);
    ret = devm_gpiochip_add_data(dev, gc, chip);
    if (ret)
    return ret;
    regs = chip.regs;
    chip.shadow_gpioe = in_8(&regs.wkup_gpioe);
    chip.shadow_ddr = in_8(&regs.wkup_ddr);
    chip.shadow_dvo = in_8(&regs.wkup_dvo);
    return 0;
    }
    static const struct of_device_id mpc52xx_wkup_gpiochip_match[] = {
    { .compatible = "fsl,mpc5200-gpio-wkup", },
    {}
    };
    static struct platform_driver mpc52xx_wkup_gpiochip_driver = {
    .driver = {
    .name = "mpc5200-gpio-wkup",
    .of_match_table = mpc52xx_wkup_gpiochip_match,
    },
    .probe = mpc52xx_wkup_gpiochip_probe,
    };
//
// GPIO LIB API implementation for simple GPIOs
//
// There's a maximum of 32 simple GPIOs. Which of these are available
// for use depends on your board setup.
// The numbering reflects the bit numbering in the port registers:
//
// 0..1  > reserved
// 2..3  > IRDA
// 4..7  > ETHR
// 8..11 > reserved
// 12..15 > USB
// 16..17 > reserved
// 18..23 > PSC3
// 24..27 > PSC2
// 28..31 > PSC1
//
#[no_mangle]
unsafe extern "C" fn mpc52xx_simple_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int mpc52xx_simple_gpio_get(struct gpio_chip *gc, unsigned int gpio)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio __iomem *regs = chip.regs;
    unsigned int ret;
    ret = (in_be32(&regs.simple_ival) >> (31 - gpio)) & 1;
    return ret;
    }
    static inline void
    __mpc52xx_simple_gpio_set(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio __iomem *regs = chip.regs;
    if (val)
    chip.shadow_dvo |= 1 << (31 - gpio);
    else
    chip.shadow_dvo &= ~(1 << (31 - gpio));
    out_be32(&regs.simple_dvo, chip.shadow_dvo);
    }
    static int
    mpc52xx_simple_gpio_set(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    unsigned long flags;
    spin_lock_irqsave(&gpio_lock, flags);
    __mpc52xx_simple_gpio_set(gc, gpio, val);
    spin_unlock_irqrestore(&gpio_lock, flags);
    pr_debug("%s: gpio: %d val: %d\n", __func__, gpio, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_simple_gpio_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int mpc52xx_simple_gpio_dir_in(struct gpio_chip *gc, unsigned int gpio)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio __iomem *regs = chip.regs;
    unsigned long flags;
    spin_lock_irqsave(&gpio_lock, flags);
// set the direction
    chip.shadow_ddr &= ~(1 << (31 - gpio));
    out_be32(&regs.simple_ddr, chip.shadow_ddr);
// and enable the pin
    chip.shadow_gpioe |= 1 << (31 - gpio);
    out_be32(&regs.simple_gpioe, chip.shadow_gpioe);
    spin_unlock_irqrestore(&gpio_lock, flags);
    return 0;
    }
    static int
    mpc52xx_simple_gpio_dir_out(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct mpc52xx_gpiochip *chip = gpiochip_get_data(gc);
    struct mpc52xx_gpio __iomem *regs = chip.regs;
    unsigned long flags;
    spin_lock_irqsave(&gpio_lock, flags);
// First set initial value
    __mpc52xx_simple_gpio_set(gc, gpio, val);
// Then set direction
    chip.shadow_ddr |= 1 << (31 - gpio);
    out_be32(&regs.simple_ddr, chip.shadow_ddr);
// Finally enable the pin
    chip.shadow_gpioe |= 1 << (31 - gpio);
    out_be32(&regs.simple_gpioe, chip.shadow_gpioe);
    spin_unlock_irqrestore(&gpio_lock, flags);
    pr_debug("%s: gpio: %d val: %d\n", __func__, gpio, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_simple_gpiochip_probe(ofdev: *mut platform_device) -> c_int {
    static int mpc52xx_simple_gpiochip_probe(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    struct device_node *np = dev.of_node;
    struct mpc52xx_gpiochip *chip;
    struct gpio_chip *gc;
    struct mpc52xx_gpio __iomem *regs;
    int ret;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    platform_set_drvdata(ofdev, chip);
    gc = &chip.gc;
    gc.base             = -1;
    gc.ngpio            = 32;
    gc.direction_input  = mpc52xx_simple_gpio_dir_in;
    gc.direction_output = mpc52xx_simple_gpio_dir_out;
    gc.get              = mpc52xx_simple_gpio_get;
    gc.set              = mpc52xx_simple_gpio_set;
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if (!gc.label)
    return -ENOMEM;
    chip.regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if (IS_ERR(chip.regs))
    return PTR_ERR(chip.regs);
    ret = devm_gpiochip_add_data(dev, gc, chip);
    if (ret)
    return ret;
    regs = chip.regs;
    chip.shadow_gpioe = in_be32(&regs.simple_gpioe);
    chip.shadow_ddr = in_be32(&regs.simple_ddr);
    chip.shadow_dvo = in_be32(&regs.simple_dvo);
    return 0;
    }
    static const struct of_device_id mpc52xx_simple_gpiochip_match[] = {
    { .compatible = "fsl,mpc5200-gpio", },
    {}
    };
    static struct platform_driver mpc52xx_simple_gpiochip_driver = {
    .driver = {
    .name = "mpc5200-gpio",
    .of_match_table = mpc52xx_simple_gpiochip_match,
    },
    .probe = mpc52xx_simple_gpiochip_probe,
    };
    static struct platform_driver * const drivers[] = {
    &mpc52xx_wkup_gpiochip_driver,
    &mpc52xx_simple_gpiochip_driver,
    };
#[no_mangle]
unsafe extern "C" fn mpc52xx_gpio_init() -> int __init {
    static int __init mpc52xx_gpio_init(void)
    {
    return platform_register_drivers(drivers, ARRAY_SIZE(drivers));
    }
// Make sure we get initialised before anyone else tries to use us
    subsys_initcall(mpc52xx_gpio_init);
#[no_mangle]
unsafe extern "C" fn mpc52xx_gpio_exit() -> void __exit {
    static void __exit mpc52xx_gpio_exit(void)
    {
    platform_unregister_drivers(drivers, ARRAY_SIZE(drivers));
    }
    module_exit(mpc52xx_gpio_exit);
    MODULE_DESCRIPTION("Freescale MPC52xx gpio driver");
    MODULE_AUTHOR("Sascha Hauer <s.hauer@pengutronix.de");
    MODULE_LICENSE("GPL v2");
