//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mm-lantiq.c
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
// Copyright (C) 2012 John Crispin <john@phrozen.org>
//

//
// By attaching hardware latches to the EBU it is possible to create output
// only gpios. This driver configures a special memory address, which when
// written to outputs 16 bit to the latches.
//
pub const LTQ_EBU_BUSCON: c_uint = 0x1e7ff		/* 16 bit access, slowest timing */;
pub const LTQ_EBU_WP: c_uint = 0x80000000	/* write protect bit */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_mm {
    pub gc: gpio_chip,
    pub regs: *mut void __iomem,
    pub /: *mut *mut u16 shadow; / shadow the latches state,
}

//
// ltq_mm_apply() - write the shadow value to the ebu address.
// @chip:     Pointer to our private data structure.
//
// Write the shadow value to the EBU to set the gpios. We need to set the
// global EBU lock to make sure that PCI/MTD don't break.
//
#[no_mangle]
unsafe extern "C" fn ltq_mm_apply(chip: *mut ltq_mm) {
    static void ltq_mm_apply(struct ltq_mm *chip)
    {
    unsigned long flags;
    spin_lock_irqsave(&ebu_lock, flags);
    ltq_ebu_w32(LTQ_EBU_BUSCON, LTQ_EBU_BUSCON1);
    __raw_writew(chip.shadow, chip.regs);
    ltq_ebu_w32(LTQ_EBU_BUSCON | LTQ_EBU_WP, LTQ_EBU_BUSCON1);
    spin_unlock_irqrestore(&ebu_lock, flags);
    }
//
// ltq_mm_set() - gpio_chip->set - set gpios.
// @gc:     Pointer to gpio_chip device structure.
// @offset: GPIO signal number.
// @value:  Value to be written to specified signal.
//
// Set the shadow value and call ltq_mm_apply. Always returns 0.
//
#[no_mangle]
unsafe extern "C" fn ltq_mm_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int ltq_mm_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct ltq_mm *chip = gpiochip_get_data(gc);
    if (value)
    chip.shadow |= (1 << offset);
    else
    chip.shadow &= ~(1 << offset);
    ltq_mm_apply(chip);
    return 0;
    }
//
// ltq_mm_dir_out() - gpio_chip->dir_out - set gpio direction.
// @gc:     Pointer to gpio_chip device structure.
// @offset: GPIO signal number.
// @value:  Value to be written to specified signal.
//
// Same as ltq_mm_set, always returns 0.
//
#[no_mangle]
unsafe extern "C" fn ltq_mm_dir_out(gc: *mut gpio_chip, offset: unsigned, value: c_int) -> c_int {
    static int ltq_mm_dir_out(struct gpio_chip *gc, unsigned offset, int value)
    {
    return ltq_mm_set(gc, offset, value);
    }
//
// ltq_mm_save_regs() - Set initial values of GPIO pins
// @chip: Pointer to our private data structure.
//
#[no_mangle]
unsafe extern "C" fn ltq_mm_save_regs(chip: *mut ltq_mm) {
    static void ltq_mm_save_regs(struct ltq_mm *chip)
    {
// tell the ebu controller which memory address we will be using
    ltq_ebu_w32(CPHYSADDR(( void *)chip.regs) | 0x1, LTQ_EBU_ADDRSEL1);
    ltq_mm_apply(chip);
    }
#[no_mangle]
unsafe extern "C" fn ltq_mm_probe(pdev: *mut platform_device) -> c_int {
    static int ltq_mm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct gpio_chip *gc;
    struct ltq_mm *chip;
    u32 shadow;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    gc = &chip.gc;
    gc.base = -1;
    gc.ngpio = 16;
    gc.direction_output = ltq_mm_dir_out;
    gc.set = ltq_mm_set;
    gc.parent = dev;
    gc.owner = THIS_MODULE;
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if (!gc.label)
    return -ENOMEM;
    chip.regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if (IS_ERR(chip.regs))
    return PTR_ERR(chip.regs);
    ltq_mm_save_regs(chip);
// store the shadow value if one was passed by the devicetree
    if (!of_property_read_u32(pdev.dev.of_node, "lantiq,shadow", &shadow))
    chip.shadow = shadow;
    return devm_gpiochip_add_data(dev, gc, chip);
    }
    static const struct of_device_id ltq_mm_match[] = {
    { .compatible = "lantiq,gpio-mm" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ltq_mm_match);
    static struct platform_driver ltq_mm_driver = {
    .probe = ltq_mm_probe,
    .driver = {
    .name = "gpio-mm-ltq",
    .of_match_table = ltq_mm_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn ltq_mm_init() -> int __init {
    static int __init ltq_mm_init(void)
    {
    return platform_driver_register(&ltq_mm_driver);
    }
    subsys_initcall(ltq_mm_init);
#[no_mangle]
unsafe extern "C" fn ltq_mm_exit() -> void __exit {
    static void __exit ltq_mm_exit(void)
    {
    platform_driver_unregister(&ltq_mm_driver);
    }
    module_exit(ltq_mm_exit);
