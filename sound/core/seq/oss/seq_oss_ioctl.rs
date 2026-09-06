//! Automatically rewritten from C to Rust
//! Source: sound/core/seq/oss/seq_oss_ioctl.c
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
// OSS compatible sequencer driver
//
// OSS compatible i/o control
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
//

#[no_mangle]
unsafe extern "C" fn snd_seq_oss_synth_info_user(dp: *mut seq_oss_devinfo, arg: *mut void __user) -> c_int {
    static int snd_seq_oss_synth_info_user(struct seq_oss_devinfo *dp, void __user *arg)
    {
    struct synth_info info;
    if (copy_from_user(&info, arg, sizeof(info)))
    return -EFAULT;
    if (snd_seq_oss_synth_make_info(dp, info.device, &info) < 0)
    return -EINVAL;
    if (copy_to_user(arg, &info, sizeof(info)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_seq_oss_midi_info_user(dp: *mut seq_oss_devinfo, arg: *mut void __user) -> c_int {
    static int snd_seq_oss_midi_info_user(struct seq_oss_devinfo *dp, void __user *arg)
    {
    struct midi_info info;
    if (copy_from_user(&info, arg, sizeof(info)))
    return -EFAULT;
    if (snd_seq_oss_midi_make_info(dp, info.device, &info) < 0)
    return -EINVAL;
    if (copy_to_user(arg, &info, sizeof(info)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_seq_oss_oob_user(dp: *mut seq_oss_devinfo, arg: *mut void __user) -> c_int {
    static int snd_seq_oss_oob_user(struct seq_oss_devinfo *dp, void __user *arg)
    {
    unsigned char ev[8];
    struct snd_seq_event tmpev;
    if (copy_from_user(ev, arg, 8))
    return -EFAULT;
    memset(&tmpev, 0, sizeof(tmpev));
    snd_seq_oss_fill_addr(dp, &tmpev, dp.addr.client, dp.addr.port);
    tmpev.time.tick = 0;
    snd_use_lock_t *lock __free(seq_oss_use_lock) = core::ptr::null_mut();
    if (!snd_seq_oss_process_event(dp, (union evrec *)ev, &tmpev, &lock))
    snd_seq_oss_dispatch(dp, &tmpev, 0, 0);
    return 0;
    }
    int
    snd_seq_oss_ioctl(struct seq_oss_devinfo *dp, unsigned int cmd, unsigned long carg)
    {
    int dev, val;
    void __user *arg = (void __user *)carg;
    int __user *p = arg;
    switch (cmd) {
    case SNDCTL_TMR_TIMEBASE:
    case SNDCTL_TMR_TEMPO:
    case SNDCTL_TMR_START:
    case SNDCTL_TMR_STOP:
    case SNDCTL_TMR_CONTINUE:
    case SNDCTL_TMR_METRONOME:
    case SNDCTL_TMR_SOURCE:
    case SNDCTL_TMR_SELECT:
    case SNDCTL_SEQ_CTRLRATE:
    return snd_seq_oss_timer_ioctl(dp.timer, cmd, arg);
    case SNDCTL_SEQ_PANIC:
    snd_seq_oss_reset(dp);
    return -EINVAL;
    case SNDCTL_SEQ_SYNC:
    if (! is_write_mode(dp.file_mode) || dp.writeq == core::ptr::null_mut())
    return 0;
    while (snd_seq_oss_writeq_sync(dp.writeq))
    ;
    if (signal_pending(current))
    return -ERESTARTSYS;
    return 0;
    case SNDCTL_SEQ_RESET:
    snd_seq_oss_reset(dp);
    return 0;
    case SNDCTL_SEQ_TESTMIDI:
    if (get_user(dev, p))
    return -EFAULT;
    return snd_seq_oss_midi_open(dp, dev, dp.file_mode);
    case SNDCTL_SEQ_GETINCOUNT:
    if (dp.readq == core::ptr::null_mut() || ! is_read_mode(dp.file_mode))
    return 0;
    return put_user(dp.readq.qlen, p) ? -EFAULT : 0;
    case SNDCTL_SEQ_GETOUTCOUNT:
    if (! is_write_mode(dp.file_mode) || dp.writeq == core::ptr::null_mut())
    return 0;
    return put_user(snd_seq_oss_writeq_get_free_size(dp.writeq), p) ? -EFAULT : 0;
    case SNDCTL_SEQ_GETTIME:
    return put_user(snd_seq_oss_timer_cur_tick(dp.timer), p) ? -EFAULT : 0;
    case SNDCTL_SEQ_RESETSAMPLES:
    if (get_user(dev, p))
    return -EFAULT;
    return snd_seq_oss_synth_ioctl(dp, dev, cmd, carg);
    case SNDCTL_SEQ_NRSYNTHS:
    return put_user(dp.max_synthdev, p) ? -EFAULT : 0;
    case SNDCTL_SEQ_NRMIDIS:
    return put_user(dp.max_mididev, p) ? -EFAULT : 0;
    case SNDCTL_SYNTH_MEMAVL:
    if (get_user(dev, p))
    return -EFAULT;
    val = snd_seq_oss_synth_ioctl(dp, dev, cmd, carg);
    return put_user(val, p) ? -EFAULT : 0;
    case SNDCTL_FM_4OP_ENABLE:
    if (get_user(dev, p))
    return -EFAULT;
    snd_seq_oss_synth_ioctl(dp, dev, cmd, carg);
    return 0;
    case SNDCTL_SYNTH_INFO:
    case SNDCTL_SYNTH_ID:
    return snd_seq_oss_synth_info_user(dp, arg);
    case SNDCTL_SEQ_OUTOFBAND:
    return snd_seq_oss_oob_user(dp, arg);
    case SNDCTL_MIDI_INFO:
    return snd_seq_oss_midi_info_user(dp, arg);
    case SNDCTL_SEQ_THRESHOLD:
    if (! is_write_mode(dp.file_mode))
    return 0;
    if (get_user(val, p))
    return -EFAULT;
    if (val < 1)
    val = 1;
    if (val >= dp.writeq.maxlen)
    val = dp.writeq.maxlen - 1;
    snd_seq_oss_writeq_set_output(dp.writeq, val);
    return 0;
    case SNDCTL_MIDI_PRETIME:
    if (dp.readq == core::ptr::null_mut() || !is_read_mode(dp.file_mode))
    return 0;
    if (get_user(val, p))
    return -EFAULT;
    if (val <= 0)
    val = -1;
    else
    val = (HZ * val) / 10;
    dp.readq.pre_event_timeout = val;
    return put_user(val, p) ? -EFAULT : 0;
    default:
    if (! is_write_mode(dp.file_mode))
    return -EIO;
    return snd_seq_oss_synth_ioctl(dp, 0, cmd, carg);
    }
    return 0;
    }
