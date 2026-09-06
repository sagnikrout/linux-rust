//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/emu10k1.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// Creative Labs, Inc.
// Definitions for EMU10K1 (SB Live!) chips
//

//
// ---- FX8010 ----
//
pub const EMU10K1_FX8010_PCM_COUNT: c_int = 8;
//
// Following definition is copied from linux/types.h to support compiling
// this header file in userspace since they are not generally available for
// uapi headers.
//

// instruction set
pub const iMAC0: c_uint = 0x00	/* R = A + (X * Y >> 31)   ; saturation */;
pub const iMAC1: c_uint = 0x01	/* R = A + (-X * Y >> 31)  ; saturation */;
pub const iMAC2: c_uint = 0x02	/* R = A + (X * Y >> 31)   ; wraparound */;
pub const iMAC3: c_uint = 0x03	/* R = A + (-X * Y >> 31)  ; wraparound */;
pub const iMACINT0: c_uint = 0x04	/* R = A + X * Y	   ; saturation */;
pub const iMACINT1: c_uint = 0x05	/* R = A + X * Y	   ; wraparound (31-bit) */;
pub const iACC3: c_uint = 0x06	/* R = A + X + Y	   ; saturation */;
pub const iMACMV: c_uint = 0x07	/* R = A, acc += X * Y >> 31 */;
pub const iANDXOR: c_uint = 0x08	/* R = (A & X) ^ Y */;
pub const iTSTNEG: c_uint = 0x09	/* R = (A >= Y) ? X : ~X */;
pub const iLIMITGE: c_uint = 0x0a	/* R = (A >= Y) ? X : Y */;
pub const iLIMITLT: c_uint = 0x0b	/* R = (A < Y) ? X : Y */;
pub const iLOG: c_uint = 0x0c	/* R = linear_data, A (log_data), X (max_exp), Y (format_word) */;
pub const iEXP: c_uint = 0x0d	/* R = log_data, A (linear_data), X (max_exp), Y (format_word) */;
pub const iINTERP: c_uint = 0x0e	/* R = A + (X * (Y - A) >> 31)  ; saturation */;
pub const iSKIP: c_uint = 0x0f	/* R = A (cc_reg), X (count), Y (cc_test) */;
pub const LOWORD_OPX_MASK: c_uint = 0x000ffc00	/* Instruction operand X			*/;
pub const LOWORD_OPY_MASK: c_uint = 0x000003ff	/* Instruction operand Y			*/;
pub const HIWORD_OPCODE_MASK: c_uint = 0x00f00000	/* Instruction opcode				*/;
pub const HIWORD_RESULT_MASK: c_uint = 0x000ffc00	/* Instruction result				*/;
pub const HIWORD_OPA_MASK: c_uint = 0x000003ff	/* Instruction operand A			*/;
// Audigy Soundcards have a different instruction format
pub const A_LOWORD_OPX_MASK: c_uint = 0x007ff000;
pub const A_LOWORD_OPY_MASK: c_uint = 0x000007ff;
pub const A_HIWORD_OPCODE_MASK: c_uint = 0x0f000000;
pub const A_HIWORD_RESULT_MASK: c_uint = 0x007ff000;
pub const A_HIWORD_OPA_MASK: c_uint = 0x000007ff;
// GPRs

// NB: 0x31 and 0x32 are shared with Center/LFE on SB live 5.1

