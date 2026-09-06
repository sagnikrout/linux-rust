//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mshyperv.c
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
// HyperV  Detection code.
//
// Copyright (C) 2010, Novell, Inc.
// Author : K. Y. Srinivasan <ksrinivasan@novell.com>
//

// Is Linux running on nested Microsoft Hypervisor
    bool hv_nested;
    struct ms_hyperv_info ms_hyperv;

//
// When running with the paravisor, controls proxying the synthetic interrupts
// from the host
//
    static bool hv_para_sint_proxy;
#[no_mangle]
pub unsafe extern "C" fn hv_get_nested_msr(reg: c_uint) -> c_uint {
    static inline unsigned int hv_get_nested_msr(unsigned int reg)
    {
    if (hv_is_sint_msr(reg))
    return reg - HV_X64_MSR_SINT0 + HV_X64_MSR_NESTED_SINT0;
    switch (reg) {
    case HV_X64_MSR_SIMP:
    return HV_X64_MSR_NESTED_SIMP;
    case HV_X64_MSR_SIEFP:
    return HV_X64_MSR_NESTED_SIEFP;
    case HV_X64_MSR_SVERSION:
    return HV_X64_MSR_NESTED_SVERSION;
    case HV_X64_MSR_SCONTROL:
    return HV_X64_MSR_NESTED_SCONTROL;
    case HV_X64_MSR_EOM:
    return HV_X64_MSR_NESTED_EOM;
    default:
    return reg;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hv_get_non_nested_msr(reg: c_uint) -> u64 {
    u64 hv_get_non_nested_msr(unsigned int reg)
    {
    u64 value;
    if (hv_is_synic_msr(reg) && ms_hyperv.paravisor_present)
    hv_ivm_msr_read(reg, &value);
    else
    rdmsrq(reg, value);
    return value;
    }
    EXPORT_SYMBOL_GPL(hv_get_non_nested_msr);
#[no_mangle]
pub unsafe extern "C" fn hv_set_non_nested_msr(reg: c_uint, value: u64) {
    void hv_set_non_nested_msr(unsigned int reg, u64 value)
    {
    if (hv_is_synic_msr(reg) && ms_hyperv.paravisor_present) {
// The hypervisor will get the intercept.
    hv_ivm_msr_write(reg, value);
// Using wrmsrq so the following goes to the paravisor.
    if (hv_is_sint_msr(reg)) {
    let mut sint: union hv_synic_sint = { .as_uint64 = value };
    sint.proxy = hv_para_sint_proxy;
    native_wrmsrq(reg, sint.as_uint64);
    }
    } else {
    native_wrmsrq(reg, value);
    }
    }
    EXPORT_SYMBOL_GPL(hv_set_non_nested_msr);
//
// Enable or disable proxying synthetic interrupts
// to the paravisor.
//
#[no_mangle]
pub unsafe extern "C" fn hv_para_set_sint_proxy(enable: bool) {
    void hv_para_set_sint_proxy(bool enable)
    {
    hv_para_sint_proxy = enable;
    }
//
// Get the SynIC register value from the paravisor.
//
#[no_mangle]
pub unsafe extern "C" fn hv_para_get_synic_register(reg: c_uint) -> u64 {
    u64 hv_para_get_synic_register(unsigned int reg)
    {
    if (WARN_ON(!ms_hyperv.paravisor_present || !hv_is_synic_msr(reg)))
    return ~0ULL;
    return native_read_msr(reg);
    }
//
// Set the SynIC register value with the paravisor.
//
#[no_mangle]
pub unsafe extern "C" fn hv_para_set_synic_register(reg: c_uint, val: u64) {
    void hv_para_set_synic_register(unsigned int reg, u64 val)
    {
    if (WARN_ON(!ms_hyperv.paravisor_present || !hv_is_synic_msr(reg)))
    return;
    native_write_msr(reg, val);
    }
#[no_mangle]
pub unsafe extern "C" fn hv_get_msr(reg: c_uint) -> u64 {
    u64 hv_get_msr(unsigned int reg)
    {
    if (hv_nested)
    reg = hv_get_nested_msr(reg);
    return hv_get_non_nested_msr(reg);
    }
    EXPORT_SYMBOL_GPL(hv_get_msr);
#[no_mangle]
pub unsafe extern "C" fn hv_set_msr(reg: c_uint, value: u64) {
    void hv_set_msr(unsigned int reg, u64 value)
    {
    if (hv_nested)
    reg = hv_get_nested_msr(reg);
    hv_set_non_nested_msr(reg, value);
    }
    EXPORT_SYMBOL_GPL(hv_set_msr);
    static void (*mshv_handler)(void);
    static void (*vmbus_handler)(void);
    static void (*hv_stimer0_handler)(void);
    static void (*hv_kexec_handler)(void);
    static void (*hv_crash_handler)(struct pt_regs *regs);
    DEFINE_IDTENTRY_SYSVEC(sysvec_hyperv_callback)
    {
    struct pt_regs *old_regs = set_irq_regs(regs);
    inc_irq_stat(HYPERVISOR_CALLBACK);
    if (mshv_handler)
    mshv_handler();
    if (vmbus_handler)
    vmbus_handler();
    add_interrupt_randomness(HYPERVISOR_CALLBACK_VECTOR);
    if (ms_hyperv.hints & HV_DEPRECATING_AEOI_RECOMMENDED)
    apic_eoi();
    set_irq_regs(old_regs);
    }
#[no_mangle]
pub unsafe extern "C" fn hv_setup_mshv_handler((*handler)(void): *mut c_void) {
    void hv_setup_mshv_handler(void (*handler)(void))
    {
    mshv_handler = handler;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_setup_vmbus_handler((*handler)(void): *mut c_void) {
    void hv_setup_vmbus_handler(void (*handler)(void))
    {
    vmbus_handler = handler;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_remove_vmbus_handler() {
    void hv_remove_vmbus_handler(void)
    {
// We have no way to deallocate the interrupt gate
    vmbus_handler = core::ptr::null_mut();
    }
//
// Routines to do per-architecture handling of stimer0
// interrupts when in Direct Mode
//
    DEFINE_IDTENTRY_SYSVEC(sysvec_hyperv_stimer0)
    {
    struct pt_regs *old_regs = set_irq_regs(regs);
    inc_irq_stat(HYPERV_STIMER0);
    if (hv_stimer0_handler)
    hv_stimer0_handler();
    add_interrupt_randomness(HYPERV_STIMER0_VECTOR);
    apic_eoi();
    set_irq_regs(old_regs);
    }
// For x86/x64, override weak placeholders in hyperv_timer.c
#[no_mangle]
pub unsafe extern "C" fn hv_setup_stimer0_handler((*handler)(void): *mut c_void) {
    void hv_setup_stimer0_handler(void (*handler)(void))
    {
    hv_stimer0_handler = handler;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_remove_stimer0_handler() {
    void hv_remove_stimer0_handler(void)
    {
// We have no way to deallocate the interrupt gate
    hv_stimer0_handler = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn hv_setup_kexec_handler((*handler)(void): *mut c_void) {
    void hv_setup_kexec_handler(void (*handler)(void))
    {
    hv_kexec_handler = handler;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_remove_kexec_handler() {
    void hv_remove_kexec_handler(void)
    {
    hv_kexec_handler = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn hv_setup_crash_handler(regs): *mut *mut void (handler)(struct pt_regs) {
    void hv_setup_crash_handler(void (*handler)(struct pt_regs *regs))
    {
    hv_crash_handler = handler;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_remove_crash_handler() {
    void hv_remove_crash_handler(void)
    {
    hv_crash_handler = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn hv_machine_shutdown() {
    static void hv_machine_shutdown(void)
    {
    if (kexec_in_progress) {
    hv_stimer_global_cleanup();
    if (hv_kexec_handler)
    hv_kexec_handler();
    }
//
// Call hv_cpu_die() on all the CPUs, otherwise later the hypervisor
// corrupts the old VP Assist Pages and can crash the kexec kernel.
//
    if (kexec_in_progress)
    cpuhp_remove_state(CPUHP_AP_HYPERV_ONLINE);
// The function calls stop_other_cpus().
    native_machine_shutdown();
// Disable the hypercall page when there is only 1 active CPU.
    if (kexec_in_progress)
    hyperv_cleanup();
    }

#[no_mangle]
unsafe extern "C" fn hv_guest_crash_shutdown(regs: *mut pt_regs) {
    static void hv_guest_crash_shutdown(struct pt_regs *regs)
    {
    if (hv_crash_handler)
    hv_crash_handler(regs);
// The function calls crash_smp_send_stop().
    native_machine_crash_shutdown(regs);
// Disable the hypercall page when there is only 1 active CPU.
    hyperv_cleanup();
    }

    static u64 hv_ref_counter_at_suspend;
    static void (*old_save_sched_clock_state)(void);
    static void (*old_restore_sched_clock_state)(void);
//
// Hyper-V clock counter resets during hibernation. Save and restore clock
// offset during suspend/resume, while also considering the time passed
// before suspend. This is to make sure that sched_clock using hv tsc page
// based clocksource, proceeds from where it left off during suspend and
// it shows correct time for the timestamps of kernel messages after resume.
//
#[no_mangle]
unsafe extern "C" fn save_hv_clock_tsc_state() {
    static void save_hv_clock_tsc_state(void)
    {
    hv_ref_counter_at_suspend = hv_read_reference_counter();
    }
#[no_mangle]
unsafe extern "C" fn restore_hv_clock_tsc_state() {
    static void restore_hv_clock_tsc_state(void)
    {
//
// Adjust the offsets used by hv tsc clocksource to
// account for the time spent before hibernation.
// adjusted value = reference counter (time) at suspend
// - reference counter (time) now.
//
    hv_adj_sched_clock_offset(hv_ref_counter_at_suspend - hv_read_reference_counter());
    }
//
// Functions to override save_sched_clock_state and restore_sched_clock_state
// functions of x86_platform. The Hyper-V clock counter is reset during
// suspend-resume and the offset used to measure time needs to be
// corrected, post resume.
//
#[no_mangle]
unsafe extern "C" fn hv_save_sched_clock_state() {
    static void hv_save_sched_clock_state(void)
    {
    old_save_sched_clock_state();
    save_hv_clock_tsc_state();
    }
#[no_mangle]
unsafe extern "C" fn hv_restore_sched_clock_state() {
    static void hv_restore_sched_clock_state(void)
    {
    restore_hv_clock_tsc_state();
    old_restore_sched_clock_state();
    }
#[no_mangle]
unsafe extern "C" fn x86_setup_ops_for_tsc_pg_clock() -> void __init {
    static void __init x86_setup_ops_for_tsc_pg_clock(void)
    {
    if (!(ms_hyperv.features & HV_MSR_REFERENCE_TSC_AVAILABLE))
    return;
    old_save_sched_clock_state = x86_platform.save_sched_clock_state;
    x86_platform.save_sched_clock_state = hv_save_sched_clock_state;
    old_restore_sched_clock_state = x86_platform.restore_sched_clock_state;
    x86_platform.restore_sched_clock_state = hv_restore_sched_clock_state;
    }

    DEFINE_STATIC_CALL(hv_hypercall, hv_std_hypercall);
    EXPORT_STATIC_CALL_TRAMP_GPL(hv_hypercall);

#[no_mangle]
unsafe extern "C" fn ms_hyperv_platform() -> uint32_t  __init {
    static uint32_t  __init ms_hyperv_platform(void)
    {
    u32 eax;
    u32 hyp_signature[3];
    if (!boot_cpu_has(X86_FEATURE_HYPERVISOR))
    return 0;
    cpuid(HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS,
    &eax, &hyp_signature[0], &hyp_signature[1], &hyp_signature[2]);
    if (eax < HYPERV_CPUID_MIN || eax > HYPERV_CPUID_MAX ||
    memcmp("Microsoft Hv", hyp_signature, 12))
    return 0;
// HYPERCALL and VP_INDEX MSRs are mandatory for all features.
    eax = cpuid_eax(HYPERV_CPUID_FEATURES);
    if (!(eax & HV_MSR_HYPERCALL_AVAILABLE)) {
    pr_warn("x86/hyperv: HYPERCALL MSR not available.\n");
    return 0;
    }
    if (!(eax & HV_MSR_VP_INDEX_AVAILABLE)) {
    pr_warn("x86/hyperv: VP_INDEX MSR not available.\n");
    return 0;
    }
    return HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS;
    }

//
// Prior to WS2016 Debug-VM sends NMIs to all CPUs which makes
// it difficult to process CHANNELMSG_UNLOAD in case of crash. Handle
// unknown NMI on the first CPU which gets it.
//
#[no_mangle]
unsafe extern "C" fn hv_nmi_unknown(val: c_uint, regs: *mut pt_regs) -> c_int {
    static int hv_nmi_unknown(unsigned int val, struct pt_regs *regs)
    {
    let mut nmi_cpu: static atomic_t = ATOMIC_INIT(-1);
    unsigned int old_cpu, this_cpu;
    if (!unknown_nmi_panic)
    return NMI_DONE;
    old_cpu = -1;
    this_cpu = raw_smp_processor_id();
    if (!atomic_try_cmpxchg(&nmi_cpu, &old_cpu, this_cpu))
    return NMI_HANDLED;
    return NMI_DONE;
    }

#[no_mangle]
unsafe extern "C" fn hv_get_tsc_khz() -> c_ulong {
    static unsigned long hv_get_tsc_khz(void)
    {
    unsigned long freq;
    rdmsrq(HV_X64_MSR_TSC_FREQUENCY, freq);
    return freq / 1000;
    }

#[no_mangle]
unsafe extern "C" fn hv_smp_prepare_boot_cpu() -> void __init {
    static void __init hv_smp_prepare_boot_cpu(void)
    {
    native_smp_prepare_boot_cpu();

    hv_init_spinlocks();

    }
#[no_mangle]
unsafe extern "C" fn hv_smp_prepare_cpus(max_cpus: c_uint) -> void __init {
    static void __init hv_smp_prepare_cpus(unsigned int max_cpus)
    {

    int i;
    int ret;

    native_smp_prepare_cpus(max_cpus);
//
// Override wakeup_secondary_cpu_64 callback for SEV-SNP
// enlightened guest.
//
    if (!ms_hyperv.paravisor_present && hv_isolation_type_snp()) {
    apic.wakeup_secondary_cpu_64 = hv_snp_boot_ap;
    return;
    }

// If AP LPs exist, we are in a kexec'd kernel and VPs already exist
    if (num_present_cpus() == 1 || hv_lp_exists(1))
    return;
    for_each_present_cpu(i) {
    if (i == 0)
    continue;
    ret = hv_call_add_logical_proc(numa_cpu_node(i), i, cpu_physical_id(i));
    BUG_ON(ret);
    }
    ret = hv_call_notify_all_processors_started();
    WARN_ON(ret);
    for_each_present_cpu(i) {
    if (i == 0)
    continue;
    ret = hv_call_create_vp(numa_cpu_node(i), hv_current_partition_id, i, i);
    BUG_ON(ret);
    }

    }

//
// When a fully enlightened TDX VM runs on Hyper-V, the firmware sets the
// HW_REDUCED flag: refer to acpi_tb_create_local_fadt(). Consequently ttyS0
// interrupts can't work because request_irq() -> ... -> irq_to_desc() returns
// NULL for ttyS0. This happens because mp_config_acpi_legacy_irqs() sees a
// nr_legacy_irqs() of 0, so it doesn't initialize the array 'mp_irqs[]', and
// later setup_IO_APIC_irqs() -> find_irq_entry() fails to find the legacy irqs
// from the array and hence doesn't create the necessary irq description info.
//
// Clone arch/x86/kernel/acpi/boot.c: acpi_generic_reduced_hw_init() here,
// except don't change 'legacy_pic', which keeps its default value
// 'default_legacy_pic'. This way, mp_config_acpi_legacy_irqs() sees a non-zero
// nr_legacy_irqs() and eventually serial console interrupts works properly.
//
#[no_mangle]
unsafe extern "C" fn reduced_hw_init() -> void __init {
    static void __init reduced_hw_init(void)
    {
    x86_init.timers.timer_init	= x86_init_noop;
    x86_init.irqs.pre_vector_init	= x86_init_noop;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_get_hypervisor_version(info: *mut union hv_hypervisor_version_info) -> c_int {
    int hv_get_hypervisor_version(union hv_hypervisor_version_info *info)
    {
    unsigned int hv_max_functions;
    hv_max_functions = cpuid_eax(HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS);
    if (hv_max_functions < HYPERV_CPUID_VERSION) {
    pr_err("%s: Could not detect Hyper-V version\n", __func__);
    return -ENODEV;
    }
    cpuid(HYPERV_CPUID_VERSION, &info.eax, &info.ebx, &info.ecx, &info.edx);
    return 0;
    }
    EXPORT_SYMBOL_GPL(hv_get_hypervisor_version);
//
// Reserved vectors hard coded in the hypervisor. If used outside, the hypervisor
// will either crash or hang or attempt to break into debugger.
//
#[no_mangle]
unsafe extern "C" fn hv_reserve_irq_vectors() {
    static void hv_reserve_irq_vectors(void)
    {
pub const HYPERV_DBG_FASTFAIL_VECTOR: c_uint = 0x29;
pub const HYPERV_DBG_ASSERT_VECTOR: c_uint = 0x2C;
pub const HYPERV_DBG_SERVICE_VECTOR: c_uint = 0x2D;
//
// The hypervisor delivers these three to the NT HAL and refuses to
// map a device interrupt to any of them.
//
// The hypervisor will provide a hint in the future when these
// vectors become available to use.
//
pub const HAL_NT_APC_VECTOR: c_uint = 0x1F;
pub const HAL_NT_DPC_VECTOR: c_uint = 0x2F;
pub const HAL_NT_CLOCK_IPI_VECTOR: c_uint = 0xD2;
    if (cpu_feature_enabled(X86_FEATURE_FRED))
    return;
    if (test_and_set_bit(HYPERV_DBG_ASSERT_VECTOR, system_vectors) ||
    test_and_set_bit(HYPERV_DBG_SERVICE_VECTOR, system_vectors) ||
    test_and_set_bit(HYPERV_DBG_FASTFAIL_VECTOR, system_vectors) ||
    test_and_set_bit(HAL_NT_APC_VECTOR, system_vectors) ||
    test_and_set_bit(HAL_NT_DPC_VECTOR, system_vectors) ||
    test_and_set_bit(HAL_NT_CLOCK_IPI_VECTOR, system_vectors))
    BUG();
    pr_info("Hyper-V: reserve vectors: 0x%x 0x%x 0x%x 0x%x 0x%x 0x%x\n",
    HYPERV_DBG_ASSERT_VECTOR, HYPERV_DBG_SERVICE_VECTOR,
    HYPERV_DBG_FASTFAIL_VECTOR, HAL_NT_APC_VECTOR,
    HAL_NT_DPC_VECTOR, HAL_NT_CLOCK_IPI_VECTOR);
    }
#[no_mangle]
unsafe extern "C" fn ms_hyperv_init_platform() -> void __init {
    static void __init ms_hyperv_init_platform(void)
    {
    int hv_max_functions_eax, eax;

    pv_info.name = "Hyper-V";

//
// Extract the features and hints
//
    ms_hyperv.features = cpuid_eax(HYPERV_CPUID_FEATURES);
    ms_hyperv.priv_high = cpuid_ebx(HYPERV_CPUID_FEATURES);
    ms_hyperv.ext_features = cpuid_ecx(HYPERV_CPUID_FEATURES);
    ms_hyperv.misc_features = cpuid_edx(HYPERV_CPUID_FEATURES);
    ms_hyperv.hints    = cpuid_eax(HYPERV_CPUID_ENLIGHTMENT_INFO);
    hv_max_functions_eax = cpuid_eax(HYPERV_CPUID_VENDOR_AND_MAX_FUNCTIONS);
    pr_info("Hyper-V: privilege flags low %#x, high %#x, ext %#x, hints %#x, misc %#x\n",
    ms_hyperv.features, ms_hyperv.priv_high,
    ms_hyperv.ext_features, ms_hyperv.hints,
    ms_hyperv.misc_features);
    ms_hyperv.max_vp_index = cpuid_eax(HYPERV_CPUID_IMPLEMENT_LIMITS);
    ms_hyperv.max_lp_index = cpuid_ebx(HYPERV_CPUID_IMPLEMENT_LIMITS);
    pr_debug("Hyper-V: max %u virtual processors, %u logical processors\n",
    ms_hyperv.max_vp_index, ms_hyperv.max_lp_index);
    hv_identify_partition_type();
    if (hv_root_partition())
    hv_reserve_irq_vectors();
    if (cc_platform_has(CC_ATTR_SNP_SECURE_AVIC))
    ms_hyperv.hints |= HV_DEPRECATING_AEOI_RECOMMENDED;
    if (ms_hyperv.hints & HV_X64_HYPERV_NESTED) {
    hv_nested = true;
    pr_info("Hyper-V: running on a nested hypervisor\n");
    }
//
// There is no check against the max function for HYPERV_CPUID_VIRT_STACK_* CPUID
// leaves as the hypervisor doesn't handle them. Even a nested root partition (L2
// root) will not get them because the nested (L1) hypervisor filters them out.
// These are handled through intercept processing by the Windows Hyper-V stack
// or the paravisor.
//
    eax = cpuid_eax(HYPERV_CPUID_VIRT_STACK_PROPERTIES);
    ms_hyperv.confidential_vmbus_available =
    eax & HYPERV_VS_PROPERTIES_EAX_CONFIDENTIAL_VMBUS_AVAILABLE;
    ms_hyperv.msi_ext_dest_id =
    eax & HYPERV_VS_PROPERTIES_EAX_EXTENDED_IOAPIC_RTE;
    if (ms_hyperv.features & HV_ACCESS_FREQUENCY_MSRS &&
    ms_hyperv.misc_features & HV_FEATURE_FREQUENCY_MSRS_AVAILABLE) {
    x86_platform.calibrate_tsc = hv_get_tsc_khz;
    x86_platform.calibrate_cpu = hv_get_tsc_khz;
    setup_force_cpu_cap(X86_FEATURE_TSC_KNOWN_FREQ);
    }
    if (ms_hyperv.priv_high & HV_ISOLATION) {
    ms_hyperv.isolation_config_a = cpuid_eax(HYPERV_CPUID_ISOLATION_CONFIG);
    ms_hyperv.isolation_config_b = cpuid_ebx(HYPERV_CPUID_ISOLATION_CONFIG);
    if (ms_hyperv.shared_gpa_boundary_active)
    ms_hyperv.shared_gpa_boundary =
    BIT_ULL(ms_hyperv.shared_gpa_boundary_bits);
    pr_info("Hyper-V: Isolation Config: Group A 0x%x, Group B 0x%x\n",
    ms_hyperv.isolation_config_a, ms_hyperv.isolation_config_b);
    if (hv_get_isolation_type() == HV_ISOLATION_TYPE_SNP) {
    static_branch_enable(&isolation_type_snp);
    if (!ms_hyperv.paravisor_present)
    hypercall_update(hv_snp_hypercall);
    } else if (hv_get_isolation_type() == HV_ISOLATION_TYPE_TDX) {
    static_branch_enable(&isolation_type_tdx);
// A TDX VM must use x2APIC and doesn't use lazy EOI.
    ms_hyperv.hints &= ~HV_X64_APIC_ACCESS_RECOMMENDED;
    if (!ms_hyperv.paravisor_present) {
    hypercall_update(hv_tdx_hypercall);
//
// Mark the Hyper-V TSC page feature as disabled
// in a TDX VM without paravisor so that the
// Invariant TSC, which is a better clocksource
// anyway, is used instead.
//
    ms_hyperv.features &= ~HV_MSR_REFERENCE_TSC_AVAILABLE;
//
// The Invariant TSC is expected to be available
// in a TDX VM without paravisor, but if not,
// print a warning message. The slower Hyper-V MSR-based
// Ref Counter should end up being the clocksource.
//
    if (!(ms_hyperv.features & HV_ACCESS_TSC_INVARIANT))
    pr_warn("Hyper-V: Invariant TSC is unavailable\n");
// HV_MSR_CRASH_CTL is unsupported.
    ms_hyperv.misc_features &= ~HV_FEATURE_GUEST_CRASH_MSR_AVAILABLE;
// Don't trust Hyper-V's TLB-flushing hypercalls.
    ms_hyperv.hints &= ~HV_X64_REMOTE_TLB_FLUSH_RECOMMENDED;
    x86_init.acpi.reduced_hw_early_init = reduced_hw_init;
    }
    }
    }
    if (hv_max_functions_eax >= HYPERV_CPUID_NESTED_FEATURES) {
    ms_hyperv.nested_features =
    cpuid_eax(HYPERV_CPUID_NESTED_FEATURES);
    pr_info("Hyper-V: Nested features: 0x%x\n",
    ms_hyperv.nested_features);
    }

    if (ms_hyperv.features & HV_ACCESS_FREQUENCY_MSRS &&
    ms_hyperv.misc_features & HV_FEATURE_FREQUENCY_MSRS_AVAILABLE) {
//
// Get the APIC frequency.
//
    u64	hv_lapic_frequency;
    rdmsrq(HV_X64_MSR_APIC_FREQUENCY, hv_lapic_frequency);
    hv_lapic_frequency = div_u64(hv_lapic_frequency, HZ);
    lapic_timer_period = hv_lapic_frequency;
    pr_info("Hyper-V: LAPIC Timer Frequency: %#x\n",
    lapic_timer_period);
    }
    register_nmi_handler(NMI_UNKNOWN, hv_nmi_unknown, NMI_FLAG_FIRST,
    "hv_nmi_unknown");

    no_timer_check = 1;

    if (hv_root_partition())
    machine_ops.power_off = hv_machine_power_off;

    machine_ops.shutdown = hv_machine_shutdown;

    if (!hv_root_partition())
    machine_ops.crash_shutdown = hv_guest_crash_shutdown;

//
// HV_ACCESS_TSC_INVARIANT is always zero for the root partition. Root
// partition doesn't need to write to synthetic MSR to enable invariant
// TSC feature. It sees what the hardware provides.
//
    if (ms_hyperv.features & HV_ACCESS_TSC_INVARIANT) {
//
// Writing to synthetic MSR 0x40000118 updates/changes the
// guest visible CPUIDs. Setting bit 0 of this MSR  enables
// guests to report invariant TSC feature through CPUID
// instruction, CPUID 0x800000007/EDX, bit 8. See code in
// early_init_intel() where this bit is examined. The
// setting of this MSR bit should happen before init_intel()
// is called.
//
    wrmsrq(HV_X64_MSR_TSC_INVARIANT_CONTROL, HV_EXPOSE_INVARIANT_TSC);
    setup_force_cpu_cap(X86_FEATURE_TSC_RELIABLE);
    }
//
// Generation 2 instances don't support reading the NMI status from
// 0x61 port.
//
    if (efi_enabled(EFI_BOOT))
    x86_platform.get_nmi_reason = hv_get_nmi_reason;

    if ((hv_get_isolation_type() == HV_ISOLATION_TYPE_VBS) ||
    ms_hyperv.paravisor_present)
    hv_vtom_init();
//
// Setup the hook to get control post apic initialization.
//
    x86_platform.apic_post_init = hyperv_init;
    hyperv_setup_mmu_ops();
// Install system interrupt handler for hypervisor callback
    sysvec_install(HYPERVISOR_CALLBACK_VECTOR, sysvec_hyperv_callback);
// Install system interrupt handler for reenlightenment notifications
    if (ms_hyperv.features & HV_ACCESS_REENLIGHTENMENT) {
    sysvec_install(HYPERV_REENLIGHTENMENT_VECTOR, sysvec_hyperv_reenlightenment);
    }
// Install system interrupt handler for stimer0
    sysvec_install(HYPERV_STIMER0_VECTOR, sysvec_hyperv_stimer0);

    smp_ops.smp_prepare_boot_cpu = hv_smp_prepare_boot_cpu;
    if (hv_root_partition() ||
    (!ms_hyperv.paravisor_present && hv_isolation_type_snp()))
    smp_ops.smp_prepare_cpus = hv_smp_prepare_cpus;

//
// Hyper-V doesn't provide irq remapping for IO-APIC. To enable x2apic,
// set x2apic destination mode to physical mode when x2apic is available
// and Hyper-V IOMMU driver makes sure cpus assigned with IO-APIC irqs
// have 8-bit APIC id.
//

    if (x2apic_supported())
    x2apic_phys = 1;

// Register Hyper-V specific clocksource
    hv_init_clocksource();
    x86_setup_ops_for_tsc_pg_clock();
    hv_vtl_init_platform();

//
// TSC should be marked as unstable only after Hyper-V
// clocksource has been initialized. This ensures that the
// stability of the sched_clock is not altered.
//
// HV_ACCESS_TSC_INVARIANT is always zero for the root partition. No
// need to check for it.
//
    if (!hv_root_partition() &&
    !(ms_hyperv.features & HV_ACCESS_TSC_INVARIANT))
    mark_tsc_unstable("running on Hyper-V");
    hardlockup_detector_disable();
    }
#[no_mangle]
unsafe extern "C" fn ms_hyperv_x2apic_available() -> bool __init {
    static bool __init ms_hyperv_x2apic_available(void)
    {
    return x2apic_supported();
    }
//
// If ms_hyperv_msi_ext_dest_id() returns true, hyperv_prepare_irq_remapping()
// returns -ENODEV and the Hyper-V IOMMU driver is not used; instead, the
// generic support of the 15-bit APIC ID is used: see __irq_msi_compose_msg().
//
// Note: for a VM on Hyper-V, the I/O-APIC is the only device which
// (logically) generates MSIs directly to the system APIC irq domain.
// There is no HPET, and PCI MSI/MSI-X interrupts are remapped by the
// pci-hyperv host bridge.
//
// Note: for a Hyper-V root partition, this will always return false.
//
#[no_mangle]
unsafe extern "C" fn ms_hyperv_msi_ext_dest_id() -> bool __init {
    static bool __init ms_hyperv_msi_ext_dest_id(void)
    {
    return ms_hyperv.msi_ext_dest_id;
    }

#[no_mangle]
unsafe extern "C" fn hv_sev_es_hcall_prepare(ghcb: *mut ghcb, regs: *mut pt_regs) {
    static void hv_sev_es_hcall_prepare(struct ghcb *ghcb, struct pt_regs *regs)
    {
// RAX and CPL are already in the GHCB
    ghcb_set_rcx(ghcb, regs.cx);
    ghcb_set_rdx(ghcb, regs.dx);
    ghcb_set_r8(ghcb, regs.r8);
    }
#[no_mangle]
unsafe extern "C" fn hv_sev_es_hcall_finish(ghcb: *mut ghcb, regs: *mut pt_regs) -> bool {
    static bool hv_sev_es_hcall_finish(struct ghcb *ghcb, struct pt_regs *regs)
    {
// No checking of the return state needed
    return true;
    }

    const __initconst struct hypervisor_x86 x86_hyper_ms_hyperv = {
    .name			= "Microsoft Hyper-V",
    .detect			= ms_hyperv_platform,
    .type			= X86_HYPER_MS_HYPERV,
    .init.x2apic_available	= ms_hyperv_x2apic_available,
    .init.msi_ext_dest_id	= ms_hyperv_msi_ext_dest_id,
    .init.init_platform	= ms_hyperv_init_platform,
    .init.guest_late_init	= ms_hyperv_late_init,

    .runtime.sev_es_hcall_prepare = hv_sev_es_hcall_prepare,
    .runtime.sev_es_hcall_finish = hv_sev_es_hcall_finish,

    };
