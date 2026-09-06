//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/display/drm_hdmi_audio_helper.c
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


// SPDX-License-Identifier: MIT
//
// Copyright (c) 2024 Linaro Ltd
//

#[no_mangle]
unsafe extern "C" fn drm_connector_hdmi_audio_startup(dev: *mut device, data: *mut c_void) -> c_int {
    static int drm_connector_hdmi_audio_startup(struct device *dev, void *data)
    {
    struct drm_connector *connector = data;
    const struct drm_connector_hdmi_audio_funcs *funcs =
    connector.hdmi_audio.funcs;
    if (funcs.startup)
    return funcs.startup(connector);
    return 0;
    }
    static int drm_connector_hdmi_audio_prepare(struct device *dev, void *data,
    struct hdmi_codec_daifmt *fmt,
    struct hdmi_codec_params *hparms)
    {
    struct drm_connector *connector = data;
    const struct drm_connector_hdmi_audio_funcs *funcs =
    connector.hdmi_audio.funcs;
    return funcs.prepare(connector, fmt, hparms);
    }
#[no_mangle]
unsafe extern "C" fn drm_connector_hdmi_audio_shutdown(dev: *mut device, data: *mut c_void) {
    static void drm_connector_hdmi_audio_shutdown(struct device *dev, void *data)
    {
    struct drm_connector *connector = data;
    const struct drm_connector_hdmi_audio_funcs *funcs =
    connector.hdmi_audio.funcs;
    return funcs.shutdown(connector);
    }
    static int drm_connector_hdmi_audio_mute_stream(struct device *dev, void *data,
    bool enable, int direction)
    {
    struct drm_connector *connector = data;
    const struct drm_connector_hdmi_audio_funcs *funcs =
    connector.hdmi_audio.funcs;
    if (funcs.mute_stream)
    return funcs.mute_stream(connector, enable, direction);
    return -ENOTSUPP;
    }
    static int drm_connector_hdmi_audio_get_dai_id(struct snd_soc_component *comment,
    struct device_node *endpoint,
    void *data)
    {
    struct drm_connector *connector = data;
    struct of_endpoint of_ep;
    int ret;
    if (connector.hdmi_audio.dai_port < 0)
    return -ENOTSUPP;
    ret = of_graph_parse_endpoint(endpoint, &of_ep);
    if (ret < 0)
    return ret;
    if (of_ep.port == connector.hdmi_audio.dai_port)
    return 0;
    return -EINVAL;
    }
    static int drm_connector_hdmi_audio_get_eld(struct device *dev, void *data,
    uint8_t *buf, size_t len)
    {
    struct drm_connector *connector = data;
    mutex_lock(&connector.eld_mutex);
    memcpy(buf, connector.eld, min(sizeof(connector.eld), len));
    mutex_unlock(&connector.eld_mutex);
    return 0;
    }
    static int drm_connector_hdmi_audio_hook_plugged_cb(struct device *dev,
    void *data,
    hdmi_codec_plugged_cb fn,
    struct device *codec_dev)
    {
    struct drm_connector *connector = data;
    mutex_lock(&connector.hdmi_audio.lock);
    connector.hdmi_audio.plugged_cb = fn;
    connector.hdmi_audio.plugged_cb_dev = codec_dev;
    if (fn)
    fn(codec_dev, connector.hdmi_audio.last_state);
    mutex_unlock(&connector.hdmi_audio.lock);
    return 0;
    }
    void drm_connector_hdmi_audio_plugged_notify(struct drm_connector *connector,
    bool plugged)
    {
    mutex_lock(&connector.hdmi_audio.lock);
    connector.hdmi_audio.last_state = plugged;
    if (connector.hdmi_audio.plugged_cb &&
    connector.hdmi_audio.plugged_cb_dev)
    connector.hdmi_audio.plugged_cb(connector.hdmi_audio.plugged_cb_dev,
    connector.hdmi_audio.last_state);
    mutex_unlock(&connector.hdmi_audio.lock);
    }
    EXPORT_SYMBOL(drm_connector_hdmi_audio_plugged_notify);
    static const struct hdmi_codec_ops drm_connector_hdmi_audio_ops = {
    .audio_startup = drm_connector_hdmi_audio_startup,
    .prepare = drm_connector_hdmi_audio_prepare,
    .audio_shutdown = drm_connector_hdmi_audio_shutdown,
    .mute_stream = drm_connector_hdmi_audio_mute_stream,
    .get_eld = drm_connector_hdmi_audio_get_eld,
    .get_dai_id = drm_connector_hdmi_audio_get_dai_id,
    .hook_plugged_cb = drm_connector_hdmi_audio_hook_plugged_cb,
    };
//
// drm_connector_hdmi_audio_init - Initialize HDMI Codec device for the DRM connector
// @connector: A pointer to the connector to allocate codec for
// @hdmi_codec_dev: device to be used as a parent for the HDMI Codec
// @funcs: callbacks for this HDMI Codec
// @max_i2s_playback_channels: maximum number of playback I2S channels
// @i2s_formats: set of I2S formats (use 0 for a bus-specific set)
// @spdif_playback: set if HDMI codec has S/PDIF playback port
// @dai_port: sound DAI port, -1 if it is not enabled
//
// Create a HDMI codec device to be used with the specified connector.
//
// Returns:
// Zero on success, error code on failure.
//
    int drm_connector_hdmi_audio_init(struct drm_connector *connector,
    struct device *hdmi_codec_dev,
    const struct drm_connector_hdmi_audio_funcs *funcs,
    unsigned int max_i2s_playback_channels,
    u64 i2s_formats,
    bool spdif_playback,
    int dai_port)
    {
    struct hdmi_codec_pdata codec_pdata = {
    .ops = &drm_connector_hdmi_audio_ops,
    .max_i2s_channels = max_i2s_playback_channels,
    .i2s = !!max_i2s_playback_channels,
    .i2s_formats = i2s_formats,
    .spdif = spdif_playback,
    .no_i2s_capture = true,
    .no_spdif_capture = true,
    .data = connector,
    };
    struct platform_device *pdev;
    if (!funcs ||
    !funcs.prepare ||
    !funcs.shutdown)
    return -EINVAL;
    connector.hdmi_audio.funcs = funcs;
    connector.hdmi_audio.dai_port = dai_port;
    pdev = platform_device_register_data(hdmi_codec_dev,
    HDMI_CODEC_DRV_NAME,
    PLATFORM_DEVID_AUTO,
    &codec_pdata, sizeof(codec_pdata));
    if (IS_ERR(pdev))
    return PTR_ERR(pdev);
    connector.hdmi_audio.codec_pdev = pdev;
    return 0;
    }
    EXPORT_SYMBOL(drm_connector_hdmi_audio_init);
