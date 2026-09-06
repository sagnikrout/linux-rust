//! Automatically rewritten from C to Rust
//! Source: sound/pci/lola/lola_pcm.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Support for Digigram Lola PCI-e boards
//
// Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
//

pub const LOLA_MAX_BDL_ENTRIES: c_int = 8;

    static struct lola_pcm *lola_get_pcm(struct snd_pcm_substream *substream)
    {
    struct lola *chip = snd_pcm_substream_chip(substream);
    return &chip.pcm[substream.stream];
    }
    static struct lola_stream *lola_get_stream(struct snd_pcm_substream *substream)
    {
    struct lola_pcm *pcm = lola_get_pcm(substream);
    let mut idx: c_uint = substream.number;
    return &pcm.streams[idx];
    }
#[no_mangle]
unsafe extern "C" fn lola_get_lrc(chip: *mut lola) -> c_uint {
    static unsigned int lola_get_lrc(struct lola *chip)
    {
    return lola_readl(chip, BAR1, LRC);
    }
#[no_mangle]
unsafe extern "C" fn lola_get_tstamp(chip: *mut lola, quick_no_sync: bool) -> c_uint {
    static unsigned int lola_get_tstamp(struct lola *chip, bool quick_no_sync)
    {
    let mut tstamp: c_uint = lola_get_lrc(chip) >> 8;
    if (chip.granularity) {
    let mut wait_banks: c_uint = quick_no_sync ? 0 : 8;
    tstamp += (wait_banks + 1) * chip.granularity - 1;
    tstamp -= tstamp % chip.granularity;
    }
    return tstamp << 8;
    }
// clear any pending interrupt status
    static void lola_stream_clear_pending_irq(struct lola *chip,
    struct lola_stream *str)
    {
    let mut val: c_uint = lola_dsd_read(chip, str.dsd, STS);
    val &= LOLA_DSD_STS_DESE | LOLA_DSD_STS_BCIS;
    if (val)
    lola_dsd_write(chip, str.dsd, STS, val);
    }
    static void lola_stream_start(struct lola *chip, struct lola_stream *str,
    unsigned int tstamp)
    {
    lola_stream_clear_pending_irq(chip, str);
    lola_dsd_write(chip, str.dsd, CTL,
    LOLA_DSD_CTL_SRUN |
    LOLA_DSD_CTL_IOCE |
    LOLA_DSD_CTL_DEIE |
    LOLA_DSD_CTL_VLRCV |
    tstamp);
    }
    static void lola_stream_stop(struct lola *chip, struct lola_stream *str,
    unsigned int tstamp)
    {
    lola_dsd_write(chip, str.dsd, CTL,
    LOLA_DSD_CTL_IOCE |
    LOLA_DSD_CTL_DEIE |
    LOLA_DSD_CTL_VLRCV |
    tstamp);
    lola_stream_clear_pending_irq(chip, str);
    }
#[no_mangle]
unsafe extern "C" fn wait_for_srst_clear(chip: *mut lola, str: *mut lola_stream) {
    static void wait_for_srst_clear(struct lola *chip, struct lola_stream *str)
    {
    let mut end_time: c_ulong = jiffies + msecs_to_jiffies(200);
    while (time_before(jiffies, end_time)) {
    unsigned int val;
    val = lola_dsd_read(chip, str.dsd, CTL);
    if (!(val & LOLA_DSD_CTL_SRST))
    return;
    msleep(1);
    }
    dev_warn(chip.card.dev, "SRST not clear (stream %d)\n", str.dsd);
    }
    static int lola_stream_wait_for_fifo(struct lola *chip,
    struct lola_stream *str,
    bool ready)
    {
    let mut val: c_uint = ready ? LOLA_DSD_STS_FIFORDY : 0;
    let mut end_time: c_ulong = jiffies + msecs_to_jiffies(200);
    while (time_before(jiffies, end_time)) {
    let mut reg: c_uint = lola_dsd_read(chip, str.dsd, STS);
    if ((reg & LOLA_DSD_STS_FIFORDY) == val)
    return 0;
    msleep(1);
    }
    dev_warn(chip.card.dev, "FIFO not ready (stream %d)\n", str.dsd);
    return -EIO;
    }
// sync for FIFO ready/empty for all linked streams;
// clear paused flag when FIFO gets ready again
//
    static int lola_sync_wait_for_fifo(struct lola *chip,
    struct snd_pcm_substream *substream,
    bool ready)
    {
    let mut val: c_uint = ready ? LOLA_DSD_STS_FIFORDY : 0;
    let mut end_time: c_ulong = jiffies + msecs_to_jiffies(200);
    struct snd_pcm_substream *s;
    let mut pending: c_int = 0;
    while (time_before(jiffies, end_time)) {
    pending = 0;
    snd_pcm_group_for_each_entry(s, substream) {
    struct lola_stream *str;
    if (s.pcm.card != substream.pcm.card)
    continue;
    str = lola_get_stream(s);
    if (str.prepared && str.paused) {
    unsigned int reg;
    reg = lola_dsd_read(chip, str.dsd, STS);
    if ((reg & LOLA_DSD_STS_FIFORDY) != val) {
    pending = str.dsd + 1;
    break;
    }
    if (ready)
    str.paused = 0;
    }
    }
    if (!pending)
    return 0;
    msleep(1);
    }
    dev_warn(chip.card.dev, "FIFO not ready (pending %d)\n", pending - 1);
    return -EIO;
    }
// finish pause - prepare for a new resume
    static void lola_sync_pause(struct lola *chip,
    struct snd_pcm_substream *substream)
    {
    struct snd_pcm_substream *s;
    lola_sync_wait_for_fifo(chip, substream, false);
    snd_pcm_group_for_each_entry(s, substream) {
    struct lola_stream *str;
    if (s.pcm.card != substream.pcm.card)
    continue;
    str = lola_get_stream(s);
    if (str.paused && str.prepared)
    lola_dsd_write(chip, str.dsd, CTL, LOLA_DSD_CTL_SRUN |
    LOLA_DSD_CTL_IOCE | LOLA_DSD_CTL_DEIE);
    }
    lola_sync_wait_for_fifo(chip, substream, true);
    }
#[no_mangle]
unsafe extern "C" fn lola_stream_reset(chip: *mut lola, str: *mut lola_stream) {
    static void lola_stream_reset(struct lola *chip, struct lola_stream *str)
    {
    if (str.prepared) {
    if (str.paused)
    lola_sync_pause(chip, str.substream);
    str.prepared = 0;
    lola_dsd_write(chip, str.dsd, CTL,
    LOLA_DSD_CTL_IOCE | LOLA_DSD_CTL_DEIE);
    lola_stream_wait_for_fifo(chip, str, false);
    lola_stream_clear_pending_irq(chip, str);
    lola_dsd_write(chip, str.dsd, CTL, LOLA_DSD_CTL_SRST);
    lola_dsd_write(chip, str.dsd, LVI, 0);
    lola_dsd_write(chip, str.dsd, BDPU, 0);
    lola_dsd_write(chip, str.dsd, BDPL, 0);
    wait_for_srst_clear(chip, str);
    }
    }
    static const struct snd_pcm_hardware lola_pcm_hw = {
    .info =			(SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_INTERLEAVED |
    SNDRV_PCM_INFO_BLOCK_TRANSFER |
    SNDRV_PCM_INFO_MMAP_VALID |
    SNDRV_PCM_INFO_PAUSE),
    .formats =		(SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE |
    SNDRV_PCM_FMTBIT_FLOAT_LE),
    .rates =		SNDRV_PCM_RATE_8000_192000,
    .rate_min =		8000,
    .rate_max =		192000,
    .channels_min =		1,
    .channels_max =		2,
    .buffer_bytes_max =	LOLA_MAX_BUF_SIZE,
    .period_bytes_min =	128,
    .period_bytes_max =	LOLA_MAX_BUF_SIZE / 2,
    .periods_min =		2,
    .periods_max =		LOLA_MAX_BDL_ENTRIES,
    .fifo_size =		0,
    };
#[no_mangle]
unsafe extern "C" fn lola_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    static int lola_pcm_open(struct snd_pcm_substream *substream)
    {
    struct lola *chip = snd_pcm_substream_chip(substream);
    struct lola_pcm *pcm = lola_get_pcm(substream);
    struct lola_stream *str = lola_get_stream(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    guard(mutex)(&chip.open_mutex);
    if (str.opened)
    return -EBUSY;
    str.substream = substream;
    str.master = core::ptr::null_mut();
    str.opened = 1;
    runtime.hw = lola_pcm_hw;
    runtime.hw.channels_max = pcm.num_streams - str.index;
    if (chip.sample_rate) {
// sample rate is locked
    runtime.hw.rate_min = chip.sample_rate;
    runtime.hw.rate_max = chip.sample_rate;
    } else {
    runtime.hw.rate_min = chip.sample_rate_min;
    runtime.hw.rate_max = chip.sample_rate_max;
    }
    chip.ref_count_rate++;
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
// period size = multiple of chip->granularity (8, 16 or 32 frames)
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
    chip.granularity);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
    chip.granularity);
    return 0;
    }
    static void lola_cleanup_slave_streams(struct lola_pcm *pcm,
    struct lola_stream *str)
    {
    int i;
    for (i = str.index + 1; i < pcm.num_streams; i++) {
    struct lola_stream *s = &pcm.streams[i];
    if (s.master != str)
    break;
    s.master = core::ptr::null_mut();
    s.opened = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn lola_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    static int lola_pcm_close(struct snd_pcm_substream *substream)
    {
    struct lola *chip = snd_pcm_substream_chip(substream);
    struct lola_stream *str = lola_get_stream(substream);
    guard(mutex)(&chip.open_mutex);
    if (str.substream == substream) {
    str.substream = core::ptr::null_mut();
    str.opened = 0;
    }
    if (--chip.ref_count_rate == 0) {
// release sample rate
    chip.sample_rate = 0;
    }
    return 0;
    }
    static int lola_pcm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *hw_params)
    {
    struct lola_stream *str = lola_get_stream(substream);
    str.bufsize = 0;
    str.period_bytes = 0;
    str.format_verb = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lola_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int lola_pcm_hw_free(struct snd_pcm_substream *substream)
    {
    struct lola *chip = snd_pcm_substream_chip(substream);
    struct lola_pcm *pcm = lola_get_pcm(substream);
    struct lola_stream *str = lola_get_stream(substream);
    guard(mutex)(&chip.open_mutex);
    lola_stream_reset(chip, str);
    lola_cleanup_slave_streams(pcm, str);
    return 0;
    }
//
// set up a BDL entry
//
    static int setup_bdle(struct snd_pcm_substream *substream,
    struct lola_stream *str, __le32 **bdlp,
    int ofs, int size)
    {
    __le32 *bdl = *bdlp;
    while (size > 0) {
    dma_addr_t addr;
    int chunk;
    if (str.frags >= LOLA_MAX_BDL_ENTRIES)
    return -EINVAL;
    addr = snd_pcm_sgbuf_get_addr(substream, ofs);
// program the address field of the BDL entry
    bdl[0] = cpu_to_le32((u32)addr);
    bdl[1] = cpu_to_le32(upper_32_bits(addr));
// program the size field of the BDL entry
    chunk = snd_pcm_sgbuf_get_chunk_size(substream, ofs, size);
    bdl[2] = cpu_to_le32(chunk);
// program the IOC to enable interrupt
// only when the whole fragment is processed
//
    size -= chunk;
    bdl[3] = size ? 0 : cpu_to_le32(0x01);
    bdl += 4;
    str.frags++;
    ofs += chunk;
    }
// bdlp = bdl;
    return ofs;
    }
//
// set up BDL entries
//
    static int lola_setup_periods(struct lola *chip, struct lola_pcm *pcm,
    struct snd_pcm_substream *substream,
    struct lola_stream *str)
    {
    __le32 *bdl;
    int i, ofs, periods, period_bytes;
    period_bytes = str.period_bytes;
    periods = str.bufsize / period_bytes;
// program the initial BDL entries
    bdl = (__le32 *)(pcm.bdl.area + LOLA_BDL_ENTRY_SIZE * str.index);
    ofs = 0;
    str.frags = 0;
    for (i = 0; i < periods; i++) {
    ofs = setup_bdle(substream, str, &bdl, ofs, period_bytes);
    if (ofs < 0)
    goto error;
    }
    return 0;
    error:
    dev_err(chip.card.dev, "Too many BDL entries: buffer=%d, period=%d\n",
    str.bufsize, period_bytes);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn lola_get_format_verb(substream: *mut snd_pcm_substream) -> c_uint {
    static unsigned int lola_get_format_verb(struct snd_pcm_substream *substream)
    {
    unsigned int verb;
    switch (substream.runtime.format) {
    case SNDRV_PCM_FORMAT_S16_LE:
    verb = 0x00000000;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    verb = 0x00000200;
    break;
    case SNDRV_PCM_FORMAT_S32_LE:
    verb = 0x00000300;
    break;
    case SNDRV_PCM_FORMAT_FLOAT_LE:
    verb = 0x00001300;
    break;
    default:
    return 0;
    }
    verb |= substream.runtime.channels;
    return verb;
    }
    static int lola_set_stream_config(struct lola *chip,
    struct lola_stream *str,
    int channels)
    {
    int i, err;
    unsigned int verb, val;
// set format info for all channels
// (with only one command for the first channel)
//
    err = lola_codec_read(chip, str.nid, LOLA_VERB_SET_STREAM_FORMAT,
    str.format_verb, 0, &val, core::ptr::null_mut());
    if (err < 0) {
    dev_err(chip.card.dev, "Cannot set stream format 0x%x\n",
    str.format_verb);
    return err;
    }
// update stream - channel config
    for (i = 0; i < channels; i++) {
    verb = (str.index << 6) | i;
    err = lola_codec_read(chip, str[i].nid,
    LOLA_VERB_SET_CHANNEL_STREAMID, 0, verb,
    &val, core::ptr::null_mut());
    if (err < 0) {
    dev_err(chip.card.dev,
    "Cannot set stream channel %d\n", i);
    return err;
    }
    }
    return 0;
    }
//
// set up the SD for streaming
//
    static int lola_setup_controller(struct lola *chip, struct lola_pcm *pcm,
    struct lola_stream *str)
    {
    dma_addr_t bdl;
    if (str.prepared)
    return -EINVAL;
// set up BDL
    bdl = pcm.bdl.addr + LOLA_BDL_ENTRY_SIZE * str.index;
    lola_dsd_write(chip, str.dsd, BDPL, (u32)bdl);
    lola_dsd_write(chip, str.dsd, BDPU, upper_32_bits(bdl));
// program the stream LVI (last valid index) of the BDL
    lola_dsd_write(chip, str.dsd, LVI, str.frags - 1);
    lola_stream_clear_pending_irq(chip, str);
    lola_dsd_write(chip, str.dsd, CTL,
    LOLA_DSD_CTL_IOCE | LOLA_DSD_CTL_DEIE | LOLA_DSD_CTL_SRUN);
    str.prepared = 1;
    return lola_stream_wait_for_fifo(chip, str, true);
    }
#[no_mangle]
unsafe extern "C" fn lola_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int lola_pcm_prepare(struct snd_pcm_substream *substream)
    {
    struct lola *chip = snd_pcm_substream_chip(substream);
    struct lola_pcm *pcm = lola_get_pcm(substream);
    struct lola_stream *str = lola_get_stream(substream);
    struct snd_pcm_runtime *runtime = substream.runtime;
    unsigned int bufsize, period_bytes, format_verb;
    int i, err;
    scoped_guard(mutex, &chip.open_mutex) {
    lola_stream_reset(chip, str);
    lola_cleanup_slave_streams(pcm, str);
    if (str.index + runtime.channels > pcm.num_streams)
    return -EINVAL;
    for (i = 1; i < runtime.channels; i++) {
    str[i].master = str;
    str[i].opened = 1;
    }
    }
    bufsize = snd_pcm_lib_buffer_bytes(substream);
    period_bytes = snd_pcm_lib_period_bytes(substream);
    format_verb = lola_get_format_verb(substream);
    str.bufsize = bufsize;
    str.period_bytes = period_bytes;
    str.format_verb = format_verb;
    err = lola_setup_periods(chip, pcm, substream, str);
    if (err < 0)
    return err;
    err = lola_set_sample_rate(chip, runtime.rate);
    if (err < 0)
    return err;
    chip.sample_rate = runtime.rate;	/* sample rate gets locked */
    err = lola_set_stream_config(chip, str, runtime.channels);
    if (err < 0)
    return err;
    err = lola_setup_controller(chip, pcm, str);
    if (err < 0) {
    lola_stream_reset(chip, str);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lola_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int lola_pcm_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct lola *chip = snd_pcm_substream_chip(substream);
    struct lola_stream *str;
    struct snd_pcm_substream *s;
    unsigned int start;
    unsigned int tstamp;
    bool sync_streams;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    case SNDRV_PCM_TRIGGER_RESUME:
    start = 1;
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_STOP:
    start = 0;
    break;
    default:
    return -EINVAL;
    }
//
// sample correct synchronization is only needed starting several
// streams. On stop or if only one stream do as quick as possible
//
    sync_streams = (start && snd_pcm_stream_linked(substream));
    tstamp = lola_get_tstamp(chip, !sync_streams);
    guard(spinlock)(&chip.reg_lock);
    snd_pcm_group_for_each_entry(s, substream) {
    if (s.pcm.card != substream.pcm.card)
    continue;
    str = lola_get_stream(s);
    if (start)
    lola_stream_start(chip, str, tstamp);
    else
    lola_stream_stop(chip, str, tstamp);
    str.running = start;
    str.paused = !start;
    snd_pcm_trigger_done(s, substream);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lola_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t lola_pcm_pointer(struct snd_pcm_substream *substream)
    {
    struct lola *chip = snd_pcm_substream_chip(substream);
    struct lola_stream *str = lola_get_stream(substream);
    let mut pos: c_uint = lola_dsd_read(chip, str.dsd, LPIB);
    if (pos >= str.bufsize)
    pos = 0;
    return bytes_to_frames(substream.runtime, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn lola_pcm_update(chip: *mut lola, pcm: *mut lola_pcm, bits: c_uint) {
    void lola_pcm_update(struct lola *chip, struct lola_pcm *pcm, unsigned int bits)
    {
    int i;
    let mut num_streams: u8 = min_t(u8, pcm.num_streams, ARRAY_SIZE(pcm.streams));
    for (i = 0; bits && i < num_streams; i++) {
    if (bits & (1 << i)) {
    struct lola_stream *str = &pcm.streams[i];
    if (str.substream && str.running)
    snd_pcm_period_elapsed(str.substream);
    bits &= ~(1 << i);
    }
    }
    }
    static const struct snd_pcm_ops lola_pcm_ops = {
    .open = lola_pcm_open,
    .close = lola_pcm_close,
    .hw_params = lola_pcm_hw_params,
    .hw_free = lola_pcm_hw_free,
    .prepare = lola_pcm_prepare,
    .trigger = lola_pcm_trigger,
    .pointer = lola_pcm_pointer,
    };
#[no_mangle]
pub unsafe extern "C" fn lola_create_pcm(chip: *mut lola) -> c_int {
    int lola_create_pcm(struct lola *chip)
    {
    struct snd_pcm *pcm;
    int i, err;
    for (i = 0; i < 2; i++) {
    chip.pcm[i].bdl =
    snd_devm_alloc_pages(&chip.pci.dev, SNDRV_DMA_TYPE_DEV,
    PAGE_SIZE);
    if (!chip.pcm[i].bdl)
    return -ENOMEM;
    }
    err = snd_pcm_new(chip.card, "Digigram Lola", 0,
    chip.pcm[SNDRV_PCM_STREAM_PLAYBACK].num_streams,
    chip.pcm[SNDRV_PCM_STREAM_CAPTURE].num_streams,
    &pcm);
    if (err < 0)
    return err;
    strscpy(pcm.name, "Digigram Lola", sizeof(pcm.name));
    pcm.private_data = chip;
    for (i = 0; i < 2; i++) {
    if (chip.pcm[i].num_streams)
    snd_pcm_set_ops(pcm, i, &lola_pcm_ops);
    }
// buffer pre-allocation
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_SG,
    &chip.pci.dev,
    1024 * 64, 32 * 1024 * 1024);
    return 0;
    }
//
    static int lola_init_stream(struct lola *chip, struct lola_stream *str,
    int idx, int nid, int dir)
    {
    unsigned int val;
    int err;
    str.nid = nid;
    str.index = idx;
    str.dsd = idx;
    if (dir == PLAY)
    str.dsd += MAX_STREAM_IN_COUNT;
    err = lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &val);
    if (err < 0) {
    dev_err(chip.card.dev, "Can't read wcaps for 0x%x\n", nid);
    return err;
    }
    if (dir == PLAY) {
// test TYPE and bits 0..11 (no test bit9 : Digital = 0/1)
    if ((val & 0x00f00dff) != 0x00000010) {
    dev_err(chip.card.dev,
    "Invalid wcaps 0x%x for 0x%x\n",
    val, nid);
    return -EINVAL;
    }
    } else {
// test TYPE and bits 0..11 (no test bit9 : Digital = 0/1)
// (bug : ignore bit8: Conn list = 0/1)
//
    if ((val & 0x00f00cff) != 0x00100010) {
    dev_err(chip.card.dev,
    "Invalid wcaps 0x%x for 0x%x\n",
    val, nid);
    return -EINVAL;
    }
// test bit9:DIGITAL and bit12:SRC_PRESENT
    if ((val & 0x00001200) == 0x00001200)
    chip.input_src_caps_mask |= (1 << idx);
    }
    err = lola_read_param(chip, nid, LOLA_PAR_STREAM_FORMATS, &val);
    if (err < 0) {
    dev_err(chip.card.dev, "Can't read FORMATS 0x%x\n", nid);
    return err;
    }
    val &= 3;
    if (val == 3)
    str.can_float = true;
    if (!(val & 1)) {
    dev_err(chip.card.dev,
    "Invalid formats 0x%x for 0x%x", val, nid);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lola_init_pcm(chip: *mut lola, dir: c_int, nidp: *mut c_int) -> c_int {
    int lola_init_pcm(struct lola *chip, int dir, int *nidp)
    {
    struct lola_pcm *pcm = &chip.pcm[dir];
    int i, nid, err;
    nid = *nidp;
    for (i = 0; i < pcm.num_streams; i++, nid++) {
    err = lola_init_stream(chip, &pcm.streams[i], i, nid, dir);
    if (err < 0)
    return err;
    }
// nidp = nid;
    return 0;
    }
