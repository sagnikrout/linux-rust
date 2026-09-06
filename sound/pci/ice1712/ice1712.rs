//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/ice1712.h
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
// ALSA driver for ICEnsemble ICE1712 (Envy24)
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
//

//
// Direct registers
//

pub const ICE1712_REG_CONTROL: c_uint = 0x00	/* byte */;
pub const ICE1712_RESET: c_uint = 0x80	/* soft reset whole chip */;
pub const ICE1712_SERR_ASSERT_DS_DMA: c_uint = 0x40    /* disabled SERR# assertion for the DS DMA Ch-C irq otherwise enabled */;
pub const ICE1712_DOS_VOL: c_uint = 0x10    /* DOS WT/FM volume control */;
pub const ICE1712_SERR_LEVEL: c_uint = 0x08	/* SERR# level otherwise edge */;
pub const ICE1712_SERR_ASSERT_SB: c_uint = 0x02	/* disabled SERR# assertion for SB irq otherwise enabled */;
pub const ICE1712_NATIVE: c_uint = 0x01	/* native mode otherwise SB */;
pub const ICE1712_REG_IRQMASK: c_uint = 0x01	/* byte */;
pub const ICE1712_IRQ_MPU1: c_uint = 0x80	/* MIDI irq mask */;
pub const ICE1712_IRQ_TIMER: c_uint = 0x40	/* Timer mask */;
pub const ICE1712_IRQ_MPU2: c_uint = 0x20	/* Secondary MIDI irq mask */;
pub const ICE1712_IRQ_PROPCM: c_uint = 0x10	/* professional multi-track */;
pub const ICE1712_IRQ_FM: c_uint = 0x08	/* FM/MIDI - legacy */;
pub const ICE1712_IRQ_PBKDS: c_uint = 0x04	/* playback DS channels */;
pub const ICE1712_IRQ_CONCAP: c_uint = 0x02	/* consumer capture */;
pub const ICE1712_IRQ_CONPBK: c_uint = 0x01	/* consumer playback */;
pub const ICE1712_REG_IRQSTAT: c_uint = 0x02	/* byte */;
// look to ICE1712_IRQ_*
pub const ICE1712_REG_INDEX: c_uint = 0x03	/* byte - indirect CCIxx regs */;
pub const ICE1712_REG_DATA: c_uint = 0x04	/* byte - indirect CCIxx regs */;
pub const ICE1712_REG_NMI_STAT1: c_uint = 0x05	/* byte */;
pub const ICE1712_REG_NMI_DATA: c_uint = 0x06	/* byte */;
pub const ICE1712_REG_NMI_INDEX: c_uint = 0x07	/* byte */;
pub const ICE1712_REG_AC97_INDEX: c_uint = 0x08	/* byte */;
pub const ICE1712_REG_AC97_CMD: c_uint = 0x09	/* byte */;
pub const ICE1712_AC97_COLD: c_uint = 0x80	/* cold reset */;
pub const ICE1712_AC97_WARM: c_uint = 0x40	/* warm reset */;
pub const ICE1712_AC97_WRITE: c_uint = 0x20	/* W: write, R: write in progress */;
pub const ICE1712_AC97_READ: c_uint = 0x10	/* W: read, R: read in progress */;
pub const ICE1712_AC97_READY: c_uint = 0x08	/* codec ready status bit */;
pub const ICE1712_AC97_PBK_VSR: c_uint = 0x02	/* playback VSR */;
pub const ICE1712_AC97_CAP_VSR: c_uint = 0x01	/* capture VSR */;
pub const ICE1712_REG_AC97_DATA: c_uint = 0x0a	/* word (little endian) */;
pub const ICE1712_REG_MPU1_CTRL: c_uint = 0x0c	/* byte */;
pub const ICE1712_REG_MPU1_DATA: c_uint = 0x0d	/* byte */;
pub const ICE1712_REG_I2C_DEV_ADDR: c_uint = 0x10	/* byte */;
pub const ICE1712_I2C_WRITE: c_uint = 0x01	/* write direction */;
pub const ICE1712_REG_I2C_BYTE_ADDR: c_uint = 0x11	/* byte */;
pub const ICE1712_REG_I2C_DATA: c_uint = 0x12	/* byte */;
pub const ICE1712_REG_I2C_CTRL: c_uint = 0x13	/* byte */;
pub const ICE1712_I2C_EEPROM: c_uint = 0x80	/* EEPROM exists */;
pub const ICE1712_I2C_BUSY: c_uint = 0x01	/* busy bit */;
pub const ICE1712_REG_CONCAP_ADDR: c_uint = 0x14	/* dword - consumer capture */;
pub const ICE1712_REG_CONCAP_COUNT: c_uint = 0x18	/* word - current/base count */;
pub const ICE1712_REG_SERR_SHADOW: c_uint = 0x1b	/* byte */;
pub const ICE1712_REG_MPU2_CTRL: c_uint = 0x1c	/* byte */;
pub const ICE1712_REG_MPU2_DATA: c_uint = 0x1d	/* byte */;
pub const ICE1712_REG_TIMER: c_uint = 0x1e	/* word */;
//
// Indirect registers
//
pub const ICE1712_IREG_PBK_COUNT_LO: c_uint = 0x00;
pub const ICE1712_IREG_PBK_COUNT_HI: c_uint = 0x01;
pub const ICE1712_IREG_PBK_CTRL: c_uint = 0x02;
pub const ICE1712_IREG_PBK_LEFT: c_uint = 0x03	/* left volume */;
pub const ICE1712_IREG_PBK_RIGHT: c_uint = 0x04	/* right volume */;
pub const ICE1712_IREG_PBK_SOFT: c_uint = 0x05	/* soft volume */;
pub const ICE1712_IREG_PBK_RATE_LO: c_uint = 0x06;
pub const ICE1712_IREG_PBK_RATE_MID: c_uint = 0x07;
pub const ICE1712_IREG_PBK_RATE_HI: c_uint = 0x08;
pub const ICE1712_IREG_CAP_COUNT_LO: c_uint = 0x10;
pub const ICE1712_IREG_CAP_COUNT_HI: c_uint = 0x11;
pub const ICE1712_IREG_CAP_CTRL: c_uint = 0x12;
pub const ICE1712_IREG_GPIO_DATA: c_uint = 0x20;
pub const ICE1712_IREG_GPIO_WRITE_MASK: c_uint = 0x21;
pub const ICE1712_IREG_GPIO_DIRECTION: c_uint = 0x22;
pub const ICE1712_IREG_CONSUMER_POWERDOWN: c_uint = 0x30;
pub const ICE1712_IREG_PRO_POWERDOWN: c_uint = 0x31;
//
// Consumer section direct DMA registers
//

