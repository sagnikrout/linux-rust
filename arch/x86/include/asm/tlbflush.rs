//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/tlbflush.h
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
    pub fn __flush_tlb_all();
}

pub const TLB_GENERATION_INVALID: c_int = 0;
extern "C" {
    pub fn cr4_update_irqsoff(set: c_ulong, clear: c_ulong);
}
extern "C" {
    pub fn cr4_read_shadow() -> c_ulong;
}
// Set in this cpu's CR4.
// Clear in this cpu's CR4.
// Set in this cpu's CR4.
// Clear in this cpu's CR4.

//
// 6 because 6 should be plenty and struct tlb_state will fit in two cache
// lines.
//
pub const TLB_NR_DYN_ASIDS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlb_context {
    pub ctx_id: u64,
    pub tlb_gen: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlb_state {
//
// cpu_tlbstate.loaded_mm should match CR3 whenever interrupts
// are on.  This means that it may not match current->active_mm,
// which will contain the previous user mm when we're in lazy TLB
// mode even if we've already switched back to swapper_pg_dir.
//
// During switch_mm_irqs_off(), loaded_mm will be set to
// LOADED_MM_SWITCHING during the brief interrupts-off window
// when CR3 and loaded_mm would otherwise be inconsistent.  This
// is for nmi_uaccess_okay()'s benefit.
//
    pub loaded_mm: *mut mm_struct,

// Last user mm for optimizing IBPB
    pub last_user_mm: *mut mm_struct,
    pub last_user_mm_spec: c_ulong,
}

//
// If set we changed the page tables in such a way that we
// needed an invalidation of all contexts (aka. PCIDs / ASIDs).
// This tells us to go invalidate all the non-loaded ctxs[]
// on the next context switch.
//
// The current ctx was kept up-to-date as it ran and does not
// need to be invalidated.
//

//
// Active LAM mode.
//
// X86_CR3_LAM_U57/U48 shifted right by X86_CR3_LAM_U57_BIT or 0 if LAM
// disabled.
//

//
// Mask that contains TLB_NR_DYN_ASIDS+1 bits to indicate
// the corresponding user PCID needs a flush next time we
// switch to it; see SWITCH_TO_USER_CR3.
//
// Access to this CR4 shadow and to H/W CR4 is protected by
// disabling interrupts when modifying either one.
//
// This is a list of all contexts that might exist in the TLB.
// There is one per ASID that we use, and the ASID (what the
// CPU calls PCID) is the index into ctxts.
//
// For each context, ctx_id indicates which mm the TLB's user
// entries came from.  As an invariant, the TLB will never
// contain entries that are out-of-date as when that mm reached
// the tlb_gen in the list.
//
// To be clear, this means that it's legal for the TLB code to
// flush the TLB without updating tlb_gen.  This can happen
// (for now, at least) due to paravirt remote flushes.
//
// NB: context 0 is a bit special, since it's also used by
// various bits of init code.  This is fine -- code that
// isn't aware of PCID will end up harmlessly flushing
// context 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlb_state_shared {
//
// We can be in one of several states:
//
// - Actively using an mm.  Our CPU's bit will be set in
// mm_cpumask(loaded_mm) and is_lazy == false;
//
// - Not using a real mm.  loaded_mm == &init_mm.  Our CPU's bit
// will not be set in mm_cpumask(&init_mm) and is_lazy == false.
//
// - Lazily using a real mm.  loaded_mm != &init_mm, our bit
// is set in mm_cpumask(loaded_mm), but is_lazy == true.
// We're heuristically guessing that the CR3 load we
// skipped more than makes up for the overhead added by
// lazy mode.
//
    pub is_lazy: bool,
}

//
// Please ignore the name of this function.  It should be called
// switch_to_kernel_thread().
//
// enter_lazy_tlb() is a hint from the scheduler that we are entering a
// kernel thread or other context without an mm.  Acceptable implementations
// include doing nothing whatsoever, switching to init_mm, or various clever
// lazy tricks to try to minimize TLB flushes.
//
// The scheduler reserves the right to call enter_lazy_tlb() several times
// in a row.  It will notify us that we're going back to a real mm by
// calling switch_mm_irqs_off().
//

extern "C" {
    pub fn nmi_uaccess_okay() -> bool;
}

// Initialize cr4 shadow for this CPU.
// How many pages can be invalidated with one INVLPGB.
extern "C" {
    pub fn initialize_tlbstate_and_flush();
}
//
// Keep stack-allocated flush_tlb_info cacheline aligned, but cap the
// alignment to avoid excessive stack usage on large-cacheline systems.
//

