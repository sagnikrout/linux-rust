//! Automatically rewritten from C to Rust
//! Source: sound/firewire/motu/motu-pcm.c
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
// motu-pcm.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

    static int motu_rate_constraint(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct snd_motu_packet_format *formats = rule.private;
    const struct snd_interval *c =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_interval *r =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval rates = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int i, pcm_channels, rate, mode;
    for (i = 0; i < ARRAY_SIZE(snd_motu_clock_rates); ++i) {
    rate = snd_motu_clock_rates[i];
    mode = i / 2;
    pcm_channels = formats.pcm_chunks[mode];
    if (!snd_interval_test(c, pcm_channels))
    continue;
    rates.min = min(rates.min, rate);
    rates.max = max(rates.max, rate);
    }
    return snd_interval_refine(r, &rates);
    }
    static int motu_channels_constraint(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct snd_motu_packet_format *formats = rule.private;
    const struct snd_interval *r =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval *c =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_interval channels = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int i, pcm_channels, rate, mode;
    for (i = 0; i < ARRAY_SIZE(snd_motu_clock_rates); ++i) {
    rate = snd_motu_clock_rates[i];
    mode = i / 2;
    if (!snd_interval_test(r, rate))
    continue;
    pcm_channels = formats.pcm_chunks[mode];
    channels.min = min(channels.min, pcm_channels);
    channels.max = max(channels.max, pcm_channels);
    }
    return snd_interval_refine(c, &channels);
    }
    static void limit_channels_and_rates(struct snd_motu *motu,
    struct snd_pcm_runtime *runtime,
    struct snd_motu_packet_format *formats)
    {
    struct snd_pcm_hardware *hw = &runtime.hw;
    unsigned int i, pcm_channels, rate, mode;
    hw.channels_min = UINT_MAX;
    hw.channels_max = 0;
    for (i = 0; i < ARRAY_SIZE(snd_motu_clock_rates); ++i) {
    rate = snd_motu_clock_rates[i];
    mode = i / 2;
    pcm_channels = formats.pcm_chunks[mode];
    if (pcm_channels == 0)
    continue;
    hw.rates |= snd_pcm_rate_to_rate_bit(rate);
    hw.channels_min = min(hw.channels_min, pcm_channels);
    hw.channels_max = max(hw.channels_max, pcm_channels);
    }
    snd_pcm_limit_hw_rates(runtime);
    }
    static int init_hw_info(struct snd_motu *motu,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_pcm_hardware *hw = &runtime.hw;
    struct amdtp_stream *stream;
    struct snd_motu_packet_format *formats;
    int err;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    hw.formats = SNDRV_PCM_FMTBIT_S32;
    stream = &motu.tx_stream;
    formats = &motu.tx_packet_formats;
    } else {
    hw.formats = SNDRV_PCM_FMTBIT_S32;
    stream = &motu.rx_stream;
    formats = &motu.rx_packet_formats;
    }
    limit_channels_and_rates(motu, runtime, formats);
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    motu_rate_constraint, formats,
    SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if (err < 0)
    return err;
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
    motu_channels_constraint, formats,
    SNDRV_PCM_HW_PARAM_RATE, -1);
    if (err < 0)
    return err;
    return amdtp_motu_add_pcm_hw_constraints(stream, runtime);
    }
