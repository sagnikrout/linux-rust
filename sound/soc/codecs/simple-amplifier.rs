//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/simple-amplifier.c
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
// Support for gpio amplifier
// Copyright 2026 CS GROUP France
// Author: Herve Codina <herve.codina@bootlin.com>
//
// Basic simple amplifier driver
// Copyright (c) 2017 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_single {
    pub gpio: *mut gpio_desc,
    pub is_inverted: bool,
    pub kctrl_val: c_int,
    pub control_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_point {
    pub gpio_val: u32,
    pub gain_db: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_range {
    pub nb_points: c_uint,
    pub min: simple_amp_point,
    pub max: simple_amp_point,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_ranges {
    pub nb_ranges: c_uint,
    pub tab_ranges: *mut simple_amp_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_labels {
    pub nb_labels: c_uint,
    pub tab_labels: *const c_char,
}

    enum simple_amp_mode {
    SIMPLE_AMP_MODE_NONE,
    SIMPLE_AMP_MODE_RANGES,
    SIMPLE_AMP_MODE_LABELS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_multi {
    pub gpios: *mut gpio_descs,
    pub kctrl_val: u32,
    pub kctrl_max: u32,
    pub control_name: *const c_char,
    pub tlv_array: *mut c_uint,
    pub mode: enum simple_amp_mode,
    union {
    pub ranges: simple_amp_ranges,
    pub labels: simple_amp_labels,
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_data {
    pub supports: c_uint,

    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp {
    pub data: *const simple_amp_data,
    pub gpiod_enable: *mut gpio_desc,
    pub mute: simple_amp_single,
    pub bypass: simple_amp_single,
    pub gain: simple_amp_multi,
}

    static int simple_amp_power_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *control, int event)
    {
    struct snd_soc_component *c = snd_soc_dapm_to_component(w.dapm);
    struct simple_amp *simple_amp = snd_soc_component_get_drvdata(c);
    int val;
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    val = 1;
    break;
    case SND_SOC_DAPM_PRE_PMD:
    val = 0;
    break;
    default:
    WARN(1, "Unexpected event");
    return -EINVAL;
    }
    gpiod_set_value_cansleep(simple_amp.gpiod_enable, val);
    return 0;
    }
    static const struct snd_soc_dapm_widget simple_amp_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("INL"),
    SND_SOC_DAPM_INPUT("INR"),
    SND_SOC_DAPM_OUT_DRV_E("DRV", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0, simple_amp_power_event,
    (SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD)),
    SND_SOC_DAPM_OUTPUT("OUTL"),
    SND_SOC_DAPM_OUTPUT("OUTR"),
    SND_SOC_DAPM_REGULATOR_SUPPLY("VCC", 20, 0),
    };
    static const struct snd_soc_dapm_route simple_amp_dapm_routes[] = {
    { "DRV", core::ptr::null_mut(), "INL" },
    { "DRV", core::ptr::null_mut(), "INR" },
    { "OUTL", core::ptr::null_mut(), "VCC" },
    { "OUTR", core::ptr::null_mut(), "VCC" },
    { "OUTL", core::ptr::null_mut(), "DRV" },
    { "OUTR", core::ptr::null_mut(), "DRV" },
    };
    static const struct snd_soc_dapm_widget simple_amp_mono_pga_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("IN"),
    SND_SOC_DAPM_OUTPUT("OUT"),
    SND_SOC_DAPM_PGA_E("PGA", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0, simple_amp_power_event,
    (SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD)),
    SND_SOC_DAPM_REGULATOR_SUPPLY("vdd", 0, 0),
    };
    static const struct snd_soc_dapm_route simple_amp_mono_pga_dapm_routes[] = {
    { "PGA", core::ptr::null_mut(), "IN" },
    { "PGA", core::ptr::null_mut(), "vdd" },
    { "OUT", core::ptr::null_mut(), "PGA" },
    };
    static const struct snd_soc_dapm_widget simple_amp_stereo_pga_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("INL"),
    SND_SOC_DAPM_INPUT("INR"),
    SND_SOC_DAPM_OUTPUT("OUTL"),
    SND_SOC_DAPM_OUTPUT("OUTR"),
    SND_SOC_DAPM_PGA_E("PGA", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0, simple_amp_power_event,
    (SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD)),
    SND_SOC_DAPM_REGULATOR_SUPPLY("vdd", 0, 0),
    };
    static const struct snd_soc_dapm_route simple_amp_stereo_pga_dapm_routes[] = {
    { "PGA", core::ptr::null_mut(), "INL" },
    { "PGA", core::ptr::null_mut(), "INR" },
    { "PGA", core::ptr::null_mut(), "vdd" },
    { "OUTL", core::ptr::null_mut(), "PGA" },
    { "OUTR", core::ptr::null_mut(), "PGA" },
    };
    static int simple_amp_single_kctrl_write_gpio(struct simple_amp_single *single,
    int kctrl_val)
    {
    int gpio_val;
    gpio_val = single.is_inverted ? !kctrl_val : kctrl_val;
    return gpiod_set_value_cansleep(single.gpio, gpio_val);
    }
    static int simple_amp_single_kctrl_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 1;
    uinfo.type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    return 0;
    }
    static int simple_amp_single_kctrl_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct simple_amp_single *single = (struct simple_amp_single *)kcontrol.private_value;
    ucontrol.value.integer.value[0] = single.kctrl_val;
    return 0;
    }
    static int simple_amp_single_kctrl_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct simple_amp_single *single = (struct simple_amp_single *)kcontrol.private_value;
    int kctrl_val;
    int err;
    kctrl_val = ucontrol.value.integer.value[0] ? 1 : 0;
    if (kctrl_val == single.kctrl_val)
    return 0;
    err = simple_amp_single_kctrl_write_gpio(single, kctrl_val);
    if (err)
    return err;
    single.kctrl_val = kctrl_val;
    return 1; /* The value changed */
    }
    static int simple_amp_single_add_kcontrol(struct snd_soc_component *component,
    struct simple_amp_single *single)
    {
    struct snd_kcontrol_new control = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = single.control_name,
    .info = simple_amp_single_kctrl_info,
    .get = simple_amp_single_kctrl_get,
    .put = simple_amp_single_kctrl_put,
    .private_value = (unsigned long)single,
    };
    int ret;
