//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/cobalt/cobalt-alsa-main.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA interface to cobalt PCM capture streams
//
// Copyright 2014-2015 Cisco Systems, Inc. and/or its affiliates.
// All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn snd_cobalt_card_free(cobsc: *mut snd_cobalt_card) {
    static void snd_cobalt_card_free(struct snd_cobalt_card *cobsc)
    {
    if (cobsc == core::ptr::null_mut())
    return;
    cobsc.s.alsa = core::ptr::null_mut();
    kfree(cobsc);
    }
#[no_mangle]
unsafe extern "C" fn snd_cobalt_card_private_free(sc: *mut snd_card) {
    static void snd_cobalt_card_private_free(struct snd_card *sc)
    {
    if (sc == core::ptr::null_mut())
    return;
    snd_cobalt_card_free(sc.private_data);
    sc.private_data = core::ptr::null_mut();
    sc.private_free = core::ptr::null_mut();
    }
    static int snd_cobalt_card_create(struct cobalt_stream *s,
    struct snd_card *sc,
    struct snd_cobalt_card **cobsc)
    {
// cobsc = kzalloc_obj(struct snd_cobalt_card);
    if (*cobsc == core::ptr::null_mut())
    return -ENOMEM;
    (*cobsc).s = s;
    (*cobsc).sc = sc;
    sc.private_data = *cobsc;
    sc.private_free = snd_cobalt_card_private_free;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cobalt_card_set_names(cobsc: *mut snd_cobalt_card) -> c_int {
    static int snd_cobalt_card_set_names(struct snd_cobalt_card *cobsc)
    {
    struct cobalt_stream *s = cobsc.s;
    struct cobalt *cobalt = s.cobalt;
    struct snd_card *sc = cobsc.sc;
// sc->driver is used by alsa-lib's configurator: simple, unique
    strscpy(sc.driver, "cobalt", sizeof(sc.driver));
// sc->shortname is a symlink in /proc/asound: COBALT-M -> cardN
    snprintf(sc.shortname,  sizeof(sc.shortname), "cobalt-%d-%d",
    cobalt.instance, s.video_channel);
// sc->longname is read from /proc/asound/cards
    snprintf(sc.longname, sizeof(sc.longname),
    "Cobalt %d HDMI %d",
    cobalt.instance, s.video_channel);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cobalt_alsa_init(s: *mut cobalt_stream) -> c_int {
    int cobalt_alsa_init(struct cobalt_stream *s)
    {
    struct cobalt *cobalt = s.cobalt;
    struct snd_card *sc = core::ptr::null_mut();
    struct snd_cobalt_card *cobsc;
    int ret;
// Numbrs steps from "Writing an ALSA Driver" by Takashi Iwai
// (1) Check and increment the device index
// This is a no-op for us.  We'll use the cobalt->instance
// (2) Create a card instance
    ret = snd_card_new(&cobalt.pci_dev.dev, SNDRV_DEFAULT_IDX1,
    SNDRV_DEFAULT_STR1, THIS_MODULE, 0, &sc);
    if (ret) {
    cobalt_err("snd_card_new() failed with err %d\n", ret);
    goto err_exit;
    }
// (3) Create a main component
    ret = snd_cobalt_card_create(s, sc, &cobsc);
    if (ret) {
    cobalt_err("snd_cobalt_card_create() failed with err %d\n",
    ret);
    goto err_exit_free;
    }
// (4) Set the driver ID and name strings
    snd_cobalt_card_set_names(cobsc);
    ret = snd_cobalt_pcm_create(cobsc);
    if (ret) {
    cobalt_err("snd_cobalt_pcm_create() failed with err %d\n",
    ret);
    goto err_exit_free;
    }
// FIXME - proc files
// (7) Set the driver data and return 0
// We do this out of normal order for PCI drivers to avoid races
    s.alsa = cobsc;
// (6) Register the card instance
    ret = snd_card_register(sc);
    if (ret) {
    s.alsa = core::ptr::null_mut();
    cobalt_err("snd_card_register() failed with err %d\n", ret);
    goto err_exit_free;
    }
    return 0;
    err_exit_free:
    if (sc != core::ptr::null_mut())
    snd_card_free(sc);
    err_exit:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cobalt_alsa_exit(s: *mut cobalt_stream) {
    void cobalt_alsa_exit(struct cobalt_stream *s)
    {
    struct snd_cobalt_card *cobsc = s.alsa;
    if (cobsc)
    snd_card_free(cobsc.sc);
    s.alsa = core::ptr::null_mut();
    }
