//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/x2apic_cluster.c
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
// __x2apic_send_IPI_mask() possibly needs to read
// x86_cpu_to_logical_apicid for all online cpus in a sequential way.
// Using per cpu variable would cost one cache line per cpu.
//
    static u32 *x86_cpu_to_logical_apicid __read_mostly;
    static DEFINE_PER_CPU(cpumask_var_t, ipi_mask);
    static DEFINE_PER_CPU_READ_MOSTLY(struct cpumask *, cluster_masks);
#[no_mangle]
unsafe extern "C" fn x2apic_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int {
    static int x2apic_acpi_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    return x2apic_enabled();
    }
#[no_mangle]
unsafe extern "C" fn x2apic_send_IPI(cpu: c_int, vector: c_int) {
    static void x2apic_send_IPI(int cpu, int vector)
    {
    let mut dest: u32 = x86_cpu_to_logical_apicid[cpu];
// x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    __x2apic_send_IPI_dest(dest, vector, APIC_DEST_LOGICAL);
    }
    static void
    __x2apic_send_IPI_mask(const struct cpumask *mask, int vector, int apic_dest)
    {
    unsigned int cpu, clustercpu;
    struct cpumask *tmpmsk;
    unsigned long flags;
    u32 dest;
// x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    local_irq_save(flags);
    tmpmsk = this_cpu_cpumask_var_ptr(ipi_mask);
    cpumask_copy(tmpmsk, mask);
// If IPI should not be sent to self, clear current CPU
    if (apic_dest != APIC_DEST_ALLINC)
    __cpumask_clear_cpu(smp_processor_id(), tmpmsk);
// Collapse cpus in a cluster so a single IPI per cluster is sent
    for_each_cpu(cpu, tmpmsk) {
    struct cpumask *cmsk = per_cpu(cluster_masks, cpu);
    dest = 0;
    for_each_cpu_and(clustercpu, tmpmsk, cmsk)
    dest |= x86_cpu_to_logical_apicid[clustercpu];
    if (!dest)
    continue;
    __x2apic_send_IPI_dest(dest, vector, APIC_DEST_LOGICAL);
// Remove cluster CPUs from tmpmask
    cpumask_andnot(tmpmsk, tmpmsk, cmsk);
    }
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn x2apic_send_IPI_mask(mask: *const cpumask, vector: c_int) {
    static void x2apic_send_IPI_mask(const struct cpumask *mask, int vector)
    {
    __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLINC);
    }
    static void
    x2apic_send_IPI_mask_allbutself(const struct cpumask *mask, int vector)
    {
    __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLBUT);
    }
#[no_mangle]
unsafe extern "C" fn x2apic_calc_apicid(cpu: c_uint) -> u32 {
    static u32 x2apic_calc_apicid(unsigned int cpu)
    {
    return x86_cpu_to_logical_apicid[cpu];
    }
#[no_mangle]
unsafe extern "C" fn init_x2apic_ldr() {
    static void init_x2apic_ldr(void)
    {
    struct cpumask *cmsk = this_cpu_read(cluster_masks);
    BUG_ON(!cmsk);
    cpumask_set_cpu(smp_processor_id(), cmsk);
    }
//
// As an optimisation during boot, set the cluster_mask for all present
// CPUs at once, to prevent each of them having to iterate over the others
// to find the existing cluster_mask.
//
#[no_mangle]
unsafe extern "C" fn prefill_clustermask(cmsk: *mut cpumask, cpu: c_uint, cluster: u32) {
    static void prefill_clustermask(struct cpumask *cmsk, unsigned int cpu, u32 cluster)
    {
    int cpu_i;
    for_each_present_cpu(cpu_i) {
    struct cpumask **cpu_cmsk = &per_cpu(cluster_masks, cpu_i);
    let mut apicid: u32 = apic.cpu_present_to_apicid(cpu_i);
    if (apicid == BAD_APICID || cpu_i == cpu || apic_cluster(apicid) != cluster)
    continue;
    if (WARN_ON_ONCE(*cpu_cmsk == cmsk))
    continue;
    BUG_ON(*cpu_cmsk);
// cpu_cmsk = cmsk;
    }
    }
#[no_mangle]
unsafe extern "C" fn alloc_clustermask(cpu: c_uint, cluster: u32, node: c_int) -> c_int {
    static int alloc_clustermask(unsigned int cpu, u32 cluster, int node)
    {
    struct cpumask *cmsk = core::ptr::null_mut();
    unsigned int cpu_i;
//
// At boot time, the CPU present mask is stable. The cluster mask is
// allocated for the first CPU in the cluster and propagated to all
// present siblings in the cluster. If the cluster mask is already set
// on entry to this function for a given CPU, there is nothing to do.
//
    if (per_cpu(cluster_masks, cpu))
    return 0;
    if (system_state < SYSTEM_RUNNING)
    goto alloc;
//
// On post boot hotplug for a CPU which was not present at boot time,
// iterate over all possible CPUs (even those which are not present
// any more) to find any existing cluster mask.
//
    for_each_possible_cpu(cpu_i) {
    let mut apicid: u32 = apic.cpu_present_to_apicid(cpu_i);
    if (apicid != BAD_APICID && apic_cluster(apicid) == cluster) {
    cmsk = per_cpu(cluster_masks, cpu_i);
//
// If the cluster is already initialized, just store
// the mask and return. There's no need to propagate.
//
    if (cmsk) {
    per_cpu(cluster_masks, cpu) = cmsk;
    return 0;
    }
    }
    }
//
// No CPU in the cluster has ever been initialized, so fall through to
// the boot time code which will also populate the cluster mask for any
// other CPU in the cluster which is (now) present.
//
    alloc:
    cmsk = kzalloc_node(sizeof(*cmsk), GFP_KERNEL, node);
    if (!cmsk)
    return -ENOMEM;
    per_cpu(cluster_masks, cpu) = cmsk;
    prefill_clustermask(cmsk, cpu, cluster);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x2apic_prepare_cpu(cpu: c_uint) -> c_int {
    static int x2apic_prepare_cpu(unsigned int cpu)
    {
    let mut phys_apicid: u32 = apic.cpu_present_to_apicid(cpu);
    let mut cluster: u32 = apic_cluster(phys_apicid);
    let mut logical_apicid: u32 = (cluster << 16) | (1 << (phys_apicid & 0xf));
    let mut node: c_int = cpu_to_node(cpu);
    x86_cpu_to_logical_apicid[cpu] = logical_apicid;
    if (alloc_clustermask(cpu, cluster, node) < 0)
    return -ENOMEM;
    if (!zalloc_cpumask_var_node(&per_cpu(ipi_mask, cpu), GFP_KERNEL, node))
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x2apic_dead_cpu(dead_cpu: c_uint) -> c_int {
    static int x2apic_dead_cpu(unsigned int dead_cpu)
    {
    struct cpumask *cmsk = per_cpu(cluster_masks, dead_cpu);
    if (cmsk)
    cpumask_clear_cpu(dead_cpu, cmsk);
    free_cpumask_var(per_cpu(ipi_mask, dead_cpu));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x2apic_cluster_probe() -> c_int {
    static int x2apic_cluster_probe(void)
    {
    u32 slots;
    if (!x2apic_mode)
    return 0;
    slots = max_t(u32, L1_CACHE_BYTES/sizeof(u32), nr_cpu_ids);
    x86_cpu_to_logical_apicid = kcalloc(slots, sizeof(u32), GFP_KERNEL);
    if (!x86_cpu_to_logical_apicid)
    return 0;
    if (cpuhp_setup_state(CPUHP_X2APIC_PREPARE, "x86/x2apic:prepare",
    x2apic_prepare_cpu, x2apic_dead_cpu) < 0) {
    pr_err("Failed to register X2APIC_PREPARE\n");
    kfree(x86_cpu_to_logical_apicid);
    x86_cpu_to_logical_apicid = core::ptr::null_mut();
    return 0;
    }
    init_x2apic_ldr();
    return 1;
    }
    static struct apic apic_x2apic_cluster __ro_after_init = {
    .name				= "cluster x2apic",
    .probe				= x2apic_cluster_probe,
    .acpi_madt_oem_check		= x2apic_acpi_madt_oem_check,
    .dest_mode_logical		= true,
    .disable_esr			= 0,
    .init_apic_ldr			= init_x2apic_ldr,
    .cpu_present_to_apicid		= default_cpu_present_to_apicid,
    .max_apic_id			= UINT_MAX,
    .x2apic_set_max_apicid		= true,
    .get_apic_id			= x2apic_get_apic_id,
    .calc_dest_apicid		= x2apic_calc_apicid,
    .send_IPI			= x2apic_send_IPI,
    .send_IPI_mask			= x2apic_send_IPI_mask,
    .send_IPI_mask_allbutself	= x2apic_send_IPI_mask_allbutself,
    .send_IPI_allbutself		= x2apic_send_IPI_allbutself,
    .send_IPI_all			= x2apic_send_IPI_all,
    .send_IPI_self			= x2apic_send_IPI_self,
    .nmi_to_offline_cpu		= true,
    .read				= native_apic_msr_read,
    .write				= native_apic_msr_write,
    .eoi				= native_apic_msr_eoi,
    .icr_read			= native_x2apic_icr_read,
    .icr_write			= native_x2apic_icr_write,
    };
    apic_driver(apic_x2apic_cluster);
