//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ads7871.c
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
// ads7871 - driver for TI ADS7871 A/D converter
//
// Copyright (c) 2010 Paul Thomas <pthomas8589@gmail.com>
//
// You need to have something like this in struct spi_board_info
// {
// .modalias	= "ads7871",
// .max_speed_hz	= 2*1000*1000,
// .chip_select	= 0,
// .bus_num	= 1,
// },
//
// From figure 18 in the datasheet
// Register addresses

//
// From figure 17 in the datasheet
// These bits get ORed with the address to form
// the instruction byte
//
// Instruction Bit masks

// From figure 18 in the datasheet
// bit masks for Rev/Oscillator Control Register
pub const MUX_CNV_BV: c_int = 7;

// From figure 18 in the datasheet
// bit masks for Rev/Oscillator Control Register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ads7871_data {
    pub spi: *mut spi_device,
    pub ____cacheline_aligned: u8 tx_buf[2],
}

    static umode_t ads7871_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type == hwmon_in && attr == hwmon_in_input)
    return 0444;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ads7871_read_reg8(spi: *mut spi_device, reg: c_int) -> c_int {
    static int ads7871_read_reg8(struct spi_device *spi, int reg)
    {
    int ret;
    reg = reg | INST_READ_BM;
    ret = spi_w8r8(spi, reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ads7871_read_reg16(spi: *mut spi_device, reg: c_int) -> c_int {
    static int ads7871_read_reg16(struct spi_device *spi, int reg)
    {
    int ret;
    reg = reg | INST_READ_BM | INST_16BIT_BM;
    ret = spi_w8r16(spi, reg);
    if (ret < 0)
    return ret;
    return le16_to_cpu(( __le16)ret);
    }
#[no_mangle]
unsafe extern "C" fn ads7871_write_reg8(pdata: *mut ads7871_data, reg: c_int, val: u8) -> c_int {
    static int ads7871_write_reg8(struct ads7871_data *pdata, int reg, u8 val)
    {
    pdata.tx_buf[0] = reg;
    pdata.tx_buf[1] = val;
    return spi_write(pdata.spi, pdata.tx_buf, 2);
    }
    static int ads7871_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct ads7871_data *pdata = dev_get_drvdata(dev);
    struct spi_device *spi = pdata.spi;
    int ret, raw_val, i = 0;
    u8 mux_cnv;
    if (type != hwmon_in || attr != hwmon_in_input)
    return -EOPNOTSUPP;
//
// TODO: add support for conversions
// other than single ended with a gain of 1
//
// MUX_M3_BM forces single ended
// This is also where the gain of the PGA would be set
    ret = ads7871_write_reg8(pdata, REG_GAIN_MUX,
    (MUX_CNV_BM | MUX_M3_BM | channel));
    if (ret < 0)
    return ret;
    ret = ads7871_read_reg8(spi, REG_GAIN_MUX);
    if (ret < 0)
    return ret;
    mux_cnv = ((ret & MUX_CNV_BM) >> MUX_CNV_BV);
//
// on 400MHz arm9 platform the conversion
// is already done when we do this test
//
    while ((i < 2) && mux_cnv) {
    i++;
    ret = ads7871_read_reg8(spi, REG_GAIN_MUX);
    if (ret < 0)
    return ret;
    mux_cnv = ((ret & MUX_CNV_BM) >> MUX_CNV_BV);
    msleep_interruptible(1);
    }
    if (mux_cnv == 0) {
    raw_val = ads7871_read_reg16(spi, REG_LS_BYTE);
    if (raw_val < 0)
    return raw_val;
//
// Use (s16) to ensure the sign bit is preserved during the shift.
// Report millivolts (2.5V = 2500mV).
//
// val = ((s16)raw_val >> 2) * 2500 / 8192;
    return 0;
    }
    return -ETIMEDOUT;
    }
    static const struct hwmon_channel_info * const ads7871_info[] = {
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT, HWMON_I_INPUT, HWMON_I_INPUT, HWMON_I_INPUT,
    HWMON_I_INPUT, HWMON_I_INPUT, HWMON_I_INPUT, HWMON_I_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops ads7871_hwmon_ops = {
    .is_visible = ads7871_is_visible,
    .read = ads7871_read,
    };
    static const struct hwmon_chip_info ads7871_chip_info = {
    .ops = &ads7871_hwmon_ops,
    .info = ads7871_info,
    };
#[no_mangle]
unsafe extern "C" fn ads7871_probe(spi: *mut spi_device) -> c_int {
    static int ads7871_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    int ret;
    uint8_t val;
    struct ads7871_data *pdata;
    struct device *hwmon_dev;
// Configure the SPI bus
    spi.mode = (SPI_MODE_0);
    spi.bits_per_word = 8;
    spi_setup(spi);
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    pdata.spi = spi;
    ads7871_write_reg8(pdata, REG_SER_CONTROL, 0);
    ads7871_write_reg8(pdata, REG_AD_CONTROL, 0);
    val = (OSC_OSCR_BM | OSC_OSCE_BM | OSC_REFE_BM | OSC_BUFE_BM);
    ads7871_write_reg8(pdata, REG_OSC_CONTROL, val);
    ret = ads7871_read_reg8(spi, REG_OSC_CONTROL);
    dev_dbg(dev, "REG_OSC_CONTROL write:%x, read:%x\n", val, ret);
//
// because there is no other error checking on an SPI bus
// we need to make sure we really have a chip
//
    if (val != ret)
    return -ENODEV;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, spi.modalias,
    pdata,
    &ads7871_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct spi_driver ads7871_driver = {
    .driver = {
    .name = DEVICE_NAME,
    },
    .probe = ads7871_probe,
    };
    module_spi_driver(ads7871_driver);
    MODULE_AUTHOR("Paul Thomas <pthomas8589@gmail.com>");
    MODULE_DESCRIPTION("TI ADS7871 A/D driver");
    MODULE_LICENSE("GPL");
