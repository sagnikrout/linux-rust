//! Automatically rewritten from C to Rust
//! Source: sound/hda/common/hwdep.c
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
// HWDEP Interface for HD-audio codec
//
// Copyright (c) 2007 Takashi Iwai <tiwai@suse.de>
//

//
// write/read an out-of-bound verb
//
    static int verb_write_ioctl(struct hda_codec *codec,
    struct hda_verb_ioctl __user *arg)
    {
    u32 verb, res;
    if (get_user(verb, &arg.verb))
    return -EFAULT;
    res = snd_hda_codec_read(codec, verb >> 24, 0,
    (verb >> 8) & 0xffff, verb & 0xff);
    if (put_user(res, &arg.res))
    return -EFAULT;
    return 0;
    }
    static int get_wcap_ioctl(struct hda_codec *codec,
    struct hda_verb_ioctl __user *arg)
    {
    u32 verb, res;
    if (get_user(verb, &arg.verb))
    return -EFAULT;
// open-code get_wcaps(verb>>24) with nospec
    verb >>= 24;
    if (verb < codec.core.start_nid ||
    verb >= codec.core.start_nid + codec.core.num_nodes) {
    res = 0;
    } else {
    verb -= codec.core.start_nid;
    verb = array_index_nospec(verb, codec.core.num_nodes);
    res = codec.wcaps[verb];
    }
    if (put_user(res, &arg.res))
    return -EFAULT;
    return 0;
    }
//
    static int hda_hwdep_ioctl(struct snd_hwdep *hw, struct file *file,
    unsigned int cmd, unsigned long arg)
    {
    struct hda_codec *codec = hw.private_data;
    void __user *argp = (void __user *)arg;
    switch (cmd) {
    case HDA_IOCTL_PVERSION:
    return put_user(HDA_HWDEP_VERSION, (int __user *)argp);
    case HDA_IOCTL_VERB_WRITE:
    return verb_write_ioctl(codec, argp);
    case HDA_IOCTL_GET_WCAP:
    return get_wcap_ioctl(codec, argp);
    }
    return -ENOIOCTLCMD;
    }

    static int hda_hwdep_ioctl_compat(struct snd_hwdep *hw, struct file *file,
    unsigned int cmd, unsigned long arg)
    {
    return hda_hwdep_ioctl(hw, file, cmd, (unsigned long)compat_ptr(arg));
    }

#[no_mangle]
unsafe extern "C" fn hda_hwdep_open(hw: *mut snd_hwdep, file: *mut file) -> c_int {
    static int hda_hwdep_open(struct snd_hwdep *hw, struct file *file)
    {
    if (!capable(CAP_SYS_RAWIO))
    return -EACCES;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_hda_create_hwdep(codec: *mut hda_codec) -> c_int {
    int snd_hda_create_hwdep(struct hda_codec *codec)
    {
    char hwname[16];
    struct snd_hwdep *hwdep;
    int err;
    sprintf(hwname, "HDA Codec %d", codec.addr);
    err = snd_hwdep_new(codec.card, hwname, codec.addr, &hwdep);
    if (err < 0)
    return err;
    codec.hwdep = hwdep;
    sprintf(hwdep.name, "HDA Codec %d", codec.addr);
    hwdep.iface = SNDRV_HWDEP_IFACE_HDA;
    hwdep.private_data = codec;
    hwdep.exclusive = 1;
    hwdep.ops.open = hda_hwdep_open;
    hwdep.ops.ioctl = hda_hwdep_ioctl;

    hwdep.ops.ioctl_compat = hda_hwdep_ioctl_compat;

// for sysfs
    hwdep.dev.groups = snd_hda_dev_attr_groups;
    dev_set_drvdata(hwdep.dev, codec);
    return 0;
    }
