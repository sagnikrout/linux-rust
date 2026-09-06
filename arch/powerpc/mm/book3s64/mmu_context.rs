//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s64/mmu_context.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// MMU context allocation for 64-bit kernels.
//
// Copyright (C) 2004 Anton Blanchard, IBM Corp. <anton@samba.org>
//

    static DEFINE_IDA(mmu_context_ida);
#[no_mangle]
unsafe extern "C" fn alloc_context_id(min_id: c_int, max_id: c_int) -> c_int {
    static int alloc_context_id(int min_id, int max_id)
    {
    return ida_alloc_range(&mmu_context_ida, min_id, max_id, GFP_KERNEL);
    }

#[no_mangle]
pub unsafe extern "C" fn hash__reserve_context_id(id: c_int) -> void __init {
    void __init hash__reserve_context_id(int id)
    {
    let mut result: c_int = ida_alloc_range(&mmu_context_ida, id, id, GFP_KERNEL);
    WARN(result != id, "mmu: Failed to reserve context id %d (rc %d)\n", id, result);
    }
#[no_mangle]
pub unsafe extern "C" fn hash__alloc_context_id() -> c_int {
    int hash__alloc_context_id(void)
    {
    unsigned long max;
    if (mmu_has_feature(MMU_FTR_68_BIT_VA))
    max = MAX_USER_CONTEXT;
    else
    max = MAX_USER_CONTEXT_65BIT_VA;
    return alloc_context_id(MIN_USER_CONTEXT, max);
    }
    EXPORT_SYMBOL_GPL(hash__alloc_context_id);

#[no_mangle]
unsafe extern "C" fn realloc_context_ids(ctx: *mut mm_context_t) -> c_int {
    static int realloc_context_ids(mm_context_t *ctx)
    {
    int i, id;
//
// id 0 (aka. ctx->id) is special, we always allocate a new one, even if
// there wasn't one allocated previously (which happens in the exec
// case where ctx is newly allocated).
//
// We have to be a bit careful here. We must keep the existing ids in
// the array, so that we can test if they're non-zero to decide if we
// need to allocate a new one. However in case of error we must free the
// ids we've allocated but *not* any of the existing ones (or risk a
// UAF). That's why we decrement i at the start of the error handling
// loop, to skip the id that we just tested but couldn't reallocate.
//
    for (i = 0; i < ARRAY_SIZE(ctx.extended_id); i++) {
    if (i == 0 || ctx.extended_id[i]) {
    id = hash__alloc_context_id();
    if (id < 0)
    goto error;
    ctx.extended_id[i] = id;
    }
    }
// The caller expects us to return id
    return ctx.id;
    error:
    for (i--; i >= 0; i--) {
    if (ctx.extended_id[i])
    ida_free(&mmu_context_ida, ctx.extended_id[i]);
    }
    return id;
    }
#[no_mangle]
unsafe extern "C" fn hash__init_new_context(mm: *mut mm_struct) -> c_int {
    static int hash__init_new_context(struct mm_struct *mm)
    {
    int index;
    mm.context.hash_context = kmalloc_obj(struct hash_mm_context);
    if (!mm.context.hash_context)
    return -ENOMEM;
//
// The old code would re-promote on fork, we don't do that when using
// slices as it could cause problem promoting slices that have been
// forced down to 4K.
//
// For book3s we have MMU_NO_CONTEXT set to be ~0. Hence check
// explicitly against context.id == 0. This ensures that we properly
// initialize context slice details for newly allocated mm's (which will
// have id == 0) and don't alter context slice inherited via fork (which
// will have id != 0).
//
// We should not be calling init_new_context() on init_mm. Hence a
// check against 0 is OK.
//
    if (mm.context.id == 0) {
    memset(mm.context.hash_context, 0, sizeof(struct hash_mm_context));
    slice_init_new_context_exec(mm);
    } else {
// This is fork. Copy hash_context details from current->mm
    memcpy(mm.context.hash_context, current.mm.context.hash_context, sizeof(struct hash_mm_context));

// inherit subpage prot details if we have one.
    if (current.mm.context.hash_context.spt) {
    mm.context.hash_context.spt = kmalloc_obj(struct subpage_prot_table);
    if (!mm.context.hash_context.spt) {
    kfree(mm.context.hash_context);
    return -ENOMEM;
    }
    }

    }
    index = realloc_context_ids(&mm.context);
    if (index < 0) {

    kfree(mm.context.hash_context.spt);

    kfree(mm.context.hash_context);
    return index;
    }
    pkey_mm_init(mm);
    return index;
    }
#[no_mangle]
pub unsafe extern "C" fn hash__setup_new_exec() {
    void hash__setup_new_exec(void)
    {
    slice_setup_new_exec();
    }

#[no_mangle]
pub unsafe extern "C" fn hash__init_new_context(mm: *mut mm_struct) -> c_int {
    static inline int hash__init_new_context(struct mm_struct *mm)
    {
    BUILD_BUG();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn radix__init_new_context(mm: *mut mm_struct) -> c_int {
    static int radix__init_new_context(struct mm_struct *mm)
    {
    unsigned long rts_field;
    int index, max_id;
    max_id = (1 << mmu_pid_bits) - 1;
    index = alloc_context_id(mmu_base_pid, max_id);
    if (index < 0)
    return index;
//
// set the process table entry,
//
    rts_field = radix__get_tree_size();
    process_tb[index].prtb0 = cpu_to_be64(rts_field | __pa(mm.pgd) | RADIX_PGD_INDEX_SIZE);
//
// Order the above store with subsequent update of the PID
// register (at which point HW can start loading/caching
// the entry) and the corresponding load by the MMU from
// the L2 cache.
//
    asm volatile("ptesync;isync" : : : "memory");

    mm.context.hash_context = core::ptr::null_mut();

    return index;
    }
#[no_mangle]
pub unsafe extern "C" fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> c_int {
    int init_new_context(struct task_struct *tsk, struct mm_struct *mm)
    {
    int index;
    if (radix_enabled())
    index = radix__init_new_context(mm);
    else
    index = hash__init_new_context(mm);
    if (index < 0)
    return index;
    mm.context.id = index;
    mm.context.pte_frag = core::ptr::null_mut();
    mm.context.pmd_frag = core::ptr::null_mut();

    mm_iommu_init(mm);

    atomic_set(&mm.context.active_cpus, 0);
    atomic_set(&mm.context.copros, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __destroy_context(context_id: c_int) {
    void __destroy_context(int context_id)
    {
    ida_free(&mmu_context_ida, context_id);
    }
    EXPORT_SYMBOL_GPL(__destroy_context);
#[no_mangle]
unsafe extern "C" fn destroy_contexts(ctx: *mut mm_context_t) {
    static void destroy_contexts(mm_context_t *ctx)
    {
    if (radix_enabled()) {
    ida_free(&mmu_context_ida, ctx.id);
    } else {

    int index, context_id;
    for (index = 0; index < ARRAY_SIZE(ctx.extended_id); index++) {
    context_id = ctx.extended_id[index];
    if (context_id)
    ida_free(&mmu_context_ida, context_id);
    }
    kfree(ctx.hash_context);

    BUILD_BUG(); // radix_enabled() should be constant true

    }
    }
#[no_mangle]
unsafe extern "C" fn pmd_frag_destroy(pmd_frag: *mut c_void) {
    static void pmd_frag_destroy(void *pmd_frag)
    {
    int count;
    struct ptdesc *ptdesc;
    ptdesc = virt_to_ptdesc(pmd_frag);
// drop all the pending references
    count = ((unsigned long)pmd_frag & ~PAGE_MASK) >> PMD_FRAG_SIZE_SHIFT;
// We allow PTE_FRAG_NR fragments from a PTE page
    if (atomic_sub_and_test(PMD_FRAG_NR - count, &ptdesc.pt_frag_refcount)) {
    pagetable_dtor(ptdesc);
    pagetable_free(ptdesc);
    }
    }
#[no_mangle]
unsafe extern "C" fn destroy_pagetable_cache(mm: *mut mm_struct) {
    static void destroy_pagetable_cache(struct mm_struct *mm)
    {
    void *frag;
    frag = mm.context.pte_frag;
    if (frag)
    pte_frag_destroy(frag);
    frag = mm.context.pmd_frag;
    if (frag)
    pmd_frag_destroy(frag);
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn destroy_context(mm: *mut mm_struct) {
    void destroy_context(struct mm_struct *mm)
    {

    WARN_ON_ONCE(!list_empty(&mm.context.iommu_group_mem_list));

//
// For tasks which were successfully initialized we end up calling
// arch_exit_mmap() which clears the process table entry. And
// arch_exit_mmap() is called before the required fullmm TLB flush
// which does a RIC=2 flush. Hence for an initialized task, we do clear
// any cached process table entries.
//
// The condition below handles the error case during task init. We have
// set the process table entry early and if we fail a task
// initialization, we need to ensure the process table entry is zeroed.
// We need not worry about process table entry caches because the task
// never ran with the PID value.
//
    if (radix_enabled())
    process_tb[mm.context.id].prtb0 = 0;
    else
    subpage_prot_free(mm);
    destroy_contexts(&mm.context);
    mm.context.id = MMU_NO_CONTEXT;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_exit_mmap(mm: *mut mm_struct) {
    void arch_exit_mmap(struct mm_struct *mm)
    {
    destroy_pagetable_cache(mm);
    if (radix_enabled()) {
//
// Radix doesn't have a valid bit in the process table
// entries. However we know that at least P9 implementation
// will avoid caching an entry with an invalid RTS field,
// and 0 is invalid. So this will do.
//
// This runs before the "fullmm" tlb flush in exit_mmap,
// which does a RIC=2 tlbie to clear the process table
// entry. See the "fullmm" comments in tlb-radix.c.
//
// No barrier required here after the store because
// this process will do the invalidate, which starts with
// ptesync.
//
    process_tb[mm.context.id].prtb0 = 0;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn radix__switch_mmu_context(prev: *mut mm_struct, next: *mut mm_struct) {
    void radix__switch_mmu_context(struct mm_struct *prev, struct mm_struct *next)
    {
    mtspr(SPRN_PID, next.context.id);
    isync();
    }

//
// cleanup_cpu_mmu_context - Clean up MMU details for this CPU (newly offlined)
//
// This clears the CPU from mm_cpumask for all processes, and then flushes the
// local TLB to ensure TLB coherency in case the CPU is onlined again.
//
// KVM guest translations are not necessarily flushed here. If KVM started
// using mm_cpumask or the Linux APIs which do, this would have to be resolved.
//

#[no_mangle]
pub unsafe extern "C" fn cleanup_cpu_mmu_context() {
    void cleanup_cpu_mmu_context(void)
    {
    let mut cpu: c_int = smp_processor_id();
    clear_tasks_mm_cpumask(cpu);
    tlbiel_all();
    }
