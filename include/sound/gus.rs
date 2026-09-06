//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/gus.h
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
// Global structures used for GUS part of ALSA driver
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

// IO ports

// GF1 registers
// global registers
pub const SNDRV_GF1_GB_ACTIVE_VOICES: c_uint = 0x0e;
pub const SNDRV_GF1_GB_VOICES_IRQ: c_uint = 0x0f;
pub const SNDRV_GF1_GB_GLOBAL_MODE: c_uint = 0x19;
pub const SNDRV_GF1_GW_LFO_BASE: c_uint = 0x1a;
pub const SNDRV_GF1_GB_VOICES_IRQ_READ: c_uint = 0x1f;
pub const SNDRV_GF1_GB_DRAM_DMA_CONTROL: c_uint = 0x41;
pub const SNDRV_GF1_GW_DRAM_DMA_LOW: c_uint = 0x42;
pub const SNDRV_GF1_GW_DRAM_IO_LOW: c_uint = 0x43;
pub const SNDRV_GF1_GB_DRAM_IO_HIGH: c_uint = 0x44;
pub const SNDRV_GF1_GB_SOUND_BLASTER_CONTROL: c_uint = 0x45;
pub const SNDRV_GF1_GB_ADLIB_TIMER_1: c_uint = 0x46;
pub const SNDRV_GF1_GB_ADLIB_TIMER_2: c_uint = 0x47;
pub const SNDRV_GF1_GB_RECORD_RATE: c_uint = 0x48;
pub const SNDRV_GF1_GB_REC_DMA_CONTROL: c_uint = 0x49;
pub const SNDRV_GF1_GB_JOYSTICK_DAC_LEVEL: c_uint = 0x4b;
pub const SNDRV_GF1_GB_RESET: c_uint = 0x4c;
pub const SNDRV_GF1_GB_DRAM_DMA_HIGH: c_uint = 0x50;
pub const SNDRV_GF1_GW_DRAM_IO16: c_uint = 0x51;
pub const SNDRV_GF1_GW_MEMORY_CONFIG: c_uint = 0x52;
pub const SNDRV_GF1_GB_MEMORY_CONTROL: c_uint = 0x53;
pub const SNDRV_GF1_GW_FIFO_RECORD_BASE_ADDR: c_uint = 0x54;
pub const SNDRV_GF1_GW_FIFO_PLAY_BASE_ADDR: c_uint = 0x55;
pub const SNDRV_GF1_GW_FIFO_SIZE: c_uint = 0x56;
pub const SNDRV_GF1_GW_INTERLEAVE: c_uint = 0x57;
pub const SNDRV_GF1_GB_COMPATIBILITY: c_uint = 0x59;
pub const SNDRV_GF1_GB_DECODE_CONTROL: c_uint = 0x5a;
pub const SNDRV_GF1_GB_VERSION_NUMBER: c_uint = 0x5b;
pub const SNDRV_GF1_GB_MPU401_CONTROL_A: c_uint = 0x5c;
pub const SNDRV_GF1_GB_MPU401_CONTROL_B: c_uint = 0x5d;
pub const SNDRV_GF1_GB_EMULATION_IRQ: c_uint = 0x60;
// voice specific registers
pub const SNDRV_GF1_VB_ADDRESS_CONTROL: c_uint = 0x00;
pub const SNDRV_GF1_VW_FREQUENCY: c_uint = 0x01;
pub const SNDRV_GF1_VW_START_HIGH: c_uint = 0x02;
pub const SNDRV_GF1_VW_START_LOW: c_uint = 0x03;

pub const SNDRV_GF1_VW_END_HIGH: c_uint = 0x04;
pub const SNDRV_GF1_VW_END_LOW: c_uint = 0x05;

pub const SNDRV_GF1_VB_VOLUME_RATE: c_uint = 0x06;
pub const SNDRV_GF1_VB_VOLUME_START: c_uint = 0x07;
pub const SNDRV_GF1_VB_VOLUME_END: c_uint = 0x08;
pub const SNDRV_GF1_VW_VOLUME: c_uint = 0x09;
pub const SNDRV_GF1_VW_CURRENT_HIGH: c_uint = 0x0a;
pub const SNDRV_GF1_VW_CURRENT_LOW: c_uint = 0x0b;

