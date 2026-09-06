//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/lib/code-patching.c
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
// Copyright 2008 Michael Ellerman, IBM Corporation.
//

#[no_mangle]
unsafe extern "C" fn __patch_mem(exec_addr: *mut c_void, val: c_ulong, patch_addr: *mut c_void, is_dword: bool) -> c_int {
    static int __patch_mem(void *exec_addr, unsigned long val, void *patch_addr, bool is_dword)
    {
    if (!IS_ENABLED(CONFIG_PPC64) || likely(!is_dword)) {
// For big endian correctness: plain address would use the wrong half
    let mut val32: u32 = val;
    __put_kernel_nofault(patch_addr, &val32, u32, failed);
    } else {
    __put_kernel_nofault(patch_addr, &val, u64, failed);
    }
    asm ("dcbst 0, %0; sync; icbi 0,%1; sync; isync" :: "r" (patch_addr),
    "r" (exec_addr));
    return 0;
    failed:
    mb();  /* sync */
    return -EPERM;
    }
#[no_mangle]
pub unsafe extern "C" fn raw_patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> c_int {
    int raw_patch_instruction(u32 *addr, ppc_inst_t instr)
    {
    if (ppc_inst_prefixed(instr))
    return __patch_mem(addr, ppc_inst_as_ulong(instr), addr, true);
    else
    return __patch_mem(addr, ppc_inst_val(instr), addr, false);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct patch_context {
    union {
    pub area: *mut vm_struct,
    pub mm: *mut mm_struct,
}

    unsigned long addr;
    pte_t *pte;
    };
    static DEFINE_PER_CPU(struct patch_context, cpu_patching_context);
#[no_mangle]
unsafe extern "C" fn mm_patch_enabled() -> bool {
    static bool mm_patch_enabled(void)
    {
    return IS_ENABLED(CONFIG_SMP) && radix_enabled();
    }
//
// The following applies for Radix MMU. Hash MMU has different requirements,
// and so is not supported.
//
// Changing mm requires context synchronising instructions on both sides of
// the context switch, as well as a hwsync between the last instruction for
// which the address of an associated storage access was translated using
// the current context.
//
// switch_mm_irqs_off() performs an isync after the context switch. It is
// the responsibility of the caller to perform the CSI and hwsync before
// starting/stopping the temp mm.
//
    static struct mm_struct *start_using_temp_mm(struct mm_struct *temp_mm)
    {
    struct mm_struct *orig_mm = current.active_mm;
    lockdep_assert_irqs_disabled();
    switch_mm_irqs_off(orig_mm, temp_mm, current);
    WARN_ON(!mm_is_thread_local(temp_mm));
    suspend_breakpoints();
    return orig_mm;
    }
    static void stop_using_temp_mm(struct mm_struct *temp_mm,
    struct mm_struct *orig_mm)
    {
    lockdep_assert_irqs_disabled();
    switch_mm_irqs_off(temp_mm, orig_mm, current);
    restore_breakpoints();
    }
#[no_mangle]
unsafe extern "C" fn text_area_cpu_up(cpu: c_uint) -> c_int {
    static int text_area_cpu_up(unsigned int cpu)
    {
    struct vm_struct *area;
    unsigned long addr;
    int err;
    area = get_vm_area(PAGE_SIZE, 0);
    if (!area) {
    WARN_ONCE(1, "Failed to create text area for cpu %d\n",
    cpu);
    return -1;
    }
// Map/unmap the area to ensure all page tables are pre-allocated
    addr = (unsigned long)area.addr;
    err = map_kernel_page(addr, __pa_symbol(empty_zero_page), PAGE_KERNEL_RO);
    if (err)
    return err;
    unmap_kernel_page(addr);
    this_cpu_write(cpu_patching_context.area, area);
    this_cpu_write(cpu_patching_context.addr, addr);
    this_cpu_write(cpu_patching_context.pte, virt_to_kpte(addr));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn text_area_cpu_down(cpu: c_uint) -> c_int {
    static int text_area_cpu_down(unsigned int cpu)
    {
    free_vm_area(this_cpu_read(cpu_patching_context.area));
    this_cpu_write(cpu_patching_context.area, core::ptr::null_mut());
    this_cpu_write(cpu_patching_context.addr, 0);
    this_cpu_write(cpu_patching_context.pte, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn put_patching_mm(mm: *mut mm_struct, patching_addr: c_ulong) {
    static void put_patching_mm(struct mm_struct *mm, unsigned long patching_addr)
    {
    struct mmu_gather tlb;
    tlb_gather_mmu(&tlb, mm);
    free_pgd_range(&tlb, patching_addr, patching_addr + PAGE_SIZE, 0, 0);
    mmput(mm);
    }
#[no_mangle]
unsafe extern "C" fn text_area_cpu_up_mm(cpu: c_uint) -> c_int {
    static int text_area_cpu_up_mm(unsigned int cpu)
    {
    struct mm_struct *mm;
    unsigned long addr;
    pte_t *pte;
    spinlock_t *ptl;
    mm = mm_alloc();
    if (WARN_ON(!mm))
    goto fail_no_mm;
//
// Choose a random page-aligned address from the interval
// [PAGE_SIZE .. DEFAULT_MAP_WINDOW - PAGE_SIZE].
// The lower address bound is PAGE_SIZE to avoid the zero-page.
//
    addr = (1 + (get_random_long() % (DEFAULT_MAP_WINDOW / PAGE_SIZE - 2))) << PAGE_SHIFT;
//
// PTE allocation uses GFP_KERNEL which means we need to
// pre-allocate the PTE here because we cannot do the
// allocation during patching when IRQs are disabled.
//
// Using get_locked_pte() to avoid open coding, the lock
// is unnecessary.
//
    pte = get_locked_pte(mm, addr, &ptl);
    if (!pte)
    goto fail_no_pte;
    pte_unmap_unlock(pte, ptl);
    this_cpu_write(cpu_patching_context.mm, mm);
    this_cpu_write(cpu_patching_context.addr, addr);
    return 0;
    fail_no_pte:
    put_patching_mm(mm, addr);
    fail_no_mm:
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn text_area_cpu_down_mm(cpu: c_uint) -> c_int {
    static int text_area_cpu_down_mm(unsigned int cpu)
    {
    put_patching_mm(this_cpu_read(cpu_patching_context.mm),
    this_cpu_read(cpu_patching_context.addr));
    this_cpu_write(cpu_patching_context.mm, core::ptr::null_mut());
    this_cpu_write(cpu_patching_context.addr, 0);
    return 0;
    }
    static __ro_after_init DEFINE_STATIC_KEY_FALSE(poking_init_done);
#[no_mangle]
pub unsafe extern "C" fn poking_init() -> void __init {
    void __init poking_init(void)
    {
    int ret;
    if (mm_patch_enabled())
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN,
    "powerpc/text_poke_mm:online",
    text_area_cpu_up_mm,
    text_area_cpu_down_mm);
    else
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN,
    "powerpc/text_poke:online",
    text_area_cpu_up,
    text_area_cpu_down);
// cpuhp_setup_state returns >= 0 on success
    if (WARN_ON(ret < 0))
    return;
    static_branch_enable(&poking_init_done);
    }
#[no_mangle]
unsafe extern "C" fn get_patch_pfn(addr: *mut c_void) -> c_ulong {
    static unsigned long get_patch_pfn(void *addr)
    {
    if (IS_ENABLED(CONFIG_EXECMEM) && is_vmalloc_or_module_addr(addr))
    return vmalloc_to_pfn(addr);
    else
    return __pa_symbol(addr) >> PAGE_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn __do_patch_mem_mm(addr: *mut c_void, val: c_ulong, is_dword: bool) -> c_int {
    static int __do_patch_mem_mm(void *addr, unsigned long val, bool is_dword)
    {
    int err;
    u32 *patch_addr;
    unsigned long text_poke_addr;
    pte_t *pte;
    let mut pfn: c_ulong = get_patch_pfn(addr);
    struct mm_struct *patching_mm;
    struct mm_struct *orig_mm;
    spinlock_t *ptl;
    patching_mm = __this_cpu_read(cpu_patching_context.mm);
    text_poke_addr = __this_cpu_read(cpu_patching_context.addr);
    patch_addr = (u32 *)(text_poke_addr + offset_in_page(addr));
    pte = get_locked_pte(patching_mm, text_poke_addr, &ptl);
    if (!pte)
    return -ENOMEM;
    __set_pte_at(patching_mm, text_poke_addr, pte, pfn_pte(pfn, PAGE_KERNEL), 0);
// order PTE update before use, also serves as the hwsync
    asm volatile("ptesync": : :"memory");
// order context switch after arbitrary prior code
    isync();
    orig_mm = start_using_temp_mm(patching_mm);
    err = __patch_mem(addr, val, patch_addr, is_dword);
// context synchronisation performed by __patch_instruction (isync or exception)
    stop_using_temp_mm(patching_mm, orig_mm);
    pte_clear(patching_mm, text_poke_addr, pte);
//
// ptesync to order PTE update before TLB invalidation done
// by radix__local_flush_tlb_page_psize (in _tlbiel_va)
//
    local_flush_tlb_page_psize(patching_mm, text_poke_addr, mmu_virtual_psize);
    pte_unmap_unlock(pte, ptl);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __do_patch_mem(addr: *mut c_void, val: c_ulong, is_dword: bool) -> c_int {
    static int __do_patch_mem(void *addr, unsigned long val, bool is_dword)
    {
    int err;
    u32 *patch_addr;
    unsigned long text_poke_addr;
    pte_t *pte;
    let mut pfn: c_ulong = get_patch_pfn(addr);
    text_poke_addr = (unsigned long)__this_cpu_read(cpu_patching_context.addr) & PAGE_MASK;
    patch_addr = (u32 *)(text_poke_addr + offset_in_page(addr));
    pte = __this_cpu_read(cpu_patching_context.pte);
    __set_pte_at(&init_mm, text_poke_addr, pte, pfn_pte(pfn, PAGE_KERNEL), 0);
// See ptesync comment in radix__set_pte_at()
    if (radix_enabled())
    asm volatile("ptesync": : :"memory");
    err = __patch_mem(addr, val, patch_addr, is_dword);
    pte_clear(&init_mm, text_poke_addr, pte);
    flush_tlb_kernel_range(text_poke_addr, text_poke_addr + PAGE_SIZE);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn patch_mem(addr: *mut c_void, val: c_ulong, is_dword: bool) -> c_int {
    static int patch_mem(void *addr, unsigned long val, bool is_dword)
    {
    int err;
    unsigned long flags;
//
// During early early boot patch_instruction is called
// when text_poke_area is not ready, but we still need
// to allow patching. We just do the plain old patching
//
    if (!IS_ENABLED(CONFIG_STRICT_KERNEL_RWX) ||
    !static_branch_likely(&poking_init_done))
    return __patch_mem(addr, val, addr, is_dword);
    local_irq_save(flags);
    if (mm_patch_enabled())
    err = __do_patch_mem_mm(addr, val, is_dword);
    else
    err = __do_patch_mem(addr, val, is_dword);
    local_irq_restore(flags);
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> c_int {
    int patch_instruction(u32 *addr, ppc_inst_t instr)
    {
    if (ppc_inst_prefixed(instr))
    return patch_mem(addr, ppc_inst_as_ulong(instr), true);
    else
    return patch_mem(addr, ppc_inst_val(instr), false);
    }
    NOKPROBE_SYMBOL(patch_instruction);
#[no_mangle]
pub unsafe extern "C" fn patch_uint(addr: *mut c_void, val: c_uint) -> c_int {
    int patch_uint(void *addr, unsigned int val)
    {
    if (!IS_ALIGNED((unsigned long)addr, sizeof(unsigned int)))
    return -EINVAL;
    return patch_mem(addr, val, false);
    }
    NOKPROBE_SYMBOL(patch_uint);
#[no_mangle]
pub unsafe extern "C" fn patch_ulong(addr: *mut c_void, val: c_ulong) -> c_int {
    int patch_ulong(void *addr, unsigned long val)
    {
    if (!IS_ALIGNED((unsigned long)addr, sizeof(unsigned long)))
    return -EINVAL;
    return patch_mem(addr, val, true);
    }
    NOKPROBE_SYMBOL(patch_ulong);

#[no_mangle]
pub unsafe extern "C" fn patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> c_int {
    int patch_instruction(u32 *addr, ppc_inst_t instr)
    {
    return patch_mem(addr, ppc_inst_val(instr), false);
    }
    NOKPROBE_SYMBOL(patch_instruction)

#[no_mangle]
unsafe extern "C" fn patch_memset64(addr: *mut u64, val: u64, count: usize) -> c_int {
    static int patch_memset64(u64 *addr, u64 val, size_t count)
    {
    for (u64 *end = addr + count; addr < end; addr++)
    __put_kernel_nofault(addr, &val, u64, failed);
    return 0;
    failed:
    return -EPERM;
    }
#[no_mangle]
unsafe extern "C" fn patch_memset32(addr: *mut u32, val: u32, count: usize) -> c_int {
    static int patch_memset32(u32 *addr, u32 val, size_t count)
    {
    for (u32 *end = addr + count; addr < end; addr++)
    __put_kernel_nofault(addr, &val, u32, failed);
    return 0;
    failed:
    return -EPERM;
    }
#[no_mangle]
unsafe extern "C" fn __patch_instructions(patch_addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> c_int {
    static int __patch_instructions(u32 *patch_addr, u32 *code, size_t len, bool repeat_instr)
    {
    let mut start: c_ulong = (unsigned long)patch_addr;
    int err;
// Repeat instruction
    if (repeat_instr) {
    let mut instr: ppc_inst_t = ppc_inst_read(code);
    if (ppc_inst_prefixed(instr)) {
    let mut val: u64 = ppc_inst_as_ulong(instr);
    err = patch_memset64((u64 *)patch_addr, val, len / 8);
    } else {
    let mut val: u32 = ppc_inst_val(instr);
    err = patch_memset32(patch_addr, val, len / 4);
    }
    } else {
    err = copy_to_kernel_nofault(patch_addr, code, len);
    }
    smp_wmb();	/* smp write barrier */
    flush_icache_range(start, start + len);
    return err;
    }
//
// A page is mapped and instructions that fit the page are patched.
// Assumes 'len' to be (PAGE_SIZE - offset_in_page(addr)) or below.
//
#[no_mangle]
unsafe extern "C" fn __do_patch_instructions_mm(addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> c_int {
    static int __do_patch_instructions_mm(u32 *addr, u32 *code, size_t len, bool repeat_instr)
    {
    struct mm_struct *patching_mm, *orig_mm;
    let mut pfn: c_ulong = get_patch_pfn(addr);
    unsigned long text_poke_addr;
    spinlock_t *ptl;
    u32 *patch_addr;
    pte_t *pte;
    int err;
    patching_mm = __this_cpu_read(cpu_patching_context.mm);
    text_poke_addr = __this_cpu_read(cpu_patching_context.addr);
    patch_addr = (u32 *)(text_poke_addr + offset_in_page(addr));
    pte = get_locked_pte(patching_mm, text_poke_addr, &ptl);
    if (!pte)
    return -ENOMEM;
    __set_pte_at(patching_mm, text_poke_addr, pte, pfn_pte(pfn, PAGE_KERNEL), 0);
// order PTE update before use, also serves as the hwsync
    asm volatile("ptesync" ::: "memory");
// order context switch after arbitrary prior code
    isync();
    orig_mm = start_using_temp_mm(patching_mm);
    kasan_disable_current();
    err = __patch_instructions(patch_addr, code, len, repeat_instr);
    kasan_enable_current();
// context synchronisation performed by __patch_instructions
    stop_using_temp_mm(patching_mm, orig_mm);
    pte_clear(patching_mm, text_poke_addr, pte);
//
// ptesync to order PTE update before TLB invalidation done
// by radix__local_flush_tlb_page_psize (in _tlbiel_va)
//
    local_flush_tlb_page_psize(patching_mm, text_poke_addr, mmu_virtual_psize);
    pte_unmap_unlock(pte, ptl);
    return err;
    }
//
// A page is mapped and instructions that fit the page are patched.
// Assumes 'len' to be (PAGE_SIZE - offset_in_page(addr)) or below.
//
#[no_mangle]
unsafe extern "C" fn __do_patch_instructions(addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> c_int {
    static int __do_patch_instructions(u32 *addr, u32 *code, size_t len, bool repeat_instr)
    {
    let mut pfn: c_ulong = get_patch_pfn(addr);
    unsigned long text_poke_addr;
    u32 *patch_addr;
    pte_t *pte;
    int err;
    text_poke_addr = (unsigned long)__this_cpu_read(cpu_patching_context.addr) & PAGE_MASK;
    patch_addr = (u32 *)(text_poke_addr + offset_in_page(addr));
    pte = __this_cpu_read(cpu_patching_context.pte);
    __set_pte_at(&init_mm, text_poke_addr, pte, pfn_pte(pfn, PAGE_KERNEL), 0);
// See ptesync comment in radix__set_pte_at()
    if (radix_enabled())
    asm volatile("ptesync" ::: "memory");
    err = __patch_instructions(patch_addr, code, len, repeat_instr);
    pte_clear(&init_mm, text_poke_addr, pte);
    flush_tlb_kernel_range(text_poke_addr, text_poke_addr + PAGE_SIZE);
    return err;
    }
//
// Patch 'addr' with 'len' bytes of instructions from 'code'.
//
// If repeat_instr is true, the same instruction is filled for
// 'len' bytes.
//
#[no_mangle]
pub unsafe extern "C" fn patch_instructions(addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> c_int {
    int patch_instructions(u32 *addr, u32 *code, size_t len, bool repeat_instr)
    {
    while (len > 0) {
    unsigned long flags;
    size_t plen;
    int err;
    plen = min_t(size_t, PAGE_SIZE - offset_in_page(addr), len);
    local_irq_save(flags);
    if (mm_patch_enabled())
    err = __do_patch_instructions_mm(addr, code, plen, repeat_instr);
    else
    err = __do_patch_instructions(addr, code, plen, repeat_instr);
    local_irq_restore(flags);
    if (err)
    return err;
    len -= plen;
    addr = (u32 *)((unsigned long)addr + plen);
    if (!repeat_instr)
    code = (u32 *)((unsigned long)code + plen);
    }
    return 0;
    }
    NOKPROBE_SYMBOL(patch_instructions);
#[no_mangle]
pub unsafe extern "C" fn patch_branch(addr: *mut u32, target: c_ulong, flags: c_int) -> c_int {
    int patch_branch(u32 *addr, unsigned long target, int flags)
    {
    ppc_inst_t instr;
    if (create_branch(&instr, addr, target, flags))
    return -ERANGE;
    return patch_instruction(addr, instr);
    }
//
// Helper to check if a given instruction is a conditional branch
// Derived from the conditional checks in analyse_instr()
//
#[no_mangle]
pub unsafe extern "C" fn is_conditional_branch(instr: ppc_inst_t) -> bool {
    bool is_conditional_branch(ppc_inst_t instr)
    {
    let mut opcode: c_uint = ppc_inst_primary_opcode(instr);
    if (opcode == 16)       /* bc, bca, bcl, bcla */
    return true;
    if (opcode == 19) {
    switch ((ppc_inst_val(instr) >> 1) & 0x3ff) {
    case 16:        /* bclr, bclrl */
    case 528:       /* bcctr, bcctrl */
    case 560:       /* bctar, bctarl */
    return true;
    }
    }
    return false;
    }
    NOKPROBE_SYMBOL(is_conditional_branch);
    int create_cond_branch(ppc_inst_t *instr, const u32 *addr,
    unsigned long target, int flags)
    {
    long offset;
    offset = target;
    if (! (flags & BRANCH_ABSOLUTE))
    offset = offset - (unsigned long)addr;
// Check we can represent the target in the instruction format
    if (!is_offset_in_cond_branch_range(offset))
    return 1;
// Mask out the flags and target, so they don't step on each other.
// instr = ppc_inst(0x40000000 | (flags & 0x3FF0003) | (offset & 0xFFFC));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn instr_is_relative_branch(instr: ppc_inst_t) -> c_int {
    int instr_is_relative_branch(ppc_inst_t instr)
    {
    if (ppc_inst_val(instr) & BRANCH_ABSOLUTE)
    return 0;
    return instr_is_branch_iform(instr) || instr_is_branch_bform(instr);
    }
#[no_mangle]
pub unsafe extern "C" fn instr_is_relative_link_branch(instr: ppc_inst_t) -> c_int {
    int instr_is_relative_link_branch(ppc_inst_t instr)
    {
    return instr_is_relative_branch(instr) && (ppc_inst_val(instr) & BRANCH_SET_LINK);
    }
#[no_mangle]
unsafe extern "C" fn branch_iform_target(instr: *const u32) -> c_ulong {
    static unsigned long branch_iform_target(const u32 *instr)
    {
    signed long imm;
    imm = ppc_inst_val(ppc_inst_read(instr)) & 0x3FFFFFC;
// If the top bit of the immediate value is set this is negative
    if (imm & 0x2000000)
    imm -= 0x4000000;
    if ((ppc_inst_val(ppc_inst_read(instr)) & BRANCH_ABSOLUTE) == 0)
    imm += (unsigned long)instr;
    return (unsigned long)imm;
    }
#[no_mangle]
unsafe extern "C" fn branch_bform_target(instr: *const u32) -> c_ulong {
    static unsigned long branch_bform_target(const u32 *instr)
    {
    signed long imm;
    imm = ppc_inst_val(ppc_inst_read(instr)) & 0xFFFC;
// If the top bit of the immediate value is set this is negative
    if (imm & 0x8000)
    imm -= 0x10000;
    if ((ppc_inst_val(ppc_inst_read(instr)) & BRANCH_ABSOLUTE) == 0)
    imm += (unsigned long)instr;
    return (unsigned long)imm;
    }
#[no_mangle]
pub unsafe extern "C" fn branch_target(instr: *const u32) -> c_ulong {
    unsigned long branch_target(const u32 *instr)
    {
    if (instr_is_branch_iform(ppc_inst_read(instr)))
    return branch_iform_target(instr);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: instr_is_branch_bform(ppc_inst_read(instr))) -> else {
    else if (instr_is_branch_bform(ppc_inst_read(instr)))
    return branch_bform_target(instr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn translate_branch(instr: *mut ppc_inst_t, dest: *const u32, src: *const u32) -> c_int {
    int translate_branch(ppc_inst_t *instr, const u32 *dest, const u32 *src)
    {
    unsigned long target;
    target = branch_target(src);
    if (instr_is_branch_iform(ppc_inst_read(src)))
    return create_branch(instr, dest, target,
    ppc_inst_val(ppc_inst_read(src)));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: instr_is_branch_bform(ppc_inst_read(src))) -> else {
    else if (instr_is_branch_bform(ppc_inst_read(src)))
    return create_cond_branch(instr, dest, target,
    ppc_inst_val(ppc_inst_read(src)));
    return 1;
    }