// Be consistent between single->kctrl_val value and the GPIO value
    ret = simple_amp_single_kctrl_write_gpio(single, single.kctrl_val);
    if (ret)
    return ret;
    return snd_soc_add_component_controls(component, &control, 1);
    }
    static u32 simple_amp_multi_ranges_kctrl_to_gpio(u32 kctrl_val,
    struct simple_amp_ranges *ranges)
    {
    struct simple_amp_range *range;
    let mut index: u32 = kctrl_val;
    unsigned int i;
    for (i = 0; i < ranges.nb_ranges; i++) {
    range = &ranges.tab_ranges[i];
    if (index < range.nb_points)
    return (range.max.gpio_val >= range.min.gpio_val) ?
    range.min.gpio_val + index :
    range.min.gpio_val - index;
    index -= range.nb_points;
    }
//
// Given index out of possible ranges. This is shouldn't happen.
// Signal the issue and return the maximum value
//
    WARN(1, "kctrl_val %u out of ranges\n", kctrl_val);
    return ranges.tab_ranges[ranges.nb_ranges - 1].max.gpio_val;
    }
    static int simple_amp_multi_kctrl_write_gpios(struct simple_amp_multi *multi,
    u32 kctrl_val)
    {
    DECLARE_BITMAP(bm, 32);
    u32 gpio_val;
    if (kctrl_val > multi.kctrl_max)
    return -EINVAL;
    if (multi.mode == SIMPLE_AMP_MODE_RANGES)
    gpio_val = simple_amp_multi_ranges_kctrl_to_gpio(kctrl_val,
    &multi.ranges);
    else
    gpio_val = kctrl_val;
    bitmap_from_arr32(bm, &gpio_val, multi.gpios.ndescs);
    return gpiod_multi_set_value_cansleep(multi.gpios, bm);
    }
    static int simple_amp_multi_kctrl_int_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct simple_amp_multi *multi = (struct simple_amp_multi *)kcontrol.private_value;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = multi.kctrl_max;
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    return 0;
    }
    static int simple_amp_multi_kctrl_int_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct simple_amp_multi *multi = (struct simple_amp_multi *)kcontrol.private_value;
    ucontrol.value.integer.value[0] = multi.kctrl_val;
    return 0;
    }
    static int simple_amp_multi_kctrl_int_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct simple_amp_multi *multi = (struct simple_amp_multi *)kcontrol.private_value;
    u32 kctrl_val;
    int ret;
    kctrl_val = ucontrol.value.integer.value[0];
    if (kctrl_val == multi.kctrl_val)
    return 0;
    ret = simple_amp_multi_kctrl_write_gpios(multi, kctrl_val);
    if (ret)
    return ret;
    multi.kctrl_val = kctrl_val;
    return 1; /* The value changed */
    }
    static int simple_amp_multi_kctrl_enum_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct simple_amp_multi *multi = (struct simple_amp_multi *)kcontrol.private_value;
    return snd_ctl_enum_info(uinfo, 1, multi.labels.nb_labels,
    multi.labels.tab_labels);
    }
    static int simple_amp_multi_kctrl_enum_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct simple_amp_multi *multi = (struct simple_amp_multi *)kcontrol.private_value;
    ucontrol.value.enumerated.item[0] = multi.kctrl_val;
    return 0;
    }
    static int simple_amp_multi_kctrl_enum_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct simple_amp_multi *multi = (struct simple_amp_multi *)kcontrol.private_value;
    u32 kctrl_val;
    int ret;
    kctrl_val = ucontrol.value.enumerated.item[0];
    if (kctrl_val == multi.kctrl_val)
    return 0;
    ret = simple_amp_multi_kctrl_write_gpios(multi, kctrl_val);
    if (ret)
    return ret;
    multi.kctrl_val = kctrl_val;
    return 1; /* The value changed */
    }
    static unsigned int *simple_amp_alloc_tlv_ranges(const struct simple_amp_ranges *ranges)
    {
    unsigned int index;
    unsigned int *tlv;
    unsigned int *t;
    unsigned int i;
    tlv = kzalloc_objs(*tlv, 2 + ranges.nb_ranges * 6);
    if (!tlv)
    return core::ptr::null_mut();
    t = tlv;
// Fill first TLV
// t++ = SNDRV_CTL_TLVT_DB_RANGE; /* Tag
// t++ = ranges->nb_ranges * 6 * sizeof(*tlv); /* Len
// Ranges are sorted from lower to higher value
    index = 0;
    for (i = 0; i < ranges.nb_ranges; i++) {
// Fill range item i
// t++ = index;  /* min
    index += ranges.tab_ranges[i].nb_points;
// t++ = index - 1;  /* max
// t++ = SNDRV_CTL_TLVT_DB_MINMAX; /* Tag
// t++ = 2 * sizeof(*tlv); /* Len
// t++ = ranges->tab_ranges[i].min.gain_db; /* min_dB
// t++ = ranges->tab_ranges[i].max.gain_db; /* max_dB
    }
    return tlv;
    }
    static int simple_amp_multi_add_kcontrol(struct snd_soc_component *component,
    struct simple_amp_multi *multi)
    {
    struct snd_kcontrol_new control = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = multi.control_name,
    .info = simple_amp_multi_kctrl_int_info,
    .get = simple_amp_multi_kctrl_int_get,
    .put = simple_amp_multi_kctrl_int_put,
    .private_value = (unsigned long)multi,
    };
    int ret;
    switch (multi.mode) {
    case SIMPLE_AMP_MODE_RANGES:
    multi.tlv_array = simple_amp_alloc_tlv_ranges(&multi.ranges);
    if (!multi.tlv_array)
    return -ENOMEM;
    control.access = SNDRV_CTL_ELEM_ACCESS_TLV_READ |
    SNDRV_CTL_ELEM_ACCESS_READWRITE;
    control.tlv.p = multi.tlv_array;
    break;
    case SIMPLE_AMP_MODE_LABELS:
// Use enumerated values
    control.info = simple_amp_multi_kctrl_enum_info;
    control.get = simple_amp_multi_kctrl_enum_get;
    control.put = simple_amp_multi_kctrl_enum_put;
    break;
    case SIMPLE_AMP_MODE_NONE:
// Already set control configuration is enough
    break;
    default:
    return -EINVAL;
    }
