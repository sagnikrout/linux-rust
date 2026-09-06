//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uv/uv_mmrs.h
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


//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// HPE UV MMR definitions
//
// (C) Copyright 2020 Hewlett Packard Enterprise Development LP
// Copyright (C) 2007-2016 Silicon Graphics, Inc. All rights reserved.
//
// This file contains MMR definitions for all UV hubs types.
//
// To minimize coding differences between hub types, the symbols are
// grouped by architecture types.
//
// UVH  - definitions common to all UV hub types.
// UVXH - definitions common to UVX class (2, 3, 4).
// UVYH - definitions common to UVY class (5).
// UV5H - definitions specific to UV type 5 hub.
// UV4AH - definitions specific to UV type 4A hub.
// UV4H - definitions specific to UV type 4 hub.
// UV3H - definitions specific to UV type 3 hub.
// UV2H - definitions specific to UV type 2 hub.
//
// If the MMR exists on all hub types but have different addresses,
// use a conditional operator to define the value at runtime.  Any
// that are not defined are blank.
// (UV4A variations only generated if different from uv4)
// #define UVHxxx (
// is_uv(UV5) ? UV5Hxxx value :
// is_uv(UV4A) ? UV4AHxxx value :
// is_uv(UV4) ? UV4Hxxx value :
// is_uv(UV3) ? UV3Hxxx value :
// is_uv(UV2) ? UV2Hxxx value :
// <ucv> or <undef value>)
//
// Class UVX has UVs (2|3|4|4A).
// Class UVY has UVs (5).
//
// union uvh_xxx {
// unsigned long       v;
// struct uvh_xxx_s {	 # Common fields only
// } s;
// struct uv5h_xxx_s {	 # Full UV5 definition (*)
// } s5;
// struct uv4ah_xxx_s {	 # Full UV4A definition (*)
// } s4a;
// struct uv4h_xxx_s {	 # Full UV4 definition (*)
// } s4;
// struct uv3h_xxx_s {	 # Full UV3 definition (*)
// } s3;
// struct uv2h_xxx_s {	 # Full UV2 definition (*)
// } s2;
// };
// (* - if present and different than the common struct)
//
// Only essential differences are enumerated. For example, if the address is
// the same for all UV's, only a single #define is generated. Likewise,
// if the contents is the same for all hubs, only the "s" structure is
// generated.
//
// (GEN Flags: undefs=function)
//
// UV bit masks

