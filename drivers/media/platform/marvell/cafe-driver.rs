//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/marvell/cafe-driver.c
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
// A driver for the CMOS camera controller in the Marvell 88ALP01 "cafe"
// multifunction chip.  Currently works with the Omnivision OV7670
// sensor.
//
// The data sheet for this device can be found at:
// http://wiki.laptop.org/images/5/5c/88ALP01_Datasheet_July_2007.pdf
//
// Copyright 2006-11 One Laptop Per Child Association, Inc.
// Copyright 2006-11 Jonathan Corbet <corbet@lwn.net>
// Copyright 2018 Lubomir Rintel <lkundrak@v3.sk>
//
// Written by Jonathan Corbet, corbet@lwn.net.
//
// v4l2_device/v4l2_subdev conversion by:
// Copyright (C) 2009 Hans Verkuil <hverkuil@kernel.org>
//

pub const CAFE_VERSION: c_uint = 0x000002;
//
// Parameters.
//
    MODULE_AUTHOR("Jonathan Corbet <corbet@lwn.net>");
    MODULE_DESCRIPTION("Marvell 88ALP01 CMOS Camera Controller driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cafe_camera {
    pub /: *mut *mut int registered; / Fully initialized?,
    pub mcam: mcam_camera,
    pub pdev: *mut pci_dev,
    pub i2c_adapter: *mut i2c_adapter,
    pub /: *mut *mut wait_queue_head_t smbus_wait; / Waiting on i2c events,
}

//
// Most of the camera controller registers are defined in mcam-core.h,
// but the Cafe platform has some additional registers of its own;
// they are described here.
//
// "General purpose register" has a couple of GPIOs used for sensor
// power and reset on OLPC XO 1.0 systems.
//
pub const REG_GPR: c_uint = 0xb4;
pub const GPR_C1EN: c_uint = 0x00000020	/* Pad 1 (power down) enable */;
pub const GPR_C0EN: c_uint = 0x00000010	/* Pad 0 (reset) enable */;
pub const GPR_C1: c_uint = 0x00000002	/* Control 1 value */;
//
// Control 0 is wired to reset on OLPC machines.  For ov7x sensors,
// it is active low.
//
pub const GPR_C0: c_uint = 0x00000001	/* Control 0 value */;
//
// These registers control the SMBUS module for communicating
// with the sensor.
//
pub const REG_TWSIC0: c_uint = 0xb8	/* TWSI (smbus) control 0 */;
pub const TWSIC0_EN: c_uint = 0x00000001	/* TWSI enable */;
pub const TWSIC0_MODE: c_uint = 0x00000002	/* 1 = 16-bit, 0 = 8-bit */;
pub const TWSIC0_SID: c_uint = 0x000003fc	/* Slave ID */;
//
// Subtle trickery: the slave ID field starts with bit 2.  But the
// Linux i2c stack wants to treat the bottommost bit as a separate
// read/write bit, which is why slave ID's are usually presented
// >>1.  For consistency with that behavior, we shift over three
// bits instead of two.
//
pub const TWSIC0_SID_SHIFT: c_int = 3;
pub const TWSIC0_CLKDIV: c_uint = 0x0007fc00	/* Clock divider */;
pub const TWSIC0_MASKACK: c_uint = 0x00400000	/* Mask ack from sensor */;
pub const TWSIC0_OVMAGIC: c_uint = 0x00800000	/* Make it work on OV sensors */;
pub const REG_TWSIC1: c_uint = 0xbc	/* TWSI control 1 */;
pub const TWSIC1_DATA: c_uint = 0x0000ffff	/* Data to/from camchip */;
pub const TWSIC1_ADDR: c_uint = 0x00ff0000	/* Address (register) */;
pub const TWSIC1_ADDR_SHIFT: c_int = 16;
pub const TWSIC1_READ: c_uint = 0x01000000	/* Set for read op */;
pub const TWSIC1_WSTAT: c_uint = 0x02000000	/* Write status */;
pub const TWSIC1_RVALID: c_uint = 0x04000000	/* Read data valid */;
pub const TWSIC1_ERROR: c_uint = 0x08000000	/* Something screwed up */;
//
// Here's the weird global control registers
//
pub const REG_GL_CSR: c_uint = 0x3004  /* Control/status register */;
pub const GCSR_SRS: c_uint = 0x00000001	/* SW Reset set */;
pub const GCSR_SRC: c_uint = 0x00000002	/* SW Reset clear */;
pub const GCSR_MRS: c_uint = 0x00000004	/* Master reset set */;
pub const GCSR_MRC: c_uint = 0x00000008	/* HW Reset clear */;
pub const GCSR_CCIC_EN: c_uint = 0x00004000    /* CCIC Clock enable */;
pub const REG_GL_IMASK: c_uint = 0x300c  /* Interrupt mask register */;
pub const GIMSK_CCIC_EN: c_uint = 0x00000004    /* CCIC Interrupt enable */;
pub const REG_GL_FCR: c_uint = 0x3038	/* GPIO functional control register */;
pub const GFCR_GPIO_ON: c_uint = 0x08		/* Camera GPIO enabled */;
pub const REG_GL_GPIOR: c_uint = 0x315c	/* GPIO register */;
pub const GGPIO_OUT: c_uint = 0x80000	/* GPIO output */;
pub const GGPIO_VAL: c_uint = 0x00008	/* Output pin value */;

