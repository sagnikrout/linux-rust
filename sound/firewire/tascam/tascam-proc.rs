//! Automatically rewritten from C to Rust
//! Source: sound/firewire/tascam/tascam-proc.c
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
// tascam-proc.h - a part of driver for TASCAM FireWire series
//
// Copyright (c) 2015 Takashi Sakamoto
//

    static void proc_read_firmware(struct snd_info_entry *entry,
    struct snd_info_buffer *buffer)
    {
    struct snd_tscm *tscm = entry.private_data;
    __be32 data;
    unsigned int reg, fpga, arm, hw;
    int err;
    err = snd_fw_transaction(tscm.unit, TCODE_READ_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_REGISTER,
    &data, sizeof(data), 0);
    if (err < 0)
    return;
    reg = be32_to_cpu(data);
    err = snd_fw_transaction(tscm.unit, TCODE_READ_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_FPGA,
    &data, sizeof(data), 0);
    if (err < 0)
    return;
    fpga = be32_to_cpu(data);
    err = snd_fw_transaction(tscm.unit, TCODE_READ_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_ARM,
    &data, sizeof(data), 0);
    if (err < 0)
    return;
    arm = be32_to_cpu(data);
    err = snd_fw_transaction(tscm.unit, TCODE_READ_QUADLET_REQUEST,
    TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_HW,
    &data, sizeof(data), 0);
    if (err < 0)
    return;
    hw = be32_to_cpu(data);
    snd_iprintf(buffer, "Register: %d (0x%08x)\n", reg & 0xffff, reg);
    snd_iprintf(buffer, "FPGA:     %d (0x%08x)\n", fpga & 0xffff, fpga);
    snd_iprintf(buffer, "ARM:      %d (0x%08x)\n", arm & 0xffff, arm);
    snd_iprintf(buffer, "Hardware: %d (0x%08x)\n", hw >> 16, hw);
    }
    static void add_node(struct snd_tscm *tscm, struct snd_info_entry *root,
    const char *name,
    void (*op)(struct snd_info_entry *e,
    struct snd_info_buffer *b))
    {
    struct snd_info_entry *entry;
    entry = snd_info_create_card_entry(tscm.card, name, root);
    if (entry)
    snd_info_set_text_ops(entry, tscm, op);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_tscm_proc_init(tscm: *mut snd_tscm) {
    void snd_tscm_proc_init(struct snd_tscm *tscm)
    {
    struct snd_info_entry *root;
//
// All nodes are automatically removed at snd_card_disconnect(),
// by following to link list.
//
    root = snd_info_create_card_entry(tscm.card, "firewire",
    tscm.card.proc_root);
    if (root == core::ptr::null_mut())
    return;
    root.mode = S_IFDIR | 0555;
    add_node(tscm, root, "firmware", proc_read_firmware);
    }
