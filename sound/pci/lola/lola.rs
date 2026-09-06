//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/lola/lola.h
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
// Support for Digigram Lola PCI-e boards
//
// Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
//

//
// Lola HD Audio Registers BAR0
//
pub const LOLA_BAR0_GCAP: c_uint = 0x00;
pub const LOLA_BAR0_VMIN: c_uint = 0x02;
pub const LOLA_BAR0_VMAJ: c_uint = 0x03;
pub const LOLA_BAR0_OUTPAY: c_uint = 0x04;
pub const LOLA_BAR0_INPAY: c_uint = 0x06;
pub const LOLA_BAR0_GCTL: c_uint = 0x08;
pub const LOLA_BAR0_WAKEEN: c_uint = 0x0c;
pub const LOLA_BAR0_STATESTS: c_uint = 0x0e;
pub const LOLA_BAR0_GSTS: c_uint = 0x10;
pub const LOLA_BAR0_OUTSTRMPAY: c_uint = 0x18;
pub const LOLA_BAR0_INSTRMPAY: c_uint = 0x1a;
pub const LOLA_BAR0_INTCTL: c_uint = 0x20;
pub const LOLA_BAR0_INTSTS: c_uint = 0x24;
pub const LOLA_BAR0_WALCLK: c_uint = 0x30;
pub const LOLA_BAR0_SSYNC: c_uint = 0x38;
pub const LOLA_BAR0_CORBLBASE: c_uint = 0x40;
pub const LOLA_BAR0_CORBUBASE: c_uint = 0x44;
pub const LOLA_BAR0_CORBWP: c_uint = 0x48	/* no ULONG access */;
pub const LOLA_BAR0_CORBRP: c_uint = 0x4a	/* no ULONG access */;
pub const LOLA_BAR0_CORBCTL: c_uint = 0x4c	/* no ULONG access */;
pub const LOLA_BAR0_CORBSTS: c_uint = 0x4d	/* UCHAR access only */;
pub const LOLA_BAR0_CORBSIZE: c_uint = 0x4e	/* no ULONG access */;
pub const LOLA_BAR0_RIRBLBASE: c_uint = 0x50;
pub const LOLA_BAR0_RIRBUBASE: c_uint = 0x54;
pub const LOLA_BAR0_RIRBWP: c_uint = 0x58;
pub const LOLA_BAR0_RINTCNT: c_uint = 0x5a	/* no ULONG access */;
pub const LOLA_BAR0_RIRBCTL: c_uint = 0x5c;
pub const LOLA_BAR0_RIRBSTS: c_uint = 0x5d	/* UCHAR access only */;
pub const LOLA_BAR0_RIRBSIZE: c_uint = 0x5e	/* no ULONG access */;
pub const LOLA_BAR0_ICW: c_uint = 0x60;
pub const LOLA_BAR0_IRR: c_uint = 0x64;
pub const LOLA_BAR0_ICS: c_uint = 0x68;
pub const LOLA_BAR0_DPLBASE: c_uint = 0x70;
pub const LOLA_BAR0_DPUBASE: c_uint = 0x74;
// stream register offsets from stream base 0x80
pub const LOLA_BAR0_SD0_OFFSET: c_uint = 0x80;
pub const LOLA_REG0_SD_CTL: c_uint = 0x00;
pub const LOLA_REG0_SD_STS: c_uint = 0x03;
pub const LOLA_REG0_SD_LPIB: c_uint = 0x04;
pub const LOLA_REG0_SD_CBL: c_uint = 0x08;
pub const LOLA_REG0_SD_LVI: c_uint = 0x0c;
pub const LOLA_REG0_SD_FIFOW: c_uint = 0x0e;
pub const LOLA_REG0_SD_FIFOSIZE: c_uint = 0x10;
pub const LOLA_REG0_SD_FORMAT: c_uint = 0x12;
pub const LOLA_REG0_SD_BDLPL: c_uint = 0x18;
pub const LOLA_REG0_SD_BDLPU: c_uint = 0x1c;
//
// Lola Digigram Registers BAR1
//
pub const LOLA_BAR1_FPGAVER: c_uint = 0x00;
pub const LOLA_BAR1_DEVER: c_uint = 0x04;
pub const LOLA_BAR1_UCBMV: c_uint = 0x08;
pub const LOLA_BAR1_JTAG: c_uint = 0x0c;
pub const LOLA_BAR1_UARTRX: c_uint = 0x10;
pub const LOLA_BAR1_UARTTX: c_uint = 0x14;
pub const LOLA_BAR1_UARTCR: c_uint = 0x18;
pub const LOLA_BAR1_NVRAMVER: c_uint = 0x1c;
pub const LOLA_BAR1_CTRLSPI: c_uint = 0x20;
pub const LOLA_BAR1_DSPI: c_uint = 0x24;
pub const LOLA_BAR1_AISPI: c_uint = 0x28;
pub const LOLA_BAR1_GRAN: c_uint = 0x2c;
pub const LOLA_BAR1_DINTCTL: c_uint = 0x80;
pub const LOLA_BAR1_DIINTCTL: c_uint = 0x84;
pub const LOLA_BAR1_DOINTCTL: c_uint = 0x88;
pub const LOLA_BAR1_LRC: c_uint = 0x90;
pub const LOLA_BAR1_DINTSTS: c_uint = 0x94;
pub const LOLA_BAR1_DIINTSTS: c_uint = 0x98;
pub const LOLA_BAR1_DOINTSTS: c_uint = 0x9c;
pub const LOLA_BAR1_DSD0_OFFSET: c_uint = 0xa0;
pub const LOLA_BAR1_DSD_SIZE: c_uint = 0x18;
pub const LOLA_BAR1_DSDnSTS: c_uint = 0x00;
pub const LOLA_BAR1_DSDnLPIB: c_uint = 0x04;
pub const LOLA_BAR1_DSDnCTL: c_uint = 0x08;
pub const LOLA_BAR1_DSDnLVI: c_uint = 0x0c;
pub const LOLA_BAR1_DSDnBDPL: c_uint = 0x10;
pub const LOLA_BAR1_DSDnBDPU: c_uint = 0x14;
pub const LOLA_BAR1_SSYNC: c_uint = 0x03e8;
pub const LOLA_BAR1_BOARD_CTRL: c_uint = 0x0f00;
pub const LOLA_BAR1_BOARD_MODE: c_uint = 0x0f02;
pub const LOLA_BAR1_SOURCE_GAIN_ENABLE: c_uint = 0x1000;
pub const LOLA_BAR1_DEST00_MIX_GAIN_ENABLE: c_uint = 0x1004;
pub const LOLA_BAR1_DEST31_MIX_GAIN_ENABLE: c_uint = 0x1080;
pub const LOLA_BAR1_SOURCE00_01_GAIN: c_uint = 0x1084;
pub const LOLA_BAR1_SOURCE30_31_GAIN: c_uint = 0x10c0;

