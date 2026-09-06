//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sb.h
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
// Header file for SoundBlaster cards
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sb_hw_type {
    SB_HW_AUTO,
    SB_HW_10,
    SB_HW_20,
    SB_HW_201,
    SB_HW_PRO,
    SB_HW_JAZZ16,		/* Media Vision Jazz16 */
    SB_HW_16,
    SB_HW_16CSP,		/* SB16 with CSP chip */
    SB_HW_ALS100,		/* Avance Logic ALS100 chip */
    SB_HW_ALS4000,		/* Avance Logic ALS4000 chip */
    SB_HW_DT019X,		/* Diamond Tech. DT-019X / Avance Logic ALS-007 */
    SB_HW_CS5530,		/* Cyrix/NatSemi 5530 VSA1 */
}

pub const SB_OPEN_PCM: c_uint = 0x01;
pub const SB_OPEN_MIDI_INPUT: c_uint = 0x02;
pub const SB_OPEN_MIDI_OUTPUT: c_uint = 0x04;
pub const SB_OPEN_MIDI_INPUT_TRIGGER: c_uint = 0x08;
pub const SB_OPEN_MIDI_OUTPUT_TRIGGER: c_uint = 0x10;
pub const SB_MODE_HALT: c_uint = 0x00;
pub const SB_MODE_PLAYBACK_8: c_uint = 0x01;
pub const SB_MODE_PLAYBACK_16: c_uint = 0x02;

pub const SB_MODE_CAPTURE_8: c_uint = 0x04;
pub const SB_MODE_CAPTURE_16: c_uint = 0x08;

pub const SB_RATE_LOCK_PLAYBACK: c_uint = 0x10;
pub const SB_RATE_LOCK_CAPTURE: c_uint = 0x20;

pub const SB_MPU_INPUT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sb {
    pub /: *mut *mut unsigned long port; / base port of DSP chip,
    pub res_port: *mut resource,
    pub /: *mut *mut unsigned long mpu_port; / MPU port for SB DSP 4.0+,
    pub /: *mut *mut int irq; / IRQ number of DSP chip,
    pub /: *mut *mut int dma8; / 8-bit DMA,
    pub /: *mut *mut int dma16; / 16-bit DMA,
    pub /: *mut *mut unsigned short version; / version of DSP chip,
    pub /: *mut *mut sb_hw_type hardware; / see to SB_HW_XXXX,
    pub /: *mut *mut unsigned long alt_port; / alternate port (ALS4000),
    pub /: *mut *mut *mut pci_dev pci; / ALS4000,
    pub /: *mut *mut unsigned int open; / see to SB_OPEN_XXXX for sb8,
// also SNDRV_SB_CSP_MODE_XXX for sb16_csp
    pub /: *mut *mut unsigned int mode; / current mode of stream,
    pub /: *mut *mut unsigned int force_mode16; / force 16-bit mode of streams,
    pub /: *mut *mut unsigned int locked_rate; / sb16 duplex,
    pub playback_format: c_uint,
    pub capture_format: c_uint,
    pub midi_timer: timer_list,
    pub p_dma_size: c_uint,
    pub p_period_size: c_uint,
    pub c_dma_size: c_uint,
    pub c_period_size: c_uint,
    pub mixer_lock: spinlock_t,
    pub name: [c_char; 32],
    pub /: *mut *mut *mut void csp; / used only when CONFIG_SND_SB16_CSP is set,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub rmidi: *mut snd_rawmidi,
    pub midi_substream_input: *mut snd_rawmidi_substream,
    pub midi_substream_output: *mut snd_rawmidi_substream,
    pub rmidi_callback: irq_handler_t,
    pub reg_lock: spinlock_t,
    pub open_lock: spinlock_t,
    pub midi_input_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
    pub saved_regs: [c_uchar; 0x20],
}

// I/O ports

