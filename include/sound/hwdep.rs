//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hwdep.h
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
// Hardware dependent layer
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

// hwdep file ops; all ops can be NULL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep_ops {
    pub orig): long long offset, int,
    pub offset): *mut long count, loff_t,
    pub offset): *mut long count, loff_t,
    pub file): *mut *mut *mut *mut int (open)(struct snd_hwdep hw, struct file,
    pub file): *mut *mut *mut *mut int (release)(struct snd_hwdep hw, struct file,
    pub wait): *mut poll_table,
    pub arg): unsigned int cmd, unsigned long,
    pub arg): unsigned int cmd, unsigned long,
    pub vma): *mut vm_area_struct,
    pub status): *mut snd_hwdep_dsp_status,
    pub image): *mut snd_hwdep_dsp_image,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep {
    pub card: *mut snd_card,
    pub list: list_head,
    pub device: c_int,
    pub id: [c_char; 32],
    pub name: [c_char; 80],
    pub iface: c_int,

    pub oss_type: c_int,
    pub ossreg: c_int,

    pub ops: snd_hwdep_ops,
    pub open_wait: wait_queue_head_t,
    pub private_data: *mut c_void,
    pub hwdep): *mut *mut void (private_free) (struct snd_hwdep,
    pub dev: *mut device,
    pub open_mutex: mutex,
    pub /: *mut *mut int used; / reference counter,
    pub /: *mut *mut unsigned int dsp_loaded; / bit fields of loaded dsp indices,
    pub /: *mut *mut unsigned int exclusive:1; / exclusive access mode,
}
