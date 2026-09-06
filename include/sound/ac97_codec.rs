//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ac97_codec.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Universal interface for Audio Codec '97
//
// For more details look to AC '97 component specification revision 2.1
// by Intel Corporation (http://developer.intel.com).
//

// maximum number of devices on the AC97 bus
pub const AC97_BUS_MAX_DEVICES: c_int = 4;
// specific - SigmaTel
pub const AC97_SIGMATEL_OUTSEL: c_uint = 0x64	/* Output Select, STAC9758 */;
pub const AC97_SIGMATEL_INSEL: c_uint = 0x66	/* Input Select, STAC9758 */;
pub const AC97_SIGMATEL_IOMISC: c_uint = 0x68	/* STAC9758 */;
pub const AC97_SIGMATEL_ANALOG: c_uint = 0x6c	/* Analog Special */;
pub const AC97_SIGMATEL_DAC2INVERT: c_uint = 0x6e;
pub const AC97_SIGMATEL_BIAS1: c_uint = 0x70;
pub const AC97_SIGMATEL_BIAS2: c_uint = 0x72;
pub const AC97_SIGMATEL_VARIOUS: c_uint = 0x72	/* STAC9758 */;
pub const AC97_SIGMATEL_MULTICHN: c_uint = 0x74	/* Multi-Channel programming */;
pub const AC97_SIGMATEL_CIC1: c_uint = 0x76;
pub const AC97_SIGMATEL_CIC2: c_uint = 0x78;
// specific - Analog Devices
pub const AC97_AD_TEST: c_uint = 0x5a	/* test register */;
pub const AC97_AD_TEST2: c_uint = 0x5c	/* undocumented test register 2 */;

pub const AC97_AD_CODEC_CFG: c_uint = 0x70	/* codec configuration */;
pub const AC97_AD_JACK_SPDIF: c_uint = 0x72	/* Jack Sense & S/PDIF */;
pub const AC97_AD_SERIAL_CFG: c_uint = 0x74	/* Serial Configuration */;
pub const AC97_AD_MISC: c_uint = 0x76	/* Misc Control Bits */;

// specific - Cirrus Logic
pub const AC97_CSR_ACMODE: c_uint = 0x5e	/* AC Mode Register */;
pub const AC97_CSR_MISC_CRYSTAL: c_uint = 0x60	/* Misc Crystal Control */;
pub const AC97_CSR_SPDIF: c_uint = 0x68	/* S/PDIF Register */;
pub const AC97_CSR_SERIAL: c_uint = 0x6a	/* Serial Port Control */;
pub const AC97_CSR_SPECF_ADDR: c_uint = 0x6c	/* Special Feature Address */;
pub const AC97_CSR_SPECF_DATA: c_uint = 0x6e	/* Special Feature Data */;
pub const AC97_CSR_BDI_STATUS: c_uint = 0x7a	/* BDI Status */;
// specific - Conexant
pub const AC97_CXR_AUDIO_MISC: c_uint = 0x5c;

