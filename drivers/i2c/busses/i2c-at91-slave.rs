//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-at91-slave.c
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
// i2c slave support for Atmel's AT91 Two-Wire Interface (TWI)
//
// Copyright (C) 2017 Juergen Fitschen <me@jue.yt>
//

#[no_mangle]
unsafe extern "C" fn atmel_twi_interrupt_slave(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t atmel_twi_interrupt_slave(int irq, void *dev_id)
    {
    struct at91_twi_dev *dev = dev_id;
    let mut status: unsigned = at91_twi_read(dev, AT91_TWI_SR);
    let mut irqstatus: unsigned = status & at91_twi_read(dev, AT91_TWI_IMR);
    u8 value;
    if (!irqstatus)
    return IRQ_NONE;
// slave address has been detected on I2C bus
    if (irqstatus & AT91_TWI_SVACC) {
    if (status & AT91_TWI_SVREAD) {
    i2c_slave_event(dev.slave,
    I2C_SLAVE_READ_REQUESTED, &value);
    writeb_relaxed(value, dev.base + AT91_TWI_THR);
    at91_twi_write(dev, AT91_TWI_IER,
    AT91_TWI_TXRDY | AT91_TWI_EOSACC);
    } else {
    i2c_slave_event(dev.slave,
    I2C_SLAVE_WRITE_REQUESTED, &value);
    at91_twi_write(dev, AT91_TWI_IER,
    AT91_TWI_RXRDY | AT91_TWI_EOSACC);
    }
    at91_twi_write(dev, AT91_TWI_IDR, AT91_TWI_SVACC);
    }
// byte transmitted to remote master
    if (irqstatus & AT91_TWI_TXRDY) {
    i2c_slave_event(dev.slave, I2C_SLAVE_READ_PROCESSED, &value);
    writeb_relaxed(value, dev.base + AT91_TWI_THR);
    }
// byte received from remote master
    if (irqstatus & AT91_TWI_RXRDY) {
    value = readb_relaxed(dev.base + AT91_TWI_RHR);
    i2c_slave_event(dev.slave, I2C_SLAVE_WRITE_RECEIVED, &value);
    }
// master sent stop
    if (irqstatus & AT91_TWI_EOSACC) {
    at91_twi_write(dev, AT91_TWI_IDR,
    AT91_TWI_TXRDY | AT91_TWI_RXRDY | AT91_TWI_EOSACC);
    at91_twi_write(dev, AT91_TWI_IER, AT91_TWI_SVACC);
    i2c_slave_event(dev.slave, I2C_SLAVE_STOP, &value);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn at91_reg_slave(slave: *mut i2c_client) -> c_int {
    static int at91_reg_slave(struct i2c_client *slave)
    {
    struct at91_twi_dev *dev = i2c_get_adapdata(slave.adapter);
    if (dev.slave)
    return -EBUSY;
    if (slave.flags & I2C_CLIENT_TEN)
    return -EAFNOSUPPORT;
// Make sure twi_clk doesn't get turned off!
    pm_runtime_get_sync(dev.dev);
    dev.slave = slave;
    dev.smr = AT91_TWI_SMR_SADR(slave.addr);
    at91_init_twi_bus(dev);
    at91_twi_write(dev, AT91_TWI_IER, AT91_TWI_SVACC);
    dev_info(dev.dev, "entered slave mode (ADR=%d)\n", slave.addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_unreg_slave(slave: *mut i2c_client) -> c_int {
    static int at91_unreg_slave(struct i2c_client *slave)
    {
    struct at91_twi_dev *dev = i2c_get_adapdata(slave.adapter);
    WARN_ON(!dev.slave);
    dev_info(dev.dev, "leaving slave mode\n");
    dev.slave = core::ptr::null_mut();
    dev.smr = 0;
    at91_init_twi_bus(dev);
    pm_runtime_put(dev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_twi_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 at91_twi_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_SLAVE;
    }
    static const struct i2c_algorithm at91_twi_algorithm_slave = {
    .reg_slave	= at91_reg_slave,
    .unreg_slave	= at91_unreg_slave,
    .functionality	= at91_twi_func,
    };
    int at91_twi_probe_slave(struct platform_device *pdev,
    u32 phy_addr, struct at91_twi_dev *dev)
    {
    int rc;
    rc = devm_request_irq(&pdev.dev, dev.irq, atmel_twi_interrupt_slave,
    0, dev_name(dev.dev), dev);
    if (rc) {
    dev_err(dev.dev, "Cannot get irq %d: %d\n", dev.irq, rc);
    return rc;
    }
    dev.adapter.algo = &at91_twi_algorithm_slave;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn at91_init_twi_bus_slave(dev: *mut at91_twi_dev) {
    void at91_init_twi_bus_slave(struct at91_twi_dev *dev)
    {
    at91_twi_write(dev, AT91_TWI_CR, AT91_TWI_MSDIS);
    if (dev.slave_detected && dev.smr) {
    at91_twi_write(dev, AT91_TWI_SMR, dev.smr);
    at91_twi_write(dev, AT91_TWI_CR, AT91_TWI_SVEN);
    }
    }
