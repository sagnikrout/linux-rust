//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-emev2.c
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


// SPDX-License-Identifier: GPL-2.0
//
// I2C driver for the Renesas EMEV2 SoC
//
// Copyright (C) 2015 Wolfram Sang <wsa@sang-engineering.com>
// Copyright 2013 Codethink Ltd.
// Copyright 2010-2015 Renesas Electronics Corporation
//

// I2C Registers
pub const I2C_OFS_IICACT0: c_uint = 0x00	/* start */;
pub const I2C_OFS_IIC0: c_uint = 0x04	/* shift */;
pub const I2C_OFS_IICC0: c_uint = 0x08	/* control */;
pub const I2C_OFS_SVA0: c_uint = 0x0c	/* slave address */;
pub const I2C_OFS_IICCL0: c_uint = 0x10	/* clock select */;
pub const I2C_OFS_IICX0: c_uint = 0x14	/* extension */;
pub const I2C_OFS_IICS0: c_uint = 0x18	/* status */;
pub const I2C_OFS_IICSE0: c_uint = 0x1c	/* status For emulation */;
pub const I2C_OFS_IICF0: c_uint = 0x20	/* IIC flag */;
// I2C IICACT0 Masks
pub const I2C_BIT_IICE0: c_uint = 0x0001;
// I2C IICC0 Masks
pub const I2C_BIT_LREL0: c_uint = 0x0040;
pub const I2C_BIT_WREL0: c_uint = 0x0020;
pub const I2C_BIT_SPIE0: c_uint = 0x0010;
pub const I2C_BIT_WTIM0: c_uint = 0x0008;
pub const I2C_BIT_ACKE0: c_uint = 0x0004;
pub const I2C_BIT_STT0: c_uint = 0x0002;
pub const I2C_BIT_SPT0: c_uint = 0x0001;
// I2C IICCL0 Masks
pub const I2C_BIT_SMC0: c_uint = 0x0008;
pub const I2C_BIT_DFC0: c_uint = 0x0004;
// I2C IICSE0 Masks
pub const I2C_BIT_MSTS0: c_uint = 0x0080;
pub const I2C_BIT_ALD0: c_uint = 0x0040;
pub const I2C_BIT_EXC0: c_uint = 0x0020;
pub const I2C_BIT_COI0: c_uint = 0x0010;
pub const I2C_BIT_TRC0: c_uint = 0x0008;
pub const I2C_BIT_ACKD0: c_uint = 0x0004;
pub const I2C_BIT_STD0: c_uint = 0x0002;
pub const I2C_BIT_SPD0: c_uint = 0x0001;
// I2C IICF0 Masks
pub const I2C_BIT_STCF: c_uint = 0x0080;
pub const I2C_BIT_IICBSY: c_uint = 0x0040;
pub const I2C_BIT_STCEN: c_uint = 0x0002;
pub const I2C_BIT_IICRSV: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_i2c_device {
    pub base: *mut void __iomem,
    pub adap: i2c_adapter,
    pub msg_done: completion,
    pub slave: *mut i2c_client,
    pub irq: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn em_clear_set_bit(priv: *mut em_i2c_device, clear: u8, set: u8, reg: u8) {
    static inline void em_clear_set_bit(struct em_i2c_device *priv, u8 clear, u8 set, u8 reg)
    {
    writeb((readb(priv.base + reg) & ~clear) | set, priv.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_wait_for_event(priv: *mut em_i2c_device) -> c_int {
    static int em_i2c_wait_for_event(struct em_i2c_device *priv)
    {
    unsigned long time_left;
    int status;
    reinit_completion(&priv.msg_done);
    time_left = wait_for_completion_timeout(&priv.msg_done, priv.adap.timeout);
    if (!time_left)
    return -ETIMEDOUT;
    status = readb(priv.base + I2C_OFS_IICSE0);
    return status & I2C_BIT_ALD0 ? -EAGAIN : status;
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_stop(priv: *mut em_i2c_device) {
    static void em_i2c_stop(struct em_i2c_device *priv)
    {
// Send Stop condition
    em_clear_set_bit(priv, 0, I2C_BIT_SPT0 | I2C_BIT_SPIE0, I2C_OFS_IICC0);
// Wait for stop condition
    em_i2c_wait_for_event(priv);
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_reset(adap: *mut i2c_adapter) {
    static void em_i2c_reset(struct i2c_adapter *adap)
    {
    struct em_i2c_device *priv = i2c_get_adapdata(adap);
    int retr;
// If I2C active
    if (readb(priv.base + I2C_OFS_IICACT0) & I2C_BIT_IICE0) {
// Disable I2C operation
    writeb(0, priv.base + I2C_OFS_IICACT0);
    retr = 1000;
    while (readb(priv.base + I2C_OFS_IICACT0) == 1 && retr)
    retr--;
    WARN_ON(retr == 0);
    }
// Transfer mode set
    writeb(I2C_BIT_DFC0, priv.base + I2C_OFS_IICCL0);
// Can Issue start without detecting a stop, Reservation disabled.
    writeb(I2C_BIT_STCEN | I2C_BIT_IICRSV, priv.base + I2C_OFS_IICF0);
// I2C enable, 9 bit interrupt mode
    writeb(I2C_BIT_WTIM0, priv.base + I2C_OFS_IICC0);
// Enable I2C operation
    writeb(I2C_BIT_IICE0, priv.base + I2C_OFS_IICACT0);
    retr = 1000;
    while (readb(priv.base + I2C_OFS_IICACT0) == 0 && retr)
    retr--;
    WARN_ON(retr == 0);
    }
    static int __em_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msg,
    int stop)
    {
    struct em_i2c_device *priv = i2c_get_adapdata(adap);
    int count, status, read = !!(msg.flags & I2C_M_RD);
// Send start condition
    em_clear_set_bit(priv, 0, I2C_BIT_ACKE0 | I2C_BIT_WTIM0, I2C_OFS_IICC0);
    em_clear_set_bit(priv, 0, I2C_BIT_STT0, I2C_OFS_IICC0);
// Send slave address and R/W type
    writeb(i2c_8bit_addr_from_msg(msg), priv.base + I2C_OFS_IIC0);
// Wait for transaction
    status = em_i2c_wait_for_event(priv);
    if (status < 0)
    goto out_reset;
// Received NACK (result of setting slave address and R/W)
    if (!(status & I2C_BIT_ACKD0)) {
    em_i2c_stop(priv);
    goto out;
    }
// Extra setup for read transactions
    if (read) {
// 8 bit interrupt mode
    em_clear_set_bit(priv, I2C_BIT_WTIM0, I2C_BIT_ACKE0, I2C_OFS_IICC0);
    em_clear_set_bit(priv, I2C_BIT_WTIM0, I2C_BIT_WREL0, I2C_OFS_IICC0);
// Wait for transaction
    status = em_i2c_wait_for_event(priv);
    if (status < 0)
    goto out_reset;
    }
// Send / receive data
    for (count = 0; count < msg.len; count++) {
    if (read) { /* Read transaction */
    msg.buf[count] = readb(priv.base + I2C_OFS_IIC0);
    em_clear_set_bit(priv, 0, I2C_BIT_WREL0, I2C_OFS_IICC0);
    } else { /* Write transaction */
// Received NACK
    if (!(status & I2C_BIT_ACKD0)) {
    em_i2c_stop(priv);
    goto out;
    }
// Write data
    writeb(msg.buf[count], priv.base + I2C_OFS_IIC0);
    }
// Wait for R/W transaction
    status = em_i2c_wait_for_event(priv);
    if (status < 0)
    goto out_reset;
    }
    if (stop)
    em_i2c_stop(priv);
    return count;
    out_reset:
    em_i2c_reset(adap);
    out:
    return status < 0 ? status : -ENXIO;
    }
    static int em_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    struct em_i2c_device *priv = i2c_get_adapdata(adap);
    int ret, i;
    if (readb(priv.base + I2C_OFS_IICF0) & I2C_BIT_IICBSY)
    return -EAGAIN;
    for (i = 0; i < num; i++) {
    ret = __em_i2c_xfer(adap, &msgs[i], (i == (num - 1)));
    if (ret < 0)
    return ret;
    }
// I2C transfer completed
    return num;
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_slave_irq(priv: *mut em_i2c_device) -> bool {
    static bool em_i2c_slave_irq(struct em_i2c_device *priv)
    {
    u8 status, value;
    enum i2c_slave_event event;
    int ret;
    if (!priv.slave)
    return false;
    status = readb(priv.base + I2C_OFS_IICSE0);
// Extension code, do not participate
    if (status & I2C_BIT_EXC0) {
    em_clear_set_bit(priv, 0, I2C_BIT_LREL0, I2C_OFS_IICC0);
    return true;
    }
// Stop detected, we don't know if it's for slave or master
    if (status & I2C_BIT_SPD0) {
// Notify slave device
    i2c_slave_event(priv.slave, I2C_SLAVE_STOP, &value);
// Pretend we did not handle the interrupt
    return false;
    }
// Only handle interrupts addressed to us
    if (!(status & I2C_BIT_COI0))
    return false;
// Enable stop interrupts
    em_clear_set_bit(priv, 0, I2C_BIT_SPIE0, I2C_OFS_IICC0);
// Transmission or Reception
    if (status & I2C_BIT_TRC0) {
    if (status & I2C_BIT_ACKD0) {
// 9 bit interrupt mode
    em_clear_set_bit(priv, 0, I2C_BIT_WTIM0, I2C_OFS_IICC0);
// Send data
    event = status & I2C_BIT_STD0 ?
    I2C_SLAVE_READ_REQUESTED :
    I2C_SLAVE_READ_PROCESSED;
    i2c_slave_event(priv.slave, event, &value);
    writeb(value, priv.base + I2C_OFS_IIC0);
    } else {
// NACK, stop transmitting
    em_clear_set_bit(priv, 0, I2C_BIT_LREL0, I2C_OFS_IICC0);
    }
    } else {
// 8 bit interrupt mode
    em_clear_set_bit(priv, I2C_BIT_WTIM0, I2C_BIT_ACKE0,
    I2C_OFS_IICC0);
    em_clear_set_bit(priv, I2C_BIT_WTIM0, I2C_BIT_WREL0,
    I2C_OFS_IICC0);
    if (status & I2C_BIT_STD0) {
    i2c_slave_event(priv.slave, I2C_SLAVE_WRITE_REQUESTED,
    &value);
    } else {
// Recv data
    value = readb(priv.base + I2C_OFS_IIC0);
    ret = i2c_slave_event(priv.slave,
    I2C_SLAVE_WRITE_RECEIVED, &value);
    if (ret < 0)
    em_clear_set_bit(priv, I2C_BIT_ACKE0, 0,
    I2C_OFS_IICC0);
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_irq_handler(this_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t em_i2c_irq_handler(int this_irq, void *dev_id)
    {
    struct em_i2c_device *priv = dev_id;
    if (em_i2c_slave_irq(priv))
    return IRQ_HANDLED;
    complete(&priv.msg_done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 em_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL | I2C_FUNC_SLAVE;
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_reg_slave(slave: *mut i2c_client) -> c_int {
    static int em_i2c_reg_slave(struct i2c_client *slave)
    {
    struct em_i2c_device *priv = i2c_get_adapdata(slave.adapter);
    if (priv.slave)
    return -EBUSY;
    if (slave.flags & I2C_CLIENT_TEN)
    return -EAFNOSUPPORT;
    priv.slave = slave;
// Set slave address
    writeb(slave.addr << 1, priv.base + I2C_OFS_SVA0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_unreg_slave(slave: *mut i2c_client) -> c_int {
    static int em_i2c_unreg_slave(struct i2c_client *slave)
    {
    struct em_i2c_device *priv = i2c_get_adapdata(slave.adapter);
    WARN_ON(!priv.slave);
    writeb(0, priv.base + I2C_OFS_SVA0);
//
// Wait for interrupt to finish. New slave irqs cannot happen because we
// cleared the slave address and, thus, only extension codes will be
// detected which do not use the slave ptr.
//
    synchronize_irq(priv.irq);
    priv.slave = core::ptr::null_mut();
    return 0;
    }
    static const struct i2c_algorithm em_i2c_algo = {
    .xfer = em_i2c_xfer,
    .functionality = em_i2c_func,
    .reg_slave = em_i2c_reg_slave,
    .unreg_slave = em_i2c_unreg_slave,
    };
#[no_mangle]
unsafe extern "C" fn em_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int em_i2c_probe(struct platform_device *pdev)
    {
    struct em_i2c_device *priv;
    struct clk *sclk;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    strscpy(priv.adap.name, "EMEV2 I2C", sizeof(priv.adap.name));
    sclk = devm_clk_get_enabled(&pdev.dev, "sclk");
    if (IS_ERR(sclk))
    return PTR_ERR(sclk);
    priv.adap.timeout = msecs_to_jiffies(100);
    priv.adap.retries = 5;
    priv.adap.dev.parent = &pdev.dev;
    priv.adap.algo = &em_i2c_algo;
    priv.adap.owner = THIS_MODULE;
    priv.adap.dev.of_node = pdev.dev.of_node;
    init_completion(&priv.msg_done);
    platform_set_drvdata(pdev, priv);
    i2c_set_adapdata(&priv.adap, priv);
    em_i2c_reset(&priv.adap);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    priv.irq = ret;
    ret = devm_request_irq(&pdev.dev, priv.irq, em_i2c_irq_handler, 0,
    "em_i2c", priv);
    if (ret)
    return ret;
    ret = i2c_add_adapter(&priv.adap);
    if (ret)
    return ret;
    dev_info(&pdev.dev, "Added i2c controller %d, irq %d\n", priv.adap.nr,
    priv.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em_i2c_remove(dev: *mut platform_device) {
    static void em_i2c_remove(struct platform_device *dev)
    {
    struct em_i2c_device *priv = platform_get_drvdata(dev);
    i2c_del_adapter(&priv.adap);
    }
    static const struct of_device_id em_i2c_ids[] = {
    { .compatible = "renesas,iic-emev2", },
    { }
    };
    static struct platform_driver em_i2c_driver = {
    .probe = em_i2c_probe,
    .remove = em_i2c_remove,
    .driver = {
    .name = "em-i2c",
    .of_match_table = em_i2c_ids,
    }
    };
    module_platform_driver(em_i2c_driver);
    MODULE_DESCRIPTION("EMEV2 I2C bus driver");
    MODULE_AUTHOR("Ian Molton");
    MODULE_AUTHOR("Wolfram Sang <wsa@sang-engineering.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DEVICE_TABLE(of, em_i2c_ids);
