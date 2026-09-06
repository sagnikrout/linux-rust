//! Automatically rewritten from C to Rust
//! Source: drivers/iio/proximity/cros_ec_mkbp_proximity.c
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
// Driver for cros-ec proximity sensor exposed through MKBP switch
//
// Copyright 2021 Google LLC.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_mkbp_proximity_data {
    pub ec: *mut cros_ec_device,
    pub indio_dev: *mut iio_dev,
    pub lock: mutex,
    pub notifier: notifier_block,
    pub last_proximity: c_int,
    pub enabled: bool,
}

    static const struct iio_event_spec cros_ec_mkbp_proximity_events[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static const struct iio_chan_spec cros_ec_mkbp_proximity_chan_spec[] = {
    {
    .type = IIO_PROXIMITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .event_spec = cros_ec_mkbp_proximity_events,
    .num_event_specs = ARRAY_SIZE(cros_ec_mkbp_proximity_events),
    },
    };
#[no_mangle]
unsafe extern "C" fn cros_ec_mkbp_proximity_parse_state(data: *const c_void) -> c_int {
    static int cros_ec_mkbp_proximity_parse_state(const void *data)
    {
    let mut switches: u32 = get_unaligned_le32(data);
    return !!(switches & BIT(EC_MKBP_FRONT_PROXIMITY));
    }
    static int cros_ec_mkbp_proximity_query(struct cros_ec_device *ec_dev,
    int *state)
    {
    DEFINE_RAW_FLEX(struct cros_ec_command, buf, data,
    MAX(sizeof(u32), sizeof(struct ec_params_mkbp_info)));
    struct ec_params_mkbp_info *params = (struct ec_params_mkbp_info *)buf.data;
    struct cros_ec_command *msg = buf;
    u32 *switches = (u32 *)buf.data;
    let mut insize: usize = sizeof(*switches);
    int ret;
    msg.command = EC_CMD_MKBP_INFO;
    msg.version = 1;
    msg.outsize = sizeof(*params);
    msg.insize = insize;
    params.info_type = EC_MKBP_INFO_CURRENT;
    params.event_type = EC_MKBP_EVENT_SWITCH;
    ret = cros_ec_cmd_xfer_status(ec_dev, msg);
    if (ret < 0)
    return ret;
    if (ret != insize) {
    dev_warn(ec_dev.dev, "wrong result size: %d != %zu\n", ret,
    insize);
    return -EPROTO;
    }
// state = cros_ec_mkbp_proximity_parse_state(switches);
    return IIO_VAL_INT;
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_mkbp_proximity_push_event(data: *mut cros_ec_mkbp_proximity_data, state: c_int) {
    static void cros_ec_mkbp_proximity_push_event(struct cros_ec_mkbp_proximity_data *data, int state)
    {
    s64 timestamp;
    u64 ev;
    int dir;
    struct iio_dev *indio_dev = data.indio_dev;
    struct cros_ec_device *ec = data.ec;
    mutex_lock(&data.lock);
    if (state != data.last_proximity) {
    if (data.enabled) {
    timestamp = ktime_to_ns(ec.last_event_time);
    if (iio_device_get_clock(indio_dev) != CLOCK_BOOTTIME)
    timestamp = iio_get_time_ns(indio_dev);
    dir = state ? IIO_EV_DIR_FALLING : IIO_EV_DIR_RISING;
    ev = IIO_UNMOD_EVENT_CODE(IIO_PROXIMITY, 0,
    IIO_EV_TYPE_THRESH, dir);
    iio_push_event(indio_dev, ev, timestamp);
    }
    data.last_proximity = state;
    }
    mutex_unlock(&data.lock);
    }
    static int cros_ec_mkbp_proximity_notify(struct notifier_block *nb,
    unsigned long queued_during_suspend,
    void *_ec)
    {
    struct cros_ec_mkbp_proximity_data *data;
    struct cros_ec_device *ec = _ec;
    let mut event_type: u8 = ec.event_data.event_type & EC_MKBP_EVENT_TYPE_MASK;
    void *switches;
    int state;
    if (event_type == EC_MKBP_EVENT_SWITCH) {
    data = container_of(nb, struct cros_ec_mkbp_proximity_data,
    notifier);
    switches = &ec.event_data.data.switches;
    state = cros_ec_mkbp_proximity_parse_state(switches);
    cros_ec_mkbp_proximity_push_event(data, state);
    }
    return NOTIFY_OK;
    }
    static int cros_ec_mkbp_proximity_read_raw(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan, int *val,
    int *val2, long mask)
    {
    struct cros_ec_mkbp_proximity_data *data = iio_priv(indio_dev);
    struct cros_ec_device *ec = data.ec;
    if (chan.type == IIO_PROXIMITY && mask == IIO_CHAN_INFO_RAW)
    return cros_ec_mkbp_proximity_query(ec, val);
    return -EINVAL;
    }
    static int cros_ec_mkbp_proximity_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct cros_ec_mkbp_proximity_data *data = iio_priv(indio_dev);
    return data.enabled;
    }
    static int cros_ec_mkbp_proximity_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir, bool state)
    {
    struct cros_ec_mkbp_proximity_data *data = iio_priv(indio_dev);
    mutex_lock(&data.lock);
    data.enabled = state;
    mutex_unlock(&data.lock);
    return 0;
    }
    static const struct iio_info cros_ec_mkbp_proximity_info = {
    .read_raw = cros_ec_mkbp_proximity_read_raw,
    .read_event_config = cros_ec_mkbp_proximity_read_event_config,
    .write_event_config = cros_ec_mkbp_proximity_write_event_config,
    };
#[no_mangle]
unsafe extern "C" fn cros_ec_mkbp_proximity_resume(dev: *mut device) -> c_int {
    static int cros_ec_mkbp_proximity_resume(struct device *dev)
    {
    struct cros_ec_mkbp_proximity_data *data = dev_get_drvdata(dev);
    struct cros_ec_device *ec = data.ec;
    int ret, state;
    ret = cros_ec_mkbp_proximity_query(ec, &state);
    if (ret < 0) {
    dev_warn(dev, "failed to fetch proximity state on resume: %d\n",
    ret);
    } else {
    cros_ec_mkbp_proximity_push_event(data, state);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(cros_ec_mkbp_proximity_pm_ops, core::ptr::null_mut(),
    cros_ec_mkbp_proximity_resume);
#[no_mangle]
unsafe extern "C" fn cros_ec_mkbp_proximity_probe(pdev: *mut platform_device) -> c_int {
    static int cros_ec_mkbp_proximity_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cros_ec_device *ec = dev_get_drvdata(dev.parent);
    struct iio_dev *indio_dev;
    struct cros_ec_mkbp_proximity_data *data;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.ec = ec;
    data.indio_dev = indio_dev;
    data.last_proximity = -1; /* Unknown to start */
    mutex_init(&data.lock);
    platform_set_drvdata(pdev, data);
    indio_dev.name = dev.driver.name;
    indio_dev.info = &cros_ec_mkbp_proximity_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = cros_ec_mkbp_proximity_chan_spec;
    indio_dev.num_channels = ARRAY_SIZE(cros_ec_mkbp_proximity_chan_spec);
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret)
    return ret;
    data.notifier.notifier_call = cros_ec_mkbp_proximity_notify;
    blocking_notifier_chain_register(&ec.event_notifier, &data.notifier);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_mkbp_proximity_remove(pdev: *mut platform_device) {
    static void cros_ec_mkbp_proximity_remove(struct platform_device *pdev)
    {
    struct cros_ec_mkbp_proximity_data *data = platform_get_drvdata(pdev);
    struct cros_ec_device *ec = data.ec;
    blocking_notifier_chain_unregister(&ec.event_notifier,
    &data.notifier);
    }
    static const struct of_device_id cros_ec_mkbp_proximity_of_match[] = {
    { .compatible = "google,cros-ec-mkbp-proximity" },
    { }
    };
    MODULE_DEVICE_TABLE(of, cros_ec_mkbp_proximity_of_match);
    static struct platform_driver cros_ec_mkbp_proximity_driver = {
    .driver = {
    .name = "cros-ec-mkbp-proximity",
    .of_match_table = cros_ec_mkbp_proximity_of_match,
    .pm = pm_sleep_ptr(&cros_ec_mkbp_proximity_pm_ops),
    },
    .probe = cros_ec_mkbp_proximity_probe,
    .remove = cros_ec_mkbp_proximity_remove,
    };
    module_platform_driver(cros_ec_mkbp_proximity_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ChromeOS EC MKBP proximity sensor driver");
