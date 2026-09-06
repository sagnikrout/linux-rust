//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/berlin2-adc.c
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


//
// Marvell Berlin2 ADC driver
//
// Copyright (C) 2015 Marvell Technology Group Ltd.
//
// Antoine Tenart <antoine.tenart@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const BERLIN2_SM_CTRL: c_uint = 0x14;

pub const BERLIN2_SM_ADC_DATA: c_uint = 0x20;

pub const BERLIN2_SM_ADC_STATUS: c_uint = 0x1c;

pub const BERLIN2_SM_TSEN_STATUS: c_uint = 0x24;

pub const BERLIN2_SM_TSEN_DATA: c_uint = 0x28;

pub const BERLIN2_SM_TSEN_CTRL: c_uint = 0x74;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct berlin2_adc_priv {
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub wq: wait_queue_head_t,
    pub data_available: bool,
    pub data: c_int,
}

    {								\
    .channel		= n,				\
    .datasheet_name		= "channel"#n,			\
    .type			= t,				\
    .indexed		= 1,				\
    .info_mask_separate	= BIT(IIO_CHAN_INFO_RAW),	\
    }
    static const struct iio_chan_spec berlin2_adc_channels[] = {
    BERLIN2_ADC_CHANNEL(0, IIO_VOLTAGE),	/* external input */
    BERLIN2_ADC_CHANNEL(1, IIO_VOLTAGE),	/* external input */
    BERLIN2_ADC_CHANNEL(2, IIO_VOLTAGE),	/* external input */
    BERLIN2_ADC_CHANNEL(3, IIO_VOLTAGE),	/* external input */
    BERLIN2_ADC_CHANNEL(4, IIO_VOLTAGE),	/* reserved */
    BERLIN2_ADC_CHANNEL(5, IIO_VOLTAGE),	/* reserved */
    {					/* temperature sensor */
    .channel		= 6,
    .datasheet_name		= "channel6",
    .type			= IIO_TEMP,
    .indexed		= 0,
    .info_mask_separate	= BIT(IIO_CHAN_INFO_PROCESSED),
    },
    BERLIN2_ADC_CHANNEL(7, IIO_VOLTAGE),	/* reserved */
    IIO_CHAN_SOFT_TIMESTAMP(8),		/* timestamp */
    };
