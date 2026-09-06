//! Automatically rewritten from C to Rust
//! Source: sound/firewire/bebob/bebob_proc.c
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
// bebob_proc.c - a part of driver for BeBoB based devices
//
// Copyright (c) 2013-2014 Takashi Sakamoto
//

// contents of information register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_info {
    pub manufacturer: u64,
    pub protocol_ver: u32,
    pub bld_ver: u32,
    pub guid: [u32; 2],
    pub model_id: u32,
    pub model_rev: u32,
    pub fw_date: u64,
    pub fw_time: u64,
    pub fw_id: u32,
    pub fw_ver: u32,
    pub base_addr: u32,
    pub max_size: u32,
    pub bld_date: u64,
    pub bld_time: u64,
// may not used in product
    pub dbg_date: u64,
    pub dbg_time: u64,
    pub dbg_id: u32,
    pub dbg_version: u32,
//
    pub __packed: },
    static void
    proc_read_hw_info(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    pub entry->private_data: *mut *mut snd_bebob bebob =,
    pub info: *mut hw_info,
    pub hw_info): info = kzalloc_obj(struct,
    if (info == core::ptr::null_mut())
    if (snd_bebob_read_block(bebob.unit, 0,
    info, sizeof(struct hw_info)) < 0)
    pub end: goto,
    snd_iprintf(buffer, "Manufacturer:\t%.8s\n",
    pub )&info->manufacturer): *mut (char,
    pub info->protocol_ver): snd_iprintf(buffer, "Protocol Ver:\t%d\n",,
    pub info->bld_ver): snd_iprintf(buffer, "Build Ver:\t%d\n",,
    snd_iprintf(buffer, "GUID:\t\t0x%.8X%.8X\n",
    pub info->guid[1]): info->guid[0],,
    pub info->model_id): snd_iprintf(buffer, "Model ID:\t0x%02X\n",,
    pub info->model_rev): snd_iprintf(buffer, "Model Rev:\t%d\n",,
    pub )&info->fw_date): *mut snd_iprintf(buffer, "Firmware Date:\t%.8s\n", (char,
    pub )&info->fw_time): *mut snd_iprintf(buffer, "Firmware Time:\t%.8s\n", (char,
    pub info->fw_id): snd_iprintf(buffer, "Firmware ID:\t0x%X\n",,
    pub info->fw_ver): snd_iprintf(buffer, "Firmware Ver:\t%d\n",,
    pub info->base_addr): snd_iprintf(buffer, "Base Addr:\t0x%X\n",,
    pub info->max_size): snd_iprintf(buffer, "Max Size:\t%d\n",,
    pub )&info->bld_date): *mut snd_iprintf(buffer, "Loader Date:\t%.8s\n", (char,
    pub )&info->bld_time): *mut snd_iprintf(buffer, "Loader Time:\t%.8s\n", (char,
    end:
    }
    static void
    proc_read_meters(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    pub entry->private_data: *mut *mut snd_bebob bebob =,
    pub bebob->spec->meter: *const *const snd_bebob_meter_spec spec =,
    pub buf: *mut u32,
    pub size: unsigned int i, c, channels,,
    if (spec == core::ptr::null_mut())
    pub 2: *mut *mut channels = spec->num,
    pub sizeof(u32): *mut *mut size = channels,
    pub GFP_KERNEL): buf = kmalloc(size,,
    if (buf == core::ptr::null_mut())
    if (spec.get(bebob, buf, size) < 0)
    pub end: goto,
    pub {: for (i = 0, c = 1; i < channels; i++),
    snd_iprintf(buffer, "%s %d:\t%d\n",
    pub buf[i]): spec->labels[i / 2], c++,,
    if ((i + 1 < channels - 1) &&
    (strcmp(spec.labels[i / 2],
    spec.labels[(i + 1) / 2]) != 0))
    pub 1: c =,
    }
    end:
    }
    static void
    proc_read_formation(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    pub entry->private_data: *mut *mut snd_bebob bebob =,
    pub formation: *mut snd_bebob_stream_formation,
    pub i: c_uint,
    pub device:\n"): snd_iprintf(buffer, "Output Stream from,
    pub "\tRate\tPCM\tMIDI\n"): snd_iprintf(buffer,,
    pub bebob->tx_stream_formations: formation =,
    pub {: for (i = 0; i < SND_BEBOB_STRM_FMT_ENTRIES; i++),
    snd_iprintf(buffer,
    "\t%d\t%d\t%d\n", snd_bebob_rate_table[i],
    pub formation[i].midi): formation[i].pcm,,
    }
    pub device:\n"): snd_iprintf(buffer, "Input Stream to,
    pub "\tRate\tPCM\tMIDI\n"): snd_iprintf(buffer,,
    pub bebob->rx_stream_formations: formation =,
    pub {: for (i = 0; i < SND_BEBOB_STRM_FMT_ENTRIES; i++),
    snd_iprintf(buffer,
    "\t%d\t%d\t%d\n", snd_bebob_rate_table[i],
    pub formation[i].midi): formation[i].pcm,,
    }
    }
    static void
    proc_read_clock(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    static const char *const clk_labels[] = {
    "Internal",
    "External",
    "SYT-Match",
}

    struct snd_bebob *bebob = entry.private_data;
    const struct snd_bebob_rate_spec *rate_spec = bebob.spec.rate;
    const struct snd_bebob_clock_spec *clk_spec = bebob.spec.clock;
    enum snd_bebob_clock_type src;
    unsigned int rate;
    if (rate_spec.get(bebob, &rate) >= 0)
    snd_iprintf(buffer, "Sampling rate: %d\n", rate);
    if (snd_bebob_stream_get_clock_src(bebob, &src) >= 0) {
    if (clk_spec)
    snd_iprintf(buffer, "Clock Source: %s\n",
    clk_labels[src]);
    else
    snd_iprintf(buffer, "Clock Source: %s (MSU-dest: %d)\n",
    clk_labels[src], bebob.sync_input_plug);
    }
    }
    static void
    add_node(struct snd_bebob *bebob, struct snd_info_entry *root, const char *name,
    void (*op)(struct snd_info_entry *e, struct snd_info_buffer *b))
    {
    struct snd_info_entry *entry;
    entry = snd_info_create_card_entry(bebob.card, name, root);
    if (entry)
    snd_info_set_text_ops(entry, bebob, op);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_bebob_proc_init(bebob: *mut snd_bebob) {
    void snd_bebob_proc_init(struct snd_bebob *bebob)
    {
    struct snd_info_entry *root;
//
// All nodes are automatically removed at snd_card_disconnect(),
// by following to link list.
//
    root = snd_info_create_card_entry(bebob.card, "firewire",
    bebob.card.proc_root);
    if (root == core::ptr::null_mut())
    return;
    root.mode = S_IFDIR | 0555;
    add_node(bebob, root, "clock", proc_read_clock);
    add_node(bebob, root, "firmware", proc_read_hw_info);
    add_node(bebob, root, "formation", proc_read_formation);
    if (bebob.spec.meter != core::ptr::null_mut())
    add_node(bebob, root, "meter", proc_read_meters);
    }
