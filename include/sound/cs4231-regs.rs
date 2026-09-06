//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs4231-regs.h
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
// Definitions for CS4231 & InterWave chips & compatible chips registers
//
// IO ports

pub const c_d_c_CS4231REGSEL: c_int = 0;
pub const c_d_c_CS4231REG: c_int = 1;
pub const c_d_c_CS4231STATUS: c_int = 2;
pub const c_d_c_CS4231PIO: c_int = 3;
// codec registers
pub const CS4231_LEFT_INPUT: c_uint = 0x00	/* left input control */;
pub const CS4231_RIGHT_INPUT: c_uint = 0x01	/* right input control */;
pub const CS4231_AUX1_LEFT_INPUT: c_uint = 0x02	/* left AUX1 input control */;
pub const CS4231_AUX1_RIGHT_INPUT: c_uint = 0x03	/* right AUX1 input control */;
pub const CS4231_AUX2_LEFT_INPUT: c_uint = 0x04	/* left AUX2 input control */;
pub const CS4231_AUX2_RIGHT_INPUT: c_uint = 0x05	/* right AUX2 input control */;
pub const CS4231_LEFT_OUTPUT: c_uint = 0x06	/* left output control register */;
pub const CS4231_RIGHT_OUTPUT: c_uint = 0x07	/* right output control register */;
pub const CS4231_PLAYBK_FORMAT: c_uint = 0x08	/* clock and data format - playback - bits 7-0 MCE */;
pub const CS4231_IFACE_CTRL: c_uint = 0x09	/* interface control - bits 7-2 MCE */;
pub const CS4231_PIN_CTRL: c_uint = 0x0a	/* pin control */;
pub const CS4231_TEST_INIT: c_uint = 0x0b	/* test and initialization */;
pub const CS4231_MISC_INFO: c_uint = 0x0c	/* miscellaneous information */;
pub const CS4231_LOOPBACK: c_uint = 0x0d	/* loopback control */;
pub const CS4231_PLY_UPR_CNT: c_uint = 0x0e	/* playback upper base count */;
pub const CS4231_PLY_LWR_CNT: c_uint = 0x0f	/* playback lower base count */;
pub const CS4231_ALT_FEATURE_1: c_uint = 0x10	/* alternate #1 feature enable */;
pub const AD1845_AF1_MIC_LEFT: c_uint = 0x10	/* alternate #1 feature + MIC left */;
pub const CS4231_ALT_FEATURE_2: c_uint = 0x11	/* alternate #2 feature enable */;
pub const AD1845_AF2_MIC_RIGHT: c_uint = 0x11	/* alternate #2 feature + MIC right */;
pub const CS4231_LEFT_LINE_IN: c_uint = 0x12	/* left line input control */;
pub const CS4231_RIGHT_LINE_IN: c_uint = 0x13	/* right line input control */;
pub const CS4231_TIMER_LOW: c_uint = 0x14	/* timer low byte */;
pub const CS4231_TIMER_HIGH: c_uint = 0x15	/* timer high byte */;
pub const CS4231_LEFT_MIC_INPUT: c_uint = 0x16	/* left MIC input control register (InterWave only) */;
pub const AD1845_UPR_FREQ_SEL: c_uint = 0x16	/* upper byte of frequency select */;
pub const CS4231_RIGHT_MIC_INPUT: c_uint = 0x17	/* right MIC input control register (InterWave only) */;
pub const AD1845_LWR_FREQ_SEL: c_uint = 0x17	/* lower byte of frequency select */;
pub const CS4236_EXT_REG: c_uint = 0x17	/* extended register access */;
pub const CS4231_IRQ_STATUS: c_uint = 0x18	/* irq status register */;
pub const CS4231_LINE_LEFT_OUTPUT: c_uint = 0x19	/* left line output control register (InterWave only) */;
pub const CS4231_VERSION: c_uint = 0x19	/* CS4231(A) - version values */;
pub const CS4231_MONO_CTRL: c_uint = 0x1a	/* mono input/output control */;
pub const CS4231_LINE_RIGHT_OUTPUT: c_uint = 0x1b	/* right line output control register (InterWave only) */;
pub const AD1845_PWR_DOWN: c_uint = 0x1b	/* power down control */;
pub const CS4235_LEFT_MASTER: c_uint = 0x1b	/* left master output control */;
pub const CS4231_REC_FORMAT: c_uint = 0x1c	/* clock and data format - record - bits 7-0 MCE */;
pub const AD1845_CLOCK: c_uint = 0x1d	/* crystal clock select and total power down */;
pub const CS4235_RIGHT_MASTER: c_uint = 0x1d	/* right master output control */;
pub const CS4231_REC_UPR_CNT: c_uint = 0x1e	/* record upper count */;
pub const CS4231_REC_LWR_CNT: c_uint = 0x1f	/* record lower count */;
// definitions for codec register select port - CODECP( REGSEL )
pub const CS4231_INIT: c_uint = 0x80	/* CODEC is initializing */;
pub const CS4231_MCE: c_uint = 0x40	/* mode change enable */;
pub const CS4231_TRD: c_uint = 0x20	/* transfer request disable */;
// definitions for codec status register - CODECP( STATUS )
pub const CS4231_GLOBALIRQ: c_uint = 0x01	/* IRQ is active */;
// definitions for codec irq status
pub const CS4231_PLAYBACK_IRQ: c_uint = 0x10;
pub const CS4231_RECORD_IRQ: c_uint = 0x20;
pub const CS4231_TIMER_IRQ: c_uint = 0x40;
pub const CS4231_ALL_IRQS: c_uint = 0x70;
pub const CS4231_REC_UNDERRUN: c_uint = 0x08;
pub const CS4231_REC_OVERRUN: c_uint = 0x04;
pub const CS4231_PLY_OVERRUN: c_uint = 0x02;
pub const CS4231_PLY_UNDERRUN: c_uint = 0x01;
// definitions for CS4231_LEFT_INPUT and CS4231_RIGHT_INPUT registers
pub const CS4231_ENABLE_MIC_GAIN: c_uint = 0x20;
pub const CS4231_MIXS_LINE: c_uint = 0x00;
pub const CS4231_MIXS_AUX1: c_uint = 0x40;
pub const CS4231_MIXS_MIC: c_uint = 0x80;
pub const CS4231_MIXS_ALL: c_uint = 0xc0;
// definitions for clock and data format register - CS4231_PLAYBK_FORMAT
pub const CS4231_LINEAR_8: c_uint = 0x00	/* 8-bit unsigned data */;
pub const CS4231_ALAW_8: c_uint = 0x60	/* 8-bit A-law companded */;
pub const CS4231_ULAW_8: c_uint = 0x20	/* 8-bit U-law companded */;
pub const CS4231_LINEAR_16: c_uint = 0x40	/* 16-bit twos complement data - little endian */;
pub const CS4231_LINEAR_16_BIG: c_uint = 0xc0	/* 16-bit twos complement data - big endian */;
pub const CS4231_ADPCM_16: c_uint = 0xa0	/* 16-bit ADPCM */;
pub const CS4231_STEREO: c_uint = 0x10	/* stereo mode */;
// bits 3-1 define frequency divisor
pub const CS4231_XTAL1: c_uint = 0x00	/* 24.576 crystal */;
pub const CS4231_XTAL2: c_uint = 0x01	/* 16.9344 crystal */;
// definitions for interface control register - CS4231_IFACE_CTRL
pub const CS4231_RECORD_PIO: c_uint = 0x80	/* record PIO enable */;
pub const CS4231_PLAYBACK_PIO: c_uint = 0x40	/* playback PIO enable */;
pub const CS4231_CALIB_MODE: c_uint = 0x18	/* calibration mode bits */;
pub const CS4231_AUTOCALIB: c_uint = 0x08	/* auto calibrate */;
pub const CS4231_SINGLE_DMA: c_uint = 0x04	/* use single DMA channel */;
pub const CS4231_RECORD_ENABLE: c_uint = 0x02	/* record enable */;
pub const CS4231_PLAYBACK_ENABLE: c_uint = 0x01	/* playback enable */;
// definitions for pin control register - CS4231_PIN_CTRL
pub const CS4231_IRQ_ENABLE: c_uint = 0x02	/* enable IRQ */;
pub const CS4231_XCTL1: c_uint = 0x40	/* external control #1 */;
pub const CS4231_XCTL0: c_uint = 0x80	/* external control #0 */;
// definitions for test and init register - CS4231_TEST_INIT
pub const CS4231_CALIB_IN_PROGRESS: c_uint = 0x20	/* auto calibrate in progress */;
pub const CS4231_DMA_REQUEST: c_uint = 0x10	/* DMA request in progress */;
// definitions for misc control register - CS4231_MISC_INFO
pub const CS4231_MODE2: c_uint = 0x40	/* MODE 2 */;
pub const CS4231_IW_MODE3: c_uint = 0x6c	/* MODE 3 - InterWave enhanced mode */;
pub const CS4231_4236_MODE3: c_uint = 0xe0	/* MODE 3 - CS4236+ enhanced mode */;
// definitions for alternate feature 1 register - CS4231_ALT_FEATURE_1
pub const CS4231_DACZ: c_uint = 0x01	/* zero DAC when underrun */;
pub const CS4231_TIMER_ENABLE: c_uint = 0x40	/* codec timer enable */;
pub const CS4231_OLB: c_uint = 0x80	/* output level bit */;
// definitions for Extended Registers - CS4236+