//
// TLB flushing:
//
// - flush_tlb_all() flushes all processes TLBs
// - flush_tlb_mm(mm) flushes the specified mm context TLB's
// - flush_tlb_page(vma, vmaddr) flushes one page
// - flush_tlb_range(vma, start, end) flushes a range of pages
// - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
// - flush_tlb_multi(cpumask, info) flushes TLBs on multiple cpus
//
// ..but the i386 has somewhat limited tlb flushing capabilities,
// and page-granular flushes are available only on i486 and up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flush_tlb_info {
//
// We support several kinds of flushes.
//
// - Fully flush a single mm.  .mm will be set, .end will be
// TLB_FLUSH_ALL, and .new_tlb_gen will be the tlb_gen to
// which the IPI sender is trying to catch us up.
//
// - Partially flush a single mm.  .mm will be set, .start and
// .end will indicate the range, and .new_tlb_gen will be set
// such that the changes between generation .new_tlb_gen-1 and
// .new_tlb_gen are entirely contained in the indicated range.
//
// - Fully flush all mms whose tlb_gens have been updated.  .mm
// will be NULL, .end will be TLB_FLUSH_ALL, and .new_tlb_gen
// will be zero.
//
    pub mm: *mut mm_struct,
    pub start: c_ulong,
    pub end: c_ulong,
    pub new_tlb_gen: u64,
    pub initiating_cpu: c_uint,
    pub stride_shift: u8,
    pub freed_tables: u8,
    pub trim_cpumask: u8,
    pub __aligned(FLUSH_TLB_INFO_ALIGN): },
    pub flush_tlb_local(void): c_void,
    pub addr): void flush_tlb_one_user(unsigned long,
    pub addr): void flush_tlb_one_kernel(unsigned long,
    pub info): *const flush_tlb_info,
    pub TLB_NR_DYN_ASIDS: return asid <,
    pub !is_dyn_asid(asid): return,

    pub asid: u16,
    pub 0: return,
    pub smp_load_acquire(&mm->context.global_asid): asid =,
// mm->context.global_asid is either 0, or a global ASID
    pub is_dyn_asid(asid)): VM_WARN_ON_ONCE(asid &&,
    pub asid: return,
    pub 0: mm->context.global_asid =,
    pub false: mm->context.asid_transition =,
//
// Notably flush_tlb_mm_range() -> broadcast_tlb_flush() ->
// finish_asid_transition() needs to observe asid_transition = true
// once it observes global_asid.
//
    pub true: mm->context.asid_transition =,
    pub asid): smp_store_release(&mm->context.global_asid,,
    pub false): WRITE_ONCE(mm->context.asid_transition,,
    pub false: return,
    pub READ_ONCE(mm->context.asid_transition): return mm &&,
    pub mm): *mut extern void mm_free_global_asid(struct mm_struct,

    pub }: *mut *mut static inline u16 mm_global_asid(struct mm_struct mm) { return 0;,
    pub }: *mut *mut static inline bool mm_in_asid_transition(struct mm_struct mm) { return false;,

    pub flush_tlb_all(void): extern void,
    pub freed_tables): bool,
    pub end): extern void flush_tlb_kernel_range(unsigned long start, unsigned long,
    pub false): flush_tlb_mm_range(vma->vm_mm, a, a + PAGE_SIZE, PAGE_SHIFT,,
    pub false: bool should_defer =,
// If remote CPUs need to be flushed then defer batch the flush
    pub true: should_defer =,
    pub should_defer: return,
//
// Bump the generation count.  This also serves as a full barrier
// that synchronizes with switch_mm(): callers are required to order
// their read of mm_cpumask after their writes to the paging
// structures.
//
    pub atomic64_inc_return(&mm->context.tlb_gen): return,
    pub mm_cpumask(mm)): cpumask_or(&batch->cpumask, &batch->cpumask,,
    pub true: batch->unmapped_pages =,
    pub -1UL): mmu_notifier_arch_invalidate_secondary_tlbs(mm, 0,,
    pub batch): *mut extern void arch_tlbbatch_flush(struct arch_tlbflush_unmap_batch,
//
// Flags that require a flush when cleared but not when they are set.
// Only include flags that would not trigger spurious page-faults.
// Non-present entries are not cached. Hardware would set the
// dirty/access bit if needed without a fault.
//
    pub _PAGE_NX: _PAGE_PKEY_BIT2 | _PAGE_PKEY_BIT3 |,
    pub newflags: unsigned long diff = oldflags ^,
    pub software_flags): BUILD_BUG_ON(flush_on_clear &,
    pub flush_on_change): BUILD_BUG_ON(flush_on_clear &,
    pub software_flags): BUILD_BUG_ON(flush_on_change &,
// Ignore software flags
    pub ~software_flags: diff &=,
    pub ~_PAGE_ACCESSED: diff &=,
//
// Did any of the 'flush_on_clear' flags was clleared set from between
// 'oldflags' and 'newflags'?
//
    pub true: return,
// Flush on modified flags.
    pub true: return,
// Ensure there are no flags that were left behind
    pub true: return,
    pub false: return,
//
// pte_needs_flush() checks whether permissions were demoted and require a
// flush. It should only be used for userspace PTEs.
//
// !PRESENT -> * ; no need for flush
    pub false: return,
// PFN changed ; needs flush
    pub true: return,
//
// check PTE flags; ignore access-bit; see comment in
// ptep_clear_flush_young().
//

//
// huge_pmd_needs_flush() checks whether permissions were demoted and require a
// flush. It should only be used for userspace huge PMDs.
//
// !PRESENT -> * ; no need for flush
    pub false: return,
// PFN changed ; needs flush
    pub true: return,
//
// check PMD flags; do not ignore access-bit; see
// pmdp_clear_flush_young().
//

    pub this_cpu_read(cpu_tlbstate.lam): u64 lam =,
    pub X86_CR3_LAM_U57_BIT: return lam <<,
    pub X86_CR3_LAM_U57_BIT): this_cpu_write(cpu_tlbstate.lam, lam >>,
    pub untag_mask): this_cpu_write(tlbstate_untag_mask,,

    pub 0: return,

    pub modules"): __compiletime_error("enter_lazy_tlb() should not be used in,

    pub X86_CR4_PGE): native_write_cr4(cr4 ^,
