//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/arm-smccc.h
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
// Copyright (c) 2015, Linaro Limited
//

//
// This file provides common defines for ARM SMC Calling Convention as
// specified in
// https://developer.arm.com/docs/den0028/latest
//
// This code is up-to-date with version DEN 0028 C
//

pub const ARM_SMCCC_TYPE_SHIFT: c_int = 31;
pub const ARM_SMCCC_SMC_32: c_int = 0;
pub const ARM_SMCCC_SMC_64: c_int = 1;
pub const ARM_SMCCC_CALL_CONV_SHIFT: c_int = 30;
pub const ARM_SMCCC_OWNER_MASK: c_uint = 0x3F;
pub const ARM_SMCCC_OWNER_SHIFT: c_int = 24;
pub const ARM_SMCCC_FUNC_MASK: c_uint = 0xFFFF;

pub const ARM_SMCCC_OWNER_ARCH: c_int = 0;
pub const ARM_SMCCC_OWNER_CPU: c_int = 1;
pub const ARM_SMCCC_OWNER_SIP: c_int = 2;
pub const ARM_SMCCC_OWNER_OEM: c_int = 3;
pub const ARM_SMCCC_OWNER_STANDARD: c_int = 4;
pub const ARM_SMCCC_OWNER_STANDARD_HYP: c_int = 5;
pub const ARM_SMCCC_OWNER_VENDOR_HYP: c_int = 6;
pub const ARM_SMCCC_OWNER_TRUSTED_APP: c_int = 48;
pub const ARM_SMCCC_OWNER_TRUSTED_APP_END: c_int = 49;
pub const ARM_SMCCC_OWNER_TRUSTED_OS: c_int = 50;
pub const ARM_SMCCC_OWNER_TRUSTED_OS_END: c_int = 63;
pub const ARM_SMCCC_FUNC_QUERY_CALL_UID: c_uint = 0xff01;
pub const ARM_SMCCC_QUIRK_NONE: c_int = 0;

pub const ARM_SMCCC_VERSION_1_0: c_uint = 0x10000;
pub const ARM_SMCCC_VERSION_1_1: c_uint = 0x10001;
pub const ARM_SMCCC_VERSION_1_2: c_uint = 0x10002;
pub const ARM_SMCCC_VERSION_1_3: c_uint = 0x10003;
pub const ARM_SMCCC_1_3_SVE_HINT: c_uint = 0x10000;

// C1-Pro erratum 4193714: SME DVMSync early acknowledgement

// KVM UID value: 28b46fb6-2ec5-11e9-a9ca-4b564d003a74

// KVM "vendor specific" services
pub const ARM_SMCCC_KVM_FUNC_FEATURES: c_int = 0;
pub const ARM_SMCCC_KVM_FUNC_PTP: c_int = 1;
// Start of pKVM hypercall range
pub const ARM_SMCCC_KVM_FUNC_HYP_MEMINFO: c_int = 2;
pub const ARM_SMCCC_KVM_FUNC_MEM_SHARE: c_int = 3;
pub const ARM_SMCCC_KVM_FUNC_MEM_UNSHARE: c_int = 4;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_5: c_int = 5;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_6: c_int = 6;
pub const ARM_SMCCC_KVM_FUNC_MMIO_GUARD: c_int = 7;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_8: c_int = 8;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_9: c_int = 9;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_10: c_int = 10;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_11: c_int = 11;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_12: c_int = 12;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_13: c_int = 13;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_14: c_int = 14;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_15: c_int = 15;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_16: c_int = 16;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_17: c_int = 17;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_18: c_int = 18;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_19: c_int = 19;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_20: c_int = 20;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_21: c_int = 21;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_22: c_int = 22;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_23: c_int = 23;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_24: c_int = 24;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_25: c_int = 25;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_26: c_int = 26;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_27: c_int = 27;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_28: c_int = 28;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_29: c_int = 29;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_30: c_int = 30;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_31: c_int = 31;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_32: c_int = 32;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_33: c_int = 33;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_34: c_int = 34;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_35: c_int = 35;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_36: c_int = 36;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_37: c_int = 37;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_38: c_int = 38;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_39: c_int = 39;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_40: c_int = 40;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_41: c_int = 41;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_42: c_int = 42;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_43: c_int = 43;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_44: c_int = 44;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_45: c_int = 45;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_46: c_int = 46;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_47: c_int = 47;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_48: c_int = 48;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_49: c_int = 49;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_50: c_int = 50;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_51: c_int = 51;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_52: c_int = 52;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_53: c_int = 53;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_54: c_int = 54;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_55: c_int = 55;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_56: c_int = 56;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_57: c_int = 57;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_58: c_int = 58;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_59: c_int = 59;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_60: c_int = 60;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_61: c_int = 61;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_62: c_int = 62;
pub const ARM_SMCCC_KVM_FUNC_PKVM_RESV_63: c_int = 63;
// End of pKVM hypercall range
pub const ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_VER: c_int = 64;
pub const ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_CPUS: c_int = 65;
pub const ARM_SMCCC_KVM_FUNC_FEATURES_2: c_int = 127;
pub const ARM_SMCCC_KVM_NUM_FUNCS: c_int = 128;

