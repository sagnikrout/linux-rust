//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/cpm_common.c
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
// Common CPM code
//
// Author: Scott Wood <scottwood@freescale.com>
//
// Copyright 2007-2008,2010 Freescale Semiconductor, Inc.
//
// Some parts derived from commproc.c/cpm2_common.c, which is:
// Copyright (c) 1997 Dan error_act (dmalek@jlc.net)
// Copyright (c) 1999-2001 Dan Malek <dan@embeddedalley.com>
// Copyright (c) 2000 MontaVista Software, Inc (source@mvista.com)
// 2006 (c) MontaVista Software, Inc.
// Vitaly Bordug <vbordug@ru.mvista.com>
//

#[no_mangle]
unsafe extern "C" fn cpm_init() -> int __init {
    static int __init cpm_init(void)
    {
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,cpm1");
    if (!np)
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,cpm2");
    if (!np)
    return -ENODEV;
    cpm_muram_init();
    of_node_put(np);
    return 0;
    }
    subsys_initcall(cpm_init);

    static u32 __iomem *cpm_udbg_txdesc;
    static u8 __iomem *cpm_udbg_txbuf;
#[no_mangle]
unsafe extern "C" fn udbg_putc_cpm(c: c_char) {
    static void udbg_putc_cpm(char c)
    {
    if (c == '\n')
    udbg_putc_cpm('\r');
    while (in_be32(&cpm_udbg_txdesc[0]) & 0x80000000)
    ;
    out_8(cpm_udbg_txbuf, c);
    out_be32(&cpm_udbg_txdesc[0], 0xa0000001);
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_init_cpm() -> void __init {
    void __init udbg_init_cpm(void)
    {

    mmu_mapin_immr();
    cpm_udbg_txdesc = (u32 __iomem  *)
    (CONFIG_PPC_EARLY_DEBUG_CPM_ADDR - PHYS_IMMR_BASE +
    VIRT_IMMR_BASE);
    cpm_udbg_txbuf = (u8 __iomem  *)
    (in_be32(&cpm_udbg_txdesc[1]) - PHYS_IMMR_BASE +
    VIRT_IMMR_BASE);

    cpm_udbg_txdesc = (u32 __iomem  *)
    CONFIG_PPC_EARLY_DEBUG_CPM_ADDR;
    cpm_udbg_txbuf = (u8 __iomem  *)in_be32(&cpm_udbg_txdesc[1]);

    if (cpm_udbg_txdesc) {

    setbat(1, 0xf0000000, 0xf0000000, 1024*1024, PAGE_KERNEL_NCG);

    udbg_putc = udbg_putc_cpm;
    }
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm2_ioports {
    pub dat: u32 dir, par, sor, odr,,
    pub res: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm2_gpio32_chip {
    pub gc: gpio_chip,
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
// shadowed data register to clear/set bits safely
    pub cpdata: u32,
}

#[no_mangle]
unsafe extern "C" fn cpm2_gpio32_save_regs(cpm2_gc: *mut cpm2_gpio32_chip) {
    static void cpm2_gpio32_save_regs(struct cpm2_gpio32_chip *cpm2_gc)
    {
    struct cpm2_ioports __iomem *iop = cpm2_gc.regs;
    cpm2_gc.cpdata = in_be32(&iop.dat);
    }
#[no_mangle]
unsafe extern "C" fn cpm2_gpio32_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int cpm2_gpio32_get(struct gpio_chip *gc, unsigned int gpio)
    {
    struct cpm2_gpio32_chip *cpm2_gc = gpiochip_get_data(gc);
    struct cpm2_ioports __iomem *iop = cpm2_gc.regs;
    u32 pin_mask;
    pin_mask = 1 << (31 - gpio);
    return !!(in_be32(&iop.dat) & pin_mask);
    }
#[no_mangle]
unsafe extern "C" fn __cpm2_gpio32_set(cpm2_gc: *mut cpm2_gpio32_chip, pin_mask: u32, value: c_int) {
    static void __cpm2_gpio32_set(struct cpm2_gpio32_chip *cpm2_gc, u32 pin_mask, int value)
    {
    struct cpm2_ioports __iomem *iop = cpm2_gc.regs;
    if (value)
    cpm2_gc.cpdata |= pin_mask;
    else
    cpm2_gc.cpdata &= ~pin_mask;
    out_be32(&iop.dat, cpm2_gc.cpdata);
    }
#[no_mangle]
unsafe extern "C" fn cpm2_gpio32_set(gc: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int cpm2_gpio32_set(struct gpio_chip *gc, unsigned int gpio, int value)
    {
    struct cpm2_gpio32_chip *cpm2_gc = gpiochip_get_data(gc);
    unsigned long flags;
    let mut pin_mask: u32 = 1 << (31 - gpio);
    spin_lock_irqsave(&cpm2_gc.lock, flags);
    __cpm2_gpio32_set(cpm2_gc, pin_mask, value);
    spin_unlock_irqrestore(&cpm2_gc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpm2_gpio32_dir_out(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    static int cpm2_gpio32_dir_out(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct cpm2_gpio32_chip *cpm2_gc = gpiochip_get_data(gc);
    struct cpm2_ioports __iomem *iop = cpm2_gc.regs;
    unsigned long flags;
    let mut pin_mask: u32 = 1 << (31 - gpio);
    spin_lock_irqsave(&cpm2_gc.lock, flags);
    setbits32(&iop.dir, pin_mask);
    __cpm2_gpio32_set(cpm2_gc, pin_mask, val);
    spin_unlock_irqrestore(&cpm2_gc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpm2_gpio32_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int cpm2_gpio32_dir_in(struct gpio_chip *gc, unsigned int gpio)
    {
    struct cpm2_gpio32_chip *cpm2_gc = gpiochip_get_data(gc);
    struct cpm2_ioports __iomem *iop = cpm2_gc.regs;
    unsigned long flags;
    let mut pin_mask: u32 = 1 << (31 - gpio);
    spin_lock_irqsave(&cpm2_gc.lock, flags);
    clrbits32(&iop.dir, pin_mask);
    spin_unlock_irqrestore(&cpm2_gc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpm2_gpio32_get_direction(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int cpm2_gpio32_get_direction(struct gpio_chip *gc, unsigned int gpio)
    {
    struct cpm2_gpio32_chip *cpm2_gc = gpiochip_get_data(gc);
    struct cpm2_ioports __iomem *iop = cpm2_gc.regs;
    let mut pin_mask: u32 = 1 << (31 - gpio);
    if (in_be32(&iop.dir) & pin_mask)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
pub unsafe extern "C" fn cpm2_gpiochip_add32(dev: *mut device) -> c_int {
    int cpm2_gpiochip_add32(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct cpm2_gpio32_chip *cpm2_gc;
    struct gpio_chip *gc;
    cpm2_gc = devm_kzalloc(dev, sizeof(*cpm2_gc), GFP_KERNEL);
    if (!cpm2_gc)
    return -ENOMEM;
    spin_lock_init(&cpm2_gc.lock);
    gc = &cpm2_gc.gc;
    gc.base = -1;
    gc.ngpio = 32;
    gc.direction_input = cpm2_gpio32_dir_in;
    gc.direction_output = cpm2_gpio32_dir_out;
    gc.get_direction = cpm2_gpio32_get_direction;
    gc.get = cpm2_gpio32_get;
    gc.set = cpm2_gpio32_set;
    gc.parent = dev;
    gc.owner = THIS_MODULE;
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if (!gc.label)
    return -ENOMEM;
    cpm2_gc.regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if (IS_ERR(cpm2_gc.regs))
    return PTR_ERR(cpm2_gc.regs);
    cpm2_gpio32_save_regs(cpm2_gc);
    return devm_gpiochip_add_data(dev, gc, cpm2_gc);
    }
