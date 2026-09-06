//! Automatically rewritten from C to Rust
//! Source: sound/pci/au88x0/au88x0_mixer.c
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
// Vortex Mixer support.
//
// There is much more than just the AC97 mixer...
//

#[no_mangle]
unsafe extern "C" fn remove_ctl(card: *mut snd_card, name: *const c_char) -> c_int {
    static int remove_ctl(struct snd_card *card, const char *name)
    {
    struct snd_ctl_elem_id id;
    memset(&id, 0, sizeof(id));
    strscpy(id.name, name);
    id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    return snd_ctl_remove_id(card, &id);
    }
#[no_mangle]
unsafe extern "C" fn snd_vortex_mixer(vortex: *mut vortex_t) -> c_int {
    static int snd_vortex_mixer(vortex_t *vortex)
    {
    struct snd_ac97_bus *pbus;
    struct snd_ac97_template ac97;
    int err;
    static const struct snd_ac97_bus_ops ops = {
    .write = vortex_codec_write,
    .read = vortex_codec_read,
    };
    err = snd_ac97_bus(vortex.card, 0, &ops, core::ptr::null_mut(), &pbus);
    if (err < 0)
    return err;
    memset(&ac97, 0, sizeof(ac97));
// Initialize AC97 codec stuff.
    ac97.private_data = vortex;
    ac97.scaps = AC97_SCAP_NO_SPDIF;
    err = snd_ac97_mixer(pbus, &ac97, &vortex.codec);
    vortex.isquad = ((vortex.codec == core::ptr::null_mut()) ?  0 : (vortex.codec.ext_id&0x80));
    remove_ctl(vortex.card, "Master Mono Playback Volume");
    remove_ctl(vortex.card, "Master Mono Playback Switch");
    return err;
    }