pub const SMCCC_ARCH_WORKAROUND_RET_UNAFFECTED: c_int = 1;
//
// ptp_kvm is a feature used for time sync between vm and host.
// ptp_kvm module in guest kernel will get service from host using
// this hypercall ID.
//

// ptp_kvm counter type ID
pub const KVM_PTP_VIRT_COUNTER: c_int = 0;
pub const KVM_PTP_PHYS_COUNTER: c_int = 1;
// Paravirtualised time calls (defined by ARM DEN0057A)

// TRNG entropy source calls (defined by ARM DEN0098)

//
// Return codes defined in ARM DEN 0070A
// ARM DEN 0070A is now merged/consolidated into ARM DEN 0028 C
//
pub const SMCCC_RET_SUCCESS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_smccc_conduit {
    SMCCC_CONDUIT_NONE,
    SMCCC_CONDUIT_SMC,
    SMCCC_CONDUIT_HVC,
}

//
// arm_smccc_1_1_get_conduit()
//
// Returns the conduit to be used for SMCCCv1.1 or later.
//
// When SMCCCv1.1 is not present, returns SMCCC_CONDUIT_NONE.
//
extern "C" {
    pub fn arm_smccc_1_1_get_conduit() -> arm_smccc_conduit;
}
//
// arm_smccc_get_version()
//
// Returns the version to be used for SMCCCv1.1 or later.
//
// When SMCCCv1.1 or above is not present, returns SMCCCv1.0, but this
// does not imply the presence of firmware or a valid conduit. Caller
// handling SMCCCv1.0 must determine the conduit by other means.
//
extern "C" {
    pub fn arm_smccc_get_version() -> u32;
}
extern "C" {
    pub fn arm_smccc_version_init(version: u32, conduit: arm_smccc_conduit) -> void __init;
}
//
// arm_smccc_get_soc_id_version()
//
// Returns the SOC ID version.
//
// When ARM_SMCCC_ARCH_SOC_ID is not present, returns SMCCC_RET_NOT_SUPPORTED.
//
extern "C" {
    pub fn arm_smccc_get_soc_id_version() -> i32;
}
//
// arm_smccc_get_soc_id_revision()
//
// Returns the SOC ID revision.
//
// When ARM_SMCCC_ARCH_SOC_ID is not present, returns SMCCC_RET_NOT_SUPPORTED.
//
extern "C" {
    pub fn arm_smccc_get_soc_id_revision() -> i32;
}
//
// Returns whether a specific hypervisor UUID is advertised for the
// Vendor Specific Hypervisor Service range.
//
extern "C" {
    pub fn arm_smccc_hypervisor_has_uuid(uuid: *const uuid_t) -> bool;
}

//
// struct arm_smccc_res - Result from SMC/HVC call
// @a0-a3 result values from registers 0 to 3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smccc_res {
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
}

//
// struct arm_smccc_1_2_regs - Arguments for or Results from SMC/HVC call
// @a0-a17 argument values from registers 0 to 17
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smccc_1_2_regs {
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
    pub a4: c_ulong,
    pub a5: c_ulong,
    pub a6: c_ulong,
    pub a7: c_ulong,
    pub a8: c_ulong,
    pub a9: c_ulong,
    pub a10: c_ulong,
    pub a11: c_ulong,
    pub a12: c_ulong,
    pub a13: c_ulong,
    pub a14: c_ulong,
    pub a15: c_ulong,
    pub a16: c_ulong,
    pub a17: c_ulong,
}

