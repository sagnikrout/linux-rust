//! Automatically rewritten from C to Rust
//! Source: sound/firewire/oxfw/oxfw-pcm.c
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
// oxfw_pcm.c - a part of driver for OXFW970/971 based devices
//
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>
//

    static int hw_rule_rate(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    u8 **formats = rule.private;
    struct snd_interval *r =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    const struct snd_interval *c =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_interval t = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    struct snd_oxfw_stream_formation formation;
    int i, err;
    for (i = 0; i < SND_OXFW_STREAM_FORMAT_ENTRIES; i++) {
    if (formats[i] == core::ptr::null_mut())
    continue;
    err = snd_oxfw_stream_parse_format(formats[i], &formation);
    if (err < 0)
    continue;
    if (!snd_interval_test(c, formation.pcm))
    continue;
    t.min = min(t.min, formation.rate);
    t.max = max(t.max, formation.rate);
    }
    return snd_interval_refine(r, &t);
    }
    static int hw_rule_channels(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    u8 **formats = rule.private;
    struct snd_interval *c =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    const struct snd_interval *r =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    struct snd_oxfw_stream_formation formation;
    int i, j, err;
    unsigned int count, list[SND_OXFW_STREAM_FORMAT_ENTRIES] = {0};
    count = 0;
    for (i = 0; i < SND_OXFW_STREAM_FORMAT_ENTRIES; i++) {
    if (formats[i] == core::ptr::null_mut())
    break;
    err = snd_oxfw_stream_parse_format(formats[i], &formation);
    if (err < 0)
    continue;
    if (!snd_interval_test(r, formation.rate))
    continue;
    if (list[count] == formation.pcm)
    continue;
    for (j = 0; j < ARRAY_SIZE(list); j++) {
    if (list[j] == formation.pcm)
    break;
    }
    if (j == ARRAY_SIZE(list)) {
    list[count] = formation.pcm;
    if (++count == ARRAY_SIZE(list))
    break;
    }
    }
    return snd_interval_list(c, count, list, 0);
    }
