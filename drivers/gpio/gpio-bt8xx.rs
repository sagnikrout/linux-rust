//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-bt8xx.c
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
    bt8xx GPIO abuser
    Copyright (C) 2008 Michael Buesch <m@bues.ch>
    Please do _only_ contact the people listed _above_ with issues related to this driver.
    All the other people listed below are not related to this driver. Their names
    are only here, because this driver is derived from the bt848 driver.
    Derived from the bt848 driver:
    Copyright (C) 1996,97,98 Ralph  Metzler
    & Marcus Metzler
    (c) 1999-2002 Gerd Knorr
    some v4l2 code lines are taken from Justin's bttv2 driver which is
    (c) 2000 Justin Schoeman
    V4L1 removal from:
    (c) 2005-2006 Nickolay V. Shmyrev
    Fixes to be fully V4L2 compliant by
    (c) 2006 Mauro Carvalho Chehab
    Cropping and overscan support
    Copyright (C) 2005, 2006 Michael H. Schimek
    Sponsored by OPQ Systems AB
//

// Steal the hardware definitions from the bttv driver.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt8xxgpio {
    pub lock: spinlock_t,
    pub mmio: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub gpio: gpio_chip,
    pub saved_outen: u32,
    pub saved_data: u32,
}

    let mut modparam_gpiobase: static int = -1/* dynamic */;
    module_param_named(gpiobase, modparam_gpiobase, int, 0444);
    MODULE_PARM_DESC(gpiobase, "The GPIO number base. -1 means dynamic, which is the default.");