//
// arm_smccc_1_2_hvc() - make HVC calls
// @args: arguments passed via struct arm_smccc_1_2_regs
// @res: result values via struct arm_smccc_1_2_regs
//
// This function is used to make HVC calls following SMC Calling Convention
// v1.2 or above. The content of the supplied param are copied from the
// structure to registers prior to the HVC instruction. The return values
// are updated with the content from registers on return from the HVC
// instruction.
//
// arm_smccc_1_2_smc() - make SMC calls
// @args: arguments passed via struct arm_smccc_1_2_regs
// @res: result values via struct arm_smccc_1_2_regs
//
// This function is used to make SMC calls following SMC Calling Convention
// v1.2 or above. The content of the supplied param are copied from the
// structure to registers prior to the SMC instruction. The return values
// are updated with the content from registers on return from the SMC
// instruction.
//

//
// struct arm_smccc_quirk - Contains quirk information
// @id: quirk identification
// @state: quirk specific information
// @a6: Qualcomm quirk entry for returning post-smc call contents of a6
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smccc_quirk {
    pub id: c_int,
    pub a6: c_ulong,
    pub state: },
}

//
// __arm_smccc_smc() - make SMC calls
// @a0-a7: arguments passed in registers 0 to 7
// @res: result values from registers 0 to 3
// @quirk: points to an arm_smccc_quirk, or NULL when no quirks are required.
//
// This function is used to make SMC calls following SMC Calling Convention.
// The content of the supplied param are copied to registers 0 to 7 prior
// to the SMC instruction. The return values are updated with the content
// from register 0 to 3 on return from the SMC instruction.  An optional
// quirk structure provides vendor specific behavior.
//

// res = (struct arm_smccc_res){};

//
// __arm_smccc_hvc() - make HVC calls
// @a0-a7: arguments passed in registers 0 to 7
// @res: result values from registers 0 to 3
// @quirk: points to an arm_smccc_quirk, or NULL when no quirks are required.
//
// This function is used to make HVC calls following SMC Calling
// Convention.  The content of the supplied param are copied to registers 0
// to 7 prior to the HVC instruction. The return values are updated with
// the content from register 0 to 3 on return from the HVC instruction.  An
// optional quirk structure provides vendor specific behavior.
//

// SMCCC v1.1 implementation madness follows

//
// We have an output list that is not necessarily used, and GCC feels
// entitled to optimise the whole sequence away. "volatile" is what
// makes it stick.
//

// ___res = (typeof(*___res)){r0, r1, r2, r3};	\
//
// arm_smccc_1_1_smc() - make an SMCCC v1.1 compliant SMC call
//
// This is a variadic macro taking one to eight source arguments, and
// an optional return structure.
//
// @a0-a7: arguments passed in registers 0 to 7
// @res: result values from registers 0 to 3
//
// This macro is used to make SMC calls following SMC Calling Convention v1.1.
// The content of the supplied param are copied to registers 0 to 7 prior
// to the SMC instruction. The return values are updated with the content
// from register 0 to 3 on return from the SMC instruction if not NULL.
//

//
// arm_smccc_1_1_hvc() - make an SMCCC v1.1 compliant HVC call
//
// This is a variadic macro taking one to eight source arguments, and
// an optional return structure.
//
// @a0-a7: arguments passed in registers 0 to 7
// @res: result values from registers 0 to 3
//
// This macro is used to make HVC calls following SMC Calling Convention v1.1.
// The content of the supplied param are copied to registers 0 to 7 prior
// to the HVC instruction. The return values are updated with the content
// from register 0 to 3 on return from the HVC instruction if not NULL.
//

//
// Like arm_smccc_1_1* but always returns SMCCC_RET_NOT_SUPPORTED.
// Used when the SMCCC conduit is not defined. The empty asm statement
// avoids compiler warnings about unused variables.
//

//
// arm_smccc_1_1_invoke() - make an SMCCC v1.1 compliant call
//
// This is a variadic macro taking one to eight source arguments, and
// an optional return structure.
//
// @a0-a7: arguments passed in registers 0 to 7
// @res: result values from registers 0 to 3
//
// This macro will make either an HVC call or an SMC call depending on the
// current SMCCC conduit. If no valid conduit is available then -1
// (SMCCC_RET_NOT_SUPPORTED) is returned in @res.a0 (if supplied).
//
// The return value also provides the conduit that was used.
//

//
// arm_smccc_1_2_invoke() - make an SMCCC v1.2 compliant call
//
// @args: SMC args are in the a0..a17 fields of the arm_smcc_1_2_regs structure
// @res: result values from registers 0 to 17
//
// This macro will make either an HVC call or an SMC call depending on the
// current SMCCC conduit. If no valid conduit is available then -1
// (SMCCC_RET_NOT_SUPPORTED) is returned in @res.a0 (if supplied).
//
// The return value also provides the conduit that was used.
//