pub const UV1_HUB_PART_NUMBER: c_uint = 0x88a5;
pub const UV2_HUB_PART_NUMBER: c_uint = 0x8eb8;
pub const UV2_HUB_PART_NUMBER_X: c_uint = 0x1111;
pub const UV3_HUB_PART_NUMBER: c_uint = 0x9578;
pub const UV3_HUB_PART_NUMBER_X: c_uint = 0x4321;
pub const UV4_HUB_PART_NUMBER: c_uint = 0x99a1;
pub const UV5_HUB_PART_NUMBER: c_uint = 0xa171;
// Error function to catch undefined references
extern "C" {
    pub fn uv_undefined(str: *mut c_char) -> c_ulong;
}
// =========================================================================
// UVH_EVENT_OCCURRED0
// =========================================================================
pub const UVH_EVENT_OCCURRED0: c_uint = 0x70000UL;
// UVH common defines
pub const UVH_EVENT_OCCURRED0_LB_HCERR_SHFT: c_int = 0;
pub const UVH_EVENT_OCCURRED0_LB_HCERR_MASK: c_uint = 0x0000000000000001UL;
// UVXH common defines
pub const UVXH_EVENT_OCCURRED0_RH_HCERR_SHFT: c_int = 2;
pub const UVXH_EVENT_OCCURRED0_RH_HCERR_MASK: c_uint = 0x0000000000000004UL;
pub const UVXH_EVENT_OCCURRED0_LH0_HCERR_SHFT: c_int = 3;
pub const UVXH_EVENT_OCCURRED0_LH0_HCERR_MASK: c_uint = 0x0000000000000008UL;
pub const UVXH_EVENT_OCCURRED0_LH1_HCERR_SHFT: c_int = 4;
pub const UVXH_EVENT_OCCURRED0_LH1_HCERR_MASK: c_uint = 0x0000000000000010UL;
pub const UVXH_EVENT_OCCURRED0_GR0_HCERR_SHFT: c_int = 5;
pub const UVXH_EVENT_OCCURRED0_GR0_HCERR_MASK: c_uint = 0x0000000000000020UL;
pub const UVXH_EVENT_OCCURRED0_GR1_HCERR_SHFT: c_int = 6;
pub const UVXH_EVENT_OCCURRED0_GR1_HCERR_MASK: c_uint = 0x0000000000000040UL;
pub const UVXH_EVENT_OCCURRED0_NI0_HCERR_SHFT: c_int = 7;
pub const UVXH_EVENT_OCCURRED0_NI0_HCERR_MASK: c_uint = 0x0000000000000080UL;
pub const UVXH_EVENT_OCCURRED0_NI1_HCERR_SHFT: c_int = 8;
pub const UVXH_EVENT_OCCURRED0_NI1_HCERR_MASK: c_uint = 0x0000000000000100UL;
pub const UVXH_EVENT_OCCURRED0_LB_AOERR0_SHFT: c_int = 9;
pub const UVXH_EVENT_OCCURRED0_LB_AOERR0_MASK: c_uint = 0x0000000000000200UL;
pub const UVXH_EVENT_OCCURRED0_RH_AOERR0_SHFT: c_int = 11;
pub const UVXH_EVENT_OCCURRED0_RH_AOERR0_MASK: c_uint = 0x0000000000000800UL;
pub const UVXH_EVENT_OCCURRED0_LH0_AOERR0_SHFT: c_int = 12;
pub const UVXH_EVENT_OCCURRED0_LH0_AOERR0_MASK: c_uint = 0x0000000000001000UL;
pub const UVXH_EVENT_OCCURRED0_LH1_AOERR0_SHFT: c_int = 13;
pub const UVXH_EVENT_OCCURRED0_LH1_AOERR0_MASK: c_uint = 0x0000000000002000UL;
pub const UVXH_EVENT_OCCURRED0_GR0_AOERR0_SHFT: c_int = 14;
pub const UVXH_EVENT_OCCURRED0_GR0_AOERR0_MASK: c_uint = 0x0000000000004000UL;
pub const UVXH_EVENT_OCCURRED0_GR1_AOERR0_SHFT: c_int = 15;
pub const UVXH_EVENT_OCCURRED0_GR1_AOERR0_MASK: c_uint = 0x0000000000008000UL;
pub const UVXH_EVENT_OCCURRED0_XB_AOERR0_SHFT: c_int = 16;
pub const UVXH_EVENT_OCCURRED0_XB_AOERR0_MASK: c_uint = 0x0000000000010000UL;
// UVYH common defines
pub const UVYH_EVENT_OCCURRED0_KT_HCERR_SHFT: c_int = 1;
pub const UVYH_EVENT_OCCURRED0_KT_HCERR_MASK: c_uint = 0x0000000000000002UL;
pub const UVYH_EVENT_OCCURRED0_RH0_HCERR_SHFT: c_int = 2;
pub const UVYH_EVENT_OCCURRED0_RH0_HCERR_MASK: c_uint = 0x0000000000000004UL;
pub const UVYH_EVENT_OCCURRED0_RH1_HCERR_SHFT: c_int = 3;
pub const UVYH_EVENT_OCCURRED0_RH1_HCERR_MASK: c_uint = 0x0000000000000008UL;
pub const UVYH_EVENT_OCCURRED0_LH0_HCERR_SHFT: c_int = 4;
pub const UVYH_EVENT_OCCURRED0_LH0_HCERR_MASK: c_uint = 0x0000000000000010UL;
pub const UVYH_EVENT_OCCURRED0_LH1_HCERR_SHFT: c_int = 5;
pub const UVYH_EVENT_OCCURRED0_LH1_HCERR_MASK: c_uint = 0x0000000000000020UL;
pub const UVYH_EVENT_OCCURRED0_LH2_HCERR_SHFT: c_int = 6;
pub const UVYH_EVENT_OCCURRED0_LH2_HCERR_MASK: c_uint = 0x0000000000000040UL;
pub const UVYH_EVENT_OCCURRED0_LH3_HCERR_SHFT: c_int = 7;
pub const UVYH_EVENT_OCCURRED0_LH3_HCERR_MASK: c_uint = 0x0000000000000080UL;
pub const UVYH_EVENT_OCCURRED0_XB_HCERR_SHFT: c_int = 8;
pub const UVYH_EVENT_OCCURRED0_XB_HCERR_MASK: c_uint = 0x0000000000000100UL;
pub const UVYH_EVENT_OCCURRED0_RDM_HCERR_SHFT: c_int = 9;
pub const UVYH_EVENT_OCCURRED0_RDM_HCERR_MASK: c_uint = 0x0000000000000200UL;
pub const UVYH_EVENT_OCCURRED0_NI0_HCERR_SHFT: c_int = 10;
pub const UVYH_EVENT_OCCURRED0_NI0_HCERR_MASK: c_uint = 0x0000000000000400UL;
pub const UVYH_EVENT_OCCURRED0_NI1_HCERR_SHFT: c_int = 11;
pub const UVYH_EVENT_OCCURRED0_NI1_HCERR_MASK: c_uint = 0x0000000000000800UL;
pub const UVYH_EVENT_OCCURRED0_LB_AOERR0_SHFT: c_int = 12;
pub const UVYH_EVENT_OCCURRED0_LB_AOERR0_MASK: c_uint = 0x0000000000001000UL;
pub const UVYH_EVENT_OCCURRED0_KT_AOERR0_SHFT: c_int = 13;
pub const UVYH_EVENT_OCCURRED0_KT_AOERR0_MASK: c_uint = 0x0000000000002000UL;
pub const UVYH_EVENT_OCCURRED0_RH0_AOERR0_SHFT: c_int = 14;
pub const UVYH_EVENT_OCCURRED0_RH0_AOERR0_MASK: c_uint = 0x0000000000004000UL;
pub const UVYH_EVENT_OCCURRED0_RH1_AOERR0_SHFT: c_int = 15;
pub const UVYH_EVENT_OCCURRED0_RH1_AOERR0_MASK: c_uint = 0x0000000000008000UL;
pub const UVYH_EVENT_OCCURRED0_LH0_AOERR0_SHFT: c_int = 16;
pub const UVYH_EVENT_OCCURRED0_LH0_AOERR0_MASK: c_uint = 0x0000000000010000UL;
pub const UVYH_EVENT_OCCURRED0_LH1_AOERR0_SHFT: c_int = 17;
pub const UVYH_EVENT_OCCURRED0_LH1_AOERR0_MASK: c_uint = 0x0000000000020000UL;
pub const UVYH_EVENT_OCCURRED0_LH2_AOERR0_SHFT: c_int = 18;
pub const UVYH_EVENT_OCCURRED0_LH2_AOERR0_MASK: c_uint = 0x0000000000040000UL;
pub const UVYH_EVENT_OCCURRED0_LH3_AOERR0_SHFT: c_int = 19;
pub const UVYH_EVENT_OCCURRED0_LH3_AOERR0_MASK: c_uint = 0x0000000000080000UL;
pub const UVYH_EVENT_OCCURRED0_XB_AOERR0_SHFT: c_int = 20;
pub const UVYH_EVENT_OCCURRED0_XB_AOERR0_MASK: c_uint = 0x0000000000100000UL;
pub const UVYH_EVENT_OCCURRED0_RDM_AOERR0_SHFT: c_int = 21;
pub const UVYH_EVENT_OCCURRED0_RDM_AOERR0_MASK: c_uint = 0x0000000000200000UL;
pub const UVYH_EVENT_OCCURRED0_RT0_AOERR0_SHFT: c_int = 22;
pub const UVYH_EVENT_OCCURRED0_RT0_AOERR0_MASK: c_uint = 0x0000000000400000UL;
pub const UVYH_EVENT_OCCURRED0_RT1_AOERR0_SHFT: c_int = 23;
pub const UVYH_EVENT_OCCURRED0_RT1_AOERR0_MASK: c_uint = 0x0000000000800000UL;
pub const UVYH_EVENT_OCCURRED0_NI0_AOERR0_SHFT: c_int = 24;
pub const UVYH_EVENT_OCCURRED0_NI0_AOERR0_MASK: c_uint = 0x0000000001000000UL;
pub const UVYH_EVENT_OCCURRED0_NI1_AOERR0_SHFT: c_int = 25;
pub const UVYH_EVENT_OCCURRED0_NI1_AOERR0_MASK: c_uint = 0x0000000002000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_AOERR1_SHFT: c_int = 26;
pub const UVYH_EVENT_OCCURRED0_LB_AOERR1_MASK: c_uint = 0x0000000004000000UL;
pub const UVYH_EVENT_OCCURRED0_KT_AOERR1_SHFT: c_int = 27;
pub const UVYH_EVENT_OCCURRED0_KT_AOERR1_MASK: c_uint = 0x0000000008000000UL;
pub const UVYH_EVENT_OCCURRED0_RH0_AOERR1_SHFT: c_int = 28;
pub const UVYH_EVENT_OCCURRED0_RH0_AOERR1_MASK: c_uint = 0x0000000010000000UL;
pub const UVYH_EVENT_OCCURRED0_RH1_AOERR1_SHFT: c_int = 29;
pub const UVYH_EVENT_OCCURRED0_RH1_AOERR1_MASK: c_uint = 0x0000000020000000UL;
pub const UVYH_EVENT_OCCURRED0_LH0_AOERR1_SHFT: c_int = 30;
pub const UVYH_EVENT_OCCURRED0_LH0_AOERR1_MASK: c_uint = 0x0000000040000000UL;
pub const UVYH_EVENT_OCCURRED0_LH1_AOERR1_SHFT: c_int = 31;
pub const UVYH_EVENT_OCCURRED0_LH1_AOERR1_MASK: c_uint = 0x0000000080000000UL;
pub const UVYH_EVENT_OCCURRED0_LH2_AOERR1_SHFT: c_int = 32;
pub const UVYH_EVENT_OCCURRED0_LH2_AOERR1_MASK: c_uint = 0x0000000100000000UL;
pub const UVYH_EVENT_OCCURRED0_LH3_AOERR1_SHFT: c_int = 33;
pub const UVYH_EVENT_OCCURRED0_LH3_AOERR1_MASK: c_uint = 0x0000000200000000UL;
pub const UVYH_EVENT_OCCURRED0_XB_AOERR1_SHFT: c_int = 34;
pub const UVYH_EVENT_OCCURRED0_XB_AOERR1_MASK: c_uint = 0x0000000400000000UL;
pub const UVYH_EVENT_OCCURRED0_RDM_AOERR1_SHFT: c_int = 35;
pub const UVYH_EVENT_OCCURRED0_RDM_AOERR1_MASK: c_uint = 0x0000000800000000UL;
pub const UVYH_EVENT_OCCURRED0_RT0_AOERR1_SHFT: c_int = 36;
pub const UVYH_EVENT_OCCURRED0_RT0_AOERR1_MASK: c_uint = 0x0000001000000000UL;
pub const UVYH_EVENT_OCCURRED0_RT1_AOERR1_SHFT: c_int = 37;
pub const UVYH_EVENT_OCCURRED0_RT1_AOERR1_MASK: c_uint = 0x0000002000000000UL;
pub const UVYH_EVENT_OCCURRED0_NI0_AOERR1_SHFT: c_int = 38;
pub const UVYH_EVENT_OCCURRED0_NI0_AOERR1_MASK: c_uint = 0x0000004000000000UL;
pub const UVYH_EVENT_OCCURRED0_NI1_AOERR1_SHFT: c_int = 39;
pub const UVYH_EVENT_OCCURRED0_NI1_AOERR1_MASK: c_uint = 0x0000008000000000UL;
pub const UVYH_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_SHFT: c_int = 40;
pub const UVYH_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_MASK: c_uint = 0x0000010000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_0_SHFT: c_int = 41;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_0_MASK: c_uint = 0x0000020000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_1_SHFT: c_int = 42;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_1_MASK: c_uint = 0x0000040000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_2_SHFT: c_int = 43;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_2_MASK: c_uint = 0x0000080000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_3_SHFT: c_int = 44;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_3_MASK: c_uint = 0x0000100000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_4_SHFT: c_int = 45;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_4_MASK: c_uint = 0x0000200000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_5_SHFT: c_int = 46;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_5_MASK: c_uint = 0x0000400000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_6_SHFT: c_int = 47;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_6_MASK: c_uint = 0x0000800000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_7_SHFT: c_int = 48;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_7_MASK: c_uint = 0x0001000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_8_SHFT: c_int = 49;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_8_MASK: c_uint = 0x0002000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_9_SHFT: c_int = 50;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_9_MASK: c_uint = 0x0004000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_10_SHFT: c_int = 51;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_10_MASK: c_uint = 0x0008000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_11_SHFT: c_int = 52;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_11_MASK: c_uint = 0x0010000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_12_SHFT: c_int = 53;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_12_MASK: c_uint = 0x0020000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_13_SHFT: c_int = 54;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_13_MASK: c_uint = 0x0040000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_14_SHFT: c_int = 55;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_14_MASK: c_uint = 0x0080000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_15_SHFT: c_int = 56;
pub const UVYH_EVENT_OCCURRED0_LB_IRQ_INT_15_MASK: c_uint = 0x0100000000000000UL;
pub const UVYH_EVENT_OCCURRED0_L1_NMI_INT_SHFT: c_int = 57;
pub const UVYH_EVENT_OCCURRED0_L1_NMI_INT_MASK: c_uint = 0x0200000000000000UL;
pub const UVYH_EVENT_OCCURRED0_STOP_CLOCK_SHFT: c_int = 58;
pub const UVYH_EVENT_OCCURRED0_STOP_CLOCK_MASK: c_uint = 0x0400000000000000UL;
pub const UVYH_EVENT_OCCURRED0_ASIC_TO_L1_SHFT: c_int = 59;
pub const UVYH_EVENT_OCCURRED0_ASIC_TO_L1_MASK: c_uint = 0x0800000000000000UL;
pub const UVYH_EVENT_OCCURRED0_L1_TO_ASIC_SHFT: c_int = 60;
pub const UVYH_EVENT_OCCURRED0_L1_TO_ASIC_MASK: c_uint = 0x1000000000000000UL;
pub const UVYH_EVENT_OCCURRED0_LA_SEQ_TRIGGER_SHFT: c_int = 61;
pub const UVYH_EVENT_OCCURRED0_LA_SEQ_TRIGGER_MASK: c_uint = 0x2000000000000000UL;
// UV4 unique defines
pub const UV4H_EVENT_OCCURRED0_KT_HCERR_SHFT: c_int = 1;
pub const UV4H_EVENT_OCCURRED0_KT_HCERR_MASK: c_uint = 0x0000000000000002UL;
pub const UV4H_EVENT_OCCURRED0_KT_AOERR0_SHFT: c_int = 10;
pub const UV4H_EVENT_OCCURRED0_KT_AOERR0_MASK: c_uint = 0x0000000000000400UL;
pub const UV4H_EVENT_OCCURRED0_RTQ0_AOERR0_SHFT: c_int = 17;
pub const UV4H_EVENT_OCCURRED0_RTQ0_AOERR0_MASK: c_uint = 0x0000000000020000UL;
pub const UV4H_EVENT_OCCURRED0_RTQ1_AOERR0_SHFT: c_int = 18;
pub const UV4H_EVENT_OCCURRED0_RTQ1_AOERR0_MASK: c_uint = 0x0000000000040000UL;
pub const UV4H_EVENT_OCCURRED0_RTQ2_AOERR0_SHFT: c_int = 19;
pub const UV4H_EVENT_OCCURRED0_RTQ2_AOERR0_MASK: c_uint = 0x0000000000080000UL;
pub const UV4H_EVENT_OCCURRED0_RTQ3_AOERR0_SHFT: c_int = 20;
pub const UV4H_EVENT_OCCURRED0_RTQ3_AOERR0_MASK: c_uint = 0x0000000000100000UL;
pub const UV4H_EVENT_OCCURRED0_NI0_AOERR0_SHFT: c_int = 21;
pub const UV4H_EVENT_OCCURRED0_NI0_AOERR0_MASK: c_uint = 0x0000000000200000UL;
pub const UV4H_EVENT_OCCURRED0_NI1_AOERR0_SHFT: c_int = 22;
pub const UV4H_EVENT_OCCURRED0_NI1_AOERR0_MASK: c_uint = 0x0000000000400000UL;
pub const UV4H_EVENT_OCCURRED0_LB_AOERR1_SHFT: c_int = 23;
pub const UV4H_EVENT_OCCURRED0_LB_AOERR1_MASK: c_uint = 0x0000000000800000UL;
pub const UV4H_EVENT_OCCURRED0_KT_AOERR1_SHFT: c_int = 24;
pub const UV4H_EVENT_OCCURRED0_KT_AOERR1_MASK: c_uint = 0x0000000001000000UL;
pub const UV4H_EVENT_OCCURRED0_RH_AOERR1_SHFT: c_int = 25;
pub const UV4H_EVENT_OCCURRED0_RH_AOERR1_MASK: c_uint = 0x0000000002000000UL;
pub const UV4H_EVENT_OCCURRED0_LH0_AOERR1_SHFT: c_int = 26;
pub const UV4H_EVENT_OCCURRED0_LH0_AOERR1_MASK: c_uint = 0x0000000004000000UL;
pub const UV4H_EVENT_OCCURRED0_LH1_AOERR1_SHFT: c_int = 27;
pub const UV4H_EVENT_OCCURRED0_LH1_AOERR1_MASK: c_uint = 0x0000000008000000UL;
pub const UV4H_EVENT_OCCURRED0_GR0_AOERR1_SHFT: c_int = 28;
pub const UV4H_EVENT_OCCURRED0_GR0_AOERR1_MASK: c_uint = 0x0000000010000000UL;
pub const UV4H_EVENT_OCCURRED0_GR1_AOERR1_SHFT: c_int = 29;
pub const UV4H_EVENT_OCCURRED0_GR1_AOERR1_MASK: c_uint = 0x0000000020000000UL;
pub const UV4H_EVENT_OCCURRED0_XB_AOERR1_SHFT: c_int = 30;
pub const UV4H_EVENT_OCCURRED0_XB_AOERR1_MASK: c_uint = 0x0000000040000000UL;
pub const UV4H_EVENT_OCCURRED0_RTQ0_AOERR1_SHFT: c_int = 31;
pub const UV4H_EVENT_OCCURRED0_RTQ0_AOERR1_MASK: c_uint = 0x0000000080000000UL;
pub const UV4H_EVENT_OCCURRED0_RTQ1_AOERR1_SHFT: c_int = 32;
pub const UV4H_EVENT_OCCURRED0_RTQ1_AOERR1_MASK: c_uint = 0x0000000100000000UL;
pub const UV4H_EVENT_OCCURRED0_RTQ2_AOERR1_SHFT: c_int = 33;
pub const UV4H_EVENT_OCCURRED0_RTQ2_AOERR1_MASK: c_uint = 0x0000000200000000UL;
pub const UV4H_EVENT_OCCURRED0_RTQ3_AOERR1_SHFT: c_int = 34;
pub const UV4H_EVENT_OCCURRED0_RTQ3_AOERR1_MASK: c_uint = 0x0000000400000000UL;
pub const UV4H_EVENT_OCCURRED0_NI0_AOERR1_SHFT: c_int = 35;
pub const UV4H_EVENT_OCCURRED0_NI0_AOERR1_MASK: c_uint = 0x0000000800000000UL;
pub const UV4H_EVENT_OCCURRED0_NI1_AOERR1_SHFT: c_int = 36;
pub const UV4H_EVENT_OCCURRED0_NI1_AOERR1_MASK: c_uint = 0x0000001000000000UL;
pub const UV4H_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_SHFT: c_int = 37;
pub const UV4H_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_MASK: c_uint = 0x0000002000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_0_SHFT: c_int = 38;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_0_MASK: c_uint = 0x0000004000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_1_SHFT: c_int = 39;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_1_MASK: c_uint = 0x0000008000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_2_SHFT: c_int = 40;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_2_MASK: c_uint = 0x0000010000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_3_SHFT: c_int = 41;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_3_MASK: c_uint = 0x0000020000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_4_SHFT: c_int = 42;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_4_MASK: c_uint = 0x0000040000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_5_SHFT: c_int = 43;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_5_MASK: c_uint = 0x0000080000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_6_SHFT: c_int = 44;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_6_MASK: c_uint = 0x0000100000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_7_SHFT: c_int = 45;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_7_MASK: c_uint = 0x0000200000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_8_SHFT: c_int = 46;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_8_MASK: c_uint = 0x0000400000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_9_SHFT: c_int = 47;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_9_MASK: c_uint = 0x0000800000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_10_SHFT: c_int = 48;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_10_MASK: c_uint = 0x0001000000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_11_SHFT: c_int = 49;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_11_MASK: c_uint = 0x0002000000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_12_SHFT: c_int = 50;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_12_MASK: c_uint = 0x0004000000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_13_SHFT: c_int = 51;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_13_MASK: c_uint = 0x0008000000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_14_SHFT: c_int = 52;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_14_MASK: c_uint = 0x0010000000000000UL;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_15_SHFT: c_int = 53;
pub const UV4H_EVENT_OCCURRED0_LB_IRQ_INT_15_MASK: c_uint = 0x0020000000000000UL;
pub const UV4H_EVENT_OCCURRED0_L1_NMI_INT_SHFT: c_int = 54;
pub const UV4H_EVENT_OCCURRED0_L1_NMI_INT_MASK: c_uint = 0x0040000000000000UL;
pub const UV4H_EVENT_OCCURRED0_STOP_CLOCK_SHFT: c_int = 55;
pub const UV4H_EVENT_OCCURRED0_STOP_CLOCK_MASK: c_uint = 0x0080000000000000UL;
pub const UV4H_EVENT_OCCURRED0_ASIC_TO_L1_SHFT: c_int = 56;
pub const UV4H_EVENT_OCCURRED0_ASIC_TO_L1_MASK: c_uint = 0x0100000000000000UL;
pub const UV4H_EVENT_OCCURRED0_L1_TO_ASIC_SHFT: c_int = 57;
pub const UV4H_EVENT_OCCURRED0_L1_TO_ASIC_MASK: c_uint = 0x0200000000000000UL;
pub const UV4H_EVENT_OCCURRED0_LA_SEQ_TRIGGER_SHFT: c_int = 58;
pub const UV4H_EVENT_OCCURRED0_LA_SEQ_TRIGGER_MASK: c_uint = 0x0400000000000000UL;
pub const UV4H_EVENT_OCCURRED0_IPI_INT_SHFT: c_int = 59;
pub const UV4H_EVENT_OCCURRED0_IPI_INT_MASK: c_uint = 0x0800000000000000UL;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT0_SHFT: c_int = 60;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT0_MASK: c_uint = 0x1000000000000000UL;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT1_SHFT: c_int = 61;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT1_MASK: c_uint = 0x2000000000000000UL;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT2_SHFT: c_int = 62;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT2_MASK: c_uint = 0x4000000000000000UL;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT3_SHFT: c_int = 63;
pub const UV4H_EVENT_OCCURRED0_EXTIO_INT3_MASK: c_uint = 0x8000000000000000UL;
// UV3 unique defines
pub const UV3H_EVENT_OCCURRED0_QP_HCERR_SHFT: c_int = 1;
pub const UV3H_EVENT_OCCURRED0_QP_HCERR_MASK: c_uint = 0x0000000000000002UL;
pub const UV3H_EVENT_OCCURRED0_QP_AOERR0_SHFT: c_int = 10;
pub const UV3H_EVENT_OCCURRED0_QP_AOERR0_MASK: c_uint = 0x0000000000000400UL;
pub const UV3H_EVENT_OCCURRED0_RT_AOERR0_SHFT: c_int = 17;
pub const UV3H_EVENT_OCCURRED0_RT_AOERR0_MASK: c_uint = 0x0000000000020000UL;
pub const UV3H_EVENT_OCCURRED0_NI0_AOERR0_SHFT: c_int = 18;
pub const UV3H_EVENT_OCCURRED0_NI0_AOERR0_MASK: c_uint = 0x0000000000040000UL;
pub const UV3H_EVENT_OCCURRED0_NI1_AOERR0_SHFT: c_int = 19;
pub const UV3H_EVENT_OCCURRED0_NI1_AOERR0_MASK: c_uint = 0x0000000000080000UL;
pub const UV3H_EVENT_OCCURRED0_LB_AOERR1_SHFT: c_int = 20;
pub const UV3H_EVENT_OCCURRED0_LB_AOERR1_MASK: c_uint = 0x0000000000100000UL;
pub const UV3H_EVENT_OCCURRED0_QP_AOERR1_SHFT: c_int = 21;
pub const UV3H_EVENT_OCCURRED0_QP_AOERR1_MASK: c_uint = 0x0000000000200000UL;
pub const UV3H_EVENT_OCCURRED0_RH_AOERR1_SHFT: c_int = 22;
pub const UV3H_EVENT_OCCURRED0_RH_AOERR1_MASK: c_uint = 0x0000000000400000UL;
pub const UV3H_EVENT_OCCURRED0_LH0_AOERR1_SHFT: c_int = 23;
pub const UV3H_EVENT_OCCURRED0_LH0_AOERR1_MASK: c_uint = 0x0000000000800000UL;
pub const UV3H_EVENT_OCCURRED0_LH1_AOERR1_SHFT: c_int = 24;
pub const UV3H_EVENT_OCCURRED0_LH1_AOERR1_MASK: c_uint = 0x0000000001000000UL;
pub const UV3H_EVENT_OCCURRED0_GR0_AOERR1_SHFT: c_int = 25;
pub const UV3H_EVENT_OCCURRED0_GR0_AOERR1_MASK: c_uint = 0x0000000002000000UL;
pub const UV3H_EVENT_OCCURRED0_GR1_AOERR1_SHFT: c_int = 26;
pub const UV3H_EVENT_OCCURRED0_GR1_AOERR1_MASK: c_uint = 0x0000000004000000UL;
pub const UV3H_EVENT_OCCURRED0_XB_AOERR1_SHFT: c_int = 27;
pub const UV3H_EVENT_OCCURRED0_XB_AOERR1_MASK: c_uint = 0x0000000008000000UL;
pub const UV3H_EVENT_OCCURRED0_RT_AOERR1_SHFT: c_int = 28;
pub const UV3H_EVENT_OCCURRED0_RT_AOERR1_MASK: c_uint = 0x0000000010000000UL;
pub const UV3H_EVENT_OCCURRED0_NI0_AOERR1_SHFT: c_int = 29;
pub const UV3H_EVENT_OCCURRED0_NI0_AOERR1_MASK: c_uint = 0x0000000020000000UL;
pub const UV3H_EVENT_OCCURRED0_NI1_AOERR1_SHFT: c_int = 30;
pub const UV3H_EVENT_OCCURRED0_NI1_AOERR1_MASK: c_uint = 0x0000000040000000UL;
pub const UV3H_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_SHFT: c_int = 31;
pub const UV3H_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_MASK: c_uint = 0x0000000080000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_0_SHFT: c_int = 32;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_0_MASK: c_uint = 0x0000000100000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_1_SHFT: c_int = 33;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_1_MASK: c_uint = 0x0000000200000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_2_SHFT: c_int = 34;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_2_MASK: c_uint = 0x0000000400000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_3_SHFT: c_int = 35;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_3_MASK: c_uint = 0x0000000800000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_4_SHFT: c_int = 36;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_4_MASK: c_uint = 0x0000001000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_5_SHFT: c_int = 37;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_5_MASK: c_uint = 0x0000002000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_6_SHFT: c_int = 38;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_6_MASK: c_uint = 0x0000004000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_7_SHFT: c_int = 39;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_7_MASK: c_uint = 0x0000008000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_8_SHFT: c_int = 40;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_8_MASK: c_uint = 0x0000010000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_9_SHFT: c_int = 41;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_9_MASK: c_uint = 0x0000020000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_10_SHFT: c_int = 42;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_10_MASK: c_uint = 0x0000040000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_11_SHFT: c_int = 43;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_11_MASK: c_uint = 0x0000080000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_12_SHFT: c_int = 44;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_12_MASK: c_uint = 0x0000100000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_13_SHFT: c_int = 45;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_13_MASK: c_uint = 0x0000200000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_14_SHFT: c_int = 46;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_14_MASK: c_uint = 0x0000400000000000UL;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_15_SHFT: c_int = 47;
pub const UV3H_EVENT_OCCURRED0_LB_IRQ_INT_15_MASK: c_uint = 0x0000800000000000UL;
pub const UV3H_EVENT_OCCURRED0_L1_NMI_INT_SHFT: c_int = 48;
pub const UV3H_EVENT_OCCURRED0_L1_NMI_INT_MASK: c_uint = 0x0001000000000000UL;
pub const UV3H_EVENT_OCCURRED0_STOP_CLOCK_SHFT: c_int = 49;
pub const UV3H_EVENT_OCCURRED0_STOP_CLOCK_MASK: c_uint = 0x0002000000000000UL;
pub const UV3H_EVENT_OCCURRED0_ASIC_TO_L1_SHFT: c_int = 50;
pub const UV3H_EVENT_OCCURRED0_ASIC_TO_L1_MASK: c_uint = 0x0004000000000000UL;
pub const UV3H_EVENT_OCCURRED0_L1_TO_ASIC_SHFT: c_int = 51;
pub const UV3H_EVENT_OCCURRED0_L1_TO_ASIC_MASK: c_uint = 0x0008000000000000UL;
pub const UV3H_EVENT_OCCURRED0_LA_SEQ_TRIGGER_SHFT: c_int = 52;
pub const UV3H_EVENT_OCCURRED0_LA_SEQ_TRIGGER_MASK: c_uint = 0x0010000000000000UL;
pub const UV3H_EVENT_OCCURRED0_IPI_INT_SHFT: c_int = 53;
pub const UV3H_EVENT_OCCURRED0_IPI_INT_MASK: c_uint = 0x0020000000000000UL;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT0_SHFT: c_int = 54;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT0_MASK: c_uint = 0x0040000000000000UL;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT1_SHFT: c_int = 55;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT1_MASK: c_uint = 0x0080000000000000UL;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT2_SHFT: c_int = 56;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT2_MASK: c_uint = 0x0100000000000000UL;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT3_SHFT: c_int = 57;
pub const UV3H_EVENT_OCCURRED0_EXTIO_INT3_MASK: c_uint = 0x0200000000000000UL;
pub const UV3H_EVENT_OCCURRED0_PROFILE_INT_SHFT: c_int = 58;
pub const UV3H_EVENT_OCCURRED0_PROFILE_INT_MASK: c_uint = 0x0400000000000000UL;
// UV2 unique defines
pub const UV2H_EVENT_OCCURRED0_QP_HCERR_SHFT: c_int = 1;
pub const UV2H_EVENT_OCCURRED0_QP_HCERR_MASK: c_uint = 0x0000000000000002UL;
pub const UV2H_EVENT_OCCURRED0_QP_AOERR0_SHFT: c_int = 10;
pub const UV2H_EVENT_OCCURRED0_QP_AOERR0_MASK: c_uint = 0x0000000000000400UL;
pub const UV2H_EVENT_OCCURRED0_RT_AOERR0_SHFT: c_int = 17;
pub const UV2H_EVENT_OCCURRED0_RT_AOERR0_MASK: c_uint = 0x0000000000020000UL;
pub const UV2H_EVENT_OCCURRED0_NI0_AOERR0_SHFT: c_int = 18;
pub const UV2H_EVENT_OCCURRED0_NI0_AOERR0_MASK: c_uint = 0x0000000000040000UL;
pub const UV2H_EVENT_OCCURRED0_NI1_AOERR0_SHFT: c_int = 19;
pub const UV2H_EVENT_OCCURRED0_NI1_AOERR0_MASK: c_uint = 0x0000000000080000UL;
pub const UV2H_EVENT_OCCURRED0_LB_AOERR1_SHFT: c_int = 20;
pub const UV2H_EVENT_OCCURRED0_LB_AOERR1_MASK: c_uint = 0x0000000000100000UL;
pub const UV2H_EVENT_OCCURRED0_QP_AOERR1_SHFT: c_int = 21;
pub const UV2H_EVENT_OCCURRED0_QP_AOERR1_MASK: c_uint = 0x0000000000200000UL;
pub const UV2H_EVENT_OCCURRED0_RH_AOERR1_SHFT: c_int = 22;
pub const UV2H_EVENT_OCCURRED0_RH_AOERR1_MASK: c_uint = 0x0000000000400000UL;
pub const UV2H_EVENT_OCCURRED0_LH0_AOERR1_SHFT: c_int = 23;
pub const UV2H_EVENT_OCCURRED0_LH0_AOERR1_MASK: c_uint = 0x0000000000800000UL;
pub const UV2H_EVENT_OCCURRED0_LH1_AOERR1_SHFT: c_int = 24;
pub const UV2H_EVENT_OCCURRED0_LH1_AOERR1_MASK: c_uint = 0x0000000001000000UL;
pub const UV2H_EVENT_OCCURRED0_GR0_AOERR1_SHFT: c_int = 25;
pub const UV2H_EVENT_OCCURRED0_GR0_AOERR1_MASK: c_uint = 0x0000000002000000UL;
pub const UV2H_EVENT_OCCURRED0_GR1_AOERR1_SHFT: c_int = 26;
pub const UV2H_EVENT_OCCURRED0_GR1_AOERR1_MASK: c_uint = 0x0000000004000000UL;
pub const UV2H_EVENT_OCCURRED0_XB_AOERR1_SHFT: c_int = 27;
pub const UV2H_EVENT_OCCURRED0_XB_AOERR1_MASK: c_uint = 0x0000000008000000UL;
pub const UV2H_EVENT_OCCURRED0_RT_AOERR1_SHFT: c_int = 28;
pub const UV2H_EVENT_OCCURRED0_RT_AOERR1_MASK: c_uint = 0x0000000010000000UL;
pub const UV2H_EVENT_OCCURRED0_NI0_AOERR1_SHFT: c_int = 29;
pub const UV2H_EVENT_OCCURRED0_NI0_AOERR1_MASK: c_uint = 0x0000000020000000UL;
pub const UV2H_EVENT_OCCURRED0_NI1_AOERR1_SHFT: c_int = 30;
pub const UV2H_EVENT_OCCURRED0_NI1_AOERR1_MASK: c_uint = 0x0000000040000000UL;
pub const UV2H_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_SHFT: c_int = 31;
pub const UV2H_EVENT_OCCURRED0_SYSTEM_SHUTDOWN_INT_MASK: c_uint = 0x0000000080000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_0_SHFT: c_int = 32;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_0_MASK: c_uint = 0x0000000100000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_1_SHFT: c_int = 33;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_1_MASK: c_uint = 0x0000000200000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_2_SHFT: c_int = 34;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_2_MASK: c_uint = 0x0000000400000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_3_SHFT: c_int = 35;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_3_MASK: c_uint = 0x0000000800000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_4_SHFT: c_int = 36;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_4_MASK: c_uint = 0x0000001000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_5_SHFT: c_int = 37;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_5_MASK: c_uint = 0x0000002000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_6_SHFT: c_int = 38;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_6_MASK: c_uint = 0x0000004000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_7_SHFT: c_int = 39;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_7_MASK: c_uint = 0x0000008000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_8_SHFT: c_int = 40;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_8_MASK: c_uint = 0x0000010000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_9_SHFT: c_int = 41;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_9_MASK: c_uint = 0x0000020000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_10_SHFT: c_int = 42;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_10_MASK: c_uint = 0x0000040000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_11_SHFT: c_int = 43;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_11_MASK: c_uint = 0x0000080000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_12_SHFT: c_int = 44;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_12_MASK: c_uint = 0x0000100000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_13_SHFT: c_int = 45;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_13_MASK: c_uint = 0x0000200000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_14_SHFT: c_int = 46;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_14_MASK: c_uint = 0x0000400000000000UL;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_15_SHFT: c_int = 47;
pub const UV2H_EVENT_OCCURRED0_LB_IRQ_INT_15_MASK: c_uint = 0x0000800000000000UL;
pub const UV2H_EVENT_OCCURRED0_L1_NMI_INT_SHFT: c_int = 48;
pub const UV2H_EVENT_OCCURRED0_L1_NMI_INT_MASK: c_uint = 0x0001000000000000UL;
pub const UV2H_EVENT_OCCURRED0_STOP_CLOCK_SHFT: c_int = 49;
pub const UV2H_EVENT_OCCURRED0_STOP_CLOCK_MASK: c_uint = 0x0002000000000000UL;
pub const UV2H_EVENT_OCCURRED0_ASIC_TO_L1_SHFT: c_int = 50;
pub const UV2H_EVENT_OCCURRED0_ASIC_TO_L1_MASK: c_uint = 0x0004000000000000UL;
pub const UV2H_EVENT_OCCURRED0_L1_TO_ASIC_SHFT: c_int = 51;
pub const UV2H_EVENT_OCCURRED0_L1_TO_ASIC_MASK: c_uint = 0x0008000000000000UL;
pub const UV2H_EVENT_OCCURRED0_LA_SEQ_TRIGGER_SHFT: c_int = 52;
pub const UV2H_EVENT_OCCURRED0_LA_SEQ_TRIGGER_MASK: c_uint = 0x0010000000000000UL;
pub const UV2H_EVENT_OCCURRED0_IPI_INT_SHFT: c_int = 53;
pub const UV2H_EVENT_OCCURRED0_IPI_INT_MASK: c_uint = 0x0020000000000000UL;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT0_SHFT: c_int = 54;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT0_MASK: c_uint = 0x0040000000000000UL;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT1_SHFT: c_int = 55;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT1_MASK: c_uint = 0x0080000000000000UL;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT2_SHFT: c_int = 56;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT2_MASK: c_uint = 0x0100000000000000UL;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT3_SHFT: c_int = 57;
pub const UV2H_EVENT_OCCURRED0_EXTIO_INT3_MASK: c_uint = 0x0200000000000000UL;
pub const UV2H_EVENT_OCCURRED0_PROFILE_INT_SHFT: c_int = 58;
pub const UV2H_EVENT_OCCURRED0_PROFILE_INT_MASK: c_uint = 0x0400000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_event_occurred0_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_event_occurred0_s {
    pub /: *mut *mut unsigned long lb_hcerr:1; / RW,
    pub rsvd_1_63:63: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_event_occurred0_s {
    pub /: *mut *mut unsigned long lb_hcerr:1; / RW,
    pub rsvd_1:1: c_ulong,
    pub /: *mut *mut unsigned long rh_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr0:1; / RW,
    pub rsvd_10:1: c_ulong,
    pub /: *mut *mut unsigned long rh_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr0:1; / RW,
    pub rsvd_17_63:47: c_ulong,
    pub sx: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_event_occurred0_s {
    pub /: *mut *mut unsigned long lb_hcerr:1; / RW,
    pub /: *mut *mut unsigned long kt_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh2_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh3_hcerr:1; / RW,
    pub /: *mut *mut unsigned long xb_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rdm_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long kt_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh2_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh3_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rdm_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rt0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rt1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long kt_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rh0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rh1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh2_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh3_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rdm_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rt0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rt1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long system_shutdown_int:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_0:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_1:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_2:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_3:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_4:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_5:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_6:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_7:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_8:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_9:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_10:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_11:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_12:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_13:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_14:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_15:1; / RW,
    pub /: *mut *mut unsigned long l1_nmi_int:1; / RW,
    pub /: *mut *mut unsigned long stop_clock:1; / RW,
    pub /: *mut *mut unsigned long asic_to_l1:1; / RW,
    pub /: *mut *mut unsigned long l1_to_asic:1; / RW,
    pub /: *mut *mut unsigned long la_seq_trigger:1; / RW,
    pub rsvd_62_63:2: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_event_occurred0_s {
    pub /: *mut *mut unsigned long lb_hcerr:1; / RW,
    pub /: *mut *mut unsigned long kt_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh2_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh3_hcerr:1; / RW,
    pub /: *mut *mut unsigned long xb_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rdm_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long kt_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh2_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh3_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rdm_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rt0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rt1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long kt_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rh0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rh1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh2_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh3_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rdm_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rt0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rt1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long system_shutdown_int:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_0:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_1:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_2:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_3:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_4:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_5:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_6:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_7:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_8:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_9:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_10:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_11:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_12:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_13:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_14:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_15:1; / RW,
    pub /: *mut *mut unsigned long l1_nmi_int:1; / RW,
    pub /: *mut *mut unsigned long stop_clock:1; / RW,
    pub /: *mut *mut unsigned long asic_to_l1:1; / RW,
    pub /: *mut *mut unsigned long l1_to_asic:1; / RW,
    pub /: *mut *mut unsigned long la_seq_trigger:1; / RW,
    pub rsvd_62_63:2: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_event_occurred0_s {
    pub /: *mut *mut unsigned long lb_hcerr:1; / RW,
    pub /: *mut *mut unsigned long kt_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rh_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long kt_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rh_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rtq0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rtq1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rtq2_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rtq3_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long kt_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rh_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long gr0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long gr1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rtq0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rtq1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rtq2_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rtq3_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long system_shutdown_int:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_0:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_1:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_2:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_3:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_4:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_5:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_6:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_7:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_8:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_9:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_10:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_11:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_12:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_13:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_14:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_15:1; / RW,
    pub /: *mut *mut unsigned long l1_nmi_int:1; / RW,
    pub /: *mut *mut unsigned long stop_clock:1; / RW,
    pub /: *mut *mut unsigned long asic_to_l1:1; / RW,
    pub /: *mut *mut unsigned long l1_to_asic:1; / RW,
    pub /: *mut *mut unsigned long la_seq_trigger:1; / RW,
    pub /: *mut *mut unsigned long ipi_int:1; / RW,
    pub /: *mut *mut unsigned long extio_int0:1; / RW,
    pub /: *mut *mut unsigned long extio_int1:1; / RW,
    pub /: *mut *mut unsigned long extio_int2:1; / RW,
    pub /: *mut *mut unsigned long extio_int3:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_event_occurred0_s {
    pub /: *mut *mut unsigned long lb_hcerr:1; / RW,
    pub /: *mut *mut unsigned long qp_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rh_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long qp_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rh_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rt_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long qp_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rh_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long gr0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long gr1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rt_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long system_shutdown_int:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_0:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_1:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_2:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_3:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_4:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_5:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_6:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_7:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_8:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_9:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_10:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_11:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_12:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_13:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_14:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_15:1; / RW,
    pub /: *mut *mut unsigned long l1_nmi_int:1; / RW,
    pub /: *mut *mut unsigned long stop_clock:1; / RW,
    pub /: *mut *mut unsigned long asic_to_l1:1; / RW,
    pub /: *mut *mut unsigned long l1_to_asic:1; / RW,
    pub /: *mut *mut unsigned long la_seq_trigger:1; / RW,
    pub /: *mut *mut unsigned long ipi_int:1; / RW,
    pub /: *mut *mut unsigned long extio_int0:1; / RW,
    pub /: *mut *mut unsigned long extio_int1:1; / RW,
    pub /: *mut *mut unsigned long extio_int2:1; / RW,
    pub /: *mut *mut unsigned long extio_int3:1; / RW,
    pub /: *mut *mut unsigned long profile_int:1; / RW,
    pub rsvd_59_63:5: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_event_occurred0_s {
    pub /: *mut *mut unsigned long lb_hcerr:1; / RW,
    pub /: *mut *mut unsigned long qp_hcerr:1; / RW,
    pub /: *mut *mut unsigned long rh_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lh1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long gr1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni0_hcerr:1; / RW,
    pub /: *mut *mut unsigned long ni1_hcerr:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long qp_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rh_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long gr1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long rt_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr0:1; / RW,
    pub /: *mut *mut unsigned long lb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long qp_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rh_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long lh1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long gr0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long gr1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long xb_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long rt_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni0_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long ni1_aoerr1:1; / RW,
    pub /: *mut *mut unsigned long system_shutdown_int:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_0:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_1:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_2:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_3:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_4:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_5:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_6:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_7:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_8:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_9:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_10:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_11:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_12:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_13:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_14:1; / RW,
    pub /: *mut *mut unsigned long lb_irq_int_15:1; / RW,
    pub /: *mut *mut unsigned long l1_nmi_int:1; / RW,
    pub /: *mut *mut unsigned long stop_clock:1; / RW,
    pub /: *mut *mut unsigned long asic_to_l1:1; / RW,
    pub /: *mut *mut unsigned long l1_to_asic:1; / RW,
    pub /: *mut *mut unsigned long la_seq_trigger:1; / RW,
    pub /: *mut *mut unsigned long ipi_int:1; / RW,
    pub /: *mut *mut unsigned long extio_int0:1; / RW,
    pub /: *mut *mut unsigned long extio_int1:1; / RW,
    pub /: *mut *mut unsigned long extio_int2:1; / RW,
    pub /: *mut *mut unsigned long extio_int3:1; / RW,
    pub /: *mut *mut unsigned long profile_int:1; / RW,
    pub rsvd_59_63:5: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_EVENT_OCCURRED0_ALIAS
// =========================================================================
pub const UVH_EVENT_OCCURRED0_ALIAS: c_uint = 0x70008UL;
// =========================================================================
// UVH_EVENT_OCCURRED1
// =========================================================================
pub const UVH_EVENT_OCCURRED1: c_uint = 0x70080UL;
// UVYH common defines
pub const UVYH_EVENT_OCCURRED1_IPI_INT_SHFT: c_int = 0;
pub const UVYH_EVENT_OCCURRED1_IPI_INT_MASK: c_uint = 0x0000000000000001UL;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT0_SHFT: c_int = 1;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT0_MASK: c_uint = 0x0000000000000002UL;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT1_SHFT: c_int = 2;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT1_MASK: c_uint = 0x0000000000000004UL;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT2_SHFT: c_int = 3;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT2_MASK: c_uint = 0x0000000000000008UL;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT3_SHFT: c_int = 4;
pub const UVYH_EVENT_OCCURRED1_EXTIO_INT3_MASK: c_uint = 0x0000000000000010UL;
pub const UVYH_EVENT_OCCURRED1_PROFILE_INT_SHFT: c_int = 5;
pub const UVYH_EVENT_OCCURRED1_PROFILE_INT_MASK: c_uint = 0x0000000000000020UL;
pub const UVYH_EVENT_OCCURRED1_BAU_DATA_SHFT: c_int = 6;
pub const UVYH_EVENT_OCCURRED1_BAU_DATA_MASK: c_uint = 0x0000000000000040UL;
pub const UVYH_EVENT_OCCURRED1_PROC_GENERAL_SHFT: c_int = 7;
pub const UVYH_EVENT_OCCURRED1_PROC_GENERAL_MASK: c_uint = 0x0000000000000080UL;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT0_SHFT: c_int = 8;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT0_MASK: c_uint = 0x0000000000000100UL;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT1_SHFT: c_int = 9;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT1_MASK: c_uint = 0x0000000000000200UL;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT2_SHFT: c_int = 10;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT2_MASK: c_uint = 0x0000000000000400UL;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT3_SHFT: c_int = 11;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT3_MASK: c_uint = 0x0000000000000800UL;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT4_SHFT: c_int = 12;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT4_MASK: c_uint = 0x0000000000001000UL;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT5_SHFT: c_int = 13;
pub const UVYH_EVENT_OCCURRED1_XH_TLB_INT5_MASK: c_uint = 0x0000000000002000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT0_SHFT: c_int = 14;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT0_MASK: c_uint = 0x0000000000004000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT1_SHFT: c_int = 15;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT1_MASK: c_uint = 0x0000000000008000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT2_SHFT: c_int = 16;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT2_MASK: c_uint = 0x0000000000010000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT3_SHFT: c_int = 17;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT3_MASK: c_uint = 0x0000000000020000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT4_SHFT: c_int = 18;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT4_MASK: c_uint = 0x0000000000040000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT5_SHFT: c_int = 19;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT5_MASK: c_uint = 0x0000000000080000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT6_SHFT: c_int = 20;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT6_MASK: c_uint = 0x0000000000100000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT7_SHFT: c_int = 21;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT7_MASK: c_uint = 0x0000000000200000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT8_SHFT: c_int = 22;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT8_MASK: c_uint = 0x0000000000400000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT9_SHFT: c_int = 23;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT9_MASK: c_uint = 0x0000000000800000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT10_SHFT: c_int = 24;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT10_MASK: c_uint = 0x0000000001000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT11_SHFT: c_int = 25;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT11_MASK: c_uint = 0x0000000002000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT12_SHFT: c_int = 26;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT12_MASK: c_uint = 0x0000000004000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT13_SHFT: c_int = 27;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT13_MASK: c_uint = 0x0000000008000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT14_SHFT: c_int = 28;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT14_MASK: c_uint = 0x0000000010000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT15_SHFT: c_int = 29;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT15_MASK: c_uint = 0x0000000020000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT16_SHFT: c_int = 30;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT16_MASK: c_uint = 0x0000000040000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT17_SHFT: c_int = 31;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT17_MASK: c_uint = 0x0000000080000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT18_SHFT: c_int = 32;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT18_MASK: c_uint = 0x0000000100000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT19_SHFT: c_int = 33;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT19_MASK: c_uint = 0x0000000200000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT20_SHFT: c_int = 34;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT20_MASK: c_uint = 0x0000000400000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT21_SHFT: c_int = 35;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT21_MASK: c_uint = 0x0000000800000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT22_SHFT: c_int = 36;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT22_MASK: c_uint = 0x0000001000000000UL;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT23_SHFT: c_int = 37;
pub const UVYH_EVENT_OCCURRED1_RDM_TLB_INT23_MASK: c_uint = 0x0000002000000000UL;
// UV4 unique defines
pub const UV4H_EVENT_OCCURRED1_PROFILE_INT_SHFT: c_int = 0;
pub const UV4H_EVENT_OCCURRED1_PROFILE_INT_MASK: c_uint = 0x0000000000000001UL;
pub const UV4H_EVENT_OCCURRED1_BAU_DATA_SHFT: c_int = 1;
pub const UV4H_EVENT_OCCURRED1_BAU_DATA_MASK: c_uint = 0x0000000000000002UL;
pub const UV4H_EVENT_OCCURRED1_PROC_GENERAL_SHFT: c_int = 2;
pub const UV4H_EVENT_OCCURRED1_PROC_GENERAL_MASK: c_uint = 0x0000000000000004UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT0_SHFT: c_int = 3;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT0_MASK: c_uint = 0x0000000000000008UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT1_SHFT: c_int = 4;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT1_MASK: c_uint = 0x0000000000000010UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT2_SHFT: c_int = 5;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT2_MASK: c_uint = 0x0000000000000020UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT3_SHFT: c_int = 6;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT3_MASK: c_uint = 0x0000000000000040UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT4_SHFT: c_int = 7;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT4_MASK: c_uint = 0x0000000000000080UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT5_SHFT: c_int = 8;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT5_MASK: c_uint = 0x0000000000000100UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT6_SHFT: c_int = 9;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT6_MASK: c_uint = 0x0000000000000200UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT7_SHFT: c_int = 10;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT7_MASK: c_uint = 0x0000000000000400UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT8_SHFT: c_int = 11;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT8_MASK: c_uint = 0x0000000000000800UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT9_SHFT: c_int = 12;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT9_MASK: c_uint = 0x0000000000001000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT10_SHFT: c_int = 13;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT10_MASK: c_uint = 0x0000000000002000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT11_SHFT: c_int = 14;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT11_MASK: c_uint = 0x0000000000004000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT12_SHFT: c_int = 15;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT12_MASK: c_uint = 0x0000000000008000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT13_SHFT: c_int = 16;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT13_MASK: c_uint = 0x0000000000010000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT14_SHFT: c_int = 17;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT14_MASK: c_uint = 0x0000000000020000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT15_SHFT: c_int = 18;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT15_MASK: c_uint = 0x0000000000040000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT16_SHFT: c_int = 19;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT16_MASK: c_uint = 0x0000000000080000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT17_SHFT: c_int = 20;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT17_MASK: c_uint = 0x0000000000100000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT18_SHFT: c_int = 21;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT18_MASK: c_uint = 0x0000000000200000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT19_SHFT: c_int = 22;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT19_MASK: c_uint = 0x0000000000400000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT20_SHFT: c_int = 23;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT20_MASK: c_uint = 0x0000000000800000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT21_SHFT: c_int = 24;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT21_MASK: c_uint = 0x0000000001000000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT22_SHFT: c_int = 25;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT22_MASK: c_uint = 0x0000000002000000UL;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT23_SHFT: c_int = 26;
pub const UV4H_EVENT_OCCURRED1_GR0_TLB_INT23_MASK: c_uint = 0x0000000004000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT0_SHFT: c_int = 27;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT0_MASK: c_uint = 0x0000000008000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT1_SHFT: c_int = 28;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT1_MASK: c_uint = 0x0000000010000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT2_SHFT: c_int = 29;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT2_MASK: c_uint = 0x0000000020000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT3_SHFT: c_int = 30;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT3_MASK: c_uint = 0x0000000040000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT4_SHFT: c_int = 31;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT4_MASK: c_uint = 0x0000000080000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT5_SHFT: c_int = 32;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT5_MASK: c_uint = 0x0000000100000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT6_SHFT: c_int = 33;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT6_MASK: c_uint = 0x0000000200000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT7_SHFT: c_int = 34;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT7_MASK: c_uint = 0x0000000400000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT8_SHFT: c_int = 35;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT8_MASK: c_uint = 0x0000000800000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT9_SHFT: c_int = 36;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT9_MASK: c_uint = 0x0000001000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT10_SHFT: c_int = 37;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT10_MASK: c_uint = 0x0000002000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT11_SHFT: c_int = 38;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT11_MASK: c_uint = 0x0000004000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT12_SHFT: c_int = 39;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT12_MASK: c_uint = 0x0000008000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT13_SHFT: c_int = 40;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT13_MASK: c_uint = 0x0000010000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT14_SHFT: c_int = 41;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT14_MASK: c_uint = 0x0000020000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT15_SHFT: c_int = 42;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT15_MASK: c_uint = 0x0000040000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT16_SHFT: c_int = 43;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT16_MASK: c_uint = 0x0000080000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT17_SHFT: c_int = 44;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT17_MASK: c_uint = 0x0000100000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT18_SHFT: c_int = 45;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT18_MASK: c_uint = 0x0000200000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT19_SHFT: c_int = 46;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT19_MASK: c_uint = 0x0000400000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT20_SHFT: c_int = 47;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT20_MASK: c_uint = 0x0000800000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT21_SHFT: c_int = 48;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT21_MASK: c_uint = 0x0001000000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT22_SHFT: c_int = 49;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT22_MASK: c_uint = 0x0002000000000000UL;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT23_SHFT: c_int = 50;
pub const UV4H_EVENT_OCCURRED1_GR1_TLB_INT23_MASK: c_uint = 0x0004000000000000UL;
// UV3 unique defines
pub const UV3H_EVENT_OCCURRED1_BAU_DATA_SHFT: c_int = 0;
pub const UV3H_EVENT_OCCURRED1_BAU_DATA_MASK: c_uint = 0x0000000000000001UL;
pub const UV3H_EVENT_OCCURRED1_POWER_MANAGEMENT_REQ_SHFT: c_int = 1;
pub const UV3H_EVENT_OCCURRED1_POWER_MANAGEMENT_REQ_MASK: c_uint = 0x0000000000000002UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT0_SHFT: c_int = 2;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT0_MASK: c_uint = 0x0000000000000004UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT1_SHFT: c_int = 3;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT1_MASK: c_uint = 0x0000000000000008UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT2_SHFT: c_int = 4;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT2_MASK: c_uint = 0x0000000000000010UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT3_SHFT: c_int = 5;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT3_MASK: c_uint = 0x0000000000000020UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT4_SHFT: c_int = 6;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT4_MASK: c_uint = 0x0000000000000040UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT5_SHFT: c_int = 7;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT5_MASK: c_uint = 0x0000000000000080UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT6_SHFT: c_int = 8;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT6_MASK: c_uint = 0x0000000000000100UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT7_SHFT: c_int = 9;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT7_MASK: c_uint = 0x0000000000000200UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT8_SHFT: c_int = 10;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT8_MASK: c_uint = 0x0000000000000400UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT9_SHFT: c_int = 11;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT9_MASK: c_uint = 0x0000000000000800UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT10_SHFT: c_int = 12;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT10_MASK: c_uint = 0x0000000000001000UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT11_SHFT: c_int = 13;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT11_MASK: c_uint = 0x0000000000002000UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT12_SHFT: c_int = 14;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT12_MASK: c_uint = 0x0000000000004000UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT13_SHFT: c_int = 15;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT13_MASK: c_uint = 0x0000000000008000UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT14_SHFT: c_int = 16;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT14_MASK: c_uint = 0x0000000000010000UL;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT15_SHFT: c_int = 17;
pub const UV3H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT15_MASK: c_uint = 0x0000000000020000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT0_SHFT: c_int = 18;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT0_MASK: c_uint = 0x0000000000040000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT1_SHFT: c_int = 19;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT1_MASK: c_uint = 0x0000000000080000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT2_SHFT: c_int = 20;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT2_MASK: c_uint = 0x0000000000100000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT3_SHFT: c_int = 21;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT3_MASK: c_uint = 0x0000000000200000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT4_SHFT: c_int = 22;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT4_MASK: c_uint = 0x0000000000400000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT5_SHFT: c_int = 23;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT5_MASK: c_uint = 0x0000000000800000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT6_SHFT: c_int = 24;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT6_MASK: c_uint = 0x0000000001000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT7_SHFT: c_int = 25;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT7_MASK: c_uint = 0x0000000002000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT8_SHFT: c_int = 26;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT8_MASK: c_uint = 0x0000000004000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT9_SHFT: c_int = 27;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT9_MASK: c_uint = 0x0000000008000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT10_SHFT: c_int = 28;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT10_MASK: c_uint = 0x0000000010000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT11_SHFT: c_int = 29;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT11_MASK: c_uint = 0x0000000020000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT12_SHFT: c_int = 30;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT12_MASK: c_uint = 0x0000000040000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT13_SHFT: c_int = 31;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT13_MASK: c_uint = 0x0000000080000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT14_SHFT: c_int = 32;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT14_MASK: c_uint = 0x0000000100000000UL;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT15_SHFT: c_int = 33;
pub const UV3H_EVENT_OCCURRED1_GR0_TLB_INT15_MASK: c_uint = 0x0000000200000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT0_SHFT: c_int = 34;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT0_MASK: c_uint = 0x0000000400000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT1_SHFT: c_int = 35;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT1_MASK: c_uint = 0x0000000800000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT2_SHFT: c_int = 36;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT2_MASK: c_uint = 0x0000001000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT3_SHFT: c_int = 37;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT3_MASK: c_uint = 0x0000002000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT4_SHFT: c_int = 38;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT4_MASK: c_uint = 0x0000004000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT5_SHFT: c_int = 39;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT5_MASK: c_uint = 0x0000008000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT6_SHFT: c_int = 40;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT6_MASK: c_uint = 0x0000010000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT7_SHFT: c_int = 41;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT7_MASK: c_uint = 0x0000020000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT8_SHFT: c_int = 42;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT8_MASK: c_uint = 0x0000040000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT9_SHFT: c_int = 43;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT9_MASK: c_uint = 0x0000080000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT10_SHFT: c_int = 44;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT10_MASK: c_uint = 0x0000100000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT11_SHFT: c_int = 45;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT11_MASK: c_uint = 0x0000200000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT12_SHFT: c_int = 46;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT12_MASK: c_uint = 0x0000400000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT13_SHFT: c_int = 47;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT13_MASK: c_uint = 0x0000800000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT14_SHFT: c_int = 48;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT14_MASK: c_uint = 0x0001000000000000UL;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT15_SHFT: c_int = 49;
pub const UV3H_EVENT_OCCURRED1_GR1_TLB_INT15_MASK: c_uint = 0x0002000000000000UL;
pub const UV3H_EVENT_OCCURRED1_RTC_INTERVAL_INT_SHFT: c_int = 50;
pub const UV3H_EVENT_OCCURRED1_RTC_INTERVAL_INT_MASK: c_uint = 0x0004000000000000UL;
pub const UV3H_EVENT_OCCURRED1_BAU_DASHBOARD_INT_SHFT: c_int = 51;
pub const UV3H_EVENT_OCCURRED1_BAU_DASHBOARD_INT_MASK: c_uint = 0x0008000000000000UL;
// UV2 unique defines
pub const UV2H_EVENT_OCCURRED1_BAU_DATA_SHFT: c_int = 0;
pub const UV2H_EVENT_OCCURRED1_BAU_DATA_MASK: c_uint = 0x0000000000000001UL;
pub const UV2H_EVENT_OCCURRED1_POWER_MANAGEMENT_REQ_SHFT: c_int = 1;
pub const UV2H_EVENT_OCCURRED1_POWER_MANAGEMENT_REQ_MASK: c_uint = 0x0000000000000002UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT0_SHFT: c_int = 2;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT0_MASK: c_uint = 0x0000000000000004UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT1_SHFT: c_int = 3;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT1_MASK: c_uint = 0x0000000000000008UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT2_SHFT: c_int = 4;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT2_MASK: c_uint = 0x0000000000000010UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT3_SHFT: c_int = 5;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT3_MASK: c_uint = 0x0000000000000020UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT4_SHFT: c_int = 6;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT4_MASK: c_uint = 0x0000000000000040UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT5_SHFT: c_int = 7;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT5_MASK: c_uint = 0x0000000000000080UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT6_SHFT: c_int = 8;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT6_MASK: c_uint = 0x0000000000000100UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT7_SHFT: c_int = 9;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT7_MASK: c_uint = 0x0000000000000200UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT8_SHFT: c_int = 10;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT8_MASK: c_uint = 0x0000000000000400UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT9_SHFT: c_int = 11;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT9_MASK: c_uint = 0x0000000000000800UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT10_SHFT: c_int = 12;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT10_MASK: c_uint = 0x0000000000001000UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT11_SHFT: c_int = 13;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT11_MASK: c_uint = 0x0000000000002000UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT12_SHFT: c_int = 14;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT12_MASK: c_uint = 0x0000000000004000UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT13_SHFT: c_int = 15;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT13_MASK: c_uint = 0x0000000000008000UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT14_SHFT: c_int = 16;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT14_MASK: c_uint = 0x0000000000010000UL;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT15_SHFT: c_int = 17;
pub const UV2H_EVENT_OCCURRED1_MESSAGE_ACCELERATOR_INT15_MASK: c_uint = 0x0000000000020000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT0_SHFT: c_int = 18;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT0_MASK: c_uint = 0x0000000000040000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT1_SHFT: c_int = 19;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT1_MASK: c_uint = 0x0000000000080000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT2_SHFT: c_int = 20;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT2_MASK: c_uint = 0x0000000000100000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT3_SHFT: c_int = 21;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT3_MASK: c_uint = 0x0000000000200000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT4_SHFT: c_int = 22;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT4_MASK: c_uint = 0x0000000000400000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT5_SHFT: c_int = 23;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT5_MASK: c_uint = 0x0000000000800000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT6_SHFT: c_int = 24;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT6_MASK: c_uint = 0x0000000001000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT7_SHFT: c_int = 25;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT7_MASK: c_uint = 0x0000000002000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT8_SHFT: c_int = 26;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT8_MASK: c_uint = 0x0000000004000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT9_SHFT: c_int = 27;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT9_MASK: c_uint = 0x0000000008000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT10_SHFT: c_int = 28;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT10_MASK: c_uint = 0x0000000010000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT11_SHFT: c_int = 29;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT11_MASK: c_uint = 0x0000000020000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT12_SHFT: c_int = 30;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT12_MASK: c_uint = 0x0000000040000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT13_SHFT: c_int = 31;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT13_MASK: c_uint = 0x0000000080000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT14_SHFT: c_int = 32;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT14_MASK: c_uint = 0x0000000100000000UL;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT15_SHFT: c_int = 33;
pub const UV2H_EVENT_OCCURRED1_GR0_TLB_INT15_MASK: c_uint = 0x0000000200000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT0_SHFT: c_int = 34;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT0_MASK: c_uint = 0x0000000400000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT1_SHFT: c_int = 35;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT1_MASK: c_uint = 0x0000000800000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT2_SHFT: c_int = 36;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT2_MASK: c_uint = 0x0000001000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT3_SHFT: c_int = 37;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT3_MASK: c_uint = 0x0000002000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT4_SHFT: c_int = 38;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT4_MASK: c_uint = 0x0000004000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT5_SHFT: c_int = 39;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT5_MASK: c_uint = 0x0000008000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT6_SHFT: c_int = 40;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT6_MASK: c_uint = 0x0000010000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT7_SHFT: c_int = 41;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT7_MASK: c_uint = 0x0000020000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT8_SHFT: c_int = 42;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT8_MASK: c_uint = 0x0000040000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT9_SHFT: c_int = 43;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT9_MASK: c_uint = 0x0000080000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT10_SHFT: c_int = 44;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT10_MASK: c_uint = 0x0000100000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT11_SHFT: c_int = 45;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT11_MASK: c_uint = 0x0000200000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT12_SHFT: c_int = 46;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT12_MASK: c_uint = 0x0000400000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT13_SHFT: c_int = 47;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT13_MASK: c_uint = 0x0000800000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT14_SHFT: c_int = 48;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT14_MASK: c_uint = 0x0001000000000000UL;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT15_SHFT: c_int = 49;
pub const UV2H_EVENT_OCCURRED1_GR1_TLB_INT15_MASK: c_uint = 0x0002000000000000UL;
pub const UV2H_EVENT_OCCURRED1_RTC_INTERVAL_INT_SHFT: c_int = 50;
pub const UV2H_EVENT_OCCURRED1_RTC_INTERVAL_INT_MASK: c_uint = 0x0004000000000000UL;
pub const UV2H_EVENT_OCCURRED1_BAU_DASHBOARD_INT_SHFT: c_int = 51;
pub const UV2H_EVENT_OCCURRED1_BAU_DASHBOARD_INT_MASK: c_uint = 0x0008000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvyh_event_occurred1_u {
    pub v: c_ulong,
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_event_occurred1_s {
    pub /: *mut *mut unsigned long ipi_int:1; / RW,
    pub /: *mut *mut unsigned long extio_int0:1; / RW,
    pub /: *mut *mut unsigned long extio_int1:1; / RW,
    pub /: *mut *mut unsigned long extio_int2:1; / RW,
    pub /: *mut *mut unsigned long extio_int3:1; / RW,
    pub /: *mut *mut unsigned long profile_int:1; / RW,
    pub /: *mut *mut unsigned long bau_data:1; / RW,
    pub /: *mut *mut unsigned long proc_general:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int16:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int17:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int18:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int19:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int20:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int21:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int22:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int23:1; / RW,
    pub rsvd_38_63:26: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_event_occurred1_s {
    pub /: *mut *mut unsigned long ipi_int:1; / RW,
    pub /: *mut *mut unsigned long extio_int0:1; / RW,
    pub /: *mut *mut unsigned long extio_int1:1; / RW,
    pub /: *mut *mut unsigned long extio_int2:1; / RW,
    pub /: *mut *mut unsigned long extio_int3:1; / RW,
    pub /: *mut *mut unsigned long profile_int:1; / RW,
    pub /: *mut *mut unsigned long bau_data:1; / RW,
    pub /: *mut *mut unsigned long proc_general:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long xh_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int16:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int17:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int18:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int19:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int20:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int21:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int22:1; / RW,
    pub /: *mut *mut unsigned long rdm_tlb_int23:1; / RW,
    pub rsvd_38_63:26: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_event_occurred1_s {
    pub /: *mut *mut unsigned long profile_int:1; / RW,
    pub /: *mut *mut unsigned long bau_data:1; / RW,
    pub /: *mut *mut unsigned long proc_general:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int16:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int17:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int18:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int19:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int20:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int21:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int22:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int23:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int16:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int17:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int18:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int19:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int20:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int21:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int22:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int23:1; / RW,
    pub rsvd_51_63:13: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_event_occurred1_s {
    pub /: *mut *mut unsigned long bau_data:1; / RW,
    pub /: *mut *mut unsigned long power_management_req:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int0:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int1:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int2:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int3:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int4:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int5:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int6:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int7:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int8:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int9:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int10:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int11:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int12:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int13:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int14:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int15:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long rtc_interval_int:1; / RW,
    pub /: *mut *mut unsigned long bau_dashboard_int:1; / RW,
    pub rsvd_52_63:12: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_event_occurred1_s {
    pub /: *mut *mut unsigned long bau_data:1; / RW,
    pub /: *mut *mut unsigned long power_management_req:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int0:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int1:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int2:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int3:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int4:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int5:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int6:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int7:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int8:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int9:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int10:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int11:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int12:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int13:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int14:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int15:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long gr0_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int0:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int1:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int2:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int3:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int4:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int5:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int6:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int7:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int8:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int9:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int10:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int11:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int12:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int13:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int14:1; / RW,
    pub /: *mut *mut unsigned long gr1_tlb_int15:1; / RW,
    pub /: *mut *mut unsigned long rtc_interval_int:1; / RW,
    pub /: *mut *mut unsigned long bau_dashboard_int:1; / RW,
    pub rsvd_52_63:12: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_EVENT_OCCURRED1_ALIAS
// =========================================================================
pub const UVH_EVENT_OCCURRED1_ALIAS: c_uint = 0x70088UL;
// =========================================================================
// UVH_EVENT_OCCURRED2
// =========================================================================
pub const UVH_EVENT_OCCURRED2: c_uint = 0x70100UL;
// UVYH common defines
pub const UVYH_EVENT_OCCURRED2_RTC_INTERVAL_INT_SHFT: c_int = 0;
pub const UVYH_EVENT_OCCURRED2_RTC_INTERVAL_INT_MASK: c_uint = 0x0000000000000001UL;
pub const UVYH_EVENT_OCCURRED2_BAU_DASHBOARD_INT_SHFT: c_int = 1;
pub const UVYH_EVENT_OCCURRED2_BAU_DASHBOARD_INT_MASK: c_uint = 0x0000000000000002UL;
pub const UVYH_EVENT_OCCURRED2_RTC_0_SHFT: c_int = 2;
pub const UVYH_EVENT_OCCURRED2_RTC_0_MASK: c_uint = 0x0000000000000004UL;
pub const UVYH_EVENT_OCCURRED2_RTC_1_SHFT: c_int = 3;
pub const UVYH_EVENT_OCCURRED2_RTC_1_MASK: c_uint = 0x0000000000000008UL;
pub const UVYH_EVENT_OCCURRED2_RTC_2_SHFT: c_int = 4;
pub const UVYH_EVENT_OCCURRED2_RTC_2_MASK: c_uint = 0x0000000000000010UL;
pub const UVYH_EVENT_OCCURRED2_RTC_3_SHFT: c_int = 5;
pub const UVYH_EVENT_OCCURRED2_RTC_3_MASK: c_uint = 0x0000000000000020UL;
pub const UVYH_EVENT_OCCURRED2_RTC_4_SHFT: c_int = 6;
pub const UVYH_EVENT_OCCURRED2_RTC_4_MASK: c_uint = 0x0000000000000040UL;
pub const UVYH_EVENT_OCCURRED2_RTC_5_SHFT: c_int = 7;
pub const UVYH_EVENT_OCCURRED2_RTC_5_MASK: c_uint = 0x0000000000000080UL;
pub const UVYH_EVENT_OCCURRED2_RTC_6_SHFT: c_int = 8;
pub const UVYH_EVENT_OCCURRED2_RTC_6_MASK: c_uint = 0x0000000000000100UL;
pub const UVYH_EVENT_OCCURRED2_RTC_7_SHFT: c_int = 9;
pub const UVYH_EVENT_OCCURRED2_RTC_7_MASK: c_uint = 0x0000000000000200UL;
pub const UVYH_EVENT_OCCURRED2_RTC_8_SHFT: c_int = 10;
pub const UVYH_EVENT_OCCURRED2_RTC_8_MASK: c_uint = 0x0000000000000400UL;
pub const UVYH_EVENT_OCCURRED2_RTC_9_SHFT: c_int = 11;
pub const UVYH_EVENT_OCCURRED2_RTC_9_MASK: c_uint = 0x0000000000000800UL;
pub const UVYH_EVENT_OCCURRED2_RTC_10_SHFT: c_int = 12;
pub const UVYH_EVENT_OCCURRED2_RTC_10_MASK: c_uint = 0x0000000000001000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_11_SHFT: c_int = 13;
pub const UVYH_EVENT_OCCURRED2_RTC_11_MASK: c_uint = 0x0000000000002000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_12_SHFT: c_int = 14;
pub const UVYH_EVENT_OCCURRED2_RTC_12_MASK: c_uint = 0x0000000000004000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_13_SHFT: c_int = 15;
pub const UVYH_EVENT_OCCURRED2_RTC_13_MASK: c_uint = 0x0000000000008000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_14_SHFT: c_int = 16;
pub const UVYH_EVENT_OCCURRED2_RTC_14_MASK: c_uint = 0x0000000000010000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_15_SHFT: c_int = 17;
pub const UVYH_EVENT_OCCURRED2_RTC_15_MASK: c_uint = 0x0000000000020000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_16_SHFT: c_int = 18;
pub const UVYH_EVENT_OCCURRED2_RTC_16_MASK: c_uint = 0x0000000000040000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_17_SHFT: c_int = 19;
pub const UVYH_EVENT_OCCURRED2_RTC_17_MASK: c_uint = 0x0000000000080000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_18_SHFT: c_int = 20;
pub const UVYH_EVENT_OCCURRED2_RTC_18_MASK: c_uint = 0x0000000000100000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_19_SHFT: c_int = 21;
pub const UVYH_EVENT_OCCURRED2_RTC_19_MASK: c_uint = 0x0000000000200000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_20_SHFT: c_int = 22;
pub const UVYH_EVENT_OCCURRED2_RTC_20_MASK: c_uint = 0x0000000000400000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_21_SHFT: c_int = 23;
pub const UVYH_EVENT_OCCURRED2_RTC_21_MASK: c_uint = 0x0000000000800000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_22_SHFT: c_int = 24;
pub const UVYH_EVENT_OCCURRED2_RTC_22_MASK: c_uint = 0x0000000001000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_23_SHFT: c_int = 25;
pub const UVYH_EVENT_OCCURRED2_RTC_23_MASK: c_uint = 0x0000000002000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_24_SHFT: c_int = 26;
pub const UVYH_EVENT_OCCURRED2_RTC_24_MASK: c_uint = 0x0000000004000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_25_SHFT: c_int = 27;
pub const UVYH_EVENT_OCCURRED2_RTC_25_MASK: c_uint = 0x0000000008000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_26_SHFT: c_int = 28;
pub const UVYH_EVENT_OCCURRED2_RTC_26_MASK: c_uint = 0x0000000010000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_27_SHFT: c_int = 29;
pub const UVYH_EVENT_OCCURRED2_RTC_27_MASK: c_uint = 0x0000000020000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_28_SHFT: c_int = 30;
pub const UVYH_EVENT_OCCURRED2_RTC_28_MASK: c_uint = 0x0000000040000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_29_SHFT: c_int = 31;
pub const UVYH_EVENT_OCCURRED2_RTC_29_MASK: c_uint = 0x0000000080000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_30_SHFT: c_int = 32;
pub const UVYH_EVENT_OCCURRED2_RTC_30_MASK: c_uint = 0x0000000100000000UL;
pub const UVYH_EVENT_OCCURRED2_RTC_31_SHFT: c_int = 33;
pub const UVYH_EVENT_OCCURRED2_RTC_31_MASK: c_uint = 0x0000000200000000UL;
// UV4 unique defines
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT0_SHFT: c_int = 0;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT0_MASK: c_uint = 0x0000000000000001UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT1_SHFT: c_int = 1;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT1_MASK: c_uint = 0x0000000000000002UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT2_SHFT: c_int = 2;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT2_MASK: c_uint = 0x0000000000000004UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT3_SHFT: c_int = 3;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT3_MASK: c_uint = 0x0000000000000008UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT4_SHFT: c_int = 4;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT4_MASK: c_uint = 0x0000000000000010UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT5_SHFT: c_int = 5;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT5_MASK: c_uint = 0x0000000000000020UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT6_SHFT: c_int = 6;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT6_MASK: c_uint = 0x0000000000000040UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT7_SHFT: c_int = 7;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT7_MASK: c_uint = 0x0000000000000080UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT8_SHFT: c_int = 8;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT8_MASK: c_uint = 0x0000000000000100UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT9_SHFT: c_int = 9;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT9_MASK: c_uint = 0x0000000000000200UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT10_SHFT: c_int = 10;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT10_MASK: c_uint = 0x0000000000000400UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT11_SHFT: c_int = 11;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT11_MASK: c_uint = 0x0000000000000800UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT12_SHFT: c_int = 12;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT12_MASK: c_uint = 0x0000000000001000UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT13_SHFT: c_int = 13;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT13_MASK: c_uint = 0x0000000000002000UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT14_SHFT: c_int = 14;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT14_MASK: c_uint = 0x0000000000004000UL;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT15_SHFT: c_int = 15;
pub const UV4H_EVENT_OCCURRED2_MESSAGE_ACCELERATOR_INT15_MASK: c_uint = 0x0000000000008000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_INTERVAL_INT_SHFT: c_int = 16;
pub const UV4H_EVENT_OCCURRED2_RTC_INTERVAL_INT_MASK: c_uint = 0x0000000000010000UL;
pub const UV4H_EVENT_OCCURRED2_BAU_DASHBOARD_INT_SHFT: c_int = 17;
pub const UV4H_EVENT_OCCURRED2_BAU_DASHBOARD_INT_MASK: c_uint = 0x0000000000020000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_0_SHFT: c_int = 18;
pub const UV4H_EVENT_OCCURRED2_RTC_0_MASK: c_uint = 0x0000000000040000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_1_SHFT: c_int = 19;
pub const UV4H_EVENT_OCCURRED2_RTC_1_MASK: c_uint = 0x0000000000080000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_2_SHFT: c_int = 20;
pub const UV4H_EVENT_OCCURRED2_RTC_2_MASK: c_uint = 0x0000000000100000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_3_SHFT: c_int = 21;
pub const UV4H_EVENT_OCCURRED2_RTC_3_MASK: c_uint = 0x0000000000200000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_4_SHFT: c_int = 22;
pub const UV4H_EVENT_OCCURRED2_RTC_4_MASK: c_uint = 0x0000000000400000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_5_SHFT: c_int = 23;
pub const UV4H_EVENT_OCCURRED2_RTC_5_MASK: c_uint = 0x0000000000800000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_6_SHFT: c_int = 24;
pub const UV4H_EVENT_OCCURRED2_RTC_6_MASK: c_uint = 0x0000000001000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_7_SHFT: c_int = 25;
pub const UV4H_EVENT_OCCURRED2_RTC_7_MASK: c_uint = 0x0000000002000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_8_SHFT: c_int = 26;
pub const UV4H_EVENT_OCCURRED2_RTC_8_MASK: c_uint = 0x0000000004000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_9_SHFT: c_int = 27;
pub const UV4H_EVENT_OCCURRED2_RTC_9_MASK: c_uint = 0x0000000008000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_10_SHFT: c_int = 28;
pub const UV4H_EVENT_OCCURRED2_RTC_10_MASK: c_uint = 0x0000000010000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_11_SHFT: c_int = 29;
pub const UV4H_EVENT_OCCURRED2_RTC_11_MASK: c_uint = 0x0000000020000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_12_SHFT: c_int = 30;
pub const UV4H_EVENT_OCCURRED2_RTC_12_MASK: c_uint = 0x0000000040000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_13_SHFT: c_int = 31;
pub const UV4H_EVENT_OCCURRED2_RTC_13_MASK: c_uint = 0x0000000080000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_14_SHFT: c_int = 32;
pub const UV4H_EVENT_OCCURRED2_RTC_14_MASK: c_uint = 0x0000000100000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_15_SHFT: c_int = 33;
pub const UV4H_EVENT_OCCURRED2_RTC_15_MASK: c_uint = 0x0000000200000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_16_SHFT: c_int = 34;
pub const UV4H_EVENT_OCCURRED2_RTC_16_MASK: c_uint = 0x0000000400000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_17_SHFT: c_int = 35;
pub const UV4H_EVENT_OCCURRED2_RTC_17_MASK: c_uint = 0x0000000800000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_18_SHFT: c_int = 36;
pub const UV4H_EVENT_OCCURRED2_RTC_18_MASK: c_uint = 0x0000001000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_19_SHFT: c_int = 37;
pub const UV4H_EVENT_OCCURRED2_RTC_19_MASK: c_uint = 0x0000002000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_20_SHFT: c_int = 38;
pub const UV4H_EVENT_OCCURRED2_RTC_20_MASK: c_uint = 0x0000004000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_21_SHFT: c_int = 39;
pub const UV4H_EVENT_OCCURRED2_RTC_21_MASK: c_uint = 0x0000008000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_22_SHFT: c_int = 40;
pub const UV4H_EVENT_OCCURRED2_RTC_22_MASK: c_uint = 0x0000010000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_23_SHFT: c_int = 41;
pub const UV4H_EVENT_OCCURRED2_RTC_23_MASK: c_uint = 0x0000020000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_24_SHFT: c_int = 42;
pub const UV4H_EVENT_OCCURRED2_RTC_24_MASK: c_uint = 0x0000040000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_25_SHFT: c_int = 43;
pub const UV4H_EVENT_OCCURRED2_RTC_25_MASK: c_uint = 0x0000080000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_26_SHFT: c_int = 44;
pub const UV4H_EVENT_OCCURRED2_RTC_26_MASK: c_uint = 0x0000100000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_27_SHFT: c_int = 45;
pub const UV4H_EVENT_OCCURRED2_RTC_27_MASK: c_uint = 0x0000200000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_28_SHFT: c_int = 46;
pub const UV4H_EVENT_OCCURRED2_RTC_28_MASK: c_uint = 0x0000400000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_29_SHFT: c_int = 47;
pub const UV4H_EVENT_OCCURRED2_RTC_29_MASK: c_uint = 0x0000800000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_30_SHFT: c_int = 48;
pub const UV4H_EVENT_OCCURRED2_RTC_30_MASK: c_uint = 0x0001000000000000UL;
pub const UV4H_EVENT_OCCURRED2_RTC_31_SHFT: c_int = 49;
pub const UV4H_EVENT_OCCURRED2_RTC_31_MASK: c_uint = 0x0002000000000000UL;
// UV3 unique defines
pub const UV3H_EVENT_OCCURRED2_RTC_0_SHFT: c_int = 0;
pub const UV3H_EVENT_OCCURRED2_RTC_0_MASK: c_uint = 0x0000000000000001UL;
pub const UV3H_EVENT_OCCURRED2_RTC_1_SHFT: c_int = 1;
pub const UV3H_EVENT_OCCURRED2_RTC_1_MASK: c_uint = 0x0000000000000002UL;
pub const UV3H_EVENT_OCCURRED2_RTC_2_SHFT: c_int = 2;
pub const UV3H_EVENT_OCCURRED2_RTC_2_MASK: c_uint = 0x0000000000000004UL;
pub const UV3H_EVENT_OCCURRED2_RTC_3_SHFT: c_int = 3;
pub const UV3H_EVENT_OCCURRED2_RTC_3_MASK: c_uint = 0x0000000000000008UL;
pub const UV3H_EVENT_OCCURRED2_RTC_4_SHFT: c_int = 4;
pub const UV3H_EVENT_OCCURRED2_RTC_4_MASK: c_uint = 0x0000000000000010UL;
pub const UV3H_EVENT_OCCURRED2_RTC_5_SHFT: c_int = 5;
pub const UV3H_EVENT_OCCURRED2_RTC_5_MASK: c_uint = 0x0000000000000020UL;
pub const UV3H_EVENT_OCCURRED2_RTC_6_SHFT: c_int = 6;
pub const UV3H_EVENT_OCCURRED2_RTC_6_MASK: c_uint = 0x0000000000000040UL;
pub const UV3H_EVENT_OCCURRED2_RTC_7_SHFT: c_int = 7;
pub const UV3H_EVENT_OCCURRED2_RTC_7_MASK: c_uint = 0x0000000000000080UL;
pub const UV3H_EVENT_OCCURRED2_RTC_8_SHFT: c_int = 8;
pub const UV3H_EVENT_OCCURRED2_RTC_8_MASK: c_uint = 0x0000000000000100UL;
pub const UV3H_EVENT_OCCURRED2_RTC_9_SHFT: c_int = 9;
pub const UV3H_EVENT_OCCURRED2_RTC_9_MASK: c_uint = 0x0000000000000200UL;
pub const UV3H_EVENT_OCCURRED2_RTC_10_SHFT: c_int = 10;
pub const UV3H_EVENT_OCCURRED2_RTC_10_MASK: c_uint = 0x0000000000000400UL;
pub const UV3H_EVENT_OCCURRED2_RTC_11_SHFT: c_int = 11;
pub const UV3H_EVENT_OCCURRED2_RTC_11_MASK: c_uint = 0x0000000000000800UL;
pub const UV3H_EVENT_OCCURRED2_RTC_12_SHFT: c_int = 12;
pub const UV3H_EVENT_OCCURRED2_RTC_12_MASK: c_uint = 0x0000000000001000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_13_SHFT: c_int = 13;
pub const UV3H_EVENT_OCCURRED2_RTC_13_MASK: c_uint = 0x0000000000002000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_14_SHFT: c_int = 14;
pub const UV3H_EVENT_OCCURRED2_RTC_14_MASK: c_uint = 0x0000000000004000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_15_SHFT: c_int = 15;
pub const UV3H_EVENT_OCCURRED2_RTC_15_MASK: c_uint = 0x0000000000008000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_16_SHFT: c_int = 16;
pub const UV3H_EVENT_OCCURRED2_RTC_16_MASK: c_uint = 0x0000000000010000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_17_SHFT: c_int = 17;
pub const UV3H_EVENT_OCCURRED2_RTC_17_MASK: c_uint = 0x0000000000020000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_18_SHFT: c_int = 18;
pub const UV3H_EVENT_OCCURRED2_RTC_18_MASK: c_uint = 0x0000000000040000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_19_SHFT: c_int = 19;
pub const UV3H_EVENT_OCCURRED2_RTC_19_MASK: c_uint = 0x0000000000080000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_20_SHFT: c_int = 20;
pub const UV3H_EVENT_OCCURRED2_RTC_20_MASK: c_uint = 0x0000000000100000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_21_SHFT: c_int = 21;
pub const UV3H_EVENT_OCCURRED2_RTC_21_MASK: c_uint = 0x0000000000200000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_22_SHFT: c_int = 22;
pub const UV3H_EVENT_OCCURRED2_RTC_22_MASK: c_uint = 0x0000000000400000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_23_SHFT: c_int = 23;
pub const UV3H_EVENT_OCCURRED2_RTC_23_MASK: c_uint = 0x0000000000800000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_24_SHFT: c_int = 24;
pub const UV3H_EVENT_OCCURRED2_RTC_24_MASK: c_uint = 0x0000000001000000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_25_SHFT: c_int = 25;
pub const UV3H_EVENT_OCCURRED2_RTC_25_MASK: c_uint = 0x0000000002000000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_26_SHFT: c_int = 26;
pub const UV3H_EVENT_OCCURRED2_RTC_26_MASK: c_uint = 0x0000000004000000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_27_SHFT: c_int = 27;
pub const UV3H_EVENT_OCCURRED2_RTC_27_MASK: c_uint = 0x0000000008000000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_28_SHFT: c_int = 28;
pub const UV3H_EVENT_OCCURRED2_RTC_28_MASK: c_uint = 0x0000000010000000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_29_SHFT: c_int = 29;
pub const UV3H_EVENT_OCCURRED2_RTC_29_MASK: c_uint = 0x0000000020000000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_30_SHFT: c_int = 30;
pub const UV3H_EVENT_OCCURRED2_RTC_30_MASK: c_uint = 0x0000000040000000UL;
pub const UV3H_EVENT_OCCURRED2_RTC_31_SHFT: c_int = 31;
pub const UV3H_EVENT_OCCURRED2_RTC_31_MASK: c_uint = 0x0000000080000000UL;
// UV2 unique defines
pub const UV2H_EVENT_OCCURRED2_RTC_0_SHFT: c_int = 0;
pub const UV2H_EVENT_OCCURRED2_RTC_0_MASK: c_uint = 0x0000000000000001UL;
pub const UV2H_EVENT_OCCURRED2_RTC_1_SHFT: c_int = 1;
pub const UV2H_EVENT_OCCURRED2_RTC_1_MASK: c_uint = 0x0000000000000002UL;
pub const UV2H_EVENT_OCCURRED2_RTC_2_SHFT: c_int = 2;
pub const UV2H_EVENT_OCCURRED2_RTC_2_MASK: c_uint = 0x0000000000000004UL;
pub const UV2H_EVENT_OCCURRED2_RTC_3_SHFT: c_int = 3;
pub const UV2H_EVENT_OCCURRED2_RTC_3_MASK: c_uint = 0x0000000000000008UL;
pub const UV2H_EVENT_OCCURRED2_RTC_4_SHFT: c_int = 4;
pub const UV2H_EVENT_OCCURRED2_RTC_4_MASK: c_uint = 0x0000000000000010UL;
pub const UV2H_EVENT_OCCURRED2_RTC_5_SHFT: c_int = 5;
pub const UV2H_EVENT_OCCURRED2_RTC_5_MASK: c_uint = 0x0000000000000020UL;
pub const UV2H_EVENT_OCCURRED2_RTC_6_SHFT: c_int = 6;
pub const UV2H_EVENT_OCCURRED2_RTC_6_MASK: c_uint = 0x0000000000000040UL;
pub const UV2H_EVENT_OCCURRED2_RTC_7_SHFT: c_int = 7;
pub const UV2H_EVENT_OCCURRED2_RTC_7_MASK: c_uint = 0x0000000000000080UL;
pub const UV2H_EVENT_OCCURRED2_RTC_8_SHFT: c_int = 8;
pub const UV2H_EVENT_OCCURRED2_RTC_8_MASK: c_uint = 0x0000000000000100UL;
pub const UV2H_EVENT_OCCURRED2_RTC_9_SHFT: c_int = 9;
pub const UV2H_EVENT_OCCURRED2_RTC_9_MASK: c_uint = 0x0000000000000200UL;
pub const UV2H_EVENT_OCCURRED2_RTC_10_SHFT: c_int = 10;
pub const UV2H_EVENT_OCCURRED2_RTC_10_MASK: c_uint = 0x0000000000000400UL;
pub const UV2H_EVENT_OCCURRED2_RTC_11_SHFT: c_int = 11;
pub const UV2H_EVENT_OCCURRED2_RTC_11_MASK: c_uint = 0x0000000000000800UL;
pub const UV2H_EVENT_OCCURRED2_RTC_12_SHFT: c_int = 12;
pub const UV2H_EVENT_OCCURRED2_RTC_12_MASK: c_uint = 0x0000000000001000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_13_SHFT: c_int = 13;
pub const UV2H_EVENT_OCCURRED2_RTC_13_MASK: c_uint = 0x0000000000002000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_14_SHFT: c_int = 14;
pub const UV2H_EVENT_OCCURRED2_RTC_14_MASK: c_uint = 0x0000000000004000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_15_SHFT: c_int = 15;
pub const UV2H_EVENT_OCCURRED2_RTC_15_MASK: c_uint = 0x0000000000008000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_16_SHFT: c_int = 16;
pub const UV2H_EVENT_OCCURRED2_RTC_16_MASK: c_uint = 0x0000000000010000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_17_SHFT: c_int = 17;
pub const UV2H_EVENT_OCCURRED2_RTC_17_MASK: c_uint = 0x0000000000020000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_18_SHFT: c_int = 18;
pub const UV2H_EVENT_OCCURRED2_RTC_18_MASK: c_uint = 0x0000000000040000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_19_SHFT: c_int = 19;
pub const UV2H_EVENT_OCCURRED2_RTC_19_MASK: c_uint = 0x0000000000080000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_20_SHFT: c_int = 20;
pub const UV2H_EVENT_OCCURRED2_RTC_20_MASK: c_uint = 0x0000000000100000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_21_SHFT: c_int = 21;
pub const UV2H_EVENT_OCCURRED2_RTC_21_MASK: c_uint = 0x0000000000200000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_22_SHFT: c_int = 22;
pub const UV2H_EVENT_OCCURRED2_RTC_22_MASK: c_uint = 0x0000000000400000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_23_SHFT: c_int = 23;
pub const UV2H_EVENT_OCCURRED2_RTC_23_MASK: c_uint = 0x0000000000800000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_24_SHFT: c_int = 24;
pub const UV2H_EVENT_OCCURRED2_RTC_24_MASK: c_uint = 0x0000000001000000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_25_SHFT: c_int = 25;
pub const UV2H_EVENT_OCCURRED2_RTC_25_MASK: c_uint = 0x0000000002000000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_26_SHFT: c_int = 26;
pub const UV2H_EVENT_OCCURRED2_RTC_26_MASK: c_uint = 0x0000000004000000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_27_SHFT: c_int = 27;
pub const UV2H_EVENT_OCCURRED2_RTC_27_MASK: c_uint = 0x0000000008000000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_28_SHFT: c_int = 28;
pub const UV2H_EVENT_OCCURRED2_RTC_28_MASK: c_uint = 0x0000000010000000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_29_SHFT: c_int = 29;
pub const UV2H_EVENT_OCCURRED2_RTC_29_MASK: c_uint = 0x0000000020000000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_30_SHFT: c_int = 30;
pub const UV2H_EVENT_OCCURRED2_RTC_30_MASK: c_uint = 0x0000000040000000UL;
pub const UV2H_EVENT_OCCURRED2_RTC_31_SHFT: c_int = 31;
pub const UV2H_EVENT_OCCURRED2_RTC_31_MASK: c_uint = 0x0000000080000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvyh_event_occurred2_u {
    pub v: c_ulong,
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_event_occurred2_s {
    pub /: *mut *mut unsigned long rtc_interval_int:1; / RW,
    pub /: *mut *mut unsigned long bau_dashboard_int:1; / RW,
    pub /: *mut *mut unsigned long rtc_0:1; / RW,
    pub /: *mut *mut unsigned long rtc_1:1; / RW,
    pub /: *mut *mut unsigned long rtc_2:1; / RW,
    pub /: *mut *mut unsigned long rtc_3:1; / RW,
    pub /: *mut *mut unsigned long rtc_4:1; / RW,
    pub /: *mut *mut unsigned long rtc_5:1; / RW,
    pub /: *mut *mut unsigned long rtc_6:1; / RW,
    pub /: *mut *mut unsigned long rtc_7:1; / RW,
    pub /: *mut *mut unsigned long rtc_8:1; / RW,
    pub /: *mut *mut unsigned long rtc_9:1; / RW,
    pub /: *mut *mut unsigned long rtc_10:1; / RW,
    pub /: *mut *mut unsigned long rtc_11:1; / RW,
    pub /: *mut *mut unsigned long rtc_12:1; / RW,
    pub /: *mut *mut unsigned long rtc_13:1; / RW,
    pub /: *mut *mut unsigned long rtc_14:1; / RW,
    pub /: *mut *mut unsigned long rtc_15:1; / RW,
    pub /: *mut *mut unsigned long rtc_16:1; / RW,
    pub /: *mut *mut unsigned long rtc_17:1; / RW,
    pub /: *mut *mut unsigned long rtc_18:1; / RW,
    pub /: *mut *mut unsigned long rtc_19:1; / RW,
    pub /: *mut *mut unsigned long rtc_20:1; / RW,
    pub /: *mut *mut unsigned long rtc_21:1; / RW,
    pub /: *mut *mut unsigned long rtc_22:1; / RW,
    pub /: *mut *mut unsigned long rtc_23:1; / RW,
    pub /: *mut *mut unsigned long rtc_24:1; / RW,
    pub /: *mut *mut unsigned long rtc_25:1; / RW,
    pub /: *mut *mut unsigned long rtc_26:1; / RW,
    pub /: *mut *mut unsigned long rtc_27:1; / RW,
    pub /: *mut *mut unsigned long rtc_28:1; / RW,
    pub /: *mut *mut unsigned long rtc_29:1; / RW,
    pub /: *mut *mut unsigned long rtc_30:1; / RW,
    pub /: *mut *mut unsigned long rtc_31:1; / RW,
    pub rsvd_34_63:30: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_event_occurred2_s {
    pub /: *mut *mut unsigned long rtc_interval_int:1; / RW,
    pub /: *mut *mut unsigned long bau_dashboard_int:1; / RW,
    pub /: *mut *mut unsigned long rtc_0:1; / RW,
    pub /: *mut *mut unsigned long rtc_1:1; / RW,
    pub /: *mut *mut unsigned long rtc_2:1; / RW,
    pub /: *mut *mut unsigned long rtc_3:1; / RW,
    pub /: *mut *mut unsigned long rtc_4:1; / RW,
    pub /: *mut *mut unsigned long rtc_5:1; / RW,
    pub /: *mut *mut unsigned long rtc_6:1; / RW,
    pub /: *mut *mut unsigned long rtc_7:1; / RW,
    pub /: *mut *mut unsigned long rtc_8:1; / RW,
    pub /: *mut *mut unsigned long rtc_9:1; / RW,
    pub /: *mut *mut unsigned long rtc_10:1; / RW,
    pub /: *mut *mut unsigned long rtc_11:1; / RW,
    pub /: *mut *mut unsigned long rtc_12:1; / RW,
    pub /: *mut *mut unsigned long rtc_13:1; / RW,
    pub /: *mut *mut unsigned long rtc_14:1; / RW,
    pub /: *mut *mut unsigned long rtc_15:1; / RW,
    pub /: *mut *mut unsigned long rtc_16:1; / RW,
    pub /: *mut *mut unsigned long rtc_17:1; / RW,
    pub /: *mut *mut unsigned long rtc_18:1; / RW,
    pub /: *mut *mut unsigned long rtc_19:1; / RW,
    pub /: *mut *mut unsigned long rtc_20:1; / RW,
    pub /: *mut *mut unsigned long rtc_21:1; / RW,
    pub /: *mut *mut unsigned long rtc_22:1; / RW,
    pub /: *mut *mut unsigned long rtc_23:1; / RW,
    pub /: *mut *mut unsigned long rtc_24:1; / RW,
    pub /: *mut *mut unsigned long rtc_25:1; / RW,
    pub /: *mut *mut unsigned long rtc_26:1; / RW,
    pub /: *mut *mut unsigned long rtc_27:1; / RW,
    pub /: *mut *mut unsigned long rtc_28:1; / RW,
    pub /: *mut *mut unsigned long rtc_29:1; / RW,
    pub /: *mut *mut unsigned long rtc_30:1; / RW,
    pub /: *mut *mut unsigned long rtc_31:1; / RW,
    pub rsvd_34_63:30: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_event_occurred2_s {
    pub /: *mut *mut unsigned long message_accelerator_int0:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int1:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int2:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int3:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int4:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int5:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int6:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int7:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int8:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int9:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int10:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int11:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int12:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int13:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int14:1; / RW,
    pub /: *mut *mut unsigned long message_accelerator_int15:1; / RW,
    pub /: *mut *mut unsigned long rtc_interval_int:1; / RW,
    pub /: *mut *mut unsigned long bau_dashboard_int:1; / RW,
    pub /: *mut *mut unsigned long rtc_0:1; / RW,
    pub /: *mut *mut unsigned long rtc_1:1; / RW,
    pub /: *mut *mut unsigned long rtc_2:1; / RW,
    pub /: *mut *mut unsigned long rtc_3:1; / RW,
    pub /: *mut *mut unsigned long rtc_4:1; / RW,
    pub /: *mut *mut unsigned long rtc_5:1; / RW,
    pub /: *mut *mut unsigned long rtc_6:1; / RW,
    pub /: *mut *mut unsigned long rtc_7:1; / RW,
    pub /: *mut *mut unsigned long rtc_8:1; / RW,
    pub /: *mut *mut unsigned long rtc_9:1; / RW,
    pub /: *mut *mut unsigned long rtc_10:1; / RW,
    pub /: *mut *mut unsigned long rtc_11:1; / RW,
    pub /: *mut *mut unsigned long rtc_12:1; / RW,
    pub /: *mut *mut unsigned long rtc_13:1; / RW,
    pub /: *mut *mut unsigned long rtc_14:1; / RW,
    pub /: *mut *mut unsigned long rtc_15:1; / RW,
    pub /: *mut *mut unsigned long rtc_16:1; / RW,
    pub /: *mut *mut unsigned long rtc_17:1; / RW,
    pub /: *mut *mut unsigned long rtc_18:1; / RW,
    pub /: *mut *mut unsigned long rtc_19:1; / RW,
    pub /: *mut *mut unsigned long rtc_20:1; / RW,
    pub /: *mut *mut unsigned long rtc_21:1; / RW,
    pub /: *mut *mut unsigned long rtc_22:1; / RW,
    pub /: *mut *mut unsigned long rtc_23:1; / RW,
    pub /: *mut *mut unsigned long rtc_24:1; / RW,
    pub /: *mut *mut unsigned long rtc_25:1; / RW,
    pub /: *mut *mut unsigned long rtc_26:1; / RW,
    pub /: *mut *mut unsigned long rtc_27:1; / RW,
    pub /: *mut *mut unsigned long rtc_28:1; / RW,
    pub /: *mut *mut unsigned long rtc_29:1; / RW,
    pub /: *mut *mut unsigned long rtc_30:1; / RW,
    pub /: *mut *mut unsigned long rtc_31:1; / RW,
    pub rsvd_50_63:14: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_event_occurred2_s {
    pub /: *mut *mut unsigned long rtc_0:1; / RW,
    pub /: *mut *mut unsigned long rtc_1:1; / RW,
    pub /: *mut *mut unsigned long rtc_2:1; / RW,
    pub /: *mut *mut unsigned long rtc_3:1; / RW,
    pub /: *mut *mut unsigned long rtc_4:1; / RW,
    pub /: *mut *mut unsigned long rtc_5:1; / RW,
    pub /: *mut *mut unsigned long rtc_6:1; / RW,
    pub /: *mut *mut unsigned long rtc_7:1; / RW,
    pub /: *mut *mut unsigned long rtc_8:1; / RW,
    pub /: *mut *mut unsigned long rtc_9:1; / RW,
    pub /: *mut *mut unsigned long rtc_10:1; / RW,
    pub /: *mut *mut unsigned long rtc_11:1; / RW,
    pub /: *mut *mut unsigned long rtc_12:1; / RW,
    pub /: *mut *mut unsigned long rtc_13:1; / RW,
    pub /: *mut *mut unsigned long rtc_14:1; / RW,
    pub /: *mut *mut unsigned long rtc_15:1; / RW,
    pub /: *mut *mut unsigned long rtc_16:1; / RW,
    pub /: *mut *mut unsigned long rtc_17:1; / RW,
    pub /: *mut *mut unsigned long rtc_18:1; / RW,
    pub /: *mut *mut unsigned long rtc_19:1; / RW,
    pub /: *mut *mut unsigned long rtc_20:1; / RW,
    pub /: *mut *mut unsigned long rtc_21:1; / RW,
    pub /: *mut *mut unsigned long rtc_22:1; / RW,
    pub /: *mut *mut unsigned long rtc_23:1; / RW,
    pub /: *mut *mut unsigned long rtc_24:1; / RW,
    pub /: *mut *mut unsigned long rtc_25:1; / RW,
    pub /: *mut *mut unsigned long rtc_26:1; / RW,
    pub /: *mut *mut unsigned long rtc_27:1; / RW,
    pub /: *mut *mut unsigned long rtc_28:1; / RW,
    pub /: *mut *mut unsigned long rtc_29:1; / RW,
    pub /: *mut *mut unsigned long rtc_30:1; / RW,
    pub /: *mut *mut unsigned long rtc_31:1; / RW,
    pub rsvd_32_63:32: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_event_occurred2_s {
    pub /: *mut *mut unsigned long rtc_0:1; / RW,
    pub /: *mut *mut unsigned long rtc_1:1; / RW,
    pub /: *mut *mut unsigned long rtc_2:1; / RW,
    pub /: *mut *mut unsigned long rtc_3:1; / RW,
    pub /: *mut *mut unsigned long rtc_4:1; / RW,
    pub /: *mut *mut unsigned long rtc_5:1; / RW,
    pub /: *mut *mut unsigned long rtc_6:1; / RW,
    pub /: *mut *mut unsigned long rtc_7:1; / RW,
    pub /: *mut *mut unsigned long rtc_8:1; / RW,
    pub /: *mut *mut unsigned long rtc_9:1; / RW,
    pub /: *mut *mut unsigned long rtc_10:1; / RW,
    pub /: *mut *mut unsigned long rtc_11:1; / RW,
    pub /: *mut *mut unsigned long rtc_12:1; / RW,
    pub /: *mut *mut unsigned long rtc_13:1; / RW,
    pub /: *mut *mut unsigned long rtc_14:1; / RW,
    pub /: *mut *mut unsigned long rtc_15:1; / RW,
    pub /: *mut *mut unsigned long rtc_16:1; / RW,
    pub /: *mut *mut unsigned long rtc_17:1; / RW,
    pub /: *mut *mut unsigned long rtc_18:1; / RW,
    pub /: *mut *mut unsigned long rtc_19:1; / RW,
    pub /: *mut *mut unsigned long rtc_20:1; / RW,
    pub /: *mut *mut unsigned long rtc_21:1; / RW,
    pub /: *mut *mut unsigned long rtc_22:1; / RW,
    pub /: *mut *mut unsigned long rtc_23:1; / RW,
    pub /: *mut *mut unsigned long rtc_24:1; / RW,
    pub /: *mut *mut unsigned long rtc_25:1; / RW,
    pub /: *mut *mut unsigned long rtc_26:1; / RW,
    pub /: *mut *mut unsigned long rtc_27:1; / RW,
    pub /: *mut *mut unsigned long rtc_28:1; / RW,
    pub /: *mut *mut unsigned long rtc_29:1; / RW,
    pub /: *mut *mut unsigned long rtc_30:1; / RW,
    pub /: *mut *mut unsigned long rtc_31:1; / RW,
    pub rsvd_32_63:32: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_EVENT_OCCURRED2_ALIAS
// =========================================================================
pub const UVH_EVENT_OCCURRED2_ALIAS: c_uint = 0x70108UL;
// =========================================================================
// UVH_EXTIO_INT0_BROADCAST
// =========================================================================
pub const UVH_EXTIO_INT0_BROADCAST: c_uint = 0x61448UL;
// UVH common defines
pub const UVH_EXTIO_INT0_BROADCAST_ENABLE_SHFT: c_int = 0;
pub const UVH_EXTIO_INT0_BROADCAST_ENABLE_MASK: c_uint = 0x0000000000000001UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_extio_int0_broadcast_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_extio_int0_broadcast_s {
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub rsvd_1_63:63: c_ulong,
    pub s: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_extio_int0_broadcast_s {
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub rsvd_1_63:63: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_extio_int0_broadcast_s {
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub rsvd_1_63:63: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_extio_int0_broadcast_s {
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub rsvd_1_63:63: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_extio_int0_broadcast_s {
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub rsvd_1_63:63: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_GR0_GAM_GR_CONFIG
// =========================================================================

// UVYH common defines
pub const UVYH_GR0_GAM_GR_CONFIG_SUBSPACE_SHFT: c_int = 10;
pub const UVYH_GR0_GAM_GR_CONFIG_SUBSPACE_MASK: c_uint = 0x0000000000000400UL;
// UV4 unique defines
pub const UV4H_GR0_GAM_GR_CONFIG_SUBSPACE_SHFT: c_int = 10;
pub const UV4H_GR0_GAM_GR_CONFIG_SUBSPACE_MASK: c_uint = 0x0000000000000400UL;
// UV3 unique defines
pub const UV3H_GR0_GAM_GR_CONFIG_M_SKT_SHFT: c_int = 0;
pub const UV3H_GR0_GAM_GR_CONFIG_M_SKT_MASK: c_uint = 0x000000000000003fUL;
pub const UV3H_GR0_GAM_GR_CONFIG_SUBSPACE_SHFT: c_int = 10;
pub const UV3H_GR0_GAM_GR_CONFIG_SUBSPACE_MASK: c_uint = 0x0000000000000400UL;
// UV2 unique defines
pub const UV2H_GR0_GAM_GR_CONFIG_N_GR_SHFT: c_int = 0;
pub const UV2H_GR0_GAM_GR_CONFIG_N_GR_MASK: c_uint = 0x000000000000000fUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvyh_gr0_gam_gr_config_u {
    pub v: c_ulong,
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_gr0_gam_gr_config_s {
    pub rsvd_0_9:10: c_ulong,
    pub /: *mut *mut unsigned long subspace:1; / RW,
    pub rsvd_11_63:53: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_gr0_gam_gr_config_s {
    pub rsvd_0_9:10: c_ulong,
    pub /: *mut *mut unsigned long subspace:1; / RW,
    pub rsvd_11_63:53: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_gr0_gam_gr_config_s {
    pub rsvd_0_9:10: c_ulong,
    pub /: *mut *mut unsigned long subspace:1; / RW,
    pub rsvd_11_63:53: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_gr0_gam_gr_config_s {
    pub /: *mut *mut unsigned long m_skt:6; / RW,
    pub /: *mut *mut unsigned long undef_6_9:4; / Undefined,
    pub /: *mut *mut unsigned long subspace:1; / RW,
    pub reserved:53: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_gr0_gam_gr_config_s {
    pub /: *mut *mut unsigned long n_gr:4; / RW,
    pub reserved:60: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_GR0_TLB_INT0_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_GR0_TLB_INT0_CONFIG_VECTOR_SHFT: c_int = 0;
pub const UVXH_GR0_TLB_INT0_CONFIG_VECTOR_MASK: c_uint = 0x00000000000000ffUL;
pub const UVXH_GR0_TLB_INT0_CONFIG_DM_SHFT: c_int = 8;
pub const UVXH_GR0_TLB_INT0_CONFIG_DM_MASK: c_uint = 0x0000000000000700UL;
pub const UVXH_GR0_TLB_INT0_CONFIG_DESTMODE_SHFT: c_int = 11;
pub const UVXH_GR0_TLB_INT0_CONFIG_DESTMODE_MASK: c_uint = 0x0000000000000800UL;
pub const UVXH_GR0_TLB_INT0_CONFIG_STATUS_SHFT: c_int = 12;
pub const UVXH_GR0_TLB_INT0_CONFIG_STATUS_MASK: c_uint = 0x0000000000001000UL;
pub const UVXH_GR0_TLB_INT0_CONFIG_P_SHFT: c_int = 13;
pub const UVXH_GR0_TLB_INT0_CONFIG_P_MASK: c_uint = 0x0000000000002000UL;
pub const UVXH_GR0_TLB_INT0_CONFIG_T_SHFT: c_int = 15;
pub const UVXH_GR0_TLB_INT0_CONFIG_T_MASK: c_uint = 0x0000000000008000UL;
pub const UVXH_GR0_TLB_INT0_CONFIG_M_SHFT: c_int = 16;
pub const UVXH_GR0_TLB_INT0_CONFIG_M_MASK: c_uint = 0x0000000000010000UL;
pub const UVXH_GR0_TLB_INT0_CONFIG_APIC_ID_SHFT: c_int = 32;
pub const UVXH_GR0_TLB_INT0_CONFIG_APIC_ID_MASK: c_uint = 0xffffffff00000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_gr0_tlb_int0_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_gr0_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_gr0_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_gr0_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_gr0_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_gr0_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s2: },
}

// =========================================================================
// UVH_GR0_TLB_INT1_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_GR0_TLB_INT1_CONFIG_VECTOR_SHFT: c_int = 0;
pub const UVXH_GR0_TLB_INT1_CONFIG_VECTOR_MASK: c_uint = 0x00000000000000ffUL;
pub const UVXH_GR0_TLB_INT1_CONFIG_DM_SHFT: c_int = 8;
pub const UVXH_GR0_TLB_INT1_CONFIG_DM_MASK: c_uint = 0x0000000000000700UL;
pub const UVXH_GR0_TLB_INT1_CONFIG_DESTMODE_SHFT: c_int = 11;
pub const UVXH_GR0_TLB_INT1_CONFIG_DESTMODE_MASK: c_uint = 0x0000000000000800UL;
pub const UVXH_GR0_TLB_INT1_CONFIG_STATUS_SHFT: c_int = 12;
pub const UVXH_GR0_TLB_INT1_CONFIG_STATUS_MASK: c_uint = 0x0000000000001000UL;
pub const UVXH_GR0_TLB_INT1_CONFIG_P_SHFT: c_int = 13;
pub const UVXH_GR0_TLB_INT1_CONFIG_P_MASK: c_uint = 0x0000000000002000UL;
pub const UVXH_GR0_TLB_INT1_CONFIG_T_SHFT: c_int = 15;
pub const UVXH_GR0_TLB_INT1_CONFIG_T_MASK: c_uint = 0x0000000000008000UL;
pub const UVXH_GR0_TLB_INT1_CONFIG_M_SHFT: c_int = 16;
pub const UVXH_GR0_TLB_INT1_CONFIG_M_MASK: c_uint = 0x0000000000010000UL;
pub const UVXH_GR0_TLB_INT1_CONFIG_APIC_ID_SHFT: c_int = 32;
pub const UVXH_GR0_TLB_INT1_CONFIG_APIC_ID_MASK: c_uint = 0xffffffff00000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_gr0_tlb_int1_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_gr0_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_gr0_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_gr0_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_gr0_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_gr0_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s2: },
}

// =========================================================================
// UVH_GR1_TLB_INT0_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_GR1_TLB_INT0_CONFIG_VECTOR_SHFT: c_int = 0;
pub const UVXH_GR1_TLB_INT0_CONFIG_VECTOR_MASK: c_uint = 0x00000000000000ffUL;
pub const UVXH_GR1_TLB_INT0_CONFIG_DM_SHFT: c_int = 8;
pub const UVXH_GR1_TLB_INT0_CONFIG_DM_MASK: c_uint = 0x0000000000000700UL;
pub const UVXH_GR1_TLB_INT0_CONFIG_DESTMODE_SHFT: c_int = 11;
pub const UVXH_GR1_TLB_INT0_CONFIG_DESTMODE_MASK: c_uint = 0x0000000000000800UL;
pub const UVXH_GR1_TLB_INT0_CONFIG_STATUS_SHFT: c_int = 12;
pub const UVXH_GR1_TLB_INT0_CONFIG_STATUS_MASK: c_uint = 0x0000000000001000UL;
pub const UVXH_GR1_TLB_INT0_CONFIG_P_SHFT: c_int = 13;
pub const UVXH_GR1_TLB_INT0_CONFIG_P_MASK: c_uint = 0x0000000000002000UL;
pub const UVXH_GR1_TLB_INT0_CONFIG_T_SHFT: c_int = 15;
pub const UVXH_GR1_TLB_INT0_CONFIG_T_MASK: c_uint = 0x0000000000008000UL;
pub const UVXH_GR1_TLB_INT0_CONFIG_M_SHFT: c_int = 16;
pub const UVXH_GR1_TLB_INT0_CONFIG_M_MASK: c_uint = 0x0000000000010000UL;
pub const UVXH_GR1_TLB_INT0_CONFIG_APIC_ID_SHFT: c_int = 32;
pub const UVXH_GR1_TLB_INT0_CONFIG_APIC_ID_MASK: c_uint = 0xffffffff00000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_gr1_tlb_int0_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_gr1_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_gr1_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_gr1_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_gr1_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_gr1_tlb_int0_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s2: },
}

// =========================================================================
// UVH_GR1_TLB_INT1_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_GR1_TLB_INT1_CONFIG_VECTOR_SHFT: c_int = 0;
pub const UVXH_GR1_TLB_INT1_CONFIG_VECTOR_MASK: c_uint = 0x00000000000000ffUL;
pub const UVXH_GR1_TLB_INT1_CONFIG_DM_SHFT: c_int = 8;
pub const UVXH_GR1_TLB_INT1_CONFIG_DM_MASK: c_uint = 0x0000000000000700UL;
pub const UVXH_GR1_TLB_INT1_CONFIG_DESTMODE_SHFT: c_int = 11;
pub const UVXH_GR1_TLB_INT1_CONFIG_DESTMODE_MASK: c_uint = 0x0000000000000800UL;
pub const UVXH_GR1_TLB_INT1_CONFIG_STATUS_SHFT: c_int = 12;
pub const UVXH_GR1_TLB_INT1_CONFIG_STATUS_MASK: c_uint = 0x0000000000001000UL;
pub const UVXH_GR1_TLB_INT1_CONFIG_P_SHFT: c_int = 13;
pub const UVXH_GR1_TLB_INT1_CONFIG_P_MASK: c_uint = 0x0000000000002000UL;
pub const UVXH_GR1_TLB_INT1_CONFIG_T_SHFT: c_int = 15;
pub const UVXH_GR1_TLB_INT1_CONFIG_T_MASK: c_uint = 0x0000000000008000UL;
pub const UVXH_GR1_TLB_INT1_CONFIG_M_SHFT: c_int = 16;
pub const UVXH_GR1_TLB_INT1_CONFIG_M_MASK: c_uint = 0x0000000000010000UL;
pub const UVXH_GR1_TLB_INT1_CONFIG_APIC_ID_SHFT: c_int = 32;
pub const UVXH_GR1_TLB_INT1_CONFIG_APIC_ID_MASK: c_uint = 0xffffffff00000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_gr1_tlb_int1_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_gr1_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_gr1_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_gr1_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_gr1_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_gr1_tlb_int1_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s2: },
}

// =========================================================================
// UVH_INT_CMPB
// =========================================================================
pub const UVH_INT_CMPB: c_uint = 0x22080UL;
// UVH common defines
pub const UVH_INT_CMPB_REAL_TIME_CMPB_SHFT: c_int = 0;
pub const UVH_INT_CMPB_REAL_TIME_CMPB_MASK: c_uint = 0x00ffffffffffffffUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_int_cmpb_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_int_cmpb_s {
    pub /: *mut *mut unsigned long real_time_cmpb:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_int_cmpb_s {
    pub /: *mut *mut unsigned long real_time_cmpb:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_int_cmpb_s {
    pub /: *mut *mut unsigned long real_time_cmpb:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_int_cmpb_s {
    pub /: *mut *mut unsigned long real_time_cmpb:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_int_cmpb_s {
    pub /: *mut *mut unsigned long real_time_cmpb:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_IPI_INT
// =========================================================================
pub const UVH_IPI_INT: c_uint = 0x60500UL;
// UVH common defines
pub const UVH_IPI_INT_VECTOR_SHFT: c_int = 0;
pub const UVH_IPI_INT_VECTOR_MASK: c_uint = 0x00000000000000ffUL;
pub const UVH_IPI_INT_DELIVERY_MODE_SHFT: c_int = 8;
pub const UVH_IPI_INT_DELIVERY_MODE_MASK: c_uint = 0x0000000000000700UL;
pub const UVH_IPI_INT_DESTMODE_SHFT: c_int = 11;
pub const UVH_IPI_INT_DESTMODE_MASK: c_uint = 0x0000000000000800UL;
pub const UVH_IPI_INT_APIC_ID_SHFT: c_int = 16;
pub const UVH_IPI_INT_APIC_ID_MASK: c_uint = 0x0000ffffffff0000UL;
pub const UVH_IPI_INT_SEND_SHFT: c_int = 63;
pub const UVH_IPI_INT_SEND_MASK: c_uint = 0x8000000000000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_ipi_int_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_ipi_int_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long delivery_mode:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub rsvd_12_15:4: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub rsvd_48_62:15: c_ulong,
    pub /: *mut *mut unsigned long send:1; / WP,
    pub s: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_ipi_int_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long delivery_mode:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub rsvd_12_15:4: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub rsvd_48_62:15: c_ulong,
    pub /: *mut *mut unsigned long send:1; / WP,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_ipi_int_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long delivery_mode:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub rsvd_12_15:4: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub rsvd_48_62:15: c_ulong,
    pub /: *mut *mut unsigned long send:1; / WP,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_ipi_int_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long delivery_mode:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub rsvd_12_15:4: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub rsvd_48_62:15: c_ulong,
    pub /: *mut *mut unsigned long send:1; / WP,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_ipi_int_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long delivery_mode:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub rsvd_12_15:4: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub rsvd_48_62:15: c_ulong,
    pub /: *mut *mut unsigned long send:1; / WP,
    pub s2: },
}

// =========================================================================
// UVH_NODE_ID
// =========================================================================
pub const UVH_NODE_ID: c_uint = 0x0UL;
// UVH common defines
pub const UVH_NODE_ID_FORCE1_SHFT: c_int = 0;
pub const UVH_NODE_ID_FORCE1_MASK: c_uint = 0x0000000000000001UL;
pub const UVH_NODE_ID_MANUFACTURER_SHFT: c_int = 1;
pub const UVH_NODE_ID_MANUFACTURER_MASK: c_uint = 0x0000000000000ffeUL;
pub const UVH_NODE_ID_PART_NUMBER_SHFT: c_int = 12;
pub const UVH_NODE_ID_PART_NUMBER_MASK: c_uint = 0x000000000ffff000UL;
pub const UVH_NODE_ID_REVISION_SHFT: c_int = 28;
pub const UVH_NODE_ID_REVISION_MASK: c_uint = 0x00000000f0000000UL;
pub const UVH_NODE_ID_NODE_ID_SHFT: c_int = 32;
pub const UVH_NODE_ID_NI_PORT_SHFT: c_int = 57;
// UVXH common defines
pub const UVXH_NODE_ID_NODE_ID_MASK: c_uint = 0x00007fff00000000UL;
pub const UVXH_NODE_ID_NODES_PER_BIT_SHFT: c_int = 50;
pub const UVXH_NODE_ID_NODES_PER_BIT_MASK: c_uint = 0x01fc000000000000UL;
pub const UVXH_NODE_ID_NI_PORT_MASK: c_uint = 0x3e00000000000000UL;
// UVYH common defines
pub const UVYH_NODE_ID_NODE_ID_MASK: c_uint = 0x0000007f00000000UL;
pub const UVYH_NODE_ID_NI_PORT_MASK: c_uint = 0x7e00000000000000UL;
// UV4 unique defines
pub const UV4H_NODE_ID_ROUTER_SELECT_SHFT: c_int = 48;
pub const UV4H_NODE_ID_ROUTER_SELECT_MASK: c_uint = 0x0001000000000000UL;
pub const UV4H_NODE_ID_RESERVED_2_SHFT: c_int = 49;
pub const UV4H_NODE_ID_RESERVED_2_MASK: c_uint = 0x0002000000000000UL;
// UV3 unique defines
pub const UV3H_NODE_ID_ROUTER_SELECT_SHFT: c_int = 48;
pub const UV3H_NODE_ID_ROUTER_SELECT_MASK: c_uint = 0x0001000000000000UL;
pub const UV3H_NODE_ID_RESERVED_2_SHFT: c_int = 49;
pub const UV3H_NODE_ID_RESERVED_2_MASK: c_uint = 0x0002000000000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_node_id_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_node_id_s {
    pub /: *mut *mut unsigned long force1:1; / RO,
    pub /: *mut *mut unsigned long manufacturer:11; / RO,
    pub /: *mut *mut unsigned long part_number:16; / RO,
    pub /: *mut *mut unsigned long revision:4; / RO,
    pub rsvd_32_63:32: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_node_id_s {
    pub /: *mut *mut unsigned long force1:1; / RO,
    pub /: *mut *mut unsigned long manufacturer:11; / RO,
    pub /: *mut *mut unsigned long part_number:16; / RO,
    pub /: *mut *mut unsigned long revision:4; / RO,
    pub /: *mut *mut unsigned long node_id:15; / RW,
    pub rsvd_47_49:3: c_ulong,
    pub /: *mut *mut unsigned long nodes_per_bit:7; / RO,
    pub /: *mut *mut unsigned long ni_port:5; / RO,
    pub rsvd_62_63:2: c_ulong,
    pub sx: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_node_id_s {
    pub /: *mut *mut unsigned long force1:1; / RO,
    pub /: *mut *mut unsigned long manufacturer:11; / RO,
    pub /: *mut *mut unsigned long part_number:16; / RO,
    pub /: *mut *mut unsigned long revision:4; / RO,
    pub /: *mut *mut unsigned long node_id:7; / RW,
    pub rsvd_39_56:18: c_ulong,
    pub /: *mut *mut unsigned long ni_port:6; / RO,
    pub rsvd_63:1: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_node_id_s {
    pub /: *mut *mut unsigned long force1:1; / RO,
    pub /: *mut *mut unsigned long manufacturer:11; / RO,
    pub /: *mut *mut unsigned long part_number:16; / RO,
    pub /: *mut *mut unsigned long revision:4; / RO,
    pub /: *mut *mut unsigned long node_id:7; / RW,
    pub rsvd_39_56:18: c_ulong,
    pub /: *mut *mut unsigned long ni_port:6; / RO,
    pub rsvd_63:1: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_node_id_s {
    pub /: *mut *mut unsigned long force1:1; / RO,
    pub /: *mut *mut unsigned long manufacturer:11; / RO,
    pub /: *mut *mut unsigned long part_number:16; / RO,
    pub /: *mut *mut unsigned long revision:4; / RO,
    pub /: *mut *mut unsigned long node_id:15; / RW,
    pub rsvd_47:1: c_ulong,
    pub /: *mut *mut unsigned long router_select:1; / RO,
    pub rsvd_49:1: c_ulong,
    pub /: *mut *mut unsigned long nodes_per_bit:7; / RO,
    pub /: *mut *mut unsigned long ni_port:5; / RO,
    pub rsvd_62_63:2: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_node_id_s {
    pub /: *mut *mut unsigned long force1:1; / RO,
    pub /: *mut *mut unsigned long manufacturer:11; / RO,
    pub /: *mut *mut unsigned long part_number:16; / RO,
    pub /: *mut *mut unsigned long revision:4; / RO,
    pub /: *mut *mut unsigned long node_id:15; / RW,
    pub rsvd_47:1: c_ulong,
    pub /: *mut *mut unsigned long router_select:1; / RO,
    pub rsvd_49:1: c_ulong,
    pub /: *mut *mut unsigned long nodes_per_bit:7; / RO,
    pub /: *mut *mut unsigned long ni_port:5; / RO,
    pub rsvd_62_63:2: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_node_id_s {
    pub /: *mut *mut unsigned long force1:1; / RO,
    pub /: *mut *mut unsigned long manufacturer:11; / RO,
    pub /: *mut *mut unsigned long part_number:16; / RO,
    pub /: *mut *mut unsigned long revision:4; / RO,
    pub /: *mut *mut unsigned long node_id:15; / RW,
    pub rsvd_47_49:3: c_ulong,
    pub /: *mut *mut unsigned long nodes_per_bit:7; / RO,
    pub /: *mut *mut unsigned long ni_port:5; / RO,
    pub rsvd_62_63:2: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_NODE_PRESENT_0
// =========================================================================

// UVYH common defines
pub const UVYH_NODE_PRESENT_0_NODES_SHFT: c_int = 0;
pub const UVYH_NODE_PRESENT_0_NODES_MASK: c_uint = 0xffffffffffffffffUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_node_present_0_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_node_present_0_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_node_present_0_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_node_present_0_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s5: },
}

// =========================================================================
// UVH_NODE_PRESENT_1
// =========================================================================

// UVYH common defines
pub const UVYH_NODE_PRESENT_1_NODES_SHFT: c_int = 0;
pub const UVYH_NODE_PRESENT_1_NODES_MASK: c_uint = 0xffffffffffffffffUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_node_present_1_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_node_present_1_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_node_present_1_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_node_present_1_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s5: },
}

// =========================================================================
// UVH_NODE_PRESENT_TABLE
// =========================================================================

// UVXH common defines
pub const UVXH_NODE_PRESENT_TABLE_NODES_SHFT: c_int = 0;
pub const UVXH_NODE_PRESENT_TABLE_NODES_MASK: c_uint = 0xffffffffffffffffUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_node_present_table_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_node_present_table_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_node_present_table_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_node_present_table_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_node_present_table_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_node_present_table_s {
    pub /: *mut *mut unsigned long nodes:64; / RW,
    pub s2: },
}

// =========================================================================
// UVH_RH10_GAM_ADDR_MAP_CONFIG
// =========================================================================

// UVYH common defines
pub const UVYH_RH10_GAM_ADDR_MAP_CONFIG_N_SKT_SHFT: c_int = 6;
pub const UVYH_RH10_GAM_ADDR_MAP_CONFIG_N_SKT_MASK: c_uint = 0x00000000000001c0UL;
pub const UVYH_RH10_GAM_ADDR_MAP_CONFIG_LS_ENABLE_SHFT: c_int = 12;
pub const UVYH_RH10_GAM_ADDR_MAP_CONFIG_LS_ENABLE_MASK: c_uint = 0x0000000000001000UL;
pub const UVYH_RH10_GAM_ADDR_MAP_CONFIG_MK_TME_KEYID_BITS_SHFT: c_int = 16;
pub const UVYH_RH10_GAM_ADDR_MAP_CONFIG_MK_TME_KEYID_BITS_MASK: c_uint = 0x00000000000f0000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh10_gam_addr_map_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh10_gam_addr_map_config_s {
    pub /: *mut *mut unsigned long undef_0_5:6; / Undefined,
    pub /: *mut *mut unsigned long n_skt:3; / RW,
    pub /: *mut *mut unsigned long undef_9_11:3; / Undefined,
    pub /: *mut *mut unsigned long ls_enable:1; / RW,
    pub /: *mut *mut unsigned long undef_13_15:3; / Undefined,
    pub /: *mut *mut unsigned long mk_tme_keyid_bits:4; / RW,
    pub rsvd_20_63:44: c_ulong,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_rh10_gam_addr_map_config_s {
    pub /: *mut *mut unsigned long undef_0_5:6; / Undefined,
    pub /: *mut *mut unsigned long n_skt:3; / RW,
    pub /: *mut *mut unsigned long undef_9_11:3; / Undefined,
    pub /: *mut *mut unsigned long ls_enable:1; / RW,
    pub /: *mut *mut unsigned long undef_13_15:3; / Undefined,
    pub /: *mut *mut unsigned long mk_tme_keyid_bits:4; / RW,
    pub rsvd_20_63:44: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rh10_gam_addr_map_config_s {
    pub /: *mut *mut unsigned long undef_0_5:6; / Undefined,
    pub /: *mut *mut unsigned long n_skt:3; / RW,
    pub /: *mut *mut unsigned long undef_9_11:3; / Undefined,
    pub /: *mut *mut unsigned long ls_enable:1; / RW,
    pub /: *mut *mut unsigned long undef_13_15:3; / Undefined,
    pub /: *mut *mut unsigned long mk_tme_keyid_bits:4; / RW,
    pub s5: },
}

// =========================================================================
// UVH_RH10_GAM_GRU_OVERLAY_CONFIG
// =========================================================================

// UVYH common defines
pub const UVYH_RH10_GAM_GRU_OVERLAY_CONFIG_BASE_SHFT: c_int = 25;
pub const UVYH_RH10_GAM_GRU_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x000ffffffe000000UL;
pub const UVYH_RH10_GAM_GRU_OVERLAY_CONFIG_N_GRU_SHFT: c_int = 52;
pub const UVYH_RH10_GAM_GRU_OVERLAY_CONFIG_N_GRU_MASK: c_uint = 0x0070000000000000UL;
pub const UVYH_RH10_GAM_GRU_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UVYH_RH10_GAM_GRU_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh10_gam_gru_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh10_gam_gru_overlay_config_s {
    pub /: *mut *mut unsigned long undef_0_24:25; / Undefined,
    pub /: *mut *mut unsigned long base:27; / RW,
    pub /: *mut *mut unsigned long n_gru:3; / RW,
    pub /: *mut *mut unsigned long undef_55_62:8; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_rh10_gam_gru_overlay_config_s {
    pub /: *mut *mut unsigned long undef_0_24:25; / Undefined,
    pub /: *mut *mut unsigned long base:27; / RW,
    pub /: *mut *mut unsigned long n_gru:3; / RW,
    pub /: *mut *mut unsigned long undef_55_62:8; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rh10_gam_gru_overlay_config_s {
    pub /: *mut *mut unsigned long undef_0_24:25; / Undefined,
    pub /: *mut *mut unsigned long base:27; / RW,
    pub /: *mut *mut unsigned long n_gru:3; / RW,
    pub /: *mut *mut unsigned long undef_55_62:8; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s5: },
}

// =========================================================================
// UVH_RH10_GAM_MMIOH_OVERLAY_CONFIG0
// =========================================================================

// UVYH common defines
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG0_BASE_SHFT: c_int = 26;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG0_BASE_MASK: c_uint = 0x000ffffffc000000UL;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_SHFT: c_int = 52;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_MASK: c_uint = 0x03f0000000000000UL;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_SHFT: c_int = 63;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_MASK: c_uint = 0x8000000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh10_gam_mmioh_overlay_config0_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh10_gam_mmioh_overlay_config0_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_rh10_gam_mmioh_overlay_config0_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rh10_gam_mmioh_overlay_config0_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s5: },
}

// =========================================================================
// UVH_RH10_GAM_MMIOH_OVERLAY_CONFIG1
// =========================================================================

// UVYH common defines
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG1_BASE_SHFT: c_int = 26;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG1_BASE_MASK: c_uint = 0x000ffffffc000000UL;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_SHFT: c_int = 52;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_MASK: c_uint = 0x03f0000000000000UL;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_SHFT: c_int = 63;
pub const UVYH_RH10_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_MASK: c_uint = 0x8000000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh10_gam_mmioh_overlay_config1_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh10_gam_mmioh_overlay_config1_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_rh10_gam_mmioh_overlay_config1_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rh10_gam_mmioh_overlay_config1_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s5: },
}

// =========================================================================
// UVH_RH10_GAM_MMIOH_REDIRECT_CONFIG0
// =========================================================================

// UVYH common defines
pub const UVYH_RH10_GAM_MMIOH_REDIRECT_CONFIG0_NASID_SHFT: c_int = 0;
pub const UVYH_RH10_GAM_MMIOH_REDIRECT_CONFIG0_NASID_MASK: c_uint = 0x000000000000007fUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh10_gam_mmioh_redirect_config0_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh10_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:7; / RW,
    pub rsvd_7_63:57: c_ulong,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_rh10_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:7; / RW,
    pub rsvd_7_63:57: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rh10_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:7; / RW,
    pub rsvd_7_63:57: c_ulong,
    pub s5: },
}

// =========================================================================
// UVH_RH10_GAM_MMIOH_REDIRECT_CONFIG1
// =========================================================================

// UVYH common defines
pub const UVYH_RH10_GAM_MMIOH_REDIRECT_CONFIG1_NASID_SHFT: c_int = 0;
pub const UVYH_RH10_GAM_MMIOH_REDIRECT_CONFIG1_NASID_MASK: c_uint = 0x000000000000007fUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh10_gam_mmioh_redirect_config1_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh10_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:7; / RW,
    pub rsvd_7_63:57: c_ulong,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_rh10_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:7; / RW,
    pub rsvd_7_63:57: c_ulong,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rh10_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:7; / RW,
    pub rsvd_7_63:57: c_ulong,
    pub s5: },
}

// =========================================================================
// UVH_RH10_GAM_MMR_OVERLAY_CONFIG
// =========================================================================

// UVYH common defines
pub const UVYH_RH10_GAM_MMR_OVERLAY_CONFIG_BASE_SHFT: c_int = 25;
pub const UVYH_RH10_GAM_MMR_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x000ffffffe000000UL;
pub const UVYH_RH10_GAM_MMR_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UVYH_RH10_GAM_MMR_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh10_gam_mmr_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh10_gam_mmr_overlay_config_s {
    pub /: *mut *mut unsigned long undef_0_24:25; / Undefined,
    pub /: *mut *mut unsigned long base:27; / RW,
    pub /: *mut *mut unsigned long undef_52_62:11; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_rh10_gam_mmr_overlay_config_s {
    pub /: *mut *mut unsigned long undef_0_24:25; / Undefined,
    pub /: *mut *mut unsigned long base:27; / RW,
    pub /: *mut *mut unsigned long undef_52_62:11; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rh10_gam_mmr_overlay_config_s {
    pub /: *mut *mut unsigned long undef_0_24:25; / Undefined,
    pub /: *mut *mut unsigned long base:27; / RW,
    pub /: *mut *mut unsigned long undef_52_62:11; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s5: },
}

// =========================================================================
// UVH_RH_GAM_ADDR_MAP_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_ADDR_MAP_CONFIG_N_SKT_SHFT: c_int = 6;
pub const UVXH_RH_GAM_ADDR_MAP_CONFIG_N_SKT_MASK: c_uint = 0x00000000000003c0UL;
// UV3 unique defines
pub const UV3H_RH_GAM_ADDR_MAP_CONFIG_M_SKT_SHFT: c_int = 0;
pub const UV3H_RH_GAM_ADDR_MAP_CONFIG_M_SKT_MASK: c_uint = 0x000000000000003fUL;
// UV2 unique defines
pub const UV2H_RH_GAM_ADDR_MAP_CONFIG_M_SKT_SHFT: c_int = 0;
pub const UV2H_RH_GAM_ADDR_MAP_CONFIG_M_SKT_MASK: c_uint = 0x000000000000003fUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_addr_map_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_addr_map_config_s {
    pub rsvd_0_5:6: c_ulong,
    pub /: *mut *mut unsigned long n_skt:4; / RW,
    pub rsvd_10_63:54: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_addr_map_config_s {
    pub rsvd_0_5:6: c_ulong,
    pub /: *mut *mut unsigned long n_skt:4; / RW,
    pub rsvd_10_63:54: c_ulong,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_addr_map_config_s {
    pub rsvd_0_5:6: c_ulong,
    pub /: *mut *mut unsigned long n_skt:4; / RW,
    pub rsvd_10_63:54: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_addr_map_config_s {
    pub /: *mut *mut unsigned long m_skt:6; / RW,
    pub /: *mut *mut unsigned long n_skt:4; / RW,
    pub rsvd_10_63:54: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_addr_map_config_s {
    pub /: *mut *mut unsigned long m_skt:6; / RW,
    pub /: *mut *mut unsigned long n_skt:4; / RW,
    pub rsvd_10_63:54: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_ALIAS_0_OVERLAY_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_ALIAS_0_OVERLAY_CONFIG_BASE_SHFT: c_int = 24;
pub const UVXH_RH_GAM_ALIAS_0_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x00000000ff000000UL;
pub const UVXH_RH_GAM_ALIAS_0_OVERLAY_CONFIG_M_ALIAS_SHFT: c_int = 48;
pub const UVXH_RH_GAM_ALIAS_0_OVERLAY_CONFIG_M_ALIAS_MASK: c_uint = 0x001f000000000000UL;
pub const UVXH_RH_GAM_ALIAS_0_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UVXH_RH_GAM_ALIAS_0_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_alias_0_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_alias_0_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_alias_0_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_alias_0_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_alias_0_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_alias_0_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_ALIAS_0_REDIRECT_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_ALIAS_0_REDIRECT_CONFIG_DEST_BASE_SHFT: c_int = 24;
pub const UVXH_RH_GAM_ALIAS_0_REDIRECT_CONFIG_DEST_BASE_MASK: c_uint = 0x00003fffff000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_alias_0_redirect_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_alias_0_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_alias_0_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_alias_0_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_alias_0_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_alias_0_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_ALIAS_1_OVERLAY_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_ALIAS_1_OVERLAY_CONFIG_BASE_SHFT: c_int = 24;
pub const UVXH_RH_GAM_ALIAS_1_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x00000000ff000000UL;
pub const UVXH_RH_GAM_ALIAS_1_OVERLAY_CONFIG_M_ALIAS_SHFT: c_int = 48;
pub const UVXH_RH_GAM_ALIAS_1_OVERLAY_CONFIG_M_ALIAS_MASK: c_uint = 0x001f000000000000UL;
pub const UVXH_RH_GAM_ALIAS_1_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UVXH_RH_GAM_ALIAS_1_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_alias_1_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_alias_1_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_alias_1_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_alias_1_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_alias_1_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_alias_1_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_ALIAS_1_REDIRECT_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_ALIAS_1_REDIRECT_CONFIG_DEST_BASE_SHFT: c_int = 24;
pub const UVXH_RH_GAM_ALIAS_1_REDIRECT_CONFIG_DEST_BASE_MASK: c_uint = 0x00003fffff000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_alias_1_redirect_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_alias_1_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_alias_1_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_alias_1_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_alias_1_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_alias_1_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_ALIAS_2_OVERLAY_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_ALIAS_2_OVERLAY_CONFIG_BASE_SHFT: c_int = 24;
pub const UVXH_RH_GAM_ALIAS_2_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x00000000ff000000UL;
pub const UVXH_RH_GAM_ALIAS_2_OVERLAY_CONFIG_M_ALIAS_SHFT: c_int = 48;
pub const UVXH_RH_GAM_ALIAS_2_OVERLAY_CONFIG_M_ALIAS_MASK: c_uint = 0x001f000000000000UL;
pub const UVXH_RH_GAM_ALIAS_2_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UVXH_RH_GAM_ALIAS_2_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_alias_2_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_alias_2_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_alias_2_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_alias_2_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_alias_2_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_alias_2_overlay_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long base:8; / RW,
    pub rsvd_32_47:16: c_ulong,
    pub /: *mut *mut unsigned long m_alias:5; / RW,
    pub rsvd_53_62:10: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_ALIAS_2_REDIRECT_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_ALIAS_2_REDIRECT_CONFIG_DEST_BASE_SHFT: c_int = 24;
pub const UVXH_RH_GAM_ALIAS_2_REDIRECT_CONFIG_DEST_BASE_MASK: c_uint = 0x00003fffff000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_alias_2_redirect_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_alias_2_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_alias_2_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_alias_2_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_alias_2_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_alias_2_redirect_config_s {
    pub rsvd_0_23:24: c_ulong,
    pub /: *mut *mut unsigned long dest_base:22; / RW,
    pub rsvd_46_63:18: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_GRU_OVERLAY_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_GRU_OVERLAY_CONFIG_N_GRU_SHFT: c_int = 52;
pub const UVXH_RH_GAM_GRU_OVERLAY_CONFIG_N_GRU_MASK: c_uint = 0x00f0000000000000UL;
pub const UVXH_RH_GAM_GRU_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UVXH_RH_GAM_GRU_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;
// UV4A unique defines
pub const UV4AH_RH_GAM_GRU_OVERLAY_CONFIG_BASE_SHFT: c_int = 26;
pub const UV4AH_RH_GAM_GRU_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x000ffffffc000000UL;
// UV4 unique defines
pub const UV4H_RH_GAM_GRU_OVERLAY_CONFIG_BASE_SHFT: c_int = 26;
pub const UV4H_RH_GAM_GRU_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x00003ffffc000000UL;
// UV3 unique defines
pub const UV3H_RH_GAM_GRU_OVERLAY_CONFIG_BASE_SHFT: c_int = 28;
pub const UV3H_RH_GAM_GRU_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x00003ffff0000000UL;
pub const UV3H_RH_GAM_GRU_OVERLAY_CONFIG_MODE_SHFT: c_int = 62;
pub const UV3H_RH_GAM_GRU_OVERLAY_CONFIG_MODE_MASK: c_uint = 0x4000000000000000UL;
// UV2 unique defines
pub const UV2H_RH_GAM_GRU_OVERLAY_CONFIG_BASE_SHFT: c_int = 28;
pub const UV2H_RH_GAM_GRU_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x00003ffff0000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_gru_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_gru_overlay_config_s {
    pub rsvd_0_45:46: c_ulong,
    pub rsvd_46_51:6: c_ulong,
    pub /: *mut *mut unsigned long n_gru:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_gru_overlay_config_s {
    pub rsvd_0_45:46: c_ulong,
    pub rsvd_46_51:6: c_ulong,
    pub /: *mut *mut unsigned long n_gru:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV4A unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4ah_rh_gam_gru_overlay_config_s {
    pub rsvd_0_24:25: c_ulong,
    pub /: *mut *mut unsigned long undef_25:1; / Undefined,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long n_gru:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4a: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_gru_overlay_config_s {
    pub rsvd_0_24:25: c_ulong,
    pub /: *mut *mut unsigned long undef_25:1; / Undefined,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub rsvd_46_51:6: c_ulong,
    pub /: *mut *mut unsigned long n_gru:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_gru_overlay_config_s {
    pub rsvd_0_27:28: c_ulong,
    pub /: *mut *mut unsigned long base:18; / RW,
    pub rsvd_46_51:6: c_ulong,
    pub /: *mut *mut unsigned long n_gru:4; / RW,
    pub rsvd_56_61:6: c_ulong,
    pub /: *mut *mut unsigned long mode:1; / RW,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_gru_overlay_config_s {
    pub rsvd_0_27:28: c_ulong,
    pub /: *mut *mut unsigned long base:18; / RW,
    pub rsvd_46_51:6: c_ulong,
    pub /: *mut *mut unsigned long n_gru:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_MMIOH_OVERLAY_CONFIG
// =========================================================================

// UV2 unique defines
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_BASE_SHFT: c_int = 27;
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x00003ffff8000000UL;
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_M_IO_SHFT: c_int = 46;
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_M_IO_MASK: c_uint = 0x000fc00000000000UL;
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_N_IO_SHFT: c_int = 52;
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_N_IO_MASK: c_uint = 0x00f0000000000000UL;
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UV2H_RH_GAM_MMIOH_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_mmioh_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_mmioh_overlay_config_s {
    pub rsvd_0_26:27: c_ulong,
    pub /: *mut *mut unsigned long base:19; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub /: *mut *mut unsigned long n_io:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_mmioh_overlay_config_s {
    pub rsvd_0_26:27: c_ulong,
    pub /: *mut *mut unsigned long base:19; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub /: *mut *mut unsigned long n_io:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_mmioh_overlay_config_s {
    pub rsvd_0_26:27: c_ulong,
    pub /: *mut *mut unsigned long base:19; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub /: *mut *mut unsigned long n_io:4; / RW,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s2: },
}

// =========================================================================
// UVH_RH_GAM_MMIOH_OVERLAY_CONFIG0
// =========================================================================

// UV4A unique defines
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG0_BASE_SHFT: c_int = 26;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG0_BASE_MASK: c_uint = 0x000ffffffc000000UL;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_SHFT: c_int = 52;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_MASK: c_uint = 0x03f0000000000000UL;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_SHFT: c_int = 63;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_MASK: c_uint = 0x8000000000000000UL;
// UV4 unique defines
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG0_BASE_SHFT: c_int = 26;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG0_BASE_MASK: c_uint = 0x00003ffffc000000UL;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_SHFT: c_int = 46;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_MASK: c_uint = 0x000fc00000000000UL;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_SHFT: c_int = 63;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_MASK: c_uint = 0x8000000000000000UL;
// UV3 unique defines
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG0_BASE_SHFT: c_int = 26;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG0_BASE_MASK: c_uint = 0x00003ffffc000000UL;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_SHFT: c_int = 46;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG0_M_IO_MASK: c_uint = 0x000fc00000000000UL;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_SHFT: c_int = 63;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG0_ENABLE_MASK: c_uint = 0x8000000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_mmioh_overlay_config0_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_mmioh_overlay_config0_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_mmioh_overlay_config0_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV4A unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4ah_rh_gam_mmioh_overlay_config0_mmr_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4a: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_mmioh_overlay_config0_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_mmioh_overlay_config0_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s3: },
}

// =========================================================================
// UVH_RH_GAM_MMIOH_OVERLAY_CONFIG1
// =========================================================================

// UV4A unique defines
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG1_BASE_SHFT: c_int = 26;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG1_BASE_MASK: c_uint = 0x000ffffffc000000UL;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_SHFT: c_int = 52;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_MASK: c_uint = 0x03f0000000000000UL;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_SHFT: c_int = 63;
pub const UV4AH_RH_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_MASK: c_uint = 0x8000000000000000UL;
// UV4 unique defines
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG1_BASE_SHFT: c_int = 26;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG1_BASE_MASK: c_uint = 0x00003ffffc000000UL;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_SHFT: c_int = 46;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_MASK: c_uint = 0x000fc00000000000UL;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_SHFT: c_int = 63;
pub const UV4H_RH_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_MASK: c_uint = 0x8000000000000000UL;
// UV3 unique defines
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG1_BASE_SHFT: c_int = 26;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG1_BASE_MASK: c_uint = 0x00003ffffc000000UL;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_SHFT: c_int = 46;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG1_M_IO_MASK: c_uint = 0x000fc00000000000UL;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_SHFT: c_int = 63;
pub const UV3H_RH_GAM_MMIOH_OVERLAY_CONFIG1_ENABLE_MASK: c_uint = 0x8000000000000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_mmioh_overlay_config1_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_mmioh_overlay_config1_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_mmioh_overlay_config1_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV4A unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4ah_rh_gam_mmioh_overlay_config1_mmr_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:26; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub /: *mut *mut unsigned long undef_62:1; / Undefined,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4a: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_mmioh_overlay_config1_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_mmioh_overlay_config1_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub /: *mut *mut unsigned long m_io:6; / RW,
    pub n_io:4: c_ulong,
    pub rsvd_56_62:7: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s3: },
}

// =========================================================================
// UVH_RH_GAM_MMIOH_REDIRECT_CONFIG0
// =========================================================================

// UV4A unique defines
pub const UV4AH_RH_GAM_MMIOH_REDIRECT_CONFIG0_NASID_SHFT: c_int = 0;
pub const UV4AH_RH_GAM_MMIOH_REDIRECT_CONFIG0_NASID_MASK: c_uint = 0x0000000000000fffUL;
// UV4 unique defines
pub const UV4H_RH_GAM_MMIOH_REDIRECT_CONFIG0_NASID_SHFT: c_int = 0;
pub const UV4H_RH_GAM_MMIOH_REDIRECT_CONFIG0_NASID_MASK: c_uint = 0x0000000000007fffUL;
// UV3 unique defines
pub const UV3H_RH_GAM_MMIOH_REDIRECT_CONFIG0_NASID_SHFT: c_int = 0;
pub const UV3H_RH_GAM_MMIOH_REDIRECT_CONFIG0_NASID_MASK: c_uint = 0x0000000000007fffUL;
// UVH common defines

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_mmioh_redirect_config0_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub sx: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4ah_rh_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:12; / RW,
    pub rsvd_12_63:52: c_ulong,
    pub s4a: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_mmioh_redirect_config0_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub s3: },
}

// =========================================================================
// UVH_RH_GAM_MMIOH_REDIRECT_CONFIG1
// =========================================================================

// UV4A unique defines
pub const UV4AH_RH_GAM_MMIOH_REDIRECT_CONFIG1_NASID_SHFT: c_int = 0;
pub const UV4AH_RH_GAM_MMIOH_REDIRECT_CONFIG1_NASID_MASK: c_uint = 0x0000000000000fffUL;
// UV4 unique defines
pub const UV4H_RH_GAM_MMIOH_REDIRECT_CONFIG1_NASID_SHFT: c_int = 0;
pub const UV4H_RH_GAM_MMIOH_REDIRECT_CONFIG1_NASID_MASK: c_uint = 0x0000000000007fffUL;
// UV3 unique defines
pub const UV3H_RH_GAM_MMIOH_REDIRECT_CONFIG1_NASID_SHFT: c_int = 0;
pub const UV3H_RH_GAM_MMIOH_REDIRECT_CONFIG1_NASID_MASK: c_uint = 0x0000000000007fffUL;
// UVH common defines

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_mmioh_redirect_config1_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub sx: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4ah_rh_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:12; / RW,
    pub rsvd_12_63:52: c_ulong,
    pub s4a: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_mmioh_redirect_config1_s {
    pub /: *mut *mut unsigned long nasid:15; / RW,
    pub rsvd_15_63:49: c_ulong,
    pub s3: },
}

// =========================================================================
// UVH_RH_GAM_MMR_OVERLAY_CONFIG
// =========================================================================

// UVXH common defines
pub const UVXH_RH_GAM_MMR_OVERLAY_CONFIG_BASE_SHFT: c_int = 26;

pub const UVXH_RH_GAM_MMR_OVERLAY_CONFIG_ENABLE_SHFT: c_int = 63;
pub const UVXH_RH_GAM_MMR_OVERLAY_CONFIG_ENABLE_MASK: c_uint = 0x8000000000000000UL;
// UV4A unique defines
pub const UV4AH_RH_GAM_GRU_OVERLAY_CONFIG_BASE_SHFT: c_int = 26;
pub const UV4AH_RH_GAM_GRU_OVERLAY_CONFIG_BASE_MASK: c_uint = 0x000ffffffc000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rh_gam_mmr_overlay_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rh_gam_mmr_overlay_config_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub rsvd_46_62:17: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_rh_gam_mmr_overlay_config_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub rsvd_46_62:17: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub sx: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rh_gam_mmr_overlay_config_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub rsvd_46_62:17: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rh_gam_mmr_overlay_config_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub rsvd_46_62:17: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rh_gam_mmr_overlay_config_s {
    pub rsvd_0_25:26: c_ulong,
    pub /: *mut *mut unsigned long base:20; / RW,
    pub rsvd_46_62:17: c_ulong,
    pub /: *mut *mut unsigned long enable:1; / RW,
    pub s2: },
}

// =========================================================================
// UVH_RTC
// =========================================================================

// UVH common defines
pub const UVH_RTC_REAL_TIME_CLOCK_SHFT: c_int = 0;
pub const UVH_RTC_REAL_TIME_CLOCK_MASK: c_uint = 0x00ffffffffffffffUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rtc_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rtc_s {
    pub /: *mut *mut unsigned long real_time_clock:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rtc_s {
    pub /: *mut *mut unsigned long real_time_clock:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rtc_s {
    pub /: *mut *mut unsigned long real_time_clock:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rtc_s {
    pub /: *mut *mut unsigned long real_time_clock:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rtc_s {
    pub /: *mut *mut unsigned long real_time_clock:56; / RW,
    pub rsvd_56_63:8: c_ulong,
    pub s2: },
}

// =========================================================================
// UVH_RTC1_INT_CONFIG
// =========================================================================
pub const UVH_RTC1_INT_CONFIG: c_uint = 0x615c0UL;
// UVH common defines
pub const UVH_RTC1_INT_CONFIG_VECTOR_SHFT: c_int = 0;
pub const UVH_RTC1_INT_CONFIG_VECTOR_MASK: c_uint = 0x00000000000000ffUL;
pub const UVH_RTC1_INT_CONFIG_DM_SHFT: c_int = 8;
pub const UVH_RTC1_INT_CONFIG_DM_MASK: c_uint = 0x0000000000000700UL;
pub const UVH_RTC1_INT_CONFIG_DESTMODE_SHFT: c_int = 11;
pub const UVH_RTC1_INT_CONFIG_DESTMODE_MASK: c_uint = 0x0000000000000800UL;
pub const UVH_RTC1_INT_CONFIG_STATUS_SHFT: c_int = 12;
pub const UVH_RTC1_INT_CONFIG_STATUS_MASK: c_uint = 0x0000000000001000UL;
pub const UVH_RTC1_INT_CONFIG_P_SHFT: c_int = 13;
pub const UVH_RTC1_INT_CONFIG_P_MASK: c_uint = 0x0000000000002000UL;
pub const UVH_RTC1_INT_CONFIG_T_SHFT: c_int = 15;
pub const UVH_RTC1_INT_CONFIG_T_MASK: c_uint = 0x0000000000008000UL;
pub const UVH_RTC1_INT_CONFIG_M_SHFT: c_int = 16;
pub const UVH_RTC1_INT_CONFIG_M_MASK: c_uint = 0x0000000000010000UL;
pub const UVH_RTC1_INT_CONFIG_APIC_ID_SHFT: c_int = 32;
pub const UVH_RTC1_INT_CONFIG_APIC_ID_MASK: c_uint = 0xffffffff00000000UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_rtc1_int_config_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_rtc1_int_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_rtc1_int_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_rtc1_int_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_rtc1_int_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_rtc1_int_config_s {
    pub /: *mut *mut unsigned long vector_:8; / RW,
    pub /: *mut *mut unsigned long dm:3; / RW,
    pub /: *mut *mut unsigned long destmode:1; / RW,
    pub /: *mut *mut unsigned long status:1; / RO,
    pub /: *mut *mut unsigned long p:1; / RO,
    pub rsvd_14:1: c_ulong,
    pub /: *mut *mut unsigned long t:1; / RO,
    pub /: *mut *mut unsigned long m:1; / RW,
    pub rsvd_17_31:15: c_ulong,
    pub /: *mut *mut unsigned long apic_id:32; / RW,
    pub s2: },
}

// =========================================================================
// UVH_SCRATCH5
// =========================================================================

pub const UV5H_SCRATCH5: c_uint = 0xb0200UL;
pub const UV4H_SCRATCH5: c_uint = 0xb0200UL;
pub const UV3H_SCRATCH5: c_uint = 0x2d0200UL;
pub const UV2H_SCRATCH5: c_uint = 0x2d0200UL;
// UVH common defines
pub const UVH_SCRATCH5_SCRATCH5_SHFT: c_int = 0;
pub const UVH_SCRATCH5_SCRATCH5_MASK: c_uint = 0xffffffffffffffffUL;
// UVXH common defines
pub const UVXH_SCRATCH5_SCRATCH5_SHFT: c_int = 0;
pub const UVXH_SCRATCH5_SCRATCH5_MASK: c_uint = 0xffffffffffffffffUL;
// UVYH common defines
pub const UVYH_SCRATCH5_SCRATCH5_SHFT: c_int = 0;
pub const UVYH_SCRATCH5_SCRATCH5_MASK: c_uint = 0xffffffffffffffffUL;
// UV5 unique defines
pub const UV5H_SCRATCH5_SCRATCH5_SHFT: c_int = 0;
pub const UV5H_SCRATCH5_SCRATCH5_MASK: c_uint = 0xffffffffffffffffUL;
// UV4 unique defines
pub const UV4H_SCRATCH5_SCRATCH5_SHFT: c_int = 0;
pub const UV4H_SCRATCH5_SCRATCH5_MASK: c_uint = 0xffffffffffffffffUL;
// UV3 unique defines
pub const UV3H_SCRATCH5_SCRATCH5_SHFT: c_int = 0;
pub const UV3H_SCRATCH5_SCRATCH5_MASK: c_uint = 0xffffffffffffffffUL;
// UV2 unique defines
pub const UV2H_SCRATCH5_SCRATCH5_SHFT: c_int = 0;
pub const UV2H_SCRATCH5_SCRATCH5_MASK: c_uint = 0xffffffffffffffffUL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union uvh_scratch5_u {
    pub v: c_ulong,
// UVH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvh_scratch5_s {
    pub /: *mut *mut unsigned long scratch5:64; / RW,
    pub s: },
// UVXH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvxh_scratch5_s {
    pub /: *mut *mut unsigned long scratch5:64; / RW,
    pub sx: },
// UVYH common struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvyh_scratch5_s {
    pub /: *mut *mut unsigned long scratch5:64; / RW,
    pub sy: },
// UV5 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv5h_scratch5_s {
    pub /: *mut *mut unsigned long scratch5:64; / RW,
    pub s5: },
// UV4 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv4h_scratch5_s {
    pub /: *mut *mut unsigned long scratch5:64; / RW,
    pub s4: },
// UV3 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv3h_scratch5_s {
    pub /: *mut *mut unsigned long scratch5:64; / RW,
    pub s3: },
// UV2 unique struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv2h_scratch5_s {
    pub /: *mut *mut unsigned long scratch5:64; / RW,
    pub s2: },
}

// =========================================================================
// UVH_SCRATCH5_ALIAS
// =========================================================================

pub const UV5H_SCRATCH5_ALIAS: c_uint = 0xb0208UL;
pub const UV4H_SCRATCH5_ALIAS: c_uint = 0xb0208UL;
pub const UV3H_SCRATCH5_ALIAS: c_uint = 0x2d0208UL;
pub const UV2H_SCRATCH5_ALIAS: c_uint = 0x2d0208UL;
// =========================================================================
// UVH_SCRATCH5_ALIAS_2
// =========================================================================

pub const UV5H_SCRATCH5_ALIAS_2: c_uint = 0xb0210UL;
pub const UV4H_SCRATCH5_ALIAS_2: c_uint = 0xb0210UL;
pub const UV3H_SCRATCH5_ALIAS_2: c_uint = 0x2d0210UL;
pub const UV2H_SCRATCH5_ALIAS_2: c_uint = 0x2d0210UL;