pub const LOLA_BAR1_DEST00_MIX00_01_GAIN: c_uint = 0x10c4;
pub const LOLA_BAR1_DEST00_MIX30_31_GAIN: c_uint = 0x1100;
pub const LOLA_BAR1_DEST01_MIX00_01_GAIN: c_uint = 0x1104;
pub const LOLA_BAR1_DEST01_MIX30_31_GAIN: c_uint = 0x1140;
pub const LOLA_BAR1_DEST31_MIX00_01_GAIN: c_uint = 0x1884;
pub const LOLA_BAR1_DEST31_MIX30_31_GAIN: c_uint = 0x18c0;

pub const LOLA_BAR1_ANALOG_CLIP_IN: c_uint = 0x18c4;
pub const LOLA_BAR1_PEAKMETERS_SOURCE00_01: c_uint = 0x18c8;
pub const LOLA_BAR1_PEAKMETERS_SOURCE30_31: c_uint = 0x1904;

pub const LOLA_BAR1_PEAKMETERS_DEST00_01: c_uint = 0x1908;
pub const LOLA_BAR1_PEAKMETERS_DEST30_31: c_uint = 0x1944;

pub const LOLA_BAR1_PEAKMETERS_AGC00_01: c_uint = 0x1948;
pub const LOLA_BAR1_PEAKMETERS_AGC14_15: c_uint = 0x1964;

