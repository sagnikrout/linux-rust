//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/emux_synth.h
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
// Defines for the Emu-series WaveTable chip
//
// Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
//

//
// compile flags
//
// Macro flag: #define SNDRV_EMUX_USE_RAW_EFFECT
//
// operators
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emux_operators {
    pub owner: *mut module,
    pub port): *mut snd_emux_port,
    pub vp): *mut *mut int (prepare)(struct snd_emux_voice,
    pub vp): *mut *mut void (trigger)(struct snd_emux_voice,
    pub vp): *mut *mut void (release)(struct snd_emux_voice,
    pub update): *mut *mut *mut void (update)(struct snd_emux_voice vp, int,
    pub vp): *mut *mut void (terminate)(struct snd_emux_voice,
    pub vp): *mut *mut void (free_voice)(struct snd_emux_voice,
    pub ch): *mut *mut *mut void (reset)(struct snd_emux emu, int,
// the first parameters are struct snd_emux
    pub count): *const *const void __user data, long,
    pub hdr): *mut snd_util_memhdr,
    pub emu): *mut *mut void (sample_reset)(struct snd_emux,
    pub count): *const *const void __user data, long,
    pub chset): *mut snd_midi_channel_set,

    pub p2): *mut *mut *mut int (oss_ioctl)(struct snd_emux emu, int cmd, int p1, int,

    pub emu): *mut *mut int (get_pitch_shift)(struct snd_emux,
}

//
// constant values
//

// simultineously
//
// flags
//

//
// emuX wavetable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emux {
    pub /: *mut *mut *mut snd_card card; / assigned card,
// following should be initialized before registration
    pub /: *mut *mut int max_voices; / Number of voices,
    pub /: *mut *mut int mem_size; / memory size (in byte),
    pub /: *mut *mut int num_ports; / number of ports to be created,
    pub /: *mut *mut snd_emux_operators ops; / operators,
    pub /: *mut *mut *mut void hw; / hardware,
    pub /: *mut *mut unsigned long flags; / other conditions,
    pub /: *mut *mut int midi_ports; / number of virtual midi devices,
    pub /: *mut *mut int midi_devidx; / device offset of virtual midi,
    pub /: *mut *mut unsigned int linear_panning: 1; / panning is linear (sbawe = 1, emu10k1 = 0),
    pub /: *mut *mut int hwdep_idx; / hwdep device index,
    pub /: *mut *mut *mut snd_hwdep hwdep; / hwdep device,
// private
    pub /: *mut *mut int num_voices; / current number of voices,
    pub /: *mut *mut *mut snd_sf_list sflist; / root of SoundFont list,
    pub /: *mut *mut *mut snd_emux_voice voices; / Voices (EMU 'channel'),
    pub /: *mut *mut int use_time; / allocation counter,
    pub /: *mut *mut spinlock_t voice_lock; / Lock for voice access,
    pub register_mutex: mutex,
    pub /: *mut *mut int client; / For the sequencer client,
    pub /: *mut *mut int ports[SNDRV_EMUX_MAX_PORTS]; / The ports for this device,
    pub portptrs: [*mut snd_emux_port; SNDRV_EMUX_MAX_PORTS],
    pub /: *mut *mut int used; / use counter,
    pub /: *const *const *const char name; / name of the device (internal),
    pub vmidi: *mut snd_rawmidi,
    pub /: *mut *mut timer_list tlist; / for pending note-offs,
    pub timer_active: c_int,
    pub /: *mut *mut *mut snd_util_memhdr memhdr; / memory chunk information,

    pub proc: *mut snd_info_entry,

    pub oss_synth: *mut snd_seq_device,

}

//
// sequencer port information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emux_port {
    pub emu: *mut snd_emux,
    pub /: *mut *mut char port_mode; / operation mode,
    pub /: *mut *mut int volume_atten; / emuX raw attenuation,
    pub /: *mut *mut unsigned long drum_flags; / drum bitmaps,
    pub /: *mut *mut int ctrls[EMUX_MD_END]; / control parameters,

    pub effect: *mut snd_emux_effect_table,

    pub oss_arg: *mut snd_seq_oss_arg,

    pub chset: snd_midi_channel_set,
}

// port_mode

//
// A structure to keep track of each hardware voice
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emux_voice {
    pub /: *mut *mut int ch; / Hardware channel number,
    pub /: *mut *mut int state; / status,
pub const SNDRV_EMUX_ST_OFF: c_uint = 0x00	/* Not playing, and inactive */;
pub const SNDRV_EMUX_ST_ON: c_uint = 0x01	/* Note on */;

pub const SNDRV_EMUX_ST_LOCKED: c_uint = 0x100	/* Not accessible */;
    pub /: *mut *mut unsigned int time; / An allocation time,
    pub /: *mut *mut unsigned char note; / Note currently assigned to this voice,
    pub key: c_uchar,
    pub /: *mut *mut unsigned char velocity; / Velocity of current note,
    pub /: *mut *mut *mut snd_sf_zone zone; / Zone assigned to this note,
    pub /: *mut *mut *mut void block; / sample block pointer (optional),
    pub /: *mut *mut *mut snd_midi_channel chan; / Midi channel for this note,
    pub /: *mut *mut *mut snd_emux_port port; / associated port,
    pub /: *mut *mut *mut snd_emux emu; / assigned root info,
    pub /: *mut *mut *mut void hw; / hardware pointer (emu8000 or emu10k1),
    pub /: *mut *mut unsigned long ontime; / jiffies at note triggered,
// Emu8k/Emu10k1 registers
    pub reg: soundfont_voice_info,
// additional registers
    pub /: *mut *mut int avol; / volume attenuation,
    pub /: *mut *mut int acutoff; / cutoff target,
    pub /: *mut *mut int apitch; / pitch offset,
    pub /: *mut *mut int apan; / pan/aux pair,
    pub aaux: c_int,
    pub /: *mut *mut int ptarget; / pitch target,
    pub /: *mut *mut int vtarget; / volume target,
    pub /: *mut *mut int ftarget; / filter target,
}

//
// update flags (can be combined)
//

//
// effect table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emux_effect_table {
// Emu8000 specific effects
    pub val: [c_short; EMUX_NUM_EFFECTS],
    pub flag: [c_uchar; EMUX_NUM_EFFECTS],
}

//
// prototypes - interface to Emu10k1 and Emu8k routines
//
extern "C" {
    pub fn snd_emux_new(remu: *mut snd_emux) -> c_int;
}
extern "C" {
    pub fn snd_emux_register(emu: *mut snd_emux, card: *mut snd_card, index: c_int, name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn snd_emux_free(emu: *mut snd_emux) -> c_int;
}
//
// exported functions
//
extern "C" {
    pub fn snd_emux_terminate_all(emu: *mut snd_emux);
}
extern "C" {
    pub fn snd_emux_lock_voice(emu: *mut snd_emux, voice: c_int);
}
extern "C" {
    pub fn snd_emux_unlock_voice(emu: *mut snd_emux, voice: c_int);
}
