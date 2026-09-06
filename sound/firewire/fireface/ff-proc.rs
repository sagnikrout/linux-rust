//! Automatically rewritten from C to Rust
//! Source: sound/firewire/fireface/ff-proc.c
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
// ff-proc.c - a part of driver for RME Fireface series
//
// Copyright (c) 2015-2017 Takashi Sakamoto
//

    const char *snd_ff_proc_get_clk_label(enum snd_ff_clock_src src)
    {
    static const char *const labels[] = {
    "Internal",
    "S/PDIF",
    "ADAT1",
    "ADAT2",
    "Word",
    "LTC",
    };
    if (src >= ARRAY_SIZE(labels))
    return core::ptr::null_mut();
    return labels[src];
    }
    static void proc_dump_status(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct snd_ff *ff = entry.private_data;
    ff.spec.protocol.dump_status(ff, buffer);
    }
    static void add_node(struct snd_ff *ff, struct snd_info_entry *root,
    const char *name,
    void (*op)(struct snd_info_entry *e,
    struct snd_info_buffer *b))
    {
    struct snd_info_entry *entry;
    entry = snd_info_create_card_entry(ff.card, name, root);
    if (entry)
    snd_info_set_text_ops(entry, ff, op);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_ff_proc_init(ff: *mut snd_ff) {
    void snd_ff_proc_init(struct snd_ff *ff)
    {
    struct snd_info_entry *root;
//
// All nodes are automatically removed at snd_card_disconnect(),
// by following to link list.
//
    root = snd_info_create_card_entry(ff.card, "firewire",
    ff.card.proc_root);
    if (root == core::ptr::null_mut())
    return;
    root.mode = S_IFDIR | 0555;
    add_node(ff, root, "status", proc_dump_status);
    }
