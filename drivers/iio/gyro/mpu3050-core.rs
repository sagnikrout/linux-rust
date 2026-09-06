//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/mpu3050-core.c
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
// MPU3050 gyroscope driver
//
// Copyright (C) 2016 Linaro Ltd.
// Author: Linus Walleij <linus.walleij@linaro.org>
//
// Based on the input subsystem driver, Copyright (C) 2011 Wistron Co.Ltd
// Joseph Lai <joseph_lai@wistron.com> and trimmed down by
// Alan Cox <alan@linux.intel.com> in turn based on bma023.c.
// Device behaviour based on a misc driver posted by Nathan Royer in 2011.
//
// TODO: add support for setting up the low pass 3dB frequency.
//

pub const MPU3050_CHIP_ID: c_uint = 0x68;
pub const MPU3050_CHIP_ID_MASK: c_uint = 0x7E;
//
// Register map: anything suffixed *_H is a big-endian high byte and always
// followed by the corresponding low byte (*_L) even though these are not
// explicitly included in the register definitions.
//
pub const MPU3050_CHIP_ID_REG: c_uint = 0x00;
pub const MPU3050_PRODUCT_ID_REG: c_uint = 0x01;
pub const MPU3050_XG_OFFS_TC: c_uint = 0x05;
pub const MPU3050_YG_OFFS_TC: c_uint = 0x08;
pub const MPU3050_ZG_OFFS_TC: c_uint = 0x0B;
pub const MPU3050_X_OFFS_USR_H: c_uint = 0x0C;
pub const MPU3050_Y_OFFS_USR_H: c_uint = 0x0E;
pub const MPU3050_Z_OFFS_USR_H: c_uint = 0x10;
pub const MPU3050_FIFO_EN: c_uint = 0x12;
pub const MPU3050_AUX_VDDIO: c_uint = 0x13;
pub const MPU3050_SLV_ADDR: c_uint = 0x14;
pub const MPU3050_SMPLRT_DIV: c_uint = 0x15;
pub const MPU3050_DLPF_FS_SYNC: c_uint = 0x16;
pub const MPU3050_INT_CFG: c_uint = 0x17;
pub const MPU3050_AUX_ADDR: c_uint = 0x18;
pub const MPU3050_INT_STATUS: c_uint = 0x1A;
pub const MPU3050_TEMP_H: c_uint = 0x1B;
pub const MPU3050_XOUT_H: c_uint = 0x1D;
pub const MPU3050_YOUT_H: c_uint = 0x1F;
pub const MPU3050_ZOUT_H: c_uint = 0x21;
pub const MPU3050_DMP_CFG1: c_uint = 0x35;
pub const MPU3050_DMP_CFG2: c_uint = 0x36;
pub const MPU3050_BANK_SEL: c_uint = 0x37;
pub const MPU3050_MEM_START_ADDR: c_uint = 0x38;
pub const MPU3050_MEM_R_W: c_uint = 0x39;
pub const MPU3050_FIFO_COUNT_H: c_uint = 0x3A;
pub const MPU3050_FIFO_R: c_uint = 0x3C;
pub const MPU3050_USR_CTRL: c_uint = 0x3D;
pub const MPU3050_PWR_MGM: c_uint = 0x3E;
// MPU memory bank read options

// Bits 8-11 select memory bank
pub const MPU3050_MEM_RAM_BANK_0: c_int = 0;
pub const MPU3050_MEM_RAM_BANK_1: c_int = 1;
pub const MPU3050_MEM_RAM_BANK_2: c_int = 2;
pub const MPU3050_MEM_RAM_BANK_3: c_int = 3;
pub const MPU3050_MEM_OTP_BANK_0: c_int = 4;

// Register bits
// FIFO Enable

//
// Digital Low Pass filter (DLPF)
// Full Scale (FS)
// and Synchronization
//
pub const MPU3050_EXT_SYNC_NONE: c_uint = 0x00;
pub const MPU3050_EXT_SYNC_TEMP: c_uint = 0x20;
pub const MPU3050_EXT_SYNC_GYROX: c_uint = 0x40;
pub const MPU3050_EXT_SYNC_GYROY: c_uint = 0x60;
pub const MPU3050_EXT_SYNC_GYROZ: c_uint = 0x80;
pub const MPU3050_EXT_SYNC_ACCELX: c_uint = 0xA0;
pub const MPU3050_EXT_SYNC_ACCELY: c_uint = 0xC0;
pub const MPU3050_EXT_SYNC_ACCELZ: c_uint = 0xE0;
pub const MPU3050_EXT_SYNC_MASK: c_uint = 0xE0;
pub const MPU3050_EXT_SYNC_SHIFT: c_int = 5;
pub const MPU3050_FS_250DPS: c_uint = 0x00;
pub const MPU3050_FS_500DPS: c_uint = 0x08;
pub const MPU3050_FS_1000DPS: c_uint = 0x10;
pub const MPU3050_FS_2000DPS: c_uint = 0x18;
pub const MPU3050_FS_MASK: c_uint = 0x18;
pub const MPU3050_FS_SHIFT: c_int = 3;
pub const MPU3050_DLPF_CFG_256HZ_NOLPF2: c_uint = 0x00;
pub const MPU3050_DLPF_CFG_188HZ: c_uint = 0x01;
pub const MPU3050_DLPF_CFG_98HZ: c_uint = 0x02;
pub const MPU3050_DLPF_CFG_42HZ: c_uint = 0x03;
pub const MPU3050_DLPF_CFG_20HZ: c_uint = 0x04;
pub const MPU3050_DLPF_CFG_10HZ: c_uint = 0x05;
pub const MPU3050_DLPF_CFG_5HZ: c_uint = 0x06;
pub const MPU3050_DLPF_CFG_2100HZ_NOLPF: c_uint = 0x07;
pub const MPU3050_DLPF_CFG_MASK: c_uint = 0x07;
pub const MPU3050_DLPF_CFG_SHIFT: c_int = 0;
// Interrupt config

