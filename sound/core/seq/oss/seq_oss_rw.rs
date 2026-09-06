//! Automatically rewritten from C to Rust
//! Source: sound/core/seq/oss/seq_oss_rw.c
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
// read/write/select interface to device file
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
//

//
// protoypes
//
    static int insert_queue(struct seq_oss_devinfo *dp, union evrec *rec, struct file *opt);
//
// read interface
//
    int
    snd_seq_oss_read(struct seq_oss_devinfo *dp, char __user *buf, int count)
    {
    struct seq_oss_readq *readq = dp.readq;
    let mut result: c_int = 0, err = 0;
    int ev_len;
    union evrec rec;
    unsigned long flags;
    if (readq == core::ptr::null_mut() || ! is_read_mode(dp.file_mode))
    return -ENXIO;
    while (count >= SHORT_EVENT_SIZE) {
    snd_seq_oss_readq_lock(readq, flags);
    err = snd_seq_oss_readq_pick(readq, &rec);
    if (err == -EAGAIN &&
    !is_nonblock_mode(dp.file_mode) && result == 0) {
    snd_seq_oss_readq_unlock(readq, flags);
    snd_seq_oss_readq_wait(readq);
    snd_seq_oss_readq_lock(readq, flags);
    if (signal_pending(current))
    err = -ERESTARTSYS;
    else
    err = snd_seq_oss_readq_pick(readq, &rec);
    }
    if (err < 0) {
    snd_seq_oss_readq_unlock(readq, flags);
    break;
    }
    ev_len = ev_length(&rec);
    if (count < ev_len) {
    err = -EINVAL;
    snd_seq_oss_readq_unlock(readq, flags);
    break;
    }
    snd_seq_oss_readq_free(readq);
    snd_seq_oss_readq_unlock(readq, flags);
    if (copy_to_user(buf, &rec, ev_len)) {
    err = -EFAULT;
    break;
    }
    result += ev_len;
    buf += ev_len;
    count -= ev_len;
    }
    return result > 0 ? result : err;
    }
//
// write interface
//
    int
    snd_seq_oss_write(struct seq_oss_devinfo *dp, const char __user *buf, int count, struct file *opt)
    {
    let mut result: c_int = 0, err = 0;
    int ev_size, fmt;
    union evrec rec;
    if (! is_write_mode(dp.file_mode) || dp.writeq == core::ptr::null_mut())
    return -ENXIO;
    while (count >= SHORT_EVENT_SIZE) {
    if (copy_from_user(&rec, buf, SHORT_EVENT_SIZE)) {
    err = -EFAULT;
    break;
    }
    if (rec.s.code == SEQ_FULLSIZE) {
// load patch
    if (result > 0) {
    err = -EINVAL;
    break;
    }
    fmt = (*(unsigned short *)rec.c) & 0xffff;
    err = snd_seq_oss_synth_load_patch(dp, rec.s.dev,
    fmt, buf, 0, count);
    return err < 0 ? err : count;
    }
    if (ev_is_long(&rec)) {
// extended code
    if (rec.s.code == SEQ_EXTENDED &&
    dp.seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC) {
    err = -EINVAL;
    break;
    }
    ev_size = LONG_EVENT_SIZE;
    if (count < ev_size)
    break;
// copy the reset 4 bytes
    if (copy_from_user(rec.c + SHORT_EVENT_SIZE,
    buf + SHORT_EVENT_SIZE,
    LONG_EVENT_SIZE - SHORT_EVENT_SIZE)) {
    err = -EFAULT;
    break;
    }
    } else {
// old-type code
    if (dp.seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC) {
    err = -EINVAL;
    break;
    }
    ev_size = SHORT_EVENT_SIZE;
    }
// insert queue
    err = insert_queue(dp, &rec, opt);
    if (err < 0)
    break;
    result += ev_size;
    buf += ev_size;
    count -= ev_size;
    }
    return result > 0 ? result : err;
    }
//
// insert event record to write queue
// return: 0 = OK, non-zero = NG
//
    static int
    insert_queue(struct seq_oss_devinfo *dp, union evrec *rec, struct file *opt)
    {
    let mut rc: c_int = 0;
    struct snd_seq_event event;
// if this is a timing event, process the current time
    if (snd_seq_oss_process_timer_event(dp.timer, rec))
    return 0; /* no need to insert queue */
// parse this event
    memset(&event, 0, sizeof(event));
// set dummy -- to be sure
    event.type = SNDRV_SEQ_EVENT_NOTEOFF;
    snd_seq_oss_fill_addr(dp, &event, dp.addr.client, dp.addr.port);
    snd_use_lock_t *lock __free(seq_oss_use_lock) = core::ptr::null_mut();
    if (snd_seq_oss_process_event(dp, rec, &event, &lock))
    return 0; /* invalid event - no need to insert queue */
    event.time.tick = snd_seq_oss_timer_cur_tick(dp.timer);
    if (dp.timer.realtime || !dp.timer.running)
    snd_seq_oss_dispatch(dp, &event, 0, 0);
    else
    rc = snd_seq_kernel_client_enqueue(dp.cseq, &event, opt,
    !is_nonblock_mode(dp.file_mode));
    return rc;
    }
//
// select / poll
//
    __poll_t
    snd_seq_oss_poll(struct seq_oss_devinfo *dp, struct file *file, poll_table * wait)
    {
    let mut mask: __poll_t = 0;
// input
    if (dp.readq && is_read_mode(dp.file_mode)) {
    if (snd_seq_oss_readq_poll(dp.readq, file, wait))
    mask |= EPOLLIN | EPOLLRDNORM;
    }
// output
    if (dp.writeq && is_write_mode(dp.file_mode)) {
    if (snd_seq_kernel_client_write_poll(dp.cseq, file, wait))
    mask |= EPOLLOUT | EPOLLWRNORM;
    }
    return mask;
    }
