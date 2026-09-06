//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ymfpci/ymfpci.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Definitions for Yahama YMF724/740/744/754 chips
//

//
// Direct registers
//

pub const YDSXGR_INTFLAG: c_uint = 0x0004;
pub const YDSXGR_ACTIVITY: c_uint = 0x0006;
pub const YDSXGR_GLOBALCTRL: c_uint = 0x0008;
pub const YDSXGR_ZVCTRL: c_uint = 0x000A;
pub const YDSXGR_TIMERCTRL: c_uint = 0x0010;
pub const YDSXGR_TIMERCOUNT: c_uint = 0x0012;
pub const YDSXGR_SPDIFOUTCTRL: c_uint = 0x0018;
pub const YDSXGR_SPDIFOUTSTATUS: c_uint = 0x001C;
pub const YDSXGR_EEPROMCTRL: c_uint = 0x0020;
pub const YDSXGR_SPDIFINCTRL: c_uint = 0x0034;
pub const YDSXGR_SPDIFINSTATUS: c_uint = 0x0038;
pub const YDSXGR_DSPPROGRAMDL: c_uint = 0x0048;
pub const YDSXGR_DLCNTRL: c_uint = 0x004C;
pub const YDSXGR_GPIOININTFLAG: c_uint = 0x0050;
pub const YDSXGR_GPIOININTENABLE: c_uint = 0x0052;
pub const YDSXGR_GPIOINSTATUS: c_uint = 0x0054;
pub const YDSXGR_GPIOOUTCTRL: c_uint = 0x0056;
pub const YDSXGR_GPIOFUNCENABLE: c_uint = 0x0058;
pub const YDSXGR_GPIOTYPECONFIG: c_uint = 0x005A;
pub const YDSXGR_AC97CMDDATA: c_uint = 0x0060;
pub const YDSXGR_AC97CMDADR: c_uint = 0x0062;
pub const YDSXGR_PRISTATUSDATA: c_uint = 0x0064;
pub const YDSXGR_PRISTATUSADR: c_uint = 0x0066;
pub const YDSXGR_SECSTATUSDATA: c_uint = 0x0068;
pub const YDSXGR_SECSTATUSADR: c_uint = 0x006A;
pub const YDSXGR_SECCONFIG: c_uint = 0x0070;
pub const YDSXGR_LEGACYOUTVOL: c_uint = 0x0080;
pub const YDSXGR_LEGACYOUTVOLL: c_uint = 0x0080;
pub const YDSXGR_LEGACYOUTVOLR: c_uint = 0x0082;
pub const YDSXGR_NATIVEDACOUTVOL: c_uint = 0x0084;
pub const YDSXGR_NATIVEDACOUTVOLL: c_uint = 0x0084;
pub const YDSXGR_NATIVEDACOUTVOLR: c_uint = 0x0086;
pub const YDSXGR_ZVOUTVOL: c_uint = 0x0088;
pub const YDSXGR_ZVOUTVOLL: c_uint = 0x0088;
pub const YDSXGR_ZVOUTVOLR: c_uint = 0x008A;
pub const YDSXGR_SECADCOUTVOL: c_uint = 0x008C;
pub const YDSXGR_SECADCOUTVOLL: c_uint = 0x008C;
pub const YDSXGR_SECADCOUTVOLR: c_uint = 0x008E;
pub const YDSXGR_PRIADCOUTVOL: c_uint = 0x0090;
pub const YDSXGR_PRIADCOUTVOLL: c_uint = 0x0090;
pub const YDSXGR_PRIADCOUTVOLR: c_uint = 0x0092;
pub const YDSXGR_LEGACYLOOPVOL: c_uint = 0x0094;
pub const YDSXGR_LEGACYLOOPVOLL: c_uint = 0x0094;
pub const YDSXGR_LEGACYLOOPVOLR: c_uint = 0x0096;
pub const YDSXGR_NATIVEDACLOOPVOL: c_uint = 0x0098;
pub const YDSXGR_NATIVEDACLOOPVOLL: c_uint = 0x0098;
pub const YDSXGR_NATIVEDACLOOPVOLR: c_uint = 0x009A;
pub const YDSXGR_ZVLOOPVOL: c_uint = 0x009C;
pub const YDSXGR_ZVLOOPVOLL: c_uint = 0x009E;
pub const YDSXGR_ZVLOOPVOLR: c_uint = 0x009E;
pub const YDSXGR_SECADCLOOPVOL: c_uint = 0x00A0;
pub const YDSXGR_SECADCLOOPVOLL: c_uint = 0x00A0;
pub const YDSXGR_SECADCLOOPVOLR: c_uint = 0x00A2;
pub const YDSXGR_PRIADCLOOPVOL: c_uint = 0x00A4;
pub const YDSXGR_PRIADCLOOPVOLL: c_uint = 0x00A4;
pub const YDSXGR_PRIADCLOOPVOLR: c_uint = 0x00A6;
pub const YDSXGR_NATIVEADCINVOL: c_uint = 0x00A8;
pub const YDSXGR_NATIVEADCINVOLL: c_uint = 0x00A8;
pub const YDSXGR_NATIVEADCINVOLR: c_uint = 0x00AA;
pub const YDSXGR_NATIVEDACINVOL: c_uint = 0x00AC;
pub const YDSXGR_NATIVEDACINVOLL: c_uint = 0x00AC;
pub const YDSXGR_NATIVEDACINVOLR: c_uint = 0x00AE;
pub const YDSXGR_BUF441OUTVOL: c_uint = 0x00B0;
pub const YDSXGR_BUF441OUTVOLL: c_uint = 0x00B0;
pub const YDSXGR_BUF441OUTVOLR: c_uint = 0x00B2;
pub const YDSXGR_BUF441LOOPVOL: c_uint = 0x00B4;
pub const YDSXGR_BUF441LOOPVOLL: c_uint = 0x00B4;
pub const YDSXGR_BUF441LOOPVOLR: c_uint = 0x00B6;
pub const YDSXGR_SPDIFOUTVOL: c_uint = 0x00B8;
pub const YDSXGR_SPDIFOUTVOLL: c_uint = 0x00B8;
pub const YDSXGR_SPDIFOUTVOLR: c_uint = 0x00BA;
pub const YDSXGR_SPDIFLOOPVOL: c_uint = 0x00BC;
pub const YDSXGR_SPDIFLOOPVOLL: c_uint = 0x00BC;
pub const YDSXGR_SPDIFLOOPVOLR: c_uint = 0x00BE;
pub const YDSXGR_ADCSLOTSR: c_uint = 0x00C0;
pub const YDSXGR_RECSLOTSR: c_uint = 0x00C4;
pub const YDSXGR_ADCFORMAT: c_uint = 0x00C8;
pub const YDSXGR_RECFORMAT: c_uint = 0x00CC;
pub const YDSXGR_P44SLOTSR: c_uint = 0x00D0;
pub const YDSXGR_STATUS: c_uint = 0x0100;
pub const YDSXGR_CTRLSELECT: c_uint = 0x0104;
pub const YDSXGR_MODE: c_uint = 0x0108;
pub const YDSXGR_SAMPLECOUNT: c_uint = 0x010C;
pub const YDSXGR_NUMOFSAMPLES: c_uint = 0x0110;
pub const YDSXGR_CONFIG: c_uint = 0x0114;
pub const YDSXGR_PLAYCTRLSIZE: c_uint = 0x0140;
pub const YDSXGR_RECCTRLSIZE: c_uint = 0x0144;
pub const YDSXGR_EFFCTRLSIZE: c_uint = 0x0148;
pub const YDSXGR_WORKSIZE: c_uint = 0x014C;
pub const YDSXGR_MAPOFREC: c_uint = 0x0150;
pub const YDSXGR_MAPOFEFFECT: c_uint = 0x0154;
pub const YDSXGR_PLAYCTRLBASE: c_uint = 0x0158;
pub const YDSXGR_RECCTRLBASE: c_uint = 0x015C;
pub const YDSXGR_EFFCTRLBASE: c_uint = 0x0160;
pub const YDSXGR_WORKBASE: c_uint = 0x0164;
pub const YDSXGR_DSPINSTRAM: c_uint = 0x1000;
pub const YDSXGR_CTRLINSTRAM: c_uint = 0x4000;
pub const YDSXG_AC97READCMD: c_uint = 0x8000;
pub const YDSXG_AC97WRITECMD: c_uint = 0x0000;
pub const PCIR_DSXG_LEGACY: c_uint = 0x40;
pub const PCIR_DSXG_ELEGACY: c_uint = 0x42;
pub const PCIR_DSXG_CTRL: c_uint = 0x48;
pub const PCIR_DSXG_PWRCTRL1: c_uint = 0x4a;
pub const PCIR_DSXG_PWRCTRL2: c_uint = 0x4e;
pub const PCIR_DSXG_FMBASE: c_uint = 0x60;
pub const PCIR_DSXG_SBBASE: c_uint = 0x62;
pub const PCIR_DSXG_MPU401BASE: c_uint = 0x64;
pub const PCIR_DSXG_JOYBASE: c_uint = 0x66;
pub const YDSXG_DSPLENGTH: c_uint = 0x0080;
pub const YDSXG_CTRLLENGTH: c_uint = 0x3000;
pub const YDSXG_DEFAULT_WORK_SIZE: c_uint = 0x0400;
pub const YDSXG_PLAYBACK_VOICES: c_int = 64;
pub const YDSXG_CAPTURE_VOICES: c_int = 2;
pub const YDSXG_EFFECT_VOICES: c_int = 5;

