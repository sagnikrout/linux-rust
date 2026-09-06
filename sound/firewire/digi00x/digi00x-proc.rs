//! Automatically rewritten from C to Rust
//! Source: sound/firewire/digi00x/digi00x-proc.c
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
// digi00x-proc.c - a part of driver for Digidesign Digi 002/003 family
//
// Copyright (c) 2014-2015 Takashi Sakamoto
//

    static int get_optical_iface_mode(struct snd_dg00x *dg00x,
    enum snd_dg00x_optical_mode *mode)
    {
    __be32 data;
    int err;
    err = snd_fw_transaction(dg00x.unit, TCODE_READ_QUADLET_REQUEST,
    DG00X_ADDR_BASE + DG00X_OFFSET_OPT_IFACE_MODE,
    &data, sizeof(data), 0);
    if (err >= 0)
// mode = be32_to_cpu(data) & 0x01;
    return err;
    }
    static void proc_read_clock(struct snd_info_entry *entry,
    struct snd_info_buffer *buf)
    {
    static const char *const source_name[] = {
    [SND_DG00X_CLOCK_INTERNAL] = "internal",
    [SND_DG00X_CLOCK_SPDIF] = "s/pdif",
    [SND_DG00X_CLOCK_ADAT] = "adat",
    [SND_DG00X_CLOCK_WORD] = "word clock",
    };
    static const char *const optical_name[] = {
    [SND_DG00X_OPT_IFACE_MODE_ADAT] = "adat",
    [SND_DG00X_OPT_IFACE_MODE_SPDIF] = "s/pdif",
    };
    struct snd_dg00x *dg00x = entry.private_data;
    enum snd_dg00x_optical_mode mode;
    unsigned int rate;
    enum snd_dg00x_clock clock;
    bool detect;
    if (get_optical_iface_mode(dg00x, &mode) < 0)
    return;
    if (snd_dg00x_stream_get_local_rate(dg00x, &rate) < 0)
    return;
    if (snd_dg00x_stream_get_clock(dg00x, &clock) < 0)
    return;
    snd_iprintf(buf, "Optical mode: %s\n", optical_name[mode]);
    snd_iprintf(buf, "Sampling Rate: %d\n", rate);
    snd_iprintf(buf, "Clock Source: %s\n", source_name[clock]);
    if (clock == SND_DG00X_CLOCK_INTERNAL)
    return;
    if (snd_dg00x_stream_check_external_clock(dg00x, &detect) < 0)
    return;
    snd_iprintf(buf, "External source: %s\n", detect ? "detected" : "not");
    if (!detect)
    return;
    if (snd_dg00x_stream_get_external_rate(dg00x, &rate) >= 0)
    snd_iprintf(buf, "External sampling rate: %d\n", rate);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_dg00x_proc_init(dg00x: *mut snd_dg00x) {
    void snd_dg00x_proc_init(struct snd_dg00x *dg00x)
    {
    struct snd_info_entry *root, *entry;
//
// All nodes are automatically removed at snd_card_disconnect(),
// by following to link list.
//
    root = snd_info_create_card_entry(dg00x.card, "firewire",
    dg00x.card.proc_root);
    if (root == core::ptr::null_mut())
    return;
    root.mode = S_IFDIR | 0555;
    entry = snd_info_create_card_entry(dg00x.card, "clock", root);
    if (entry)
    snd_info_set_text_ops(entry, dg00x, proc_read_clock);
    }
