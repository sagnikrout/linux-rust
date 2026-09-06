//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/common/soc-acpi-intel-hsw-bdw-match.c
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
// soc-acpi-intel-hsw-bdw-match.c - tables and support for ACPI enumeration.
//
// Copyright (c) 2017, Intel Corporation.
//

    struct snd_soc_acpi_mach snd_soc_acpi_intel_broadwell_machines[] = {
    {
    .id = "INT343A",
    .drv_name = "bdw_rt286",
    .sof_tplg_filename = "sof-bdw-rt286.tplg",
    },
    {
    .id = "10EC5650",
    .drv_name = "bdw-rt5650",
    .sof_tplg_filename = "sof-bdw-rt5650.tplg",
    },
    {
    .id = "RT5677CE",
    .drv_name = "bdw-rt5677",
    .sof_tplg_filename = "sof-bdw-rt5677.tplg",
    },
    {
    .id = "INT33CA",
    .drv_name = "hsw_rt5640",
    .sof_tplg_filename = "sof-bdw-rt5640.tplg",
    },
    {}
    };
    EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_broadwell_machines);