// SIEN:IMOD 0:0 = legacy irq, 0:1 = INTA, 1:0 = serialized IRQ

// Macro flag: #define SUPPORT_JOYSTICK

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ymfpci_playback_bank {
    pub format: __le32,
    pub loop_default: __le32,
    pub /: *mut *mut __le32 base; / 32-bit address,
    pub /: *mut *mut __le32 loop_start; / 32-bit offset,
    pub /: *mut *mut __le32 loop_end; / 32-bit offset,
    pub /: *mut *mut __le32 loop_frac; / 8-bit fraction - loop_start,
    pub /: *mut *mut __le32 delta_end; / pitch delta end,
    pub lpfK_end: __le32,
    pub eg_gain_end: __le32,
    pub left_gain_end: __le32,
    pub right_gain_end: __le32,
    pub eff1_gain_end: __le32,
    pub eff2_gain_end: __le32,
    pub eff3_gain_end: __le32,
    pub lpfQ: __le32,
    pub status: __le32,
    pub num_of_frames: __le32,
    pub loop_count: __le32,
    pub start: __le32,
    pub start_frac: __le32,
    pub delta: __le32,
    pub lpfK: __le32,
    pub eg_gain: __le32,
    pub left_gain: __le32,
    pub right_gain: __le32,
    pub eff1_gain: __le32,
    pub eff2_gain: __le32,
    pub eff3_gain: __le32,
    pub lpfD1: __le32,
    pub lpfD2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ymfpci_capture_bank {
    pub /: *mut *mut __le32 base; / 32-bit address,
    pub /: *mut *mut __le32 loop_end; / 32-bit offset,
    pub /: *mut *mut __le32 start; / 32-bit offset,
    pub /: *mut *mut __le32 num_of_loops; / counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ymfpci_effect_bank {
    pub /: *mut *mut __le32 base; / 32-bit address,
    pub /: *mut *mut __le32 loop_end; / 32-bit offset,
    pub /: *mut *mut __le32 start; / 32-bit offset,
    pub temp: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ymfpci_voice_type {
    YMFPCI_PCM,
    YMFPCI_SYNTH,
    YMFPCI_MIDI
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ymfpci_voice {
    pub chip: *mut snd_ymfpci,
    pub number: c_int,
    pub 1: midi:,
    pub bank: *mut snd_ymfpci_playback_bank,
    pub bank_addr: dma_addr_t,
    pub voice): *mut *mut *mut void (interrupt)(struct snd_ymfpci chip, struct snd_ymfpci_voice,
    pub ypcm: *mut snd_ymfpci_pcm,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ymfpci_pcm_type {
    PLAYBACK_VOICE,
    CAPTURE_REC,
    CAPTURE_AC97,
    EFFECT_DRY_LEFT,
    EFFECT_DRY_RIGHT,
    EFFECT_EFF1,
    EFFECT_EFF2,
    EFFECT_EFF3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ymfpci_pcm {
    pub chip: *mut snd_ymfpci,
    pub type: snd_ymfpci_pcm_type,
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut *mut snd_ymfpci_voice voices[2]; / playback only,
    pub 1: swap_rear:,
    pub update_pcm_vol: c_uint,
    pub /: *mut *mut u32 period_size; / cached from runtime->period_size,
    pub /: *mut *mut u32 buffer_size; / cached from runtime->buffer_size,
    pub period_pos: u32,
    pub last_pos: u32,
    pub capture_bank_number: u32,
    pub shift: u32,
}

// spdif
// volumes
// address bases
// capture set up

// All Chips
// YMF 744/754

pub const DSXG_PCI_NUM_SAVED_LEGACY_REGS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ymfpci {
    pub irq: c_int,
    pub /: *mut *mut unsigned int device_id; / PCI device ID,
    pub /: *mut *mut unsigned char rev; / PCI revision,
    pub reg_area_phys: c_ulong,
    pub reg_area_virt: *mut void __iomem,
    pub old_legacy_ctrl: u16,

    pub gameport: *mut gameport,

    pub work_ptr: *mut snd_dma_buffer,
    pub bank_size_playback: c_uint,
    pub bank_size_capture: c_uint,
    pub bank_size_effect: c_uint,
    pub work_size: c_uint,
    pub bank_base_playback: *mut c_void,
    pub bank_base_capture: *mut c_void,
    pub bank_base_effect: *mut c_void,
    pub work_base: *mut c_void,
    pub bank_base_playback_addr: dma_addr_t,
    pub bank_base_capture_addr: dma_addr_t,
    pub bank_base_effect_addr: dma_addr_t,
    pub work_base_addr: dma_addr_t,
    pub ac3_tmp_base: snd_dma_buffer,
    pub ctrl_playback: *mut __le32,
    pub bank_playback: [*mut snd_ymfpci_playback_bank; YDSXG_PLAYBACK_VOICES][2],
    pub bank_capture: [*mut snd_ymfpci_capture_bank; YDSXG_CAPTURE_VOICES][2],
    pub bank_effect: [*mut snd_ymfpci_effect_bank; YDSXG_EFFECT_VOICES][2],
    pub start_count: c_int,
    pub active_bank: u32,
    pub voices: [snd_ymfpci_voice; 64],
    pub src441_used: c_int,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub rawmidi: *mut snd_rawmidi,
    pub timer: *mut snd_timer,
    pub timer_ticks: c_uint,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub pcm2: *mut snd_pcm,
    pub pcm_spdif: *mut snd_pcm,
    pub pcm_4ch: *mut snd_pcm,
    pub capture_substream: [*mut snd_pcm_substream; YDSXG_CAPTURE_VOICES],
    pub effect_substream: [*mut snd_pcm_substream; YDSXG_EFFECT_VOICES],
    pub ctl_vol_recsrc: *mut snd_kcontrol,
    pub ctl_vol_adcrec: *mut snd_kcontrol,
    pub ctl_vol_spdifrec: *mut snd_kcontrol,
    pub spdif_pcm_bits: unsigned short spdif_bits,,
    pub spdif_pcm_ctl: *mut snd_kcontrol,
    pub mode_dup4ch: c_int,
    pub rear_opened: c_int,
    pub spdif_opened: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ymfpci_pcm_mixer {
    pub left: u16,
    pub right: u16,
    pub ctl: *mut snd_kcontrol,
    pub pcm_mixer: [}; 32],
    pub reg_lock: spinlock_t,
    pub voice_lock: spinlock_t,
    pub interrupt_sleep: wait_queue_head_t,
    pub interrupt_sleep_count: core::sync::atomic::AtomicI32,
    pub proc_entry: *mut snd_info_entry,
    pub dsp_microcode: *const firmware,
    pub controller_microcode: *const firmware,
    pub saved_regs: [u32; YDSXGR_NUM_SAVED_REGS],
    pub saved_ydsxgr_mode: u32,
    pub saved_dsxg_pci_regs: [u16; DSXG_PCI_NUM_SAVED_REGS],
}

extern "C" {
    pub fn snd_ymfpci_free_gameport(chip: *mut snd_ymfpci);
}
extern "C" {
    pub fn snd_ymfpci_pcm(chip: *mut snd_ymfpci, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ymfpci_pcm2(chip: *mut snd_ymfpci, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ymfpci_pcm_spdif(chip: *mut snd_ymfpci, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ymfpci_pcm_4ch(chip: *mut snd_ymfpci, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ymfpci_mixer(chip: *mut snd_ymfpci, rear_switch: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ymfpci_timer(chip: *mut snd_ymfpci, device: c_int) -> c_int;
}