// Interrupt status

// USR_CTRL

// PWR_MGM
pub const MPU3050_PWR_MGM_PLL_X: c_uint = 0x01;
pub const MPU3050_PWR_MGM_PLL_Y: c_uint = 0x02;
pub const MPU3050_PWR_MGM_PLL_Z: c_uint = 0x03;
pub const MPU3050_PWR_MGM_CLKSEL_MASK: c_uint = 0x07;

pub const MPU3050_PWR_MGM_MASK: c_uint = 0xff;
//
// Fullscale precision is (for finest precision) +/- 250 deg/s, so the full
// scale is actually 500 deg/s. All 16 bits are then used to cover this scale,
// in two's complement.
//
    static unsigned int mpu3050_fs_precision[] = {
    IIO_DEGREE_TO_RAD(250),
    IIO_DEGREE_TO_RAD(500),
    IIO_DEGREE_TO_RAD(1000),
    IIO_DEGREE_TO_RAD(2000)
    };
//
// Regulator names
//
    static const char mpu3050_reg_vdd[] = "vdd";
    static const char mpu3050_reg_vlogic[] = "vlogic";
#[no_mangle]
unsafe extern "C" fn mpu3050_get_freq(mpu3050: *mut mpu3050) -> c_uint {
    static unsigned int mpu3050_get_freq(struct mpu3050 *mpu3050)
    {
    unsigned int freq;
    if (mpu3050.lpf == MPU3050_DLPF_CFG_256HZ_NOLPF2)
    freq = 8000;
    else
    freq = 1000;
    freq /= (mpu3050.divisor + 1);
    return freq;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_start_sampling(mpu3050: *mut mpu3050) -> c_int {
    static int mpu3050_start_sampling(struct mpu3050 *mpu3050)
    {
    __be16 raw_val[3];
    int ret;
    int i;
// Reset
    ret = regmap_set_bits(mpu3050.map, MPU3050_PWR_MGM,
    MPU3050_PWR_MGM_RESET);
    if (ret)
    return ret;
// Turn on the Z-axis PLL
    ret = regmap_update_bits(mpu3050.map, MPU3050_PWR_MGM,
    MPU3050_PWR_MGM_CLKSEL_MASK,
    MPU3050_PWR_MGM_PLL_Z);
    if (ret)
    return ret;
// Write calibration offset registers
    for (i = 0; i < 3; i++)
    raw_val[i] = cpu_to_be16(mpu3050.calibration[i]);
    ret = regmap_bulk_write(mpu3050.map, MPU3050_X_OFFS_USR_H, raw_val,
    sizeof(raw_val));
    if (ret)
    return ret;
// Set low pass filter (sample rate), sync and full scale
    ret = regmap_write(mpu3050.map, MPU3050_DLPF_FS_SYNC,
    MPU3050_EXT_SYNC_NONE << MPU3050_EXT_SYNC_SHIFT |
    mpu3050.fullscale << MPU3050_FS_SHIFT |
    mpu3050.lpf << MPU3050_DLPF_CFG_SHIFT);
    if (ret)
    return ret;
// Set up sampling frequency
    ret = regmap_write(mpu3050.map, MPU3050_SMPLRT_DIV, mpu3050.divisor);
    if (ret)
    return ret;
//
// Max 50 ms start-up time after setting DLPF_FS_SYNC
// according to the data sheet, then wait for the next sample
// at this frequency T = 1000/f ms.
//
    msleep(50 + 1000 / mpu3050_get_freq(mpu3050));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_set_8khz_samplerate(mpu3050: *mut mpu3050) -> c_int {
    static int mpu3050_set_8khz_samplerate(struct mpu3050 *mpu3050)
    {
    int ret;
    u8 divisor;
    enum mpu3050_lpf lpf;
    lpf = mpu3050.lpf;
    divisor = mpu3050.divisor;
    mpu3050.lpf = LPF_256_HZ_NOLPF; /* 8 kHz base frequency */
    mpu3050.divisor = 0; /* Divide by 1 */
    ret = mpu3050_start_sampling(mpu3050);
    mpu3050.lpf = lpf;
    mpu3050.divisor = divisor;
    return ret;
    }
    static int mpu3050_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2,
    long mask)
    {
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    int ret;
    __be16 raw_val;
    switch (mask) {
    case IIO_CHAN_INFO_OFFSET:
    switch (chan.type) {
    case IIO_TEMP:
//
// The temperature scaling is (x+23000)/280 Celsius
// for the "best fit straight line" temperature range
// of -30C..85C.  The 23000 includes room temperature
// offset of +35C, 280 is the precision scale and x is
// the 16-bit signed integer reported by hardware.
//
// Temperature value itself represents temperature of
// the sensor die.
//
// val = 23000;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_CALIBBIAS:
    switch (chan.type) {
    case IIO_ANGL_VEL:
// val = mpu3050->calibration[chan->scan_index-1];
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = mpu3050_get_freq(mpu3050);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_TEMP:
// Millidegrees, see about temperature scaling above
// val = 1000;
// val2 = 280;
    return IIO_VAL_FRACTIONAL;
    case IIO_ANGL_VEL:
//
// Convert to the corresponding full scale in
// radians. All 16 bits are used with sign to
// span the available scale: to account for the one
// missing value if we multiply by 1/S16_MAX, instead
// multiply with 2/U16_MAX.
//
// val = mpu3050_fs_precision[mpu3050->fullscale] * 2;
// val2 = U16_MAX;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_RAW:
// Resume device
    ret = pm_runtime_resume_and_get(mpu3050.dev);
    if (ret)
    return ret;
    mutex_lock(&mpu3050.lock);
    ret = mpu3050_set_8khz_samplerate(mpu3050);
    if (ret)
    goto out_read_raw_unlock;
    switch (chan.type) {
    case IIO_TEMP:
    ret = regmap_bulk_read(mpu3050.map, MPU3050_TEMP_H,
    &raw_val, sizeof(raw_val));
    if (ret) {
    dev_err(mpu3050.dev,
    "error reading temperature\n");
    goto out_read_raw_unlock;
    }
// val = (s16)be16_to_cpu(raw_val);
    ret = IIO_VAL_INT;
    goto out_read_raw_unlock;
    case IIO_ANGL_VEL:
    ret = regmap_bulk_read(mpu3050.map,
    MPU3050_AXIS_REGS(chan.scan_index-1),
    &raw_val,
    sizeof(raw_val));
    if (ret) {
    dev_err(mpu3050.dev,
    "error reading axis data\n");
    goto out_read_raw_unlock;
    }
// val = (s16)be16_to_cpu(raw_val);
    ret = IIO_VAL_INT;
    goto out_read_raw_unlock;
    default:
    ret = -EINVAL;
    goto out_read_raw_unlock;
    }
    default:
    break;
    }
    return -EINVAL;
    out_read_raw_unlock:
    mutex_unlock(&mpu3050.lock);
    pm_runtime_put_autosuspend(mpu3050.dev);
    return ret;
    }
    static int mpu3050_write_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    int val, int val2, long mask)
    {
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
//
// Couldn't figure out a way to precalculate these at compile time.
//
    unsigned int fs250 =
    DIV_ROUND_CLOSEST(mpu3050_fs_precision[0] * 1000000 * 2,
    U16_MAX);
    unsigned int fs500 =
    DIV_ROUND_CLOSEST(mpu3050_fs_precision[1] * 1000000 * 2,
    U16_MAX);
    unsigned int fs1000 =
    DIV_ROUND_CLOSEST(mpu3050_fs_precision[2] * 1000000 * 2,
    U16_MAX);
    unsigned int fs2000 =
    DIV_ROUND_CLOSEST(mpu3050_fs_precision[3] * 1000000 * 2,
    U16_MAX);
    switch (mask) {
    case IIO_CHAN_INFO_CALIBBIAS:
    if (chan.type != IIO_ANGL_VEL)
    return -EINVAL;
    mpu3050.calibration[chan.scan_index-1] = val;
    return 0;
    case IIO_CHAN_INFO_SAMP_FREQ:
//
// The max samplerate is 8000 Hz, the minimum
// 1000 / 256 ~= 4 Hz
//
    if (val < 4 || val > 8000)
    return -EINVAL;
//
// Above 1000 Hz we must turn off the digital low pass filter
// so we get a base frequency of 8kHz to the divider
//
    if (val > 1000) {
    mpu3050.lpf = LPF_256_HZ_NOLPF;
    mpu3050.divisor = DIV_ROUND_CLOSEST(8000, val) - 1;
    return 0;
    }
    mpu3050.lpf = LPF_188_HZ;
    mpu3050.divisor = DIV_ROUND_CLOSEST(1000, val) - 1;
    return 0;
    case IIO_CHAN_INFO_SCALE:
    if (chan.type != IIO_ANGL_VEL)
    return -EINVAL;
//
// We support +/-250, +/-500, +/-1000 and +/2000 deg/s
// which means we need to round to the closest radians
// which will be roughly +/-4.3, +/-8.7, +/-17.5, +/-35
// rad/s. The scale is then for the 16 bits used to cover
// it 2/(2^16) of that.
//
// Just too large, set the max range
    if (val != 0) {
    mpu3050.fullscale = FS_2000_DPS;
    return 0;
    }
//
// Now we're dealing with fractions below zero in millirad/s
// do some integer interpolation and match with the closest
// fullscale in the table.
//
    if (val2 <= fs250 ||
    val2 < ((fs500 + fs250) / 2))
    mpu3050.fullscale = FS_250_DPS;
    else if (val2 <= fs500 ||
    val2 < ((fs1000 + fs500) / 2))
    mpu3050.fullscale = FS_500_DPS;
    else if (val2 <= fs1000 ||
    val2 < ((fs2000 + fs1000) / 2))
    mpu3050.fullscale = FS_1000_DPS;
    else
// Catch-all
    mpu3050.fullscale = FS_2000_DPS;
    return 0;
    default:
    break;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t mpu3050_trigger_handler(int irq, void *p)
    {
    const struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    int ret;
    struct {
    __be16 chans[4];
    aligned_s64 timestamp;
    } scan;
    s64 timestamp;
    let mut datums_from_fifo: c_uint = 0;
//
// If we're using the hardware trigger, get the precise timestamp from
// the top half of the threaded IRQ handler. Otherwise get the
// timestamp here so it will be close in time to the actual values
// read from the registers.
//
    if (iio_trigger_using_own(indio_dev))
    timestamp = mpu3050.hw_timestamp;
    else
    timestamp = iio_get_time_ns(indio_dev);
    mutex_lock(&mpu3050.lock);
// Using the hardware IRQ trigger? Check the buffer then.
    if (mpu3050.hw_irq_trigger) {
    __be16 raw_fifocnt;
    u16 fifocnt;
// X, Y, Z + temperature
    let mut bytes_per_datum: c_uint = 8;
    let mut fifo_overflow: bool = false;
    ret = regmap_bulk_read(mpu3050.map,
    MPU3050_FIFO_COUNT_H,
    &raw_fifocnt,
    sizeof(raw_fifocnt));
    if (ret)
    goto out_trigger_unlock;
    fifocnt = be16_to_cpu(raw_fifocnt);
    if (fifocnt == 512) {
    dev_info(mpu3050.dev,
    "FIFO overflow! Emptying and resetting FIFO\n");
    fifo_overflow = true;
// Reset and enable the FIFO
    ret = regmap_set_bits(mpu3050.map, MPU3050_USR_CTRL,
    MPU3050_USR_CTRL_FIFO_EN |
    MPU3050_USR_CTRL_FIFO_RST);
    if (ret) {
    dev_info(mpu3050.dev, "error resetting FIFO\n");
    goto out_trigger_unlock;
    }
    mpu3050.pending_fifo_footer = false;
    }
    if (fifocnt)
    dev_dbg(mpu3050.dev,
    "%d bytes in the FIFO\n",
    fifocnt);
    while (!fifo_overflow && fifocnt > bytes_per_datum) {
    unsigned int toread;
    unsigned int offset;
    __be16 fifo_values[5];
//
// If there is a FIFO footer in the pipe, first clear
// that out. This follows the complex algorithm in the
// datasheet that states that you may never leave the
// FIFO empty after the first reading: you have to
// always leave two footer bytes in it. The footer is
// in practice just two zero bytes.
//
    if (mpu3050.pending_fifo_footer) {
    toread = bytes_per_datum + 2;
    offset = 0;
    } else {
    toread = bytes_per_datum;
    offset = 1;
// Put in some dummy value
    fifo_values[0] = cpu_to_be16(0xAAAA);
    }
    ret = regmap_bulk_read(mpu3050.map,
    MPU3050_FIFO_R,
    &fifo_values[offset],
    toread);
    if (ret)
    goto out_trigger_unlock;
    dev_dbg(mpu3050.dev,
    "%04x %04x %04x %04x %04x\n",
    fifo_values[0],
    fifo_values[1],
    fifo_values[2],
    fifo_values[3],
    fifo_values[4]);
// Index past the footer (fifo_values[0]) and push
    iio_push_to_buffers_with_ts_unaligned(indio_dev,
    &fifo_values[1],
    sizeof(__be16) * 4,
    timestamp);
    fifocnt -= toread;
    datums_from_fifo++;
    mpu3050.pending_fifo_footer = true;
//
// If we're emptying the FIFO, just make sure to
// check if something new appeared.
//
    if (fifocnt < bytes_per_datum) {
    ret = regmap_bulk_read(mpu3050.map,
    MPU3050_FIFO_COUNT_H,
    &raw_fifocnt,
    sizeof(raw_fifocnt));
    if (ret)
    goto out_trigger_unlock;
    fifocnt = be16_to_cpu(raw_fifocnt);
    }
    if (fifocnt < bytes_per_datum)
    dev_dbg(mpu3050.dev,
    "%d bytes left in the FIFO\n",
    fifocnt);
//
// At this point, the timestamp that triggered the
// hardware interrupt is no longer valid for what
// we are reading (the interrupt likely fired for
// the value on the top of the FIFO), so set the
// timestamp to zero and let userspace deal with it.
//
    timestamp = 0;
    }
    }
//
// If we picked some datums from the FIFO that's enough, else
// fall through and just read from the current value registers.
// This happens in two cases:
//
// - We are using some other trigger (external, like an HRTimer)
// than the sensor's own sample generator. In this case the
// sensor is just set to the max sampling frequency and we give
// the trigger a copy of the latest value every time we get here.
//
// - The hardware trigger is active but unused and we actually use
// another trigger which calls here with a frequency higher
// than what the device provides data. We will then just read
// duplicate values directly from the hardware registers.
//
    if (datums_from_fifo) {
    dev_dbg(mpu3050.dev,
    "read %d datums from the FIFO\n",
    datums_from_fifo);
    goto out_trigger_unlock;
    }
    ret = regmap_bulk_read(mpu3050.map, MPU3050_TEMP_H, scan.chans,
    sizeof(scan.chans));
    if (ret) {
    dev_err(mpu3050.dev,
    "error reading axis data\n");
    goto out_trigger_unlock;
    }
    iio_push_to_buffers_with_timestamp(indio_dev, &scan, timestamp);
    out_trigger_unlock:
    mutex_unlock(&mpu3050.lock);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int mpu3050_buffer_preenable(struct iio_dev *indio_dev)
    {
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    int ret;
    ret = pm_runtime_resume_and_get(mpu3050.dev);
    if (ret)
    return ret;
// Unless we have OUR trigger active, run at full speed
    if (!mpu3050.hw_irq_trigger) {
    ret = mpu3050_set_8khz_samplerate(mpu3050);
    if (ret)
    pm_runtime_put_autosuspend(mpu3050.dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int mpu3050_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    pm_runtime_put_autosuspend(mpu3050.dev);
    return 0;
    }
    static const struct iio_buffer_setup_ops mpu3050_buffer_setup_ops = {
    .preenable = mpu3050_buffer_preenable,
    .postdisable = mpu3050_buffer_postdisable,
    };
    static const struct iio_mount_matrix *
    mpu3050_get_mount_matrix(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    return &mpu3050.orientation;
    }
    static const struct iio_chan_spec_ext_info mpu3050_ext_info[] = {
    IIO_MOUNT_MATRIX(IIO_SHARED_BY_TYPE, mpu3050_get_mount_matrix),
    { }
    };

    {								\
    .type = IIO_ANGL_VEL,					\
    .modified = 1,						\
    .channel2 = IIO_MOD_##axis,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_CALIBBIAS),			\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),\
    .ext_info = mpu3050_ext_info,				\
    .scan_index = index,					\
    .scan_type = {						\
    .sign = 's',					\
    .realbits = 16,					\
    .storagebits = 16,				\
    .endianness = IIO_BE,				\
    },							\
    }
    static const struct iio_chan_spec mpu3050_channels[] = {
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_OFFSET),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 16,
    .storagebits = 16,
    .endianness = IIO_BE,
    },
    },
    MPU3050_AXIS_CHANNEL(X, 1),
    MPU3050_AXIS_CHANNEL(Y, 2),
    MPU3050_AXIS_CHANNEL(Z, 3),
    IIO_CHAN_SOFT_TIMESTAMP(4),
    };
// Four channels apart from timestamp, scan mask = 0x0f
    static const unsigned long mpu3050_scan_masks[] = { 0xf, 0 };
//
// These are just the hardcoded factors resulting from the more elaborate
// calculations done with fractions in the scale raw get/set functions.
//
    static IIO_CONST_ATTR(anglevel_scale_available,
    "0.000122070 "
    "0.000274658 "
    "0.000518798 "
    "0.001068115");
    static struct attribute *mpu3050_attributes[] = {
    &iio_const_attr_anglevel_scale_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group mpu3050_attribute_group = {
    .attrs = mpu3050_attributes,
    };
    static const struct iio_info mpu3050_info = {
    .read_raw = mpu3050_read_raw,
    .write_raw = mpu3050_write_raw,
    .attrs = &mpu3050_attribute_group,
    };
//
// mpu3050_read_mem() - read MPU-3050 internal memory
// @mpu3050: device to read from
// @bank: target bank
// @addr: target address
// @len: number of bytes
// @buf: the buffer to store the read bytes in
//
    static int mpu3050_read_mem(struct mpu3050 *mpu3050,
    u8 bank,
    u8 addr,
    u8 len,
    u8 *buf)
    {
    int ret;
    ret = regmap_write(mpu3050.map,
    MPU3050_BANK_SEL,
    bank);
    if (ret)
    return ret;
    ret = regmap_write(mpu3050.map,
    MPU3050_MEM_START_ADDR,
    addr);
    if (ret)
    return ret;
    return regmap_bulk_read(mpu3050.map,
    MPU3050_MEM_R_W,
    buf,
    len);
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_hw_init(mpu3050: *mut mpu3050) -> c_int {
    static int mpu3050_hw_init(struct mpu3050 *mpu3050)
    {
    int ret;
    __le64 otp_le;
    u64 otp;
// Reset
    ret = regmap_set_bits(mpu3050.map, MPU3050_PWR_MGM,
    MPU3050_PWR_MGM_RESET);
    if (ret)
    return ret;
// Turn on the PLL
    ret = regmap_update_bits(mpu3050.map,
    MPU3050_PWR_MGM,
    MPU3050_PWR_MGM_CLKSEL_MASK,
    MPU3050_PWR_MGM_PLL_Z);
    if (ret)
    return ret;
// Disable IRQs
    ret = regmap_write(mpu3050.map,
    MPU3050_INT_CFG,
    0);
    if (ret)
    return ret;
// Read out the 8 bytes of OTP (one-time-programmable) memory
    ret = mpu3050_read_mem(mpu3050,
    (MPU3050_MEM_PRFTCH |
    MPU3050_MEM_USER_BANK |
    MPU3050_MEM_OTP_BANK_0),
    0,
    sizeof(otp_le),
    (u8 *)&otp_le);
    if (ret)
    return ret;
// This is device-unique data so it goes into the entropy pool
    add_device_randomness(&otp_le, sizeof(otp_le));
    otp = le64_to_cpu(otp_le);
    dev_info(mpu3050.dev,
    "die ID: %04llX, wafer ID: %02llX, A lot ID: %04llX, "
    "W lot ID: %03llX, WP ID: %01llX, rev ID: %02llX\n",
// Die ID, bits 0-12
    FIELD_GET(GENMASK_ULL(12, 0), otp),
// Wafer ID, bits 13-17
    FIELD_GET(GENMASK_ULL(17, 13), otp),
// A lot ID, bits 18-33
    FIELD_GET(GENMASK_ULL(33, 18), otp),
// W lot ID, bits 34-45
    FIELD_GET(GENMASK_ULL(45, 34), otp),
// WP ID, bits 47-49
    FIELD_GET(GENMASK_ULL(49, 47), otp),
// rev ID, bits 50-55
    FIELD_GET(GENMASK_ULL(55, 50), otp));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_power_up(mpu3050: *mut mpu3050) -> c_int {
    static int mpu3050_power_up(struct mpu3050 *mpu3050)
    {
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(mpu3050.regs), mpu3050.regs);
    if (ret) {
    dev_err(mpu3050.dev, "cannot enable regulators\n");
    return ret;
    }
//
// 20-100 ms start-up time for register read/write according to
// the datasheet, be on the safe side and wait 200 ms.
//
    msleep(200);
// Take device out of sleep mode
    ret = regmap_clear_bits(mpu3050.map, MPU3050_PWR_MGM,
    MPU3050_PWR_MGM_SLEEP);
    if (ret) {
    regulator_bulk_disable(ARRAY_SIZE(mpu3050.regs), mpu3050.regs);
    dev_err(mpu3050.dev, "error setting power mode\n");
    return ret;
    }
    usleep_range(10000, 20000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_power_down(mpu3050: *mut mpu3050) -> c_int {
    static int mpu3050_power_down(struct mpu3050 *mpu3050)
    {
    int ret;
//
// Put MPU-3050 into sleep mode before cutting regulators.
// This is important, because we may not be the sole user
// of the regulator so the power may stay on after this, and
// then we would be wasting power unless we go to sleep mode
// first.
//
    ret = regmap_set_bits(mpu3050.map, MPU3050_PWR_MGM,
    MPU3050_PWR_MGM_SLEEP);
    if (ret)
    dev_err(mpu3050.dev, "error putting to sleep\n");
    ret = regulator_bulk_disable(ARRAY_SIZE(mpu3050.regs), mpu3050.regs);
    if (ret)
    dev_err(mpu3050.dev, "error disabling regulators\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_irq_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t mpu3050_irq_handler(int irq, void *p)
    {
    struct iio_trigger *trig = p;
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    if (!mpu3050.hw_irq_trigger)
    return IRQ_NONE;
// Get the time stamp as close in time as possible
    mpu3050.hw_timestamp = iio_get_time_ns(indio_dev);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_irq_thread(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t mpu3050_irq_thread(int irq, void *p)
    {
    struct iio_trigger *trig = p;
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    unsigned int val;
    int ret;
// ACK IRQ and check if it was from us
    ret = regmap_read(mpu3050.map, MPU3050_INT_STATUS, &val);
    if (ret) {
    dev_err(mpu3050.dev, "error reading IRQ status\n");
    return IRQ_HANDLED;
    }
    if (!(val & MPU3050_INT_STATUS_RAW_RDY))
    return IRQ_NONE;
    iio_trigger_poll_nested(p);
    return IRQ_HANDLED;
    }
//
// mpu3050_drdy_trigger_set_state() - set data ready interrupt state
// @trig: trigger instance
// @enable: true if trigger should be enabled, false to disable
//
    static int mpu3050_drdy_trigger_set_state(struct iio_trigger *trig,
    bool enable)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    unsigned int val;
    int ret;
// Disabling trigger: disable interrupt and return
    if (!enable) {
// Disable all interrupts
    ret = regmap_write(mpu3050.map,
    MPU3050_INT_CFG,
    0);
    if (ret)
    dev_err(mpu3050.dev, "error disabling IRQ\n");
// Clear IRQ flag
    ret = regmap_read(mpu3050.map, MPU3050_INT_STATUS, &val);
    if (ret)
    dev_err(mpu3050.dev, "error clearing IRQ status\n");
// Disable all things in the FIFO and reset it
    ret = regmap_write(mpu3050.map, MPU3050_FIFO_EN, 0);
    if (ret)
    dev_err(mpu3050.dev, "error disabling FIFO\n");
    ret = regmap_write(mpu3050.map, MPU3050_USR_CTRL,
    MPU3050_USR_CTRL_FIFO_RST);
    if (ret)
    dev_err(mpu3050.dev, "error resetting FIFO\n");
    pm_runtime_put_autosuspend(mpu3050.dev);
    mpu3050.hw_irq_trigger = false;
    return 0;
    } else {
// Else we're enabling the trigger from this point
    pm_runtime_get_sync(mpu3050.dev);
    mpu3050.hw_irq_trigger = true;
// Disable all things in the FIFO
    ret = regmap_write(mpu3050.map, MPU3050_FIFO_EN, 0);
    if (ret)
    return ret;
// Reset and enable the FIFO
    ret = regmap_set_bits(mpu3050.map, MPU3050_USR_CTRL,
    MPU3050_USR_CTRL_FIFO_EN |
    MPU3050_USR_CTRL_FIFO_RST);
    if (ret)
    return ret;
    mpu3050.pending_fifo_footer = false;
// Turn on the FIFO for temp+X+Y+Z
    ret = regmap_write(mpu3050.map, MPU3050_FIFO_EN,
    MPU3050_FIFO_EN_TEMP_OUT |
    MPU3050_FIFO_EN_GYRO_XOUT |
    MPU3050_FIFO_EN_GYRO_YOUT |
    MPU3050_FIFO_EN_GYRO_ZOUT |
    MPU3050_FIFO_EN_FOOTER);
    if (ret)
    return ret;
// Configure the sample engine
    ret = mpu3050_start_sampling(mpu3050);
    if (ret)
    return ret;
// Clear IRQ flag
    ret = regmap_read(mpu3050.map, MPU3050_INT_STATUS, &val);
    if (ret)
    dev_err(mpu3050.dev, "error clearing IRQ status\n");
// Give us interrupts whenever there is new data ready
    val = MPU3050_INT_RAW_RDY_EN;
    if (mpu3050.irq_actl)
    val |= MPU3050_INT_ACTL;
    if (mpu3050.irq_latch)
    val |= MPU3050_INT_LATCH_EN;
    if (mpu3050.irq_opendrain)
    val |= MPU3050_INT_OPEN;
    ret = regmap_write(mpu3050.map, MPU3050_INT_CFG, val);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct iio_trigger_ops mpu3050_trigger_ops = {
    .set_trigger_state = mpu3050_drdy_trigger_set_state,
    };
#[no_mangle]
unsafe extern "C" fn mpu3050_trigger_probe(indio_dev: *mut iio_dev, irq: c_int) -> c_int {
    static int mpu3050_trigger_probe(struct iio_dev *indio_dev, int irq)
    {
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    struct device *dev = mpu3050.dev;
    unsigned long irq_trig;
    int ret;
    mpu3050.trig = devm_iio_trigger_alloc(&indio_dev.dev,
    "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!mpu3050.trig)
    return -ENOMEM;
// Check if IRQ is open drain
    mpu3050.irq_opendrain = device_property_read_bool(dev, "drive-open-drain");
//
// Configure the interrupt generator hardware to supply whatever
// the interrupt is configured for, edges low/high level low/high,
// we can provide it all.
//
    irq_trig = irq_get_trigger_type(irq);
    switch (irq_trig) {
    case IRQF_TRIGGER_RISING:
    dev_info(&indio_dev.dev,
    "pulse interrupts on the rising edge\n");
    break;
    case IRQF_TRIGGER_FALLING:
    mpu3050.irq_actl = true;
    dev_info(&indio_dev.dev,
    "pulse interrupts on the falling edge\n");
    break;
    case IRQF_TRIGGER_HIGH:
    mpu3050.irq_latch = true;
    dev_info(&indio_dev.dev,
    "interrupts active high level\n");
//
// With level IRQs, we mask the IRQ until it is processed,
// but with edge IRQs (pulses) we can queue several interrupts
// in the top half.
//
    irq_trig |= IRQF_ONESHOT;
    break;
    case IRQF_TRIGGER_LOW:
    mpu3050.irq_latch = true;
    mpu3050.irq_actl = true;
    irq_trig |= IRQF_ONESHOT;
    dev_info(&indio_dev.dev,
    "interrupts active low level\n");
    break;
    default:
// This is the most preferred mode, if possible
    dev_err(&indio_dev.dev,
    "unsupported IRQ trigger specified (%lx), enforce "
    "rising edge\n", irq_trig);
    irq_trig = IRQF_TRIGGER_RISING;
    break;
    }
// An open drain line can be shared with several devices
    if (mpu3050.irq_opendrain)
    irq_trig |= IRQF_SHARED;
    ret = request_threaded_irq(irq,
    mpu3050_irq_handler,
    mpu3050_irq_thread,
    irq_trig,
    mpu3050.trig.name,
    mpu3050.trig);
    if (ret) {
    dev_err(dev, "can't get IRQ %d, error %d\n", irq, ret);
    return ret;
    }
    mpu3050.irq = irq;
    mpu3050.trig.dev.parent = dev;
    mpu3050.trig.ops = &mpu3050_trigger_ops;
    iio_trigger_set_drvdata(mpu3050.trig, indio_dev);
    ret = iio_trigger_register(mpu3050.trig);
    if (ret)
    goto err_iio_trigger;
    indio_dev.trig = iio_trigger_get(mpu3050.trig);
    return 0;
    err_iio_trigger:
    free_irq(mpu3050.irq, mpu3050.trig);
    return ret;
    }
    int mpu3050_common_probe(struct device *dev,
    struct regmap *map,
    int irq,
    const char *name)
    {
    struct iio_dev *indio_dev;
    struct mpu3050 *mpu3050;
    unsigned int val;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*mpu3050));
    if (!indio_dev)
    return -ENOMEM;
    mpu3050 = iio_priv(indio_dev);
    mpu3050.dev = dev;
    mpu3050.map = map;
    mutex_init(&mpu3050.lock);
// Default fullscale: 2000 degrees per second
    mpu3050.fullscale = FS_2000_DPS;
// 1 kHz, divide by 100, default frequency = 10 Hz
    mpu3050.lpf = MPU3050_DLPF_CFG_188HZ;
    mpu3050.divisor = 99;
// Read the mounting matrix, if present
    ret = iio_read_mount_matrix(dev, &mpu3050.orientation);
    if (ret)
    return ret;
// Fetch and turn on regulators
    mpu3050.regs[0].supply = mpu3050_reg_vdd;
    mpu3050.regs[1].supply = mpu3050_reg_vlogic;
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(mpu3050.regs),
    mpu3050.regs);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot get regulators\n");
    ret = mpu3050_power_up(mpu3050);
    if (ret)
    return ret;
    ret = regmap_read(map, MPU3050_CHIP_ID_REG, &val);
    if (ret) {
    dev_err(dev, "could not read device ID\n");
    ret = -ENODEV;
    goto err_power_down;
    }
    if ((val & MPU3050_CHIP_ID_MASK) != MPU3050_CHIP_ID) {
    dev_err(dev, "unsupported chip id %02x\n",
    (u8)(val & MPU3050_CHIP_ID_MASK));
    ret = -ENODEV;
    goto err_power_down;
    }
    ret = regmap_read(map, MPU3050_PRODUCT_ID_REG, &val);
    if (ret) {
    dev_err(dev, "could not read device ID\n");
    ret = -ENODEV;
    goto err_power_down;
    }
    dev_info(dev, "found MPU-3050 part no: %d, version: %d\n",
    ((val >> 4) & 0xf), (val & 0xf));
    ret = mpu3050_hw_init(mpu3050);
    if (ret)
    goto err_power_down;
    indio_dev.channels = mpu3050_channels;
    indio_dev.num_channels = ARRAY_SIZE(mpu3050_channels);
    indio_dev.info = &mpu3050_info;
    indio_dev.available_scan_masks = mpu3050_scan_masks;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.name = name;
    ret = iio_triggered_buffer_setup(indio_dev, iio_pollfunc_store_time,
    mpu3050_trigger_handler,
    &mpu3050_buffer_setup_ops);
    if (ret) {
    dev_err(dev, "triggered buffer setup failed\n");
    goto err_power_down;
    }
    dev_set_drvdata(dev, indio_dev);
// Check if we have an assigned IRQ to use as trigger
    if (irq) {
    ret = mpu3050_trigger_probe(indio_dev, irq);
    if (ret)
    dev_err(dev, "failed to register trigger\n");
    }
// Enable runtime PM
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
//
// Set autosuspend to two orders of magnitude larger than the
// start-up time. 100ms start-up time means 10000ms autosuspend,
// i.e. 10 seconds.
//
    pm_runtime_set_autosuspend_delay(dev, 10000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_put(dev);
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(dev, "device register failed\n");
    goto err_iio_device_register;
    }
    return 0;
    err_iio_device_register:
    pm_runtime_get_sync(dev);
    pm_runtime_put_noidle(dev);
    pm_runtime_disable(dev);
    if (irq)
    free_irq(mpu3050.irq, mpu3050.trig);
    iio_triggered_buffer_cleanup(indio_dev);
    err_power_down:
    mpu3050_power_down(mpu3050);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mpu3050_common_remove(dev: *mut device) {
    void mpu3050_common_remove(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct mpu3050 *mpu3050 = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    pm_runtime_get_sync(dev);
    pm_runtime_put_noidle(dev);
    pm_runtime_disable(dev);
    if (mpu3050.irq)
    free_irq(mpu3050.irq, mpu3050.trig);
    iio_triggered_buffer_cleanup(indio_dev);
    mpu3050_power_down(mpu3050);
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_runtime_suspend(dev: *mut device) -> c_int {
    static int mpu3050_runtime_suspend(struct device *dev)
    {
    return mpu3050_power_down(iio_priv(dev_get_drvdata(dev)));
    }
#[no_mangle]
unsafe extern "C" fn mpu3050_runtime_resume(dev: *mut device) -> c_int {
    static int mpu3050_runtime_resume(struct device *dev)
    {
    return mpu3050_power_up(iio_priv(dev_get_drvdata(dev)));
    }
    DEFINE_RUNTIME_DEV_PM_OPS(mpu3050_dev_pm_ops, mpu3050_runtime_suspend,
    mpu3050_runtime_resume, core::ptr::null_mut());
    MODULE_AUTHOR("Linus Walleij");
    MODULE_DESCRIPTION("MPU3050 gyroscope driver");
    MODULE_LICENSE("GPL");
