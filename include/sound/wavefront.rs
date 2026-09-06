//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wavefront.h
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
// Driver for Turtle Beach Wavefront cards (Maui,Tropez,Tropez+)
//
// Copyright (c) by Paul Barton-Davis <pbd@op.net>
//

pub const NUM_MIDIKEYS: c_int = 128;

pub const NUM_MIDICHANNELS: c_int = 16;

// Pseudo-commands not part of the WaveFront command set.
//
pub const WFC_DEBUG_DRIVER: c_int = 0;
pub const WFC_FX_IOCTL: c_int = 1;
pub const WFC_PATCH_STATUS: c_int = 2;
pub const WFC_PROGRAM_STATUS: c_int = 3;
pub const WFC_SAMPLE_STATUS: c_int = 4;
pub const WFC_DISABLE_INTERRUPTS: c_int = 5;
pub const WFC_ENABLE_INTERRUPTS: c_int = 6;
pub const WFC_INTERRUPT_STATUS: c_int = 7;
pub const WFC_ROMSAMPLES_RDONLY: c_int = 8;
pub const WFC_IDENTIFY_SLOT_TYPE: c_int = 9;
// Wavefront synth commands
//
pub const WFC_DOWNLOAD_SAMPLE: c_uint = 0x80;
pub const WFC_DOWNLOAD_BLOCK: c_uint = 0x81;
pub const WFC_DOWNLOAD_MULTISAMPLE: c_uint = 0x82;
pub const WFC_DOWNLOAD_SAMPLE_ALIAS: c_uint = 0x83;
pub const WFC_DELETE_SAMPLE: c_uint = 0x84;
pub const WFC_REPORT_FREE_MEMORY: c_uint = 0x85;
pub const WFC_DOWNLOAD_PATCH: c_uint = 0x86;
pub const WFC_DOWNLOAD_PROGRAM: c_uint = 0x87;
pub const WFC_SET_SYNTHVOL: c_uint = 0x89;
pub const WFC_SET_NVOICES: c_uint = 0x8B;
pub const WFC_DOWNLOAD_DRUM: c_uint = 0x90;
pub const WFC_GET_SYNTHVOL: c_uint = 0x92;
pub const WFC_GET_NVOICES: c_uint = 0x94;
pub const WFC_DISABLE_CHANNEL: c_uint = 0x9A;
pub const WFC_ENABLE_CHANNEL: c_uint = 0x9B;
pub const WFC_MISYNTH_OFF: c_uint = 0x9D;
pub const WFC_MISYNTH_ON: c_uint = 0x9E;
pub const WFC_FIRMWARE_VERSION: c_uint = 0x9F;
pub const WFC_GET_NSAMPLES: c_uint = 0xA0;
pub const WFC_DISABLE_DRUM_PROGRAM: c_uint = 0xA2;
pub const WFC_UPLOAD_PATCH: c_uint = 0xA3;
pub const WFC_UPLOAD_PROGRAM: c_uint = 0xA4;
pub const WFC_SET_TUNING: c_uint = 0xA6;
pub const WFC_GET_TUNING: c_uint = 0xA7;
pub const WFC_VMIDI_ON: c_uint = 0xA8;
pub const WFC_VMIDI_OFF: c_uint = 0xA9;
pub const WFC_MIDI_STATUS: c_uint = 0xAA;
pub const WFC_GET_CHANNEL_STATUS: c_uint = 0xAB;
pub const WFC_DOWNLOAD_SAMPLE_HEADER: c_uint = 0xAC;
pub const WFC_UPLOAD_SAMPLE_HEADER: c_uint = 0xAD;
pub const WFC_UPLOAD_MULTISAMPLE: c_uint = 0xAE;
pub const WFC_UPLOAD_SAMPLE_ALIAS: c_uint = 0xAF;
pub const WFC_IDENTIFY_SAMPLE_TYPE: c_uint = 0xB0;
pub const WFC_DOWNLOAD_EDRUM_PROGRAM: c_uint = 0xB1;
pub const WFC_UPLOAD_EDRUM_PROGRAM: c_uint = 0xB2;
pub const WFC_SET_EDRUM_CHANNEL: c_uint = 0xB3;
pub const WFC_INSTOUT_LEVELS: c_uint = 0xB4;
pub const WFC_PEAKOUT_LEVELS: c_uint = 0xB5;
pub const WFC_REPORT_CHANNEL_PROGRAMS: c_uint = 0xB6;
pub const WFC_HARDWARE_VERSION: c_uint = 0xCF;
pub const WFC_UPLOAD_SAMPLE_PARAMS: c_uint = 0xD7;
pub const WFC_DOWNLOAD_OS: c_uint = 0xF1;
pub const WFC_NOOP: c_uint = 0xFF;
pub const WF_MAX_SAMPLE: c_int = 512;
pub const WF_MAX_PATCH: c_int = 256;
pub const WF_MAX_PROGRAM: c_int = 128;

