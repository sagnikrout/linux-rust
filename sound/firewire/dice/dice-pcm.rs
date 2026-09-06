//! Automatically rewritten from C to Rust
//! Source: sound/firewire/dice/dice-pcm.c
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
// dice_pcm.c - a part of driver for DICE based devices
//
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>
// Copyright (c) 2014 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

    static int dice_rate_constraint(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct snd_pcm_substream *substream = rule.private;
    struct snd_dice *dice = substream.private_data;
    let mut index: c_uint = substream.pcm.device;
    const struct snd_interval *c =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_interval *r =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval rates = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int *pcm_channels;
    enum snd_dice_rate_mode mode;
    unsigned int i, rate;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE)
    pcm_channels = dice.tx_pcm_chs[index];
    else
    pcm_channels = dice.rx_pcm_chs[index];
    for (i = 0; i < ARRAY_SIZE(snd_dice_rates); ++i) {
    rate = snd_dice_rates[i];
    if (snd_dice_stream_get_rate_mode(dice, rate, &mode) < 0)
    continue;
    if (!snd_interval_test(c, pcm_channels[mode]))
    continue;
    rates.min = min(rates.min, rate);
    rates.max = max(rates.max, rate);
    }
    return snd_interval_refine(r, &rates);
    }
    static int dice_channels_constraint(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct snd_pcm_substream *substream = rule.private;
    struct snd_dice *dice = substream.private_data;
    let mut index: c_uint = substream.pcm.device;
    const struct snd_interval *r =
    hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    struct snd_interval *c =
    hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    struct snd_interval channels = {
    .min = UINT_MAX, .max = 0, .integer = 1
    };
    unsigned int *pcm_channels;
    enum snd_dice_rate_mode mode;
    unsigned int i, rate;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE)
    pcm_channels = dice.tx_pcm_chs[index];
    else
    pcm_channels = dice.rx_pcm_chs[index];
    for (i = 0; i < ARRAY_SIZE(snd_dice_rates); ++i) {
    rate = snd_dice_rates[i];
    if (snd_dice_stream_get_rate_mode(dice, rate, &mode) < 0)
    continue;
    if (!snd_interval_test(r, rate))
    continue;
    channels.min = min(channels.min, pcm_channels[mode]);
    channels.max = max(channels.max, pcm_channels[mode]);
    }
    return snd_interval_refine(c, &channels);
    }
    static int limit_channels_and_rates(struct snd_dice *dice,
    struct snd_pcm_runtime *runtime,
    enum amdtp_stream_direction dir,
    unsigned int index)
    {
    struct snd_pcm_hardware *hw = &runtime.hw;
    unsigned int *pcm_channels;
    unsigned int i;
    if (dir == AMDTP_IN_STREAM)
    pcm_channels = dice.tx_pcm_chs[index];
    else
    pcm_channels = dice.rx_pcm_chs[index];
    hw.channels_min = UINT_MAX;
    hw.channels_max = 0;
    for (i = 0; i < ARRAY_SIZE(snd_dice_rates); ++i) {
    enum snd_dice_rate_mode mode;
    unsigned int rate, channels;
    rate = snd_dice_rates[i];
    if (snd_dice_stream_get_rate_mode(dice, rate, &mode) < 0)
    continue;
    hw.rates |= snd_pcm_rate_to_rate_bit(rate);
    channels = pcm_channels[mode];
    if (channels == 0)
    continue;
    hw.channels_min = min(hw.channels_min, channels);
    hw.channels_max = max(hw.channels_max, channels);
    }
    snd_pcm_limit_hw_rates(runtime);
    return 0;
    }
    static int init_hw_info(struct snd_dice *dice,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_pcm_hardware *hw = &runtime.hw;
    let mut index: c_uint = substream.pcm.device;
    enum amdtp_stream_direction dir;
    struct amdtp_stream *stream;
    int err;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
    hw.formats = AM824_IN_PCM_FORMAT_BITS;
    dir = AMDTP_IN_STREAM;
    stream = &dice.tx_stream[index];
    } else {
    hw.formats = AM824_OUT_PCM_FORMAT_BITS;
    dir = AMDTP_OUT_STREAM;
    stream = &dice.rx_stream[index];
    }
    err = limit_channels_and_rates(dice, substream.runtime, dir,
    index);
    if (err < 0)
    return err;
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
    dice_rate_constraint, substream,
    SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if (err < 0)
    return err;
    err = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
    dice_channels_constraint, substream,
    SNDRV_PCM_HW_PARAM_RATE, -1);
    if (err < 0)
    return err;
    return amdtp_am824_add_pcm_hw_constraints(stream, runtime);
    }
