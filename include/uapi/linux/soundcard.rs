//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/soundcard.h
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
// Copyright by Hannu Savolainen 1993-1997
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met: 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer. 2.
// Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//
// OSS interface version. With versions earlier than 3.6 this value is
// an integer with value less than 361. In versions 3.6 and later
// it's a six digit hexadecimal value. For example value
// of 0x030600 represents OSS version 3.6.0.
// Use ioctl(fd, OSS_GETVERSION, &int) to get the version number of
// the currently active driver.
//
pub const SOUND_VERSION: c_uint = 0x030802;
// Macro flag: #define OPEN_SOUND_SYSTEM
// In Linux we need to be prepared for cross compiling

// Endian macros.

//
// Supported card ID numbers (Should be somewhere else?)
//
pub const SNDCARD_ADLIB: c_int = 1;
pub const SNDCARD_SB: c_int = 2;
pub const SNDCARD_PAS: c_int = 3;
pub const SNDCARD_GUS: c_int = 4;
pub const SNDCARD_MPU401: c_int = 5;
pub const SNDCARD_SB16: c_int = 6;
pub const SNDCARD_SB16MIDI: c_int = 7;
pub const SNDCARD_UART6850: c_int = 8;
pub const SNDCARD_GUS16: c_int = 9;
pub const SNDCARD_MSS: c_int = 10;
pub const SNDCARD_PSS: c_int = 11;
pub const SNDCARD_SSCAPE: c_int = 12;
pub const SNDCARD_PSS_MPU: c_int = 13;
pub const SNDCARD_PSS_MSS: c_int = 14;
pub const SNDCARD_SSCAPE_MSS: c_int = 15;
pub const SNDCARD_TRXPRO: c_int = 16;
pub const SNDCARD_TRXPRO_SB: c_int = 17;
pub const SNDCARD_TRXPRO_MPU: c_int = 18;
pub const SNDCARD_MAD16: c_int = 19;
pub const SNDCARD_MAD16_MPU: c_int = 20;
pub const SNDCARD_CS4232: c_int = 21;
pub const SNDCARD_CS4232_MPU: c_int = 22;
pub const SNDCARD_MAUI: c_int = 23;
pub const SNDCARD_PSEUDO_MSS: c_int = 24;
pub const SNDCARD_GUSPNP: c_int = 25;
pub const SNDCARD_UART401: c_int = 26;
// Sound card numbers 27 to N are reserved. Don't add more numbers here.
//
// IOCTL Commands for /dev/sequencer
//

// Use already defined ioctl defines if they exist (except with Sun or Sparc)

// Ioctl's have the command encoded in the lower word,
// and the size of any in or out parameters in the upper
// word.  The high 2 bits of the upper word are used
// to encode the in/out status of the parameter; for now
// we restrict parameters to at most 8191 bytes.
//
// #define	SIOCTYPE		(0xff<<8)
pub const SIOCPARM_MASK: c_uint = 0x1fff		/* parameters must be < 8192 bytes */;
pub const SIOC_VOID: c_uint = 0x00000000	/* no parameters */;
pub const SIOC_OUT: c_uint = 0x20000000	/* copy out parameters */;
pub const SIOC_IN: c_uint = 0x40000000	/* copy in parameters */;

// the 0x20000000 is so we can distinguish new ioctl's from old

// this should be _SIORW, but stdio got there first

//
// Some big endian/little endian handling macros
//

//
// Sample loading mechanism for internal synthesizers (/dev/sequencer)
// The following patch_info structure has been designed to support
// Gravis UltraSound. It tries to be universal format for uploading
// sample based patches but is probably too limited.
//
// (PBD) As Hannu guessed, the GUS structure is too limited for
// the WaveFront, but this is the right place for a constant definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct patch_info {
    pub /: *mut *mut unsigned short key; / Use WAVE_PATCH here,

    pub /: *mut *mut short device_no; / Synthesizer number,
    pub /: *mut *mut short instr_no; / Midi pgm#,
    pub mode: c_uint,
