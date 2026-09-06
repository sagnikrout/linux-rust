//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/security.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Security related flags and so on.
//
// Copyright 2018, Michael Ellerman, IBM Corporation.

    let mut __read_mostly: u64 powerpc_security_features = SEC_FTR_DEFAULT;
    enum branch_cache_flush_type {
    BRANCH_CACHE_FLUSH_NONE	= 0x1,
    BRANCH_CACHE_FLUSH_SW	= 0x2,
    BRANCH_CACHE_FLUSH_HW	= 0x4,
    };
    let mut count_cache_flush_type: static enum branch_cache_flush_type = BRANCH_CACHE_FLUSH_NONE;
    let mut link_stack_flush_type: static enum branch_cache_flush_type = BRANCH_CACHE_FLUSH_NONE;
    bool barrier_nospec_enabled;
    static bool no_nospec;
    static bool btb_flush_enabled;

    static bool no_spectrev2;

#[no_mangle]
unsafe extern "C" fn enable_barrier_nospec(enable: bool) {
    static void enable_barrier_nospec(bool enable)
    {
    barrier_nospec_enabled = enable;
    do_barrier_nospec_fixups(enable);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_barrier_nospec() -> void __init {
    void __init setup_barrier_nospec(void)
    {
    bool enable;
//
// It would make sense to check SEC_FTR_SPEC_BAR_ORI31 below as well.
// But there's a good reason not to. The two flags we check below are
// both are enabled by default in the kernel, so if the hcall is not
// functional they will be enabled.
// On a system where the host firmware has been updated (so the ori
// functions as a barrier), but on which the hypervisor (KVM/Qemu) has
// not been updated, we would like to enable the barrier. Dropping the
// check for SEC_FTR_SPEC_BAR_ORI31 achieves that. The only downside is
// we potentially enable the barrier on systems where the host firmware
// is not updated, but that's harmless as it's a no-op.
//
    enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) &&
    security_ftr_enabled(SEC_FTR_BNDS_CHK_SPEC_BAR);
    if (!no_nospec && !cpu_mitigations_off())
    enable_barrier_nospec(enable);
    }
#[no_mangle]
unsafe extern "C" fn handle_nospectre_v1(p: *mut c_char) -> int __init {
    static int __init handle_nospectre_v1(char *p)
    {
    no_nospec = true;
    return 0;
    }
    early_param("nospectre_v1", handle_nospectre_v1);

#[no_mangle]
unsafe extern "C" fn barrier_nospec_set(data: *mut c_void, val: u64) -> c_int {
    static int barrier_nospec_set(void *data, u64 val)
    {
    switch (val) {
    case 0:
    case 1:
    break;
    default:
    return -EINVAL;
    }
    if (!!val == !!barrier_nospec_enabled)
    return 0;
    enable_barrier_nospec(!!val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn barrier_nospec_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int barrier_nospec_get(void *data, u64 *val)
    {
// val = barrier_nospec_enabled ? 1 : 0;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_barrier_nospec, barrier_nospec_get,
    barrier_nospec_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn barrier_nospec_debugfs_init() -> __init int {
    static __init int barrier_nospec_debugfs_init(void)
    {
    debugfs_create_file_unsafe("barrier_nospec", 0600,
    arch_debugfs_dir, core::ptr::null_mut(),
    &fops_barrier_nospec);
    return 0;
    }
    device_initcall(barrier_nospec_debugfs_init);
#[no_mangle]
unsafe extern "C" fn security_feature_debugfs_init() -> __init int {
    static __init int security_feature_debugfs_init(void)
    {
    debugfs_create_x64("security_features", 0400, arch_debugfs_dir,
    &powerpc_security_features);
    return 0;
    }
    device_initcall(security_feature_debugfs_init);

#[no_mangle]
unsafe extern "C" fn handle_nospectre_v2(p: *mut c_char) -> int __init {
    static int __init handle_nospectre_v2(char *p)
    {
    no_spectrev2 = true;
    return 0;
    }
    early_param("nospectre_v2", handle_nospectre_v2);

#[no_mangle]
pub unsafe extern "C" fn setup_spectre_v2() -> void __init {
    void __init setup_spectre_v2(void)
    {
    if (no_spectrev2 || cpu_mitigations_off())
    do_btb_flush_fixups();
    else
    btb_flush_enabled = true;
    }

#[no_mangle]
pub unsafe extern "C" fn cpu_show_meltdown(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    ssize_t cpu_show_meltdown(struct device *dev, struct device_attribute *attr, char *buf)
    {
    bool thread_priv;
    thread_priv = security_ftr_enabled(SEC_FTR_L1D_THREAD_PRIV);
    if (rfi_flush) {
    struct seq_buf s;
    seq_buf_init(&s, buf, PAGE_SIZE - 1);
    seq_buf_printf(&s, "Mitigation: RFI Flush");
    if (thread_priv)
    seq_buf_printf(&s, ", L1D private per thread");
    seq_buf_printf(&s, "\n");
    return s.len;
    }
    if (thread_priv)
    return sysfs_emit(buf, "Vulnerable: L1D private per thread\n");
    if (!security_ftr_enabled(SEC_FTR_L1D_FLUSH_HV) &&
    !security_ftr_enabled(SEC_FTR_L1D_FLUSH_PR))
    return sysfs_emit(buf, "Not affected\n");
    return sysfs_emit(buf, "Vulnerable\n");
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_show_l1tf(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    ssize_t cpu_show_l1tf(struct device *dev, struct device_attribute *attr, char *buf)
    {
    return cpu_show_meltdown(dev, attr, buf);
    }

#[no_mangle]
pub unsafe extern "C" fn cpu_show_spectre_v1(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    ssize_t cpu_show_spectre_v1(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct seq_buf s;
    seq_buf_init(&s, buf, PAGE_SIZE - 1);
    if (security_ftr_enabled(SEC_FTR_BNDS_CHK_SPEC_BAR)) {
    if (barrier_nospec_enabled)
    seq_buf_printf(&s, "Mitigation: __user pointer sanitization");
    else
    seq_buf_printf(&s, "Vulnerable");
    if (security_ftr_enabled(SEC_FTR_SPEC_BAR_ORI31))
    seq_buf_printf(&s, ", ori31 speculation barrier enabled");
    seq_buf_printf(&s, "\n");
    } else
    seq_buf_printf(&s, "Not affected\n");
    return s.len;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_show_spectre_v2(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    ssize_t cpu_show_spectre_v2(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct seq_buf s;
    bool bcs, ccd;
    seq_buf_init(&s, buf, PAGE_SIZE - 1);
    bcs = security_ftr_enabled(SEC_FTR_BCCTRL_SERIALISED);
    ccd = security_ftr_enabled(SEC_FTR_COUNT_CACHE_DISABLED);
    if (bcs || ccd) {
    seq_buf_printf(&s, "Mitigation: ");
    if (bcs)
    seq_buf_printf(&s, "Indirect branch serialisation (kernel only)");
    if (bcs && ccd)
    seq_buf_printf(&s, ", ");
    if (ccd)
    seq_buf_printf(&s, "Indirect branch cache disabled");
    } else if (count_cache_flush_type != BRANCH_CACHE_FLUSH_NONE) {
    seq_buf_printf(&s, "Mitigation: Software count cache flush");
    if (count_cache_flush_type == BRANCH_CACHE_FLUSH_HW)
    seq_buf_printf(&s, " (hardware accelerated)");
    } else if (btb_flush_enabled) {
    seq_buf_printf(&s, "Mitigation: Branch predictor state flush");
    } else {
    seq_buf_printf(&s, "Vulnerable");
    }
    if (bcs || ccd || count_cache_flush_type != BRANCH_CACHE_FLUSH_NONE) {
    if (link_stack_flush_type != BRANCH_CACHE_FLUSH_NONE)
    seq_buf_printf(&s, ", Software link stack flush");
    if (link_stack_flush_type == BRANCH_CACHE_FLUSH_HW)
    seq_buf_printf(&s, " (hardware accelerated)");
    }
    seq_buf_printf(&s, "\n");
    return s.len;
    }

//
// Store-forwarding barrier support.
//
    static enum stf_barrier_type stf_enabled_flush_types;
    static bool no_stf_barrier;
    static bool stf_barrier;
#[no_mangle]
unsafe extern "C" fn handle_no_stf_barrier(p: *mut c_char) -> int __init {
    static int __init handle_no_stf_barrier(char *p)
    {
    pr_info("stf-barrier: disabled on command line.");
    no_stf_barrier = true;
    return 0;
    }
    early_param("no_stf_barrier", handle_no_stf_barrier);
#[no_mangle]
pub unsafe extern "C" fn stf_barrier_type_get() -> enum stf_barrier_type {
    enum stf_barrier_type stf_barrier_type_get(void)
    {
    return stf_enabled_flush_types;
    }
// This is the generic flag used by other architectures
#[no_mangle]
unsafe extern "C" fn handle_ssbd(p: *mut c_char) -> int __init {
    static int __init handle_ssbd(char *p)
    {
    if (!p || strncmp(p, "auto", 5) == 0 || strncmp(p, "on", 2) == 0 ) {
// Until firmware tells us, we have the barrier with auto
    return 0;
    } else if (strncmp(p, "off", 3) == 0) {
    handle_no_stf_barrier(core::ptr::null_mut());
    return 0;
    } else
    return 1;
    return 0;
    }
    early_param("spec_store_bypass_disable", handle_ssbd);
// This is the generic flag used by other architectures
#[no_mangle]
unsafe extern "C" fn handle_no_ssbd(p: *mut c_char) -> int __init {
    static int __init handle_no_ssbd(char *p)
    {
    handle_no_stf_barrier(core::ptr::null_mut());
    return 0;
    }
    early_param("nospec_store_bypass_disable", handle_no_ssbd);
#[no_mangle]
unsafe extern "C" fn stf_barrier_enable(enable: bool) {
    static void stf_barrier_enable(bool enable)
    {
    if (enable)
    do_stf_barrier_fixups(stf_enabled_flush_types);
    else
    do_stf_barrier_fixups(STF_BARRIER_NONE);
    stf_barrier = enable;
    }
#[no_mangle]
pub unsafe extern "C" fn setup_stf_barrier() {
    void setup_stf_barrier(void)
    {
    enum stf_barrier_type type;
    bool enable;
// Default to fallback in case fw-features are not available
    if (cpu_has_feature(CPU_FTR_ARCH_300))
    type = STF_BARRIER_EIEIO;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cpu_has_feature(CPU_FTR_ARCH_207S)) -> else {
    else if (cpu_has_feature(CPU_FTR_ARCH_207S))
    type = STF_BARRIER_SYNC_ORI;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cpu_has_feature(CPU_FTR_ARCH_206)) -> else {
    else if (cpu_has_feature(CPU_FTR_ARCH_206))
    type = STF_BARRIER_FALLBACK;
    else
    type = STF_BARRIER_NONE;
    enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) &&
    security_ftr_enabled(SEC_FTR_STF_BARRIER);
    if (type == STF_BARRIER_FALLBACK) {
    pr_info("stf-barrier: fallback barrier available\n");
    } else if (type == STF_BARRIER_SYNC_ORI) {
    pr_info("stf-barrier: hwsync barrier available\n");
    } else if (type == STF_BARRIER_EIEIO) {
    pr_info("stf-barrier: eieio barrier available\n");
    }
    stf_enabled_flush_types = type;
    if (!no_stf_barrier && !cpu_mitigations_off())
    stf_barrier_enable(enable);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_show_spec_store_bypass(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    ssize_t cpu_show_spec_store_bypass(struct device *dev, struct device_attribute *attr, char *buf)
    {
    if (stf_barrier && stf_enabled_flush_types != STF_BARRIER_NONE) {
    const char *type;
    switch (stf_enabled_flush_types) {
    case STF_BARRIER_EIEIO:
    type = "eieio";
    break;
    case STF_BARRIER_SYNC_ORI:
    type = "hwsync";
    break;
    case STF_BARRIER_FALLBACK:
    type = "fallback";
    break;
    default:
    type = "unknown";
    }
    return sysfs_emit(buf, "Mitigation: Kernel entry/exit barrier (%s)\n", type);
    }
    if (!security_ftr_enabled(SEC_FTR_L1D_FLUSH_HV) &&
    !security_ftr_enabled(SEC_FTR_L1D_FLUSH_PR))
    return sysfs_emit(buf, "Not affected\n");
    return sysfs_emit(buf, "Vulnerable\n");
    }
#[no_mangle]
unsafe extern "C" fn ssb_prctl_get(task: *mut task_struct) -> c_int {
    static int ssb_prctl_get(struct task_struct *task)
    {
//
// The STF_BARRIER feature is on by default, so if it's off that means
// firmware has explicitly said the CPU is not vulnerable via either
// the hypercall or device tree.
//
    if (!security_ftr_enabled(SEC_FTR_STF_BARRIER))
    return PR_SPEC_NOT_AFFECTED;
//
// If the system's CPU has no known barrier (see setup_stf_barrier())
// then assume that the CPU is not vulnerable.
//
    if (stf_enabled_flush_types == STF_BARRIER_NONE)
    return PR_SPEC_NOT_AFFECTED;
//
// Otherwise the CPU is vulnerable. The barrier is not a global or
// per-process mitigation, so the only value that can be reported here
// is PR_SPEC_ENABLE, which appears as "vulnerable" in /proc.
//
    return PR_SPEC_ENABLE;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prctl_spec_ctrl_get(task: *mut task_struct, which: c_ulong) -> c_int {
    int arch_prctl_spec_ctrl_get(struct task_struct *task, unsigned long which)
    {
    switch (which) {
    case PR_SPEC_STORE_BYPASS:
    return ssb_prctl_get(task);
    default:
    return -ENODEV;
    }
    }

#[no_mangle]
unsafe extern "C" fn stf_barrier_set(data: *mut c_void, val: u64) -> c_int {
    static int stf_barrier_set(void *data, u64 val)
    {
    bool enable;
    if (val == 1)
    enable = true;
#[no_mangle]
pub unsafe extern "C" fn if(0: val ==) -> else {
    else if (val == 0)
    enable = false;
    else
    return -EINVAL;
// Only do anything if we're changing state
    if (enable != stf_barrier)
    stf_barrier_enable(enable);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stf_barrier_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int stf_barrier_get(void *data, u64 *val)
    {
// val = stf_barrier ? 1 : 0;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_stf_barrier, stf_barrier_get, stf_barrier_set,
    "%llu\n");
#[no_mangle]
unsafe extern "C" fn stf_barrier_debugfs_init() -> __init int {
    static __init int stf_barrier_debugfs_init(void)
    {
    debugfs_create_file_unsafe("stf_barrier", 0600, arch_debugfs_dir,
    core::ptr::null_mut(), &fops_stf_barrier);
    return 0;
    }
    device_initcall(stf_barrier_debugfs_init);

#[no_mangle]
unsafe extern "C" fn update_branch_cache_flush() {
    static void update_branch_cache_flush(void)
    {
    u32 *site, __maybe_unused *site2;

    site = &patch__call_kvm_flush_link_stack;
    site2 = &patch__call_kvm_flush_link_stack_p9;
// This controls the branch from guest_exit_cont to kvm_flush_link_stack
    if (link_stack_flush_type == BRANCH_CACHE_FLUSH_NONE) {
    patch_instruction_site(site, ppc_inst(PPC_RAW_NOP()));
    patch_instruction_site(site2, ppc_inst(PPC_RAW_NOP()));
    } else {
// Could use HW flush, but that could also flush count cache
    patch_branch_site(site, (u64)&kvm_flush_link_stack, BRANCH_SET_LINK);
    patch_branch_site(site2, (u64)&kvm_flush_link_stack, BRANCH_SET_LINK);
    }

// Patch out the bcctr first, then nop the rest
    site = &patch__call_flush_branch_caches3;
    patch_instruction_site(site, ppc_inst(PPC_RAW_NOP()));
    site = &patch__call_flush_branch_caches2;
    patch_instruction_site(site, ppc_inst(PPC_RAW_NOP()));
    site = &patch__call_flush_branch_caches1;
    patch_instruction_site(site, ppc_inst(PPC_RAW_NOP()));
// This controls the branch from _switch to flush_branch_caches
    if (count_cache_flush_type == BRANCH_CACHE_FLUSH_NONE &&
    link_stack_flush_type == BRANCH_CACHE_FLUSH_NONE) {
// Nothing to be done
    } else if (count_cache_flush_type == BRANCH_CACHE_FLUSH_HW &&
    link_stack_flush_type == BRANCH_CACHE_FLUSH_HW) {
// Patch in the bcctr last
    site = &patch__call_flush_branch_caches1;
    patch_instruction_site(site, ppc_inst(0x39207fff)); // li r9,0x7fff
    site = &patch__call_flush_branch_caches2;
    patch_instruction_site(site, ppc_inst(0x7d2903a6)); // mtctr r9
    site = &patch__call_flush_branch_caches3;
    patch_instruction_site(site, ppc_inst(PPC_INST_BCCTR_FLUSH));
    } else {
    patch_branch_site(site, (u64)&flush_branch_caches, BRANCH_SET_LINK);
// If we just need to flush the link stack, early return
    if (count_cache_flush_type == BRANCH_CACHE_FLUSH_NONE) {
    patch_instruction_site(&patch__flush_link_stack_return,
    ppc_inst(PPC_RAW_BLR()));
// If we have flush instruction, early return
    } else if (count_cache_flush_type == BRANCH_CACHE_FLUSH_HW) {
    patch_instruction_site(&patch__flush_count_cache_return,
    ppc_inst(PPC_RAW_BLR()));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn toggle_branch_cache_flush(enable: bool) {
    static void toggle_branch_cache_flush(bool enable)
    {
    if (!enable || !security_ftr_enabled(SEC_FTR_FLUSH_COUNT_CACHE)) {
    if (count_cache_flush_type != BRANCH_CACHE_FLUSH_NONE)
    count_cache_flush_type = BRANCH_CACHE_FLUSH_NONE;
    pr_info("count-cache-flush: flush disabled.\n");
    } else {
    if (security_ftr_enabled(SEC_FTR_BCCTR_FLUSH_ASSIST)) {
    count_cache_flush_type = BRANCH_CACHE_FLUSH_HW;
    pr_info("count-cache-flush: hardware flush enabled.\n");
    } else {
    count_cache_flush_type = BRANCH_CACHE_FLUSH_SW;
    pr_info("count-cache-flush: software flush enabled.\n");
    }
    }
    if (!enable || !security_ftr_enabled(SEC_FTR_FLUSH_LINK_STACK)) {
    if (link_stack_flush_type != BRANCH_CACHE_FLUSH_NONE)
    link_stack_flush_type = BRANCH_CACHE_FLUSH_NONE;
    pr_info("link-stack-flush: flush disabled.\n");
    } else {
    if (security_ftr_enabled(SEC_FTR_BCCTR_LINK_FLUSH_ASSIST)) {
    link_stack_flush_type = BRANCH_CACHE_FLUSH_HW;
    pr_info("link-stack-flush: hardware flush enabled.\n");
    } else {
    link_stack_flush_type = BRANCH_CACHE_FLUSH_SW;
    pr_info("link-stack-flush: software flush enabled.\n");
    }
    }
    update_branch_cache_flush();
    }
#[no_mangle]
pub unsafe extern "C" fn setup_count_cache_flush() {
    void setup_count_cache_flush(void)
    {
    let mut enable: bool = true;
    if (no_spectrev2 || cpu_mitigations_off()) {
    if (security_ftr_enabled(SEC_FTR_BCCTRL_SERIALISED) ||
    security_ftr_enabled(SEC_FTR_COUNT_CACHE_DISABLED))
    pr_warn("Spectre v2 mitigations not fully under software control, can't disable\n");
    enable = false;
    }
//
// There's no firmware feature flag/hypervisor bit to tell us we need to
// flush the link stack on context switch. So we set it here if we see
// either of the Spectre v2 mitigations that aim to protect userspace.
//
    if (security_ftr_enabled(SEC_FTR_COUNT_CACHE_DISABLED) ||
    security_ftr_enabled(SEC_FTR_FLUSH_COUNT_CACHE))
    security_ftr_set(SEC_FTR_FLUSH_LINK_STACK);
    toggle_branch_cache_flush(enable);
    }
    static enum l1d_flush_type enabled_flush_types;
    static void *l1d_flush_fallback_area;
    static bool no_rfi_flush;
    static bool no_entry_flush;
    static bool no_uaccess_flush;
    bool rfi_flush;
    static bool entry_flush;
    static bool uaccess_flush;
    DEFINE_STATIC_KEY_FALSE(uaccess_flush_key);
    EXPORT_SYMBOL(uaccess_flush_key);
#[no_mangle]
unsafe extern "C" fn handle_no_rfi_flush(p: *mut c_char) -> int __init {
    static int __init handle_no_rfi_flush(char *p)
    {
    pr_info("rfi-flush: disabled on command line.");
    no_rfi_flush = true;
    return 0;
    }
    early_param("no_rfi_flush", handle_no_rfi_flush);
#[no_mangle]
unsafe extern "C" fn handle_no_entry_flush(p: *mut c_char) -> int __init {
    static int __init handle_no_entry_flush(char *p)
    {
    pr_info("entry-flush: disabled on command line.");
    no_entry_flush = true;
    return 0;
    }
    early_param("no_entry_flush", handle_no_entry_flush);
#[no_mangle]
unsafe extern "C" fn handle_no_uaccess_flush(p: *mut c_char) -> int __init {
    static int __init handle_no_uaccess_flush(char *p)
    {
    pr_info("uaccess-flush: disabled on command line.");
    no_uaccess_flush = true;
    return 0;
    }
    early_param("no_uaccess_flush", handle_no_uaccess_flush);
//
// The RFI flush is not KPTI, but because users will see doco that says to use
// nopti we hijack that option here to also disable the RFI flush.
//
#[no_mangle]
unsafe extern "C" fn handle_no_pti(p: *mut c_char) -> int __init {
    static int __init handle_no_pti(char *p)
    {
    pr_info("rfi-flush: disabling due to 'nopti' on command line.\n");
    handle_no_rfi_flush(core::ptr::null_mut());
    return 0;
    }
    early_param("nopti", handle_no_pti);
#[no_mangle]
unsafe extern "C" fn do_nothing(unused: *mut c_void) {
    static void do_nothing(void *unused)
    {
//
// We don't need to do the flush explicitly, just enter+exit kernel is
// sufficient, the RFI exit handlers will do the right thing.
//
    }
#[no_mangle]
pub unsafe extern "C" fn rfi_flush_enable(enable: bool) {
    void rfi_flush_enable(bool enable)
    {
    if (enable) {
    do_rfi_flush_fixups(enabled_flush_types);
    on_each_cpu(do_nothing, core::ptr::null_mut(), 1);
    } else
    do_rfi_flush_fixups(L1D_FLUSH_NONE);
    rfi_flush = enable;
    }
#[no_mangle]
unsafe extern "C" fn entry_flush_enable(enable: bool) {
    static void entry_flush_enable(bool enable)
    {
    if (enable) {
    do_entry_flush_fixups(enabled_flush_types);
    on_each_cpu(do_nothing, core::ptr::null_mut(), 1);
    } else {
    do_entry_flush_fixups(L1D_FLUSH_NONE);
    }
    entry_flush = enable;
    }
#[no_mangle]
unsafe extern "C" fn uaccess_flush_enable(enable: bool) {
    static void uaccess_flush_enable(bool enable)
    {
    if (enable) {
    do_uaccess_flush_fixups(enabled_flush_types);
    static_branch_enable(&uaccess_flush_key);
    on_each_cpu(do_nothing, core::ptr::null_mut(), 1);
    } else {
    static_branch_disable(&uaccess_flush_key);
    do_uaccess_flush_fixups(L1D_FLUSH_NONE);
    }
    uaccess_flush = enable;
    }
#[no_mangle]
unsafe extern "C" fn init_fallback_flush() -> void __ref {
    static void __ref init_fallback_flush(void)
    {
    u64 l1d_size, limit;
    int cpu;
// Only allocate the fallback flush area once (at boot time).
    if (l1d_flush_fallback_area)
    return;
    l1d_size = ppc64_caches.l1d.size;
//
// If there is no d-cache-size property in the device tree, l1d_size
// could be zero. That leads to the loop in the asm wrapping around to
// 2^64-1, and then walking off the end of the fallback area and
// eventually causing a page fault which is fatal. Just default to
// something vaguely sane.
//
    if (!l1d_size)
    l1d_size = (64 * 1024);
    limit = min(ppc64_bolted_size(), ppc64_rma_size);
//
// Align to L1d size, and size it at 2x L1d size, to catch possible
// hardware prefetch runoff. We don't have a recipe for load patterns to
// reliably avoid the prefetcher.
//
    l1d_flush_fallback_area = memblock_alloc_try_nid(l1d_size * 2,
    l1d_size, MEMBLOCK_LOW_LIMIT,
    limit, NUMA_NO_NODE);
    if (!l1d_flush_fallback_area)
    panic("%s: Failed to allocate %llu bytes align=0x%llx max_addr=%pa\n",
    __func__, l1d_size * 2, l1d_size, &limit);
    for_each_possible_cpu(cpu) {
    struct paca_struct *paca = paca_ptrs[cpu];
    paca.rfi_flush_fallback_area = l1d_flush_fallback_area;
    paca.l1d_flush_size = l1d_size;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn setup_rfi_flush(types: enum l1d_flush_type, enable: bool) {
    void setup_rfi_flush(enum l1d_flush_type types, bool enable)
    {
    if (types & L1D_FLUSH_FALLBACK) {
    pr_info("rfi-flush: fallback displacement flush available\n");
    init_fallback_flush();
    }
    if (types & L1D_FLUSH_ORI)
    pr_info("rfi-flush: ori type flush available\n");
    if (types & L1D_FLUSH_MTTRIG)
    pr_info("rfi-flush: mttrig type flush available\n");
    enabled_flush_types = types;
    if (!cpu_mitigations_off() && !no_rfi_flush)
    rfi_flush_enable(enable);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_entry_flush(enable: bool) {
    void setup_entry_flush(bool enable)
    {
    if (cpu_mitigations_off())
    return;
    if (!no_entry_flush)
    entry_flush_enable(enable);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_uaccess_flush(enable: bool) {
    void setup_uaccess_flush(bool enable)
    {
    if (cpu_mitigations_off())
    return;
    if (!no_uaccess_flush)
    uaccess_flush_enable(enable);
    }

#[no_mangle]
unsafe extern "C" fn count_cache_flush_set(data: *mut c_void, val: u64) -> c_int {
    static int count_cache_flush_set(void *data, u64 val)
    {
    bool enable;
    if (val == 1)
    enable = true;
#[no_mangle]
pub unsafe extern "C" fn if(0: val ==) -> else {
    else if (val == 0)
    enable = false;
    else
    return -EINVAL;
    toggle_branch_cache_flush(enable);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn count_cache_flush_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int count_cache_flush_get(void *data, u64 *val)
    {
    if (count_cache_flush_type == BRANCH_CACHE_FLUSH_NONE)
// val = 0;
    else
// val = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn link_stack_flush_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int link_stack_flush_get(void *data, u64 *val)
    {
    if (link_stack_flush_type == BRANCH_CACHE_FLUSH_NONE)
// val = 0;
    else
// val = 1;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_count_cache_flush, count_cache_flush_get,
    count_cache_flush_set, "%llu\n");
    DEFINE_DEBUGFS_ATTRIBUTE(fops_link_stack_flush, link_stack_flush_get,
    count_cache_flush_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn count_cache_flush_debugfs_init() -> __init int {
    static __init int count_cache_flush_debugfs_init(void)
    {
    debugfs_create_file_unsafe("count_cache_flush", 0600,
    arch_debugfs_dir, core::ptr::null_mut(),
    &fops_count_cache_flush);
    debugfs_create_file_unsafe("link_stack_flush", 0600,
    arch_debugfs_dir, core::ptr::null_mut(),
    &fops_link_stack_flush);
    return 0;
    }
    device_initcall(count_cache_flush_debugfs_init);
#[no_mangle]
unsafe extern "C" fn rfi_flush_set(data: *mut c_void, val: u64) -> c_int {
    static int rfi_flush_set(void *data, u64 val)
    {
    bool enable;
    if (val == 1)
    enable = true;
#[no_mangle]
pub unsafe extern "C" fn if(0: val ==) -> else {
    else if (val == 0)
    enable = false;
    else
    return -EINVAL;
// Only do anything if we're changing state
    if (enable != rfi_flush)
    rfi_flush_enable(enable);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rfi_flush_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int rfi_flush_get(void *data, u64 *val)
    {
// val = rfi_flush ? 1 : 0;
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(fops_rfi_flush, rfi_flush_get, rfi_flush_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn entry_flush_set(data: *mut c_void, val: u64) -> c_int {
    static int entry_flush_set(void *data, u64 val)
    {
    bool enable;
    if (val == 1)
    enable = true;
#[no_mangle]
pub unsafe extern "C" fn if(0: val ==) -> else {
    else if (val == 0)
    enable = false;
    else
    return -EINVAL;
// Only do anything if we're changing state
    if (enable != entry_flush)
    entry_flush_enable(enable);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn entry_flush_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int entry_flush_get(void *data, u64 *val)
    {
// val = entry_flush ? 1 : 0;
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(fops_entry_flush, entry_flush_get, entry_flush_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn uaccess_flush_set(data: *mut c_void, val: u64) -> c_int {
    static int uaccess_flush_set(void *data, u64 val)
    {
    bool enable;
    if (val == 1)
    enable = true;
#[no_mangle]
pub unsafe extern "C" fn if(0: val ==) -> else {
    else if (val == 0)
    enable = false;
    else
    return -EINVAL;
// Only do anything if we're changing state
    if (enable != uaccess_flush)
    uaccess_flush_enable(enable);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uaccess_flush_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int uaccess_flush_get(void *data, u64 *val)
    {
// val = uaccess_flush ? 1 : 0;
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(fops_uaccess_flush, uaccess_flush_get, uaccess_flush_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn rfi_flush_debugfs_init() -> __init int {
    static __init int rfi_flush_debugfs_init(void)
    {
    debugfs_create_file("rfi_flush", 0600, arch_debugfs_dir, core::ptr::null_mut(), &fops_rfi_flush);
    debugfs_create_file("entry_flush", 0600, arch_debugfs_dir, core::ptr::null_mut(), &fops_entry_flush);
    debugfs_create_file("uaccess_flush", 0600, arch_debugfs_dir, core::ptr::null_mut(), &fops_uaccess_flush);
    return 0;
    }
    device_initcall(rfi_flush_debugfs_init);

