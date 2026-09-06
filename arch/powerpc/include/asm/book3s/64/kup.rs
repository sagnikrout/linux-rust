//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/kup.h
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
// AMR and IAMR are going to be different when
// returning to userspace.
//
// If kuap feature is not enabled, do the mtspr
// only if AMR value is different.
//
// Restore IAMR only when returning to userspace
//
// If kuep feature is not enabled, do the mtspr
// only if IAMR value is different.
//
// No isync required, see kuap_user_restore()

//
// AMR is going to be mostly the same since we are
// returning to the kernel. Compare and do a mtspr.
//
// No isync required, see kuap_restore_amr()
// No need to restore IAMR when returning to kernel space.
//

// Prevent access to userspace using any key values

//
// if (pkey) {
//
// save AMR -> stack;
// if (kuap) {
// if (AMR != BLOCKED)
// KUAP_BLOCKED -> AMR;
// }
// if (from_user) {
// save IAMR -> stack;
// if (kuep) {
// KUEP_BLOCKED ->IAMR
// }
// return;
// }
//
// if (kuap) {
// if (from_kernel) {
// save AMR -> stack;
// if (AMR != BLOCKED)
// KUAP_BLOCKED -> AMR;
// }
//
// }
//

//
// if both pkey and kuap is disabled, nothing to do
//
// if pkey is disabled and we are entering from userspace
// don't do anything.
//
// Without pkey we are not changing AMR outside the kernel
// hence skip this completely.
//
// pkey is enabled or pkey is disabled but entering from kernel
//
// update kernel AMR with AMR_KUAP_BLOCKED only
// if KUAP feature is enabled
//
// We don't isync here because we very recently entered via an interrupt
//
// if entering from kernel we don't need save IAMR
//
// update kernel IAMR with AMR_KUEP_BLOCKED only
// if KUEP feature is enabled
//

// usage of kthread_use_mm() should inherit the
// AMR value of the operating address space. But, the AMR value is
// thread-specific and we inherit the address space and not thread
// access restrictions. Because of this ignore AMR value when accessing
// userspace via kernel thread.
//

//
// No isync required here because we are about to rfi
// back to previous context before any user accesses
// would be made, which is a CSI.
//
// No isync required here because we are about to rfi
// back to previous context before any user accesses
// would be made, which is a CSI.
//
// No need to restore IAMR when returning to kernel space.
//

// __kuap_lock() not required, book3s/64 does that in ASM
//
// We support individually allowing read or write, but we don't support nesting
// because that would require an expensive read/modify write of the AMR.
//
// We return AMR_KUAP_BLOCKED when we don't support KUAP because
// prevent_user_access_return needs to return AMR_KUAP_BLOCKED to
// cause restore_user_access to do a flush.
//
// This has no effect in terms of actually blocking things on hash,
// so it doesn't break anything.
//
extern "C" {
    pub fn mfspr(_arg: SPRN_AMR) -> return;
}
//
// ISA v3.0B says we need a CSI (Context Synchronising Instruction) both
// before and after the move to AMR. See table 6 on page 1134.
//
// For radix this will be a storage protection fault (DSISR_PROTFAULT).
// For hash this will be a key fault (DSISR_KEYFAULT)
//
// We do have exception table entry, but accessing the
// userspace results in fault.  This could be because we
// didn't unlock the AMR or access is denied by userspace
// using a key value that blocks access. We are only interested
// in catching the use case of accessing without unlocking
// the AMR. Hence check for BLOCK_WRITE/READ against AMR.
//
// This is written so we can resolve to a single case at build time

