//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-card.h
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


// SPDX-License-Identifier: GPL-2.0
//
// soc-card.h
//
// Copyright (C) 2019 Renesas Electronics Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_card_subclass {
    SND_SOC_CARD_CLASS_ROOT		= 0,
    SND_SOC_CARD_CLASS_RUNTIME	= 1,
}

extern "C" {
    pub fn snd_soc_card_suspend_pre(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_card_suspend_post(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_card_resume_pre(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_card_resume_post(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_card_probe(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_card_late_probe(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_card_fixup_controls(card: *mut snd_soc_card);
}
extern "C" {
    pub fn snd_soc_card_remove(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_card_set_topology_name(card: *mut snd_soc_card, preifx: *const c_char);
}

// vendor = card->pci_subsystem_vendor;
// device = card->pci_subsystem_device;

// device driver data
extern "C" {
    pub fn snd_soc_rtd_to_codec(_arg: rtd, _arg: 0) -> return;
}
