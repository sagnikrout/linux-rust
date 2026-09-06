//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/ak8974.c
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
// Driver for the Asahi Kasei EMD Corporation AK8974
// and Aichi Steel AMI305 magnetometer chips.
// Based on a patch from Samu Onkalo and the AK8975 IIO driver.
//
// Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
// Copyright (c) 2010 NVIDIA Corporation.
// Copyright (C) 2016 Linaro Ltd.
//
// Author: Samu Onkalo <samu.p.onkalo@nokia.com>
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// 16-bit registers are little-endian. LSB is at the address defined below
// and MSB is at the next higher address.
//
// These registers are common for AK8974 and AMI30x
pub const AK8974_SELFTEST: c_uint = 0x0C;
pub const AK8974_SELFTEST_IDLE: c_uint = 0x55;
pub const AK8974_SELFTEST_OK: c_uint = 0xAA;
pub const AK8974_INFO: c_uint = 0x0D;
pub const AK8974_WHOAMI: c_uint = 0x0F;
pub const AK8974_WHOAMI_VALUE_AMI306: c_uint = 0x46;
pub const AK8974_WHOAMI_VALUE_AMI305: c_uint = 0x47;
pub const AK8974_WHOAMI_VALUE_AK8974: c_uint = 0x48;
pub const AK8974_WHOAMI_VALUE_HSCDTD008A: c_uint = 0x49;
pub const AK8974_DATA_X: c_uint = 0x10;
pub const AK8974_DATA_Y: c_uint = 0x12;
pub const AK8974_DATA_Z: c_uint = 0x14;
pub const AK8974_INT_SRC: c_uint = 0x16;
pub const AK8974_STATUS: c_uint = 0x18;
pub const AK8974_INT_CLEAR: c_uint = 0x1A;
pub const AK8974_CTRL1: c_uint = 0x1B;
pub const AK8974_CTRL2: c_uint = 0x1C;
pub const AK8974_CTRL3: c_uint = 0x1D;
pub const AK8974_INT_CTRL: c_uint = 0x1E;
pub const AK8974_INT_THRES: c_uint = 0x26  /* Absolute any axis value threshold */;
pub const AK8974_PRESET: c_uint = 0x30;
// AK8974-specific offsets
pub const AK8974_OFFSET_X: c_uint = 0x20;
pub const AK8974_OFFSET_Y: c_uint = 0x22;
pub const AK8974_OFFSET_Z: c_uint = 0x24;
// AMI305-specific offsets
pub const AMI305_OFFSET_X: c_uint = 0x6C;
pub const AMI305_OFFSET_Y: c_uint = 0x72;
pub const AMI305_OFFSET_Z: c_uint = 0x78;
// Different temperature registers
pub const AK8974_TEMP: c_uint = 0x31;
pub const AMI305_TEMP: c_uint = 0x60;
// AMI306-specific control register
pub const AMI306_CTRL4: c_uint = 0x5C;
// AMI306 factory calibration data
// fine axis sensitivity
pub const AMI306_FINEOUTPUT_X: c_uint = 0x90;
pub const AMI306_FINEOUTPUT_Y: c_uint = 0x92;
pub const AMI306_FINEOUTPUT_Z: c_uint = 0x94;
// axis sensitivity
pub const AMI306_SENS_X: c_uint = 0x96;
pub const AMI306_SENS_Y: c_uint = 0x98;
pub const AMI306_SENS_Z: c_uint = 0x9A;
// axis cross-interference
pub const AMI306_GAIN_PARA_XZ: c_uint = 0x9C;
pub const AMI306_GAIN_PARA_XY: c_uint = 0x9D;
pub const AMI306_GAIN_PARA_YZ: c_uint = 0x9E;
pub const AMI306_GAIN_PARA_YX: c_uint = 0x9F;
pub const AMI306_GAIN_PARA_ZY: c_uint = 0xA0;
pub const AMI306_GAIN_PARA_ZX: c_uint = 0xA1;
// offset at ZERO magnetic field
pub const AMI306_OFFZERO_X: c_uint = 0xF8;
pub const AMI306_OFFZERO_Y: c_uint = 0xFA;
pub const AMI306_OFFZERO_Z: c_uint = 0xFC;

pub const AK8974_CTRL3_RESDEF: c_uint = 0x00;

// HSCDTD008A-specific control register
pub const HSCDTD008A_CTRL4: c_uint = 0x1E;