#[no_mangle]
unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_open(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    struct amdtp_domain *d = &motu.domain;
    enum snd_motu_clock_source src;
    int err;
    err = snd_motu_stream_lock_try(motu);
    if (err < 0)
    return err;
    scoped_guard(mutex, &motu.mutex) {
    err = snd_motu_stream_cache_packet_formats(motu);
    if (err < 0)
    goto err_locked;
    err = init_hw_info(motu, substream);
    if (err < 0)
    goto err_locked;
    err = snd_motu_protocol_get_clock_source(motu, &src);
    if (err < 0)
    goto err_locked;
// When source of clock is not internal or any stream is reserved for
// transmission of PCM frames, the available sampling rate is limited
// at current one.
    if ((src != SND_MOTU_CLOCK_SOURCE_INTERNAL &&
    src != SND_MOTU_CLOCK_SOURCE_SPH) ||
    (motu.substreams_counter > 0 && d.events_per_period > 0)) {
    let mut frames_per_period: c_uint = d.events_per_period;
    let mut frames_per_buffer: c_uint = d.events_per_buffer;
    unsigned int rate;
    err = snd_motu_protocol_get_clock_rate(motu, &rate);
    if (err < 0)
    goto err_locked;
    substream.runtime.hw.rate_min = rate;
    substream.runtime.hw.rate_max = rate;
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
    snd_motu_stream_lock_release(motu);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_close(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    snd_motu_stream_lock_release(motu);
    return 0;
    }
    static int pcm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_motu *motu = substream.private_data;
    let mut err: c_int = 0;
    if (substream.runtime.state == SNDRV_PCM_STATE_OPEN) {
    let mut rate: c_uint = params_rate(hw_params);
    let mut frames_per_period: c_uint = params_period_size(hw_params);
    let mut frames_per_buffer: c_uint = params_buffer_size(hw_params);
    guard(mutex)(&motu.mutex);
    err = snd_motu_stream_reserve_duplex(motu, rate,
    frames_per_period, frames_per_buffer);
    if (err >= 0)
    ++motu.substreams_counter;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    guard(mutex)(&motu.mutex);
    if (substream.runtime.state != SNDRV_PCM_STATE_OPEN)
    --motu.substreams_counter;
    snd_motu_stream_stop_duplex(motu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int capture_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    int err;
    scoped_guard(mutex, &motu.mutex) {
    err = snd_motu_stream_start_duplex(motu);
    }
    if (err >= 0)
    amdtp_stream_pcm_prepare(&motu.tx_stream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int playback_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    int err;
    scoped_guard(mutex, &motu.mutex) {
    err = snd_motu_stream_start_duplex(motu);
    }
    if (err >= 0)
    amdtp_stream_pcm_prepare(&motu.rx_stream);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int capture_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_motu *motu = substream.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(&motu.tx_stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(&motu.tx_stream, core::ptr::null_mut());
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int playback_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_motu *motu = substream.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(&motu.rx_stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(&motu.rx_stream, core::ptr::null_mut());
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t capture_pointer(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    return amdtp_domain_stream_pcm_pointer(&motu.domain, &motu.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t playback_pointer(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    return amdtp_domain_stream_pcm_pointer(&motu.domain, &motu.rx_stream);
    }
#[no_mangle]
unsafe extern "C" fn capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int capture_ack(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&motu.domain, &motu.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int playback_ack(struct snd_pcm_substream *substream)
    {
    struct snd_motu *motu = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&motu.domain, &motu.rx_stream);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_create_pcm_devices(motu: *mut snd_motu) -> c_int {
    int snd_motu_create_pcm_devices(struct snd_motu *motu)
    {
    static const struct snd_pcm_ops capture_ops = {
    .open      = pcm_open,
    .close     = pcm_close,
    .hw_params = pcm_hw_params,
    .hw_free   = pcm_hw_free,
    .prepare   = capture_prepare,
    .trigger   = capture_trigger,
    .pointer   = capture_pointer,
    .ack       = capture_ack,
    };
    static const struct snd_pcm_ops playback_ops = {
    .open      = pcm_open,
    .close     = pcm_close,
    .hw_params = pcm_hw_params,
    .hw_free   = pcm_hw_free,
    .prepare   = playback_prepare,
    .trigger   = playback_trigger,
    .pointer   = playback_pointer,
    .ack       = playback_ack,
    };
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(motu.card, motu.card.driver, 0, 1, 1, &pcm);
    if (err < 0)
    return err;
    pcm.private_data = motu;
    pcm.nonatomic = true;
    strscpy(pcm.name, motu.card.shortname);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &capture_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &playback_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);
    return 0;
    }
