//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ad1889.h
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
// Analog Devices 1889 audio driver
// Copyright (C) 2004, Kyle McMartin <kyle@parisc-linux.org>
//
pub const AD_DS_WSMC: c_uint = 0x00 /* wave/synthesis channel mixer control */;
pub const AD_DS_WSMC_SYEN: c_uint = 0x0004 /* synthesis channel enable */;
pub const AD_DS_WSMC_SYRQ: c_uint = 0x0030 /* synth. fifo request point */;
pub const AD_DS_WSMC_WA16: c_uint = 0x0100 /* wave channel 16bit select */;
pub const AD_DS_WSMC_WAST: c_uint = 0x0200 /* wave channel stereo select */;
pub const AD_DS_WSMC_WAEN: c_uint = 0x0400 /* wave channel enable */;
pub const AD_DS_WSMC_WARQ: c_uint = 0x3000 /* wave fifo request point */;
pub const AD_DS_RAMC: c_uint = 0x02 /* resampler/ADC channel mixer control */;
pub const AD_DS_RAMC_AD16: c_uint = 0x0001 /* ADC channel 16bit select */;
pub const AD_DS_RAMC_ADST: c_uint = 0x0002 /* ADC channel stereo select */;
pub const AD_DS_RAMC_ADEN: c_uint = 0x0004 /* ADC channel enable */;
pub const AD_DS_RAMC_ACRQ: c_uint = 0x0030 /* ADC fifo request point */;
pub const AD_DS_RAMC_REEN: c_uint = 0x0400 /* resampler channel enable */;
pub const AD_DS_RAMC_RERQ: c_uint = 0x3000 /* res. fifo request point */;
pub const AD_DS_WADA: c_uint = 0x04 /* wave channel mix attenuation */;
pub const AD_DS_WADA_RWAM: c_uint = 0x0080 /* right wave mute */;
pub const AD_DS_WADA_RWAA: c_uint = 0x001f /* right wave attenuation */;
pub const AD_DS_WADA_LWAM: c_uint = 0x8000 /* left wave mute */;
pub const AD_DS_WADA_LWAA: c_uint = 0x3e00 /* left wave attenuation */;
pub const AD_DS_SYDA: c_uint = 0x06 /* synthesis channel mix attenuation */;
pub const AD_DS_SYDA_RSYM: c_uint = 0x0080 /* right synthesis mute */;
pub const AD_DS_SYDA_RSYA: c_uint = 0x001f /* right synthesis attenuation */;
pub const AD_DS_SYDA_LSYM: c_uint = 0x8000 /* left synthesis mute */;
pub const AD_DS_SYDA_LSYA: c_uint = 0x3e00 /* left synthesis attenuation */;
pub const AD_DS_WAS: c_uint = 0x08 /* wave channel sample rate */;
pub const AD_DS_WAS_WAS: c_uint = 0xffff /* sample rate mask */;
pub const AD_DS_RES: c_uint = 0x0a /* resampler channel sample rate */;
pub const AD_DS_RES_RES: c_uint = 0xffff /* sample rate mask */;
pub const AD_DS_CCS: c_uint = 0x0c /* chip control/status */;
pub const AD_DS_CCS_ADO: c_uint = 0x0001 /* ADC channel overflow */;
pub const AD_DS_CCS_REO: c_uint = 0x0002 /* resampler channel overflow */;
pub const AD_DS_CCS_SYU: c_uint = 0x0004 /* synthesis channel underflow */;
pub const AD_DS_CCS_WAU: c_uint = 0x0008 /* wave channel underflow */;
// bits 4 -> 7, 9, 11 -> 14 reserved
pub const AD_DS_CCS_XTD: c_uint = 0x0100 /* xtd delay control (4096 clock cycles) */;
pub const AD_DS_CCS_PDALL: c_uint = 0x0400 /* power */;
pub const AD_DS_CCS_CLKEN: c_uint = 0x8000 /* clock */;
pub const AD_DMA_RESBA: c_uint = 0x40 /* RES base address */;
pub const AD_DMA_RESCA: c_uint = 0x44 /* RES current address */;
pub const AD_DMA_RESBC: c_uint = 0x48 /* RES base count */;
pub const AD_DMA_RESCC: c_uint = 0x4c /* RES current count */;
pub const AD_DMA_ADCBA: c_uint = 0x50 /* ADC base address */;
pub const AD_DMA_ADCCA: c_uint = 0x54 /* ADC current address */;
pub const AD_DMA_ADCBC: c_uint = 0x58 /* ADC base count */;
pub const AD_DMA_ADCCC: c_uint = 0x5c /* ADC current count */;
pub const AD_DMA_SYNBA: c_uint = 0x60 /* synth base address */;
pub const AD_DMA_SYNCA: c_uint = 0x64 /* synth current address */;
pub const AD_DMA_SYNBC: c_uint = 0x68 /* synth base count */;
pub const AD_DMA_SYNCC: c_uint = 0x6c /* synth current count */;
pub const AD_DMA_WAVBA: c_uint = 0x70 /* wave base address */;
pub const AD_DMA_WAVCA: c_uint = 0x74 /* wave current address */;
pub const AD_DMA_WAVBC: c_uint = 0x78 /* wave base count */;
pub const AD_DMA_WAVCC: c_uint = 0x7c /* wave current count */;
pub const AD_DMA_RESIC: c_uint = 0x80 /* RES dma interrupt current byte count */;
pub const AD_DMA_RESIB: c_uint = 0x84 /* RES dma interrupt base byte count */;
pub const AD_DMA_ADCIC: c_uint = 0x88 /* ADC dma interrupt current byte count */;
pub const AD_DMA_ADCIB: c_uint = 0x8c /* ADC dma interrupt base byte count */;
pub const AD_DMA_SYNIC: c_uint = 0x90 /* synth dma interrupt current byte count */;
pub const AD_DMA_SYNIB: c_uint = 0x94 /* synth dma interrupt base byte count */;
pub const AD_DMA_WAVIC: c_uint = 0x98 /* wave dma interrupt current byte count */;
pub const AD_DMA_WAVIB: c_uint = 0x9c /* wave dma interrupt base byte count */;
pub const AD_DMA_ICC: c_uint = 0xffffff /* current byte count mask */;
pub const AD_DMA_IBC: c_uint = 0xffffff /* base byte count mask */;
// bits 24 -> 31 reserved
// 4 bytes pad
pub const AD_DMA_ADC: c_uint = 0xa8	/* ADC      dma control and status */;
pub const AD_DMA_SYNTH: c_uint = 0xb0	/* Synth    dma control and status */;
pub const AD_DMA_WAV: c_uint = 0xb8	/* wave     dma control and status */;
pub const AD_DMA_RES: c_uint = 0xa0	/* Resample dma control and status */;
pub const AD_DMA_SGDE: c_uint = 0x0001 /* SGD mode enable */;
pub const AD_DMA_LOOP: c_uint = 0x0002 /* loop enable */;
pub const AD_DMA_IM: c_uint = 0x000c /* interrupt mode mask */;

