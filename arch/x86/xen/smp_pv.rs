//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/smp_pv.c
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
// Xen SMP support
//
// This file implements the Xen versions of smp_ops.  SMP under Xen is
// very straightforward.  Bringing a CPU up is simply a matter of
// loading its initial context and setting it running.
//
// IPIs are handled through the Xen event mechanism.
//
// Because virtual CPUs can be scheduled onto any real CPU, there's no
// useful topology information for the kernel to make use of.  As a
// result, all CPUs are treated as if they're single-core and
// single-threaded.
//

    cpumask_var_t xen_cpu_initialized_map;
    static DEFINE_PER_CPU(struct xen_common_irq, xen_irq_work) = { .irq = -1 };
    static DEFINE_PER_CPU(struct xen_common_irq, xen_pmu_irq) = { .irq = -1 };
    static irqreturn_t xen_irq_work_interrupt(int irq, void *dev_id);
#[no_mangle]
unsafe extern "C" fn cpu_bringup() {
    static void cpu_bringup(void)
    {
    int cpu;
    cr4_init();
    cpuhp_ap_sync_alive();
    cpu_init();
    fpu__init_cpu();
    touch_softlockup_watchdog();
// PVH runs in ring 0 and allows us to do native syscalls. Yay!
    if (!xen_feature(XENFEAT_supervisor_mode_kernel))
    xen_enable_syscall();
    cpu = smp_processor_id();
    identify_secondary_cpu(cpu);
    set_cpu_sibling_map(cpu);
    speculative_store_bypass_ht_init();
    xen_setup_cpu_clockevents();
    notify_cpu_starting(cpu);
    set_cpu_online(cpu, true);
    smp_mb();
// We can take interrupts now: we're officially "up".
    local_irq_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_bringup_and_idle() -> asmlinkage __visible void {
    asmlinkage __visible void cpu_bringup_and_idle(void)
    {
    cpu_bringup();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_intr_free_pv(cpu: c_uint) {
    void xen_smp_intr_free_pv(unsigned int cpu)
    {
    kfree(per_cpu(xen_irq_work, cpu).name);
    per_cpu(xen_irq_work, cpu).name = core::ptr::null_mut();
    if (per_cpu(xen_irq_work, cpu).irq >= 0) {
    unbind_from_irqhandler(per_cpu(xen_irq_work, cpu).irq, core::ptr::null_mut());
    per_cpu(xen_irq_work, cpu).irq = -1;
    }
    kfree(per_cpu(xen_pmu_irq, cpu).name);
    per_cpu(xen_pmu_irq, cpu).name = core::ptr::null_mut();
    if (per_cpu(xen_pmu_irq, cpu).irq >= 0) {
    unbind_from_irqhandler(per_cpu(xen_pmu_irq, cpu).irq, core::ptr::null_mut());
    per_cpu(xen_pmu_irq, cpu).irq = -1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_intr_init_pv(cpu: c_uint) -> c_int {
    int xen_smp_intr_init_pv(unsigned int cpu)
    {
    int rc;
    char *callfunc_name, *pmu_name;
    callfunc_name = kasprintf(GFP_KERNEL, "irqwork%d", cpu);
    per_cpu(xen_irq_work, cpu).name = callfunc_name;
    rc = bind_ipi_to_irqhandler(XEN_IRQ_WORK_VECTOR,
    cpu,
    xen_irq_work_interrupt,
    IRQF_PERCPU|IRQF_NOBALANCING,
    callfunc_name,
    core::ptr::null_mut());
    if (rc < 0)
    goto fail;
    per_cpu(xen_irq_work, cpu).irq = rc;
    if (is_xen_pmu) {
    pmu_name = kasprintf(GFP_KERNEL, "pmu%d", cpu);
    per_cpu(xen_pmu_irq, cpu).name = pmu_name;
    rc = bind_virq_to_irqhandler(VIRQ_XENPMU, cpu,
    xen_pmu_irq_handler,
    IRQF_PERCPU|IRQF_NOBALANCING,
    pmu_name, core::ptr::null_mut());
    if (rc < 0)
    goto fail;
    per_cpu(xen_pmu_irq, cpu).irq = rc;
    }
    return 0;
    fail:
    xen_smp_intr_free_pv(cpu);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_smp_config() -> void __init {
    static void __init xen_pv_smp_config(void)
    {
    let mut apicid: u32 = 0;
    int i;
    topology_register_boot_apic(apicid);
    for (i = 0; i < nr_cpu_ids; i++)
    topology_register_apic(apicid++, CPU_ACPIID_INVALID, true);
// Pretend to be a proper enumerated system
    smp_found_config = 1;
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_smp_prepare_boot_cpu() -> void __init {
    static void __init xen_pv_smp_prepare_boot_cpu(void)
    {
    BUG_ON(smp_processor_id() != 0);
    native_smp_prepare_boot_cpu();
    if (!xen_feature(XENFEAT_writable_page_tables))
// We've switched to the "real" per-cpu gdt, so make
// sure the old memory can be recycled.
    make_lowmem_page_readwrite(xen_initial_gdt);
    xen_setup_vcpu_info_placement();
//
// The alternative logic (which patches the unlock/lock) runs before
// the smp bootup up code is activated. Hence we need to set this up
// the core kernel is being patched. Otherwise we will have only
// modules patched but not core code.
//
    xen_init_spinlocks();
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_smp_prepare_cpus(max_cpus: c_uint) -> void __init {
    static void __init xen_pv_smp_prepare_cpus(unsigned int max_cpus)
    {
    unsigned cpu;
    if (ioapic_is_disabled) {
    char *m = (max_cpus == 0) ?
    "The nosmp parameter is incompatible with Xen; " \
    "use Xen dom0_max_vcpus=1 parameter" :
    "The noapic parameter is incompatible with Xen";
    xen_raw_printk(m);
    panic(m);
    }
    xen_init_lock_cpu(0);
    smp_prepare_cpus_common();
    speculative_store_bypass_ht_init();
    xen_pmu_init(0);
    if (xen_smp_intr_init(0) || xen_smp_intr_init_pv(0))
    BUG();
    if (!alloc_cpumask_var(&xen_cpu_initialized_map, GFP_KERNEL))
    panic("could not allocate xen_cpu_initialized_map\n");
    cpumask_copy(xen_cpu_initialized_map, cpumask_of(0));
// Restrict the possible_map according to max_cpus.
    while ((num_possible_cpus() > 1) && (num_possible_cpus() > max_cpus)) {
    for (cpu = nr_cpu_ids - 1; !cpu_possible(cpu); cpu--)
    continue;
    set_cpu_possible(cpu, false);
    }
    for_each_possible_cpu(cpu)
    set_cpu_present(cpu, true);
    }
    static int
    cpu_initialize_context(unsigned int cpu, struct task_struct *idle)
    {
    struct vcpu_guest_context *ctxt;
    struct desc_struct *gdt;
    unsigned long gdt_mfn;
    if (cpumask_test_and_set_cpu(cpu, xen_cpu_initialized_map))
    return 0;
    ctxt = kzalloc_obj(*ctxt);
    if (ctxt == core::ptr::null_mut()) {
    cpumask_clear_cpu(cpu, xen_cpu_initialized_map);
    return -ENOMEM;
    }
    gdt = get_cpu_gdt_rw(cpu);
//
// Bring up the CPU in cpu_bringup_and_idle() with the stack
// pointing just below where pt_regs would be if it were a normal
// kernel entry.
//
    ctxt.user_regs.eip = (unsigned long)asm_cpu_bringup_and_idle;
    ctxt.flags = VGCF_IN_KERNEL;
    ctxt.user_regs.eflags = 0x1000; /* IOPL_RING1 */
    ctxt.user_regs.ds = __USER_DS;
    ctxt.user_regs.es = __USER_DS;
    ctxt.user_regs.ss = __KERNEL_DS;
    ctxt.user_regs.cs = __KERNEL_CS;
    ctxt.user_regs.esp = (unsigned long)task_pt_regs(idle);
    xen_copy_trap_info(ctxt.trap_ctxt);
    BUG_ON((unsigned long)gdt & ~PAGE_MASK);
    gdt_mfn = arbitrary_virt_to_mfn(gdt);
    make_lowmem_page_readonly(gdt);
    make_lowmem_page_readonly(mfn_to_virt(gdt_mfn));
    ctxt.gdt_frames[0] = gdt_mfn;
    ctxt.gdt_ents      = GDT_ENTRIES;
//
// Set SS:SP that Xen will use when entering guest kernel mode
// from guest user mode.  Subsequent calls to load_sp0() can
// change this value.
//
    ctxt.kernel_ss = __KERNEL_DS;
    ctxt.kernel_sp = task_top_of_stack(idle);
    ctxt.gs_base_kernel = per_cpu_offset(cpu);
    ctxt.event_callback_eip    =
    (unsigned long)xen_asm_exc_xen_hypervisor_callback;
    ctxt.failsafe_callback_eip =
    (unsigned long)xen_failsafe_callback;
    per_cpu(xen_cr3, cpu) = __pa(swapper_pg_dir);
    ctxt.ctrlreg[3] = xen_pfn_to_cr3(virt_to_gfn(swapper_pg_dir));
    if (HYPERVISOR_vcpu_op(VCPUOP_initialise, xen_vcpu_nr(cpu), ctxt))
    BUG();
    kfree(ctxt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_kick_ap(cpu: c_uint, idle: *mut task_struct) -> c_int {
    static int xen_pv_kick_ap(unsigned int cpu, struct task_struct *idle)
    {
    int rc;
    rc = common_cpu_up(cpu, idle);
    if (rc)
    return rc;
    xen_setup_runstate_info(cpu);
// make sure interrupts start blocked
    per_cpu(xen_vcpu, cpu).evtchn_upcall_mask = 1;
    rc = cpu_initialize_context(cpu, idle);
    if (rc)
    return rc;
    xen_pmu_init(cpu);
//
// Why is this a BUG? If the hypercall fails then everything can be
// rolled back, no?
//
    BUG_ON(HYPERVISOR_vcpu_op(VCPUOP_up, xen_vcpu_nr(cpu), core::ptr::null_mut()));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_poll_sync_state() {
    static void xen_pv_poll_sync_state(void)
    {
    HYPERVISOR_sched_op(SCHEDOP_yield, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn xen_pv_cpu_disable() -> c_int {
    static int xen_pv_cpu_disable(void)
    {
    let mut cpu: c_uint = smp_processor_id();
    if (cpu == 0)
    return -EBUSY;
    cpu_disable_common();
    load_cr3(swapper_pg_dir);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_cpu_die(cpu: c_uint) {
    static void xen_pv_cpu_die(unsigned int cpu)
    {
    while (HYPERVISOR_vcpu_op(VCPUOP_is_up, xen_vcpu_nr(cpu), core::ptr::null_mut())) {
    __set_current_state(TASK_UNINTERRUPTIBLE);
    schedule_timeout(HZ/10);
    }
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_cleanup_dead_cpu(cpu: c_uint) {
    static void xen_pv_cleanup_dead_cpu(unsigned int cpu)
    {
    xen_smp_intr_free(cpu);
    xen_uninit_lock_cpu(cpu);
    xen_teardown_timer(cpu);
    xen_pmu_finish(cpu);
    }
    static void __noreturn xen_pv_play_dead(void) /* used only with HOTPLUG_CPU */
    {
    play_dead_common();
    HYPERVISOR_vcpu_op(VCPUOP_down, xen_vcpu_nr(smp_processor_id()), core::ptr::null_mut());
    xen_cpu_bringup_again((unsigned long)task_pt_regs(current));
    BUG();
    }

#[no_mangle]
unsafe extern "C" fn xen_pv_cpu_disable() -> c_int {
    static int xen_pv_cpu_disable(void)
    {
    return -ENOSYS;
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_cpu_die(cpu: c_uint) {
    static void xen_pv_cpu_die(unsigned int cpu)
    {
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_cleanup_dead_cpu(cpu: c_uint) {
    static void xen_pv_cleanup_dead_cpu(unsigned int cpu)
    {
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_play_dead() -> void __noreturn {
    static void __noreturn xen_pv_play_dead(void)
    {
    BUG();
    }

#[no_mangle]
unsafe extern "C" fn stop_self(v: *mut c_void) {
    static void stop_self(void *v)
    {
    let mut cpu: c_int = smp_processor_id();
// make sure we're not pinning something down
    load_cr3(swapper_pg_dir);
// should set up a minimal gdt
    set_cpu_online(cpu, false);
    HYPERVISOR_vcpu_op(VCPUOP_down, xen_vcpu_nr(cpu), core::ptr::null_mut());
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn xen_pv_stop_other_cpus(wait: c_int) {
    static void xen_pv_stop_other_cpus(int wait)
    {
    smp_call_function(stop_self, core::ptr::null_mut(), wait);
    }
#[no_mangle]
unsafe extern "C" fn xen_irq_work_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xen_irq_work_interrupt(int irq, void *dev_id)
    {
    irq_work_run();
    inc_irq_stat(IRQ_WORK);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_smp_count_cpus() -> void __init {
    void __init xen_smp_count_cpus(void)
    {
    unsigned int cpus;
    for (cpus = 0; cpus < nr_cpu_ids; cpus++) {
    if (HYPERVISOR_vcpu_op(VCPUOP_is_up, cpus, core::ptr::null_mut()) < 0)
    break;
    }
    pr_info("Xen PV: Detected %u vCPUS\n", cpus);
    if (cpus < nr_cpu_ids)
    set_nr_cpu_ids(cpus);
    }
    static const struct smp_ops xen_smp_ops __initconst = {
    .smp_prepare_boot_cpu = xen_pv_smp_prepare_boot_cpu,
    .smp_prepare_cpus = xen_pv_smp_prepare_cpus,
    .smp_cpus_done = xen_smp_cpus_done,
    .kick_ap_alive = xen_pv_kick_ap,
    .cpu_die = xen_pv_cpu_die,
    .cleanup_dead_cpu = xen_pv_cleanup_dead_cpu,
    .poll_sync_state = xen_pv_poll_sync_state,
    .cpu_disable = xen_pv_cpu_disable,
    .play_dead = xen_pv_play_dead,
    .stop_other_cpus = xen_pv_stop_other_cpus,
    .smp_send_reschedule = xen_smp_send_reschedule,
    .send_call_func_ipi = xen_smp_send_call_function_ipi,
    .send_call_func_single_ipi = xen_smp_send_call_function_single_ipi,
    };
#[no_mangle]
pub unsafe extern "C" fn xen_smp_init() -> void __init {
    void __init xen_smp_init(void)
    {
    smp_ops = xen_smp_ops;
// Avoid searching for BIOS MP tables
    x86_init.mpparse.find_mptable		= x86_init_noop;
    x86_init.mpparse.early_parse_smp_cfg	= x86_init_noop;
// XEN/PV Dom0 has halfways sane topology information via CPUID/MADT
    if (xen_initial_domain())
    x86_init.mpparse.parse_smp_cfg	= x86_init_noop;
    else
    x86_init.mpparse.parse_smp_cfg	= xen_pv_smp_config;
    }
