//! Automatically rewritten from C to Rust
//! Source: drivers/iio/orientation/hid-sensor-rotation.c
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
// HID Sensors Driver
// Copyright (c) 2014, Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_rot_state {
    pub callbacks: hid_sensor_hub_callbacks,
    pub common_attributes: hid_sensor_common,
    pub quaternion: hid_sensor_hub_attribute_info,
    struct {
    pub sampled_vals): IIO_DECLARE_QUATERNION(s32,,
//
// ABI regression avoidance: There are two copies of the same
// timestamp in case of userspace depending on broken alignment
// from older kernels.
//
    pub timestamp: [aligned_s64; 2],
    pub scan: },
    pub scale_pre_decml: c_int,
    pub scale_post_decml: c_int,
    pub scale_precision: c_int,
    pub value_offset: c_int,
    pub timestamp: i64,
}

    static const u32 rotation_sensitivity_addresses[] = {
    HID_USAGE_SENSOR_DATA_ORIENTATION,
    HID_USAGE_SENSOR_ORIENT_QUATERNION,
    };
    enum {
    DEV_ROT_SCAN_TYPE_16BIT,
    DEV_ROT_SCAN_TYPE_32BIT,
    };
    static const struct iio_scan_type dev_rot_scan_types[] = {
    [DEV_ROT_SCAN_TYPE_16BIT] = {
    .sign = 's',
    .realbits = 16,
// Storage bits has to stay 32 to not break userspace.
    .storagebits = 32,
    .repeat = 4,
    },
    [DEV_ROT_SCAN_TYPE_32BIT] = {
    .sign = 's',
    .realbits = 32,
    .storagebits = 32,
    .repeat = 4,
    },
    };
// Channel definitions
    static const struct iio_chan_spec dev_rot_channels[] = {
    {
    .type = IIO_ROT,
    .modified = 1,
    .channel2 = IIO_MOD_QUATERNION,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SAMP_FREQ) |
    BIT(IIO_CHAN_INFO_OFFSET) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_HYSTERESIS),
    .scan_index = 0,
    .has_ext_scan_type = 1,
    .ext_scan_type = dev_rot_scan_types,
    .num_ext_scan_type = ARRAY_SIZE(dev_rot_scan_types),
    },
    IIO_CHAN_SOFT_TIMESTAMP(1)
    };
// Channel read_raw handler
    static int dev_rot_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int size, int *vals, int *val_len, long mask)
    {
    struct dev_rot_state *rot_state = iio_priv(indio_dev);
    struct hid_sensor_hub_device *hsdev = rot_state.common_attributes.hsdev;
    struct hid_sensor_hub_attribute_info *info = &rot_state.quaternion;
    let mut usage_id: u32 = HID_USAGE_SENSOR_ORIENT_QUATERNION;
    union {
    s16 val16[4];
    s32 val32[4];
    } raw_buf;
    int ret_type;
    int i;
    vals[0] = 0;
    vals[1] = 0;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (size >= 4) {
    if (info.size <= 0 || info.size > sizeof(raw_buf))
    return -EINVAL;
    hid_sensor_power_state(&rot_state.common_attributes, true);
    ret_type = sensor_hub_input_attr_read_values(hsdev,
    hsdev.usage,
    usage_id,
    info.report_id,
    SENSOR_HUB_SYNC,
    info.size,
    (u8 *)&raw_buf);
    hid_sensor_power_state(&rot_state.common_attributes, false);
    if (ret_type < 0)
    return ret_type;
    switch (info.size) {
    case sizeof(raw_buf.val16):
    for (i = 0; i < ARRAY_SIZE(raw_buf.val16); i++)
    vals[i] = raw_buf.val16[i];
    break;
    case sizeof(raw_buf.val32):
    for (i = 0; i < ARRAY_SIZE(raw_buf.val32); i++)
    vals[i] = raw_buf.val32[i];
    break;
    default:
    return -EINVAL;
    }
    ret_type = IIO_VAL_INT_MULTIPLE;
// val_len =  4;
    } else
    ret_type = -EINVAL;
    break;
    case IIO_CHAN_INFO_SCALE:
    vals[0] = rot_state.scale_pre_decml;
    vals[1] = rot_state.scale_post_decml;
    return rot_state.scale_precision;
    case IIO_CHAN_INFO_OFFSET:
// vals = rot_state->value_offset;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret_type = hid_sensor_read_samp_freq_value(
    &rot_state.common_attributes, &vals[0], &vals[1]);
    break;
    case IIO_CHAN_INFO_HYSTERESIS:
    ret_type = hid_sensor_read_raw_hyst_value(
    &rot_state.common_attributes, &vals[0], &vals[1]);
    break;
    default:
    ret_type = -EINVAL;
    break;
    }
    return ret_type;
    }