pub const s_b_SB_RESET: c_uint = 0x6;
pub const s_b_SB_READ: c_uint = 0xa;
pub const s_b_SB_WRITE: c_uint = 0xc;
pub const s_b_SB_COMMAND: c_uint = 0xc;
pub const s_b_SB_STATUS: c_uint = 0xc;
pub const s_b_SB_DATA_AVAIL: c_uint = 0xe;
pub const s_b_SB_DATA_AVAIL_16: c_uint = 0xf;
pub const s_b_SB_MIXER_ADDR: c_uint = 0x4;
pub const s_b_SB_MIXER_DATA: c_uint = 0x5;
pub const s_b_SB_OPL3_LEFT: c_uint = 0x0;
pub const s_b_SB_OPL3_RIGHT: c_uint = 0x2;
pub const s_b_SB_OPL3_BOTH: c_uint = 0x8;
pub const SB_DSP_OUTPUT: c_uint = 0x14;
pub const SB_DSP_INPUT: c_uint = 0x24;
pub const SB_DSP_BLOCK_SIZE: c_uint = 0x48;
pub const SB_DSP_HI_OUTPUT: c_uint = 0x91;
pub const SB_DSP_HI_INPUT: c_uint = 0x99;
pub const SB_DSP_LO_OUTPUT_AUTO: c_uint = 0x1c;
pub const SB_DSP_LO_INPUT_AUTO: c_uint = 0x2c;
pub const SB_DSP_HI_OUTPUT_AUTO: c_uint = 0x90;
pub const SB_DSP_HI_INPUT_AUTO: c_uint = 0x98;
pub const SB_DSP_IMMED_INT: c_uint = 0xf2;
pub const SB_DSP_GET_VERSION: c_uint = 0xe1;
pub const SB_DSP_SPEAKER_ON: c_uint = 0xd1;
pub const SB_DSP_SPEAKER_OFF: c_uint = 0xd3;
pub const SB_DSP_DMA8_OFF: c_uint = 0xd0;
pub const SB_DSP_DMA8_ON: c_uint = 0xd4;
pub const SB_DSP_DMA8_EXIT: c_uint = 0xda;
pub const SB_DSP_DMA16_OFF: c_uint = 0xd5;
pub const SB_DSP_DMA16_ON: c_uint = 0xd6;
pub const SB_DSP_DMA16_EXIT: c_uint = 0xd9;
pub const SB_DSP_SAMPLE_RATE: c_uint = 0x40;
pub const SB_DSP_SAMPLE_RATE_OUT: c_uint = 0x41;
pub const SB_DSP_SAMPLE_RATE_IN: c_uint = 0x42;
pub const SB_DSP_MONO_8BIT: c_uint = 0xa0;
pub const SB_DSP_MONO_16BIT: c_uint = 0xa4;
pub const SB_DSP_STEREO_8BIT: c_uint = 0xa8;
pub const SB_DSP_STEREO_16BIT: c_uint = 0xac;
pub const SB_DSP_MIDI_INPUT_IRQ: c_uint = 0x31;
pub const SB_DSP_MIDI_UART_IRQ: c_uint = 0x35;
pub const SB_DSP_MIDI_OUTPUT: c_uint = 0x38;
pub const SB_DSP4_OUT8_AI: c_uint = 0xc6;
pub const SB_DSP4_IN8_AI: c_uint = 0xce;
pub const SB_DSP4_OUT16_AI: c_uint = 0xb6;
pub const SB_DSP4_IN16_AI: c_uint = 0xbe;
pub const SB_DSP4_MODE_UNS_MONO: c_uint = 0x00;
pub const SB_DSP4_MODE_SIGN_MONO: c_uint = 0x10;
pub const SB_DSP4_MODE_UNS_STEREO: c_uint = 0x20;
pub const SB_DSP4_MODE_SIGN_STEREO: c_uint = 0x30;
pub const SB_DSP4_OUTPUT: c_uint = 0x3c;
pub const SB_DSP4_INPUT_LEFT: c_uint = 0x3d;
pub const SB_DSP4_INPUT_RIGHT: c_uint = 0x3e;
// registers for SB 2.0 mixer
pub const SB_DSP20_MASTER_DEV: c_uint = 0x02;
pub const SB_DSP20_PCM_DEV: c_uint = 0x0A;
pub const SB_DSP20_CD_DEV: c_uint = 0x08;
pub const SB_DSP20_FM_DEV: c_uint = 0x06;
// registers for SB PRO mixer
pub const SB_DSP_MASTER_DEV: c_uint = 0x22;
pub const SB_DSP_PCM_DEV: c_uint = 0x04;
pub const SB_DSP_LINE_DEV: c_uint = 0x2e;
pub const SB_DSP_CD_DEV: c_uint = 0x28;
pub const SB_DSP_FM_DEV: c_uint = 0x26;
pub const SB_DSP_MIC_DEV: c_uint = 0x0a;
pub const SB_DSP_CAPTURE_SOURCE: c_uint = 0x0c;
pub const SB_DSP_CAPTURE_FILT: c_uint = 0x0c;
pub const SB_DSP_PLAYBACK_FILT: c_uint = 0x0e;
pub const SB_DSP_STEREO_SW: c_uint = 0x0e;
pub const SB_DSP_MIXS_MIC0: c_uint = 0x00	/* same as MIC */;
pub const SB_DSP_MIXS_CD: c_uint = 0x01;
pub const SB_DSP_MIXS_MIC: c_uint = 0x02;
pub const SB_DSP_MIXS_LINE: c_uint = 0x03;
// registers (only for left channel) for SB 16 mixer
pub const SB_DSP4_MASTER_DEV: c_uint = 0x30;
pub const SB_DSP4_BASS_DEV: c_uint = 0x46;
pub const SB_DSP4_TREBLE_DEV: c_uint = 0x44;
pub const SB_DSP4_SYNTH_DEV: c_uint = 0x34;
pub const SB_DSP4_PCM_DEV: c_uint = 0x32;
pub const SB_DSP4_SPEAKER_DEV: c_uint = 0x3b;
pub const SB_DSP4_LINE_DEV: c_uint = 0x38;
pub const SB_DSP4_MIC_DEV: c_uint = 0x3a;
pub const SB_DSP4_OUTPUT_SW: c_uint = 0x3c;
pub const SB_DSP4_CD_DEV: c_uint = 0x36;
pub const SB_DSP4_IGAIN_DEV: c_uint = 0x3f;
pub const SB_DSP4_OGAIN_DEV: c_uint = 0x41;
pub const SB_DSP4_MIC_AGC: c_uint = 0x43;
// additional registers for SB 16 mixer
pub const SB_DSP4_IRQSETUP: c_uint = 0x80;
pub const SB_DSP4_DMASETUP: c_uint = 0x81;
pub const SB_DSP4_IRQSTATUS: c_uint = 0x82;
pub const SB_DSP4_MPUSETUP: c_uint = 0x84;
pub const SB_DSP4_3DSE: c_uint = 0x90;
// Registers for DT-019x / ALS-007 mixer
pub const SB_DT019X_MASTER_DEV: c_uint = 0x62;
pub const SB_DT019X_PCM_DEV: c_uint = 0x64;
pub const SB_DT019X_SYNTH_DEV: c_uint = 0x66;
pub const SB_DT019X_CD_DEV: c_uint = 0x68;
pub const SB_DT019X_MIC_DEV: c_uint = 0x6a;
pub const SB_DT019X_SPKR_DEV: c_uint = 0x6a;
pub const SB_DT019X_LINE_DEV: c_uint = 0x6e;
pub const SB_DT019X_OUTPUT_SW2: c_uint = 0x4c;
pub const SB_DT019X_CAPTURE_SW: c_uint = 0x6c;
pub const SB_DT019X_CAP_CD: c_uint = 0x02;
pub const SB_DT019X_CAP_MIC: c_uint = 0x04;
pub const SB_DT019X_CAP_LINE: c_uint = 0x06;
pub const SB_DT019X_CAP_SYNTH: c_uint = 0x07;
pub const SB_DT019X_CAP_MAIN: c_uint = 0x07;
pub const SB_ALS4000_MONO_IO_CTRL: c_uint = 0x4b;
pub const SB_ALS4000_OUT_MIXER_CTRL_2: c_uint = 0x4c;
pub const SB_ALS4000_MIC_IN_GAIN: c_uint = 0x4d;
pub const SB_ALS4000_ANALOG_REFRNC_VOLT_CTRL: c_uint = 0x4e;
pub const SB_ALS4000_FMDAC: c_uint = 0x4f;
pub const SB_ALS4000_3D_SND_FX: c_uint = 0x50;
pub const SB_ALS4000_3D_TIME_DELAY: c_uint = 0x51;
pub const SB_ALS4000_3D_AUTO_MUTE: c_uint = 0x52;
pub const SB_ALS4000_ANALOG_BLOCK_CTRL: c_uint = 0x53;
pub const SB_ALS4000_3D_DELAYLINE_PATTERN: c_uint = 0x54;
pub const SB_ALS4000_CR3_CONFIGURATION: c_uint = 0xc3 /* bit 7 is Digital Loop Enable */;
pub const SB_ALS4000_QSOUND: c_uint = 0xdb;
// IRQ setting bitmap
pub const SB_IRQSETUP_IRQ9: c_uint = 0x01;
pub const SB_IRQSETUP_IRQ5: c_uint = 0x02;
pub const SB_IRQSETUP_IRQ7: c_uint = 0x04;
pub const SB_IRQSETUP_IRQ10: c_uint = 0x08;
// IRQ types
pub const SB_IRQTYPE_8BIT: c_uint = 0x01;
pub const SB_IRQTYPE_16BIT: c_uint = 0x02;
pub const SB_IRQTYPE_MPUIN: c_uint = 0x04;
pub const ALS4K_IRQTYPE_CR1E_DMA: c_uint = 0x20;
// DMA setting bitmap
pub const SB_DMASETUP_DMA0: c_uint = 0x01;
pub const SB_DMASETUP_DMA1: c_uint = 0x02;
pub const SB_DMASETUP_DMA3: c_uint = 0x08;
pub const SB_DMASETUP_DMA5: c_uint = 0x20;
pub const SB_DMASETUP_DMA6: c_uint = 0x40;
pub const SB_DMASETUP_DMA7: c_uint = 0x80;
//
// sb_common.c
extern "C" {
    pub fn snd_sbdsp_command(chip: *mut snd_sb, val: c_uchar) -> c_int;
}
extern "C" {
    pub fn snd_sbdsp_get_byte(chip: *mut snd_sb) -> c_int;
}
extern "C" {
    pub fn snd_sbdsp_reset(chip: *mut snd_sb) -> c_int;
}
// sb_mixer.c
extern "C" {
    pub fn snd_sbmixer_write(chip: *mut snd_sb, reg: c_uchar, data: c_uchar);
}
extern "C" {
    pub fn snd_sbmixer_read(chip: *mut snd_sb, reg: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int;
}

extern "C" {
    pub fn snd_sbmixer_suspend(chip: *mut snd_sb);
}
extern "C" {
    pub fn snd_sbmixer_resume(chip: *mut snd_sb);
}

// sb8_init.c
extern "C" {
    pub fn snd_sb8dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
}
// sb8.c
extern "C" {
    pub fn snd_sb8dsp_interrupt(chip: *mut snd_sb) -> irqreturn_t;
}
extern "C" {
    pub fn snd_sb8_playback_open(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_sb8_capture_open(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_sb8_playback_close(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_sb8_capture_close(substream: *mut snd_pcm_substream) -> c_int;
}
// midi8.c
extern "C" {
    pub fn snd_sb8dsp_midi_interrupt(chip: *mut snd_sb) -> irqreturn_t;
}
extern "C" {
    pub fn snd_sb8dsp_midi(chip: *mut snd_sb, device: c_int) -> c_int;
}
// sb16_init.c
extern "C" {
    pub fn snd_sb16dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_sb16dsp_configure(chip: *mut snd_sb) -> c_int;
}
// sb16.c
extern "C" {
    pub fn snd_sb16dsp_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
// exported mixer stuffs

extern "C" {
    pub fn snd_sbmixer_add_ctl(chip: *mut snd_sb, name: *const c_char, index: c_int, type: c_int, value: c_ulong) -> c_int;
}
// for ease of use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbmix_elem {
    pub name: *const c_char,
    pub type: c_int,
    pub private_value: c_ulong,
}

extern "C" {
    pub fn snd_sbmixer_add_ctl(_arg: chip, _arg: c->name, _arg: 0, _arg: c->type, _arg: c->private_value) -> return;
}