pub const SNDRV_GF1_VB_PAN: c_uint = 0x0c;
pub const SNDRV_GF1_VW_OFFSET_RIGHT: c_uint = 0x0c;
pub const SNDRV_GF1_VB_VOLUME_CONTROL: c_uint = 0x0d;
pub const SNDRV_GF1_VB_UPPER_ADDRESS: c_uint = 0x10;
pub const SNDRV_GF1_VW_EFFECT_HIGH: c_uint = 0x11;
pub const SNDRV_GF1_VW_EFFECT_LOW: c_uint = 0x12;

pub const SNDRV_GF1_VW_OFFSET_LEFT: c_uint = 0x13;
pub const SNDRV_GF1_VB_ACCUMULATOR: c_uint = 0x14;
pub const SNDRV_GF1_VB_MODE: c_uint = 0x15;
pub const SNDRV_GF1_VW_EFFECT_VOLUME: c_uint = 0x16;
pub const SNDRV_GF1_VB_FREQUENCY_LFO: c_uint = 0x17;
pub const SNDRV_GF1_VB_VOLUME_LFO: c_uint = 0x18;
pub const SNDRV_GF1_VW_OFFSET_RIGHT_FINAL: c_uint = 0x1b;
pub const SNDRV_GF1_VW_OFFSET_LEFT_FINAL: c_uint = 0x1c;
pub const SNDRV_GF1_VW_EFFECT_VOLUME_FINAL: c_uint = 0x1d;
// ICS registers
pub const SNDRV_ICS_MIC_DEV: c_int = 0;
pub const SNDRV_ICS_LINE_DEV: c_int = 1;
pub const SNDRV_ICS_CD_DEV: c_int = 2;
pub const SNDRV_ICS_GF1_DEV: c_int = 3;
pub const SNDRV_ICS_NONE_DEV: c_int = 4;
pub const SNDRV_ICS_MASTER_DEV: c_int = 5;
// LFO
pub const SNDRV_LFO_TREMOLO: c_int = 0;
pub const SNDRV_LFO_VIBRATO: c_int = 1;
// misc
pub const SNDRV_GF1_DMA_UNSIGNED: c_uint = 0x80;
pub const SNDRV_GF1_DMA_16BIT: c_uint = 0x40;
pub const SNDRV_GF1_DMA_IRQ: c_uint = 0x20;
pub const SNDRV_GF1_DMA_WIDTH16: c_uint = 0x04;
pub const SNDRV_GF1_DMA_READ: c_uint = 0x02	/* read from GUS's DRAM */;
pub const SNDRV_GF1_DMA_ENABLE: c_uint = 0x01;
// ramp ranges

pub const SNDRV_GF1_MIN_VOLUME: c_int = 1800;
pub const SNDRV_GF1_MAX_VOLUME: c_int = 4095;

pub const SNDRV_GF1_MAX_OFFSET: c_int = 255;
pub const SNDRV_GF1_MAX_TDEPTH: c_int = 90;
// defines for memory manager
pub const SNDRV_GF1_MEM_BLOCK_16BIT: c_uint = 0x0001;
pub const SNDRV_GF1_MEM_OWNER_DRIVER: c_uint = 0x0001;
pub const SNDRV_GF1_MEM_OWNER_WAVE_SIMPLE: c_uint = 0x0002;
pub const SNDRV_GF1_MEM_OWNER_WAVE_GF1: c_uint = 0x0003;
pub const SNDRV_GF1_MEM_OWNER_WAVE_IWFFFF: c_uint = 0x0004;
// constants for interrupt handlers
pub const SNDRV_GF1_HANDLER_MIDI_OUT: c_uint = 0x00010000;
pub const SNDRV_GF1_HANDLER_MIDI_IN: c_uint = 0x00020000;
pub const SNDRV_GF1_HANDLER_TIMER1: c_uint = 0x00040000;
pub const SNDRV_GF1_HANDLER_TIMER2: c_uint = 0x00080000;
pub const SNDRV_GF1_HANDLER_VOICE: c_uint = 0x00100000;
pub const SNDRV_GF1_HANDLER_DMA_WRITE: c_uint = 0x00200000;
pub const SNDRV_GF1_HANDLER_DMA_READ: c_uint = 0x00400000;

