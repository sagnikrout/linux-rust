//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/smp.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// SMP initialisation and IPI support
// Based on arch/arm64/kernel/smp.c
//
// Copyright (C) 2012 ARM Ltd.
// Copyright (C) 2015 Regents of the University of California
// Copyright (C) 2017 SiFive
//

    enum ipi_message_type {
    IPI_RESCHEDULE,
    IPI_CALL_FUNC,
    IPI_CPU_STOP,
    IPI_CPU_CRASH_STOP,
    IPI_IRQ_WORK,
    IPI_TIMER,
    IPI_CPU_BACKTRACE,
    IPI_KGDB_ROUNDUP,
    IPI_MAX
    };
    static const char * const ipi_names[] = {
    [IPI_RESCHEDULE]	= "Rescheduling interrupts",
    [IPI_CALL_FUNC]		= "Function call interrupts",
    [IPI_CPU_STOP]		= "CPU stop interrupts",
    [IPI_CPU_CRASH_STOP]	= "CPU stop (for crash dump) interrupts",
    [IPI_IRQ_WORK]		= "IRQ work interrupts",
    [IPI_TIMER]		= "Timer broadcast interrupts",
    [IPI_CPU_BACKTRACE]     = "CPU backtrace interrupts",
    [IPI_KGDB_ROUNDUP]	= "KGDB roundup interrupts",
    };
    unsigned long __cpuid_to_hartid_map[NR_CPUS] __ro_after_init = {
    [0 ... NR_CPUS-1] = INVALID_HARTID
    };
    EXPORT_SYMBOL_GPL(__cpuid_to_hartid_map);
