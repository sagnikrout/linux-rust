//! Automatically rewritten from C to Rust
//! Source: sound/firewire/bebob/bebob_pcm.c
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
// bebob_pcm.c - a part of driver for BeBoB based devices
//
// Copyright (c) 2013-2014 Takashi Sakamoto
//

    static int
    hw_rule_rate(struct snd_pcm_hw_params *params, struct snd_pcm_hw_rule *rule)
    {
    struct snd_bebob_stream_formation *formations = rule.private;
    struct snd_interval *r =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    const struct snd_interval *c =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_interval t = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int i;
    for (i = 0; i < SND_BEBOB_STRM_FMT_ENTRIES; i++) {
// entry is invalid
    if (formations[i].pcm == 0)
    continue;
    if (!snd_interval_test(c, formations[i].pcm))
    continue;
    t.min = min(t.min, snd_bebob_rate_table[i]);
    t.max = max(t.max, snd_bebob_rate_table[i]);
    }
    return snd_interval_refine(r, &t);
    }
    static int
    hw_rule_channels(struct snd_pcm_hw_params *params, struct snd_pcm_hw_rule *rule)
    {
    struct snd_bebob_stream_formation *formations = rule.private;
    struct snd_interval *c =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    const struct snd_interval *r =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval t = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int i;
    for (i = 0; i < SND_BEBOB_STRM_FMT_ENTRIES; i++) {
// entry is invalid
    if (formations[i].pcm == 0)
    continue;
    if (!snd_interval_test(r, snd_bebob_rate_table[i]))
    continue;
    t.min = min(t.min, formations[i].pcm);
    t.max = max(t.max, formations[i].pcm);
    }
    return snd_interval_refine(c, &t);
    }
    static void
    limit_channels_and_rates(struct snd_pcm_hardware *hw,
    struct snd_bebob_stream_formation *formations)
    {
    unsigned int i;
    hw.channels_min = UINT_MAX;
    hw.channels_max = 0;
    hw.rate_min = UINT_MAX;
    hw.rate_max = 0;
    hw.rates = 0;
    for (i = 0; i < SND_BEBOB_STRM_FMT_ENTRIES; i++) {
// entry has no PCM channels
    if (formations[i].pcm == 0)
    continue;
    hw.channels_min = min(hw.channels_min, formations[i].pcm);
    hw.channels_max = max(hw.channels_max, formations[i].pcm);
    hw.rate_min = min(hw.rate_min, snd_bebob_rate_table[i]);
    hw.rate_max = max(hw.rate_max, snd_bebob_rate_table[i]);
    hw.rates |= snd_pcm_rate_to_rate_bit(snd_bebob_rate_table[i]);
    }
    }
    static int
    pcm_init_hw_params(struct snd_bebob *bebob,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct amdtp_stream *s;
    struct snd_bebob_stream_formation *formations;
    int err;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    runtime.hw.formats = AM824_IN_PCM_FORMAT_BITS;
    s = &bebob.tx_stream;
    formations = bebob.tx_stream_formations;
    } else {
    runtime.hw.formats = AM824_OUT_PCM_FORMAT_BITS;
    s = &bebob.rx_stream;
    formations = bebob.rx_stream_formations;
    }
    limit_channels_and_rates(&runtime.hw, formations);
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
    hw_rule_channels, formations,
    SNDRV_PCM_HW_PARAM_RATE, -1);
    if (err < 0)
    goto end;
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    hw_rule_rate, formations,
    SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if (err < 0)
    goto end;
    err = amdtp_am824_add_pcm_hw_constraints(s, runtime);
    end:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_open(struct snd_pcm_substream *substream)
    {
    struct snd_bebob *bebob = substream.private_data;
    const struct snd_bebob_rate_spec *spec = bebob.spec.rate;
    struct amdtp_domain *d = &bebob.domain;
    enum snd_bebob_clock_type src;
    int err;
    err = snd_bebob_stream_lock_try(bebob);
    if (err < 0)
    return err;
    err = pcm_init_hw_params(bebob, substream);
    if (err < 0)
    goto err_locked;
    err = snd_bebob_stream_get_clock_src(bebob, &src);
    if (err < 0)
    goto err_locked;
    scoped_guard(mutex, &bebob.mutex) {
// When source of clock is not internal or any stream is reserved for
// transmission of PCM frames, the available sampling rate is limited
// at current one.
    if (src == SND_BEBOB_CLOCK_TYPE_EXTERNAL ||
    (bebob.substreams_counter > 0 && d.events_per_period > 0)) {
    let mut frames_per_period: c_uint = d.events_per_period;
    let mut frames_per_buffer: c_uint = d.events_per_buffer;
    unsigned int sampling_rate;
    err = spec.get(bebob, &sampling_rate);
    if (err < 0) {
    dev_err(&bebob.unit.device,
    "fail to get sampling rate: %d\n", err);
    goto err_locked;
    }
    substream.runtime.hw.rate_min = sampling_rate;
    substream.runtime.hw.rate_max = sampling_rate;
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
    snd_bebob_stream_lock_release(bebob);
    return err;
    }
    static int
    pcm_close(struct snd_pcm_substream *substream)
    {
    struct snd_bebob *bebob = substream.private_data;
    snd_bebob_stream_lock_release(bebob);
    return 0;
    }
    static int pcm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_bebob *bebob = substream.private_data;
    let mut err: c_int = 0;
    if (substream.runtime.state == SNDRV_PCM_STATE_OPEN) {
    let mut rate: c_uint = params_rate(hw_params);
    let mut frames_per_period: c_uint = params_period_size(hw_params);
    let mut frames_per_buffer: c_uint = params_buffer_size(hw_params);
    guard(mutex)(&bebob.mutex);
    err = snd_bebob_stream_reserve_duplex(bebob, rate,
    frames_per_period, frames_per_buffer);
    if (err >= 0)
    ++bebob.substreams_counter;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_bebob *bebob = substream.private_data;
    guard(mutex)(&bebob.mutex);
    if (substream.runtime.state != SNDRV_PCM_STATE_OPEN)
    bebob.substreams_counter--;
    snd_bebob_stream_stop_duplex(bebob);
    return 0;
    }
    static int
    pcm_capture_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_bebob *bebob = substream.private_data;
    int err;
    err = snd_bebob_stream_start_duplex(bebob);
    if (err >= 0)
    amdtp_stream_pcm_prepare(&bebob.tx_stream);
    return err;
    }
    static int
    pcm_playback_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_bebob *bebob = substream.private_data;
    int err;
    err = snd_bebob_stream_start_duplex(bebob);
    if (err >= 0)
    amdtp_stream_pcm_prepare(&bebob.rx_stream);
    return err;
    }
    static int
    pcm_capture_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_bebob *bebob = substream.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(&bebob.tx_stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(&bebob.tx_stream, core::ptr::null_mut());
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int
    pcm_playback_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_bebob *bebob = substream.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(&bebob.rx_stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(&bebob.rx_stream, core::ptr::null_mut());
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
    struct snd_bebob *bebob = sbstrm.private_data;
    return amdtp_domain_stream_pcm_pointer(&bebob.domain,
    &bebob.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_pointer(sbstrm: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t pcm_playback_pointer(struct snd_pcm_substream *sbstrm)
    {
    struct snd_bebob *bebob = sbstrm.private_data;
    return amdtp_domain_stream_pcm_pointer(&bebob.domain,
    &bebob.rx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_capture_ack(struct snd_pcm_substream *substream)
    {
    struct snd_bebob *bebob = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&bebob.domain, &bebob.tx_stream);
    }
#[no_mangle]
unsafe extern "C" fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_playback_ack(struct snd_pcm_substream *substream)
    {
    struct snd_bebob *bebob = substream.private_data;
    return amdtp_domain_stream_pcm_ack(&bebob.domain, &bebob.rx_stream);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_bebob_create_pcm_devices(bebob: *mut snd_bebob) -> c_int {
    int snd_bebob_create_pcm_devices(struct snd_bebob *bebob)
    {
    static const struct snd_pcm_ops capture_ops = {
    .open		= pcm_open,
    .close		= pcm_close,
    .hw_params	= pcm_hw_params,
    .hw_free	= pcm_hw_free,
    .prepare	= pcm_capture_prepare,
    .trigger	= pcm_capture_trigger,
    .pointer	= pcm_capture_pointer,
    .ack		= pcm_capture_ack,
    };
    static const struct snd_pcm_ops playback_ops = {
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
    err = snd_pcm_new(bebob.card, bebob.card.driver, 0, 1, 1, &pcm);
    if (err < 0)
    goto end;
    pcm.private_data = bebob;
    pcm.nonatomic = true;
    snprintf(pcm.name, sizeof(pcm.name),
    "%s PCM", bebob.card.shortname);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &capture_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);
    end:
    return err;
    }