pub const AD_DMA_IM_CNT: c_uint = 0x0004 /* interrupt on count */;
pub const AD_DMA_IM_SGD: c_uint = 0x0008 /* interrupt on SGD flag */;
pub const AD_DMA_IM_EOL: c_uint = 0x000c /* interrupt on End of Linked List */;
pub const AD_DMA_SGDS: c_uint = 0x0030 /* SGD status */;
pub const AD_DMA_SFLG: c_uint = 0x0040 /* SGD flag */;
pub const AD_DMA_EOL: c_uint = 0x0080 /* SGD end of list */;
// bits 8 -> 15 reserved
pub const AD_DMA_DISR: c_uint = 0xc0 /* dma interrupt status */;
pub const AD_DMA_DISR_RESI: c_uint = 0x000001 /* resampler channel interrupt */;
pub const AD_DMA_DISR_ADCI: c_uint = 0x000002 /* ADC channel interrupt */;
pub const AD_DMA_DISR_SYNI: c_uint = 0x000004 /* synthesis channel interrupt */;
pub const AD_DMA_DISR_WAVI: c_uint = 0x000008 /* wave channel interrupt */;
// bits 4, 5 reserved
pub const AD_DMA_DISR_SEPS: c_uint = 0x000040 /* serial eeprom status */;
// bits 7 -> 13 reserved
pub const AD_DMA_DISR_PMAI: c_uint = 0x004000 /* pci master abort interrupt */;
pub const AD_DMA_DISR_PTAI: c_uint = 0x008000 /* pci target abort interrupt */;
pub const AD_DMA_DISR_PTAE: c_uint = 0x010000 /* pci target abort interrupt enable */;
pub const AD_DMA_DISR_PMAE: c_uint = 0x020000 /* pci master abort interrupt enable */;
// bits 19 -> 31 reserved
// interrupt mask

