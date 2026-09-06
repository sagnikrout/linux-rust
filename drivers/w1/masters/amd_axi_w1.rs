//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/amd_axi_w1.c
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
// amd_axi_w1 - AMD 1Wire programmable logic bus host driver
//
// Copyright (C) 2022-2023 Advanced Micro Devices, Inc. All Rights Reserved.
//

// 1-wire AMD IP definition
pub const AXIW1_IPID: c_uint = 0x10ee4453;
// Registers offset
pub const AXIW1_INST_REG: c_uint = 0x0;
pub const AXIW1_CTRL_REG: c_uint = 0x4;
pub const AXIW1_IRQE_REG: c_uint = 0x8;
pub const AXIW1_STAT_REG: c_uint = 0xC;
pub const AXIW1_DATA_REG: c_uint = 0x10;
pub const AXIW1_IPVER_REG: c_uint = 0x18;
pub const AXIW1_IPID_REG: c_uint = 0x1C;
// Instructions
pub const AXIW1_INITPRES: c_uint = 0x0800;
pub const AXIW1_READBIT: c_uint = 0x0C00;
pub const AXIW1_WRITEBIT: c_uint = 0x0E00;
pub const AXIW1_READBYTE: c_uint = 0x0D00;
pub const AXIW1_WRITEBYTE: c_uint = 0x0F00;
// Status flag masks

// Control flag

pub const AXI_CLEAR: c_int = 0;

// Interrupt Enable

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_axi_w1_local {
    pub dev: *mut device,
    pub base_addr: *mut void __iomem,
    pub irq: c_int,
    pub /: *mut *mut atomic_t flag; / Set on IRQ, cleared once serviced,
    pub wait_queue: wait_queue_head_t,
    pub bus_host: w1_bus_master,
}

//
// amd_axi_w1_wait_irq_interruptible_timeout() - Wait for IRQ with timeout.
//
// @amd_axi_w1_local:	Pointer to device structure
// @IRQ:		IRQ channel to wait on
//
// Return:		%0 - OK, %-EINTR - Interrupted, %-EBUSY - Timed out
//
    static int amd_axi_w1_wait_irq_interruptible_timeout(struct amd_axi_w1_local *amd_axi_w1_local,
    u32 IRQ)
    {
    int ret;
// Enable the IRQ requested and wait for flag to indicate it's been triggered
    iowrite32(IRQ, amd_axi_w1_local.base_addr + AXIW1_IRQE_REG);
    ret = wait_event_interruptible_timeout(amd_axi_w1_local.wait_queue,
    atomic_read(&amd_axi_w1_local.flag) != 0,
    AXIW1_TIMEOUT);
    if (ret < 0) {
    dev_err(amd_axi_w1_local.dev, "Wait IRQ Interrupted\n");
    return -EINTR;
    }
    if (!ret) {
    dev_err(amd_axi_w1_local.dev, "Wait IRQ Timeout\n");
    return -EBUSY;
    }
    atomic_set(&amd_axi_w1_local.flag, 0);
    return 0;
    }
//
// amd_axi_w1_touch_bit() - Performs the touch-bit function - write a 0 or 1 and reads the level.
//
// @data:	Pointer to device structure
// @bit:	The level to write
//
// Return:	The level read
//
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_touch_bit(data: *mut c_void, bit: u8) -> u8 {
    static u8 amd_axi_w1_touch_bit(void *data, u8 bit)
    {
    struct amd_axi_w1_local *amd_axi_w1_local = data;
    let mut val: u8 = 0;
    int rc;
// Wait for READY signal to be 1 to ensure 1-wire IP is ready
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_READY) == 0) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local,
    AXIW1_READY_IRQ_EN);
    if (rc < 0)
    return 1; /* Callee doesn't test for error. Return inactive bus state */
    }
    if (bit)
// Read. Write read Bit command in register 0
    iowrite32(AXIW1_READBIT, amd_axi_w1_local.base_addr + AXIW1_INST_REG);
    else
// Write. Write tx Bit command in instruction register with bit to transmit
    iowrite32(AXIW1_WRITEBIT + (bit & 0x01),
    amd_axi_w1_local.base_addr + AXIW1_INST_REG);
// Write Go signal and clear control reset signal in control register
    iowrite32(AXIW1_GO, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
// Wait for done signal to be 1
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_DONE) != 1) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local, AXIW1_DONE_IRQ_EN);
    if (rc < 0)
    return 1; /* Callee doesn't test for error. Return inactive bus state */
    }
