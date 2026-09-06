//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/ipi.c
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

    DEFINE_STATIC_KEY_FALSE(apic_use_ipi_shorthand);

    static int apic_ipi_shorthand_off __ro_after_init;
#[no_mangle]
unsafe extern "C" fn apic_ipi_shorthand(str: *mut c_char) -> __init int {
    static __init int apic_ipi_shorthand(char *str)
    {
    get_option(&str, &apic_ipi_shorthand_off);
    return 1;
    }
    __setup("no_ipi_broadcast=", apic_ipi_shorthand);
#[no_mangle]
unsafe extern "C" fn print_ipi_mode() -> int __init {
    static int __init print_ipi_mode(void)
    {
    pr_info("IPI shorthand broadcast: %s\n",
    str_disabled_enabled(apic_ipi_shorthand_off));
    return 0;
    }
    late_initcall(print_ipi_mode);
#[no_mangle]
pub unsafe extern "C" fn apic_smt_update() {
    void apic_smt_update(void)
    {
//
// Do not switch to broadcast mode if:
// - Disabled on the command line
// - Only a single CPU is online
// - Not all present CPUs have been at least booted once
//
// The latter is important as the local APIC might be in some
// random state and a broadcast might cause havoc. That's
// especially true for NMI broadcasting.
//
    if (apic_ipi_shorthand_off || num_online_cpus() == 1 ||
    !cpumask_equal(cpu_present_mask, &cpus_booted_once_mask)) {
    static_branch_disable(&apic_use_ipi_shorthand);
    } else {
    static_branch_enable(&apic_use_ipi_shorthand);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn apic_send_IPI_allbutself(vector: c_uint) {
    void apic_send_IPI_allbutself(unsigned int vector)
    {
    if (num_online_cpus() < 2)
    return;
    if (static_branch_likely(&apic_use_ipi_shorthand))
    __apic_send_IPI_allbutself(vector);
    else
    __apic_send_IPI_mask_allbutself(cpu_online_mask, vector);
    }
//
// Send a 'reschedule' IPI to another CPU. It goes straight through and
// wastes no time serializing anything. Worst case is that we lose a
// reschedule ...
//
#[no_mangle]
pub unsafe extern "C" fn native_smp_send_reschedule(cpu: c_int) {
    void native_smp_send_reschedule(int cpu)
    {
    if (unlikely(cpu_is_offline(cpu))) {
    WARN(1, "sched: Unexpected reschedule of offline CPU#%d!\n", cpu);
    return;
    }
    __apic_send_IPI(cpu, RESCHEDULE_VECTOR);
    }
#[no_mangle]
pub unsafe extern "C" fn native_send_call_func_single_ipi(cpu: c_int) {
    void native_send_call_func_single_ipi(int cpu)
    {
    __apic_send_IPI(cpu, CALL_FUNCTION_SINGLE_VECTOR);
    }
#[no_mangle]
pub unsafe extern "C" fn native_send_call_func_ipi(mask: *const cpumask) {
    void native_send_call_func_ipi(const struct cpumask *mask)
    {
    if (static_branch_likely(&apic_use_ipi_shorthand)) {
    let mut cpu: c_uint = smp_processor_id();
    if (!cpumask_or_equal(mask, cpumask_of(cpu), cpu_online_mask))
    goto sendmask;
    if (cpumask_test_cpu(cpu, mask))
    __apic_send_IPI_all(CALL_FUNCTION_VECTOR);
#[no_mangle]
pub unsafe extern "C" fn if(1: num_online_cpus() >) -> else {
    else if (num_online_cpus() > 1)
    __apic_send_IPI_allbutself(CALL_FUNCTION_VECTOR);
    return;
    }
    sendmask:
    __apic_send_IPI_mask(mask, CALL_FUNCTION_VECTOR);
    }
#[no_mangle]
pub unsafe extern "C" fn apic_send_nmi_to_offline_cpu(cpu: c_uint) {
    void apic_send_nmi_to_offline_cpu(unsigned int cpu)
    {
    if (WARN_ON_ONCE(!apic.nmi_to_offline_cpu))
    return;
    if (WARN_ON_ONCE(!cpumask_test_cpu(cpu, &cpus_booted_once_mask)))
    return;
    apic.send_IPI(cpu, NMI_VECTOR);
    }

#[no_mangle]
pub unsafe extern "C" fn __prepare_ICR2(mask: c_uint) -> c_int {
    static inline int __prepare_ICR2(unsigned int mask)
    {
    return SET_XAPIC_DEST_FIELD(mask);
    }
#[no_mangle]
pub unsafe extern "C" fn apic_mem_wait_icr_idle_timeout() -> u32 {
    u32 apic_mem_wait_icr_idle_timeout(void)
    {
    int cnt;
    for (cnt = 0; cnt < 1000; cnt++) {
    if (!(apic_read(APIC_ICR) & APIC_ICR_BUSY))
    return 0;
    irq_stat_inc_and_enable(IRQ_COUNT_ICR_READ_RETRY);
    udelay(100);
    }
    return APIC_ICR_BUSY;
    }
#[no_mangle]
pub unsafe extern "C" fn apic_mem_wait_icr_idle() {
    void apic_mem_wait_icr_idle(void)
    {
    while (native_apic_mem_read(APIC_ICR) & APIC_ICR_BUSY)
    cpu_relax();
    }
//
// This is safe against interruption because it only writes the lower 32
// bits of the APIC_ICR register. The destination field is ignored for
// short hand IPIs.
//
// wait_icr_idle()
// write(ICR2, dest)
// NMI
// wait_icr_idle()
// write(ICR)
// wait_icr_idle()
// write(ICR)
//
// This function does not need to disable interrupts as there is no ICR2
// interaction. The memory write is direct except when the machine is
// affected by the 11AP Pentium erratum, which turns the plain write into
// an XCHG operation.
//
#[no_mangle]
unsafe extern "C" fn __default_send_IPI_shortcut(shortcut: c_uint, vector: c_int) {
    static void __default_send_IPI_shortcut(unsigned int shortcut, int vector)
    {
//
// Wait for the previous ICR command to complete.  Use
// safe_apic_wait_icr_idle() for the NMI vector as there have been
// issues where otherwise the system hangs when the panic CPU tries
// to stop the others before launching the kdump kernel.
//
    if (unlikely(vector == NMI_VECTOR))
    apic_mem_wait_icr_idle_timeout();
    else
    apic_mem_wait_icr_idle();
// Destination field (ICR2) and the destination mode are ignored
    native_apic_mem_write(APIC_ICR, __prepare_ICR(shortcut, vector, 0));
    }
//
// This is used to send an IPI with no shorthand notation (the destination is
// specified in bits 56 to 63 of the ICR).
//
    void __default_send_IPI_dest_field(unsigned int dest_mask, int vector,
    unsigned int dest_mode)
    {
// See comment in __default_send_IPI_shortcut()
    if (unlikely(vector == NMI_VECTOR))
    apic_mem_wait_icr_idle_timeout();
    else
    apic_mem_wait_icr_idle();
// Set the IPI destination field in the ICR
    native_apic_mem_write(APIC_ICR2, __prepare_ICR2(dest_mask));
// Send it with the proper destination mode
    native_apic_mem_write(APIC_ICR, __prepare_ICR(0, vector, dest_mode));
    }
#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_single_phys(cpu: c_int, vector: c_int) {
    void default_send_IPI_single_phys(int cpu, int vector)
    {
    unsigned long flags;
    local_irq_save(flags);
    __default_send_IPI_dest_field(per_cpu(x86_cpu_to_apicid, cpu),
    vector, APIC_DEST_PHYSICAL);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_mask_sequence_phys(mask: *const cpumask, vector: c_int) {
    void default_send_IPI_mask_sequence_phys(const struct cpumask *mask, int vector)
    {
    unsigned long flags;
    unsigned long cpu;
    local_irq_save(flags);
    for_each_cpu(cpu, mask) {
    __default_send_IPI_dest_field(per_cpu(x86_cpu_to_apicid,
    cpu), vector, APIC_DEST_PHYSICAL);
    }
    local_irq_restore(flags);
    }
    void default_send_IPI_mask_allbutself_phys(const struct cpumask *mask,
    int vector)
    {
    unsigned int cpu, this_cpu = smp_processor_id();
    unsigned long flags;
    local_irq_save(flags);
    for_each_cpu(cpu, mask) {
    if (cpu == this_cpu)
    continue;
    __default_send_IPI_dest_field(per_cpu(x86_cpu_to_apicid,
    cpu), vector, APIC_DEST_PHYSICAL);
    }
    local_irq_restore(flags);
    }
//
// Helper function for APICs which insist on cpumasks
//
#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_single(cpu: c_int, vector: c_int) {
    void default_send_IPI_single(int cpu, int vector)
    {
    __apic_send_IPI_mask(cpumask_of(cpu), vector);
    }
#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_allbutself(vector: c_int) {
    void default_send_IPI_allbutself(int vector)
    {
    __default_send_IPI_shortcut(APIC_DEST_ALLBUT, vector);
    }
#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_all(vector: c_int) {
    void default_send_IPI_all(int vector)
    {
    __default_send_IPI_shortcut(APIC_DEST_ALLINC, vector);
    }
#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_self(vector: c_int) {
    void default_send_IPI_self(int vector)
    {
    __default_send_IPI_shortcut(APIC_DEST_SELF, vector);
    }

#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_mask_sequence_logical(mask: *const cpumask, vector: c_int) {
    void default_send_IPI_mask_sequence_logical(const struct cpumask *mask, int vector)
    {
    unsigned long flags;
    unsigned int cpu;
    local_irq_save(flags);
    for_each_cpu(cpu, mask)
    __default_send_IPI_dest_field(1U << cpu, vector, APIC_DEST_LOGICAL);
    local_irq_restore(flags);
    }
    void default_send_IPI_mask_allbutself_logical(const struct cpumask *mask,
    int vector)
    {
    unsigned int cpu, this_cpu = smp_processor_id();
    unsigned long flags;
    local_irq_save(flags);
    for_each_cpu(cpu, mask) {
    if (cpu == this_cpu)
    continue;
    __default_send_IPI_dest_field(1U << cpu, vector, APIC_DEST_LOGICAL);
    }
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn default_send_IPI_mask_logical(cpumask: *const cpumask, vector: c_int) {
    void default_send_IPI_mask_logical(const struct cpumask *cpumask, int vector)
    {
    let mut mask: c_ulong = cpumask_bits(cpumask)[0];
    unsigned long flags;
    if (!mask)
    return;
    local_irq_save(flags);
    WARN_ON(mask & ~cpumask_bits(cpu_online_mask)[0]);
    __default_send_IPI_dest_field(mask, vector, APIC_DEST_LOGICAL);
    local_irq_restore(flags);
    }
