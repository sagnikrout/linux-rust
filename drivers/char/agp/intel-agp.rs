//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/agp/intel-agp.h
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
//
// Common Intel AGPGART and GTT definitions.
//
// Intel registers
pub const INTEL_APSIZE: c_uint = 0xb4;
pub const INTEL_ATTBASE: c_uint = 0xb8;
pub const INTEL_AGPCTRL: c_uint = 0xb0;
pub const INTEL_NBXCFG: c_uint = 0x50;
pub const INTEL_ERRSTS: c_uint = 0x91;
// Intel i830 registers
pub const I830_GMCH_CTRL: c_uint = 0x52;
pub const I830_GMCH_ENABLED: c_uint = 0x4;
pub const I830_GMCH_MEM_MASK: c_uint = 0x1;
pub const I830_GMCH_MEM_64M: c_uint = 0x1;
pub const I830_GMCH_MEM_128M: c_int = 0;
pub const I830_GMCH_GMS_MASK: c_uint = 0x70;
pub const I830_GMCH_GMS_DISABLED: c_uint = 0x00;
pub const I830_GMCH_GMS_LOCAL: c_uint = 0x10;
pub const I830_GMCH_GMS_STOLEN_512: c_uint = 0x20;
pub const I830_GMCH_GMS_STOLEN_1024: c_uint = 0x30;
pub const I830_GMCH_GMS_STOLEN_8192: c_uint = 0x40;
pub const I830_RDRAM_CHANNEL_TYPE: c_uint = 0x03010;

// This one is for I830MP w. an external graphic card
pub const INTEL_I830_ERRSTS: c_uint = 0x92;
// Intel 855GM/852GM registers
pub const I855_GMCH_GMS_MASK: c_uint = 0xF0;
pub const I855_GMCH_GMS_STOLEN_0M: c_uint = 0x0;

pub const I85X_CAPID: c_uint = 0x44;
pub const I85X_VARIANT_MASK: c_uint = 0x7;
pub const I85X_VARIANT_SHIFT: c_int = 5;
pub const I855_GME: c_uint = 0x0;
pub const I855_GM: c_uint = 0x4;
pub const I852_GME: c_uint = 0x2;
pub const I852_GM: c_uint = 0x5;
// Intel i845 registers
pub const INTEL_I845_AGPM: c_uint = 0x51;
pub const INTEL_I845_ERRSTS: c_uint = 0xc8;
// Intel i860 registers
pub const INTEL_I860_MCHCFG: c_uint = 0x50;
pub const INTEL_I860_ERRSTS: c_uint = 0xc8;
// Intel i810 registers
pub const I810_GMADR_BAR: c_int = 0;
pub const I810_MMADR_BAR: c_int = 1;
pub const I810_PTE_BASE: c_uint = 0x10000;
pub const I810_PTE_MAIN_UNCACHED: c_uint = 0x00000000;
pub const I810_PTE_LOCAL: c_uint = 0x00000002;
pub const I810_PTE_VALID: c_uint = 0x00000001;
pub const I830_PTE_SYSTEM_CACHED: c_uint = 0x00000006;
pub const I810_SMRAM_MISCC: c_uint = 0x70;
pub const I810_GFX_MEM_WIN_SIZE: c_uint = 0x00010000;
pub const I810_GFX_MEM_WIN_32M: c_uint = 0x00010000;
pub const I810_GMS: c_uint = 0x000000c0;
pub const I810_GMS_DISABLE: c_uint = 0x00000000;
pub const I810_PGETBL_CTL: c_uint = 0x2020;
pub const I810_PGETBL_ENABLED: c_uint = 0x00000001;
// Note: PGETBL_CTL2 has a different offset on G33.
pub const I965_PGETBL_CTL2: c_uint = 0x20c4;
pub const I965_PGETBL_SIZE_MASK: c_uint = 0x0000000e;

pub const GFX_FLSH_CNTL: c_uint = 0x2170 /* 915+ */;
pub const I810_DRAM_CTL: c_uint = 0x3000;
pub const I810_DRAM_ROW_0: c_uint = 0x00000001;
pub const I810_DRAM_ROW_0_SDRAM: c_uint = 0x00000001;
// Intel 815 register
pub const INTEL_815_APCONT: c_uint = 0x51;

// Intel i820 registers
pub const INTEL_I820_RDCR: c_uint = 0x51;
pub const INTEL_I820_ERRSTS: c_uint = 0xc8;
// Intel i840 registers
pub const INTEL_I840_MCHCFG: c_uint = 0x50;
pub const INTEL_I840_ERRSTS: c_uint = 0xc8;
// Intel i850 registers
pub const INTEL_I850_MCHCFG: c_uint = 0x50;
pub const INTEL_I850_ERRSTS: c_uint = 0xc8;
// intel 915G registers
pub const I915_GMADR_BAR: c_int = 2;
pub const I915_MMADR_BAR: c_int = 0;
pub const I915_PTE_BAR: c_int = 3;

