//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/arm64/include/asm/cputype.h
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
// Copyright (C) 2012 ARM Ltd.
//

pub const MPIDR_LEVEL_BITS_SHIFT: c_int = 3;

pub const MIDR_REVISION_MASK: c_uint = 0xf;

pub const MIDR_PARTNUM_SHIFT: c_int = 4;

pub const MIDR_ARCHITECTURE_SHIFT: c_int = 16;

pub const MIDR_VARIANT_SHIFT: c_int = 20;

pub const MIDR_IMPLEMENTOR_SHIFT: c_int = 24;

pub const ARM_CPU_IMP_ARM: c_uint = 0x41;
pub const ARM_CPU_IMP_APM: c_uint = 0x50;
pub const ARM_CPU_IMP_CAVIUM: c_uint = 0x43;
pub const ARM_CPU_IMP_BRCM: c_uint = 0x42;
pub const ARM_CPU_IMP_QCOM: c_uint = 0x51;
pub const ARM_CPU_IMP_NVIDIA: c_uint = 0x4E;
pub const ARM_CPU_IMP_FUJITSU: c_uint = 0x46;
pub const ARM_CPU_IMP_HISI: c_uint = 0x48;
pub const ARM_CPU_IMP_APPLE: c_uint = 0x61;
pub const ARM_CPU_IMP_AMPERE: c_uint = 0xC0;
pub const ARM_CPU_IMP_MICROSOFT: c_uint = 0x6D;
pub const ARM_CPU_PART_AEM_V8: c_uint = 0xD0F;
pub const ARM_CPU_PART_FOUNDATION: c_uint = 0xD00;
pub const ARM_CPU_PART_CORTEX_A57: c_uint = 0xD07;
pub const ARM_CPU_PART_CORTEX_A72: c_uint = 0xD08;
pub const ARM_CPU_PART_CORTEX_A53: c_uint = 0xD03;
pub const ARM_CPU_PART_CORTEX_A73: c_uint = 0xD09;
pub const ARM_CPU_PART_CORTEX_A75: c_uint = 0xD0A;
pub const ARM_CPU_PART_CORTEX_A35: c_uint = 0xD04;
pub const ARM_CPU_PART_CORTEX_A55: c_uint = 0xD05;
pub const ARM_CPU_PART_CORTEX_A76: c_uint = 0xD0B;
pub const ARM_CPU_PART_NEOVERSE_N1: c_uint = 0xD0C;
pub const ARM_CPU_PART_CORTEX_A77: c_uint = 0xD0D;
pub const ARM_CPU_PART_CORTEX_A76AE: c_uint = 0xD0E;
pub const ARM_CPU_PART_NEOVERSE_V1: c_uint = 0xD40;
pub const ARM_CPU_PART_CORTEX_A78: c_uint = 0xD41;
pub const ARM_CPU_PART_CORTEX_A78AE: c_uint = 0xD42;
pub const ARM_CPU_PART_CORTEX_X1: c_uint = 0xD44;
pub const ARM_CPU_PART_CORTEX_A510: c_uint = 0xD46;
pub const ARM_CPU_PART_CORTEX_A520: c_uint = 0xD80;
pub const ARM_CPU_PART_CORTEX_A710: c_uint = 0xD47;
pub const ARM_CPU_PART_CORTEX_A715: c_uint = 0xD4D;
pub const ARM_CPU_PART_CORTEX_X2: c_uint = 0xD48;
pub const ARM_CPU_PART_NEOVERSE_N2: c_uint = 0xD49;
pub const ARM_CPU_PART_CORTEX_A78C: c_uint = 0xD4B;
pub const ARM_CPU_PART_CORTEX_X1C: c_uint = 0xD4C;
pub const ARM_CPU_PART_CORTEX_X3: c_uint = 0xD4E;
pub const ARM_CPU_PART_NEOVERSE_V2: c_uint = 0xD4F;
pub const ARM_CPU_PART_CORTEX_A720: c_uint = 0xD81;
pub const ARM_CPU_PART_CORTEX_X4: c_uint = 0xD82;
pub const ARM_CPU_PART_NEOVERSE_V3AE: c_uint = 0xD83;
pub const ARM_CPU_PART_NEOVERSE_V3: c_uint = 0xD84;
pub const ARM_CPU_PART_CORTEX_X925: c_uint = 0xD85;
pub const ARM_CPU_PART_CORTEX_A725: c_uint = 0xD87;
pub const ARM_CPU_PART_CORTEX_A720AE: c_uint = 0xD89;
pub const ARM_CPU_PART_NEOVERSE_N3: c_uint = 0xD8E;
pub const APM_CPU_PART_XGENE: c_uint = 0x000;
pub const APM_CPU_VAR_POTENZA: c_uint = 0x00;
pub const CAVIUM_CPU_PART_THUNDERX: c_uint = 0x0A1;
pub const CAVIUM_CPU_PART_THUNDERX_81XX: c_uint = 0x0A2;
pub const CAVIUM_CPU_PART_THUNDERX_83XX: c_uint = 0x0A3;
pub const CAVIUM_CPU_PART_THUNDERX2: c_uint = 0x0AF;
// OcteonTx2 series
pub const CAVIUM_CPU_PART_OCTX2_98XX: c_uint = 0x0B1;
pub const CAVIUM_CPU_PART_OCTX2_96XX: c_uint = 0x0B2;
pub const CAVIUM_CPU_PART_OCTX2_95XX: c_uint = 0x0B3;
pub const CAVIUM_CPU_PART_OCTX2_95XXN: c_uint = 0x0B4;
pub const CAVIUM_CPU_PART_OCTX2_95XXMM: c_uint = 0x0B5;
pub const CAVIUM_CPU_PART_OCTX2_95XXO: c_uint = 0x0B6;
pub const BRCM_CPU_PART_BRAHMA_B53: c_uint = 0x100;
pub const BRCM_CPU_PART_VULCAN: c_uint = 0x516;
pub const QCOM_CPU_PART_FALKOR_V1: c_uint = 0x800;
pub const QCOM_CPU_PART_FALKOR: c_uint = 0xC00;
pub const QCOM_CPU_PART_KRYO: c_uint = 0x200;
pub const QCOM_CPU_PART_KRYO_2XX_GOLD: c_uint = 0x800;
pub const QCOM_CPU_PART_KRYO_2XX_SILVER: c_uint = 0x801;
pub const QCOM_CPU_PART_KRYO_3XX_GOLD: c_uint = 0x802;
pub const QCOM_CPU_PART_KRYO_3XX_SILVER: c_uint = 0x803;
pub const QCOM_CPU_PART_KRYO_4XX_GOLD: c_uint = 0x804;
pub const QCOM_CPU_PART_KRYO_4XX_SILVER: c_uint = 0x805;
pub const QCOM_CPU_PART_ORYON_X1: c_uint = 0x001;
pub const NVIDIA_CPU_PART_DENVER: c_uint = 0x003;
pub const NVIDIA_CPU_PART_CARMEL: c_uint = 0x004;
pub const NVIDIA_CPU_PART_OLYMPUS: c_uint = 0x010;
pub const FUJITSU_CPU_PART_A64FX: c_uint = 0x001;
pub const HISI_CPU_PART_TSV110: c_uint = 0xD01;
pub const HISI_CPU_PART_HIP09: c_uint = 0xD02;
pub const HISI_CPU_PART_HIP12: c_uint = 0xD06;
pub const APPLE_CPU_PART_M1_ICESTORM: c_uint = 0x022;
pub const APPLE_CPU_PART_M1_FIRESTORM: c_uint = 0x023;
pub const APPLE_CPU_PART_M1_ICESTORM_PRO: c_uint = 0x024;
pub const APPLE_CPU_PART_M1_FIRESTORM_PRO: c_uint = 0x025;
pub const APPLE_CPU_PART_M1_ICESTORM_MAX: c_uint = 0x028;
pub const APPLE_CPU_PART_M1_FIRESTORM_MAX: c_uint = 0x029;
pub const APPLE_CPU_PART_M2_BLIZZARD: c_uint = 0x032;
pub const APPLE_CPU_PART_M2_AVALANCHE: c_uint = 0x033;
pub const APPLE_CPU_PART_M2_BLIZZARD_PRO: c_uint = 0x034;
pub const APPLE_CPU_PART_M2_AVALANCHE_PRO: c_uint = 0x035;
pub const APPLE_CPU_PART_M2_BLIZZARD_MAX: c_uint = 0x038;
pub const APPLE_CPU_PART_M2_AVALANCHE_MAX: c_uint = 0x039;
pub const AMPERE_CPU_PART_AMPERE1: c_uint = 0xAC3;
pub const AMPERE_CPU_PART_AMPERE1A: c_uint = 0xAC4;
pub const MICROSOFT_CPU_PART_AZURE_COBALT_100: c_uint = 0xD49 /* Based on r0p0 of ARM Neoverse N2 */;