// The AMI305 has elaborate FW version and serial number registers
pub const AMI305_VER: c_uint = 0xE8;
pub const AMI305_SN: c_uint = 0xEA;
pub const AK8974_MAX_RANGE: c_int = 2048;
pub const AK8974_POWERON_DELAY: c_int = 50;
pub const AK8974_ACTIVATE_DELAY: c_int = 1;
pub const AK8974_SELFTEST_DELAY: c_int = 1;
//
// Set the autosuspend to two orders of magnitude larger than the poweron
// delay to make sane reasonable power tradeoff savings (5 seconds in
// this case).
//
pub const AK8974_AUTOSUSPEND_DELAY: c_int = 5000;
pub const AK8974_MEASTIME: c_int = 3;
pub const AK8974_PWR_ON: c_int = 1;
pub const AK8974_PWR_OFF: c_int = 0;
//
// struct ak8974 - state container for the AK8974 driver
// @i2c: parent I2C client
// @orientation: mounting matrix, flipped axis etc
// @map: regmap to access the AK8974 registers over I2C
// @regs: the avdd and dvdd power regulators
// @name: the name of the part
// @variant: the whoami ID value (for selecting code paths)
// @lock: locks the magnetometer for exclusive use during a measurement
// @drdy_irq: uses the DRDY IRQ line
// @drdy_complete: completion for DRDY
// @drdy_active_low: the DRDY IRQ is active low
// @scan: timestamps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak8974 {
    pub i2c: *mut i2c_client,
    pub orientation: iio_mount_matrix,
    pub map: *mut regmap,
    pub regs: [regulator_bulk_data; 2],
    pub name: *const c_char,
    pub variant: u8,
    pub lock: mutex,
    pub drdy_irq: bool,
    pub drdy_complete: completion,
    pub drdy_active_low: bool,
// Ensure timestamp is naturally aligned
    struct {
    pub channels: [__le16; 3],
    pub ts: aligned_s64,
    pub scan: },
}

    static const char ak8974_reg_avdd[] = "avdd";
    static const char ak8974_reg_dvdd[] = "dvdd";
