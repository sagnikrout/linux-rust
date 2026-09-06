//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/smpboot.c
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
// x86 SMP booting functions
//
// (c) 1995 Alan Cox, Building #3 <alan@lxorguk.ukuu.org.uk>
// (c) 1998, 1999, 2000, 2009 Ingo Molnar <mingo@redhat.com>
// Copyright 2001 Andi Kleen, SuSE Labs.
//
// Much of the core SMP work is based on previous work by Thomas Radke, to
// whom a great many thanks are extended.
//
// Thanks to Intel for making available several different Pentium,
// Pentium Pro and Pentium-II/Xeon MP machines.
// Original development of Linux SMP code supported by Caldera.
//
// Fixes
// Felix Koop	:	NR_CPUS used properly
// Jose Renau	:	Handle single CPU case.
// Alan Cox	:	By repeated request 8) - Total BogoMIPS report.
// Greg Wright	:	Fix for kernel stacks panic.
// Erich Boleyn	:	MP v1.4 and additional changes.
// Matthias Sattler	:	Changes for 2.1 kernel map.
// Michel Lespinasse	:	Changes for 2.1 kernel map.
// Michael Chastain	:	Change trampoline.S to gnu as.
// Alan Cox	:	Dumb bug: 'B' step PPro's are fine
// Ingo Molnar	:	Added APIC timers, based on code
// from Jose Renau
// Ingo Molnar	:	various cleanups and rewrites
// Tigran Aivazian	:	fixed "0.00 in /proc/uptime on SMP" bug.
// Maciej W. Rozycki	:	Bits for genuine 82489DX APICs
// Andi Kleen		:	Changed for SMP boot into long mode.
// Martin J. Bligh	: 	Added support for multi-quad systems
// Dave Jones	:	Report invalid combinations of Athlon CPUs.
// Rusty Russell	:	Hacked into shape for new "hotplug" boot process.
// Andi Kleen              :       Converted to new state machine.
// Ashok Raj		: 	CPU hotplug support
// Glauber Costa		:	i386 and x86_64 integration
//

// representing HT siblings of each logical CPU
    DEFINE_PER_CPU_READ_MOSTLY(cpumask_var_t, cpu_sibling_map);
    EXPORT_PER_CPU_SYMBOL(cpu_sibling_map);
// representing HT and core siblings of each logical CPU
    DEFINE_PER_CPU_READ_MOSTLY(cpumask_var_t, cpu_core_map);
    EXPORT_PER_CPU_SYMBOL(cpu_core_map);
// representing HT, core, and die siblings of each logical CPU
    DEFINE_PER_CPU_READ_MOSTLY(cpumask_var_t, cpu_die_map);
    EXPORT_PER_CPU_SYMBOL(cpu_die_map);
// Representing CPUs for which sibling maps can be computed
    static cpumask_var_t cpu_sibling_setup_mask;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwait_cpu_dead {
    pub control: c_uint,
    pub status: c_uint,
}

pub const CPUDEAD_MWAIT_WAIT: c_uint = 0xDEADBEEF;
pub const CPUDEAD_MWAIT_KEXEC_HLT: c_uint = 0x4A17DEAD;
//
// Cache line aligned data for mwait_play_dead(). Separate on purpose so
// that it's unlikely to be touched by other CPUs.
//
    static DEFINE_PER_CPU_ALIGNED(struct mwait_cpu_dead, mwait_cpu_dead);
// Maximum number of SMT threads on any online core
    let mut __max_smt_threads: int __read_mostly = 1;
// Flag to indicate if a complete sched domain rebuild is required
    bool x86_topology_update;
