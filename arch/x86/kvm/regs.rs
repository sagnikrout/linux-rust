//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/regs.h
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
// DR6_ACTIVE_LOW combines fixed-1 and active-low bits.
// We can regard all the bits in DR6_FIXED_1 as active_low bits;
// they will never be 0 for now, but when they are defined
// in the future it will require no code change.
//
// DR6_ACTIVE_LOW is also used as the init/reset value for DR6.
//
pub const DR6_ACTIVE_LOW: c_uint = 0xffff0ff0;
pub const DR6_VOLATILE: c_uint = 0x0001e80f;

pub const DR7_BP_EN_MASK: c_uint = 0x000000ff;

pub const DR7_VOLATILE: c_uint = 0xffff2bff;
extern "C" {
    pub fn kvm_post_set_cr0(vcpu: *mut kvm_vcpu, old_cr0: c_ulong, cr0: c_ulong);
}
extern "C" {
    pub fn kvm_post_set_cr4(vcpu: *mut kvm_vcpu, old_cr4: c_ulong, cr4: c_ulong);
}
extern "C" {
    pub fn kvm_set_cr0(vcpu: *mut kvm_vcpu, cr0: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_set_cr3(vcpu: *mut kvm_vcpu, cr3: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_set_cr4(vcpu: *mut kvm_vcpu, cr4: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_set_cr8(vcpu: *mut kvm_vcpu, cr8: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_set_dr(vcpu: *mut kvm_vcpu, dr: c_int, val: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_get_dr(vcpu: *mut kvm_vcpu, dr: c_int) -> c_ulong;
}
extern "C" {
    pub fn kvm_get_cr8(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn kvm_lmsw(vcpu: *mut kvm_vcpu, msw: c_ulong);
}
extern "C" {
    pub fn load_pdptrs(vcpu: *mut kvm_vcpu, cr3: c_ulong) -> c_int;
}

//
// If running with protected guest state, the CS register is not
// accessible. The hypercall register values will have had to been
// provided in 64-bit mode, so assume the guest is in 64-bit.
//

extern "C" {
    pub fn is_64_bit_mode(GENMASK(63: vcpu) ?, GENMASK(31: 0) :, _arg: 0) -> return;
}

extern "C" {
    pub fn GENMASK(_arg: 31, _arg: 0) -> return;
}

//
// Using the register cache from interrupt context is generally not allowed, as
// caching a register and marking it available/dirty can't be done atomically,
// i.e. accesses from interrupt context may clobber state or read stale data if
// the vCPU task is in the process of updating the cache.  The exception is if
// KVM is handling a PMI IRQ/NMI VM-Exit, as that bound code sequence doesn't
// touch the cache, it runs after the cache is reset (post VM-Exit), and PMIs
// need to access several registers that are cacheable.
//

//
// avail  dirty
// 0	  0	  register in VMCS/VMCB
// 0	  1	  *INVALID
// 1	  0	  register in vcpu->arch
// 1	  1	  register in vcpu->arch, needs to be stored back
//
extern "C" {
    pub fn test_bit(_arg: reg, _arg: vcpu->arch.regs_avail) -> return;
}
extern "C" {
    pub fn test_bit(_arg: reg, _arg: vcpu->arch.regs_dirty) -> return;
}
//
// kvm_register_test_and_mark_available() is a special snowflake that uses an
// arch bitop directly to avoid the explicit instrumentation that comes with
// the generic bitops.  This allows code that cannot be instrumented (noinstr
// functions), e.g. the low level VM-Enter/VM-Exit paths, to cache registers.
//
extern "C" {
    pub fn arch___test_and_set_bit(_arg: reg, _arg: vcpu->arch.regs_avail) -> return;
}
//
// Note the bitwise-AND!  In practice, a straight write would also work
// as KVM initializes the mask to all ones and never clears registers
// that are eagerly synchronized.  Using a bitwise-AND adds a bit of
// sanity checking as incorrectly marking an eagerly sync'd register
// unavailable will generate a WARN due to an unexpected cache request.
//
// The "raw" register helpers are only for cases where the full 64 bits of a
// register are read/written irrespective of current vCPU mode.  In other words,
// odds are good you shouldn't be using the raw variants.
//
extern "C" {
    pub fn kvm_register_read_raw(_arg: vcpu, kvm_reg_mode_mask(vcpu: reg) &) -> return;
}
extern "C" {
    pub fn kvm_register_write_raw(_arg: vcpu, _arg: reg, kvm_reg_mode_mask(vcpu): val &) -> return;
}
extern "C" {
    pub fn kvm_register_read_raw(_arg: vcpu, _arg: VCPU_REGS_RSP) -> return;
}
extern "C" {
    pub fn kvm_read_cr0_bits(_arg: vcpu, _arg: ~0UL) -> return;
}
extern "C" {
    pub fn kvm_read_cr4_bits(_arg: vcpu, _arg: ~0UL) -> return;
}

extern "C" {
    pub fn kvm_is_cr0_bit_set(_arg: vcpu, _arg: X86_CR0_PE) -> return;
}
extern "C" {
    pub fn kvm_is_cr4_bit_set(_arg: vcpu, _arg: X86_CR4_PAE) -> return;
}
extern "C" {
    pub fn kvm_is_cr4_bit_set(_arg: vcpu, _arg: X86_CR4_PSE) -> return;
}
extern "C" {
    pub fn likely(_arg: kvm_is_cr0_bit_set(vcpu, _arg: X86_CR0_PG)) -> return;
}
// Bits [63:32] are reserved
extern "C" {
    pub fn kvm_x86_call(_arg: get_segment_base)(vcpu, _arg: seg) -> return;
}
extern "C" {
    pub fn kvm_get_linear_rip(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn kvm_is_linear_rip(vcpu: *mut kvm_vcpu, linear_rip: c_ulong) -> bool;
}
extern "C" {
    pub fn kvm_get_rflags(vcpu: *mut kvm_vcpu) -> c_ulong;
}
extern "C" {
    pub fn __kvm_set_rflags(vcpu: *mut kvm_vcpu, rflags: c_ulong);
}
extern "C" {
    pub fn kvm_set_rflags(vcpu: *mut kvm_vcpu, rflags: c_ulong);
}
extern "C" {
    pub fn kvm_run_sync_regs_to_user(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_run_sync_regs_from_user(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_update_dr0123(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_update_dr7(vcpu: *mut kvm_vcpu);
}
