//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mce/inject.c
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
// Machine check injection support.
// Copyright 2008 Intel Corporation.
//
// Authors:
// Andi Kleen
// Ying Huang
//
// The AMD part (from mce_amd_inj.c): a simple MCE injection facility
// for testing different aspects of the RAS code. This driver should be
// built as module so that it can be loaded on production kernels for
// testing purposes.
//
// Copyright (c) 2010-17:  Borislav Petkov <bp@alien8.de>
// Advanced Micro Devices Inc.
//

    let mut hw_injection_possible: static bool = true;
//
// Collect all the MCi_XXX settings
//
    static struct mce i_mce;
    static struct dentry *dfs_inj;
pub const MAX_FLAG_OPT_SIZE: c_int = 4;
pub const NBCFG: c_uint = 0x44;
    enum injection_type {
    SW_INJ = 0,	/* SW injection, simply decode the error */
    HW_INJ,		/* Trigger a #MC */
    DFR_INT_INJ,    /* Trigger Deferred error interrupt */
    THR_INT_INJ,    /* Trigger threshold interrupt */
    N_INJ_TYPES,
    };
    static const char * const flags_options[] = {
    [SW_INJ] = "sw",
    [HW_INJ] = "hw",
    [DFR_INT_INJ] = "df",
    [THR_INT_INJ] = "th",
    core::ptr::null_mut()
    };
// Set default injection to SW_INJ
    let mut inj_type: static enum injection_type = SW_INJ;

    static int inj_##reg##_set(void *data, u64 val)				\
    {									\
    struct mce *m = (struct mce *)data;				\
    \
    m.reg = val;							\
    return 0;							\
    }
    MCE_INJECT_SET(status);
    MCE_INJECT_SET(misc);
    MCE_INJECT_SET(addr);
    MCE_INJECT_SET(synd);

    static int inj_##reg##_get(void *data, u64 *val)			\
    {									\
    struct mce *m = (struct mce *)data;				\
    \
// val = m->reg;							\
    return 0;							\
    }
    MCE_INJECT_GET(status);
    MCE_INJECT_GET(misc);
    MCE_INJECT_GET(addr);
    MCE_INJECT_GET(synd);
    MCE_INJECT_GET(ipid);
    DEFINE_SIMPLE_ATTRIBUTE(status_fops, inj_status_get, inj_status_set, "%llx\n");
    DEFINE_SIMPLE_ATTRIBUTE(misc_fops, inj_misc_get, inj_misc_set, "%llx\n");
    DEFINE_SIMPLE_ATTRIBUTE(addr_fops, inj_addr_get, inj_addr_set, "%llx\n");
    DEFINE_SIMPLE_ATTRIBUTE(synd_fops, inj_synd_get, inj_synd_set, "%llx\n");
