//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/irq.c
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
// Derived from arch/i386/kernel/irq.c
// Copyright (C) 1992 Linus Torvalds
// Adapted from arch/i386 by Gary Thomas
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
// Updated and modified by Cort Dougan <cort@fsmlabs.com>
// Copyright (C) 1996-2001 Cort Dougan
// Adapted for Power Macintosh by Paul Mackerras
// Copyright (C) 1996 Paul Mackerras (paulus@cs.anu.edu.au)
//
// This file contains the code used by various IRQ handling routines:
// asking for different IRQ's should be done through these routines
// instead of just grabbing them. Thus setups with different IRQ numbers
// shouldn't result in any weird surprises, and installing new handlers
// should be easier.
//
// The MPC8xx has an interrupt mask in the SIU.  If a bit is set, the
// interrupt is _enabled_.  As expected, IRQ0 is bit 0 in the 32-bit
// mask register (of which only 16 are defined), hence the weird shifting
// and complement of the cached_irq_mask.  I want to be able to stuff
// this right into the SIU SMASK register.
// Many of the prep/chrp functions are conditional compiled on CONFIG_PPC_8xx
// to reduce code space and undefined function references.
//

// Macro flag: #define CREATE_TRACE_POINTS

    DEFINE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat);
    EXPORT_PER_CPU_SYMBOL(irq_stat);

    atomic_t ppc_n_lost_interrupts;

    extern int tau_initialized;
    u32 tau_interrupts(unsigned long cpu);

#[no_mangle]
pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: c_int) -> c_int {
    int arch_show_interrupts(struct seq_file *p, int prec)
    {
    int j;

    if (tau_initialized) {
    seq_printf(p, "%*s:", prec, "TAU");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", tau_interrupts(j), 10);
    seq_puts(p, "  PowerPC             Thermal Assist (cpu temp)\n");
    }

    seq_printf(p, "%*s:", prec, "LOC");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).timer_irqs_event, 10);
    seq_printf(p, "  Local timer interrupts for timer event device\n");
    seq_printf(p, "%*s:", prec, "BCT");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).broadcast_irqs_event, 10);
    seq_printf(p, "  Broadcast timer interrupts for timer event device\n");
    seq_printf(p, "%*s:", prec, "LOC");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).timer_irqs_others, 10);
    seq_printf(p, "  Local timer interrupts for others\n");
    seq_printf(p, "%*s:", prec, "SPU");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).spurious_irqs, 10);
    seq_printf(p, "  Spurious interrupts\n");
    seq_printf(p, "%*s:", prec, "PMI");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).pmu_irqs, 10);
    seq_printf(p, "  Performance monitoring interrupts\n");
    seq_printf(p, "%*s:", prec, "MCE");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).mce_exceptions, 10);
    seq_printf(p, "  Machine check exceptions\n");

    if (cpu_has_feature(CPU_FTR_HVMODE)) {
    seq_printf(p, "%*s:", prec, "HMI");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", paca_ptrs[j].hmi_irqs, 10);
    seq_printf(p, "  Hypervisor Maintenance Interrupts\n");
    }

    seq_printf(p, "%*s:", prec, "NMI");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).sreset_irqs, 10);
    seq_printf(p, "  System Reset interrupts\n");

    seq_printf(p, "%*s:", prec, "WDG");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).soft_nmi_irqs, 10);
    seq_printf(p, "  Watchdog soft-NMI interrupts\n");

    if (cpu_has_feature(CPU_FTR_DBELL)) {
    seq_printf(p, "%*s:", prec, "DBL");
    for_each_online_cpu(j)
    seq_put_decimal_ull_width(p, " ", per_cpu(irq_stat, j).doorbell_irqs, 10);
    seq_printf(p, "  Doorbell interrupts\n");
    }

    return 0;
    }