// If read, Retrieve data from register
    if (bit)
    val = (u8)(ioread32(amd_axi_w1_local.base_addr + AXIW1_DATA_REG) & AXIW1_READDATA);
// Clear Go signal in register 1
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
    return val;
    }
//
// amd_axi_w1_read_byte - Performs the read byte function.
//
// @data:	Pointer to device structure
// Return:	The value read
//
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_read_byte(data: *mut c_void) -> u8 {
    static u8 amd_axi_w1_read_byte(void *data)
    {
    struct amd_axi_w1_local *amd_axi_w1_local = data;
    let mut val: u8 = 0;
    int rc;
// Wait for READY signal to be 1 to ensure 1-wire IP is ready
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_READY) == 0) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local,
    AXIW1_READY_IRQ_EN);
    if (rc < 0)
    return 0xFF; /* Return inactive bus state */
    }
// Write read Byte command in instruction register
    iowrite32(AXIW1_READBYTE, amd_axi_w1_local.base_addr + AXIW1_INST_REG);
// Write Go signal and clear control reset signal in control register
    iowrite32(AXIW1_GO, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
// Wait for done signal to be 1
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_DONE) != 1) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local, AXIW1_DONE_IRQ_EN);
    if (rc < 0)
    return 0xFF; /* Return inactive bus state */
    }
// Retrieve LSB bit in data register to get RX byte
    val = (u8)(ioread32(amd_axi_w1_local.base_addr + AXIW1_DATA_REG) & 0x000000FF);
// Clear Go signal in control register
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
    return val;
    }
//
// amd_axi_w1_write_byte - Performs the write byte function.
//
// @data:	The ds2482 channel pointer
// @val:	The value to write
//
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_write_byte(data: *mut c_void, val: u8) {
    static void amd_axi_w1_write_byte(void *data, u8 val)
    {
    struct amd_axi_w1_local *amd_axi_w1_local = data;
    int rc;
// Wait for READY signal to be 1 to ensure 1-wire IP is ready
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_READY) == 0) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local,
    AXIW1_READY_IRQ_EN);
    if (rc < 0)
    return;
    }
// Write tx Byte command in instruction register with bit to transmit
    iowrite32(AXIW1_WRITEBYTE + val, amd_axi_w1_local.base_addr + AXIW1_INST_REG);
// Write Go signal and clear control reset signal in register 1
    iowrite32(AXIW1_GO, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
// Wait for done signal to be 1
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_DONE) != 1) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local,
    AXIW1_DONE_IRQ_EN);
    if (rc < 0)
    return;
    }
// Clear Go signal in control register
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
    }
//
// amd_axi_w1_reset_bus() - Issues a reset bus sequence.
//
// @data:	the bus host data struct
// Return:	0=Device present, 1=No device present or error
//
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_reset_bus(data: *mut c_void) -> u8 {
    static u8 amd_axi_w1_reset_bus(void *data)
    {
    struct amd_axi_w1_local *amd_axi_w1_local = data;
    let mut val: u8 = 0;
    int rc;
// Reset 1-wire Axi IP
    iowrite32(AXI_RESET, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
// Wait for READY signal to be 1 to ensure 1-wire IP is ready
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_READY) == 0) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local,
    AXIW1_READY_IRQ_EN);
    if (rc < 0)
    return 1; /* Something went wrong with the hardware */
    }
// Write Initialization command in instruction register
    iowrite32(AXIW1_INITPRES, amd_axi_w1_local.base_addr + AXIW1_INST_REG);
// Write Go signal and clear control reset signal in register 1
    iowrite32(AXIW1_GO, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
// Wait for done signal to be 1
    while ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_DONE) != 1) {
    rc = amd_axi_w1_wait_irq_interruptible_timeout(amd_axi_w1_local, AXIW1_DONE_IRQ_EN);
    if (rc < 0)
    return 1; /* Something went wrong with the hardware */
    }
// Retrieve MSB bit in status register to get failure bit
    if ((ioread32(amd_axi_w1_local.base_addr + AXIW1_STAT_REG) & AXIW1_PRESENCE) != 0)
    val = 1;
// Clear Go signal in control register
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
    return val;
    }