//
// The least significant byte has the same format than the GUS .PAT
// files
//
pub const WAVE_16_BITS: c_uint = 0x01	/* bit 0 = 8 or 16 bit wave data. */;
pub const WAVE_UNSIGNED: c_uint = 0x02	/* bit 1 = Signed - Unsigned data. */;
pub const WAVE_LOOPING: c_uint = 0x04	/* bit 2 = looping enabled-1. */;
pub const WAVE_BIDIR_LOOP: c_uint = 0x08	/* bit 3 = Set is bidirectional looping. */;
pub const WAVE_LOOP_BACK: c_uint = 0x10	/* bit 4 = Set is looping backward. */;
pub const WAVE_SUSTAIN_ON: c_uint = 0x20	/* bit 5 = Turn sustaining on. (Env. pts. 3)*/;
pub const WAVE_ENVELOPES: c_uint = 0x40	/* bit 6 = Enable envelopes - 1 */;
pub const WAVE_FAST_RELEASE: c_uint = 0x80	/* bit 7 = Shut off immediately after note off */;
// (use the env_rate/env_offs fields).
// Linux specific bits
pub const WAVE_VIBRATO: c_uint = 0x00010000	/* The vibrato info is valid */;
pub const WAVE_TREMOLO: c_uint = 0x00020000	/* The tremolo info is valid */;
pub const WAVE_SCALE: c_uint = 0x00040000	/* The scaling info is valid */;
pub const WAVE_FRACTIONS: c_uint = 0x00080000	/* Fraction information is valid */;
// Reserved bits
pub const WAVE_ROM: c_uint = 0x40000000	/* For future use */;
pub const WAVE_MULAW: c_uint = 0x20000000	/* For future use */;
// Other bits must be zeroed
    pub /: *mut *mut int len; / Size of the wave data in bytes,
    pub /: *mut *mut int loop_start, loop_end; / Byte offsets from the beginning,
//
// The base_freq and base_note fields are used when computing the
// playback speed for a note. The base_note defines the tone frequency
// which is heard if the sample is played using the base_freq as the
// playback speed.
//
// The low_note and high_note fields define the minimum and maximum note
// frequencies for which this sample is valid. It is possible to define
// more than one samples for an instrument number at the same time. The
// low_note and high_note fields are used to select the most suitable one.
//
// The fields base_note, high_note and low_note should contain
// the note frequency multiplied by 1000. For example value for the
// middle A is 440*1000.
//
    pub base_freq: c_uint,
    pub base_note: c_uint,
    pub high_note: c_uint,
    pub low_note: c_uint,
    pub /: *mut *mut int panning; / -128=left, 127=right,
    pub detuning: c_int,
// New fields introduced in version 1.99.5
// Envelope. Enabled by mode bit WAVE_ENVELOPES
    pub /: *mut *mut unsigned char env_rate[ 6 ]; / GUS HW ramping rate,
    pub /: *mut *mut unsigned char env_offset[ 6 ]; / 255 == 100%,
//
// The tremolo, vibrato and scale info are not supported yet.
// Enable by setting the mode bits WAVE_TREMOLO, WAVE_VIBRATO or
// WAVE_SCALE
//
    pub tremolo_sweep: c_uchar,
    pub tremolo_rate: c_uchar,
    pub tremolo_depth: c_uchar,
    pub vibrato_sweep: c_uchar,
    pub vibrato_rate: c_uchar,
    pub vibrato_depth: c_uchar,
    pub scale_frequency: c_int,
    pub /: *mut *mut unsigned int scale_factor; / from 0 to 2048 or 0 to 2,
    pub volume: c_int,
    pub fractions: c_int,
    pub reserved1: c_int,
    pub spare: [c_int; 2],
    pub /: *mut *mut char data[1]; / The waveform data starts here,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysex_info {
    pub /: *mut *mut short key; / Use SYSEX_PATCH or MAUI_PATCH here,

    pub /: *mut *mut short device_no; / Synthesizer number,
    pub /: *mut *mut int len; / Size of the sysex data in bytes,
    pub /: *mut *mut unsigned char data[1]; / Sysex data starts here,
}

//
// /dev/sequencer input events.
//
// The data written to the /dev/sequencer is a stream of events. Events
// are records of 4 or 8 bytes. The first byte defines the size.
// Any number of events can be written with a write call. There
// is a set of macros for sending these events. Use these macros if you
// want to maximize portability of your program.
//
// Events SEQ_WAIT, SEQ_MIDIPUTC and SEQ_ECHO. Are also input events.
// (All input events are currently 4 bytes long. Be prepared to support
// 8 byte events also. If you receive any event having first byte >= 128,
// it's a 8 byte event.
//
// The events are documented at the end of this file.
//
// Normal events (4 bytes)
// There is also a 8 byte version of most of the 4 byte events. The
// 8 byte one is recommended.
//
pub const SEQ_NOTEOFF: c_int = 0;

