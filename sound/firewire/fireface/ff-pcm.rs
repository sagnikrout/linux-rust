//! Automatically rewritten from C to Rust
//! Source: sound/firewire/fireface/ff-pcm.c
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
// ff-pcm.c - a part of driver for RME Fireface series
//
// Copyright (c) 2015-2017 Takashi Sakamoto
//

    static int hw_rule_rate(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    const unsigned int *pcm_channels = rule.private;
    struct snd_interval *r =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    const struct snd_interval *c =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_interval t = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(amdtp_rate_table); i++) {
    enum snd_ff_stream_mode mode;
    int err;
    err = snd_ff_stream_get_multiplier_mode(i, &mode);
    if (err < 0)
    continue;
    if (!snd_interval_test(c, pcm_channels[mode]))
    continue;
    t.min = min(t.min, amdtp_rate_table[i]);
    t.max = max(t.max, amdtp_rate_table[i]);
    }
    return snd_interval_refine(r, &t);
    }
    static int hw_rule_channels(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    const unsigned int *pcm_channels = rule.private;
    struct snd_interval *c =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    const struct snd_interval *r =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval t = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(amdtp_rate_table); i++) {
    enum snd_ff_stream_mode mode;
    int err;
    err = snd_ff_stream_get_multiplier_mode(i, &mode);
    if (err < 0)
    continue;
    if (!snd_interval_test(r, amdtp_rate_table[i]))
    continue;
    t.min = min(t.min, pcm_channels[mode]);
    t.max = max(t.max, pcm_channels[mode]);
    }
    return snd_interval_refine(c, &t);
    }
    static void limit_channels_and_rates(struct snd_pcm_hardware *hw,
    const unsigned int *pcm_channels)
    {
    unsigned int rate, channels;
    int i;
    hw.channels_min = UINT_MAX;
    hw.channels_max = 0;
    hw.rate_min = UINT_MAX;
    hw.rate_max = 0;
    for (i = 0; i < ARRAY_SIZE(amdtp_rate_table); i++) {
    enum snd_ff_stream_mode mode;
    int err;
    err = snd_ff_stream_get_multiplier_mode(i, &mode);
    if (err < 0)
    continue;
    channels = pcm_channels[mode];
    if (pcm_channels[mode] == 0)
    continue;
    hw.channels_min = min(hw.channels_min, channels);
    hw.channels_max = max(hw.channels_max, channels);
    rate = amdtp_rate_table[i];
    hw.rates |= snd_pcm_rate_to_rate_bit(rate);
    hw.rate_min = min(hw.rate_min, rate);
    hw.rate_max = max(hw.rate_max, rate);
    }
    }
    static int pcm_init_hw_params(struct snd_ff *ff,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct amdtp_stream *s;
    const unsigned int *pcm_channels;
    int err;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    runtime.hw.formats = SNDRV_PCM_FMTBIT_S32;
    s = &ff.tx_stream;
    pcm_channels = ff.spec.pcm_capture_channels;
    } else {
    runtime.hw.formats = SNDRV_PCM_FMTBIT_S32;
    s = &ff.rx_stream;
    pcm_channels = ff.spec.pcm_playback_channels;
    }
    limit_channels_and_rates(&runtime.hw, pcm_channels);
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
    hw_rule_channels, (void *)pcm_channels,
    SNDRV_PCM_HW_PARAM_RATE, -1);
    if (err < 0)
    return err;
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    hw_rule_rate, (void *)pcm_channels,
    SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if (err < 0)
    return err;
    return amdtp_ff_add_pcm_hw_constraints(s, runtime);
    }
