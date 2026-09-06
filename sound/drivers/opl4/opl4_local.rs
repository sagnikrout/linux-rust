//! Automatically rewritten from C Header to Rust Module
//! Source: sound/drivers/opl4/opl4_local.h
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


//
// Local definitions for the OPL4 driver
//
// Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, this software may be distributed and/or modified under the
// terms of the GNU General Public License as published by the Free Software
// Foundation; either version 2 of the License, or (at your option) any later
// version.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

//
// Register numbers
//
pub const OPL4_REG_TEST0: c_uint = 0x00;
pub const OPL4_REG_TEST1: c_uint = 0x01;
pub const OPL4_REG_MEMORY_CONFIGURATION: c_uint = 0x02;
pub const OPL4_MODE_BIT: c_uint = 0x01;
pub const OPL4_MTYPE_BIT: c_uint = 0x02;
pub const OPL4_TONE_HEADER_MASK: c_uint = 0x1c;
pub const OPL4_DEVICE_ID_MASK: c_uint = 0xe0;
pub const OPL4_REG_MEMORY_ADDRESS_HIGH: c_uint = 0x03;
pub const OPL4_REG_MEMORY_ADDRESS_MID: c_uint = 0x04;
pub const OPL4_REG_MEMORY_ADDRESS_LOW: c_uint = 0x05;
pub const OPL4_REG_MEMORY_DATA: c_uint = 0x06;
//
// Offsets to the register banks for voices. To get the
// register number just add the voice number to the bank offset.
//
// Wave Table Number low bits (0x08 to 0x1F)
//
pub const OPL4_REG_TONE_NUMBER: c_uint = 0x08;
// Wave Table Number high bit, F-Number low bits (0x20 to 0x37)
pub const OPL4_REG_F_NUMBER: c_uint = 0x20;
pub const OPL4_TONE_NUMBER_BIT8: c_uint = 0x01;
pub const OPL4_F_NUMBER_LOW_MASK: c_uint = 0xfe;
// F-Number high bits, Octave, Pseudo-Reverb (0x38 to 0x4F)
pub const OPL4_REG_OCTAVE: c_uint = 0x38;
pub const OPL4_F_NUMBER_HIGH_MASK: c_uint = 0x07;
pub const OPL4_BLOCK_MASK: c_uint = 0xf0;
pub const OPL4_PSEUDO_REVERB_BIT: c_uint = 0x08;
// Total Level, Level Direct (0x50 to 0x67)
pub const OPL4_REG_LEVEL: c_uint = 0x50;
pub const OPL4_TOTAL_LEVEL_MASK: c_uint = 0xfe;
pub const OPL4_LEVEL_DIRECT_BIT: c_uint = 0x01;
// Key On, Damp, LFO RST, CH, Panpot (0x68 to 0x7F)
pub const OPL4_REG_MISC: c_uint = 0x68;
pub const OPL4_KEY_ON_BIT: c_uint = 0x80;
pub const OPL4_DAMP_BIT: c_uint = 0x40;
pub const OPL4_LFO_RESET_BIT: c_uint = 0x20;
pub const OPL4_OUTPUT_CHANNEL_BIT: c_uint = 0x10;
pub const OPL4_PAN_POT_MASK: c_uint = 0x0f;
// LFO, VIB (0x80 to 0x97)
pub const OPL4_REG_LFO_VIBRATO: c_uint = 0x80;
pub const OPL4_LFO_FREQUENCY_MASK: c_uint = 0x38;
pub const OPL4_VIBRATO_DEPTH_MASK: c_uint = 0x07;
pub const OPL4_CHORUS_SEND_MASK: c_uint = 0xc0 /* ML only */;
// Attack / Decay 1 rate (0x98 to 0xAF)
pub const OPL4_REG_ATTACK_DECAY1: c_uint = 0x98;
pub const OPL4_ATTACK_RATE_MASK: c_uint = 0xf0;
pub const OPL4_DECAY1_RATE_MASK: c_uint = 0x0f;
// Decay level / 2 rate (0xB0 to 0xC7)
pub const OPL4_REG_LEVEL_DECAY2: c_uint = 0xb0;
pub const OPL4_DECAY_LEVEL_MASK: c_uint = 0xf0;
pub const OPL4_DECAY2_RATE_MASK: c_uint = 0x0f;
// Release rate / Rate correction (0xC8 to 0xDF)
pub const OPL4_REG_RELEASE_CORRECTION: c_uint = 0xc8;
pub const OPL4_RELEASE_RATE_MASK: c_uint = 0x0f;
pub const OPL4_RATE_INTERPOLATION_MASK: c_uint = 0xf0;
// AM (0xE0 to 0xF7)
pub const OPL4_REG_TREMOLO: c_uint = 0xe0;
pub const OPL4_TREMOLO_DEPTH_MASK: c_uint = 0x07;
pub const OPL4_REVERB_SEND_MASK: c_uint = 0xe0 /* ML only */;
// Mixer
pub const OPL4_REG_MIX_CONTROL_FM: c_uint = 0xf8;
pub const OPL4_REG_MIX_CONTROL_PCM: c_uint = 0xf9;
pub const OPL4_MIX_LEFT_MASK: c_uint = 0x07;
pub const OPL4_MIX_RIGHT_MASK: c_uint = 0x38;
pub const OPL4_REG_ATC: c_uint = 0xfa;
pub const OPL4_ATC_BIT: c_uint = 0x01 /* ???, ML only */;
// bits in the OPL3 Status register
pub const OPL4_STATUS_BUSY: c_uint = 0x01;
pub const OPL4_STATUS_LOAD: c_uint = 0x02;
pub const OPL4_MAX_VOICES: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opl4_sound {
    pub tone: u16,
    pub pitch_offset: i16,
    pub key_scaling: u8,
    pub panpot: i8,
    pub vibrato: u8,
    pub tone_attenuate: u8,
    pub volume_factor: u8,
    pub reg_lfo_vibrato: u8,
    pub reg_attack_decay1: u8,
    pub reg_level_decay2: u8,
    pub reg_release_correction: u8,
    pub reg_tremolo: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opl4_region {
    pub key_max: u8 key_min,,
    pub sound: opl4_sound,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opl4_region_ptr {
    pub count: c_int,
    pub regions: *const opl4_region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opl4_voice {
    pub list: list_head,
    pub number: c_int,
    pub chan: *mut snd_midi_channel,
    pub note: c_int,
    pub velocity: c_int,
    pub sound: *const opl4_sound,
    pub level_direct: u8,
    pub reg_f_number: u8,
    pub reg_misc: u8,
    pub reg_lfo_vibrato: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_opl4 {
    pub fm_port: c_ulong,
    pub pcm_port: c_ulong,
    pub res_fm_port: *mut resource,
    pub res_pcm_port: *mut resource,
    pub hardware: c_ushort,
    pub reg_lock: spinlock_t,
    pub card: *mut snd_card,

    pub proc_entry: *mut snd_info_entry,
    pub memory_access: c_int,

    pub access_mutex: mutex,

    pub used: c_int,
    pub seq_dev_num: c_int,
    pub seq_client: c_int,
    pub seq_dev: *mut snd_seq_device,
    pub chset: *mut snd_midi_channel_set,
    pub voices: [opl4_voice; OPL4_MAX_VOICES],
    pub off_voices: list_head,
    pub on_voices: list_head,

}

// opl4_lib.c
extern "C" {
    pub fn snd_opl4_write(opl4: *mut snd_opl4, reg: u8, value: u8);
}
extern "C" {
    pub fn snd_opl4_read(opl4: *mut snd_opl4, reg: u8) -> u8;
}
extern "C" {
    pub fn snd_opl4_read_memory(opl4: *mut snd_opl4, buf: *mut c_char, offset: c_int, size: c_int);
}
extern "C" {
    pub fn snd_opl4_write_memory(opl4: *mut snd_opl4, buf: *const c_char, offset: c_int, size: c_int);
}
// opl4_mixer.c
extern "C" {
    pub fn snd_opl4_create_mixer(opl4: *mut snd_opl4) -> c_int;
}

// opl4_proc.c
extern "C" {
    pub fn snd_opl4_create_proc(opl4: *mut snd_opl4) -> c_int;
}
extern "C" {
    pub fn snd_opl4_free_proc(opl4: *mut snd_opl4);
}

// opl4_seq.c
// opl4_synth.c
extern "C" {
    pub fn snd_opl4_synth_reset(opl4: *mut snd_opl4);
}
extern "C" {
    pub fn snd_opl4_synth_shutdown(opl4: *mut snd_opl4);
}
extern "C" {
    pub fn snd_opl4_note_on(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl4_note_off(p: *mut c_void, note: c_int, vel: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl4_terminate_note(p: *mut c_void, note: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl4_control(p: *mut c_void, type: c_int, chan: *mut snd_midi_channel);
}
extern "C" {
    pub fn snd_opl4_sysex(p: *mut c_void, buf: *mut c_uchar, len: c_int, parsed: c_int, chset: *mut snd_midi_channel_set);
}
// yrw801.c
extern "C" {
    pub fn snd_yrw801_detect(opl4: *mut snd_opl4) -> c_int;
}