pub const ICE1712_DS_INTMASK: c_uint = 0x00	/* word - interrupt mask */;
pub const ICE1712_DS_INTSTAT: c_uint = 0x02	/* word - interrupt status */;
pub const ICE1712_DS_DATA: c_uint = 0x04	/* dword - channel data */;
pub const ICE1712_DS_INDEX: c_uint = 0x08	/* dword - channel index */;
//
// Consumer section channel registers
//
pub const ICE1712_DSC_ADDR0: c_uint = 0x00	/* dword - base address 0 */;
pub const ICE1712_DSC_COUNT0: c_uint = 0x01	/* word - count 0 */;
pub const ICE1712_DSC_ADDR1: c_uint = 0x02	/* dword - base address 1 */;
pub const ICE1712_DSC_COUNT1: c_uint = 0x03	/* word - count 1 */;
pub const ICE1712_DSC_CONTROL: c_uint = 0x04	/* byte - control & status */;
pub const ICE1712_BUFFER1: c_uint = 0x80	/* buffer1 is active */;
pub const ICE1712_BUFFER1_AUTO: c_uint = 0x40	/* buffer1 auto init */;
pub const ICE1712_BUFFER0_AUTO: c_uint = 0x20	/* buffer0 auto init */;
pub const ICE1712_FLUSH: c_uint = 0x10	/* flush FIFO */;
pub const ICE1712_STEREO: c_uint = 0x08	/* stereo */;
pub const ICE1712_16BIT: c_uint = 0x04	/* 16-bit data */;
pub const ICE1712_PAUSE: c_uint = 0x02	/* pause */;
pub const ICE1712_START: c_uint = 0x01	/* start */;
pub const ICE1712_DSC_RATE: c_uint = 0x05	/* dword - rate */;
pub const ICE1712_DSC_VOLUME: c_uint = 0x06	/* word - volume control */;
//
// Professional multi-track direct control registers
//

