//! Automatically rewritten from C Header to Rust Module
//! Source: sound/virtio/virtio_ctl_msg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// virtio-snd: Virtio sound device
// Copyright (C) 2021 OpenSynergy GmbH
//

extern "C" {
    pub fn virtsnd_ctl_msg_ref(msg: *mut virtio_snd_msg);
}
extern "C" {
    pub fn virtsnd_ctl_msg_unref(msg: *mut virtio_snd_msg);
}
//
// virtsnd_ctl_msg_send_sync() - Simplified sending of synchronous message.
// @snd: VirtIO sound device.
// @msg: Control message.
//
// After returning from this function, the message will be deleted. If message
// content is still needed, the caller must additionally to
// virtsnd_ctl_msg_ref/unref() it.
//
// The msg_timeout_ms module parameter defines the message completion timeout.
// If the message is not completed within this time, the function will return an
// error.
//
// Context: Any context that permits to sleep.
// Return: 0 on success, -errno on failure.
//
// The return value is a message status code (VIRTIO_SND_S_XXX) converted to an
// appropriate -errno value.
//
extern "C" {
    pub fn virtsnd_ctl_msg_send(_arg: snd, _arg: msg, _arg: NULL, _arg: NULL, _arg: false) -> return;
}
//
// virtsnd_ctl_msg_send_async() - Simplified sending of asynchronous message.
// @snd: VirtIO sound device.
// @msg: Control message.
//
// Context: Any context.
// Return: 0 on success, -errno on failure.
//
extern "C" {
    pub fn virtsnd_ctl_msg_send(_arg: snd, _arg: msg, _arg: NULL, _arg: NULL, _arg: true) -> return;
}
extern "C" {
    pub fn virtsnd_ctl_msg_cancel_all(snd: *mut virtio_snd);
}
extern "C" {
    pub fn virtsnd_ctl_msg_complete(msg: *mut virtio_snd_msg);
}
extern "C" {
    pub fn virtsnd_ctl_notify_cb(vqueue: *mut virtqueue);
}
