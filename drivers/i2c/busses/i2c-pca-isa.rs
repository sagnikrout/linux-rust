//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-pca-isa.c
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
// i2c-pca-isa.c driver for PCA9564 on ISA boards
// Copyright (C) 2004 Arcom Control Systems
// Copyright (C) 2008 Pengutronix
//

pub const IO_SIZE: c_int = 4;
    static unsigned long base;
    let mut irq: static int = -1;
// Data sheet recommends 59kHz for 100kHz operation due to variation
// in the actual clock rate
    let mut clock: static int = 59000;
    static struct i2c_adapter pca_isa_ops;
    static wait_queue_head_t pca_wait;
#[no_mangle]
unsafe extern "C" fn pca_isa_writebyte(pd: *mut c_void, reg: c_int, val: c_int) {
    static void pca_isa_writebyte(void *pd, int reg, int val)
    {

    static char *names[] = { "T/O", "DAT", "ADR", "CON" };
    printk(KERN_DEBUG "*** write %s at %#lx <= %#04x\n", names[reg],
    base+reg, val);

    outb(val, base+reg);
    }
#[no_mangle]
unsafe extern "C" fn pca_isa_readbyte(pd: *mut c_void, reg: c_int) -> c_int {
    static int pca_isa_readbyte(void *pd, int reg)
    {
    let mut res: c_int = inb(base+reg);

    {
    static char *names[] = { "STA", "DAT", "ADR", "CON" };
    printk(KERN_DEBUG "*** read  %s => %#04x\n", names[reg], res);
    }

    return res;
    }
#[no_mangle]
unsafe extern "C" fn pca_isa_waitforcompletion(pd: *mut c_void) -> c_int {
    static int pca_isa_waitforcompletion(void *pd)
    {
    unsigned long timeout;
    long ret;
    if (irq > -1) {
    ret = wait_event_timeout(pca_wait,
    pca_isa_readbyte(pd, I2C_PCA_CON)
    & I2C_PCA_CON_SI, pca_isa_ops.timeout);
    } else {
// Do polling
    timeout = jiffies + pca_isa_ops.timeout;
    do {
    ret = time_before(jiffies, timeout);
    if (pca_isa_readbyte(pd, I2C_PCA_CON)
    & I2C_PCA_CON_SI)
    break;
    udelay(100);
    } while (ret);
    }
    return ret > 0;
    }
#[no_mangle]
unsafe extern "C" fn pca_isa_resetchip(pd: *mut c_void) {
    static void pca_isa_resetchip(void *pd)
    {
// apparently only an external reset will do it. not a lot can be done
    printk(KERN_WARNING DRIVER ": Haven't figured out how to do a reset yet\n");
    }
#[no_mangle]
unsafe extern "C" fn pca_handler(this_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    wake_up(&pca_wait);
    return IRQ_HANDLED;
    }
    static struct i2c_algo_pca_data pca_isa_data = {
// .data intentionally left NULL, not needed with ISA
    .write_byte		= pca_isa_writebyte,
    .read_byte		= pca_isa_readbyte,
    .wait_for_completion_cb	= pca_isa_waitforcompletion,
    .reset_chip		= pca_isa_resetchip,
    };
    static struct i2c_adapter pca_isa_ops = {
    .owner          = THIS_MODULE,
    .algo_data	= &pca_isa_data,
    .name		= "PCA9564/PCA9665 ISA Adapter",
    .timeout	= HZ,
    };
#[no_mangle]
unsafe extern "C" fn pca_isa_match(dev: *mut device, id: c_uint) -> c_int {
    static int pca_isa_match(struct device *dev, unsigned int id)
    {
    let mut match: c_int = base != 0;
    if (match) {
    if (irq <= -1)
    dev_warn(dev, "Using polling mode (specify irq)\n");
    } else
    dev_err(dev, "Please specify I/O base\n");
    return match;
    }
#[no_mangle]
unsafe extern "C" fn pca_isa_probe(dev: *mut device, id: c_uint) -> c_int {
    static int pca_isa_probe(struct device *dev, unsigned int id)
    {
    init_waitqueue_head(&pca_wait);
    dev_info(dev, "i/o base %#08lx. irq %d\n", base, irq);

    if (check_legacy_ioport(base)) {
    dev_err(dev, "I/O address %#08lx is not available\n", base);
    goto out;
    }

    if (!request_region(base, IO_SIZE, "i2c-pca-isa")) {
    dev_err(dev, "I/O address %#08lx is in use\n", base);
    goto out;
    }
    if (irq > -1) {
    if (request_irq(irq, pca_handler, 0, "i2c-pca-isa", &pca_isa_ops) < 0) {
    dev_err(dev, "Request irq%d failed\n", irq);
    goto out_region;
    }
    }
    pca_isa_data.i2c_clock = clock;
    if (i2c_pca_add_bus(&pca_isa_ops) < 0) {
    dev_err(dev, "Failed to add i2c bus\n");
    goto out_irq;
    }
    return 0;
    out_irq:
    if (irq > -1)
    free_irq(irq, &pca_isa_ops);
    out_region:
    release_region(base, IO_SIZE);
    out:
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn pca_isa_remove(dev: *mut device, id: c_uint) {
    static void pca_isa_remove(struct device *dev, unsigned int id)
    {
    i2c_del_adapter(&pca_isa_ops);
    if (irq > -1) {
    disable_irq(irq);
    free_irq(irq, &pca_isa_ops);
    }
    release_region(base, IO_SIZE);
    }
    static struct isa_driver pca_isa_driver = {
    .match		= pca_isa_match,
    .probe		= pca_isa_probe,
    .remove		= pca_isa_remove,
    .driver = {
    .owner	= THIS_MODULE,
    .name	= DRIVER,
    }
    };
    MODULE_AUTHOR("Ian Campbell <icampbell@arcom.com>");
    MODULE_DESCRIPTION("ISA base PCA9564/PCA9665 driver");
    MODULE_LICENSE("GPL");
    module_param_hw(base, ulong, ioport, 0);
    MODULE_PARM_DESC(base, "I/O base address");
    module_param_hw(irq, int, irq, 0);
    MODULE_PARM_DESC(irq, "IRQ");
    module_param(clock, int, 0);
    MODULE_PARM_DESC(clock, "Clock rate in hertz.\n\t\t"
    "For PCA9564: 330000,288000,217000,146000,"
    "88000,59000,44000,36000\n"
    "\t\tFor PCA9665:\tStandard: 60300 - 100099\n"
    "\t\t\t\tFast: 100100 - 400099\n"
    "\t\t\t\tFast+: 400100 - 10000099\n"
    "\t\t\t\tTurbo: Up to 1265800");
    module_isa_driver(pca_isa_driver, 1);