pub const AC97_CXR_SPDIF_PCM: c_uint = 0x0;
pub const AC97_CXR_SPDIF_AC3: c_uint = 0x2;
// specific - ALC
pub const AC97_ALC650_SPDIF_INPUT_STATUS1: c_uint = 0x60;
// S/PDIF input status 1 bit defines
pub const AC97_ALC650_PRO: c_uint = 0x0001  /* Professional status */;
pub const AC97_ALC650_NAUDIO: c_uint = 0x0002  /* Non audio stream */;
pub const AC97_ALC650_COPY: c_uint = 0x0004  /* Copyright status */;
pub const AC97_ALC650_PRE: c_uint = 0x0038  /* Preemphasis status */;
pub const AC97_ALC650_PRE_SHIFT: c_int = 3;
pub const AC97_ALC650_MODE: c_uint = 0x00C0  /* Preemphasis status */;
pub const AC97_ALC650_MODE_SHIFT: c_int = 6;
pub const AC97_ALC650_CC_MASK: c_uint = 0x7f00  /* Category Code mask */;
pub const AC97_ALC650_CC_SHIFT: c_int = 8;
pub const AC97_ALC650_L: c_uint = 0x8000  /* Generation Level status */;
pub const AC97_ALC650_SPDIF_INPUT_STATUS2: c_uint = 0x62;
// S/PDIF input status 2 bit defines
pub const AC97_ALC650_SOUCE_MASK: c_uint = 0x000f  /* Source number */;
pub const AC97_ALC650_CHANNEL_MASK: c_uint = 0x00f0  /* Channel number */;
pub const AC97_ALC650_CHANNEL_SHIFT: c_int = 4;
pub const AC97_ALC650_SPSR_MASK: c_uint = 0x0f00  /* S/PDIF Sample Rate bits */;
pub const AC97_ALC650_SPSR_SHIFT: c_int = 8;
pub const AC97_ALC650_SPSR_44K: c_uint = 0x0000  /* Use 44.1kHz Sample rate */;
pub const AC97_ALC650_SPSR_48K: c_uint = 0x0200  /* Use 48kHz Sample rate */;
pub const AC97_ALC650_SPSR_32K: c_uint = 0x0300  /* Use 32kHz Sample rate */;
pub const AC97_ALC650_CLOCK_ACCURACY: c_uint = 0x3000  /* Clock accuracy */;
pub const AC97_ALC650_CLOCK_SHIFT: c_int = 12;
pub const AC97_ALC650_CLOCK_LOCK: c_uint = 0x4000  /* Clock locked status */;
pub const AC97_ALC650_V: c_uint = 0x8000  /* Validity status */;
pub const AC97_ALC650_SURR_DAC_VOL: c_uint = 0x64;
pub const AC97_ALC650_LFE_DAC_VOL: c_uint = 0x66;
pub const AC97_ALC650_UNKNOWN1: c_uint = 0x68;
pub const AC97_ALC650_MULTICH: c_uint = 0x6a;
pub const AC97_ALC650_UNKNOWN2: c_uint = 0x6c;
pub const AC97_ALC650_REVISION: c_uint = 0x6e;
pub const AC97_ALC650_UNKNOWN3: c_uint = 0x70;
pub const AC97_ALC650_UNKNOWN4: c_uint = 0x72;
pub const AC97_ALC650_MISC: c_uint = 0x74;
pub const AC97_ALC650_GPIO_SETUP: c_uint = 0x76;
pub const AC97_ALC650_GPIO_STATUS: c_uint = 0x78;
pub const AC97_ALC650_CLOCK: c_uint = 0x7a;
// specific - Yamaha YMF7x3
pub const AC97_YMF7X3_DIT_CTRL: c_uint = 0x66	/* DIT Control (YMF743) / 2 (YMF753) */;
pub const AC97_YMF7X3_3D_MODE_SEL: c_uint = 0x68	/* 3D Mode Select */;
// specific - C-Media
pub const AC97_CM9738_VENDOR_CTRL: c_uint = 0x5a;
pub const AC97_CM9739_MULTI_CHAN: c_uint = 0x64;
pub const AC97_CM9739_SPDIF_IN_STATUS: c_uint = 0x68 /* 32bit */;
pub const AC97_CM9739_SPDIF_CTRL: c_uint = 0x6c;
// specific - wolfson
pub const AC97_WM97XX_FMIXER_VOL: c_uint = 0x72;
pub const AC97_WM9704_RMIXER_VOL: c_uint = 0x74;
pub const AC97_WM9704_TEST: c_uint = 0x5a;
pub const AC97_WM9704_RPCM_VOL: c_uint = 0x70;
pub const AC97_WM9711_OUT3VOL: c_uint = 0x16;
// ac97->scaps

// ac97->flags

