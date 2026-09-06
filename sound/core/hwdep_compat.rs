//! Automatically rewritten from C to Rust
//! Source: sound/core/hwdep_compat.c
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
// 32bit -> 64bit ioctl wrapper for hwdep API
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
//
// This file is included from hwdep.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep_dsp_image32 {
    pub index: u32,
    pub name: [c_uchar; 64],
    pub /: *mut *mut u32 image; / pointer,
    pub length: u32,
    pub driver_data: u32,
    pub /: *mut *mut } / don't set packed attribute here,
    static int snd_hwdep_dsp_load_compat(struct snd_hwdep *hw,
    struct snd_hwdep_dsp_image32 __user *src)
    {
    pub {}: snd_hwdep_dsp_image info =,
    pub ptr: compat_caddr_t,
    if (copy_from_user(&info, src, 4 + 64) ||
    get_user(ptr, &src.image) ||
    get_user(info.length, &src.length) ||
    get_user(info.driver_data, &src.driver_data))
    pub -EFAULT: return,
    pub compat_ptr(ptr): info.image =,
    pub &info): return snd_hwdep_dsp_load(hw,,
    }
    enum {
    SNDRV_HWDEP_IOCTL_DSP_LOAD32   = _IOW('H', 0x03, struct snd_hwdep_dsp_image32)
}

    static long snd_hwdep_ioctl_compat(struct file * file, unsigned int cmd,
    unsigned long arg)
    {
    struct snd_hwdep *hw = file.private_data;
    void __user *argp = compat_ptr(arg);
    switch (cmd) {
    case SNDRV_HWDEP_IOCTL_PVERSION:
    case SNDRV_HWDEP_IOCTL_INFO:
    case SNDRV_HWDEP_IOCTL_DSP_STATUS:
    return snd_hwdep_ioctl(file, cmd, (unsigned long)argp);
    case SNDRV_HWDEP_IOCTL_DSP_LOAD32:
    return snd_hwdep_dsp_load_compat(hw, argp);
    }
    if (hw.ops.ioctl_compat)
    return hw.ops.ioctl_compat(hw, file, cmd, arg);
    return -ENOIOCTLCMD;
    }
