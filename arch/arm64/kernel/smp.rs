//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/smp.c
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
// Based on arch/arm/kernel/smp.c
//
// Copyright (C) 2012 ARM Ltd.
//

//
// as from 2.5, kernels no longer have an init_tasks structure
// so we need some other way of telling a new secondary core
// where to place its SVC stack
//
    struct secondary_data secondary_data;
// Number of CPUs which aren't online, but looping in kernel text.
    static int cpus_stuck_in_kernel;
    static int ipi_irq_base __ro_after_init;
    let mut __ro_after_init: static int nr_ipi = NR_IPI;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipi_descs {
    pub descs: [*mut irq_desc; MAX_IPI],
}

    static DEFINE_PER_CPU_READ_MOSTLY(struct ipi_descs, pcpu_ipi_desc);

    static bool percpu_ipi_descs __ro_after_init;
    static bool crash_stop;
    static void ipi_setup(int cpu);

    static void ipi_teardown(int cpu);
    static int op_cpu_kill(unsigned int cpu);

#[no_mangle]
pub unsafe extern "C" fn op_cpu_kill(cpu: c_uint) -> c_int {
    static inline int op_cpu_kill(unsigned int cpu)
    {
    return -ENOSYS;
    }

//
// Boot a secondary CPU, and assign it the specified idle task.
// This also gives us the initial stack to use for this CPU.
//
#[no_mangle]
unsafe extern "C" fn boot_secondary(cpu: c_uint, idle: *mut task_struct) -> c_int {
    static int boot_secondary(unsigned int cpu, struct task_struct *idle)
    {
    const struct cpu_operations *ops = get_cpu_ops(cpu);
    if (ops.cpu_boot)
    return ops.cpu_boot(cpu);
    return -EOPNOTSUPP;
    }
    static DECLARE_COMPLETION(cpu_running);
#[no_mangle]
pub unsafe extern "C" fn __cpu_up(cpu: c_uint, idle: *mut task_struct) -> c_int {
    int __cpu_up(unsigned int cpu, struct task_struct *idle)
    {
    int ret;
    long status;
//
// We need to tell the secondary core where to find its stack and the
// page tables.
//
    secondary_data.task = idle;
    update_cpu_boot_status(CPU_MMU_OFF);
// Now bring the CPU into our world
    ret = boot_secondary(cpu, idle);
    if (ret) {
    if (ret != -EPERM)
    pr_err("CPU%u: failed to boot: %d\n", cpu, ret);
    return ret;
    }
//
// CPU was successfully started, wait for it to come online or
// time out.
//
    wait_for_completion_timeout(&cpu_running,
    msecs_to_jiffies(5000));
    if (cpu_online(cpu))
    return 0;
    pr_crit("CPU%u: failed to come online\n", cpu);
    secondary_data.task = core::ptr::null_mut();
    status = READ_ONCE(secondary_data.status);
    if (status == CPU_MMU_OFF)
    status = READ_ONCE(__early_cpu_boot_status);
    switch (status & CPU_BOOT_STATUS_MASK) {
    default:
    pr_err("CPU%u: failed in unknown state : 0x%lx\n",
    cpu, status);
    cpus_stuck_in_kernel++;
    break;
    case CPU_KILL_ME:
    if (!op_cpu_kill(cpu)) {
    pr_crit("CPU%u: died during early boot\n", cpu);
    break;
    }
    pr_crit("CPU%u: may not have shut down cleanly\n", cpu);
    fallthrough;
    case CPU_STUCK_IN_KERNEL:
    pr_crit("CPU%u: is stuck in kernel\n", cpu);
    if (status & CPU_STUCK_REASON_52_BIT_VA)
    pr_crit("CPU%u: does not support 52-bit VAs\n", cpu);
    if (status & CPU_STUCK_REASON_NO_GRAN) {
    pr_crit("CPU%u: does not support %luK granule\n",
    cpu, PAGE_SIZE / SZ_1K);
    }
    cpus_stuck_in_kernel++;
    break;
    case CPU_PANIC_KERNEL:
    panic("CPU%u detected unsupported configuration\n", cpu);
    }
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn init_gic_priority_masking() {
    static void init_gic_priority_masking(void)
    {
    u32 cpuflags;
    if (WARN_ON(!gic_enable_sre()))
    return;
    cpuflags = read_sysreg(daif);
    WARN_ON(!(cpuflags & PSR_I_BIT));
    WARN_ON(!(cpuflags & PSR_F_BIT));
    gic_write_pmr(GIC_PRIO_IRQON | GIC_PRIO_PSR_I_SET);
    }
//
// This is the secondary CPU boot entry.  We're using this CPUs
// idle thread stack, but a set of temporary page tables.
//
#[no_mangle]
pub unsafe extern "C" fn secondary_start_kernel() -> asmlinkage notrace void {
    asmlinkage notrace void secondary_start_kernel(void)
    {
    let mut mpidr: u64 = read_cpuid_mpidr() & MPIDR_HWID_BITMASK;
    struct mm_struct *mm = &init_mm;
    const struct cpu_operations *ops;
    let mut cpu: c_uint = smp_processor_id();
//
// All kernel threads share the same mm context; grab a
// reference and switch to it.
//
    mmgrab(mm);
    current.active_mm = mm;
//
// TTBR0 is only used for the identity mapping at this stage. Make it
// point to zero page to avoid speculatively fetching new entries.
//
    cpu_uninstall_idmap();
    if (system_uses_irq_prio_masking())
    init_gic_priority_masking();
    rcutree_report_cpu_starting(cpu);
    trace_hardirqs_off();
//
// If the system has established the capabilities, make sure
// this CPU ticks all of those. If it doesn't, the CPU will
// fail to come online.
//
    check_local_cpu_capabilities();
    ops = get_cpu_ops(cpu);
    if (ops.cpu_postboot)
    ops.cpu_postboot();
//
// Log the CPU info before it is marked online and might get read.
//
    cpuinfo_store_cpu();
    store_cpu_topology(cpu);
//
// Enable GIC and timers.
//
    notify_cpu_starting(cpu);
    ipi_setup(cpu);
    numa_add_cpu(cpu);
//
// OK, now it's safe to let the boot CPU continue.  Wait for
// the CPU migration code to notice that the CPU is online
// before we continue.
//
    pr_info("CPU%u: Booted secondary processor 0x%010lx [0x%08x]\n",
    cpu, (unsigned long)mpidr,
    read_cpuid_id());
    update_cpu_boot_status(CPU_BOOT_SUCCESS);
    set_cpu_online(cpu, true);
    complete(&cpu_running);
//
// Secondary CPUs enter the kernel with all DAIF exceptions masked.
//
// As with setup_arch() we must unmask Debug and SError exceptions, and
// as the root irqchip has already been detected and initialized we can
// unmask IRQ and FIQ at the same time.
//
    local_daif_restore(DAIF_PROCCTX);
//
// OK, it's off to the idle thread for us
//
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
    }

#[no_mangle]
unsafe extern "C" fn op_cpu_disable(cpu: c_uint) -> c_int {
    static int op_cpu_disable(unsigned int cpu)
    {
    const struct cpu_operations *ops = get_cpu_ops(cpu);
//
// If we don't have a cpu_die method, abort before we reach the point
// of no return. CPU0 may not have an cpu_ops, so test for it.
//
    if (!ops || !ops.cpu_die)
    return -EOPNOTSUPP;
//
// We may need to abort a hot unplug for some other mechanism-specific
// reason.
//
    if (ops.cpu_disable)
    return ops.cpu_disable(cpu);
    return 0;
    }
//
// __cpu_disable runs on the processor to be shutdown.
//
#[no_mangle]
pub unsafe extern "C" fn __cpu_disable() -> c_int {
    int __cpu_disable(void)
    {
    let mut cpu: c_uint = smp_processor_id();
    int ret;
    ret = op_cpu_disable(cpu);
    if (ret)
    return ret;
    remove_cpu_topology(cpu);
    numa_remove_cpu(cpu);
//
// Take this CPU offline.  Once we clear this, we can't return,
// and we must not schedule until we're ready to give up the cpu.
//
    set_cpu_online(cpu, false);
    ipi_teardown(cpu);
//
// OK - migrate IRQs away from this CPU
//
    irq_migrate_all_off_this_cpu();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn op_cpu_kill(cpu: c_uint) -> c_int {
    static int op_cpu_kill(unsigned int cpu)
    {
    const struct cpu_operations *ops = get_cpu_ops(cpu);
//
// If we have no means of synchronising with the dying CPU, then assume
// that it is really dead. We can only wait for an arbitrary length of
// time and hope that it's dead, so let's skip the wait and just hope.
//
    if (!ops.cpu_kill)
    return 0;
    return ops.cpu_kill(cpu);
    }
//
// Called on the thread which is asking for a CPU to be shutdown after the
// shutdown completed.
//
#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_cleanup_dead_cpu(cpu: c_uint) {
    void arch_cpuhp_cleanup_dead_cpu(unsigned int cpu)
    {
    int err;
    pr_debug("CPU%u: shutdown\n", cpu);
//
// Now that the dying CPU is beyond the point of no return w.r.t.
// in-kernel synchronisation, try to get the firmware to help us to
// verify that it has really left the kernel before we consider
// clobbering anything it might still be using.
//
    err = op_cpu_kill(cpu);
    if (err)
    pr_warn("CPU%d may not have shut down cleanly: %d\n", cpu, err);
    }
//
// Called from the idle thread for the CPU which has been shutdown.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_die() -> void __noreturn {
    void __noreturn cpu_die(void)
    {
    let mut cpu: c_uint = smp_processor_id();
    const struct cpu_operations *ops = get_cpu_ops(cpu);
    idle_task_exit();
    local_daif_mask();
// Tell cpuhp_bp_sync_dead() that this CPU is now safe to dispose of
    cpuhp_ap_report_dead();
//
// Actually shutdown the CPU. This must never fail. The specific hotplug
// mechanism must perform all required cache maintenance to ensure that
// no dirty lines are lost in the process of shutting down the CPU.
//
    ops.cpu_die(cpu);
    BUG();
    }

#[no_mangle]
unsafe extern "C" fn __cpu_try_die(cpu: c_int) {
    static void __cpu_try_die(int cpu)
    {

    const struct cpu_operations *ops = get_cpu_ops(cpu);
    if (ops && ops.cpu_die)
    ops.cpu_die(cpu);

    }
//
// Kill the calling secondary CPU, early in bringup before it is turned
// online.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_die_early() -> void __noreturn {
    void __noreturn cpu_die_early(void)
    {
    let mut cpu: c_int = smp_processor_id();
    pr_crit("CPU%d: will not boot\n", cpu);
// Mark this CPU absent
    set_cpu_present(cpu, 0);
    rcutree_report_cpu_dead();
    if (IS_ENABLED(CONFIG_HOTPLUG_CPU)) {
    update_cpu_boot_status(CPU_KILL_ME);
    __cpu_try_die(cpu);
    }
    update_cpu_boot_status(CPU_STUCK_IN_KERNEL);
    cpu_park_loop();
    }
#[no_mangle]
unsafe extern "C" fn hyp_mode_check() -> void __init {
    static void __init hyp_mode_check(void)
    {
    if (is_hyp_mode_available())
    pr_info("CPU: All CPU(s) started at EL2\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_hyp_mode_mismatched()) -> else {
    else if (is_hyp_mode_mismatched())
    WARN_TAINT(1, TAINT_CPU_OUT_OF_SPEC,
    "CPU: CPUs started in inconsistent modes");
    else
    pr_info("CPU: All CPU(s) started at EL1\n");
    if (IS_ENABLED(CONFIG_KVM) && !is_kernel_in_hyp_mode()) {
    kvm_compute_layout();
    kvm_apply_hyp_relocations();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn smp_cpus_done(max_cpus: c_uint) -> void __init {
    void __init smp_cpus_done(unsigned int max_cpus)
    {
    pr_info("SMP: Total of %d processors activated.\n", num_online_cpus());
    hyp_mode_check();
    setup_system_features();
    setup_user_features();
    mark_linear_text_alias_ro();
    }
#[no_mangle]
pub unsafe extern "C" fn smp_prepare_boot_cpu() -> void __init {
    void __init smp_prepare_boot_cpu(void)
    {
//
// The runtime per-cpu areas have been allocated by
// setup_per_cpu_areas(), and CPU0's boot time per-cpu area will be
// freed shortly, so we must move over to the runtime per-cpu area.
//
    set_my_cpu_offset(per_cpu_offset(smp_processor_id()));
    cpuinfo_store_boot_cpu();
    setup_boot_cpu_features();
// Conditionally switch to GIC PMR for interrupt masking
    if (system_uses_irq_prio_masking())
    init_gic_priority_masking();
    kasan_init_hw_tags();
// Init percpu seeds for random tags after cpus are set up.
    kasan_init_sw_tags();
    }
//
// Duplicate MPIDRs are a recipe for disaster. Scan all initialized
// entries and check for duplicates. If any is found just ignore the
// cpu. cpu_logical_map was initialized to INVALID_HWID to avoid
// matching valid MPIDR values.
//
#[no_mangle]
unsafe extern "C" fn is_mpidr_duplicate(cpu: c_uint, hwid: u64) -> bool __init {
    static bool __init is_mpidr_duplicate(unsigned int cpu, u64 hwid)
    {
    unsigned int i;
    for (i = 1; (i < cpu) && (i < NR_CPUS); i++)
    if (cpu_logical_map(i) == hwid)
    return true;
    return false;
    }
//
// Initialize cpu operations for a logical cpu and
// set it in the possible mask on success
//
#[no_mangle]
unsafe extern "C" fn smp_cpu_setup(cpu: c_int) -> int __init {
    static int __init smp_cpu_setup(int cpu)
    {
    const struct cpu_operations *ops;
    if (init_cpu_ops(cpu))
    return -ENODEV;
    ops = get_cpu_ops(cpu);
    if (ops.cpu_init(cpu))
    return -ENODEV;
    set_cpu_possible(cpu, true);
    return 0;
    }
    static bool bootcpu_valid __initdata;
    let mut cpu_count: static unsigned int = 1;
#[no_mangle]
pub unsafe extern "C" fn arch_register_cpu(cpu: c_int) -> c_int {
    int arch_register_cpu(int cpu)
    {
    let mut acpi_handle: acpi_handle = acpi_get_processor_handle(cpu);
    struct cpu *c = &per_cpu(cpu_devices, cpu);
    if (!acpi_disabled && !acpi_handle &&
    IS_ENABLED(CONFIG_ACPI_HOTPLUG_CPU))
    return -EPROBE_DEFER;

// For now block anything that looks like physical CPU Hotplug
    if (invalid_logical_cpuid(cpu) || !cpu_present(cpu)) {
    pr_err_once("Changing CPU present bit is not supported\n");
    return -ENODEV;
    }

//
// Availability of the acpi handle is sufficient to establish
// that _STA has already been checked. No need to recheck here.
//
    c.hotpluggable = arch_cpu_is_hotpluggable(cpu);
    return register_cpu(c, cpu);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_unregister_cpu(cpu: c_int) {
    void arch_unregister_cpu(int cpu)
    {
    let mut acpi_handle: acpi_handle = acpi_get_processor_handle(cpu);
    struct cpu *c = &per_cpu(cpu_devices, cpu);
    unsigned long long sta;
    acpi_status status;
    status = acpi_evaluate_integer(acpi_handle, "_STA", core::ptr::null_mut(), &sta);
    if (!ACPI_FAILURE(status) &&
    cpu_present(cpu) && !(sta & ACPI_STA_DEVICE_PRESENT))
    pr_err_once("Changing CPU present bit is not supported\n");
    unregister_cpu(c);
    }

    static struct acpi_madt_generic_interrupt cpu_madt_gicc[NR_CPUS];
    struct acpi_madt_generic_interrupt *acpi_cpu_get_madt_gicc(int cpu)
    {
    return &cpu_madt_gicc[cpu];
    }
    EXPORT_SYMBOL_GPL(acpi_cpu_get_madt_gicc);
#[no_mangle]
unsafe extern "C" fn acpi_cpu_is_present(cpu: c_int) -> bool {
    static bool acpi_cpu_is_present(int cpu)
    {
    return acpi_cpu_get_madt_gicc(cpu).flags & ACPI_MADT_ENABLED;
    }
//
// acpi_map_gic_cpu_interface - parse processor MADT entry
//
// Carry out sanity checks on MADT processor entry and initialize
// cpu_logical_map on success
//
    static void __init
    acpi_map_gic_cpu_interface(struct acpi_madt_generic_interrupt *processor)
    {
    let mut hwid: u64 = processor.arm_mpidr;
    if (!(processor.flags &
    (ACPI_MADT_ENABLED | ACPI_MADT_GICC_ONLINE_CAPABLE))) {
    pr_debug("skipping disabled CPU entry with 0x%llx MPIDR\n", hwid);
    return;
    }
    if (hwid & ~MPIDR_HWID_BITMASK || hwid == INVALID_HWID) {
    pr_err("skipping CPU entry with invalid MPIDR 0x%llx\n", hwid);
    return;
    }
    if (is_mpidr_duplicate(cpu_count, hwid)) {
    pr_err("duplicate CPU MPIDR 0x%llx in MADT\n", hwid);
    return;
    }
// Check if GICC structure of boot CPU is available in the MADT
    if (cpu_logical_map(0) == hwid) {
    if (bootcpu_valid) {
    pr_err("duplicate boot CPU MPIDR: 0x%llx in MADT\n",
    hwid);
    return;
    }
    bootcpu_valid = true;
    cpu_madt_gicc[0] = *processor;
    return;
    }
    if (cpu_count >= NR_CPUS)
    return;
// map the logical cpu id to cpu MPIDR
    set_cpu_logical_map(cpu_count, hwid);
    cpu_madt_gicc[cpu_count] = *processor;
//
// Set-up the ACPI parking protocol cpu entries
// while initializing the cpu_logical_map to
// avoid parsing MADT entries multiple times for
// nothing (ie a valid cpu_logical_map entry should
// contain a valid parking protocol data set to
// initialize the cpu if the parking protocol is
// the only available enable method).
//
    acpi_set_mailbox_entry(cpu_count, processor);
    cpu_count++;
    }
    static int __init
    acpi_parse_gic_cpu_interface(union acpi_subtable_headers *header,
    const unsigned long end)
    {
    struct acpi_madt_generic_interrupt *processor;
    processor = (struct acpi_madt_generic_interrupt *)header;
    if (BAD_MADT_GICC_ENTRY(processor, end))
    return -EINVAL;
    acpi_table_print_madt_entry(&header.common);
    acpi_map_gic_cpu_interface(processor);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_parse_and_init_cpus() -> void __init {
    static void __init acpi_parse_and_init_cpus(void)
    {
    int i;
//
// do a walk of MADT to determine how many CPUs
// we have including disabled CPUs, and get information
// we need for SMP init.
//
    acpi_table_parse_madt(ACPI_MADT_TYPE_GENERIC_INTERRUPT,
    acpi_parse_gic_cpu_interface, 0);
//
// In ACPI, SMP and CPU NUMA information is provided in separate
// static tables, namely the MADT and the SRAT.
//
// Thus, it is simpler to first create the cpu logical map through
// an MADT walk and then map the logical cpus to their node ids
// as separate steps.
//
    acpi_map_cpus_to_nodes();
    for (i = 0; i < nr_cpu_ids; i++)
    early_map_cpu_to_node(i, acpi_numa_get_nid(i));
    }

#[no_mangle]
unsafe extern "C" fn acpi_cpu_is_present(cpu: c_int) -> bool {
    static bool acpi_cpu_is_present(int cpu)
    {
    return false;
    }

//
// Enumerate the possible CPU set from the device tree and build the
// cpu logical map array containing MPIDR values related to logical
// cpus. Assumes that cpu_logical_map(0) has already been initialized.
//
#[no_mangle]
unsafe extern "C" fn of_parse_and_init_cpus() -> void __init {
    static void __init of_parse_and_init_cpus(void)
    {
    struct device_node *dn;
    for_each_of_cpu_node(dn) {
    let mut hwid: u64 = of_get_cpu_hwid(dn, 0);
    if (hwid & ~MPIDR_HWID_BITMASK)
    goto next;
    if (is_mpidr_duplicate(cpu_count, hwid)) {
    pr_err("%pOF: duplicate cpu reg properties in the DT\n",
    dn);
    goto next;
    }
//
// The numbering scheme requires that the boot CPU
// must be assigned logical id 0. Record it so that
// the logical map built from DT is validated and can
// be used.
//
    if (hwid == cpu_logical_map(0)) {
    if (bootcpu_valid) {
    pr_err("%pOF: duplicate boot cpu reg property in DT\n",
    dn);
    goto next;
    }
    bootcpu_valid = true;
    early_map_cpu_to_node(0, of_node_to_nid(dn));
//
// cpu_logical_map has already been
// initialized and the boot cpu doesn't need
// the enable-method so continue without
// incrementing cpu.
//
    continue;
    }
    if (cpu_count >= NR_CPUS)
    goto next;
    pr_debug("cpu logical map 0x%llx\n", hwid);
    set_cpu_logical_map(cpu_count, hwid);
    early_map_cpu_to_node(cpu_count, of_node_to_nid(dn));
    next:
    cpu_count++;
    }
    }
//
// Enumerate the possible CPU set from the device tree or ACPI and build the
// cpu logical map array containing MPIDR values related to logical
// cpus. Assumes that cpu_logical_map(0) has already been initialized.
//
#[no_mangle]
pub unsafe extern "C" fn smp_init_cpus() -> void __init {
    void __init smp_init_cpus(void)
    {
    int i;
    if (acpi_disabled)
    of_parse_and_init_cpus();
    else
    acpi_parse_and_init_cpus();
    if (!bootcpu_valid) {
    pr_err("missing boot CPU MPIDR, not enabling secondaries\n");
    return;
    }
//
// For the nosmp/maxcpus=0 case, do not mark the secondary CPUs
// possible.
//
    if (!setup_max_cpus)
    return;
    if (cpu_count > nr_cpu_ids)
    pr_warn("Number of cores (%d) exceeds configured maximum of %u - clipping\n",
    cpu_count, nr_cpu_ids);
//
// We need to set the cpu_logical_map entries before enabling
// the cpus so that cpu processor description entries (DT cpu nodes
// and ACPI MADT entries) can be retrieved by matching the cpu hwid
// with entries in cpu_logical_map while initializing the cpus.
// If the cpu set-up fails, invalidate the cpu_logical_map entry.
//
    for (i = 1; i < nr_cpu_ids; i++) {
    if (cpu_logical_map(i) != INVALID_HWID) {
    if (smp_cpu_setup(i))
    set_cpu_logical_map(i, INVALID_HWID);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn smp_prepare_cpus(max_cpus: c_uint) -> void __init {
    void __init smp_prepare_cpus(unsigned int max_cpus)
    {
    const struct cpu_operations *ops;
    int err;
    unsigned int cpu;
    unsigned int this_cpu;
    init_cpu_topology();
    this_cpu = smp_processor_id();
    store_cpu_topology(this_cpu);
    numa_store_cpu_info(this_cpu);
    numa_add_cpu(this_cpu);
//
// If UP is mandated by "nosmp" (which implies "maxcpus=0"), don't set
// secondary CPUs present.
//
    if (max_cpus == 0)
    return;
//
// Initialise the present map (which describes the set of CPUs
// actually populated at the present time) and release the
// secondaries from the bootloader.
//
    for_each_possible_cpu(cpu) {
    if (cpu == smp_processor_id())
    continue;
    ops = get_cpu_ops(cpu);
    if (!ops)
    continue;
    err = ops.cpu_prepare(cpu);
    if (err)
    continue;
    if (acpi_disabled || acpi_cpu_is_present(cpu))
    set_cpu_present(cpu, true);
    numa_store_cpu_info(cpu);
    }
    }
    static const char *ipi_types[MAX_IPI] __tracepoint_string = {
    [IPI_RESCHEDULE]	= "Rescheduling interrupts",
    [IPI_CALL_FUNC]		= "Function call interrupts",
    [IPI_CPU_STOP]		= "CPU stop interrupts",
    [IPI_CPU_STOP_NMI]	= "CPU stop NMIs",
    [IPI_TIMER]		= "Timer broadcast interrupts",
    [IPI_IRQ_WORK]		= "IRQ work interrupts",
    [IPI_CPU_BACKTRACE]	= "CPU backtrace interrupts",
    [IPI_KGDB_ROUNDUP]	= "KGDB roundup interrupts",
    };
    static void smp_cross_call(const struct cpumask *target, unsigned int ipinr);
    unsigned long irq_err_count;
#[no_mangle]
pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: c_int) -> c_int {
    int arch_show_interrupts(struct seq_file *p, int prec)
    {
    unsigned int cpu, i;
    for (i = 0; i < MAX_IPI; i++) {
    seq_printf(p, "%*s%u: ", prec - 1, "IPI", i);
    for_each_online_cpu(cpu)
    seq_printf(p, "%10u ", irq_desc_kstat_cpu(get_ipi_desc(cpu, i), cpu));
    seq_printf(p, " %s\n", ipi_types[i]);
    }
    seq_printf(p, "%*s: %10lu\n", prec, "Err", irq_err_count);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *const cpumask) {
    void arch_send_call_function_ipi_mask(const struct cpumask *mask)
    {
    smp_cross_call(mask, IPI_CALL_FUNC);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: c_int) {
    void arch_send_call_function_single_ipi(int cpu)
    {
    smp_cross_call(cpumask_of(cpu), IPI_CALL_FUNC);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_irq_work_raise() {
    void arch_irq_work_raise(void)
    {
    smp_cross_call(cpumask_of(smp_processor_id()), IPI_IRQ_WORK);
    }

//
// arm64_nmi_cpu_stop() - stop the local CPU after it is told to stop.
// @regs: register state to record in the vmcore on a crash stop, or NULL for
// panic_smp_self_stop(), which has no interrupted context to save.
// @die_on_crash: on the kdump crash path, power the CPU off via PSCI CPU_OFF
// (so a capture kernel can reclaim it) rather than parking it.
//
// The single point every arm64 stop path funnels through, keeping the
// bookkeeping (mask interrupts, save the crash context, mark offline, mask
// SDEI, optionally power off) in one place:
//
// - the regular IPI_CPU_STOP and pseudo-NMI IPI_CPU_STOP_NMI handlers;
// - panic_smp_self_stop(), a CPU parking itself on a parallel panic();
// - the SDEI cross-CPU NMI handler (drivers/firmware/arm_sdei_nmi.c),
// which reaches CPUs the stop IPIs could not.
//
// The IPI stop handlers pass @die_on_crash true. The SDEI handler and
// panic_smp_self_stop() pass false and only park. For SDEI that is required,
// not just conservative: it runs inside an SDEI event that is deliberately
// never completed (completing it has firmware resume the wedged context), and
// a CPU_OFF from that not-yet-completed context wedges EL3 on some firmware --
// a documented follow-up. Parking also matches this path's own fallback when
// CPU_OFF is unavailable.
//
#[no_mangle]
pub unsafe extern "C" fn arm64_nmi_cpu_stop(regs: *mut pt_regs, die_on_crash: bool) -> void __noreturn {
    void __noreturn arm64_nmi_cpu_stop(struct pt_regs *regs, bool die_on_crash)
    {
    let mut cpu: c_uint = smp_processor_id();
    let mut crash: bool = IS_ENABLED(CONFIG_KEXEC_CORE) && crash_stop;
//
// Use local_daif_mask() instead of local_irq_disable() to make sure
// that pseudo-NMIs are disabled. The "stop" code starts with an IRQ
// and falls back to NMI (which might be pseudo). If the IRQ finally
// goes through right as we're timing out then the NMI could interrupt
// us. It's better to prevent the NMI and let the IRQ finish since the
// pt_regs will be better.
//
    local_daif_mask();

    if (crash && regs)
    crash_save_cpu(regs, cpu);

// the ack a stop requester (e.g. smp_send_stop()) polls for
    set_cpu_online(cpu, false);
    sdei_mask_local_cpu();
    if (crash && die_on_crash)
    __cpu_try_die(cpu);
// just in case
    cpu_park_loop();
    }
    NOKPROBE_SYMBOL(arm64_nmi_cpu_stop);
//
// We need to implement panic_smp_self_stop() for parallel panic() calls, so
// that cpu_online_mask gets correctly updated and smp_send_stop() can skip
// CPUs that have already stopped themselves.
//
#[no_mangle]
pub unsafe extern "C" fn panic_smp_self_stop() -> void __noreturn {
    void __noreturn panic_smp_self_stop(void)
    {
    arm64_nmi_cpu_stop(core::ptr::null_mut(), false);
    }
#[no_mangle]
unsafe extern "C" fn arm64_send_ipi(mask: *const cpumask_t, nr: c_uint) {
    static void arm64_send_ipi(const cpumask_t *mask, unsigned int nr)
    {
    unsigned int cpu;
    if (!percpu_ipi_descs)
    __ipi_send_mask(get_ipi_desc(0, nr), mask);
    else
    for_each_cpu(cpu, mask)
    __ipi_send_single(get_ipi_desc(cpu, nr), cpu);
    }
#[no_mangle]
unsafe extern "C" fn arm64_backtrace_ipi(mask: *mut cpumask_t) {
    static void arm64_backtrace_ipi(cpumask_t *mask)
    {
    arm64_send_ipi(mask, IPI_CPU_BACKTRACE);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: c_int) {
    void arch_trigger_cpumask_backtrace(const cpumask_t *mask, int exclude_cpu)
    {
//
// Prefer the SDEI cross-CPU NMI provider when active: firmware
// dispatches the event out of EL3 and reaches CPUs that have
// interrupts locally masked, without the per-IRQ-mask cost that
// pseudo-NMI pays for the same reach. The plain IPI path below
// can't reach such a CPU unless pseudo-NMI is enabled.
//
    if (sdei_nmi_trigger_cpumask_backtrace(mask, exclude_cpu))
    return;
//
// NOTE: though nmi_trigger_cpumask_backtrace() has "nmi_" in the name,
// nothing about it truly needs to be implemented using an NMI, it's
// just that it's _allowed_ to work with NMIs. If ipi_should_be_nmi()
// returned false our backtrace attempt will just use a regular IPI.
//
    nmi_trigger_cpumask_backtrace(mask, exclude_cpu, arm64_backtrace_ipi);
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
    __ipi_send_single(get_ipi_desc(cpu, IPI_KGDB_ROUNDUP), cpu);
    }
    }

//
// Main handler for inter-processor interrupts
//
#[no_mangle]
unsafe extern "C" fn do_handle_IPI(ipinr: c_int) {
    static void do_handle_IPI(int ipinr)
    {
    let mut cpu: c_uint = smp_processor_id();
    if ((unsigned)ipinr < NR_IPI)
    trace_ipi_entry(ipi_types[ipinr]);
    switch (ipinr) {
    case IPI_RESCHEDULE:
    scheduler_ipi();
    break;
    case IPI_CALL_FUNC:
    generic_smp_call_function_interrupt();
    break;
    case IPI_CPU_STOP:
    case IPI_CPU_STOP_NMI:
    arm64_nmi_cpu_stop(get_irq_regs(), true);
    break;

    case IPI_TIMER:
    tick_receive_broadcast();
    break;

    case IPI_IRQ_WORK:
    irq_work_run();
    break;

    case IPI_CPU_BACKTRACE:
//
// NOTE: in some cases this _won't_ be NMI context. See the
// comment in arch_trigger_cpumask_backtrace().
//
    nmi_cpu_backtrace(get_irq_regs());
    break;
    case IPI_KGDB_ROUNDUP:
    kgdb_nmicallback(cpu, get_irq_regs());
    break;
    default:
    pr_crit("CPU%u: Unknown IPI message 0x%x\n", cpu, ipinr);
    break;
    }
    if ((unsigned)ipinr < NR_IPI)
    trace_ipi_exit(ipi_types[ipinr]);
    }
#[no_mangle]
unsafe extern "C" fn ipi_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ipi_handler(int irq, void *data)
    {
    let mut ipi: c_uint = (irq - ipi_irq_base) % nr_ipi;
    do_handle_IPI(ipi);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn smp_cross_call(target: *const cpumask, ipinr: c_uint) {
    static void smp_cross_call(const struct cpumask *target, unsigned int ipinr)
    {
    trace_ipi_raise(target, ipi_types[ipinr]);
    arm64_send_ipi(target, ipinr);
    }
#[no_mangle]
unsafe extern "C" fn ipi_should_be_nmi(ipi: enum ipi_msg_type) -> bool {
    static bool ipi_should_be_nmi(enum ipi_msg_type ipi)
    {
    if (!system_uses_irq_prio_masking())
    return false;
    switch (ipi) {
    case IPI_CPU_STOP_NMI:
    case IPI_CPU_BACKTRACE:
    case IPI_KGDB_ROUNDUP:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn ipi_setup(cpu: c_int) {
    static void ipi_setup(int cpu)
    {
    int i;
    if (WARN_ON_ONCE(!ipi_irq_base))
    return;
    for (i = 0; i < nr_ipi; i++) {
    if (!percpu_ipi_descs) {
    if (ipi_should_be_nmi(i)) {
    prepare_percpu_nmi(ipi_irq_base + i);
    enable_percpu_nmi(ipi_irq_base + i, 0);
    } else {
    enable_percpu_irq(ipi_irq_base + i, 0);
    }
    } else {
    enable_irq(irq_desc_get_irq(get_ipi_desc(cpu, i)));
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn ipi_teardown(cpu: c_int) {
    static void ipi_teardown(int cpu)
    {
    int i;
    if (WARN_ON_ONCE(!ipi_irq_base))
    return;
    for (i = 0; i < nr_ipi; i++) {
    if (!percpu_ipi_descs) {
    if (ipi_should_be_nmi(i)) {
    disable_percpu_nmi(ipi_irq_base + i);
    teardown_percpu_nmi(ipi_irq_base + i);
    } else {
    disable_percpu_irq(ipi_irq_base + i);
    }
    } else {
    disable_irq_nosync(irq_desc_get_irq(get_ipi_desc(cpu, i)));
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn ipi_setup_sgi(ipi: c_int) {
    static void ipi_setup_sgi(int ipi)
    {
    int err, irq, cpu;
    irq = ipi_irq_base + ipi;
    if (ipi_should_be_nmi(ipi)) {
    err = request_percpu_nmi(irq, ipi_handler, "IPI", core::ptr::null_mut(), &irq_stat);
    WARN(err, "Could not request IRQ %d as NMI, err=%d\n", irq, err);
    } else {
    err = request_percpu_irq(irq, ipi_handler, "IPI", &irq_stat);
    WARN(err, "Could not request IRQ %d as IRQ, err=%d\n", irq, err);
    }
    for_each_possible_cpu(cpu)
    get_ipi_desc(cpu, ipi) = irq_to_desc(irq);
    irq_set_status_flags(irq, IRQ_HIDDEN);
    }
#[no_mangle]
unsafe extern "C" fn ipi_setup_lpi(ipi: c_int, ncpus: c_int) {
    static void ipi_setup_lpi(int ipi, int ncpus)
    {
    for (int cpu = 0; cpu < ncpus; cpu++) {
    int err, irq;
    irq = ipi_irq_base + (cpu * nr_ipi) + ipi;
    err = irq_force_affinity(irq, cpumask_of(cpu));
    WARN(err, "Could not force affinity IRQ %d, err=%d\n", irq, err);
    err = request_irq(irq, ipi_handler, IRQF_NO_AUTOEN, "IPI",
    core::ptr::null_mut());
    WARN(err, "Could not request IRQ %d, err=%d\n", irq, err);
    irq_set_status_flags(irq, (IRQ_HIDDEN | IRQ_NO_BALANCING_MASK));
    get_ipi_desc(cpu, ipi) = irq_to_desc(irq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_smp_ipi_range_percpu(ipi_base: c_int, n: c_int, ncpus: c_int) -> void __init {
    void __init set_smp_ipi_range_percpu(int ipi_base, int n, int ncpus)
    {
    int i;
    WARN_ON(n < MAX_IPI);
    nr_ipi = min(n, MAX_IPI);
    percpu_ipi_descs = !!ncpus;
    ipi_irq_base = ipi_base;
    for (i = 0; i < nr_ipi; i++) {
    if (!percpu_ipi_descs)
    ipi_setup_sgi(i);
    else
    ipi_setup_lpi(i, ncpus);
    }
// Setup the boot CPU immediately
    ipi_setup(smp_processor_id());
    }
#[no_mangle]
pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: c_int) {
    void arch_smp_send_reschedule(int cpu)
    {
    smp_cross_call(cpumask_of(cpu), IPI_RESCHEDULE);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_send_wakeup_ipi(cpu: c_uint) {
    void arch_send_wakeup_ipi(unsigned int cpu)
    {
//
// We use a scheduler IPI to wake the CPU as this avoids the need for a
// dedicated IPI and we can safely handle spurious scheduler IPIs.
//
    smp_send_reschedule(cpu);
    }

#[no_mangle]
pub unsafe extern "C" fn tick_broadcast(mask: *const cpumask) {
    void tick_broadcast(const struct cpumask *mask)
    {
    smp_cross_call(mask, IPI_TIMER);
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
pub unsafe extern "C" fn smp_send_stop() {
    void smp_send_stop(void)
    {
    static unsigned long stop_in_progress;
    static cpumask_t mask;
    unsigned long timeout;
//
// If this cpu is the only one alive at this point in time, online or
// not, there are no stop messages to be sent around, so just back out.
//
    if (num_other_online_cpus() == 0)
    goto skip_ipi;
// Only proceed if this is the first CPU to reach this code
    if (test_and_set_bit(0, &stop_in_progress))
    return;
//
// Send an IPI to all currently online CPUs except the CPU running
// this code.
//
// NOTE: we don't do anything here to prevent other CPUs from coming
// online after we snapshot `cpu_online_mask`. Ideally, the calling code
// should do something to prevent other CPUs from coming up. This code
// can be called in the panic path and thus it doesn't seem wise to
// grab the CPU hotplug mutex ourselves. Worst case:
// - If a CPU comes online as we're running, we'll likely notice it
// during the 1 second wait below and then we'll catch it when we try
// with an NMI (assuming NMIs are enabled) since we re-snapshot the
// mask before sending an NMI.
// - If we leave the function and see that CPUs are still online we'll
// at least print a warning. Especially without NMIs this function
// isn't foolproof anyway so calling code will just have to accept
// the fact that there could be cases where a CPU can't be stopped.
//
    cpumask_copy(&mask, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mask);
    if (system_state <= SYSTEM_RUNNING)
    pr_crit("SMP: stopping secondary CPUs\n");
//
// Start with a normal IPI and wait up to one second for other CPUs to
// stop. We do this first because it gives other processors a chance
// to exit critical sections / drop locks and makes the rest of the
// stop process (especially console flush) more robust.
//
    smp_cross_call(&mask, IPI_CPU_STOP);
    timeout = USEC_PER_SEC;
    while (num_other_online_cpus() && timeout--)
    udelay(1);
//
// If CPUs are still online, try an NMI. There's no excuse for this to
// be slow, so we only give them an extra 10 ms to respond.
//
    if (num_other_online_cpus() && ipi_should_be_nmi(IPI_CPU_STOP_NMI)) {
    smp_rmb();
    cpumask_copy(&mask, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mask);
    pr_info("SMP: retry stop with NMI for CPUs %*pbl\n",
    cpumask_pr_args(&mask));
    smp_cross_call(&mask, IPI_CPU_STOP_NMI);
    timeout = USEC_PER_MSEC * 10;
    while (num_other_online_cpus() && timeout--)
    udelay(1);
    }
//
// If CPUs are *still* online, try the SDEI cross-CPU NMI. Firmware
// delivers it regardless of the target's DAIF state, so it reaches
// a CPU spinning with interrupts masked, which neither rung above
// could (without pseudo-NMI there is no NMI rung at all). Allow
// 100ms: a firmware round-trip per CPU, with headroom.
//
    if (num_other_online_cpus() && sdei_nmi_active()) {
// re-snapshot after the rungs above took CPUs offline
    smp_rmb();
    cpumask_copy(&mask, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mask);
    pr_info("SMP: retry stop with SDEI NMI for CPUs %*pbl\n",
    cpumask_pr_args(&mask));
    sdei_nmi_stop_cpus(&mask);
    timeout = USEC_PER_MSEC * 100;
    while (num_other_online_cpus() && timeout--)
    udelay(1);
    }
    if (num_other_online_cpus()) {
    smp_rmb();
    cpumask_copy(&mask, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mask);
    pr_warn("SMP: failed to stop secondary CPUs %*pbl\n",
    cpumask_pr_args(&mask));
    }
    skip_ipi:
    sdei_mask_local_cpu();
    }

#[no_mangle]
pub unsafe extern "C" fn crash_smp_send_stop() {
    void crash_smp_send_stop(void)
    {
//
// This function can be called twice in panic path, but obviously
// we execute this only once.
//
// We use this same boolean to tell whether the IPI we send was a
// stop or a "crash stop".
//
    if (crash_stop)
    return;
    crash_stop = 1;
    smp_send_stop();
    sdei_handler_abort();
    }
#[no_mangle]
pub unsafe extern "C" fn smp_crash_stop_failed() -> bool {
    bool smp_crash_stop_failed(void)
    {
    return num_other_online_cpus() != 0;
    }

#[no_mangle]
unsafe extern "C" fn have_cpu_die() -> bool {
    static bool have_cpu_die(void)
    {

    let mut any_cpu: c_int = raw_smp_processor_id();
    const struct cpu_operations *ops = get_cpu_ops(any_cpu);
    if (ops && ops.cpu_die)
    return true;

    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn cpus_are_stuck_in_kernel() -> bool {
    bool cpus_are_stuck_in_kernel(void)
    {
    let mut smp_spin_tables: bool = (num_possible_cpus() > 1 && !have_cpu_die());
    return !!cpus_stuck_in_kernel || smp_spin_tables ||
    is_protected_kvm_enabled();
    }