// rates indexes
pub const AC97_RATES_FRONT_DAC: c_int = 0;
pub const AC97_RATES_SURR_DAC: c_int = 1;
pub const AC97_RATES_LFE_DAC: c_int = 2;
pub const AC97_RATES_ADC: c_int = 3;
pub const AC97_RATES_MIC_ADC: c_int = 4;
pub const AC97_RATES_SPDIF: c_int = 5;
pub const AC97_NUM_GPIOS: c_int = 16;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ac97_build_ops {
    pub ac97): *mut *mut int (build_3d) (struct snd_ac97,
    pub ac97): *mut *mut int (build_specific) (struct snd_ac97,
    pub ac97): *mut *mut int (build_spdif) (struct snd_ac97,
    pub ac97): *mut *mut int (build_post_spdif) (struct snd_ac97,

    pub ac97): *mut *mut void (suspend) (struct snd_ac97,
    pub ac97): *mut *mut void (resume) (struct snd_ac97,

    pub /: *mut *mut *mut *mut void (update_jacks) (struct snd_ac97 ac97); / for jack-sharing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ac97_bus_ops {
    pub ac97): *mut *mut void (reset) (struct snd_ac97,
    pub ac97): *mut *mut void (warm_reset)(struct snd_ac97,
    pub val): *mut *mut *mut void (write) (struct snd_ac97 ac97, unsigned short reg, unsigned short,
    pub reg): *mut *mut *mut unsigned short (read) (struct snd_ac97 ac97, unsigned short,
    pub ac97): *mut *mut void (wait) (struct snd_ac97,
    pub ac97): *mut *mut void (init) (struct snd_ac97,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ac97_bus {
// -- lowlevel (hardware) driver specific --
    pub ops: *const snd_ac97_bus_ops,
    pub private_data: *mut c_void,
    pub bus): *mut *mut void (private_free) (struct snd_ac97_bus,
// ---
    pub card: *mut snd_card,
    pub /: *mut *mut unsigned short num; / bus number,
    pub /: *mut *mut isdin: 1;/ independent SDIN,
    pub /: *mut *mut unsigned int clock; / AC'97 base clock (usually 48000Hz),
    pub /: *mut *mut spinlock_t bus_lock; / used mainly for slot allocation,
    pub /: *mut *mut unsigned short used_slots[2][4]; / actually used PCM slots,
    pub /: *mut *mut unsigned short pcms_count; / count of PCMs,
    pub pcms: *mut ac97_pcm,
    pub codec: [*mut snd_ac97; 4],
    pub proc: *mut snd_info_entry,
}

// static resolution table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ac97_res_table {
    pub /: *mut *mut unsigned short reg; / register,
    pub /: *mut *mut unsigned short bits; / resolution bitmask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub ac97): *mut *mut void (private_free) (struct snd_ac97,
    pub /: *mut *mut *mut pci_dev pci; / assigned PCI device - used for quirks,
    pub /: *mut *mut unsigned short num; / number of codec: 0 = primary, 1 = secondary,
    pub /: *mut *mut unsigned short addr; / physical address of codec [0-3],
    pub /: *mut *mut unsigned int scaps; / driver capabilities,
    pub /: *const *const *const snd_ac97_res_table res_table; / static resolution,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ac97 {
// -- lowlevel (hardware) driver specific --
    pub build_ops: *const snd_ac97_build_ops,
    pub private_data: *mut c_void,
    pub ac97): *mut *mut void (private_free) (struct snd_ac97,
// ---
    pub bus: *mut snd_ac97_bus,
    pub /: *mut *mut *mut pci_dev pci; / assigned PCI device - used for quirks,
    pub proc: *mut snd_info_entry,
    pub proc_regs: *mut snd_info_entry,
    pub subsystem_vendor: c_ushort,
    pub subsystem_device: c_ushort,
    pub reg_mutex: mutex,
    pub /: *mut *mut mutex page_mutex; / mutex for AD18xx multi-codecs and paging (2.3),
    pub /: *mut *mut unsigned short num; / number of codec: 0 = primary, 1 = secondary,
    pub /: *mut *mut unsigned short addr; / physical address of codec [0-3],
    pub /: *mut *mut unsigned int id; / identification of codec,
    pub /: *mut *mut unsigned short caps; / capabilities (register 0),
    pub /: *mut *mut unsigned short ext_id; / extended feature identification (register 28),
    pub /: *mut *mut unsigned short ext_mid; / extended modem ID (register 3C),
    pub /: *const *const *const snd_ac97_res_table res_table; / static resolution,
    pub /: *mut *mut unsigned int scaps; / driver capabilities,
    pub /: *mut *mut unsigned int flags; / specific code,
    pub /: *mut *mut *mut unsigned int rates[6]; / see AC97_RATES_ defines,
    pub spdif_status: c_uint,
    pub /: *mut *mut unsigned short regs[0x80]; / register cache,
    pub /: *mut *mut DECLARE_BITMAP(reg_accessed, 0x80); / bit flags,
    pub C69: unsigned short unchained[3]; // 0 = C34, 1 = C79, 2 =,
    pub C69: unsigned short chained[3]; // 0 = C34, 1 = C79, 2 =,
    pub word): unsigned short id[3]; // codec IDs (lower 16-bit,
    pub registers: unsigned short pcmreg[3]; // PCM,
    pub bits: unsigned short codec_cfg[3]; // CODEC_CFG,
    pub only: unsigned char swap_mic_linein; // AD1986/AD1986A,
    pub /: *mut *mut unsigned char lo_as_master; / LO as master,
    pub ad18xx: },
    pub /: *mut *mut unsigned int dev_flags; / device specific,
    pub spec: },
