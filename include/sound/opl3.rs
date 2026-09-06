//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/opl3.h
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
// Definitions of the OPL-3 registers.
//
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// Hannu Savolainen 1993-1996
//
// The OPL-3 mode is switched on by writing 0x01, to the offset 5
// of the right side.
//
// Another special register at the right side is at offset 4. It contains
// a bit mask defining which voices are used as 4 OP voices.
//
// The percussive mode is implemented in the left side only.
//
// With the above exceptions the both sides can be operated independently.
//
// A 4 OP voice can be created by setting the corresponding
// bit at offset 4 of the right side.
//
// For example setting the rightmost bit (0x01) changes the
// first voice on the right side to the 4 OP mode. The fourth
// voice is made inaccessible.
//
// If a voice is set to the 2 OP mode, it works like 2 OP modes
// of the original YM3812 (AdLib). In addition the voice can
// be connected the left, right or both stereo channels. It can
// even be left unconnected. This works with 4 OP voices also.
//
// The stereo connection bits are located in the FEEDBACK_CONNECTION
// register of the voice (0xC0-0xC8). In 4 OP voices these bits are
// in the second half of the voice.
//

//
// Register numbers for the global registers
//
pub const OPL3_REG_TEST: c_uint = 0x01;
pub const OPL3_ENABLE_WAVE_SELECT: c_uint = 0x20;
pub const OPL3_REG_TIMER1: c_uint = 0x02;
pub const OPL3_REG_TIMER2: c_uint = 0x03;
pub const OPL3_REG_TIMER_CONTROL: c_uint = 0x04	/* Left side */;
pub const OPL3_IRQ_RESET: c_uint = 0x80;
pub const OPL3_TIMER1_MASK: c_uint = 0x40;
pub const OPL3_TIMER2_MASK: c_uint = 0x20;
pub const OPL3_TIMER1_START: c_uint = 0x01;
pub const OPL3_TIMER2_START: c_uint = 0x02;
pub const OPL3_REG_CONNECTION_SELECT: c_uint = 0x04	/* Right side */;
pub const OPL3_LEFT_4OP_0: c_uint = 0x01;
pub const OPL3_LEFT_4OP_1: c_uint = 0x02;
pub const OPL3_LEFT_4OP_2: c_uint = 0x04;
pub const OPL3_RIGHT_4OP_0: c_uint = 0x08;
pub const OPL3_RIGHT_4OP_1: c_uint = 0x10;
pub const OPL3_RIGHT_4OP_2: c_uint = 0x20;
pub const OPL3_REG_MODE: c_uint = 0x05	/* Right side */;
pub const OPL3_OPL3_ENABLE: c_uint = 0x01	/* OPL3 mode */;
pub const OPL3_OPL4_ENABLE: c_uint = 0x02	/* OPL4 mode */;
pub const OPL3_REG_KBD_SPLIT: c_uint = 0x08	/* Left side */;
pub const OPL3_COMPOSITE_SINE_WAVE_MODE: c_uint = 0x80	/* Don't use with OPL-3? */;
pub const OPL3_KEYBOARD_SPLIT: c_uint = 0x40;
pub const OPL3_REG_PERCUSSION: c_uint = 0xbd	/* Left side only */;
pub const OPL3_TREMOLO_DEPTH: c_uint = 0x80;
pub const OPL3_VIBRATO_DEPTH: c_uint = 0x40;
pub const OPL3_PERCUSSION_ENABLE: c_uint = 0x20;
pub const OPL3_BASSDRUM_ON: c_uint = 0x10;
pub const OPL3_SNAREDRUM_ON: c_uint = 0x08;
pub const OPL3_TOMTOM_ON: c_uint = 0x04;
pub const OPL3_CYMBAL_ON: c_uint = 0x02;
pub const OPL3_HIHAT_ON: c_uint = 0x01;
//
// Offsets to the register banks for operators. To get the
// register number just add the operator offset to the bank offset
//
// AM/VIB/EG/KSR/Multiple (0x20 to 0x35)
//
pub const OPL3_REG_AM_VIB: c_uint = 0x20;
pub const OPL3_TREMOLO_ON: c_uint = 0x80;
pub const OPL3_VIBRATO_ON: c_uint = 0x40;
pub const OPL3_SUSTAIN_ON: c_uint = 0x20;
pub const OPL3_KSR: c_uint = 0x10	/* Key scaling rate */;
pub const OPL3_MULTIPLE_MASK: c_uint = 0x0f	/* Frequency multiplier */;
//
// KSL/Total level (0x40 to 0x55)
//
pub const OPL3_REG_KSL_LEVEL: c_uint = 0x40;
pub const OPL3_KSL_MASK: c_uint = 0xc0	/* Envelope scaling bits */;
pub const OPL3_TOTAL_LEVEL_MASK: c_uint = 0x3f	/* Strength (volume) of OP */;
//
// Attack / Decay rate (0x60 to 0x75)
//
pub const OPL3_REG_ATTACK_DECAY: c_uint = 0x60;
pub const OPL3_ATTACK_MASK: c_uint = 0xf0;
pub const OPL3_DECAY_MASK: c_uint = 0x0f;
//
// Sustain level / Release rate (0x80 to 0x95)
//
pub const OPL3_REG_SUSTAIN_RELEASE: c_uint = 0x80;
pub const OPL3_SUSTAIN_MASK: c_uint = 0xf0;
pub const OPL3_RELEASE_MASK: c_uint = 0x0f;
//
// Wave select (0xE0 to 0xF5)
//
pub const OPL3_REG_WAVE_SELECT: c_uint = 0xe0;
pub const OPL3_WAVE_SELECT_MASK: c_uint = 0x07;
//
// Offsets to the register banks for voices. Just add to the
// voice number to get the register number.
//
// F-Number low bits (0xA0 to 0xA8).
//
pub const OPL3_REG_FNUM_LOW: c_uint = 0xa0;
//
// F-number high bits / Key on / Block (octave) (0xB0 to 0xB8)
//
pub const OPL3_REG_KEYON_BLOCK: c_uint = 0xb0;
pub const OPL3_KEYON_BIT: c_uint = 0x20;
pub const OPL3_BLOCKNUM_MASK: c_uint = 0x1c;
pub const OPL3_FNUM_HIGH_MASK: c_uint = 0x03;
//
// Feedback / Connection (0xc0 to 0xc8)
//
// These registers have two new bits when the OPL-3 mode
// is selected. These bits controls connecting the voice
// to the stereo channels. For 4 OP voices this bit is
// defined in the second half of the voice (add 3 to the
// register offset).
//
// For 4 OP voices the connection bit is used in the
// both halves (gives 4 ways to connect the operators).
//
pub const OPL3_REG_FEEDBACK_CONNECTION: c_uint = 0xc0;
pub const OPL3_FEEDBACK_MASK: c_uint = 0x0e	/* Valid just for 1st OP of a voice */;
pub const OPL3_CONNECTION_BIT: c_uint = 0x01;
//
// In the 4 OP mode there is four possible configurations how the
// operators can be connected together (in 2 OP modes there is just
// AM or FM). The 4 OP connection mode is defined by the rightmost
// bit of the FEEDBACK_CONNECTION (0xC0-0xC8) on the both halves.
//
// First half      Second half     Mode
//
// +---+
// v   |
// 0               0               >+-1-+--2--3--4-->
//
// +---+
// |   |
// 0               1               >+-1-+--2-+
// |->
// >--3----4-+
//
// +---+
// |   |
// 1               0               >+-1-+-----+
// |->
// >--2--3--4-+
//
// +---+
// |   |
// 1               1               >+-1-+--+
// |
// >--2--3-+->
// |
// >--4----+
//
pub const OPL3_STEREO_BITS: c_uint = 0x30	/* OPL-3 only */;
pub const OPL3_VOICE_TO_LEFT: c_uint = 0x10;
pub const OPL3_VOICE_TO_RIGHT: c_uint = 0x20;
//
pub const OPL3_LEFT: c_uint = 0x0000;
pub const OPL3_RIGHT: c_uint = 0x0100;
pub const OPL3_HW_AUTO: c_uint = 0x0000;
pub const OPL3_HW_OPL2: c_uint = 0x0200;
pub const OPL3_HW_OPL3: c_uint = 0x0300;
pub const OPL3_HW_OPL3_SV: c_uint = 0x0301	/* S3 SonicVibes */;
pub const OPL3_HW_OPL3_CS: c_uint = 0x0302	/* CS4232/CS4236+ */;
pub const OPL3_HW_OPL3_FM801: c_uint = 0x0303	/* FM801 */;
pub const OPL3_HW_OPL3_CS4281: c_uint = 0x0304	/* CS4281 */;
pub const OPL3_HW_OPL4: c_uint = 0x0400	/* YMF278B/YMF295 */;
pub const OPL3_HW_OPL4_ML: c_uint = 0x0401	/* YMF704/YMF721 */;
pub const OPL3_HW_MASK: c_uint = 0xff00;
pub const MAX_OPL2_VOICES: c_int = 9;
pub const MAX_OPL3_VOICES: c_int = 18;
//
// Instrument record, aka "Patch"
//
// FM operator
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm_operator {
    pub am_vib: c_uchar,
    pub ksl_level: c_uchar,
    pub attack_decay: c_uchar,
    pub sustain_release: c_uchar,
    pub wave_select: c_uchar,
    pub __packed: },