pub const I915_IFPADDR: c_uint = 0x60;
pub const I830_HIC: c_uint = 0x70;
// Intel 965G registers
pub const I965_MSAC: c_uint = 0x62;
pub const I965_IFPADDR: c_uint = 0x70;
// Intel 7505 registers
pub const INTEL_I7505_APSIZE: c_uint = 0x74;
pub const INTEL_I7505_NCAPID: c_uint = 0x60;
pub const INTEL_I7505_NISTAT: c_uint = 0x6c;
pub const INTEL_I7505_ATTBASE: c_uint = 0x78;
pub const INTEL_I7505_ERRSTS: c_uint = 0x42;
pub const INTEL_I7505_AGPCTRL: c_uint = 0x70;
pub const INTEL_I7505_MCHCFG: c_uint = 0x50;
// pci devices ids
pub const PCI_DEVICE_ID_INTEL_E7221_HB: c_uint = 0x2588;
pub const PCI_DEVICE_ID_INTEL_E7221_IG: c_uint = 0x258a;
pub const PCI_DEVICE_ID_INTEL_82946GZ_HB: c_uint = 0x2970;
pub const PCI_DEVICE_ID_INTEL_82946GZ_IG: c_uint = 0x2972;
pub const PCI_DEVICE_ID_INTEL_82G35_HB: c_uint = 0x2980;
pub const PCI_DEVICE_ID_INTEL_82G35_IG: c_uint = 0x2982;
pub const PCI_DEVICE_ID_INTEL_82965Q_HB: c_uint = 0x2990;
pub const PCI_DEVICE_ID_INTEL_82965Q_IG: c_uint = 0x2992;
pub const PCI_DEVICE_ID_INTEL_82965G_HB: c_uint = 0x29A0;
pub const PCI_DEVICE_ID_INTEL_82965G_IG: c_uint = 0x29A2;
pub const PCI_DEVICE_ID_INTEL_82965GM_HB: c_uint = 0x2A00;
pub const PCI_DEVICE_ID_INTEL_82965GM_IG: c_uint = 0x2A02;
pub const PCI_DEVICE_ID_INTEL_82965GME_HB: c_uint = 0x2A10;
pub const PCI_DEVICE_ID_INTEL_82965GME_IG: c_uint = 0x2A12;
pub const PCI_DEVICE_ID_INTEL_82945GME_HB: c_uint = 0x27AC;
pub const PCI_DEVICE_ID_INTEL_82945GME_IG: c_uint = 0x27AE;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_M_HB: c_uint = 0xA010;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_M_IG: c_uint = 0xA011;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_HB: c_uint = 0xA000;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_IG: c_uint = 0xA001;
pub const PCI_DEVICE_ID_INTEL_G33_HB: c_uint = 0x29C0;
pub const PCI_DEVICE_ID_INTEL_G33_IG: c_uint = 0x29C2;
pub const PCI_DEVICE_ID_INTEL_Q35_HB: c_uint = 0x29B0;
pub const PCI_DEVICE_ID_INTEL_Q35_IG: c_uint = 0x29B2;
pub const PCI_DEVICE_ID_INTEL_Q33_HB: c_uint = 0x29D0;
pub const PCI_DEVICE_ID_INTEL_Q33_IG: c_uint = 0x29D2;
pub const PCI_DEVICE_ID_INTEL_B43_HB: c_uint = 0x2E40;
pub const PCI_DEVICE_ID_INTEL_B43_IG: c_uint = 0x2E42;
pub const PCI_DEVICE_ID_INTEL_B43_1_HB: c_uint = 0x2E90;
pub const PCI_DEVICE_ID_INTEL_B43_1_IG: c_uint = 0x2E92;
pub const PCI_DEVICE_ID_INTEL_GM45_HB: c_uint = 0x2A40;
pub const PCI_DEVICE_ID_INTEL_GM45_IG: c_uint = 0x2A42;
pub const PCI_DEVICE_ID_INTEL_EAGLELAKE_HB: c_uint = 0x2E00;
pub const PCI_DEVICE_ID_INTEL_EAGLELAKE_IG: c_uint = 0x2E02;
pub const PCI_DEVICE_ID_INTEL_Q45_HB: c_uint = 0x2E10;
pub const PCI_DEVICE_ID_INTEL_Q45_IG: c_uint = 0x2E12;
pub const PCI_DEVICE_ID_INTEL_G45_HB: c_uint = 0x2E20;
pub const PCI_DEVICE_ID_INTEL_G45_IG: c_uint = 0x2E22;
pub const PCI_DEVICE_ID_INTEL_G41_HB: c_uint = 0x2E30;
pub const PCI_DEVICE_ID_INTEL_G41_IG: c_uint = 0x2E32;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_D_HB: c_uint = 0x0040;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_D2_HB: c_uint = 0x0069;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_D_IG: c_uint = 0x0042;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_M_HB: c_uint = 0x0044;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_MA_HB: c_uint = 0x0062;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_MC2_HB: c_uint = 0x006a;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_M_IG: c_uint = 0x0046;
