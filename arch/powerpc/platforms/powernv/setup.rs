//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/setup.c
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
// PowerNV setup code.
//
// Copyright 2011 IBM Corp.
//

    static bool __init fw_feature_is(const char *state, const char *name,
    struct device_node *fw_features)
    {
    struct device_node *np;
    let mut rc: bool = false;
    np = of_get_child_by_name(fw_features, name);
    if (np) {
    rc = of_property_read_bool(np, state);
    of_node_put(np);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn init_fw_feat_flags(np: *mut device_node) -> void __init {
    static void __init init_fw_feat_flags(struct device_node *np)
    {
    if (fw_feature_is("enabled", "inst-spec-barrier-ori31,31,0", np))
    security_ftr_set(SEC_FTR_SPEC_BAR_ORI31);
    if (fw_feature_is("enabled", "fw-bcctrl-serialized", np))
    security_ftr_set(SEC_FTR_BCCTRL_SERIALISED);
    if (fw_feature_is("enabled", "inst-l1d-flush-ori30,30,0", np))
    security_ftr_set(SEC_FTR_L1D_FLUSH_ORI30);
    if (fw_feature_is("enabled", "inst-l1d-flush-trig2", np))
    security_ftr_set(SEC_FTR_L1D_FLUSH_TRIG2);
    if (fw_feature_is("enabled", "fw-l1d-thread-split", np))
    security_ftr_set(SEC_FTR_L1D_THREAD_PRIV);
    if (fw_feature_is("enabled", "fw-count-cache-disabled", np))
    security_ftr_set(SEC_FTR_COUNT_CACHE_DISABLED);
    if (fw_feature_is("enabled", "fw-count-cache-flush-bcctr2,0,0", np))
    security_ftr_set(SEC_FTR_BCCTR_FLUSH_ASSIST);
    if (fw_feature_is("enabled", "needs-count-cache-flush-on-context-switch", np))
    security_ftr_set(SEC_FTR_FLUSH_COUNT_CACHE);
//
// The features below are enabled by default, so we instead look to see
// if firmware has *disabled* them, and clear them if so.
//
    if (fw_feature_is("disabled", "speculation-policy-favor-security", np))
    security_ftr_clear(SEC_FTR_FAVOUR_SECURITY);
    if (fw_feature_is("disabled", "needs-l1d-flush-msr-pr-0-to-1", np))
    security_ftr_clear(SEC_FTR_L1D_FLUSH_PR);
    if (fw_feature_is("disabled", "needs-l1d-flush-msr-hv-1-to-0", np))
    security_ftr_clear(SEC_FTR_L1D_FLUSH_HV);
    if (fw_feature_is("disabled", "needs-spec-barrier-for-bound-checks", np))
    security_ftr_clear(SEC_FTR_BNDS_CHK_SPEC_BAR);
    if (fw_feature_is("enabled", "no-need-l1d-flush-msr-pr-1-to-0", np))
    security_ftr_clear(SEC_FTR_L1D_FLUSH_ENTRY);
    if (fw_feature_is("enabled", "no-need-l1d-flush-kernel-on-user-access", np))
    security_ftr_clear(SEC_FTR_L1D_FLUSH_UACCESS);
    if (fw_feature_is("enabled", "no-need-store-drain-on-priv-state-switch", np))
    security_ftr_clear(SEC_FTR_STF_BARRIER);
    }
#[no_mangle]
unsafe extern "C" fn pnv_setup_security_mitigations() -> void __init {
    static void __init pnv_setup_security_mitigations(void)
    {
    struct device_node *np, *fw_features;
    enum l1d_flush_type type;
    bool enable;
// Default to fallback in case fw-features are not available
    type = L1D_FLUSH_FALLBACK;
    np = of_find_node_by_name(core::ptr::null_mut(), "ibm,opal");
    fw_features = of_get_child_by_name(np, "fw-features");
    of_node_put(np);
    if (fw_features) {
    init_fw_feat_flags(fw_features);
    of_node_put(fw_features);
    if (security_ftr_enabled(SEC_FTR_L1D_FLUSH_TRIG2))
    type = L1D_FLUSH_MTTRIG;
    if (security_ftr_enabled(SEC_FTR_L1D_FLUSH_ORI30))
    type = L1D_FLUSH_ORI;
    }
//
// The issues addressed by the entry and uaccess flush don't affect P7
// or P8, so on bare metal disable them explicitly in case firmware does
// not include the features to disable them. POWER9 and newer processors
// should have the appropriate firmware flags.
//
    if (pvr_version_is(PVR_POWER7) || pvr_version_is(PVR_POWER7p) ||
    pvr_version_is(PVR_POWER8E) || pvr_version_is(PVR_POWER8NVL) ||
    pvr_version_is(PVR_POWER8)) {
    security_ftr_clear(SEC_FTR_L1D_FLUSH_ENTRY);
    security_ftr_clear(SEC_FTR_L1D_FLUSH_UACCESS);
    }
    enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) && \
    (security_ftr_enabled(SEC_FTR_L1D_FLUSH_PR)   || \
    security_ftr_enabled(SEC_FTR_L1D_FLUSH_HV));
    setup_rfi_flush(type, enable);
    setup_count_cache_flush();
    enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) &&
    security_ftr_enabled(SEC_FTR_L1D_FLUSH_ENTRY);
    setup_entry_flush(enable);
    enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) &&
    security_ftr_enabled(SEC_FTR_L1D_FLUSH_UACCESS);
    setup_uaccess_flush(enable);
    setup_stf_barrier();
    }
#[no_mangle]
unsafe extern "C" fn pnv_check_guarded_cores() -> void __init {
    static void __init pnv_check_guarded_cores(void)
    {
    struct device_node *dn;
    let mut bad_count: c_int = 0;
    for_each_node_by_type(dn, "cpu") {
    if (of_property_match_string(dn, "status", "bad") >= 0)
    bad_count++;
    }
    if (bad_count) {
    printk("  _     _______________\n");
    pr_cont(" | |   /               \\\n");
    pr_cont(" | |   |    WARNING!   |\n");
    pr_cont(" | |   |               |\n");
    pr_cont(" | |   | It looks like |\n");
    pr_cont(" |_|   |  you have %*d |\n", 3, bad_count);
    pr_cont("  _    | guarded cores |\n");
    pr_cont(" (_)   \\_______________/\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn pnv_setup_arch() -> void __init {
    static void __init pnv_setup_arch(void)
    {
    set_arch_panic_timeout(10, ARCH_PANIC_TIMEOUT);
    pnv_setup_security_mitigations();
// Initialize SMP
    pnv_smp_init();
// Setup RTC and NVRAM callbacks
    if (firmware_has_feature(FW_FEATURE_OPAL))
    opal_nvram_init();
// Enable NAP mode
    powersave_nap = 1;
    pnv_check_guarded_cores();
// XXX PMCS
    pnv_rng_init();
    }
#[no_mangle]
unsafe extern "C" fn pnv_add_hw_description() -> void __init {
    static void __init pnv_add_hw_description(void)
    {
    struct device_node *dn;
    const char *s;
    dn = of_find_node_by_path("/ibm,opal/firmware");
    if (!dn)
    return;
    if (of_property_read_string(dn, "version", &s) == 0 ||
    of_property_read_string(dn, "git-id", &s) == 0)
    seq_buf_printf(&ppc_hw_desc, "opal:%s ", s);
    if (of_property_read_string(dn, "mi-version", &s) == 0)
    seq_buf_printf(&ppc_hw_desc, "mi:%s ", s);
    of_node_put(dn);
    }
#[no_mangle]
unsafe extern "C" fn pnv_init() -> void __init {
    static void __init pnv_init(void)
    {
    pnv_add_hw_description();
//
// Initialize the LPC bus now so that legacy serial
// ports can be found on it
//
    opal_lpc_init();

    if (firmware_has_feature(FW_FEATURE_OPAL))
    hvc_opal_init_early();
    else

    add_preferred_console("hvc", 0, core::ptr::null_mut());

    if (!radix_enabled()) {
    let mut size: usize = sizeof(struct slb_entry) * mmu_slb_size;
    int i;
// Allocate per cpu area to save old slb contents during MCE
    for_each_possible_cpu(i) {
    paca_ptrs[i].mce_faulty_slbs =
    memblock_alloc_node(size,
    __alignof__(struct slb_entry),
    cpu_to_node(i));
    }
    }

    }
#[no_mangle]
unsafe extern "C" fn pnv_init_IRQ() -> void __init {
    static void __init pnv_init_IRQ(void)
    {
// Try using a XIVE if available, otherwise use a XICS
    if (!xive_native_init())
    xics_init();
    WARN_ON(!ppc_md.get_irq);
    }
#[no_mangle]
unsafe extern "C" fn pnv_show_cpuinfo(m: *mut seq_file) {
    static void pnv_show_cpuinfo(struct seq_file *m)
    {
    struct device_node *root;
    const char *model = "";
    root = of_find_node_by_path("/");
    if (root)
    model = of_get_property(root, "model", core::ptr::null_mut());
    seq_printf(m, "machine\t\t: PowerNV %s\n", model);
    if (firmware_has_feature(FW_FEATURE_OPAL))
    seq_printf(m, "firmware\t: OPAL\n");
    else
    seq_printf(m, "firmware\t: BML\n");
    of_node_put(root);
    if (radix_enabled())
    seq_printf(m, "MMU\t\t: Radix\n");
    else
    seq_printf(m, "MMU\t\t: Hash\n");
    }
#[no_mangle]
unsafe extern "C" fn pnv_prepare_going_down() {
    static void pnv_prepare_going_down(void)
    {
//
// Disable all notifiers from OPAL, we can't
// service interrupts anymore anyway
//
    opal_event_shutdown();
// Print flash update message if one is scheduled.
    opal_flash_update_print_message();
    smp_send_stop();
    hard_irq_disable();
    }
#[no_mangle]
unsafe extern "C" fn pnv_restart(cmd: *mut c_char) -> void  __noreturn {
    static void  __noreturn pnv_restart(char *cmd)
    {
    long rc;
    pnv_prepare_going_down();
    do {
    if (!cmd || *cmd == '\0')
    rc = opal_cec_reboot();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(cmd, 0: "full") ==) -> else {
    else if (strcmp(cmd, "full") == 0)
    rc = opal_cec_reboot2(OPAL_REBOOT_FULL_IPL, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(cmd, 0: "mpipl") ==) -> else {
    else if (strcmp(cmd, "mpipl") == 0)
    rc = opal_cec_reboot2(OPAL_REBOOT_MPIPL, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(cmd, 0: "error") ==) -> else {
    else if (strcmp(cmd, "error") == 0)
    rc = opal_cec_reboot2(OPAL_REBOOT_PLATFORM_ERROR, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(cmd, 0: "fast") ==) -> else {
    else if (strcmp(cmd, "fast") == 0)
    rc = opal_cec_reboot2(OPAL_REBOOT_FAST, core::ptr::null_mut());
    else
    rc = OPAL_UNSUPPORTED;
    if (rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT) {
// Opal is busy wait for some time and retry
    opal_poll_events(core::ptr::null_mut());
    mdelay(10);
    } else	if (cmd && rc) {
// Unknown error while issuing reboot
    if (rc == OPAL_UNSUPPORTED)
    pr_err("Unsupported '%s' reboot.\n", cmd);
    else
    pr_err("Unable to issue '%s' reboot. Err=%ld\n",
    cmd, rc);
    pr_info("Forcing a cec-reboot\n");
    cmd = core::ptr::null_mut();
    rc = OPAL_BUSY;
    } else if (rc != OPAL_SUCCESS) {
// Unknown error while issuing cec-reboot
    pr_err("Unable to reboot. Err=%ld\n", rc);
    }
    } while (rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT);
    for (;;)
    opal_poll_events(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn pnv_power_off() -> void __noreturn {
    static void __noreturn pnv_power_off(void)
    {
    let mut rc: c_long = OPAL_BUSY;
    pnv_prepare_going_down();
    while (rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT) {
    rc = opal_cec_power_down(0);
    if (rc == OPAL_BUSY_EVENT)
    opal_poll_events(core::ptr::null_mut());
    else
    mdelay(10);
    }
    for (;;)
    opal_poll_events(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn pnv_halt() -> void __noreturn {
    static void __noreturn pnv_halt(void)
    {
    pnv_power_off();
    }
#[no_mangle]
unsafe extern "C" fn pnv_progress(s: *mut c_char, hex: c_ushort) {
    static void pnv_progress(char *s, unsigned short hex)
    {
    }
#[no_mangle]
unsafe extern "C" fn pnv_shutdown() {
    static void pnv_shutdown(void)
    {
// Let the PCI code clear up IODA tables
    pnv_pci_shutdown();
//
// Stop OPAL activity: Unregister all OPAL interrupts so they
// don't fire up while we kexec and make sure all potentially
// DMA'ing ops are complete (such as dump retrieval).
//
    opal_shutdown();
    }

#[no_mangle]
unsafe extern "C" fn pnv_kexec_wait_secondaries_down() {
    static void pnv_kexec_wait_secondaries_down(void)
    {
    int my_cpu, i, notified = -1;
// Called with interrupts disabled, so the CPU is pinned.
    my_cpu = raw_smp_processor_id();
    for_each_online_cpu(i) {
    uint8_t status;
    int64_t rc, timeout = 1000;
    if (i == my_cpu)
    continue;
    for (;;) {
    rc = opal_query_cpu_status(get_hard_smp_processor_id(i),
    &status);
    if (rc != OPAL_SUCCESS || status != OPAL_THREAD_STARTED)
    break;
    barrier();
    if (i != notified) {
    printk(KERN_INFO "kexec: waiting for cpu %d "
    "(physical %d) to enter OPAL\n",
    i, paca_ptrs[i].hw_cpu_id);
    notified = i;
    }
//
// On crash secondaries might be unreachable or hung,
// so timeout if we've waited too long
//
    mdelay(1);
    if (timeout-- == 0) {
    printk(KERN_ERR "kexec: timed out waiting for "
    "cpu %d (physical %d) to enter OPAL\n",
    i, paca_ptrs[i].hw_cpu_id);
    break;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pnv_kexec_cpu_down(crash_shutdown: c_int, secondary: c_int) {
    static void pnv_kexec_cpu_down(int crash_shutdown, int secondary)
    {
    u64 reinit_flags;
    if (xive_enabled())
    xive_teardown_cpu();
    else
    xics_kexec_teardown_cpu(secondary);
// On OPAL, we return all CPUs to firmware
    if (!firmware_has_feature(FW_FEATURE_OPAL))
    return;
    if (secondary) {
// Return secondary CPUs to firmware on OPAL v3
    mb();
    get_paca().kexec_state = KEXEC_STATE_REAL_MODE;
    mb();
// Return the CPU to OPAL
    opal_return_cpu();
    } else {
// Primary waits for the secondaries to have reached OPAL
    pnv_kexec_wait_secondaries_down();
// Switch XIVE back to emulation mode
    if (xive_enabled())
    xive_shutdown();
//
// We might be running as little-endian - now that interrupts
// are disabled, reset the HILE bit to big-endian so we don't
// take interrupts in the wrong endian later
//
// We reinit to enable both radix and hash on P9 to ensure
// the mode used by the next kernel is always supported.
//
    reinit_flags = OPAL_REINIT_CPUS_HILE_BE;
    if (cpu_has_feature(CPU_FTR_ARCH_300))
    reinit_flags |= OPAL_REINIT_CPUS_MMU_RADIX |
    OPAL_REINIT_CPUS_MMU_HASH;
    opal_reinit_cpus(reinit_flags);
    }
    }

#[no_mangle]
unsafe extern "C" fn pnv_memory_block_size() -> c_ulong {
    static unsigned long pnv_memory_block_size(void)
    {
    return memory_block_size;
    }

#[no_mangle]
unsafe extern "C" fn pnv_setup_machdep_opal() -> void __init {
    static void __init pnv_setup_machdep_opal(void)
    {
    ppc_md.get_boot_time = opal_get_boot_time;
    ppc_md.restart = pnv_restart;
    pm_power_off = pnv_power_off;
    ppc_md.halt = pnv_halt;
// ppc_md.system_reset_exception gets filled in by pnv_smp_init()
    ppc_md.machine_check_exception = opal_machine_check;
    ppc_md.mce_check_early_recovery = opal_mce_check_early_recovery;
    if (opal_check_token(OPAL_HANDLE_HMI2))
    ppc_md.hmi_exception_early = opal_hmi_exception_early2;
    else
    ppc_md.hmi_exception_early = opal_hmi_exception_early;
    ppc_md.handle_hmi_exception = opal_handle_hmi_exception;
    }
#[no_mangle]
unsafe extern "C" fn pnv_probe() -> int __init {
    static int __init pnv_probe(void)
    {
    if (firmware_has_feature(FW_FEATURE_OPAL))
    pnv_setup_machdep_opal();
    pr_debug("PowerNV detected !\n");
    pnv_init();
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn pnv_tm_init() -> void __init {
    void __init pnv_tm_init(void)
    {
    if (!firmware_has_feature(FW_FEATURE_OPAL) ||
    !pvr_version_is(PVR_POWER9) ||
    early_cpu_has_feature(CPU_FTR_TM))
    return;
    if (opal_reinit_cpus(OPAL_REINIT_CPUS_TM_SUSPEND_DISABLED) != OPAL_SUCCESS)
    return;
    pr_info("Enabling TM (Transactional Memory) with Suspend Disabled\n");
    cur_cpu_spec.cpu_features |= CPU_FTR_TM;
// Make sure "normal" HTM is off (it should be)
    cur_cpu_spec.cpu_user_features2 &= ~PPC_FEATURE2_HTM;
// Turn on no suspend mode, and HTM no SC
    cur_cpu_spec.cpu_user_features2 |= PPC_FEATURE2_HTM_NO_SUSPEND | \
    PPC_FEATURE2_HTM_NOSC;
    tm_suspend_disabled = true;
    }

//
// Returns the cpu frequency for 'cpu' in Hz. This is used by
// /proc/cpuinfo
//
#[no_mangle]
unsafe extern "C" fn pnv_get_proc_freq(cpu: c_uint) -> c_ulong {
    static unsigned long pnv_get_proc_freq(unsigned int cpu)
    {
    unsigned long ret_freq;
    ret_freq = cpufreq_get(cpu) * 1000ul;
//
// If the backend cpufreq driver does not exist,
// then fallback to old way of reporting the clockrate.
//
    if (!ret_freq)
    ret_freq = ppc_proc_freq;
    return ret_freq;
    }
#[no_mangle]
unsafe extern "C" fn pnv_machine_check_early(regs: *mut pt_regs) -> c_long {
    static long pnv_machine_check_early(struct pt_regs *regs)
    {
    let mut handled: c_long = 0;
    if (cur_cpu_spec && cur_cpu_spec.machine_check_early)
    handled = cur_cpu_spec.machine_check_early(regs);
    return handled;
    }
    define_machine(powernv) {
    .name			= "PowerNV",
    .compatible		= "ibm,powernv",
    .probe			= pnv_probe,
    .setup_arch		= pnv_setup_arch,
    .init_IRQ		= pnv_init_IRQ,
    .show_cpuinfo		= pnv_show_cpuinfo,
    .get_proc_freq          = pnv_get_proc_freq,
    .discover_phbs		= pnv_pci_init,
    .progress		= pnv_progress,
    .machine_shutdown	= pnv_shutdown,
    .power_save             = core::ptr::null_mut(),
    .machine_check_early	= pnv_machine_check_early,

    .kexec_cpu_down		= pnv_kexec_cpu_down,

    .memory_block_size	= pnv_memory_block_size,

    };