//
// NOTES:
// - Qualcomm Kryo 5XX Prime / Gold ID themselves as MIDR_CORTEX_A77
// - Qualcomm Kryo 5XX Silver IDs itself as MIDR_QCOM_KRYO_4XX_SILVER
// - Qualcomm Kryo 6XX Prime IDs itself as MIDR_CORTEX_X1
// - Qualcomm Kryo 6XX Gold IDs itself as ARM_CPU_PART_CORTEX_A78
// - Qualcomm Kryo 6XX Silver IDs itself as MIDR_CORTEX_A55
//

// Fujitsu Erratum 010001 affects A64FX 1.0 and 1.1, (v0r0 and v1r0)

//
// Represent a range of MIDR values for a given CPU model and a
// range of variant/revision values.
//
// @model	- CPU model as defined by MIDR_CPU_MODEL
// @rv_min	- Minimum value for the revision/variant as defined by
// MIDR_CPU_VAR_REV
// @rv_max	- Maximum value for the variant/revision for the range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct midr_range {
    pub model: u32,
    pub rv_min: u32,
    pub rv_max: u32,
}

//
// The CPU ID never changes at run time, so we might as well tell the
// compiler that it's constant.  Use this function to read the CPU ID
// rather than directly reading processor_id or read_cpuid() directly.
//
extern "C" {
    pub fn read_cpuid(_arg: MIDR_EL1) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_impl_cpu {
    pub midr: u64,
    pub revidr: u64,
    pub aidr: u64,
}

extern "C" {
    pub fn cpu_errata_set_target_impl(num: u64, impl_cpus: *mut c_void) -> bool;
}
extern "C" {
    pub fn read_cpuid(_arg: MPIDR_EL1) -> return;
}
extern "C" {
    pub fn MIDR_IMPLEMENTOR(_arg: read_cpuid_id()) -> return;
}
extern "C" {
    pub fn MIDR_PARTNUM(_arg: read_cpuid_id()) -> return;
}
extern "C" {
    pub fn read_cpuid(_arg: CTR_EL0) -> return;
}