pub const C_00000000: c_uint = 0x40;
pub const C_00000001: c_uint = 0x41;
pub const C_00000002: c_uint = 0x42;
pub const C_00000003: c_uint = 0x43;
pub const C_00000004: c_uint = 0x44;
pub const C_00000008: c_uint = 0x45;
pub const C_00000010: c_uint = 0x46;
pub const C_00000020: c_uint = 0x47;
pub const C_00000100: c_uint = 0x48;
pub const C_00010000: c_uint = 0x49;
pub const C_00080000: c_uint = 0x4a;
pub const C_10000000: c_uint = 0x4b;
pub const C_20000000: c_uint = 0x4c;
pub const C_40000000: c_uint = 0x4d;
pub const C_80000000: c_uint = 0x4e;
pub const C_7fffffff: c_uint = 0x4f;
pub const C_ffffffff: c_uint = 0x50;
pub const C_fffffffe: c_uint = 0x51;
pub const C_c0000000: c_uint = 0x52;
pub const C_4f1bbcdc: c_uint = 0x53;
pub const C_5a7ef9db: c_uint = 0x54;
pub const C_00100000: c_uint = 0x55		/* ?? */;
pub const GPR_ACCU: c_uint = 0x56		/* ACCUM, accumulator */;
pub const GPR_COND: c_uint = 0x57		/* CCR, condition register */;
pub const GPR_NOISE0: c_uint = 0x58		/* noise source */;
pub const GPR_NOISE1: c_uint = 0x59		/* noise source */;
pub const GPR_IRQ: c_uint = 0x5a		/* IRQ register */;
pub const GPR_DBAC: c_uint = 0x5b		/* TRAM Delay Base Address Counter */;
// Audigy constants
pub const A_C_00000000: c_uint = 0xc0;
pub const A_C_00000001: c_uint = 0xc1;
pub const A_C_00000002: c_uint = 0xc2;
pub const A_C_00000003: c_uint = 0xc3;
pub const A_C_00000004: c_uint = 0xc4;
pub const A_C_00000008: c_uint = 0xc5;
pub const A_C_00000010: c_uint = 0xc6;
pub const A_C_00000020: c_uint = 0xc7;
pub const A_C_00000100: c_uint = 0xc8;
pub const A_C_00010000: c_uint = 0xc9;
pub const A_C_00000800: c_uint = 0xca;
pub const A_C_10000000: c_uint = 0xcb;
pub const A_C_20000000: c_uint = 0xcc;
pub const A_C_40000000: c_uint = 0xcd;
pub const A_C_80000000: c_uint = 0xce;
pub const A_C_7fffffff: c_uint = 0xcf;
pub const A_C_ffffffff: c_uint = 0xd0;
pub const A_C_fffffffe: c_uint = 0xd1;
pub const A_C_c0000000: c_uint = 0xd2;
pub const A_C_4f1bbcdc: c_uint = 0xd3;
pub const A_C_5a7ef9db: c_uint = 0xd4;
pub const A_C_00100000: c_uint = 0xd5;
pub const A_GPR_ACCU: c_uint = 0xd6		/* ACCUM, accumulator */;
pub const A_GPR_COND: c_uint = 0xd7		/* CCR, condition register */;
pub const A_GPR_NOISE0: c_uint = 0xd8		/* noise source */;
pub const A_GPR_NOISE1: c_uint = 0xd9		/* noise source */;
pub const A_GPR_IRQ: c_uint = 0xda		/* IRQ register */;
pub const A_GPR_DBAC: c_uint = 0xdb		/* TRAM Delay Base Address Counter - internal */;
pub const A_GPR_DBACE: c_uint = 0xde		/* TRAM Delay Base Address Counter - external */;
// Each FX general purpose register is 32 bits in length, all bits are used
pub const FXGPREGBASE: c_uint = 0x100		/* FX general purpose registers base       	*/;
pub const A_FXGPREGBASE: c_uint = 0x400		/* Audigy GPRs, 0x400 to 0x5ff			*/;
pub const A_TANKMEMCTLREGBASE: c_uint = 0x100		/* Tank memory control registers base - only for Audigy */;
pub const A_TANKMEMCTLREG_MASK: c_uint = 0x1f		/* only 5 bits used - only for Audigy */;
// Tank audio data is logarithmically compressed down to 16 bits before writing to TRAM and is
// decompressed back to 20 bits on a read.  There are a total of 160 locations, the last 32
// locations are for external TRAM.
pub const TANKMEMDATAREGBASE: c_uint = 0x200		/* Tank memory data registers base     		*/;
pub const TANKMEMDATAREG_MASK: c_uint = 0x000fffff	/* 20 bit tank audio data field			*/;
// Combined address field and memory opcode or flag field.  160 locations, last 32 are external
pub const TANKMEMADDRREGBASE: c_uint = 0x300		/* Tank memory address registers base		*/;
pub const TANKMEMADDRREG_ADDR_MASK: c_uint = 0x000fffff	/* 20 bit tank address field			*/;
pub const TANKMEMADDRREG_CLEAR: c_uint = 0x00800000	/* Clear tank memory				*/;
pub const TANKMEMADDRREG_ALIGN: c_uint = 0x00400000	/* Align read or write relative to tank access	*/;
pub const TANKMEMADDRREG_WRITE: c_uint = 0x00200000	/* Write to tank memory				*/;
pub const TANKMEMADDRREG_READ: c_uint = 0x00100000	/* Read from tank memory			*/;