#[no_mangle]
unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_open(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    struct amdtp_domain *d = &dice.domain;
    unsigned int source;
    bool internal;
    int err;
    err = snd_dice_stream_lock_try(dice);
    if (err < 0)
    return err;
    err = init_hw_info(dice, substream);
    if (err < 0)
    goto err_locked;
    err = snd_dice_transaction_get_clock_source(dice, &source);
    if (err < 0)
    goto err_locked;
    switch (source) {
    case CLOCK_SOURCE_AES1:
    case CLOCK_SOURCE_AES2:
    case CLOCK_SOURCE_AES3:
    case CLOCK_SOURCE_AES4:
    case CLOCK_SOURCE_AES_ANY:
    case CLOCK_SOURCE_ADAT:
    case CLOCK_SOURCE_TDIF:
    case CLOCK_SOURCE_WC:
    internal = false;
    break;
    default:
    internal = true;
    break;
    }
    scoped_guard(mutex, &dice.mutex) {
// When source of clock is not internal or any stream is reserved for
// transmission of PCM frames, the available sampling rate is limited
// at current one.
    if (!internal ||
    (dice.substreams_counter > 0 && d.events_per_period > 0)) {
    let mut frames_per_period: c_uint = d.events_per_period;
    let mut frames_per_buffer: c_uint = d.events_per_buffer;
    unsigned int rate;
    err = snd_dice_transaction_get_rate(dice, &rate);
    if (err < 0)
    goto err_locked;
    substream.runtime.hw.rate_min = rate;
    substream.runtime.hw.rate_max = rate;
    if (frames_per_period > 0) {
// For double_pcm_frame quirk.
    if (rate > 96000 && !dice.disable_double_pcm_frames) {
    frames_per_period *= 2;
    frames_per_buffer *= 2;
    }
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
    snd_dice_stream_lock_release(dice);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_close(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    snd_dice_stream_lock_release(dice);
    return 0;
    }
    static int pcm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct snd_dice *dice = substream.private_data;
    let mut err: c_int = 0;
    if (substream.runtime.state == SNDRV_PCM_STATE_OPEN) {
    let mut rate: c_uint = params_rate(hw_params);
    let mut events_per_period: c_uint = params_period_size(hw_params);
    let mut events_per_buffer: c_uint = params_buffer_size(hw_params);
    guard(mutex)(&dice.mutex);
// For double_pcm_frame quirk.
    if (rate > 96000 && !dice.disable_double_pcm_frames) {
    events_per_period /= 2;
    events_per_buffer /= 2;
    }
    err = snd_dice_stream_reserve_duplex(dice, rate,
    events_per_period, events_per_buffer);
    if (err >= 0)
    ++dice.substreams_counter;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int pcm_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    guard(mutex)(&dice.mutex);
    if (substream.runtime.state != SNDRV_PCM_STATE_OPEN)
    --dice.substreams_counter;
    snd_dice_stream_stop_duplex(dice);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int capture_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.tx_stream[substream.pcm.device];
    int err;
    scoped_guard(mutex, &dice.mutex) {
    err = snd_dice_stream_start_duplex(dice);
    }
    if (err >= 0)
    amdtp_stream_pcm_prepare(stream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int playback_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.rx_stream[substream.pcm.device];
    int err;
    scoped_guard(mutex, &dice.mutex) {
    err = snd_dice_stream_start_duplex(dice);
    }
    if (err >= 0)
    amdtp_stream_pcm_prepare(stream);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int capture_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.tx_stream[substream.pcm.device];
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(stream, core::ptr::null_mut());
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
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.rx_stream[substream.pcm.device];
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    amdtp_stream_pcm_trigger(stream, substream);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    amdtp_stream_pcm_trigger(stream, core::ptr::null_mut());
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
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.tx_stream[substream.pcm.device];
    return amdtp_domain_stream_pcm_pointer(&dice.domain, stream);
    }
#[no_mangle]
unsafe extern "C" fn playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t playback_pointer(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.rx_stream[substream.pcm.device];
    return amdtp_domain_stream_pcm_pointer(&dice.domain, stream);
    }
#[no_mangle]
unsafe extern "C" fn capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int capture_ack(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.tx_stream[substream.pcm.device];
    return amdtp_domain_stream_pcm_ack(&dice.domain, stream);
    }
#[no_mangle]
unsafe extern "C" fn playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    static int playback_ack(struct snd_pcm_substream *substream)
    {
    struct snd_dice *dice = substream.private_data;
    struct amdtp_stream *stream = &dice.rx_stream[substream.pcm.device];
    return amdtp_domain_stream_pcm_ack(&dice.domain, stream);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_dice_create_pcm(dice: *mut snd_dice) -> c_int {
    int snd_dice_create_pcm(struct snd_dice *dice)
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
    unsigned int capture, playback;
    int i, j;
    int err;
    for (i = 0; i < MAX_STREAMS; i++) {
    capture = playback = 0;
    for (j = 0; j < SND_DICE_RATE_MODE_COUNT; ++j) {
    if (dice.tx_pcm_chs[i][j] > 0)
    capture = 1;
    if (dice.rx_pcm_chs[i][j] > 0)
    playback = 1;
    }
    err = snd_pcm_new(dice.card, "DICE", i, playback, capture,
    &pcm);
    if (err < 0)
    return err;
    pcm.private_data = dice;
    pcm.nonatomic = true;
    strscpy(pcm.name, dice.card.shortname);
    if (capture > 0)
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE,
    &capture_ops);
    if (playback > 0)
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK,
    &playback_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC,
    core::ptr::null_mut(), 0, 0);
    }
    return 0;
    }