// GCTL reset bit

// GCTL unsolicited response enable bit

// CORB/RIRB control, read/write pointer
pub const LOLA_RBCTL_DMA_EN: c_uint = 0x02	/* enable DMA */;
pub const LOLA_RBCTL_IRQ_EN: c_uint = 0x01	/* enable IRQ */;
pub const LOLA_RBRWP_CLR: c_uint = 0x8000	/* read/write pointer clear */;
pub const LOLA_RIRB_EX_UNSOL_EV: c_uint = 0x40000000;
pub const LOLA_RIRB_EX_ERROR: c_uint = 0x80000000;
// CORB int mask: CMEI[0]
pub const LOLA_CORB_INT_CMEI: c_uint = 0x01;

// RIRB int mask: overrun[2], response[0]
pub const LOLA_RIRB_INT_RESPONSE: c_uint = 0x01;
pub const LOLA_RIRB_INT_OVERRUN: c_uint = 0x04;

// DINTCTL and DINTSTS
pub const LOLA_DINT_GLOBAL: c_uint = 0x80000000 /* global interrupt enable bit */;
pub const LOLA_DINT_CTRL: c_uint = 0x40000000 /* controller interrupt enable bit */;
pub const LOLA_DINT_FIFOERR: c_uint = 0x20000000 /* global fifo error enable bit */;
pub const LOLA_DINT_MUERR: c_uint = 0x10000000 /* global microcontroller underrun error */;
// DSDnCTL bits
pub const LOLA_DSD_CTL_SRST: c_uint = 0x01	/* stream reset bit */;
pub const LOLA_DSD_CTL_SRUN: c_uint = 0x02	/* stream DMA start bit */;
pub const LOLA_DSD_CTL_IOCE: c_uint = 0x04	/* interrupt on completion enable */;
pub const LOLA_DSD_CTL_DEIE: c_uint = 0x10	/* descriptor error interrupt enable */;
pub const LOLA_DSD_CTL_VLRCV: c_uint = 0x20	/* valid LRCountValue information in bits 8..31 */;
pub const LOLA_LRC_MASK: c_uint = 0xffffff00;
// DSDnSTS
pub const LOLA_DSD_STS_BCIS: c_uint = 0x04	/* buffer completion interrupt status */;
pub const LOLA_DSD_STS_DESE: c_uint = 0x10	/* descriptor error interrupt */;
pub const LOLA_DSD_STS_FIFORDY: c_uint = 0x20	/* fifo ready */;
pub const LOLA_CORB_ENTRIES: c_int = 256;
pub const MAX_STREAM_IN_COUNT: c_int = 16;
pub const MAX_STREAM_OUT_COUNT: c_int = 16;
pub const MAX_STREAM_COUNT: c_int = 16;

pub const MAX_STREAM_BUFFER_COUNT: c_int = 16;
pub const MAX_AUDIO_INOUT_COUNT: c_int = 16;
pub const LOLA_CLOCK_TYPE_INTERNAL: c_int = 0;
pub const LOLA_CLOCK_TYPE_AES: c_int = 1;
pub const LOLA_CLOCK_TYPE_AES_SYNC: c_int = 2;
pub const LOLA_CLOCK_TYPE_WORDCLOCK: c_int = 3;
pub const LOLA_CLOCK_TYPE_ETHERSOUND: c_int = 4;
pub const LOLA_CLOCK_TYPE_VIDEO: c_int = 5;
pub const LOLA_CLOCK_FORMAT_NONE: c_int = 0;
pub const LOLA_CLOCK_FORMAT_NTSC: c_int = 1;
pub const LOLA_CLOCK_FORMAT_PAL: c_int = 2;
pub const MAX_SAMPLE_CLOCK_COUNT: c_int = 48;
// parameters used with mixer widget's mixer capabilities
pub const LOLA_PEAK_METER_CAN_AGC_MASK: c_int = 1;
pub const LOLA_PEAK_METER_CAN_ANALOG_CLIP_MASK: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_bar {
    pub addr: c_ulong,
    pub remap_addr: *mut void __iomem,
}

