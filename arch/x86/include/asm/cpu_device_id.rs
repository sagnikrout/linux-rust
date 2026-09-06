//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpu_device_id.h
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
// Can't use <linux/bitfield.h> because it generates expressions that
// cannot be used in structure initializers. Bitfield construction
// here must match the union in struct cpuinfo_86:
// union {
// struct {
// __u8	x86_model;
// __u8	x86;
// __u8	x86_vendor;
// __u8	x86_reserved;
// };
// __u32		x86_vfm;
// };
//
pub const VFM_MODEL_BIT: c_int = 0;
pub const VFM_FAMILY_BIT: c_int = 8;
pub const VFM_VENDOR_BIT: c_int = 16;
pub const VFM_RSVD_BIT: c_int = 24;

//
// Declare drivers belonging to specific x86 CPUs
// Similar in spirit to pci_device_id and related PCI functions
//

// Get the INTEL_FAM* model defines

// And the X86_VENDOR_* ones

// Centaur FAM6 models
pub const X86_CENTAUR_FAM6_C7_A: c_uint = 0xa;
pub const X86_CENTAUR_FAM6_C7_D: c_uint = 0xd;
pub const X86_CENTAUR_FAM6_NANO: c_uint = 0xf;
// x86_cpu_id::flags

//
// X86_MATCH_CPU -  Base macro for CPU matching
// @_vendor:	The vendor name, e.g. INTEL, AMD, HYGON, ..., ANY
// The name is expanded to X86_VENDOR_@_vendor
// @_family:	The family number or X86_FAMILY_ANY
// @_model:	The model number, model constant or X86_MODEL_ANY
// @_steppings:	Bitmask for steppings, stepping constant or X86_STEPPING_ANY
// @_feature:	A X86_FEATURE bit or X86_FEATURE_ANY
// @_data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is casted to unsigned long internally.
//
// Use only if you need all selectors. Otherwise use one of the shorter
// macros of the X86_MATCH_* family. If there is no matching shorthand
// macro, consider to add one. If you really need to wrap one of the macros
// into another macro at the usage site for good reasons, then please
// start this local macro with X86_MATCH to allow easy grepping.
//

//
// X86_MATCH_VENDOR_FAM_FEATURE - Macro for matching vendor, family and CPU feature
// @vendor:	The vendor name, e.g. INTEL, AMD, HYGON, ..., ANY
// The name is expanded to X86_VENDOR_@vendor
// @family:	The family number or X86_FAMILY_ANY
// @feature:	A X86_FEATURE bit
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is casted to unsigned long internally.
//

//
// X86_MATCH_VENDOR_FEATURE - Macro for matching vendor and CPU feature
// @vendor:	The vendor name, e.g. INTEL, AMD, HYGON, ..., ANY
// The name is expanded to X86_VENDOR_@vendor
// @feature:	A X86_FEATURE bit
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is casted to unsigned long internally.
//

//
// X86_MATCH_FEATURE - Macro for matching a CPU feature
// @feature:	A X86_FEATURE bit
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is casted to unsigned long internally.
//

//
// X86_MATCH_VENDOR_FAM_MODEL - Match vendor, family and model
// @vendor:	The vendor name, e.g. INTEL, AMD, HYGON, ..., ANY
// The name is expanded to X86_VENDOR_@vendor
// @family:	The family number or X86_FAMILY_ANY
// @model:	The model number, model constant or X86_MODEL_ANY
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is casted to unsigned long internally.
//

//
// X86_MATCH_VENDOR_FAM - Match vendor and family
// @vendor:	The vendor name, e.g. INTEL, AMD, HYGON, ..., ANY
// The name is expanded to X86_VENDOR_@vendor
// @family:	The family number or X86_FAMILY_ANY
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is casted to unsigned long internally.
//

//
// X86_MATCH_VFM - Match encoded vendor/family/model
// @vfm:	Encoded 8-bits each for vendor, family, model
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is cast to unsigned long internally.
//

//
// X86_MATCH_VFM_STEPS - Match encoded vendor/family/model and steppings
// range.
// @vfm:	Encoded 8-bits each for vendor, family, model
// @min_step:	Lowest stepping number to match
// @max_step:	Highest stepping number to match
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is cast to unsigned long internally.
//

//
// X86_MATCH_VFM_FEATURE - Match encoded vendor/family/model/feature
// @vfm:	Encoded 8-bits each for vendor, family, model
// @feature:	A X86_FEATURE bit
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is cast to unsigned long internally.
//

//
// X86_MATCH_VFM_CPU_TYPE - Match encoded vendor/family/model/type
// @vfm:	Encoded 8-bits each for vendor, family, model
// @type:	CPU type e.g. P-core, E-core
// @data:	Driver specific data or NULL. The internal storage
// format is unsigned long. The supplied value, pointer
// etc. is cast to unsigned long internally.
//

extern "C" {
    pub fn x86_match_min_microcode_rev(table: *const x86_cpu_id) -> bool;
}