pub const SEQ_NOTEON: c_int = 1;

pub const SEQ_PGMCHANGE: c_int = 3;

pub const SEQ_MIDIPUTC: c_int = 5;

pub const SEQ_AFTERTOUCH: c_int = 9;
pub const SEQ_CONTROLLER: c_int = 10;
//
// Midi controller numbers
//
// Controllers 0 to 31 (0x00 to 0x1f) and
// 32 to 63 (0x20 to 0x3f) are continuous
// controllers.
// In the MIDI 1.0 these controllers are sent using
// two messages. Controller numbers 0 to 31 are used
// to send the MSB and the controller numbers 32 to 63
// are for the LSB. Note that just 7 bits are used in MIDI bytes.
//
pub const CTL_BANK_SELECT: c_uint = 0x00;
pub const CTL_MODWHEEL: c_uint = 0x01;
pub const CTL_BREATH: c_uint = 0x02;
// undefined		0x03
pub const CTL_FOOT: c_uint = 0x04;
pub const CTL_PORTAMENTO_TIME: c_uint = 0x05;
pub const CTL_DATA_ENTRY: c_uint = 0x06;
pub const CTL_MAIN_VOLUME: c_uint = 0x07;
pub const CTL_BALANCE: c_uint = 0x08;
// undefined		0x09
pub const CTL_PAN: c_uint = 0x0a;
pub const CTL_EXPRESSION: c_uint = 0x0b;
// undefined		0x0c
// undefined		0x0d
// undefined		0x0e
// undefined		0x0f
pub const CTL_GENERAL_PURPOSE1: c_uint = 0x10;
pub const CTL_GENERAL_PURPOSE2: c_uint = 0x11;
pub const CTL_GENERAL_PURPOSE3: c_uint = 0x12;
pub const CTL_GENERAL_PURPOSE4: c_uint = 0x13;
// undefined		0x14 - 0x1f
// undefined		0x20
// The controller numbers 0x21 to 0x3f are reserved for the
// least significant bytes of the controllers 0x00 to 0x1f.
// These controllers are not recognised by the driver.
// Controllers 64 to 69 (0x40 to 0x45) are on/off switches.
// 0=OFF and 127=ON (intermediate values are possible)
pub const CTL_DAMPER_PEDAL: c_uint = 0x40;
pub const CTL_SUSTAIN: c_uint = 0x40	/* Alias */;
pub const CTL_HOLD: c_uint = 0x40	/* Alias */;
pub const CTL_PORTAMENTO: c_uint = 0x41;
pub const CTL_SOSTENUTO: c_uint = 0x42;
pub const CTL_SOFT_PEDAL: c_uint = 0x43;
// undefined		0x44
pub const CTL_HOLD2: c_uint = 0x45;
// undefined		0x46 - 0x4f
pub const CTL_GENERAL_PURPOSE5: c_uint = 0x50;
pub const CTL_GENERAL_PURPOSE6: c_uint = 0x51;
pub const CTL_GENERAL_PURPOSE7: c_uint = 0x52;
pub const CTL_GENERAL_PURPOSE8: c_uint = 0x53;
// undefined		0x54 - 0x5a
pub const CTL_EXT_EFF_DEPTH: c_uint = 0x5b;
pub const CTL_TREMOLO_DEPTH: c_uint = 0x5c;
pub const CTL_CHORUS_DEPTH: c_uint = 0x5d;
pub const CTL_DETUNE_DEPTH: c_uint = 0x5e;
pub const CTL_CELESTE_DEPTH: c_uint = 0x5e	/* Alias for the above one */;
pub const CTL_PHASER_DEPTH: c_uint = 0x5f;
pub const CTL_DATA_INCREMENT: c_uint = 0x60;
pub const CTL_DATA_DECREMENT: c_uint = 0x61;
pub const CTL_NONREG_PARM_NUM_LSB: c_uint = 0x62;
pub const CTL_NONREG_PARM_NUM_MSB: c_uint = 0x63;
pub const CTL_REGIST_PARM_NUM_LSB: c_uint = 0x64;
pub const CTL_REGIST_PARM_NUM_MSB: c_uint = 0x65;
// undefined		0x66 - 0x78
// reserved		0x79 - 0x7f
// Pseudo controllers (not midi compatible)
pub const CTRL_PITCH_BENDER: c_int = 255;
pub const CTRL_PITCH_BENDER_RANGE: c_int = 254;

