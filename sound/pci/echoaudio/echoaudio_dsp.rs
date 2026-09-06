//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/echoaudio/echoaudio_dsp.h
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
// Echogals: Darla20, Gina20, Layla20, and Darla24

pub const NUM_ASIC_TESTS: c_int = 5;

// Echo24: Gina24, Layla24, Mona, Mia, Mia-midi

// 3G: Gina3G, Layla3G

// Macro flag: #define DSP_56361

pub const MIN_MTC_1X_RATE: c_int = 32000;
// Indigo: Indigo, Indigo IO, Indigo DJ

// Macro flag: #define DSP_56361

//
// Max inputs and outputs
//

//
// These are the offsets for the memory-mapped DSP registers; the DSP base
// address is treated as the start of a u32 array.
//
pub const CHI32_CONTROL_REG: c_int = 4;
pub const CHI32_STATUS_REG: c_int = 5;
pub const CHI32_VECTOR_REG: c_int = 6;
pub const CHI32_DATA_REG: c_int = 7;
//
// Interesting bits within the DSP registers
//
pub const CHI32_VECTOR_BUSY: c_uint = 0x00000001;
pub const CHI32_STATUS_REG_HF3: c_uint = 0x00000008;
pub const CHI32_STATUS_REG_HF4: c_uint = 0x00000010;
pub const CHI32_STATUS_REG_HF5: c_uint = 0x00000020;
pub const CHI32_STATUS_HOST_READ_FULL: c_uint = 0x00000004;
pub const CHI32_STATUS_HOST_WRITE_EMPTY: c_uint = 0x00000002;
pub const CHI32_STATUS_IRQ: c_uint = 0x00000040;
//
// DSP commands sent via slave mode; these are sent to the DSP by write_dsp()
//
pub const DSP_FNC_SET_COMMPAGE_ADDR: c_uint = 0x02;
pub const DSP_FNC_LOAD_LAYLA_ASIC: c_uint = 0xa0;
pub const DSP_FNC_LOAD_GINA24_ASIC: c_uint = 0xa0;
pub const DSP_FNC_LOAD_MONA_PCI_CARD_ASIC: c_uint = 0xa0;
pub const DSP_FNC_LOAD_LAYLA24_PCI_CARD_ASIC: c_uint = 0xa0;
pub const DSP_FNC_LOAD_MONA_EXTERNAL_ASIC: c_uint = 0xa1;
pub const DSP_FNC_LOAD_LAYLA24_EXTERNAL_ASIC: c_uint = 0xa1;
pub const DSP_FNC_LOAD_3G_ASIC: c_uint = 0xa0;
//
// Defines to handle the MIDI input state engine; these are used to properly
// extract MIDI time code bytes and their timestamps from the MIDI input stream.
//
pub const MIDI_IN_STATE_NORMAL: c_int = 0;
pub const MIDI_IN_STATE_TS_HIGH: c_int = 1;
pub const MIDI_IN_STATE_TS_LOW: c_int = 2;
pub const MIDI_IN_STATE_F1_DATA: c_int = 3;

// ----------------------------------------------------------------------------
pub const LAYLA24_MAGIC_NUMBER: c_int = 677376000;
pub const LAYLA24_CONTINUOUS_CLOCK: c_uint = 0x000e;
//
// DSP vector commands
//
pub const DSP_VC_RESET: c_uint = 0x80ff;

pub const DSP_VC_ACK_INT: c_uint = 0x8073;
pub const DSP_VC_SET_VMIXER_GAIN: c_uint = 0x0000	/* Not used, only for compile */;
pub const DSP_VC_START_TRANSFER: c_uint = 0x0075	/* Handshke rqd. */;
pub const DSP_VC_METERS_ON: c_uint = 0x0079;
pub const DSP_VC_METERS_OFF: c_uint = 0x007b;
pub const DSP_VC_UPDATE_OUTVOL: c_uint = 0x007d	/* Handshke rqd. */;
pub const DSP_VC_UPDATE_INGAIN: c_uint = 0x007f	/* Handshke rqd. */;
pub const DSP_VC_ADD_AUDIO_BUFFER: c_uint = 0x0081	/* Handshke rqd. */;
pub const DSP_VC_TEST_ASIC: c_uint = 0x00eb;
pub const DSP_VC_UPDATE_CLOCKS: c_uint = 0x00ef	/* Handshke rqd. */;
pub const DSP_VC_SET_LAYLA_SAMPLE_RATE: c_uint = 0x00f1	/* Handshke rqd. */;
pub const DSP_VC_SET_GD_AUDIO_STATE: c_uint = 0x00f1	/* Handshke rqd. */;
pub const DSP_VC_WRITE_CONTROL_REG: c_uint = 0x00f1	/* Handshke rqd. */;
pub const DSP_VC_MIDI_WRITE: c_uint = 0x00f5	/* Handshke rqd. */;
pub const DSP_VC_STOP_TRANSFER: c_uint = 0x00f7	/* Handshke rqd. */;
pub const DSP_VC_UPDATE_FLAGS: c_uint = 0x00fd	/* Handshke rqd. */;
pub const DSP_VC_GO_COMATOSE: c_uint = 0x00f9;

