//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/common/soc-acpi-intel-skl-match.c
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
// soc-acpi-intel-skl-match.c - tables and support for SKL ACPI enumeration.
//
// Copyright (c) 2018, Intel Corporation.
//

    static const struct snd_soc_acpi_codecs skl_codecs = {
    .num_codecs = 1,
    .codecs = {"10508825"}
    };
    struct snd_soc_acpi_mach snd_soc_acpi_intel_skl_machines[] = {
    {
    .id = "INT343A",
    .drv_name = "skl_alc286s_i2s",
    .fw_filename = "intel/dsp_fw_release.bin",
    },
    {
    .id = "INT343B",
    .drv_name = "skl_n88l25_s4567",
    .fw_filename = "intel/dsp_fw_release.bin",
    .machine_quirk = snd_soc_acpi_codec_list,
    .quirk_data = &skl_codecs,
    },
    {
    .id = "MX98357A",
    .drv_name = "skl_n88l25_m98357a",
    .fw_filename = "intel/dsp_fw_release.bin",
    .machine_quirk = snd_soc_acpi_codec_list,
    .quirk_data = &skl_codecs,
    },
    {},
    };
    EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_skl_machines);