// CORB/RIRB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_rb {
    pub /: *mut *mut *mut __le32 buf; / CORB/RIRB buffer, 8 byte per each entry,
    pub /: *mut *mut dma_addr_t addr; / physical address of CORB/RIRB buffer,
    pub /: *mut *mut unsigned short rp, wp; / read/write pointers,
    pub /: *mut *mut int cmds; / number of pending requests,
}

// Pin widget setup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_pin {
    pub nid: c_uint,
    pub is_analog: bool,
    pub amp_mute: c_uint,
    pub amp_step_size: c_uint,
    pub amp_num_steps: c_uint,
    pub amp_offset: c_uint,
    pub max_level: c_uint,
    pub config_default_reg: c_uint,
    pub fixed_gain_list_len: c_uint,
    pub cur_gain_step: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_pin_array {
    pub num_pins: c_uint,
    pub num_analog_pins: c_uint,
    pub pins: [lola_pin; MAX_PINS],
}

// Clock widget setup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_sample_clock {
    pub type: c_uint,
    pub format: c_uint,
    pub freq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_clock_widget {
    pub nid: c_uint,
    pub items: c_uint,
    pub cur_index: c_uint,
    pub cur_freq: c_uint,
    pub cur_valid: bool,
    pub sample_clock: [lola_sample_clock; MAX_SAMPLE_CLOCK_COUNT],
    pub idx_lookup: [c_uint; MAX_SAMPLE_CLOCK_COUNT],
}

pub const LOLA_MIXER_DIM: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_mixer_array {
    pub src_gain_enable: u32,
    pub dest_mix_gain_enable: [u32; LOLA_MIXER_DIM],
    pub src_gain: [u16; LOLA_MIXER_DIM],
    pub dest_mix_gain: [u16; LOLA_MIXER_DIM][LOLA_MIXER_DIM],
}

// Mixer widget setup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_mixer_widget {
    pub nid: c_uint,
    pub caps: c_uint,
    pub array: *mut lola_mixer_array __iomem,
    pub array_saved: *mut lola_mixer_array,
    pub src_stream_outs: c_uint,
    pub src_phys_ins: c_uint,
    pub dest_stream_ins: c_uint,
    pub dest_phys_outs: c_uint,
    pub src_stream_out_ofs: c_uint,
    pub dest_phys_out_ofs: c_uint,
    pub src_mask: c_uint,
    pub dest_mask: c_uint,
}

// Audio stream
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_stream {
    pub /: *mut *mut unsigned int nid; / audio widget NID,
    pub /: *mut *mut unsigned int index; / array index,
    pub /: *mut *mut unsigned int dsd; / DSD index,
    pub can_float: bool,
    pub /: *mut *mut *mut snd_pcm_substream substream; / assigned PCM substream,
    pub /: *mut *mut *mut lola_stream master; / master stream (for multi-channel),
// buffer setup
    pub bufsize: c_uint,
    pub period_bytes: c_uint,
    pub frags: c_uint,
// format + channel setup
    pub format_verb: c_uint,
// flags
    pub opened:1: c_uint,
    pub prepared:1: c_uint,
    pub paused:1: c_uint,
    pub running:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola_pcm {
    pub num_streams: c_uint,
    pub /: *mut *mut *mut snd_dma_buffer bdl; / BDL buffer,
    pub streams: [lola_stream; MAX_STREAM_COUNT],
}

// card instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lola {
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
// pci resources
    pub bar: [lola_bar; 2],
    pub irq: c_int,
// locks
    pub reg_lock: spinlock_t,
    pub open_mutex: mutex,
// CORB/RIRB
    pub corb: lola_rb,
    pub rirb: lola_rb,
    pub /: *mut *mut unsigned int res, res_ex; / last read values,
// last command (for debugging)
    pub last_extdata: unsigned int last_cmd_nid, last_verb, last_data,,
// CORB/RIRB buffers
    pub rb: *mut snd_dma_buffer,
// unsolicited events
    pub last_unsol_res: c_uint,
// streams
    pub pcm: [lola_pcm; 2],
// input src
    pub input_src_caps_mask: c_uint,
    pub input_src_mask: c_uint,
// pins
    pub pin: [lola_pin_array; 2],
// clock
    pub clock: lola_clock_widget,
    pub ref_count_rate: c_int,
    pub sample_rate: c_uint,
// mixer
    pub mixer: lola_mixer_widget,
// hw info
    pub version: c_uint,
    pub lola_caps: c_uint,
// parameters
    pub granularity: c_uint,
    pub sample_rate_min: c_uint,
    pub sample_rate_max: c_uint,
// flags
    pub initialized:1: c_uint,
    pub cold_reset:1: c_uint,
    pub polling_mode:1: c_uint,
// for debugging
    pub debug_res: c_uint,
    pub debug_res_ex: c_uint,
}

