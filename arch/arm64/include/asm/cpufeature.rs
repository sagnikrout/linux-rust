//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/cpufeature.h
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
// Copyright (C) 2014 Linaro Ltd. <ard.biesheuvel@linaro.org>
//

pub const MAX_CPU_FEATURES: c_int = 192;

pub const ARM64_SW_FEATURE_OVERRIDE_NOKASLR: c_int = 0;
pub const ARM64_SW_FEATURE_OVERRIDE_HVHE: c_int = 4;
pub const ARM64_SW_FEATURE_OVERRIDE_RODATA_OFF: c_int = 8;

//
// CPU feature register tracking
//
// The safe value of a CPUID feature field is dependent on the implications
// of the values assigned to it by the architecture. Based on the relationship
// between the values, the features are classified into 3 types - LOWER_SAFE,
// HIGHER_SAFE and EXACT.
//
// The lowest value of all the CPUs is chosen for LOWER_SAFE and highest
// for HIGHER_SAFE. It is expected that all CPUs have the same value for
// a field when EXACT is specified, failing which, the safe value specified
// in the table is chosen.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftr_type {
    FTR_EXACT,			/* Use a predefined safe value */
    FTR_LOWER_SAFE,			/* Smaller value is safe */
    FTR_HIGHER_SAFE,		/* Bigger value is safe */
    FTR_HIGHER_OR_ZERO_SAFE,	/* Bigger value is safe, but 0 is biggest */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm64_ftr_bits {
    pub /: *mut *mut bool sign; / Value is signed ?,
    pub visible: bool,
    pub /: *mut *mut bool strict; / CPU Sanity check: strict matching required ?,
    pub type: ftr_type,
    pub shift: u8,
    pub width: u8,
    pub /: *mut *mut s64 safe_val; / safe value for FTR_EXACT features,
}

//
// Describe the early feature override to the core override code:
//
// @val			Values that are to be merged into the final
// sanitised value of the register. Only the bitfields
// set to 1 in @mask are valid
// @mask		Mask of the features that are overridden by @val
//
// A @mask field set to full-1 indicates that the corresponding field
// in @val is a valid override.
//
// A @mask field set to full-0 with the corresponding @val field set
// to full-0 denotes that this field has no override
//
// A @mask field set to full-0 with the corresponding @val field set
// to full-1 denotes that this field has an invalid override.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm64_ftr_override {
    pub val: u64,
    pub mask: u64,
}

//
// @arm64_ftr_reg - Feature register
// @strict_mask		Bits which should match across all CPUs for sanity.
// @sys_val		Safe value across the CPUs (system view)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm64_ftr_reg {
    pub name: *const c_char,
    pub strict_mask: u64,
    pub user_mask: u64,
    pub sys_val: u64,
    pub user_val: u64,
    pub override: *mut arm64_ftr_override,
    pub ftr_bits: *const arm64_ftr_bits,
}