#[no_mangle]
unsafe extern "C" fn limit_channels_and_rates(hw: *mut snd_pcm_hardware, formats: *mut u8) {
    static void limit_channels_and_rates(struct snd_pcm_hardware *hw, u8 **formats)
    {
    struct snd_oxfw_stream_formation formation;
    int i, err;
    hw.channels_min = UINT_MAX;
    hw.channels_max = 0;
    hw.rate_min = UINT_MAX;
    hw.rate_max = 0;
    hw.rates = 0;
    for (i = 0; i < SND_OXFW_STREAM_FORMAT_ENTRIES; i++) {
    if (formats[i] == core::ptr::null_mut())
    break;
    err = snd_oxfw_stream_parse_format(formats[i], &formation);
    if (err < 0)
    continue;
    hw.channels_min = min(hw.channels_min, formation.pcm);
    hw.channels_max = max(hw.channels_max, formation.pcm);
    hw.rate_min = min(hw.rate_min, formation.rate);
    hw.rate_max = max(hw.rate_max, formation.rate);
    hw.rates |= snd_pcm_rate_to_rate_bit(formation.rate);
    }
    }
    static int init_hw_params(struct snd_oxfw *oxfw,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    u8 **formats;
    struct amdtp_stream *stream;
    int err;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    runtime.hw.formats = AM824_IN_PCM_FORMAT_BITS;
    stream = &oxfw.tx_stream;
    formats = oxfw.tx_stream_formats;
    } else {
    runtime.hw.formats = AM824_OUT_PCM_FORMAT_BITS;
    stream = &oxfw.rx_stream;
    formats = oxfw.rx_stream_formats;
    }
    limit_channels_and_rates(&runtime.hw, formats);
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
    hw_rule_channels, formats,
    SNDRV_PCM_HW_PARAM_RATE, -1);
    if (err < 0)
    goto end;
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    hw_rule_rate, formats,
    SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if (err < 0)
    goto end;
    err = amdtp_am824_add_pcm_hw_constraints(stream, runtime);
    end:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn limit_to_current_params(substream: *mut snd_pcm_substream) -> c_int {
    static int limit_to_current_params(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    struct snd_oxfw_stream_formation formation;
    enum avc_general_plug_dir dir;
    int err;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE)
    dir = AVC_GENERAL_PLUG_DIR_OUT;
    else
    dir = AVC_GENERAL_PLUG_DIR_IN;
    err = snd_oxfw_stream_get_current_formation(oxfw, dir, &formation);
    if (err < 0)
    goto end;
    substream.runtime.hw.channels_min = formation.pcm;
    substream.runtime.hw.channels_max = formation.pcm;
    substream.runtime.hw.rate_min = formation.rate;
    substream.runtime.hw.rate_max = formation.rate;
    end:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_open(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    struct amdtp_domain *d = &oxfw.domain;
    int err;
    err = snd_oxfw_stream_lock_try(oxfw);
    if (err < 0)
    return err;
    err = init_hw_params(oxfw, substream);
    if (err < 0)
    goto err_locked;
    scoped_guard(mutex, &oxfw.mutex) {
// When source of clock is not internal or any stream is reserved for
// transmission of PCM frames, the available sampling rate is limited
// at current one.
    if (oxfw.substreams_count > 0 && d.events_per_period > 0) {
    let mut frames_per_period: c_uint = d.events_per_period;
    let mut frames_per_buffer: c_uint = d.events_per_buffer;
    err = limit_to_current_params(substream);
    if (err < 0)
    goto err_locked;
    if (frames_per_period > 0) {
    err = snd_pcm_hw_constraint_minmax(substream.runtime,
    SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
    frames_per_period, frames_per_period);
    if (err < 0)
    goto err_locked;
    err = snd_pcm_hw_constraint_minmax(substream.runtime,
    SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
    frames_per_buffer, frames_per_buffer);
    if (err < 0)
    goto err_locked;
    }
    }
    }
    snd_pcm_set_sync(substream);
    return 0;
    err_locked:
    snd_oxfw_stream_lock_release(oxfw);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_close(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    snd_oxfw_stream_lock_release(oxfw);
    return 0;
    }
    static int pcm_capture_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    let mut err: c_int = 0;
    if (substream.runtime.state == SNDRV_PCM_STATE_OPEN) {
    let mut rate: c_uint = params_rate(hw_params);
    let mut channels: c_uint = params_channels(hw_params);
    let mut frames_per_period: c_uint = params_period_size(hw_params);
    let mut frames_per_buffer: c_uint = params_buffer_size(hw_params);
    guard(mutex)(&oxfw.mutex);
    err = snd_oxfw_stream_reserve_duplex(oxfw, &oxfw.tx_stream,
    rate, channels, frames_per_period,
    frames_per_buffer);
    if (err >= 0)
    ++oxfw.substreams_count;
    }
    return err;
    }
    static int pcm_playback_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    let mut err: c_int = 0;
    if (substream.runtime.state == SNDRV_PCM_STATE_OPEN) {
    let mut rate: c_uint = params_rate(hw_params);
    let mut channels: c_uint = params_channels(hw_params);
    let mut frames_per_period: c_uint = params_period_size(hw_params);
    let mut frames_per_buffer: c_uint = params_buffer_size(hw_params);
    guard(mutex)(&oxfw.mutex);
    err = snd_oxfw_stream_reserve_duplex(oxfw, &oxfw.rx_stream,
    rate, channels, frames_per_period,
    frames_per_buffer);
    if (err >= 0)
    ++oxfw.substreams_count;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_capture_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    guard(mutex)(&oxfw.mutex);
    if (substream.runtime.state != SNDRV_PCM_STATE_OPEN)
    --oxfw.substreams_count;
    snd_oxfw_stream_stop_duplex(oxfw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_playback_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    guard(mutex)(&oxfw.mutex);
    if (substream.runtime.state != SNDRV_PCM_STATE_OPEN)
    --oxfw.substreams_count;
    snd_oxfw_stream_stop_duplex(oxfw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    int err;
    scoped_guard(mutex, &oxfw.mutex) {
    err = snd_oxfw_stream_start_duplex(oxfw);
    if (err < 0)
    return err;
    }
    amdtp_stream_pcm_prepare(&oxfw.tx_stream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    int err;
    scoped_guard(mutex, &oxfw.mutex) {
    err = snd_oxfw_stream_start_duplex(oxfw);
    if (err < 0)
    return err;
    }
    amdtp_stream_pcm_prepare(&oxfw.rx_stream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int pcm_capture_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    struct snd_pcm_substream *pcm;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    pcm = substream;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    pcm = core::ptr::null_mut();
    break;
    default:
    return -EINVAL;
    }
    amdtp_stream_pcm_trigger(&oxfw.tx_stream, pcm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int pcm_playback_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    struct snd_pcm_substream *pcm;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    pcm = substream;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    pcm = core::ptr::null_mut();
    break;
    default:
    return -EINVAL;
    }
    amdtp_stream_pcm_trigger(&oxfw.rx_stream, pcm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_pointer(sbstm: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t pcm_capture_pointer(struct snd_pcm_substream *sbstm)
    {
    struct snd_oxfw *oxfw = sbstm.private_data;
    return amdtp_domain_stream_pcm_pointer(&oxfw.domain, &oxfw.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_pointer(sbstm: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t pcm_playback_pointer(struct snd_pcm_substream *sbstm)
    {
    struct snd_oxfw *oxfw = sbstm.private_data;
    return amdtp_domain_stream_pcm_pointer(&oxfw.domain, &oxfw.rx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_capture_ack(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&oxfw.domain, &oxfw.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_playback_ack(struct snd_pcm_substream *substream)
    {
    struct snd_oxfw *oxfw = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&oxfw.domain, &oxfw.rx_stream);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_oxfw_create_pcm(oxfw: *mut snd_oxfw) -> c_int {
    int snd_oxfw_create_pcm(struct snd_oxfw *oxfw)
    {
    static const struct snd_pcm_ops capture_ops = {
    .open      = pcm_open,
    .close     = pcm_close,
    .hw_params = pcm_capture_hw_params,
    .hw_free   = pcm_capture_hw_free,
    .prepare   = pcm_capture_prepare,
    .trigger   = pcm_capture_trigger,
    .pointer   = pcm_capture_pointer,
    .ack       = pcm_capture_ack,
    };
    static const struct snd_pcm_ops playback_ops = {
    .open      = pcm_open,
    .close     = pcm_close,
    .hw_params = pcm_playback_hw_params,
    .hw_free   = pcm_playback_hw_free,
    .prepare   = pcm_playback_prepare,
    .trigger   = pcm_playback_trigger,
    .pointer   = pcm_playback_pointer,
    .ack       = pcm_playback_ack,
    };
    struct snd_pcm *pcm;
    let mut cap: c_uint = 0;
    int err;
    if (oxfw.has_output)
    cap = 1;
    err = snd_pcm_new(oxfw.card, oxfw.card.driver, 0, 1, cap, &pcm);
    if (err < 0)
    return err;
    pcm.private_data = oxfw;
    pcm.nonatomic = true;
    strscpy(pcm.name, oxfw.card.shortname);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &playback_ops);
    if (cap > 0)
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &capture_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);
    return 0;
    }
