//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/sfnt_info.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Patch record compatible with AWE driver on OSS
//
// Copyright (C) 1999-2000 Takashi Iwai
//

//
// patch information record
//

// patch interface header: 16 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_patch_info {
    pub /: *mut *mut unsigned short key; / use the key below,

    pub /: *mut *mut short device_no; / synthesizer number,
    pub /: *mut *mut unsigned short sf_id; / file id (should be zero),
    pub /: *mut *mut short optarg; / optional argument,
    pub /: *mut *mut int len; / data length (without this header),
    pub /: *mut *mut short type; / patch operation type,

// 4 is obsolete

// 7 is not used

    pub /: *mut *mut short reserved; / word alignment data,
// the actual patch data begins after this
}

//
// open patch
//
pub const SNDRV_SFNT_PATCH_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_open_parm {
    pub /: *mut *mut unsigned short type; / sample type,
pub const SNDRV_SFNT_PAT_TYPE_MISC: c_int = 0;
pub const SNDRV_SFNT_PAT_TYPE_GUS: c_int = 6;
pub const SNDRV_SFNT_PAT_TYPE_MAP: c_int = 7;
pub const SNDRV_SFNT_PAT_LOCKED: c_uint = 0x100	/* lock the samples */;
pub const SNDRV_SFNT_PAT_SHARED: c_uint = 0x200	/* sample is shared */;
    pub reserved: c_short,
    pub name: [c_char; SNDRV_SFNT_PATCH_NAME_LEN],
}

//
// raw voice information record
//
// wave table envelope & effect parameters to control EMU8000
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_voice_parm {
    pub /: *mut *mut unsigned short moddelay; / modulation delay (0x8000),
    pub /: *mut *mut unsigned short modatkhld; / modulation attack & hold time (0x7f7f),
    pub /: *mut *mut unsigned short moddcysus; / modulation decay & sustain (0x7f7f),
    pub /: *mut *mut unsigned short modrelease; / modulation release time (0x807f),
    pub /: *mut *mut short modkeyhold, modkeydecay; / envelope change per key (not used),
    pub /: *mut *mut unsigned short voldelay; / volume delay (0x8000),
    pub /: *mut *mut unsigned short volatkhld; / volume attack & hold time (0x7f7f),
    pub /: *mut *mut unsigned short voldcysus; / volume decay & sustain (0x7f7f),
    pub /: *mut *mut unsigned short volrelease; / volume release time (0x807f),
    pub /: *mut *mut short volkeyhold, volkeydecay; / envelope change per key (not used),
    pub /: *mut *mut unsigned short lfo1delay; / LFO1 delay (0x8000),
    pub /: *mut *mut unsigned short lfo2delay; / LFO2 delay (0x8000),
    pub /: *mut *mut unsigned short pefe; / modulation pitch & cutoff (0x0000),
    pub /: *mut *mut unsigned short fmmod; / LFO1 pitch & cutoff (0x0000),
    pub /: *mut *mut unsigned short tremfrq; / LFO1 volume & freq (0x0000),
    pub /: *mut *mut unsigned short fm2frq2; / LFO2 pitch & freq (0x0000),
    pub /: *mut *mut unsigned char cutoff; / initial cutoff (0xff),
    pub /: *mut *mut unsigned char filterQ; / initial filter Q [0-15] (0x0),
    pub /: *mut *mut unsigned char chorus; / chorus send (0x00),
    pub /: *mut *mut unsigned char reverb; / reverb send (0x00),
    pub /: *mut *mut unsigned short reserved[4]; / not used,
}

// wave table parameters: 92 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_voice_info {
    pub /: *mut *mut unsigned short sf_id; / file id (should be zero),
    pub /: *mut *mut unsigned short sample; / sample id,
    pub /: *mut *mut int start, end; / sample offset correction,
    pub /: *mut *mut int loopstart, loopend; / loop offset correction,
    pub /: *mut *mut short rate_offset; / sample rate pitch offset,
    pub /: *mut *mut unsigned short mode; / sample mode,
pub const SNDRV_SFNT_MODE_ROMSOUND: c_uint = 0x8000;
pub const SNDRV_SFNT_MODE_STEREO: c_int = 1;
pub const SNDRV_SFNT_MODE_LOOPING: c_int = 2;

pub const SNDRV_SFNT_MODE_INIT_PARM: c_int = 8;
    pub /: *mut *mut short root; / midi root key,
    pub /: *mut *mut short tune; / pitch tuning (in cents),
    pub /: *mut *mut unsigned char low, high; / key note range,
    pub /: *mut *mut unsigned char vellow, velhigh; / velocity range,
    pub /: *mut *mut signed char fixkey, fixvel; / fixed key, velocity,
    pub /: *mut *mut signed char pan, fixpan; / panning, fixed panning,
    pub /: *mut *mut short exclusiveClass; / exclusive class (0 = none),
    pub /: *mut *mut unsigned char amplitude; / sample volume (127 max),
    pub /: *mut *mut unsigned char attenuation; / attenuation (0.375dB),
    pub /: *mut *mut short scaleTuning; / pitch scale tuning(%), normally 100,
    pub /: *mut *mut soundfont_voice_parm parm; / voice envelope parameters,
    pub /: *mut *mut unsigned short sample_mode; / sample mode_flag (set by driver),
}

// instrument info header: 4 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_voice_rec_hdr {
    pub /: *mut *mut unsigned char bank; / midi bank number,
    pub /: *mut *mut unsigned char instr; / midi preset number,
    pub /: *mut *mut char nvoices; / number of voices,
    pub /: *mut *mut char write_mode; / write mode; normally 0,

}

//
// sample wave information
//
// wave table sample header: 32 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_sample_info {
    pub /: *mut *mut unsigned short sf_id; / file id (should be zero),
    pub /: *mut *mut unsigned short sample; / sample id,
    pub /: *mut *mut int start, end; / start & end offset,
    pub /: *mut *mut int loopstart, loopend; / loop start & end offset,
    pub /: *mut *mut int size; / size (0 = ROM),
    pub /: *mut *mut short dummy; / not used,
    pub /: *mut *mut unsigned short mode_flags; / mode flags,

    pub /: *mut *mut unsigned int truesize; / used memory size (set by driver),
}

//
// voice preset mapping (aliasing)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_voice_map {
    pub /: *mut *mut int map_bank, map_instr, map_key; / key = -1 means all keys,
    pub src_key: int src_bank, src_instr,,
}

//
// ioctls for hwdep
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emux_misc_mode {
    pub /: *mut *mut int port; / -1 = all,
    pub mode: c_int,
    pub value: c_int,
    pub /: *mut *mut int value2; / reserved,
}

