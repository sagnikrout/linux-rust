//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-elektor.c
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
// -------------------------------------------------------------------------
// i2c-elektor.c i2c-hw access for PCF8584 style isa bus adaptes
// -------------------------------------------------------------------------
// Copyright (C) 1995-97 Simon G. Vogl
    1998-99 Hans Berglund
//
// -------------------------------------------------------------------------
// With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and even
    Frodo Looijaard <frodol@dds.nl> */
// Partially rewriten by Oleg I. Vdovikin for mmapped support of
    for Alpha Processor Inc. UP-2000(+) boards */

pub const DEFAULT_BASE: c_uint = 0x330;
    static int base;
    static u8 __iomem *base_iomem;
    static int irq;
    let mut clock: static int = 0x1c;
    let mut own: static int = 0x55;
    static int mmapped;
// vdovikin: removed static struct i2c_pcf_isa gpi; code -
    this module in real supports only one device, due to missing arguments
    in some functions, called from the algo-pcf module. Sometimes it's
    need to be rewriten - but for now just remove this for simpler reading */
    static wait_queue_head_t pcf_wait;
    static int pcf_pending;
    static DEFINE_SPINLOCK(lock);
    static struct i2c_adapter pcf_isa_ops;
// ----- local functions ----------------------------------------------
#[no_mangle]
unsafe extern "C" fn pcf_isa_setbyte(data: *mut c_void, ctl: c_int, val: c_int) {
    static void pcf_isa_setbyte(void *data, int ctl, int val)
    {
    u8 __iomem *address = ctl ? (base_iomem + 1) : base_iomem;
// enable irq if any specified for serial operation
    if (ctl && irq && (val & I2C_PCF_ESO)) {
    val |= I2C_PCF_ENI;
    }
    pr_debug("%s: Write %p 0x%02X\n", pcf_isa_ops.name, address, val);
    iowrite8(val, address);

// API UP2000 needs some hardware fudging to make the write stick
    iowrite8(val, address);

    }
#[no_mangle]
unsafe extern "C" fn pcf_isa_getbyte(data: *mut c_void, ctl: c_int) -> c_int {
    static int pcf_isa_getbyte(void *data, int ctl)
    {
    u8 __iomem *address = ctl ? (base_iomem + 1) : base_iomem;
    let mut val: c_int = ioread8(address);
    pr_debug("%s: Read %p 0x%02X\n", pcf_isa_ops.name, address, val);
    return (val);
    }
#[no_mangle]
unsafe extern "C" fn pcf_isa_getown(data: *mut c_void) -> c_int {
    static int pcf_isa_getown(void *data)
    {
    return (own);
    }
#[no_mangle]
unsafe extern "C" fn pcf_isa_getclock(data: *mut c_void) -> c_int {
    static int pcf_isa_getclock(void *data)
    {
    return (clock);
    }
#[no_mangle]
unsafe extern "C" fn pcf_isa_waitforpin(data: *mut c_void) {
    static void pcf_isa_waitforpin(void *data)
    {
    DEFINE_WAIT(wait);
    let mut timeout: c_int = 2;
    unsigned long flags;
    if (irq > 0) {
    spin_lock_irqsave(&lock, flags);
    if (pcf_pending == 0) {
    spin_unlock_irqrestore(&lock, flags);
    prepare_to_wait(&pcf_wait, &wait, TASK_INTERRUPTIBLE);
    if (schedule_timeout(timeout*HZ)) {
    spin_lock_irqsave(&lock, flags);
    if (pcf_pending == 1) {
    pcf_pending = 0;
    }
    spin_unlock_irqrestore(&lock, flags);
    }
    finish_wait(&pcf_wait, &wait);
    } else {
    pcf_pending = 0;
    spin_unlock_irqrestore(&lock, flags);
    }
    } else {
    udelay(100);
    }
    }
#[no_mangle]
unsafe extern "C" fn pcf_isa_handler(this_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    spin_lock(&lock);
    pcf_pending = 1;
    spin_unlock(&lock);
    wake_up_interruptible(&pcf_wait);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pcf_isa_init() -> c_int {
    static int pcf_isa_init(void)
    {
    if (!mmapped) {
    if (!request_region(base, 2, pcf_isa_ops.name)) {
    printk(KERN_ERR "%s: requested I/O region (%#x:2) is "
    "in use\n", pcf_isa_ops.name, base);
    return -ENODEV;
    }
    base_iomem = ioport_map(base, 2);
    if (!base_iomem) {
    printk(KERN_ERR "%s: remap of I/O region %#x failed\n",
    pcf_isa_ops.name, base);
    release_region(base, 2);
    return -ENODEV;
    }
    } else {
    if (!request_mem_region(base, 2, pcf_isa_ops.name)) {
    printk(KERN_ERR "%s: requested memory region (%#x:2) "
    "is in use\n", pcf_isa_ops.name, base);
    return -ENODEV;
    }
    base_iomem = ioremap(base, 2);
    if (base_iomem == core::ptr::null_mut()) {
    printk(KERN_ERR "%s: remap of memory region %#x "
    "failed\n", pcf_isa_ops.name, base);
    release_mem_region(base, 2);
    return -ENODEV;
    }
    }
    pr_debug("%s: registers %#x remapped to %p\n", pcf_isa_ops.name, base,
    base_iomem);
    if (irq > 0) {
    if (request_irq(irq, pcf_isa_handler, 0, pcf_isa_ops.name,
    core::ptr::null_mut()) < 0) {
    printk(KERN_ERR "%s: Request irq%d failed\n",
    pcf_isa_ops.name, irq);
    irq = 0;
    } else
    enable_irq(irq);
    }
    return 0;
    }
// ------------------------------------------------------------------------
// Encapsulate the above functions in the correct operations structure.
// This is only done when more than one hardware adapter is supported.
//
    static struct i2c_algo_pcf_data pcf_isa_data = {
    .setpcf	    = pcf_isa_setbyte,
    .getpcf	    = pcf_isa_getbyte,
    .getown	    = pcf_isa_getown,
    .getclock   = pcf_isa_getclock,
    .waitforpin = pcf_isa_waitforpin,
    };
    static struct i2c_adapter pcf_isa_ops = {
    .owner		= THIS_MODULE,
    .class		= I2C_CLASS_HWMON,
    .algo_data	= &pcf_isa_data,
    .name		= "i2c-elektor",
    };
#[no_mangle]
unsafe extern "C" fn elektor_match(dev: *mut device, id: c_uint) -> c_int {
    static int elektor_match(struct device *dev, unsigned int id)
    {

// check to see we have memory mapped PCF8584 connected to the
    Cypress cy82c693 PCI-ISA bridge as on UP2000 board */
    if (base == 0) {
    struct pci_dev *cy693_dev;
    cy693_dev = pci_get_device(PCI_VENDOR_ID_CONTAQ,
    PCI_DEVICE_ID_CONTAQ_82C693, core::ptr::null_mut());
    if (cy693_dev) {
    unsigned char config;
// yeap, we've found cypress, let's check config
    if (!pci_read_config_byte(cy693_dev, 0x47, &config)) {
    dev_dbg(dev, "found cy82c693, config "
    "register 0x47 = 0x%02x\n", config);
// UP2000 board has this register set to 0xe1,
    but the most significant bit as seems can be
    reset during the proper initialisation
    sequence if guys from API decides to do that
    (so, we can even enable Tsunami Pchip
    window for the upper 1 Gb) */
// so just check for ROMCS at 0xe0000,
    ROMCS enabled for writes
    and external XD Bus buffer in use. */
    if ((config & 0x7f) == 0x61) {
// seems to be UP2000 like board
    base = 0xe0000;
    mmapped = 1;
// UP2000 drives ISA with
    8.25 MHz (PCI/4) clock
    (this can be read from cypress) */
    clock = I2C_PCF_CLK | I2C_PCF_TRNS90;
    dev_info(dev, "found API UP2000 like "
    "board, will probe PCF8584 "
    "later\n");
    }
    }
    pci_dev_put(cy693_dev);
    }
    }

// sanity checks for mmapped I/O
    if (mmapped && base < 0xc8000) {
    dev_err(dev, "incorrect base address (%#x) specified "
    "for mmapped I/O\n", base);
    return 0;
    }
    if (base == 0) {
    base = DEFAULT_BASE;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn elektor_probe(dev: *mut device, id: c_uint) -> c_int {
    static int elektor_probe(struct device *dev, unsigned int id)
    {
    init_waitqueue_head(&pcf_wait);
    if (pcf_isa_init())
    return -ENODEV;
    pcf_isa_ops.dev.parent = dev;
    if (i2c_pcf_add_bus(&pcf_isa_ops) < 0)
    goto fail;
    dev_info(dev, "found device at %#x\n", base);
    return 0;
    fail:
    if (irq > 0) {
    disable_irq(irq);
    free_irq(irq, core::ptr::null_mut());
    }
    if (!mmapped) {
    ioport_unmap(base_iomem);
    release_region(base, 2);
    } else {
    iounmap(base_iomem);
    release_mem_region(base, 2);
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn elektor_remove(dev: *mut device, id: c_uint) {
    static void elektor_remove(struct device *dev, unsigned int id)
    {
    i2c_del_adapter(&pcf_isa_ops);
    if (irq > 0) {
    disable_irq(irq);
    free_irq(irq, core::ptr::null_mut());
    }
    if (!mmapped) {
    ioport_unmap(base_iomem);
    release_region(base, 2);
    } else {
    iounmap(base_iomem);
    release_mem_region(base, 2);
    }
    }
    static struct isa_driver i2c_elektor_driver = {
    .match		= elektor_match,
    .probe		= elektor_probe,
    .remove		= elektor_remove,
    .driver = {
    .owner	= THIS_MODULE,
    .name	= "i2c-elektor",
    },
    };
    MODULE_AUTHOR("Hans Berglund <hb@spacetec.no>");
    MODULE_DESCRIPTION("I2C-Bus adapter routines for PCF8584 ISA bus adapter");
    MODULE_LICENSE("GPL");
    module_param_hw(base, int, ioport_or_iomem, 0);
    module_param_hw(irq, int, irq, 0);
    module_param(clock, int, 0);
    module_param(own, int, 0);
    module_param_hw(mmapped, int, other, 0);
    module_isa_driver(i2c_elektor_driver, 1);