//
// /proc/stat helpers
//
#[no_mangle]
pub unsafe extern "C" fn arch_irq_stat_cpu(cpu: c_uint) -> u64 {
    u64 arch_irq_stat_cpu(unsigned int cpu)
    {
    let mut sum: u64 = per_cpu(irq_stat, cpu).timer_irqs_event;
    sum += per_cpu(irq_stat, cpu).broadcast_irqs_event;
    sum += per_cpu(irq_stat, cpu).pmu_irqs;
    sum += per_cpu(irq_stat, cpu).mce_exceptions;
    sum += per_cpu(irq_stat, cpu).spurious_irqs;
    sum += per_cpu(irq_stat, cpu).timer_irqs_others;

    sum += paca_ptrs[cpu].hmi_irqs;

    sum += per_cpu(irq_stat, cpu).sreset_irqs;

    sum += per_cpu(irq_stat, cpu).soft_nmi_irqs;

    sum += per_cpu(irq_stat, cpu).doorbell_irqs;

    return sum;
    }
#[no_mangle]
pub unsafe extern "C" fn check_stack_overflow(sp: c_ulong) {
    static inline void check_stack_overflow(unsigned long sp)
    {
    if (!IS_ENABLED(CONFIG_DEBUG_STACKOVERFLOW))
    return;
    sp &= THREAD_SIZE - 1;
// check for stack overflow: is there less than 1/4th free?
    if (unlikely(sp < THREAD_SIZE / 4)) {
    pr_err("do_IRQ: stack overflow: %ld\n", sp);
    dump_stack();
    }
    }

#[no_mangle]
unsafe extern "C" fn call_do_softirq(sp: *const c_void) -> __always_inline void {
    static __always_inline void call_do_softirq(const void *sp)
    {
// Temporarily switch r1 to sp, call __do_softirq() then restore r1.
    asm volatile (
    PPC_STLU "	%%r1, %[offset](%[sp])	;"
    "mr		%%r1, %[sp]		;"

    "bl		%[callee]@notoc		;"

    "bl		%[callee]		;"

    PPC_LL "	%%r1, 0(%%r1)		;"
    : // Outputs
    : // Inputs
    [sp] "b" (sp), [offset] "i" (THREAD_SIZE - STACK_FRAME_MIN_SIZE),
    [callee] "i" (__do_softirq)
    : // Clobbers
    "lr", "xer", "ctr", "memory", "cr0", "cr1", "cr5", "cr6", "cr7", "r0",
// r2 may be clobbered by the callee when using PCREL mode in the ELFv2 ABI.

    "r2",

    "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10",
    "r11", "r12"
    );
    }

    DEFINE_STATIC_CALL_RET0(ppc_get_irq, *ppc_md.get_irq);
#[no_mangle]
unsafe extern "C" fn __do_irq(regs: *mut pt_regs, oldsp: c_ulong) {
    static void __do_irq(struct pt_regs *regs, unsigned long oldsp)
    {
    unsigned int irq;
    trace_irq_entry(regs);
    check_stack_overflow(oldsp);
//
// Query the platform PIC for the interrupt & ack it.
//
// This will typically lower the interrupt line to the CPU
//
    irq = static_call(ppc_get_irq)();
// We can hard enable interrupts now to allow perf interrupts
    if (should_hard_irq_enable(regs))
    do_hard_irq_enable();
// And finally process it
    if (unlikely(!irq))
    __this_cpu_inc(irq_stat.spurious_irqs);
    else
    generic_handle_irq(irq);
    trace_irq_exit(regs);
    }
#[no_mangle]
unsafe extern "C" fn call_do_irq(regs: *mut pt_regs, sp: *mut c_void) -> __always_inline void {
    static __always_inline void call_do_irq(struct pt_regs *regs, void *sp)
    {
    register unsigned long r3 asm("r3") = (unsigned long)regs;
// Temporarily switch r1 to sp, call __do_irq() then restore r1.
    asm volatile (
    PPC_STLU "	%%r1, %[offset](%[sp])	;"
    "mr		%%r4, %%r1		;"
    "mr		%%r1, %[sp]		;"

    "bl		%[callee]@notoc		;"

    "bl		%[callee]		;"

    PPC_LL "	%%r1, 0(%%r1)		;"
    : // Outputs
    "+r" (r3)
    : // Inputs
    [sp] "b" (sp), [offset] "i" (THREAD_SIZE - STACK_FRAME_MIN_SIZE),
    [callee] "i" (__do_irq)
    : // Clobbers
    "lr", "xer", "ctr", "memory", "cr0", "cr1", "cr5", "cr6", "cr7", "r0",
// r2 may be clobbered by the callee when using PCREL mode in the ELFv2 ABI.

    "r2",

    "r4", "r5", "r6", "r7", "r8", "r9", "r10",
    "r11", "r12"
    );
    }
#[no_mangle]
pub unsafe extern "C" fn __do_IRQ(regs: *mut pt_regs) {
    void __do_IRQ(struct pt_regs *regs)
    {
    struct pt_regs *old_regs = set_irq_regs(regs);
    void *cursp, *irqsp;
// Switch to the irq stack to handle this
    cursp = (void *)(current_stack_pointer & ~(THREAD_SIZE - 1));
    irqsp = hardirq_ctx[raw_smp_processor_id()];
// Already there ? If not switch stack and call
    if (unlikely(cursp == irqsp))
    __do_irq(regs, current_stack_pointer);
    else
    call_do_irq(regs, irqsp);
    set_irq_regs(old_regs);
    }
    DEFINE_INTERRUPT_HANDLER_ASYNC(do_IRQ)
    {
    __do_IRQ(regs);
    }
#[no_mangle]
unsafe extern "C" fn alloc_vm_stack() -> *mut void __init {
    static void *__init alloc_vm_stack(void)
    {
    return __vmalloc_node(THREAD_SIZE, THREAD_ALIGN, THREADINFO_GFP,
    NUMA_NO_NODE, (void *)_RET_IP_);
    }
#[no_mangle]
unsafe extern "C" fn vmap_irqstack_init() -> void __init {
    static void __init vmap_irqstack_init(void)
    {
    int i;
    for_each_possible_cpu(i) {
    softirq_ctx[i] = alloc_vm_stack();
    hardirq_ctx[i] = alloc_vm_stack();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn init_IRQ() -> void __init {
    void __init init_IRQ(void)
    {
    if (IS_ENABLED(CONFIG_VMAP_STACK))
    vmap_irqstack_init();
    if (ppc_md.init_IRQ)
    ppc_md.init_IRQ();
    if (!WARN_ON(!ppc_md.get_irq))
    static_call_update(ppc_get_irq, ppc_md.get_irq);
    }

    void   *critirq_ctx[NR_CPUS] __read_mostly;
    void    *dbgirq_ctx[NR_CPUS] __read_mostly;
    void *mcheckirq_ctx[NR_CPUS] __read_mostly;

    void *softirq_ctx[NR_CPUS] __read_mostly;
    void *hardirq_ctx[NR_CPUS] __read_mostly;

#[no_mangle]
pub unsafe extern "C" fn do_softirq_own_stack() {
    void do_softirq_own_stack(void)
    {
    call_do_softirq(softirq_ctx[smp_processor_id()]);
    }

#[no_mangle]
pub unsafe extern "C" fn virq_to_hw(virq: c_uint) -> irq_hw_number_t {
    irq_hw_number_t virq_to_hw(unsigned int virq)
    {
    struct irq_data *irq_data = irq_get_irq_data(virq);
    return WARN_ON(!irq_data) ? 0 : irq_data.hwirq;
    }
    EXPORT_SYMBOL_GPL(virq_to_hw);

#[no_mangle]
pub unsafe extern "C" fn irq_choose_cpu(mask: *const cpumask) -> c_int {
    int irq_choose_cpu(const struct cpumask *mask)
    {
    int cpuid;
    if (cpumask_equal(mask, cpu_online_mask)) {
    static int irq_rover;
    static DEFINE_RAW_SPINLOCK(irq_rover_lock);
    unsigned long flags;
// Round-robin distribution...
    do_round_robin:
    raw_spin_lock_irqsave(&irq_rover_lock, flags);
    irq_rover = cpumask_next_wrap(irq_rover, cpu_online_mask);
    cpuid = irq_rover;
    raw_spin_unlock_irqrestore(&irq_rover_lock, flags);
    } else {
    cpuid = cpumask_first_and(mask, cpu_online_mask);
    if (cpuid >= nr_cpu_ids)
    goto do_round_robin;
    }
    return get_hard_smp_processor_id(cpuid);
    }

#[no_mangle]
pub unsafe extern "C" fn irq_choose_cpu(mask: *const cpumask) -> c_int {
    int irq_choose_cpu(const struct cpumask *mask)
    {
    return hard_smp_processor_id();
    }