// Reset the 1-wire AXI IP. Put the IP in reset state and clear registers
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_reset(amd_axi_w1_local: *mut amd_axi_w1_local) {
    static void amd_axi_w1_reset(struct amd_axi_w1_local *amd_axi_w1_local)
    {
    iowrite32(AXI_RESET, amd_axi_w1_local.base_addr + AXIW1_CTRL_REG);
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_INST_REG);
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_IRQE_REG);
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_STAT_REG);
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_DATA_REG);
    }
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_irq(irq: c_int, lp: *mut c_void) -> irqreturn_t {
    static irqreturn_t amd_axi_w1_irq(int irq, void *lp)
    {
    struct amd_axi_w1_local *amd_axi_w1_local = lp;
// Reset interrupt trigger
    iowrite32(AXI_CLEAR, amd_axi_w1_local.base_addr + AXIW1_IRQE_REG);
    atomic_set(&amd_axi_w1_local.flag, 1);
    wake_up_interruptible(&amd_axi_w1_local.wait_queue);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_probe(pdev: *mut platform_device) -> c_int {
    static int amd_axi_w1_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct amd_axi_w1_local *lp;
    struct clk *clk;
    u32 ver_major, ver_minor;
    int val, rc = 0;
    lp = devm_kzalloc(dev, sizeof(*lp), GFP_KERNEL);
    if (!lp)
    return -ENOMEM;
    lp.dev = dev;
    lp.base_addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lp.base_addr))
    return PTR_ERR(lp.base_addr);
    lp.irq = platform_get_irq(pdev, 0);
    if (lp.irq < 0)
    return lp.irq;
    rc = devm_request_irq(dev, lp.irq, &amd_axi_w1_irq, IRQF_TRIGGER_HIGH, DRIVER_NAME, lp);
    if (rc)
    return rc;
// Initialize wait queue and flag
    init_waitqueue_head(&lp.wait_queue);
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
// Verify IP presence in HW
    if (ioread32(lp.base_addr + AXIW1_IPID_REG) != AXIW1_IPID) {
    dev_err(dev, "AMD 1-wire IP not detected in hardware\n");
    return -ENODEV;
    }
//
// Allow for future driver expansion supporting new hardware features
// This driver currently only supports hardware 1.x, but include logic
// to detect if a potentially incompatible future version is used
// by reading major version ID. It is highly undesirable for new IP versions
// to break the API, but this code will at least allow for graceful failure
// should that happen. Future new features can be enabled by hardware
// incrementing the minor version and augmenting the driver to detect capability
// using the minor version number
//
    val = ioread32(lp.base_addr + AXIW1_IPVER_REG);
    ver_major = FIELD_GET(AXIW1_MAJORVER_MASK, val);
    ver_minor = FIELD_GET(AXIW1_MINORVER_MASK, val);
    if (ver_major != 1) {
    dev_err(dev, "AMD AXI W1 host version %u.%u is not supported by this driver",
    ver_major, ver_minor);
    return -ENODEV;
    }
    lp.bus_host.data = lp;
    lp.bus_host.touch_bit = amd_axi_w1_touch_bit;
    lp.bus_host.read_byte = amd_axi_w1_read_byte;
    lp.bus_host.write_byte = amd_axi_w1_write_byte;
    lp.bus_host.reset_bus = amd_axi_w1_reset_bus;
    amd_axi_w1_reset(lp);
    platform_set_drvdata(pdev, lp);
    rc = w1_add_master_device(&lp.bus_host);
    if (rc) {
    dev_err(dev, "Could not add host device\n");
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_axi_w1_remove(pdev: *mut platform_device) {
    static void amd_axi_w1_remove(struct platform_device *pdev)
    {
    struct amd_axi_w1_local *lp = platform_get_drvdata(pdev);
    w1_remove_master_device(&lp.bus_host);
    }
    static const struct of_device_id amd_axi_w1_of_match[] = {
    { .compatible = "amd,axi-1wire-host" },
    { /* end of list */ },
    };
    MODULE_DEVICE_TABLE(of, amd_axi_w1_of_match);
    static struct platform_driver amd_axi_w1_driver = {
    .probe = amd_axi_w1_probe,
    .remove = amd_axi_w1_remove,
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = amd_axi_w1_of_match,
    },
    };
    module_platform_driver(amd_axi_w1_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Kris Chaplin <kris.chaplin@amd.com>");
    MODULE_DESCRIPTION("Driver for AMD AXI 1 Wire IP core");