#[no_mangle]
unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_open(struct snd_pcm_substream *substream)
    {
    struct snd_ff *ff = substream.private_data;
    struct amdtp_domain *d = &ff.domain;
    unsigned int rate;
    enum snd_ff_clock_src src;
    int i, err;
    err = snd_ff_stream_lock_try(ff);
    if (err < 0)
    return err;
    err = pcm_init_hw_params(ff, substream);
    if (err < 0)
    goto release_lock;
    err = ff.spec.protocol.get_clock(ff, &rate, &src);
    if (err < 0)
    goto release_lock;
    scoped_guard(mutex, &ff.mutex) {
// When source of clock is not internal or any stream is reserved for
// transmission of PCM frames, the available sampling rate is limited
// at current one.
    if (src != SND_FF_CLOCK_SRC_INTERNAL) {
    for (i = 0; i < CIP_SFC_COUNT; ++i) {
    if (amdtp_rate_table[i] == rate)
    break;
    }
// The unit is configured at sampling frequency which packet
// streaming engine can't support.
    if (i >= CIP_SFC_COUNT) {
    err = -EIO;
    goto release_lock;
    }
    substream.runtime.hw.rate_min = rate;
    substream.runtime.hw.rate_max = rate;
    } else {
    if (ff.substreams_counter > 0) {
    let mut frames_per_period: c_uint = d.events_per_period;
    let mut frames_per_buffer: c_uint = d.events_per_buffer;
    rate = amdtp_rate_table[ff.rx_stream.sfc];
    substream.runtime.hw.rate_min = rate;
    substream.runtime.hw.rate_max = rate;
    err = snd_pcm_hw_constraint_minmax(substream.runtime,
    SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
    frames_per_period, frames_per_period);
    if (err < 0)
    goto release_lock;
    err = snd_pcm_hw_constraint_minmax(substream.runtime,
    SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
    frames_per_buffer, frames_per_buffer);
    if (err < 0)
    goto release_lock;
    }
    }
    }
    snd_pcm_set_sync(substream);
    return 0;
    release_lock:
    snd_ff_stream_lock_release(ff);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_close(struct snd_pcm_substream *substream)
    {
    struct snd_ff *ff = substream.private_data;
    snd_ff_stream_lock_release(ff);
    return 0;
    }
    static int pcm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_ff *ff = substream.private_data;
    let mut err: c_int = 0;
    if (substream.runtime.state == SNDRV_PCM_STATE_OPEN) {
    let mut rate: c_uint = params_rate(hw_params);
    let mut frames_per_period: c_uint = params_period_size(hw_params);
    let mut frames_per_buffer: c_uint = params_buffer_size(hw_params);
    guard(mutex)(&ff.mutex);
    err = snd_ff_stream_reserve_duplex(ff, rate, frames_per_period,
    frames_per_buffer);
    if (err >= 0)
    ++ff.substreams_counter;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_ff *ff = substream.private_data;
    guard(mutex)(&ff.mutex);
    if (substream.runtime.state != SNDRV_PCM_STATE_OPEN)
    --ff.substreams_counter;
    snd_ff_stream_stop_duplex(ff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_ff *ff = substream.private_data;
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    guard(mutex)(&ff.mutex);
    err = snd_ff_stream_start_duplex(ff, runtime.rate);
    if (err >= 0)
    amdtp_stream_pcm_prepare(&ff.tx_stream);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_ff *ff = substream.private_data;
    struct snd_pcm_runtime *runtime = substream.runtime;
    int err;
    guard(mutex)(&ff.mutex);
    err = snd_ff_stream_start_duplex(ff, runtime.rate);
    if (err >= 0)
    amdtp_stream_pcm_prepare(&ff.rx_stream);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int pcm_capture_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_ff *ff = substream.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(&ff.tx_stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(&ff.tx_stream, core::ptr::null_mut());
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int pcm_playback_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_ff *ff = substream.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(&ff.rx_stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(&ff.rx_stream, core::ptr::null_mut());
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_pointer(sbstrm: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t pcm_capture_pointer(struct snd_pcm_substream *sbstrm)
    {
    struct snd_ff *ff = sbstrm.private_data;
    return amdtp_domain_stream_pcm_pointer(&ff.domain, &ff.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_pointer(sbstrm: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t pcm_playback_pointer(struct snd_pcm_substream *sbstrm)
    {
    struct snd_ff *ff = sbstrm.private_data;
    return amdtp_domain_stream_pcm_pointer(&ff.domain, &ff.rx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_capture_ack(struct snd_pcm_substream *substream)
    {
    struct snd_ff *ff = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&ff.domain, &ff.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_playback_ack(struct snd_pcm_substream *substream)
    {
    struct snd_ff *ff = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&ff.domain, &ff.rx_stream);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_ff_create_pcm_devices(ff: *mut snd_ff) -> c_int {
    int snd_ff_create_pcm_devices(struct snd_ff *ff)
    {
    static const struct snd_pcm_ops pcm_capture_ops = {
    .open		= pcm_open,
    .close		= pcm_close,
    .hw_params	= pcm_hw_params,
    .hw_free	= pcm_hw_free,
    .prepare	= pcm_capture_prepare,
    .trigger	= pcm_capture_trigger,
    .pointer	= pcm_capture_pointer,
    .ack		= pcm_capture_ack,
    };
    static const struct snd_pcm_ops pcm_playback_ops = {
    .open		= pcm_open,
    .close		= pcm_close,
    .hw_params	= pcm_hw_params,
    .hw_free	= pcm_hw_free,
    .prepare	= pcm_playback_prepare,
    .trigger	= pcm_playback_trigger,
    .pointer	= pcm_playback_pointer,
    .ack		= pcm_playback_ack,
    };
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(ff.card, ff.card.driver, 0, 1, 1, &pcm);
    if (err < 0)
    return err;
    pcm.private_data = ff;
    pcm.nonatomic = true;
    snprintf(pcm.name, sizeof(pcm.name),
    "%s PCM", ff.card.shortname);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &pcm_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &pcm_capture_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);
    return 0;
    }