// Vector commands for families that use either the 56301 or 56361
pub const DSP_VC_ACK_INT: c_uint = 0x80F5;
pub const DSP_VC_SET_VMIXER_GAIN: c_uint = 0x00DB	/* Handshke rqd. */;
pub const DSP_VC_START_TRANSFER: c_uint = 0x00DD	/* Handshke rqd. */;
pub const DSP_VC_METERS_ON: c_uint = 0x00EF;
pub const DSP_VC_METERS_OFF: c_uint = 0x00F1;
pub const DSP_VC_UPDATE_OUTVOL: c_uint = 0x00E3	/* Handshke rqd. */;
pub const DSP_VC_UPDATE_INGAIN: c_uint = 0x00E5	/* Handshke rqd. */;
pub const DSP_VC_ADD_AUDIO_BUFFER: c_uint = 0x00E1	/* Handshke rqd. */;
pub const DSP_VC_TEST_ASIC: c_uint = 0x00ED;
pub const DSP_VC_UPDATE_CLOCKS: c_uint = 0x00E9	/* Handshke rqd. */;
pub const DSP_VC_SET_LAYLA24_FREQUENCY_REG: c_uint = 0x00E9	/* Handshke rqd. */;
pub const DSP_VC_SET_LAYLA_SAMPLE_RATE: c_uint = 0x00EB	/* Handshke rqd. */;
pub const DSP_VC_SET_GD_AUDIO_STATE: c_uint = 0x00EB	/* Handshke rqd. */;
pub const DSP_VC_WRITE_CONTROL_REG: c_uint = 0x00EB	/* Handshke rqd. */;
pub const DSP_VC_MIDI_WRITE: c_uint = 0x00E7	/* Handshke rqd. */;
pub const DSP_VC_STOP_TRANSFER: c_uint = 0x00DF	/* Handshke rqd. */;
pub const DSP_VC_UPDATE_FLAGS: c_uint = 0x00FB	/* Handshke rqd. */;
pub const DSP_VC_GO_COMATOSE: c_uint = 0x00d9;

//
// Timeouts
//

//
// Flags for .Flags field in the comm page
//
pub const DSP_FLAG_MIDI_INPUT: c_uint = 0x0001	/* Enable MIDI input */;
pub const DSP_FLAG_SPDIF_NONAUDIO: c_uint = 0x0002	/* Sets the "non-audio" bit;
// in the S/PDIF out status
// bits.  Clear this flag for
// audio data;
// set it for AC3 or WMA or
// some such
pub const DSP_FLAG_PROFESSIONAL_SPDIF: c_uint = 0x0008	/* 1 Professional, 0 Consumer */;
//
// Clock detect bits reported by the DSP for Gina20, Layla20, Darla24, and Mia
//
pub const GLDM_CLOCK_DETECT_BIT_WORD: c_uint = 0x0002;
pub const GLDM_CLOCK_DETECT_BIT_SUPER: c_uint = 0x0004;
pub const GLDM_CLOCK_DETECT_BIT_SPDIF: c_uint = 0x0008;
pub const GLDM_CLOCK_DETECT_BIT_ESYNC: c_uint = 0x0010;
//
// Clock detect bits reported by the DSP for Gina24, Mona, and Layla24
//
pub const GML_CLOCK_DETECT_BIT_WORD96: c_uint = 0x0002;
pub const GML_CLOCK_DETECT_BIT_WORD48: c_uint = 0x0004;
pub const GML_CLOCK_DETECT_BIT_SPDIF48: c_uint = 0x0008;
pub const GML_CLOCK_DETECT_BIT_SPDIF96: c_uint = 0x0010;