// constants for DMA flags
pub const SNDRV_GF1_DMA_TRIGGER: c_int = 1;
// ---
// GF1 specific structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gf1_bank_info {
    pub address: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gf1_mem_block {
    pub /: *mut *mut unsigned short flags; / flags - SNDRV_GF1_MEM_BLOCK_XXXX,
    pub /: *mut *mut unsigned short owner; / owner - SNDRV_GF1_MEM_OWNER_XXXX,
    pub /: *mut *mut unsigned int share; / share count,
    pub /: *mut *mut unsigned int share_id[4]; / share ID,
    pub ptr: c_uint,
    pub size: c_uint,
    pub name: *mut c_char,
    pub next: *mut snd_gf1_mem_block,
    pub prev: *mut snd_gf1_mem_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gf1_mem {
    pub banks_8: [snd_gf1_bank_info; 4],
    pub banks_16: [snd_gf1_bank_info; 4],
    pub first: *mut snd_gf1_mem_block,
    pub last: *mut snd_gf1_mem_block,
    pub memory_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gf1_dma_block {
    pub /: *mut *mut *mut void buffer; / buffer in computer's RAM,
    pub /: *mut *mut unsigned long buf_addr; / buffer address,
    pub /: *mut *mut unsigned int addr; / address in onboard memory,
    pub /: *mut *mut unsigned int count; / count in bytes,
    pub /: *mut *mut unsigned int cmd; / DMA command (format),
    pub private_data): *mut *mut *mut void (ack)(struct snd_gus_card  gus, void,
    pub private_data: *mut c_void,
    pub next: *mut snd_gf1_dma_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gus_port {
    pub chset: *mut *mut snd_midi_channel_set,
    pub gus: *mut *mut snd_gus_card,
    pub /: *mut *mut int mode; / operation mode,
    pub /: *mut *mut int client; / sequencer client number,
    pub /: *mut *mut int port; / sequencer port number,
    pub 1: unsigned int midi_has_voices:,
}

pub const SNDRV_GF1_VOICE_TYPE_PCM: c_int = 0;
pub const SNDRV_GF1_VOICE_TYPE_SYNTH: c_int = 1;
pub const SNDRV_GF1_VOICE_TYPE_MIDI: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_gus_volume_state {
    VENV_BEFORE,
    VENV_ATTACK,
    VENV_SUSTAIN,
    VENV_RELEASE,
    VENV_DONE,
    VENV_VOLUME
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gus_voice {
    pub number: c_int,
    pub 1: midi:,
    pub flags: c_uint,
    pub client: c_uchar,
    pub port: c_uchar,
    pub index: c_uchar,
    pub pad: c_uchar,

    pub interrupt_stat_wave: c_uint,
    pub interrupt_stat_volume: c_uint,

    pub voice): *mut *mut *mut *mut void (handler_wave) (struct snd_gus_card  gus, struct snd_gus_voice,
    pub voice): *mut *mut *mut *mut void (handler_volume) (struct snd_gus_card  gus, struct snd_gus_voice,
    pub voice): *mut *mut *mut *mut void (handler_effect) (struct snd_gus_card  gus, struct snd_gus_voice,
    pub gus): *mut *mut *mut void (volume_change) (struct snd_gus_card,
    pub sample_ops: *mut snd_gus_sample_ops,
// running status / registers
    pub fc_register: c_ushort,
    pub fc_lfo: c_ushort,
    pub gf1_volume: c_ushort,
    pub control: c_uchar,
    pub mode: c_uchar,
    pub gf1_pan: c_uchar,
    pub effect_accumulator: c_uchar,
    pub volume_control: c_uchar,
    pub venv_value_next: c_uchar,
    pub venv_state: snd_gus_volume_state,
    pub venv_state_prev: snd_gus_volume_state,
    pub vlo: c_ushort,
    pub vro: c_ushort,
    pub gf1_effect_volume: c_ushort,
// ---
    pub private_data: *mut c_void,
    pub voice): *mut *mut void (private_free)(struct snd_gus_voice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gf1 {
    pub /: *mut *mut effect:1; / use effect voices,
    pub /: *mut *mut unsigned long port; / port of GF1 chip,
    pub res_port1: *mut resource,
    pub res_port2: *mut resource,
    pub /: *mut *mut int irq; / IRQ number,
    pub /: *mut *mut int dma1; / DMA1 number,
    pub /: *mut *mut int dma2; / DMA2 number,
    pub /: *mut *mut unsigned int memory; / GUS's DRAM size in bytes,
    pub /: *mut *mut unsigned int rom_memory; / GUS's ROM size in bytes,
    pub /: *mut *mut unsigned int rom_present; / bitmask,
    pub /: *mut *mut unsigned int rom_banks; / GUS's ROM banks,
    pub mem_alloc: snd_gf1_mem,
// registers
    pub reg_page: c_ushort,
    pub reg_regsel: c_ushort,
    pub reg_data8: c_ushort,
    pub reg_data16: c_ushort,
    pub reg_irqstat: c_ushort,
    pub reg_dram: c_ushort,
    pub reg_timerctrl: c_ushort,
    pub reg_timerdata: c_ushort,
    pub ics_regs: [c_uchar; 6][2],
// ---------
    pub /: *mut *mut unsigned char active_voices; / active voices,
    pub /: *mut *mut unsigned char active_voice; / selected voice (GF1PAGE register),
    pub /: *mut *mut snd_gus_voice voices[32]; / GF1 voices,
    pub default_voice_address: c_uint,
    pub /: *mut *mut unsigned short playback_freq; / GF1 playback (mixing) frequency,
    pub /: *mut *mut unsigned short mode; / see to SNDRV_GF1_MODE_XXXX,
    pub volume_ramp: c_uchar,
    pub smooth_pan: c_uchar,
    pub full_range_pan: c_uchar,
    pub pad0: c_uchar,
    pub lfos: *mut c_uchar,
// interrupt handlers
    pub gus): *mut *mut *mut void (interrupt_handler_midi_out) (struct snd_gus_card,
    pub gus): *mut *mut *mut void (interrupt_handler_midi_in) (struct snd_gus_card,
    pub gus): *mut *mut *mut void (interrupt_handler_timer1) (struct snd_gus_card,
    pub gus): *mut *mut *mut void (interrupt_handler_timer2) (struct snd_gus_card,
    pub gus): *mut *mut *mut void (interrupt_handler_dma_write) (struct snd_gus_card,
    pub gus): *mut *mut *mut void (interrupt_handler_dma_read) (struct snd_gus_card,

    pub interrupt_stat_midi_out: c_uint,
    pub interrupt_stat_midi_in: c_uint,
    pub interrupt_stat_timer1: c_uint,
    pub interrupt_stat_timer2: c_uint,
    pub interrupt_stat_dma_write: c_uint,
    pub interrupt_stat_dma_read: c_uint,
    pub interrupt_stat_voice_lost: c_uint,

// synthesizer
    pub seq_client: c_int,
    pub seq_ports: [snd_gus_port; 4],
// timer
    pub timer_enabled: c_ushort,
    pub timer1: *mut snd_timer,
    pub timer2: *mut snd_timer,
// midi
    pub uart_cmd: c_ushort,
    pub uart_framing: c_uint,
    pub uart_overrun: c_uint,
// dma operations
    pub dma_flags: c_uint,
    pub dma_shared: c_uint,
    pub dma_data_pcm: *mut snd_gf1_dma_block,
    pub dma_data_pcm_last: *mut snd_gf1_dma_block,
    pub dma_data_synth: *mut snd_gf1_dma_block,
    pub dma_data_synth_last: *mut snd_gf1_dma_block,
    pub private_data): *mut *mut *mut void (dma_ack)(struct snd_gus_card  gus, void,
    pub dma_private_data: *mut c_void,
// pcm
    pub pcm_channels: c_int,
    pub pcm_alloc_voices: c_int,
    pub pcm_volume_level_left: c_ushort,
    pub pcm_volume_level_right: c_ushort,
    pub pcm_volume_level_left1: c_ushort,
    pub pcm_volume_level_right1: c_ushort,
    pub pcm_rcntrl_reg: c_uchar,
    pub pad_end: c_uchar,
}

// main structure for GUS card
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_gus_card {
    pub card: *mut snd_card,
    pub /: *mut *mut uart_enable:1; / enable MIDI UART,
    pub /: *mut *mut unsigned short revision; / revision of chip,
    pub /: *mut *mut unsigned short max_cntrl_val; / GUS MAX control value,
    pub /: *mut *mut unsigned short mix_cntrl_reg; / mixer control register,
    pub /: *mut *mut unsigned short joystick_dac; / joystick DAC level,
    pub /: *mut *mut int timer_dev; / timer device,
    pub /: *mut *mut snd_gf1 gf1; / gf1 specific variables,
    pub pcm: *mut snd_pcm,
    pub pcm_cap_substream: *mut snd_pcm_substream,
    pub c_dma_size: c_uint,
    pub c_period_size: c_uint,
    pub c_pos: c_uint,
    pub midi_uart: *mut snd_rawmidi,
    pub midi_substream_output: *mut snd_rawmidi_substream,
    pub midi_substream_input: *mut snd_rawmidi_substream,
    pub reg_lock: spinlock_t,
    pub voice_alloc: spinlock_t,
    pub active_voice_lock: spinlock_t,
    pub event_lock: spinlock_t,
    pub dma_lock: spinlock_t,
    pub pcm_volume_level_lock: spinlock_t,
    pub uart_cmd_lock: spinlock_t,
    pub dma_mutex: mutex,
    pub register_mutex: mutex,
}

// I/O functions for GF1/InterWave chip - gus_io.c
extern "C" {
    pub fn inb(_arg: GUSP(gus, _arg: MIDISTAT)) -> return;
}
extern "C" {
    pub fn inb(_arg: GUSP(gus, _arg: MIDIDATA)) -> return;
}
extern "C" {
    pub fn snd_gf1_delay(gus: *mut *mut snd_gus_card);
}
extern "C" {
    pub fn snd_gf1_ctrl_stop(gus: *mut *mut snd_gus_card, reg: c_uchar);
}
extern "C" {
    pub fn snd_gf1_write8(gus: *mut *mut snd_gus_card, reg: c_uchar, data: c_uchar);
}
extern "C" {
    pub fn snd_gf1_look8(gus: *mut *mut snd_gus_card, reg: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn snd_gf1_look8(_arg: gus, 0x80: reg |) -> return;
}
extern "C" {
    pub fn snd_gf1_write16(gus: *mut *mut snd_gus_card, reg: c_uchar, data: c_uint);
}
extern "C" {
    pub fn snd_gf1_look16(gus: *mut *mut snd_gus_card, reg: c_uchar) -> c_ushort;
}
extern "C" {
    pub fn snd_gf1_look16(_arg: gus, 0x80: reg |) -> return;
}
extern "C" {
    pub fn snd_gf1_adlib_write(gus: *mut *mut snd_gus_card, reg: c_uchar, data: c_uchar);
}
extern "C" {
    pub fn snd_gf1_dram_addr(gus: *mut *mut snd_gus_card, addr: c_uint);
}
extern "C" {
    pub fn snd_gf1_poke(gus: *mut *mut snd_gus_card, addr: c_uint, data: c_uchar);
}
extern "C" {
    pub fn snd_gf1_peek(gus: *mut *mut snd_gus_card, addr: c_uint) -> c_uchar;
}
extern "C" {
    pub fn snd_gf1_write_addr(gus: *mut *mut snd_gus_card, reg: c_uchar, addr: c_uint, w_16bit: c_short);
}
extern "C" {
    pub fn snd_gf1_read_addr(gus: *mut *mut snd_gus_card, reg: c_uchar, w_16bit: c_short) -> c_uint;
}
extern "C" {
    pub fn snd_gf1_i_ctrl_stop(gus: *mut *mut snd_gus_card, reg: c_uchar);
}
extern "C" {
    pub fn snd_gf1_i_write8(gus: *mut *mut snd_gus_card, reg: c_uchar, data: c_uchar);
}
extern "C" {
    pub fn snd_gf1_i_look8(gus: *mut *mut snd_gus_card, reg: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn snd_gf1_i_write16(gus: *mut *mut snd_gus_card, reg: c_uchar, data: c_uint);
}
extern "C" {
    pub fn snd_gf1_i_look8(_arg: gus, 0x80: reg |) -> return;
}
extern "C" {
    pub fn snd_gf1_i_look16(gus: *mut *mut snd_gus_card, reg: c_uchar) -> c_ushort;
}
extern "C" {
    pub fn snd_gf1_i_look16(_arg: gus, 0x80: reg |) -> return;
}
extern "C" {
    pub fn snd_gf1_select_active_voices(gus: *mut *mut snd_gus_card);
}
// gus_lfo.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SND_IW_LFO_PROGRAM {
    pub freq_and_control: c_ushort,
    pub depth_final: c_uchar,
    pub depth_inc: c_uchar,
    pub twave: c_ushort,
    pub depth: c_ushort,
}

// gus_mem.c
extern "C" {
    pub fn snd_gf1_mem_xfree(alloc: *mut *mut snd_gf1_mem, block: *mut *mut snd_gf1_mem_block) -> c_int;
}
extern "C" {
    pub fn snd_gf1_mem_free(alloc: *mut *mut snd_gf1_mem, address: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_gf1_mem_free_owner(alloc: *mut *mut snd_gf1_mem, owner: c_int) -> c_int;
}
extern "C" {
    pub fn snd_gf1_mem_init(gus: *mut *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gf1_mem_done(gus: *mut *mut snd_gus_card) -> c_int;
}
// gus_mem_proc.c
extern "C" {
    pub fn snd_gf1_mem_proc_init(gus: *mut *mut snd_gus_card) -> c_int;
}
// gus_dma.c
extern "C" {
    pub fn snd_gf1_dma_init(gus: *mut *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gf1_dma_done(gus: *mut *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gf1_dma_suspend(gus: *mut snd_gus_card);
}
// gus_volume.c
extern "C" {
    pub fn snd_gf1_lvol_to_gvol_raw(vol: c_uint) -> c_ushort;
}
extern "C" {
    pub fn snd_gf1_translate_freq(gus: *mut *mut snd_gus_card, freq2: c_uint) -> c_ushort;
}
// gus_reset.c
extern "C" {
    pub fn snd_gf1_set_default_handlers(gus: *mut *mut snd_gus_card, what: c_uint);
}
extern "C" {
    pub fn snd_gf1_smart_stop_voice(gus: *mut *mut snd_gus_card, voice: c_ushort);
}
extern "C" {
    pub fn snd_gf1_stop_voice(gus: *mut *mut snd_gus_card, voice: c_ushort);
}
extern "C" {
    pub fn snd_gf1_stop_voices(gus: *mut *mut snd_gus_card, v_min: c_ushort, v_max: c_ushort);
}
extern "C" {
    pub fn snd_gf1_free_voice(gus: *mut *mut snd_gus_card, voice: *mut snd_gus_voice);
}
extern "C" {
    pub fn snd_gf1_start(gus: *mut *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gf1_stop(gus: *mut *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gf1_suspend(gus: *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gf1_resume(gus: *mut snd_gus_card) -> c_int;
}
// gus_mixer.c
extern "C" {
    pub fn snd_gf1_new_mixer(gus: *mut *mut snd_gus_card) -> c_int;
}
// gus_pcm.c
extern "C" {
    pub fn snd_gf1_pcm_new(gus: *mut snd_gus_card, pcm_dev: c_int, control_index: c_int) -> c_int;
}
// gus.c
extern "C" {
    pub fn snd_gus_initialize(gus: *mut *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gus_suspend(gus: *mut snd_gus_card) -> c_int;
}
extern "C" {
    pub fn snd_gus_resume(gus: *mut snd_gus_card) -> c_int;
}
// gus_irq.c
extern "C" {
    pub fn snd_gus_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}

extern "C" {
    pub fn snd_gus_irq_profile_init(gus: *mut snd_gus_card);
}

// gus_uart.c
extern "C" {
    pub fn snd_gf1_rawmidi_new(gus: *mut snd_gus_card, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_gf1_uart_suspend(gus: *mut snd_gus_card);
}
extern "C" {
    pub fn snd_gf1_uart_resume(gus: *mut snd_gus_card);
}
// gus_dram.c
// gus_timer.c
extern "C" {
    pub fn snd_gf1_timers_init(gus: *mut snd_gus_card);
}
extern "C" {
    pub fn snd_gf1_timers_done(gus: *mut snd_gus_card);
}
extern "C" {
    pub fn snd_gf1_timers_resume(gus: *mut snd_gus_card);
}