// # of bytes we send to the board when sending it various kinds of
//
pub const WF_PROGRAM_BYTES: c_int = 32;
pub const WF_PATCH_BYTES: c_int = 132;
pub const WF_SAMPLE_BYTES: c_int = 27;
pub const WF_SAMPLE_HDR_BYTES: c_int = 25;
pub const WF_ALIAS_BYTES: c_int = 25;
pub const WF_DRUM_BYTES: c_int = 9;

pub const WF_ACK: c_uint = 0x80;
pub const WF_DMA_ACK: c_uint = 0x81;
// OR-values for MIDI status bits
pub const WF_MIDI_VIRTUAL_ENABLED: c_uint = 0x1;
pub const WF_MIDI_VIRTUAL_IS_EXTERNAL: c_uint = 0x2;
pub const WF_MIDI_IN_TO_SYNTH_DISABLED: c_uint = 0x4;
// slot indexes for struct address_info: makes code a little more mnemonic
pub const WF_SYNTH_SLOT: c_int = 0;
pub const WF_INTERNAL_MIDI_SLOT: c_int = 1;
pub const WF_EXTERNAL_MIDI_SLOT: c_int = 2;
// Magic MIDI bytes used to switch I/O streams on the ICS2115 MPU401
//
pub const WF_EXTERNAL_SWITCH: c_uint = 0xFD;
pub const WF_INTERNAL_SWITCH: c_uint = 0xF9;
// Debugging flags
pub const WF_DEBUG_CMD: c_uint = 0x1;
pub const WF_DEBUG_DATA: c_uint = 0x2;
pub const WF_DEBUG_LOAD_PATCH: c_uint = 0x4;
pub const WF_DEBUG_IO: c_uint = 0x8;
// WavePatch file format stuff

pub const WF_NUM_LAYERS: c_int = 4;
pub const WF_NAME_LENGTH: c_int = 32;
pub const WF_SOURCE_LENGTH: c_int = 260;

pub type wavefront_envelope = wf_envelope;
pub type wavefront_lfo = wf_lfo;
pub type wavefront_patch = wf_patch;
pub type wavefront_layer = wf_layer;
pub type wavefront_program = wf_program;
pub type wavefront_sample_offset = wf_sample_offset;
// Sample slot types
pub const WF_ST_SAMPLE: c_int = 0;
pub const WF_ST_MULTISAMPLE: c_int = 1;
pub const WF_ST_ALIAS: c_int = 2;
pub const WF_ST_EMPTY: c_int = 3;
// pseudo's
pub const WF_ST_DRUM: c_int = 4;
pub const WF_ST_PROGRAM: c_int = 5;
pub const WF_ST_PATCH: c_int = 6;
pub const WF_ST_SAMPLEHDR: c_int = 7;
pub const WF_ST_MASK: c_uint = 0xf;
// Flags for slot status. These occupy the upper bits of the same byte
//
pub const WF_SLOT_USED: c_uint = 0x80   /* XXX don't rely on this being accurate */;
pub const WF_SLOT_FILLED: c_uint = 0x40;
pub const WF_SLOT_ROM: c_uint = 0x20;
pub const WF_SLOT_MASK: c_uint = 0xf0;
// channel constants
pub const WF_CH_MONO: c_int = 0;
pub const WF_CH_LEFT: c_int = 1;
pub const WF_CH_RIGHT: c_int = 2;
// Sample formats
pub const LINEAR_16BIT: c_int = 0;
pub const WHITE_NOISE: c_int = 1;
pub const LINEAR_8BIT: c_int = 2;
pub const MULAW_8BIT: c_int = 3;

//

// This structure is meant to be padded only to 16 bits on their
//
// How to get MIDI channel status from the data returned by
//

// Hannu Solvainen hoped that his "patch_info" struct in soundcard.h
//
// the first two fields are used by the OSS "patch loading" interface
//
pub const WAVEFRONT_FIND_FREE_SAMPLE_SLOT: c_int = 999;
//
// The maximum number of bytes we will ever move to or from user space
//

//
pub const WFCTL_WFCMD: c_uint = 0x1;
pub const WFCTL_LOAD_SPP: c_uint = 0x2;
// Modulator table
pub const WF_MOD_LFO1: c_int = 0;
pub const WF_MOD_LFO2: c_int = 1;
pub const WF_MOD_ENV1: c_int = 2;
pub const WF_MOD_ENV2: c_int = 3;
pub const WF_MOD_KEYBOARD: c_int = 4;
pub const WF_MOD_LOGKEY: c_int = 5;
pub const WF_MOD_VELOCITY: c_int = 6;
pub const WF_MOD_LOGVEL: c_int = 7;
pub const WF_MOD_RANDOM: c_int = 8;
pub const WF_MOD_PRESSURE: c_int = 9;
pub const WF_MOD_MOD_WHEEL: c_int = 10;

