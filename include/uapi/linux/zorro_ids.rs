//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/zorro_ids.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Zorro board IDs
//
// Please keep sorted.
//
pub const ZORRO_MANUF_PACIFIC_PERIPHERALS: c_uint = 0x00D3;

pub const ZORRO_MANUF_MACROSYSTEMS_USA_2: c_uint = 0x0100;

pub const ZORRO_MANUF_KUPKE_1: c_uint = 0x00DD;

pub const ZORRO_MANUF_MEMPHIS: c_uint = 0x0100;

pub const ZORRO_MANUF_3_STATE: c_uint = 0x0200;

pub const ZORRO_MANUF_COMMODORE_BRAUNSCHWEIG: c_uint = 0x0201;

pub const ZORRO_MANUF_COMMODORE_WEST_CHESTER_1: c_uint = 0x0202;

pub const ZORRO_MANUF_COMMODORE_WEST_CHESTER_2: c_uint = 0x0203;

pub const ZORRO_MANUF_PROGRESSIVE_PERIPHERALS_AND_SYSTEMS_2: c_uint = 0x02F4;

pub const ZORRO_MANUF_KOLFF_COMPUTER_SUPPLIES: c_uint = 0x02FF;

pub const ZORRO_MANUF_CARDCO_1: c_uint = 0x03EC;

pub const ZORRO_MANUF_A_SQUARED: c_uint = 0x03ED;

pub const ZORRO_MANUF_COMSPEC_COMMUNICATIONS: c_uint = 0x03EE;

pub const ZORRO_MANUF_ANAKIN_RESEARCH: c_uint = 0x03F1;

pub const ZORRO_MANUF_MICROBOTICS: c_uint = 0x03F2;

pub const ZORRO_MANUF_ACCESS_ASSOCIATES_ALEGRA: c_uint = 0x03F4;
pub const ZORRO_MANUF_EXPANSION_TECHNOLOGIES: c_uint = 0x03F6;
pub const ZORRO_MANUF_ASDG: c_uint = 0x03FF;

pub const ZORRO_MANUF_IMTRONICS_1: c_uint = 0x0404;

pub const ZORRO_MANUF_CBM_UNIVERSITY_OF_LOWELL: c_uint = 0x0406;

pub const ZORRO_MANUF_AMERISTAR: c_uint = 0x041D;

pub const ZORRO_MANUF_SUPRA: c_uint = 0x0420;

pub const ZORRO_MANUF_COMPUTER_SYSTEMS_ASSOCIATES: c_uint = 0x0422;

pub const ZORRO_MANUF_MARC_MICHAEL_GROTH: c_uint = 0x0439;
pub const ZORRO_MANUF_M_TECH: c_uint = 0x0502;

pub const ZORRO_MANUF_GREAT_VALLEY_PRODUCTS_1: c_uint = 0x06E1;

pub const ZORRO_MANUF_BYTEBOX: c_uint = 0x07DA;

pub const ZORRO_MANUF_DKB_POWER_COMPUTING: c_uint = 0x07DC;

pub const ZORRO_MANUF_GREAT_VALLEY_PRODUCTS_2: c_uint = 0x07E1;

// #define  ZORRO_PROD_GVP_A2000_030				ZORRO_ID(GREAT_VALLEY_PRODUCTS_2, 0x0D, 0)
// #define  ZORRO_PROD_GVP_GFORCE_040_SCSI_2			ZORRO_ID(GREAT_VALLEY_PRODUCTS_2, 0x0D, 0)

pub const ZORRO_MANUF_CALIFORNIA_ACCESS_SYNERGY: c_uint = 0x07E5;

pub const ZORRO_MANUF_XETEC: c_uint = 0x07E6;

pub const ZORRO_MANUF_PROGRESSIVE_PERIPHERALS_AND_SYSTEMS: c_uint = 0x07EA;

pub const ZORRO_MANUF_XEBEC: c_uint = 0x07EC;
pub const ZORRO_MANUF_SPIRIT_TECHNOLOGY: c_uint = 0x07F2;

pub const ZORRO_MANUF_SPIRIT_TECHNOLOGY_2: c_uint = 0x07F3;
pub const ZORRO_MANUF_BSC_ALFADATA_1: c_uint = 0x07FE;

pub const ZORRO_MANUF_BSC_ALFADATA_2: c_uint = 0x0801;

pub const ZORRO_MANUF_CARDCO_2: c_uint = 0x0802;

pub const ZORRO_MANUF_JOCHHEIM: c_uint = 0x0804;

pub const ZORRO_MANUF_CHECKPOINT_TECHNOLOGIES: c_uint = 0x0807;

pub const ZORRO_MANUF_EDOTRONIK: c_uint = 0x0810;

pub const ZORRO_MANUF_NES_INC: c_uint = 0x0813;

pub const ZORRO_MANUF_ICD: c_uint = 0x0817;

pub const ZORRO_MANUF_KUPKE_2: c_uint = 0x0819;

pub const ZORRO_MANUF_GREAT_VALLEY_PRODUCTS_3: c_uint = 0x081D;

pub const ZORRO_MANUF_INTERWORKS_NETWORK: c_uint = 0x081E;
pub const ZORRO_MANUF_HARDITAL_SYNTHESIS: c_uint = 0x0820;

pub const ZORRO_MANUF_APPLIED_ENGINEERING: c_uint = 0x0828;

pub const ZORRO_MANUF_BSC_ALFADATA_3: c_uint = 0x082C;

pub const ZORRO_MANUF_PHOENIX: c_uint = 0x0835;