//
// CPU capabilities:
//
// We use arm64_cpu_capabilities to represent system features, errata work
// arounds (both used internally by kernel and tracked in system_cpucaps) and
// ELF HWCAPs (which are exposed to user).
//
// To support systems with heterogeneous CPUs, we need to make sure that we
// detect the capabilities correctly on the system and take appropriate
// measures to ensure there are no incompatibilities.
//
// This comment tries to explain how we treat the capabilities.
// Each capability has the following list of attributes :
//
// 1) Scope of Detection : The system detects a given capability by
// performing some checks at runtime. This could be, e.g, checking the
// value of a field in CPU ID feature register or checking the cpu
// model. The capability provides a call back ( @matches() ) to
// perform the check. Scope defines how the checks should be performed.
// There are three cases:
//
// a) SCOPE_LOCAL_CPU: check all the CPUs and "detect" if at least one
// matches. This implies, we have to run the check on all the
// booting CPUs, until the system decides that state of the
// capability is finalised. (See section 2 below)
// Or
// b) SCOPE_SYSTEM: check all the CPUs and "detect" if all the CPUs
// matches. This implies, we run the check only once, when the
// system decides to finalise the state of the capability. If the
// capability relies on a field in one of the CPU ID feature
// registers, we use the sanitised value of the register from the
// CPU feature infrastructure to make the decision.
// Or
// c) SCOPE_BOOT_CPU: Check only on the primary boot CPU to detect the
// feature. This category is for features that are "finalised"
// (or used) by the kernel very early even before the SMP cpus
// are brought up.
//
// The process of detection is usually denoted by "update" capability
// state in the code.
//
// 2) Finalise the state : The kernel should finalise the state of a
// capability at some point during its execution and take necessary
// actions if any. Usually, this is done, after all the boot-time
// enabled CPUs are brought up by the kernel, so that it can make
// better decision based on the available set of CPUs. However, there
// are some special cases, where the action is taken during the early
// boot by the primary boot CPU. (e.g, running the kernel at EL2 with
// Virtualisation Host Extensions). The kernel usually disallows any
// changes to the state of a capability once it finalises the capability
// and takes any action, as it may be impossible to execute the actions
// safely. A CPU brought up after a capability is "finalised" is
// referred to as "Late CPU" w.r.t the capability. e.g, all secondary
// CPUs are treated "late CPUs" for capabilities determined by the boot
// CPU.
//
// At the moment there are two passes of finalising the capabilities.
// a) Boot CPU scope capabilities - Finalised by primary boot CPU via
// setup_boot_cpu_capabilities().
// b) Everything except (a) - Run via setup_system_capabilities().
//
// 3) Verification: When a CPU is brought online (e.g, by user or by the
// kernel), the kernel should make sure that it is safe to use the CPU,
// by verifying that the CPU is compliant with the state of the
// capabilities finalised already. This happens via :
//
// secondary_start_kernel()-> check_local_cpu_capabilities()
//
// As explained in (2) above, capabilities could be finalised at
// different points in the execution. Each newly booted CPU is verified
// against the capabilities that have been finalised by the time it
// boots.
//
// a) SCOPE_BOOT_CPU : All CPUs are verified against the capability
// except for the primary boot CPU.
//
// b) SCOPE_LOCAL_CPU, SCOPE_SYSTEM: All CPUs hotplugged on by the
// user after the kernel boot are verified against the capability.
//
// If there is a conflict, the kernel takes an action, based on the
// severity (e.g, a CPU could be prevented from booting or cause a
// kernel panic). The CPU is allowed to "affect" the state of the
// capability, if it has not been finalised already. See section 5
// for more details on conflicts.
//
// 4) Action: As mentioned in (2), the kernel can take an action for each
// detected capability, on all CPUs on the system. Appropriate actions
// include, turning on an architectural feature, modifying the control
// registers (e.g, SCTLR, TCR etc.) or patching the kernel via
// alternatives. The kernel patching is batched and performed at later
// point. The actions are always initiated only after the capability
// is finalised. This is usually denoted by "enabling" the capability.
// The actions are initiated as follows :
// a) Action is triggered on all online CPUs, after the capability is
// finalised, invoked within the stop_machine() context from
// enable_cpu_capabilitie().
//
// b) Any late CPU, brought up after (1), the action is triggered via:
//
// check_local_cpu_capabilities() -> verify_local_cpu_capabilities()
//
// 5) Conflicts: Based on the state of the capability on a late CPU vs.
// the system state, we could have the following combinations :
//
// x-----------------------------x
// | Type  | System   | Late CPU |
// |-----------------------------|
// |  a    |   y      |    n     |
// |-----------------------------|
// |  b    |   n      |    y     |
// x-----------------------------x
//
// Two separate flag bits are defined to indicate whether each kind of
// conflict can be allowed:
// ARM64_CPUCAP_OPTIONAL_FOR_LATE_CPU - Case(a) is allowed
// ARM64_CPUCAP_PERMITTED_FOR_LATE_CPU - Case(b) is allowed
//
// Case (a) is not permitted for a capability that the system requires
// all CPUs to have in order for the capability to be enabled. This is
// typical for capabilities that represent enhanced functionality.
//
// Case (b) is not permitted for a capability that must be enabled
// during boot if any CPU in the system requires it in order to run
// safely. This is typical for erratum work arounds that cannot be
// enabled after the corresponding capability is finalised.
//
// In some non-typical cases either both (a) and (b), or neither,
// should be permitted. This can be described by including neither
// or both flags in the capability's type field.
//
// In case of a conflict, the CPU is prevented from booting. If the
// ARM64_CPUCAP_PANIC_ON_CONFLICT flag is specified for the capability,
// then a kernel panic is triggered.
//
// Decide how the capability is detected.
// On any local CPU vs System wide vs the primary boot CPU
//