pub const BAR0: c_int = 0;
pub const BAR1: c_int = 1;
// Helper macros

// GET verbs HDAudio
pub const LOLA_VERB_GET_STREAM_FORMAT: c_uint = 0xa00;
pub const LOLA_VERB_GET_AMP_GAIN_MUTE: c_uint = 0xb00;
pub const LOLA_VERB_PARAMETERS: c_uint = 0xf00;
pub const LOLA_VERB_GET_POWER_STATE: c_uint = 0xf05;
pub const LOLA_VERB_GET_CONV: c_uint = 0xf06;
pub const LOLA_VERB_GET_UNSOLICITED_RESPONSE: c_uint = 0xf08;
pub const LOLA_VERB_GET_DIGI_CONVERT_1: c_uint = 0xf0d;
pub const LOLA_VERB_GET_CONFIG_DEFAULT: c_uint = 0xf1c;
pub const LOLA_VERB_GET_SUBSYSTEM_ID: c_uint = 0xf20;
// GET verbs Digigram
pub const LOLA_VERB_GET_FIXED_GAIN: c_uint = 0xfc0;
pub const LOLA_VERB_GET_GAIN_SELECT: c_uint = 0xfc1;
pub const LOLA_VERB_GET_MAX_LEVEL: c_uint = 0xfc2;
pub const LOLA_VERB_GET_CLOCK_LIST: c_uint = 0xfc3;
pub const LOLA_VERB_GET_CLOCK_SELECT: c_uint = 0xfc4;
pub const LOLA_VERB_GET_CLOCK_STATUS: c_uint = 0xfc5;
// SET verbs HDAudio
pub const LOLA_VERB_SET_STREAM_FORMAT: c_uint = 0x200;
pub const LOLA_VERB_SET_AMP_GAIN_MUTE: c_uint = 0x300;
pub const LOLA_VERB_SET_POWER_STATE: c_uint = 0x705;
pub const LOLA_VERB_SET_CHANNEL_STREAMID: c_uint = 0x706;
pub const LOLA_VERB_SET_UNSOLICITED_ENABLE: c_uint = 0x708;
pub const LOLA_VERB_SET_DIGI_CONVERT_1: c_uint = 0x70d;
// SET verbs Digigram
pub const LOLA_VERB_SET_GAIN_SELECT: c_uint = 0xf81;
pub const LOLA_VERB_SET_CLOCK_SELECT: c_uint = 0xf84;
pub const LOLA_VERB_SET_GRANULARITY_STEPS: c_uint = 0xf86;
pub const LOLA_VERB_SET_SOURCE_GAIN: c_uint = 0xf87;
pub const LOLA_VERB_SET_MIX_GAIN: c_uint = 0xf88;
pub const LOLA_VERB_SET_DESTINATION_GAIN: c_uint = 0xf89;
pub const LOLA_VERB_SET_SRC: c_uint = 0xf8a;
// Parameter IDs used with LOLA_VERB_PARAMETERS
pub const LOLA_PAR_VENDOR_ID: c_uint = 0x00;
pub const LOLA_PAR_FUNCTION_TYPE: c_uint = 0x05;
pub const LOLA_PAR_AUDIO_WIDGET_CAP: c_uint = 0x09;
pub const LOLA_PAR_PCM: c_uint = 0x0a;
pub const LOLA_PAR_STREAM_FORMATS: c_uint = 0x0b;
pub const LOLA_PAR_PIN_CAP: c_uint = 0x0c;
pub const LOLA_PAR_AMP_IN_CAP: c_uint = 0x0d;
pub const LOLA_PAR_CONNLIST_LEN: c_uint = 0x0e;
pub const LOLA_PAR_POWER_STATE: c_uint = 0x0f;
pub const LOLA_PAR_GPIO_CAP: c_uint = 0x11;
pub const LOLA_PAR_AMP_OUT_CAP: c_uint = 0x12;
pub const LOLA_PAR_SPECIFIC_CAPS: c_uint = 0x80;
pub const LOLA_PAR_FIXED_GAIN_LIST: c_uint = 0x81;
// extract results of LOLA_PAR_SPECIFIC_CAPS