// Channel write_raw handler
    static int dev_rot_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct dev_rot_state *rot_state = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = hid_sensor_write_samp_freq_value(
    &rot_state.common_attributes, val, val2);
    break;
    case IIO_CHAN_INFO_HYSTERESIS:
    ret = hid_sensor_write_raw_hyst_value(
    &rot_state.common_attributes, val, val2);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static int dev_rot_get_current_scan_type(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct dev_rot_state *rot_state = iio_priv(indio_dev);
    switch (rot_state.quaternion.size / 4) {
    case sizeof(s16):
    return DEV_ROT_SCAN_TYPE_16BIT;
    case sizeof(s32):
    return DEV_ROT_SCAN_TYPE_32BIT;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info dev_rot_info = {
    .read_raw_multi = &dev_rot_read_raw,
    .write_raw = &dev_rot_write_raw,
    .get_current_scan_type = &dev_rot_get_current_scan_type,
    };
// Callback handler to send event after all samples are received and captured
    static int dev_rot_proc_event(struct hid_sensor_hub_device *hsdev,
    u32 usage_id, void *priv)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(priv);
    struct dev_rot_state *rot_state = iio_priv(indio_dev);
    dev_dbg(&indio_dev.dev, "dev_rot_proc_event\n");
    if (atomic_read(&rot_state.common_attributes.data_ready)) {
    if (!rot_state.timestamp)
    rot_state.timestamp = iio_get_time_ns(indio_dev);
//
// ABI regression avoidance: IIO previously had an incorrect
// implementation of iio_push_to_buffers_with_timestamp() that
// put the timestamp in the last 8 bytes of the buffer, which
// was incorrect according to the IIO ABI. To avoid breaking
// userspace that may be depending on this broken behavior, we
// put the timestamp in both the correct place [0] and the old
// incorrect place [1].
//
    rot_state.scan.timestamp[0] = rot_state.timestamp;
    rot_state.scan.timestamp[1] = rot_state.timestamp;
    iio_push_to_buffers(indio_dev, &rot_state.scan);
    rot_state.timestamp = 0;
    }
    return 0;
    }
// Capture samples in local storage
    static int dev_rot_capture_sample(struct hid_sensor_hub_device *hsdev,
    u32 usage_id,
    size_t raw_len, char *raw_data,
    void *priv)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(priv);
    struct dev_rot_state *rot_state = iio_priv(indio_dev);
    if (usage_id == HID_USAGE_SENSOR_ORIENT_QUATERNION) {
    if (raw_len / 4 == sizeof(s16)) {
    rot_state.scan.sampled_vals[0] = ((s16 *)raw_data)[0];
    rot_state.scan.sampled_vals[1] = ((s16 *)raw_data)[1];
    rot_state.scan.sampled_vals[2] = ((s16 *)raw_data)[2];
    rot_state.scan.sampled_vals[3] = ((s16 *)raw_data)[3];
    } else {
    memcpy(&rot_state.scan.sampled_vals, raw_data,
    sizeof(rot_state.scan.sampled_vals));
    }
    dev_dbg(&indio_dev.dev, "Recd Quat len:%zu::%zu\n", raw_len,
    sizeof(rot_state.scan.sampled_vals));
    } else if (usage_id == HID_USAGE_SENSOR_TIME_TIMESTAMP) {
    rot_state.timestamp = hid_sensor_convert_timestamp(&rot_state.common_attributes,
// (s64 *)raw_data);
    }
    return 0;
    }
// Parse report which is specific to an usage id
    static int dev_rot_parse_report(struct platform_device *pdev,
    struct hid_sensor_hub_device *hsdev,
    u32 usage_id,
    struct dev_rot_state *st)
    {
    int ret;
    ret = sensor_hub_input_get_attribute_info(hsdev,
    HID_INPUT_REPORT,
    usage_id,
    HID_USAGE_SENSOR_ORIENT_QUATERNION,
    &st.quaternion);
    if (ret)
    return ret;
    dev_dbg(&pdev.dev, "dev_rot %x:%x\n", st.quaternion.index,
    st.quaternion.report_id);
    dev_dbg(&pdev.dev, "dev_rot: attrib size %d\n",
    st.quaternion.size);
    st.scale_precision = hid_sensor_format_scale(
    hsdev.usage,
    &st.quaternion,
    &st.scale_pre_decml, &st.scale_post_decml);
    return 0;
    }
// Function to initialize the processing for usage id
#[no_mangle]
unsafe extern "C" fn hid_dev_rot_probe(pdev: *mut platform_device) -> c_int {
    static int hid_dev_rot_probe(struct platform_device *pdev)
    {
    struct hid_sensor_hub_device *hsdev = dev_get_platdata(&pdev.dev);
    int ret;
    char *name;
    struct iio_dev *indio_dev;
    struct dev_rot_state *rot_state;
    indio_dev = devm_iio_device_alloc(&pdev.dev,
    sizeof(struct dev_rot_state));
    if (!indio_dev)
    return -ENOMEM;
    platform_set_drvdata(pdev, indio_dev);
    rot_state = iio_priv(indio_dev);
    rot_state.common_attributes.hsdev = hsdev;
    rot_state.common_attributes.pdev = pdev;
    switch (hsdev.usage) {
    case HID_USAGE_SENSOR_DEVICE_ORIENTATION:
    name = "dev_rotation";
    break;
    case HID_USAGE_SENSOR_RELATIVE_ORIENTATION:
    name = "relative_orientation";
    break;
    case HID_USAGE_SENSOR_GEOMAGNETIC_ORIENTATION:
    name = "geomagnetic_orientation";
    break;
    default:
    return -EINVAL;
    }
    ret = hid_sensor_parse_common_attributes(hsdev,
    hsdev.usage,
    &rot_state.common_attributes,
    rotation_sensitivity_addresses,
    ARRAY_SIZE(rotation_sensitivity_addresses));
    if (ret) {
    dev_err(&pdev.dev, "failed to setup common attributes\n");
    return ret;
    }
    ret = dev_rot_parse_report(pdev, hsdev, hsdev.usage, rot_state);
    if (ret) {
    dev_err(&pdev.dev, "failed to setup attributes\n");
    return ret;
    }
    indio_dev.channels = dev_rot_channels;
    indio_dev.num_channels = ARRAY_SIZE(dev_rot_channels);
    indio_dev.info = &dev_rot_info;
    indio_dev.name = name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    atomic_set(&rot_state.common_attributes.data_ready, 0);
    ret = hid_sensor_setup_trigger(indio_dev, name,
    &rot_state.common_attributes);
    if (ret) {
    dev_err(&pdev.dev, "trigger setup failed\n");
    return ret;
    }
    rot_state.callbacks.send_event = dev_rot_proc_event;
    rot_state.callbacks.capture_sample = dev_rot_capture_sample;
    rot_state.callbacks.pdev = pdev;
    ret = sensor_hub_register_callback(hsdev, hsdev.usage,
    &rot_state.callbacks);
    if (ret) {
    dev_err(&pdev.dev, "callback reg failed\n");
    goto error_remove_trigger;
    }
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(&pdev.dev, "device register failed\n");
    goto error_remove_callback;
    }
    return 0;
    error_remove_callback:
    sensor_hub_remove_callback(hsdev, hsdev.usage);
    error_remove_trigger:
    hid_sensor_remove_trigger(indio_dev, &rot_state.common_attributes);
    return ret;
    }
// Function to deinitialize the processing for usage id
#[no_mangle]
unsafe extern "C" fn hid_dev_rot_remove(pdev: *mut platform_device) {
    static void hid_dev_rot_remove(struct platform_device *pdev)
    {
    struct hid_sensor_hub_device *hsdev = dev_get_platdata(&pdev.dev);
    struct iio_dev *indio_dev = platform_get_drvdata(pdev);
    struct dev_rot_state *rot_state = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    sensor_hub_remove_callback(hsdev, hsdev.usage);
    hid_sensor_remove_trigger(indio_dev, &rot_state.common_attributes);
    }
    static const struct platform_device_id hid_dev_rot_ids[] = {
    {
// Format: HID-SENSOR-usage_id_in_hex_lowercase
    .name = "HID-SENSOR-20008a",
    },
    {
// Relative orientation(AG) sensor
    .name = "HID-SENSOR-20008e",
    },
    {
// Geomagnetic orientation(AM) sensor
    .name = "HID-SENSOR-2000c1",
    },
    { }
    };
    MODULE_DEVICE_TABLE(platform, hid_dev_rot_ids);
    static struct platform_driver hid_dev_rot_platform_driver = {
    .id_table = hid_dev_rot_ids,
    .driver = {
    .name	= KBUILD_MODNAME,
    .pm     = &hid_sensor_pm_ops,
    },
    .probe		= hid_dev_rot_probe,
    .remove		= hid_dev_rot_remove,
    };
    module_platform_driver(hid_dev_rot_platform_driver);
    MODULE_DESCRIPTION("HID Sensor Device Rotation");
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HID");
