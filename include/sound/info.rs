//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/info.h
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
// Header file for info interface
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

// buffer for information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_buffer {
    pub /: *mut *mut *mut char buffer; / pointer to begin of buffer,
    pub /: *mut *mut unsigned int curr; / current position in buffer,
    pub /: *mut *mut unsigned int size; / current size,
    pub /: *mut *mut unsigned int len; / total length of buffer,
    pub /: *mut *mut int stop; / stop flag,
    pub /: *mut *mut int error; / error code,
}

pub const SNDRV_INFO_CONTENT_TEXT: c_int = 0;
pub const SNDRV_INFO_CONTENT_DATA: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_entry_text {
    pub buffer): *mut snd_info_buffer,
    pub buffer): *mut snd_info_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_entry_ops {
    pub file_private_data): *mut unsigned short mode, void,
    pub file_private_data): *mut unsigned short mode, void,
    pub pos): size_t count, loff_t,
    pub pos): size_t count, loff_t,
    pub orig): loff_t offset, int,
    pub wait): *mut poll_table,
    pub arg): *mut *mut file file, unsigned int cmd, unsigned long,
    pub vma): *mut vm_area_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_entry {
    pub name: *const c_char,
    pub mode: umode_t,
    pub size: c_long,
    pub content: c_ushort,
    pub text: snd_info_entry_text,
    pub ops: *const snd_info_entry_ops,
    pub c: },
    pub parent: *mut snd_info_entry,
    pub module: *mut module,
    pub private_data: *mut c_void,
    pub entry): *mut *mut void (private_free)(struct snd_info_entry,
    pub p: *mut proc_dir_entry,
    pub access: mutex,
    pub children: list_head,
    pub list: list_head,
}

extern "C" {
    pub fn snd_info_minor_register() -> c_int;
}

pub const snd_info_minor_register(): c_int = 0;

extern "C" {
    pub fn snd_card_info_read_oss(buffer: *mut snd_info_buffer);
}

//
// snd_iprintf - printf on the procfs buffer
// @buf: the procfs buffer
// @fmt: the printf format
//
// Outputs the string on the procfs buffer just like printf().
//
// Return: zero for success, or a negative error code.
//

extern "C" {
    pub fn snd_info_init() -> c_int;
}
extern "C" {
    pub fn snd_info_done() -> c_int;
}
extern "C" {
    pub fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn snd_info_free_entry(entry: *mut snd_info_entry);
}
extern "C" {
    pub fn snd_info_card_create(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_info_card_register(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_info_card_free(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_info_card_disconnect(card: *mut snd_card);
}
extern "C" {
    pub fn snd_info_card_id_change(card: *mut snd_card);
}
extern "C" {
    pub fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
}
// for card drivers
// entryp = snd_info_create_card_entry(card, name, card->proc_root);
extern "C" {
    pub fn snd_info_check_reserved_words(str: *const c_char) -> c_int;
}

//
// snd_card_ro_proc_new - Create a read-only text proc file entry for the card
// @card: the card instance
// @name: the file name
// @private_data: the arbitrary private data
// @read: the read callback
//
// This proc file entry will be registered via snd_card_register() call, and
// it will be removed automatically at the card removal, too.
//
extern "C" {
    pub fn snd_card_rw_proc_new(_arg: card, _arg: name, _arg: private_data, _arg: read, _arg: NULL) -> return;
}
//
// OSS info part
//

pub const SNDRV_OSS_INFO_DEV_AUDIO: c_int = 0;
pub const SNDRV_OSS_INFO_DEV_SYNTH: c_int = 1;
pub const SNDRV_OSS_INFO_DEV_MIDI: c_int = 2;
pub const SNDRV_OSS_INFO_DEV_TIMERS: c_int = 4;
pub const SNDRV_OSS_INFO_DEV_MIXERS: c_int = 5;
pub const SNDRV_OSS_INFO_DEV_COUNT: c_int = 6;
extern "C" {
    pub fn snd_oss_info_register(dev: c_int, num: c_int, string: *mut c_char) -> c_int;
}