pub const AD_DMA_CHSS: c_uint = 0xc4 /* dma channel stop status */;
pub const AD_DMA_CHSS_RESS: c_uint = 0x000001 /* resampler channel stopped */;
pub const AD_DMA_CHSS_ADCS: c_uint = 0x000002 /* ADC channel stopped */;
pub const AD_DMA_CHSS_SYNS: c_uint = 0x000004 /* synthesis channel stopped */;
pub const AD_DMA_CHSS_WAVS: c_uint = 0x000008 /* wave channel stopped */;
pub const AD_GPIO_IPC: c_uint = 0xc8	/* gpio port control */;
pub const AD_GPIO_OP: c_uint = 0xca	/* gpio output port status */;
pub const AD_GPIO_IP: c_uint = 0xcc	/* gpio  input port status */;
pub const AD_AC97_BASE: c_uint = 0x100	/* ac97 base register */;
pub const AD_AC97_RESET: c_uint = 0x100   /* reset */;
pub const AD_AC97_PWR_CTL: c_uint = 0x126	/* == AC97_POWERDOWN */;
pub const AD_AC97_PWR_ADC: c_uint = 0x0001 /* ADC ready status */;
pub const AD_AC97_PWR_DAC: c_uint = 0x0002 /* DAC ready status */;
pub const AD_AC97_PWR_PR0: c_uint = 0x0100 /* PR0 (ADC) powerdown */;
pub const AD_AC97_PWR_PR1: c_uint = 0x0200 /* PR1 (DAC) powerdown */;
pub const AD_MISC_CTL: c_uint = 0x176 /* misc control */;
pub const AD_MISC_CTL_DACZ: c_uint = 0x8000 /* set for zero fill, unset for repeat */;
pub const AD_MISC_CTL_ARSR: c_uint = 0x0001 /* set for SR1, unset for SR0 */;
pub const AD_MISC_CTL_ALSR: c_uint = 0x0100;
pub const AD_MISC_CTL_DLSR: c_uint = 0x0400;
pub const AD_MISC_CTL_DRSR: c_uint = 0x0004;
pub const AD_AC97_SR0: c_uint = 0x178 /* sample rate 0, 0xbb80 == 48K */;
pub const AD_AC97_SR0_48K: c_uint = 0xbb80 /* 48KHz */;
pub const AD_AC97_SR1: c_uint = 0x17a /* sample rate 1 */;
pub const AD_AC97_ACIC: c_uint = 0x180 /* ac97 codec interface control */;
pub const AD_AC97_ACIC_ACIE: c_uint = 0x0001 /* analog codec interface enable */;
pub const AD_AC97_ACIC_ACRD: c_uint = 0x0002 /* analog codec reset disable */;
pub const AD_AC97_ACIC_ASOE: c_uint = 0x0004 /* audio stream output enable */;
pub const AD_AC97_ACIC_VSRM: c_uint = 0x0008 /* variable sample rate mode */;
pub const AD_AC97_ACIC_FSDH: c_uint = 0x0100 /* force SDATA_OUT high */;
pub const AD_AC97_ACIC_FSYH: c_uint = 0x0200 /* force sync high */;
pub const AD_AC97_ACIC_ACRDY: c_uint = 0x8000 /* analog codec ready status */;
// bits 10 -> 14 reserved
pub const AD_DS_MEMSIZE: c_int = 512;
pub const AD_OPL_MEMSIZE: c_int = 16;
pub const AD_MIDI_MEMSIZE: c_int = 16;
pub const AD_WAV_STATE: c_int = 0;
pub const AD_ADC_STATE: c_int = 1;
pub const AD_MAX_STATES: c_int = 2;
pub const AD_CHAN_WAV: c_uint = 0x0001;
pub const AD_CHAN_ADC: c_uint = 0x0002;
pub const AD_CHAN_RES: c_uint = 0x0004;
pub const AD_CHAN_SYN: c_uint = 0x0008;
// The chip would support 4 GB buffers and 16 MB periods,
// but let's not overdo it ...

pub const PERIOD_BYTES_MIN: c_int = 32;

pub const PERIODS_MIN: c_int = 2;