// jack-sharing info
    pub indep_surround: c_uchar,
    pub channel_mode: c_uchar,

    pub /: *mut *mut unsigned int power_up; / power states,
    pub power_work: delayed_work,

    pub dev: device,
    pub gpio_priv: *mut snd_ac97_gpio_priv,
    pub /: *mut *mut *mut snd_pcm_chmap chmaps[2]; / channel-maps (optional),
}

// conditions
// functions
// create new AC97 bus
// create mixer controls
extern "C" {
    pub fn snd_ac97_write(ac97: *mut snd_ac97, reg: c_ushort, value: c_ushort);
}
extern "C" {
    pub fn snd_ac97_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn snd_ac97_write_cache(ac97: *mut snd_ac97, reg: c_ushort, value: c_ushort);
}
extern "C" {
    pub fn snd_ac97_update(ac97: *mut snd_ac97, reg: c_ushort, value: c_ushort) -> c_int;
}
extern "C" {
    pub fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: c_ushort, mask: c_ushort, value: c_ushort) -> c_int;
}

extern "C" {
    pub fn snd_ac97_update_power(ac97: *mut snd_ac97, reg: c_int, powerup: c_int) -> c_int;
}

extern "C" {
    pub fn snd_ac97_suspend(ac97: *mut snd_ac97);
}
extern "C" {
    pub fn snd_ac97_resume(ac97: *mut snd_ac97);
}

// quirk types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_quirk {
    pub /: *mut *mut unsigned short subvendor; / PCI subsystem vendor id,
    pub /: *mut *mut unsigned short subdevice; / PCI subsystem device id,
    pub /: *mut *mut unsigned short mask; / device id bit mask, 0 = accept all,
    pub /: *mut *mut unsigned int codec_id; / codec id (if any), 0 = accept all,
    pub /: *const *const *const char name; / name shown as info,
    pub /: *mut *mut int type; / quirk type above,
}

extern "C" {
    pub fn snd_ac97_set_rate(ac97: *mut snd_ac97, reg: c_int, rate: c_uint) -> c_int;
}
//
// PCM allocation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ac97_pcm_cfg {
    AC97_PCM_CFG_FRONT = 2,
    AC97_PCM_CFG_REAR = 10,		/* alias surround */
    AC97_PCM_CFG_LFE = 11,		/* center + lfe */
    AC97_PCM_CFG_40 = 4,		/* front + rear */
    AC97_PCM_CFG_51 = 6,		/* front + rear + center/lfe */
    AC97_PCM_CFG_SPDIF = 20
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_pcm {
    pub bus: *mut snd_ac97_bus,
    pub /: *mut *mut spdif: 1; / spdif pcm,
    pub /: *mut *mut unsigned short aslots; / active slots,
    pub /: *mut *mut unsigned short cur_dbl; / current double-rate state,
    pub /: *mut *mut unsigned int rates; / available rates,
    pub /: *mut *mut unsigned short slots; / driver input: requested AC97 slot numbers,
    pub /: *mut *mut unsigned short rslots[4]; / allocated slots per codecs,
    pub rate_table: [c_uchar; 4],
    pub /: *mut *mut *mut snd_ac97 codec[4]; / allocated codecs,
    pub /: *mut *mut } r[2]; / 0 = standard rates, 1 = double rates,
    pub /: *mut *mut unsigned long private_value; / used by the hardware driver,
}

extern "C" {
    pub fn snd_ac97_pcm_close(pcm: *mut ac97_pcm) -> c_int;
}
extern "C" {
    pub fn snd_ac97_pcm_double_rate_rules(runtime: *mut snd_pcm_runtime) -> c_int;
}
// ad hoc AC97 device driver access
// AC97 platform_data adding function
