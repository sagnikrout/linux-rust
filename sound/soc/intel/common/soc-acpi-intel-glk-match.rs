//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/common/soc-acpi-intel-glk-match.c
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
// soc-acpi-intel-glk-match.c - tables and support for GLK ACPI enumeration.
//
// Copyright (c) 2018, Intel Corporation.
//

    static const struct snd_soc_acpi_codecs essx_83x6 = {
    .num_codecs = 3,
    .codecs = { "ESSX8316", "ESSX8326", "ESSX8336"},
    };
    static const struct snd_soc_acpi_codecs glk_codecs = {
    .num_codecs = 1,
    .codecs = {"MX98357A"}
    };
    static const struct snd_soc_acpi_codecs glk_rt5682_rt5682s_hp = {
    .num_codecs = 2,
    .codecs = {"10EC5682", "RTL5682"},
    };
    struct snd_soc_acpi_mach snd_soc_acpi_intel_glk_machines[] = {
    {
    .id = "INT343A",
    .drv_name = "glk_alc298s_i2s",
    .fw_filename = "intel/dsp_fw_glk.bin",
    .sof_tplg_filename = "sof-glk-alc298.tplg",
    },
    {
    .id = "DLGS7219",
    .drv_name = "glk_da7219_def",
    .fw_filename = "intel/dsp_fw_glk.bin",
    .machine_quirk = snd_soc_acpi_codec_list,
    .quirk_data = &glk_codecs,
    .sof_tplg_filename = "sof-glk-da7219.tplg",
    },
    {
    .comp_ids = &glk_rt5682_rt5682s_hp,
    .drv_name = "glk_rt5682_def",
    .fw_filename = "intel/dsp_fw_glk.bin",
    .machine_quirk = snd_soc_acpi_codec_list,
    .quirk_data = &glk_codecs,
    .sof_tplg_filename = "sof-glk-rt5682.tplg",
    },
    {
    .id = "10134242",
    .drv_name = "glk_cs4242_mx98357a",
    .fw_filename = "intel/dsp_fw_glk.bin",
    .machine_quirk = snd_soc_acpi_codec_list,
    .quirk_data = &glk_codecs,
    .sof_tplg_filename = "sof-glk-cs42l42.tplg",
    },
    {
    .comp_ids = &essx_83x6,
    .drv_name = "sof-essx8336",
    .sof_tplg_filename = "sof-glk-es8336", /* the tplg suffix is added at run time */
    .tplg_quirk_mask = SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
    SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
    SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
    },
    {},
    };
    EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_glk_machines);