pub const WF_MOD_BREATH: c_int = 11;

pub const WF_MOD_FOOT: c_int = 12;

pub const WF_MOD_VOLUME: c_int = 13;

pub const WF_MOD_PAN: c_int = 14;

pub const WF_MOD_EXPR: c_int = 15;

// FX-related material
// support for each of these will be forthcoming once I or someone
//
pub const WFFX_SETOUTGAIN: c_int = 0;
pub const WFFX_SETSTEREOOUTGAIN: c_int = 1;
pub const WFFX_SETREVERBIN1GAIN: c_int = 2;
pub const WFFX_SETREVERBIN2GAIN: c_int = 3;
pub const WFFX_SETREVERBIN3GAIN: c_int = 4;
pub const WFFX_SETCHORUSINPORT: c_int = 5;
pub const WFFX_SETREVERBIN1PORT: c_int = 6;
pub const WFFX_SETREVERBIN2PORT: c_int = 7;
pub const WFFX_SETREVERBIN3PORT: c_int = 8;
pub const WFFX_SETEFFECTPORT: c_int = 9;
pub const WFFX_SETAUXPORT: c_int = 10;
pub const WFFX_SETREVERBTYPE: c_int = 11;
pub const WFFX_SETREVERBDELAY: c_int = 12;
pub const WFFX_SETCHORUSLFO: c_int = 13;
pub const WFFX_SETCHORUSPMD: c_int = 14;
pub const WFFX_SETCHORUSAMD: c_int = 15;
pub const WFFX_SETEFFECT: c_int = 16;
pub const WFFX_SETBASEALL: c_int = 17;
pub const WFFX_SETREVERBALL: c_int = 18;
pub const WFFX_SETCHORUSALL: c_int = 20;
pub const WFFX_SETREVERBDEF: c_int = 22;
pub const WFFX_SETCHORUSDEF: c_int = 23;
pub const WFFX_DELAYSETINGAIN: c_int = 24;
pub const WFFX_DELAYSETFBGAIN: c_int = 25;
pub const WFFX_DELAYSETFBLPF: c_int = 26;
pub const WFFX_DELAYSETGAIN: c_int = 27;
pub const WFFX_DELAYSETTIME: c_int = 28;
pub const WFFX_DELAYSETFBTIME: c_int = 29;
pub const WFFX_DELAYSETALL: c_int = 30;
pub const WFFX_DELAYSETDEF: c_int = 32;
pub const WFFX_SDELAYSETINGAIN: c_int = 33;
pub const WFFX_SDELAYSETFBGAIN: c_int = 34;
pub const WFFX_SDELAYSETFBLPF: c_int = 35;
pub const WFFX_SDELAYSETGAIN: c_int = 36;
pub const WFFX_SDELAYSETTIME: c_int = 37;
pub const WFFX_SDELAYSETFBTIME: c_int = 38;
pub const WFFX_SDELAYSETALL: c_int = 39;
pub const WFFX_SDELAYSETDEF: c_int = 41;
pub const WFFX_DEQSETINGAIN: c_int = 42;
pub const WFFX_DEQSETFILTER: c_int = 43;
pub const WFFX_DEQSETALL: c_int = 44;
pub const WFFX_DEQSETDEF: c_int = 46;
pub const WFFX_MUTE: c_int = 47;
pub const WFFX_FLANGESETBALANCE: c_int = 48;
pub const WFFX_FLANGESETDELAY: c_int = 49;
pub const WFFX_FLANGESETDWFFX_TH: c_int = 50;
pub const WFFX_FLANGESETFBGAIN: c_int = 51;
pub const WFFX_FLANGESETINGAIN: c_int = 52;
pub const WFFX_FLANGESETLFO: c_int = 53;
pub const WFFX_FLANGESETALL: c_int = 54;
pub const WFFX_FLANGESETDEF: c_int = 56;
pub const WFFX_PITCHSETSHIFT: c_int = 57;
pub const WFFX_PITCHSETBALANCE: c_int = 58;
pub const WFFX_PITCHSETALL: c_int = 59;
pub const WFFX_PITCHSETDEF: c_int = 61;
pub const WFFX_SRSSETINGAIN: c_int = 62;
pub const WFFX_SRSSETSPACE: c_int = 63;
pub const WFFX_SRSSETCENTER: c_int = 64;
pub const WFFX_SRSSETGAIN: c_int = 65;
pub const WFFX_SRSSETMODE: c_int = 66;
pub const WFFX_SRSSETDEF: c_int = 68;
// Allow direct user-space control over FX memory/coefficient data.
//
pub const WFFX_MEMSET: c_int = 69;