#[no_mangle]
unsafe extern "C" fn ak8974_get_u16_val(ak8974: *mut ak8974, reg: u8, val: *mut u16) -> c_int {
    static int ak8974_get_u16_val(struct ak8974 *ak8974, u8 reg, u16 *val)
    {
    int ret;
    __le16 bulk;
    ret = regmap_bulk_read(ak8974.map, reg, &bulk, 2);
    if (ret)
    return ret;
// val = le16_to_cpu(bulk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_set_u16_val(ak8974: *mut ak8974, reg: u8, val: u16) -> c_int {
    static int ak8974_set_u16_val(struct ak8974 *ak8974, u8 reg, u16 val)
    {
    let mut bulk: __le16 = cpu_to_le16(val);
    return regmap_bulk_write(ak8974.map, reg, &bulk, 2);
    }
#[no_mangle]
unsafe extern "C" fn ak8974_set_power(ak8974: *mut ak8974, mode: bool) -> c_int {
    static int ak8974_set_power(struct ak8974 *ak8974, bool mode)
    {
    int ret;
    u8 val;
    val = mode ? AK8974_CTRL1_POWER : 0;
    val |= AK8974_CTRL1_FORCE_EN;
    ret = regmap_write(ak8974.map, AK8974_CTRL1, val);
    if (ret < 0)
    return ret;
    if (mode)
    msleep(AK8974_ACTIVATE_DELAY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_reset(ak8974: *mut ak8974) -> c_int {
    static int ak8974_reset(struct ak8974 *ak8974)
    {
    int ret;
// Power on to get register access. Sets CTRL1 reg to reset state
    ret = ak8974_set_power(ak8974, AK8974_PWR_ON);
    if (ret)
    return ret;
    ret = regmap_write(ak8974.map, AK8974_CTRL2, AK8974_CTRL2_RESDEF);
    if (ret)
    return ret;
    ret = regmap_write(ak8974.map, AK8974_CTRL3, AK8974_CTRL3_RESDEF);
    if (ret)
    return ret;
    if (ak8974.variant != AK8974_WHOAMI_VALUE_HSCDTD008A) {
    ret = regmap_write(ak8974.map, AK8974_INT_CTRL,
    AK8974_INT_CTRL_RESDEF);
    if (ret)
    return ret;
    } else {
    ret = regmap_write(ak8974.map, HSCDTD008A_CTRL4,
    HSCDTD008A_CTRL4_RESDEF);
    if (ret)
    return ret;
    }
// After reset, power off is default state
    return ak8974_set_power(ak8974, AK8974_PWR_OFF);
    }
#[no_mangle]
unsafe extern "C" fn ak8974_configure(ak8974: *mut ak8974) -> c_int {
    static int ak8974_configure(struct ak8974 *ak8974)
    {
    int ret;
    ret = regmap_write(ak8974.map, AK8974_CTRL2, AK8974_CTRL2_DRDY_EN |
    AK8974_CTRL2_INT_EN);
    if (ret)
    return ret;
    ret = regmap_write(ak8974.map, AK8974_CTRL3, 0);
    if (ret)
    return ret;
    if (ak8974.variant == AK8974_WHOAMI_VALUE_AMI306) {
// magic from datasheet: set high-speed measurement mode
    ret = ak8974_set_u16_val(ak8974, AMI306_CTRL4, 0xA07E);
    if (ret)
    return ret;
    }
    if (ak8974.variant == AK8974_WHOAMI_VALUE_HSCDTD008A)
    return 0;
    ret = regmap_write(ak8974.map, AK8974_INT_CTRL, AK8974_INT_CTRL_POL);
    if (ret)
    return ret;
    return regmap_write(ak8974.map, AK8974_PRESET, 0);
    }
#[no_mangle]
unsafe extern "C" fn ak8974_trigmeas(ak8974: *mut ak8974) -> c_int {
    static int ak8974_trigmeas(struct ak8974 *ak8974)
    {
    unsigned int clear;
    u8 mask;
    u8 val;
    int ret;
// Clear any previous measurement overflow status
    ret = regmap_read(ak8974.map, AK8974_INT_CLEAR, &clear);
    if (ret)
    return ret;
// If we have a DRDY IRQ line, use it
    if (ak8974.drdy_irq) {
    mask = AK8974_CTRL2_INT_EN |
    AK8974_CTRL2_DRDY_EN |
    AK8974_CTRL2_DRDY_POL;
    val = AK8974_CTRL2_DRDY_EN;
    if (!ak8974.drdy_active_low)
    val |= AK8974_CTRL2_DRDY_POL;
    init_completion(&ak8974.drdy_complete);
    ret = regmap_update_bits(ak8974.map, AK8974_CTRL2,
    mask, val);
    if (ret)
    return ret;
    }
// Force a measurement
    return regmap_set_bits(ak8974.map, AK8974_CTRL3, AK8974_CTRL3_FORCE);
    }
#[no_mangle]
unsafe extern "C" fn ak8974_await_drdy(ak8974: *mut ak8974) -> c_int {
    static int ak8974_await_drdy(struct ak8974 *ak8974)
    {
    let mut timeout: c_int = 2;
    unsigned int val;
    int ret;
    if (ak8974.drdy_irq) {
    ret = wait_for_completion_timeout(&ak8974.drdy_complete,
    1 + msecs_to_jiffies(1000));
    if (!ret) {
    dev_err(&ak8974.i2c.dev,
    "timeout waiting for DRDY IRQ\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
// Default delay-based poll loop
    do {
    msleep(AK8974_MEASTIME);
    ret = regmap_read(ak8974.map, AK8974_STATUS, &val);
    if (ret < 0)
    return ret;
    if (val & AK8974_STATUS_DRDY)
    return 0;
    } while (--timeout);
    dev_err(&ak8974.i2c.dev, "timeout waiting for DRDY\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_getresult(ak8974: *mut ak8974, result: *mut __le16) -> c_int {
    static int ak8974_getresult(struct ak8974 *ak8974, __le16 *result)
    {
    unsigned int src;
    int ret;
    ret = ak8974_await_drdy(ak8974);
    if (ret)
    return ret;
    ret = regmap_read(ak8974.map, AK8974_INT_SRC, &src);
    if (ret < 0)
    return ret;
// Out of range overflow! Strong magnet close?
    if (src & AK8974_INT_RANGE) {
    dev_err(&ak8974.i2c.dev,
    "range overflow in sensor\n");
    return -ERANGE;
    }
    return regmap_bulk_read(ak8974.map, AK8974_DATA_X, result, 6);
    }
#[no_mangle]
unsafe extern "C" fn ak8974_drdy_irq(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t ak8974_drdy_irq(int irq, void *d)
    {
    struct ak8974 *ak8974 = d;
    if (!ak8974.drdy_irq)
    return IRQ_NONE;
// TODO: timestamp here to get good measurement stamps
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_drdy_irq_thread(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t ak8974_drdy_irq_thread(int irq, void *d)
    {
    struct ak8974 *ak8974 = d;
    unsigned int val;
    int ret;
// Check if this was a DRDY from us
    ret = regmap_read(ak8974.map, AK8974_STATUS, &val);
    if (ret < 0) {
    dev_err(&ak8974.i2c.dev, "error reading DRDY status\n");
    return IRQ_HANDLED;
    }
    if (val & AK8974_STATUS_DRDY) {
// Yes this was our IRQ
    complete(&ak8974.drdy_complete);
    return IRQ_HANDLED;
    }
// We may be on a shared IRQ, let the next client check
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_selftest(ak8974: *mut ak8974) -> c_int {
    static int ak8974_selftest(struct ak8974 *ak8974)
    {
    struct device *dev = &ak8974.i2c.dev;
    unsigned int val;
    int ret;
    ret = regmap_read(ak8974.map, AK8974_SELFTEST, &val);
    if (ret)
    return ret;
    if (val != AK8974_SELFTEST_IDLE) {
    dev_err(dev, "selftest not idle before test\n");
    return -EIO;
    }
// Trigger self-test
    ret = regmap_set_bits(ak8974.map, AK8974_CTRL3, AK8974_CTRL3_SELFTEST);
    if (ret) {
    dev_err(dev, "could not write CTRL3\n");
    return ret;
    }
    msleep(AK8974_SELFTEST_DELAY);
    ret = regmap_read(ak8974.map, AK8974_SELFTEST, &val);
    if (ret)
    return ret;
    if (val != AK8974_SELFTEST_OK) {
    dev_err(dev, "selftest result NOT OK (%02x)\n", val);
    return -EIO;
    }
    ret = regmap_read(ak8974.map, AK8974_SELFTEST, &val);
    if (ret)
    return ret;
    if (val != AK8974_SELFTEST_IDLE) {
    dev_err(dev, "selftest not idle after test (%02x)\n", val);
    return -EIO;
    }
    dev_dbg(dev, "passed self-test\n");
    return 0;
    }
    static void ak8974_read_calib_data(struct ak8974 *ak8974, unsigned int reg,
    __le16 *tab, size_t tab_size)
    {
    let mut ret: c_int = regmap_bulk_read(ak8974.map, reg, tab, tab_size);
    if (ret) {
    memset(tab, 0xFF, tab_size);
    dev_warn(&ak8974.i2c.dev,
    "can't read calibration data (regs %u..%zu): %d\n",
    reg, reg + tab_size - 1, ret);
    } else {
    add_device_randomness(tab, tab_size);
    }
    }
#[no_mangle]
unsafe extern "C" fn ak8974_detect(ak8974: *mut ak8974) -> c_int {
    static int ak8974_detect(struct ak8974 *ak8974)
    {
    unsigned int whoami;
    const char *name;
    int ret;
    unsigned int fw;
    u16 sn;
    ret = regmap_read(ak8974.map, AK8974_WHOAMI, &whoami);
    if (ret)
    return ret;
    name = "ami305";
    switch (whoami) {
    case AK8974_WHOAMI_VALUE_AMI306:
    name = "ami306";
    fallthrough;
    case AK8974_WHOAMI_VALUE_AMI305:
    ret = regmap_read(ak8974.map, AMI305_VER, &fw);
    if (ret)
    return ret;
    fw &= 0x7f; /* only bits 0 thru 6 valid */
    ret = ak8974_get_u16_val(ak8974, AMI305_SN, &sn);
    if (ret)
    return ret;
    add_device_randomness(&sn, sizeof(sn));
    dev_info(&ak8974.i2c.dev,
    "detected %s, FW ver %02x, S/N: %04x\n",
    name, fw, sn);
    break;
    case AK8974_WHOAMI_VALUE_AK8974:
    name = "ak8974";
    dev_info(&ak8974.i2c.dev, "detected AK8974\n");
    break;
    case AK8974_WHOAMI_VALUE_HSCDTD008A:
    name = "hscdtd008a";
    dev_info(&ak8974.i2c.dev, "detected hscdtd008a\n");
    break;
    default:
    dev_err(&ak8974.i2c.dev, "unsupported device (%02x) ",
    whoami);
    return -ENODEV;
    }
    ak8974.name = name;
    ak8974.variant = whoami;
    if (whoami == AK8974_WHOAMI_VALUE_AMI306) {
    __le16 fab_data1[9], fab_data2[3];
    int i;
    ak8974_read_calib_data(ak8974, AMI306_FINEOUTPUT_X,
    fab_data1, sizeof(fab_data1));
    ak8974_read_calib_data(ak8974, AMI306_OFFZERO_X,
    fab_data2, sizeof(fab_data2));
    for (i = 0; i < 3; ++i) {
    static const char axis[] = "XYZ";
    static const char pgaxis[] = "ZYZXYX";
    let mut offz: unsigned = le16_to_cpu(fab_data2[i]) & 0x7F;
    let mut fine: unsigned = le16_to_cpu(fab_data1[i]);
    let mut sens: unsigned = le16_to_cpu(fab_data1[i + 3]);
    let mut pgain1: unsigned = le16_to_cpu(fab_data1[i + 6]);
    let mut pgain2: unsigned = pgain1 >> 8;
    pgain1 &= 0xFF;
    dev_info(&ak8974.i2c.dev,
    "factory calibration for axis %c: offz=%u sens=%u fine=%u pga%c=%u pga%c=%u\n",
    axis[i], offz, sens, fine, pgaxis[i * 2],
    pgain1, pgaxis[i * 2 + 1], pgain2);
    }
    }
    return 0;
    }
    static int ak8974_measure_channel(struct ak8974 *ak8974, unsigned long address,
    int *val)
    {
    __le16 hw_values[3];
    int ret;
    pm_runtime_get_sync(&ak8974.i2c.dev);
    mutex_lock(&ak8974.lock);
//
// We read all axes and discard all but one, for optimized
// reading, use the triggered buffer.
//
    ret = ak8974_trigmeas(ak8974);
    if (ret)
    goto out_unlock;
    ret = ak8974_getresult(ak8974, hw_values);
    if (ret)
    goto out_unlock;
//
// This explicit cast to (s16) is necessary as the measurement
// is done in 2's complement with positive and negative values.
// The following assignment to *val will then convert the signed
// s16 value to a signed int value.
//
// val = (s16)le16_to_cpu(hw_values[address]);
    out_unlock:
    mutex_unlock(&ak8974.lock);
    pm_runtime_put_autosuspend(&ak8974.i2c.dev);
    return ret;
    }
    static int ak8974_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2,
    long mask)
    {
    struct ak8974 *ak8974 = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (chan.address > 2) {
    dev_err(&ak8974.i2c.dev, "faulty channel address\n");
    return -EIO;
    }
    ret = ak8974_measure_channel(ak8974, chan.address, val);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (ak8974.variant) {
    case AK8974_WHOAMI_VALUE_AMI306:
    case AK8974_WHOAMI_VALUE_AMI305:
//
// The datasheet for AMI305 and AMI306, page 6
// specifies the range of the sensor to be
// +/- 12 Gauss.
//
// val = 12;
//
// 12 bits are used, +/- 2^11
// [ -2048 .. 2047 ] (manual page 20)
// [ 0xf800 .. 0x07ff ]
//
// val2 = 11;
    return IIO_VAL_FRACTIONAL_LOG2;
    case AK8974_WHOAMI_VALUE_HSCDTD008A:
//
// The datasheet for HSCDTF008A, page 3 specifies the
// range of the sensor as +/- 2.4 mT per axis, which
// corresponds to +/- 2400 uT = +/- 24 Gauss.
//
// val = 24;
//
// 15 bits are used (set up in CTRL4), +/- 2^14
// [ -16384 .. 16383 ] (manual page 24)
// [ 0xc000 .. 0x3fff ]
//
// val2 = 14;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
// GUESSING +/- 12 Gauss
// val = 12;
// GUESSING 12 bits ADC +/- 2^11
// val2 = 11;
    return IIO_VAL_FRACTIONAL_LOG2;
    }
    break;
    default:
// Unknown request
    break;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_fill_buffer(indio_dev: *mut iio_dev) {
    static void ak8974_fill_buffer(struct iio_dev *indio_dev)
    {
    struct ak8974 *ak8974 = iio_priv(indio_dev);
    int ret;
    pm_runtime_get_sync(&ak8974.i2c.dev);
    mutex_lock(&ak8974.lock);
    ret = ak8974_trigmeas(ak8974);
    if (ret) {
    dev_err(&ak8974.i2c.dev, "error triggering measure\n");
    goto out_unlock;
    }
    ret = ak8974_getresult(ak8974, ak8974.scan.channels);
    if (ret) {
    dev_err(&ak8974.i2c.dev, "error getting measures\n");
    goto out_unlock;
    }
    iio_push_to_buffers_with_ts(indio_dev, &ak8974.scan, sizeof(ak8974.scan),
    iio_get_time_ns(indio_dev));
    out_unlock:
    mutex_unlock(&ak8974.lock);
    pm_runtime_put_autosuspend(&ak8974.i2c.dev);
    }
#[no_mangle]
unsafe extern "C" fn ak8974_handle_trigger(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ak8974_handle_trigger(int irq, void *p)
    {
    const struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    ak8974_fill_buffer(indio_dev);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static const struct iio_mount_matrix *
    ak8974_get_mount_matrix(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct ak8974 *ak8974 = iio_priv(indio_dev);
    return &ak8974.orientation;
    }
    static const struct iio_chan_spec_ext_info ak8974_ext_info[] = {
    IIO_MOUNT_MATRIX(IIO_SHARED_BY_DIR, ak8974_get_mount_matrix),
    { }
    };

    {								\
    .type = IIO_MAGN,					\
    .modified = 1,						\
    .channel2 = IIO_MOD_##axis,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_SCALE),			\
    .ext_info = ak8974_ext_info,				\
    .address = index,					\
    .scan_index = index,					\
    .scan_type = {						\
    .sign = 's',					\
    .realbits = bits,				\
    .storagebits = 16,				\
    .endianness = IIO_LE				\
    },							\
    }
//
// We have no datasheet for the AK8974 but we guess that its
// ADC is 12 bits. The AMI305 and AMI306 certainly has 12bit
// ADC.
//
    static const struct iio_chan_spec ak8974_12_bits_channels[] = {
    AK8974_AXIS_CHANNEL(X, 0, 12),
    AK8974_AXIS_CHANNEL(Y, 1, 12),
    AK8974_AXIS_CHANNEL(Z, 2, 12),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
//
// The HSCDTD008A has 15 bits resolution the way we set it up
// in CTRL4.
//
    static const struct iio_chan_spec ak8974_15_bits_channels[] = {
    AK8974_AXIS_CHANNEL(X, 0, 15),
    AK8974_AXIS_CHANNEL(Y, 1, 15),
    AK8974_AXIS_CHANNEL(Z, 2, 15),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
    static const unsigned long ak8974_scan_masks[] = { 0x7, 0 };
    static const struct iio_info ak8974_info = {
    .read_raw = &ak8974_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ak8974_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ak8974_writeable_reg(struct device *dev, unsigned int reg)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    struct iio_dev *indio_dev = i2c_get_clientdata(i2c);
    struct ak8974 *ak8974 = iio_priv(indio_dev);
    switch (reg) {
    case AK8974_CTRL1:
    case AK8974_CTRL2:
    case AK8974_CTRL3:
    case AK8974_INT_CTRL:
    case AK8974_INT_THRES:
    case AK8974_INT_THRES + 1:
    return true;
    case AK8974_PRESET:
    case AK8974_PRESET + 1:
    return ak8974.variant != AK8974_WHOAMI_VALUE_HSCDTD008A;
    case AK8974_OFFSET_X:
    case AK8974_OFFSET_X + 1:
    case AK8974_OFFSET_Y:
    case AK8974_OFFSET_Y + 1:
    case AK8974_OFFSET_Z:
    case AK8974_OFFSET_Z + 1:
    return ak8974.variant == AK8974_WHOAMI_VALUE_AK8974 ||
    ak8974.variant == AK8974_WHOAMI_VALUE_HSCDTD008A;
    case AMI305_OFFSET_X:
    case AMI305_OFFSET_X + 1:
    case AMI305_OFFSET_Y:
    case AMI305_OFFSET_Y + 1:
    case AMI305_OFFSET_Z:
    case AMI305_OFFSET_Z + 1:
    return ak8974.variant == AK8974_WHOAMI_VALUE_AMI305 ||
    ak8974.variant == AK8974_WHOAMI_VALUE_AMI306;
    case AMI306_CTRL4:
    case AMI306_CTRL4 + 1:
    return ak8974.variant == AK8974_WHOAMI_VALUE_AMI306;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn ak8974_precious_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ak8974_precious_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = AK8974_INT_CLEAR;
    }
    static const struct regmap_config ak8974_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xff,
    .writeable_reg = ak8974_writeable_reg,
    .precious_reg = ak8974_precious_reg,
    };
#[no_mangle]
unsafe extern "C" fn ak8974_probe(i2c: *mut i2c_client) -> c_int {
    static int ak8974_probe(struct i2c_client *i2c)
    {
    struct iio_dev *indio_dev;
    struct ak8974 *ak8974;
    unsigned long irq_trig;
    let mut irq: c_int = i2c.irq;
    int ret;
// Register with IIO
    indio_dev = devm_iio_device_alloc(&i2c.dev, sizeof(*ak8974));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    ak8974 = iio_priv(indio_dev);
    i2c_set_clientdata(i2c, indio_dev);
    ak8974.i2c = i2c;
    mutex_init(&ak8974.lock);
    ret = iio_read_mount_matrix(&i2c.dev, &ak8974.orientation);
    if (ret)
    return ret;
    ak8974.regs[0].supply = ak8974_reg_avdd;
    ak8974.regs[1].supply = ak8974_reg_dvdd;
    ret = devm_regulator_bulk_get(&i2c.dev,
    ARRAY_SIZE(ak8974.regs),
    ak8974.regs);
    if (ret < 0)
    return dev_err_probe(&i2c.dev, ret, "cannot get regulators\n");
    ret = regulator_bulk_enable(ARRAY_SIZE(ak8974.regs), ak8974.regs);
    if (ret < 0) {
    dev_err(&i2c.dev, "cannot enable regulators\n");
    return ret;
    }
// Take runtime PM online
    pm_runtime_get_noresume(&i2c.dev);
    pm_runtime_set_active(&i2c.dev);
    pm_runtime_enable(&i2c.dev);
    ak8974.map = devm_regmap_init_i2c(i2c, &ak8974_regmap_config);
    if (IS_ERR(ak8974.map)) {
    dev_err(&i2c.dev, "failed to allocate register map\n");
    pm_runtime_put_noidle(&i2c.dev);
    pm_runtime_disable(&i2c.dev);
    return PTR_ERR(ak8974.map);
    }
    ret = ak8974_set_power(ak8974, AK8974_PWR_ON);
    if (ret) {
    dev_err(&i2c.dev, "could not power on\n");
    goto disable_pm;
    }
    ret = ak8974_detect(ak8974);
    if (ret) {
    dev_err(&i2c.dev, "neither AK8974 nor AMI30x found\n");
    goto disable_pm;
    }
    ret = ak8974_selftest(ak8974);
    if (ret)
    dev_err(&i2c.dev, "selftest failed (continuing anyway)\n");
    ret = ak8974_reset(ak8974);
    if (ret) {
    dev_err(&i2c.dev, "AK8974 reset failed\n");
    goto disable_pm;
    }
    switch (ak8974.variant) {
    case AK8974_WHOAMI_VALUE_AMI306:
    case AK8974_WHOAMI_VALUE_AMI305:
    indio_dev.channels = ak8974_12_bits_channels;
    indio_dev.num_channels = ARRAY_SIZE(ak8974_12_bits_channels);
    break;
    case AK8974_WHOAMI_VALUE_HSCDTD008A:
    indio_dev.channels = ak8974_15_bits_channels;
    indio_dev.num_channels = ARRAY_SIZE(ak8974_15_bits_channels);
    break;
    default:
    indio_dev.channels = ak8974_12_bits_channels;
    indio_dev.num_channels = ARRAY_SIZE(ak8974_12_bits_channels);
    break;
    }
    indio_dev.info = &ak8974_info;
    indio_dev.available_scan_masks = ak8974_scan_masks;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.name = ak8974.name;
    ret = iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(),
    ak8974_handle_trigger,
    core::ptr::null_mut());
    if (ret) {
    dev_err(&i2c.dev, "triggered buffer setup failed\n");
    goto disable_pm;
    }
// If we have a valid DRDY IRQ, make use of it
    if (irq > 0) {
    irq_trig = irq_get_trigger_type(irq);
    if (irq_trig == IRQF_TRIGGER_RISING) {
    dev_info(&i2c.dev, "enable rising edge DRDY IRQ\n");
    } else if (irq_trig == IRQF_TRIGGER_FALLING) {
    ak8974.drdy_active_low = true;
    dev_info(&i2c.dev, "enable falling edge DRDY IRQ\n");
    } else {
    irq_trig = IRQF_TRIGGER_RISING;
    }
    irq_trig |= IRQF_ONESHOT;
    irq_trig |= IRQF_SHARED;
    ret = devm_request_threaded_irq(&i2c.dev,
    irq,
    ak8974_drdy_irq,
    ak8974_drdy_irq_thread,
    irq_trig,
    ak8974.name,
    ak8974);
    if (ret)
    goto no_irq;
    ak8974.drdy_irq = true;
    }
    no_irq:
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(&i2c.dev, "device register failed\n");
    goto cleanup_buffer;
    }
    pm_runtime_set_autosuspend_delay(&i2c.dev,
    AK8974_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(&i2c.dev);
    pm_runtime_put(&i2c.dev);
    return 0;
    cleanup_buffer:
    iio_triggered_buffer_cleanup(indio_dev);
    disable_pm:
    pm_runtime_put_noidle(&i2c.dev);
    pm_runtime_disable(&i2c.dev);
    ak8974_set_power(ak8974, AK8974_PWR_OFF);
    regulator_bulk_disable(ARRAY_SIZE(ak8974.regs), ak8974.regs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_remove(i2c: *mut i2c_client) {
    static void ak8974_remove(struct i2c_client *i2c)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(i2c);
    struct ak8974 *ak8974 = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    pm_runtime_get_sync(&i2c.dev);
    pm_runtime_put_noidle(&i2c.dev);
    pm_runtime_disable(&i2c.dev);
    ak8974_set_power(ak8974, AK8974_PWR_OFF);
    regulator_bulk_disable(ARRAY_SIZE(ak8974.regs), ak8974.regs);
    }
#[no_mangle]
unsafe extern "C" fn ak8974_runtime_suspend(dev: *mut device) -> c_int {
    static int ak8974_runtime_suspend(struct device *dev)
    {
    struct ak8974 *ak8974 =
    iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    ak8974_set_power(ak8974, AK8974_PWR_OFF);
    regulator_bulk_disable(ARRAY_SIZE(ak8974.regs), ak8974.regs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak8974_runtime_resume(dev: *mut device) -> c_int {
    static int ak8974_runtime_resume(struct device *dev)
    {
    struct ak8974 *ak8974 =
    iio_priv(i2c_get_clientdata(to_i2c_client(dev)));
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ak8974.regs), ak8974.regs);
    if (ret)
    return ret;
    msleep(AK8974_POWERON_DELAY);
    ret = ak8974_set_power(ak8974, AK8974_PWR_ON);
    if (ret)
    goto out_regulator_disable;
    ret = ak8974_configure(ak8974);
    if (ret)
    goto out_disable_power;
    return 0;
    out_disable_power:
    ak8974_set_power(ak8974, AK8974_PWR_OFF);
    out_regulator_disable:
    regulator_bulk_disable(ARRAY_SIZE(ak8974.regs), ak8974.regs);
    return ret;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(ak8974_dev_pm_ops, ak8974_runtime_suspend,
    ak8974_runtime_resume, core::ptr::null_mut());
    static const struct i2c_device_id ak8974_id[] = {
    { .name = "ami305" },
    { .name = "ami306" },
    { .name = "ak8974" },
    { .name = "hscdtd008a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ak8974_id);
    static const struct of_device_id ak8974_of_match[] = {
    { .compatible = "asahi-kasei,ak8974", },
    { .compatible = "alps,hscdtd008a", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ak8974_of_match);
    static struct i2c_driver ak8974_driver = {
    .driver	 = {
    .name	= "ak8974",
    .pm = pm_ptr(&ak8974_dev_pm_ops),
    .of_match_table = ak8974_of_match,
    },
    .probe = ak8974_probe,
    .remove	  = ak8974_remove,
    .id_table = ak8974_id,
    };
    module_i2c_driver(ak8974_driver);
    MODULE_DESCRIPTION("AK8974 and AMI30x 3-axis magnetometer driver");
    MODULE_AUTHOR("Samu Onkalo");
    MODULE_AUTHOR("Linus Walleij");
    MODULE_LICENSE("GPL v2");