#[no_mangle]
unsafe extern "C" fn berlin2_adc_read(indio_dev: *mut iio_dev, channel: c_int) -> c_int {
    static int berlin2_adc_read(struct iio_dev *indio_dev, int channel)
    {
    struct berlin2_adc_priv *priv = iio_priv(indio_dev);
    int data, ret;
    mutex_lock(&priv.lock);
// Enable the interrupts
    regmap_write(priv.regmap, BERLIN2_SM_ADC_STATUS,
    BERLIN2_SM_ADC_STATUS_INT_EN(channel));
// Configure the ADC
    regmap_update_bits(priv.regmap, BERLIN2_SM_CTRL,
    BERLIN2_SM_CTRL_ADC_RESET |
    BERLIN2_SM_CTRL_ADC_SEL_MASK |
    BERLIN2_SM_CTRL_ADC_START,
    BERLIN2_SM_CTRL_ADC_SEL(channel) |
    BERLIN2_SM_CTRL_ADC_START);
    ret = wait_event_interruptible_timeout(priv.wq, priv.data_available,
    msecs_to_jiffies(1000));
// Disable the interrupts
    regmap_clear_bits(priv.regmap, BERLIN2_SM_ADC_STATUS,
    BERLIN2_SM_ADC_STATUS_INT_EN(channel));
    if (ret == 0)
    ret = -ETIMEDOUT;
    if (ret < 0) {
    mutex_unlock(&priv.lock);
    return ret;
    }
    regmap_clear_bits(priv.regmap, BERLIN2_SM_CTRL,
    BERLIN2_SM_CTRL_ADC_START);
    data = priv.data;
    priv.data_available = false;
    mutex_unlock(&priv.lock);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn berlin2_adc_tsen_read(indio_dev: *mut iio_dev) -> c_int {
    static int berlin2_adc_tsen_read(struct iio_dev *indio_dev)
    {
    struct berlin2_adc_priv *priv = iio_priv(indio_dev);
    int data, ret;
    mutex_lock(&priv.lock);
// Enable interrupts
    regmap_write(priv.regmap, BERLIN2_SM_TSEN_STATUS,
    BERLIN2_SM_TSEN_STATUS_INT_EN);
// Configure the ADC
    regmap_update_bits(priv.regmap, BERLIN2_SM_CTRL,
    BERLIN2_SM_CTRL_TSEN_RESET |
    BERLIN2_SM_CTRL_ADC_ROTATE,
    BERLIN2_SM_CTRL_ADC_ROTATE);
// Configure the temperature sensor
    regmap_update_bits(priv.regmap, BERLIN2_SM_TSEN_CTRL,
    BERLIN2_SM_TSEN_CTRL_TRIM_MASK |
    BERLIN2_SM_TSEN_CTRL_SETTLING_MASK |
    BERLIN2_SM_TSEN_CTRL_START,
    BERLIN2_SM_TSEN_CTRL_TRIM(3) |
    BERLIN2_SM_TSEN_CTRL_SETTLING_12 |
    BERLIN2_SM_TSEN_CTRL_START);
    ret = wait_event_interruptible_timeout(priv.wq, priv.data_available,
    msecs_to_jiffies(1000));
// Disable interrupts
    regmap_clear_bits(priv.regmap, BERLIN2_SM_TSEN_STATUS,
    BERLIN2_SM_TSEN_STATUS_INT_EN);
    if (ret == 0)
    ret = -ETIMEDOUT;
    if (ret < 0) {
    mutex_unlock(&priv.lock);
    return ret;
    }
    regmap_clear_bits(priv.regmap, BERLIN2_SM_TSEN_CTRL,
    BERLIN2_SM_TSEN_CTRL_START);
    data = priv.data;
    priv.data_available = false;
    mutex_unlock(&priv.lock);
    return data;
    }
    static int berlin2_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    int temp;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (chan.type != IIO_VOLTAGE)
    return -EINVAL;
// val = berlin2_adc_read(indio_dev, chan->channel);
    if (*val < 0)
    return *val;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_PROCESSED:
    if (chan.type != IIO_TEMP)
    return -EINVAL;
    temp = berlin2_adc_tsen_read(indio_dev);
    if (temp < 0)
    return temp;
    if (temp > 2047)
    temp -= 4096;
// Convert to milli Celsius
// val = ((temp * 100000) / 264 - 270000);
    return IIO_VAL_INT;
    default:
    break;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn berlin2_adc_irq(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t berlin2_adc_irq(int irq, void *private)
    {
    struct berlin2_adc_priv *priv = iio_priv(private);
    unsigned val;
    regmap_read(priv.regmap, BERLIN2_SM_ADC_STATUS, &val);
    if (val & BERLIN2_SM_ADC_STATUS_DATA_RDY_MASK) {
    regmap_read(priv.regmap, BERLIN2_SM_ADC_DATA, &priv.data);
    priv.data &= BERLIN2_SM_ADC_MASK;
    val &= ~BERLIN2_SM_ADC_STATUS_DATA_RDY_MASK;
    regmap_write(priv.regmap, BERLIN2_SM_ADC_STATUS, val);
    priv.data_available = true;
    wake_up_interruptible(&priv.wq);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn berlin2_adc_tsen_irq(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t berlin2_adc_tsen_irq(int irq, void *private)
    {
    struct berlin2_adc_priv *priv = iio_priv(private);
    unsigned val;
    regmap_read(priv.regmap, BERLIN2_SM_TSEN_STATUS, &val);
    if (val & BERLIN2_SM_TSEN_STATUS_DATA_RDY) {
    regmap_read(priv.regmap, BERLIN2_SM_TSEN_DATA, &priv.data);
    priv.data &= BERLIN2_SM_TSEN_MASK;
    val &= ~BERLIN2_SM_TSEN_STATUS_DATA_RDY;
    regmap_write(priv.regmap, BERLIN2_SM_TSEN_STATUS, val);
    priv.data_available = true;
    wake_up_interruptible(&priv.wq);
    }
    return IRQ_HANDLED;
    }
    static const struct iio_info berlin2_adc_info = {
    .read_raw	= berlin2_adc_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn berlin2_adc_powerdown(regmap: *mut c_void) {
    static void berlin2_adc_powerdown(void *regmap)
    {
    regmap_clear_bits(regmap, BERLIN2_SM_CTRL, BERLIN2_SM_CTRL_ADC_POWER);
    }
#[no_mangle]
unsafe extern "C" fn berlin2_adc_probe(pdev: *mut platform_device) -> c_int {
    static int berlin2_adc_probe(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev;
    struct berlin2_adc_priv *priv;
    struct device_node *parent_np = of_get_parent(pdev.dev.of_node);
    int irq, tsen_irq;
    int ret;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*priv));
    if (!indio_dev) {
    of_node_put(parent_np);
    return -ENOMEM;
    }
    priv = iio_priv(indio_dev);
    priv.regmap = syscon_node_to_regmap(parent_np);
    of_node_put(parent_np);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    irq = platform_get_irq_byname(pdev, "adc");
    if (irq < 0)
    return irq;
    tsen_irq = platform_get_irq_byname(pdev, "tsen");
    if (tsen_irq < 0)
    return tsen_irq;
    ret = devm_request_irq(&pdev.dev, irq, berlin2_adc_irq, 0,
    pdev.dev.driver.name, indio_dev);
    if (ret)
    return ret;
    ret = devm_request_irq(&pdev.dev, tsen_irq, berlin2_adc_tsen_irq,
    0, pdev.dev.driver.name, indio_dev);
    if (ret)
    return ret;
    init_waitqueue_head(&priv.wq);
    mutex_init(&priv.lock);
    indio_dev.name = dev_name(&pdev.dev);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &berlin2_adc_info;
    indio_dev.channels = berlin2_adc_channels;
    indio_dev.num_channels = ARRAY_SIZE(berlin2_adc_channels);
// Power up the ADC
    regmap_set_bits(priv.regmap, BERLIN2_SM_CTRL,
    BERLIN2_SM_CTRL_ADC_POWER);
    ret = devm_add_action_or_reset(&pdev.dev, berlin2_adc_powerdown,
    priv.regmap);
    if (ret)
    return ret;
    return devm_iio_device_register(&pdev.dev, indio_dev);
    }
    static const struct of_device_id berlin2_adc_match[] = {
    { .compatible = "marvell,berlin2-adc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, berlin2_adc_match);
    static struct platform_driver berlin2_adc_driver = {
    .driver	= {
    .name		= "berlin2-adc",
    .of_match_table	= berlin2_adc_match,
    },
    .probe	= berlin2_adc_probe,
    };
    module_platform_driver(berlin2_adc_driver);
    MODULE_AUTHOR("Antoine Tenart <antoine.tenart@free-electrons.com>");
    MODULE_DESCRIPTION("Marvell Berlin2 ADC driver");
    MODULE_LICENSE("GPL v2");
