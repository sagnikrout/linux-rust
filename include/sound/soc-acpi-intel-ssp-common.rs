//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-acpi-intel-ssp-common.h
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
// Copyright(c) 2023 Intel Corporation.
//
// Cirrus Logic

// Dialog

// Everest

// Nuvoton

// Realtek

// Texas Instruments

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_acpi_intel_codec {
    CODEC_NONE,

// headphone codec
    CODEC_CS42L42,
    CODEC_DA7219,
    CODEC_ES8316,
    CODEC_ES8326,
    CODEC_ES8336,
    CODEC_NAU8825,
    CODEC_RT5650,
    CODEC_RT5682,
    CODEC_RT5682S,

// speaker amplifier
    CODEC_CS35L41,
    CODEC_MAX98357A,
    CODEC_MAX98360A,
    CODEC_MAX98373,
    CODEC_MAX98390,
    CODEC_NAU8318,
    CODEC_RT1011,
    CODEC_RT1015,
    CODEC_RT1015P,
    CODEC_RT1019P,
    CODEC_RT1308,
    CODEC_TAS2563,
}
