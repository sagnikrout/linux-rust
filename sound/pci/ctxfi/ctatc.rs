//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctatc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File	ctatc.h
//
// @Brief
// This file contains the definition of the device resource management object.
//
// @Author	Liu Chun
// @Date 	Mar 28 2008
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CTALSADEVS {
    FRONT,
    SURROUND,
    CLFE,
    SIDE,
    IEC958,
    MIXER,
    NUM_CTALSADEVS		/* This should always be the last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_atc_chip_sub_details {
    pub subsys: u16,
    pub nm_model: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_atc_chip_details {
    pub vendor: u16,
    pub device: u16,
    pub sub_details: *const ct_atc_chip_sub_details,
    pub nm_card: *const c_char,
}

// alsa pcm stream descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_atc_pcm {
    pub substream: *mut snd_pcm_substream,
    pub apcm): *mut *mut void (interrupt)(struct ct_atc_pcm,
    pub timer: *mut ct_timer_instance,
    pub started:1: c_uint,
// Only mono and interleaved modes are supported now.
    pub vm_block: *mut ct_vm_block,
    pub /: *mut *mut *mut void src; / SRC for interacting with host memory,
    pub /: *mut *mut *mut *mut void srccs; / SRCs for sample rate conversion,
    pub /: *mut *mut *mut *mut void srcimps; / SRC Input Mappers,
    pub /: *mut *mut *mut *mut void amixers; / AMIXERs for routing converted data,
    pub /: *mut *mut *mut void mono; / A SUM resource for mixing chs to one,
    pub /: *mut *mut unsigned char n_srcc; / Number of converting SRCs,
    pub /: *mut *mut unsigned char n_srcimp; / Number of SRC Input Mappers,
    pub /: *mut *mut unsigned char n_amixer; / Number of AMIXERs,
}

// Chip resource management object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_atc {
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub /: *mut *mut unsigned int rsr; / reference sample rate in Hz,
    pub /: *mut *mut unsigned int msr; / master sample rate in rsr,
    pub /: *mut *mut unsigned int pll_rate; / current rate of Phase Lock Loop,
    pub chip_type: c_int,
    pub model: c_int,
    pub chip_name: *const c_char,
    pub model_name: *const c_char,
    pub /: *mut *mut unsigned char rca_state; / 0 = dedicated RCA, 1 = 7.1ch Front,
    pub /: *mut *mut *mut ct_vm vm; / device virtual memory manager for this card,
    pub apcm): *mut *mut *mut int (map_audio_buffer)(struct ct_atc atc, struct ct_atc_pcm,
    pub apcm): *mut *mut *mut void (unmap_audio_buffer)(struct ct_atc atc, struct ct_atc_pcm,
    pub index): *mut *mut *mut unsigned long (get_ptp_phys)(struct ct_atc atc, int,
    pub atc_mutex: mutex,
    pub apcm): *mut ct_atc_pcm,
    pub apcm): *mut *mut *mut int (pcm_playback_start)(struct ct_atc atc, struct ct_atc_pcm,
    pub apcm): *mut *mut *mut int (pcm_playback_stop)(struct ct_atc atc, struct ct_atc_pcm,
    pub apcm): *mut ct_atc_pcm,
    pub apcm): *mut ct_atc_pcm,
    pub apcm): *mut *mut *mut int (pcm_capture_prepare)(struct ct_atc atc, struct ct_atc_pcm,
    pub apcm): *mut *mut *mut int (pcm_capture_start)(struct ct_atc atc, struct ct_atc_pcm,
    pub apcm): *mut *mut *mut int (pcm_capture_stop)(struct ct_atc atc, struct ct_atc_pcm,
    pub apcm): *mut ct_atc_pcm,
    pub apcm): *mut ct_atc_pcm,
    pub atc): *mut *mut int (select_line_in)(struct ct_atc,
    pub atc): *mut *mut int (select_mic_in)(struct ct_atc,
    pub atc): *mut *mut int (select_digit_io)(struct ct_atc,
    pub state): *mut *mut *mut int (line_front_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (line_surround_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (line_clfe_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (line_rear_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (line_in_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (mic_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (rca_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (spdif_out_unmute)(struct ct_atc atc, unsigned char,
    pub state): *mut *mut *mut int (spdif_in_unmute)(struct ct_atc atc, unsigned char,
    pub status): *mut *mut *mut int (spdif_out_get_status)(struct ct_atc atc, unsigned int,
    pub status): *mut *mut *mut int (spdif_out_set_status)(struct ct_atc atc, unsigned int,
    pub state): *mut *mut *mut int (spdif_out_passthru)(struct ct_atc atc, unsigned char,
    pub atc): *mut *mut capabilities (capabilities)(ct_atc,
    pub atc): *mut *mut void (dedicated_rca_select)(struct ct_atc,
    pub atc): *mut *mut int (output_switch_get)(struct ct_atc,
    pub position): *mut *mut *mut int (output_switch_put)(struct ct_atc atc, int,
    pub atc): *mut *mut int (mic_source_switch_get)(struct ct_atc,
    pub position): *mut *mut *mut int (mic_source_switch_put)(struct ct_atc atc, int,
// Don't touch! Used for internal object.
    pub /: *mut *mut *mut void rsc_mgrs[NUM_RSCTYP]; / chip resource managers,
    pub /: *mut *mut *mut void mixer; / internal mixer object,
    pub /: *mut *mut *mut hw hw; / chip specific hardware access object,
    pub /: *mut *mut *mut *mut void daios; / digital audio io resources,
    pub /: *mut *mut *mut *mut void pcm; / SUMs for collecting all pcm stream,
    pub /: *mut *mut *mut *mut void srcs; / Sample Rate Converters for input signal,
    pub /: *mut *mut *mut *mut void srcimps; / input mappers for SRCs,
    pub timer: *mut ct_timer,

    pub atc): *mut *mut int (suspend)(struct ct_atc,
    pub atc): *mut *mut int (resume)(struct ct_atc,
    pub pcms: [*mut snd_pcm; NUM_PCMS],
}

extern "C" {
    pub fn ct_atc_create_alsa_devs(atc: *mut ct_atc) -> c_int;
}
