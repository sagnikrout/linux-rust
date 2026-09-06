//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/smp.c
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
// SMP support for pSeries machines.
//
// Dave Engebretsen, Peter Bergner, and
// Mike Corrigan {engebret|bergner|mikec}@us.ibm.com
//
// Plus various changes from other IBM teams...
//

//
// The Primary thread of each non-boot processor was started from the OF client
// interface by prom_hold_cpus and is spinning on secondary_hold_spinloop.
//
    static cpumask_var_t of_spin_mask;
// Query where a cpu is now.  Return codes #defined in plpar_wrappers.h
#[no_mangle]
pub unsafe extern "C" fn smp_query_cpu_stopped(pcpu: c_uint) -> c_int {
    int smp_query_cpu_stopped(unsigned int pcpu)
    {
    int cpu_status, status;
    let mut qcss_tok: c_int = rtas_function_token(RTAS_FN_QUERY_CPU_STOPPED_STATE);
    if (qcss_tok == RTAS_UNKNOWN_SERVICE) {
    printk_once(KERN_INFO
    "Firmware doesn't support query-cpu-stopped-state\n");
    return QCSS_HARDWARE_ERROR;
    }
    status = rtas_call(qcss_tok, 1, 2, &cpu_status, pcpu);
    if (status != 0) {
    printk(KERN_ERR
    "RTAS query-cpu-stopped-state failed: %i\n", status);
    return status;
    }
    return cpu_status;
    }
//
// smp_startup_cpu() - start the given cpu
//
// At boot time, there is nothing to do for primary threads which were
// started from Open Firmware.  For anything else, call RTAS with the
// appropriate start location.
//
// Returns:
// 0	- failure
// 1	- success
//
#[no_mangle]
pub unsafe extern "C" fn smp_startup_cpu(lcpu: c_uint) -> c_int {
    static inline int smp_startup_cpu(unsigned int lcpu)
    {
    int status;
    unsigned long start_here =
    __pa(ppc_function_entry(generic_secondary_smp_init));
    unsigned int pcpu;
    int start_cpu;
    if (cpumask_test_cpu(lcpu, of_spin_mask))
// Already started by OF and sitting in spin loop
    return 1;
    pcpu = get_hard_smp_processor_id(lcpu);
// Check to see if the CPU out of FW already for kexec
    if (smp_query_cpu_stopped(pcpu) == QCSS_NOT_STOPPED){
    cpumask_set_cpu(lcpu, of_spin_mask);
    return 1;
    }
//
// If the RTAS start-cpu token does not exist then presume the
// cpu is already spinning.
//
    start_cpu = rtas_function_token(RTAS_FN_START_CPU);
    if (start_cpu == RTAS_UNKNOWN_SERVICE)
    return 1;
    status = rtas_call(start_cpu, 3, 1, core::ptr::null_mut(), pcpu, start_here, pcpu);
    if (status != 0) {
    printk(KERN_ERR "start-cpu failed: %i\n", status);
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn smp_setup_cpu(cpu: c_int) {
    static void smp_setup_cpu(int cpu)
    {
    if (xive_enabled())
    xive_smp_setup_cpu();
#[no_mangle]
pub unsafe extern "C" fn if(boot_cpuid: cpu !=) -> else {
    else if (cpu != boot_cpuid)
    xics_setup_cpu();
//
// Initialize VPA on non-boot cpus since boot-cpu vpa was
// already initialized in pSeries_setup_arch()
//
    if (firmware_has_feature(FW_FEATURE_SPLPAR) &&
    cpu != boot_cpuid)
    vpa_init(cpu);
    cpumask_clear_cpu(cpu, of_spin_mask);
    }
#[no_mangle]
unsafe extern "C" fn smp_pSeries_kick_cpu(nr: c_int) -> c_int {
    static int smp_pSeries_kick_cpu(int nr)
    {
    if (nr < 0 || nr >= nr_cpu_ids)
    return -EINVAL;
    if (!smp_startup_cpu(nr))
    return -ENOENT;
//
// The processor is currently spinning, waiting for the
// cpu_start field to become non-zero After we set cpu_start,
// the processor will continue on to secondary_start
//
    paca_ptrs[nr].cpu_start = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pseries_smp_prepare_cpu(cpu: c_int) -> c_int {
    static int pseries_smp_prepare_cpu(int cpu)
    {
    if (xive_enabled())
    return xive_smp_prepare_cpu(cpu);
    return 0;
    }
// Cause IPI as setup by the interrupt controller (xics or xive)
    static void (*ic_cause_ipi)(int cpu) __ro_after_init;
// Use msgsndp doorbells target is a sibling, else use interrupt controller
#[no_mangle]
unsafe extern "C" fn dbell_or_ic_cause_ipi(cpu: c_int) {
    static void dbell_or_ic_cause_ipi(int cpu)
    {
    if (doorbell_try_core_ipi(cpu))
    return;
    ic_cause_ipi(cpu);
    }
#[no_mangle]
unsafe extern "C" fn pseries_cause_nmi_ipi(cpu: c_int) -> c_int {
    static int pseries_cause_nmi_ipi(int cpu)
    {
    int hwcpu;
    if (cpu == NMI_IPI_ALL_OTHERS) {
    hwcpu = H_SIGNAL_SYS_RESET_ALL_OTHERS;
    } else {
    if (cpu < 0) {
    WARN_ONCE(true, "incorrect cpu parameter %d", cpu);
    return 0;
    }
    hwcpu = get_hard_smp_processor_id(cpu);
    }
    if (plpar_signal_sys_reset(hwcpu) == H_SUCCESS)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pSeries_smp_probe() -> __init void {
    static __init void pSeries_smp_probe(void)
    {
    if (xive_enabled()) {
    if (xive_smp_probe() < 0)
    return;
    } else {
    xics_smp_probe();
    }
// No doorbell facility, must use the interrupt controller for IPIs
    if (!cpu_has_feature(CPU_FTR_DBELL))
    return;
// Doorbells can only be used for IPIs between SMT siblings
    if (!cpu_has_feature(CPU_FTR_SMT))
    return;
    check_kvm_guest();
    if (is_kvm_guest()) {
//
// KVM emulates doorbells by disabling FSCR[MSGP] so msgsndp
// faults to the hypervisor which then reads the instruction
// from guest memory, which tends to be slower than using XIVE.
//
    if (xive_enabled())
    return;
//
// XICS hcalls aren't as fast, so we can use msgsndp (which
// also helps exercise KVM emulation), however KVM can't
// emulate secure guests because it can't read the instruction
// out of their memory.
//
    if (is_secure_guest())
    return;
    }
//
// Under PowerVM, FSCR[MSGP] is enabled as guest vCPU siblings are
// gang scheduled on the same physical core, so doorbells are always
// faster than the interrupt controller, and they can be used by
// secure guests.
//
    ic_cause_ipi = smp_ops.cause_ipi;
    smp_ops.cause_ipi = dbell_or_ic_cause_ipi;
    }
    static struct smp_ops_t pseries_smp_ops = {
    .message_pass	= core::ptr::null_mut(),	/* Use smp_muxed_ipi_message_pass */
    .cause_ipi	= core::ptr::null_mut(),	/* Filled at runtime by pSeries_smp_probe() */
    .cause_nmi_ipi	= pseries_cause_nmi_ipi,
    .probe		= pSeries_smp_probe,
    .prepare_cpu	= pseries_smp_prepare_cpu,
    .kick_cpu	= smp_pSeries_kick_cpu,
    .setup_cpu	= smp_setup_cpu,
    .cpu_bootable	= smp_generic_cpu_bootable,
    };
// This is called very early
#[no_mangle]
pub unsafe extern "C" fn smp_init_pseries() -> void __init {
    void __init smp_init_pseries(void)
    {
    int i;
    pr_debug(" . smp_init_pSeries()\n");
    smp_ops = &pseries_smp_ops;
    alloc_bootmem_cpumask_var(&of_spin_mask);
//
// Mark threads which are still spinning in hold loops
//
// We know prom_init will not have started them if RTAS supports
// query-cpu-stopped-state.
//
    if (rtas_function_token(RTAS_FN_QUERY_CPU_STOPPED_STATE) == RTAS_UNKNOWN_SERVICE) {
    if (cpu_has_feature(CPU_FTR_SMT)) {
    for_each_present_cpu(i) {
    if (cpu_thread_in_core(i) == 0)
    cpumask_set_cpu(i, of_spin_mask);
    }
    } else
    cpumask_copy(of_spin_mask, cpu_present_mask);
    cpumask_clear_cpu(boot_cpuid, of_spin_mask);
    }
    pr_debug(" <- smp_init_pSeries()\n");
    }
