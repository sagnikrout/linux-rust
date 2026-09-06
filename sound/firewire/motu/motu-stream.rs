//! Automatically rewritten from C to Rust
//! Source: sound/firewire/motu/motu-stream.c
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
// motu-stream.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

pub const READY_TIMEOUT_MS: c_int = 200;
pub const ISOC_COMM_CONTROL_OFFSET: c_uint = 0x0b00;
pub const ISOC_COMM_CONTROL_MASK: c_uint = 0xffff0000;
pub const CHANGE_RX_ISOC_COMM_STATE: c_uint = 0x80000000;
pub const RX_ISOC_COMM_IS_ACTIVATED: c_uint = 0x40000000;
pub const RX_ISOC_COMM_CHANNEL_MASK: c_uint = 0x3f000000;
pub const RX_ISOC_COMM_CHANNEL_SHIFT: c_int = 24;
pub const CHANGE_TX_ISOC_COMM_STATE: c_uint = 0x00800000;
pub const TX_ISOC_COMM_IS_ACTIVATED: c_uint = 0x00400000;
pub const TX_ISOC_COMM_CHANNEL_MASK: c_uint = 0x003f0000;
pub const TX_ISOC_COMM_CHANNEL_SHIFT: c_int = 16;
pub const PACKET_FORMAT_OFFSET: c_uint = 0x0b10;
pub const TX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS: c_uint = 0x00000080;
pub const RX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS: c_uint = 0x00000040;
pub const TX_PACKET_TRANSMISSION_SPEED_MASK: c_uint = 0x0000000f;
    static int keep_resources(struct snd_motu *motu, unsigned int rate,
    struct amdtp_stream *stream)
    {
    struct fw_iso_resources *resources;
    struct snd_motu_packet_format *packet_format;
    let mut midi_ports: c_uint = 0;
    int err;
    if (stream == &motu.rx_stream) {
    resources = &motu.rx_resources;
    packet_format = &motu.rx_packet_formats;
    if ((motu.spec.flags & SND_MOTU_SPEC_RX_MIDI_2ND_Q) ||
    (motu.spec.flags & SND_MOTU_SPEC_RX_MIDI_3RD_Q))
    midi_ports = 1;
    } else {
    resources = &motu.tx_resources;
    packet_format = &motu.tx_packet_formats;
    if ((motu.spec.flags & SND_MOTU_SPEC_TX_MIDI_2ND_Q) ||
    (motu.spec.flags & SND_MOTU_SPEC_TX_MIDI_3RD_Q))
    midi_ports = 1;
    }
    err = amdtp_motu_set_parameters(stream, rate, midi_ports,
    packet_format);
    if (err < 0)
    return err;
    return fw_iso_resources_allocate(resources,
    amdtp_stream_get_max_payload(stream),
    fw_parent_device(motu.unit).max_speed);
    }
