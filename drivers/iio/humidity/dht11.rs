//! Automatically rewritten from C to Rust
//! Source: drivers/iio/humidity/dht11.c
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
// DHT11/DHT22 bit banging GPIO driver
//
// Copyright (c) Harald Geyer <harald@ccbib.org>
//

pub const DHT11_EDGES_PREAMBLE: c_int = 2;
pub const DHT11_BITS_PER_READ: c_int = 40;
//
// Note that when reading the sensor actually 84 edges are detected, but
// since the last edge is not significant, we only store 83:
//

    DHT11_EDGES_PREAMBLE + 1)
//
// Data transmission timing:
// Data bits are encoded as pulse length (high time) on the data line.
// 0-bit: 22-30uS -- typically 26uS (AM2302)
// 1-bit: 68-75uS -- typically 70uS (AM2302)
// The acutal timings also depend on the properties of the cable, with
// longer cables typically making pulses shorter.
//
// Our decoding depends on the time resolution of the system:
// timeres > 34uS ... don't know what a 1-tick pulse is
// 34uS > timeres > 30uS ... no problem (30kHz and 32kHz clocks)
// 30uS > timeres > 23uS ... don't know what a 2-tick pulse is
// timeres < 23uS ... no problem
//
// Luckily clocks in the 33-44kHz range are quite uncommon, so we can
// support most systems if the threshold for decoding a pulse as 1-bit
// is chosen carefully. If somebody really wants to support clocks around
// 40kHz, where this driver is most unreliable, there are two options.
// a) select an implementation using busy loop polling on those systems
// b) use the checksum to do some probabilistic decoding
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dht11 {
    pub dev: *mut device,
    pub gpiod: *mut gpio_desc,
    pub irq: c_int,
    pub completion: completion,
// The iio sysfs interface doesn't prevent concurrent reads:
    pub lock: mutex,
    pub timestamp: i64,
    pub temperature: c_int,
    pub humidity: c_int,
// num_edges: -1 means "no transmission in progress"
    pub num_edges: c_int,
    pub edges: [{s64 ts; int value; }; DHT11_EDGES_PER_READ],
}