// Instrument data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm_instrument {
    pub op: [fm_operator; 4],
    pub feedback_connection: [c_uchar; 2],
    pub echo_delay: c_uchar,
    pub echo_atten: c_uchar,
    pub chorus_spread: c_uchar,
    pub trnsps: c_uchar,
    pub fix_dur: c_uchar,
    pub modes: c_uchar,
    pub fix_key: c_uchar,
}

// type
pub const FM_PATCH_OPL2: c_uint = 0x01		/* OPL2 2 operators FM instrument */;
pub const FM_PATCH_OPL3: c_uint = 0x02		/* OPL3 4 operators FM instrument */;
// Instrument record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm_patch {
    pub prog: c_uchar,
    pub bank: c_uchar,
    pub type: c_uchar,
    pub inst: fm_instrument,
    pub name: [c_char; 24],
    pub next: *mut fm_patch,
}

//
// A structure to keep track of each hardware voice
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_opl3_voice {
    pub /: *mut *mut int state; / status,

    pub /: *mut *mut unsigned int time; / An allocation time,
    pub /: *mut *mut unsigned char note; / Note currently assigned to this voice,
    pub /: *mut *mut unsigned long note_off; / note-off time,
    pub /: *mut *mut int note_off_check; / check note-off time,
    pub /: *mut *mut unsigned char keyon_reg; / KON register shadow,
    pub /: *mut *mut *mut snd_midi_channel chan; / Midi channel for this note,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_opl3 {
    pub l_port: c_ulong,
    pub r_port: c_ulong,
    pub res_l_port: *mut resource,
    pub res_r_port: *mut resource,
    pub hardware: c_ushort,
// hardware access
    pub val): *mut *mut *mut void (command) (struct snd_opl3  opl3, unsigned short cmd, unsigned char,
    pub timer_enable: c_ushort,
    pub /: *mut *mut int seq_dev_num; / sequencer device number,
    pub timer1: *mut snd_timer,
    pub timer2: *mut snd_timer,
    pub timer_lock: spinlock_t,
    pub private_data: *mut c_void,
    pub ): *mut *mut void (private_free)(struct snd_opl3,
    pub hwdep: *mut snd_hwdep,
    pub reg_lock: spinlock_t,
    pub /: *mut *mut *mut snd_card card; / The card that this belongs to,
    pub /: *mut *mut unsigned char fm_mode; / OPL mode, see SNDRV_DM_FM_MODE_XXX,
    pub /: *mut *mut unsigned char rhythm; / percussion mode flag,
    pub /: *mut *mut unsigned char max_voices; / max number of voices,

    pub /: *mut *mut int synth_mode; / synth mode,
    pub seq_client: c_int,
    pub /: *mut *mut *mut snd_seq_device seq_dev; / sequencer device,
    pub chset: *mut *mut snd_midi_channel_set,

    pub /: *mut *mut *mut snd_seq_device oss_seq_dev; / OSS sequencer device,
    pub oss_chset: *mut *mut snd_midi_channel_set,

pub const OPL3_PATCH_HASH_SIZE: c_int = 32;
    pub patch_table: [*mut fm_patch; OPL3_PATCH_HASH_SIZE],
    pub /: *mut *mut snd_opl3_voice voices[MAX_OPL3_VOICES]; / Voices (OPL3 'channel'),
    pub /: *mut *mut int use_time; / allocation counter,
    pub /: *mut *mut unsigned short connection_reg; / connection reg shadow,
    pub /: *mut *mut unsigned char drum_reg; / percussion reg shadow,
    pub /: *mut *mut spinlock_t voice_lock; / Lock for voice access,
    pub /: *mut *mut timer_list tlist; / timer for note-offs and effects,
    pub /: *mut *mut int sys_timer_status; / system timer run status,
    pub /: *mut *mut spinlock_t sys_timer_lock; / Lock for system timer access,

}

// opl3.c
extern "C" {
    pub fn snd_opl3_interrupt(hw: *mut *mut snd_hwdep);
}
extern "C" {
    pub fn snd_opl3_init(opl3: *mut snd_opl3) -> c_int;
}
extern "C" {
    pub fn snd_opl3_timer_new(opl3: *mut *mut snd_opl3, timer1_dev: c_int, timer2_dev: c_int) -> c_int;
}
// opl3_synth
extern "C" {
    pub fn snd_opl3_open(hw: *mut *mut snd_hwdep, file: *mut file) -> c_int;
}
extern "C" {
    pub fn snd_opl3_release(hw: *mut *mut snd_hwdep, file: *mut file) -> c_int;
}
extern "C" {
    pub fn snd_opl3_reset(opl3: *mut *mut snd_opl3);
}

extern "C" {
    pub fn snd_opl3_clear_patches(opl3: *mut snd_opl3);
}