//
// Debugging and related.
//

    dev_err(&(cam).pdev.dev, fmt, ##arg);

    dev_warn(&(cam).pdev.dev, fmt, ##arg);
// --------------------------------------------------------------------
//
// The I2C/SMBUS interface to the camera itself starts here.  The
// controller handles SMBUS itself, presenting a relatively simple register
// interface; all we have to do is to tell it where to route the data.
//

#[no_mangle]
unsafe extern "C" fn cafe_smbus_write_done(mcam: *mut mcam_camera) -> c_int {
    static int cafe_smbus_write_done(struct mcam_camera *mcam)
    {
    unsigned long flags;
    int c1;
//
// We must delay after the interrupt, or the controller gets confused
// and never does give us good status.  Fortunately, we don't do this
// often.
//
    udelay(20);
    spin_lock_irqsave(&mcam.dev_lock, flags);
    c1 = mcam_reg_read(mcam, REG_TWSIC1);
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
    return (c1 & (TWSIC1_WSTAT|TWSIC1_ERROR)) != TWSIC1_WSTAT;
    }
    static int cafe_smbus_write_data(struct cafe_camera *cam,
    u16 addr, u8 command, u8 value)
    {
    unsigned int rval;
    unsigned long flags;
    struct mcam_camera *mcam = &cam.mcam;
    spin_lock_irqsave(&mcam.dev_lock, flags);
    rval = TWSIC0_EN | ((addr << TWSIC0_SID_SHIFT) & TWSIC0_SID);
    rval |= TWSIC0_OVMAGIC;  /* Make OV sensors work */
//
// Marvell sez set clkdiv to all 1's for now.
//
    rval |= TWSIC0_CLKDIV;
    mcam_reg_write(mcam, REG_TWSIC0, rval);
    (void) mcam_reg_read(mcam, REG_TWSIC1); /* force write */
    rval = value | ((command << TWSIC1_ADDR_SHIFT) & TWSIC1_ADDR);
    mcam_reg_write(mcam, REG_TWSIC1, rval);
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
// Unfortunately, reading TWSIC1 too soon after sending a command
// causes the device to die.
// Use a busy-wait because we often send a large quantity of small
// commands at-once; using msleep() would cause a lot of context
// switches which take longer than 2ms, resulting in a noticeable
// boot-time and capture-start delays.
//
    mdelay(2);
//
// Another sad fact is that sometimes, commands silently complete but
// cafe_smbus_write_done() never becomes aware of this.
// This happens at random and appears to possible occur with any
// command.
// We don't understand why this is. We work around this issue
// with the timeout in the wait below, assuming that all commands
// complete within the timeout.
//
    wait_event_timeout(cam.smbus_wait, cafe_smbus_write_done(mcam),
    CAFE_SMBUS_TIMEOUT);
    spin_lock_irqsave(&mcam.dev_lock, flags);
    rval = mcam_reg_read(mcam, REG_TWSIC1);
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
    if (rval & TWSIC1_WSTAT) {
    cam_err(cam, "SMBUS write (%02x/%02x/%02x) timed out\n", addr,
    command, value);
    return -EIO;
    }
    if (rval & TWSIC1_ERROR) {
    cam_err(cam, "SMBUS write (%02x/%02x/%02x) error\n", addr,
    command, value);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cafe_smbus_read_done(mcam: *mut mcam_camera) -> c_int {
    static int cafe_smbus_read_done(struct mcam_camera *mcam)
    {
    unsigned long flags;
    int c1;
//
// We must delay after the interrupt, or the controller gets confused
// and never does give us good status.  Fortunately, we don't do this
// often.
//
    udelay(20);
    spin_lock_irqsave(&mcam.dev_lock, flags);
    c1 = mcam_reg_read(mcam, REG_TWSIC1);
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
    return c1 & (TWSIC1_RVALID|TWSIC1_ERROR);
    }
    static int cafe_smbus_read_data(struct cafe_camera *cam,
    u16 addr, u8 command, u8 *value)
    {
    unsigned int rval;
    unsigned long flags;
    struct mcam_camera *mcam = &cam.mcam;
    spin_lock_irqsave(&mcam.dev_lock, flags);
    rval = TWSIC0_EN | ((addr << TWSIC0_SID_SHIFT) & TWSIC0_SID);
    rval |= TWSIC0_OVMAGIC; /* Make OV sensors work */
//
// Marvel sez set clkdiv to all 1's for now.
//
    rval |= TWSIC0_CLKDIV;
    mcam_reg_write(mcam, REG_TWSIC0, rval);
    (void) mcam_reg_read(mcam, REG_TWSIC1); /* force write */
    rval = TWSIC1_READ | ((command << TWSIC1_ADDR_SHIFT) & TWSIC1_ADDR);
    mcam_reg_write(mcam, REG_TWSIC1, rval);
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
    wait_event_timeout(cam.smbus_wait,
    cafe_smbus_read_done(mcam), CAFE_SMBUS_TIMEOUT);
    spin_lock_irqsave(&mcam.dev_lock, flags);
    rval = mcam_reg_read(mcam, REG_TWSIC1);
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
    if (rval & TWSIC1_ERROR) {
    cam_err(cam, "SMBUS read (%02x/%02x) error\n", addr, command);
    return -EIO;
    }
    if (!(rval & TWSIC1_RVALID)) {
    cam_err(cam, "SMBUS read (%02x/%02x) timed out\n", addr,
    command);
    return -EIO;
    }
// value = rval & 0xff;
    return 0;
    }
//
// Perform a transfer over SMBUS.  This thing is called under
// the i2c bus lock, so we shouldn't race with ourselves...
//
    static int cafe_smbus_xfer(struct i2c_adapter *adapter, u16 addr,
    unsigned short flags, char rw, u8 command,
    int size, union i2c_smbus_data *data)
    {
    struct cafe_camera *cam = i2c_get_adapdata(adapter);
    let mut ret: c_int = -EINVAL;
//
// This interface would appear to only do byte data ops.  OK
// it can do word too, but the cam chip has no use for that.
//
    if (size != I2C_SMBUS_BYTE_DATA) {
    cam_err(cam, "funky xfer size %d\n", size);
    return -EINVAL;
    }
    if (rw == I2C_SMBUS_WRITE)
    ret = cafe_smbus_write_data(cam, addr, command, data.byte);
#[no_mangle]
pub unsafe extern "C" fn if(I2C_SMBUS_READ: rw ==) -> else {
    else if (rw == I2C_SMBUS_READ)
    ret = cafe_smbus_read_data(cam, addr, command, &data.byte);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cafe_smbus_enable_irq(cam: *mut cafe_camera) {
    static void cafe_smbus_enable_irq(struct cafe_camera *cam)
    {
    unsigned long flags;
    spin_lock_irqsave(&cam.mcam.dev_lock, flags);
    mcam_reg_set_bit(&cam.mcam, REG_IRQMASK, TWSIIRQS);
    spin_unlock_irqrestore(&cam.mcam.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn cafe_smbus_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 cafe_smbus_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_SMBUS_READ_BYTE_DATA  |
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA;
    }
    static const struct i2c_algorithm cafe_smbus_algo = {
    .smbus_xfer = cafe_smbus_xfer,
    .functionality = cafe_smbus_func
    };
#[no_mangle]
unsafe extern "C" fn cafe_smbus_setup(cam: *mut cafe_camera) -> c_int {
    static int cafe_smbus_setup(struct cafe_camera *cam)
    {
    struct i2c_adapter *adap;
    int ret;
    adap = kzalloc_obj(*adap);
    if (adap == core::ptr::null_mut())
    return -ENOMEM;
    adap.owner = THIS_MODULE;
    adap.algo = &cafe_smbus_algo;
    strscpy(adap.name, "cafe_ccic", sizeof(adap.name));
    adap.dev.parent = &cam.pdev.dev;
    i2c_set_adapdata(adap, cam);
    ret = i2c_add_adapter(adap);
    if (ret) {
    printk(KERN_ERR "Unable to register cafe i2c adapter\n");
    kfree(adap);
    return ret;
    }
    cam.i2c_adapter = adap;
    cafe_smbus_enable_irq(cam);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cafe_smbus_shutdown(cam: *mut cafe_camera) {
    static void cafe_smbus_shutdown(struct cafe_camera *cam)
    {
    i2c_del_adapter(cam.i2c_adapter);
    kfree(cam.i2c_adapter);
    }
//
// Controller-level stuff
//
#[no_mangle]
unsafe extern "C" fn cafe_ctlr_init(mcam: *mut mcam_camera) {
    static void cafe_ctlr_init(struct mcam_camera *mcam)
    {
    unsigned long flags;
    spin_lock_irqsave(&mcam.dev_lock, flags);
//
// Added magic to bring up the hardware on the B-Test board
//
    mcam_reg_write(mcam, 0x3038, 0x8);
    mcam_reg_write(mcam, 0x315c, 0x80008);
//
// Go through the dance needed to wake the device up.
// Note that these registers are global and shared
// with the NAND and SD devices.  Interaction between the
// three still needs to be examined.
//
    mcam_reg_write(mcam, REG_GL_CSR, GCSR_SRS|GCSR_MRS); /* Needed? */
    mcam_reg_write(mcam, REG_GL_CSR, GCSR_SRC|GCSR_MRC);
    mcam_reg_write(mcam, REG_GL_CSR, GCSR_SRC|GCSR_MRS);
//
// Here we must wait a bit for the controller to come around.
//
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
    msleep(5);
    spin_lock_irqsave(&mcam.dev_lock, flags);
    mcam_reg_write(mcam, REG_GL_CSR, GCSR_CCIC_EN|GCSR_SRC|GCSR_MRC);
    mcam_reg_set_bit(mcam, REG_GL_IMASK, GIMSK_CCIC_EN);
//
// Mask all interrupts.
//
    mcam_reg_write(mcam, REG_IRQMASK, 0);
    spin_unlock_irqrestore(&mcam.dev_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn cafe_ctlr_power_up(mcam: *mut mcam_camera) -> c_int {
    static int cafe_ctlr_power_up(struct mcam_camera *mcam)
    {
//
// Part one of the sensor dance: turn the global
// GPIO signal on.
//
    mcam_reg_write(mcam, REG_GL_FCR, GFCR_GPIO_ON);
    mcam_reg_write(mcam, REG_GL_GPIOR, GGPIO_OUT|GGPIO_VAL);
//
// Put the sensor into operational mode (assumes OLPC-style
// wiring).  Control 0 is reset - set to 1 to operate.
// Control 1 is power down, set to 0 to operate.
//
    mcam_reg_write(mcam, REG_GPR, GPR_C1EN|GPR_C0EN); /* pwr up, reset */
    mcam_reg_write(mcam, REG_GPR, GPR_C1EN|GPR_C0EN|GPR_C0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cafe_ctlr_power_down(mcam: *mut mcam_camera) {
    static void cafe_ctlr_power_down(struct mcam_camera *mcam)
    {
    mcam_reg_write(mcam, REG_GPR, GPR_C1EN|GPR_C0EN|GPR_C1);
    mcam_reg_write(mcam, REG_GL_FCR, GFCR_GPIO_ON);
    mcam_reg_write(mcam, REG_GL_GPIOR, GGPIO_OUT);
    }
//
// The platform interrupt handler.
//
#[no_mangle]
unsafe extern "C" fn cafe_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cafe_irq(int irq, void *data)
    {
    struct cafe_camera *cam = data;
    struct mcam_camera *mcam = &cam.mcam;
    unsigned int irqs, handled;
    spin_lock(&mcam.dev_lock);
    irqs = mcam_reg_read(mcam, REG_IRQSTAT);
    handled = cam.registered && mccic_irq(mcam, irqs);
    if (irqs & TWSIIRQS) {
    mcam_reg_write(mcam, REG_IRQSTAT, TWSIIRQS);
    wake_up(&cam.smbus_wait);
    handled = 1;
    }
    spin_unlock(&mcam.dev_lock);
    return IRQ_RETVAL(handled);
    }
// --------------------------------------------------------------------------
    static struct ov7670_config sensor_cfg = {
//
// Exclude QCIF mode, because it only captures a tiny portion
// of the sensor FOV
//
    .min_width = 320,
    .min_height = 240,
//
// Set the clock speed for the XO 1; I don't believe this
// driver has ever run anywhere else.
//
    .clock_speed = 45,
    .use_smbus = 1,
    };
    static struct i2c_board_info ov7670_info = {
    .type = "ov7670",
    .addr = 0x42 >> 1,
    .platform_data = &sensor_cfg,
    };
// --------------------------------------------------------------------------
//
// PCI interface stuff.
//
    static int cafe_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    int ret;
    struct cafe_camera *cam;
    struct mcam_camera *mcam;
    struct v4l2_async_connection *asd;
    struct i2c_client *i2c_dev;
//
// Start putting together one of our big camera structures.
//
    ret = -ENOMEM;
    cam = kzalloc_obj(struct cafe_camera);
    if (cam == core::ptr::null_mut())
    goto out;
    pci_set_drvdata(pdev, cam);
    cam.pdev = pdev;
    mcam = &cam.mcam;
    mcam.chip_id = MCAM_CAFE;
    spin_lock_init(&mcam.dev_lock);
    init_waitqueue_head(&cam.smbus_wait);
    mcam.plat_power_up = cafe_ctlr_power_up;
    mcam.plat_power_down = cafe_ctlr_power_down;
    mcam.dev = &pdev.dev;
//
// Vmalloc mode for buffers is traditional with this driver.
// We *might* be able to run DMA_contig, especially on a system
// with CMA in it.
//
    mcam.buffer_mode = B_vmalloc;
//
// Get set up on the PCI bus.
//
    ret = pci_enable_device(pdev);
    if (ret)
    goto out_free;
    pci_set_master(pdev);
    ret = -EIO;
    mcam.regs = pci_iomap(pdev, 0, 0);
    if (!mcam.regs) {
    printk(KERN_ERR "Unable to ioremap cafe-ccic regs\n");
    goto out_disable;
    }
    mcam.regs_size = pci_resource_len(pdev, 0);
    ret = request_irq(pdev.irq, cafe_irq, IRQF_SHARED, "cafe-ccic", cam);
    if (ret)
    goto out_iounmap;
//
// Initialize the controller.
//
    cafe_ctlr_init(mcam);
//
// Set up I2C/SMBUS communications.  We have to drop the mutex here
// because the sensor could attach in this call chain, leading to
// unsightly deadlocks.
//
    ret = cafe_smbus_setup(cam);
    if (ret)
    goto out_pdown;
    ret = v4l2_device_register(mcam.dev, &mcam.v4l2_dev);
    if (ret)
    goto out_smbus_shutdown;
    v4l2_async_nf_init(&mcam.notifier, &mcam.v4l2_dev);
    asd = v4l2_async_nf_add_i2c(&mcam.notifier,
    i2c_adapter_id(cam.i2c_adapter),
    ov7670_info.addr,
    struct v4l2_async_connection);
    if (IS_ERR(asd)) {
    ret = PTR_ERR(asd);
    goto out_v4l2_device_unregister;
    }
    ret = mccic_register(mcam);
    if (ret)
    goto out_v4l2_device_unregister;
    clkdev_create(mcam.mclk, "xclk", "%d-%04x",
    i2c_adapter_id(cam.i2c_adapter), ov7670_info.addr);
    i2c_dev = i2c_new_client_device(cam.i2c_adapter, &ov7670_info);
    if (IS_ERR(i2c_dev)) {
    ret = PTR_ERR(i2c_dev);
    goto out_mccic_shutdown;
    }
    cam.registered = 1;
    return 0;
    out_mccic_shutdown:
    mccic_shutdown(mcam);
    out_v4l2_device_unregister:
    v4l2_device_unregister(&mcam.v4l2_dev);
    out_smbus_shutdown:
    cafe_smbus_shutdown(cam);
    out_pdown:
    cafe_ctlr_power_down(mcam);
    free_irq(pdev.irq, cam);
    out_iounmap:
    pci_iounmap(pdev, mcam.regs);
    out_disable:
    pci_disable_device(pdev);
    out_free:
    kfree(cam);
    out:
    return ret;
    }
//
// Shut down an initialized device
//
#[no_mangle]
unsafe extern "C" fn cafe_shutdown(cam: *mut cafe_camera) {
    static void cafe_shutdown(struct cafe_camera *cam)
    {
    mccic_shutdown(&cam.mcam);
    v4l2_device_unregister(&cam.mcam.v4l2_dev);
    cafe_smbus_shutdown(cam);
    free_irq(cam.pdev.irq, cam);
    pci_iounmap(cam.pdev, cam.mcam.regs);
    }
#[no_mangle]
unsafe extern "C" fn cafe_pci_remove(pdev: *mut pci_dev) {
    static void cafe_pci_remove(struct pci_dev *pdev)
    {
    struct cafe_camera *cam = pci_get_drvdata(pdev);
    if (cam == core::ptr::null_mut()) {
    printk(KERN_WARNING "pci_remove on unknown pdev %p\n", pdev);
    return;
    }
    cafe_shutdown(cam);
    pci_disable_device(pdev);
    kfree(cam);
    }
//
// Basic power management.
//
#[no_mangle]
unsafe extern "C" fn cafe_pci_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cafe_pci_suspend(struct device *dev)
    {
    struct cafe_camera *cam = dev_get_drvdata(dev);
    mccic_suspend(&cam.mcam);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cafe_pci_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cafe_pci_resume(struct device *dev)
    {
    struct cafe_camera *cam = dev_get_drvdata(dev);
    cafe_ctlr_init(&cam.mcam);
    return mccic_resume(&cam.mcam);
    }
    static const struct pci_device_id cafe_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL,
    PCI_DEVICE_ID_MARVELL_88ALP01_CCIC) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, cafe_ids);
    static SIMPLE_DEV_PM_OPS(cafe_pci_pm_ops, cafe_pci_suspend, cafe_pci_resume);
    static struct pci_driver cafe_pci_driver = {
    .name = "cafe1000-ccic",
    .id_table = cafe_ids,
    .probe = cafe_pci_probe,
    .remove = cafe_pci_remove,
    .driver.pm = &cafe_pci_pm_ops,
    };
#[no_mangle]
unsafe extern "C" fn cafe_init() -> int __init {
    static int __init cafe_init(void)
    {
    int ret;
    printk(KERN_NOTICE "Marvell M88ALP01 'CAFE' Camera Controller version %d\n",
    CAFE_VERSION);
    ret = pci_register_driver(&cafe_pci_driver);
    if (ret) {
    printk(KERN_ERR "Unable to register cafe_ccic driver\n");
    goto out;
    }
    ret = 0;
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cafe_exit() -> void __exit {
    static void __exit cafe_exit(void)
    {
    pci_unregister_driver(&cafe_pci_driver);
    }
    module_init(cafe_init);
    module_exit(cafe_exit);