#[no_mangle]
unsafe extern "C" fn begin_session(motu: *mut snd_motu) -> c_int {
    static int begin_session(struct snd_motu *motu)
    {
    __be32 reg;
    u32 data;
    int err;
// Configure the unit to start isochronous communication.
    err = snd_motu_transaction_read(motu, ISOC_COMM_CONTROL_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg) & ~ISOC_COMM_CONTROL_MASK;
    data |= CHANGE_RX_ISOC_COMM_STATE | RX_ISOC_COMM_IS_ACTIVATED |
    (motu.rx_resources.channel << RX_ISOC_COMM_CHANNEL_SHIFT) |
    CHANGE_TX_ISOC_COMM_STATE | TX_ISOC_COMM_IS_ACTIVATED |
    (motu.tx_resources.channel << TX_ISOC_COMM_CHANNEL_SHIFT);
    reg = cpu_to_be32(data);
    return snd_motu_transaction_write(motu, ISOC_COMM_CONTROL_OFFSET, &reg,
    sizeof(reg));
    }
#[no_mangle]
unsafe extern "C" fn finish_session(motu: *mut snd_motu) {
    static void finish_session(struct snd_motu *motu)
    {
    __be32 reg;
    u32 data;
    int err;
    err = snd_motu_protocol_switch_fetching_mode(motu, false);
    if (err < 0)
    return;
    err = snd_motu_transaction_read(motu, ISOC_COMM_CONTROL_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return;
    data = be32_to_cpu(reg);
    data &= ~(RX_ISOC_COMM_IS_ACTIVATED | TX_ISOC_COMM_IS_ACTIVATED);
    data |= CHANGE_RX_ISOC_COMM_STATE | CHANGE_TX_ISOC_COMM_STATE;
    reg = cpu_to_be32(data);
    snd_motu_transaction_write(motu, ISOC_COMM_CONTROL_OFFSET, &reg,
    sizeof(reg));
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_stream_cache_packet_formats(motu: *mut snd_motu) -> c_int {
    int snd_motu_stream_cache_packet_formats(struct snd_motu *motu)
    {
    int err;
    err = snd_motu_protocol_cache_packet_formats(motu);
    if (err < 0)
    return err;
    if (motu.spec.flags & SND_MOTU_SPEC_TX_MIDI_2ND_Q) {
    motu.tx_packet_formats.midi_flag_offset = 4;
    motu.tx_packet_formats.midi_byte_offset = 6;
    } else if (motu.spec.flags & SND_MOTU_SPEC_TX_MIDI_3RD_Q) {
    motu.tx_packet_formats.midi_flag_offset = 8;
    motu.tx_packet_formats.midi_byte_offset = 7;
    }
    if (motu.spec.flags & SND_MOTU_SPEC_RX_MIDI_2ND_Q) {
    motu.rx_packet_formats.midi_flag_offset = 4;
    motu.rx_packet_formats.midi_byte_offset = 6;
    } else if (motu.spec.flags & SND_MOTU_SPEC_RX_MIDI_3RD_Q) {
    motu.rx_packet_formats.midi_flag_offset = 8;
    motu.rx_packet_formats.midi_byte_offset = 7;
    }
    return 0;
    }
    int snd_motu_stream_reserve_duplex(struct snd_motu *motu, unsigned int rate,
    unsigned int frames_per_period,
    unsigned int frames_per_buffer)
    {
    unsigned int curr_rate;
    int err;
    err = snd_motu_protocol_get_clock_rate(motu, &curr_rate);
    if (err < 0)
    return err;
    if (rate == 0)
    rate = curr_rate;
    if (motu.substreams_counter == 0 || curr_rate != rate) {
    amdtp_domain_stop(&motu.domain);
    finish_session(motu);
    fw_iso_resources_free(&motu.tx_resources);
    fw_iso_resources_free(&motu.rx_resources);
    kfree(motu.cache.event_offsets);
    motu.cache.event_offsets = core::ptr::null_mut();
    err = snd_motu_protocol_set_clock_rate(motu, rate);
    if (err < 0) {
    dev_err(&motu.unit.device,
    "fail to set sampling rate: %d\n", err);
    return err;
    }
    err = snd_motu_stream_cache_packet_formats(motu);
    if (err < 0)
    return err;
    err = keep_resources(motu, rate, &motu.tx_stream);
    if (err < 0)
    return err;
    err = keep_resources(motu, rate, &motu.rx_stream);
    if (err < 0) {
    fw_iso_resources_free(&motu.tx_resources);
    return err;
    }
    err = amdtp_domain_set_events_per_period(&motu.domain,
    frames_per_period, frames_per_buffer);
    if (err < 0) {
    fw_iso_resources_free(&motu.tx_resources);
    fw_iso_resources_free(&motu.rx_resources);
    return err;
    }
    motu.cache.size = motu.tx_stream.syt_interval * frames_per_buffer;
    motu.cache.event_offsets = kcalloc(motu.cache.size, sizeof(*motu.cache.event_offsets),
    GFP_KERNEL);
    if (!motu.cache.event_offsets) {
    fw_iso_resources_free(&motu.tx_resources);
    fw_iso_resources_free(&motu.rx_resources);
    return -ENOMEM;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ensure_packet_formats(motu: *mut snd_motu) -> c_int {
    static int ensure_packet_formats(struct snd_motu *motu)
    {
    __be32 reg;
    u32 data;
    int err;
    err = snd_motu_transaction_read(motu, PACKET_FORMAT_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg);
    data &= ~(TX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS |
    RX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS|
    TX_PACKET_TRANSMISSION_SPEED_MASK);
    if (motu.spec.tx_fixed_pcm_chunks[0] == motu.tx_packet_formats.pcm_chunks[0])
    data |= TX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS;
    if (motu.spec.rx_fixed_pcm_chunks[0] == motu.rx_packet_formats.pcm_chunks[0])
    data |= RX_PACKET_EXCLUDE_DIFFERED_DATA_CHUNKS;
    data |= fw_parent_device(motu.unit).max_speed;
    reg = cpu_to_be32(data);
    return snd_motu_transaction_write(motu, PACKET_FORMAT_OFFSET, &reg,
    sizeof(reg));
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_stream_start_duplex(motu: *mut snd_motu) -> c_int {
    int snd_motu_stream_start_duplex(struct snd_motu *motu)
    {
    let mut generation: c_uint = motu.rx_resources.generation;
    let mut err: c_int = 0;
    if (motu.substreams_counter == 0)
    return 0;
    if (amdtp_streaming_error(&motu.rx_stream) ||
    amdtp_streaming_error(&motu.tx_stream)) {
    amdtp_domain_stop(&motu.domain);
    finish_session(motu);
    }
    if (generation != fw_parent_device(motu.unit).card.generation) {
    err = fw_iso_resources_update(&motu.rx_resources);
    if (err < 0)
    return err;
    err = fw_iso_resources_update(&motu.tx_resources);
    if (err < 0)
    return err;
    }
    if (!amdtp_stream_running(&motu.rx_stream)) {
    let mut spd: c_int = fw_parent_device(motu.unit).max_speed;
    err = ensure_packet_formats(motu);
    if (err < 0)
    return err;
    if (motu.spec.flags & SND_MOTU_SPEC_REGISTER_DSP) {
    err = snd_motu_register_dsp_message_parser_init(motu);
    if (err < 0)
    return err;
    } else if (motu.spec.flags & SND_MOTU_SPEC_COMMAND_DSP) {
    err = snd_motu_command_dsp_message_parser_init(motu, motu.tx_stream.sfc);
    if (err < 0)
    return err;
    }
    err = begin_session(motu);
    if (err < 0) {
    dev_err(&motu.unit.device,
    "fail to start isochronous comm: %d\n", err);
    goto stop_streams;
    }
    err = amdtp_domain_add_stream(&motu.domain, &motu.tx_stream,
    motu.tx_resources.channel, spd);
    if (err < 0)
    goto stop_streams;
    err = amdtp_domain_add_stream(&motu.domain, &motu.rx_stream,
    motu.rx_resources.channel, spd);
    if (err < 0)
    goto stop_streams;
    motu.cache.tail = 0;
    motu.cache.tx_cycle_count = UINT_MAX;
    motu.cache.head = 0;
    motu.cache.rx_cycle_count = UINT_MAX;
// NOTE: The device requires both of replay; the sequence of the number of data
// blocks per packet, and the sequence of source packet header per data block as
// presentation time.
    err = amdtp_domain_start(&motu.domain, 0, true, false);
    if (err < 0)
    goto stop_streams;
    if (!amdtp_domain_wait_ready(&motu.domain, READY_TIMEOUT_MS)) {
    err = -ETIMEDOUT;
    goto stop_streams;
    }
    err = snd_motu_protocol_switch_fetching_mode(motu, true);
    if (err < 0) {
    dev_err(&motu.unit.device,
    "fail to enable frame fetching: %d\n", err);
    goto stop_streams;
    }
    }
    return 0;
    stop_streams:
    amdtp_domain_stop(&motu.domain);
    finish_session(motu);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_stream_stop_duplex(motu: *mut snd_motu) {
    void snd_motu_stream_stop_duplex(struct snd_motu *motu)
    {
    if (motu.substreams_counter == 0) {
    amdtp_domain_stop(&motu.domain);
    finish_session(motu);
    fw_iso_resources_free(&motu.tx_resources);
    fw_iso_resources_free(&motu.rx_resources);
    kfree(motu.cache.event_offsets);
    motu.cache.event_offsets = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn init_stream(motu: *mut snd_motu, s: *mut amdtp_stream) -> c_int {
    static int init_stream(struct snd_motu *motu, struct amdtp_stream *s)
    {
    struct fw_iso_resources *resources;
    enum amdtp_stream_direction dir;
    int err;
    if (s == &motu.tx_stream) {
    resources = &motu.tx_resources;
    dir = AMDTP_IN_STREAM;
    } else {
    resources = &motu.rx_resources;
    dir = AMDTP_OUT_STREAM;
    }
    err = fw_iso_resources_init(resources, motu.unit);
    if (err < 0)
    return err;
    err = amdtp_motu_init(s, motu.unit, dir, motu.spec, &motu.cache);
    if (err < 0)
    fw_iso_resources_destroy(resources);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn destroy_stream(motu: *mut snd_motu, s: *mut amdtp_stream) {
    static void destroy_stream(struct snd_motu *motu, struct amdtp_stream *s)
    {
    amdtp_stream_destroy(s);
    if (s == &motu.tx_stream)
    fw_iso_resources_destroy(&motu.tx_resources);
    else
    fw_iso_resources_destroy(&motu.rx_resources);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_stream_init_duplex(motu: *mut snd_motu) -> c_int {
    int snd_motu_stream_init_duplex(struct snd_motu *motu)
    {
    int err;
    err = init_stream(motu, &motu.tx_stream);
    if (err < 0)
    return err;
    err = init_stream(motu, &motu.rx_stream);
    if (err < 0) {
    destroy_stream(motu, &motu.tx_stream);
    return err;
    }
    err = amdtp_domain_init(&motu.domain);
    if (err < 0) {
    destroy_stream(motu, &motu.tx_stream);
    destroy_stream(motu, &motu.rx_stream);
    }
    return err;
    }
// This function should be called before starting streams or after stopping
// streams.
#[no_mangle]
pub unsafe extern "C" fn snd_motu_stream_destroy_duplex(motu: *mut snd_motu) {
    void snd_motu_stream_destroy_duplex(struct snd_motu *motu)
    {
    amdtp_domain_destroy(&motu.domain);
    destroy_stream(motu, &motu.rx_stream);
    destroy_stream(motu, &motu.tx_stream);
    motu.substreams_counter = 0;
    }
#[no_mangle]
unsafe extern "C" fn motu_lock_changed(motu: *mut snd_motu) {
    static void motu_lock_changed(struct snd_motu *motu)
    {
    motu.dev_lock_changed = true;
    wake_up(&motu.hwdep_wait);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_stream_lock_try(motu: *mut snd_motu) -> c_int {
    int snd_motu_stream_lock_try(struct snd_motu *motu)
    {
    guard(spinlock_irq)(&motu.lock);
    if (motu.dev_lock_count < 0)
    return -EBUSY;
    if (motu.dev_lock_count++ == 0)
    motu_lock_changed(motu);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_stream_lock_release(motu: *mut snd_motu) {
    void snd_motu_stream_lock_release(struct snd_motu *motu)
    {
    guard(spinlock_irq)(&motu.lock);
    if (WARN_ON(motu.dev_lock_count <= 0))
    return;
    if (--motu.dev_lock_count == 0)
    motu_lock_changed(motu);
    }
