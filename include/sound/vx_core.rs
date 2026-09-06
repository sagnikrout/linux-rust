//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/vx_core.h
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
// Driver for Digigram VX soundcards
//
// Hardware core part
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
//

pub const VX_DRIVER_VERSION: c_uint = 0x010000	/* 1.0.0 */;
//
pub const SIZE_MAX_CMD: c_uint = 0x10;
pub const SIZE_MAX_STATUS: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vx_rmh {
    pub /: *mut *mut u16 LgCmd; / length of the command to send (WORDs),
    pub /: *mut *mut u16 LgStat; / length of the status received (WORDs),
    pub Cmd: [u32; SIZE_MAX_CMD],
    pub Stat: [u32; SIZE_MAX_STATUS],
    pub /: *mut *mut u16 DspStat; / status type, RMP_SSIZE_XXX,
}

pub type pcx_time_t = u64;
pub const VX_MAX_PIPES: c_int = 16;
pub const VX_MAX_PERIODS: c_int = 32;
pub const VX_MAX_CODECS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vx_ibl_info {
    pub /: *mut *mut int size; / the current IBL size (0 = query) in bytes,
    pub /: *mut *mut int max_size; / max. IBL size in bytes,
    pub /: *mut *mut int min_size; / min. IBL size in bytes,
    pub /: *mut *mut int granularity; / granularity,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vx_pipe {
    pub number: c_int,
    pub 1: unsigned int is_capture:,
    pub 1: unsigned int data_mode:,
    pub 1: unsigned int running:,
    pub 1: unsigned int prepared:,
    pub channels: c_int,
    pub differed_type: c_uint,
    pub pcx_time: pcx_time_t,
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut int hbuf_size; / H-buffer size in bytes,
    pub /: *mut *mut int buffer_bytes; / the ALSA pcm buffer size in bytes,
    pub /: *mut *mut int period_bytes; / the ALSA pcm period size in bytes,
    pub /: *mut *mut int hw_ptr; / the current hardware pointer in bytes,
    pub /: *mut *mut int position; / the current position in frames (playback only),
    pub /: *mut *mut int transferred; / the transferred size (per period) in frames,
    pub /: *mut *mut int align; / size of alignment,
    pub /: *mut *mut u64 cur_count; / current sample position (for playback),
    pub /: *mut *mut unsigned int references; / an output pipe may be used for monitoring and/or playback,
    pub only)*/: *mut *mut *mut vx_pipe monitoring_pipe; / pointer to the monitoring pipe (capture pipe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_vx_ops {
// low-level i/o
    pub reg): *mut *mut *mut unsigned char (in8)(struct vx_core chip, int,
    pub reg): *mut *mut *mut unsigned int (in32)(struct vx_core chip, int,
    pub val): *mut *mut *mut void (out8)(struct vx_core chip, int reg, unsigned char,
    pub val): *mut *mut *mut void (out32)(struct vx_core chip, int reg, unsigned int,
// irq
    pub chip): *mut *mut int (test_and_ack)(struct vx_core,
    pub enable): *mut *mut *mut void (validate_irq)(struct vx_core chip, int,
// codec
    pub data): *mut *mut *mut void (write_codec)(struct vx_core chip, int codec, unsigned int,
    pub data): *mut *mut *mut void (akm_write)(struct vx_core chip, int reg, unsigned int,
    pub chip): *mut *mut void (reset_codec)(struct vx_core,
    pub src): *mut *mut *mut void (change_audio_source)(struct vx_core chip, int,
    pub src): *mut *mut *mut void (set_clock_source)(struct vx_core chp, int,
// chip init
    pub fw): *const *const *const int (load_dsp)(struct vx_core chip, int idx, struct firmware,
    pub chip): *mut *mut void (reset_dsp)(struct vx_core,
    pub cold_reset): *mut *mut *mut void (reset_board)(struct vx_core chip, int,
    pub chip): *mut *mut int (add_controls)(struct vx_core,
// pcm
    pub count): *mut *mut vx_pipe pipe, int,
    pub count): *mut *mut vx_pipe pipe, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_vx_hardware {
    pub name: *const c_char,
    pub /: *mut *mut int type; / VX_TYPE_XXX,
// hardware specs
    pub num_codecs: c_uint,
    pub num_ins: c_uint,
    pub num_outs: c_uint,
    pub output_level_max: c_uint,
    pub output_level_db_scale: *const c_uint,
}

// hwdep id string

// hardware type
// VX222 PCI
// VX-pocket
// chip status
// min/max values for analog output for old codecs
pub const VX_ANALOG_OUT_LEVEL_MAX: c_uint = 0xe3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vx_core {
// ALSA stuff
    pub card: *mut snd_card,
    pub pcm: [*mut snd_pcm; VX_MAX_CODECS],
    pub /: *mut *mut int type; / VX_TYPE_XXX,
    pub irq: c_int,
// ports are defined externally
// low-level functions
    pub hw: *const snd_vx_hardware,
    pub ops: *const snd_vx_ops,
    pub lock: mutex,
    pub chip_status: c_uint,
    pub pcm_running: c_uint,
    pub hwdep: *mut snd_hwdep,
    pub /: *mut *mut vx_rmh irq_rmh; / RMH used in interrupts,
    pub /: *mut *mut unsigned int audio_info; / see VX_AUDIO_INFO,
    pub audio_ins: c_uint,
    pub audio_outs: c_uint,
    pub playback_pipes: *mut vx_pipe,
    pub capture_pipes: *mut vx_pipe,
// clock and audio sources
    pub /: *mut *mut unsigned int audio_source; / current audio input source,
    pub audio_source_target: c_uint,
    pub /: *mut *mut unsigned int clock_mode; / clock mode (VX_CLOCK_MODE_XXX),
    pub /: *mut *mut unsigned int clock_source; / current clock source (INTERNAL_QUARTZ or UER_SYNC),
    pub /: *mut *mut unsigned int freq; / current frequency,
    pub /: *mut *mut unsigned int freq_detected; / detected frequency from digital in,
    pub /: *mut *mut unsigned int uer_detected; / VX_UER_MODE_XXX,
    pub /: *mut *mut unsigned int uer_bits; / IEC958 status bits,
    pub /: *mut *mut vx_ibl_info ibl; / IBL information,
// mixer setting
    pub /: *mut *mut int output_level[VX_MAX_CODECS][2]; / analog output level,
    pub /: *mut *mut int audio_gain[2][4]; / digital audio level (playback/capture),
    pub /: *mut *mut unsigned char audio_active[4]; / mute/unmute on digital playback,
    pub /: *mut *mut int audio_monitor[4]; / playback hw-monitor level,
    pub /: *mut *mut unsigned char audio_monitor_active[4]; / playback hw-monitor mute/unmute,
    pub mixer_mutex: mutex,
    pub /: *const *const *const firmware firmware[4]; / loaded firmware data,
}

//
// constructor
//
extern "C" {
    pub fn snd_vx_setup_firmware(chip: *mut vx_core) -> c_int;
}
extern "C" {
    pub fn snd_vx_load_boot_image(chip: *mut vx_core, dsp: *const firmware) -> c_int;
}
extern "C" {
    pub fn snd_vx_dsp_boot(chip: *mut vx_core, dsp: *const firmware) -> c_int;
}
extern "C" {
    pub fn snd_vx_dsp_load(chip: *mut vx_core, dsp: *const firmware) -> c_int;
}
extern "C" {
    pub fn snd_vx_free_firmware(chip: *mut vx_core);
}
//
// interrupt handler; exported for pcmcia
//
extern "C" {
    pub fn snd_vx_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn snd_vx_threaded_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t;
}
//
// lowlevel functions
//

extern "C" {
    pub fn vx_send_msg(chip: *mut vx_core, rmh: *mut vx_rmh) -> c_int;
}
extern "C" {
    pub fn vx_send_msg_nolock(chip: *mut vx_core, rmh: *mut vx_rmh) -> c_int;
}
extern "C" {
    pub fn vx_send_rih(chip: *mut vx_core, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn vx_send_rih_nolock(chip: *mut vx_core, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn vx_reset_codec(chip: *mut vx_core, cold_reset: c_int);
}
//
// check the bit on the specified register
// returns zero if a bit matches, or a negative error code.
// exported for vxpocket driver
//
extern "C" {
    pub fn snd_vx_check_reg_bit(chip: *mut vx_core, reg: c_int, mask: c_int, bit: c_int, time: c_int) -> c_int;
}

//
// pseudo-DMA transfer
//
// error with hardware code,
// the return value is -(VX_ERR_MASK | actual-hw-error-code)
//
pub const VX_ERR_MASK: c_uint = 0x1000000;

//
// pcm stuff
//
extern "C" {
    pub fn snd_vx_pcm_new(chip: *mut vx_core) -> c_int;
}
extern "C" {
    pub fn vx_pcm_update_intr(chip: *mut vx_core, events: c_uint);
}
//
// mixer stuff
//
extern "C" {
    pub fn snd_vx_mixer_new(chip: *mut vx_core) -> c_int;
}
extern "C" {
    pub fn vx_toggle_dac_mute(chip: *mut vx_core, mute: c_int);
}
extern "C" {
    pub fn vx_sync_audio_source(chip: *mut vx_core) -> c_int;
}
extern "C" {
    pub fn vx_set_monitor_level(chip: *mut vx_core, audio: c_int, level: c_int, active: c_int) -> c_int;
}
//
// IEC958 & clock stuff
//
extern "C" {
    pub fn vx_set_iec958_status(chip: *mut vx_core, bits: c_uint);
}
extern "C" {
    pub fn vx_set_clock(chip: *mut vx_core, freq: c_uint) -> c_int;
}
extern "C" {
    pub fn vx_set_internal_clock(chip: *mut vx_core, freq: c_uint);
}
extern "C" {
    pub fn vx_change_frequency(chip: *mut vx_core) -> c_int;
}
//
// PM
//
extern "C" {
    pub fn snd_vx_suspend(card: *mut vx_core) -> c_int;
}
extern "C" {
    pub fn snd_vx_resume(card: *mut vx_core) -> c_int;
}
//
// hardware constants
//

// audio input source
// clock source
// clock mode
// SPDIF/UER type
// register indices
// aliases for VX board
// aliases for VXPOCKET board
// RMH status type
// bits for ICR register
pub const ICR_HF1: c_uint = 0x10;
pub const ICR_HF0: c_uint = 0x08;
pub const ICR_TREQ: c_uint = 0x02	/* Interrupt mode + HREQ set on for transfer (->DSP) request */;
pub const ICR_RREQ: c_uint = 0x01	/* Interrupt mode + RREQ set on for transfer (->PC) request */;
// bits for CVR register
pub const CVR_HC: c_uint = 0x80;
// bits for ISR register
pub const ISR_HF3: c_uint = 0x10;
pub const ISR_HF2: c_uint = 0x08;
pub const ISR_CHK: c_uint = 0x10;
pub const ISR_ERR: c_uint = 0x08;
pub const ISR_TX_READY: c_uint = 0x04;
pub const ISR_TX_EMPTY: c_uint = 0x02;
pub const ISR_RX_FULL: c_uint = 0x01;
// Constants used to access the DATA register
pub const VX_DATA_CODEC_MASK: c_uint = 0x80;
pub const VX_DATA_XICOR_MASK: c_uint = 0x80;
// Constants used to access the CSUER register (both for VX2 and VXP)
pub const VX_SUER_FREQ_MASK: c_uint = 0x0c;
pub const VX_SUER_FREQ_32KHz_MASK: c_uint = 0x0c;
pub const VX_SUER_FREQ_44KHz_MASK: c_uint = 0x00;
pub const VX_SUER_FREQ_48KHz_MASK: c_uint = 0x04;
pub const VX_SUER_DATA_PRESENT_MASK: c_uint = 0x02;
pub const VX_SUER_CLOCK_PRESENT_MASK: c_uint = 0x01;
pub const VX_CUER_HH_BITC_SEL_MASK: c_uint = 0x08;
pub const VX_CUER_MH_BITC_SEL_MASK: c_uint = 0x04;
pub const VX_CUER_ML_BITC_SEL_MASK: c_uint = 0x02;
pub const VX_CUER_LL_BITC_SEL_MASK: c_uint = 0x01;
pub const XX_UER_CBITS_OFFSET_MASK: c_uint = 0x1f;
// bits for audio_info

// DSP Interrupt Request values
pub const VXP_IRQ_OFFSET: c_uint = 0x40 /* add 0x40 offset for vxpocket and vx222/v2 */;
// call with vx_send_irq_dsp()
pub const IRQ_MESS_WRITE_END: c_uint = 0x30;
pub const IRQ_MESS_WRITE_NEXT: c_uint = 0x32;
pub const IRQ_MESS_READ_NEXT: c_uint = 0x34;
pub const IRQ_MESS_READ_END: c_uint = 0x36;
pub const IRQ_MESSAGE: c_uint = 0x38;
pub const IRQ_RESET_CHK: c_uint = 0x3A;
pub const IRQ_CONNECT_STREAM_NEXT: c_uint = 0x26;
pub const IRQ_CONNECT_STREAM_END: c_uint = 0x28;
pub const IRQ_PAUSE_START_CONNECT: c_uint = 0x2A;
pub const IRQ_END_CONNECTION: c_uint = 0x2C;
// Is there async. events pending ( IT Source Test )
pub const ASYNC_EVENTS_PENDING: c_uint = 0x008000;
pub const HBUFFER_EVENTS_PENDING: c_uint = 0x004000   // Not always accurate;
pub const NOTIF_EVENTS_PENDING: c_uint = 0x002000;
pub const TIME_CODE_EVENT_PENDING: c_uint = 0x001000;
pub const FREQUENCY_CHANGE_EVENT_PENDING: c_uint = 0x000800;
pub const END_OF_BUFFER_EVENTS_PENDING: c_uint = 0x000400;
pub const FATAL_DSP_ERROR: c_uint = 0xff0000;
// Stream Format Header Defines
pub const HEADER_FMT_BASE: c_uint = 0xFED00000;
pub const HEADER_FMT_MONO: c_uint = 0x000000C0;
pub const HEADER_FMT_INTEL: c_uint = 0x00008000;
pub const HEADER_FMT_16BITS: c_uint = 0x00002000;
pub const HEADER_FMT_24BITS: c_uint = 0x00004000;
pub const HEADER_FMT_UPTO11: c_uint = 0x00000200	/* frequency is less or equ. to 11k.*/;
pub const HEADER_FMT_UPTO32: c_uint = 0x00000100	/* frequency is over 11k and less then 32k.*/;
// Constants used to access the Codec
pub const XX_CODEC_SELECTOR: c_uint = 0x20;
// codec commands
pub const XX_CODEC_ADC_CONTROL_REGISTER: c_uint = 0x01;
pub const XX_CODEC_DAC_CONTROL_REGISTER: c_uint = 0x02;
pub const XX_CODEC_LEVEL_LEFT_REGISTER: c_uint = 0x03;
pub const XX_CODEC_LEVEL_RIGHT_REGISTER: c_uint = 0x04;
pub const XX_CODEC_PORT_MODE_REGISTER: c_uint = 0x05;
pub const XX_CODEC_STATUS_REPORT_REGISTER: c_uint = 0x06;
pub const XX_CODEC_CLOCK_CONTROL_REGISTER: c_uint = 0x07;
//
// Audio-level control values
//
pub const CVAL_M110DB: c_uint = 0x000	/* -110dB */;
pub const CVAL_M99DB: c_uint = 0x02C;
pub const CVAL_M21DB: c_uint = 0x163;
pub const CVAL_M18DB: c_uint = 0x16F;
pub const CVAL_M10DB: c_uint = 0x18F;
pub const CVAL_0DB: c_uint = 0x1B7;
pub const CVAL_18DB: c_uint = 0x1FF	/* +18dB */;
pub const CVAL_MAX: c_uint = 0x1FF;
pub const AUDIO_IO_HAS_MUTE_LEVEL: c_uint = 0x400000;
pub const AUDIO_IO_HAS_MUTE_MONITORING_1: c_uint = 0x200000;
pub const AUDIO_IO_HAS_MUTE_MONITORING_2: c_uint = 0x100000;
pub const VALID_AUDIO_IO_DIGITAL_LEVEL: c_uint = 0x01;
pub const VALID_AUDIO_IO_MONITORING_LEVEL: c_uint = 0x02;
pub const VALID_AUDIO_IO_MUTE_LEVEL: c_uint = 0x04;
pub const VALID_AUDIO_IO_MUTE_MONITORING_1: c_uint = 0x08;
pub const VALID_AUDIO_IO_MUTE_MONITORING_2: c_uint = 0x10;