// Be consistent between multi->kctrl_val value and the GPIOs value
    ret = simple_amp_multi_kctrl_write_gpios(multi, multi.kctrl_val);
    if (ret)
    goto err_free_tlv_array;
    ret = snd_soc_add_component_controls(component, &control, 1);
    if (ret)
    goto err_free_tlv_array;
    return 0;
    err_free_tlv_array:
    kfree(multi.tlv_array);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn simple_amp_add_basic_dapm(component: *mut snd_soc_component) -> c_int {
    static int simple_amp_add_basic_dapm(struct snd_soc_component *component)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    struct simple_amp *simple_amp = snd_soc_component_get_drvdata(component);
    struct device *dev = component.dev;
    int ret;
// Add basic dapm widgets and routes
    ret = snd_soc_dapm_new_controls(dapm, simple_amp.data.dapm_widgets,
    simple_amp.data.num_dapm_widgets);
    if (ret) {
    dev_err(dev, "Failed to add basic dapm widgets (%d)\n", ret);
    return ret;
    }
    ret = snd_soc_dapm_add_routes(dapm, simple_amp.data.dapm_routes,
    simple_amp.data.num_dapm_routes);
    if (ret) {
    dev_err(dev, "Failed to add basic dapm routes (%d)\n", ret);
    return ret;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_supply {
    pub prop_name: *const c_char,
    pub dapm_widget: snd_soc_dapm_widget,
    pub dapm_route: snd_soc_dapm_route,
}

    static const struct simple_amp_supply simple_amp_supplies[] = {
    {
    .prop_name = "vddio-supply",
    .dapm_widget = SND_SOC_DAPM_REGULATOR_SUPPLY("vddio", 0, 0),
    .dapm_route = { "PGA", core::ptr::null_mut(), "vddio" },
    }, {
    .prop_name = "vdda1-supply",
    .dapm_widget = SND_SOC_DAPM_REGULATOR_SUPPLY("vdda1", 0, 0),
    .dapm_route = { "PGA", core::ptr::null_mut(), "vdda1" },
    }, {
    .prop_name = "vdda2-supply",
    .dapm_widget = SND_SOC_DAPM_REGULATOR_SUPPLY("vdda2", 0, 0),
    .dapm_route = { "PGA", core::ptr::null_mut(), "vdda2" },
    },
    { /* End of list */}
    };
#[no_mangle]
unsafe extern "C" fn simple_amp_add_power_supplies(component: *mut snd_soc_component) -> c_int {
    static int simple_amp_add_power_supplies(struct snd_soc_component *component)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    struct simple_amp *simple_amp = snd_soc_component_get_drvdata(component);
    const struct simple_amp_supply *supply;
    struct device *dev = component.dev;
    int ret;
//
// Those additional power supplies are attached to the PGA.
// If PGA is not supported, simply skipped them.
//
    if (!(simple_amp.data.supports & SIMPLE_AUDIO_SUPPORT_PGA)) {
    dev_err(dev, "Extra power supplied need PGA\n");
    return -EINVAL;
    }
    supply = simple_amp_supplies;
    do {
    if (!of_property_present(dev.of_node, supply.prop_name))
    continue;
    ret = snd_soc_dapm_new_controls(dapm, &supply.dapm_widget, 1);
    if (ret) {
    dev_err(dev, "Failed to add control for '%s' (%d)\n",
    supply.prop_name, ret);
    return ret;
    }
    ret = snd_soc_dapm_add_routes(dapm, &supply.dapm_route, 1);
    if (ret) {
    dev_err(dev, "Failed to add route for '%s' (%d)\n",
    supply.prop_name, ret);
    return ret;
    }
    } while ((++supply).prop_name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn simple_amp_component_probe(component: *mut snd_soc_component) -> c_int {
    static int simple_amp_component_probe(struct snd_soc_component *component)
    {
    struct simple_amp *simple_amp = snd_soc_component_get_drvdata(component);
    int ret;
// Add basic dapm widgets and routes
    ret = simple_amp_add_basic_dapm(component);
    if (ret)
    return ret;
// Add additional power supplies
    if (simple_amp.data.supports & SIMPLE_AUDIO_SUPPORT_POWER_SUPPLIES) {
    ret = simple_amp_add_power_supplies(component);
    if (ret)
    return ret;
    }
    if (simple_amp.mute.gpio) {
//
// The name of the GPIO used is mute. According to this name, 1
// means muted and 0 means un-muted.
//
// An inversion is expected by ALSA. Indeed from ALSA point of
// view, 1 means 'on' (un-muted) and 0 means 'off' (muted).
//
    simple_amp.mute.is_inverted = true;
    simple_amp.mute.kctrl_val = 1; /* Un-muted */
    ret = simple_amp_single_add_kcontrol(component, &simple_amp.mute);
    if (ret)
    return ret;
    }
    if (simple_amp.bypass.gpio) {
    ret = simple_amp_single_add_kcontrol(component, &simple_amp.bypass);
    if (ret)
    return ret;
    }
    if (simple_amp.gain.gpios) {
    ret = simple_amp_multi_add_kcontrol(component, &simple_amp.gain);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn simple_amp_component_remove(component: *mut snd_soc_component) {
    static void simple_amp_component_remove(struct snd_soc_component *component)
    {
    struct simple_amp *simple_amp = snd_soc_component_get_drvdata(component);
    kfree(simple_amp.gain.tlv_array);
    simple_amp.gain.tlv_array = core::ptr::null_mut();
    }
    static const struct snd_soc_component_driver simple_amp_component_driver = {
    .probe = simple_amp_component_probe,
    .remove = simple_amp_component_remove,
    };
    static int simple_amp_parse_single_gpio(struct device *dev,
    struct simple_amp_single *single,
    const char *gpio_property)
    {
// Start with the inactive value
    single.is_inverted = false;
    single.kctrl_val = 0;
    single.gpio = devm_gpiod_get_optional(dev, gpio_property, GPIOD_OUT_LOW);
    if (IS_ERR(single.gpio))
    return dev_err_probe(dev, PTR_ERR(single.gpio),
    "Failed to get '%s' gpio\n",
    gpio_property);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn simple_amp_cmp_ranges(a: *const c_void, b: *const c_void) -> c_int {
    static int simple_amp_cmp_ranges(const void *a, const void *b)
    {
    const struct simple_amp_range *a_range = a;
    const struct simple_amp_range *b_range = b;
// Ranges a and b don't overlap. This has been already checked
    return a_range.min.gain_db - b_range.max.gain_db;
    }
    static int simple_amp_check_new_range(const struct simple_amp_range *new_range,
    const struct simple_amp_range *tab_ranges,
    unsigned int nb_ranges)
    {
    unsigned int i;
    for (i = 0; i < nb_ranges; i++) {
// Check for range overlaps
    if (new_range.min.gain_db >= tab_ranges[i].min.gain_db &&
    new_range.min.gain_db <= tab_ranges[i].max.gain_db)
    return -EINVAL;
    if (new_range.max.gain_db >= tab_ranges[i].min.gain_db &&
    new_range.max.gain_db <= tab_ranges[i].max.gain_db)
    return -EINVAL;
    if (new_range.min.gain_db <= tab_ranges[i].min.gain_db &&
    new_range.max.gain_db >= tab_ranges[i].max.gain_db)
    return -EINVAL;
    }
    return 0;
    }
    static int simple_amp_parse_ranges(struct device *dev,
    struct simple_amp_multi *multi,
    const char *ranges_property)
    {
    struct simple_amp_ranges *ranges = &multi.ranges;
    struct simple_amp_range *range;
    struct device_node *np = dev.of_node;
    struct simple_amp_point first_point;
    unsigned int max_gpio_val;
    unsigned int i;
    int ret;
    u32 u;
    s32 s;
    max_gpio_val = (1 << multi.gpios.ndescs) - 1;
    ret = of_property_count_u32_elems(np, ranges_property);
    if (ret < 0)
    return ret;
// The ranges array cannot be empty
    if (ret == 0)
    return -EINVAL;
//
// One range item is composed of 2 points and each point is composed of
// 2 values.
//
    if (ret % 4)
    return -EINVAL;
    ranges.nb_ranges = ret / 4;
// The worst case is one range per possible gpio value
    if (ranges.nb_ranges > max_gpio_val + 1)
    return -EINVAL;
    ranges.tab_ranges = devm_kcalloc(dev, ranges.nb_ranges,
    sizeof(*ranges.tab_ranges),
    GFP_KERNEL);
    if (!ranges.tab_ranges)
    return -ENOMEM;
    multi.kctrl_max = 0;
    for (i = 0; i < ranges.nb_ranges; i++) {
    range = &ranges.tab_ranges[i];
// First gpios value
    ret = of_property_read_u32_index(np, ranges_property, i * 4, &u);
    if (ret)
    return ret;
    if (u > max_gpio_val)
    return -EINVAL;
    range.min.gpio_val = u;
// First Gain value
    ret = of_property_read_s32_index(np, ranges_property, i * 4 + 1, &s);
    if (ret)
    return ret;
    range.min.gain_db = s;
// Second gpios value
    ret = of_property_read_u32_index(np, ranges_property, i * 4 + 2, &u);
    if (ret)
    return ret;
    if (u > max_gpio_val)
    return -EINVAL;
    range.max.gpio_val = u;
// Second Gain value
    ret = of_property_read_s32_index(np, ranges_property, i * 4 + 3, &s);
    if (ret)
    return ret;
    range.max.gain_db = s;
// Save the first point for later usage
    if (i == 0)
    first_point = range.min;
// Fix min and max if needed
    if (range.min.gain_db > range.max.gain_db)
    swap(range.min, range.max);
    ret = simple_amp_check_new_range(range, ranges.tab_ranges, i);
    if (ret)
    return ret;
    range.nb_points = abs_diff(range.min.gpio_val,
    range.max.gpio_val) + 1;
    multi.kctrl_max += range.nb_points;
    }
    multi.kctrl_max -= 1;
// Sort the tab_range array by gain_db value
    sort(ranges.tab_ranges, ranges.nb_ranges, sizeof(*ranges.tab_ranges),
    simple_amp_cmp_ranges, core::ptr::null_mut());
//
// multi->kctrl_val is the index in tab_ranges.
//
// Choose to have the initial amplification value set to the first point
// available in the first range available in the tab_ranges array before
// sorting.
//
// This first point has been identified before sorting. Search for it in
// the sorted array in order to set the multi->kctrl_val initial value.
//
    multi.kctrl_val = 0;
    for (i = 0; i < ranges.nb_ranges; i++) {
    range = &ranges.tab_ranges[i];
    if (range.min.gpio_val == first_point.gpio_val &&
    range.min.gain_db == first_point.gain_db)
    break;
    multi.kctrl_val += range.nb_points;
    if (range.max.gpio_val == first_point.gpio_val &&
    range.max.gain_db == first_point.gain_db) {
    multi.kctrl_val--;
    break;
    }
    }
    return 0;
    }
    static int simple_amp_parse_labels(struct device *dev,
    struct simple_amp_multi *multi,
    const char *labels_property)
    {
    struct simple_amp_labels *labels = &multi.labels;
    struct device_node *np = dev.of_node;
    int ret;
    ret = of_property_count_strings(np, labels_property);
    if (ret < 0)
    return ret;
// The labels array cannot be empty
    if (ret == 0)
    return -EINVAL;
    labels.nb_labels = ret;
    if (labels.nb_labels > (1 << multi.gpios.ndescs))
    return -EINVAL;
    labels.tab_labels = devm_kcalloc(dev, labels.nb_labels,
    sizeof(*labels.tab_labels),
    GFP_KERNEL);
    if (!labels.tab_labels)
    return -ENOMEM;
    multi.kctrl_max = labels.nb_labels - 1;
    multi.kctrl_val = 0;
    return of_property_read_string_array(np, labels_property, labels.tab_labels,
    labels.nb_labels);
    }
    static int simple_amp_parse_multi_gpio(struct device *dev,
    struct simple_amp_multi *multi,
    const char *gpios_property,
    const char *ranges_property,
    const char *labels_property)
    {
    struct device_node *np = dev.of_node;
    int ret;
// Start with the value 0 (GPIO inactive). Can be changed later
    multi.kctrl_val = 0;
    multi.gpios = devm_gpiod_get_array_optional(dev, gpios_property, GPIOD_OUT_LOW);
    if (IS_ERR(multi.gpios))
    return dev_err_probe(dev, PTR_ERR(multi.gpios),
    "Failed to get '%s' gpios\n",
    gpios_property);
    if (!multi.gpios)
    return 0;
    if (multi.gpios.ndescs > 16)
    return dev_err_probe(dev, -EINVAL,
    "Number of '%s' gpios limited to 16\n",
    gpios_property);
// Set default value for the kctrl_max. Can be changed later
    multi.kctrl_max = (1 << multi.gpios.ndescs) - 1;
    multi.mode = SIMPLE_AMP_MODE_NONE;
    if (of_property_present(np, ranges_property)) {
    ret = simple_amp_parse_ranges(dev, multi, ranges_property);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to parse '%s'\n",
    ranges_property);
    multi.mode = SIMPLE_AMP_MODE_RANGES;
    } else if (of_property_present(np, labels_property)) {
    ret = simple_amp_parse_labels(dev, multi, labels_property);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to parse '%s'\n",
    labels_property);
    multi.mode = SIMPLE_AMP_MODE_LABELS;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn simple_amp_probe(pdev: *mut platform_device) -> c_int {
    static int simple_amp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct simple_amp *simple_amp;
    int ret;
    simple_amp = devm_kzalloc(dev, sizeof(*simple_amp), GFP_KERNEL);
    if (!simple_amp)
    return -ENOMEM;
    platform_set_drvdata(pdev, simple_amp);
    simple_amp.data = of_device_get_match_data(dev);
    if (!simple_amp.data)
    return -EINVAL;
    simple_amp.gpiod_enable = devm_gpiod_get_optional(dev, "enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(simple_amp.gpiod_enable))
    return dev_err_probe(dev, PTR_ERR(simple_amp.gpiod_enable),
    "Failed to get 'enable' gpio");
    if (simple_amp.data.supports & SIMPLE_AUDIO_SUPPORT_MUTE) {
    ret = simple_amp_parse_single_gpio(dev, &simple_amp.mute, "mute");
    if (ret)
    return ret;
    }
    if (simple_amp.data.supports & SIMPLE_AUDIO_SUPPORT_BYPASS) {
    ret = simple_amp_parse_single_gpio(dev, &simple_amp.bypass, "bypass");
    if (ret)
    return ret;
    }
    if (simple_amp.data.supports & SIMPLE_AUDIO_SUPPORT_PGA) {
    ret = simple_amp_parse_multi_gpio(dev, &simple_amp.gain, "gain",
    "gain-ranges", "gain-labels");
    if (ret)
    return ret;
    }
// Set controls name
    simple_amp.gain.control_name = "Volume";
    simple_amp.mute.control_name = "Switch";
    simple_amp.bypass.control_name = "Bypass Switch";
    if (simple_amp.gain.mode == SIMPLE_AMP_MODE_LABELS) {
//
// The gain widget control will use enumerated values.
//
// Having just "Voltage" and "Switch" widget names with
// enumerated values and boolean value can confuse ALSA in terms
// of possible values (strings).
//
// Make things clear and avoid the just "Switch" name in that
// case.
//
    simple_amp.mute.control_name = "Out Switch";
    }
    return devm_snd_soc_register_component(dev,
    &simple_amp_component_driver,
    core::ptr::null_mut(), 0);
    }
    static const struct simple_amp_data simple_audio_amplifier_data = {
    .dapm_widgets		= simple_amp_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(simple_amp_dapm_widgets),
    .dapm_routes		= simple_amp_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(simple_amp_dapm_routes),
    };
    static const struct simple_amp_data simple_audio_mono_pga_data = {
    .supports		= SIMPLE_AUDIO_SUPPORT_PGA |
    SIMPLE_AUDIO_SUPPORT_POWER_SUPPLIES |
    SIMPLE_AUDIO_SUPPORT_MUTE |
    SIMPLE_AUDIO_SUPPORT_BYPASS,
    .dapm_widgets		= simple_amp_mono_pga_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(simple_amp_mono_pga_dapm_widgets),
    .dapm_routes		= simple_amp_mono_pga_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(simple_amp_mono_pga_dapm_routes),
    };
    static const struct simple_amp_data simple_audio_stereo_pga_data = {
    .supports		= SIMPLE_AUDIO_SUPPORT_PGA |
    SIMPLE_AUDIO_SUPPORT_POWER_SUPPLIES |
    SIMPLE_AUDIO_SUPPORT_MUTE |
    SIMPLE_AUDIO_SUPPORT_BYPASS,
    .dapm_widgets		= simple_amp_stereo_pga_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(simple_amp_stereo_pga_dapm_widgets),
    .dapm_routes		= simple_amp_stereo_pga_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(simple_amp_stereo_pga_dapm_routes),
    };
    static const struct of_device_id simple_amp_ids[] = {
    { .compatible = "dioo,dio2125",		  .data = &simple_audio_amplifier_data},
    { .compatible = "simple-audio-amplifier", .data = &simple_audio_amplifier_data},
    { .compatible = "gpio-audio-amp-mono",	  .data = &simple_audio_mono_pga_data},
    { .compatible = "gpio-audio-amp-stereo",  .data = &simple_audio_stereo_pga_data},
    { }
    };
    MODULE_DEVICE_TABLE(of, simple_amp_ids);
    static struct platform_driver simple_amp_driver = {
    .driver = {
    .name = "simple-amplifier",
    .of_match_table = simple_amp_ids,
    },
    .probe = simple_amp_probe,
    };
    module_platform_driver(simple_amp_driver);
    MODULE_DESCRIPTION("ASoC Simple Audio Amplifier driver");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
    MODULE_LICENSE("GPL");