//
// dht11_edges_print: show the data as actually received by the
// driver.
//
#[no_mangle]
unsafe extern "C" fn dht11_edges_print(dht11: *mut dht11) {
    static void dht11_edges_print(struct dht11 *dht11)
    {
    int i;
    dev_dbg(dht11.dev, "%d edges detected:\n", dht11.num_edges);
    for (i = 1; i < dht11.num_edges; ++i) {
    dev_dbg(dht11.dev, "%d: %lld ns %s\n", i,
    dht11.edges[i].ts - dht11.edges[i - 1].ts,
    str_high_low(dht11.edges[i - 1].value));
    }
    }

#[no_mangle]
unsafe extern "C" fn dht11_decode_byte(bits: *mut c_char) -> c_uchar {
    static unsigned char dht11_decode_byte(char *bits)
    {
    let mut ret: c_uchar = 0;
    int i;
    for (i = 0; i < 8; ++i) {
    ret <<= 1;
    if (bits[i])
    ++ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dht11_decode(dht11: *mut dht11, offset: c_int) -> c_int {
    static int dht11_decode(struct dht11 *dht11, int offset)
    {
    int i, t;
    char bits[DHT11_BITS_PER_READ];
    unsigned char temp_int, temp_dec, hum_int, hum_dec, checksum;
    for (i = 0; i < DHT11_BITS_PER_READ; ++i) {
    t = dht11.edges[offset + 2 * i + 2].ts -
    dht11.edges[offset + 2 * i + 1].ts;
    if (!dht11.edges[offset + 2 * i + 1].value) {
    dev_dbg(dht11.dev,
    "lost synchronisation at edge %d\n",
    offset + 2 * i + 1);
    return -EIO;
    }
    bits[i] = t > DHT11_THRESHOLD;
    }
    hum_int = dht11_decode_byte(bits);
    hum_dec = dht11_decode_byte(&bits[8]);
    temp_int = dht11_decode_byte(&bits[16]);
    temp_dec = dht11_decode_byte(&bits[24]);
    checksum = dht11_decode_byte(&bits[32]);
    if (((hum_int + hum_dec + temp_int + temp_dec) & 0xff) != checksum) {
    dev_dbg(dht11.dev, "invalid checksum\n");
    return -EIO;
    }
    dht11.timestamp = ktime_get_boottime_ns();
    if (hum_int < 4) {  /* DHT22: 100000 = (3*256+232)*100 */
    dht11.temperature = (((temp_int & 0x7f) << 8) + temp_dec) *
    ((temp_int & 0x80) ? -100 : 100);
    dht11.humidity = ((hum_int << 8) + hum_dec) * 100;
    } else if (temp_dec == 0 && hum_dec == 0) {  /* DHT11 */
    dht11.temperature = temp_int * 1000;
    dht11.humidity = hum_int * 1000;
    } else {
    dev_err(dht11.dev,
    "Don't know how to decode data: %d %d %d %d\n",
    hum_int, hum_dec, temp_int, temp_dec);
    return -EIO;
    }
    return 0;
    }
//
// IRQ handler called on GPIO edges
//
#[no_mangle]
unsafe extern "C" fn dht11_handle_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t dht11_handle_irq(int irq, void *data)
    {
    struct iio_dev *iio = data;
    struct dht11 *dht11 = iio_priv(iio);
    if (dht11.num_edges < DHT11_EDGES_PER_READ && dht11.num_edges >= 0) {
    dht11.edges[dht11.num_edges].ts = ktime_get_boottime_ns();
    dht11.edges[dht11.num_edges++].value =
    gpiod_get_value(dht11.gpiod);
    if (dht11.num_edges >= DHT11_EDGES_PER_READ)
    complete(&dht11.completion);
    }
    return IRQ_HANDLED;
    }
    static int dht11_read_raw(struct iio_dev *iio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long m)
    {
    struct dht11 *dht11 = iio_priv(iio_dev);
    int ret, timeres, offset;
    mutex_lock(&dht11.lock);
    if (dht11.timestamp + DHT11_DATA_VALID_TIME < ktime_get_boottime_ns()) {
    timeres = ktime_get_resolution_ns();
    dev_dbg(dht11.dev, "current timeresolution: %dns\n", timeres);
    if (timeres > DHT11_MIN_TIMERES) {
    dev_err(dht11.dev, "timeresolution %dns too low\n",
    timeres);
// In theory a better clock could become available
// at some point ... and there is no error code
// that really fits better.
//
    ret = -EAGAIN;
    goto err;
    }
    if (timeres > DHT11_AMBIG_LOW && timeres < DHT11_AMBIG_HIGH)
    dev_warn(dht11.dev,
    "timeresolution: %dns - decoding ambiguous\n",
    timeres);
    reinit_completion(&dht11.completion);
    dht11.num_edges = 0;
    ret = gpiod_direction_output(dht11.gpiod, 0);
    if (ret)
    goto err;
    usleep_range(DHT11_START_TRANSMISSION_MIN,
    DHT11_START_TRANSMISSION_MAX);
    ret = gpiod_direction_input(dht11.gpiod);
    if (ret)
    goto err;
    ret = request_irq(dht11.irq, dht11_handle_irq,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
    iio_dev.name, iio_dev);
    if (ret)
    goto err;
    ret = wait_for_completion_killable_timeout(&dht11.completion,
    HZ);
    free_irq(dht11.irq, iio_dev);

    dht11_edges_print(dht11);

    if (ret == 0 && dht11.num_edges < DHT11_EDGES_PER_READ - 1) {
    dev_err(dht11.dev, "Only %d signal edges detected\n",
    dht11.num_edges);
    ret = -ETIMEDOUT;
    }
    if (ret < 0)
    goto err;
    offset = DHT11_EDGES_PREAMBLE +
    dht11.num_edges - DHT11_EDGES_PER_READ;
    for (; offset >= 0; --offset) {
    ret = dht11_decode(dht11, offset);
    if (!ret)
    break;
    }
    if (ret)
    goto err;
    }
    ret = IIO_VAL_INT;
    if (chan.type == IIO_TEMP)
// val = dht11->temperature;
#[no_mangle]
pub unsafe extern "C" fn if(IIO_HUMIDITYRELATIVE: chan->type ==) -> else {
    else if (chan.type == IIO_HUMIDITYRELATIVE)
// val = dht11->humidity;
    else
    ret = -EINVAL;
    err:
    dht11.num_edges = -1;
    mutex_unlock(&dht11.lock);
    return ret;
    }
    static const struct iio_info dht11_iio_info = {
    .read_raw		= dht11_read_raw,
    };
    static const struct iio_chan_spec dht11_chan_spec[] = {
    { .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED), },
    { .type = IIO_HUMIDITYRELATIVE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED), }
    };
    static const struct of_device_id dht11_dt_ids[] = {
    { .compatible = "dht11", },
    { }
    };
    MODULE_DEVICE_TABLE(of, dht11_dt_ids);
#[no_mangle]
unsafe extern "C" fn dht11_probe(pdev: *mut platform_device) -> c_int {
    static int dht11_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dht11 *dht11;
    struct iio_dev *iio;
    iio = devm_iio_device_alloc(dev, sizeof(*dht11));
    if (!iio)
    return -ENOMEM;
    dht11 = iio_priv(iio);
    dht11.dev = dev;
    dht11.gpiod = devm_gpiod_get(dev, core::ptr::null_mut(), GPIOD_IN);
    if (IS_ERR(dht11.gpiod))
    return PTR_ERR(dht11.gpiod);
    dht11.irq = gpiod_to_irq(dht11.gpiod);
    if (dht11.irq < 0) {
    dev_err(dev, "GPIO %d has no interrupt\n", desc_to_gpio(dht11.gpiod));
    return -EINVAL;
    }
    dht11.timestamp = ktime_get_boottime_ns() - DHT11_DATA_VALID_TIME - 1;
    dht11.num_edges = -1;
    platform_set_drvdata(pdev, iio);
    init_completion(&dht11.completion);
    mutex_init(&dht11.lock);
    iio.name = pdev.name;
    iio.info = &dht11_iio_info;
    iio.modes = INDIO_DIRECT_MODE;
    iio.channels = dht11_chan_spec;
    iio.num_channels = ARRAY_SIZE(dht11_chan_spec);
    return devm_iio_device_register(dev, iio);
    }
    static struct platform_driver dht11_driver = {
    .driver = {
    .name	= "dht11",
    .of_match_table = dht11_dt_ids,
    },
    .probe  = dht11_probe,
    };
    module_platform_driver(dht11_driver);
    MODULE_AUTHOR("Harald Geyer <harald@ccbib.org>");
    MODULE_DESCRIPTION("DHT11 humidity/temperature sensor driver");
    MODULE_LICENSE("GPL v2");
