//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/emu8000.h
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
// Defines for the emu8000 (AWE32/64)
//
// Copyright (C) 1999 Steve Ratcliffe
// Copyright (C) 1999-2000 Takashi Iwai <tiwai@suse.de>
//

//
// Hardware parameters.
//

pub const EMU8000_DRAM_OFFSET: c_uint = 0x200000	/* Beginning of on board ram */;

// Flags to set a dma channel to read or write
pub const EMU8000_RAM_READ: c_int = 0;
pub const EMU8000_RAM_WRITE: c_int = 1;
pub const EMU8000_RAM_CLOSE: c_int = 2;
pub const EMU8000_RAM_MODE_MASK: c_uint = 0x03;
pub const EMU8000_RAM_RIGHT: c_uint = 0x10	/* use 'right' DMA channel */;
//
// Structure to hold all state information for the emu8000 driver.
//
// Note 1: The chip supports 32 channels in hardware this is max_channels
// some of the channels may be used for other things so max_channels is
// the number in use for wave voices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu8000 {
    pub emu: *mut snd_emux,
    pub /: *mut *mut int index; / sequencer client index,
    pub /: *mut *mut int seq_ports; / number of sequencer ports,
    pub /: *mut *mut int fm_chorus_depth; / FM OPL3 chorus depth,
    pub /: *mut *mut int fm_reverb_depth; / FM OPL3 reverb depth,
    pub /: *mut *mut int mem_size; / memory size,
    pub /: *mut *mut unsigned long port1; / Port usually base+0,
    pub /: *mut *mut unsigned long port2; / Port usually at base+0x400,
    pub /: *mut *mut unsigned long port3; / Port usually at base+0x800,
    pub /: *mut *mut unsigned short last_reg;/ Last register command,
    pub reg_lock: spinlock_t,
    pub dram_checked: c_int,
    pub /: *mut *mut *mut snd_card card; / The card that this belongs to,
    pub chorus_mode: c_int,
    pub reverb_mode: c_int,
    pub bass_level: c_int,
    pub treble_level: c_int,
    pub memhdr: *mut snd_util_memhdr,
    pub control_lock: spinlock_t,
    pub controls: [*mut snd_kcontrol; EMU8000_NUM_CONTROLS],
    pub /: *mut *mut *mut snd_pcm pcm; / pcm on emu8000 wavetable,
}

// sequencer device id

// exported functions
extern "C" {
    pub fn snd_emu8000_dma_chan(emu: *mut snd_emu8000, ch: c_int, mode: c_int);
}
extern "C" {
    pub fn snd_emu8000_init_fm(emu: *mut snd_emu8000);
}
extern "C" {
    pub fn snd_emu8000_update_chorus_mode(emu: *mut snd_emu8000);
}
extern "C" {
    pub fn snd_emu8000_update_reverb_mode(emu: *mut snd_emu8000);
}
extern "C" {
    pub fn snd_emu8000_update_equalizer(emu: *mut snd_emu8000);
}
extern "C" {
    pub fn snd_emu8000_load_chorus_fx(emu: *mut snd_emu8000, mode: c_int, buf: *const void __user, len: c_long) -> c_int;
}
extern "C" {
    pub fn snd_emu8000_load_reverb_fx(emu: *mut snd_emu8000, mode: c_int, buf: *const void __user, len: c_long) -> c_int;
}