pub const CS4236_LEFT_LINE: c_uint = 0x08	/* left LINE alternate volume */;
pub const CS4236_RIGHT_LINE: c_uint = 0x18	/* right LINE alternate volume */;
pub const CS4236_LEFT_MIC: c_uint = 0x28	/* left MIC volume */;
pub const CS4236_RIGHT_MIC: c_uint = 0x38	/* right MIC volume */;
pub const CS4236_LEFT_MIX_CTRL: c_uint = 0x48	/* synthesis and left input mixer control */;
pub const CS4236_RIGHT_MIX_CTRL: c_uint = 0x58	/* right input mixer control */;
pub const CS4236_LEFT_FM: c_uint = 0x68	/* left FM volume */;
pub const CS4236_RIGHT_FM: c_uint = 0x78	/* right FM volume */;
pub const CS4236_LEFT_DSP: c_uint = 0x88	/* left DSP serial port volume */;
pub const CS4236_RIGHT_DSP: c_uint = 0x98	/* right DSP serial port volume */;
pub const CS4236_RIGHT_LOOPBACK: c_uint = 0xa8	/* right loopback monitor volume */;
pub const CS4236_DAC_MUTE: c_uint = 0xb8	/* DAC mute and IFSE enable */;
pub const CS4236_ADC_RATE: c_uint = 0xc8	/* indenpendent ADC sample frequency */;
pub const CS4236_DAC_RATE: c_uint = 0xd8	/* indenpendent DAC sample frequency */;
pub const CS4236_LEFT_MASTER: c_uint = 0xe8	/* left master digital audio volume */;
pub const CS4236_RIGHT_MASTER: c_uint = 0xf8	/* right master digital audio volume */;
pub const CS4236_LEFT_WAVE: c_uint = 0x0c	/* left wavetable serial port volume */;
pub const CS4236_RIGHT_WAVE: c_uint = 0x1c	/* right wavetable serial port volume */;
pub const CS4236_VERSION: c_uint = 0x9c	/* chip version and ID */;
// definitions for extended registers - OPTI93X
pub const OPTi931_AUX_LEFT_INPUT: c_uint = 0x10;
pub const OPTi931_AUX_RIGHT_INPUT: c_uint = 0x11;
pub const OPTi93X_MIC_LEFT_INPUT: c_uint = 0x14;
pub const OPTi93X_MIC_RIGHT_INPUT: c_uint = 0x15;
pub const OPTi93X_OUT_LEFT: c_uint = 0x16;
pub const OPTi93X_OUT_RIGHT: c_uint = 0x17;
