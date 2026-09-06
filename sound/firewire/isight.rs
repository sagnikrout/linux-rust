//! Automatically rewritten from C to Rust
//! Source: sound/firewire/isight.c
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
// Apple iSight audio driver
//
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>
//

pub const OUI_APPLE: c_uint = 0x000a27;
pub const MODEL_APPLE_ISIGHT: c_uint = 0x000008;
pub const SW_ISIGHT_AUDIO: c_uint = 0x000010;
pub const REG_AUDIO_ENABLE: c_uint = 0x000;
pub const AUDIO_ENABLE: c_uint = 0x80000000;
pub const REG_DEF_AUDIO_GAIN: c_uint = 0x204;
pub const REG_GAIN_RAW_START: c_uint = 0x210;
pub const REG_GAIN_RAW_END: c_uint = 0x214;
pub const REG_GAIN_DB_START: c_uint = 0x218;
pub const REG_GAIN_DB_END: c_uint = 0x21c;
pub const REG_SAMPLE_RATE_INQUIRY: c_uint = 0x280;
pub const REG_ISO_TX_CONFIG: c_uint = 0x300;
pub const SPEED_SHIFT: c_int = 16;
pub const REG_SAMPLE_RATE: c_uint = 0x400;
pub const RATE_48000: c_uint = 0x80000000;
pub const REG_GAIN: c_uint = 0x500;
pub const REG_MUTE: c_uint = 0x504;
pub const MAX_FRAMES_PER_PACKET: c_int = 475;
pub const QUEUE_LENGTH: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isight {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub device: *mut fw_device,
    pub audio_base: u64,
    pub pcm: *mut snd_pcm_substream,
    pub mutex: mutex,
    pub buffer: iso_packets_buffer,
    pub resources: fw_iso_resources,
    pub context: *mut fw_iso_context,
    pub pcm_active: bool,
    pub pcm_running: bool,
    pub first_packet: bool,
    pub packet_index: c_int,
    pub total_samples: u32,
    pub buffer_pointer: c_uint,
    pub period_counter: c_uint,
    pub gain_max: s32 gain_min,,
    pub gain_tlv: [c_uint; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_payload {
    pub sample_count: __be32,
    pub signature: __be32,
    pub sample_total: __be32,
    pub reserved: __be32,
    pub MAX_FRAMES_PER_PACKET]: *mut *mut __be16 samples[2,
}

    MODULE_DESCRIPTION("iSight audio driver");
    MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
    MODULE_LICENSE("GPL");
    static struct fw_iso_packet audio_packet = {
    .payload_length = sizeof(struct audio_payload),
    .interrupt = 1,
    .header_length = 4,
    };
#[no_mangle]
unsafe extern "C" fn isight_update_pointers(isight: *mut isight, count: c_uint) {
    static void isight_update_pointers(struct isight *isight, unsigned int count)
    {
    struct snd_pcm_runtime *runtime = isight.pcm.runtime;
    unsigned int ptr;
    smp_wmb(); /* update buffer data before buffer pointer */
    ptr = isight.buffer_pointer;
    ptr += count;
    if (ptr >= runtime.buffer_size)
    ptr -= runtime.buffer_size;
    WRITE_ONCE(isight.buffer_pointer, ptr);
    isight.period_counter += count;
    if (isight.period_counter >= runtime.period_size) {
    isight.period_counter -= runtime.period_size;
    snd_pcm_period_elapsed(isight.pcm);
    }
    }
    static void isight_samples(struct isight *isight,
    const __be16 *samples, unsigned int count)
    {
    struct snd_pcm_runtime *runtime;
    unsigned int count1;
    if (!READ_ONCE(isight.pcm_running))
    return;
    runtime = isight.pcm.runtime;
    if (isight.buffer_pointer + count <= runtime.buffer_size) {
    memcpy(runtime.dma_area + isight.buffer_pointer * 4,
    samples, count * 4);
    } else {
    count1 = runtime.buffer_size - isight.buffer_pointer;
    memcpy(runtime.dma_area + isight.buffer_pointer * 4,
    samples, count1 * 4);
    samples += count1 * 2;
    memcpy(runtime.dma_area, samples, (count - count1) * 4);
    }
    isight_update_pointers(isight, count);
    }
#[no_mangle]
unsafe extern "C" fn isight_pcm_abort(isight: *mut isight) {
    static void isight_pcm_abort(struct isight *isight)
    {
    if (READ_ONCE(isight.pcm_active))
    snd_pcm_stop_xrun(isight.pcm);
    }
#[no_mangle]
unsafe extern "C" fn isight_dropped_samples(isight: *mut isight, total: c_uint) {
    static void isight_dropped_samples(struct isight *isight, unsigned int total)
    {
    struct snd_pcm_runtime *runtime;
    u32 dropped;
    unsigned int count1;
    if (!READ_ONCE(isight.pcm_running))
    return;
    runtime = isight.pcm.runtime;
    dropped = total - isight.total_samples;
    if (dropped < runtime.buffer_size) {
    if (isight.buffer_pointer + dropped <= runtime.buffer_size) {
    memset(runtime.dma_area + isight.buffer_pointer * 4,
    0, dropped * 4);
    } else {
    count1 = runtime.buffer_size - isight.buffer_pointer;
    memset(runtime.dma_area + isight.buffer_pointer * 4,
    0, count1 * 4);
    memset(runtime.dma_area, 0, (dropped - count1) * 4);
    }
    isight_update_pointers(isight, dropped);
    } else {
    isight_pcm_abort(isight);
    }
    }
    static void isight_packet(struct fw_iso_context *context, u32 cycle,
    size_t header_length, void *header, void *data)
    {
    struct isight *isight = data;
    const struct audio_payload *payload;
    unsigned int index, length, count, total;
    int err;
    if (isight.packet_index < 0)
    return;
    index = isight.packet_index;
    payload = isight.buffer.packets[index].buffer;
    length = be32_to_cpup(header) >> 16;
    if (likely(length >= 16 &&
    payload.signature == cpu_to_be32(0x73676874/*"sght"*/))) {
    count = be32_to_cpu(payload.sample_count);
    if (likely(count <= (length - 16) / 4 &&
    count <= MAX_FRAMES_PER_PACKET)) {
    total = be32_to_cpu(payload.sample_total);
    if (unlikely(total != isight.total_samples)) {
    if (!isight.first_packet)
    isight_dropped_samples(isight, total);
    isight.first_packet = false;
    isight.total_samples = total;
    }
    isight_samples(isight, payload.samples, count);
    isight.total_samples += count;
    }
    }
    err = fw_iso_context_queue(isight.context, &audio_packet,
    &isight.buffer.iso_buffer,
    isight.buffer.packets[index].offset);
    if (err < 0) {
    dev_err(&isight.unit.device, "queueing error: %d\n", err);
    isight_pcm_abort(isight);
    isight.packet_index = -1;
    return;
    }
    fw_iso_context_queue_flush(isight.context);
    if (++index >= QUEUE_LENGTH)
    index = 0;
    isight.packet_index = index;
    }
#[no_mangle]
unsafe extern "C" fn isight_connect(isight: *mut isight) -> c_int {
    static int isight_connect(struct isight *isight)
    {
    int ch, err;
    __be32 value;
    retry_after_bus_reset:
    ch = fw_iso_resources_allocate(&isight.resources,
    sizeof(struct audio_payload),
    isight.device.max_speed);
    if (ch < 0) {
    err = ch;
    goto error;
    }
    value = cpu_to_be32(ch | (isight.device.max_speed << SPEED_SHIFT));
    err = snd_fw_transaction(isight.unit, TCODE_WRITE_QUADLET_REQUEST,
    isight.audio_base + REG_ISO_TX_CONFIG,
    &value, 4, FW_FIXED_GENERATION |
    isight.resources.generation);
    if (err == -EAGAIN) {
    fw_iso_resources_free(&isight.resources);
    goto retry_after_bus_reset;
    } else if (err < 0) {
    goto err_resources;
    }
    return 0;
    err_resources:
    fw_iso_resources_free(&isight.resources);
    error:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn isight_open(substream: *mut snd_pcm_substream) -> c_int {
    static int isight_open(struct snd_pcm_substream *substream)
    {
    static const struct snd_pcm_hardware hardware = {
    .info = SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_BATCH |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER,
    .formats = SNDRV_PCM_FMTBIT_S16_BE,
    .rates = SNDRV_PCM_RATE_48000,
    .rate_min = 48000,
    .rate_max = 48000,
    .channels_min = 2,
    .channels_max = 2,
    .buffer_bytes_max = 4 * 1024 * 1024,
    .period_bytes_min = MAX_FRAMES_PER_PACKET * 4,
    .period_bytes_max = 1024 * 1024,
    .periods_min = 2,
    .periods_max = UINT_MAX,
    };
    struct isight *isight = substream.private_data;
    substream.runtime.hw = hardware;
    return iso_packets_buffer_init(&isight.buffer, isight.unit,
    QUEUE_LENGTH,
    sizeof(struct audio_payload),
    DMA_FROM_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn isight_close(substream: *mut snd_pcm_substream) -> c_int {
    static int isight_close(struct snd_pcm_substream *substream)
    {
    struct isight *isight = substream.private_data;
    iso_packets_buffer_destroy(&isight.buffer, isight.unit);
    return 0;
    }
    static int isight_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct isight *isight = substream.private_data;
    WRITE_ONCE(isight.pcm_active, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn reg_read(isight: *mut isight, offset: c_int, value: *mut __be32) -> c_int {
    static int reg_read(struct isight *isight, int offset, __be32 *value)
    {
    return snd_fw_transaction(isight.unit, TCODE_READ_QUADLET_REQUEST,
    isight.audio_base + offset, value, 4, 0);
    }
#[no_mangle]
unsafe extern "C" fn reg_write(isight: *mut isight, offset: c_int, value: __be32) -> c_int {
    static int reg_write(struct isight *isight, int offset, __be32 value)
    {
    return snd_fw_transaction(isight.unit, TCODE_WRITE_QUADLET_REQUEST,
    isight.audio_base + offset, &value, 4, 0);
    }
#[no_mangle]
unsafe extern "C" fn isight_stop_streaming(isight: *mut isight) {
    static void isight_stop_streaming(struct isight *isight)
    {
    __be32 value;
    if (!isight.context)
    return;
    fw_iso_context_stop(isight.context);
    fw_iso_context_destroy(isight.context);
    isight.context = core::ptr::null_mut();
    fw_iso_resources_free(&isight.resources);
    value = 0;
    snd_fw_transaction(isight.unit, TCODE_WRITE_QUADLET_REQUEST,
    isight.audio_base + REG_AUDIO_ENABLE,
    &value, 4, FW_QUIET);
    }
#[no_mangle]
unsafe extern "C" fn isight_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int isight_hw_free(struct snd_pcm_substream *substream)
    {
    struct isight *isight = substream.private_data;
    WRITE_ONCE(isight.pcm_active, false);
    guard(mutex)(&isight.mutex);
    isight_stop_streaming(isight);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isight_start_streaming(isight: *mut isight) -> c_int {
    static int isight_start_streaming(struct isight *isight)
    {
    unsigned int i;
    int err;
    if (isight.context) {
    if (isight.packet_index < 0)
    isight_stop_streaming(isight);
    else
    return 0;
    }
    err = reg_write(isight, REG_SAMPLE_RATE, cpu_to_be32(RATE_48000));
    if (err < 0)
    goto error;
    err = isight_connect(isight);
    if (err < 0)
    goto error;
    err = reg_write(isight, REG_AUDIO_ENABLE, cpu_to_be32(AUDIO_ENABLE));
    if (err < 0)
    goto err_resources;
    isight.context = fw_iso_context_create(isight.device.card,
    FW_ISO_CONTEXT_RECEIVE,
    isight.resources.channel,
    isight.device.max_speed,
    4, isight_packet, isight);
    if (IS_ERR(isight.context)) {
    err = PTR_ERR(isight.context);
    isight.context = core::ptr::null_mut();
    goto err_resources;
    }
    for (i = 0; i < QUEUE_LENGTH; ++i) {
    err = fw_iso_context_queue(isight.context, &audio_packet,
    &isight.buffer.iso_buffer,
    isight.buffer.packets[i].offset);
    if (err < 0)
    goto err_context;
    }
    isight.first_packet = true;
    isight.packet_index = 0;
    err = fw_iso_context_start(isight.context, -1, 0,
    FW_ISO_CONTEXT_MATCH_ALL_TAGS/*?*/);
    if (err < 0)
    goto err_context;
    return 0;
    err_context:
    fw_iso_context_destroy(isight.context);
    isight.context = core::ptr::null_mut();
    err_resources:
    fw_iso_resources_free(&isight.resources);
    reg_write(isight, REG_AUDIO_ENABLE, 0);
    error:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn isight_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int isight_prepare(struct snd_pcm_substream *substream)
    {
    struct isight *isight = substream.private_data;
    isight.buffer_pointer = 0;
    isight.period_counter = 0;
    guard(mutex)(&isight.mutex);
    return isight_start_streaming(isight);
    }
#[no_mangle]
unsafe extern "C" fn isight_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int isight_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct isight *isight = substream.private_data;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    WRITE_ONCE(isight.pcm_running, true);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    WRITE_ONCE(isight.pcm_running, false);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isight_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t isight_pointer(struct snd_pcm_substream *substream)
    {
    struct isight *isight = substream.private_data;
    return READ_ONCE(isight.buffer_pointer);
    }
#[no_mangle]
unsafe extern "C" fn isight_create_pcm(isight: *mut isight) -> c_int {
    static int isight_create_pcm(struct isight *isight)
    {
    static const struct snd_pcm_ops ops = {
    .open      = isight_open,
    .close     = isight_close,
    .hw_params = isight_hw_params,
    .hw_free   = isight_hw_free,
    .prepare   = isight_prepare,
    .trigger   = isight_trigger,
    .pointer   = isight_pointer,
    };
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(isight.card, "iSight", 0, 0, 1, &pcm);
    if (err < 0)
    return err;
    pcm.private_data = isight;
    pcm.nonatomic = true;
    strscpy(pcm.name, "iSight");
    isight.pcm = pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream;
    isight.pcm.ops = &ops;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);
    return 0;
    }
    static int isight_gain_info(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_info *info)
    {
    struct isight *isight = ctl.private_data;
    info.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    info.count = 1;
    info.value.integer.min = isight.gain_min;
    info.value.integer.max = isight.gain_max;
    return 0;
    }
    static int isight_gain_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct isight *isight = ctl.private_data;
    __be32 gain;
    int err;
    err = reg_read(isight, REG_GAIN, &gain);
    if (err < 0)
    return err;
    value.value.integer.value[0] = (s32)be32_to_cpu(gain);
    return 0;
    }
    static int isight_gain_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct isight *isight = ctl.private_data;
    if (value.value.integer.value[0] < isight.gain_min ||
    value.value.integer.value[0] > isight.gain_max)
    return -EINVAL;
    return reg_write(isight, REG_GAIN,
    cpu_to_be32(value.value.integer.value[0]));
    }
    static int isight_mute_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct isight *isight = ctl.private_data;
    __be32 mute;
    int err;
    err = reg_read(isight, REG_MUTE, &mute);
    if (err < 0)
    return err;
    value.value.integer.value[0] = !mute;
    return 0;
    }
    static int isight_mute_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct isight *isight = ctl.private_data;
    return reg_write(isight, REG_MUTE,
    ( __be32)!value.value.integer.value[0]);
    }
#[no_mangle]
unsafe extern "C" fn isight_create_mixer(isight: *mut isight) -> c_int {
    static int isight_create_mixer(struct isight *isight)
    {
    static const struct snd_kcontrol_new gain_control = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Mic Capture Volume",
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
    SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    .info = isight_gain_info,
    .get = isight_gain_get,
    .put = isight_gain_put,
    };
    static const struct snd_kcontrol_new mute_control = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Mic Capture Switch",
    .info = snd_ctl_boolean_mono_info,
    .get = isight_mute_get,
    .put = isight_mute_put,
    };
    __be32 value;
    struct snd_kcontrol *ctl;
    int err;
    err = reg_read(isight, REG_GAIN_RAW_START, &value);
    if (err < 0)
    return err;
    isight.gain_min = be32_to_cpu(value);
    err = reg_read(isight, REG_GAIN_RAW_END, &value);
    if (err < 0)
    return err;
    isight.gain_max = be32_to_cpu(value);
    isight.gain_tlv[SNDRV_CTL_TLVO_TYPE] = SNDRV_CTL_TLVT_DB_MINMAX;
    isight.gain_tlv[SNDRV_CTL_TLVO_LEN] = 2 * sizeof(unsigned int);
    err = reg_read(isight, REG_GAIN_DB_START, &value);
    if (err < 0)
    return err;
    isight.gain_tlv[SNDRV_CTL_TLVO_DB_MINMAX_MIN] =
    (s32)be32_to_cpu(value) * 100;
    err = reg_read(isight, REG_GAIN_DB_END, &value);
    if (err < 0)
    return err;
    isight.gain_tlv[SNDRV_CTL_TLVO_DB_MINMAX_MAX] =
    (s32)be32_to_cpu(value) * 100;
    ctl = snd_ctl_new1(&gain_control, isight);
    if (ctl)
    ctl.tlv.p = isight.gain_tlv;
    err = snd_ctl_add(isight.card, ctl);
    if (err < 0)
    return err;
    err = snd_ctl_add(isight.card, snd_ctl_new1(&mute_control, isight));
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isight_card_free(card: *mut snd_card) {
    static void isight_card_free(struct snd_card *card)
    {
    struct isight *isight = card.private_data;
    fw_iso_resources_destroy(&isight.resources);
    }
#[no_mangle]
unsafe extern "C" fn get_unit_base(unit: *mut fw_unit) -> u64 {
    static u64 get_unit_base(struct fw_unit *unit)
    {
    struct fw_csr_iterator i;
    int key, value;
    fw_csr_iterator_init(&i, unit.directory);
    while (fw_csr_iterator_next(&i, &key, &value))
    if (key == CSR_OFFSET)
    return CSR_REGISTER_BASE + value * 4;
    return 0;
    }
    static int isight_probe(struct fw_unit *unit,
    const struct ieee1394_device_id *id)
    {
    struct fw_device *fw_dev = fw_parent_device(unit);
    struct snd_card *card;
    struct isight *isight;
    int err;
    err = snd_card_new(&unit.device, -1, core::ptr::null_mut(), THIS_MODULE,
    sizeof(*isight), &card);
    if (err < 0)
    return err;
    isight = card.private_data;
    isight.card = card;
    mutex_init(&isight.mutex);
    isight.unit = fw_unit_get(unit);
    isight.device = fw_dev;
    isight.audio_base = get_unit_base(unit);
    if (!isight.audio_base) {
    dev_err(&unit.device, "audio unit base not found\n");
    err = -ENXIO;
    goto error;
    }
    fw_iso_resources_init(&isight.resources, unit);
    card.private_free = isight_card_free;
    strscpy(card.driver, "iSight");
    strscpy(card.shortname, "Apple iSight");
    snprintf(card.longname, sizeof(card.longname),
    "Apple iSight (GUID %08x%08x) at %s, S%d",
    fw_dev.config_rom[3], fw_dev.config_rom[4],
    dev_name(&unit.device), 100 << fw_dev.max_speed);
    strscpy(card.mixername, "iSight");
    err = isight_create_pcm(isight);
    if (err < 0)
    goto error;
    err = isight_create_mixer(isight);
    if (err < 0)
    goto error;
    err = snd_card_register(card);
    if (err < 0)
    goto error;
    dev_set_drvdata(&unit.device, isight);
    return 0;
    error:
    snd_card_free(card);
    mutex_destroy(&isight.mutex);
    fw_unit_put(isight.unit);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn isight_bus_reset(unit: *mut fw_unit) {
    static void isight_bus_reset(struct fw_unit *unit)
    {
    struct isight *isight = dev_get_drvdata(&unit.device);
    if (fw_iso_resources_update(&isight.resources) < 0) {
    isight_pcm_abort(isight);
    guard(mutex)(&isight.mutex);
    isight_stop_streaming(isight);
    }
    }
#[no_mangle]
unsafe extern "C" fn isight_remove(unit: *mut fw_unit) {
    static void isight_remove(struct fw_unit *unit)
    {
    struct isight *isight = dev_get_drvdata(&unit.device);
    isight_pcm_abort(isight);
    snd_card_disconnect(isight.card);
    scoped_guard(mutex, &isight.mutex) {
    isight_stop_streaming(isight);
    }
// Block till all of ALSA character devices are released.
    snd_card_free(isight.card);
    mutex_destroy(&isight.mutex);
    fw_unit_put(isight.unit);
    }
    static const struct ieee1394_device_id isight_id_table[] = {
    {
    .match_flags  = IEEE1394_MATCH_SPECIFIER_ID |
    IEEE1394_MATCH_VERSION,
    .specifier_id = OUI_APPLE,
    .version      = SW_ISIGHT_AUDIO,
    },
    { }
    };
    MODULE_DEVICE_TABLE(ieee1394, isight_id_table);
    static struct fw_driver isight_driver = {
    .driver   = {
    .owner	= THIS_MODULE,
    .name	= KBUILD_MODNAME,
    .bus	= &fw_bus_type,
    },
    .probe    = isight_probe,
    .update   = isight_bus_reset,
    .remove   = isight_remove,
    .id_table = isight_id_table,
    };
#[no_mangle]
unsafe extern "C" fn alsa_isight_init() -> int __init {
    static int __init alsa_isight_init(void)
    {
    return driver_register(&isight_driver.driver);
    }
#[no_mangle]
unsafe extern "C" fn alsa_isight_exit() -> void __exit {
    static void __exit alsa_isight_exit(void)
    {
    driver_unregister(&isight_driver.driver);
    }
    module_init(alsa_isight_init);
    module_exit(alsa_isight_exit);
