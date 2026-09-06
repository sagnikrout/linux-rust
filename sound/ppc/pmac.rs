//! Automatically rewritten from C Header to Rust Module
//! Source: sound/ppc/pmac.h
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
// Driver for PowerMac onboard soundchips
// Copyright (c) 2001 by Takashi Iwai <tiwai@suse.de>
// based on dmasound.c.
//

// maximum number of fragments
pub const PMAC_MAX_FRAGS: c_int = 32;
// Macro flag: #define PMAC_SUPPORT_AUTOMUTE
//
// DBDMA space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_dbdma {
    pub dma_base: dma_addr_t,
    pub addr: dma_addr_t,
    pub cmds: *mut dbdma_cmd __iomem,
    pub space: *mut c_void,
    pub size: c_int,
}

//
// playback/capture stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_stream {
    pub /: *mut *mut int running; / boolean,
    pub /: *mut *mut int stream; / PLAYBACK/CAPTURE,
    pub /: *mut *mut int dma_size; / in bytes,
    pub /: *mut *mut int period_size; / in bytes,
    pub /: *mut *mut int buffer_size; / in kbytes,
    pub cur_period: int nperiods,,
    pub cmd: pmac_dbdma,
    pub dma: *mut volatile struct dbdma_regs __iomem,
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut unsigned int cur_freqs; / currently available frequencies,
    pub /: *mut *mut unsigned int cur_formats; / currently available formats,
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_pmac_model {
    PMAC_AWACS, PMAC_SCREAMER, PMAC_BURGUNDY, PMAC_DACA, PMAC_TUMBLER,
    PMAC_SNAPPER
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pmac {
    pub card: *mut snd_card,
// h/w info
    pub node: *mut device_node,
    pub pdev: *mut pci_dev,
    pub revision: c_uint,
    pub manufacturer: c_uint,
    pub subframe: c_uint,
    pub device_id: c_uint,
    pub model: snd_pmac_model,
    pub 1: unsigned int has_iic :,
    pub 1: unsigned int is_pbook_3400 :,
    pub 1: unsigned int is_pbook_G3 :,
    pub 1: unsigned int is_k2 :,
    pub 1: unsigned int can_byte_swap :,
    pub 1: unsigned int can_duplex :,
    pub 1: unsigned int can_capture :,
    pub 1: unsigned int auto_mute :,
    pub 1: unsigned int initialized :,
    pub 1: unsigned int feature_is_set :,
    pub requested: c_uint,
    pub rsrc: [resource; 3],
    pub num_freqs: c_int,
    pub freq_table: *const c_int,
    pub /: *mut *mut unsigned int freqs_ok; / bit flags,
    pub /: *mut *mut unsigned int formats_ok; / pcm hwinfo,
    pub active: c_int,
    pub rate_index: c_int,
    pub /: *mut *mut int format; / current format,
    pub reg_lock: spinlock_t,
    pub awacs: *mut volatile struct awacs_regs __iomem,
    pub /: *mut *mut int awacs_reg[8]; / register cache,
    pub hp_stat_mask: c_uint,
    pub latch_base: *mut unsigned char __iomem,
    pub macio_base: *mut unsigned char __iomem,
    pub playback: pmac_stream,
    pub capture: pmac_stream,
    pub extra_dma: pmac_dbdma,
    pub rx_irq: int irq, tx_irq,,
    pub pcm: *mut snd_pcm,
    pub beep: *mut pmac_beep,
    pub /: *mut *mut unsigned int control_mask; / control mask,
// mixer stuffs
    pub mixer_data: *mut c_void,
    pub ): *mut *mut void (mixer_free)(struct snd_pmac,
    pub master_sw_ctl: *mut snd_kcontrol,
    pub speaker_sw_ctl: *mut snd_kcontrol,
    pub /: *mut *mut *mut snd_kcontrol drc_sw_ctl; / only used for tumbler -ReneR,
    pub hp_detect_ctl: *mut snd_kcontrol,
    pub lineout_sw_ctl: *mut snd_kcontrol,
// lowlevel callbacks
    pub chip): *mut *mut void (set_format)(struct snd_pmac,
    pub do_notify): *mut *mut *mut void (update_automute)(struct snd_pmac chip, int,
    pub chip): *mut *mut int (detect_headphone)(struct snd_pmac,

    pub chip): *mut *mut void (suspend)(struct snd_pmac,
    pub chip): *mut *mut void (resume)(struct snd_pmac,

}

// exported functions
extern "C" {
    pub fn snd_pmac_new(card: *mut snd_card, chip_return: *mut snd_pmac) -> c_int;
}
extern "C" {
    pub fn snd_pmac_pcm_new(chip: *mut snd_pmac) -> c_int;
}
extern "C" {
    pub fn snd_pmac_attach_beep(chip: *mut snd_pmac) -> c_int;
}
extern "C" {
    pub fn snd_pmac_detach_beep(chip: *mut snd_pmac);
}
extern "C" {
    pub fn snd_pmac_beep_stop(chip: *mut snd_pmac);
}
extern "C" {
    pub fn snd_pmac_rate_index(chip: *mut snd_pmac, rec: *mut pmac_stream, rate: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_pmac_beep_dma_start(chip: *mut snd_pmac, bytes: c_int, addr: c_ulong, speed: c_int);
}
extern "C" {
    pub fn snd_pmac_beep_dma_stop(chip: *mut snd_pmac);
}

extern "C" {
    pub fn snd_pmac_suspend(chip: *mut snd_pmac);
}
extern "C" {
    pub fn snd_pmac_resume(chip: *mut snd_pmac);
}

// initialize mixer
extern "C" {
    pub fn snd_pmac_awacs_init(chip: *mut snd_pmac) -> c_int;
}
extern "C" {
    pub fn snd_pmac_burgundy_init(chip: *mut snd_pmac) -> c_int;
}
extern "C" {
    pub fn snd_pmac_daca_init(chip: *mut snd_pmac) -> c_int;
}
extern "C" {
    pub fn snd_pmac_tumbler_init(chip: *mut snd_pmac) -> c_int;
}
extern "C" {
    pub fn snd_pmac_tumbler_post_init() -> c_int;
}
// i2c functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_keywest {
    pub addr: c_int,
    pub client: *mut i2c_client,
    pub id: c_int,
    pub i2c): *mut *mut int (init_client)(struct pmac_keywest,
    pub name: *mut c_char,
}

extern "C" {
    pub fn snd_pmac_keywest_init(i2c: *mut pmac_keywest) -> c_int;
}
extern "C" {
    pub fn snd_pmac_keywest_cleanup(i2c: *mut pmac_keywest);
}
// misc

extern "C" {
    pub fn snd_pmac_add_automute(chip: *mut snd_pmac) -> c_int;
}