// Use the user provided IPID value on a sw injection.
#[no_mangle]
unsafe extern "C" fn inj_ipid_set(data: *mut c_void, val: u64) -> c_int {
    static int inj_ipid_set(void *data, u64 val)
    {
    struct mce *m = (struct mce *)data;
    if (cpu_feature_enabled(X86_FEATURE_SMCA)) {
    if (inj_type == SW_INJ)
    m.ipid = val;
    }
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(ipid_fops, inj_ipid_get, inj_ipid_set, "%llx\n");
#[no_mangle]
unsafe extern "C" fn setup_inj_struct(m: *mut mce) {
    static void setup_inj_struct(struct mce *m)
    {
    memset(m, 0, sizeof(struct mce));
    m.cpuvendor = boot_cpu_data.x86_vendor;
    m.time	     = ktime_get_real_seconds();
    m.cpuid     = cpuid_eax(1);
    m.microcode = boot_cpu_data.microcode;
    }
// Update fake mce registers on current CPU.
#[no_mangle]
unsafe extern "C" fn inject_mce(m: *mut mce) {
    static void inject_mce(struct mce *m)
    {
    struct mce *i = &per_cpu(injectm, m.extcpu);
// Make sure no one reads partially written injectm
    i.finished = 0;
    mb();
    m.finished = 0;
// First set the fields after finished
    i.extcpu = m.extcpu;
    mb();
// Now write record in order, finished last (except above)
    memcpy(i, m, sizeof(struct mce));
// Finally activate it
    mb();
    i.finished = 1;
    }
#[no_mangle]
unsafe extern "C" fn raise_poll(m: *mut mce) {
    static void raise_poll(struct mce *m)
    {
    unsigned long flags;
    mce_banks_t b;
    memset(&b, 0xff, sizeof(mce_banks_t));
    local_irq_save(flags);
    machine_check_poll(0, &b);
    local_irq_restore(flags);
    m.finished = 0;
    }
#[no_mangle]
unsafe extern "C" fn raise_exception(m: *mut mce, pregs: *mut pt_regs) {
    static void raise_exception(struct mce *m, struct pt_regs *pregs)
    {
    struct pt_regs regs;
    unsigned long flags;
    if (!pregs) {
    memset(&regs, 0, sizeof(struct pt_regs));
    regs.ip = m.ip;
    regs.cs = m.cs;
    pregs = &regs;
    }
// do_machine_check() expects interrupts disabled -- at least
    local_irq_save(flags);
    do_machine_check(pregs);
    local_irq_restore(flags);
    m.finished = 0;
    }
    static cpumask_var_t mce_inject_cpumask;
    static DEFINE_MUTEX(mce_inject_mutex);
#[no_mangle]
unsafe extern "C" fn mce_raise_notify(cmd: c_uint, regs: *mut pt_regs) -> c_int {
    static int mce_raise_notify(unsigned int cmd, struct pt_regs *regs)
    {
    let mut cpu: c_int = smp_processor_id();
    struct mce *m = this_cpu_ptr(&injectm);
    if (!cpumask_test_cpu(cpu, mce_inject_cpumask))
    return NMI_DONE;
    cpumask_clear_cpu(cpu, mce_inject_cpumask);
    if (m.inject_flags & MCJ_EXCEPTION)
    raise_exception(m, regs);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: m->status) -> else {
    else if (m.status)
    raise_poll(m);
    return NMI_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mce_irq_ipi(info: *mut c_void) {
    static void mce_irq_ipi(void *info)
    {
    let mut cpu: c_int = smp_processor_id();
    struct mce *m = this_cpu_ptr(&injectm);
    if (cpumask_test_cpu(cpu, mce_inject_cpumask) &&
    m.inject_flags & MCJ_EXCEPTION) {
    cpumask_clear_cpu(cpu, mce_inject_cpumask);
    raise_exception(m, core::ptr::null_mut());
    }
    }
// Inject mce on current CPU
#[no_mangle]
unsafe extern "C" fn raise_local() -> c_int {
    static int raise_local(void)
    {
    struct mce *m = this_cpu_ptr(&injectm);
    let mut context: c_int = MCJ_CTX(m.inject_flags);
    let mut ret: c_int = 0;
    let mut cpu: c_int = m.extcpu;
    if (m.inject_flags & MCJ_EXCEPTION) {
    pr_info("Triggering MCE exception on CPU %d\n", cpu);
    switch (context) {
    case MCJ_CTX_IRQ:
//
// Could do more to fake interrupts like
// calling irq_enter, but the necessary
// machinery isn't exported currently.
//
    fallthrough;
    case MCJ_CTX_PROCESS:
    raise_exception(m, core::ptr::null_mut());
    break;
    default:
    pr_info("Invalid MCE context\n");
    ret = -EINVAL;
    }
    pr_info("MCE exception done on CPU %d\n", cpu);
    } else if (m.status) {
    pr_info("Starting machine check poll CPU %d\n", cpu);
    raise_poll(m);
    pr_info("Machine check poll done on CPU %d\n", cpu);
    } else
    m.finished = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn raise_mce(m: *mut mce) -> void __maybe_unused {
    static void __maybe_unused raise_mce(struct mce *m)
    {
    let mut context: c_int = MCJ_CTX(m.inject_flags);
    inject_mce(m);
    if (context == MCJ_CTX_RANDOM)
    return;
    if (m.inject_flags & (MCJ_IRQ_BROADCAST | MCJ_NMI_BROADCAST)) {
    unsigned long start;
    int cpu;
    cpus_read_lock();
    cpumask_copy(mce_inject_cpumask, cpu_online_mask);
    cpumask_clear_cpu(get_cpu(), mce_inject_cpumask);
    for_each_online_cpu(cpu) {
    struct mce *mcpu = &per_cpu(injectm, cpu);
    if (!mcpu.finished ||
    MCJ_CTX(mcpu.inject_flags) != MCJ_CTX_RANDOM)
    cpumask_clear_cpu(cpu, mce_inject_cpumask);
    }
    if (!cpumask_empty(mce_inject_cpumask)) {
    if (m.inject_flags & MCJ_IRQ_BROADCAST) {
//
// don't wait because mce_irq_ipi is necessary
// to be sync with following raise_local
//
    preempt_disable();
    smp_call_function_many(mce_inject_cpumask,
    mce_irq_ipi, core::ptr::null_mut(), 0);
    preempt_enable();
    } else if (m.inject_flags & MCJ_NMI_BROADCAST)
    __apic_send_IPI_mask(mce_inject_cpumask, NMI_VECTOR);
    }
    start = jiffies;
    while (!cpumask_empty(mce_inject_cpumask)) {
    if (!time_before(jiffies, start + 2*HZ)) {
    pr_err("Timeout waiting for mce inject %lx\n",
// cpumask_bits(mce_inject_cpumask));
    break;
    }
    cpu_relax();
    }
    raise_local();
    put_cpu();
    cpus_read_unlock();
    } else {
    preempt_disable();
    raise_local();
    preempt_enable();
    }
    }
    static int mce_inject_raise(struct notifier_block *nb, unsigned long val,
    void *data)
    {
    struct mce *m = (struct mce *)data;
    if (!m)
    return NOTIFY_DONE;
    mutex_lock(&mce_inject_mutex);
    raise_mce(m);
    mutex_unlock(&mce_inject_mutex);
    return NOTIFY_DONE;
    }
    static struct notifier_block inject_nb = {
    .notifier_call  = mce_inject_raise,
    };
//
// Caller needs to be make sure this cpu doesn't disappear
// from under us, i.e.: get_cpu/put_cpu.
//
#[no_mangle]
unsafe extern "C" fn toggle_hw_mce_inject(cpu: c_uint, enable: bool) -> c_int {
    static int toggle_hw_mce_inject(unsigned int cpu, bool enable)
    {
    struct msr val;
    int err;
    err = rdmsrq_on_cpu(cpu, MSR_K7_HWCR, &val.q);
    if (err) {
    pr_err("%s: error reading HWCR\n", __func__);
    return err;
    }
    enable ? (val.l |= BIT(18)) : (val.l &= ~BIT(18));
    err = wrmsrq_on_cpu(cpu, MSR_K7_HWCR, val.q);
    if (err)
    pr_err("%s: error writing HWCR\n", __func__);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __set_inj(buf: *const c_char) -> c_int {
    static int __set_inj(const char *buf)
    {
    int i;
    for (i = 0; i < N_INJ_TYPES; i++) {
    if (!strncmp(flags_options[i], buf, strlen(flags_options[i]))) {
    if (i > SW_INJ && !hw_injection_possible)
    continue;
    inj_type = i;
    return 0;
    }
    }
    return -EINVAL;
    }
    static ssize_t flags_read(struct file *filp, char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    char buf[MAX_FLAG_OPT_SIZE];
    int n;
    n = sprintf(buf, "%s\n", flags_options[inj_type]);
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, n);
    }
    static ssize_t flags_write(struct file *filp, const char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    char buf[MAX_FLAG_OPT_SIZE], *__buf;
    int err;
    if (!cnt || cnt > MAX_FLAG_OPT_SIZE)
    return -EINVAL;
    if (copy_from_user(&buf, ubuf, cnt))
    return -EFAULT;
    buf[cnt - 1] = 0;
// strip whitespace
    __buf = strstrip(buf);
    err = __set_inj(__buf);
    if (err) {
    pr_err("%s: Invalid flags value: %s\n", __func__, __buf);
    return err;
    }
// ppos += cnt;
    return cnt;
    }
    static const struct file_operations flags_fops = {
    .read           = flags_read,
    .write          = flags_write,
    .llseek         = generic_file_llseek,
    };
//
// On which CPU to inject?
//
    MCE_INJECT_GET(extcpu);
#[no_mangle]
unsafe extern "C" fn inj_extcpu_set(data: *mut c_void, val: u64) -> c_int {
    static int inj_extcpu_set(void *data, u64 val)
    {
    struct mce *m = (struct mce *)data;
    if (val >= nr_cpu_ids || !cpu_online(val)) {
    pr_err("%s: Invalid CPU: %llu\n", __func__, val);
    return -EINVAL;
    }
    m.extcpu = val;
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(extcpu_fops, inj_extcpu_get, inj_extcpu_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn trigger_mce(info: *mut c_void) {
    static void trigger_mce(void *info)
    {
    asm volatile("int $18");
    }
#[no_mangle]
unsafe extern "C" fn trigger_dfr_int(info: *mut c_void) {
    static void trigger_dfr_int(void *info)
    {
    asm volatile("int %0" :: "i" (DEFERRED_ERROR_VECTOR));
    }
#[no_mangle]
unsafe extern "C" fn trigger_thr_int(info: *mut c_void) {
    static void trigger_thr_int(void *info)
    {
    asm volatile("int %0" :: "i" (THRESHOLD_APIC_VECTOR));
    }
#[no_mangle]
unsafe extern "C" fn get_nbc_for_node(node_id: c_int) -> u32 {
    static u32 get_nbc_for_node(int node_id)
    {
    u32 cores_per_node;
    cores_per_node = topology_num_threads_per_package() / topology_amd_nodes_per_pkg();
    return cores_per_node * node_id;
    }
#[no_mangle]
unsafe extern "C" fn toggle_nb_mca_mst_cpu(nid: u16) {
    static void toggle_nb_mca_mst_cpu(u16 nid)
    {
    struct amd_northbridge *nb;
    struct pci_dev *F3;
    u32 val;
    int err;
    nb = node_to_amd_nb(nid);
    if (!nb)
    return;
    F3 = nb.misc;
    if (!F3)
    return;
    err = pci_read_config_dword(F3, NBCFG, &val);
    if (err) {
    pr_err("%s: Error reading F%dx%03x.\n",
    __func__, PCI_FUNC(F3.devfn), NBCFG);
    return;
    }
    if (val & BIT(27))
    return;
    pr_err("%s: Set D18F3x44[NbMcaToMstCpuEn] which BIOS hasn't done.\n",
    __func__);
    val |= BIT(27);
    err = pci_write_config_dword(F3, NBCFG, val);
    if (err)
    pr_err("%s: Error writing F%dx%03x.\n",
    __func__, PCI_FUNC(F3.devfn), NBCFG);
    }
#[no_mangle]
unsafe extern "C" fn prepare_msrs(info: *mut c_void) {
    static void prepare_msrs(void *info)
    {
    let mut m: mce = *(struct mce *)info;
    let mut b: u8 = m.bank;
    wrmsrq(MSR_IA32_MCG_STATUS, m.mcgstatus);
    if (boot_cpu_has(X86_FEATURE_SMCA)) {
    if (m.inject_flags == DFR_INT_INJ) {
    wrmsrq(MSR_AMD64_SMCA_MCx_DESTAT(b), m.status);
    wrmsrq(MSR_AMD64_SMCA_MCx_DEADDR(b), m.addr);
    } else {
    wrmsrq(MSR_AMD64_SMCA_MCx_STATUS(b), m.status);
    wrmsrq(MSR_AMD64_SMCA_MCx_ADDR(b), m.addr);
    }
    wrmsrq(MSR_AMD64_SMCA_MCx_SYND(b), m.synd);
    if (m.misc)
    wrmsrq(MSR_AMD64_SMCA_MCx_MISC(b), m.misc);
    } else {
    wrmsrq(MSR_IA32_MCx_STATUS(b), m.status);
    wrmsrq(MSR_IA32_MCx_ADDR(b), m.addr);
    if (m.misc)
    wrmsrq(MSR_IA32_MCx_MISC(b), m.misc);
    }
    }
#[no_mangle]
unsafe extern "C" fn do_inject() {
    static void do_inject(void)
    {
    let mut cpu: c_uint = i_mce.extcpu;
    struct mce_hw_err err;
    let mut mcg_status: u64 = 0;
    let mut b: u8 = i_mce.bank;
    i_mce.tsc = rdtsc_ordered();
    i_mce.status |= MCI_STATUS_VAL;
    if (i_mce.misc)
    i_mce.status |= MCI_STATUS_MISCV;
    if (i_mce.synd)
    i_mce.status |= MCI_STATUS_SYNDV;
    if (inj_type == SW_INJ) {
    err.m = i_mce;
    mce_log(&err);
    return;
    }
// prep MCE global settings for the injection
    mcg_status = MCG_STATUS_MCIP | MCG_STATUS_EIPV;
    if (!(i_mce.status & MCI_STATUS_PCC))
    mcg_status |= MCG_STATUS_RIPV;
//
// Ensure necessary status bits for deferred errors:
// - MCx_STATUS[Deferred]: make sure it is a deferred error
// - MCx_STATUS[UC] cleared: deferred errors are _not_ UC
//
    if (inj_type == DFR_INT_INJ) {
    i_mce.status |= MCI_STATUS_DEFERRED;
    i_mce.status &= ~MCI_STATUS_UC;
    }
//
// For multi node CPUs, logging and reporting of bank 4 errors happens
// only on the node base core. Refer to D18F3x44[NbMcaToMstCpuEn] for
// Fam10h and later BKDGs.
//
    if (boot_cpu_has(X86_FEATURE_AMD_DCM) &&
    b == 4 &&
    boot_cpu_data.x86 < 0x17) {
    toggle_nb_mca_mst_cpu(topology_amd_node_id(cpu));
    cpu = get_nbc_for_node(topology_amd_node_id(cpu));
    }
    cpus_read_lock();
    if (!cpu_online(cpu))
    goto err;
    toggle_hw_mce_inject(cpu, true);
    i_mce.mcgstatus = mcg_status;
    i_mce.inject_flags = inj_type;
    smp_call_function_single(cpu, prepare_msrs, &i_mce, 0);
    toggle_hw_mce_inject(cpu, false);
    switch (inj_type) {
    case DFR_INT_INJ:
    smp_call_function_single(cpu, trigger_dfr_int, core::ptr::null_mut(), 0);
    break;
    case THR_INT_INJ:
    smp_call_function_single(cpu, trigger_thr_int, core::ptr::null_mut(), 0);
    break;
    default:
    smp_call_function_single(cpu, trigger_mce, core::ptr::null_mut(), 0);
    }
    err:
    cpus_read_unlock();
    }
//
// This denotes into which bank we're injecting and triggers
// the injection, at the same time.
//
#[no_mangle]
unsafe extern "C" fn inj_bank_set(data: *mut c_void, val: u64) -> c_int {
    static int inj_bank_set(void *data, u64 val)
    {
    struct mce *m = (struct mce *)data;
    u8 n_banks;
    u64 cap;
// Get bank count on target CPU so we can handle non-uniform values.
    rdmsrq_on_cpu(m.extcpu, MSR_IA32_MCG_CAP, &cap);
    n_banks = cap & MCG_BANKCNT_MASK;
    if (val >= n_banks) {
    pr_err("MCA bank %llu non-existent on CPU%d\n", val, m.extcpu);
    return -EINVAL;
    }
    m.bank = val;
//
// sw-only injection allows to write arbitrary values into the MCA
// registers because it tests only the decoding paths.
//
    if (inj_type == SW_INJ)
    goto inject;
//
// Read IPID value to determine if a bank is populated on the target
// CPU.
//
    if (cpu_feature_enabled(X86_FEATURE_SMCA)) {
    u64 ipid;
    if (rdmsrq_on_cpu(m.extcpu, MSR_AMD64_SMCA_MCx_IPID(val), &ipid)) {
    pr_err("Error reading IPID on CPU%d\n", m.extcpu);
    return -EINVAL;
    }
    if (!ipid) {
    pr_err("Cannot inject into unpopulated bank %llu\n", val);
    return -ENODEV;
    }
    }
    inject:
    do_inject();
// Reset injection struct
    setup_inj_struct(&i_mce);
    return 0;
    }
    MCE_INJECT_GET(bank);
    DEFINE_SIMPLE_ATTRIBUTE(bank_fops, inj_bank_get, inj_bank_set, "%llu\n");
    static const char readme_msg[] =
    "Description of the files and their usages:\n"
    "\n"
    "Note1: i refers to the bank number below.\n"
    "Note2: See respective BKDGs for the exact bit definitions of the files below\n"
    "as they mirror the hardware registers.\n"
    "\n"
    "status:\t Set MCi_STATUS: the bits in that MSR control the error type and\n"
    "\t attributes of the error which caused the MCE.\n"
    "\n"
    "misc:\t Set MCi_MISC: provide auxiliary info about the error. It is mostly\n"
    "\t used for error thresholding purposes and its validity is indicated by\n"
    "\t MCi_STATUS[MiscV].\n"
    "\n"
    "synd:\t Set MCi_SYND: provide syndrome info about the error. Only valid on\n"
    "\t Scalable MCA systems, and its validity is indicated by MCi_STATUS[SyndV].\n"
    "\n"
    "addr:\t Error address value to be written to MCi_ADDR. Log address information\n"
    "\t associated with the error.\n"
    "\n"
    "cpu:\t The CPU to inject the error on.\n"
    "\n"
    "bank:\t Specify the bank you want to inject the error into: the number of\n"
    "\t banks in a processor varies and is family/model-specific, therefore, the\n"
    "\t supplied value is sanity-checked. Setting the bank value also triggers the\n"
    "\t injection.\n"
    "\n"
    "flags:\t Injection type to be performed. Writing to this file will trigger a\n"
    "\t real machine check, an APIC interrupt or invoke the error decoder routines\n"
    "\t for AMD processors.\n"
    "\n"
    "\t Allowed error injection types:\n"
    "\t  - \"sw\": Software error injection. Decode error to a human-readable \n"
    "\t    format only. Safe to use.\n"
    "\t  - \"hw\": Hardware error injection. Causes the #MC exception handler to \n"
    "\t    handle the error. Be warned: might cause system panic if MCi_STATUS[PCC] \n"
    "\t    is set. Therefore, consider setting (debugfs_mountpoint)/mce/fake_panic \n"
    "\t    before injecting.\n"
    "\t  - \"df\": Trigger APIC interrupt for Deferred error. Causes deferred \n"
    "\t    error APIC interrupt handler to handle the error if the feature is \n"
    "\t    is present in hardware. \n"
    "\t  - \"th\": Trigger APIC interrupt for Threshold errors. Causes threshold \n"
    "\t    APIC interrupt handler to handle the error. \n"
    "\n"
    "ipid:\t IPID (AMD-specific)\n"
    "\n";
    static ssize_t
    inj_readme_read(struct file *filp, char __user *ubuf,
    size_t cnt, loff_t *ppos)
    {
    return simple_read_from_buffer(ubuf, cnt, ppos,
    readme_msg, strlen(readme_msg));
    }
    static const struct file_operations readme_fops = {
    .read		= inj_readme_read,
    };
    static struct dfs_node {
    char *name;
    const struct file_operations *fops;
    umode_t perm;
    } dfs_fls[] = {
    { .name = "status",	.fops = &status_fops, .perm = S_IRUSR | S_IWUSR },
    { .name = "misc",	.fops = &misc_fops,   .perm = S_IRUSR | S_IWUSR },
    { .name = "addr",	.fops = &addr_fops,   .perm = S_IRUSR | S_IWUSR },
    { .name = "synd",	.fops = &synd_fops,   .perm = S_IRUSR | S_IWUSR },
    { .name = "ipid",	.fops = &ipid_fops,   .perm = S_IRUSR | S_IWUSR },
    { .name = "bank",	.fops = &bank_fops,   .perm = S_IRUSR | S_IWUSR },
    { .name = "flags",	.fops = &flags_fops,  .perm = S_IRUSR | S_IWUSR },
    { .name = "cpu",	.fops = &extcpu_fops, .perm = S_IRUSR | S_IWUSR },
    { .name = "README",	.fops = &readme_fops, .perm = S_IRUSR | S_IRGRP | S_IROTH },
    };
#[no_mangle]
unsafe extern "C" fn debugfs_init() -> void __init {
    static void __init debugfs_init(void)
    {
    unsigned int i;
    dfs_inj = debugfs_create_dir("mce-inject", core::ptr::null_mut());
    for (i = 0; i < ARRAY_SIZE(dfs_fls); i++)
    debugfs_create_file(dfs_fls[i].name, dfs_fls[i].perm, dfs_inj,
    &i_mce, dfs_fls[i].fops);
    }
#[no_mangle]
unsafe extern "C" fn check_hw_inj_possible() {
    static void check_hw_inj_possible(void)
    {
    int cpu;
    u8 bank;
//
// This behavior exists only on SMCA systems though its not directly
// related to SMCA.
//
    if (!cpu_feature_enabled(X86_FEATURE_SMCA))
    return;
    cpu = get_cpu();
    for (bank = 0; bank < MAX_NR_BANKS; ++bank) {
    let mut status: u64 = MCI_STATUS_VAL, ipid;
// Check whether bank is populated
    rdmsrq(MSR_AMD64_SMCA_MCx_IPID(bank), ipid);
    if (!ipid)
    continue;
    toggle_hw_mce_inject(cpu, true);
    wrmsrq_safe(mca_msr_reg(bank, MCA_STATUS), status);
    rdmsrq_safe(mca_msr_reg(bank, MCA_STATUS), &status);
    wrmsrq_safe(mca_msr_reg(bank, MCA_STATUS), 0);
    if (!status) {
    hw_injection_possible = false;
    pr_warn("Platform does not allow *hardware* error injection."
    "Try using APEI EINJ instead.\n");
    }
    toggle_hw_mce_inject(cpu, false);
    break;
    }
    put_cpu();
    }
#[no_mangle]
unsafe extern "C" fn inject_init() -> int __init {
    static int __init inject_init(void)
    {
    if (!alloc_cpumask_var(&mce_inject_cpumask, GFP_KERNEL))
    return -ENOMEM;
    check_hw_inj_possible();
    debugfs_init();
    register_nmi_handler(NMI_LOCAL, mce_raise_notify, 0, "mce_notify");
    mce_register_injector_chain(&inject_nb);
    setup_inj_struct(&i_mce);
    pr_info("Machine check injector initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inject_exit() -> void __exit {
    static void __exit inject_exit(void)
    {
    mce_unregister_injector_chain(&inject_nb);
    unregister_nmi_handler(NMI_LOCAL, "mce_notify");
    debugfs_remove_recursive(dfs_inj);
    dfs_inj = core::ptr::null_mut();
    memset(&dfs_fls, 0, sizeof(dfs_fls));
    free_cpumask_var(mce_inject_cpumask);
    }
    module_init(inject_init);
    module_exit(inject_exit);
    MODULE_DESCRIPTION("Machine check injection support");
    MODULE_LICENSE("GPL");
