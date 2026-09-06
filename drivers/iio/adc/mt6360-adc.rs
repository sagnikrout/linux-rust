//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/mt6360-adc.c
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

pub const MT6360_REG_PMUCHGCTRL3: c_uint = 0x313;
pub const MT6360_REG_PMUADCCFG: c_uint = 0x356;
pub const MT6360_REG_PMUADCIDLET: c_uint = 0x358;
pub const MT6360_REG_PMUADCRPT1: c_uint = 0x35A;
// PMUCHGCTRL3 0x313

pub const MT6360_AICR_SHFT: c_int = 2;
pub const MT6360_AICR_400MA: c_uint = 0x6;
// PMUADCCFG 0x356

// PMUADCRPT1 0x35A

pub const MT6360_PREFERCH_SHFT: c_int = 4;

pub const MT6360_NO_PREFER: c_int = 15;
// Time in ms
pub const ADC_WAIT_TIME_MS: c_int = 25;
pub const ADC_CONV_TIMEOUT_MS: c_int = 100;
pub const ADC_LOOP_TIME_US: c_int = 2000;
    enum {
    MT6360_CHAN_USBID = 0,
    MT6360_CHAN_VBUSDIV5,
    MT6360_CHAN_VBUSDIV2,
    MT6360_CHAN_VSYS,
    MT6360_CHAN_VBAT,
    MT6360_CHAN_IBUS,
    MT6360_CHAN_IBAT,
    MT6360_CHAN_CHG_VDDP,
    MT6360_CHAN_TEMP_JC,
    MT6360_CHAN_VREF_TS,
    MT6360_CHAN_TS,
    MT6360_CHAN_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6360_adc_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
// Due to only one set of ADC control, this lock is used to prevent the race condition
    pub adc_lock: mutex,
    pub last_off_timestamps: [ktime_t; MT6360_CHAN_MAX],
}