// extract results of LOLA_PAR_AMP_IN_CAP / LOLA_PAR_AMP_OUT_CAP

pub const LOLA_GRANULARITY_MIN: c_int = 8;
pub const LOLA_GRANULARITY_MAX: c_int = 32;
pub const LOLA_GRANULARITY_STEP: c_int = 8;
// parameters used with unsolicited command/response
pub const LOLA_UNSOLICITED_TAG_MASK: c_uint = 0x3f;
pub const LOLA_UNSOLICITED_TAG: c_uint = 0x1a;
pub const LOLA_UNSOLICITED_ENABLE: c_uint = 0x80;
pub const LOLA_UNSOL_RESP_TAG_OFFSET: c_int = 26;
// count values in the Vendor Specific Mixer Widget's Audio Widget Capabilities

extern "C" {
    pub fn lola_codec_flush(chip: *mut lola) -> c_int;
}

// PCM
extern "C" {
    pub fn lola_create_pcm(chip: *mut lola) -> c_int;
}
extern "C" {
    pub fn lola_init_pcm(chip: *mut lola, dir: c_int, nidp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn lola_pcm_update(chip: *mut lola, pcm: *mut lola_pcm, bits: c_uint);
}
// clock
extern "C" {
    pub fn lola_init_clock_widget(chip: *mut lola, nid: c_int) -> c_int;
}
extern "C" {
    pub fn lola_set_granularity(chip: *mut lola, val: c_uint, force: bool) -> c_int;
}
extern "C" {
    pub fn lola_enable_clock_events(chip: *mut lola) -> c_int;
}
extern "C" {
    pub fn lola_set_clock_index(chip: *mut lola, idx: c_uint) -> c_int;
}
extern "C" {
    pub fn lola_set_clock(chip: *mut lola, idx: c_int) -> c_int;
}
extern "C" {
    pub fn lola_set_sample_rate(chip: *mut lola, rate: c_int) -> c_int;
}
extern "C" {
    pub fn lola_update_ext_clock_freq(chip: *mut lola, val: c_uint) -> bool;
}
extern "C" {
    pub fn lola_sample_rate_convert(coded: c_uint) -> c_uint;
}
// mixer
extern "C" {
    pub fn lola_init_pins(chip: *mut lola, dir: c_int, nidp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn lola_init_mixer_widget(chip: *mut lola, nid: c_int) -> c_int;
}
extern "C" {
    pub fn lola_free_mixer(chip: *mut lola);
}
extern "C" {
    pub fn lola_create_mixer(chip: *mut lola) -> c_int;
}
extern "C" {
    pub fn lola_setup_all_analog_gains(chip: *mut lola, dir: c_int, mute: bool) -> c_int;
}
extern "C" {
    pub fn lola_set_src_config(chip: *mut lola, src_mask: c_uint, update: bool) -> c_int;
}
// proc

extern "C" {
    pub fn lola_proc_debug_new(chip: *mut lola);
}

// Macro flag: #define lola_proc_debug_new(chip)

