//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/common/soc-acpi-intel-cnl-match.c
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
// soc-acpi-intel-cnl-match.c - tables and support for CNL ACPI enumeration.
//
// Copyright (c) 2018, Intel Corporation.
//

    static const struct snd_soc_acpi_codecs essx_83x6 = {
    .num_codecs = 3,
    .codecs = { "ESSX8316", "ESSX8326", "ESSX8336"},
    };
    struct snd_soc_acpi_mach snd_soc_acpi_intel_cnl_machines[] = {
    {
    .id = "INT34C2",
    .drv_name = "cnl_rt274",
    .fw_filename = "intel/dsp_fw_cnl.bin",
    .sof_tplg_filename = "sof-cnl-rt274.tplg",
    },
    {
    .comp_ids = &essx_83x6,
    .drv_name = "sof-essx8336",
// cnl and cml are identical
    .sof_tplg_filename = "sof-cml-es8336", /* the tplg suffix is added at run time */
    .tplg_quirk_mask = SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER |
    SND_SOC_ACPI_TPLG_INTEL_SSP_MSB |
    SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER,
    },
    {},
    };
    EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cnl_machines);
    static const struct snd_soc_acpi_endpoint single_endpoint = {
    .num = 0,
    .aggregated = 0,
    .group_position = 0,
    .group_id = 0,
    };
    static const struct snd_soc_acpi_adr_device rt5682_2_adr[] = {
    {
    .adr = 0x000220025D568200ull,
    .num_endpoints = 1,
    .endpoints = &single_endpoint,
    .name_prefix = "rt5682"
    }
    };
    static const struct snd_soc_acpi_link_adr up_extreme_rt5682_2[] = {
    {
    .mask = BIT(2),
    .num_adr = ARRAY_SIZE(rt5682_2_adr),
    .adr_d = rt5682_2_adr,
    },
    {}
    };
    struct snd_soc_acpi_mach snd_soc_acpi_intel_cnl_sdw_machines[] = {
    {
    .link_mask = BIT(2),
    .links = up_extreme_rt5682_2,
    .drv_name = "sof_sdw",
    .sof_tplg_filename = "sof-cnl-rt5682-sdw2.tplg"
    },
    {
    .link_mask = GENMASK(3, 0),
    .links = sdw_mockup_headset_2amps_mic,
    .drv_name = "sof_sdw",
    .sof_tplg_filename = "sof-cml-rt711-rt1308-rt715.tplg",
    },
    {
    .link_mask = BIT(0) | BIT(1) | BIT(3),
    .links = sdw_mockup_headset_1amp_mic,
    .drv_name = "sof_sdw",
    .sof_tplg_filename = "sof-cml-rt711-rt1308-mono-rt715.tplg",
    },
    {}
    };
    EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cnl_sdw_machines);
