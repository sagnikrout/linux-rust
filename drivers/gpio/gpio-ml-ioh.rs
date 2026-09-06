//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ml-ioh.c
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
// Copyright (C) 2010 OKI SEMICONDUCTOR Co., LTD.
//

pub const IOH_EDGE_FALLING: c_int = 0;

pub const IOH_IRQ_BASE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioh_reg_comn {
    pub ien: u32,
    pub istatus: u32,
    pub idisp: u32,
    pub iclr: u32,
    pub imask: u32,
    pub imaskclr: u32,
    pub po: u32,
    pub pi: u32,
    pub pm: u32,
    pub im_0: u32,
    pub im_1: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioh_regs {
    pub regs: [ioh_reg_comn; 8],
    pub reserve1: [u32; 16],
    pub ioh_sel_reg: [u32; 4],
    pub reserve2: [u32; 11],
    pub srst: u32,
}

//
// struct ioh_gpio_reg_data - The register store data.
// @ien_reg:	To store contents of interrupt enable register.
// @imask_reg:	To store contents of interrupt mask regist
// @po_reg:	To store contents of PO register.
// @pm_reg:	To store contents of PM register.
// @im0_reg:	To store contents of interrupt mode regist0
// @im1_reg:	To store contents of interrupt mode regist1
// @use_sel_reg: To store contents of GPIO_USE_SEL0~3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioh_gpio_reg_data {
    pub ien_reg: u32,
    pub imask_reg: u32,
    pub po_reg: u32,
    pub pm_reg: u32,
    pub im0_reg: u32,
    pub im1_reg: u32,
    pub use_sel_reg: u32,
}

//
// struct ioh_gpio - GPIO private data structure.
// @base:			PCI base address of Memory mapped I/O register.
// @reg:			Memory mapped IOH GPIO register list.
// @dev:			Pointer to device structure.
// @gpio:			Data for GPIO infrastructure.
// @ioh_gpio_reg:		Memory mapped Register data is saved here
// when suspend.
// @gpio_use_sel:		Save GPIO_USE_SEL1~4 register for PM
// @ch:				Indicate GPIO channel
// @irq_base:		Save base of IRQ number for interrupt
// @spinlock:		Shared register access lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioh_gpio {
    pub base: *mut void __iomem,
    pub reg: *mut ioh_regs __iomem,
    pub dev: *mut device,
    pub gpio: gpio_chip,
    pub ioh_gpio_reg: ioh_gpio_reg_data,
    pub gpio_use_sel: u32,
    pub ch: c_int,
    pub irq_base: c_int,
    pub spinlock: *mut raw_spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioh_gpio_device {
    pub spinlock: raw_spinlock_t,
    pub chip: [ioh_gpio; 8],
}

    static const int num_ports[] = {6, 12, 16, 16, 15, 16, 16, 12};