#[no_mangle]
pub unsafe extern "C" fn arch_update_cpu_topology() -> c_int {
    int arch_update_cpu_topology(void)
    {
    let mut retval: c_int = x86_topology_update;
    x86_topology_update = false;
    return retval;
    }
    static unsigned int smpboot_warm_reset_vector_count;
#[no_mangle]
pub unsafe extern "C" fn smpboot_setup_warm_reset_vector(start_eip: c_ulong) {
    static inline void smpboot_setup_warm_reset_vector(unsigned long start_eip)
    {
    unsigned long flags;
    spin_lock_irqsave(&rtc_lock, flags);
    if (!smpboot_warm_reset_vector_count++) {
    CMOS_WRITE(0xa, 0xf);
// ((volatile unsigned short *)phys_to_virt(TRAMPOLINE_PHYS_HIGH)) = start_eip >> 4;
// ((volatile unsigned short *)phys_to_virt(TRAMPOLINE_PHYS_LOW)) = start_eip & 0xf;
    }
    spin_unlock_irqrestore(&rtc_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn smpboot_restore_warm_reset_vector() {
    static inline void smpboot_restore_warm_reset_vector(void)
    {
    unsigned long flags;
//
// Paranoid:  Set warm reset code and vector here back
// to default values.
//
    spin_lock_irqsave(&rtc_lock, flags);
    if (!--smpboot_warm_reset_vector_count) {
    CMOS_WRITE(0, 0xf);
// ((volatile u32 *)phys_to_virt(TRAMPOLINE_PHYS_LOW)) = 0;
    }
    spin_unlock_irqrestore(&rtc_lock, flags);
    }
// Run the next set of setup steps for the upcoming CPU
#[no_mangle]
unsafe extern "C" fn ap_starting() {
    static void ap_starting(void)
    {
    let mut cpuid: c_int = smp_processor_id();
// Mop up eventual mwait_play_dead() wreckage
    this_cpu_write(mwait_cpu_dead.status, 0);
    this_cpu_write(mwait_cpu_dead.control, 0);
//
// If woken up by an INIT in an 82489DX configuration the alive
// synchronization guarantees that the CPU does not reach this
// point before an INIT_deassert IPI reaches the local APIC, so it
// is now safe to touch the local APIC.
//
// Set up this CPU, first the APIC, which is probably redundant on
// most boards.
//
    apic_ap_setup();
// Save the processor parameters.
    identify_secondary_cpu(cpuid);
//
// The topology information must be up to date before
// notify_cpu_starting().
//
    set_cpu_sibling_map(cpuid);
    ap_init_aperfmperf();
    pr_debug("Stack at about %p\n", &cpuid);
    wmb();
//
// This runs the AP through all the cpuhp states to its target
// state CPUHP_ONLINE.
//
    notify_cpu_starting(cpuid);
    }
#[no_mangle]
unsafe extern "C" fn ap_calibrate_delay() {
    static void ap_calibrate_delay(void)
    {
//
// Calibrate the delay loop and update loops_per_jiffy in cpu_data.
// identify_secondary_cpu() stored a value that is close but not as
// accurate as the value just calculated.
//
// As this is invoked after the TSC synchronization check,
// calibrate_delay_is_known() will skip the calibration routine
// when TSC is synchronized across sockets.
//
    calibrate_delay();
    cpu_data(smp_processor_id()).loops_per_jiffy = loops_per_jiffy;
    }
//
// Activate a secondary processor.
//
#[no_mangle]
unsafe extern "C" fn start_secondary(unused: *mut c_void) -> void notrace __noendbr {
    static void notrace __noendbr start_secondary(void *unused)
    {
//
// Don't put *anything* except direct CPU state initialization
// before cpu_init(), SMP booting is too fragile that we want to
// limit the things done here to the most necessary things.
//
    cr4_init();
//
// 32-bit specific. 64-bit reaches this code with the correct page
// table established. Yet another historical divergence.
//
    if (IS_ENABLED(CONFIG_X86_32)) {
// switch away from the initial page table
    load_cr3(swapper_pg_dir);
    __flush_tlb_all();
    }
    cpu_init_exception_handling(false);
//
// Load the microcode before reaching the AP alive synchronization
// point below so it is not part of the full per CPU serialized
// bringup part when "parallel" bringup is enabled.
//
// That's even safe when hyperthreading is enabled in the CPU as
// the core code starts the primary threads first and leaves the
// secondary threads waiting for SIPI. Loading microcode on
// physical cores concurrently is a safe operation.
//
// This covers both the Intel specific issue that concurrent
// microcode loading on SMT siblings must be prohibited and the
// vendor independent issue`that microcode loading which changes
// CPUID, MSRs etc. must be strictly serialized to maintain
// software state correctness.
//
    load_ucode_ap();
//
// Synchronization point with the hotplug core. Sets this CPUs
// synchronization state to ALIVE and spin-waits for the control CPU to
// release this CPU for further bringup.
//
    cpuhp_ap_sync_alive();
    cpu_init();
    fpu__init_cpu();
    rcutree_report_cpu_starting(raw_smp_processor_id());
    x86_cpuinit.early_percpu_clock_init();
    ap_starting();
// Check TSC synchronization with the control CPU.
    check_tsc_sync_target();
//
// Calibrate the delay loop after the TSC synchronization check.
// This allows to skip the calibration when TSC is synchronized
// across sockets.
//
    ap_calibrate_delay();
    speculative_store_bypass_ht_init();
//
// Lock vector_lock, set CPU online and bring the vector
// allocator online. Online must be set with vector_lock held
// to prevent a concurrent irq setup/teardown from seeing a
// half valid vector space.
//
    lock_vector_lock();
    set_cpu_online(smp_processor_id(), true);
    lapic_online();
    unlock_vector_lock();
    x86_platform.nmi_init();
// enable local interrupts
    local_irq_enable();
    x86_cpuinit.setup_percpu_clockev();
    wmb();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
    }
    ANNOTATE_NOENDBR_SYM(start_secondary);
    static bool
    topology_same_node(struct cpuinfo_x86 *c, struct cpuinfo_x86 *o)
    {
    let mut cpu1: c_int = c.cpu_index, cpu2 = o.cpu_index;
    return (cpu_to_node(cpu1) == cpu_to_node(cpu2));
    }
    static bool
    topology_sane(struct cpuinfo_x86 *c, struct cpuinfo_x86 *o, const char *name)
    {
    let mut cpu1: c_int = c.cpu_index, cpu2 = o.cpu_index;
    return !WARN_ONCE(!topology_same_node(c, o),
    "sched: CPU #%d's %s-sibling CPU #%d is not on the same node! "
    "[node: %d != %d]. Ignoring dependency.\n",
    cpu1, name, cpu2, cpu_to_node(cpu1), cpu_to_node(cpu2));
    }

    do {									\
    cpumask_set_cpu((c1), mfunc(c2));				\
    cpumask_set_cpu((c2), mfunc(c1));				\
    } while (0)
#[no_mangle]
unsafe extern "C" fn match_smt(c: *mut cpuinfo_x86, o: *mut cpuinfo_x86) -> bool {
    static bool match_smt(struct cpuinfo_x86 *c, struct cpuinfo_x86 *o)
    {
    if (boot_cpu_has(X86_FEATURE_TOPOEXT)) {
    let mut cpu1: c_int = c.cpu_index, cpu2 = o.cpu_index;
    if (c.topo.pkg_id == o.topo.pkg_id &&
    c.topo.die_id == o.topo.die_id &&
    c.topo.amd_node_id == o.topo.amd_node_id &&
    per_cpu_llc_id(cpu1) == per_cpu_llc_id(cpu2)) {
    if (c.topo.core_id == o.topo.core_id)
    return topology_sane(c, o, "smt");
    if ((c.topo.cu_id != 0xff) &&
    (o.topo.cu_id != 0xff) &&
    (c.topo.cu_id == o.topo.cu_id))
    return topology_sane(c, o, "smt");
    }
    } else if (c.topo.pkg_id == o.topo.pkg_id &&
    c.topo.die_id == o.topo.die_id &&
    c.topo.core_id == o.topo.core_id) {
    return topology_sane(c, o, "smt");
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn match_die(c: *mut cpuinfo_x86, o: *mut cpuinfo_x86) -> bool {
    static bool match_die(struct cpuinfo_x86 *c, struct cpuinfo_x86 *o)
    {
    if (c.topo.pkg_id != o.topo.pkg_id || c.topo.die_id != o.topo.die_id)
    return false;
    if (cpu_feature_enabled(X86_FEATURE_TOPOEXT) && topology_amd_nodes_per_pkg() > 1)
    return c.topo.amd_node_id == o.topo.amd_node_id;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn match_l2c(c: *mut cpuinfo_x86, o: *mut cpuinfo_x86) -> bool {
    static bool match_l2c(struct cpuinfo_x86 *c, struct cpuinfo_x86 *o)
    {
    let mut cpu1: c_int = c.cpu_index, cpu2 = o.cpu_index;
// If the arch didn't set up l2c_id, fall back to SMT
    if (per_cpu_l2c_id(cpu1) == BAD_APICID)
    return match_smt(c, o);
// Do not match if L2 cache id does not match:
    if (per_cpu_l2c_id(cpu1) != per_cpu_l2c_id(cpu2))
    return false;
    return topology_sane(c, o, "l2c");
    }
//
// Unlike the other levels, we do not enforce keeping a
// multicore group inside a NUMA node.  If this happens, we will
// discard the MC level of the topology later.
//
#[no_mangle]
unsafe extern "C" fn match_pkg(c: *mut cpuinfo_x86, o: *mut cpuinfo_x86) -> bool {
    static bool match_pkg(struct cpuinfo_x86 *c, struct cpuinfo_x86 *o)
    {
    if (c.topo.pkg_id == o.topo.pkg_id)
    return true;
    return false;
    }
//
// Define intel_cod_cpu[] for Intel COD (Cluster-on-Die) CPUs.
//
// Any Intel CPU that has multiple nodes per package and does not
// match intel_cod_cpu[] has the SNC (Sub-NUMA Cluster) topology.
//
// When in SNC mode, these CPUs enumerate an LLC that is shared
// by multiple NUMA nodes. The LLC is shared for off-package data
// access but private to the NUMA node (half of the package) for
// on-package access. CPUID (the source of the information about
// the LLC) can only enumerate the cache as shared or unshared,
// but not this particular configuration.
//
    static const struct x86_cpu_id intel_cod_cpu[] = {
    X86_MATCH_VFM(INTEL_HASWELL_X,	 0),	/* COD */
    X86_MATCH_VFM(INTEL_BROADWELL_X, 0),	/* COD */
    X86_MATCH_VFM(INTEL_ANY,	 1),	/* SNC */
    {}
    };
//
// Allows splitting the LLC by matching 'core_id % split_llc'.
//
// This is mostly a debug hack to emulate systems with multiple LLCs per node
// on systems that do not naturally have this.
//
    let mut split_llc: static unsigned int = 0;
#[no_mangle]
unsafe extern "C" fn split_llc_setup(str: *mut c_char) -> int __init {
    static int __init split_llc_setup(char *str)
    {
    get_option(&str, &split_llc);
    return 0;
    }
    early_param("split_llc", split_llc_setup);
#[no_mangle]
unsafe extern "C" fn match_llc(c: *mut cpuinfo_x86, o: *mut cpuinfo_x86) -> bool {
    static bool match_llc(struct cpuinfo_x86 *c, struct cpuinfo_x86 *o)
    {
    const struct x86_cpu_id *id = x86_match_cpu(intel_cod_cpu);
    let mut cpu1: c_int = c.cpu_index, cpu2 = o.cpu_index;
    let mut intel_snc: bool = id && id.driver_data;
// Do not match if we do not have a valid APICID for cpu:
    if (per_cpu_llc_id(cpu1) == BAD_APICID)
    return false;
// Do not match if LLC id does not match:
    if (per_cpu_llc_id(cpu1) != per_cpu_llc_id(cpu2))
    return false;
    if (split_llc &&
    (per_cpu_core_id(cpu1) % split_llc) !=
    (per_cpu_core_id(cpu2) % split_llc))
    return false;
//
// Allow the SNC topology without warning. Return of false
// means 'c' does not share the LLC of 'o'. This will be
// reflected to userspace.
//
    if (match_pkg(c, o) && !topology_same_node(c, o) && intel_snc)
    return false;
    return topology_sane(c, o, "llc");
    }
#[no_mangle]
pub unsafe extern "C" fn x86_sched_itmt_flags() -> c_int {
    static inline int x86_sched_itmt_flags(void)
    {
    return sysctl_sched_itmt_enabled ? SD_ASYM_PACKING : 0;
    }

#[no_mangle]
unsafe extern "C" fn x86_core_flags() -> c_int {
    static int x86_core_flags(void)
    {
    return cpu_core_flags() | x86_sched_itmt_flags();
    }

#[no_mangle]
unsafe extern "C" fn x86_cluster_flags() -> c_int {
    static int x86_cluster_flags(void)
    {
    return cpu_cluster_flags() | x86_sched_itmt_flags();
    }

    static struct sched_domain_topology_level x86_topology[] = {
    SDTL_INIT(tl_smt_mask, cpu_smt_flags, SMT),

    SDTL_INIT(tl_cls_mask, x86_cluster_flags, CLS),

    SDTL_INIT(tl_mc_mask, x86_core_flags, MC),

    SDTL_INIT(tl_pkg_mask, x86_sched_itmt_flags, PKG),
    { core::ptr::null_mut() },
    };
#[no_mangle]
unsafe extern "C" fn build_sched_topology() -> void __init {
    static void __init build_sched_topology(void)
    {
    struct sched_domain_topology_level *topology = x86_topology;
//
// When there is NUMA topology inside the package invalidate the
// PKG domain since the NUMA domains will auto-magically create the
// right spanning domains based on the SLIT.
//
    if (topology_num_nodes_per_package() > 1) {
    let mut pkgdom: c_uint = ARRAY_SIZE(x86_topology) - 2;
    memset(&x86_topology[pkgdom], 0, sizeof(x86_topology[pkgdom]));
    }
//
// Drop the SMT domains if there is only one thread per-core
// since it'll get degenerated by the scheduler anyways.
//
    if (cpu_smt_num_threads <= 1)
    ++topology;
    set_sched_topology(topology);
    }

//
// Test if the on-trace cluster at (N,N) is symmetric.
// Uses upper triangle iteration to avoid obvious duplicates.
//
#[no_mangle]
unsafe extern "C" fn slit_cluster_symmetric(N: c_int) -> bool {
    static bool slit_cluster_symmetric(int N)
    {
    let mut u: c_int = topology_num_nodes_per_package();
    for (int k = 0; k < u; k++) {
    for (int l = k; l < u; l++) {
    if (node_distance(N + k, N + l) !=
    node_distance(N + l, N + k))
    return false;
    }
    }
    return true;
    }
//
// Return the package-id of the cluster, or ~0 if indeterminate.
// Each node in the on-trace cluster should have the same package-id.
//
#[no_mangle]
unsafe extern "C" fn slit_cluster_package(N: c_int) -> u32 {
    static u32 slit_cluster_package(int N)
    {
    let mut u: c_int = topology_num_nodes_per_package();
    let mut pkg_id: u32 = ~0;
    for (int n = 0; n < u; n++) {
    const struct cpumask *cpus = cpumask_of_node(N + n);
    int cpu;
    for_each_cpu(cpu, cpus) {
    let mut id: u32 = topology_logical_package_id(cpu);
    if (pkg_id == ~0)
    pkg_id = id;
    if (pkg_id != id)
    return ~0;
    }
    }
    return pkg_id;
    }
//
// Validate the SLIT table is of the form expected for SNC, specifically:
//
// - each on-trace cluster should be symmetric,
// - each on-trace cluster should have a unique package-id.
//
// If you NUMA_EMU on top of SNC, you get to keep the pieces.
//
#[no_mangle]
unsafe extern "C" fn slit_validate() -> bool {
    static bool slit_validate(void)
    {
    let mut u: c_int = topology_num_nodes_per_package();
    u32 pkg_id, prev_pkg_id = ~0;
    for (int pkg = 0; pkg < topology_max_packages(); pkg++) {
    let mut n: c_int = pkg * u;
//
// Ensure the on-trace cluster is symmetric and each cluster
// has a different package id.
//
    if (!slit_cluster_symmetric(n))
    return false;
    pkg_id = slit_cluster_package(n);
    if (pkg_id == ~0)
    return false;
    if (pkg && pkg_id == prev_pkg_id)
    return false;
    prev_pkg_id = pkg_id;
    }
    return true;
    }
//
// Compute a sanitized SLIT table for SNC; notably SNC-3 can end up with
// asymmetric off-trace clusters, reflecting physical assymmetries. However
// this leads to 'unfortunate' sched_domain configurations.
//
// For example dual socket GNR with SNC-3:
//
// node distances:
// node     0    1    2    3    4    5
// 0:   10   15   17   21   28   26
// 1:   15   10   15   23   26   23
// 2:   17   15   10   26   23   21
// 3:   21   28   26   10   15   17
// 4:   23   26   23   15   10   15
// 5:   26   23   21   17   15   10
//
// Fix things up by averaging out the off-trace clusters; resulting in:
//
// node     0    1    2    3    4    5
// 0:   10   15   17   24   24   24
// 1:   15   10   15   24   24   24
// 2:   17   15   10   24   24   24
// 3:   24   24   24   10   15   17
// 4:   24   24   24   15   10   15
// 5:   24   24   24   17   15   10
//
#[no_mangle]
unsafe extern "C" fn slit_cluster_distance(i: c_int, j: c_int) -> c_int {
    static int slit_cluster_distance(int i, int j)
    {
    let mut slit_valid: static int = -1;
    let mut u: c_int = topology_num_nodes_per_package();
    let mut d: c_long = 0;
    int x, y;
    if (slit_valid < 0) {
    slit_valid = slit_validate();
    if (!slit_valid)
    pr_err(FW_BUG "SLIT table doesn't have the expected form for SNC -- fixup disabled!\n");
    else
    pr_info("Fixing up SNC SLIT table.\n");
    }
//
// Is this a unit cluster on the trace?
//
    if ((i / u) == (j / u) || !slit_valid)
    return node_distance(i, j);
//
// Off-trace cluster.
//
// Notably average out the symmetric pair of off-trace clusters to
// ensure the resulting SLIT table is symmetric.
//
    x = i - (i % u);
    y = j - (j % u);
    for (i = x; i < x + u; i++) {
    for (j = y; j < y + u; j++) {
    d += node_distance(i, j);
    d += node_distance(j, i);
    }
    }
    return d / (2*u*u);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_sched_node_distance(from: c_int, to: c_int) -> c_int {
    int arch_sched_node_distance(int from, int to)
    {
    let mut d: c_int = node_distance(from, to);
    switch (boot_cpu_data.x86_vfm) {
    case INTEL_GRANITERAPIDS_X:
    case INTEL_ATOM_DARKMONT_X:
    if (topology_max_packages() == 1 ||
    topology_num_nodes_per_package() < 3)
    return d;
//
// Handle SNC-3 asymmetries.
//
    return slit_cluster_distance(from, to);
    }
    return d;
    }

#[no_mangle]
pub unsafe extern "C" fn set_cpu_sibling_map(cpu: c_int) {
    void set_cpu_sibling_map(int cpu)
    {
    let mut has_smt: bool = __max_threads_per_core > 1;
    let mut has_mp: bool = has_smt || topology_num_cores_per_package() > 1;
    struct cpuinfo_x86 *c = &cpu_data(cpu);
    struct cpuinfo_x86 *o;
    int i, threads;
    cpumask_set_cpu(cpu, cpu_sibling_setup_mask);
    if (!has_mp) {
    cpumask_set_cpu(cpu, topology_sibling_cpumask(cpu));
    cpumask_set_cpu(cpu, cpu_llc_shared_mask(cpu));
    cpumask_set_cpu(cpu, cpu_l2c_shared_mask(cpu));
    cpumask_set_cpu(cpu, topology_core_cpumask(cpu));
    cpumask_set_cpu(cpu, topology_die_cpumask(cpu));
    c.booted_cores = 1;
    return;
    }
    for_each_cpu(i, cpu_sibling_setup_mask) {
    o = &cpu_data(i);
    if (match_pkg(c, o) && !topology_same_node(c, o))
    WARN_ON_ONCE(topology_num_nodes_per_package() == 1);
    if ((i == cpu) || (has_smt && match_smt(c, o)))
    link_mask(topology_sibling_cpumask, cpu, i);
    if ((i == cpu) || (has_mp && match_llc(c, o)))
    link_mask(cpu_llc_shared_mask, cpu, i);
    if ((i == cpu) || (has_mp && match_l2c(c, o)))
    link_mask(cpu_l2c_shared_mask, cpu, i);
    if ((i == cpu) || (has_mp && match_die(c, o)))
    link_mask(topology_die_cpumask, cpu, i);
    }
    threads = cpumask_weight(topology_sibling_cpumask(cpu));
    if (threads > __max_smt_threads)
    __max_smt_threads = threads;
    for_each_cpu(i, topology_sibling_cpumask(cpu))
    cpu_data(i).smt_active = threads > 1;
//
// This needs a separate iteration over the cpus because we rely on all
// topology_sibling_cpumask links to be set-up.
//
    for_each_cpu(i, cpu_sibling_setup_mask) {
    o = &cpu_data(i);
    if ((i == cpu) || (has_mp && match_pkg(c, o))) {
    link_mask(topology_core_cpumask, cpu, i);
//
// Does this new cpu bringup a new core?
//
    if (threads == 1) {
//
// for each core in package, increment
// the booted_cores for this new cpu
//
    if (cpumask_first(
    topology_sibling_cpumask(i)) == i)
    c.booted_cores++;
//
// increment the core count for all
// the other cpus in this package
//
    if (i != cpu)
    cpu_data(i).booted_cores++;
    } else if (i != cpu && !c.booted_cores)
    c.booted_cores = cpu_data(i).booted_cores;
    }
    }
    }
// maps the cpu to the sched domain representing multi-core
    const struct cpumask *cpu_coregroup_mask(int cpu)
    {
    return cpu_llc_shared_mask(cpu);
    }
    const struct cpumask *cpu_clustergroup_mask(int cpu)
    {
    return cpu_l2c_shared_mask(cpu);
    }
    EXPORT_SYMBOL_GPL(cpu_clustergroup_mask);
#[no_mangle]
unsafe extern "C" fn impress_friends() {
    static void impress_friends(void)
    {
    int cpu;
    let mut bogosum: c_ulong = 0;
//
// Allow the user to impress friends.
//
    pr_debug("Before bogomips\n");
    for_each_online_cpu(cpu)
    bogosum += cpu_data(cpu).loops_per_jiffy;
    pr_info("Total of %d processors activated (%lu.%02lu BogoMIPS)\n",
    num_online_cpus(),
    bogosum/(500000/HZ),
    (bogosum/(5000/HZ))%100);
    pr_debug("Before bogocount - setting activated=1\n");
    }
//
// The Multiprocessor Specification 1.4 (1997) example code suggests
// that there should be a 10ms delay between the BSP asserting INIT
// and de-asserting INIT, when starting a remote processor.
// But that slows boot and resume on modern processors, which include
// many cores and don't require that delay.
//
// Cmdline "cpu_init_udelay=" is available to override this delay.
//
pub const UDELAY_10MS_LEGACY: c_int = 10000;
    let mut init_udelay: static unsigned int = UINT_MAX;
#[no_mangle]
unsafe extern "C" fn cpu_init_udelay(str: *mut c_char) -> int __init {
    static int __init cpu_init_udelay(char *str)
    {
    get_option(&str, &init_udelay);
    return 0;
    }
    early_param("cpu_init_udelay", cpu_init_udelay);
#[no_mangle]
unsafe extern "C" fn smp_set_init_udelay() -> void __init {
    static void __init smp_set_init_udelay(void)
    {
// if cmdline changed it from default, leave it alone
    if (init_udelay != UINT_MAX)
    return;
// if modern processor, use no delay
    if ((boot_cpu_data.x86_vendor == X86_VENDOR_INTEL && boot_cpu_data.x86_vfm >= INTEL_PENTIUM_PRO) ||
    (boot_cpu_data.x86_vendor == X86_VENDOR_HYGON && boot_cpu_data.x86 >= 0x18) ||
    (boot_cpu_data.x86_vendor == X86_VENDOR_AMD   && boot_cpu_data.x86 >= 0xF)) {
    init_udelay = 0;
    return;
    }
// else, use legacy delay
    init_udelay = UDELAY_10MS_LEGACY;
    }
//
// Wake up AP by INIT, INIT, STARTUP sequence.
//
#[no_mangle]
unsafe extern "C" fn send_init_sequence(phys_apicid: u32) {
    static void send_init_sequence(u32 phys_apicid)
    {
    let mut maxlvt: c_int = lapic_get_maxlvt();
// Be paranoid about clearing APIC errors.
    if (APIC_INTEGRATED(boot_cpu_apic_version)) {
// Due to the Pentium erratum 3AP.
    if (maxlvt > 3)
    apic_write(APIC_ESR, 0);
    apic_read(APIC_ESR);
    }
// Assert INIT on the target CPU
    apic_icr_write(APIC_INT_LEVELTRIG | APIC_INT_ASSERT | APIC_DM_INIT, phys_apicid);
    safe_apic_wait_icr_idle();
    udelay(init_udelay);
// Deassert INIT on the target CPU
    apic_icr_write(APIC_INT_LEVELTRIG | APIC_DM_INIT, phys_apicid);
    safe_apic_wait_icr_idle();
    }
//
// Wake up AP by INIT, INIT, STARTUP sequence.
//
#[no_mangle]
unsafe extern "C" fn wakeup_secondary_cpu_via_init(phys_apicid: u32, start_eip: c_ulong, cpu: c_uint) -> c_int {
    static int wakeup_secondary_cpu_via_init(u32 phys_apicid, unsigned long start_eip, unsigned int cpu)
    {
    let mut send_status: c_ulong = 0, accept_status = 0;
    int num_starts, j, maxlvt;
    preempt_disable();
    maxlvt = lapic_get_maxlvt();
    send_init_sequence(phys_apicid);
    mb();
//
// Should we send STARTUP IPIs ?
//
// Determine this based on the APIC version.
// If we don't have an integrated APIC, don't send the STARTUP IPIs.
//
    if (APIC_INTEGRATED(boot_cpu_apic_version))
    num_starts = 2;
    else
    num_starts = 0;
//
// Run STARTUP IPI loop.
//
    pr_debug("#startup loops: %d\n", num_starts);
    for (j = 1; j <= num_starts; j++) {
    pr_debug("Sending STARTUP #%d\n", j);
    if (maxlvt > 3)		/* Due to the Pentium erratum 3AP.  */
    apic_write(APIC_ESR, 0);
    apic_read(APIC_ESR);
    pr_debug("After apic_write\n");
//
// STARTUP IPI
//
// Target chip
// Boot on the stack
// Kick the second
    apic_icr_write(APIC_DM_STARTUP | (start_eip >> 12),
    phys_apicid);
//
// Give the other CPU some time to accept the IPI.
//
    if (init_udelay == 0)
    udelay(10);
    else
    udelay(300);
    pr_debug("Startup point 1\n");
    pr_debug("Waiting for send to finish...\n");
    send_status = safe_apic_wait_icr_idle();
//
// Give the other CPU some time to accept the IPI.
//
    if (init_udelay == 0)
    udelay(10);
    else
    udelay(200);
    if (maxlvt > 3)		/* Due to the Pentium erratum 3AP.  */
    apic_write(APIC_ESR, 0);
    accept_status = (apic_read(APIC_ESR) & 0xEF);
    if (send_status || accept_status)
    break;
    }
    pr_debug("After Startup\n");
    if (send_status)
    pr_err("APIC never delivered???\n");
    if (accept_status)
    pr_err("APIC delivery error (%lx)\n", accept_status);
    preempt_enable();
    return (send_status | accept_status);
    }
// reduce the number of lines printed when booting a large cpu count system
#[no_mangle]
unsafe extern "C" fn announce_cpu(cpu: c_int, apicid: c_int) {
    static void announce_cpu(int cpu, int apicid)
    {
    static int width, node_width, first = 1;
    let mut current_node: static int = NUMA_NO_NODE;
    let mut node: c_int = early_cpu_to_node(cpu);
    if (!width)
    width = num_digits(num_possible_cpus()) + 1; /* + '#' sign */
    if (!node_width)
    node_width = num_digits(num_possible_nodes()) + 1; /* + '#' */
    if (system_state < SYSTEM_RUNNING) {
    if (first)
    pr_info("x86: Booting SMP configuration:\n");
    if (node != current_node) {
    if (current_node > (-1))
    pr_cont("\n");
    current_node = node;
    printk(KERN_INFO ".... node %*s#%d, CPUs:  ",
    node_width - num_digits(node), " ", node);
    }
// Add padding for the BSP
    if (first)
    pr_cont("%*s", width + 1, " ");
    first = 0;
    pr_cont("%*s#%d", width - num_digits(cpu), " ", cpu);
    } else
    pr_info("Booting Node %d Processor %d APIC 0x%x\n",
    node, cpu, apicid);
    }
#[no_mangle]
pub unsafe extern "C" fn common_cpu_up(cpu: c_uint, idle: *mut task_struct) -> c_int {
    int common_cpu_up(unsigned int cpu, struct task_struct *idle)
    {
    int ret;
    per_cpu(current_task, cpu) = idle;
    cpu_init_stack_canary(cpu, idle);
// Initialize the interrupt stack(s)
    ret = irq_init_percpu_irqstack(cpu);
    if (ret)
    return ret;

// Stack for startup_32 can be just as for start_secondary onwards
    per_cpu(cpu_current_top_of_stack, cpu) = task_top_of_stack(idle);

    return 0;
    }
//
// NOTE - on most systems this is a PHYSICAL apic ID, but on multiquad
// (ie clustered apic addressing mode), this is a LOGICAL apic ID.
// Returns zero if startup was successfully sent, else error code from
// ->wakeup_secondary_cpu.
//
#[no_mangle]
unsafe extern "C" fn do_boot_cpu(apicid: u32, cpu: c_uint, idle: *mut task_struct) -> c_int {
    static int do_boot_cpu(u32 apicid, unsigned int cpu, struct task_struct *idle)
    {
    let mut start_ip: c_ulong = real_mode_header.trampoline_start;
    int ret;

// If 64-bit wakeup method exists, use the 64-bit mode trampoline IP
    if (apic.wakeup_secondary_cpu_64)
    start_ip = real_mode_header.trampoline_start64;

    idle.thread.sp = (unsigned long)task_pt_regs(idle);
    initial_code = (unsigned long)start_secondary;
    if (IS_ENABLED(CONFIG_X86_32)) {
    early_gdt_descr.address = (unsigned long)get_cpu_gdt_rw(cpu);
    initial_stack  = idle.thread.sp;
    } else if (!(smpboot_control & STARTUP_PARALLEL_MASK)) {
    smpboot_control = cpu;
    }
// Enable the espfix hack for this CPU
    init_espfix_ap(cpu);
// So we see what's up
    announce_cpu(cpu, apicid);
//
// This grunge runs the startup process for
// the targeted processor.
//
    if (x86_platform.legacy.warm_reset) {
    pr_debug("Setting warm reset code and vector.\n");
    smpboot_setup_warm_reset_vector(start_ip);
//
// Be paranoid about clearing APIC errors.
//
    if (APIC_INTEGRATED(boot_cpu_apic_version)) {
    apic_write(APIC_ESR, 0);
    apic_read(APIC_ESR);
    }
    }
    smp_mb();
//
// Wake up a CPU in difference cases:
// - Use a method from the APIC driver if one defined, with wakeup
// straight to 64-bit mode preferred over wakeup to RM.
// Otherwise,
// - Use an INIT boot APIC message
//
    if (apic.wakeup_secondary_cpu_64)
    ret = apic.wakeup_secondary_cpu_64(apicid, start_ip, cpu);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: apic->wakeup_secondary_cpu) -> else {
    else if (apic.wakeup_secondary_cpu)
    ret = apic.wakeup_secondary_cpu(apicid, start_ip, cpu);
    else
    ret = wakeup_secondary_cpu_via_init(apicid, start_ip, cpu);
// If the wakeup mechanism failed, cleanup the warm reset vector
    if (ret)
    arch_cpuhp_cleanup_kick_cpu(cpu);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn native_kick_ap(cpu: c_uint, tidle: *mut task_struct) -> c_int {
    int native_kick_ap(unsigned int cpu, struct task_struct *tidle)
    {
    let mut apicid: u32 = apic.cpu_present_to_apicid(cpu);
    int err;
    lockdep_assert_irqs_enabled();
    pr_debug("++++++++++++++++++++=_---CPU UP  %u\n", cpu);
    if (apicid == BAD_APICID || !apic_id_valid(apicid)) {
    pr_err("CPU %u has invalid APIC ID %x. Aborting bringup\n", cpu, apicid);
    return -EINVAL;
    }
    if (!test_bit(apicid, phys_cpu_present_map)) {
    pr_err("CPU %u APIC ID %x is not present. Aborting bringup\n", cpu, apicid);
    return -EINVAL;
    }
//
// Save current MTRR state in case it was changed since early boot
// (e.g. by the ACPI SMI) to initialize new CPUs with MTRRs in sync:
//
    mtrr_save_state();
// the FPU context is blank, nobody can own it
    per_cpu(fpu_fpregs_owner_ctx, cpu) = core::ptr::null_mut();
    err = common_cpu_up(cpu, tidle);
    if (err)
    return err;
    err = do_boot_cpu(apicid, cpu, tidle);
    if (err)
    pr_err("do_boot_cpu failed(%d) to wakeup CPU#%u\n", err, cpu);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_kick_ap_alive(cpu: c_uint, tidle: *mut task_struct) -> c_int {
    int arch_cpuhp_kick_ap_alive(unsigned int cpu, struct task_struct *tidle)
    {
    return smp_ops.kick_ap_alive(cpu, tidle);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_cleanup_kick_cpu(cpu: c_uint) {
    void arch_cpuhp_cleanup_kick_cpu(unsigned int cpu)
    {
// Cleanup possible dangling ends...
    if (smp_ops.kick_ap_alive == native_kick_ap && x86_platform.legacy.warm_reset)
    smpboot_restore_warm_reset_vector();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_cleanup_dead_cpu(cpu: c_uint) {
    void arch_cpuhp_cleanup_dead_cpu(unsigned int cpu)
    {
    if (smp_ops.cleanup_dead_cpu)
    smp_ops.cleanup_dead_cpu(cpu);
    if (system_state == SYSTEM_RUNNING)
    pr_info("CPU %u is now offline\n", cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_sync_state_poll() {
    void arch_cpuhp_sync_state_poll(void)
    {
    if (smp_ops.poll_sync_state)
    smp_ops.poll_sync_state();
    }
//
// arch_disable_smp_support() - Disables SMP support for x86 at boottime
//
#[no_mangle]
pub unsafe extern "C" fn arch_disable_smp_support() -> void __init {
    void __init arch_disable_smp_support(void)
    {
    disable_ioapic_support();
    }
//
// Fall back to non SMP mode after errors.
//
// RED-PEN audit/test this more. I bet there is more state messed up here.
//
#[no_mangle]
unsafe extern "C" fn disable_smp() -> __init void {
    static __init void disable_smp(void)
    {
    pr_info("SMP disabled\n");
    disable_ioapic_support();
    topology_reset_possible_cpus_up();
    cpumask_set_cpu(0, topology_sibling_cpumask(0));
    cpumask_set_cpu(0, topology_core_cpumask(0));
    cpumask_set_cpu(0, topology_die_cpumask(0));
    }
#[no_mangle]
pub unsafe extern "C" fn smp_prepare_cpus_common() -> void __init {
    void __init smp_prepare_cpus_common(void)
    {
    unsigned int cpu, node;
// Mark all except the boot CPU as hotpluggable
    for_each_possible_cpu(cpu) {
    if (cpu)
    per_cpu(cpu_info.cpu_index, cpu) = nr_cpu_ids;
    }
    for_each_possible_cpu(cpu) {
    node = cpu_to_node(cpu);
    zalloc_cpumask_var_node(&per_cpu(cpu_sibling_map,    cpu), GFP_KERNEL, node);
    zalloc_cpumask_var_node(&per_cpu(cpu_core_map,       cpu), GFP_KERNEL, node);
    zalloc_cpumask_var_node(&per_cpu(cpu_die_map,        cpu), GFP_KERNEL, node);
    zalloc_cpumask_var_node(&per_cpu(cpu_llc_shared_map, cpu), GFP_KERNEL, node);
    zalloc_cpumask_var_node(&per_cpu(cpu_l2c_shared_map, cpu), GFP_KERNEL, node);
    }
    set_cpu_sibling_map(0);
    }
#[no_mangle]
pub unsafe extern "C" fn smp_prepare_boot_cpu() -> void __init {
    void __init smp_prepare_boot_cpu(void)
    {
    smp_ops.smp_prepare_boot_cpu();
    }

// Establish whether parallel bringup can be supported.
#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_init_parallel_bringup() -> bool __init {
    bool __init arch_cpuhp_init_parallel_bringup(void)
    {
    if (!x86_cpuinit.parallel_bringup) {
    pr_info("Parallel CPU startup disabled by the platform\n");
    return false;
    }
    smpboot_control = STARTUP_READ_APICID;
    pr_debug("Parallel CPU startup enabled: 0x%08x\n", smpboot_control);
    return true;
    }

//
// Prepare for SMP bootup.
// @max_cpus: configured maximum number of CPUs, It is a legacy parameter
// for common interface support.
//
#[no_mangle]
pub unsafe extern "C" fn native_smp_prepare_cpus(max_cpus: c_uint) -> void __init {
    void __init native_smp_prepare_cpus(unsigned int max_cpus)
    {
    smp_prepare_cpus_common();
    switch (apic_intr_mode) {
    case APIC_PIC:
    case APIC_VIRTUAL_WIRE_NO_CONFIG:
    disable_smp();
    return;
    case APIC_SYMMETRIC_IO_NO_ROUTING:
    disable_smp();
// Setup local timer
    x86_init.timers.setup_percpu_clockev();
    return;
    case APIC_VIRTUAL_WIRE:
    case APIC_SYMMETRIC_IO:
    break;
    }
// Setup local timer
    x86_init.timers.setup_percpu_clockev();
    pr_info("CPU0: ");
    print_cpu_info(&cpu_data(0));
    uv_system_init();
    smp_set_init_udelay();
    speculative_store_bypass_ht_init();
    snp_set_wakeup_secondary_cpu();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_thaw_secondary_cpus_begin() {
    void arch_thaw_secondary_cpus_begin(void)
    {
    set_cache_aps_delayed_init(true);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_thaw_secondary_cpus_end() {
    void arch_thaw_secondary_cpus_end(void)
    {
    cache_aps_init();
    }
//
// Early setup to make printk work.
//
#[no_mangle]
pub unsafe extern "C" fn native_smp_prepare_boot_cpu() -> void __init {
    void __init native_smp_prepare_boot_cpu(void)
    {
    let mut me: c_int = smp_processor_id();
// SMP handles this from setup_per_cpu_areas()
    if (!IS_ENABLED(CONFIG_SMP))
    switch_gdt_and_percpu_base(me);
    native_pv_lock_init();
    }
#[no_mangle]
pub unsafe extern "C" fn native_smp_cpus_done(max_cpus: c_uint) -> void __init {
    void __init native_smp_cpus_done(unsigned int max_cpus)
    {
    pr_debug("Boot done\n");
    build_sched_topology();
    nmi_selftest();
    impress_friends();
    cache_aps_init();
    }
// correctly size the local cpu masks
#[no_mangle]
pub unsafe extern "C" fn setup_cpu_local_masks() -> void __init {
    void __init setup_cpu_local_masks(void)
    {
    alloc_bootmem_cpumask_var(&cpu_sibling_setup_mask);
    }

// Recompute SMT state for all CPUs on offline
#[no_mangle]
unsafe extern "C" fn recompute_smt_state() {
    static void recompute_smt_state(void)
    {
    int max_threads, cpu;
    max_threads = 0;
    for_each_online_cpu (cpu) {
    let mut threads: c_int = cpumask_weight(topology_sibling_cpumask(cpu));
    if (threads > max_threads)
    max_threads = threads;
    }
    __max_smt_threads = max_threads;
    }
#[no_mangle]
unsafe extern "C" fn remove_siblinginfo(cpu: c_int) {
    static void remove_siblinginfo(int cpu)
    {
    int sibling;
    struct cpuinfo_x86 *c = &cpu_data(cpu);
    for_each_cpu(sibling, topology_core_cpumask(cpu)) {
    cpumask_clear_cpu(cpu, topology_core_cpumask(sibling));
//
// last thread sibling in this cpu core going down
//
    if (cpumask_weight(topology_sibling_cpumask(cpu)) == 1)
    cpu_data(sibling).booted_cores--;
    }
    for_each_cpu(sibling, topology_die_cpumask(cpu))
    cpumask_clear_cpu(cpu, topology_die_cpumask(sibling));
    for_each_cpu(sibling, topology_sibling_cpumask(cpu)) {
    cpumask_clear_cpu(cpu, topology_sibling_cpumask(sibling));
    if (cpumask_weight(topology_sibling_cpumask(sibling)) == 1)
    cpu_data(sibling).smt_active = false;
    }
    for_each_cpu(sibling, cpu_llc_shared_mask(cpu))
    cpumask_clear_cpu(cpu, cpu_llc_shared_mask(sibling));
    for_each_cpu(sibling, cpu_l2c_shared_mask(cpu))
    cpumask_clear_cpu(cpu, cpu_l2c_shared_mask(sibling));
    cpumask_clear(cpu_llc_shared_mask(cpu));
    cpumask_clear(cpu_l2c_shared_mask(cpu));
    cpumask_clear(topology_sibling_cpumask(cpu));
    cpumask_clear(topology_core_cpumask(cpu));
    cpumask_clear(topology_die_cpumask(cpu));
    c.topo.core_id = 0;
    c.booted_cores = 0;
    cpumask_clear_cpu(cpu, cpu_sibling_setup_mask);
    recompute_smt_state();
    }
#[no_mangle]
unsafe extern "C" fn remove_cpu_from_maps(cpu: c_int) {
    static void remove_cpu_from_maps(int cpu)
    {
    set_cpu_online(cpu, false);
    numa_remove_cpu(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_disable_common() {
    void cpu_disable_common(void)
    {
    let mut cpu: c_int = smp_processor_id();
    remove_siblinginfo(cpu);
//
// Stop allowing kernel-mode FPU. This is needed so that if the CPU is
// brought online again, the initial state is not allowed:
//
    this_cpu_write(kernel_fpu_allowed, false);
// It's now safe to remove this processor from the online map
    lock_vector_lock();
    remove_cpu_from_maps(cpu);
    unlock_vector_lock();
    fixup_irqs();
    lapic_offline();
    }
#[no_mangle]
pub unsafe extern "C" fn native_cpu_disable() -> c_int {
    int native_cpu_disable(void)
    {
    int ret;
    ret = lapic_can_unplug_cpu();
    if (ret)
    return ret;
    cpu_disable_common();
//
// Disable the local APIC. Otherwise IPI broadcasts will reach
// it. It still responds normally to INIT, NMI, SMI, and SIPI
// messages.
//
// Disabling the APIC must happen after cpu_disable_common()
// which invokes fixup_irqs().
//
// Disabling the APIC preserves already set bits in IRR, but
// an interrupt arriving after disabling the local APIC does not
// set the corresponding IRR bit.
//
// fixup_irqs() scans IRR for set bits so it can raise a not
// yet handled interrupt on the new destination CPU via an IPI
// but obviously it can't do so for IRR bits which are not set.
// IOW, interrupts arriving after disabling the local APIC will
// be lost.
//
    apic_soft_disable();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn play_dead_common() {
    void play_dead_common(void)
    {
    idle_task_exit();
    cpuhp_ap_report_dead();
    local_irq_disable();
    }
//
// We need to flush the caches before going to sleep, lest we have
// dirty data in our caches when we come back up.
//
#[no_mangle]
pub unsafe extern "C" fn mwait_play_dead(eax_hint: c_uint) -> void __noreturn {
    void __noreturn mwait_play_dead(unsigned int eax_hint)
    {
    struct mwait_cpu_dead *md = this_cpu_ptr(&mwait_cpu_dead);
// Set up state for the kexec() hack below
    md.status = CPUDEAD_MWAIT_WAIT;
    md.control = CPUDEAD_MWAIT_WAIT;
    wbinvd();
    while (1) {
//
// The CLFLUSH is a workaround for erratum AAI65 for
// the Xeon 7400 series.  It's not clear it is actually
// needed, but it should be harmless in either case.
// The WBINVD is insufficient due to the spurious-wakeup
// case where we return around the loop.
//
    mb();
    clflush(md);
    mb();
    __monitor(md, 0, 0);
    mb();
    __mwait(eax_hint, 0);
    if (READ_ONCE(md.control) == CPUDEAD_MWAIT_KEXEC_HLT) {
//
// Kexec is about to happen. Don't go back into mwait() as
// the kexec kernel might overwrite text and data including
// page tables and stack. So mwait() would resume when the
// monitor cache line is written to and then the CPU goes
// south due to overwritten text, page tables and stack.
//
// Note: This does _NOT_ protect against a stray MCE, NMI,
// SMI. They will resume execution at the instruction
// following the HLT instruction and run into the problem
// which this is trying to prevent.
//
    WRITE_ONCE(md.status, CPUDEAD_MWAIT_KEXEC_HLT);
    while(1)
    native_halt();
    }
    }
    }
//
// Kick all "offline" CPUs out of mwait on kexec(). See comment in
// mwait_play_dead().
//
#[no_mangle]
pub unsafe extern "C" fn smp_kick_mwait_play_dead() {
    void smp_kick_mwait_play_dead(void)
    {
    let mut newstate: u32 = CPUDEAD_MWAIT_KEXEC_HLT;
    struct mwait_cpu_dead *md;
    unsigned int cpu, i;
    for_each_cpu_andnot(cpu, cpu_present_mask, cpu_online_mask) {
    md = per_cpu_ptr(&mwait_cpu_dead, cpu);
// Does it sit in mwait_play_dead() ?
    if (READ_ONCE(md.status) != CPUDEAD_MWAIT_WAIT)
    continue;
// Wait up to 5ms
    for (i = 0; READ_ONCE(md.status) != newstate && i < 1000; i++) {
// Bring it out of mwait
    WRITE_ONCE(md.control, newstate);
    udelay(5);
    }
    if (READ_ONCE(md.status) != newstate)
    pr_err_once("CPU%u is stuck in mwait_play_dead()\n", cpu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hlt_play_dead() -> void __noreturn {
    void __noreturn hlt_play_dead(void)
    {
    if (__this_cpu_read(cpu_info.x86) >= 4)
    wbinvd();
    while (1)
    native_halt();
    }
#[no_mangle]
pub unsafe extern "C" fn native_play_dead() -> void __noreturn {
    void __noreturn native_play_dead(void)
    {
    if (cpu_feature_enabled(X86_FEATURE_KERNEL_IBRS))
    __update_spec_ctrl(0);
    play_dead_common();
    tboot_shutdown(TB_SHUTDOWN_WFS);
// Below returns only on error.
    cpuidle_play_dead();
    hlt_play_dead();
    }

#[no_mangle]
pub unsafe extern "C" fn native_cpu_disable() -> c_int {
    int native_cpu_disable(void)
    {
    return -ENOSYS;
    }
#[no_mangle]
pub unsafe extern "C" fn native_play_dead() -> void __noreturn {
    void __noreturn native_play_dead(void)
    {
    BUG();
    }
