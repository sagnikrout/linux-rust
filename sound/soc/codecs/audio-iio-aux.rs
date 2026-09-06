//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/audio-iio-aux.c
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
// ALSA SoC glue to use IIO devices as audio components
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_iio_aux_chan {
    pub iio_chan: *mut iio_channel,
    pub name: *const c_char,
    pub max: c_int,
    pub min: c_int,
    pub is_invert_range: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_iio_aux {
    pub dev: *mut device,
    pub num_chans: c_uint,
    pub __counted_by(num_chans): audio_iio_aux_chan chans[],
}

    static int audio_iio_aux_info_volsw(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct audio_iio_aux_chan *chan = (struct audio_iio_aux_chan *)kcontrol.private_value;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = chan.max - chan.min;
    uinfo.type = (uinfo.value.integer.max == 1) ?
    SNDRV_CTL_ELEM_TYPE_BOOLEAN : SNDRV_CTL_ELEM_TYPE_INTEGER;
    return 0;
    }
    static int audio_iio_aux_get_volsw(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct audio_iio_aux_chan *chan = (struct audio_iio_aux_chan *)kcontrol.private_value;
    let mut max: c_int = chan.max;
    let mut min: c_int = chan.min;
    let mut invert_range: bool = chan.is_invert_range;
    int ret;
    int val;
    ret = iio_read_channel_raw(chan.iio_chan, &val);
    if (ret < 0)
    return ret;
    ucontrol.value.integer.value[0] = val - min;
    if (invert_range)
    ucontrol.value.integer.value[0] = max - ucontrol.value.integer.value[0];
    return 0;
    }
    static int audio_iio_aux_put_volsw(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct audio_iio_aux_chan *chan = (struct audio_iio_aux_chan *)kcontrol.private_value;
    let mut max: c_int = chan.max;
    let mut min: c_int = chan.min;
    let mut invert_range: bool = chan.is_invert_range;
    int val;
    int ret;
    int tmp;
    val = ucontrol.value.integer.value[0];
    if (val < 0)
    return -EINVAL;
    if (val > max - min)
    return -EINVAL;
    val = val + min;
    if (invert_range)
    val = max - val;
    ret = iio_read_channel_raw(chan.iio_chan, &tmp);
    if (ret < 0)
    return ret;
    if (tmp == val)
    return 0;
    ret = iio_write_channel_raw(chan.iio_chan, val);
    if (ret)
    return ret;
    return 1; /* The value changed */
    }
    static int audio_iio_aux_add_controls(struct snd_soc_component *component,
    struct audio_iio_aux_chan *chan)
    {
    struct snd_kcontrol_new control = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = chan.name,
    .info = audio_iio_aux_info_volsw,
    .get = audio_iio_aux_get_volsw,
    .put = audio_iio_aux_put_volsw,
    .private_value = (unsigned long)chan,
    };
    return snd_soc_add_component_controls(component, &control, 1);
    }
//
// These data could be on stack but they are pretty big.
// As ASoC internally copy them and protect them against concurrent accesses
// (snd_soc_bind_card() protects using client_mutex), keep them in the global
// data area.
//
    static struct snd_soc_dapm_widget widgets[3];
    static struct snd_soc_dapm_route routes[2];
// Be sure sizes are correct (need 3 widgets and 2 routes)
    static_assert(ARRAY_SIZE(widgets) >= 3, "3 widgets are needed");
    static_assert(ARRAY_SIZE(routes) >= 2, "2 routes are needed");
    static int audio_iio_aux_add_dapms(struct snd_soc_component *component,
    struct audio_iio_aux_chan *chan)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    int ret;
// Allocated names are not needed afterwards (duplicated in ASoC internals)
    char *input_name __free(kfree) = kasprintf(GFP_KERNEL, "%s IN", chan.name);
    if (!input_name)
    return -ENOMEM;
    char *output_name __free(kfree) = kasprintf(GFP_KERNEL, "%s OUT", chan.name);
    if (!output_name)
    return -ENOMEM;
    char *pga_name __free(kfree) = kasprintf(GFP_KERNEL, "%s PGA", chan.name);
    if (!pga_name)
    return -ENOMEM;
    widgets[0] = SND_SOC_DAPM_INPUT(input_name);
    widgets[1] = SND_SOC_DAPM_OUTPUT(output_name);
    widgets[2] = SND_SOC_DAPM_PGA(pga_name, SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0);
    ret = snd_soc_dapm_new_controls(dapm, widgets, 3);
    if (ret)
    return ret;
    routes[0].sink = pga_name;
    routes[0].control = core::ptr::null_mut();
    routes[0].source = input_name;
    routes[1].sink = output_name;
    routes[1].control = core::ptr::null_mut();
    routes[1].source = pga_name;
    return snd_soc_dapm_add_routes(dapm, routes, 2);
    }
