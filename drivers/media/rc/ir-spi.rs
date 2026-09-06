//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/ir-spi.c
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
// SPI driven IR LED device driver
//
// Copyright (c) 2016 Samsung Electronics Co., Ltd.
// Copyright (c) Andi Shyti <andi@etezian.org>

pub const IR_SPI_DEFAULT_FREQUENCY: c_int = 38000;
pub const IR_SPI_BITS_PER_PULSE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_spi_data {
    pub freq: u32,
    pub negated: bool,
    pub pulse: u16,
    pub space: u16,
    pub rc: *mut rc_dev,
    pub spi: *mut spi_device,
    pub regulator: *mut regulator,
}

#[no_mangle]
unsafe extern "C" fn ir_spi_tx(dev: *mut rc_dev, buffer: *mut c_uint, count: c_uint) -> c_int {
    static int ir_spi_tx(struct rc_dev *dev, unsigned int *buffer, unsigned int count)
    {
    int i;
    int ret;
    let mut len: c_uint = 0;
    struct ir_spi_data *idata = dev.priv;
    struct spi_transfer xfer;
    u16 *tx_buf;
// convert the pulse/space signal to raw binary signal
    for (i = 0; i < count; i++) {
    buffer[i] = DIV_ROUND_CLOSEST_ULL((u64)buffer[i] * idata.freq,
    1000000);
    len += buffer[i];
    }
    tx_buf = kmalloc_array(len, sizeof(*tx_buf), GFP_KERNEL);
    if (!tx_buf)
    return -ENOMEM;
    len = 0;
    for (i = 0; i < count; i++) {
    int j;
    u16 val;
//
// The first value in buffer is a pulse, so that 0, 2, 4, ...
// contain a pulse duration. On the contrary, 1, 3, 5, ...
// contain a space duration.
//
    val = (i % 2) ? idata.space : idata.pulse;
    for (j = 0; j < buffer[i]; j++)
    tx_buf[len++] = val;
    }
    memset(&xfer, 0, sizeof(xfer));
    xfer.speed_hz = idata.freq * IR_SPI_BITS_PER_PULSE;
    xfer.len = len * sizeof(*tx_buf);
    xfer.tx_buf = tx_buf;
    ret = regulator_enable(idata.regulator);
    if (ret)
    goto err_free_tx_buf;
    ret = spi_sync_transfer(idata.spi, &xfer, 1);
    if (ret)
    dev_err(&idata.spi.dev, "unable to deliver the signal\n");
    regulator_disable(idata.regulator);
    err_free_tx_buf:
    kfree(tx_buf);
    return ret ? ret : count;
    }
#[no_mangle]
unsafe extern "C" fn ir_spi_set_tx_carrier(dev: *mut rc_dev, carrier: u32) -> c_int {
    static int ir_spi_set_tx_carrier(struct rc_dev *dev, u32 carrier)
    {
    struct ir_spi_data *idata = dev.priv;
    if (!carrier)
    return -EINVAL;
    if (carrier > idata.spi.max_speed_hz / IR_SPI_BITS_PER_PULSE)
    return -EINVAL;
    idata.freq = carrier;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_spi_set_duty_cycle(dev: *mut rc_dev, duty_cycle: u32) -> c_int {
    static int ir_spi_set_duty_cycle(struct rc_dev *dev, u32 duty_cycle)
    {
    struct ir_spi_data *idata = dev.priv;
    let mut bits: c_int = (duty_cycle * 15) / 100;
    idata.pulse = GENMASK(bits, 0);
    if (idata.negated) {
    idata.pulse = ~idata.pulse;
    idata.space = 0xffff;
    } else {
    idata.space = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_spi_probe(spi: *mut spi_device) -> c_int {
    static int ir_spi_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    int ret;
    u8 dc;
    struct ir_spi_data *idata;
    idata = devm_kzalloc(dev, sizeof(*idata), GFP_KERNEL);
    if (!idata)
    return -ENOMEM;
    idata.regulator = devm_regulator_get(dev, "irda_regulator");
    if (IS_ERR(idata.regulator))
    return PTR_ERR(idata.regulator);
    idata.rc = devm_rc_allocate_device(&spi.dev, RC_DRIVER_IR_RAW_TX);
    if (!idata.rc)
    return -ENOMEM;
    idata.rc.tx_ir           = ir_spi_tx;
    idata.rc.s_tx_carrier    = ir_spi_set_tx_carrier;
    idata.rc.s_tx_duty_cycle = ir_spi_set_duty_cycle;
    idata.rc.device_name	   = "IR SPI";
    idata.rc.driver_name     = IR_SPI_DRIVER_NAME;
    idata.rc.priv            = idata;
    idata.spi                 = spi;
    idata.negated = device_property_read_bool(dev, "led-active-low");
    ret = device_property_read_u8(dev, "duty-cycle", &dc);
    if (ret)
    dc = 50;
//
// ir_spi_set_duty_cycle() cannot fail, it returns int
// to be compatible with the rc->s_tx_duty_cycle function.
//
    ir_spi_set_duty_cycle(idata.rc, dc);
    idata.freq = IR_SPI_DEFAULT_FREQUENCY;
    return devm_rc_register_device(dev, idata.rc);
    }
    static const struct of_device_id ir_spi_of_match[] = {
    { .compatible = "ir-spi-led" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ir_spi_of_match);
    static const struct spi_device_id ir_spi_ids[] = {
    { "ir-spi-led" },
    {}
    };
    MODULE_DEVICE_TABLE(spi, ir_spi_ids);
    static struct spi_driver ir_spi_driver = {
    .probe = ir_spi_probe,
    .id_table = ir_spi_ids,
    .driver = {
    .name = IR_SPI_DRIVER_NAME,
    .of_match_table = ir_spi_of_match,
    },
    };
    module_spi_driver(ir_spi_driver);
    MODULE_AUTHOR("Andi Shyti <andi@etezian.org>");
    MODULE_DESCRIPTION("SPI IR LED");
    MODULE_LICENSE("GPL v2");
