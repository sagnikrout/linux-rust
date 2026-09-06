//! Automatically rewritten from C to Rust
//! Source: sound/firewire/tascam/tascam-stream.c
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
// tascam-stream.c - a part of driver for TASCAM FireWire series
//
// Copyright (c) 2015 Takashi Sakamoto
//

pub const CLOCK_STATUS_MASK: c_uint = 0xffff0000;
pub const CLOCK_CONFIG_MASK: c_uint = 0x0000ffff;
pub const READY_TIMEOUT_MS: c_int = 4000;
#[no_mangle]
unsafe extern "C" fn get_clock(tscm: *mut snd_tscm, data: *mut u32) -> c_int {
    static int get_clock(struct snd_tscm *tscm, u32 *data)
    {
    let mut trial: c_int = 0;
    __be32 reg;
    int err;
    while (trial++ < 5) {
    err = snd_fw_transaction(tscm.unit, TCODE_READ_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_CLOCK_STATUS,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// data = be32_to_cpu(reg);
    if (*data & CLOCK_STATUS_MASK)
    break;
// In intermediate state after changing clock status.
    msleep(50);
    }
// Still in the intermediate state.
    if (trial >= 5)
    return -EAGAIN;
    return 0;
    }
    static int set_clock(struct snd_tscm *tscm, unsigned int rate,
    enum snd_tscm_clock clock)
    {
    u32 data;
    __be32 reg;
    int err;
    err = get_clock(tscm, &data);
    if (err < 0)
    return err;
    data &= CLOCK_CONFIG_MASK;
    if (rate > 0) {
    data &= 0x000000ff;
// Base rate.
    if ((rate % 44100) == 0) {
    data |= 0x00000100;
// Multiplier.
    if (rate / 44100 == 2)
    data |= 0x00008000;
    } else if ((rate % 48000) == 0) {
    data |= 0x00000200;
// Multiplier.
    if (rate / 48000 == 2)
    data |= 0x00008000;
    } else {
    return -EAGAIN;
    }
    }
    if (clock != INT_MAX) {
    data &= 0x0000ff00;
    data |= clock + 1;
    }
    reg = cpu_to_be32(data);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_CLOCK_STATUS,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    if (data & 0x00008000)
    reg = cpu_to_be32(0x0000001a);
    else
    reg = cpu_to_be32(0x0000000d);
    return snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_MULTIPLEX_MODE,
    &reg, sizeof(reg), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_get_rate(tscm: *mut snd_tscm, rate: *mut c_uint) -> c_int {
    int snd_tscm_stream_get_rate(struct snd_tscm *tscm, unsigned int *rate)
    {
    u32 data;
    int err;
    err = get_clock(tscm, &data);
    if (err < 0)
    return err;
    data = (data & 0xff000000) >> 24;
// Check base rate.
    if ((data & 0x0f) == 0x01)
// rate = 44100;
#[no_mangle]
pub unsafe extern "C" fn if(0x02: (data & 0x0f) ==) -> else {
    else if ((data & 0x0f) == 0x02)
// rate = 48000;
    else
    return -EAGAIN;
// Check multiplier.
    if ((data & 0xf0) == 0x80)
// rate *= 2;
#[no_mangle]
pub unsafe extern "C" fn if(0x00: (data & 0xf0) !=) -> else {
    else if ((data & 0xf0) != 0x00)
    return -EAGAIN;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_get_clock(tscm: *mut snd_tscm, clock: *mut enum snd_tscm_clock) -> c_int {
    int snd_tscm_stream_get_clock(struct snd_tscm *tscm, enum snd_tscm_clock *clock)
    {
    u32 data;
    int err;
    err = get_clock(tscm, &data);
    if (err < 0)
    return err;
// clock = ((data & 0x00ff0000) >> 16) - 1;
    if (*clock < 0 || *clock > SND_TSCM_CLOCK_ADAT)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_data_channels(tscm: *mut snd_tscm) -> c_int {
    static int enable_data_channels(struct snd_tscm *tscm)
    {
    __be32 reg;
    u32 data;
    unsigned int i;
    int err;
    data = 0;
    for (i = 0; i < tscm.spec.pcm_capture_analog_channels; ++i)
    data |= BIT(i);
    if (tscm.spec.has_adat)
    data |= 0x0000ff00;
    if (tscm.spec.has_spdif)
    data |= 0x00030000;
    reg = cpu_to_be32(data);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_TX_PCM_CHANNELS,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    data = 0;
    for (i = 0; i < tscm.spec.pcm_playback_analog_channels; ++i)
    data |= BIT(i);
    if (tscm.spec.has_adat)
    data |= 0x0000ff00;
    if (tscm.spec.has_spdif)
    data |= 0x00030000;
    reg = cpu_to_be32(data);
    return snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_RX_PCM_CHANNELS,
    &reg, sizeof(reg), 0);
    }
#[no_mangle]
unsafe extern "C" fn set_stream_formats(tscm: *mut snd_tscm, rate: c_uint) -> c_int {
    static int set_stream_formats(struct snd_tscm *tscm, unsigned int rate)
    {
    __be32 reg;
    int err;
// Set an option for unknown purpose.
    reg = cpu_to_be32(0x00200000);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_SET_OPTION,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    return enable_data_channels(tscm);
    }
#[no_mangle]
unsafe extern "C" fn finish_session(tscm: *mut snd_tscm) {
    static void finish_session(struct snd_tscm *tscm)
    {
    __be32 reg;
    reg = 0;
    snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_START_STREAMING,
    &reg, sizeof(reg), 0);
    reg = 0;
    snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_ON,
    &reg, sizeof(reg), 0);
// Unregister channels.
    reg = cpu_to_be32(0x00000000);
    snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_TX_CH,
    &reg, sizeof(reg), 0);
    reg = cpu_to_be32(0x00000000);
    snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_UNKNOWN,
    &reg, sizeof(reg), 0);
    reg = cpu_to_be32(0x00000000);
    snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_CH,
    &reg, sizeof(reg), 0);
    }
#[no_mangle]
unsafe extern "C" fn begin_session(tscm: *mut snd_tscm) -> c_int {
    static int begin_session(struct snd_tscm *tscm)
    {
    __be32 reg;
    int err;
// Register the isochronous channel for transmitting stream.
    reg = cpu_to_be32(tscm.tx_resources.channel);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_TX_CH,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// Unknown.
    reg = cpu_to_be32(0x00000002);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_UNKNOWN,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// Register the isochronous channel for receiving stream.
    reg = cpu_to_be32(tscm.rx_resources.channel);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_CH,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    reg = cpu_to_be32(0x00000001);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_START_STREAMING,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
    reg = cpu_to_be32(0x00000001);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_RX_ON,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// Set an option for unknown purpose.
    reg = cpu_to_be32(0x00002000);
    err = snd_fw_transaction(tscm.unit, TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_SET_OPTION,
    &reg, sizeof(reg), 0);
    if (err < 0)
    return err;
// Start multiplexing PCM samples on packets.
    reg = cpu_to_be32(0x00000001);
    return snd_fw_transaction(tscm.unit,
    TCODE_WRITE_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_ISOC_TX_ON,
    &reg, sizeof(reg), 0);
    }
    static int keep_resources(struct snd_tscm *tscm, unsigned int rate,
    struct amdtp_stream *stream)
    {
    struct fw_iso_resources *resources;
    int speed;
    int err;
    if (stream == &tscm.tx_stream) {
    resources = &tscm.tx_resources;
    speed = fw_parent_device(tscm.unit).max_speed;
    } else {
    resources = &tscm.rx_resources;
    speed = SCODE_400;
    }
    err = amdtp_tscm_set_parameters(stream, rate);
    if (err < 0)
    return err;
    return fw_iso_resources_allocate(resources, amdtp_stream_get_max_payload(stream), speed);
    }
#[no_mangle]
unsafe extern "C" fn init_stream(tscm: *mut snd_tscm, s: *mut amdtp_stream) -> c_int {
    static int init_stream(struct snd_tscm *tscm, struct amdtp_stream *s)
    {
    struct fw_iso_resources *resources;
    enum amdtp_stream_direction dir;
    unsigned int pcm_channels;
    int err;
    if (s == &tscm.tx_stream) {
    resources = &tscm.tx_resources;
    dir = AMDTP_IN_STREAM;
    pcm_channels = tscm.spec.pcm_capture_analog_channels;
    } else {
    resources = &tscm.rx_resources;
    dir = AMDTP_OUT_STREAM;
    pcm_channels = tscm.spec.pcm_playback_analog_channels;
    }
    if (tscm.spec.has_adat)
    pcm_channels += 8;
    if (tscm.spec.has_spdif)
    pcm_channels += 2;
    err = fw_iso_resources_init(resources, tscm.unit);
    if (err < 0)
    return err;
    err = amdtp_tscm_init(s, tscm.unit, dir, pcm_channels);
    if (err < 0)
    fw_iso_resources_free(resources);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn destroy_stream(tscm: *mut snd_tscm, s: *mut amdtp_stream) {
    static void destroy_stream(struct snd_tscm *tscm, struct amdtp_stream *s)
    {
    amdtp_stream_destroy(s);
    if (s == &tscm.tx_stream)
    fw_iso_resources_destroy(&tscm.tx_resources);
    else
    fw_iso_resources_destroy(&tscm.rx_resources);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_init_duplex(tscm: *mut snd_tscm) -> c_int {
    int snd_tscm_stream_init_duplex(struct snd_tscm *tscm)
    {
    int err;
    err = init_stream(tscm, &tscm.tx_stream);
    if (err < 0)
    return err;
    err = init_stream(tscm, &tscm.rx_stream);
    if (err < 0) {
    destroy_stream(tscm, &tscm.tx_stream);
    return err;
    }
    err = amdtp_domain_init(&tscm.domain);
    if (err < 0) {
    destroy_stream(tscm, &tscm.tx_stream);
    destroy_stream(tscm, &tscm.rx_stream);
    }
    return err;
    }
// At bus reset, streaming is stopped and some registers are clear.
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_update_duplex(tscm: *mut snd_tscm) {
    void snd_tscm_stream_update_duplex(struct snd_tscm *tscm)
    {
    amdtp_domain_stop(&tscm.domain);
    amdtp_stream_pcm_abort(&tscm.tx_stream);
    amdtp_stream_pcm_abort(&tscm.rx_stream);
    }
// This function should be called before starting streams or after stopping
// streams.
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_destroy_duplex(tscm: *mut snd_tscm) {
    void snd_tscm_stream_destroy_duplex(struct snd_tscm *tscm)
    {
    amdtp_domain_destroy(&tscm.domain);
    destroy_stream(tscm, &tscm.rx_stream);
    destroy_stream(tscm, &tscm.tx_stream);
    }
    int snd_tscm_stream_reserve_duplex(struct snd_tscm *tscm, unsigned int rate,
    unsigned int frames_per_period,
    unsigned int frames_per_buffer)
    {
    unsigned int curr_rate;
    int err;
    err = snd_tscm_stream_get_rate(tscm, &curr_rate);
    if (err < 0)
    return err;
    if (tscm.substreams_counter == 0 || rate != curr_rate) {
    amdtp_domain_stop(&tscm.domain);
    finish_session(tscm);
    fw_iso_resources_free(&tscm.tx_resources);
    fw_iso_resources_free(&tscm.rx_resources);
    err = set_clock(tscm, rate, INT_MAX);
    if (err < 0)
    return err;
    err = keep_resources(tscm, rate, &tscm.tx_stream);
    if (err < 0)
    return err;
    err = keep_resources(tscm, rate, &tscm.rx_stream);
    if (err < 0) {
    fw_iso_resources_free(&tscm.tx_resources);
    return err;
    }
    err = amdtp_domain_set_events_per_period(&tscm.domain,
    frames_per_period, frames_per_buffer);
    if (err < 0) {
    fw_iso_resources_free(&tscm.tx_resources);
    fw_iso_resources_free(&tscm.rx_resources);
    return err;
    }
    tscm.need_long_tx_init_skip = (rate != curr_rate);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_start_duplex(tscm: *mut snd_tscm, rate: c_uint) -> c_int {
    int snd_tscm_stream_start_duplex(struct snd_tscm *tscm, unsigned int rate)
    {
    let mut generation: c_uint = tscm.rx_resources.generation;
    int err;
    if (tscm.substreams_counter == 0)
    return 0;
    if (amdtp_streaming_error(&tscm.rx_stream) ||
    amdtp_streaming_error(&tscm.tx_stream)) {
    amdtp_domain_stop(&tscm.domain);
    finish_session(tscm);
    }
    if (generation != fw_parent_device(tscm.unit).card.generation) {
    err = fw_iso_resources_update(&tscm.tx_resources);
    if (err < 0)
    goto error;
    err = fw_iso_resources_update(&tscm.rx_resources);
    if (err < 0)
    goto error;
    }
    if (!amdtp_stream_running(&tscm.rx_stream)) {
    unsigned int tx_init_skip_cycles;
    err = set_stream_formats(tscm, rate);
    if (err < 0)
    goto error;
    err = begin_session(tscm);
    if (err < 0)
    goto error;
    err = amdtp_domain_add_stream(&tscm.domain, &tscm.rx_stream, tscm.rx_resources.channel,
    fw_parent_device(tscm.unit).max_speed);
    if (err < 0)
    goto error;
    err = amdtp_domain_add_stream(&tscm.domain, &tscm.tx_stream, tscm.tx_resources.channel,
    SCODE_400);
    if (err < 0)
    goto error;
    if (tscm.need_long_tx_init_skip)
    tx_init_skip_cycles = 16000;
    else
    tx_init_skip_cycles = 0;
// MEMO: Just after starting packet streaming, it transfers packets without any
// event. Enough after receiving the sequence of packets, it multiplexes events into
// the packet. However, just after changing sampling transfer frequency, it stops
// multiplexing during packet transmission. Enough after, it restarts multiplexing
// again. The device ignores presentation time expressed by the value of syt field
// of CIP header in received packets. The sequence of the number of data blocks per
// packet is important for media clock recovery.
    err = amdtp_domain_start(&tscm.domain, tx_init_skip_cycles, true, true);
    if (err < 0)
    goto error;
    if (!amdtp_domain_wait_ready(&tscm.domain, READY_TIMEOUT_MS)) {
    err = -ETIMEDOUT;
    goto error;
    }
    }
    return 0;
    error:
    amdtp_domain_stop(&tscm.domain);
    finish_session(tscm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_stop_duplex(tscm: *mut snd_tscm) {
    void snd_tscm_stream_stop_duplex(struct snd_tscm *tscm)
    {
    if (tscm.substreams_counter == 0) {
    amdtp_domain_stop(&tscm.domain);
    finish_session(tscm);
    fw_iso_resources_free(&tscm.tx_resources);
    fw_iso_resources_free(&tscm.rx_resources);
    tscm.need_long_tx_init_skip = false;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_lock_changed(tscm: *mut snd_tscm) {
    void snd_tscm_stream_lock_changed(struct snd_tscm *tscm)
    {
    tscm.dev_lock_changed = true;
    wake_up(&tscm.hwdep_wait);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_lock_try(tscm: *mut snd_tscm) -> c_int {
    int snd_tscm_stream_lock_try(struct snd_tscm *tscm)
    {
    guard(spinlock_irq)(&tscm.lock);
// user land lock this
    if (tscm.dev_lock_count < 0)
    return -EBUSY;
// this is the first time
    if (tscm.dev_lock_count++ == 0)
    snd_tscm_stream_lock_changed(tscm);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_stream_lock_release(tscm: *mut snd_tscm) {
    void snd_tscm_stream_lock_release(struct snd_tscm *tscm)
    {
    guard(spinlock_irq)(&tscm.lock);
    if (WARN_ON(tscm.dev_lock_count <= 0))
    return;
    if (--tscm.dev_lock_count == 0)
    snd_tscm_stream_lock_changed(tscm);
    }