// cc_reg constants

// FX buses
// These are arbitrary mappings; our DSP code simply expects
// the config files to route the channels this way.
// The numbers are documented in {audigy,sb-live}-mixer.rst.
pub const FXBUS_PCM_LEFT: c_uint = 0x00;
pub const FXBUS_PCM_RIGHT: c_uint = 0x01;
pub const FXBUS_PCM_LEFT_REAR: c_uint = 0x02;
pub const FXBUS_PCM_RIGHT_REAR: c_uint = 0x03;
pub const FXBUS_MIDI_LEFT: c_uint = 0x04;
pub const FXBUS_MIDI_RIGHT: c_uint = 0x05;
pub const FXBUS_PCM_CENTER: c_uint = 0x06;
pub const FXBUS_PCM_LFE: c_uint = 0x07;
pub const FXBUS_PCM_LEFT_FRONT: c_uint = 0x08;
pub const FXBUS_PCM_RIGHT_FRONT: c_uint = 0x09;
pub const FXBUS_MIDI_REVERB: c_uint = 0x0c;
pub const FXBUS_MIDI_CHORUS: c_uint = 0x0d;
pub const FXBUS_PCM_LEFT_SIDE: c_uint = 0x0e;
pub const FXBUS_PCM_RIGHT_SIDE: c_uint = 0x0f;
pub const FXBUS_PT_LEFT: c_uint = 0x14;
pub const FXBUS_PT_RIGHT: c_uint = 0x15;
// Inputs
pub const EXTIN_AC97_L: c_uint = 0x00	/* AC'97 capture channel - left */;
pub const EXTIN_AC97_R: c_uint = 0x01	/* AC'97 capture channel - right */;
pub const EXTIN_SPDIF_CD_L: c_uint = 0x02	/* internal S/PDIF CD - onboard - left */;
pub const EXTIN_SPDIF_CD_R: c_uint = 0x03	/* internal S/PDIF CD - onboard - right */;
pub const EXTIN_ZOOM_L: c_uint = 0x04	/* Zoom Video I2S - left */;
pub const EXTIN_ZOOM_R: c_uint = 0x05	/* Zoom Video I2S - right */;
pub const EXTIN_TOSLINK_L: c_uint = 0x06	/* LiveDrive - TOSLink Optical - left */;
pub const EXTIN_TOSLINK_R: c_uint = 0x07	/* LiveDrive - TOSLink Optical - right */;
pub const EXTIN_LINE1_L: c_uint = 0x08	/* LiveDrive - Line/Mic 1 - left */;
pub const EXTIN_LINE1_R: c_uint = 0x09	/* LiveDrive - Line/Mic 1 - right */;
pub const EXTIN_COAX_SPDIF_L: c_uint = 0x0a	/* LiveDrive - Coaxial S/PDIF - left */;
pub const EXTIN_COAX_SPDIF_R: c_uint = 0x0b /* LiveDrive - Coaxial S/PDIF - right */;
pub const EXTIN_LINE2_L: c_uint = 0x0c	/* LiveDrive - Line/Mic 2 - left */;
pub const EXTIN_LINE2_R: c_uint = 0x0d	/* LiveDrive - Line/Mic 2 - right */;
// Outputs
pub const EXTOUT_AC97_L: c_uint = 0x00	/* AC'97 playback channel - left */;
pub const EXTOUT_AC97_R: c_uint = 0x01	/* AC'97 playback channel - right */;
pub const EXTOUT_TOSLINK_L: c_uint = 0x02	/* LiveDrive - TOSLink Optical - left */;
pub const EXTOUT_TOSLINK_R: c_uint = 0x03	/* LiveDrive - TOSLink Optical - right */;
pub const EXTOUT_AC97_CENTER: c_uint = 0x04	/* SB Live 5.1 - center */;
pub const EXTOUT_AC97_LFE: c_uint = 0x05 /* SB Live 5.1 - LFE */;
pub const EXTOUT_HEADPHONE_L: c_uint = 0x06	/* LiveDrive - Headphone - left */;
pub const EXTOUT_HEADPHONE_R: c_uint = 0x07	/* LiveDrive - Headphone - right */;
pub const EXTOUT_REAR_L: c_uint = 0x08	/* Rear channel - left */;
pub const EXTOUT_REAR_R: c_uint = 0x09	/* Rear channel - right */;
pub const EXTOUT_ADC_CAP_L: c_uint = 0x0a	/* ADC Capture buffer - left */;
pub const EXTOUT_ADC_CAP_R: c_uint = 0x0b	/* ADC Capture buffer - right */;
pub const EXTOUT_MIC_CAP: c_uint = 0x0c	/* MIC Capture buffer */;
pub const EXTOUT_AC97_REAR_L: c_uint = 0x0d	/* SB Live 5.1 (c) 2003 - Rear Left */;
pub const EXTOUT_AC97_REAR_R: c_uint = 0x0e	/* SB Live 5.1 (c) 2003 - Rear Right */;
pub const EXTOUT_ACENTER: c_uint = 0x11 /* Analog Center */;
pub const EXTOUT_ALFE: c_uint = 0x12 /* Analog LFE */;
// Audigy Inputs
pub const A_EXTIN_AC97_L: c_uint = 0x00	/* AC'97 capture channel - left */;
pub const A_EXTIN_AC97_R: c_uint = 0x01	/* AC'97 capture channel - right */;
pub const A_EXTIN_SPDIF_CD_L: c_uint = 0x02	/* digital CD left */;
pub const A_EXTIN_SPDIF_CD_R: c_uint = 0x03	/* digital CD left */;
pub const A_EXTIN_OPT_SPDIF_L: c_uint = 0x04    /* audigy drive Optical SPDIF - left */;
pub const A_EXTIN_OPT_SPDIF_R: c_uint = 0x05    /*                              right */;
pub const A_EXTIN_LINE2_L: c_uint = 0x08	/* audigy drive line2/mic2 - left */;
pub const A_EXTIN_LINE2_R: c_uint = 0x09	/*                           right */;
pub const A_EXTIN_ADC_L: c_uint = 0x0a    /* Philips ADC - left */;
pub const A_EXTIN_ADC_R: c_uint = 0x0b    /*               right */;
pub const A_EXTIN_AUX2_L: c_uint = 0x0c	/* audigy drive aux2 - left */;
pub const A_EXTIN_AUX2_R: c_uint = 0x0d	/*                   - right */;
// Audigiy Outputs
pub const A_EXTOUT_FRONT_L: c_uint = 0x00	/* digital front left */;
pub const A_EXTOUT_FRONT_R: c_uint = 0x01	/*               right */;
pub const A_EXTOUT_CENTER: c_uint = 0x02	/* digital front center */;
pub const A_EXTOUT_LFE: c_uint = 0x03	/* digital front lfe */;
pub const A_EXTOUT_HEADPHONE_L: c_uint = 0x04	/* headphone audigy drive left */;
pub const A_EXTOUT_HEADPHONE_R: c_uint = 0x05	/*                        right */;
pub const A_EXTOUT_REAR_L: c_uint = 0x06	/* digital rear left */;
pub const A_EXTOUT_REAR_R: c_uint = 0x07	/*              right */;
pub const A_EXTOUT_AFRONT_L: c_uint = 0x08	/* analog front left */;
pub const A_EXTOUT_AFRONT_R: c_uint = 0x09	/*              right */;
pub const A_EXTOUT_ACENTER: c_uint = 0x0a	/* analog center */;
pub const A_EXTOUT_ALFE: c_uint = 0x0b	/* analog LFE */;
pub const A_EXTOUT_ASIDE_L: c_uint = 0x0c	/* analog side left  - Audigy 2 ZS */;
pub const A_EXTOUT_ASIDE_R: c_uint = 0x0d	/*             right - Audigy 2 ZS */;
pub const A_EXTOUT_AREAR_L: c_uint = 0x0e	/* analog rear left */;
pub const A_EXTOUT_AREAR_R: c_uint = 0x0f	/*             right */;
pub const A_EXTOUT_AC97_L: c_uint = 0x10	/* AC97 left (front) */;
pub const A_EXTOUT_AC97_R: c_uint = 0x11	/*      right */;
pub const A_EXTOUT_ADC_CAP_L: c_uint = 0x16	/* ADC capture buffer left */;
pub const A_EXTOUT_ADC_CAP_R: c_uint = 0x17	/*                    right */;
pub const A_EXTOUT_MIC_CAP: c_uint = 0x18	/* Mic capture buffer */;
// Definitions for debug register. Note that these are for emu10k1 ONLY.
pub const EMU10K1_DBG_ZC: c_uint = 0x80000000	/* zero tram counter */;
pub const EMU10K1_DBG_SATURATION_OCCURED: c_uint = 0x02000000	/* saturation control */;
pub const EMU10K1_DBG_SATURATION_ADDR: c_uint = 0x01ff0000	/* saturation address */;
pub const EMU10K1_DBG_SINGLE_STEP: c_uint = 0x00008000	/* single step mode */;
pub const EMU10K1_DBG_STEP: c_uint = 0x00004000	/* start single step */;
pub const EMU10K1_DBG_CONDITION_CODE: c_uint = 0x00003e00	/* condition code */;
pub const EMU10K1_DBG_SINGLE_STEP_ADDR: c_uint = 0x000001ff	/* single step address */;
// Definitions for emu10k2 debug register.
pub const A_DBG_ZC: c_uint = 0x40000000	/* zero tram counter */;
pub const A_DBG_SATURATION_OCCURED: c_uint = 0x20000000;
pub const A_DBG_SATURATION_ADDR: c_uint = 0x0ffc0000;
pub const A_DBG_SINGLE_STEP: c_uint = 0x00020000	/* Set to zero to start dsp */;
pub const A_DBG_STEP: c_uint = 0x00010000;
pub const A_DBG_CONDITION_CODE: c_uint = 0x0000f800;
pub const A_DBG_STEP_ADDR: c_uint = 0x000003ff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_info {
    pub /: *mut *mut unsigned int internal_tram_size; / in samples,
    pub /: *mut *mut unsigned int external_tram_size; / in samples,
    pub /: *mut *mut char fxbus_names[16][32]; / names of FXBUSes,
    pub /: *mut *mut char extin_names[16][32]; / names of external inputs,
    pub /: *mut *mut char extout_names[32][32]; / names of external outputs,
    pub /: *mut *mut unsigned int gpr_controls; / count of GPR controls,
}

