//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/tlbflush.h
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

// TLB flush actions. Used as argument to tlbiel_all()
//
// This is used for host machine check and bootup.
//
// This uses early_radix_enabled and implementations use
// early_cpu_has_feature etc because that works early in boot
// and this is the machine check path which is not performance
// critical.
//
// This is used for guest machine check.
//

//
// Book3S 64 does not require spurious fault flushes because the PTE
// must be re-fetched in case of an access permission problem. So the
// only reason for a spurious fault should be concurrent modification
// to the PTE, in which case the PTE will eventually be re-fetched by
// the MMU when it attempts the access again.
//
// See: Power ISA Version 3.1B, 6.10.1.2 Modifying a Translation Table
// Entry, Setting a Reference or Change Bit or Upgrading Access
// Authority (PTE Subject to Atomic Hardware Updates):
//
// "If the only change being made to a valid PTE that is subject to
// atomic hardware updates is to set the Reference or Change bit to
// 1 or to upgrade access authority, a simpler sequence suffices
// because the translation hardware will refetch the PTE if an
// access is attempted for which the only problems were reference
// and/or change bits needing to be set or insufficient access
// authority."
//
// The nest MMU in POWER9 does not perform this PTE re-fetch, but
// it avoids the spurious fault problem by flushing the TLB before
// upgrading PTE permissions, see radix__ptep_set_access_flags.
//
// The return value of this function doesn't matter for hash,
// ptep_modify_prot_start() does a pte_update() which does or schedules
// any necessary hash table update and flush.
//
// We do not expect kernel mappings or non-PTEs or not-present PTEs.
//
// Must flush on any change except READ, WRITE, EXEC, DIRTY, ACCESSED.
//
// In theory, some changed software bits could be tolerated, in
// practice those should rarely if ever matter.
//
// If any of the above was present in old but cleared in new, flush.
// With the exception of _PAGE_ACCESSED, don't worry about flushing
// if that was cleared (see the comment in ptep_clear_flush_young()).
//
extern "C" {
    pub fn __pte_flags_need_flush(_arg: pte_val(oldpte), _arg: pte_val(newpte)) -> return;
}

extern "C" {
    pub fn __pte_flags_need_flush(_arg: pmd_val(oldpmd), _arg: pmd_val(newpmd)) -> return;
}

