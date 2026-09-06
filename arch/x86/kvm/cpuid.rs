//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/cpuid.h
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

extern "C" {
    pub fn kvm_initialize_cpu_caps();
}
extern "C" {
    pub fn kvm_vcpu_after_set_cpuid(vcpu: *mut kvm_vcpu);
}
//
// Magic value used by KVM when querying userspace-provided CPUID entries and
// doesn't care about the CPIUD index because the index of the function in
// question is not significant.  Note, this magic value must have at least one
// bit set in bits[63:32] and must be consumed as a u64 by kvm_find_cpuid_entry2()
// to avoid false positives when processing guest CPUID input.
//
// KVM_CPUID_INDEX_NOT_SIGNIFICANT should never be used directly outside of
// kvm_find_cpuid_entry2() and kvm_find_cpuid_entry().
//

extern "C" {
    pub fn kvm_init_xstate_sizes() -> void __init;
}
extern "C" {
    pub fn xstate_required_size(xstate_bv: u64, compacted: bool) -> u32;
}
extern "C" {
    pub fn cpuid_query_maxphyaddr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn cpuid_query_maxguestphyaddr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_reserved_gpa_bits_raw(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn IS_ALIGNED(_arg: gpa, kvm_vcpu_is_legal_gpa(vcpu: alignment) &&, _arg: gpa) -> return;
}
extern "C" {
    pub fn kvm_vcpu_is_legal_aligned_gpa(_arg: vcpu, _arg: gpa, _arg: PAGE_SIZE) -> return;
}
// reg = kvm_cpu_caps[leaf];
//
// XSAVES is a special snowflake.  Due to lack of a dedicated intercept
// on SVM, KVM must assume that XSAVES (and thus XRSTORS) is usable by
// the guest if the host supports XSAVES and *XSAVE* is exposed to the
// guest.  Because the guest can execute XSAVES and XRSTORS, i.e. can
// indirectly consume XSS, KVM must ensure XSS is zeroed when running
// the guest, i.e. must set XSAVES in vCPU capabilities.  But to reject
// direct XSS reads and writes (to minimize the virtualization hole and
// honor userspace's CPUID), KVM needs to check the raw guest CPUID,
// not KVM's view of guest capabilities.
//
// For all other features, guest capabilities are accurate.  Expand
// this allowlist with extreme vigilance.
//
extern "C" {
    pub fn x86_family(_arg: best->eax) -> return;
}
extern "C" {
    pub fn x86_model(_arg: best->eax) -> return;
}
extern "C" {
    pub fn x86_stepping(_arg: best->eax) -> return;
}
//
// Except for MWAIT, querying dynamic feature bits is disallowed, so
// that KVM can defer runtime updates until the next CPUID emulation.
//
extern "C" {
    pub fn kvm_vcpu_is_legal_gpa(_arg: vcpu, _arg: cr3) -> return;
}