#[no_mangle]
unsafe extern "C" fn mt6360_adc_read_channel(mad: *mut mt6360_adc_data, channel: c_int, val: *mut c_int) -> c_int {
    static int mt6360_adc_read_channel(struct mt6360_adc_data *mad, int channel, int *val)
    {
    __be16 adc_enable;
    u8 rpt[3];
    ktime_t predict_end_t, timeout;
    unsigned int pre_wait_time;
    int ret;
    mutex_lock(&mad.adc_lock);
// Select the preferred ADC channel
    ret = regmap_update_bits(mad.regmap, MT6360_REG_PMUADCRPT1, MT6360_PREFERCH_MASK,
    channel << MT6360_PREFERCH_SHFT);
    if (ret)
    goto out_adc_lock;
    adc_enable = cpu_to_be16(MT6360_ADCEN_MASK | BIT(channel));
    ret = regmap_raw_write(mad.regmap, MT6360_REG_PMUADCCFG, &adc_enable, sizeof(adc_enable));
    if (ret)
    goto out_adc_lock;
    predict_end_t = ktime_add_ms(mad.last_off_timestamps[channel], 2 * ADC_WAIT_TIME_MS);
    if (ktime_after(ktime_get(), predict_end_t))
    pre_wait_time = ADC_WAIT_TIME_MS;
    else
    pre_wait_time = 3 * ADC_WAIT_TIME_MS;
    if (msleep_interruptible(pre_wait_time)) {
    ret = -ERESTARTSYS;
    goto out_adc_conv;
    }
    timeout = ktime_add_ms(ktime_get(), ADC_CONV_TIMEOUT_MS);
    while (true) {
    ret = regmap_raw_read(mad.regmap, MT6360_REG_PMUADCRPT1, rpt, sizeof(rpt));
    if (ret)
    goto out_adc_conv;
//
// There are two functions, ZCV and TypeC OTP, running ADC VBAT and TS in
// background, and ADC samples are taken on a fixed frequency no matter read the
// previous one or not.
// To avoid conflict, We set minimum time threshold after enable ADC and
// check report channel is the same.
// The worst case is run the same ADC twice and background function is also running,
// ADC conversion sequence is desire channel before start ADC, background ADC,
// desire channel after start ADC.
// So the minimum correct data is three times of typical conversion time.
//
    if ((rpt[0] & MT6360_RPTCH_MASK) == channel)
    break;
    if (ktime_compare(ktime_get(), timeout) > 0) {
    ret = -ETIMEDOUT;
    goto out_adc_conv;
    }
    usleep_range(ADC_LOOP_TIME_US / 2, ADC_LOOP_TIME_US);
    }
// val = get_unaligned_be16(&rpt[1]);
    ret = IIO_VAL_INT;
    out_adc_conv:
// Only keep ADC enable
    adc_enable = cpu_to_be16(MT6360_ADCEN_MASK);
    regmap_raw_write(mad.regmap, MT6360_REG_PMUADCCFG, &adc_enable, sizeof(adc_enable));
    mad.last_off_timestamps[channel] = ktime_get();
// Config prefer channel to NO_PREFER
    regmap_update_bits(mad.regmap, MT6360_REG_PMUADCRPT1, MT6360_PREFERCH_MASK,
    MT6360_NO_PREFER << MT6360_PREFERCH_SHFT);
    out_adc_lock:
    mutex_unlock(&mad.adc_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_adc_read_scale(mad: *mut mt6360_adc_data, channel: c_int, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int mt6360_adc_read_scale(struct mt6360_adc_data *mad, int channel, int *val, int *val2)
    {
    unsigned int regval;
    int ret;
    switch (channel) {
    case MT6360_CHAN_USBID:
    case MT6360_CHAN_VSYS:
    case MT6360_CHAN_VBAT:
    case MT6360_CHAN_CHG_VDDP:
    case MT6360_CHAN_VREF_TS:
    case MT6360_CHAN_TS:
// val = 1250;
    return IIO_VAL_INT;
    case MT6360_CHAN_VBUSDIV5:
// val = 6250;
    return IIO_VAL_INT;
    case MT6360_CHAN_VBUSDIV2:
    case MT6360_CHAN_IBUS:
    case MT6360_CHAN_IBAT:
// val = 2500;
    if (channel == MT6360_CHAN_IBUS) {
// IBUS will be affected by input current limit for the different Ron
// Check whether the config is <400mA or not
    ret = regmap_read(mad.regmap, MT6360_REG_PMUCHGCTRL3, &regval);
    if (ret)
    return ret;
    regval = (regval & MT6360_AICR_MASK) >> MT6360_AICR_SHFT;
    if (regval < MT6360_AICR_400MA)
// val = 1900;
    }
    return IIO_VAL_INT;
    case MT6360_CHAN_TEMP_JC:
// val = 105;
// val2 = 100;
    return IIO_VAL_FRACTIONAL;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_adc_read_offset(mad: *mut mt6360_adc_data, channel: c_int, val: *mut c_int) -> c_int {
    static int mt6360_adc_read_offset(struct mt6360_adc_data *mad, int channel, int *val)
    {
// val = (channel == MT6360_CHAN_TEMP_JC) ? -80 : 0;
    return IIO_VAL_INT;
    }
    static int mt6360_adc_read_raw(struct iio_dev *iio_dev, const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct mt6360_adc_data *mad = iio_priv(iio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return mt6360_adc_read_channel(mad, chan.channel, val);
    case IIO_CHAN_INFO_SCALE:
    return mt6360_adc_read_scale(mad, chan.channel, val, val2);
    case IIO_CHAN_INFO_OFFSET:
    return mt6360_adc_read_offset(mad, chan.channel, val);
    }
    return -EINVAL;
    }
    static const char *mt6360_channel_labels[MT6360_CHAN_MAX] = {
    "usbid", "vbusdiv5", "vbusdiv2", "vsys", "vbat", "ibus", "ibat", "chg_vddp",
    "temp_jc", "vref_ts", "ts",
    };
    static int mt6360_adc_read_label(struct iio_dev *iio_dev, const struct iio_chan_spec *chan,
    char *label)
    {
    return sysfs_emit(label, "%s\n", mt6360_channel_labels[chan.channel]);
    }
    static const struct iio_info mt6360_adc_iio_info = {
    .read_raw = mt6360_adc_read_raw,
    .read_label = mt6360_adc_read_label,
    };

    .type = _type,						\
    .channel = MT6360_CHAN_##_idx,				\
    .scan_index = MT6360_CHAN_##_idx,			\
    .datasheet_name = #_idx,				\
    .scan_type =  {						\
    .sign = 'u',					\
    .realbits = 16,					\
    .storagebits = 16,				\
    .endianness = IIO_CPU,				\
    },							\
    .indexed = 1,						\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_SCALE) |	\
    BIT(IIO_CHAN_INFO_OFFSET),	\
    }
    static const struct iio_chan_spec mt6360_adc_channels[] = {
    MT6360_ADC_CHAN(USBID, IIO_VOLTAGE),
    MT6360_ADC_CHAN(VBUSDIV5, IIO_VOLTAGE),
    MT6360_ADC_CHAN(VBUSDIV2, IIO_VOLTAGE),
    MT6360_ADC_CHAN(VSYS, IIO_VOLTAGE),
    MT6360_ADC_CHAN(VBAT, IIO_VOLTAGE),
    MT6360_ADC_CHAN(IBUS, IIO_CURRENT),
    MT6360_ADC_CHAN(IBAT, IIO_CURRENT),
    MT6360_ADC_CHAN(CHG_VDDP, IIO_VOLTAGE),
    MT6360_ADC_CHAN(TEMP_JC, IIO_TEMP),
    MT6360_ADC_CHAN(VREF_TS, IIO_VOLTAGE),
    MT6360_ADC_CHAN(TS, IIO_VOLTAGE),
    IIO_CHAN_SOFT_TIMESTAMP(MT6360_CHAN_MAX),
    };
#[no_mangle]
unsafe extern "C" fn mt6360_adc_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t mt6360_adc_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct mt6360_adc_data *mad = iio_priv(indio_dev);
    struct {
    u16 values[MT6360_CHAN_MAX];
    aligned_s64 timestamp;
    } data = { };
    let mut i: c_int = 0, bit, val, ret;
    iio_for_each_active_channel(indio_dev, bit) {
    ret = mt6360_adc_read_channel(mad, bit, &val);
    if (ret < 0) {
    dev_warn(&indio_dev.dev, "Failed to get channel %d conversion val\n", bit);
    goto out;
    }
    data.values[i++] = val;
    }
    iio_push_to_buffers_with_timestamp(indio_dev, &data, iio_get_time_ns(indio_dev));
    out:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn mt6360_adc_reset(info: *mut mt6360_adc_data) -> c_int {
    static inline int mt6360_adc_reset(struct mt6360_adc_data *info)
    {
    __be16 adc_enable;
    ktime_t all_off_time;
    int i, ret;
// Clear ADC idle wait time to 0
    ret = regmap_write(info.regmap, MT6360_REG_PMUADCIDLET, 0);
    if (ret)
    return ret;
// Only keep ADC enable, but keep all channels off
    adc_enable = cpu_to_be16(MT6360_ADCEN_MASK);
    ret = regmap_raw_write(info.regmap, MT6360_REG_PMUADCCFG, &adc_enable, sizeof(adc_enable));
    if (ret)
    return ret;
// Reset all channel off time to the current one
    all_off_time = ktime_get();
    for (i = 0; i < MT6360_CHAN_MAX; i++)
    info.last_off_timestamps[i] = all_off_time;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_adc_probe(pdev: *mut platform_device) -> c_int {
    static int mt6360_adc_probe(struct platform_device *pdev)
    {
    struct mt6360_adc_data *mad;
    struct regmap *regmap;
    struct iio_dev *indio_dev;
    int ret;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(&pdev.dev, "Failed to get parent regmap\n");
    return -ENODEV;
    }
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*mad));
    if (!indio_dev)
    return -ENOMEM;
    mad = iio_priv(indio_dev);
    mad.dev = &pdev.dev;
    mad.regmap = regmap;
    mutex_init(&mad.adc_lock);
    ret = mt6360_adc_reset(mad);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to reset adc\n");
    return ret;
    }
    indio_dev.name = dev_name(&pdev.dev);
    indio_dev.info = &mt6360_adc_iio_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = mt6360_adc_channels;
    indio_dev.num_channels = ARRAY_SIZE(mt6360_adc_channels);
    ret = devm_iio_triggered_buffer_setup(&pdev.dev, indio_dev, core::ptr::null_mut(),
    mt6360_adc_trigger_handler, core::ptr::null_mut());
    if (ret) {
    dev_err(&pdev.dev, "Failed to allocate iio trigger buffer\n");
    return ret;
    }
    return devm_iio_device_register(&pdev.dev, indio_dev);
    }
    static const struct of_device_id mt6360_adc_of_id[] = {
    { .compatible = "mediatek,mt6360-adc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, mt6360_adc_of_id);
    static struct platform_driver mt6360_adc_driver = {
    .driver = {
    .name = "mt6360-adc",
    .of_match_table = mt6360_adc_of_id,
    },
    .probe = mt6360_adc_probe,
    };
    module_platform_driver(mt6360_adc_driver);
    MODULE_AUTHOR("Gene Chen <gene_chen@richtek.com>");
    MODULE_DESCRIPTION("MT6360 ADC Driver");
    MODULE_LICENSE("GPL v2");
