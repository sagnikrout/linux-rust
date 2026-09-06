//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/alienware-wmi-wmax.c
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
// Alienware WMAX WMI device driver
//
// Copyright (C) 2014 Dell Inc <Dell.Client.Kernel@dell.com>
// Copyright (C) 2025 Kurt Borja <kuurtb@gmail.com>
//

pub const WMAX_METHOD_HDMI_SOURCE: c_uint = 0x1;
pub const WMAX_METHOD_HDMI_STATUS: c_uint = 0x2;
pub const WMAX_METHOD_HDMI_CABLE: c_uint = 0x5;
pub const WMAX_METHOD_AMPLIFIER_CABLE: c_uint = 0x6;
pub const WMAX_METHOD_DEEP_SLEEP_CONTROL: c_uint = 0x0B;
pub const WMAX_METHOD_DEEP_SLEEP_STATUS: c_uint = 0x0C;
pub const WMAX_METHOD_BRIGHTNESS: c_uint = 0x3;
pub const WMAX_METHOD_ZONE_CONTROL: c_uint = 0x4;
pub const AWCC_METHOD_GET_FAN_SENSORS: c_uint = 0x13;
pub const AWCC_METHOD_THERMAL_INFORMATION: c_uint = 0x14;
pub const AWCC_METHOD_THERMAL_CONTROL: c_uint = 0x15;
pub const AWCC_METHOD_FWUP_GPIO_CONTROL: c_uint = 0x20;
pub const AWCC_METHOD_READ_TOTAL_GPIOS: c_uint = 0x21;
pub const AWCC_METHOD_READ_GPIO_STATUS: c_uint = 0x22;
pub const AWCC_METHOD_GAME_SHIFT_STATUS: c_uint = 0x25;
pub const AWCC_FAILURE_CODE: c_uint = 0xFFFFFFFF;
pub const AWCC_FAILURE_CODE_2: c_uint = 0xFFFFFFFE;