#[no_mangle]
unsafe extern "C" fn bt8xxgpio_gpio_direction_input(gpio: *mut gpio_chip, nr: unsigned) -> c_int {
    static int bt8xxgpio_gpio_direction_input(struct gpio_chip *gpio, unsigned nr)
    {
    struct bt8xxgpio *bg = gpiochip_get_data(gpio);
    u32 outen, data;
    guard(spinlock_irqsave)(&bg.lock);
    data = bgread(BT848_GPIO_DATA);
    data &= ~(1 << nr);
    bgwrite(data, BT848_GPIO_DATA);
    outen = bgread(BT848_GPIO_OUT_EN);
    outen &= ~(1 << nr);
    bgwrite(outen, BT848_GPIO_OUT_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bt8xxgpio_gpio_get(gpio: *mut gpio_chip, nr: unsigned) -> c_int {
    static int bt8xxgpio_gpio_get(struct gpio_chip *gpio, unsigned nr)
    {
    struct bt8xxgpio *bg = gpiochip_get_data(gpio);
    u32 val;
    guard(spinlock_irqsave)(&bg.lock);
    val = bgread(BT848_GPIO_DATA);
    return !!(val & (1 << nr));
    }
    static int bt8xxgpio_gpio_direction_output(struct gpio_chip *gpio,
    unsigned nr, int val)
    {
    struct bt8xxgpio *bg = gpiochip_get_data(gpio);
    u32 outen, data;
    guard(spinlock_irqsave)(&bg.lock);
    outen = bgread(BT848_GPIO_OUT_EN);
    outen |= (1 << nr);
    bgwrite(outen, BT848_GPIO_OUT_EN);
    data = bgread(BT848_GPIO_DATA);
    if (val)
    data |= (1 << nr);
    else
    data &= ~(1 << nr);
    bgwrite(data, BT848_GPIO_DATA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bt8xxgpio_gpio_set(gpio: *mut gpio_chip, nr: c_uint, val: c_int) -> c_int {
    static int bt8xxgpio_gpio_set(struct gpio_chip *gpio, unsigned int nr, int val)
    {
    struct bt8xxgpio *bg = gpiochip_get_data(gpio);
    u32 data;
    guard(spinlock_irqsave)(&bg.lock);
    data = bgread(BT848_GPIO_DATA);
    if (val)
    data |= (1 << nr);
    else
    data &= ~(1 << nr);
    bgwrite(data, BT848_GPIO_DATA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bt8xxgpio_gpio_setup(bg: *mut bt8xxgpio) {
    static void bt8xxgpio_gpio_setup(struct bt8xxgpio *bg)
    {
    struct gpio_chip *c = &bg.gpio;
    c.label = dev_name(&bg.pdev.dev);
    c.owner = THIS_MODULE;
    c.direction_input = bt8xxgpio_gpio_direction_input;
    c.get = bt8xxgpio_gpio_get;
    c.direction_output = bt8xxgpio_gpio_direction_output;
    c.set = bt8xxgpio_gpio_set;
    c.dbg_show = core::ptr::null_mut();
    c.base = modparam_gpiobase;
    c.ngpio = BT8XXGPIO_NR_GPIOS;
    c.can_sleep = false;
    }
    static int bt8xxgpio_probe(struct pci_dev *dev,
    const struct pci_device_id *pci_id)
    {
    struct bt8xxgpio *bg;
    void __iomem *mmio;
    int err;
    mmio = devm_ioremap_resource(&dev.dev, pci_resource_n(dev, 0));
    if (IS_ERR(mmio))
    return PTR_ERR(mmio);
    bg = devm_kzalloc(&dev.dev, sizeof(struct bt8xxgpio), GFP_KERNEL);
    if (!bg)
    return -ENOMEM;
    bg.mmio = mmio;
    bg.pdev = dev;
    spin_lock_init(&bg.lock);
    err = pci_enable_device(dev);
    if (err) {
    dev_err(&dev.dev, "can't enable device.\n");
    return err;
    }
    pci_set_master(dev);
    pci_set_drvdata(dev, bg);
// Disable interrupts
    bgwrite(0, BT848_INT_MASK);
// gpio init
    bgwrite(0, BT848_GPIO_DMA_CTL);
    bgwrite(0, BT848_GPIO_REG_INP);
    bgwrite(0, BT848_GPIO_OUT_EN);
    bt8xxgpio_gpio_setup(bg);
    err = gpiochip_add_data(&bg.gpio, bg);
    if (err) {
    dev_err(&dev.dev, "failed to register GPIOs\n");
    goto err_disable;
    }
    return 0;
    err_disable:
    pci_disable_device(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bt8xxgpio_remove(pdev: *mut pci_dev) {
    static void bt8xxgpio_remove(struct pci_dev *pdev)
    {
    struct bt8xxgpio *bg = pci_get_drvdata(pdev);
    gpiochip_remove(&bg.gpio);
    bgwrite(0, BT848_INT_MASK);
    bgwrite(~0x0, BT848_INT_STAT);
    bgwrite(0x0, BT848_GPIO_OUT_EN);
    pci_disable_device(pdev);
    }
#[no_mangle]
unsafe extern "C" fn bt8xxgpio_suspend(dev: *mut device) -> c_int {
    static int bt8xxgpio_suspend(struct device *dev)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct bt8xxgpio *bg = pci_get_drvdata(pdev);
    scoped_guard(spinlock_irqsave, &bg.lock) {
    bg.saved_outen = bgread(BT848_GPIO_OUT_EN);
    bg.saved_data = bgread(BT848_GPIO_DATA);
    bgwrite(0, BT848_INT_MASK);
    bgwrite(~0x0, BT848_INT_STAT);
    bgwrite(0x0, BT848_GPIO_OUT_EN);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bt8xxgpio_resume(dev: *mut device) -> c_int {
    static int bt8xxgpio_resume(struct device *dev)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct bt8xxgpio *bg = pci_get_drvdata(pdev);
    guard(spinlock_irqsave)(&bg.lock);
    bgwrite(0, BT848_INT_MASK);
    bgwrite(0, BT848_GPIO_DMA_CTL);
    bgwrite(0, BT848_GPIO_REG_INP);
    bgwrite(bg.saved_outen, BT848_GPIO_OUT_EN);
    bgwrite(bg.saved_data & bg.saved_outen,
    BT848_GPIO_DATA);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(bt8xxgpio_pm_ops, bt8xxgpio_suspend, bt8xxgpio_resume);
    static const struct pci_device_id bt8xxgpio_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_BROOKTREE, PCI_DEVICE_ID_BT848) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROOKTREE, PCI_DEVICE_ID_BT849) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROOKTREE, PCI_DEVICE_ID_BT878) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROOKTREE, PCI_DEVICE_ID_BT879) },
    { 0, },
    };
    MODULE_DEVICE_TABLE(pci, bt8xxgpio_pci_tbl);
    static struct pci_driver bt8xxgpio_pci_driver = {
    .name		= "bt8xxgpio",
    .id_table	= bt8xxgpio_pci_tbl,
    .probe		= bt8xxgpio_probe,
    .remove		= bt8xxgpio_remove,
    .driver.pm	= &bt8xxgpio_pm_ops,
    };
    module_pci_driver(bt8xxgpio_pci_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michael Buesch");
    MODULE_DESCRIPTION("Abuse a BT8xx framegrabber card as generic GPIO card");
