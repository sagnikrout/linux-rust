//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/oss/seq_oss_readq.h
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
// read fifo queue
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
//

//
// definition of read queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_oss_readq {
    pub qlen: c_int,
    pub maxlen: c_int,
    pub tail: int head,,
    pub pre_event_timeout: c_ulong,
    pub input_time: c_ulong,
    pub midi_sleep: wait_queue_head_t,
    pub lock: spinlock_t,
    pub __counted_by(maxlen): evrec q[],
}

extern "C" {
    pub fn snd_seq_oss_readq_delete(q: *mut seq_oss_readq);
}
extern "C" {
    pub fn snd_seq_oss_readq_clear(readq: *mut seq_oss_readq);
}
extern "C" {
    pub fn snd_seq_oss_readq_poll(readq: *mut seq_oss_readq, file: *mut file, wait: *mut poll_table) -> c_uint;
}
extern "C" {
    pub fn snd_seq_oss_readq_puts(readq: *mut seq_oss_readq, dev: c_int, data: *mut c_uchar, len: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_readq_put_event(readq: *mut seq_oss_readq, ev: *mut evrec) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_readq_put_timestamp(readq: *mut seq_oss_readq, curt: c_ulong, seq_mode: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_readq_pick(q: *mut seq_oss_readq, rec: *mut evrec) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_readq_wait(q: *mut seq_oss_readq);
}
extern "C" {
    pub fn snd_seq_oss_readq_free(q: *mut seq_oss_readq);
}