pub const ICE1712_MT_IRQ: c_uint = 0x00	/* byte - interrupt mask */;
pub const ICE1712_MULTI_CAPTURE: c_uint = 0x80	/* capture IRQ */;
pub const ICE1712_MULTI_PLAYBACK: c_uint = 0x40	/* playback IRQ */;
pub const ICE1712_MULTI_CAPSTATUS: c_uint = 0x02	/* capture IRQ status */;
pub const ICE1712_MULTI_PBKSTATUS: c_uint = 0x01	/* playback IRQ status */;
pub const ICE1712_MT_RATE: c_uint = 0x01	/* byte - sampling rate select */;
pub const ICE1712_SPDIF_MASTER: c_uint = 0x10	/* S/PDIF input is master clock */;
pub const ICE1712_MT_I2S_FORMAT: c_uint = 0x02	/* byte - I2S data format */;
pub const ICE1712_MT_AC97_INDEX: c_uint = 0x04	/* byte - AC'97 index */;
pub const ICE1712_MT_AC97_CMD: c_uint = 0x05	/* byte - AC'97 command & status */;
// look to ICE1712_AC97_*
pub const ICE1712_MT_AC97_DATA: c_uint = 0x06	/* word - AC'97 data */;
pub const ICE1712_MT_PLAYBACK_ADDR: c_uint = 0x10	/* dword - playback address */;
pub const ICE1712_MT_PLAYBACK_SIZE: c_uint = 0x14	/* word - playback size */;
pub const ICE1712_MT_PLAYBACK_COUNT: c_uint = 0x16	/* word - playback count */;
pub const ICE1712_MT_PLAYBACK_CONTROL: c_uint = 0x18	/* byte - control */;
pub const ICE1712_CAPTURE_START_SHADOW: c_uint = 0x04	/* capture start */;
pub const ICE1712_PLAYBACK_PAUSE: c_uint = 0x02	/* playback pause */;
pub const ICE1712_PLAYBACK_START: c_uint = 0x01	/* playback start */;
pub const ICE1712_MT_CAPTURE_ADDR: c_uint = 0x20	/* dword - capture address */;
pub const ICE1712_MT_CAPTURE_SIZE: c_uint = 0x24	/* word - capture size */;
pub const ICE1712_MT_CAPTURE_COUNT: c_uint = 0x26	/* word - capture count */;
pub const ICE1712_MT_CAPTURE_CONTROL: c_uint = 0x28	/* byte - control */;
pub const ICE1712_CAPTURE_START: c_uint = 0x01	/* capture start */;
pub const ICE1712_MT_ROUTE_PSDOUT03: c_uint = 0x30	/* word */;
pub const ICE1712_MT_ROUTE_SPDOUT: c_uint = 0x32	/* word */;
pub const ICE1712_MT_ROUTE_CAPTURE: c_uint = 0x34	/* dword */;
pub const ICE1712_MT_MONITOR_VOLUME: c_uint = 0x38	/* word */;
pub const ICE1712_MT_MONITOR_INDEX: c_uint = 0x3a	/* byte */;
pub const ICE1712_MT_MONITOR_RATE: c_uint = 0x3b	/* byte */;
pub const ICE1712_MT_MONITOR_ROUTECTRL: c_uint = 0x3c	/* byte */;
pub const ICE1712_ROUTE_AC97: c_uint = 0x01	/* route digital mixer output to AC'97 */;
pub const ICE1712_MT_MONITOR_PEAKINDEX: c_uint = 0x3e	/* byte */;
pub const ICE1712_MT_MONITOR_PEAKDATA: c_uint = 0x3f	/* byte */;
//
// Codec configuration bits
//
// PCI[60] System Configuration
pub const ICE1712_CFG_CLOCK: c_uint = 0xc0;
pub const ICE1712_CFG_CLOCK512: c_uint = 0x00	/* 22.5692Mhz, 44.1kHz*512 */;
pub const ICE1712_CFG_CLOCK384: c_uint = 0x40	/* 16.9344Mhz, 44.1kHz*384 */;
pub const ICE1712_CFG_EXT: c_uint = 0x80	/* external clock */;
pub const ICE1712_CFG_2xMPU401: c_uint = 0x20	/* two MPU401 UARTs */;
pub const ICE1712_CFG_NO_CON_AC97: c_uint = 0x10	/* consumer AC'97 codec is not present */;
pub const ICE1712_CFG_ADC_MASK: c_uint = 0x0c	/* one, two, three, four stereo ADCs */;
pub const ICE1712_CFG_DAC_MASK: c_uint = 0x03	/* one, two, three, four stereo DACs */;
// PCI[61] AC-Link Configuration
pub const ICE1712_CFG_PRO_I2S: c_uint = 0x80	/* multitrack converter: I2S or AC'97 */;
pub const ICE1712_CFG_AC97_PACKED: c_uint = 0x01	/* split or packed mode - AC'97 */;
// PCI[62] I2S Features
pub const ICE1712_CFG_I2S_VOLUME: c_uint = 0x80	/* volume/mute capability */;
pub const ICE1712_CFG_I2S_96KHZ: c_uint = 0x40	/* supports 96kHz sampling */;
pub const ICE1712_CFG_I2S_RESMASK: c_uint = 0x30	/* resolution mask, 16,18,20,24-bit */;
pub const ICE1712_CFG_I2S_OTHER: c_uint = 0x0f	/* other I2S IDs */;
// PCI[63] S/PDIF Configuration
pub const ICE1712_CFG_I2S_CHIPID: c_uint = 0xfc	/* I2S chip ID */;
pub const ICE1712_CFG_SPDIF_IN: c_uint = 0x02	/* S/PDIF input is present */;
pub const ICE1712_CFG_SPDIF_OUT: c_uint = 0x01	/* S/PDIF output is present */;
//
// DMA mode values
// identical with DMA_XXX on i386 architecture.
//
pub const ICE1712_DMA_MODE_WRITE: c_uint = 0x48;
pub const ICE1712_DMA_AUTOINIT: c_uint = 0x10;
//
// I2C EEPROM Address
//
pub const ICE_I2C_EEPROM_ADDR: c_uint = 0xA0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ice1712_eeprom {
    pub /: *mut *mut unsigned int subvendor; / PCI[2c-2f],
    pub /: *mut *mut unsigned char size; / size of EEPROM image in bytes,
    pub /: *mut *mut unsigned char version; / must be 1 (or 2 for vt1724),
    pub data: [c_uchar; 32],
    pub gpiomask: c_uint,
    pub gpiostate: c_uint,
    pub gpiodir: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ak4xxx_private {
    pub /: *mut *mut unsigned int cif:1; / CIF mode,
    pub /: *mut *mut unsigned char caddr; / C0 and C1 bits,
    pub /: *mut *mut unsigned int data_mask; / DATA gpio bit,
    pub /: *mut *mut unsigned int clk_mask; / CLK gpio bit,
    pub /: *mut *mut unsigned int cs_mask; / bit mask for select/deselect address,
    pub /: *mut *mut unsigned int cs_addr; / bits to select address,
    pub /: *mut *mut unsigned int cs_none; / bits to deselect address,
    pub /: *mut *mut unsigned int add_flags; / additional bits at init,
    pub /: *mut *mut unsigned int mask_flags; / total mask bits,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_akm4xxx_ops {
    pub rate): *mut *mut *mut void (set_rate_val)(struct snd_akm4xxx ak, unsigned int,
    pub ops: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ice1712_spdif {
    pub cs8403_bits: c_uchar,
    pub cs8403_stream_bits: c_uchar,
    pub stream_ctl: *mut snd_kcontrol,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ice1712_spdif_ops {
    pub ): *mut *mut *mut void (open)(struct snd_ice1712 , struct snd_pcm_substream,
    pub rate): *mut *mut *mut void (setup_rate)(struct snd_ice1712 , int,
    pub ): *mut *mut *mut void (close)(struct snd_ice1712 , struct snd_pcm_substream,
    pub ucontrol): *mut *mut *mut void (default_get)(struct snd_ice1712 , struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut int (default_put)(struct snd_ice1712 , struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut void (stream_get)(struct snd_ice1712 , struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut int (stream_put)(struct snd_ice1712 , struct snd_ctl_elem_value,
    pub ops: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ice1712 {
    pub conp_dma_size: c_ulong,
    pub conc_dma_size: c_ulong,
    pub prop_dma_size: c_ulong,
    pub proc_dma_size: c_ulong,
    pub irq: c_int,
    pub port: c_ulong,
    pub ddma_port: c_ulong,
    pub dmapath_port: c_ulong,
    pub profi_port: c_ulong,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub pcm_ds: *mut snd_pcm,
    pub pcm_pro: *mut snd_pcm,
    pub playback_con_substream: *mut snd_pcm_substream,
    pub playback_con_substream_ds: [*mut snd_pcm_substream; 6],
    pub capture_con_substream: *mut snd_pcm_substream,
    pub playback_pro_substream: *mut snd_pcm_substream,
    pub capture_pro_substream: *mut snd_pcm_substream,
    pub playback_pro_size: c_uint,
    pub capture_pro_size: c_uint,
    pub playback_con_virt_addr: [c_uint; 6],
    pub playback_con_active_buf: [c_uint; 6],
    pub capture_con_virt_addr: c_uint,
    pub ac97_ext_id: c_uint,
    pub ac97: *mut snd_ac97,
    pub rmidi: [*mut snd_rawmidi; 2],
    pub reg_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
    pub eeprom: snd_ice1712_eeprom,
    pub card_info: *const snd_ice1712_card_info,
    pub pro_volumes: [c_uint; 20],
    pub /: *mut *mut unsigned int omni:1; / Delta Omni I/O,
    pub /: *mut *mut unsigned int dxr_enable:1; / Terratec DXR enable for DMX6FIRE,
    pub vt1724:1: c_uint,
    pub vt1720:1: c_uint,
    pub /: *mut *mut unsigned int has_spdif:1; / VT1720/4 - has SPDIF I/O,
    pub /: *mut *mut unsigned int force_pdma4:1; / VT1720/4 - PDMA4 as non-spdif,
    pub /: *mut *mut unsigned int force_rdma1:1; / VT1720/4 - RDMA1 as non-spdif,
    pub /: *mut *mut unsigned int midi_output:1; / VT1720/4: MIDI output triggered,
    pub /: *mut *mut unsigned int midi_input:1; / VT1720/4: MIDI input triggered,
    pub /: *mut *mut unsigned int own_routing:1; / VT1720/4: use own routing ctls,
    pub /: *mut *mut unsigned int num_total_dacs; / total DACs,
    pub /: *mut *mut unsigned int num_total_adcs; / total ADCs,
    pub /: *mut *mut unsigned int cur_rate; / current rate,
    pub open_mutex: mutex,
    pub pcm_reserved: [*mut snd_pcm_substream; 4],
    pub /: *const *const *const snd_pcm_hw_constraint_list hw_rates; / card-specific rate constraints,
    pub akm_codecs: c_uint,
    pub akm: *mut snd_akm4xxx,
    pub spdif: snd_ice1712_spdif,
    pub /: *mut *mut mutex i2c_mutex; / I2C mutex for ICE1724 registers,
    pub /: *mut *mut *mut snd_i2c_bus i2c; / I2C bus,
    pub /: *mut *mut *mut snd_i2c_device cs8427; / CS8427 I2C device,
    pub /: *mut *mut unsigned int cs8427_timeout; / CS8427 reset timeout in HZ/100,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice1712_gpio {
    pub /: *mut *mut unsigned int direction; / current direction bits,
    pub /: *mut *mut unsigned int write_mask; / current mask bits,
    pub /: *mut *mut unsigned int saved[2]; / for ewx_i2c,
// operators
    pub data): *mut *mut *mut void (set_mask)(struct snd_ice1712 ice, unsigned int,
    pub ice): *mut *mut unsigned int (get_mask)(struct snd_ice1712,
    pub data): *mut *mut *mut void (set_dir)(struct snd_ice1712 ice, unsigned int,
    pub ice): *mut *mut unsigned int (get_dir)(struct snd_ice1712,
    pub data): *mut *mut *mut void (set_data)(struct snd_ice1712 ice, unsigned int,
    pub ice): *mut *mut unsigned int (get_data)(struct snd_ice1712,
// misc operators - move to another place?
    pub rate): *mut *mut *mut void (set_pro_rate)(struct snd_ice1712 ice, unsigned int,
    pub ice): *mut *mut void (i2s_mclk_changed)(struct snd_ice1712,
    pub gpio: },
    pub gpio_mutex: mutex,
// other board-specific data
    pub spec: *mut c_void,
// VT172x specific
    pub pro_rate_default: c_int,
    pub ice): *mut *mut int (is_spdif_master)(struct snd_ice1712,
    pub ice): *mut *mut unsigned int (get_rate)(struct snd_ice1712,
    pub rate): *mut *mut *mut void (set_rate)(struct snd_ice1712 ice, unsigned int,
    pub rate): *mut *mut *mut unsigned char (set_mclk)(struct snd_ice1712 ice, unsigned int,
    pub type): *mut *mut *mut int (set_spdif_clock)(struct snd_ice1712 ice, int,
    pub ice): *mut *mut int (get_spdif_master_type)(struct snd_ice1712,
    pub ext_clock_names: *const *const c_char,
    pub ext_clock_count: c_int,
    pub ): *mut *mut *mut void (pro_open)(struct snd_ice1712 , struct snd_pcm_substream,

    pub ): *mut *mut int (pm_suspend)(struct snd_ice1712,
    pub ): *mut *mut int (pm_resume)(struct snd_ice1712,
    pub pm_suspend_enabled:1: c_uint,
    pub pm_saved_is_spdif_master:1: c_uint,
    pub pm_saved_spdif_ctrl: c_uint,
    pub pm_saved_spdif_cfg: c_uchar,
    pub pm_saved_route: c_uint,

}

//
// gpio access functions
//
// save and restore gpio status
// The access to gpio will be protected by mutex, so don't forget to
// restore!
//
// for bit controls

extern "C" {
    pub fn snd_ice1712_gpio_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn snd_ice1712_gpio_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
//
// set gpio direction, write mask and data
//
// route access functions
extern "C" {
    pub fn snd_ice1724_get_route_val(ice: *mut snd_ice1712, shift: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ice1712_spdif_build_controls(ice: *mut snd_ice1712) -> c_int;
}
extern "C" {
    pub fn snd_ice1712_akm4xxx_free(ice: *mut snd_ice1712);
}
extern "C" {
    pub fn snd_ice1712_akm4xxx_build_controls(ice: *mut snd_ice1712) -> c_int;
}
extern "C" {
    pub fn snd_ice1712_init_cs8427(ice: *mut snd_ice1712, addr: c_int) -> c_int;
}
extern "C" {
    pub fn inb(_arg: ICEREG(ice, _arg: DATA)) -> return;
}
//
// entry pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub driver: *const c_char,
    pub ): *mut *mut int (chip_init)(struct snd_ice1712,
    pub ): *mut *mut void (chip_exit)(struct snd_ice1712,
    pub ): *mut *mut int (build_controls)(struct snd_ice1712,
    pub no_mpu401:1: c_uint,
    pub mpu401_1_info_flags: c_uint,
    pub mpu401_2_info_flags: c_uint,
    pub mpu401_1_name: *const c_char,
    pub mpu401_2_name: *const c_char,
    pub eeprom_size: c_uint,
    pub eeprom_data: *const c_uchar,
}