#[no_mangle]
unsafe extern "C" fn audio_iio_aux_component_probe(component: *mut snd_soc_component) -> c_int {
    static int audio_iio_aux_component_probe(struct snd_soc_component *component)
    {
    struct audio_iio_aux *iio_aux = snd_soc_component_get_drvdata(component);
    struct audio_iio_aux_chan *chan;
    int ret;
    int i;
    for (i = 0; i < iio_aux.num_chans; i++) {
    chan = iio_aux.chans + i;
    ret = iio_read_max_channel_raw(chan.iio_chan, &chan.max);
    if (ret)
    return dev_err_probe(component.dev, ret,
    "chan[%d] %s: Cannot get max raw value\n",
    i, chan.name);
    ret = iio_read_min_channel_raw(chan.iio_chan, &chan.min);
    if (ret)
    return dev_err_probe(component.dev, ret,
    "chan[%d] %s: Cannot get min raw value\n",
    i, chan.name);
    if (chan.min > chan.max) {
//
// This should never happen but to avoid any check
// later, just swap values here to ensure that the
// minimum value is lower than the maximum value.
//
    dev_dbg(component.dev, "chan[%d] %s: Swap min and max\n",
    i, chan.name);
    swap(chan.min, chan.max);
    }
// Set initial value
    ret = iio_write_channel_raw(chan.iio_chan,
    chan.is_invert_range ? chan.max : chan.min);
    if (ret)
    return dev_err_probe(component.dev, ret,
    "chan[%d] %s: Cannot set initial value\n",
    i, chan.name);
    ret = audio_iio_aux_add_controls(component, chan);
    if (ret)
    return ret;
    ret = audio_iio_aux_add_dapms(component, chan);
    if (ret)
    return ret;
    dev_dbg(component.dev, "chan[%d]: Added %s (min=%d, max=%d, invert=%s)\n",
    i, chan.name, chan.min, chan.max,
    str_on_off(chan.is_invert_range));
    }
    return 0;
    }
    static const struct snd_soc_component_driver audio_iio_aux_component_driver = {
    .probe = audio_iio_aux_component_probe,
    };
#[no_mangle]
unsafe extern "C" fn audio_iio_aux_probe(pdev: *mut platform_device) -> c_int {
    static int audio_iio_aux_probe(struct platform_device *pdev)
    {
    struct audio_iio_aux_chan *iio_aux_chan;
    struct device *dev = &pdev.dev;
    struct audio_iio_aux *iio_aux;
    int count;
    int ret;
    int i;
    count = device_property_string_array_count(dev, "io-channel-names");
    if (count < 0)
    return dev_err_probe(dev, count, "failed to count io-channel-names\n");
    iio_aux = devm_kzalloc(dev, struct_size(iio_aux, chans, count), GFP_KERNEL);
    if (!iio_aux)
    return -ENOMEM;
    iio_aux.dev = dev;
    iio_aux.num_chans = count;
    const char **names __free(kfree) = kcalloc(iio_aux.num_chans,
    sizeof(*names),
    GFP_KERNEL);
    if (!names)
    return -ENOMEM;
    u32 *invert_ranges __free(kfree) = kcalloc(iio_aux.num_chans,
    sizeof(*invert_ranges),
    GFP_KERNEL);
    if (!invert_ranges)
    return -ENOMEM;
    ret = device_property_read_string_array(dev, "io-channel-names",
    names, iio_aux.num_chans);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to read io-channel-names\n");
//
// snd-control-invert-range is optional and can contain fewer items
// than the number of channels. Unset values default to 0.
//
    count = device_property_count_u32(dev, "snd-control-invert-range");
    if (count > 0) {
    count = min_t(unsigned int, count, iio_aux.num_chans);
    ret = device_property_read_u32_array(dev, "snd-control-invert-range",
    invert_ranges, count);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to read snd-control-invert-range\n");
    }
    for (i = 0; i < iio_aux.num_chans; i++) {
    iio_aux_chan = iio_aux.chans + i;
    iio_aux_chan.name = names[i];
    iio_aux_chan.is_invert_range = invert_ranges[i];
    iio_aux_chan.iio_chan = devm_iio_channel_get(dev, iio_aux_chan.name);
    if (IS_ERR(iio_aux_chan.iio_chan))
    return dev_err_probe(dev, PTR_ERR(iio_aux_chan.iio_chan),
    "get IIO channel '%s' failed\n",
    iio_aux_chan.name);
    }
    platform_set_drvdata(pdev, iio_aux);
    return devm_snd_soc_register_component(dev, &audio_iio_aux_component_driver,
    core::ptr::null_mut(), 0);
    }
    static const struct of_device_id audio_iio_aux_ids[] = {
    { .compatible = "audio-iio-aux" },
    { }
    };
    MODULE_DEVICE_TABLE(of, audio_iio_aux_ids);
    static struct platform_driver audio_iio_aux_driver = {
    .driver = {
    .name = "audio-iio-aux",
    .of_match_table = audio_iio_aux_ids,
    },
    .probe = audio_iio_aux_probe,
    };
    module_platform_driver(audio_iio_aux_driver);
    MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
    MODULE_DESCRIPTION("IIO ALSA SoC aux driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