//
// The capability is detected on the Boot CPU and is used by kernel
// during early boot. i.e, the capability should be "detected" and
// "enabled" as early as possibly on all booting CPUs.
//

//
// Is it permitted for a late CPU to have this capability when system
// hasn't already enabled it ?
//

// Is it safe for a late CPU to miss this capability when system has it

// Panic when a conflict is detected

//
// When paired with SCOPE_LOCAL_CPU, all early CPUs must satisfy the
// condition. This is different from SCOPE_SYSTEM where the check is performed
// only once at the end of the SMP boot on the sanitised ID registers.
// SCOPE_SYSTEM is not suitable for cases where the capability depends on
// properties local to a CPU like MIDR_EL1.
//

//
// CPU errata workarounds that need to be enabled at boot time if one or
// more CPUs in the system requires it. When one of these capabilities
// has been enabled, it is safe to allow any CPU to boot that doesn't
// require the workaround. However, it is not safe if a "late" CPU
// requires a workaround and the system hasn't enabled it already.
//

//
// CPU feature detected at boot time based on system-wide value of a
// feature. It is safe for a late CPU to have this feature even though
// the system hasn't enabled it, although the feature will not be used
// by Linux in this case. If the system has enabled this feature already,
// then every late CPU must have it.
//

//
// CPU feature detected at boot time based on feature of one or more CPUs.
// All possible conflicts for a late CPU are ignored.
// NOTE: this means that a late CPU with the feature will *not* cause the
// capability to be advertised by cpus_have_*cap()!
//

//
// CPU feature detected at boot time and present on all early CPUs. Late CPUs
// are permitted to have the feature even if it hasn't been enabled, although
// the feature will not be used by Linux in this case. If all early CPUs have
// the feature, then every late CPU must have it.
//

//
// CPU feature detected at boot time, on one or more CPUs. A late CPU
// is not allowed to have the capability when the system doesn't have it.
// It is Ok for a late CPU to miss the feature.
//

//
// CPU feature used early in the boot based on the boot CPU. All secondary
// CPUs must match the state of the capability as detected by the boot CPU. In
// case of a conflict, a kernel panic is triggered.
//

//
// CPU feature used early in the boot based on the boot CPU. It is safe for a
// late CPU to have this feature even though the boot CPU hasn't enabled it,
// although the feature will not be used by Linux in this case. If the boot CPU
// has enabled this feature already, then every late CPU must have it.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm64_cpu_capabilities {
    pub desc: *const c_char,
    pub capability: u16,
    pub type: u16,
    pub scope): *const *const *const bool (matches)(struct arm64_cpu_capabilities caps, int,
//
// Take the appropriate actions to configure this capability
// for this CPU. If the capability is detected by the kernel
// this will be called on all the CPUs in the system,
// including the hotplugged CPUs, regardless of whether the
// capability is available on that specific CPU. This is
// useful for some capabilities (e.g, working around CPU
// errata), where all the CPUs must take some action (e.g,
// changing system control/configuration). Thus, if an action
// is required only if the CPU has the capability, then the
// routine must check it before taking any action.
//
    pub cap): *const *const void (cpu_enable)(struct arm64_cpu_capabilities,
    pub midr_range: midr_range,
    pub /: *mut *mut u32 midr_rv; / revision/variant,
    pub revidr_mask: u32,
    pub fixed_revs: *const *const },
}

