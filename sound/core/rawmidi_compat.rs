//! Automatically rewritten from C to Rust
//! Source: sound/core/rawmidi_compat.c
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
// 32bit -> 64bit ioctl wrapper for raw MIDI API
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
//
// This file included from rawmidi.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_params32 {
    pub stream: i32,
    pub buffer_size: u32,
    pub avail_min: u32,
    pub /: *mut *mut unsigned int no_active_sensing; / avoid bit-field,
    pub mode: c_uint,
    pub reserved: [c_uchar; 12],
    pub __packed: },
    static int snd_rawmidi_ioctl_params_compat(struct snd_rawmidi_file *rfile,
    struct snd_rawmidi_params32 __user *src)
    {
    pub params: snd_rawmidi_params,
    pub val: c_uint,
    if (get_user(params.stream, &src.stream) ||
    get_user(params.buffer_size, &src.buffer_size) ||
    get_user(params.avail_min, &src.avail_min) ||
    get_user(params.mode, &src.mode) ||
    get_user(val, &src.no_active_sensing))
    pub -EFAULT: return,
    pub val: params.no_active_sensing =,
    switch (params.stream) {
    case SNDRV_RAWMIDI_STREAM_OUTPUT:
    if (!rfile.output)
    pub -EINVAL: return,
    pub &params): return snd_rawmidi_output_params(rfile->output,,
    case SNDRV_RAWMIDI_STREAM_INPUT:
    if (!rfile.input)
    pub -EINVAL: return,
    pub &params): return snd_rawmidi_input_params(rfile->input,,
    }
    pub -EINVAL: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_snd_rawmidi_status64 {
    pub stream: i32,
    pub /: *mut *mut u8 rsvd[4]; / alignment,
    pub tstamp_sec: i64,
    pub tstamp_nsec: i64,
    pub avail: u32,
    pub xruns: u32,
    pub reserved: [c_uchar; 16],
    pub __packed: },
    static int snd_rawmidi_ioctl_status_compat64(struct snd_rawmidi_file *rfile,
    struct compat_snd_rawmidi_status64 __user *src)
    {
    pub err: c_int,
    pub status: snd_rawmidi_status64,
    pub compat_status: compat_snd_rawmidi_status64,
    if (get_user(status.stream, &src.stream))
    pub -EFAULT: return,
    switch (status.stream) {
    case SNDRV_RAWMIDI_STREAM_OUTPUT:
    if (!rfile.output)
    pub -EINVAL: return,
    pub &status): err = snd_rawmidi_output_status(rfile->output,,
    case SNDRV_RAWMIDI_STREAM_INPUT:
    if (!rfile.input)
    pub -EINVAL: return,
    pub &status): err = snd_rawmidi_input_status(rfile->input,,
    default:
    pub -EINVAL: return,
    }
    if (err < 0)
    pub err: return,
    compat_status = (struct compat_snd_rawmidi_status64) {
    .stream = status.stream,
    .tstamp_sec = status.tstamp_sec,
    .tstamp_nsec = status.tstamp_nsec,
    .avail = status.avail,
    .xruns = status.xruns,
}

    if (copy_to_user(src, &compat_status, sizeof(*src)))
    return -EFAULT;
    return 0;
    }
    enum {
    SNDRV_RAWMIDI_IOCTL_PARAMS32 = _IOWR('W', 0x10, struct snd_rawmidi_params32),
    SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT32 = _IOWR('W', 0x20, struct snd_rawmidi_status32),
    SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT64 = _IOWR('W', 0x20, struct compat_snd_rawmidi_status64),
    };
#[no_mangle]
unsafe extern "C" fn snd_rawmidi_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long snd_rawmidi_ioctl_compat(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct snd_rawmidi_file *rfile;
    void __user *argp = compat_ptr(arg);
    rfile = file.private_data;
    switch (cmd) {
    case SNDRV_RAWMIDI_IOCTL_PVERSION:
    case SNDRV_RAWMIDI_IOCTL_INFO:
    case SNDRV_RAWMIDI_IOCTL_DROP:
    case SNDRV_RAWMIDI_IOCTL_DRAIN:

    case SNDRV_UMP_IOCTL_ENDPOINT_INFO:
    case SNDRV_UMP_IOCTL_BLOCK_INFO:

    return snd_rawmidi_ioctl(file, cmd, (unsigned long)argp);
    case SNDRV_RAWMIDI_IOCTL_PARAMS32:
    return snd_rawmidi_ioctl_params_compat(rfile, argp);
    case SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT32:
    return snd_rawmidi_ioctl_status32(rfile, argp);
    case SNDRV_RAWMIDI_IOCTL_STATUS_COMPAT64:
    return snd_rawmidi_ioctl_status_compat64(rfile, argp);
    }
    return -ENOIOCTLCMD;
    }
