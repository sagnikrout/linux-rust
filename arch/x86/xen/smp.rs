//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/smp.c
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

    static DEFINE_PER_CPU(struct xen_common_irq, xen_resched_irq) = { .irq = -1 };
    static DEFINE_PER_CPU(struct xen_common_irq, xen_callfunc_irq) = { .irq = -1 };
    static DEFINE_PER_CPU(struct xen_common_irq, xen_callfuncsingle_irq) = { .irq = -1 };
    static DEFINE_PER_CPU(struct xen_common_irq, xen_debug_irq) = { .irq = -1 };
    static irqreturn_t xen_call_function_interrupt(int irq, void *dev_id);
    static irqreturn_t xen_call_function_single_interrupt(int irq, void *dev_id);
//
// Reschedule call back.
//
#[no_mangle]
unsafe extern "C" fn xen_reschedule_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xen_reschedule_interrupt(int irq, void *dev_id)
    {
    inc_irq_stat(RESCHEDULE);
    scheduler_ipi();
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_intr_free(cpu: c_uint) {
    void xen_smp_intr_free(unsigned int cpu)
    {
    kfree(per_cpu(xen_resched_irq, cpu).name);
    per_cpu(xen_resched_irq, cpu).name = core::ptr::null_mut();
    if (per_cpu(xen_resched_irq, cpu).irq >= 0) {
    unbind_from_irqhandler(per_cpu(xen_resched_irq, cpu).irq, core::ptr::null_mut());
    per_cpu(xen_resched_irq, cpu).irq = -1;
    }
    kfree(per_cpu(xen_callfunc_irq, cpu).name);
    per_cpu(xen_callfunc_irq, cpu).name = core::ptr::null_mut();
    if (per_cpu(xen_callfunc_irq, cpu).irq >= 0) {
    unbind_from_irqhandler(per_cpu(xen_callfunc_irq, cpu).irq, core::ptr::null_mut());
    per_cpu(xen_callfunc_irq, cpu).irq = -1;
    }
    kfree(per_cpu(xen_debug_irq, cpu).name);
    per_cpu(xen_debug_irq, cpu).name = core::ptr::null_mut();
    if (per_cpu(xen_debug_irq, cpu).irq >= 0) {
    unbind_from_irqhandler(per_cpu(xen_debug_irq, cpu).irq, core::ptr::null_mut());
    per_cpu(xen_debug_irq, cpu).irq = -1;
    }
    kfree(per_cpu(xen_callfuncsingle_irq, cpu).name);
    per_cpu(xen_callfuncsingle_irq, cpu).name = core::ptr::null_mut();
    if (per_cpu(xen_callfuncsingle_irq, cpu).irq >= 0) {
    unbind_from_irqhandler(per_cpu(xen_callfuncsingle_irq, cpu).irq,
    core::ptr::null_mut());
    per_cpu(xen_callfuncsingle_irq, cpu).irq = -1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_intr_init(cpu: c_uint) -> c_int {
    int xen_smp_intr_init(unsigned int cpu)
    {
    int rc;
    char *resched_name, *callfunc_name, *debug_name;
    resched_name = kasprintf(GFP_KERNEL, "resched%d", cpu);
    if (!resched_name)
    goto fail_mem;
    per_cpu(xen_resched_irq, cpu).name = resched_name;
    rc = bind_ipi_to_irqhandler(XEN_RESCHEDULE_VECTOR,
    cpu,
    xen_reschedule_interrupt,
    IRQF_PERCPU|IRQF_NOBALANCING,
    resched_name,
    core::ptr::null_mut());
    if (rc < 0)
    goto fail;
    per_cpu(xen_resched_irq, cpu).irq = rc;
    callfunc_name = kasprintf(GFP_KERNEL, "callfunc%d", cpu);
    if (!callfunc_name)
    goto fail_mem;
    per_cpu(xen_callfunc_irq, cpu).name = callfunc_name;
    rc = bind_ipi_to_irqhandler(XEN_CALL_FUNCTION_VECTOR,
    cpu,
    xen_call_function_interrupt,
    IRQF_PERCPU|IRQF_NOBALANCING,
    callfunc_name,
    core::ptr::null_mut());
    if (rc < 0)
    goto fail;
    per_cpu(xen_callfunc_irq, cpu).irq = rc;
    if (!xen_fifo_events) {
    debug_name = kasprintf(GFP_KERNEL, "debug%d", cpu);
    if (!debug_name)
    goto fail_mem;
    per_cpu(xen_debug_irq, cpu).name = debug_name;
    rc = bind_virq_to_irqhandler(VIRQ_DEBUG, cpu,
    xen_debug_interrupt,
    IRQF_PERCPU | IRQF_NOBALANCING,
    debug_name, core::ptr::null_mut());
    if (rc < 0)
    goto fail;
    per_cpu(xen_debug_irq, cpu).irq = rc;
    }
    callfunc_name = kasprintf(GFP_KERNEL, "callfuncsingle%d", cpu);
    if (!callfunc_name)
    goto fail_mem;
    per_cpu(xen_callfuncsingle_irq, cpu).name = callfunc_name;
    rc = bind_ipi_to_irqhandler(XEN_CALL_FUNCTION_SINGLE_VECTOR,
    cpu,
    xen_call_function_single_interrupt,
    IRQF_PERCPU|IRQF_NOBALANCING,
    callfunc_name,
    core::ptr::null_mut());
    if (rc < 0)
    goto fail;
    per_cpu(xen_callfuncsingle_irq, cpu).irq = rc;
    return 0;
    fail_mem:
    rc = -ENOMEM;
    fail:
    xen_smp_intr_free(cpu);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_cpus_done(max_cpus: c_uint) -> void __init {
    void __init xen_smp_cpus_done(unsigned int max_cpus)
    {
    if (xen_hvm_domain())
    native_smp_cpus_done(max_cpus);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_send_reschedule(cpu: c_int) {
    void xen_smp_send_reschedule(int cpu)
    {
    xen_send_IPI_one(cpu, XEN_RESCHEDULE_VECTOR);
    }
    static void __xen_send_IPI_mask(const struct cpumask *mask,
    int vector)
    {
    unsigned cpu;
    for_each_cpu_and(cpu, mask, cpu_online_mask)
    xen_send_IPI_one(cpu, vector);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_send_call_function_ipi(mask: *const cpumask) {
    void xen_smp_send_call_function_ipi(const struct cpumask *mask)
    {
    int cpu;
    __xen_send_IPI_mask(mask, XEN_CALL_FUNCTION_VECTOR);
// Make sure other vcpus get a chance to run if they need to.
    for_each_cpu(cpu, mask) {
    if (xen_vcpu_stolen(cpu)) {
    HYPERVISOR_sched_op(SCHEDOP_yield, core::ptr::null_mut());
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_send_call_function_single_ipi(cpu: c_int) {
    void xen_smp_send_call_function_single_ipi(int cpu)
    {
    __xen_send_IPI_mask(cpumask_of(cpu),
    XEN_CALL_FUNCTION_SINGLE_VECTOR);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_map_vector(vector: c_int) -> c_int {
    static inline int xen_map_vector(int vector)
    {
    int xen_vector;
    switch (vector) {
    case RESCHEDULE_VECTOR:
    xen_vector = XEN_RESCHEDULE_VECTOR;
    break;
    case CALL_FUNCTION_VECTOR:
    xen_vector = XEN_CALL_FUNCTION_VECTOR;
    break;
    case CALL_FUNCTION_SINGLE_VECTOR:
    xen_vector = XEN_CALL_FUNCTION_SINGLE_VECTOR;
    break;
    case IRQ_WORK_VECTOR:
    xen_vector = XEN_IRQ_WORK_VECTOR;
    break;

    case NMI_VECTOR:
    case APIC_DM_NMI: /* Some use that instead of NMI_VECTOR */
    xen_vector = XEN_NMI_VECTOR;
    break;

    default:
    xen_vector = -1;
    printk(KERN_ERR "xen: vector 0x%x is not implemented\n",
    vector);
    }
    return xen_vector;
    }
    void xen_send_IPI_mask(const struct cpumask *mask,
    int vector)
    {
    let mut xen_vector: c_int = xen_map_vector(vector);
    if (xen_vector >= 0)
    __xen_send_IPI_mask(mask, xen_vector);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_send_IPI_all(vector: c_int) {
    void xen_send_IPI_all(int vector)
    {
    let mut xen_vector: c_int = xen_map_vector(vector);
    if (xen_vector >= 0)
    __xen_send_IPI_mask(cpu_online_mask, xen_vector);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_send_IPI_self(vector: c_int) {
    void xen_send_IPI_self(int vector)
    {
    let mut xen_vector: c_int = xen_map_vector(vector);
    if (xen_vector >= 0)
    xen_send_IPI_one(smp_processor_id(), xen_vector);
    }
    void xen_send_IPI_mask_allbutself(const struct cpumask *mask,
    int vector)
    {
    unsigned cpu;
    let mut this_cpu: c_uint = smp_processor_id();
    let mut xen_vector: c_int = xen_map_vector(vector);
    if (!(num_online_cpus() > 1) || (xen_vector < 0))
    return;
    for_each_cpu_and(cpu, mask, cpu_online_mask) {
    if (this_cpu == cpu)
    continue;
    xen_send_IPI_one(cpu, xen_vector);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xen_send_IPI_allbutself(vector: c_int) {
    void xen_send_IPI_allbutself(int vector)
    {
    xen_send_IPI_mask_allbutself(cpu_online_mask, vector);
    }
#[no_mangle]
unsafe extern "C" fn xen_call_function_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xen_call_function_interrupt(int irq, void *dev_id)
    {
    generic_smp_call_function_interrupt();
    inc_irq_stat(CALL_FUNCTION);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn xen_call_function_single_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xen_call_function_single_interrupt(int irq, void *dev_id)
    {
    generic_smp_call_function_single_interrupt();
    inc_irq_stat(CALL_FUNCTION);
    return IRQ_HANDLED;
    }
