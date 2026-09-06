//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/reverse_cpuid.h
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
// Define a KVM-only feature flag.
//
// For features that are scattered by cpufeatures.h, __feature_translate() also
// needs to be updated to translate the kernel-defined feature into the
// KVM-defined feature.
//
// For features that are 100% KVM-only, i.e. not defined by cpufeatures.h,
// forego the intermediate KVM_X86_FEATURE and directly define X86_FEATURE_* so
// that X86_FEATURE_* can be used in KVM.  No __feature_translate() handling is
// needed in this case.
//

// Intel-defined SGX sub-features, CPUID level 0x12 (EAX).

// Intel-defined sub-features, CPUID level 0x00000007:1 (ECX)

// Intel-defined sub-features, CPUID level 0x00000007:1 (EDX)

// Intel-defined sub-features, CPUID level 0x00000007:2 (EDX)

//
// Intel-defined sub-features, CPUID level 0x0000001E:1 (EAX).  Note, several
// of the bits are aliases to features of the same name that are enumerated via
// various CPUID.0x7 sub-leafs.
//

// Intel-defined sub-features, CPUID level 0x00000024:0 (EBX)

// Intel-defined sub-features, CPUID level 0x00000024:1 (ECX)

// CPUID level 0x80000007 (EDX).

// CPUID level 0x80000022 (EAX)

// CPUID level 0x80000021 (ECX)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_reg {
    pub function: u32,
    pub index: u32,
    pub reg: c_int,
}

//
// Reverse CPUID and its derivatives can only be used for hardware-defined
// feature words, i.e. words whose bits directly correspond to a CPUID leaf.
// Retrieving a feature bit or masking guest CPUID from a Linux-defined word
// is nonsensical as the bit number/mask is an arbitrary software-defined value
// and can't be used by KVM to query/control guest capabilities.  And obviously
// the leaf being queried must have an entry in the lookup table.
//
// Translate feature bits that are scattered in the kernel's cpufeatures word
// into KVM feature words that align with hardware's definitions.
//

//
// Retrieve the bit mask from an X86_FEATURE_* definition.  Features contain
// the hardware defined bit number (stored in bits 4:0) and a software defined
// "word" (stored in bits 31:5).  The word is used to index into arrays of
// bit masks that hold the per-cpu feature capabilities, e.g. this_cpu_has().
//

extern "C" {
    pub fn __cpuid_entry_get_reg(_arg: entry, _arg: cpuid.reg) -> return;
}
extern "C" {
    pub fn cpuid_entry_get(_arg: entry, _arg: x86_feature) -> return;
}
// reg &= ~__feature_bit(x86_feature);
// reg |= __feature_bit(x86_feature);
//
// Open coded instead of using cpuid_entry_{clear,set}() to coerce the
// compiler into using CMOV instead of Jcc when possible.
//
// reg |= __feature_bit(x86_feature);
// reg &= ~__feature_bit(x86_feature);
