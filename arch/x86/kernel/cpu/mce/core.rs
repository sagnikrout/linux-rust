//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mce/core.c
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
// Machine check handler.
//
// K8 parts Copyright 2002,2003 Andi Kleen, SuSE Labs.
// Rest from unknown author(s).
// 2004 Andi Kleen. Rewrote most of it.
// Copyright 2008 Intel Corporation
// Author: Andi Kleen
//

// sysfs synchronization
    static DEFINE_MUTEX(mce_sysfs_mutex);
// Macro flag: #define CREATE_TRACE_POINTS

    DEFINE_PER_CPU_READ_MOSTLY(unsigned int, mce_num_banks);
    DEFINE_PER_CPU_READ_MOSTLY(struct mce_bank[MAX_NR_BANKS], mce_banks_array);
pub const ATTR_LEN: c_int = 16;
// One object for each MCE bank, shared by all CPUs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_bank_dev {
    pub /: *mut *mut device_attribute attr; / device attribute,
    pub /: *mut *mut char attrname[ATTR_LEN]; / attribute name,
    pub /: *mut *mut u8 bank; / bank number,
}

    static struct mce_bank_dev mce_bank_devs[MAX_NR_BANKS];
    struct mce_vendor_flags mce_flags __read_mostly;
    struct mca_config mca_cfg __read_mostly = {
    .bootlog  = -1,
    .monarch_timeout = -1
    };
    static DEFINE_PER_CPU(struct mce_hw_err, hw_errs_seen);
//
// MCA banks polled by the period polling timer for corrected events.
// With Intel CMCI, this only has MCA banks which do not support CMCI (if any).
//
    DEFINE_PER_CPU(mce_banks_t, mce_poll_banks) = {
    [0 ... BITS_TO_LONGS(MAX_NR_BANKS)-1] = ~0UL
    };
//
// MCA banks controlled through firmware first for corrected errors.
// This is a global list of banks for which we won't enable CMCI and we
// won't poll. Firmware controls these banks and is responsible for
// reporting corrected errors through GHES. Uncorrected/recoverable
// errors are still notified through a machine check.
//
    mce_banks_t mce_banks_ce_disabled;
    static struct work_struct mce_work;
    static struct irq_work mce_irq_work;
//
// CPU/chipset specific EDAC code can register a notifier call here to print
// MCE errors in a human-readable form.
//
    BLOCKING_NOTIFIER_HEAD(x86_mce_decoder_chain);