pub const SEQ_BALANCE: c_int = 11;
pub const SEQ_VOLMODE: c_int = 12;
//
// Volume mode decides how volumes are used
//
pub const VOL_METHOD_ADAGIO: c_int = 1;
pub const VOL_METHOD_LINEAR: c_int = 2;
//
// Note! SEQ_WAIT, SEQ_MIDIPUTC and SEQ_ECHO are used also as
// input events.
//
// Event codes 0xf0 to 0xfc are reserved for future extensions.
//
pub const SEQ_FULLSIZE: c_uint = 0xfd	/* Long events */;
//
// SEQ_FULLSIZE events are used for loading patches/samples to the
// synthesizer devices. These events are passed directly to the driver
// of the associated synthesizer device. There is no limit to the size
// of the extended events. These events are not queued but executed
// immediately when the write() is called (execution can take several
// seconds of time).
//
// When a SEQ_FULLSIZE message is written to the device, it must
// be written using exactly one write() call. Other events cannot
// be mixed to the same write.
//
// For FM synths (YM3812/OPL3) use struct sbi_instrument and write it to the
// /dev/sequencer. Don't write other data together with the instrument structure
// Set the key field of the structure to FM_PATCH. The device field is used to
// route the patch to the corresponding device.
//
// For wave table use struct patch_info. Initialize the key field
// to WAVE_PATCH.
//
pub const SEQ_PRIVATE: c_uint = 0xfe	/* Low level HW dependent events (8 bytes) */;
pub const SEQ_EXTENDED: c_uint = 0xff	/* Extended events (8 bytes) OBSOLETE */;
//
// Record for FM patches
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbi_instrument {
    pub /: *mut *mut unsigned short key; / FM_PATCH or OPL3_PATCH,

    pub /: *mut *mut short device; / Synth# (0-4),
    pub /: *mut *mut int channel; / Program# to be initialized,
    pub /: *mut *mut sbi_instr_data operators; / Register settings for operator cells (.SBI format),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_info {
    pub name: [c_char; 30],
    pub /: *mut *mut int device; / 0-N. INITIALIZE BEFORE CALLING,
    pub synth_type: c_int,
pub const SYNTH_TYPE_FM: c_int = 0;
pub const SYNTH_TYPE_SAMPLE: c_int = 1;

    pub synth_subtype: c_int,
pub const FM_TYPE_ADLIB: c_uint = 0x00;
pub const FM_TYPE_OPL3: c_uint = 0x01;
pub const MIDI_TYPE_MPU401: c_uint = 0x401;
pub const SAMPLE_TYPE_BASIC: c_uint = 0x10;

pub const SAMPLE_TYPE_WAVEFRONT: c_uint = 0x11;
    pub /: *mut *mut int perc_mode; / No longer supported,
    pub nr_voices: c_int,
    pub /: *mut *mut int nr_drums; / Obsolete field,
    pub instr_bank_size: c_int,
    pub capabilities: c_uint,
pub const SYNTH_CAP_PERCMODE: c_uint = 0x00000001 /* No longer used */;
pub const SYNTH_CAP_OPL3: c_uint = 0x00000002 /* Set if OPL3 supported */;
pub const SYNTH_CAP_INPUT: c_uint = 0x00000004 /* Input (MIDI) device */;
    pub /: *mut *mut int dummies[19]; / Reserve space,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sound_timer_info {
    pub name: [c_char; 32],
    pub caps: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct midi_info {
    pub name: [c_char; 30],
    pub /: *mut *mut int device; / 0-N. INITIALIZE BEFORE CALLING,
    pub /: *mut *mut unsigned int capabilities; / To be defined later,
    pub dev_type: c_int,
    pub /: *mut *mut int dummies[18]; / Reserve space,
}

//
// ioctl commands for the /dev/midi##
//

//
// IOCTL commands for /dev/dsp and /dev/audio
//

// Audio data formats (Note! U8=8 and S16_LE=16 for compatibility)

//
// Buffer status queries.
//
// Note! 'bytes' could be more than fragments*fragsize

// internal buffers which may
// cause some delays and
// decrease precision of timing

// Sometimes it's a DSP
// but usually not

//
// Application's profile defines the way how playback underrun situations should be handled.
//
// APF_NORMAL (the default) and APF_NETWORK make the driver to cleanup the
// playback buffer whenever an underrun occurs. This consumes some time
// prevents looping the existing buffer.
// APF_CPUINTENS is intended to be set by CPU intensive applications which
// are likely to run out of time occasionally. In this mode the buffer cleanup is
// disabled which saves CPU time but also let's the previous buffer content to
// be played during the "pause" after the underrun.
//

// Some alias names

//
// ioctl calls to be used in communication with coprocessors and
// DSP chips.
//
pub const CPF_NONE: c_uint = 0x0000;
pub const CPF_FIRST: c_uint = 0x0001	/* First block */;
pub const CPF_LAST: c_uint = 0x0002	/* Last block */;

//
// IOCTL commands for /dev/mixer
//
// Mixer devices
//
// There can be up to 20 different analog mixer channels. The
// SOUND_MIXER_NRDEVICES gives the currently supported maximum.
// The SOUND_MIXER_READ_DEVMASK returns a bitmask which tells
// the devices supported by the particular mixer.
//
pub const SOUND_MIXER_NRDEVICES: c_int = 25;
pub const SOUND_MIXER_VOLUME: c_int = 0;
pub const SOUND_MIXER_BASS: c_int = 1;
pub const SOUND_MIXER_TREBLE: c_int = 2;
pub const SOUND_MIXER_SYNTH: c_int = 3;
pub const SOUND_MIXER_PCM: c_int = 4;
pub const SOUND_MIXER_SPEAKER: c_int = 5;
pub const SOUND_MIXER_LINE: c_int = 6;
pub const SOUND_MIXER_MIC: c_int = 7;
pub const SOUND_MIXER_CD: c_int = 8;

pub const SOUND_MIXER_ALTPCM: c_int = 10;

//
// The AD1848 codec and compatibles have three line level inputs
// (line, aux1 and aux2). Since each card manufacturer have assigned
// different meanings to these inputs, it's inpractical to assign
// specific meanings (line, cd, synth etc.) to them.
//

// Some on/off settings (SOUND_SPECIAL_MIN - SOUND_SPECIAL_MAX)
// Not counted to SOUND_MIXER_NRDEVICES, but use the same number space
pub const SOUND_ONOFF_MIN: c_int = 28;
pub const SOUND_ONOFF_MAX: c_int = 30;
// Note!	Number 31 cannot be used since the sign bit is reserved
pub const SOUND_MIXER_NONE: c_int = 31;
//
// The following unsupported macros are no longer functional.
// Use SOUND_MIXER_PRIVATE# macros in future.
//

// Device bitmask identifiers
pub const SOUND_MIXER_RECSRC: c_uint = 0xff	/* Arg contains a bit for each recording source */;
pub const SOUND_MIXER_DEVMASK: c_uint = 0xfe	/* Arg contains a bit for each supported device */;
pub const SOUND_MIXER_RECMASK: c_uint = 0xfd	/* Arg contains a bit for each supported recording source */;
pub const SOUND_MIXER_CAPS: c_uint = 0xfc;

pub const SOUND_MIXER_STEREODEVS: c_uint = 0xfb	/* Mixer channels supporting stereo */;
pub const SOUND_MIXER_OUTSRC: c_uint = 0xfa	/* Arg contains a bit for each input source to output */;
pub const SOUND_MIXER_OUTMASK: c_uint = 0xf9	/* Arg contains a bit for each supported input source to output */;
// Device mask bits

// Obsolete macros

// Obsolete macros

// Obsolete macros

//
// A mechanism for accessing "proprietary" mixer features. This method
// permits passing 128 bytes of arbitrary data between a mixer application
// and the mixer driver. Interpretation of the record is defined by
// the particular mixer driver.
//

//
// Two ioctls for special souncard function
//

//
// The SOUND_MIXER_PRIVATE# commands can be redefined by low level drivers.
// These features can be used when accessing device specific features.
//

//
// SOUND_MIXER_GETLEVELS and SOUND_MIXER_SETLEVELS calls can be used
// for querying current mixer settings from the driver and for loading
// default volume settings _prior_ activating the mixer (loading
// doesn't affect current state of the mixer hardware). These calls
// are for internal use only.
//

//
// An ioctl for identifying the driver version. It will return value
// of the SOUND_VERSION macro used when compiling the driver.
// This call was introduced in OSS version 3.6 and it will not work
// with earlier versions (returns EINVAL).
//

//
// Level 2 event types for /dev/sequencer
//
// The 4 most significant bits of byte 0 specify the class of
// the event:
//
// 0x8X = system level events,
// 0x9X = device/port specific events, event[1] = device/port,
// The last 4 bits give the subtype:
// 0x02	= Channel event (event[3] = chn).
// 0x01	= note event (event[4] = note).
// (0x01 is not used alone but always with bit 0x02).
// event[2] = MIDI message code (0x80=note off etc.)
//
pub const EV_SEQ_LOCAL: c_uint = 0x80;
pub const EV_TIMING: c_uint = 0x81;
pub const EV_CHN_COMMON: c_uint = 0x92;
pub const EV_CHN_VOICE: c_uint = 0x93;
pub const EV_SYSEX: c_uint = 0x94;
//
// Event types 200 to 220 are reserved for application use.
// These numbers will not be used by the driver.
//
// Events for event type EV_CHN_VOICE
//
pub const MIDI_NOTEOFF: c_uint = 0x80;
pub const MIDI_NOTEON: c_uint = 0x90;
pub const MIDI_KEY_PRESSURE: c_uint = 0xA0;
//
// Events for event type EV_CHN_COMMON
//
pub const MIDI_CTL_CHANGE: c_uint = 0xB0;
pub const MIDI_PGM_CHANGE: c_uint = 0xC0;
pub const MIDI_CHN_PRESSURE: c_uint = 0xD0;
pub const MIDI_PITCH_BEND: c_uint = 0xE0;
pub const MIDI_SYSTEM_PREFIX: c_uint = 0xF0;
//
// Timer event types
//

pub const TMR_STOP: c_int = 3;
pub const TMR_START: c_int = 4;
pub const TMR_CONTINUE: c_int = 5;
pub const TMR_TEMPO: c_int = 6;
pub const TMR_ECHO: c_int = 8;

//
// Local event types
//
pub const LOCL_STARTAUDIO: c_int = 1;

//
// Some convenience macros to simplify programming of the
// /dev/sequencer interface
//
// This is a legacy interface for applications written against
// the OSSlib-3.8 style interface. It is no longer possible
// to actually link against OSSlib with this header, but we
// still provide these macros for programs using them.
//
// If you want to use OSSlib, it is recommended that you get
// the GPL version of OSS-4.x and build against that version
// of the header.
//
// We redefine the extern keyword so that usr/include/headers_check.pl
// does not complain about SEQ_USE_EXTBUF.
//

// Sample seqbuf_dump() implementation:
//
// SEQ_DEFINEBUF (2048);	-- Defines a buffer for 2048 bytes
//
// int seqfd;		-- The file descriptor for /dev/sequencer.
//
// void
// seqbuf_dump ()
// {
// if (_seqbufptr)
// if (write (seqfd, _seqbuf, _seqbufptr) == -1)
// {
// perror ("write /dev/sequencer");
// exit (-1);
// }
// _seqbufptr = 0;
// }
//

//
// This variation of the sequencer macros is used just to format one event
// using fixed buffer.
//
// The program using the macro library must define the following macros before
// using this library.
//
// #define _seqbuf 		 name of the buffer (unsigned char[])
// #define _SEQ_ADVBUF(len)	 If the applic needs to know the exact
// size of the event, this macro can be used.
// Otherwise this must be defined as empty.
// #define _seqbufptr		 Define the name of index variable or 0 if
// not required.
//

//
// Midi voice messages
//

//
// Midi channel messages
//

// (short *)&_seqbuf[_seqbufptr+6] = (w14);\
//
// SEQ_SYSEX permits sending of sysex messages. (It may look that it permits
// sending any MIDI bytes but it's absolutely not possible. Trying to do
// so _will_ cause problems with MPU401 intelligent mode).
//
// Sysex messages are sent in blocks of 1 to 6 bytes. Longer messages must be
// sent by calling SEQ_SYSEX() several times (there must be no other events
// between them). First sysex fragment must have 0xf0 in the first byte
// and the last byte (buf[len-1] of the last fragment must be 0xf7. No byte
// between these sysex start and end markers cannot be larger than 0x7f. Also
// lengths of each fragments (except the last one) must be 6.
//
// Breaking the above rules may work with some MIDI ports but is likely to
// cause fatal problems with some other devices (such as MPU401).
//

//
// The following 5 macros are incorrectly implemented and obsolete.
// Use SEQ_BENDER and SEQ_CONTROL (with proper controller) instead.
//

//
// Timing and synchronization macros
//

// (unsigned int *)&_seqbuf[_seqbufptr+4] = (parm); \

//
// Local control events
//

// (unsigned int *)&_seqbuf[_seqbufptr+4] = (parm); \

//
// Events for the level 1 interface only
//

//
// Patch loading.
//