//
// An optional list of "matches/cpu_enable" pair for the same
// "capability" of the same "type" as described by the parent.
// Only matches(), cpu_enable() and fields relevant to these
// methods are significant in the list. The cpu_enable is
// invoked only if the corresponding entry "matches()".
// However, if a cpu_enable() method is associated
// with multiple matches(), care should be taken that either
// the match criteria are mutually exclusive, or that the
// method is robust against being called multiple times.
//
// Generic helper for handling capabilities with multiple (match,enable) pairs
// of call backs, sharing the same capability bit.
// Iterate over each entry to see if at least one matches.
//
// Only defined for code run in VHE hyp context
extern "C" {
    pub fn __is_defined(_arg: __KVM_VHE_HYPERVISOR__) -> return;
}
// Only defined for code run in NVHE hyp context
extern "C" {
    pub fn __is_defined(_arg: __KVM_NVHE_HYPERVISOR__) -> return;
}
extern "C" {
    pub fn is_vhe_hyp_code(is_nvhe_hyp_code(: ) ||) -> return;
}
extern "C" {
    pub fn DECLARE_BITMAP(_arg: system_cpucaps, _arg: ARM64_NCAPS) -> extern;
}
extern "C" {
    pub fn DECLARE_BITMAP(_arg: boot_cpucaps, _arg: ARM64_NCAPS) -> extern;
}

extern "C" {
    pub fn this_cpu_has_cap(cap: c_uint) -> bool;
}
extern "C" {
    pub fn cpu_set_feature(num: c_uint);
}
extern "C" {
    pub fn cpu_have_feature(num: c_uint) -> bool;
}
extern "C" {
    pub fn cpu_get_elf_hwcap() -> c_ulong;
}
extern "C" {
    pub fn cpu_get_elf_hwcap2() -> c_ulong;
}
extern "C" {
    pub fn cpu_get_elf_hwcap3() -> c_ulong;
}