#[no_mangle]
pub unsafe extern "C" fn smp_setup_processor_id() -> void __init {
    void __init smp_setup_processor_id(void)
    {
    cpuid_to_hartid_map(0) = boot_cpu_hartid;
    pr_info("Booting Linux on hartid %lu\n", boot_cpu_hartid);
    }
    static DEFINE_PER_CPU_READ_MOSTLY(int, ipi_dummy_dev);
    static int ipi_virq_base __ro_after_init;
    let mut __ro_after_init: static int nr_ipi = IPI_MAX;
    static struct irq_desc *ipi_desc[IPI_MAX] __read_mostly;
#[no_mangle]
pub unsafe extern "C" fn riscv_hartid_to_cpuid(hartid: c_ulong) -> c_int {
    int riscv_hartid_to_cpuid(unsigned long hartid)
    {
    int i;
    for (i = 0; i < NR_CPUS; i++)
    if (cpuid_to_hartid_map(i) == hartid)
    return i;
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn ipi_stop() {
    static void ipi_stop(void)
    {
    set_cpu_online(smp_processor_id(), false);
    while (1)
    wait_for_interrupt();
    }

    let mut waiting_for_crash_ipi: static atomic_t = ATOMIC_INIT(0);
#[no_mangle]
pub unsafe extern "C" fn ipi_cpu_crash_stop(cpu: c_uint, regs: *mut pt_regs) {
    static inline void ipi_cpu_crash_stop(unsigned int cpu, struct pt_regs *regs)
    {
    crash_save_cpu(regs, cpu);
    atomic_dec(&waiting_for_crash_ipi);
    local_irq_disable();

    if (cpu_has_hotplug(cpu))
    cpu_ops.cpu_stop();

    for(;;)
    wait_for_interrupt();
    }

#[no_mangle]
pub unsafe extern "C" fn ipi_cpu_crash_stop(cpu: c_uint, regs: *mut pt_regs) {
    static inline void ipi_cpu_crash_stop(unsigned int cpu, struct pt_regs *regs)
    {
    unreachable();
    }

#[no_mangle]
unsafe extern "C" fn send_ipi_mask(mask: *const cpumask, op: enum ipi_message_type) {
    static void send_ipi_mask(const struct cpumask *mask, enum ipi_message_type op)
    {
    __ipi_send_mask(ipi_desc[op], mask);
    }
#[no_mangle]
unsafe extern "C" fn send_ipi_single(cpu: c_int, op: enum ipi_message_type) {
    static void send_ipi_single(int cpu, enum ipi_message_type op)
    {
    __ipi_send_mask(ipi_desc[op], cpumask_of(cpu));
    }

#[no_mangle]
pub unsafe extern "C" fn arch_irq_work_raise() {
    void arch_irq_work_raise(void)
    {
    send_ipi_single(smp_processor_id(), IPI_IRQ_WORK);
    }

#[no_mangle]
unsafe extern "C" fn handle_IPI(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t handle_IPI(int irq, void *data)
    {
    let mut cpu: c_uint = smp_processor_id();
    let mut ipi: c_int = irq - ipi_virq_base;
    switch (ipi) {
    case IPI_RESCHEDULE:
    scheduler_ipi();
    break;
    case IPI_CALL_FUNC:
    generic_smp_call_function_interrupt();
    break;
    case IPI_CPU_STOP:
    ipi_stop();
    break;
    case IPI_CPU_CRASH_STOP:
    ipi_cpu_crash_stop(cpu, get_irq_regs());
    break;
    case IPI_IRQ_WORK:
    irq_work_run();
    break;

    case IPI_TIMER:
    tick_receive_broadcast();
    break;

    case IPI_CPU_BACKTRACE:
    nmi_cpu_backtrace(get_irq_regs());
    break;
    case IPI_KGDB_ROUNDUP:
    kgdb_nmicallback(cpu, get_irq_regs());
    break;
    default:
    pr_warn("CPU%d: unhandled IPI%d\n", cpu, ipi);
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_ipi_enable() {
    void riscv_ipi_enable(void)
    {
    int i;
    if (WARN_ON_ONCE(!ipi_virq_base))
    return;
    for (i = 0; i < nr_ipi; i++)
    enable_percpu_irq(ipi_virq_base + i, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_ipi_disable() {
    void riscv_ipi_disable(void)
    {
    int i;
    if (WARN_ON_ONCE(!ipi_virq_base))
    return;
    for (i = 0; i < nr_ipi; i++)
    disable_percpu_irq(ipi_virq_base + i);
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_ipi_have_virq_range() -> bool {
    bool riscv_ipi_have_virq_range(void)
    {
    return (ipi_virq_base) ? true : false;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_ipi_set_virq_range(virq: c_int, nr: c_int) {
    void riscv_ipi_set_virq_range(int virq, int nr)
    {
    int i, err;
    if (WARN_ON(ipi_virq_base))
    return;
    WARN_ON(nr < IPI_MAX);
    nr_ipi = min(nr, IPI_MAX);
    ipi_virq_base = virq;
// Request IPIs
    for (i = 0; i < nr_ipi; i++) {
    err = request_percpu_irq(ipi_virq_base + i, handle_IPI,
    ipi_names[i], &ipi_dummy_dev);
    WARN_ON(err);
    ipi_desc[i] = irq_to_desc(ipi_virq_base + i);
    irq_set_status_flags(ipi_virq_base + i, IRQ_HIDDEN);
    }
// Enabled IPIs for boot CPU immediately
    riscv_ipi_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn show_ipi_stats(p: *mut seq_file, prec: c_int) {
    void show_ipi_stats(struct seq_file *p, int prec)
    {
    unsigned int cpu, i;
    for (i = 0; i < IPI_MAX; i++) {
    seq_printf(p, "%*s%u:", prec - 1, "IPI", i);
    for_each_online_cpu(cpu)
    seq_printf(p, "%10u ", irq_desc_kstat_cpu(ipi_desc[i], cpu));
    seq_printf(p, " %s\n", ipi_names[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *mut cpumask) {
    void arch_send_call_function_ipi_mask(struct cpumask *mask)
    {
    send_ipi_mask(mask, IPI_CALL_FUNC);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: c_int) {
    void arch_send_call_function_single_ipi(int cpu)
    {
    send_ipi_single(cpu, IPI_CALL_FUNC);
    }

#[no_mangle]
pub unsafe extern "C" fn tick_broadcast(mask: *const cpumask) {
    void tick_broadcast(const struct cpumask *mask)
    {
    send_ipi_mask(mask, IPI_TIMER);
    }

#[no_mangle]
pub unsafe extern "C" fn smp_send_stop() {
    void smp_send_stop(void)
    {
    unsigned long timeout;
    if (num_online_cpus() > 1) {
    cpumask_t mask;
    cpumask_copy(&mask, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mask);
    if (system_state <= SYSTEM_RUNNING)
    pr_crit("SMP: stopping secondary CPUs\n");
    send_ipi_mask(&mask, IPI_CPU_STOP);
    }
// Wait up to one second for other CPUs to stop
    timeout = USEC_PER_SEC;
    while (num_online_cpus() > 1 && timeout--)
    udelay(1);
    if (num_online_cpus() > 1)
    pr_warn("SMP: failed to stop secondary CPUs %*pbl\n",
    cpumask_pr_args(cpu_online_mask));
    }

//
// The number of CPUs online, not counting this CPU (which may not be
// fully online and so not counted in num_online_cpus()).
//
#[no_mangle]
pub unsafe extern "C" fn num_other_online_cpus() -> c_uint {
    static inline unsigned int num_other_online_cpus(void)
    {
    let mut this_cpu_online: c_uint = cpu_online(smp_processor_id());
    return num_online_cpus() - this_cpu_online;
    }
#[no_mangle]
pub unsafe extern "C" fn crash_smp_send_stop() {
    void crash_smp_send_stop(void)
    {
    static int cpus_stopped;
    cpumask_t mask;
    unsigned long timeout;
//
// This function can be called twice in panic path, but obviously
// we execute this only once.
//
    if (cpus_stopped)
    return;
    cpus_stopped = 1;
//
// If this cpu is the only one alive at this point in time, online or
// not, there are no stop messages to be sent around, so just back out.
//
    if (num_other_online_cpus() == 0)
    return;
    cpumask_copy(&mask, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mask);
    atomic_set(&waiting_for_crash_ipi, num_other_online_cpus());
    pr_crit("SMP: stopping secondary CPUs\n");
    send_ipi_mask(&mask, IPI_CPU_CRASH_STOP);
// Wait up to one second for other CPUs to stop
    timeout = USEC_PER_SEC;
    while ((atomic_read(&waiting_for_crash_ipi) > 0) && timeout--)
    udelay(1);
    if (atomic_read(&waiting_for_crash_ipi) > 0)
    pr_warn("SMP: failed to stop secondary CPUs %*pbl\n",
    cpumask_pr_args(&mask));
    }
#[no_mangle]
pub unsafe extern "C" fn smp_crash_stop_failed() -> bool {
    bool smp_crash_stop_failed(void)
    {
    return (atomic_read(&waiting_for_crash_ipi) > 0);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: c_int) {
    void arch_smp_send_reschedule(int cpu)
    {
    send_ipi_single(cpu, IPI_RESCHEDULE);
    }
    EXPORT_SYMBOL_GPL(arch_smp_send_reschedule);
#[no_mangle]
unsafe extern "C" fn riscv_backtrace_ipi(mask: *mut cpumask_t) {
    static void riscv_backtrace_ipi(cpumask_t *mask)
    {
    send_ipi_mask(mask, IPI_CPU_BACKTRACE);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: c_int) {
    void arch_trigger_cpumask_backtrace(const cpumask_t *mask, int exclude_cpu)
    {
    nmi_trigger_cpumask_backtrace(mask, exclude_cpu, riscv_backtrace_ipi);
    }

#[no_mangle]
pub unsafe extern "C" fn kgdb_roundup_cpus() {
    void kgdb_roundup_cpus(void)
    {
    let mut this_cpu: c_int = raw_smp_processor_id();
    int cpu;
    for_each_online_cpu(cpu) {
// No need to roundup ourselves
    if (cpu == this_cpu)
    continue;
    send_ipi_single(cpu, IPI_KGDB_ROUNDUP);
    }
    }
