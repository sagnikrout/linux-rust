//! Automatically rewritten from C to Rust
//! Source: sound/core/pcm_compat.c
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
// 32bit -> 64bit ioctl wrapper for PCM API
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
//
// This file included from pcm_native.c

    static int snd_pcm_ioctl_delay_compat(struct snd_pcm_substream *substream,
    s32 __user *src)
    {
    snd_pcm_sframes_t delay;
    int err;
    err = snd_pcm_delay(substream, &delay);
    if (err)
    return err;
    if (put_user(delay, src))
    return -EFAULT;
    return 0;
    }
    static int snd_pcm_ioctl_rewind_compat(struct snd_pcm_substream *substream,
    u32 __user *src)
    {
    snd_pcm_uframes_t frames;
    int err;
    if (get_user(frames, src))
    return -EFAULT;
    err = snd_pcm_rewind(substream, frames);
    if (put_user(err, src))
    return -EFAULT;
    return err < 0 ? err : 0;
    }
    static int snd_pcm_ioctl_forward_compat(struct snd_pcm_substream *substream,
    u32 __user *src)
    {
    snd_pcm_uframes_t frames;
    int err;
    if (get_user(frames, src))
    return -EFAULT;
    err = snd_pcm_forward(substream, frames);
    if (put_user(err, src))
    return -EFAULT;
    return err < 0 ? err : 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_params32 {
    pub flags: u32,
    pub /: *mut *mut snd_mask masks[SNDRV_PCM_HW_PARAM_LAST_MASK - SNDRV_PCM_HW_PARAM_FIRST_MASK + 1]; / this must be identical,
    pub /: *mut *mut snd_mask mres[5]; / reserved masks,
    pub 1]: snd_interval intervals[SNDRV_PCM_HW_PARAM_LAST_INTERVAL - SNDRV_PCM_HW_PARAM_FIRST_INTERVAL +,
    pub /: *mut *mut snd_interval ires[9]; / reserved intervals,
    pub rmask: u32,
    pub cmask: u32,
    pub info: u32,
    pub msbits: u32,
    pub rate_num: u32,
    pub rate_den: u32,
    pub fifo_size: u32,
    pub reserved: [c_uchar; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_sw_params32 {
    pub tstamp_mode: i32,
    pub period_step: u32,
    pub sleep_min: u32,
    pub avail_min: u32,
    pub xfer_align: u32,
    pub start_threshold: u32,
    pub stop_threshold: u32,
    pub silence_threshold: u32,
    pub silence_size: u32,
    pub boundary: u32,
    pub proto: u32,
    pub tstamp_type: u32,
    pub reserved: [c_uchar; 56],
}

    static int snd_pcm_ioctl_sw_params_compat(struct snd_pcm_substream *substream,
    struct snd_pcm_sw_params32 __user *src)
    {
    struct snd_pcm_sw_params params;
    snd_pcm_uframes_t boundary;
    int err;
    memset(&params, 0, sizeof(params));
    if (get_user(params.tstamp_mode, &src.tstamp_mode) ||
    get_user(params.period_step, &src.period_step) ||
    get_user(params.sleep_min, &src.sleep_min) ||
    get_user(params.avail_min, &src.avail_min) ||
    get_user(params.xfer_align, &src.xfer_align) ||
    get_user(params.start_threshold, &src.start_threshold) ||
    get_user(params.stop_threshold, &src.stop_threshold) ||
    get_user(params.silence_threshold, &src.silence_threshold) ||
    get_user(params.silence_size, &src.silence_size) ||
    get_user(params.tstamp_type, &src.tstamp_type) ||
    get_user(params.proto, &src.proto))
    return -EFAULT;
//
// Check silent_size parameter.  Since we have 64bit boundary,
// silence_size must be compared with the 32bit boundary.
//
    boundary = recalculate_boundary(substream.runtime);
    if (boundary && params.silence_size >= boundary)
    params.silence_size = substream.runtime.boundary;
    err = snd_pcm_sw_params(substream, &params);
    if (err < 0)
    return err;
    if (boundary && put_user(boundary, &src.boundary))
    return -EFAULT;
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_channel_info32 {
    pub channel: u32,
    pub offset: u32,
    pub first: u32,
    pub step: u32,
}

    static int snd_pcm_ioctl_channel_info_compat(struct snd_pcm_substream *substream,
    struct snd_pcm_channel_info32 __user *src)
    {
    struct snd_pcm_channel_info info;
    int err;
    if (get_user(info.channel, &src.channel) ||
    get_user(info.offset, &src.offset) ||
    get_user(info.first, &src.first) ||
    get_user(info.step, &src.step))
    return -EFAULT;
    err = snd_pcm_channel_info(substream, &info);
    if (err < 0)
    return err;
    if (put_user(info.channel, &src.channel) ||
    put_user(info.offset, &src.offset) ||
    put_user(info.first, &src.first) ||
    put_user(info.step, &src.step))
    return -EFAULT;
    return err;
    }

// X32 ABI has the same struct as x86-64 for snd_pcm_channel_info
    static int snd_pcm_channel_info_user(struct snd_pcm_substream *substream,
    struct snd_pcm_channel_info __user *src);

    snd_pcm_channel_info_user(s, p)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_snd_pcm_status64 {
    pub state: snd_pcm_state_t,
    pub /: *mut *mut u8 rsvd[4]; / alignment,
    pub trigger_tstamp_sec: i64,
    pub trigger_tstamp_nsec: i64,
    pub tstamp_sec: i64,
    pub tstamp_nsec: i64,
    pub appl_ptr: u32,
    pub hw_ptr: u32,
    pub delay: i32,
    pub avail: u32,
    pub avail_max: u32,
    pub overrange: u32,
    pub suspended_state: snd_pcm_state_t,
    pub audio_tstamp_data: u32,
    pub audio_tstamp_sec: i64,
    pub audio_tstamp_nsec: i64,
    pub driver_tstamp_sec: i64,
    pub driver_tstamp_nsec: i64,
    pub audio_tstamp_accuracy: u32,
    pub reserved: [*mut c_uchar; 52-4*sizeof(s64)],
    pub __packed: },
    static int snd_pcm_status_user_compat64(struct snd_pcm_substream *substream,
    struct compat_snd_pcm_status64 __user *src,
    bool ext)
    {
    pub status: snd_pcm_status64,
    pub compat_status64: compat_snd_pcm_status64,
    pub err: c_int,
    pub sizeof(status)): memset(&status, 0,,
    pub sizeof(compat_status64)): memset(&compat_status64, 0,,
//
// with extension, parameters are read/write,
// get audio_tstamp_data from user,
// ignore rest of status structure
//
    if (ext && get_user(status.audio_tstamp_data,
    (u32 __user *)(&src.audio_tstamp_data)))
    pub -EFAULT: return,
    pub &status): err = snd_pcm_status64(substream,,
    if (err < 0)
    pub err: return,
    if (clear_user(src, sizeof(*src)))
    pub -EFAULT: return,
    compat_status64 = (struct compat_snd_pcm_status64) {
    .state = status.state,
    .trigger_tstamp_sec = status.trigger_tstamp_sec,
    .trigger_tstamp_nsec = status.trigger_tstamp_nsec,
    .tstamp_sec = status.tstamp_sec,
    .tstamp_nsec = status.tstamp_nsec,
    .appl_ptr = status.appl_ptr,
    .hw_ptr = status.hw_ptr,
    .delay = status.delay,
    .avail = status.avail,
    .avail_max = status.avail_max,
    .overrange = status.overrange,
    .suspended_state = status.suspended_state,
    .audio_tstamp_data = status.audio_tstamp_data,
    .audio_tstamp_sec = status.audio_tstamp_sec,
    .audio_tstamp_nsec = status.audio_tstamp_nsec,
    .driver_tstamp_sec = status.audio_tstamp_sec,
    .driver_tstamp_nsec = status.audio_tstamp_nsec,
    .audio_tstamp_accuracy = status.audio_tstamp_accuracy,
}

    if (copy_to_user(src, &compat_status64, sizeof(compat_status64)))
    return -EFAULT;
    return err;
    }
// both for HW_PARAMS and HW_REFINE
    static int snd_pcm_ioctl_hw_params_compat(struct snd_pcm_substream *substream,
    int refine,
    struct snd_pcm_hw_params32 __user *data32)
    {
    struct snd_pcm_runtime *runtime;
    int err;
    runtime = substream.runtime;
    if (!runtime)
    return -ENOTTY;
    struct snd_pcm_hw_params *data __free(kfree) =
    kmalloc_obj(*data);
    if (!data)
    return -ENOMEM;
// only fifo_size (RO from userspace) is different, so just copy all
    if (copy_from_user(data, data32, sizeof(*data32)))
    return -EFAULT;
    if (refine) {
    err = snd_pcm_hw_refine(substream, data);
    if (err < 0)
    return err;
    err = fixup_unreferenced_params(substream, data);
    } else {
    err = snd_pcm_hw_params(substream, data);
    }
    if (err < 0)
    return err;
    if (copy_to_user(data32, data, sizeof(*data32)) ||
    put_user(data.fifo_size, &data32.fifo_size))
    return -EFAULT;
    if (! refine) {
    let mut new_boundary: c_uint = recalculate_boundary(runtime);
    if (new_boundary)
    runtime.boundary = new_boundary;
    }
    return err;
    }
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_xferi32 {
    pub result: i32,
    pub buf: u32,
    pub frames: u32,
}

    static int snd_pcm_ioctl_xferi_compat(struct snd_pcm_substream *substream,
    int dir, struct snd_xferi32 __user *data32)
    {
    compat_caddr_t buf;
    u32 frames;
    int err;
    if (! substream.runtime)
    return -ENOTTY;
    if (substream.stream != dir)
    return -EINVAL;
    if (snd_pcm_get_state(substream) == SNDRV_PCM_STATE_OPEN)
    return -EBADFD;
    if (get_user(buf, &data32.buf) ||
    get_user(frames, &data32.frames))
    return -EFAULT;
    if (dir == SNDRV_PCM_STREAM_PLAYBACK)
    err = snd_pcm_lib_write(substream, compat_ptr(buf), frames);
    else
    err = snd_pcm_lib_read(substream, compat_ptr(buf), frames);
    if (err < 0)
    return err;
// copy the result
    if (put_user(err, &data32.result))
    return -EFAULT;
    return 0;
    }
// snd_xfern needs remapping of bufs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_xfern32 {
    pub result: i32,
    pub /: *mut *mut *mut *mut u32 bufs; / this is void ;,
    pub frames: u32,
}

//
// xfern ioctl nees to copy (up to) 128 pointers on stack.
// although we may pass the copied pointers through f_op->ioctl, but the ioctl
// handler there expands again the same 128 pointers on stack, so it is better
// to handle the function (calling pcm_readv/writev) directly in this handler.
//
    static int snd_pcm_ioctl_xfern_compat(struct snd_pcm_substream *substream,
    int dir, struct snd_xfern32 __user *data32)
    {
    compat_caddr_t buf;
    compat_caddr_t __user *bufptr;
    u32 frames;
    int err, ch, i;
    if (! substream.runtime)
    return -ENOTTY;
    if (substream.stream != dir)
    return -EINVAL;
    if (snd_pcm_get_state(substream) == SNDRV_PCM_STATE_OPEN)
    return -EBADFD;
    ch = substream.runtime.channels;
    if (ch > 128)
    return -EINVAL;
    if (get_user(buf, &data32.bufs) ||
    get_user(frames, &data32.frames))
    return -EFAULT;
    bufptr = compat_ptr(buf);
    void __user **bufs __free(kfree) =
    kmalloc_array(ch, sizeof(void __user *), GFP_KERNEL);
    if (bufs == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0; i < ch; i++) {
    u32 ptr;
    if (get_user(ptr, bufptr))
    return -EFAULT;
    bufs[i] = compat_ptr(ptr);
    bufptr++;
    }
    if (dir == SNDRV_PCM_STREAM_PLAYBACK)
    err = snd_pcm_lib_writev(substream, bufs, frames);
    else
    err = snd_pcm_lib_readv(substream, bufs, frames);
    if (err >= 0) {
    if (put_user(err, &data32.result))
    return -EFAULT;
    }
    return err;
    }

// X32 ABI has 64bit timespec and 64bit alignment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_mmap_status_x32 {
    pub state: snd_pcm_state_t,
    pub pad1: i32,
    pub hw_ptr: u32,
    pub /: *mut *mut u32 pad2; / alignment,
    pub tstamp: __snd_timespec64,
    pub suspended_state: snd_pcm_state_t,
    pub pad3: i32,
    pub audio_tstamp: __snd_timespec64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_mmap_control_x32 {
    pub appl_ptr: u32,
    pub avail_min: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_sync_ptr_x32 {
    pub flags: u32,
    pub /: *mut *mut u32 rsvd; / alignment,
    union {
    pub status: snd_pcm_mmap_status_x32,
    pub reserved: [c_uchar; 64],
    pub s: },
    union {
    pub control: snd_pcm_mmap_control_x32,
    pub reserved: [c_uchar; 64],
    pub c: },
    pub __packed: },
    static int snd_pcm_ioctl_sync_ptr_x32(struct snd_pcm_substream *substream,
    struct snd_pcm_sync_ptr_x32 __user *src)
    {
    pub substream->runtime: *mut *mut snd_pcm_runtime runtime =,
    pub status: *mut volatile struct snd_pcm_mmap_status,
    pub control: *mut volatile struct snd_pcm_mmap_control,
    pub sflags: u32,
    pub scontrol: snd_pcm_mmap_control,
    pub sstatus: snd_pcm_mmap_status,
    pub boundary: snd_pcm_uframes_t,
    pub err: c_int,
    if (snd_BUG_ON(!runtime))
    pub -EINVAL: return,
    if (snd_pcm_sync_ptr_get_user(sflags, scontrol, src))
    pub -EFAULT: return,
    if (sflags & SNDRV_PCM_SYNC_PTR_HWSYNC) {
    pub snd_pcm_hwsync(substream): err =,
    if (err < 0)
    pub err: return,
    }
    pub runtime->status: status =,
    pub runtime->control: control =,
    pub recalculate_boundary(runtime): boundary =,
    if (!boundary)
    pub 0x7fffffff: boundary =,
    scoped_guard(pcm_stream_lock_irq, substream) {
    if (!(sflags & SNDRV_PCM_SYNC_PTR_APPL)) {
    pub scontrol.appl_ptr): err = pcm_lib_apply_appl_ptr(substream,,
    if (err < 0)
    pub err: return,
    } else {
    pub boundary: scontrol.appl_ptr = control->appl_ptr %,
    }
    if (!(sflags & SNDRV_PCM_SYNC_PTR_AVAIL_MIN))
    pub scontrol.avail_min: control->avail_min =,
    else
    pub control->avail_min: scontrol.avail_min =,
    pub status->state: sstatus.state =,
    pub boundary: sstatus.hw_ptr = status->hw_ptr %,
    pub status->tstamp: sstatus.tstamp =,
    pub status->suspended_state: sstatus.suspended_state =,
    pub status->audio_tstamp: sstatus.audio_tstamp =,
    }
    if (!(sflags & SNDRV_PCM_SYNC_PTR_APPL))
    pub SNDRV_DMA_SYNC_DEVICE): snd_pcm_dma_buffer_sync(substream,,
    if (snd_pcm_sync_ptr_put_user(sstatus, scontrol, src))
    pub -EFAULT: return,
    pub 0: return,
    }
    pub __pad_before_u32: [typedef char; 4],
    pub __pad_after_u32: [typedef char; 0],    pub __pad_before_u32: [typedef char; 0],
    pub __pad_after_u32: [typedef char; 4],
// PCM 2.0.15 API definition had a bug in mmap control; it puts the avail_min
// at the wrong offset due to a typo in padding type.
// The bug hits only 32bit.
// A workaround for incorrect read/write is needed only in 32bit compat mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_pcm_mmap_control64_buggy {
    pub __pad1: __pad_before_u32,
    pub appl_ptr: __u32,
    pub /: *mut *mut __pad_before_u32 __pad2; / SiC! here is the bug,
    pub __pad3: __pad_before_u32,
    pub avail_min: __u32,
    pub __pad4: __pad_after_uframe,
}

    static int snd_pcm_ioctl_sync_ptr_buggy(struct snd_pcm_substream *substream,
    struct snd_pcm_sync_ptr __user *_sync_ptr)
    {
    struct snd_pcm_runtime *runtime = substream.runtime;
    struct snd_pcm_sync_ptr sync_ptr;
    struct __snd_pcm_mmap_control64_buggy *sync_cp;
    volatile struct snd_pcm_mmap_status *status;
    volatile struct snd_pcm_mmap_control *control;
    int err;
    memset(&sync_ptr, 0, sizeof(sync_ptr));
    sync_cp = (struct __snd_pcm_mmap_control64_buggy *)&sync_ptr.c.control;
    if (get_user(sync_ptr.flags, (unsigned __user *)&(_sync_ptr.flags)))
    return -EFAULT;
    if (copy_from_user(sync_cp, &(_sync_ptr.c.control), sizeof(*sync_cp)))
    return -EFAULT;
    status = runtime.status;
    control = runtime.control;
    if (sync_ptr.flags & SNDRV_PCM_SYNC_PTR_HWSYNC) {
    err = snd_pcm_hwsync(substream);
    if (err < 0)
    return err;
    }
    scoped_guard(pcm_stream_lock_irq, substream) {
    if (!(sync_ptr.flags & SNDRV_PCM_SYNC_PTR_APPL)) {
    err = pcm_lib_apply_appl_ptr(substream, sync_cp.appl_ptr);
    if (err < 0)
    return err;
    } else {
    sync_cp.appl_ptr = control.appl_ptr;
    }
    if (!(sync_ptr.flags & SNDRV_PCM_SYNC_PTR_AVAIL_MIN))
    control.avail_min = sync_cp.avail_min;
    else
    sync_cp.avail_min = control.avail_min;
    sync_ptr.s.status.state = status.state;
    sync_ptr.s.status.hw_ptr = status.hw_ptr;
    sync_ptr.s.status.tstamp = status.tstamp;
    sync_ptr.s.status.suspended_state = status.suspended_state;
    sync_ptr.s.status.audio_tstamp = status.audio_tstamp;
    }
    if (!(sync_ptr.flags & SNDRV_PCM_SYNC_PTR_APPL))
    snd_pcm_dma_buffer_sync(substream, SNDRV_DMA_SYNC_DEVICE);
    if (copy_to_user(_sync_ptr, &sync_ptr, sizeof(sync_ptr)))
    return -EFAULT;
    return 0;
    }
//
    enum {
    SNDRV_PCM_IOCTL_HW_REFINE32 = _IOWR('A', 0x10, struct snd_pcm_hw_params32),
    SNDRV_PCM_IOCTL_HW_PARAMS32 = _IOWR('A', 0x11, struct snd_pcm_hw_params32),
    SNDRV_PCM_IOCTL_SW_PARAMS32 = _IOWR('A', 0x13, struct snd_pcm_sw_params32),
    SNDRV_PCM_IOCTL_STATUS_COMPAT32 = _IOR('A', 0x20, struct snd_pcm_status32),
    SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT32 = _IOWR('A', 0x24, struct snd_pcm_status32),
    SNDRV_PCM_IOCTL_DELAY32 = _IOR('A', 0x21, s32),
    SNDRV_PCM_IOCTL_CHANNEL_INFO32 = _IOR('A', 0x32, struct snd_pcm_channel_info32),
    SNDRV_PCM_IOCTL_REWIND32 = _IOW('A', 0x46, u32),
    SNDRV_PCM_IOCTL_FORWARD32 = _IOW('A', 0x49, u32),
    SNDRV_PCM_IOCTL_WRITEI_FRAMES32 = _IOW('A', 0x50, struct snd_xferi32),
    SNDRV_PCM_IOCTL_READI_FRAMES32 = _IOR('A', 0x51, struct snd_xferi32),
    SNDRV_PCM_IOCTL_WRITEN_FRAMES32 = _IOW('A', 0x52, struct snd_xfern32),
    SNDRV_PCM_IOCTL_READN_FRAMES32 = _IOR('A', 0x53, struct snd_xfern32),
    SNDRV_PCM_IOCTL_STATUS_COMPAT64 = _IOR('A', 0x20, struct compat_snd_pcm_status64),
    SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT64 = _IOWR('A', 0x24, struct compat_snd_pcm_status64),

    SNDRV_PCM_IOCTL_CHANNEL_INFO_X32 = _IOR('A', 0x32, struct snd_pcm_channel_info),
    SNDRV_PCM_IOCTL_SYNC_PTR_X32 = _IOWR('A', 0x23, struct snd_pcm_sync_ptr_x32),

    };
#[no_mangle]
unsafe extern "C" fn snd_pcm_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long snd_pcm_ioctl_compat(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct snd_pcm_file *pcm_file;
    struct snd_pcm_substream *substream;
    void __user *argp = compat_ptr(arg);
    pcm_file = file.private_data;
    if (! pcm_file)
    return -ENOTTY;
    substream = pcm_file.substream;
    if (! substream)
    return -ENOTTY;
//
// When PCM is used on 32bit mode, we need to disable
// mmap of the old PCM status/control records because
// of the size incompatibility.
//
    pcm_file.no_compat_mmap = 1;
    switch (cmd) {
    case SNDRV_PCM_IOCTL_PVERSION:
    case SNDRV_PCM_IOCTL_INFO:
    case SNDRV_PCM_IOCTL_TSTAMP:
    case SNDRV_PCM_IOCTL_TTSTAMP:
    case SNDRV_PCM_IOCTL_USER_PVERSION:
    case SNDRV_PCM_IOCTL_HWSYNC:
    case SNDRV_PCM_IOCTL_PREPARE:
    case SNDRV_PCM_IOCTL_RESET:
    case SNDRV_PCM_IOCTL_START:
    case SNDRV_PCM_IOCTL_DROP:
    case SNDRV_PCM_IOCTL_DRAIN:
    case SNDRV_PCM_IOCTL_PAUSE:
    case SNDRV_PCM_IOCTL_HW_FREE:
    case SNDRV_PCM_IOCTL_RESUME:
    case SNDRV_PCM_IOCTL_XRUN:
    case SNDRV_PCM_IOCTL_LINK:
    case SNDRV_PCM_IOCTL_UNLINK:
    case __SNDRV_PCM_IOCTL_SYNC_PTR32:
    return snd_pcm_common_ioctl(file, substream, cmd, argp);
    case __SNDRV_PCM_IOCTL_SYNC_PTR64:

    if (in_x32_syscall())
    return snd_pcm_ioctl_sync_ptr_x32(substream, argp);

    return snd_pcm_ioctl_sync_ptr_buggy(substream, argp);
    case SNDRV_PCM_IOCTL_HW_REFINE32:
    return snd_pcm_ioctl_hw_params_compat(substream, 1, argp);
    case SNDRV_PCM_IOCTL_HW_PARAMS32:
    return snd_pcm_ioctl_hw_params_compat(substream, 0, argp);
    case SNDRV_PCM_IOCTL_SW_PARAMS32:
    return snd_pcm_ioctl_sw_params_compat(substream, argp);
    case SNDRV_PCM_IOCTL_STATUS_COMPAT32:
    return snd_pcm_status_user32(substream, argp, false);
    case SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT32:
    return snd_pcm_status_user32(substream, argp, true);
    case SNDRV_PCM_IOCTL_CHANNEL_INFO32:
    return snd_pcm_ioctl_channel_info_compat(substream, argp);
    case SNDRV_PCM_IOCTL_WRITEI_FRAMES32:
    return snd_pcm_ioctl_xferi_compat(substream, SNDRV_PCM_STREAM_PLAYBACK, argp);
    case SNDRV_PCM_IOCTL_READI_FRAMES32:
    return snd_pcm_ioctl_xferi_compat(substream, SNDRV_PCM_STREAM_CAPTURE, argp);
    case SNDRV_PCM_IOCTL_WRITEN_FRAMES32:
    return snd_pcm_ioctl_xfern_compat(substream, SNDRV_PCM_STREAM_PLAYBACK, argp);
    case SNDRV_PCM_IOCTL_READN_FRAMES32:
    return snd_pcm_ioctl_xfern_compat(substream, SNDRV_PCM_STREAM_CAPTURE, argp);
    case SNDRV_PCM_IOCTL_DELAY32:
    return snd_pcm_ioctl_delay_compat(substream, argp);
    case SNDRV_PCM_IOCTL_REWIND32:
    return snd_pcm_ioctl_rewind_compat(substream, argp);
    case SNDRV_PCM_IOCTL_FORWARD32:
    return snd_pcm_ioctl_forward_compat(substream, argp);
    case SNDRV_PCM_IOCTL_STATUS_COMPAT64:
    return snd_pcm_status_user_compat64(substream, argp, false);
    case SNDRV_PCM_IOCTL_STATUS_EXT_COMPAT64:
    return snd_pcm_status_user_compat64(substream, argp, true);

    case SNDRV_PCM_IOCTL_CHANNEL_INFO_X32:
    return snd_pcm_ioctl_channel_info_x32(substream, argp);

    }
    return -ENOIOCTLCMD;
    }