pub const ZORRO_MANUF_ADVANCED_STORAGE_SYSTEMS: c_uint = 0x0836;

pub const ZORRO_MANUF_IMPULSE: c_uint = 0x0838;

pub const ZORRO_MANUF_IVS: c_uint = 0x0840;

pub const ZORRO_MANUF_VECTOR_1: c_uint = 0x0841;

pub const ZORRO_MANUF_XPERT_PRODEV: c_uint = 0x0845;

pub const ZORRO_MANUF_HYDRA_SYSTEMS: c_uint = 0x0849;

pub const ZORRO_MANUF_SUNRIZE_INDUSTRIES: c_uint = 0x084F;

pub const ZORRO_MANUF_TRICERATOPS: c_uint = 0x0850;

pub const ZORRO_MANUF_APPLIED_MAGIC: c_uint = 0x0851;

pub const ZORRO_MANUF_GFX_BASE: c_uint = 0x085E;

pub const ZORRO_MANUF_ROCTEC: c_uint = 0x0860;

pub const ZORRO_MANUF_KATO: c_uint = 0x0861;

// ID clash!!
pub const ZORRO_MANUF_HELFRICH_1: c_uint = 0x0861;

pub const ZORRO_MANUF_ATLANTIS: c_uint = 0x0862;
pub const ZORRO_MANUF_PROTAR: c_uint = 0x0864;
pub const ZORRO_MANUF_ACS: c_uint = 0x0865;
pub const ZORRO_MANUF_SOFTWARE_RESULTS_ENTERPRISES: c_uint = 0x0866;

pub const ZORRO_MANUF_MASOBOSHI: c_uint = 0x086D;

pub const ZORRO_MANUF_MAINHATTAN_DATA: c_uint = 0x086F;

pub const ZORRO_MANUF_VILLAGE_TRONIC: c_uint = 0x0877;

pub const ZORRO_MANUF_UTILITIES_UNLIMITED: c_uint = 0x087B;

pub const ZORRO_MANUF_AMITRIX: c_uint = 0x0880;

pub const ZORRO_MANUF_ARMAX: c_uint = 0x0885;

pub const ZORRO_MANUF_ZEUS: c_uint = 0x088D;

pub const ZORRO_MANUF_NEWTEK: c_uint = 0x088F;

pub const ZORRO_MANUF_M_TECH_GERMANY: c_uint = 0x0890;

pub const ZORRO_MANUF_GREAT_VALLEY_PRODUCTS_4: c_uint = 0x0891;

pub const ZORRO_MANUF_APOLLO_1: c_uint = 0x0892;

pub const ZORRO_MANUF_HELFRICH_2: c_uint = 0x0893;

pub const ZORRO_MANUF_MACROSYSTEMS_USA: c_uint = 0x089B;

pub const ZORRO_MANUF_ELBOX_COMPUTER: c_uint = 0x089E;

pub const ZORRO_MANUF_HARMS_PROFESSIONAL: c_uint = 0x0A00;

pub const ZORRO_MANUF_MICRONIK: c_uint = 0x0A50;

pub const ZORRO_MANUF_MICRONIK2: c_uint = 0x0F0F;

pub const ZORRO_MANUF_MEGAMICRO: c_uint = 0x1000;

pub const ZORRO_MANUF_IMTRONICS_2: c_uint = 0x1028;

// unofficial ID
pub const ZORRO_MANUF_INDIVIDUAL_COMPUTERS: c_uint = 0x1212;

pub const ZORRO_MANUF_KUPKE_3: c_uint = 0x1248;

pub const ZORRO_MANUF_ITH: c_uint = 0x1388;

pub const ZORRO_MANUF_VMC: c_uint = 0x1389;

pub const ZORRO_MANUF_CSLAB: c_uint = 0x1400;

pub const ZORRO_MANUF_INFORMATION: c_uint = 0x157C;

pub const ZORRO_MANUF_VORTEX: c_uint = 0x2017;

pub const ZORRO_MANUF_EXPANSION_SYSTEMS: c_uint = 0x2062;

pub const ZORRO_MANUF_READYSOFT: c_uint = 0x2100;

pub const ZORRO_MANUF_PHASE5: c_uint = 0x2140;

pub const ZORRO_MANUF_DPS: c_uint = 0x2169;

pub const ZORRO_MANUF_APOLLO_2: c_uint = 0x2200;

pub const ZORRO_MANUF_APOLLO_3: c_uint = 0x2222;

pub const ZORRO_MANUF_PETSOFF_LP: c_uint = 0x38A5;

pub const ZORRO_MANUF_UWE_GERLACH: c_uint = 0x3FF7;

pub const ZORRO_MANUF_ACT: c_uint = 0x4231;

pub const ZORRO_MANUF_MACROSYSTEMS_GERMANY: c_uint = 0x4754;

pub const ZORRO_MANUF_COMBITEC: c_uint = 0x6766;
pub const ZORRO_MANUF_SKI_PERIPHERALS: c_uint = 0x8000;

pub const ZORRO_MANUF_REIS_WARE_2: c_uint = 0xA9AD;

pub const ZORRO_MANUF_CAMERON: c_uint = 0xAA01;

pub const ZORRO_MANUF_REIS_WARE: c_uint = 0xAA11;

pub const ZORRO_MANUF_PHOENIX_2: c_uint = 0xB5A8;

pub const ZORRO_MANUF_COMBITEC_2: c_uint = 0xC008;

//
// Test and illegal Manufacturer IDs.
//
pub const ZORRO_MANUF_HACKER: c_uint = 0x07DB;