#[no_mangle]
pub unsafe extern "C" fn mce_prep_record_common(m: *mut mce) {
    void mce_prep_record_common(struct mce *m)
    {
    m.cpuid	= cpuid_eax(1);
    m.cpuvendor	= boot_cpu_data.x86_vendor;
    m.mcgcap	= native_rdmsrq(MSR_IA32_MCG_CAP);
// need the internal __ version to avoid deadlocks
    m.time		= __ktime_get_real_seconds();
    }
#[no_mangle]
pub unsafe extern "C" fn mce_prep_record_per_cpu(cpu: c_uint, m: *mut mce) {
    void mce_prep_record_per_cpu(unsigned int cpu, struct mce *m)
    {
    m.cpu		= cpu;
    m.extcpu	= cpu;
    m.apicid	= cpu_data(cpu).topo.initial_apicid;
    m.microcode	= cpu_data(cpu).microcode;
    m.ppin		= topology_ppin(cpu);
    m.socketid	= topology_physical_package_id(cpu);
    }
// Do initial initialization of struct mce_hw_err
#[no_mangle]
pub unsafe extern "C" fn mce_prep_record(err: *mut mce_hw_err) {
    void mce_prep_record(struct mce_hw_err *err)
    {
    struct mce *m = &err.m;
    memset(err, 0, sizeof(struct mce_hw_err));
    mce_prep_record_common(m);
    mce_prep_record_per_cpu(smp_processor_id(), m);
    }
    DEFINE_PER_CPU(struct mce, injectm);
    EXPORT_PER_CPU_SYMBOL_GPL(injectm);
#[no_mangle]
pub unsafe extern "C" fn mce_log(err: *mut mce_hw_err) {
    void mce_log(struct mce_hw_err *err)
    {
    if (mce_gen_pool_add(err)) {
    pr_info(HW_ERR "Machine check events logged\n");
    irq_work_queue(&mce_irq_work);
    }
    }
    EXPORT_SYMBOL_GPL(mce_log);
#[no_mangle]
pub unsafe extern "C" fn mce_register_decode_chain(nb: *mut notifier_block) {
    void mce_register_decode_chain(struct notifier_block *nb)
    {
    if (WARN_ON(nb.priority < MCE_PRIO_LOWEST ||
    nb.priority > MCE_PRIO_HIGHEST))
    return;
    blocking_notifier_chain_register(&x86_mce_decoder_chain, nb);
    }
    EXPORT_SYMBOL_GPL(mce_register_decode_chain);
#[no_mangle]
pub unsafe extern "C" fn mce_unregister_decode_chain(nb: *mut notifier_block) {
    void mce_unregister_decode_chain(struct notifier_block *nb)
    {
    blocking_notifier_chain_unregister(&x86_mce_decoder_chain, nb);
    }
    EXPORT_SYMBOL_GPL(mce_unregister_decode_chain);
#[no_mangle]
unsafe extern "C" fn __print_mce(err: *mut mce_hw_err) {
    static void __print_mce(struct mce_hw_err *err)
    {
    struct mce *m = &err.m;
    pr_emerg(HW_ERR "CPU %d: Machine Check%s: %Lx Bank %d: %016Lx\n",
    m.extcpu,
    (m.mcgstatus & MCG_STATUS_MCIP ? " Exception" : ""),
    m.mcgstatus, m.bank, m.status);
    if (m.ip) {
    pr_emerg(HW_ERR "RIP%s %02x:<%016Lx> ",
    !(m.mcgstatus & MCG_STATUS_EIPV) ? " !INEXACT!" : "",
    m.cs, m.ip);
    if (m.cs == __KERNEL_CS)
    pr_cont("{%pS}", (void *)(unsigned long)m.ip);
    pr_cont("\n");
    }
    pr_emerg(HW_ERR "TSC %llx ", m.tsc);
    if (m.addr)
    pr_cont("ADDR %llx ", m.addr);
    if (m.misc)
    pr_cont("MISC %llx ", m.misc);
    if (m.ppin)
    pr_cont("PPIN %llx ", m.ppin);
    if (mce_flags.smca) {
    if (m.synd)
    pr_cont("SYND %llx ", m.synd);
    if (err.vendor.amd.synd1)
    pr_cont("SYND1 %llx ", err.vendor.amd.synd1);
    if (err.vendor.amd.synd2)
    pr_cont("SYND2 %llx ", err.vendor.amd.synd2);
    if (m.ipid)
    pr_cont("IPID %llx ", m.ipid);
    }
    pr_cont("\n");
//
// Note this output is parsed by external tools and old fields
// should not be changed.
//
    pr_emerg(HW_ERR "PROCESSOR %u:%x TIME %llu SOCKET %u APIC %x microcode %x\n",
    m.cpuvendor, m.cpuid, m.time, m.socketid, m.apicid,
    m.microcode);
    }
#[no_mangle]
unsafe extern "C" fn print_mce(err: *mut mce_hw_err) {
    static void print_mce(struct mce_hw_err *err)
    {
    struct mce *m = &err.m;
    __print_mce(err);
    if (m.cpuvendor != X86_VENDOR_AMD && m.cpuvendor != X86_VENDOR_HYGON)
    pr_emerg_ratelimited(HW_ERR "Run the above through 'mcelog --ascii'\n");
    }

    static atomic_t mce_panicked;
    static int fake_panic;
    static atomic_t mce_fake_panicked;
// Panic in progress. Enable interrupts and wait for final IPI
#[no_mangle]
unsafe extern "C" fn wait_for_panic() {
    static void wait_for_panic(void)
    {
    let mut timeout: c_long = PANIC_TIMEOUT*USEC_PER_SEC;
    preempt_disable();
    local_irq_enable();
    while (timeout-- > 0)
    udelay(1);
    if (panic_timeout == 0)
    panic_timeout = mca_cfg.panic_timeout;
    panic("Panicing machine check CPU died");
    }
    static const char *mce_dump_aux_info(struct mce *m)
    {
    if (boot_cpu_has_bug(X86_BUG_TDX_PW_MCE))
    return tdx_dump_mce_info(m);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mce_panic(msg: *const c_char, final: *mut mce_hw_err, exp: *mut c_char) -> noinstr void {
    static noinstr void mce_panic(const char *msg, struct mce_hw_err *final, char *exp)
    {
    struct llist_node *pending;
    struct mce_evt_llist *l;
    let mut apei_err: c_int = 0;
    const char *memmsg;
//
// Allow instrumentation around external facilities usage. Not that it
// matters a whole lot since the machine is going to panic anyway.
//
    instrumentation_begin();
    if (!fake_panic) {
//
// Make sure only one CPU runs in machine check panic
//
    if (atomic_inc_return(&mce_panicked) > 1)
    wait_for_panic();
    barrier();
    bust_spinlocks(1);
    console_verbose();
    } else {
// Don't log too much for fake panic
    if (atomic_inc_return(&mce_fake_panicked) > 1)
    goto out;
    }
    pending = mce_gen_pool_prepare_records();
// First print corrected ones that are still unlogged
    llist_for_each_entry(l, pending, llnode) {
    struct mce_hw_err *err = &l.err;
    struct mce *m = &err.m;
    if (!(m.status & MCI_STATUS_UC)) {
    print_mce(err);
    if (!apei_err)
    apei_err = apei_write_mce(m);
    }
    }
// Now print uncorrected but with the final one last
    llist_for_each_entry(l, pending, llnode) {
    struct mce_hw_err *err = &l.err;
    struct mce *m = &err.m;
    if (!(m.status & MCI_STATUS_UC))
    continue;
    if (!final || mce_cmp(m, &final.m)) {
    print_mce(err);
    if (!apei_err)
    apei_err = apei_write_mce(m);
    }
    }
    if (final) {
    print_mce(final);
    if (!apei_err)
    apei_err = apei_write_mce(&final.m);
    }
    if (exp)
    pr_emerg(HW_ERR "Machine check: %s\n", exp);
    memmsg = mce_dump_aux_info(&final.m);
    if (memmsg)
    pr_emerg(HW_ERR "Machine check: %s\n", memmsg);
    if (!fake_panic) {
    if (panic_timeout == 0)
    panic_timeout = mca_cfg.panic_timeout;
//
// Kdump skips the poisoned page in order to avoid
// touching the error bits again. Poison the page even
// if the error is fatal and the machine is about to
// panic.
//
    if (kexec_crash_loaded()) {
    if (final && (final.m.status & MCI_STATUS_ADDRV)) {
    struct page *p;
    p = pfn_to_online_page(final.m.addr >> PAGE_SHIFT);
    if (p)
    SetPageHWPoison(p);
    }
    }
    panic(msg);
    } else
    pr_emerg(HW_ERR "Fake kernel panic: %s\n", msg);
    out:
    instrumentation_end();
    }
// Support code for software error injection
#[no_mangle]
unsafe extern "C" fn msr_to_offset(msr: u32) -> c_int {
    static int msr_to_offset(u32 msr)
    {
    let mut bank: unsigned = __this_cpu_read(injectm.bank);
    if (msr == mca_cfg.rip_msr)
    return offsetof(struct mce, ip);
    if (msr == mca_msr_reg(bank, MCA_STATUS))
    return offsetof(struct mce, status);
    if (msr == mca_msr_reg(bank, MCA_ADDR))
    return offsetof(struct mce, addr);
    if (msr == mca_msr_reg(bank, MCA_MISC))
    return offsetof(struct mce, misc);
    if (msr == MSR_IA32_MCG_STATUS)
    return offsetof(struct mce, mcgstatus);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn ex_handler_msr_mce(regs: *mut pt_regs, wrmsr: bool) {
    void ex_handler_msr_mce(struct pt_regs *regs, bool wrmsr)
    {
    if (wrmsr) {
    pr_emerg("MSR access error: WRMSR to 0x%x (tried to write 0x%08x%08x) at rIP: 0x%lx (%pS)\n",
    (unsigned int)regs.cx, (unsigned int)regs.dx, (unsigned int)regs.ax,
    regs.ip, (void *)regs.ip);
    } else {
    pr_emerg("MSR access error: RDMSR from 0x%x at rIP: 0x%lx (%pS)\n",
    (unsigned int)regs.cx, regs.ip, (void *)regs.ip);
    }
    show_stack_regs(regs);
    panic("MCA architectural violation!\n");
    while (true)
    cpu_relax();
    }
// MSR access wrappers used for error injection
#[no_mangle]
pub unsafe extern "C" fn mce_rdmsrq(msr: u32) -> noinstr u64 {
    noinstr u64 mce_rdmsrq(u32 msr)
    {
    EAX_EDX_DECLARE_ARGS(val, low, high);
    if (__this_cpu_read(injectm.finished)) {
    int offset;
    u64 ret;
    instrumentation_begin();
    offset = msr_to_offset(msr);
    if (offset < 0)
    ret = 0;
    else
    ret = *(u64 *)((char *)this_cpu_ptr(&injectm) + offset);
    instrumentation_end();
    return ret;
    }
//
// RDMSR on MCA MSRs should not fault. If they do, this is very much an
// architectural violation and needs to be reported to hw vendor. Panic
// the box to not allow any further progress.
//
    asm volatile("1: rdmsr\n"
    "2:\n"
    _ASM_EXTABLE_TYPE(1b, 2b, EX_TYPE_RDMSR_IN_MCE)
    : EAX_EDX_RET(val, low, high) : "c" (msr));
    return EAX_EDX_VAL(val, low, high);
    }
#[no_mangle]
pub unsafe extern "C" fn mce_wrmsrq(msr: u32, v: u64) -> noinstr void {
    noinstr void mce_wrmsrq(u32 msr, u64 v)
    {
    u32 low, high;
    if (__this_cpu_read(injectm.finished)) {
    int offset;
    instrumentation_begin();
    offset = msr_to_offset(msr);
    if (offset >= 0)
// (u64 *)((char *)this_cpu_ptr(&injectm) + offset) = v;
    instrumentation_end();
    return;
    }
    low  = (u32)v;
    high = (u32)(v >> 32);
// See comment in mce_rdmsrq()
    asm volatile("1: wrmsr\n"
    "2:\n"
    _ASM_EXTABLE_TYPE(1b, 2b, EX_TYPE_WRMSR_IN_MCE)
    : : "c" (msr), "a"(low), "d" (high) : "memory");
    }
//
// Collect all global (w.r.t. this processor) status about this machine
// check into our "mce" struct so that we can use it later to assess
// the severity of the problem as we read per-bank specific details.
//
#[no_mangle]
unsafe extern "C" fn mce_gather_info(err: *mut mce_hw_err, regs: *mut pt_regs) -> noinstr void {
    static noinstr void mce_gather_info(struct mce_hw_err *err, struct pt_regs *regs)
    {
    struct mce *m;
//
// Enable instrumentation around mce_prep_record() which calls external
// facilities.
//
    instrumentation_begin();
    mce_prep_record(err);
    instrumentation_end();
    m = &err.m;
    m.mcgstatus = mce_rdmsrq(MSR_IA32_MCG_STATUS);
    if (regs) {
//
// Get the address of the instruction at the time of
// the machine check error.
//
    if (m.mcgstatus & (MCG_STATUS_RIPV|MCG_STATUS_EIPV)) {
    m.ip = regs.ip;
    m.cs = regs.cs;
//
// When in VM86 mode make the cs look like ring 3
// always. This is a lie, but it's better than passing
// the additional vm86 bit around everywhere.
//
    if (v8086_mode(regs))
    m.cs |= 3;
    }
// Use accurate RIP reporting if available.
    if (mca_cfg.rip_msr)
    m.ip = mce_rdmsrq(mca_cfg.rip_msr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mce_available(c: *mut cpuinfo_x86) -> bool {
    bool mce_available(struct cpuinfo_x86 *c)
    {
    if (mca_cfg.disabled)
    return false;
    return cpu_has(c, X86_FEATURE_MCE) && cpu_has(c, X86_FEATURE_MCA);
    }
#[no_mangle]
unsafe extern "C" fn mce_schedule_work() {
    static void mce_schedule_work(void)
    {
    if (!mce_gen_pool_empty())
    schedule_work(&mce_work);
    }
#[no_mangle]
unsafe extern "C" fn mce_irq_work_cb(entry: *mut irq_work) {
    static void mce_irq_work_cb(struct irq_work *entry)
    {
    mce_schedule_work();
    }
#[no_mangle]
pub unsafe extern "C" fn mce_usable_address(m: *mut mce) -> bool {
    bool mce_usable_address(struct mce *m)
    {
    if (!(m.status & MCI_STATUS_ADDRV))
    return false;
    switch (m.cpuvendor) {
    case X86_VENDOR_AMD:
    return amd_mce_usable_address(m);
    case X86_VENDOR_INTEL:
    case X86_VENDOR_ZHAOXIN:
    return intel_mce_usable_address(m);
    default:
    return true;
    }
    }
    EXPORT_SYMBOL_GPL(mce_usable_address);
#[no_mangle]
pub unsafe extern "C" fn mce_is_memory_error(m: *mut mce) -> bool {
    bool mce_is_memory_error(struct mce *m)
    {
    switch (m.cpuvendor) {
    case X86_VENDOR_AMD:
    case X86_VENDOR_HYGON:
    return amd_mce_is_memory_error(m);
    case X86_VENDOR_INTEL:
    case X86_VENDOR_ZHAOXIN:
//
// Intel SDM Volume 3B - 15.9.2 Compound Error Codes
//
// Bit 7 of the MCACOD field of IA32_MCi_STATUS is used for
// indicating a memory error. Bit 8 is used for indicating a
// cache hierarchy error. The combination of bit 2 and bit 3
// is used for indicating a `generic' cache hierarchy error
// But we can't just blindly check the above bits, because if
// bit 11 is set, then it is a bus/interconnect error - and
// either way the above bits just gives more detail on what
// bus/interconnect error happened. Note that bit 12 can be
// ignored, as it's the "filter" bit.
//
    return (m.status & 0xef80) == BIT(7) ||
    (m.status & 0xef00) == BIT(8) ||
    (m.status & 0xeffc) == 0xc;
    default:
    return false;
    }
    }
    EXPORT_SYMBOL_GPL(mce_is_memory_error);
#[no_mangle]
unsafe extern "C" fn whole_page(m: *mut mce) -> bool {
    static bool whole_page(struct mce *m)
    {
    if (!mca_cfg.ser || !(m.status & MCI_STATUS_MISCV))
    return true;
    return MCI_MISC_ADDR_LSB(m.misc) >= PAGE_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn mce_is_correctable(m: *mut mce) -> bool {
    bool mce_is_correctable(struct mce *m)
    {
    if (m.cpuvendor == X86_VENDOR_AMD && m.status & MCI_STATUS_DEFERRED)
    return false;
    if (m.cpuvendor == X86_VENDOR_HYGON && m.status & MCI_STATUS_DEFERRED)
    return false;
    if (m.status & MCI_STATUS_UC)
    return false;
    return true;
    }
    EXPORT_SYMBOL_GPL(mce_is_correctable);
    static int mce_early_notifier(struct notifier_block *nb, unsigned long val,
    void *data)
    {
    struct mce_hw_err *err = to_mce_hw_err(data);
    if (!err)
    return NOTIFY_DONE;
// Emit the trace record:
    trace_mce_record(err);
    mce_work_trigger();
    return NOTIFY_DONE;
    }
    static struct notifier_block early_nb = {
    .notifier_call	= mce_early_notifier,
    .priority	= MCE_PRIO_EARLY,
    };
    static int uc_decode_notifier(struct notifier_block *nb, unsigned long val,
    void *data)
    {
    struct mce *mce = (struct mce *)data;
    unsigned long pfn;
    if (!mce || !mce_usable_address(mce))
    return NOTIFY_DONE;
    if (mce.severity != MCE_AO_SEVERITY &&
    mce.severity != MCE_DEFERRED_SEVERITY)
    return NOTIFY_DONE;
    pfn = (mce.addr & MCI_ADDR_PHYSADDR) >> PAGE_SHIFT;
    if (!memory_failure(pfn, 0)) {
    set_mce_nospec(pfn);
    mce.kflags |= MCE_HANDLED_UC;
    }
    return NOTIFY_OK;
    }
    static struct notifier_block mce_uc_nb = {
    .notifier_call	= uc_decode_notifier,
    .priority	= MCE_PRIO_UC,
    };
    static int mce_default_notifier(struct notifier_block *nb, unsigned long val,
    void *data)
    {
    struct mce_hw_err *err = to_mce_hw_err(data);
    if (!err)
    return NOTIFY_DONE;
    if (mca_cfg.print_all || !(err.m.kflags))
    __print_mce(err);
    return NOTIFY_DONE;
    }
    static struct notifier_block mce_default_nb = {
    .notifier_call	= mce_default_notifier,
// lowest prio, we want it to run last.
    .priority	= MCE_PRIO_LOWEST,
    };
//
// Read ADDR and MISC registers.
//
#[no_mangle]
unsafe extern "C" fn mce_read_aux(err: *mut mce_hw_err, i: c_int) -> noinstr void {
    static noinstr void mce_read_aux(struct mce_hw_err *err, int i)
    {
    struct mce *m = &err.m;
    if (m.status & MCI_STATUS_MISCV)
    m.misc = mce_rdmsrq(mca_msr_reg(i, MCA_MISC));
    if (m.status & MCI_STATUS_ADDRV) {
    if (m.kflags & MCE_CHECK_DFR_REGS)
    m.addr = mce_rdmsrq(MSR_AMD64_SMCA_MCx_DEADDR(i));
    else
    m.addr = mce_rdmsrq(mca_msr_reg(i, MCA_ADDR));
//
// Mask the reported address by the reported granularity.
//
    if (mca_cfg.ser && (m.status & MCI_STATUS_MISCV)) {
    let mut shift: u8 = MCI_MISC_ADDR_LSB(m.misc);
    m.addr >>= shift;
    m.addr <<= shift;
    }
    smca_extract_err_addr(m);
    }
    if (mce_flags.smca) {
    m.ipid = mce_rdmsrq(MSR_AMD64_SMCA_MCx_IPID(i));
    if (m.status & MCI_STATUS_SYNDV) {
    m.synd = mce_rdmsrq(MSR_AMD64_SMCA_MCx_SYND(i));
    err.vendor.amd.synd1 = mce_rdmsrq(MSR_AMD64_SMCA_MCx_SYND1(i));
    err.vendor.amd.synd2 = mce_rdmsrq(MSR_AMD64_SMCA_MCx_SYND2(i));
    }
    }
    }
//
// We have three scenarios for checking for Deferred errors:
//
// 1) Non-SMCA systems check MCA_STATUS and log error if found.
// 2) SMCA systems check MCA_STATUS. If error is found then log it and also
// clear MCA_DESTAT.
// 3) SMCA systems check MCA_DESTAT, if error was not found in MCA_STATUS, and
// log it.
//
#[no_mangle]
unsafe extern "C" fn smca_should_log_poll_error(m: *mut mce) -> bool {
    static bool smca_should_log_poll_error(struct mce *m)
    {
    if (m.status & MCI_STATUS_VAL)
    return true;
    m.status = mce_rdmsrq(MSR_AMD64_SMCA_MCx_DESTAT(m.bank));
    if ((m.status & MCI_STATUS_VAL) && (m.status & MCI_STATUS_DEFERRED)) {
    m.kflags |= MCE_CHECK_DFR_REGS;
    return true;
    }
    return false;
    }
//
// Newer Intel systems that support software error
// recovery need to make additional checks. Other
// CPUs should skip over uncorrected errors, but log
// everything else.
//
#[no_mangle]
unsafe extern "C" fn ser_should_log_poll_error(m: *mut mce) -> bool {
    static bool ser_should_log_poll_error(struct mce *m)
    {
// Log "not enabled" (speculative) errors
    if (!(m.status & MCI_STATUS_EN))
    return true;
//
// Log UCNA (SDM: 15.6.3 "UCR Error Classification")
// UC == 1 && PCC == 0 && S == 0
//
    if (!(m.status & MCI_STATUS_PCC) && !(m.status & MCI_STATUS_S))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn should_log_poll_error(flags: enum mcp_flags, err: *mut mce_hw_err) -> bool {
    static bool should_log_poll_error(enum mcp_flags flags, struct mce_hw_err *err)
    {
    struct mce *m = &err.m;
    if (mce_flags.smca)
    return smca_should_log_poll_error(m);
// If this entry is not valid, ignore it.
    if (!(m.status & MCI_STATUS_VAL))
    return false;
//
// If we are logging everything (at CPU online) or this
// is a corrected error, then we must log it.
//
    if ((flags & MCP_UC) || !(m.status & MCI_STATUS_UC))
    return true;
    if (mca_cfg.ser)
    return ser_should_log_poll_error(m);
    if (m.status & MCI_STATUS_UC)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn clear_bank(m: *mut mce) {
    static void clear_bank(struct mce *m)
    {
    if (m.cpuvendor == X86_VENDOR_AMD)
    return amd_clear_bank(m);
    mce_wrmsrq(mca_msr_reg(m.bank, MCA_STATUS), 0);
    }
//
// Poll for corrected events or events that happened before reset.
// Those are just logged through /dev/mcelog.
//
// This is executed in standard interrupt context.
//
// Note: spec recommends to panic for fatal unsignalled
// errors here. However this would be quite problematic --
// we would need to reimplement the Monarch handling and
// it would mess up the exclusion between exception handler
// and poll handler -- * so we skip this for now.
// These cases should not happen anyways, or only when the CPU
// is already totally * confused. In this case it's likely it will
// not fully execute the machine check handler either.
//
#[no_mangle]
pub unsafe extern "C" fn machine_check_poll(flags: enum mcp_flags, b: *mut mce_banks_t) {
    void machine_check_poll(enum mcp_flags flags, mce_banks_t *b)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
    struct mce_hw_err err;
    struct mce *m;
    int i;
    inc_irq_stat(MCE_POLL);
    mce_gather_info(&err, core::ptr::null_mut());
    m = &err.m;
    if (flags & MCP_TIMESTAMP)
    m.tsc = rdtsc();
    for (i = 0; i < this_cpu_read(mce_num_banks); i++) {
    if (!mce_banks[i].ctl || !test_bit(i, *b))
    continue;
    m.misc = 0;
    m.addr = 0;
    m.bank = i;
    barrier();
    m.status = mce_rdmsrq(mca_msr_reg(i, MCA_STATUS));
//
// Update storm tracking here, before checking for the
// MCI_STATUS_VAL bit. Valid corrected errors count
// towards declaring, or maintaining, storm status. No
// error in a bank counts towards avoiding, or ending,
// storm status.
//
    if (!mca_cfg.cmci_disabled)
    mce_track_storm(m);
// Verify that the error should be logged based on hardware conditions.
    if (!should_log_poll_error(flags, &err))
    continue;
    mce_read_aux(&err, i);
    m.severity = mce_severity(m, core::ptr::null_mut(), core::ptr::null_mut(), false);
//
// Don't get the IP here because it's unlikely to
// have anything to do with the actual error location.
//
    if (mca_cfg.dont_log_ce && !mce_usable_address(m))
    goto clear_it;
    if (flags & MCP_QUEUE_LOG)
    mce_gen_pool_add(&err);
    else
    mce_log(&err);
    clear_it:
    clear_bank(m);
    }
//
// Don't clear MCG_STATUS here because it's only defined for
// exceptions.
//
    sync_core();
    }
    EXPORT_SYMBOL_GPL(machine_check_poll);
//
// During IFU recovery Sandy Bridge -EP4S processors set the RIPV and
// EIPV bits in MCG_STATUS to zero on the affected logical processor (SDM
// Vol 3B Table 15-20). But this confuses both the code that determines
// whether the machine check occurred in kernel or user mode, and also
// the severity assessment code. Pretend that EIPV was set, and take the
// ip/cs values from the pt_regs that mce_gather_info() ignored earlier.
//
    static __always_inline void
    quirk_sandybridge_ifu(int bank, struct mce *m, struct pt_regs *regs)
    {
    if (bank != 0)
    return;
    if ((m.mcgstatus & (MCG_STATUS_EIPV|MCG_STATUS_RIPV)) != 0)
    return;
    if ((m.status & (MCI_STATUS_OVER|MCI_STATUS_UC|
    MCI_STATUS_EN|MCI_STATUS_MISCV|MCI_STATUS_ADDRV|
    MCI_STATUS_PCC|MCI_STATUS_S|MCI_STATUS_AR|
    MCACOD)) !=
    (MCI_STATUS_UC|MCI_STATUS_EN|
    MCI_STATUS_MISCV|MCI_STATUS_ADDRV|MCI_STATUS_S|
    MCI_STATUS_AR|MCACOD_INSTR))
    return;
    m.mcgstatus |= MCG_STATUS_EIPV;
    m.ip = regs.ip;
    m.cs = regs.cs;
    }
//
// Disable fast string copy and return from the MCE handler upon the first SRAR
// MCE on bank 1 due to a CPU erratum on Intel Skylake/Cascade Lake/Cooper Lake
// CPUs.
// The fast string copy instructions ("REP; MOVS*") could consume an
// uncorrectable memory error in the cache line _right after_ the desired region
// to copy and raise an MCE with RIP pointing to the instruction _after_ the
// "REP; MOVS*".
// This mitigation addresses the issue completely with the caveat of performance
// degradation on the CPU affected. This is still better than the OS crashing on
// MCEs raised on an irrelevant process due to "REP; MOVS*" accesses from a
// kernel context (e.g., copy_page).
//
// Returns true when fast string copy on CPU has been disabled.
//
#[no_mangle]
unsafe extern "C" fn quirk_skylake_repmov() -> noinstr bool {
    static noinstr bool quirk_skylake_repmov(void)
    {
    let mut mcgstatus: u64 = mce_rdmsrq(MSR_IA32_MCG_STATUS);
    let mut misc_enable: u64 = mce_rdmsrq(MSR_IA32_MISC_ENABLE);
    u64 mc1_status;
//
// Apply the quirk only to local machine checks, i.e., no broadcast
// sync is needed.
//
    if (!(mcgstatus & MCG_STATUS_LMCES) ||
    !(misc_enable & MSR_IA32_MISC_ENABLE_FAST_STRING))
    return false;
    mc1_status = mce_rdmsrq(MSR_IA32_MCx_STATUS(1));
// Check for a software-recoverable data fetch error.
    if ((mc1_status &
    (MCI_STATUS_VAL | MCI_STATUS_OVER | MCI_STATUS_UC | MCI_STATUS_EN |
    MCI_STATUS_ADDRV | MCI_STATUS_MISCV | MCI_STATUS_PCC |
    MCI_STATUS_AR | MCI_STATUS_S)) ==
    (MCI_STATUS_VAL |                   MCI_STATUS_UC | MCI_STATUS_EN |
    MCI_STATUS_ADDRV | MCI_STATUS_MISCV |
    MCI_STATUS_AR | MCI_STATUS_S)) {
    misc_enable &= ~MSR_IA32_MISC_ENABLE_FAST_STRING;
    mce_wrmsrq(MSR_IA32_MISC_ENABLE, misc_enable);
    mce_wrmsrq(MSR_IA32_MCx_STATUS(1), 0);
    instrumentation_begin();
    pr_err_once("Erratum detected, disable fast string copy instructions.\n");
    instrumentation_end();
    return true;
    }
    return false;
    }
//
// Some Zen-based Instruction Fetch Units set EIPV=RIPV=0 on poison consumption
// errors. This means mce_gather_info() will not save the "ip" and "cs" registers.
//
// However, the context is still valid, so save the "cs" register for later use.
//
// The "ip" register is truly unknown, so don't save it or fixup EIPV/RIPV.
//
// The Instruction Fetch Unit is at MCA bank 1 for all affected systems.
//
#[no_mangle]
unsafe extern "C" fn quirk_zen_ifu(bank: c_int, m: *mut mce, regs: *mut pt_regs) -> __always_inline void {
    static __always_inline void quirk_zen_ifu(int bank, struct mce *m, struct pt_regs *regs)
    {
    if (bank != 1)
    return;
    if (!(m.status & MCI_STATUS_POISON))
    return;
    m.cs = regs.cs;
    }
//
// Do a quick check if any of the events requires a panic.
// This decides if we keep the events around or clear them.
//
    static __always_inline int mce_no_way_out(struct mce_hw_err *err, char **msg, unsigned long *validp,
    struct pt_regs *regs)
    {
    struct mce *m = &err.m;
    char *tmp = *msg;
    int i;
    for (i = 0; i < this_cpu_read(mce_num_banks); i++) {
    m.status = mce_rdmsrq(mca_msr_reg(i, MCA_STATUS));
    if (!(m.status & MCI_STATUS_VAL))
    continue;
    arch___set_bit(i, validp);
    if (mce_flags.snb_ifu_quirk)
    quirk_sandybridge_ifu(i, m, regs);
    if (mce_flags.zen_ifu_quirk)
    quirk_zen_ifu(i, m, regs);
    m.bank = i;
    if (mce_severity(m, regs, &tmp, true) >= MCE_PANIC_SEVERITY) {
    mce_read_aux(err, i);
// msg = tmp;
    return 1;
    }
    }
    return 0;
    }
//
// Variable to establish order between CPUs while scanning.
// Each CPU spins initially until executing is equal its number.
//
    static atomic_t mce_executing;
//
// Defines order of CPUs on entry. First CPU becomes Monarch.
//
    static atomic_t mce_callin;
//
// Track which CPUs entered the MCA broadcast synchronization and which not in
// order to print holdouts.
//
    let mut mce_missing_cpus: static cpumask_t = CPU_MASK_ALL;
//
// Check if a timeout waiting for other CPUs happened.
//
#[no_mangle]
unsafe extern "C" fn mce_timed_out(t: *mut u64, msg: *const c_char) -> noinstr int {
    static noinstr int mce_timed_out(u64 *t, const char *msg)
    {
    let mut ret: c_int = 0;
// Enable instrumentation around calls to external facilities
    instrumentation_begin();
//
// The others already did panic for some reason.
// Bail out like in a timeout.
// rmb() to tell the compiler that system_state
// might have been modified by someone else.
//
    rmb();
    if (atomic_read(&mce_panicked))
    wait_for_panic();
    if (!mca_cfg.monarch_timeout)
    goto out;
    if ((s64)*t < SPINUNIT) {
    if (cpumask_and(&mce_missing_cpus, cpu_online_mask, &mce_missing_cpus))
    pr_emerg("CPUs not responding to MCE broadcast (may include false positives): %*pbl\n",
    cpumask_pr_args(&mce_missing_cpus));
    mce_panic(msg, core::ptr::null_mut(), core::ptr::null_mut());
    ret = 1;
    goto out;
    }
// t -= SPINUNIT;
    out:
    touch_nmi_watchdog();
    instrumentation_end();
    return ret;
    }
//
// The Monarch's reign.  The Monarch is the CPU who entered
// the machine check handler first. It waits for the others to
// raise the exception too and then grades them. When any
// error is fatal panic. Only then let the others continue.
//
// The other CPUs entering the MCE handler will be controlled by the
// Monarch. They are called Subjects.
//
// This way we prevent any potential data corruption in a unrecoverable case
// and also makes sure always all CPU's errors are examined.
//
// Also this detects the case of a machine check event coming from outer
// space (not detected by any CPUs) In this case some external agent wants
// us to shut down, so panic too.
//
// The other CPUs might still decide to panic if the handler happens
// in a unrecoverable place, but in this case the system is in a semi-stable
// state and won't corrupt anything by itself. It's ok to let the others
// continue for a bit first.
//
// All the spin loops have timeouts; when a timeout happens a CPU
// typically elects itself to be Monarch.
//
#[no_mangle]
unsafe extern "C" fn mce_reign() {
    static void mce_reign(void)
    {
    struct mce_hw_err *err = core::ptr::null_mut();
    struct mce *m = core::ptr::null_mut();
    let mut global_worst: c_int = 0;
    char *msg = core::ptr::null_mut();
    int cpu;
//
// This CPU is the Monarch and the other CPUs have run
// through their handlers.
// Grade the severity of the errors of all the CPUs.
//
    for_each_possible_cpu(cpu) {
    struct mce_hw_err *etmp = &per_cpu(hw_errs_seen, cpu);
    struct mce *mtmp = &etmp.m;
    if (mtmp.severity > global_worst) {
    global_worst = mtmp.severity;
    err = &per_cpu(hw_errs_seen, cpu);
    m = &err.m;
    }
    }
//
// Cannot recover? Panic here then.
// This dumps all the mces in the log buffer and stops the
// other CPUs.
//
    if (m && global_worst >= MCE_PANIC_SEVERITY) {
// call mce_severity() to get "msg" for panic
    mce_severity(m, core::ptr::null_mut(), &msg, true);
    mce_panic("Fatal machine check", err, msg);
    }
//
// For UC somewhere we let the CPU who detects it handle it.
// Also must let continue the others, otherwise the handling
// CPU could deadlock on a lock.
//
// No machine check event found. Must be some external
// source or one CPU is hung. Panic.
//
    if (global_worst <= MCE_KEEP_SEVERITY)
    mce_panic("Fatal machine check from unknown source", core::ptr::null_mut(), core::ptr::null_mut());
//
// Now clear all the hw_errs_seen so that they don't reappear on
// the next mce.
//
    for_each_possible_cpu(cpu)
    memset(&per_cpu(hw_errs_seen, cpu), 0, sizeof(struct mce_hw_err));
    }
    static atomic_t global_nwo;
//
// Start of Monarch synchronization. This waits until all CPUs have
// entered the exception handler and then determines if any of them
// saw a fatal event that requires panic. Then it executes them
// in the entry order.
// TBD double check parallel CPU hotunplug
//
#[no_mangle]
unsafe extern "C" fn mce_start(no_way_out: *mut c_int) -> noinstr int {
    static noinstr int mce_start(int *no_way_out)
    {
    let mut timeout: u64 = (u64)mca_cfg.monarch_timeout * NSEC_PER_USEC;
    int order, ret = -1;
    if (!timeout)
    return ret;
    raw_atomic_add(*no_way_out, &global_nwo);
//
// Rely on the implied barrier below, such that global_nwo
// is updated before mce_callin.
//
    order = raw_atomic_inc_return(&mce_callin);
    arch_cpumask_clear_cpu(smp_processor_id(), &mce_missing_cpus);
// Enable instrumentation around calls to external facilities
    instrumentation_begin();
//
// Wait for everyone.
//
    while (raw_atomic_read(&mce_callin) != num_online_cpus()) {
    if (mce_timed_out(&timeout,
    "Timeout: Not all CPUs entered broadcast exception handler")) {
    raw_atomic_set(&global_nwo, 0);
    goto out;
    }
    ndelay(SPINUNIT);
    }
//
// mce_callin should be read before global_nwo
//
    smp_rmb();
    if (order == 1) {
//
// Monarch: Starts executing now, the others wait.
//
    raw_atomic_set(&mce_executing, 1);
    } else {
//
// Subject: Now start the scanning loop one by one in
// the original callin order.
// This way when there are any shared banks it will be
// only seen by one CPU before cleared, avoiding duplicates.
//
    while (raw_atomic_read(&mce_executing) < order) {
    if (mce_timed_out(&timeout,
    "Timeout: Subject CPUs unable to finish machine check processing")) {
    raw_atomic_set(&global_nwo, 0);
    goto out;
    }
    ndelay(SPINUNIT);
    }
    }
//
// Cache the global no_way_out state.
//
// no_way_out = raw_atomic_read(&global_nwo);
    ret = order;
    out:
    instrumentation_end();
    return ret;
    }
//
// Synchronize between CPUs after main scanning loop.
// This invokes the bulk of the Monarch processing.
//
#[no_mangle]
unsafe extern "C" fn mce_end(order: c_int) -> noinstr int {
    static noinstr int mce_end(int order)
    {
    let mut timeout: u64 = (u64)mca_cfg.monarch_timeout * NSEC_PER_USEC;
    let mut ret: c_int = -1;
// Allow instrumentation around external facilities.
    instrumentation_begin();
    if (!timeout)
    goto reset;
    if (order < 0)
    goto reset;
//
// Allow others to run.
//
    atomic_inc(&mce_executing);
    if (order == 1) {
//
// Monarch: Wait for everyone to go through their scanning
// loops.
//
    while (atomic_read(&mce_executing) <= num_online_cpus()) {
    if (mce_timed_out(&timeout,
    "Timeout: Monarch CPU unable to finish machine check processing"))
    goto reset;
    ndelay(SPINUNIT);
    }
    mce_reign();
    barrier();
    ret = 0;
    } else {
//
// Subject: Wait for Monarch to finish.
//
    while (atomic_read(&mce_executing) != 0) {
    if (mce_timed_out(&timeout,
    "Timeout: Monarch CPU did not finish machine check processing"))
    goto reset;
    ndelay(SPINUNIT);
    }
//
// Don't reset anything. That's done by the Monarch.
//
    ret = 0;
    goto out;
    }
//
// Reset all global state.
//
    reset:
    atomic_set(&global_nwo, 0);
    atomic_set(&mce_callin, 0);
    cpumask_setall(&mce_missing_cpus);
    barrier();
//
// Let others run again.
//
    atomic_set(&mce_executing, 0);
    out:
    instrumentation_end();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mce_clear_state(toclear: *mut c_ulong) -> __always_inline void {
    static __always_inline void mce_clear_state(unsigned long *toclear)
    {
    int i;
    for (i = 0; i < this_cpu_read(mce_num_banks); i++) {
    if (arch_test_bit(i, toclear))
    mce_wrmsrq(mca_msr_reg(i, MCA_STATUS), 0);
    }
    }
//
// Cases where we avoid rendezvous handler timeout:
// 1) If this CPU is offline.
//
// 2) If crashing_cpu was set, e.g. we're entering kdump and we need to
// skip those CPUs which remain looping in the 1st kernel - see
// crash_nmi_callback().
//
// Note: there still is a small window between kexec-ing and the new,
// kdump kernel establishing a new #MC handler where a broadcasted MCE
// might not get handled properly.
//
#[no_mangle]
unsafe extern "C" fn mce_check_crashing_cpu() -> noinstr bool {
    static noinstr bool mce_check_crashing_cpu(void)
    {
    let mut cpu: c_uint = smp_processor_id();
    if (arch_cpu_is_offline(cpu) ||
    (crashing_cpu != -1 && crashing_cpu != cpu)) {
    u64 mcgstatus;
    mcgstatus = native_rdmsrq(MSR_IA32_MCG_STATUS);
    if (boot_cpu_data.x86_vendor == X86_VENDOR_ZHAOXIN) {
    if (mcgstatus & MCG_STATUS_LMCES)
    return false;
    }
    if (mcgstatus & MCG_STATUS_RIPV) {
    native_wrmsrq(MSR_IA32_MCG_STATUS, 0);
    return true;
    }
    }
    return false;
    }
    static __always_inline int
    __mc_scan_banks(struct mce_hw_err *err, struct pt_regs *regs,
    struct mce_hw_err *final, unsigned long *toclear,
    unsigned long *valid_banks, int no_way_out, int *worst)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
    struct mca_config *cfg = &mca_cfg;
    int severity, i, taint = 0;
    struct mce *m = &err.m;
    for (i = 0; i < this_cpu_read(mce_num_banks); i++) {
    arch___clear_bit(i, toclear);
    if (!arch_test_bit(i, valid_banks))
    continue;
    if (!mce_banks[i].ctl)
    continue;
    m.misc = 0;
    m.addr = 0;
    m.bank = i;
    m.status = mce_rdmsrq(mca_msr_reg(i, MCA_STATUS));
    if (!(m.status & MCI_STATUS_VAL))
    continue;
//
// Corrected or non-signaled errors are handled by
// machine_check_poll(). Leave them alone, unless this panics.
//
    if (!(m.status & (cfg.ser ? MCI_STATUS_S : MCI_STATUS_UC)) &&
    !no_way_out)
    continue;
// Set taint even when machine check was not enabled.
    taint++;
    severity = mce_severity(m, regs, core::ptr::null_mut(), true);
//
// When machine check was for corrected/deferred handler don't
// touch, unless we're panicking.
//
    if ((severity == MCE_KEEP_SEVERITY ||
    severity == MCE_UCNA_SEVERITY) && !no_way_out)
    continue;
    arch___set_bit(i, toclear);
// Machine check event was not enabled. Clear, but ignore.
    if (severity == MCE_NO_SEVERITY)
    continue;
    mce_read_aux(err, i);
// assuming valid severity level != 0
    m.severity = severity;
//
// Enable instrumentation around the mce_log() call which is
// done in #MC context, where instrumentation is disabled.
//
    instrumentation_begin();
    mce_log(err);
    instrumentation_end();
    if (severity > *worst) {
// final = *err;
// worst = severity;
    }
    }
// mce_clear_state will clear *final, save locally for use later
// err = *final;
    return taint;
    }
#[no_mangle]
unsafe extern "C" fn kill_me_now(ch: *mut callback_head) {
    static void kill_me_now(struct callback_head *ch)
    {
    struct task_struct *p = container_of(ch, struct task_struct, mce_kill_me);
    p.mce_count = 0;
    force_sig(SIGBUS);
    }
#[no_mangle]
unsafe extern "C" fn kill_me_maybe(cb: *mut callback_head) {
    static void kill_me_maybe(struct callback_head *cb)
    {
    struct task_struct *p = container_of(cb, struct task_struct, mce_kill_me);
    let mut flags: c_int = MF_ACTION_REQUIRED;
    unsigned long pfn;
    int ret;
    p.mce_count = 0;
    pr_err("Uncorrected hardware memory error in user-access at %llx", p.mce_addr);
    if (!p.mce_ripv)
    flags |= MF_MUST_KILL;
    pfn = (p.mce_addr & MCI_ADDR_PHYSADDR) >> PAGE_SHIFT;
    ret = memory_failure(pfn, flags);
    if (!ret) {
    set_mce_nospec(pfn);
    sync_core();
    return;
    }
//
// -EHWPOISON from memory_failure() means that it already sent SIGBUS
// to the current process with the proper error info,
// -EOPNOTSUPP means hwpoison_filter() filtered the error event,
//
// In both cases, no further processing is required.
//
    if (ret == -EHWPOISON || ret == -EOPNOTSUPP)
    return;
    pr_err("Memory error not recovered");
    kill_me_now(cb);
    }
#[no_mangle]
unsafe extern "C" fn kill_me_never(cb: *mut callback_head) {
    static void kill_me_never(struct callback_head *cb)
    {
    struct task_struct *p = container_of(cb, struct task_struct, mce_kill_me);
    unsigned long pfn;
    p.mce_count = 0;
    pr_err("Kernel accessed poison in user space at %llx\n", p.mce_addr);
    pfn = (p.mce_addr & MCI_ADDR_PHYSADDR) >> PAGE_SHIFT;
    if (!memory_failure(pfn, 0))
    set_mce_nospec(pfn);
    }
#[no_mangle]
unsafe extern "C" fn queue_task_work(err: *mut mce_hw_err, msg: *mut c_char, ): *mut *mut void (func)(struct callback_head) {
    static void queue_task_work(struct mce_hw_err *err, char *msg, void (*func)(struct callback_head *))
    {
    let mut count: c_int = ++current.mce_count;
    struct mce *m = &err.m;
// First call, save all the details
    if (count == 1) {
    current.mce_addr = m.addr;
    current.mce_kflags = m.kflags;
    current.mce_ripv = !!(m.mcgstatus & MCG_STATUS_RIPV);
    current.mce_whole_page = whole_page(m);
    current.mce_kill_me.func = func;
    }
// Ten is likely overkill. Don't expect more than two faults before task_work()
    if (count > 10)
    mce_panic("Too many consecutive machine checks while accessing user data",
    err, msg);
// Second or later call, make sure page address matches the one from first call
    if (count > 1 && (current.mce_addr >> PAGE_SHIFT) != (m.addr >> PAGE_SHIFT))
    mce_panic("Consecutive machine checks to different user pages", err, msg);
// Do not call task_work_add() more than once
    if (count > 1)
    return;
    task_work_add(current, &current.mce_kill_me, TWA_RESUME);
    }
// Handle unconfigured int18 (should never happen)
#[no_mangle]
unsafe extern "C" fn unexpected_machine_check(regs: *mut pt_regs) -> noinstr void {
    static noinstr void unexpected_machine_check(struct pt_regs *regs)
    {
    instrumentation_begin();
    pr_err("CPU#%d: Unexpected int18 (Machine Check)\n",
    smp_processor_id());
    instrumentation_end();
    }
//
// The actual machine check handler. This only handles real exceptions when
// something got corrupted coming in through int 18.
//
// This is executed in #MC context not subject to normal locking rules.
// This implies that most kernel services cannot be safely used. Don't even
// think about putting a printk in there!
//
// On Intel systems this is entered on all CPUs in parallel through
// MCE broadcast. However some CPUs might be broken beyond repair,
// so be always careful when synchronizing with others.
//
// Tracing and kprobes are disabled: if we interrupted a kernel context
// with IF=1, we need to minimize stack usage.  There are also recursion
// issues: if the machine check was due to a failure of the memory
// backing the user stack, tracing that reads the user stack will cause
// potentially infinite recursion.
//
// Currently, the #MC handler calls out to a number of external facilities
// and, therefore, allows instrumentation around them. The optimal thing to
// have would be to do the absolutely minimal work required in #MC context
// and have instrumentation disabled only around that. Further processing can
// then happen in process context where instrumentation is allowed. Achieving
// that requires careful auditing and modifications. Until then, the code
// allows instrumentation temporarily, where required.
//
#[no_mangle]
pub unsafe extern "C" fn do_machine_check(regs: *mut pt_regs) -> noinstr void {
    noinstr void do_machine_check(struct pt_regs *regs)
    {
    let mut worst: c_int = 0, order, no_way_out, kill_current_task, lmce, taint = 0;
    DECLARE_BITMAP(valid_banks, MAX_NR_BANKS) = { 0 };
    DECLARE_BITMAP(toclear, MAX_NR_BANKS) = { 0 };
    struct mce_hw_err *final;
    struct mce_hw_err err;
    char *msg = core::ptr::null_mut();
    struct mce *m;
    if (unlikely(mce_flags.p5))
    return pentium_machine_check(regs);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: unlikely(mce_flags.winchip)) -> else {
    else if (unlikely(mce_flags.winchip))
    return winchip_machine_check(regs);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: unlikely(!mca_cfg.initialized)) -> else {
    else if (unlikely(!mca_cfg.initialized))
    return unexpected_machine_check(regs);
    if (mce_flags.skx_repmov_quirk && quirk_skylake_repmov())
    goto clear;
//
// Establish sequential order between the CPUs entering the machine
// check handler.
//
    order = -1;
//
// If no_way_out gets set, there is no safe way to recover from this
// MCE.
//
    no_way_out = 0;
//
// If kill_current_task is not set, there might be a way to recover from this
// error.
//
    kill_current_task = 0;
//
// MCEs are always local on AMD. Same is determined by MCG_STATUS_LMCES
// on Intel.
//
    lmce = 1;
    inc_irq_stat(MCE_EXCEPTION);
    mce_gather_info(&err, regs);
    m = &err.m;
    m.tsc = rdtsc();
    final = this_cpu_ptr(&hw_errs_seen);
// final = err;
    no_way_out = mce_no_way_out(&err, &msg, valid_banks, regs);
    barrier();
//
// When no restart IP might need to kill or panic.
// Assume the worst for now, but if we find the
// severity is MCE_AR_SEVERITY we have other options.
//
    if (!(m.mcgstatus & MCG_STATUS_RIPV))
    kill_current_task = 1;
//
// Check if this MCE is signaled to only this logical processor,
// on Intel, Zhaoxin only.
//
    if (m.cpuvendor == X86_VENDOR_INTEL ||
    m.cpuvendor == X86_VENDOR_ZHAOXIN)
    lmce = m.mcgstatus & MCG_STATUS_LMCES;
//
// Local machine check may already know that we have to panic.
// Broadcast machine check begins rendezvous in mce_start()
// Go through all banks in exclusion of the other CPUs. This way we
// don't report duplicated events on shared banks because the first one
// to see it will clear it.
//
    if (lmce) {
    if (no_way_out)
    mce_panic("Fatal local machine check", &err, msg);
    } else {
    order = mce_start(&no_way_out);
    }
    taint = __mc_scan_banks(&err, regs, final, toclear, valid_banks, no_way_out, &worst);
    if (!no_way_out)
    mce_clear_state(toclear);
//
// Do most of the synchronization with other CPUs.
// When there's any problem use only local no_way_out state.
//
    if (!lmce) {
    if (mce_end(order) < 0) {
    if (!no_way_out)
    no_way_out = worst >= MCE_PANIC_SEVERITY;
    if (no_way_out)
    mce_panic("Fatal machine check on current CPU", &err, msg);
    }
    } else {
//
// If there was a fatal machine check we should have
// already called mce_panic earlier in this function.
// Since we re-read the banks, we might have found
// something new. Check again to see if we found a
// fatal error. We call "mce_severity()" again to
// make sure we have the right "msg".
//
    if (worst >= MCE_PANIC_SEVERITY) {
    mce_severity(m, regs, &msg, true);
    mce_panic("Local fatal machine check!", &err, msg);
    }
    }
//
// Enable instrumentation around the external facilities like task_work_add()
// (via queue_task_work()), fixup_exception() etc. For now, that is. Fixing this
// properly would need a lot more involved reorganization.
//
    instrumentation_begin();
    if (taint)
    add_taint(TAINT_MACHINE_CHECK, LOCKDEP_NOW_UNRELIABLE);
    if (worst != MCE_AR_SEVERITY && !kill_current_task)
    goto out;
// Fault was in user mode and we need to take some action
    if ((m.cs & 3) == 3) {
// If this triggers there is no way to recover. Die hard.
    BUG_ON(!on_thread_stack() || !user_mode(regs));
    if (!mce_usable_address(m))
    queue_task_work(&err, msg, kill_me_now);
    else
    queue_task_work(&err, msg, kill_me_maybe);
    } else if (m.mcgstatus & MCG_STATUS_SEAM_NR) {
//
// Saved RIP on stack makes it look like the machine check
// was taken in the kernel on the instruction following
// the entry to SEAM mode. But MCG_STATUS_SEAM_NR indicates
// that the machine check was taken inside SEAM non-root
// mode.  CPU core has already marked that guest as dead.
// It is OK for the kernel to resume execution at the
// apparent point of the machine check as the fault did
// not occur there. Mark the page as poisoned so it won't
// be added to free list when the guest is terminated.
//
    if (mce_usable_address(m)) {
    struct page *p = pfn_to_online_page(m.addr >> PAGE_SHIFT);
    if (p)
    SetPageHWPoison(p);
    }
    } else {
//
// Handle an MCE which has happened in kernel space but from
// which the kernel can recover: ex_has_fault_handler() has
// already verified that the rIP at which the error happened is
// a rIP from which the kernel can recover (by jumping to
// recovery code specified in _ASM_EXTABLE_FAULT()) and the
// corresponding exception handler which would do that is the
// proper one.
//
    if (m.kflags & MCE_IN_KERNEL_RECOV) {
    if (!fixup_exception(regs, X86_TRAP_MC, 0, 0))
    mce_panic("Failed kernel mode recovery", &err, msg);
    }
    if (m.kflags & MCE_IN_KERNEL_COPYIN)
    queue_task_work(&err, msg, kill_me_never);
    }
    out:
// Given it didn't panic, mark it as recoverable
    hwerr_log_error_type(HWERR_RECOV_OTHERS);
    instrumentation_end();
    clear:
    mce_wrmsrq(MSR_IA32_MCG_STATUS, 0);
    }
    EXPORT_SYMBOL_GPL(do_machine_check);

#[no_mangle]
pub unsafe extern "C" fn memory_failure(pfn: c_ulong, flags: c_int) -> c_int {
    int memory_failure(unsigned long pfn, int flags)
    {
// mce_severity() should not hand us an ACTION_REQUIRED error
    BUG_ON(flags & MF_ACTION_REQUIRED);
    pr_err("Uncorrected memory error in page 0x%lx ignored\n"
    "Rebuild kernel with CONFIG_MEMORY_FAILURE=y for smarter handling\n",
    pfn);
    return 0;
    }

//
// Periodic polling timer for "silent" machine check errors.  If the
// poller finds an MCE, poll 2x faster.  When the poller finds no more
// errors, poll 2x slower (up to check_interval seconds).
//
    let mut check_interval: static unsigned long = INITIAL_CHECK_INTERVAL;
    static DEFINE_PER_CPU(unsigned long, mce_next_interval); /* in jiffies */
    static DEFINE_PER_CPU(struct timer_list, mce_timer);
#[no_mangle]
unsafe extern "C" fn __start_timer(t: *mut timer_list, interval: c_ulong) {
    static void __start_timer(struct timer_list *t, unsigned long interval)
    {
    let mut when: c_ulong = jiffies + interval;
    unsigned long flags;
    local_irq_save(flags);
    if (!timer_pending(t) || time_before(when, t.expires))
    mod_timer(t, round_jiffies(when));
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn mc_poll_banks_default() {
    static void mc_poll_banks_default(void)
    {
    machine_check_poll(0, this_cpu_ptr(&mce_poll_banks));
    }
    void (*mc_poll_banks)(void) = mc_poll_banks_default;
#[no_mangle]
unsafe extern "C" fn should_enable_timer(iv: c_ulong) -> bool {
    static bool should_enable_timer(unsigned long iv)
    {
    return !mca_cfg.ignore_ce && iv;
    }
#[no_mangle]
unsafe extern "C" fn mce_timer_fn(t: *mut timer_list) {
    static void mce_timer_fn(struct timer_list *t)
    {
    struct timer_list *cpu_t = this_cpu_ptr(&mce_timer);
    unsigned long iv;
    WARN_ON(cpu_t != t);
    iv = __this_cpu_read(mce_next_interval);
    if (mce_available(this_cpu_ptr(&cpu_info)))
    mc_poll_banks();
//
// Alert userspace if needed. If we logged an MCE, reduce the polling
// interval, otherwise increase the polling interval.
//
    if (!mce_gen_pool_empty())
    iv = max(iv / 2, (unsigned long) HZ/100);
    else
    iv = min(iv * 2, round_jiffies_relative(check_interval * HZ));
    if (mce_get_storm_mode()) {
    __start_timer(t, HZ);
    } else if (should_enable_timer(iv)) {
    __this_cpu_write(mce_next_interval, iv);
    __start_timer(t, iv);
    }
    }
//
// When a storm starts on any bank on this CPU, switch to polling
// once per second. When the storm ends, revert to the default
// polling interval.
//
#[no_mangle]
pub unsafe extern "C" fn mce_timer_kick(storm: bool) {
    void mce_timer_kick(bool storm)
    {
    struct timer_list *t = this_cpu_ptr(&mce_timer);
    mce_set_storm_mode(storm);
    if (storm)
    __start_timer(t, HZ);
    else
    __this_cpu_write(mce_next_interval, check_interval * HZ);
    }
// Must not be called in IRQ context where timer_delete_sync() can deadlock
#[no_mangle]
unsafe extern "C" fn mce_timer_delete_all() {
    static void mce_timer_delete_all(void)
    {
    int cpu;
    for_each_online_cpu(cpu)
    timer_delete_sync(&per_cpu(mce_timer, cpu));
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_mce_banks_init() {
    static void __mcheck_cpu_mce_banks_init(void)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
    let mut n_banks: u8 = this_cpu_read(mce_num_banks);
    int i;
    for (i = 0; i < n_banks; i++) {
    struct mce_bank *b = &mce_banks[i];
//
// Init them all by default.
//
// The required vendor quirks will be applied before
// __mcheck_cpu_init_prepare_banks() does the final bank setup.
//
    b.ctl = -1ULL;
    b.init = true;
    }
    }
//
// Initialize Machine Checks for a CPU.
//
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_cap_init() {
    static void __mcheck_cpu_cap_init(void)
    {
    u64 cap;
    u8 b;
    rdmsrq(MSR_IA32_MCG_CAP, cap);
    b = cap & MCG_BANKCNT_MASK;
    if (b > MAX_NR_BANKS) {
    pr_warn("CPU%d: Using only %u machine check banks out of %u\n",
    smp_processor_id(), MAX_NR_BANKS, b);
    b = MAX_NR_BANKS;
    }
    this_cpu_write(mce_num_banks, b);
    __mcheck_cpu_mce_banks_init();
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_init_generic() {
    static void __mcheck_cpu_init_generic(void)
    {
    u64 cap;
    rdmsrq(MSR_IA32_MCG_CAP, cap);
    if (cap & MCG_CTL_P)
    wrmsrq(MSR_IA32_MCG_CTL, ~0ULL);
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_init_prepare_banks() {
    static void __mcheck_cpu_init_prepare_banks(void)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
    u64 msrval;
    int i;
//
// Log the machine checks left over from the previous reset. Log them
// only, do not start processing them. That will happen in mcheck_late_init()
// when all consumers have been registered on the notifier chain.
//
    if (mca_cfg.bootlog) {
    mce_banks_t all_banks;
    bitmap_fill(all_banks, MAX_NR_BANKS);
    machine_check_poll(MCP_UC | MCP_QUEUE_LOG, &all_banks);
    }
    for (i = 0; i < this_cpu_read(mce_num_banks); i++) {
    struct mce_bank *b = &mce_banks[i];
    if (!b.init)
    continue;
    wrmsrq(mca_msr_reg(i, MCA_CTL), b.ctl);
    wrmsrq(mca_msr_reg(i, MCA_STATUS), 0);
    rdmsrq(mca_msr_reg(i, MCA_CTL), msrval);
    b.init = !!msrval;
    }
    }
#[no_mangle]
unsafe extern "C" fn amd_apply_global_quirks(c: *mut cpuinfo_x86) {
    static void amd_apply_global_quirks(struct cpuinfo_x86 *c)
    {
    if (c.x86 < 0x11 && mca_cfg.bootlog < 0) {
//
// Lots of broken BIOS around that don't clear them
// by default and leave crap in there. Don't log:
//
    mca_cfg.bootlog = 0;
    }
//
// overflow_recov is supported for F15h Models 00h-0fh
// even though we don't have a CPUID bit for it.
//
    if (c.x86 == 0x15 && c.x86_model <= 0xf)
    mce_flags.overflow_recov = 1;
    if (c.x86 >= 0x17 && c.x86 <= 0x1A)
    mce_flags.zen_ifu_quirk = 1;
    }
#[no_mangle]
unsafe extern "C" fn intel_apply_global_quirks(c: *mut cpuinfo_x86) {
    static void intel_apply_global_quirks(struct cpuinfo_x86 *c)
    {
// Older CPUs (prior to family 6) don't need quirks.
    if (c.x86_vfm < INTEL_PENTIUM_PRO)
    return;
//
// All newer Intel systems support MCE broadcasting. Enable
// synchronization with a one second timeout.
//
    if (c.x86_vfm >= INTEL_CORE_YONAH && mca_cfg.monarch_timeout < 0)
    mca_cfg.monarch_timeout = USEC_PER_SEC;
//
// There are also broken BIOSes on some Pentium M and
// earlier systems:
//
    if (c.x86_vfm < INTEL_CORE_YONAH && mca_cfg.bootlog < 0)
    mca_cfg.bootlog = 0;
    if (c.x86_vfm == INTEL_SANDYBRIDGE_X)
    mce_flags.snb_ifu_quirk = 1;
//
// Skylake, Cascacde Lake and Cooper Lake require a quirk on
// rep movs.
//
    if (c.x86_vfm == INTEL_SKYLAKE_X)
    mce_flags.skx_repmov_quirk = 1;
    }
#[no_mangle]
unsafe extern "C" fn zhaoxin_apply_global_quirks(c: *mut cpuinfo_x86) {
    static void zhaoxin_apply_global_quirks(struct cpuinfo_x86 *c)
    {
//
// All newer Zhaoxin CPUs support MCE broadcasting. Enable
// synchronization with a one second timeout.
//
    if (c.x86 > 6 || (c.x86_model == 0x19 || c.x86_model == 0x1f)) {
    if (mca_cfg.monarch_timeout < 0)
    mca_cfg.monarch_timeout = USEC_PER_SEC;
    }
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_ancient_init(c: *mut cpuinfo_x86) -> bool {
    static bool __mcheck_cpu_ancient_init(struct cpuinfo_x86 *c)
    {
    if (c.x86 != 5)
    return false;
    switch (c.x86_vendor) {
    case X86_VENDOR_INTEL:
    intel_p5_mcheck_init(c);
    mce_flags.p5 = 1;
    return true;
    case X86_VENDOR_CENTAUR:
    winchip_mcheck_init(c);
    mce_flags.winchip = 1;
    return true;
    default:
    return false;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mce_centaur_feature_init(c: *mut cpuinfo_x86) {
    static void mce_centaur_feature_init(struct cpuinfo_x86 *c)
    {
    struct mca_config *cfg = &mca_cfg;
//
// All newer Centaur CPUs support MCE broadcasting. Enable
// synchronization with a one second timeout.
//
    if ((c.x86 == 6 && c.x86_model == 0xf && c.x86_stepping >= 0xe) ||
    c.x86 > 6) {
    if (cfg.monarch_timeout < 0)
    cfg.monarch_timeout = USEC_PER_SEC;
    }
    }
#[no_mangle]
unsafe extern "C" fn mce_zhaoxin_feature_init(c: *mut cpuinfo_x86) {
    static void mce_zhaoxin_feature_init(struct cpuinfo_x86 *c)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
//
// These CPUs have MCA bank 8 which reports only one error type called
// SVAD (System View Address Decoder). The reporting of that error is
// controlled by IA32_MC8.CTL.0.
//
// If enabled, prefetching on these CPUs will cause SVAD MCE when
// virtual machines start and result in a system  panic. Always disable
// bank 8 SVAD error by default.
//
    if ((c.x86 == 7 && c.x86_model == 0x1b) ||
    (c.x86_model == 0x19 || c.x86_model == 0x1f)) {
    if (this_cpu_read(mce_num_banks) > 8)
    mce_banks[8].ctl = 0;
    }
    intel_init_cmci();
    intel_init_lmce();
    }
#[no_mangle]
unsafe extern "C" fn mce_zhaoxin_feature_clear(c: *mut cpuinfo_x86) {
    static void mce_zhaoxin_feature_clear(struct cpuinfo_x86 *c)
    {
    intel_clear_lmce();
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_init_vendor(c: *mut cpuinfo_x86) {
    static void __mcheck_cpu_init_vendor(struct cpuinfo_x86 *c)
    {
    switch (c.x86_vendor) {
    case X86_VENDOR_INTEL:
    mce_intel_feature_init(c);
    break;
    case X86_VENDOR_AMD:
    case X86_VENDOR_HYGON:
    mce_amd_feature_init(c);
    break;
    case X86_VENDOR_CENTAUR:
    mce_centaur_feature_init(c);
    break;
    case X86_VENDOR_ZHAOXIN:
    mce_zhaoxin_feature_init(c);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_clear_vendor(c: *mut cpuinfo_x86) {
    static void __mcheck_cpu_clear_vendor(struct cpuinfo_x86 *c)
    {
    switch (c.x86_vendor) {
    case X86_VENDOR_INTEL:
    mce_intel_feature_clear(c);
    break;
    case X86_VENDOR_ZHAOXIN:
    mce_zhaoxin_feature_clear(c);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mce_start_timer(t: *mut timer_list) {
    static void mce_start_timer(struct timer_list *t)
    {
    let mut iv: c_ulong = check_interval * HZ;
    if (should_enable_timer(iv)) {
    this_cpu_write(mce_next_interval, iv);
    __start_timer(t, iv);
    }
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_setup_timer() {
    static void __mcheck_cpu_setup_timer(void)
    {
    struct timer_list *t = this_cpu_ptr(&mce_timer);
    timer_setup(t, mce_timer_fn, TIMER_PINNED);
    }
#[no_mangle]
unsafe extern "C" fn __mcheck_cpu_init_timer() {
    static void __mcheck_cpu_init_timer(void)
    {
    struct timer_list *t = this_cpu_ptr(&mce_timer);
    timer_setup(t, mce_timer_fn, TIMER_PINNED);
    mce_start_timer(t);
    }
#[no_mangle]
pub unsafe extern "C" fn filter_mce(m: *mut mce) -> bool {
    bool filter_mce(struct mce *m)
    {
    if (boot_cpu_data.x86_vendor == X86_VENDOR_AMD)
    return amd_filter_mce(m);
    if (boot_cpu_data.x86_vendor == X86_VENDOR_INTEL)
    return intel_filter_mce(m);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn exc_machine_check_kernel(regs: *mut pt_regs) -> __always_inline void {
    static __always_inline void exc_machine_check_kernel(struct pt_regs *regs)
    {
    irqentry_state_t irq_state;
    WARN_ON_ONCE(user_mode(regs));
//
// Only required when from kernel mode. See
// mce_check_crashing_cpu() for details.
//
    if (mca_cfg.initialized && mce_check_crashing_cpu())
    return;
    irq_state = irqentry_nmi_enter(regs);
    do_machine_check(regs);
    irqentry_nmi_exit(regs, irq_state);
    }
#[no_mangle]
unsafe extern "C" fn exc_machine_check_user(regs: *mut pt_regs) -> __always_inline void {
    static __always_inline void exc_machine_check_user(struct pt_regs *regs)
    {
    irqentry_enter_from_user_mode(regs);
    do_machine_check(regs);
    irqentry_exit_to_user_mode(regs);
    }

// MCE hit kernel mode
    DEFINE_IDTENTRY_MCE(exc_machine_check)
    {
    unsigned long dr7;
    dr7 = local_db_save();
    exc_machine_check_kernel(regs);
    local_db_restore(dr7);
    }
// The user mode variant.
    DEFINE_IDTENTRY_MCE_USER(exc_machine_check)
    {
    unsigned long dr7;
    dr7 = local_db_save();
    exc_machine_check_user(regs);
    local_db_restore(dr7);
    }

//
// When occurred on different ring level, i.e., from user or kernel
// context, #MCE needs to be handled on different stack: User #MCE
// on current task stack, while kernel #MCE on a dedicated stack.
//
// This is exactly how FRED event delivery invokes an exception
// handler: ring 3 event on level 0 stack, i.e., current task stack;
// ring 0 event on the #MCE dedicated stack specified in the
// IA32_FRED_STKLVLS MSR. So unlike IDT, the FRED machine check entry
// stub doesn't do stack switch.
//
    DEFINE_FREDENTRY_MCE(exc_machine_check)
    {
    unsigned long dr7;
    dr7 = local_db_save();
    if (user_mode(regs))
    exc_machine_check_user(regs);
    else
    exc_machine_check_kernel(regs);
    local_db_restore(dr7);
    }

// 32bit unified entry point
    DEFINE_IDTENTRY_RAW(exc_machine_check)
    {
    unsigned long dr7;
    dr7 = local_db_save();
    if (user_mode(regs))
    exc_machine_check_user(regs);
    else
    exc_machine_check_kernel(regs);
    local_db_restore(dr7);
    }

#[no_mangle]
pub unsafe extern "C" fn mca_bsp_init(c: *mut cpuinfo_x86) {
    void mca_bsp_init(struct cpuinfo_x86 *c)
    {
    u64 cap;
    if (!mce_available(c))
    return;
    if (c.x86_vendor == X86_VENDOR_UNKNOWN) {
    mca_cfg.disabled = 1;
    pr_info("unknown CPU type - not enabling MCE support\n");
    return;
    }
    mce_flags.overflow_recov = cpu_feature_enabled(X86_FEATURE_OVERFLOW_RECOV);
    mce_flags.succor	 = cpu_feature_enabled(X86_FEATURE_SUCCOR);
    mce_flags.smca		 = cpu_feature_enabled(X86_FEATURE_SMCA);
    if (mce_flags.smca)
    smca_bsp_init();
    rdmsrq(MSR_IA32_MCG_CAP, cap);
// Use accurate RIP reporting if available.
    if ((cap & MCG_EXT_P) && MCG_EXT_CNT(cap) >= 9)
    mca_cfg.rip_msr = MSR_IA32_MCG_EIP;
    if (cap & MCG_SER_P)
    mca_cfg.ser = 1;
    switch (c.x86_vendor) {
    case X86_VENDOR_AMD:
    amd_apply_global_quirks(c);
    break;
    case X86_VENDOR_INTEL:
    intel_apply_global_quirks(c);
    break;
    case X86_VENDOR_ZHAOXIN:
    zhaoxin_apply_global_quirks(c);
    break;
    }
    if (mca_cfg.monarch_timeout < 0)
    mca_cfg.monarch_timeout = 0;
    if (mca_cfg.bootlog != 0)
    mca_cfg.panic_timeout = 30;
    }
//
// Called for each booted CPU to set up machine checks.
// Must be called with preempt off:
//
#[no_mangle]
pub unsafe extern "C" fn mcheck_cpu_init(c: *mut cpuinfo_x86) {
    void mcheck_cpu_init(struct cpuinfo_x86 *c)
    {
    if (mca_cfg.disabled)
    return;
    if (__mcheck_cpu_ancient_init(c))
    return;
    if (!mce_available(c))
    return;
    __mcheck_cpu_cap_init();
    if (!mce_gen_pool_init()) {
    mca_cfg.disabled = 1;
    pr_emerg("Couldn't allocate MCE records pool!\n");
    return;
    }
    mca_cfg.initialized = 1;
    __mcheck_cpu_setup_timer();
    __mcheck_cpu_init_generic();
    __mcheck_cpu_init_vendor(c);
    __mcheck_cpu_init_prepare_banks();
    cr4_set_bits(X86_CR4_MCE);
    }
//
// Called for each booted CPU to clear some machine checks opt-ins
//
#[no_mangle]
pub unsafe extern "C" fn mcheck_cpu_clear(c: *mut cpuinfo_x86) {
    void mcheck_cpu_clear(struct cpuinfo_x86 *c)
    {
    if (mca_cfg.disabled)
    return;
    if (!mce_available(c))
    return;
//
// Possibly to clear general settings generic to x86
// __mcheck_cpu_clear_generic(c);
//
    __mcheck_cpu_clear_vendor(c);
    }
#[no_mangle]
unsafe extern "C" fn __mce_disable_bank(arg: *mut c_void) {
    static void __mce_disable_bank(void *arg)
    {
    let mut bank: c_int = *((int *)arg);
    __clear_bit(bank, this_cpu_ptr(mce_poll_banks));
    cmci_disable_bank(bank);
    }
#[no_mangle]
pub unsafe extern "C" fn mce_disable_bank(bank: c_int) {
    void mce_disable_bank(int bank)
    {
    if (bank >= this_cpu_read(mce_num_banks)) {
    pr_warn(FW_BUG
    "Ignoring request to disable invalid MCA bank %d.\n",
    bank);
    return;
    }
    set_bit(bank, mce_banks_ce_disabled);
    on_each_cpu(__mce_disable_bank, &bank, 1);
    }
//
// mce=off Disables machine check
// mce=no_cmci Disables CMCI
// mce=no_lmce Disables LMCE
// mce=dont_log_ce Clears corrected events silently, no log created for CEs.
// mce=print_all Print all machine check logs to console
// mce=ignore_ce Disables polling and CMCI, corrected events are not cleared.
// mce=TOLERANCELEVEL[,monarchtimeout] (number, see above)
// monarchtimeout is how long to wait for other CPUs on machine
// check, or 0 to not wait
// mce=bootlog Log MCEs from before booting. Disabled by default on AMD Fam10h
    and older.
// mce=nobootlog Don't log MCEs from before booting.
// mce=bios_cmci_threshold Don't program the CMCI threshold
// mce=recovery force enable copy_mc_fragile()
//
#[no_mangle]
unsafe extern "C" fn mcheck_enable(str: *mut c_char) -> int __init {
    static int __init mcheck_enable(char *str)
    {
    struct mca_config *cfg = &mca_cfg;
    if (*str == 0) {
    enable_p5_mce();
    return 1;
    }
    if (*str == '=')
    str++;
    if (!strcmp(str, "off"))
    cfg.disabled = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "no_cmci")) -> else {
    else if (!strcmp(str, "no_cmci"))
    cfg.cmci_disabled = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "no_lmce")) -> else {
    else if (!strcmp(str, "no_lmce"))
    cfg.lmce_disabled = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "dont_log_ce")) -> else {
    else if (!strcmp(str, "dont_log_ce"))
    cfg.dont_log_ce = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "print_all")) -> else {
    else if (!strcmp(str, "print_all"))
    cfg.print_all = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "ignore_ce")) -> else {
    else if (!strcmp(str, "ignore_ce"))
    cfg.ignore_ce = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, !strcmp(str: "bootlog") ||, _arg: "nobootlog")) -> else {
    else if (!strcmp(str, "bootlog") || !strcmp(str, "nobootlog"))
    cfg.bootlog = (str[0] == 'b');
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "bios_cmci_threshold")) -> else {
    else if (!strcmp(str, "bios_cmci_threshold"))
    cfg.bios_cmci_threshold = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(str, _arg: "recovery")) -> else {
    else if (!strcmp(str, "recovery"))
    cfg.recovery = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: isdigit(str[0])) -> else {
    else if (isdigit(str[0]))
    get_option(&str, &(cfg.monarch_timeout));
    else {
    pr_info("mce argument %s ignored. Please use /sys\n", str);
    return 0;
    }
    return 1;
    }
    __setup("mce", mcheck_enable);
#[no_mangle]
pub unsafe extern "C" fn mcheck_init() -> int __init {
    int __init mcheck_init(void)
    {
    mce_register_decode_chain(&early_nb);
    mce_register_decode_chain(&mce_uc_nb);
    mce_register_decode_chain(&mce_default_nb);
    INIT_WORK(&mce_work, mce_gen_pool_process);
    init_irq_work(&mce_irq_work, mce_irq_work_cb);
    return 0;
    }
//
// mce_syscore: PM support
//
// Disable machine checks on suspend and shutdown. We can't really handle
// them later.
//
#[no_mangle]
unsafe extern "C" fn mce_disable_error_reporting() {
    static void mce_disable_error_reporting(void)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
    int i;
    for (i = 0; i < this_cpu_read(mce_num_banks); i++) {
    struct mce_bank *b = &mce_banks[i];
    if (b.init)
    wrmsrq(mca_msr_reg(i, MCA_CTL), 0);
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn vendor_disable_error_reporting() {
    static void vendor_disable_error_reporting(void)
    {
//
// Don't clear on Intel or AMD or Hygon or Zhaoxin CPUs. Some of these
// MSRs are socket-wide. Disabling them for just a single offlined CPU
// is bad, since it will inhibit reporting for all shared resources on
// the socket like the last level cache (LLC), the integrated memory
// controller (iMC), etc.
//
    if (boot_cpu_data.x86_vendor == X86_VENDOR_INTEL ||
    boot_cpu_data.x86_vendor == X86_VENDOR_HYGON ||
    boot_cpu_data.x86_vendor == X86_VENDOR_AMD ||
    boot_cpu_data.x86_vendor == X86_VENDOR_ZHAOXIN)
    return;
    mce_disable_error_reporting();
    }
#[no_mangle]
unsafe extern "C" fn mce_syscore_suspend(data: *mut c_void) -> c_int {
    static int mce_syscore_suspend(void *data)
    {
    vendor_disable_error_reporting();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mce_syscore_shutdown(data: *mut c_void) {
    static void mce_syscore_shutdown(void *data)
    {
    vendor_disable_error_reporting();
    }
//
// On resume clear all MCE state. Don't want to see leftovers from the BIOS.
// Only one CPU is active at this time, the others get re-added later using
// CPU hotplug:
//
#[no_mangle]
unsafe extern "C" fn mce_syscore_resume(data: *mut c_void) {
    static void mce_syscore_resume(void *data)
    {
    __mcheck_cpu_init_generic();
    __mcheck_cpu_init_vendor(raw_cpu_ptr(&cpu_info));
    __mcheck_cpu_init_prepare_banks();
    cr4_set_bits(X86_CR4_MCE);
    }
    static const struct syscore_ops mce_syscore_ops = {
    .suspend	= mce_syscore_suspend,
    .shutdown	= mce_syscore_shutdown,
    .resume		= mce_syscore_resume,
    };
    static struct syscore mce_syscore = {
    .ops = &mce_syscore_ops,
    };
//
// mce_device: Sysfs support
//
#[no_mangle]
unsafe extern "C" fn mce_cpu_restart(data: *mut c_void) {
    static void mce_cpu_restart(void *data)
    {
    if (!mce_available(raw_cpu_ptr(&cpu_info)))
    return;
    __mcheck_cpu_init_generic();
    __mcheck_cpu_init_prepare_banks();
    __mcheck_cpu_init_timer();
    cr4_set_bits(X86_CR4_MCE);
    }
// Reinit MCEs after user configuration changes
#[no_mangle]
unsafe extern "C" fn mce_restart() {
    static void mce_restart(void)
    {
    mce_timer_delete_all();
    on_each_cpu(mce_cpu_restart, core::ptr::null_mut(), 1);
    mce_schedule_work();
    }
// Toggle features for corrected errors
#[no_mangle]
unsafe extern "C" fn mce_disable_cmci(data: *mut c_void) {
    static void mce_disable_cmci(void *data)
    {
    if (!mce_available(raw_cpu_ptr(&cpu_info)))
    return;
    cmci_clear();
    }
#[no_mangle]
unsafe extern "C" fn mce_enable_ce(all: *mut c_void) {
    static void mce_enable_ce(void *all)
    {
    if (!mce_available(raw_cpu_ptr(&cpu_info)))
    return;
    cmci_reenable();
    cmci_recheck();
    if (all)
    __mcheck_cpu_init_timer();
    }
    static const struct bus_type mce_subsys = {
    .name		= "machinecheck",
    .dev_name	= "machinecheck",
    };
    DEFINE_PER_CPU(struct device *, mce_device);
    static inline struct mce_bank_dev *attr_to_bank(struct device_attribute *attr)
    {
    return container_of(attr, struct mce_bank_dev, attr);
    }
    static ssize_t show_bank(struct device *s, struct device_attribute *attr,
    char *buf)
    {
    let mut bank: u8 = attr_to_bank(attr).bank;
    struct mce_bank *b;
    if (bank >= per_cpu(mce_num_banks, s.id))
    return -EINVAL;
    b = &per_cpu(mce_banks_array, s.id)[bank];
    if (!b.init)
    return -ENODEV;
    return sprintf(buf, "%llx\n", b.ctl);
    }
    static ssize_t set_bank(struct device *s, struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut bank: u8 = attr_to_bank(attr).bank;
    struct mce_bank *b;
    u64 new;
    if (kstrtou64(buf, 0, &new) < 0)
    return -EINVAL;
    if (bank >= per_cpu(mce_num_banks, s.id))
    return -EINVAL;
    b = &per_cpu(mce_banks_array, s.id)[bank];
    if (!b.init)
    return -ENODEV;
    b.ctl = new;
    mutex_lock(&mce_sysfs_mutex);
    mce_restart();
    mutex_unlock(&mce_sysfs_mutex);
    return size;
    }
    static ssize_t set_ignore_ce(struct device *s,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    u64 new;
    if (kstrtou64(buf, 0, &new) < 0)
    return -EINVAL;
    mutex_lock(&mce_sysfs_mutex);
    if (mca_cfg.ignore_ce ^ !!new) {
    if (new) {
// disable ce features
    mce_timer_delete_all();
    on_each_cpu(mce_disable_cmci, core::ptr::null_mut(), 1);
    mca_cfg.ignore_ce = true;
    } else {
// enable ce features
    mca_cfg.ignore_ce = false;
    on_each_cpu(mce_enable_ce, (void *)1, 1);
    }
    }
    mutex_unlock(&mce_sysfs_mutex);
    return size;
    }
    static ssize_t set_cmci_disabled(struct device *s,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    u64 new;
    if (kstrtou64(buf, 0, &new) < 0)
    return -EINVAL;
    mutex_lock(&mce_sysfs_mutex);
    if (mca_cfg.cmci_disabled ^ !!new) {
    if (new) {
// disable cmci
    on_each_cpu(mce_disable_cmci, core::ptr::null_mut(), 1);
    mca_cfg.cmci_disabled = true;
    } else {
// enable cmci
    mca_cfg.cmci_disabled = false;
    on_each_cpu(mce_enable_ce, core::ptr::null_mut(), 1);
    }
    }
    mutex_unlock(&mce_sysfs_mutex);
    return size;
    }
    static ssize_t store_int_with_restart(struct device *s,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut old_check_interval: c_ulong = check_interval;
    let mut ret: isize = device_store_ulong(s, attr, buf, size);
    if (check_interval == old_check_interval)
    return ret;
    mutex_lock(&mce_sysfs_mutex);
    mce_restart();
    mutex_unlock(&mce_sysfs_mutex);
    return ret;
    }
    static DEVICE_INT_ATTR(monarch_timeout, 0644, mca_cfg.monarch_timeout);
    static DEVICE_BOOL_ATTR(dont_log_ce, 0644, mca_cfg.dont_log_ce);
    static DEVICE_BOOL_ATTR(print_all, 0644, mca_cfg.print_all);
    static struct dev_ext_attribute dev_attr_check_interval = {
    __ATTR(check_interval, 0644, device_show_int, store_int_with_restart),
    &check_interval
    };
    static struct dev_ext_attribute dev_attr_ignore_ce = {
    __ATTR(ignore_ce, 0644, device_show_bool, set_ignore_ce),
    &mca_cfg.ignore_ce
    };
    static struct dev_ext_attribute dev_attr_cmci_disabled = {
    __ATTR(cmci_disabled, 0644, device_show_bool, set_cmci_disabled),
    &mca_cfg.cmci_disabled
    };
    static struct device_attribute *mce_device_attrs[] = {
    &dev_attr_check_interval.attr,

    &dev_attr_trigger,

    &dev_attr_monarch_timeout.attr,
    &dev_attr_dont_log_ce.attr,
    &dev_attr_print_all.attr,
    &dev_attr_ignore_ce.attr,
    &dev_attr_cmci_disabled.attr,
    core::ptr::null_mut()
    };
    static cpumask_var_t mce_device_initialized;
#[no_mangle]
unsafe extern "C" fn mce_device_release(dev: *mut device) {
    static void mce_device_release(struct device *dev)
    {
    kfree(dev);
    }
// Per CPU device init. All of the CPUs still share the same bank device:
#[no_mangle]
unsafe extern "C" fn mce_device_create(cpu: c_uint) -> c_int {
    static int mce_device_create(unsigned int cpu)
    {
    struct device *dev;
    int err;
    int i, j;
    dev = per_cpu(mce_device, cpu);
    if (dev)
    return 0;
    dev = kzalloc_obj(*dev);
    if (!dev)
    return -ENOMEM;
    dev.id  = cpu;
    dev.bus = &mce_subsys;
    dev.release = &mce_device_release;
    err = device_register(dev);
    if (err) {
    put_device(dev);
    return err;
    }
    for (i = 0; mce_device_attrs[i]; i++) {
    err = device_create_file(dev, mce_device_attrs[i]);
    if (err)
    goto error;
    }
    for (j = 0; j < per_cpu(mce_num_banks, cpu); j++) {
    err = device_create_file(dev, &mce_bank_devs[j].attr);
    if (err)
    goto error2;
    }
    cpumask_set_cpu(cpu, mce_device_initialized);
    per_cpu(mce_device, cpu) = dev;
    return 0;
    error2:
    while (--j >= 0)
    device_remove_file(dev, &mce_bank_devs[j].attr);
    error:
    while (--i >= 0)
    device_remove_file(dev, mce_device_attrs[i]);
    device_unregister(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mce_device_remove(cpu: c_uint) {
    static void mce_device_remove(unsigned int cpu)
    {
    struct device *dev = per_cpu(mce_device, cpu);
    int i;
    if (!cpumask_test_cpu(cpu, mce_device_initialized))
    return;
    for (i = 0; mce_device_attrs[i]; i++)
    device_remove_file(dev, mce_device_attrs[i]);
    for (i = 0; i < per_cpu(mce_num_banks, cpu); i++)
    device_remove_file(dev, &mce_bank_devs[i].attr);
    device_unregister(dev);
    cpumask_clear_cpu(cpu, mce_device_initialized);
    per_cpu(mce_device, cpu) = core::ptr::null_mut();
    }
// Make sure there are no machine checks on offlined CPUs.
#[no_mangle]
unsafe extern "C" fn mce_disable_cpu() {
    static void mce_disable_cpu(void)
    {
    if (!mce_available(raw_cpu_ptr(&cpu_info)))
    return;
    if (!cpuhp_tasks_frozen)
    cmci_clear();
    vendor_disable_error_reporting();
    }
#[no_mangle]
unsafe extern "C" fn mce_reenable_cpu() {
    static void mce_reenable_cpu(void)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
    int i;
    if (!mce_available(raw_cpu_ptr(&cpu_info)))
    return;
    if (!cpuhp_tasks_frozen)
    cmci_reenable();
    for (i = 0; i < this_cpu_read(mce_num_banks); i++) {
    struct mce_bank *b = &mce_banks[i];
    if (b.init)
    wrmsrq(mca_msr_reg(i, MCA_CTL), b.ctl);
    }
    }
#[no_mangle]
unsafe extern "C" fn mce_cpu_dead(cpu: c_uint) -> c_int {
    static int mce_cpu_dead(unsigned int cpu)
    {
// intentionally ignoring frozen here
    if (!cpuhp_tasks_frozen)
    cmci_rediscover();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mce_cpu_online(cpu: c_uint) -> c_int {
    static int mce_cpu_online(unsigned int cpu)
    {
    struct timer_list *t = this_cpu_ptr(&mce_timer);
    mce_device_create(cpu);
    mce_threshold_create_device(cpu);
    mce_reenable_cpu();
    mce_start_timer(t);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mce_cpu_pre_down(cpu: c_uint) -> c_int {
    static int mce_cpu_pre_down(unsigned int cpu)
    {
    struct timer_list *t = this_cpu_ptr(&mce_timer);
    mce_disable_cpu();
    timer_delete_sync(t);
    mce_threshold_remove_device(cpu);
    mce_device_remove(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mce_init_banks() -> __init void {
    static __init void mce_init_banks(void)
    {
    int i;
    for (i = 0; i < MAX_NR_BANKS; i++) {
    struct mce_bank_dev *b = &mce_bank_devs[i];
    struct device_attribute *a = &b.attr;
    b.bank = i;
    sysfs_attr_init(&a.attr);
    a.attr.name	= b.attrname;
    snprintf(b.attrname, ATTR_LEN, "bank%d", i);
    a.attr.mode	= 0644;
    a.show		= show_bank;
    a.store	= set_bank;
    }
    }
//
// When running on XEN, this initcall is ordered against the XEN mcelog
// initcall:
//
// device_initcall(xen_late_init_mcelog);
// device_initcall_sync(mcheck_init_device);
//
#[no_mangle]
unsafe extern "C" fn mcheck_init_device() -> __init int {
    static __init int mcheck_init_device(void)
    {
    int err;
//
// Check if we have a spare virtual bit. This will only become
// a problem if/when we move beyond 5-level page tables.
//
    MAYBE_BUILD_BUG_ON(__VIRTUAL_MASK_SHIFT >= 63);
    if (!mce_available(&boot_cpu_data)) {
    err = -EIO;
    goto err_out;
    }
    if (!zalloc_cpumask_var(&mce_device_initialized, GFP_KERNEL)) {
    err = -ENOMEM;
    goto err_out;
    }
    mce_init_banks();
    err = subsys_system_register(&mce_subsys, core::ptr::null_mut());
    if (err)
    goto err_out_mem;
    err = cpuhp_setup_state(CPUHP_X86_MCE_DEAD, "x86/mce:dead", core::ptr::null_mut(),
    mce_cpu_dead);
    if (err)
    goto err_out_mem;
//
// Invokes mce_cpu_online() on all CPUs which are online when
// the state is installed.
//
    err = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "x86/mce:online",
    mce_cpu_online, mce_cpu_pre_down);
    if (err < 0)
    goto err_out_online;
    register_syscore(&mce_syscore);
    return 0;
    err_out_online:
    cpuhp_remove_state(CPUHP_X86_MCE_DEAD);
    err_out_mem:
    free_cpumask_var(mce_device_initialized);
    err_out:
    pr_err("Unable to init MCE device (rc: %d)\n", err);
    return err;
    }
    device_initcall_sync(mcheck_init_device);
//
// Old style boot options parsing. Only for compatibility.
//
#[no_mangle]
unsafe extern "C" fn mcheck_disable(str: *mut c_char) -> int __init {
    static int __init mcheck_disable(char *str)
    {
    mca_cfg.disabled = 1;
    return 1;
    }
    __setup("nomce", mcheck_disable);

    struct dentry *mce_get_debugfs_dir(void)
    {
    static struct dentry *dmce;
    if (!dmce)
    dmce = debugfs_create_dir("mce", core::ptr::null_mut());
    return dmce;
    }
#[no_mangle]
unsafe extern "C" fn mce_reset() {
    static void mce_reset(void)
    {
    atomic_set(&mce_fake_panicked, 0);
    atomic_set(&mce_executing, 0);
    atomic_set(&mce_callin, 0);
    atomic_set(&global_nwo, 0);
    cpumask_setall(&mce_missing_cpus);
    }
#[no_mangle]
unsafe extern "C" fn fake_panic_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int fake_panic_get(void *data, u64 *val)
    {
// val = fake_panic;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fake_panic_set(data: *mut c_void, val: u64) -> c_int {
    static int fake_panic_set(void *data, u64 val)
    {
    mce_reset();
    fake_panic = val;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fake_panic_fops, fake_panic_get, fake_panic_set,
    "%llu\n");
#[no_mangle]
unsafe extern "C" fn mcheck_debugfs_init() -> void __init {
    static void __init mcheck_debugfs_init(void)
    {
    struct dentry *dmce;
    dmce = mce_get_debugfs_dir();
    debugfs_create_file_unsafe("fake_panic", 0444, dmce, core::ptr::null_mut(),
    &fake_panic_fops);
    }

    static void __init mcheck_debugfs_init(void) { }

#[no_mangle]
unsafe extern "C" fn mcheck_late_init() -> int __init {
    static int __init mcheck_late_init(void)
    {
    if (mca_cfg.recovery)
    enable_copy_mc_fragile();
    mcheck_debugfs_init();
//
// Flush out everything that has been logged during early boot, now that
// everything has been initialized (workqueues, decoders, ...).
//
    mce_schedule_work();
    return 0;
    }
    late_initcall(mcheck_late_init);
