//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/oxygen.h
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


// SPDX-License-Identifier: GPL-2.0

// Macro flag: #define OXYGEN_H_INCLUDED

// 1 << PCM_x == OXYGEN_CHANNEL_x
pub const PCM_A: c_int = 0;
pub const PCM_B: c_int = 1;
pub const PCM_C: c_int = 2;
pub const PCM_SPDIF: c_int = 3;
pub const PCM_MULTICH: c_int = 4;
pub const PCM_AC97: c_int = 5;
pub const PCM_COUNT: c_int = 6;

pub const OXYGEN_IO_SIZE: c_uint = 0x100;
pub const OXYGEN_EEPROM_ID: c_uint = 0x434d	/* "CM" */;
// model-specific configuration of outputs/inputs
pub const PLAYBACK_0_TO_I2S: c_uint = 0x0001;
// PLAYBACK_0_TO_AC97_0		not implemented
pub const PLAYBACK_1_TO_SPDIF: c_uint = 0x0004;
pub const PLAYBACK_2_TO_AC97_1: c_uint = 0x0008;
pub const CAPTURE_0_FROM_I2S_1: c_uint = 0x0010;
pub const CAPTURE_0_FROM_I2S_2: c_uint = 0x0020;
// CAPTURE_0_FROM_AC97_0		not implemented
pub const CAPTURE_1_FROM_SPDIF: c_uint = 0x0080;
pub const CAPTURE_2_FROM_I2S_2: c_uint = 0x0100;
pub const CAPTURE_2_FROM_AC97_1: c_uint = 0x0200;
pub const CAPTURE_3_FROM_I2S_3: c_uint = 0x0400;
pub const MIDI_OUTPUT: c_uint = 0x0800;
pub const MIDI_INPUT: c_uint = 0x1000;
pub const AC97_CD_INPUT: c_uint = 0x2000;
pub const AC97_FMIC_SWITCH: c_uint = 0x4000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oxygen_model {
    pub shortname: *const c_char,
    pub longname: *const c_char,
    pub chip: *const c_char,
    pub chip): *mut *mut void (init)(struct oxygen,
    pub template): *mut *mut int (control_filter)(struct snd_kcontrol_new,
    pub chip): *mut *mut int (mixer_init)(struct oxygen,
    pub chip): *mut *mut void (cleanup)(struct oxygen,
    pub chip): *mut *mut void (suspend)(struct oxygen,
    pub chip): *mut *mut void (resume)(struct oxygen,
    pub hardware): *mut snd_pcm_hardware,
    pub params): *mut snd_pcm_hw_params,
    pub params): *mut snd_pcm_hw_params,
    pub chip): *mut *mut void (update_dac_volume)(struct oxygen,
    pub chip): *mut *mut void (update_dac_mute)(struct oxygen,
    pub mixed): *mut *mut *mut void (update_center_lfe_mix)(struct oxygen chip, bool,
    pub play_routing): c_uint,
    pub chip): *mut *mut void (gpio_changed)(struct oxygen,
    pub chip): *mut *mut void (uart_input)(struct oxygen,
    pub mute): unsigned int reg, unsigned int,
    pub buffer): *mut snd_info_buffer,
    pub dac_tlv: *const c_uint,
    pub model_data_size: usize,
    pub device_config: c_uint,
    pub dac_channels_pcm: u8,
    pub dac_channels_mixer: u8,
    pub dac_volume_min: u8,
    pub dac_volume_max: u8,
    pub misc_flags: u8,
    pub function_flags: u8,
    pub dac_mclks: u8,
    pub adc_mclks: u8,
    pub dac_i2s_format: u16,
    pub adc_i2s_format: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oxygen {
    pub addr: c_ulong,
    pub reg_lock: spinlock_t,
    pub mutex: mutex,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub midi: *mut snd_rawmidi,
    pub irq: c_int,
    pub model_data: *mut c_void,
    pub interrupt_mask: c_uint,
    pub dac_volume: [u8; 8],
    pub dac_mute: u8,
    pub pcm_active: u8,
    pub pcm_running: u8,
    pub dac_routing: u8,
    pub spdif_playback_enable: u8,
    pub has_ac97_0: u8,
    pub has_ac97_1: u8,
    pub spdif_bits: u32,
    pub spdif_pcm_bits: u32,
    pub streams: [*mut snd_pcm_substream; PCM_COUNT],
    pub controls: [*mut snd_kcontrol; CONTROL_COUNT],
    pub spdif_input_bits_work: work_struct,
    pub gpio_work: work_struct,
    pub ac97_waitqueue: wait_queue_head_t,
    pub _8: [u8; OXYGEN_IO_SIZE],
    pub 2]: __le16 _16[OXYGEN_IO_SIZE /,
    pub 4]: __le32 _32[OXYGEN_IO_SIZE /,
    pub saved_registers: },
    pub saved_ac97_registers: [u16; 2][0x40],
    pub uart_input_count: c_uint,
    pub uart_input: [u8; 32],
    pub model: oxygen_model,
}

// oxygen_lib.c
extern "C" {
    pub fn oxygen_pci_shutdown(pci: *mut pci_dev);
}
// oxygen_mixer.c
extern "C" {
    pub fn oxygen_mixer_init(chip: *mut oxygen) -> c_int;
}
extern "C" {
    pub fn oxygen_update_dac_routing(chip: *mut oxygen);
}
extern "C" {
    pub fn oxygen_update_spdif_source(chip: *mut oxygen);
}
// oxygen_pcm.c
extern "C" {
    pub fn oxygen_pcm_init(chip: *mut oxygen) -> c_int;
}
// oxygen_io.c
extern "C" {
    pub fn oxygen_read8(chip: *mut oxygen, reg: c_uint) -> u8;
}
extern "C" {
    pub fn oxygen_read16(chip: *mut oxygen, reg: c_uint) -> u16;
}
extern "C" {
    pub fn oxygen_read32(chip: *mut oxygen, reg: c_uint) -> u32;
}
extern "C" {
    pub fn oxygen_write8(chip: *mut oxygen, reg: c_uint, value: u8);
}
extern "C" {
    pub fn oxygen_write16(chip: *mut oxygen, reg: c_uint, value: u16);
}
extern "C" {
    pub fn oxygen_write32(chip: *mut oxygen, reg: c_uint, value: u32);
}
extern "C" {
    pub fn oxygen_write_spi(chip: *mut oxygen, control: u8, data: c_uint) -> c_int;
}
extern "C" {
    pub fn oxygen_write_i2c(chip: *mut oxygen, device: u8, map: u8, data: u8);
}
extern "C" {
    pub fn oxygen_reset_uart(chip: *mut oxygen);
}
extern "C" {
    pub fn oxygen_write_uart(chip: *mut oxygen, data: u8);
}
extern "C" {
    pub fn oxygen_read_eeprom(chip: *mut oxygen, index: c_uint) -> u16;
}
extern "C" {
    pub fn oxygen_write_eeprom(chip: *mut oxygen, index: c_uint, value: u16);
}