#[no_mangle]
unsafe extern "C" fn ioh_gpio_set(gpio: *mut gpio_chip, nr: c_uint, val: c_int) -> c_int {
    static int ioh_gpio_set(struct gpio_chip *gpio, unsigned int nr, int val)
    {
    u32 reg_val;
    struct ioh_gpio *chip =	gpiochip_get_data(gpio);
    unsigned long flags;
    raw_spin_lock_irqsave(chip.spinlock, flags);
    reg_val = ioread32(&chip.reg.regs[chip.ch].po);
    if (val)
    reg_val |= BIT(nr);
    else
    reg_val &= ~BIT(nr);
    iowrite32(reg_val, &chip.reg.regs[chip.ch].po);
    raw_spin_unlock_irqrestore(chip.spinlock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioh_gpio_get(gpio: *mut gpio_chip, nr: unsigned) -> c_int {
    static int ioh_gpio_get(struct gpio_chip *gpio, unsigned nr)
    {
    struct ioh_gpio *chip =	gpiochip_get_data(gpio);
    return !!(ioread32(&chip.reg.regs[chip.ch].pi) & BIT(nr));
    }
    static int ioh_gpio_direction_output(struct gpio_chip *gpio, unsigned nr,
    int val)
    {
    struct ioh_gpio *chip =	gpiochip_get_data(gpio);
    u32 pm;
    u32 reg_val;
    unsigned long flags;
    raw_spin_lock_irqsave(chip.spinlock, flags);
    pm = ioread32(&chip.reg.regs[chip.ch].pm);
    pm &= BIT(num_ports[chip.ch]) - 1;
    pm |= BIT(nr);
    iowrite32(pm, &chip.reg.regs[chip.ch].pm);
    reg_val = ioread32(&chip.reg.regs[chip.ch].po);
    if (val)
    reg_val |= BIT(nr);
    else
    reg_val &= ~BIT(nr);
    iowrite32(reg_val, &chip.reg.regs[chip.ch].po);
    raw_spin_unlock_irqrestore(chip.spinlock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioh_gpio_direction_input(gpio: *mut gpio_chip, nr: unsigned) -> c_int {
    static int ioh_gpio_direction_input(struct gpio_chip *gpio, unsigned nr)
    {
    struct ioh_gpio *chip =	gpiochip_get_data(gpio);
    u32 pm;
    unsigned long flags;
    raw_spin_lock_irqsave(chip.spinlock, flags);
    pm = ioread32(&chip.reg.regs[chip.ch].pm);
    pm &= BIT(num_ports[chip.ch]) - 1;
    pm &= ~BIT(nr);
    iowrite32(pm, &chip.reg.regs[chip.ch].pm);
    raw_spin_unlock_irqrestore(chip.spinlock, flags);
    return 0;
    }
//
// Save register configuration and disable interrupts.
//
#[no_mangle]
unsafe extern "C" fn ioh_gpio_save_reg_conf(chip: *mut ioh_gpio) {
    static void ioh_gpio_save_reg_conf(struct ioh_gpio *chip)
    {
    int i;
    for (i = 0; i < 8; i ++, chip++) {
    chip.ioh_gpio_reg.po_reg =
    ioread32(&chip.reg.regs[chip.ch].po);
    chip.ioh_gpio_reg.pm_reg =
    ioread32(&chip.reg.regs[chip.ch].pm);
    chip.ioh_gpio_reg.ien_reg =
    ioread32(&chip.reg.regs[chip.ch].ien);
    chip.ioh_gpio_reg.imask_reg =
    ioread32(&chip.reg.regs[chip.ch].imask);
    chip.ioh_gpio_reg.im0_reg =
    ioread32(&chip.reg.regs[chip.ch].im_0);
    chip.ioh_gpio_reg.im1_reg =
    ioread32(&chip.reg.regs[chip.ch].im_1);
    if (i < 4)
    chip.ioh_gpio_reg.use_sel_reg =
    ioread32(&chip.reg.ioh_sel_reg[i]);
    }
    }
//
// This function restores the register configuration of the GPIO device.
//
#[no_mangle]
unsafe extern "C" fn ioh_gpio_restore_reg_conf(chip: *mut ioh_gpio) {
    static void ioh_gpio_restore_reg_conf(struct ioh_gpio *chip)
    {
    int i;
    for (i = 0; i < 8; i ++, chip++) {
    iowrite32(chip.ioh_gpio_reg.po_reg,
    &chip.reg.regs[chip.ch].po);
    iowrite32(chip.ioh_gpio_reg.pm_reg,
    &chip.reg.regs[chip.ch].pm);
    iowrite32(chip.ioh_gpio_reg.ien_reg,
    &chip.reg.regs[chip.ch].ien);
    iowrite32(chip.ioh_gpio_reg.imask_reg,
    &chip.reg.regs[chip.ch].imask);
    iowrite32(chip.ioh_gpio_reg.im0_reg,
    &chip.reg.regs[chip.ch].im_0);
    iowrite32(chip.ioh_gpio_reg.im1_reg,
    &chip.reg.regs[chip.ch].im_1);
    if (i < 4)
    iowrite32(chip.ioh_gpio_reg.use_sel_reg,
    &chip.reg.ioh_sel_reg[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn ioh_gpio_to_irq(gpio: *mut gpio_chip, offset: unsigned) -> c_int {
    static int ioh_gpio_to_irq(struct gpio_chip *gpio, unsigned offset)
    {
    struct ioh_gpio *chip = gpiochip_get_data(gpio);
    return chip.irq_base + offset;
    }
#[no_mangle]
unsafe extern "C" fn ioh_gpio_setup(chip: *mut ioh_gpio, num_port: c_int) {
    static void ioh_gpio_setup(struct ioh_gpio *chip, int num_port)
    {
    struct gpio_chip *gpio = &chip.gpio;
    gpio.label = dev_name(chip.dev);
    gpio.owner = THIS_MODULE;
    gpio.direction_input = ioh_gpio_direction_input;
    gpio.get = ioh_gpio_get;
    gpio.direction_output = ioh_gpio_direction_output;
    gpio.set = ioh_gpio_set;
    gpio.dbg_show = core::ptr::null_mut();
    gpio.base = -1;
    gpio.ngpio = num_port;
    gpio.can_sleep = false;
    gpio.to_irq = ioh_gpio_to_irq;
    }
#[no_mangle]
unsafe extern "C" fn ioh_irq_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int ioh_irq_type(struct irq_data *d, unsigned int type)
    {
    u32 im;
    void __iomem *im_reg;
    u32 ien;
    u32 im_pos;
    int ch;
    unsigned long flags;
    u32 val;
    let mut irq: c_int = d.irq;
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct ioh_gpio *chip = gc.private;
    ch = irq - chip.irq_base;
    if (irq <= chip.irq_base + 7) {
    im_reg = &chip.reg.regs[chip.ch].im_0;
    im_pos = ch;
    } else {
    im_reg = &chip.reg.regs[chip.ch].im_1;
    im_pos = ch - 8;
    }
    dev_dbg(chip.dev, "%s:irq=%d type=%d ch=%d pos=%d type=%d\n",
    __func__, irq, type, ch, im_pos, type);
    raw_spin_lock_irqsave(chip.spinlock, flags);
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    val = IOH_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    val = IOH_EDGE_FALLING;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    val = IOH_EDGE_BOTH;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    val = IOH_LEVEL_H;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    val = IOH_LEVEL_L;
    break;
    case IRQ_TYPE_PROBE:
    goto end;
    default:
    dev_warn(chip.dev, "%s: unknown type(%dd)",
    __func__, type);
    goto end;
    }
// Set interrupt mode
    im = ioread32(im_reg) & ~(IOH_IM_MASK << (im_pos * 4));
    iowrite32(im | (val << (im_pos * 4)), im_reg);
// iclr
    iowrite32(BIT(ch), &chip.reg.regs[chip.ch].iclr);
// IMASKCLR
    iowrite32(BIT(ch), &chip.reg.regs[chip.ch].imaskclr);
// Enable interrupt
    ien = ioread32(&chip.reg.regs[chip.ch].ien);
    iowrite32(ien | BIT(ch), &chip.reg.regs[chip.ch].ien);
    end:
    raw_spin_unlock_irqrestore(chip.spinlock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioh_irq_unmask(d: *mut irq_data) {
    static void ioh_irq_unmask(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct ioh_gpio *chip = gc.private;
    iowrite32(BIT(d.irq - chip.irq_base),
    &chip.reg.regs[chip.ch].imaskclr);
    }
#[no_mangle]
unsafe extern "C" fn ioh_irq_mask(d: *mut irq_data) {
    static void ioh_irq_mask(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct ioh_gpio *chip = gc.private;
    iowrite32(BIT(d.irq - chip.irq_base),
    &chip.reg.regs[chip.ch].imask);
    }
#[no_mangle]
unsafe extern "C" fn ioh_irq_disable(d: *mut irq_data) {
    static void ioh_irq_disable(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct ioh_gpio *chip = gc.private;
    unsigned long flags;
    u32 ien;
    raw_spin_lock_irqsave(chip.spinlock, flags);
    ien = ioread32(&chip.reg.regs[chip.ch].ien);
    ien &= ~BIT(d.irq - chip.irq_base);
    iowrite32(ien, &chip.reg.regs[chip.ch].ien);
    raw_spin_unlock_irqrestore(chip.spinlock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ioh_irq_enable(d: *mut irq_data) {
    static void ioh_irq_enable(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct ioh_gpio *chip = gc.private;
    unsigned long flags;
    u32 ien;
    raw_spin_lock_irqsave(chip.spinlock, flags);
    ien = ioread32(&chip.reg.regs[chip.ch].ien);
    ien |= BIT(d.irq - chip.irq_base);
    iowrite32(ien, &chip.reg.regs[chip.ch].ien);
    raw_spin_unlock_irqrestore(chip.spinlock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ioh_gpio_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ioh_gpio_handler(int irq, void *dev_id)
    {
    struct ioh_gpio *chip = dev_id;
    u32 reg_val;
    int i, j;
    let mut ret: c_int = IRQ_NONE;
    for (i = 0; i < 8; i++, chip++) {
    reg_val = ioread32(&chip.reg.regs[i].istatus);
    for (j = 0; j < num_ports[i]; j++) {
    if (reg_val & BIT(j)) {
    dev_dbg(chip.dev,
    "%s:[%d]:irq=%d status=0x%x\n",
    __func__, j, irq, reg_val);
    iowrite32(BIT(j),
    &chip.reg.regs[chip.ch].iclr);
    generic_handle_irq(chip.irq_base + j);
    ret = IRQ_HANDLED;
    }
    }
    }
    return ret;
    }
    static int ioh_gpio_alloc_generic_chip(struct ioh_gpio *chip,
    unsigned int irq_start,
    unsigned int num)
    {
    struct irq_chip_generic *gc;
    struct irq_chip_type *ct;
    int rv;
    gc = devm_irq_alloc_generic_chip(chip.dev, "ioh_gpio", 1, irq_start,
    chip.base, handle_simple_irq);
    if (!gc)
    return -ENOMEM;
    gc.private = chip;
    ct = gc.chip_types;
    ct.chip.irq_mask = ioh_irq_mask;
    ct.chip.irq_unmask = ioh_irq_unmask;
    ct.chip.irq_set_type = ioh_irq_type;
    ct.chip.irq_disable = ioh_irq_disable;
    ct.chip.irq_enable = ioh_irq_enable;
    rv = devm_irq_setup_generic_chip(chip.dev, gc, IRQ_MSK(num),
    IRQ_GC_INIT_MASK_CACHE,
    IRQ_NOREQUEST | IRQ_NOPROBE, 0);
    return rv;
    }
    static int ioh_gpio_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    int ret;
    int i, j;
    struct ioh_gpio *chip;
    struct ioh_gpio_device *priv;
    void __iomem *base;
    int irq_base;
    ret = pcim_enable_device(pdev);
    if (ret) {
    dev_err(dev, "%s : pcim_enable_device failed", __func__);
    return ret;
    }
    ret = pcim_iomap_regions(pdev, BIT(1), KBUILD_MODNAME);
    if (ret) {
    dev_err(dev, "pcim_iomap_regions failed-%d", ret);
    return ret;
    }
    base = pcim_iomap_table(pdev)[1];
    if (!base) {
    dev_err(dev, "%s : pcim_iomap_table failed", __func__);
    return -ENOMEM;
    }
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    raw_spin_lock_init(&priv.spinlock);
    chip = priv.chip;
    for (i = 0; i < 8; i++, chip++) {
    chip.dev = dev;
    chip.base = base;
    chip.reg = chip.base;
    chip.ch = i;
    chip.spinlock = &priv.spinlock;
    ioh_gpio_setup(chip, num_ports[i]);
    ret = devm_gpiochip_add_data(dev, &chip.gpio, chip);
    if (ret) {
    dev_err(dev, "IOH gpio: Failed to register GPIO\n");
    return ret;
    }
    }
    chip = priv.chip;
    for (j = 0; j < 8; j++, chip++) {
    irq_base = devm_irq_alloc_descs(dev, -1, IOH_IRQ_BASE,
    num_ports[j], NUMA_NO_NODE);
    if (irq_base < 0) {
    dev_warn(dev,
    "ml_ioh_gpio: Failed to get IRQ base num\n");
    return irq_base;
    }
    chip.irq_base = irq_base;
    ret = ioh_gpio_alloc_generic_chip(chip,
    irq_base, num_ports[j]);
    if (ret)
    return ret;
    }
    chip = priv.chip;
    ret = devm_request_irq(dev, pdev.irq, ioh_gpio_handler,
    IRQF_SHARED, KBUILD_MODNAME, chip);
    if (ret != 0) {
    dev_err(dev, "%s request_irq failed\n", __func__);
    return ret;
    }
    pci_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioh_gpio_suspend(dev: *mut device) -> c_int {
    static int ioh_gpio_suspend(struct device *dev)
    {
    struct ioh_gpio_device *priv = dev_get_drvdata(dev);
    unsigned long flags;
    raw_spin_lock_irqsave(&priv.spinlock, flags);
    ioh_gpio_save_reg_conf(priv.chip);
    raw_spin_unlock_irqrestore(&priv.spinlock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ioh_gpio_resume(dev: *mut device) -> c_int {
    static int ioh_gpio_resume(struct device *dev)
    {
    struct ioh_gpio_device *priv = dev_get_drvdata(dev);
    unsigned long flags;
    raw_spin_lock_irqsave(&priv.spinlock, flags);
    iowrite32(0x01, &priv.chip.reg.srst);
    iowrite32(0x00, &priv.chip.reg.srst);
    ioh_gpio_restore_reg_conf(priv.chip);
    raw_spin_unlock_irqrestore(&priv.spinlock, flags);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ioh_gpio_pm_ops, ioh_gpio_suspend, ioh_gpio_resume);
    static const struct pci_device_id ioh_gpio_pcidev_id[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_ROHM, 0x802E) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, ioh_gpio_pcidev_id);
    static struct pci_driver ioh_gpio_driver = {
    .name = "ml_ioh_gpio",
    .id_table = ioh_gpio_pcidev_id,
    .probe = ioh_gpio_probe,
    .driver = {
    .pm = pm_sleep_ptr(&ioh_gpio_pm_ops),
    },
    };
    module_pci_driver(ioh_gpio_driver);
    MODULE_DESCRIPTION("OKI SEMICONDUCTOR ML-IOH series GPIO Driver");
    MODULE_LICENSE("GPL");