pub const GML_CLOCK_DETECT_BIT_ESYNC: c_uint = 0x0020;
pub const GML_CLOCK_DETECT_BIT_ADAT: c_uint = 0x0040;
//
// Layla clock numbers to send to DSP
//
pub const LAYLA20_CLOCK_INTERNAL: c_int = 0;
pub const LAYLA20_CLOCK_SPDIF: c_int = 1;
pub const LAYLA20_CLOCK_WORD: c_int = 2;
pub const LAYLA20_CLOCK_SUPER: c_int = 3;
//
// Gina/Darla clock states
//
pub const GD_CLOCK_NOCHANGE: c_int = 0;
pub const GD_CLOCK_44: c_int = 1;
pub const GD_CLOCK_48: c_int = 2;
pub const GD_CLOCK_SPDIFIN: c_int = 3;
pub const GD_CLOCK_UNDEF: c_uint = 0xff;
//
// Gina/Darla S/PDIF status bits
//
pub const GD_SPDIF_STATUS_NOCHANGE: c_int = 0;
pub const GD_SPDIF_STATUS_44: c_int = 1;
pub const GD_SPDIF_STATUS_48: c_int = 2;
pub const GD_SPDIF_STATUS_UNDEF: c_uint = 0xff;
//
// Layla20 output clocks
//
pub const LAYLA20_OUTPUT_CLOCK_SUPER: c_int = 0;
pub const LAYLA20_OUTPUT_CLOCK_WORD: c_int = 1;
//
pub const GD24_96000: c_uint = 0x0;
pub const GD24_48000: c_uint = 0x1;
pub const GD24_44100: c_uint = 0x2;
pub const GD24_32000: c_uint = 0x3;
pub const GD24_22050: c_uint = 0x4;
pub const GD24_16000: c_uint = 0x5;
pub const GD24_11025: c_uint = 0x6;
pub const GD24_8000: c_uint = 0x7;
pub const GD24_88200: c_uint = 0x8;
pub const GD24_EXT_SYNC: c_uint = 0x9;
//
// Return values from the DSP when ASIC is loaded
//
pub const ASIC_ALREADY_LOADED: c_uint = 0x1;
pub const ASIC_NOT_LOADED: c_uint = 0x0;
//
// DSP Audio formats
//
// These are the audio formats that the DSP can transfer
// via input and output pipes.  LE means little-endian,
// BE means big-endian.
//
// DSP_AUDIOFORM_MS_8
//
// 8-bit mono unsigned samples.  For playback,
// mono data is duplicated out the left and right channels
// of the output bus.  The "MS" part of the name
// means mono->stereo.
//
// DSP_AUDIOFORM_MS_16LE
//
// 16-bit signed little-endian mono samples.  Playback works
// like the previous code.
//
// DSP_AUDIOFORM_MS_24LE
//
// 24-bit signed little-endian mono samples.  Data is packed
// three bytes per sample; if you had two samples 0x112233 and 0x445566
// they would be stored in memory like this: 33 22 11 66 55 44.
//
// DSP_AUDIOFORM_MS_32LE
//
// 24-bit signed little-endian mono samples in a 32-bit
// container.  In other words, each sample is a 32-bit signed
// integer, where the actual audio data is left-justified
// in the 32 bits and only the 24 most significant bits are valid.
//
// DSP_AUDIOFORM_SS_8
// DSP_AUDIOFORM_SS_16LE
// DSP_AUDIOFORM_SS_24LE
// DSP_AUDIOFORM_SS_32LE
//
// Like the previous ones, except now with stereo interleaved
// data.  "SS" means stereo->stereo.
//
// DSP_AUDIOFORM_MM_32LE
//
// Similar to DSP_AUDIOFORM_MS_32LE, except that the mono
// data is not duplicated out both the left and right outputs.
// This mode is used by the ASIO driver.  Here, "MM" means
// mono->mono.
//
// DSP_AUDIOFORM_MM_32BE
//
// Just like DSP_AUDIOFORM_MM_32LE, but now the data is
// in big-endian format.
//

