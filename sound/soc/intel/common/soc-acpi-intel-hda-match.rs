//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/common/soc-acpi-intel-hda-match.c
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
// Copyright (c) 2018, Intel Corporation.
//
// soc-acpi-intel-hda-match.c - tables and support for HDA+ACPI enumeration.
//

    struct snd_soc_acpi_mach snd_soc_acpi_intel_hda_machines[] = {
    {
// .id is not used in this file
    .drv_name = "skl_hda_dsp_generic",
    .sof_tplg_filename = "sof-hda-generic", /* the tplg suffix is added at run time */
    .tplg_quirk_mask = SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
    },
    {},
    };
    EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_hda_machines);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel Common ACPI Match module");