extern "C" {
    pub fn alternative_has_cap_likely(_arg: ARM64_ALWAYS_BOOT) -> return;
}
extern "C" {
    pub fn alternative_has_cap_likely(_arg: ARM64_ALWAYS_SYSTEM) -> return;
}
//
// Test for a capability with a runtime check.
//
// Before the capability is detected, this returns false.
//
extern "C" {
    pub fn arch_test_bit(_arg: num, _arg: system_cpucaps) -> return;
}
//
// Test for a capability without a runtime check.
//
// Before boot capabilities are finalized, this will BUG().
// After boot capabilities are finalized, this is patched to avoid a runtime
// check.
//
// @num must be a compile-time constant.
//
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: num) -> return;
}
//
// Test for a capability without a runtime check.
//
// Before system capabilities are finalized, this will BUG().
// After system capabilities are finalized, this is patched to avoid a runtime
// check.
//
// @num must be a compile-time constant.
//
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: num) -> return;
}
extern "C" {
    pub fn cpuid_feature_extract_signed_field_width(_arg: features, _arg: field, _arg: 4) -> return;
}
extern "C" {
    pub fn cpuid_feature_extract_unsigned_field_width(_arg: features, _arg: field, _arg: 4) -> return;
}
extern "C" {
    pub fn cpuid_feature_extract_field_width(_arg: features, _arg: field, _arg: 4, _arg: sign) -> return;
}
extern "C" {
    pub fn setup_boot_cpu_features() -> void __init;
}
extern "C" {
    pub fn setup_system_features() -> void __init;
}
extern "C" {
    pub fn setup_user_features() -> void __init;
}
extern "C" {
    pub fn check_local_cpu_capabilities();
}
extern "C" {
    pub fn read_sanitised_ftr_reg(id: u32) -> u64;
}
extern "C" {
    pub fn __read_sysreg_by_encoding(sys_id: u32) -> u64;
}
extern "C" {
    pub fn id_aa64mmfr0_mixed_endian_el0(_arg: read_cpuid(ID_AA64MMFR0_EL1)) -> return;
}
extern "C" {
    pub fn id_aa64mmfr0_mixed_endian_el0(_arg: read_sanitised_ftr_reg(SYS_ID_AA64MMFR0_EL1)) -> return;
}
extern "C" {
    pub fn alternative_has_cap_likely(_arg: ARM64_HAS_FPSIMD) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_PAN) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_SVE) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_SME) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_SME2) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_SME_FA64) -> return;
}
extern "C" {
    pub fn system_supports_sme() -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_FPMR) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_CNP) -> return;
}
extern "C" {
    pub fn cpus_have_final_boot_cap(_arg: ARM64_HAS_ADDRESS_AUTH) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_GENERIC_AUTH) -> return;
}
extern "C" {
    pub fn system_supports_address_auth(system_supports_generic_auth(: ) &&) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_GIC_PRIO_MASKING) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_MTE) -> return;
}
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_BTI) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_TLB_RANGE) -> return;
}
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_HAS_LPA2) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_S1POE) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_GCS) -> return;
}
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_HAFT) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_MPAM) -> return;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_MPAM_HCR) -> return;
}
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_HAS_PMUV3) -> return;
}
extern "C" {
    pub fn cpu_supports_bbml3() -> bool;
}
extern "C" {
    pub fn alternative_has_cap_unlikely(_arg: ARM64_HAS_BBML3) -> return;
}
extern "C" {
    pub fn do_emulate_mrs(regs: *mut pt_regs, sys_reg: u32, rt: u32) -> c_int;
}
extern "C" {
    pub fn try_emulate_mrs(regs: *mut pt_regs, isn: u32) -> bool;
}
//
// A future PE could use a value unknown to the kernel.
// However, by the "D10.1.4 Principles of the ID scheme
// for fields in ID registers", ARM DDI 0487C.a, any new
// value is guaranteed to be higher than what we know already.
// As a safe limit, we return the limit supported by the kernel.
//
// Check whether hardware update of the Access flag is supported
//
// Use cached version to avoid emulated msr operation on KVM
// guests.
//

// Check whether the cpu supports the Activity Monitors Unit (AMU)
extern "C" {
    pub fn cpu_has_amu_feat(cpu: c_int) -> bool;
}

// Get a cpu that supports the Activity Monitors Unit (AMU)
extern "C" {
    pub fn get_cpu_with_amu_feat() -> c_int;
}
//
// Return the default here even if any reserved
// value is fetched from the system register.
//
extern "C" {
    pub fn arm64_ftr_safe_value(ftrp: *const arm64_ftr_bits, new: i64, cur: i64) -> i64;
}
//
// When it encounters an invalid override (e.g., an override that
// cannot be honoured due to a missing CPU feature), the early idreg
// override code will set the mask to 0x0 and the value to non-zero for
// the field in question. In order to determine whether the override is
// valid or not for the field we are interested in, we first need to
// disregard bits belonging to other fields.
//
// The override is valid if all value bits are accounted for in the
// mask. If so, replace the masked bits with the override value.
//
// Extract the field from the updated value
extern "C" {
    pub fn cpuid_feature_extract_unsigned_field(_arg: val, _arg: feat) -> return;
}
//
// Software features are pseudo CPU features that have no underlying
// CPUID system register value to apply the override to.
//
extern "C" {
    pub fn arm64_test_sw_feature_override(_arg: ARM64_SW_FEATURE_OVERRIDE_NOKASLR) -> return;
}
extern "C" {
    pub fn get_kvm_ipa_limit() -> u32;
}
extern "C" {
    pub fn dump_cpu_features();
}