pub const EMU10K1_GPR_TRANSLATION_NONE: c_int = 0;
pub const EMU10K1_GPR_TRANSLATION_TABLE100: c_int = 1;
pub const EMU10K1_GPR_TRANSLATION_BASS: c_int = 2;
pub const EMU10K1_GPR_TRANSLATION_TREBLE: c_int = 3;
pub const EMU10K1_GPR_TRANSLATION_ONOFF: c_int = 4;
pub const EMU10K1_GPR_TRANSLATION_NEGATE: c_int = 5;
pub const EMU10K1_GPR_TRANSLATION_NEG_TABLE100: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum emu10k1_ctl_elem_iface {
    EMU10K1_CTL_ELEM_IFACE_MIXER = 2,	/* virtual mixer device */
    EMU10K1_CTL_ELEM_IFACE_PCM = 3,		/* PCM device */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emu10k1_ctl_elem_id {
    pub /: *mut *mut unsigned int pad; / don't use,
    pub /: *mut *mut int iface; / interface identifier,
    pub /: *mut *mut unsigned int device; / device/client number,
    pub /: *mut *mut unsigned int subdevice; / subdevice (substream) number,
    pub /: *mut *mut unsigned char name[44]; / ASCII name of item,
    pub /: *mut *mut unsigned int index; / index of item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_control_gpr {
    pub /: *mut *mut emu10k1_ctl_elem_id id; / full control ID definition,
    pub /: *mut *mut unsigned int vcount; / visible count,
    pub /: *mut *mut unsigned int count; / count of GPR (1..16),
    pub /: *mut *mut unsigned short gpr[32]; / GPR number(s),
    pub /: *mut *mut int value[32]; / initial values,
    pub /: *mut *mut int min; / minimum range,
    pub /: *mut *mut int max; / maximum range,
    pub /: *mut *mut *mut unsigned int translation; / translation type (EMU10K1_GPR_TRANSLATION),
    pub tlv: *const c_uint,
}

// old ABI without TLV support
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_control_old_gpr {
    pub id: emu10k1_ctl_elem_id,
    pub vcount: c_uint,
    pub count: c_uint,
    pub gpr: [c_ushort; 32],
    pub value: [c_uint; 32],
    pub min: c_uint,
    pub max: c_uint,
    pub translation: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_code {
    pub name: [c_char; 128],
    pub /: *mut *mut __EMU10K1_DECLARE_BITMAP(gpr_valid, 0x200); / bitmask of valid initializers,
    pub /: *mut *mut *mut __u32 gpr_map; / initializers,
    pub /: *mut *mut unsigned int gpr_add_control_count; / count of GPR controls to add/replace,
    pub /: *mut *mut *mut snd_emu10k1_fx8010_control_gpr gpr_add_controls; / GPR controls to add/replace,
    pub /: *mut *mut unsigned int gpr_del_control_count; / count of GPR controls to remove,
    pub /: *mut *mut *mut emu10k1_ctl_elem_id gpr_del_controls; / IDs of GPR controls to remove,
    pub /: *mut *mut unsigned int gpr_list_control_count; / count of GPR controls to list,
    pub /: *mut *mut unsigned int gpr_list_control_total; / total count of GPR controls,
    pub /: *mut *mut *mut snd_emu10k1_fx8010_control_gpr gpr_list_controls; / listed GPR controls,
    pub /: *mut *mut __EMU10K1_DECLARE_BITMAP(tram_valid, 0x100); / bitmask of valid initializers,
    pub /: *mut *mut *mut __u32 tram_data_map; / data initializers,
    pub /: *mut *mut *mut __u32 tram_addr_map; / map initializers,
    pub /: *mut *mut __EMU10K1_DECLARE_BITMAP(code_valid, 1024); / bitmask of valid instructions,
    pub /: *mut *mut *mut __u32 code; / one instruction - 64 bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_tram {
    pub /: *mut *mut unsigned int address; / 31.bit == 1 -> external TRAM,
    pub /: *mut *mut unsigned int size; / size in samples (4 bytes),
    pub /: *mut *mut *mut unsigned int samples; / pointer to samples (20-bit),
// NULL->clear memory
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_pcm_rec {
    pub /: *mut *mut unsigned int substream; / substream number,
    pub /: *mut *mut unsigned int res1; / reserved,
    pub /: *mut *mut unsigned int channels; / 16-bit channels count, zero = remove this substream,
    pub /: *mut *mut unsigned int tram_start; / ring buffer position in TRAM (in samples),
    pub /: *mut *mut unsigned int buffer_size; / count of buffered samples,
    pub /: *mut *mut unsigned short gpr_size; / GPR containing size of ringbuffer in samples (host),
    pub /: *mut *mut unsigned short gpr_ptr; / GPR containing current pointer in the ring buffer (host = reset, FX8010),
    pub /: *mut *mut unsigned short gpr_count; / GPR containing count of samples between two interrupts (host),
    pub /: *mut *mut unsigned short gpr_tmpcount; / GPR containing current count of samples to interrupt (host = set, FX8010),
    pub /: *mut *mut unsigned short gpr_trigger; / GPR containing trigger (activate) information (host),
    pub /: *mut *mut unsigned short gpr_running; / GPR containing info if PCM is running (FX8010),
    pub /: *mut *mut unsigned char pad; / reserved,
    pub /: *mut *mut unsigned char etram[32]; / external TRAM address & data (one per channel),
    pub /: *mut *mut unsigned int res2; / reserved,
}

