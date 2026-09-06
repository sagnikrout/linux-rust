//! Automatically rewritten from C to Rust
//! Source: sound/drivers/aloop.c
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
// Loopback soundcard
//
// Original code:
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
// More accurate positioning and full-duplex support:
// Copyright (c) Ahmet İnan <ainan at mathematik.uni-freiburg.de>
//
// Major (almost complete) rewrite:
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
//
// A next major update in 2010 (separate timers for playback and capture):
// Copyright (c) Jaroslav Kysela <perex@perex.cz>
//

    MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("A loopback soundcard");
    MODULE_LICENSE("GPL");
pub const MAX_PCM_SUBSTREAMS: c_int = 8;
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = {1, [1 ... (SNDRV_CARDS - 1)] = 0};
    static int pcm_substreams[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS - 1)] = 8};
    static int pcm_notify[SNDRV_CARDS];
    static char *timer_source[SNDRV_CARDS];
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for loopback soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for loopback soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable this loopback soundcard.");
    module_param_array(pcm_substreams, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(pcm_substreams, "PCM substreams # (1-8) for loopback driver.");
    module_param_array(pcm_notify, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(pcm_notify, "Break capture when PCM format/rate/channels changes.");
    module_param_array(timer_source, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(timer_source, "Sound card name or number and device/subdevice number of timer to be used. Empty string for jiffies timer [default], 'hrtimer' for high-resolution timer.");
pub const NO_PITCH: c_int = 100000;

    struct loopback_cable;
    struct loopback_pcm;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopback_ops {
// optional
// call in loopback->cable_lock
//
    pub dpcm): *mut *mut int (open)(struct loopback_pcm,
// required
// call in cable->lock
//
    pub dpcm): *mut *mut int (start)(struct loopback_pcm,
// required
// call in cable->lock
//
    pub dpcm): *mut *mut int (stop)(struct loopback_pcm,
// optional
    pub dpcm): *mut *mut int (stop_sync)(struct loopback_pcm,
// optional
    pub dpcm): *mut *mut int (close_substream)(struct loopback_pcm,
// optional
// call in loopback->cable_lock
//
    pub dpcm): *mut *mut int (close_cable)(struct loopback_pcm,
// optional
// call in cable->lock
//
    pub cable): *mut *mut unsigned int (pos_update)(struct loopback_cable,
// optional
    void (*dpcm_info)(struct loopback_pcm *dpcm,
    pub buffer): *mut snd_info_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopback_cable {
    pub lock: spinlock_t,
    pub streams: [*mut loopback_pcm; 2],
// in-flight peer stops running outside cable->lock
    pub stop_count: snd_refcount,
    pub hw: snd_pcm_hardware,
// flags
    pub valid: c_uint,
    pub running: c_uint,
    pub pause: c_uint,
// timer specific
    pub ops: *const loopback_ops,
// If sound timer is used
    struct {
    pub stream: c_int,
    pub id: snd_timer_id,
    pub event_work: work_struct,
    pub instance: *mut snd_timer_instance,
    pub snd_timer: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopback_setup {
    pub 1: unsigned int notify:,
    pub rate_shift: c_uint,
    pub format: snd_pcm_format_t,
    pub rate: c_uint,
    pub access: snd_pcm_access_t,
    pub channels: c_uint,
    pub active_id: snd_ctl_elem_id,
    pub format_id: snd_ctl_elem_id,
    pub rate_id: snd_ctl_elem_id,
    pub channels_id: snd_ctl_elem_id,
    pub access_id: snd_ctl_elem_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopback {
    pub card: *mut snd_card,
    pub cable_lock: mutex,
    pub cables: [*mut loopback_cable; MAX_PCM_SUBSTREAMS][2],
    pub pcm: [*mut snd_pcm; 2],
    pub setup: [loopback_setup; MAX_PCM_SUBSTREAMS][2],
    pub timer_source: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopback_pcm {
    pub loopback: *mut loopback,
    pub substream: *mut snd_pcm_substream,
    pub cable: *mut loopback_cable,
    pub pcm_buffer_size: c_uint,
    pub /: *mut *mut unsigned int buf_pos; / position in buffer,
    pub silent_size: c_uint,
// PCM parameters
    pub pcm_period_size: c_uint,
    pub /: *mut *mut unsigned int pcm_bps; / bytes per second,
    pub /: *mut *mut *mut unsigned int pcm_salign; / bytes per sample  channels,
    pub /: *mut *mut unsigned int pcm_rate_shift; / rate shift value,
// flags
    pub :1: unsigned int period_update_pending,
// timer stuff
    pub jiffies: *mut *mut unsigned int irq_pos; / fractional IRQ position in,
// ticks
//
    pub /: *mut *mut unsigned int period_size_frac; / period size in jiffies ticks,
    pub last_drift: c_uint,
    pub last_jiffies: c_ulong,
// If jiffies / hrtimer is used
    pub timer: timer_list,

    pub hrtimer: hrtimer,

// size of per channel buffer in case of non-interleaved access
    pub channel_buf_n: c_uint,
}

    static struct platform_device *devices[SNDRV_CARDS];
#[no_mangle]
pub unsafe extern "C" fn byte_pos(dpcm: *mut loopback_pcm, x: c_uint) -> c_uint {
    static inline unsigned int byte_pos(struct loopback_pcm *dpcm, unsigned int x)
    {
    if (dpcm.pcm_rate_shift == NO_PITCH) {
    x /= HZ;
    } else {
    x = div_u64(NO_PITCH * (unsigned long long)x,
    HZ * (unsigned long long)dpcm.pcm_rate_shift);
    }
    return x - (x % dpcm.pcm_salign);
    }
#[no_mangle]
pub unsafe extern "C" fn frac_pos(dpcm: *mut loopback_pcm, x: c_uint) -> c_uint {
    static inline unsigned int frac_pos(struct loopback_pcm *dpcm, unsigned int x)
    {
    if (dpcm.pcm_rate_shift == NO_PITCH) {	/* no pitch */
    return x * HZ;
    } else {
    x = div_u64(dpcm.pcm_rate_shift * (unsigned long long)x * HZ,
    NO_PITCH);
    }
    return x;
    }
    static inline struct loopback_setup *get_setup(struct loopback_pcm *dpcm)
    {
    let mut device: c_int = dpcm.substream.pstr.pcm.device;
    if (dpcm.substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    device ^= 1;
    return &dpcm.loopback.setup[dpcm.substream.number][device];
    }
#[no_mangle]
pub unsafe extern "C" fn get_notify(dpcm: *mut loopback_pcm) -> c_uint {
    static inline unsigned int get_notify(struct loopback_pcm *dpcm)
    {
    return get_setup(dpcm).notify;
    }
#[no_mangle]
pub unsafe extern "C" fn get_rate_shift(dpcm: *mut loopback_pcm) -> c_uint {
    static inline unsigned int get_rate_shift(struct loopback_pcm *dpcm)
    {
    return get_setup(dpcm).rate_shift;
    }
// call in cable->lock
#[no_mangle]
unsafe extern "C" fn loopback_jiffies_timer_start(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_jiffies_timer_start(struct loopback_pcm *dpcm)
    {
    unsigned long tick;
    let mut rate_shift: c_uint = get_rate_shift(dpcm);
    if (rate_shift != dpcm.pcm_rate_shift) {
    dpcm.pcm_rate_shift = rate_shift;
    dpcm.period_size_frac = frac_pos(dpcm, dpcm.pcm_period_size);
    }
    if (dpcm.period_size_frac <= dpcm.irq_pos) {
    dpcm.irq_pos %= dpcm.period_size_frac;
    dpcm.period_update_pending = 1;
    }
    tick = dpcm.period_size_frac - dpcm.irq_pos;
    tick = DIV_ROUND_UP(tick, dpcm.pcm_bps);
    mod_timer(&dpcm.timer, jiffies + tick);
    return 0;
    }

// call in cable->lock
#[no_mangle]
unsafe extern "C" fn loopback_hrtimer_start(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_hrtimer_start(struct loopback_pcm *dpcm)
    {
    unsigned long tick;
    let mut rate_shift: c_uint = get_rate_shift(dpcm);
    if (rate_shift != dpcm.pcm_rate_shift) {
    dpcm.pcm_rate_shift = rate_shift;
    dpcm.period_size_frac = frac_pos(dpcm, dpcm.pcm_period_size);
    }
    if (dpcm.period_size_frac <= dpcm.irq_pos) {
    dpcm.irq_pos %= dpcm.period_size_frac;
    dpcm.period_update_pending = 1;
    }
    tick = dpcm.period_size_frac - dpcm.irq_pos;
    tick = DIV_ROUND_UP(tick, dpcm.pcm_bps);
    hrtimer_start(&dpcm.hrtimer,
    ns_to_ktime(div_u64((u64)tick * NSEC_PER_SEC, HZ)),
    HRTIMER_MODE_REL_SOFT);
    return 0;
    }

// call in cable->lock
#[no_mangle]
unsafe extern "C" fn loopback_snd_timer_start(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_snd_timer_start(struct loopback_pcm *dpcm)
    {
    struct loopback_cable *cable = dpcm.cable;
    int err;
// Loopback device has to use same period as timer card. Therefore
// wake up for each snd_pcm_period_elapsed() call of timer card.
//
    err = snd_timer_start(cable.snd_timer.instance, 1);
    if (err < 0) {
// do not report error if trying to start but already
// running. For example called by opposite substream
// of the same cable
//
    if (err == -EBUSY)
    return 0;
    pcm_err(dpcm.substream.pcm,
    "snd_timer_start(%d,%d,%d) failed with %d",
    cable.snd_timer.id.card,
    cable.snd_timer.id.device,
    cable.snd_timer.id.subdevice,
    err);
    }
    return err;
    }
// call in cable->lock
#[no_mangle]
pub unsafe extern "C" fn loopback_jiffies_timer_stop(dpcm: *mut loopback_pcm) -> c_int {
    static inline int loopback_jiffies_timer_stop(struct loopback_pcm *dpcm)
    {
    timer_delete(&dpcm.timer);
    dpcm.timer.expires = 0;
    return 0;
    }

// call in cable->lock
#[no_mangle]
pub unsafe extern "C" fn loopback_hrtimer_stop(dpcm: *mut loopback_pcm) -> c_int {
    static inline int loopback_hrtimer_stop(struct loopback_pcm *dpcm)
    {
    hrtimer_try_to_cancel(&dpcm.hrtimer);
    return 0;
    }

// call in cable->lock
#[no_mangle]
unsafe extern "C" fn loopback_snd_timer_stop(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_snd_timer_stop(struct loopback_pcm *dpcm)
    {
    struct loopback_cable *cable = dpcm.cable;
    int err;
// only stop if both devices (playback and capture) are not running
    if (cable.running ^ cable.pause)
    return 0;
    err = snd_timer_stop(cable.snd_timer.instance);
    if (err < 0) {
    pcm_err(dpcm.substream.pcm,
    "snd_timer_stop(%d,%d,%d) failed with %d",
    cable.snd_timer.id.card,
    cable.snd_timer.id.device,
    cable.snd_timer.id.subdevice,
    err);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn loopback_jiffies_timer_stop_sync(dpcm: *mut loopback_pcm) -> c_int {
    static inline int loopback_jiffies_timer_stop_sync(struct loopback_pcm *dpcm)
    {
    timer_delete_sync(&dpcm.timer);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn loopback_hrtimer_stop_sync(dpcm: *mut loopback_pcm) -> c_int {
    static inline int loopback_hrtimer_stop_sync(struct loopback_pcm *dpcm)
    {
    hrtimer_cancel(&dpcm.hrtimer);
    return 0;
    }

// call in loopback->cable_lock
#[no_mangle]
unsafe extern "C" fn loopback_snd_timer_close_cable(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_snd_timer_close_cable(struct loopback_pcm *dpcm)
    {
    struct loopback_cable *cable = dpcm.cable;
// snd_timer was not opened
    if (!cable.snd_timer.instance)
    return 0;
// will only be called from free_cable() when other stream was
// already closed. Other stream cannot be reopened as long as
// loopback->cable_lock is locked. Therefore no need to lock
// cable->lock;
//
    snd_timer_close(cable.snd_timer.instance);
// wait till drain work has finished if requested
    cancel_work_sync(&cable.snd_timer.event_work);
    snd_timer_instance_free(cable.snd_timer.instance);
    memset(&cable.snd_timer, 0, sizeof(cable.snd_timer));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_access_interleaved(access: snd_pcm_access_t) -> bool {
    static bool is_access_interleaved(snd_pcm_access_t access)
    {
    switch (access) {
    case SNDRV_PCM_ACCESS_MMAP_INTERLEAVED:
    case SNDRV_PCM_ACCESS_RW_INTERLEAVED:
    return true;
    default:
    return false;
    }
    };
#[no_mangle]
unsafe extern "C" fn loopback_check_format(cable: *mut loopback_cable, stream: c_int) -> c_int {
    static int loopback_check_format(struct loopback_cable *cable, int stream)
    {
    struct loopback_pcm *dpcm_play, *dpcm_capt;
    struct snd_pcm_runtime *runtime, *cruntime;
    struct loopback_setup *setup;
    struct snd_card *card;
    let mut stop_capture: bool = false;
    int check;
    scoped_guard(spinlock_irqsave, &cable.lock) {
    dpcm_play = cable.streams[SNDRV_PCM_STREAM_PLAYBACK];
    dpcm_capt = cable.streams[SNDRV_PCM_STREAM_CAPTURE];
    if (cable.valid != CABLE_VALID_BOTH) {
    if (stream == SNDRV_PCM_STREAM_CAPTURE || !dpcm_play)
    return 0;
    } else {
    if (!dpcm_play || !dpcm_capt)
    return -EIO;
    runtime = dpcm_play.substream.runtime;
    cruntime = dpcm_capt.substream.runtime;
    if (!runtime || !cruntime)
    return -EIO;
    check = runtime.format != cruntime.format ||
    runtime.rate != cruntime.rate ||
    runtime.channels != cruntime.channels ||
    is_access_interleaved(runtime.access) !=
    is_access_interleaved(cruntime.access);
    if (!check)
    return 0;
    if (stream == SNDRV_PCM_STREAM_CAPTURE)
    return -EIO;
#[no_mangle]
pub unsafe extern "C" fn if(SNDRV_PCM_STATE_RUNNING: cruntime->state ==) -> else {
// close must not free the peer runtime below
    snd_refcount_get(&cable.stop_count);
    stop_capture = true;
    }
    }
    setup = get_setup(dpcm_play);
    card = dpcm_play.loopback.card;
    runtime = dpcm_play.substream.runtime;
    if (setup.format != runtime.format) {
    snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE,
    &setup.format_id);
    setup.format = runtime.format;
    }
    if (setup.rate != runtime.rate) {
    snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE,
    &setup.rate_id);
    setup.rate = runtime.rate;
    }
    if (setup.channels != runtime.channels) {
    snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE,
    &setup.channels_id);
    setup.channels = runtime.channels;
    }
    if (is_access_interleaved(setup.access) !=
    is_access_interleaved(runtime.access)) {
    snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE,
    &setup.access_id);
    setup.access = runtime.access;
    }
    }
    if (stop_capture) {
    snd_pcm_stop(dpcm_capt.substream, SNDRV_PCM_STATE_DRAINING);
    snd_refcount_put(&cable.stop_count);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loopback_active_notify(dpcm: *mut loopback_pcm) {
    static void loopback_active_notify(struct loopback_pcm *dpcm)
    {
    snd_ctl_notify(dpcm.loopback.card,
    SNDRV_CTL_EVENT_MASK_VALUE,
    &get_setup(dpcm).active_id);
    }
#[no_mangle]
unsafe extern "C" fn loopback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    static int loopback_trigger(struct snd_pcm_substream *substream, int cmd)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct loopback_pcm *dpcm = runtime.private_data;
    struct loopback_cable *cable = dpcm.cable;
    let mut err: c_int = 0, stream = 1 << substream.stream;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    err = loopback_check_format(cable, substream.stream);
    if (err < 0)
    return err;
    dpcm.last_jiffies = jiffies;
    dpcm.pcm_rate_shift = 0;
    dpcm.last_drift = 0;
    scoped_guard(spinlock, &cable.lock) {
    cable.running |= stream;
    cable.pause &= ~stream;
    err = cable.ops.start(dpcm);
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    loopback_active_notify(dpcm);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    scoped_guard(spinlock, &cable.lock) {
    cable.running &= ~stream;
    cable.pause &= ~stream;
    err = cable.ops.stop(dpcm);
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    loopback_active_notify(dpcm);
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    scoped_guard(spinlock, &cable.lock) {
    cable.pause |= stream;
    err = cable.ops.stop(dpcm);
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    loopback_active_notify(dpcm);
    break;
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    case SNDRV_PCM_TRIGGER_RESUME:
    scoped_guard(spinlock, &cable.lock) {
    dpcm.last_jiffies = jiffies;
    cable.pause &= ~stream;
    err = cable.ops.start(dpcm);
    }
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK)
    loopback_active_notify(dpcm);
    break;
    default:
    return -EINVAL;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn params_change(substream: *mut snd_pcm_substream) {
    static void params_change(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct loopback_pcm *dpcm = runtime.private_data;
    struct loopback_cable *cable = dpcm.cable;
    cable.hw.formats = pcm_format_to_bits(runtime.format);
    cable.hw.rate_min = runtime.rate;
    cable.hw.rate_max = runtime.rate;
    cable.hw.channels_min = runtime.channels;
    cable.hw.channels_max = runtime.channels;
    if (cable.snd_timer.instance) {
    cable.hw.period_bytes_min =
    frames_to_bytes(runtime, runtime.period_size);
    cable.hw.period_bytes_max = cable.hw.period_bytes_min;
    }
    }
#[no_mangle]
unsafe extern "C" fn loopback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int loopback_prepare(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct loopback_pcm *dpcm = runtime.private_data;
    struct loopback_cable *cable = dpcm.cable;
    int err, bps, salign;
    if (cable.ops.stop_sync) {
    err = cable.ops.stop_sync(dpcm);
    if (err < 0)
    return err;
    }
    salign = (snd_pcm_format_physical_width(runtime.format) *
    runtime.channels) / 8;
    bps = salign * runtime.rate;
    if (bps <= 0 || salign <= 0)
    return -EINVAL;
    dpcm.buf_pos = 0;
    dpcm.pcm_buffer_size = frames_to_bytes(runtime, runtime.buffer_size);
    dpcm.channel_buf_n = dpcm.pcm_buffer_size / runtime.channels;
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE) {
// clear capture buffer
    dpcm.silent_size = dpcm.pcm_buffer_size;
    snd_pcm_format_set_silence(runtime.format, runtime.dma_area,
    runtime.buffer_size * runtime.channels);
    }
    dpcm.irq_pos = 0;
    dpcm.period_update_pending = 0;
    dpcm.pcm_bps = bps;
    dpcm.pcm_salign = salign;
    dpcm.pcm_period_size = frames_to_bytes(runtime, runtime.period_size);
    guard(mutex)(&dpcm.loopback.cable_lock);
    if (!(cable.valid & ~(1 << substream.stream)) ||
    (get_setup(dpcm).notify &&
    substream.stream == SNDRV_PCM_STREAM_PLAYBACK))
    params_change(substream);
    cable.valid |= 1 << substream.stream;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clear_capture_buf(dpcm: *mut loopback_pcm, bytes: c_uint) {
    static void clear_capture_buf(struct loopback_pcm *dpcm, unsigned int bytes)
    {
    struct snd_pcm_runtime *runtime = dpcm.substream.runtime;
    char *dst = runtime.dma_area;
    let mut dst_off: c_uint = dpcm.buf_pos;
    if (dpcm.silent_size >= dpcm.pcm_buffer_size)
    return;
    if (dpcm.silent_size + bytes > dpcm.pcm_buffer_size)
    bytes = dpcm.pcm_buffer_size - dpcm.silent_size;
    for (;;) {
    let mut size: c_uint = bytes;
    if (dst_off + size > dpcm.pcm_buffer_size)
    size = dpcm.pcm_buffer_size - dst_off;
    snd_pcm_format_set_silence(runtime.format, dst + dst_off,
    bytes_to_frames(runtime, size) *
    runtime.channels);
    dpcm.silent_size += size;
    bytes -= size;
    if (!bytes)
    break;
    dst_off = 0;
    }
    }
    static void copy_play_buf_part_n(struct loopback_pcm *play, struct loopback_pcm *capt,
    unsigned int size, unsigned int src_off, unsigned int dst_off)
    {
    let mut channels: c_uint = capt.substream.runtime.channels;
    let mut size_p_ch: c_uint = size / channels;
    let mut src_off_ch: c_uint = src_off / channels;
    let mut dst_off_ch: c_uint = dst_off / channels;
    int i;
    for (i = 0; i < channels; i++) {
    memcpy(capt.substream.runtime.dma_area + capt.channel_buf_n * i + dst_off_ch,
    play.substream.runtime.dma_area + play.channel_buf_n * i + src_off_ch,
    size_p_ch);
    }
    }
    static void copy_play_buf(struct loopback_pcm *play,
    struct loopback_pcm *capt,
    unsigned int bytes)
    {
    struct snd_pcm_runtime *runtime = play.substream.runtime;
    char *src = runtime.dma_area;
    char *dst = capt.substream.runtime.dma_area;
    let mut src_off: c_uint = play.buf_pos;
    let mut dst_off: c_uint = capt.buf_pos;
    let mut clear_bytes: c_uint = 0;
// check if playback is draining, trim the capture copy size
// when our pointer is at the end of playback ring buffer
    if (runtime.state == SNDRV_PCM_STATE_DRAINING &&
    snd_pcm_playback_hw_avail(runtime) < runtime.buffer_size) {
    snd_pcm_uframes_t appl_ptr, appl_ptr1, diff;
    appl_ptr = appl_ptr1 = runtime.control.appl_ptr;
    appl_ptr1 -= appl_ptr1 % runtime.buffer_size;
    appl_ptr1 += play.buf_pos / play.pcm_salign;
    if (appl_ptr < appl_ptr1)
    appl_ptr1 -= runtime.buffer_size;
    diff = (appl_ptr - appl_ptr1) * play.pcm_salign;
    if (diff < bytes) {
    clear_bytes = bytes - diff;
    bytes = diff;
    }
    }
    for (;;) {
    let mut size: c_uint = bytes;
    if (src_off + size > play.pcm_buffer_size)
    size = play.pcm_buffer_size - src_off;
    if (dst_off + size > capt.pcm_buffer_size)
    size = capt.pcm_buffer_size - dst_off;
    if (!is_access_interleaved(runtime.access))
    copy_play_buf_part_n(play, capt, size, src_off, dst_off);
    else
    memcpy(dst + dst_off, src + src_off, size);
    capt.silent_size = 0;
    bytes -= size;
    if (!bytes)
    break;
    src_off = (src_off + size) % play.pcm_buffer_size;
    dst_off = (dst_off + size) % capt.pcm_buffer_size;
    }
    if (clear_bytes > 0) {
    clear_capture_buf(capt, clear_bytes);
    capt.silent_size = 0;
    }
    }
    static inline unsigned int bytepos_delta(struct loopback_pcm *dpcm,
    unsigned int jiffies_delta)
    {
    unsigned long last_pos;
    unsigned int delta;
    last_pos = byte_pos(dpcm, dpcm.irq_pos);
    dpcm.irq_pos += jiffies_delta * dpcm.pcm_bps;
    delta = byte_pos(dpcm, dpcm.irq_pos) - last_pos;
    if (delta >= dpcm.last_drift)
    delta -= dpcm.last_drift;
    dpcm.last_drift = 0;
    if (dpcm.irq_pos >= dpcm.period_size_frac) {
    dpcm.irq_pos %= dpcm.period_size_frac;
    dpcm.period_update_pending = 1;
    }
    return delta;
    }
    static inline void bytepos_finish(struct loopback_pcm *dpcm,
    unsigned int delta)
    {
    dpcm.buf_pos += delta;
    dpcm.buf_pos %= dpcm.pcm_buffer_size;
    }
// call in cable->lock
    static unsigned int loopback_jiffies_timer_pos_update
    (struct loopback_cable *cable)
    {
    struct loopback_pcm *dpcm_play =
    cable.streams[SNDRV_PCM_STREAM_PLAYBACK];
    struct loopback_pcm *dpcm_capt =
    cable.streams[SNDRV_PCM_STREAM_CAPTURE];
    let mut delta_play: c_ulong = 0, delta_capt = 0, cur_jiffies;
    unsigned int running, count1, count2;
    cur_jiffies = jiffies;
    running = cable.running ^ cable.pause;
    if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) {
    delta_play = cur_jiffies - dpcm_play.last_jiffies;
    dpcm_play.last_jiffies += delta_play;
    }
    if (running & (1 << SNDRV_PCM_STREAM_CAPTURE)) {
    delta_capt = cur_jiffies - dpcm_capt.last_jiffies;
    dpcm_capt.last_jiffies += delta_capt;
    }
    if (delta_play == 0 && delta_capt == 0)
    goto unlock;
    if (delta_play > delta_capt) {
    count1 = bytepos_delta(dpcm_play, delta_play - delta_capt);
    bytepos_finish(dpcm_play, count1);
    delta_play = delta_capt;
    } else if (delta_play < delta_capt) {
    count1 = bytepos_delta(dpcm_capt, delta_capt - delta_play);
    clear_capture_buf(dpcm_capt, count1);
    bytepos_finish(dpcm_capt, count1);
    delta_capt = delta_play;
    }
    if (delta_play == 0 && delta_capt == 0)
    goto unlock;
// note delta_capt == delta_play at this moment
    count1 = bytepos_delta(dpcm_play, delta_play);
    count2 = bytepos_delta(dpcm_capt, delta_capt);
    if (count1 < count2) {
    dpcm_capt.last_drift = count2 - count1;
    count1 = count2;
    } else if (count1 > count2) {
    dpcm_play.last_drift = count1 - count2;
    }
    copy_play_buf(dpcm_play, dpcm_capt, count1);
    bytepos_finish(dpcm_play, count1);
    bytepos_finish(dpcm_capt, count1);
    unlock:
    return running;
    }
#[no_mangle]
unsafe extern "C" fn loopback_jiffies_timer_function(t: *mut timer_list) {
    static void loopback_jiffies_timer_function(struct timer_list *t)
    {
    struct loopback_pcm *dpcm = timer_container_of(dpcm, t, timer);
    let mut period_elapsed: bool = false;
    scoped_guard(spinlock_irqsave, &dpcm.cable.lock) {
    if (loopback_jiffies_timer_pos_update(dpcm.cable) &
    (1 << dpcm.substream.stream)) {
    loopback_jiffies_timer_start(dpcm);
    if (dpcm.period_update_pending) {
    dpcm.period_update_pending = 0;
    period_elapsed = true;
    }
    }
    }
    if (period_elapsed)
    snd_pcm_period_elapsed(dpcm.substream);
    }

#[no_mangle]
unsafe extern "C" fn loopback_hrtimer_function(t: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart loopback_hrtimer_function(struct hrtimer *t)
    {
    struct loopback_pcm *dpcm = container_of(t, struct loopback_pcm, hrtimer);
    let mut period_elapsed: bool = false;
    scoped_guard(spinlock_irqsave, &dpcm.cable.lock) {
    if (loopback_jiffies_timer_pos_update(dpcm.cable) &
    (1 << dpcm.substream.stream)) {
    loopback_hrtimer_start(dpcm);
    if (dpcm.period_update_pending) {
    dpcm.period_update_pending = 0;
    period_elapsed = true;
    }
    }
    }
    if (period_elapsed)
    snd_pcm_period_elapsed(dpcm.substream);
    return HRTIMER_NORESTART;
    }

// call in cable->lock
    static int loopback_snd_timer_check_resolution(struct snd_pcm_runtime *runtime,
    unsigned long resolution)
    {
    if (resolution != runtime.timer_resolution) {
    struct loopback_pcm *dpcm = runtime.private_data;
    struct loopback_cable *cable = dpcm.cable;
// Worst case estimation of possible values for resolution
// resolution <= (512 * 1024) frames / 8kHz in nsec
// resolution <= 65.536.000.000 nsec
//
// period_size <= 65.536.000.000 nsec / 1000nsec/usec * 192kHz +
// 500.000
// period_size <= 12.582.912.000.000  <64bit
// / 1.000.000 usec/sec
//
    snd_pcm_uframes_t period_size_usec =
    resolution / 1000 * runtime.rate;
// round to nearest sample rate
    snd_pcm_uframes_t period_size =
    (period_size_usec + 500 * 1000) / (1000 * 1000);
    pcm_err(dpcm.substream.pcm,
    "Period size (%lu frames) of loopback device is not corresponding to timer resolution (%lu nsec = %lu frames) of card timer %d,%d,%d. Use period size of %lu frames for loopback device.",
    runtime.period_size, resolution, period_size,
    cable.snd_timer.id.card,
    cable.snd_timer.id.device,
    cable.snd_timer.id.subdevice,
    period_size);
    return -EINVAL;
    }
    return 0;
    }
    static void loopback_snd_timer_period_elapsed(struct loopback_cable *cable,
    int event,
    unsigned long resolution)
    {
    struct loopback_pcm *dpcm_play, *dpcm_capt;
    struct snd_pcm_substream *substream_play, *substream_capt;
    struct snd_pcm_runtime *valid_runtime;
    unsigned int running, elapsed_bytes;
    let mut xrun: bool = false;
    scoped_guard(spinlock_irqsave, &cable.lock) {
    running = cable.running ^ cable.pause;
// no need to do anything if no stream is running
    if (!running)
    return;
    dpcm_play = cable.streams[SNDRV_PCM_STREAM_PLAYBACK];
    dpcm_capt = cable.streams[SNDRV_PCM_STREAM_CAPTURE];
    if (event == SNDRV_TIMER_EVENT_MSTOP) {
    if (!dpcm_play ||
    dpcm_play.substream.runtime.state !=
    SNDRV_PCM_STATE_DRAINING)
    return;
    }
    substream_play = (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) ?
    dpcm_play.substream : core::ptr::null_mut();
    substream_capt = (running & (1 << SNDRV_PCM_STREAM_CAPTURE)) ?
    dpcm_capt.substream : core::ptr::null_mut();
    valid_runtime = (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) ?
    dpcm_play.substream.runtime :
    dpcm_capt.substream.runtime;
// resolution is only valid for SNDRV_TIMER_EVENT_TICK events
    if (event == SNDRV_TIMER_EVENT_TICK) {
// The hardware rules guarantee that playback and capture period
// are the same. Therefore only one device has to be checked
// here.
//
    if (loopback_snd_timer_check_resolution(valid_runtime,
    resolution) < 0) {
    xrun = true;
    break;
    }
    }
    elapsed_bytes = frames_to_bytes(valid_runtime,
    valid_runtime.period_size);
// The same timer interrupt is used for playback and capture device
    if ((running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) &&
    (running & (1 << SNDRV_PCM_STREAM_CAPTURE))) {
    copy_play_buf(dpcm_play, dpcm_capt, elapsed_bytes);
    bytepos_finish(dpcm_play, elapsed_bytes);
    bytepos_finish(dpcm_capt, elapsed_bytes);
    } else if (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) {
    bytepos_finish(dpcm_play, elapsed_bytes);
    } else if (running & (1 << SNDRV_PCM_STREAM_CAPTURE)) {
    clear_capture_buf(dpcm_capt, elapsed_bytes);
    bytepos_finish(dpcm_capt, elapsed_bytes);
    }
    }
    if (xrun) {
    if (substream_play)
    snd_pcm_stop_xrun(substream_play);
    if (substream_capt)
    snd_pcm_stop_xrun(substream_capt);
    return;
    }
    if (substream_play)
    snd_pcm_period_elapsed(substream_play);
    if (substream_capt)
    snd_pcm_period_elapsed(substream_capt);
    }
    static void loopback_snd_timer_function(struct snd_timer_instance *timeri,
    unsigned long resolution,
    unsigned long ticks)
    {
    struct loopback_cable *cable = timeri.callback_data;
    loopback_snd_timer_period_elapsed(cable, SNDRV_TIMER_EVENT_TICK,
    resolution);
    }
#[no_mangle]
unsafe extern "C" fn loopback_snd_timer_work(work: *mut work_struct) {
    static void loopback_snd_timer_work(struct work_struct *work)
    {
    struct loopback_cable *cable;
    cable = container_of(work, struct loopback_cable, snd_timer.event_work);
    loopback_snd_timer_period_elapsed(cable, SNDRV_TIMER_EVENT_MSTOP, 0);
    }
    static void loopback_snd_timer_event(struct snd_timer_instance *timeri,
    int event,
    struct timespec64 *tstamp,
    unsigned long resolution)
    {
// Do not lock cable->lock here because timer->lock is already hold.
// There are other functions which first lock cable->lock and than
// timer->lock e.g.
// loopback_trigger()
// spin_lock(&cable->lock)
// loopback_snd_timer_start()
// snd_timer_start()
// spin_lock(&timer->lock)
// Therefore when using the oposit order of locks here it could result
// in a deadlock.
//
    if (event == SNDRV_TIMER_EVENT_MSTOP) {
    struct loopback_cable *cable = timeri.callback_data;
// sound card of the timer was stopped. Therefore there will not
// be any further timer callbacks. Due to this forward audio
// data from here if in draining state. When still in running
// state the streaming will be aborted by the usual timeout. It
// should not be aborted here because may be the timer sound
// card does only a recovery and the timer is back soon.
// This work triggers loopback_snd_timer_work()
//
    schedule_work(&cable.snd_timer.event_work);
    }
    }
    static void loopback_jiffies_timer_dpcm_info(struct loopback_pcm *dpcm,
    struct snd_info_buffer *buffer)
    {
    snd_iprintf(buffer, "    update_pending:\t%u\n",
    dpcm.period_update_pending);
    snd_iprintf(buffer, "    irq_pos:\t\t%u\n", dpcm.irq_pos);
    snd_iprintf(buffer, "    period_frac:\t%u\n", dpcm.period_size_frac);
    snd_iprintf(buffer, "    last_jiffies:\t%lu (%lu)\n",
    dpcm.last_jiffies, jiffies);
    snd_iprintf(buffer, "    timer_expires:\t%lu\n", dpcm.timer.expires);
    }

    static void loopback_hrtimer_dpcm_info(struct loopback_pcm *dpcm,
    struct snd_info_buffer *buffer)
    {
    snd_iprintf(buffer, "    update_pending:\t%u\n",
    dpcm.period_update_pending);
    snd_iprintf(buffer, "    irq_pos:\t\t%u\n", dpcm.irq_pos);
    snd_iprintf(buffer, "    period_frac:\t%u\n", dpcm.period_size_frac);
    snd_iprintf(buffer, "    last_jiffies:\t%lu (%lu)\n",
    dpcm.last_jiffies, jiffies);
    snd_iprintf(buffer, "    timer_expires:\t%llu\n",
    ktime_to_ns(hrtimer_get_expires(&dpcm.hrtimer)));
    }

    static void loopback_snd_timer_dpcm_info(struct loopback_pcm *dpcm,
    struct snd_info_buffer *buffer)
    {
    struct loopback_cable *cable = dpcm.cable;
    snd_iprintf(buffer, "    sound timer:\thw:%d,%d,%d\n",
    cable.snd_timer.id.card,
    cable.snd_timer.id.device,
    cable.snd_timer.id.subdevice);
    snd_iprintf(buffer, "    timer open:\t\t%s\n",
    snd_pcm_direction_name(cable.snd_timer.stream));
    }
#[no_mangle]
unsafe extern "C" fn loopback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    static snd_pcm_uframes_t loopback_pointer(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct loopback_pcm *dpcm = runtime.private_data;
    snd_pcm_uframes_t pos;
    guard(spinlock)(&dpcm.cable.lock);
    if (dpcm.cable.ops.pos_update)
    dpcm.cable.ops.pos_update(dpcm.cable);
    pos = dpcm.buf_pos;
    return bytes_to_frames(runtime, pos);
    }
    static const struct snd_pcm_hardware loopback_pcm_hardware =
    {
    .info =		(SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP |
    SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_PAUSE |
    SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_NONINTERLEAVED),
    .formats =	(SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_BE |
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_3BE |
    SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S32_BE |
    SNDRV_PCM_FMTBIT_FLOAT_LE | SNDRV_PCM_FMTBIT_FLOAT_BE |
    SNDRV_PCM_FMTBIT_DSD_U8 |
    SNDRV_PCM_FMTBIT_DSD_U16_LE | SNDRV_PCM_FMTBIT_DSD_U16_BE |
    SNDRV_PCM_FMTBIT_DSD_U32_LE | SNDRV_PCM_FMTBIT_DSD_U32_BE),
    .rates =	SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_768000,
    .rate_min =		8000,
    .rate_max =		768000,
    .channels_min =		1,
    .channels_max =		32,
    .buffer_bytes_max =	2 * 1024 * 1024,
    .period_bytes_min =	64,
// note check overflow in frac_pos() using pcm_rate_shift before
    changing period_bytes_max value */
    .period_bytes_max =	1024 * 1024,
    .periods_min =		1,
    .periods_max =		1024,
    .fifo_size =		0,
    };
#[no_mangle]
unsafe extern "C" fn loopback_runtime_free(runtime: *mut snd_pcm_runtime) {
    static void loopback_runtime_free(struct snd_pcm_runtime *runtime)
    {
    struct loopback_pcm *dpcm = runtime.private_data;
    kfree(dpcm);
    }
#[no_mangle]
unsafe extern "C" fn loopback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int loopback_hw_free(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct loopback_pcm *dpcm = runtime.private_data;
    struct loopback_cable *cable = dpcm.cable;
    guard(mutex)(&dpcm.loopback.cable_lock);
    cable.valid &= ~(1 << substream.stream);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_cable_index(substream: *mut snd_pcm_substream) -> c_uint {
    static unsigned int get_cable_index(struct snd_pcm_substream *substream)
    {
    if (!substream.pcm.device)
    return substream.stream;
    else
    return !substream.stream;
    }
    static int rule_format(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct loopback_pcm *dpcm = rule.private;
    struct loopback_cable *cable = dpcm.cable;
    struct snd_mask m;
    snd_mask_none(&m);
    scoped_guard(mutex, &dpcm.loopback.cable_lock) {
    m.bits[0] = (u_int32_t)cable.hw.formats;
    m.bits[1] = (u_int32_t)(cable.hw.formats >> 32);
    }
    return snd_mask_refine(hw_param_mask(params, rule.var), &m);
    }
    static int rule_rate(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct loopback_pcm *dpcm = rule.private;
    struct loopback_cable *cable = dpcm.cable;
    struct snd_interval t;
    scoped_guard(mutex, &dpcm.loopback.cable_lock) {
    t.min = cable.hw.rate_min;
    t.max = cable.hw.rate_max;
    }
    t.openmin = t.openmax = 0;
    t.integer = 0;
    return snd_interval_refine(hw_param_interval(params, rule.var), &t);
    }
    static int rule_channels(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct loopback_pcm *dpcm = rule.private;
    struct loopback_cable *cable = dpcm.cable;
    struct snd_interval t;
    scoped_guard(mutex, &dpcm.loopback.cable_lock) {
    t.min = cable.hw.channels_min;
    t.max = cable.hw.channels_max;
    }
    t.openmin = t.openmax = 0;
    t.integer = 0;
    return snd_interval_refine(hw_param_interval(params, rule.var), &t);
    }
    static int rule_period_bytes(struct snd_pcm_hw_params *params,
    struct snd_pcm_hw_rule *rule)
    {
    struct loopback_pcm *dpcm = rule.private;
    struct loopback_cable *cable = dpcm.cable;
    struct snd_interval t;
    scoped_guard(mutex, &dpcm.loopback.cable_lock) {
    t.min = cable.hw.period_bytes_min;
    t.max = cable.hw.period_bytes_max;
    }
    t.openmin = 0;
    t.openmax = 0;
    t.integer = 0;
    return snd_interval_refine(hw_param_interval(params, rule.var), &t);
    }
#[no_mangle]
unsafe extern "C" fn free_cable(substream: *mut snd_pcm_substream) {
    static void free_cable(struct snd_pcm_substream *substream)
    {
    struct loopback *loopback = substream.private_data;
    let mut dev: c_int = get_cable_index(substream);
    struct loopback_cable *cable;
    struct loopback_pcm *dpcm;
    bool other_alive;
    cable = loopback.cables[substream.number][dev];
    if (!cable)
    return;
    scoped_guard(spinlock_irq, &cable.lock) {
    cable.streams[substream.stream] = core::ptr::null_mut();
    other_alive = cable.streams[!substream.stream];
    }
// Pair with the stop_count increment in loopback_check_format().
    snd_refcount_sync(&cable.stop_count);
    if (other_alive)
    return;
    dpcm = substream.runtime.private_data;
    if (cable.ops && cable.ops.close_cable && dpcm)
    cable.ops.close_cable(dpcm);
// free the cable
    loopback.cables[substream.number][dev] = core::ptr::null_mut();
    kfree(cable);
    }
#[no_mangle]
unsafe extern "C" fn loopback_jiffies_timer_open(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_jiffies_timer_open(struct loopback_pcm *dpcm)
    {
    timer_setup(&dpcm.timer, loopback_jiffies_timer_function, 0);
    return 0;
    }
    static const struct loopback_ops loopback_jiffies_timer_ops = {
    .open = loopback_jiffies_timer_open,
    .start = loopback_jiffies_timer_start,
    .stop = loopback_jiffies_timer_stop,
    .stop_sync = loopback_jiffies_timer_stop_sync,
    .close_substream = loopback_jiffies_timer_stop_sync,
    .pos_update = loopback_jiffies_timer_pos_update,
    .dpcm_info = loopback_jiffies_timer_dpcm_info,
    };

#[no_mangle]
unsafe extern "C" fn loopback_hrtimer_open(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_hrtimer_open(struct loopback_pcm *dpcm)
    {
    hrtimer_setup(&dpcm.hrtimer, loopback_hrtimer_function,
    CLOCK_MONOTONIC, HRTIMER_MODE_REL_SOFT);
    return 0;
    }
    static const struct loopback_ops loopback_hrtimer_ops = {
    .open = loopback_hrtimer_open,
    .start = loopback_hrtimer_start,
    .stop = loopback_hrtimer_stop,
    .stop_sync = loopback_hrtimer_stop_sync,
    .close_substream = loopback_hrtimer_stop_sync,
    .pos_update = loopback_jiffies_timer_pos_update,
    .dpcm_info = loopback_hrtimer_dpcm_info,
    };

    static int loopback_parse_timer_id(const char *str,
    struct snd_timer_id *tid)
    {
// [<pref>:](<card name>|<card idx>)[{.,}<dev idx>[{.,}<subdev idx>]]
    let mut sep_dev: *const char  const = ".,";
    let mut sep_pref: *const char  const = ":";
    const char *name = str;
    char *sep, save = '\0';
    let mut card_idx: c_int = 0, dev = 0, subdev = 0;
    int err;
    sep = strpbrk(str, sep_pref);
    if (sep)
    name = sep + 1;
    sep = strpbrk(name, sep_dev);
    if (sep) {
    save = *sep;
// sep = '\0';
    }
    err = kstrtoint(name, 0, &card_idx);
    if (err == -EINVAL) {
// Must be the name, not number
    for (card_idx = 0; card_idx < snd_ecards_limit; card_idx++) {
    struct snd_card *card = snd_card_ref(card_idx);
    if (card) {
    if (!strcmp(card.id, name))
    err = 0;
    snd_card_unref(card);
    }
    if (!err)
    break;
    }
    }
    if (sep) {
// sep = save;
    if (!err) {
    char *sep2, save2 = '\0';
    sep2 = strpbrk(sep + 1, sep_dev);
    if (sep2) {
    save2 = *sep2;
// sep2 = '\0';
    }
    err = kstrtoint(sep + 1, 0, &dev);
    if (sep2) {
// sep2 = save2;
    if (!err)
    err = kstrtoint(sep2 + 1, 0, &subdev);
    }
    }
    }
    if (card_idx == -1)
    tid.dev_class = SNDRV_TIMER_CLASS_GLOBAL;
    if (!err && tid) {
    tid.card = card_idx;
    tid.device = dev;
    tid.subdevice = subdev;
    }
    return err;
    }
// call in loopback->cable_lock
#[no_mangle]
unsafe extern "C" fn loopback_snd_timer_open(dpcm: *mut loopback_pcm) -> c_int {
    static int loopback_snd_timer_open(struct loopback_pcm *dpcm)
    {
    let mut err: c_int = 0;
    struct snd_timer_id tid = {
    .dev_class = SNDRV_TIMER_CLASS_PCM,
    .dev_sclass = SNDRV_TIMER_SCLASS_APPLICATION,
    };
    struct snd_timer_instance *timeri;
    struct loopback_cable *cable = dpcm.cable;
// check if timer was already opened. It is only opened once
// per playback and capture subdevice (aka cable).
//
    if (cable.snd_timer.instance)
    goto exit;
    err = loopback_parse_timer_id(dpcm.loopback.timer_source, &tid);
    if (err < 0) {
    pcm_err(dpcm.substream.pcm,
    "Parsing timer source \'%s\' failed with %d",
    dpcm.loopback.timer_source, err);
    goto exit;
    }
    cable.snd_timer.stream = dpcm.substream.stream;
    cable.snd_timer.id = tid;
    timeri = snd_timer_instance_new(dpcm.loopback.card.id);
    if (!timeri) {
    err = -ENOMEM;
    goto exit;
    }
// The callback has to be called from another work. If
// SNDRV_TIMER_IFLG_FAST is specified it will be called from the
// snd_pcm_period_elapsed() call of the selected sound card.
// snd_pcm_period_elapsed() helds snd_pcm_stream_lock_irqsave().
// Due to our callback loopback_snd_timer_function() also calls
// snd_pcm_period_elapsed() which calls snd_pcm_stream_lock_irqsave().
// This would end up in a dead lock.
//
    timeri.flags |= SNDRV_TIMER_IFLG_AUTO;
    timeri.callback = loopback_snd_timer_function;
    timeri.callback_data = (void *)cable;
    timeri.ccallback = loopback_snd_timer_event;
// initialise a work used for draining
    INIT_WORK(&cable.snd_timer.event_work, loopback_snd_timer_work);
// The mutex loopback->cable_lock is kept locked.
// Therefore snd_timer_open() cannot be called a second time
// by the other device of the same cable.
// Therefore the following issue cannot happen:
// [proc1] Call loopback_timer_open() ->
// Unlock cable->lock for snd_timer_close/open() call
// [proc2] Call loopback_timer_open() -> snd_timer_open(),
// snd_timer_start()
// [proc1] Call snd_timer_open() and overwrite running timer
// instance
//
    err = snd_timer_open(timeri, &cable.snd_timer.id, current.pid);
    if (err < 0) {
    pcm_err(dpcm.substream.pcm,
    "snd_timer_open (%d,%d,%d) failed with %d",
    cable.snd_timer.id.card,
    cable.snd_timer.id.device,
    cable.snd_timer.id.subdevice,
    err);
    snd_timer_instance_free(timeri);
    goto exit;
    }
    cable.snd_timer.instance = timeri;
    exit:
    return err;
    }
// stop_sync() is not required for sound timer because it does not need to be
// restarted in loopback_prepare() on Xrun recovery
//
    static const struct loopback_ops loopback_snd_timer_ops = {
    .open = loopback_snd_timer_open,
    .start = loopback_snd_timer_start,
    .stop = loopback_snd_timer_stop,
    .close_cable = loopback_snd_timer_close_cable,
    .dpcm_info = loopback_snd_timer_dpcm_info,
    };
#[no_mangle]
unsafe extern "C" fn loopback_open(substream: *mut snd_pcm_substream) -> c_int {
    static int loopback_open(struct snd_pcm_substream *substream)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct loopback *loopback = substream.private_data;
    struct loopback_pcm *dpcm;
    struct loopback_cable *cable = core::ptr::null_mut();
    let mut err: c_int = 0;
    let mut dev: c_int = get_cable_index(substream);
    guard(mutex)(&loopback.cable_lock);
    dpcm = kzalloc_obj(*dpcm);
    if (!dpcm)
    return -ENOMEM;
    dpcm.loopback = loopback;
    dpcm.substream = substream;
    cable = loopback.cables[substream.number][dev];
    if (!cable) {
    cable = kzalloc_obj(*cable);
    if (!cable) {
    err = -ENOMEM;
    goto unlock;
    }
    spin_lock_init(&cable.lock);
    snd_refcount_init(&cable.stop_count);
    cable.hw = loopback_pcm_hardware;

    if (loopback.timer_source && !strcmp(loopback.timer_source, "hrtimer"))
    cable.ops = &loopback_hrtimer_ops;
    else

    if (loopback.timer_source && loopback.timer_source[0])
    cable.ops = &loopback_snd_timer_ops;
    else
    cable.ops = &loopback_jiffies_timer_ops;
    loopback.cables[substream.number][dev] = cable;
    }
    dpcm.cable = cable;
    runtime.private_data = dpcm;
    if (cable.ops.open) {
    err = cable.ops.open(dpcm);
    if (err < 0)
    goto unlock;
    }
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
// use dynamic rules based on actual runtime->hw values
// note that the default rules created in the PCM midlevel code
// are cached -> they do not reflect the actual state
    err = snd_pcm_hw_rule_add(runtime, 0,
    SNDRV_PCM_HW_PARAM_FORMAT,
    rule_format, dpcm,
    SNDRV_PCM_HW_PARAM_FORMAT, -1);
    if (err < 0)
    goto unlock;
    err = snd_pcm_hw_rule_add(runtime, 0,
    SNDRV_PCM_HW_PARAM_RATE,
    rule_rate, dpcm,
    SNDRV_PCM_HW_PARAM_RATE, -1);
    if (err < 0)
    goto unlock;
    err = snd_pcm_hw_rule_add(runtime, 0,
    SNDRV_PCM_HW_PARAM_CHANNELS,
    rule_channels, dpcm,
    SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if (err < 0)
    goto unlock;
// In case of sound timer the period time of both devices of the same
// loop has to be the same.
// This rule only takes effect if a sound timer was chosen
//
    if (cable.snd_timer.instance) {
    err = snd_pcm_hw_rule_add(runtime, 0,
    SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
    rule_period_bytes, dpcm,
    SNDRV_PCM_HW_PARAM_PERIOD_BYTES, -1);
    if (err < 0)
    goto unlock;
    }
// loopback_runtime_free() has not to be called if kfree(dpcm) was
// already called here. Otherwise it will end up with a double free.
//
    runtime.private_free = loopback_runtime_free;
    if (get_notify(dpcm))
    runtime.hw = loopback_pcm_hardware;
    else
    runtime.hw = cable.hw;
    scoped_guard(spinlock_irq, &cable.lock) {
    cable.streams[substream.stream] = dpcm;
    }
    unlock:
    if (err < 0) {
    free_cable(substream);
    kfree(dpcm);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn loopback_close(substream: *mut snd_pcm_substream) -> c_int {
    static int loopback_close(struct snd_pcm_substream *substream)
    {
    struct loopback *loopback = substream.private_data;
    struct loopback_pcm *dpcm = substream.runtime.private_data;
    let mut err: c_int = 0;
    if (dpcm.cable.ops.close_substream)
    err = dpcm.cable.ops.close_substream(dpcm);
    guard(mutex)(&loopback.cable_lock);
    free_cable(substream);
    return err;
    }
    static const struct snd_pcm_ops loopback_pcm_ops = {
    .open =		loopback_open,
    .close =	loopback_close,
    .hw_free =	loopback_hw_free,
    .prepare =	loopback_prepare,
    .trigger =	loopback_trigger,
    .pointer =	loopback_pointer,
    };
    static int loopback_pcm_new(struct loopback *loopback,
    int device, int substreams)
    {
    struct snd_pcm *pcm;
    int err;
    err = snd_pcm_new(loopback.card, "Loopback PCM", device,
    substreams, substreams, &pcm);
    if (err < 0)
    return err;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &loopback_pcm_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &loopback_pcm_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);
    pcm.private_data = loopback;
    pcm.info_flags = 0;
    strscpy(pcm.name, "Loopback PCM");
    loopback.pcm[device] = pcm;
    return 0;
    }
    static int loopback_rate_shift_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 80000;
    uinfo.value.integer.max = 120000;
    uinfo.value.integer.step = 1;
    return 0;
    }
    static int loopback_rate_shift_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    guard(mutex)(&loopback.cable_lock);
    ucontrol.value.integer.value[0] =
    loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].rate_shift;
    return 0;
    }
    static int loopback_rate_shift_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    let mut change: c_int = 0;
    val = ucontrol.value.integer.value[0];
    if (val < 80000)
    val = 80000;
    if (val > 120000)
    val = 120000;
    guard(mutex)(&loopback.cable_lock);
    if (val != loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].rate_shift) {
    loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].rate_shift = val;
    change = 1;
    }
    return change;
    }
    static int loopback_notify_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    guard(mutex)(&loopback.cable_lock);
    ucontrol.value.integer.value[0] =
    loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].notify;
    return 0;
    }
    static int loopback_notify_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    unsigned int val;
    let mut change: c_int = 0;
    val = ucontrol.value.integer.value[0] ? 1 : 0;
    guard(mutex)(&loopback.cable_lock);
    if (val != loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].notify) {
    loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].notify = val;
    change = 1;
    }
    return change;
    }
    static int loopback_active_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    struct loopback_cable *cable;
    let mut val: c_uint = 0;
    guard(mutex)(&loopback.cable_lock);
    cable = loopback.cables[kcontrol.id.subdevice][kcontrol.id.device ^ 1];
    if (cable != core::ptr::null_mut()) {
    let mut running: c_uint = cable.running ^ cable.pause;
    val = (running & (1 << SNDRV_PCM_STREAM_PLAYBACK)) ? 1 : 0;
    }
    ucontrol.value.integer.value[0] = val;
    return 0;
    }
    static int loopback_format_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = SNDRV_PCM_FORMAT_LAST;
    uinfo.value.integer.step = 1;
    return 0;
    }
    static int loopback_format_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    ucontrol.value.integer.value[0] =
    loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].format;
    return 0;
    }
    static int loopback_rate_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 192000;
    uinfo.value.integer.step = 1;
    return 0;
    }
    static int loopback_rate_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    guard(mutex)(&loopback.cable_lock);
    ucontrol.value.integer.value[0] =
    loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].rate;
    return 0;
    }
    static int loopback_channels_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 1;
    uinfo.value.integer.max = 1024;
    uinfo.value.integer.step = 1;
    return 0;
    }
    static int loopback_channels_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    guard(mutex)(&loopback.cable_lock);
    ucontrol.value.integer.value[0] =
    loopback.setup[kcontrol.id.subdevice]
    [kcontrol.id.device].channels;
    return 0;
    }
    static int loopback_access_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    static const char * const texts[] = {"Interleaved", "Non-interleaved"};
    return snd_ctl_enum_info(uinfo, 1, ARRAY_SIZE(texts), texts);
    }
    static int loopback_access_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct loopback *loopback = snd_kcontrol_chip(kcontrol);
    snd_pcm_access_t access;
    guard(mutex)(&loopback.cable_lock);
    access = loopback.setup[kcontrol.id.subdevice][kcontrol.id.device].access;
    ucontrol.value.enumerated.item[0] = !is_access_interleaved(access);
    return 0;
    }
    static const struct snd_kcontrol_new loopback_controls[]  = {
    {
    .iface =        SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         "PCM Rate Shift 100000",
    .info =         loopback_rate_shift_info,
    .get =          loopback_rate_shift_get,
    .put =          loopback_rate_shift_put,
    },
    {
    .iface =        SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         "PCM Notify",
    .info =         snd_ctl_boolean_mono_info,
    .get =          loopback_notify_get,
    .put =          loopback_notify_put,
    },
pub const ACTIVE_IDX: c_int = 2;
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =        SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         "PCM Slave Active",
    .info =         snd_ctl_boolean_mono_info,
    .get =          loopback_active_get,
    },
pub const FORMAT_IDX: c_int = 3;
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =        SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         "PCM Slave Format",
    .info =         loopback_format_info,
    .get =          loopback_format_get
    },
pub const RATE_IDX: c_int = 4;
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =        SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         "PCM Slave Rate",
    .info =         loopback_rate_info,
    .get =          loopback_rate_get
    },
pub const CHANNELS_IDX: c_int = 5;
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =        SNDRV_CTL_ELEM_IFACE_PCM,
    .name =         "PCM Slave Channels",
    .info =         loopback_channels_info,
    .get =          loopback_channels_get
    },
pub const ACCESS_IDX: c_int = 6;
    {
    .access =	SNDRV_CTL_ELEM_ACCESS_READ,
    .iface =	SNDRV_CTL_ELEM_IFACE_PCM,
    .name =		"PCM Slave Access Mode",
    .info =		loopback_access_info,
    .get =		loopback_access_get,
    },
    };
#[no_mangle]
unsafe extern "C" fn loopback_mixer_new(loopback: *mut loopback, notify: c_int) -> c_int {
    static int loopback_mixer_new(struct loopback *loopback, int notify)
    {
    struct snd_card *card = loopback.card;
    struct snd_pcm *pcm;
    struct snd_kcontrol *kctl;
    struct loopback_setup *setup;
    int err, dev, substr, substr_count, idx;
    strscpy(card.mixername, "Loopback Mixer");
    for (dev = 0; dev < 2; dev++) {
    pcm = loopback.pcm[dev];
    substr_count =
    pcm.streams[SNDRV_PCM_STREAM_CAPTURE].substream_count;
    for (substr = 0; substr < substr_count; substr++) {
    setup = &loopback.setup[substr][dev];
    setup.notify = notify;
    setup.rate_shift = NO_PITCH;
    setup.format = SNDRV_PCM_FORMAT_S16_LE;
    setup.access = SNDRV_PCM_ACCESS_RW_INTERLEAVED;
    setup.rate = 48000;
    setup.channels = 2;
    for (idx = 0; idx < ARRAY_SIZE(loopback_controls);
    idx++) {
    kctl = snd_ctl_new1(&loopback_controls[idx],
    loopback);
    if (!kctl)
    return -ENOMEM;
    kctl.id.device = dev;
    kctl.id.subdevice = substr;
// Add the control before copying the id so that
// the numid field of the id is set in the copy.
//
    err = snd_ctl_add(card, kctl);
    if (err < 0)
    return err;
    switch (idx) {
    case ACTIVE_IDX:
    setup.active_id = kctl.id;
    break;
    case FORMAT_IDX:
    setup.format_id = kctl.id;
    break;
    case RATE_IDX:
    setup.rate_id = kctl.id;
    break;
    case CHANNELS_IDX:
    setup.channels_id = kctl.id;
    break;
    case ACCESS_IDX:
    setup.access_id = kctl.id;
    break;
    default:
    break;
    }
    }
    }
    }
    return 0;
    }
    static void print_dpcm_info(struct snd_info_buffer *buffer,
    struct loopback_pcm *dpcm,
    const char *id)
    {
    snd_iprintf(buffer, "  %s\n", id);
    if (dpcm == core::ptr::null_mut()) {
    snd_iprintf(buffer, "    inactive\n");
    return;
    }
    snd_iprintf(buffer, "    buffer_size:\t%u\n", dpcm.pcm_buffer_size);
    snd_iprintf(buffer, "    buffer_pos:\t\t%u\n", dpcm.buf_pos);
    snd_iprintf(buffer, "    silent_size:\t%u\n", dpcm.silent_size);
    snd_iprintf(buffer, "    period_size:\t%u\n", dpcm.pcm_period_size);
    snd_iprintf(buffer, "    bytes_per_sec:\t%u\n", dpcm.pcm_bps);
    snd_iprintf(buffer, "    sample_align:\t%u\n", dpcm.pcm_salign);
    snd_iprintf(buffer, "    rate_shift:\t\t%u\n", dpcm.pcm_rate_shift);
    if (dpcm.cable.ops.dpcm_info)
    dpcm.cable.ops.dpcm_info(dpcm, buffer);
    }
    static void print_substream_info(struct snd_info_buffer *buffer,
    struct loopback *loopback,
    int sub,
    int num)
    {
    struct loopback_cable *cable = loopback.cables[sub][num];
    snd_iprintf(buffer, "Cable %i substream %i:\n", num, sub);
    if (cable == core::ptr::null_mut()) {
    snd_iprintf(buffer, "  inactive\n");
    return;
    }
    snd_iprintf(buffer, "  valid: %u\n", cable.valid);
    snd_iprintf(buffer, "  running: %u\n", cable.running);
    snd_iprintf(buffer, "  pause: %u\n", cable.pause);
    print_dpcm_info(buffer, cable.streams[0], "Playback");
    print_dpcm_info(buffer, cable.streams[1], "Capture");
    }
    static void print_cable_info(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct loopback *loopback = entry.private_data;
    int sub, num;
    guard(mutex)(&loopback.cable_lock);
    num = entry.name[strlen(entry.name)-1];
    num = num == '0' ? 0 : 1;
    for (sub = 0; sub < MAX_PCM_SUBSTREAMS; sub++)
    print_substream_info(buffer, loopback, sub, num);
    }
#[no_mangle]
unsafe extern "C" fn loopback_cable_proc_new(loopback: *mut loopback, cidx: c_int) -> c_int {
    static int loopback_cable_proc_new(struct loopback *loopback, int cidx)
    {
    char name[32];
    snprintf(name, sizeof(name), "cable#%d", cidx);
    return snd_card_ro_proc_new(loopback.card, name, loopback,
    print_cable_info);
    }
    static void loopback_set_timer_source(struct loopback *loopback,
    const char *value)
    {
    if (loopback.timer_source) {
    devm_kfree(loopback.card.dev, loopback.timer_source);
    loopback.timer_source = core::ptr::null_mut();
    }
    if (value && *value)
    loopback.timer_source = devm_kstrdup(loopback.card.dev,
    value, GFP_KERNEL);
    }
    static void print_timer_source_info(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct loopback *loopback = entry.private_data;
    guard(mutex)(&loopback.cable_lock);
    snd_iprintf(buffer, "%s\n",
    loopback.timer_source ? loopback.timer_source : "");
    }
    static void change_timer_source_info(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct loopback *loopback = entry.private_data;
    char line[64];
    guard(mutex)(&loopback.cable_lock);
    if (!snd_info_get_line(buffer, line, sizeof(line)))
    loopback_set_timer_source(loopback, strim(line));
    }
#[no_mangle]
unsafe extern "C" fn loopback_timer_source_proc_new(loopback: *mut loopback) -> c_int {
    static int loopback_timer_source_proc_new(struct loopback *loopback)
    {
    return snd_card_rw_proc_new(loopback.card, "timer_source", loopback,
    print_timer_source_info,
    change_timer_source_info);
    }
#[no_mangle]
unsafe extern "C" fn loopback_probe(devptr: *mut platform_device) -> c_int {
    static int loopback_probe(struct platform_device *devptr)
    {
    struct snd_card *card;
    struct loopback *loopback;
    let mut dev: c_int = devptr.id;
    int err;
    if (dev < 0 || dev >= SNDRV_CARDS) {
    dev_warn(&devptr.dev,
    "Invalid card index %d, using default 0\n", dev);
    dev = 0;
    }
    err = snd_devm_card_new(&devptr.dev, index[dev], id[dev], THIS_MODULE,
    sizeof(struct loopback), &card);
    if (err < 0)
    return err;
    loopback = card.private_data;
    if (pcm_substreams[dev] < 1)
    pcm_substreams[dev] = 1;
    if (pcm_substreams[dev] > MAX_PCM_SUBSTREAMS)
    pcm_substreams[dev] = MAX_PCM_SUBSTREAMS;
    loopback.card = card;
    loopback_set_timer_source(loopback, timer_source[dev]);
    mutex_init(&loopback.cable_lock);
    err = loopback_pcm_new(loopback, 0, pcm_substreams[dev]);
    if (err < 0)
    return err;
    err = loopback_pcm_new(loopback, 1, pcm_substreams[dev]);
    if (err < 0)
    return err;
    err = loopback_mixer_new(loopback, pcm_notify[dev] ? 1 : 0);
    if (err < 0)
    return err;
    loopback_cable_proc_new(loopback, 0);
    loopback_cable_proc_new(loopback, 1);
    loopback_timer_source_proc_new(loopback);
    strscpy(card.driver, "Loopback");
    strscpy(card.shortname, "Loopback");
    sprintf(card.longname, "Loopback %i", dev + 1);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    platform_set_drvdata(devptr, card);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loopback_suspend(pdev: *mut device) -> c_int {
    static int loopback_suspend(struct device *pdev)
    {
    struct snd_card *card = dev_get_drvdata(pdev);
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loopback_resume(pdev: *mut device) -> c_int {
    static int loopback_resume(struct device *pdev)
    {
    struct snd_card *card = dev_get_drvdata(pdev);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(loopback_pm, loopback_suspend, loopback_resume);

    static struct platform_driver loopback_driver = {
    .probe		= loopback_probe,
    .driver		= {
    .name	= SND_LOOPBACK_DRIVER,
    .pm	= &loopback_pm,
    },
    };
#[no_mangle]
unsafe extern "C" fn loopback_unregister_all() {
    static void loopback_unregister_all(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(devices); ++i)
    platform_device_unregister(devices[i]);
    platform_driver_unregister(&loopback_driver);
    }
#[no_mangle]
unsafe extern "C" fn alsa_card_loopback_init() -> int __init {
    static int __init alsa_card_loopback_init(void)
    {
    int i, err, cards;
    err = platform_driver_register(&loopback_driver);
    if (err < 0)
    return err;
    cards = 0;
    for (i = 0; i < SNDRV_CARDS; i++) {
    struct platform_device *device;
    if (!enable[i])
    continue;
    device = platform_device_register_simple(SND_LOOPBACK_DRIVER,
    i, core::ptr::null_mut(), 0);
    if (IS_ERR(device))
    continue;
    if (!platform_get_drvdata(device)) {
    platform_device_unregister(device);
    continue;
    }
    devices[i] = device;
    cards++;
    }
    if (!cards) {

    pr_err("aloop: No loopback enabled\n");

    loopback_unregister_all();
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alsa_card_loopback_exit() -> void __exit {
    static void __exit alsa_card_loopback_exit(void)
    {
    loopback_unregister_all();
    }
    module_init(alsa_card_loopback_init)
    module_exit(alsa_card_loopback_exit)
