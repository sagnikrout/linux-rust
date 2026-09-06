//! Automatically rewritten from C to Rust
//! Source: sound/core/ctljack.c
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
// Helper functions for jack-detection kcontrols
//
// Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
//

    static int jack_detect_kctl_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    ucontrol.value.integer.value[0] = kcontrol.private_value;
    return 0;
    }
    static const struct snd_kcontrol_new jack_detect_kctl = {
// name is filled later
    .iface = SNDRV_CTL_ELEM_IFACE_CARD,
    .access = SNDRV_CTL_ELEM_ACCESS_READ,
    .info = jack_detect_kctl_info,
    .get = jack_detect_kctl_get,
    };
#[no_mangle]
unsafe extern "C" fn get_available_index(card: *mut snd_card, name: *const c_char) -> c_int {
    static int get_available_index(struct snd_card *card, const char *name)
    {
    struct snd_ctl_elem_id sid;
    memset(&sid, 0, sizeof(sid));
    sid.index = 0;
    sid.iface = SNDRV_CTL_ELEM_IFACE_CARD;
    strscpy(sid.name, name, sizeof(sid.name));
    while (snd_ctl_find_id(card, &sid)) {
    sid.index++;
// reset numid; otherwise snd_ctl_find_id() hits this again
    sid.numid = 0;
    }
    return sid.index;
    }
#[no_mangle]
unsafe extern "C" fn jack_kctl_name_gen(name: *mut c_char, src_name: *const c_char, size: usize) {
    static void jack_kctl_name_gen(char *name, const char *src_name, size_t size)
    {
    let mut count: usize = strlen(src_name);
    const char *suf = " Jack";
    let mut suf_len: usize = strlen(suf);
    let mut append_suf: bool = true;
    if (count >= suf_len)
    append_suf = strncmp(&src_name[count - suf_len], suf, suf_len) != 0;
    if (append_suf)
    snprintf(name, size, "%s%s", src_name, suf);
    else
    strscpy(name, src_name, size);
    }
    struct snd_kcontrol *
    snd_kctl_jack_new(const char *name, struct snd_card *card)
    {
    struct snd_kcontrol *kctl;
    kctl = snd_ctl_new1(&jack_detect_kctl, core::ptr::null_mut());
    if (!kctl)
    return core::ptr::null_mut();
    jack_kctl_name_gen(kctl.id.name, name, sizeof(kctl.id.name));
    kctl.id.index = get_available_index(card, kctl.id.name);
    kctl.private_value = 0;
    return kctl;
    }
    void snd_kctl_jack_report(struct snd_card *card,
    struct snd_kcontrol *kctl, bool status)
    {
    if (kctl.private_value == status)
    return;
    kctl.private_value = status;
    snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &kctl.id);
    }