pub const DSP_AUDIOFORM_INVALID: c_uint = 0xFF	/* Invalid audio format */;
//
// Super-interleave is defined as interleaving by 4 or more.  Darla20 and Gina20
// do not support super interleave.
//
// 16 bit, 24 bit, and 32 bit little endian samples are supported for super
// interleave.  The interleave factor must be even.  16 - way interleave is the
// current maximum, so you can interleave by 4, 6, 8, 10, 12, 14, and 16.
//
// The actual format code is derived by taking the define below and or-ing with
// the interleave factor.  So, 32 bit interleave by 6 is 0x86 and
// 16 bit interleave by 16 is (0x40 | 0x10) = 0x50.
//
pub const DSP_AUDIOFORM_SUPER_INTERLEAVE_16LE: c_uint = 0x40;
pub const DSP_AUDIOFORM_SUPER_INTERLEAVE_24LE: c_uint = 0xc0;
pub const DSP_AUDIOFORM_SUPER_INTERLEAVE_32LE: c_uint = 0x80;
//
// Gina24, Mona, and Layla24 control register defines
//
pub const GML_CONVERTER_ENABLE: c_uint = 0x0010;
pub const GML_SPDIF_PRO_MODE: c_uint = 0x0020	/* Professional S/PDIF == 1,;
pub const GML_SPDIF_SAMPLE_RATE0: c_uint = 0x0040;
pub const GML_SPDIF_SAMPLE_RATE1: c_uint = 0x0080;
pub const GML_SPDIF_TWO_CHANNEL: c_uint = 0x0100	/* 1 == two channels,;
pub const GML_SPDIF_NOT_AUDIO: c_uint = 0x0200;
pub const GML_SPDIF_COPY_PERMIT: c_uint = 0x0400;
pub const GML_SPDIF_24_BIT: c_uint = 0x0800	/* 1 == 24 bit, 0 == 20 bit */;
pub const GML_ADAT_MODE: c_uint = 0x1000	/* 1 == ADAT mode, 0 == S/PDIF mode */;
pub const GML_SPDIF_OPTICAL_MODE: c_uint = 0x2000	/* 1 == optical mode, 0 == RCA mode */;
pub const GML_SPDIF_CDROM_MODE: c_uint = 0x3000	/* 1 == CDROM mode,;
// 0 == RCA or optical mode
pub const GML_DOUBLE_SPEED_MODE: c_uint = 0x4000	/* 1 == double speed,;
pub const GML_DIGITAL_IN_AUTO_MUTE: c_uint = 0x800000;

pub const GML_48KHZ: c_uint = 0x2;
pub const GML_44KHZ: c_uint = 0x3;
pub const GML_32KHZ: c_uint = 0x4;
pub const GML_22KHZ: c_uint = 0x5;
pub const GML_16KHZ: c_uint = 0x6;
pub const GML_11KHZ: c_uint = 0x7;
pub const GML_8KHZ: c_uint = 0x8;
pub const GML_SPDIF_CLOCK: c_uint = 0x9;
pub const GML_ADAT_CLOCK: c_uint = 0xA;
pub const GML_WORD_CLOCK: c_uint = 0xB;
pub const GML_ESYNC_CLOCK: c_uint = 0xC;
pub const GML_ESYNCx2_CLOCK: c_uint = 0xD;
pub const GML_CLOCK_CLEAR_MASK: c_uint = 0xffffbff0;

pub const GML_DIGITAL_MODE_CLEAR_MASK: c_uint = 0xffffcfff;
pub const GML_SPDIF_FORMAT_CLEAR_MASK: c_uint = 0xfffff01f;
//
// Mia sample rate and clock setting constants
//
pub const MIA_32000: c_uint = 0x0040;
pub const MIA_44100: c_uint = 0x0042;
pub const MIA_48000: c_uint = 0x0041;
pub const MIA_88200: c_uint = 0x0142;
pub const MIA_96000: c_uint = 0x0141;
pub const MIA_SPDIF: c_uint = 0x00000044;
pub const MIA_SPDIF96: c_uint = 0x00000144;

//
// 3G register bits
//
pub const E3G_CONVERTER_ENABLE: c_uint = 0x0010;
pub const E3G_SPDIF_PRO_MODE: c_uint = 0x0020	/* Professional S/PDIF == 1,;
pub const E3G_SPDIF_SAMPLE_RATE0: c_uint = 0x0040;
pub const E3G_SPDIF_SAMPLE_RATE1: c_uint = 0x0080;
pub const E3G_SPDIF_TWO_CHANNEL: c_uint = 0x0100	/* 1 == two channels,;
pub const E3G_SPDIF_NOT_AUDIO: c_uint = 0x0200;
pub const E3G_SPDIF_COPY_PERMIT: c_uint = 0x0400;
pub const E3G_SPDIF_24_BIT: c_uint = 0x0800	/* 1 == 24 bit, 0 == 20 bit */;
pub const E3G_DOUBLE_SPEED_MODE: c_uint = 0x4000	/* 1 == double speed,;
pub const E3G_PHANTOM_POWER: c_uint = 0x8000	/* 1 == phantom power on,;

pub const E3G_48KHZ: c_uint = 0x2;
pub const E3G_44KHZ: c_uint = 0x3;
pub const E3G_32KHZ: c_uint = 0x4;
pub const E3G_22KHZ: c_uint = 0x5;
pub const E3G_16KHZ: c_uint = 0x6;
pub const E3G_11KHZ: c_uint = 0x7;
pub const E3G_8KHZ: c_uint = 0x8;
pub const E3G_SPDIF_CLOCK: c_uint = 0x9;
pub const E3G_ADAT_CLOCK: c_uint = 0xA;
pub const E3G_WORD_CLOCK: c_uint = 0xB;
pub const E3G_CONTINUOUS_CLOCK: c_uint = 0xE;
pub const E3G_ADAT_MODE: c_uint = 0x1000;
pub const E3G_SPDIF_OPTICAL_MODE: c_uint = 0x2000;
pub const E3G_CLOCK_CLEAR_MASK: c_uint = 0xbfffbff0;
pub const E3G_DIGITAL_MODE_CLEAR_MASK: c_uint = 0xffffcfff;
pub const E3G_SPDIF_FORMAT_CLEAR_MASK: c_uint = 0xfffff01f;
// Clock detect bits reported by the DSP
pub const E3G_CLOCK_DETECT_BIT_WORD96: c_uint = 0x0001;
pub const E3G_CLOCK_DETECT_BIT_WORD48: c_uint = 0x0002;
pub const E3G_CLOCK_DETECT_BIT_SPDIF48: c_uint = 0x0004;
pub const E3G_CLOCK_DETECT_BIT_ADAT: c_uint = 0x0004;
pub const E3G_CLOCK_DETECT_BIT_SPDIF96: c_uint = 0x0008;

// Frequency control register
pub const E3G_MAGIC_NUMBER: c_int = 677376000;

pub const E3G_FREQ_REG_MAX: c_uint = 0xffff;
// 3G external box types
pub const E3G_GINA3G_BOX_TYPE: c_uint = 0x00;
pub const E3G_LAYLA3G_BOX_TYPE: c_uint = 0x10;
pub const E3G_ASIC_NOT_LOADED: c_uint = 0xffff;
pub const E3G_BOX_TYPE_MASK: c_uint = 0xf0;
// Indigo express control register values
pub const INDIGO_EXPRESS_32000: c_uint = 0x02;
pub const INDIGO_EXPRESS_44100: c_uint = 0x01;
pub const INDIGO_EXPRESS_48000: c_uint = 0x00;
pub const INDIGO_EXPRESS_DOUBLE_SPEED: c_uint = 0x10;
pub const INDIGO_EXPRESS_QUAD_SPEED: c_uint = 0x04;
pub const INDIGO_EXPRESS_CLOCK_MASK: c_uint = 0x17;
//
// Gina20 & Layla20 have input gain controls for the analog inputs;
// this is the magic number for the hardware that gives you 0 dB at -10.
//
pub const GL20_INPUT_GAIN_MAGIC_NUMBER: c_uint = 0xC8;
//
// Defines how much time must pass between DSP load attempts
//

//
// Size of arrays for the comm page.  MAX_PLAY_TAPS and MAX_REC_TAPS are
// no longer used, but the sizes must still be right for the DSP to see
// the comm page correctly.
//
pub const MONITOR_ARRAY_SIZE: c_uint = 0x180;
pub const VMIXER_ARRAY_SIZE: c_uint = 0x40;
pub const MIDI_OUT_BUFFER_SIZE: c_int = 32;
pub const MIDI_IN_BUFFER_SIZE: c_int = 256;
pub const MAX_PLAY_TAPS: c_int = 168;
pub const MAX_REC_TAPS: c_int = 192;
pub const DSP_MIDI_OUT_FIFO_SIZE: c_int = 64;
// sg_entry is a single entry for the scatter-gather list.  The array of struct
pub const MAX_SGLIST_ENTRIES: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_entry {
    pub addr: __le32,
    pub size: __le32,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comm_page {
    pub /: *mut *mut __le32 comm_size; / size of this object 0x000 4,
    pub /: *mut *mut __le32 flags; / See Appendix A below 0x004 4,
    pub /: *mut *mut __le32 unused; / Unused entry 0x008 4,
    pub /: *mut *mut __le32 sample_rate; / Card sample rate in Hz 0x00c 4,
    pub /: *mut *mut __le32 handshake; / DSP command handshake 0x010 4,
    pub /: *mut *mut __le32 cmd_start; / Chs. to start mask 0x014 4,
    pub /: *mut *mut __le32 cmd_stop; / Chs. to stop mask 0x018 4,
    pub /: *mut *mut __le32 cmd_reset; / Chs. to reset mask 0x01c 4,
    pub /: *mut *mut *mut __le16 audio_format[DSP_MAXPIPES]; / Chs. audio format 0x020 322,
    pub sglist_addr: [sg_entry; DSP_MAXPIPES],
// Chs. Physical sglist addrs	0x060	32*8
    pub position: [__le32; DSP_MAXPIPES],
// Positions for ea. ch.	0x160	32*4
    pub vu_meter: [i8; DSP_MAXPIPES],
// VU meters			0x1e0	32*1
    pub peak_meter: [i8; DSP_MAXPIPES],
// Peak meters			0x200	32*1
    pub line_out_level: [i8; DSP_MAXAUDIOOUTPUTS],
// Output gain			0x220	16*1
    pub line_in_level: [i8; DSP_MAXAUDIOINPUTS],
// Input gain			0x230	16*1
    pub monitors: [i8; MONITOR_ARRAY_SIZE],
// Monitor map			0x240	0x180
    pub play_coeff: [__le32; MAX_PLAY_TAPS],
// Gina/Darla play filters - obsolete	0x3c0	168*4
    pub rec_coeff: [__le32; MAX_REC_TAPS],
// Gina/Darla record filters - obsolete	0x660	192*4
    pub midi_input: [__le16; MIDI_IN_BUFFER_SIZE],
// MIDI input data transfer buffer	0x960	256*2
    pub /: *mut *mut u8 gd_clock_state; / Chg Gina/Darla clock state 0xb60 1,
    pub /: *mut *mut u8 gd_spdif_status; / Chg. Gina/Darla S/PDIF state 0xb61 1,
    pub /: *mut *mut u8 gd_resampler_state; / Should always be 3 0xb62 1,
    pub /: *mut *mut u8 filler2; / 0xb63 1,
    pub /: *mut *mut __le32 nominal_level_mask; / -10 level enable mask 0xb64 4,
    pub /: *mut *mut __le16 input_clock; / Chg. Input clock state 0xb68 2,
    pub /: *mut *mut __le16 output_clock; / Chg. Output clock state 0xb6a 2,
    pub /: *mut *mut __le32 status_clocks; / Current Input clock state 0xb6c 4,
    pub /: *mut *mut __le32 ext_box_status; / External box status 0xb70 4,
    pub /: *mut *mut __le32 cmd_add_buffer; / Pipes to add (obsolete) 0xb74 4,
    pub midi_out_free_count: __le32,
// # of bytes free in MIDI output FIFO	0xb78	4
    pub /: *mut *mut __le32 unused2; / Cyclic pipes 0xb7c 4,
    pub control_register: __le32,
// Mona, Gina24, Layla24, 3G ctrl reg	0xb80	4
    pub /: *mut *mut __le32 e3g_frq_register; / 3G frequency register 0xb84 4,
    pub /: *mut *mut *mut u8 filler[24]; / filler 0xb88 241,
    pub vmixer: [i8; VMIXER_ARRAY_SIZE],
// Vmixer levels		0xba0	64*1
    pub midi_output: [u8; MIDI_OUT_BUFFER_SIZE],
// MIDI output data		0xbe0	32*1
}