// Arbitrary limit based on supported models
pub const AWCC_MAX_RES_COUNT: c_int = 16;

    static bool force_hwmon;
    module_param_unsafe(force_hwmon, bool, 0);
    MODULE_PARM_DESC(force_hwmon, "Force probing for HWMON support without checking if the WMI backend is available");
    static bool force_platform_profile;
    module_param_unsafe(force_platform_profile, bool, 0);
    MODULE_PARM_DESC(force_platform_profile, "Forces auto-detecting thermal profiles without checking if WMI thermal backend is available");
    static bool force_gmode;
    module_param_unsafe(force_gmode, bool, 0);
    MODULE_PARM_DESC(force_gmode, "Forces G-Mode when performance profile is selected");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct awcc_quirks {
    pub hwmon: bool,
    pub pprof: bool,
    pub gmode: bool,
}

    static struct awcc_quirks g_series_quirks = {
    .hwmon = true,
    .pprof = true,
    .gmode = true,
    };
    static struct awcc_quirks generic_quirks = {
    .hwmon = true,
    .pprof = true,
    .gmode = false,
    };
    static struct awcc_quirks empty_quirks;
    static const struct dmi_system_id awcc_dmi_table[] __initconst = {
    {
    .ident = "Alienware 16 Area-51",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware 16 Area-51"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Alienware 16X Aurora",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware 16X Aurora"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Alienware 18 Area-51",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware 18 Area-51"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Alienware 16 Aurora",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware 16 Aurora"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Alienware Area-51m",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware Area-51m"),
    },
    .driver_data = &generic_quirks,
    },
    {
    .ident = "Alienware m15",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware m15"),
    },
    .driver_data = &generic_quirks,
    },
    {
    .ident = "Alienware m16 R1 AMD",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware m16 R1 AMD"),
    },
    .driver_data = &generic_quirks,
    },
    {
    .ident = "Alienware m16 R1",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware m16 R1"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Alienware m16 R2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware m16 R2"),
    },
    .driver_data = &generic_quirks,
    },
    {
    .ident = "Alienware m17",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware m17"),
    },
    .driver_data = &generic_quirks,
    },
    {
    .ident = "Alienware m18",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware m18"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Alienware x15",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware x15"),
    },
    .driver_data = &generic_quirks,
    },
    {
    .ident = "Alienware x16",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware x16"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Alienware x17",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Alienware"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alienware x17"),
    },
    .driver_data = &generic_quirks,
    },
    {
    .ident = "Dell Inc. G15",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Dell G15"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Dell Inc. G16",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Dell G16"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Dell Inc. G3",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "G3"),
    },
    .driver_data = &g_series_quirks,
    },
    {
    .ident = "Dell Inc. G5",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "G5"),
    },
    .driver_data = &g_series_quirks,
    },
    {}
    };
    enum AWCC_GET_FAN_SENSORS_OPERATIONS {
    AWCC_OP_GET_TOTAL_FAN_TEMPS		= 0x01,
    AWCC_OP_GET_FAN_TEMP_ID			= 0x02,
    };
    enum AWCC_THERMAL_INFORMATION_OPERATIONS {
    AWCC_OP_GET_SYSTEM_DESCRIPTION		= 0x02,
    AWCC_OP_GET_RESOURCE_ID			= 0x03,
    AWCC_OP_GET_TEMPERATURE			= 0x04,
    AWCC_OP_GET_FAN_RPM			= 0x05,
    AWCC_OP_GET_FAN_MIN_RPM			= 0x08,
    AWCC_OP_GET_FAN_MAX_RPM			= 0x09,
    AWCC_OP_GET_CURRENT_PROFILE		= 0x0B,
    AWCC_OP_GET_FAN_BOOST			= 0x0C,
    };
    enum AWCC_THERMAL_CONTROL_OPERATIONS {
    AWCC_OP_ACTIVATE_PROFILE		= 0x01,
    AWCC_OP_SET_FAN_BOOST			= 0x02,
    };
    enum AWCC_GAME_SHIFT_STATUS_OPERATIONS {
    AWCC_OP_TOGGLE_GAME_SHIFT		= 0x01,
    AWCC_OP_GET_GAME_SHIFT_STATUS		= 0x02,
    };
    enum AWCC_THERMAL_TABLES {
    AWCC_THERMAL_TABLE_LEGACY		= 0x9,
    AWCC_THERMAL_TABLE_USTT			= 0xA,
    };
    enum AWCC_TEMP_SENSOR_TYPES {
    AWCC_TEMP_SENSOR_CPU			= 0x01,
    AWCC_TEMP_SENSOR_FRONT			= 0x03,
    AWCC_TEMP_SENSOR_GPU			= 0x06,
    };
    enum AWCC_FAN_TYPES {
    AWCC_FAN_CPU_1				= 0x32,
    AWCC_FAN_GPU_1				= 0x33,
    AWCC_FAN_PCI				= 0x34,
    AWCC_FAN_MID				= 0x35,
    AWCC_FAN_TOP_1				= 0x36,
    AWCC_FAN_SIDE				= 0x37,
    AWCC_FAN_U2_1				= 0x38,
    AWCC_FAN_U2_2				= 0x39,
    AWCC_FAN_FRONT_1			= 0x3A,
    AWCC_FAN_CPU_2				= 0x3B,
    AWCC_FAN_GPU_2				= 0x3C,
    AWCC_FAN_TOP_2				= 0x3D,
    AWCC_FAN_TOP_3				= 0x3E,
    AWCC_FAN_FRONT_2			= 0x3F,
    AWCC_FAN_BOTTOM_1			= 0x40,
    AWCC_FAN_BOTTOM_2			= 0x41,
    };
    enum awcc_thermal_profile {
    AWCC_PROFILE_SPECIAL_CUSTOM			= 0x00,
    AWCC_PROFILE_LEGACY_QUIET			= 0x96,
    AWCC_PROFILE_LEGACY_BALANCED			= 0x97,
    AWCC_PROFILE_LEGACY_BALANCED_PERFORMANCE	= 0x98,
    AWCC_PROFILE_LEGACY_PERFORMANCE			= 0x99,
    AWCC_PROFILE_USTT_BALANCED			= 0xA0,
    AWCC_PROFILE_USTT_BALANCED_PERFORMANCE		= 0xA1,
    AWCC_PROFILE_USTT_COOL				= 0xA2,
    AWCC_PROFILE_USTT_QUIET				= 0xA3,
    AWCC_PROFILE_USTT_PERFORMANCE			= 0xA4,
    AWCC_PROFILE_USTT_LOW_POWER			= 0xA5,
    AWCC_PROFILE_SPECIAL_GMODE			= 0xAB,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmax_led_args {
    pub led_mask: u32,
    pub colors: color_platform,
    pub state: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmax_brightness_args {
    pub led_mask: u32,
    pub percentage: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmax_basic_args {
    pub arg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmax_u32_args {
    pub operation: u8,
    pub arg1: u8,
    pub arg2: u8,
    pub arg3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct awcc_fan_data {
    pub auto_channels_temp: c_ulong,
    pub min_rpm: u32,
    pub max_rpm: u32,
    pub suspend_cache: u8,
    pub id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct awcc_priv {
    pub wdev: *mut wmi_device,
    union {
    pub system_description: u32,
    struct {
    pub fan_count: u8,
    pub temp_count: u8,
    pub unknown_count: u8,
    pub profile_count: u8,
}

    u8 res_count[4];
    };
    struct device *ppdev;
    u8 supported_profiles[PLATFORM_PROFILE_LAST];
    struct device *hwdev;
    struct awcc_fan_data **fan_data;
    unsigned long temp_sensors[AWCC_ID_BITMAP_LONGS];
    u32 gpio_count;
    };
    static struct awcc_quirks *awcc;
//
// The HDMI mux sysfs node indicates the status of the HDMI input mux.
// It can toggle between standard system GPU output and HDMI input.
//
    static ssize_t cable_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct alienfx_platdata *pdata = dev_get_platdata(dev);
    struct wmax_basic_args in_args = {
    .arg = 0,
    };
    u32 out_data;
    int ret;
    ret = alienware_wmi_command(pdata.wdev, WMAX_METHOD_HDMI_CABLE,
    &in_args, sizeof(in_args), &out_data);
    if (!ret) {
    if (out_data == 0)
    return sysfs_emit(buf, "[unconnected] connected unknown\n");
#[no_mangle]
pub unsafe extern "C" fn if(1: out_data ==) -> else {
    else if (out_data == 1)
    return sysfs_emit(buf, "unconnected [connected] unknown\n");
    }
    pr_err("alienware-wmi: unknown HDMI cable status: %d\n", ret);
    return sysfs_emit(buf, "unconnected connected [unknown]\n");
    }
    static ssize_t source_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct alienfx_platdata *pdata = dev_get_platdata(dev);
    struct wmax_basic_args in_args = {
    .arg = 0,
    };
    u32 out_data;
    int ret;
    ret = alienware_wmi_command(pdata.wdev, WMAX_METHOD_HDMI_STATUS,
    &in_args, sizeof(in_args), &out_data);
    if (!ret) {
    if (out_data == 1)
    return sysfs_emit(buf, "[input] gpu unknown\n");
#[no_mangle]
pub unsafe extern "C" fn if(2: out_data ==) -> else {
    else if (out_data == 2)
    return sysfs_emit(buf, "input [gpu] unknown\n");
    }
    pr_err("alienware-wmi: unknown HDMI source status: %u\n", ret);
    return sysfs_emit(buf, "input gpu [unknown]\n");
    }
    static ssize_t source_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct alienfx_platdata *pdata = dev_get_platdata(dev);
    struct wmax_basic_args args;
    int ret;
    if (strcmp(buf, "gpu\n") == 0)
    args.arg = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(buf, 0: "input\n") ==) -> else {
    else if (strcmp(buf, "input\n") == 0)
    args.arg = 2;
    else
    args.arg = 3;
    pr_debug("alienware-wmi: setting hdmi to %d : %s", args.arg, buf);
    ret = alienware_wmi_command(pdata.wdev, WMAX_METHOD_HDMI_SOURCE, &args,
    sizeof(args), core::ptr::null_mut());
    if (ret < 0)
    pr_err("alienware-wmi: HDMI toggle failed: results: %u\n", ret);
    return count;
    }
    static DEVICE_ATTR_RO(cable);
    static DEVICE_ATTR_RW(source);
#[no_mangle]
unsafe extern "C" fn hdmi_group_visible(kobj: *mut kobject) -> bool {
    static bool hdmi_group_visible(struct kobject *kobj)
    {
    let mut alienware_interface: return = = WMAX && alienfx.hdmi_mux;
    }
    DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE(hdmi);
    static struct attribute *hdmi_attrs[] = {
    &dev_attr_cable.attr,
    &dev_attr_source.attr,
    core::ptr::null_mut(),
    };
    const struct attribute_group wmax_hdmi_attribute_group = {
    .name = "hdmi",
    .is_visible = SYSFS_GROUP_VISIBLE(hdmi),
    .attrs = hdmi_attrs,
    };
//
// Alienware GFX amplifier support
// - Currently supports reading cable status
// - Leaving expansion room to possibly support dock/undock events later
//
    static ssize_t status_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct alienfx_platdata *pdata = dev_get_platdata(dev);
    struct wmax_basic_args in_args = {
    .arg = 0,
    };
    u32 out_data;
    int ret;
    ret = alienware_wmi_command(pdata.wdev, WMAX_METHOD_AMPLIFIER_CABLE,
    &in_args, sizeof(in_args), &out_data);
    if (!ret) {
    if (out_data == 0)
    return sysfs_emit(buf, "[unconnected] connected unknown\n");
#[no_mangle]
pub unsafe extern "C" fn if(1: out_data ==) -> else {
    else if (out_data == 1)
    return sysfs_emit(buf, "unconnected [connected] unknown\n");
    }
    pr_err("alienware-wmi: unknown amplifier cable status: %d\n", ret);
    return sysfs_emit(buf, "unconnected connected [unknown]\n");
    }
    static DEVICE_ATTR_RO(status);
#[no_mangle]
unsafe extern "C" fn amplifier_group_visible(kobj: *mut kobject) -> bool {
    static bool amplifier_group_visible(struct kobject *kobj)
    {
    let mut alienware_interface: return = = WMAX && alienfx.amplifier;
    }
    DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE(amplifier);
    static struct attribute *amplifier_attrs[] = {
    &dev_attr_status.attr,
    core::ptr::null_mut(),
    };
    const struct attribute_group wmax_amplifier_attribute_group = {
    .name = "amplifier",
    .is_visible = SYSFS_GROUP_VISIBLE(amplifier),
    .attrs = amplifier_attrs,
    };
//
// Deep Sleep Control support
// - Modifies BIOS setting for deep sleep control allowing extra wakeup events
//
    static ssize_t deepsleep_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct alienfx_platdata *pdata = dev_get_platdata(dev);
    struct wmax_basic_args in_args = {
    .arg = 0,
    };
    u32 out_data;
    int ret;
    ret = alienware_wmi_command(pdata.wdev, WMAX_METHOD_DEEP_SLEEP_STATUS,
    &in_args, sizeof(in_args), &out_data);
    if (!ret) {
    if (out_data == 0)
    return sysfs_emit(buf, "[disabled] s5 s5_s4\n");
#[no_mangle]
pub unsafe extern "C" fn if(1: out_data ==) -> else {
    else if (out_data == 1)
    return sysfs_emit(buf, "disabled [s5] s5_s4\n");
#[no_mangle]
pub unsafe extern "C" fn if(2: out_data ==) -> else {
    else if (out_data == 2)
    return sysfs_emit(buf, "disabled s5 [s5_s4]\n");
    }
    pr_err("alienware-wmi: unknown deep sleep status: %d\n", ret);
    return sysfs_emit(buf, "disabled s5 s5_s4 [unknown]\n");
    }
    static ssize_t deepsleep_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct alienfx_platdata *pdata = dev_get_platdata(dev);
    struct wmax_basic_args args;
    int ret;
    if (strcmp(buf, "disabled\n") == 0)
    args.arg = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(buf, 0: "s5\n") ==) -> else {
    else if (strcmp(buf, "s5\n") == 0)
    args.arg = 1;
    else
    args.arg = 2;
    pr_debug("alienware-wmi: setting deep sleep to %d : %s", args.arg, buf);
    ret = alienware_wmi_command(pdata.wdev, WMAX_METHOD_DEEP_SLEEP_CONTROL,
    &args, sizeof(args), core::ptr::null_mut());
    if (!ret)
    pr_err("alienware-wmi: deep sleep control failed: results: %u\n", ret);
    return count;
    }
    static DEVICE_ATTR_RW(deepsleep);
#[no_mangle]
unsafe extern "C" fn deepsleep_group_visible(kobj: *mut kobject) -> bool {
    static bool deepsleep_group_visible(struct kobject *kobj)
    {
    let mut alienware_interface: return = = WMAX && alienfx.deepslp;
    }
    DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE(deepsleep);
    static struct attribute *deepsleep_attrs[] = {
    &dev_attr_deepsleep.attr,
    core::ptr::null_mut(),
    };
    const struct attribute_group wmax_deepsleep_attribute_group = {
    .name = "deepsleep",
    .is_visible = SYSFS_GROUP_VISIBLE(deepsleep),
    .attrs = deepsleep_attrs,
    };
//
// AWCC Helpers
//
    static int awcc_profile_to_pprof(enum awcc_thermal_profile profile,
    enum platform_profile_option *pprof)
    {
    switch (profile) {
    case AWCC_PROFILE_SPECIAL_CUSTOM:
// pprof = PLATFORM_PROFILE_CUSTOM;
    break;
    case AWCC_PROFILE_LEGACY_QUIET:
    case AWCC_PROFILE_USTT_QUIET:
// pprof = PLATFORM_PROFILE_QUIET;
    break;
    case AWCC_PROFILE_LEGACY_BALANCED:
    case AWCC_PROFILE_USTT_BALANCED:
// pprof = PLATFORM_PROFILE_BALANCED;
    break;
    case AWCC_PROFILE_LEGACY_BALANCED_PERFORMANCE:
    case AWCC_PROFILE_USTT_BALANCED_PERFORMANCE:
// pprof = PLATFORM_PROFILE_BALANCED_PERFORMANCE;
    break;
    case AWCC_PROFILE_LEGACY_PERFORMANCE:
    case AWCC_PROFILE_USTT_PERFORMANCE:
    case AWCC_PROFILE_SPECIAL_GMODE:
// pprof = PLATFORM_PROFILE_PERFORMANCE;
    break;
    case AWCC_PROFILE_USTT_COOL:
// pprof = PLATFORM_PROFILE_COOL;
    break;
    case AWCC_PROFILE_USTT_LOW_POWER:
// pprof = PLATFORM_PROFILE_LOW_POWER;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int awcc_wmi_command(struct wmi_device *wdev, u32 method_id,
    struct wmax_u32_args *args, u32 *out)
    {
    int ret;
    ret = alienware_wmi_command(wdev, method_id, args, sizeof(*args), out);
    if (ret)
    return ret;
    if (*out == AWCC_FAILURE_CODE || *out == AWCC_FAILURE_CODE_2)
    return -EBADRQC;
    return 0;
    }
    static int awcc_get_fan_sensors(struct wmi_device *wdev, u8 operation,
    u8 fan_id, u8 index, u32 *out)
    {
    struct wmax_u32_args args = {
    .operation = operation,
    .arg1 = fan_id,
    .arg2 = index,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_GET_FAN_SENSORS, &args, out);
    }
    static int awcc_thermal_information(struct wmi_device *wdev, u8 operation, u8 arg,
    u32 *out)
    {
    struct wmax_u32_args args = {
    .operation = operation,
    .arg1 = arg,
    .arg2 = 0,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_INFORMATION, &args, out);
    }
#[no_mangle]
unsafe extern "C" fn awcc_fwup_gpio_control(wdev: *mut wmi_device, pin: u8, status: u8) -> c_int {
    static int awcc_fwup_gpio_control(struct wmi_device *wdev, u8 pin, u8 status)
    {
    struct wmax_u32_args args = {
    .operation = pin,
    .arg1 = status,
    .arg2 = 0,
    .arg3 = 0,
    };
    u32 out;
    return awcc_wmi_command(wdev, AWCC_METHOD_FWUP_GPIO_CONTROL, &args, &out);
    }
#[no_mangle]
unsafe extern "C" fn awcc_read_total_gpios(wdev: *mut wmi_device, count: *mut u32) -> c_int {
    static int awcc_read_total_gpios(struct wmi_device *wdev, u32 *count)
    {
    let mut args: wmax_u32_args = {};
    return awcc_wmi_command(wdev, AWCC_METHOD_READ_TOTAL_GPIOS, &args, count);
    }
#[no_mangle]
unsafe extern "C" fn awcc_read_gpio_status(wdev: *mut wmi_device, pin: u8, status: *mut u32) -> c_int {
    static int awcc_read_gpio_status(struct wmi_device *wdev, u8 pin, u32 *status)
    {
    struct wmax_u32_args args = {
    .operation = pin,
    .arg1 = 0,
    .arg2 = 0,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_READ_GPIO_STATUS, &args, status);
    }
    static int awcc_game_shift_status(struct wmi_device *wdev, u8 operation,
    u32 *out)
    {
    struct wmax_u32_args args = {
    .operation = operation,
    .arg1 = 0,
    .arg2 = 0,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_GAME_SHIFT_STATUS, &args, out);
    }
//
// awcc_op_get_resource_id - Get the resource ID at a given index
// @wdev: AWCC WMI device
// @index: Index
// @out: Value returned by the WMI call
//
// Get the resource ID at a given @index. Resource IDs are listed in the
// following order:
//
// - Fan IDs
// - Sensor IDs
// - Unknown IDs
// - Thermal Profile IDs
//
// The total number of IDs of a given type can be obtained with
// AWCC_OP_GET_SYSTEM_DESCRIPTION.
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn awcc_op_get_resource_id(wdev: *mut wmi_device, index: u8, out: *mut u8) -> c_int {
    static int awcc_op_get_resource_id(struct wmi_device *wdev, u8 index, u8 *out)
    {
    struct wmax_u32_args args = {
    .operation = AWCC_OP_GET_RESOURCE_ID,
    .arg1 = index,
    .arg2 = 0,
    .arg3 = 0,
    };
    u32 out_data;
    int ret;
    ret = awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_INFORMATION, &args, &out_data);
    if (ret)
    return ret;
// out = FIELD_GET(AWCC_RESOURCE_ID_MASK, out_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn awcc_op_get_fan_rpm(wdev: *mut wmi_device, fan_id: u8, out: *mut u32) -> c_int {
    static int awcc_op_get_fan_rpm(struct wmi_device *wdev, u8 fan_id, u32 *out)
    {
    struct wmax_u32_args args = {
    .operation = AWCC_OP_GET_FAN_RPM,
    .arg1 = fan_id,
    .arg2 = 0,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_INFORMATION, &args, out);
    }
#[no_mangle]
unsafe extern "C" fn awcc_op_get_temperature(wdev: *mut wmi_device, temp_id: u8, out: *mut u32) -> c_int {
    static int awcc_op_get_temperature(struct wmi_device *wdev, u8 temp_id, u32 *out)
    {
    struct wmax_u32_args args = {
    .operation = AWCC_OP_GET_TEMPERATURE,
    .arg1 = temp_id,
    .arg2 = 0,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_INFORMATION, &args, out);
    }
#[no_mangle]
unsafe extern "C" fn awcc_op_get_fan_boost(wdev: *mut wmi_device, fan_id: u8, out: *mut u32) -> c_int {
    static int awcc_op_get_fan_boost(struct wmi_device *wdev, u8 fan_id, u32 *out)
    {
    struct wmax_u32_args args = {
    .operation = AWCC_OP_GET_FAN_BOOST,
    .arg1 = fan_id,
    .arg2 = 0,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_INFORMATION, &args, out);
    }
#[no_mangle]
unsafe extern "C" fn awcc_op_get_current_profile(wdev: *mut wmi_device, out: *mut u32) -> c_int {
    static int awcc_op_get_current_profile(struct wmi_device *wdev, u32 *out)
    {
    struct wmax_u32_args args = {
    .operation = AWCC_OP_GET_CURRENT_PROFILE,
    .arg1 = 0,
    .arg2 = 0,
    .arg3 = 0,
    };
    return awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_INFORMATION, &args, out);
    }
#[no_mangle]
unsafe extern "C" fn awcc_op_activate_profile(wdev: *mut wmi_device, profile: u8) -> c_int {
    static int awcc_op_activate_profile(struct wmi_device *wdev, u8 profile)
    {
    struct wmax_u32_args args = {
    .operation = AWCC_OP_ACTIVATE_PROFILE,
    .arg1 = profile,
    .arg2 = 0,
    .arg3 = 0,
    };
    u32 out;
    return awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_CONTROL, &args, &out);
    }
#[no_mangle]
unsafe extern "C" fn awcc_op_set_fan_boost(wdev: *mut wmi_device, fan_id: u8, boost: u8) -> c_int {
    static int awcc_op_set_fan_boost(struct wmi_device *wdev, u8 fan_id, u8 boost)
    {
    struct wmax_u32_args args = {
    .operation = AWCC_OP_SET_FAN_BOOST,
    .arg1 = fan_id,
    .arg2 = boost,
    .arg3 = 0,
    };
    u32 out;
    return awcc_wmi_command(wdev, AWCC_METHOD_THERMAL_CONTROL, &args, &out);
    }
//
// HWMON
// - Provides temperature and fan speed monitoring as well as manual fan
// control
//
    static umode_t awcc_hwmon_is_visible(const void *drvdata, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct awcc_priv *priv = drvdata;
    unsigned int temp_count;
    switch (type) {
    case hwmon_temp:
    temp_count = bitmap_weight(priv.temp_sensors, AWCC_ID_BITMAP_SIZE);
    return channel < temp_count ? 0444 : 0;
    case hwmon_fan:
    return channel < priv.fan_count ? 0444 : 0;
    case hwmon_pwm:
    return channel < priv.fan_count ? 0444 : 0;
    default:
    return 0;
    }
    }
    static int awcc_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    const struct awcc_fan_data *fan;
    u32 state;
    int ret;
    u8 temp;
    switch (type) {
    case hwmon_temp:
    temp = find_nth_bit(priv.temp_sensors, AWCC_ID_BITMAP_SIZE, channel);
    switch (attr) {
    case hwmon_temp_input:
    ret = awcc_op_get_temperature(priv.wdev, temp, &state);
    if (ret)
    return ret;
// val = state * MILLIDEGREE_PER_DEGREE;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case hwmon_fan:
    fan = priv.fan_data[channel];
    switch (attr) {
    case hwmon_fan_input:
    ret = awcc_op_get_fan_rpm(priv.wdev, fan.id, &state);
    if (ret)
    return ret;
// val = state;
    break;
    case hwmon_fan_min:
// val = fan->min_rpm;
    break;
    case hwmon_fan_max:
// val = fan->max_rpm;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case hwmon_pwm:
    fan = priv.fan_data[channel];
    switch (attr) {
    case hwmon_pwm_auto_channels_temp:
// val = fan->auto_channels_temp;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int awcc_hwmon_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    u8 temp;
    switch (type) {
    case hwmon_temp:
    temp = find_nth_bit(priv.temp_sensors, AWCC_ID_BITMAP_SIZE, channel);
    switch (temp) {
    case AWCC_TEMP_SENSOR_CPU:
// str = "CPU";
    break;
    case AWCC_TEMP_SENSOR_FRONT:
// str = "Front";
    break;
    case AWCC_TEMP_SENSOR_GPU:
// str = "GPU";
    break;
    default:
// str = "Unknown";
    break;
    }
    break;
    case hwmon_fan:
    switch (priv.fan_data[channel].id) {
    case AWCC_FAN_CPU_1:
    case AWCC_FAN_CPU_2:
// str = "CPU Fan";
    break;
    case AWCC_FAN_GPU_1:
    case AWCC_FAN_GPU_2:
// str = "GPU Fan";
    break;
    case AWCC_FAN_PCI:
// str = "PCI Fan";
    break;
    case AWCC_FAN_MID:
// str = "Mid Fan";
    break;
    case AWCC_FAN_TOP_1:
    case AWCC_FAN_TOP_2:
    case AWCC_FAN_TOP_3:
// str = "Top Fan";
    break;
    case AWCC_FAN_SIDE:
// str = "Side Fan";
    break;
    case AWCC_FAN_U2_1:
    case AWCC_FAN_U2_2:
// str = "U.2 Fan";
    break;
    case AWCC_FAN_FRONT_1:
    case AWCC_FAN_FRONT_2:
// str = "Front Fan";
    break;
    case AWCC_FAN_BOTTOM_1:
    case AWCC_FAN_BOTTOM_2:
// str = "Bottom Fan";
    break;
    default:
// str = "Unknown Fan";
    break;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static const struct hwmon_ops awcc_hwmon_ops = {
    .is_visible = awcc_hwmon_is_visible,
    .read = awcc_hwmon_read,
    .read_string = awcc_hwmon_read_string,
    };
    static const struct hwmon_channel_info * const awcc_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_LABEL | HWMON_T_INPUT,
    HWMON_T_LABEL | HWMON_T_INPUT,
    HWMON_T_LABEL | HWMON_T_INPUT,
    HWMON_T_LABEL | HWMON_T_INPUT,
    HWMON_T_LABEL | HWMON_T_INPUT,
    HWMON_T_LABEL | HWMON_T_INPUT
    ),
    HWMON_CHANNEL_INFO(fan,
    HWMON_F_LABEL | HWMON_F_INPUT | HWMON_F_MIN | HWMON_F_MAX,
    HWMON_F_LABEL | HWMON_F_INPUT | HWMON_F_MIN | HWMON_F_MAX,
    HWMON_F_LABEL | HWMON_F_INPUT | HWMON_F_MIN | HWMON_F_MAX,
    HWMON_F_LABEL | HWMON_F_INPUT | HWMON_F_MIN | HWMON_F_MAX,
    HWMON_F_LABEL | HWMON_F_INPUT | HWMON_F_MIN | HWMON_F_MAX,
    HWMON_F_LABEL | HWMON_F_INPUT | HWMON_F_MIN | HWMON_F_MAX
    ),
    HWMON_CHANNEL_INFO(pwm,
    HWMON_PWM_AUTO_CHANNELS_TEMP,
    HWMON_PWM_AUTO_CHANNELS_TEMP,
    HWMON_PWM_AUTO_CHANNELS_TEMP,
    HWMON_PWM_AUTO_CHANNELS_TEMP,
    HWMON_PWM_AUTO_CHANNELS_TEMP,
    HWMON_PWM_AUTO_CHANNELS_TEMP
    ),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info awcc_hwmon_chip_info = {
    .ops = &awcc_hwmon_ops,
    .info = awcc_hwmon_info,
    };
    static ssize_t fan_boost_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    let mut index: c_int = to_sensor_dev_attr(attr).index;
    struct awcc_fan_data *fan = priv.fan_data[index];
    u32 boost;
    int ret;
    ret = awcc_op_get_fan_boost(priv.wdev, fan.id, &boost);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%u\n", boost);
    }
    static ssize_t fan_boost_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    let mut index: c_int = to_sensor_dev_attr(attr).index;
    struct awcc_fan_data *fan = priv.fan_data[index];
    unsigned long val;
    int ret;
    ret = kstrtoul(buf, 0, &val);
    if (ret)
    return ret;
    ret = awcc_op_set_fan_boost(priv.wdev, fan.id, clamp_val(val, 0, 255));
    return ret ? ret : count;
    }
    static SENSOR_DEVICE_ATTR_RW(fan1_boost, fan_boost, 0);
    static SENSOR_DEVICE_ATTR_RW(fan2_boost, fan_boost, 1);
    static SENSOR_DEVICE_ATTR_RW(fan3_boost, fan_boost, 2);
    static SENSOR_DEVICE_ATTR_RW(fan4_boost, fan_boost, 3);
    static SENSOR_DEVICE_ATTR_RW(fan5_boost, fan_boost, 4);
    static SENSOR_DEVICE_ATTR_RW(fan6_boost, fan_boost, 5);
#[no_mangle]
unsafe extern "C" fn fan_boost_attr_visible(kobj: *mut kobject, attr: *mut attribute, n: c_int) -> umode_t {
    static umode_t fan_boost_attr_visible(struct kobject *kobj, struct attribute *attr, int n)
    {
    struct awcc_priv *priv = dev_get_drvdata(kobj_to_dev(kobj));
    return n < priv.fan_count ? attr.mode : 0;
    }
#[no_mangle]
unsafe extern "C" fn fan_boost_group_visible(kobj: *mut kobject) -> bool {
    static bool fan_boost_group_visible(struct kobject *kobj)
    {
    return true;
    }
    DEFINE_SYSFS_GROUP_VISIBLE(fan_boost);
    static struct attribute *fan_boost_attrs[] = {
    &sensor_dev_attr_fan1_boost.dev_attr.attr,
    &sensor_dev_attr_fan2_boost.dev_attr.attr,
    &sensor_dev_attr_fan3_boost.dev_attr.attr,
    &sensor_dev_attr_fan4_boost.dev_attr.attr,
    &sensor_dev_attr_fan5_boost.dev_attr.attr,
    &sensor_dev_attr_fan6_boost.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group fan_boost_group = {
    .attrs = fan_boost_attrs,
    .is_visible = SYSFS_GROUP_VISIBLE(fan_boost),
    };
    static const struct attribute_group *awcc_hwmon_groups[] = {
    &fan_boost_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn awcc_hwmon_temps_init(wdev: *mut wmi_device) -> c_int {
    static int awcc_hwmon_temps_init(struct wmi_device *wdev)
    {
    struct awcc_priv *priv = dev_get_drvdata(&wdev.dev);
    unsigned int i;
    int ret;
    u8 id;
    for (i = 0; i < priv.temp_count; i++) {
//
// Temperature sensors IDs are listed after the fan IDs at
// offset `fan_count`
//
    ret = awcc_op_get_resource_id(wdev, i + priv.fan_count, &id);
    if (ret)
    return ret;
    __set_bit(id, priv.temp_sensors);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn awcc_hwmon_fans_init(wdev: *mut wmi_device) -> c_int {
    static int awcc_hwmon_fans_init(struct wmi_device *wdev)
    {
    struct awcc_priv *priv = dev_get_drvdata(&wdev.dev);
    unsigned long fan_temps[AWCC_ID_BITMAP_LONGS];
    unsigned long gather[AWCC_ID_BITMAP_LONGS];
    u32 min_rpm, max_rpm, temp_count, temp_id;
    struct awcc_fan_data *fan_data;
    unsigned int i, j;
    int ret;
    u8 id;
    for (i = 0; i < priv.fan_count; i++) {
    fan_data = devm_kzalloc(&wdev.dev, sizeof(*fan_data), GFP_KERNEL);
    if (!fan_data)
    return -ENOMEM;
//
// Fan IDs are listed first at offset 0
//
    ret = awcc_op_get_resource_id(wdev, i, &id);
    if (ret)
    return ret;
    ret = awcc_thermal_information(wdev, AWCC_OP_GET_FAN_MIN_RPM, id,
    &min_rpm);
    if (ret)
    return ret;
    ret = awcc_thermal_information(wdev, AWCC_OP_GET_FAN_MAX_RPM, id,
    &max_rpm);
    if (ret)
    return ret;
    ret = awcc_get_fan_sensors(wdev, AWCC_OP_GET_TOTAL_FAN_TEMPS, id,
    0, &temp_count);
    if (ret)
    return ret;
    bitmap_zero(fan_temps, AWCC_ID_BITMAP_SIZE);
    for (j = 0; j < temp_count; j++) {
    ret = awcc_get_fan_sensors(wdev, AWCC_OP_GET_FAN_TEMP_ID,
    id, j, &temp_id);
    if (ret)
    break;
    temp_id = FIELD_GET(AWCC_RESOURCE_ID_MASK, temp_id);
    __set_bit(temp_id, fan_temps);
    }
    fan_data.id = id;
    fan_data.min_rpm = min_rpm;
    fan_data.max_rpm = max_rpm;
    bitmap_gather(gather, fan_temps, priv.temp_sensors, AWCC_ID_BITMAP_SIZE);
    bitmap_copy(&fan_data.auto_channels_temp, gather, BITS_PER_LONG);
    priv.fan_data[i] = fan_data;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn awcc_hwmon_init(wdev: *mut wmi_device) -> c_int {
    static int awcc_hwmon_init(struct wmi_device *wdev)
    {
    struct awcc_priv *priv = dev_get_drvdata(&wdev.dev);
    int ret;
    priv.fan_data = devm_kcalloc(&wdev.dev, priv.fan_count,
    sizeof(*priv.fan_data), GFP_KERNEL);
    if (!priv.fan_data)
    return -ENOMEM;
    ret = awcc_hwmon_temps_init(wdev);
    if (ret)
    return ret;
    ret = awcc_hwmon_fans_init(wdev);
    if (ret)
    return ret;
    priv.hwdev = devm_hwmon_device_register_with_info(&wdev.dev, "alienware_wmi",
    priv, &awcc_hwmon_chip_info,
    awcc_hwmon_groups);
    return PTR_ERR_OR_ZERO(priv.hwdev);
    }
#[no_mangle]
unsafe extern "C" fn awcc_hwmon_suspend(dev: *mut device) {
    static void awcc_hwmon_suspend(struct device *dev)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    struct awcc_fan_data *fan;
    unsigned int i;
    u32 boost;
    int ret;
    for (i = 0; i < priv.fan_count; i++) {
    fan = priv.fan_data[i];
    ret = awcc_thermal_information(priv.wdev, AWCC_OP_GET_FAN_BOOST,
    fan.id, &boost);
    if (ret)
    dev_err(dev, "Failed to store Fan %u boost while suspending\n", i);
    fan.suspend_cache = ret ? 0 : clamp_val(boost, 0, 255);
    awcc_op_set_fan_boost(priv.wdev, fan.id, 0);
    if (ret)
    dev_err(dev, "Failed to set Fan %u boost to 0 while suspending\n", i);
    }
    }
#[no_mangle]
unsafe extern "C" fn awcc_hwmon_resume(dev: *mut device) {
    static void awcc_hwmon_resume(struct device *dev)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    struct awcc_fan_data *fan;
    unsigned int i;
    int ret;
    for (i = 0; i < priv.fan_count; i++) {
    fan = priv.fan_data[i];
    if (!fan.suspend_cache)
    continue;
    ret = awcc_op_set_fan_boost(priv.wdev, fan.id, fan.suspend_cache);
    if (ret)
    dev_err(dev, "Failed to restore Fan %u boost while resuming\n", i);
    }
    }
//
// Thermal Profile control
// - Provides thermal profile control through the Platform Profile API
//
    static int awcc_platform_profile_get(struct device *dev,
    enum platform_profile_option *profile)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    u32 out_data;
    int ret;
    ret = awcc_op_get_current_profile(priv.wdev, &out_data);
    if (ret)
    return ret;
    return awcc_profile_to_pprof(out_data, profile);
    }
    static int awcc_platform_profile_set(struct device *dev,
    enum platform_profile_option profile)
    {
    struct awcc_priv *priv = dev_get_drvdata(dev);
    if (awcc.gmode) {
    u32 gmode_status;
    int ret;
    ret = awcc_game_shift_status(priv.wdev,
    AWCC_OP_GET_GAME_SHIFT_STATUS,
    &gmode_status);
    if (ret < 0)
    return ret;
    if ((profile == PLATFORM_PROFILE_PERFORMANCE && !gmode_status) ||
    (profile != PLATFORM_PROFILE_PERFORMANCE && gmode_status)) {
    ret = awcc_game_shift_status(priv.wdev,
    AWCC_OP_TOGGLE_GAME_SHIFT,
    &gmode_status);
    if (ret < 0)
    return ret;
    }
    }
    return awcc_op_activate_profile(priv.wdev, priv.supported_profiles[profile]);
    }
#[no_mangle]
unsafe extern "C" fn awcc_platform_profile_probe(drvdata: *mut c_void, choices: *mut c_ulong) -> c_int {
    static int awcc_platform_profile_probe(void *drvdata, unsigned long *choices)
    {
    enum platform_profile_option profile;
    struct awcc_priv *priv = drvdata;
    u8 id, offset = 0;
    int ret;
//
// Thermal profile IDs are listed last at offset
// fan_count + temp_count + unknown_count
//
    for (unsigned int i = 0; i < ARRAY_SIZE(priv.res_count) - 1; i++)
    offset += priv.res_count[i];
    for (unsigned int i = 0; i < priv.profile_count; i++) {
    ret = awcc_op_get_resource_id(priv.wdev, i + offset, &id);
//
// Some devices report an incorrect number of thermal profiles
// so the resource ID list may end prematurely
//
    if (ret == -EBADRQC)
    break;
    if (ret)
    return ret;
//
// G-Mode profile ID is not listed consistently across modeles
// that support it, therefore we handle it through quirks.
//
    if (id == AWCC_PROFILE_SPECIAL_GMODE)
    continue;
    ret = awcc_profile_to_pprof(id, &profile);
    if (ret) {
    dev_dbg(&priv.wdev.dev, "Unmapped thermal profile ID 0x%02x\n", id);
    continue;
    }
    priv.supported_profiles[profile] = id;
    __set_bit(profile, choices);
    }
    if (bitmap_empty(choices, PLATFORM_PROFILE_LAST))
    return -ENODEV;
    if (awcc.gmode) {
    priv.supported_profiles[PLATFORM_PROFILE_PERFORMANCE] =
    AWCC_PROFILE_SPECIAL_GMODE;
    __set_bit(PLATFORM_PROFILE_PERFORMANCE, choices);
    }
// Every model supports the "custom" profile
    priv.supported_profiles[PLATFORM_PROFILE_CUSTOM] =
    AWCC_PROFILE_SPECIAL_CUSTOM;
    __set_bit(PLATFORM_PROFILE_CUSTOM, choices);
    return 0;
    }
    static const struct platform_profile_ops awcc_platform_profile_ops = {
    .probe = awcc_platform_profile_probe,
    .profile_get = awcc_platform_profile_get,
    .profile_set = awcc_platform_profile_set,
    };
#[no_mangle]
unsafe extern "C" fn awcc_platform_profile_init(wdev: *mut wmi_device) -> c_int {
    static int awcc_platform_profile_init(struct wmi_device *wdev)
    {
    struct awcc_priv *priv = dev_get_drvdata(&wdev.dev);
    priv.ppdev = devm_platform_profile_register(&wdev.dev, "alienware-wmi",
    priv, &awcc_platform_profile_ops);
    return PTR_ERR_OR_ZERO(priv.ppdev);
    }
//
// DebugFS
//
#[no_mangle]
unsafe extern "C" fn awcc_debugfs_system_description_read(seq: *mut seq_file, data: *mut c_void) -> c_int {
    static int awcc_debugfs_system_description_read(struct seq_file *seq, void *data)
    {
    struct device *dev = seq.private;
    struct awcc_priv *priv = dev_get_drvdata(dev);
    seq_printf(seq, "0x%08x\n", priv.system_description);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn awcc_debugfs_hwmon_data_read(seq: *mut seq_file, data: *mut c_void) -> c_int {
    static int awcc_debugfs_hwmon_data_read(struct seq_file *seq, void *data)
    {
    struct device *dev = seq.private;
    struct awcc_priv *priv = dev_get_drvdata(dev);
    const struct awcc_fan_data *fan;
    unsigned int bit;
    seq_printf(seq, "Number of fans: %u\n", priv.fan_count);
    seq_printf(seq, "Number of temperature sensors: %u\n\n", priv.temp_count);
    for (u32 i = 0; i < priv.fan_count; i++) {
    fan = priv.fan_data[i];
    seq_printf(seq, "Fan %u:\n", i);
    seq_printf(seq, "  ID: 0x%02x\n", fan.id);
    seq_printf(seq, "  Related temperature sensors bitmap: %lu\n",
    fan.auto_channels_temp);
    }
    seq_puts(seq, "\nTemperature sensor IDs:\n");
    for_each_set_bit(bit, priv.temp_sensors, AWCC_ID_BITMAP_SIZE)
    seq_printf(seq, "  0x%02x\n", bit);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn awcc_debugfs_pprof_data_read(seq: *mut seq_file, data: *mut c_void) -> c_int {
    static int awcc_debugfs_pprof_data_read(struct seq_file *seq, void *data)
    {
    struct device *dev = seq.private;
    struct awcc_priv *priv = dev_get_drvdata(dev);
    seq_printf(seq, "Number of thermal profiles: %u\n\n", priv.profile_count);
    for (u32 i = 0; i < PLATFORM_PROFILE_LAST; i++) {
    if (!priv.supported_profiles[i])
    continue;
    seq_printf(seq, "Platform profile %u:\n", i);
    seq_printf(seq, "  ID: 0x%02x\n", priv.supported_profiles[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn awcc_gpio_pin_show(seq: *mut seq_file, data: *mut c_void) -> c_int {
    static int awcc_gpio_pin_show(struct seq_file *seq, void *data)
    {
    let mut pin: c_ulong = debugfs_get_aux_num(seq.file);
    struct wmi_device *wdev = seq.private;
    u32 status;
    int ret;
    ret = awcc_read_gpio_status(wdev, pin, &status);
    if (ret)
    return ret;
    seq_printf(seq, "%u\n", status);
    return 0;
    }
    static ssize_t awcc_gpio_pin_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    let mut pin: c_ulong = debugfs_get_aux_num(file);
    struct seq_file *seq = file.private_data;
    struct wmi_device *wdev = seq.private;
    bool status;
    int ret;
    if (!ppos || *ppos)
    return -EINVAL;
    ret = kstrtobool_from_user(buf, count, &status);
    if (ret)
    return ret;
    ret = awcc_fwup_gpio_control(wdev, pin, status);
    if (ret)
    return ret;
    return count;
    }
    DEFINE_SHOW_STORE_ATTRIBUTE(awcc_gpio_pin);
#[no_mangle]
unsafe extern "C" fn awcc_debugfs_remove(data: *mut c_void) {
    static void awcc_debugfs_remove(void *data)
    {
    struct dentry *root = data;
    debugfs_remove(root);
    }
#[no_mangle]
unsafe extern "C" fn awcc_debugfs_init(wdev: *mut wmi_device) {
    static void awcc_debugfs_init(struct wmi_device *wdev)
    {
    struct awcc_priv *priv = dev_get_drvdata(&wdev.dev);
    struct dentry *root, *gpio_ctl;
    u32 gpio_count;
    char name[64];
    int ret;
    scnprintf(name, sizeof(name), "%s-%s", "alienware-wmi", dev_name(&wdev.dev));
    root = debugfs_create_dir(name, core::ptr::null_mut());
    debugfs_create_devm_seqfile(&wdev.dev, "system_description", root,
    awcc_debugfs_system_description_read);
    if (awcc.hwmon)
    debugfs_create_devm_seqfile(&wdev.dev, "hwmon_data", root,
    awcc_debugfs_hwmon_data_read);
    if (awcc.pprof)
    debugfs_create_devm_seqfile(&wdev.dev, "pprof_data", root,
    awcc_debugfs_pprof_data_read);
    ret = awcc_read_total_gpios(wdev, &gpio_count);
    if (ret) {
    dev_dbg(&wdev.dev, "Failed to get total GPIO Pin count\n");
    goto out_add_action;
    } else if (gpio_count > AWCC_MAX_RES_COUNT) {
    dev_dbg(&wdev.dev, "Reported GPIO Pin count may be incorrect: %u\n", gpio_count);
    goto out_add_action;
    }
    gpio_ctl = debugfs_create_dir("gpio_ctl", root);
    priv.gpio_count = gpio_count;
    debugfs_create_u32("total_gpios", 0444, gpio_ctl, &priv.gpio_count);
    for (unsigned int i = 0; i < gpio_count; i++) {
    scnprintf(name, sizeof(name), "pin%u", i);
    debugfs_create_file_aux_num(name, 0644, gpio_ctl, wdev, i,
    &awcc_gpio_pin_fops);
    }
    out_add_action:
    devm_add_action_or_reset(&wdev.dev, awcc_debugfs_remove, root);
    }
#[no_mangle]
unsafe extern "C" fn alienware_awcc_setup(wdev: *mut wmi_device) -> c_int {
    static int alienware_awcc_setup(struct wmi_device *wdev)
    {
    struct awcc_priv *priv;
    int ret;
    priv = devm_kzalloc(&wdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ret = awcc_thermal_information(wdev, AWCC_OP_GET_SYSTEM_DESCRIPTION,
    0, &priv.system_description);
    if (ret < 0)
    return ret;
// Sanity check
    for (unsigned int i = 0; i < ARRAY_SIZE(priv.res_count); i++) {
    if (priv.res_count[i] > AWCC_MAX_RES_COUNT) {
    dev_err(&wdev.dev, "Malformed system description: 0x%08x\n",
    priv.system_description);
    return -ENXIO;
    }
    }
    priv.wdev = wdev;
    dev_set_drvdata(&wdev.dev, priv);
    if (awcc.hwmon) {
    ret = awcc_hwmon_init(wdev);
    if (ret)
    return ret;
    }
    if (awcc.pprof) {
    ret = awcc_platform_profile_init(wdev);
    if (ret)
    return ret;
    }
    awcc_debugfs_init(wdev);
    return 0;
    }
//
// WMAX WMI driver
//
    static int wmax_wmi_update_led(struct alienfx_priv *priv,
    struct wmi_device *wdev, u8 location)
    {
    struct wmax_led_args in_args = {
    .led_mask = 1 << location,
    .colors = priv.colors[location],
    .state = priv.lighting_control_state,
    };
    return alienware_wmi_command(wdev, WMAX_METHOD_ZONE_CONTROL, &in_args,
    sizeof(in_args), core::ptr::null_mut());
    }
    static int wmax_wmi_update_brightness(struct alienfx_priv *priv,
    struct wmi_device *wdev, u8 brightness)
    {
    struct wmax_brightness_args in_args = {
    .led_mask = 0xFF,
    .percentage = brightness,
    };
    return alienware_wmi_command(wdev, WMAX_METHOD_BRIGHTNESS, &in_args,
    sizeof(in_args), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn wmax_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int wmax_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct alienfx_platdata pdata = {
    .wdev = wdev,
    .ops = {
    .upd_led = wmax_wmi_update_led,
    .upd_brightness = wmax_wmi_update_brightness,
    },
    };
    int ret;
    if (awcc)
    ret = alienware_awcc_setup(wdev);
    else
    ret = alienware_alienfx_setup(&pdata);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wmax_wmi_suspend(dev: *mut device) -> c_int {
    static int wmax_wmi_suspend(struct device *dev)
    {
    if (awcc && awcc.hwmon)
    awcc_hwmon_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wmax_wmi_resume(dev: *mut device) -> c_int {
    static int wmax_wmi_resume(struct device *dev)
    {
    if (awcc && awcc.hwmon)
    awcc_hwmon_resume(dev);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(wmax_wmi_pm_ops, wmax_wmi_suspend, wmax_wmi_resume);
    static const struct wmi_device_id alienware_wmax_device_id_table[] = {
    { WMAX_CONTROL_GUID, core::ptr::null_mut() },
    { },
    };
    MODULE_DEVICE_TABLE(wmi, alienware_wmax_device_id_table);
    static struct wmi_driver alienware_wmax_wmi_driver = {
    .driver = {
    .name = "alienware-wmi-wmax",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm = pm_sleep_ptr(&wmax_wmi_pm_ops),
    },
    .id_table = alienware_wmax_device_id_table,
    .probe = wmax_wmi_probe,
    .no_singleton = true,
    };
#[no_mangle]
pub unsafe extern "C" fn alienware_wmax_wmi_init() -> int __init {
    int __init alienware_wmax_wmi_init(void)
    {
    const struct dmi_system_id *id;
    id = dmi_first_match(awcc_dmi_table);
    if (id)
    awcc = id.driver_data;
    if (force_hwmon) {
    if (!awcc)
    awcc = &empty_quirks;
    awcc.hwmon = true;
    }
    if (force_platform_profile) {
    if (!awcc)
    awcc = &empty_quirks;
    awcc.pprof = true;
    }
    if (force_gmode) {
    if (awcc)
    awcc.gmode = true;
    else
    pr_warn("force_gmode requires platform profile support\n");
    }
    return wmi_driver_register(&alienware_wmax_wmi_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn alienware_wmax_wmi_exit() -> void __exit {
    void __exit alienware_wmax_wmi_exit(void)
    {
    wmi_driver_unregister(&alienware_wmax_wmi_driver);
    }
