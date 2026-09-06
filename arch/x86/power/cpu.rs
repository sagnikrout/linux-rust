//! Automatically rewritten from C to Rust
//! Source: arch/x86/power/cpu.c
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
// Suspend support specific for i386/x86-64.
//
// Copyright (c) 2007 Rafael J. Wysocki <rjw@sisk.pl>
// Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
// Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
//

    __visible unsigned long saved_context_ebx;
    __visible unsigned long saved_context_esp, saved_context_ebp;
    __visible unsigned long saved_context_esi, saved_context_edi;
    __visible unsigned long saved_context_eflags;

    struct saved_context saved_context;
#[no_mangle]
unsafe extern "C" fn msr_save_context(ctxt: *mut saved_context) {
    static void msr_save_context(struct saved_context *ctxt)
    {
    struct saved_msr *msr = ctxt.saved_msrs.array;
    struct saved_msr *end = msr + ctxt.saved_msrs.num;
    while (msr < end) {
    if (msr.valid)
    rdmsrq(msr.info.msr_no, msr.info.reg.q);
    msr++;
    }
    }
#[no_mangle]
unsafe extern "C" fn msr_restore_context(ctxt: *mut saved_context) {
    static void msr_restore_context(struct saved_context *ctxt)
    {
    struct saved_msr *msr = ctxt.saved_msrs.array;
    struct saved_msr *end = msr + ctxt.saved_msrs.num;
    while (msr < end) {
    if (msr.valid)
    wrmsrq(msr.info.msr_no, msr.info.reg.q);
    msr++;
    }
    }
//
// __save_processor_state() - Save CPU registers before creating a
// hibernation image and before restoring
// the memory state from it
// @ctxt: Structure to store the registers contents in.
//
// NOTE: If there is a CPU register the modification of which by the
// boot kernel (ie. the kernel used for loading the hibernation image)
// might affect the operations of the restored target kernel (ie. the one
// saved in the hibernation image), then its contents must be saved by this
// function.  In other words, if kernel A is hibernated and different
// kernel B is used for loading the hibernation image into memory, the
// kernel A's __save_processor_state() function must save all registers
// needed by kernel A, so that it can operate correctly after the resume
// regardless of what kernel B does in the meantime.
//
#[no_mangle]
unsafe extern "C" fn __save_processor_state(ctxt: *mut saved_context) {
    static void __save_processor_state(struct saved_context *ctxt)
    {

    mtrr_save_fixed_ranges(core::ptr::null_mut());

    kernel_fpu_begin();
//
// descriptor tables
//
    store_idt(&ctxt.idt);
//
// We save it here, but restore it only in the hibernate case.
// For ACPI S3 resume, this is loaded via 'early_gdt_desc' in 64-bit
// mode in "secondary_startup_64". In 32-bit mode it is done via
// 'pmode_gdt' in wakeup_start.
//
    ctxt.gdt_desc.size = GDT_SIZE - 1;
    ctxt.gdt_desc.address = (unsigned long)get_cpu_gdt_rw(smp_processor_id());
    store_tr(ctxt.tr);
// XMM0..XMM15 should be handled by kernel_fpu_begin().
//
// segment registers
//
    savesegment(gs, ctxt.gs);

    savesegment(fs, ctxt.fs);
    savesegment(ds, ctxt.ds);
    savesegment(es, ctxt.es);
    rdmsrq(MSR_FS_BASE, ctxt.fs_base);
    rdmsrq(MSR_GS_BASE, ctxt.kernelmode_gs_base);
    rdmsrq(MSR_KERNEL_GS_BASE, ctxt.usermode_gs_base);
    mtrr_save_fixed_ranges(core::ptr::null_mut());
    rdmsrq(MSR_EFER, ctxt.efer);

//
// control registers
//
    ctxt.cr0 = read_cr0();
    ctxt.cr2 = read_cr2();
    ctxt.cr3 = __read_cr3();
    ctxt.cr4 = __read_cr4();
    ctxt.misc_enable_saved = !rdmsrq_safe(MSR_IA32_MISC_ENABLE,
    &ctxt.misc_enable);
    msr_save_context(ctxt);
    }
// Needed by apm.c
#[no_mangle]
pub unsafe extern "C" fn save_processor_state() {
    void save_processor_state(void)
    {
    __save_processor_state(&saved_context);
    x86_platform.save_sched_clock_state();
    }

    EXPORT_SYMBOL(save_processor_state);

#[no_mangle]
unsafe extern "C" fn do_fpu_end() {
    static void do_fpu_end(void)
    {
//
// Restore FPU regs if necessary.
//
    kernel_fpu_end();
    }
#[no_mangle]
unsafe extern "C" fn fix_processor_context() {
    static void fix_processor_context(void)
    {
    let mut cpu: c_int = smp_processor_id();

    struct desc_struct *desc = get_cpu_gdt_rw(cpu);
    tss_desc tss;

//
// We need to reload TR, which requires that we change the
// GDT entry to indicate "available" first.
//
// XXX: This could probably all be replaced by a call to
// force_reload_TR().
//
    set_tss_desc(cpu, &get_cpu_entry_area(cpu).tss.x86_tss);

    memcpy(&tss, &desc[GDT_ENTRY_TSS], sizeof(tss_desc));
    tss.type = 0x9; /* The available 64-bit TSS (see AMD vol 2, pg 91 */
    write_gdt_entry(desc, GDT_ENTRY_TSS, &tss, DESC_TSS);
    syscall_init();				/* This sets MSR_*STAR and related */

    if (boot_cpu_has(X86_FEATURE_SEP))
    enable_sep_cpu();

    load_TR_desc();				/* This does ltr */
    load_mm_ldt(current.active_mm);	/* This does lldt */
    initialize_tlbstate_and_flush();
    fpu__resume_cpu();
// The processor is back on the direct GDT, load back the fixmap
    load_fixmap_gdt(cpu);
    }
//
// __restore_processor_state() - Restore the contents of CPU registers saved
// by __save_processor_state()
// @ctxt: Structure to load the registers contents from.
//
// The asm code that gets us here will have restored a usable GDT, although
// it will be pointing to the wrong alias.
//
#[no_mangle]
unsafe extern "C" fn __restore_processor_state(ctxt: *mut saved_context) -> void notrace {
    static void notrace __restore_processor_state(struct saved_context *ctxt)
    {
    struct cpuinfo_x86 *c;
    if (ctxt.misc_enable_saved)
    wrmsrq(MSR_IA32_MISC_ENABLE, ctxt.misc_enable);
//
// control registers
//
// cr4 was introduced in the Pentium CPU

    if (ctxt.cr4)
    __write_cr4(ctxt.cr4);

// CONFIG X86_64
    wrmsrq(MSR_EFER, ctxt.efer);
    __write_cr4(ctxt.cr4);

    write_cr3(ctxt.cr3);
    write_cr2(ctxt.cr2);
    write_cr0(ctxt.cr0);
// Restore the IDT.
    load_idt(&ctxt.idt);
//
// Just in case the asm code got us here with the SS, DS, or ES
// out of sync with the GDT, update them.
//
    loadsegment(ss, __KERNEL_DS);
    loadsegment(ds, __USER_DS);
    loadsegment(es, __USER_DS);
//
// Restore percpu access.  Percpu access can happen in exception
// handlers or in complicated helpers like load_gs_index().
//

    wrmsrq(MSR_GS_BASE, ctxt.kernelmode_gs_base);
//
// Reinitialize FRED to ensure the FRED MSRs contain the same values
// as before hibernation.
//
// Note, the setup of FRED RSPs requires access to percpu data
// structures.  Therefore, FRED reinitialization can only occur after
// the percpu access pointer (i.e., MSR_GS_BASE) is restored.
//
    if (ctxt.cr4 & X86_CR4_FRED) {
    cpu_init_fred_exceptions();
    cpu_init_fred_rsps();
    }

    loadsegment(fs, __KERNEL_PERCPU);

// Restore the TSS, RO GDT, LDT, and usermode-relevant MSRs.
    fix_processor_context();
//
// Now that we have descriptor tables fully restored and working
// exception handling, restore the usermode segments.
//

    loadsegment(ds, ctxt.es);
    loadsegment(es, ctxt.es);
    loadsegment(fs, ctxt.fs);
    load_gs_index(ctxt.gs);
//
// Restore FSBASE and GSBASE after restoring the selectors, since
// restoring the selectors clobbers the bases.  Keep in mind
// that MSR_KERNEL_GS_BASE is horribly misnamed.
//
    wrmsrq(MSR_FS_BASE, ctxt.fs_base);
    wrmsrq(MSR_KERNEL_GS_BASE, ctxt.usermode_gs_base);

    loadsegment(gs, ctxt.gs);

    do_fpu_end();
    tsc_verify_tsc_adjust(true);
    x86_platform.restore_sched_clock_state();
    cache_bp_restore();
    perf_restore_debug_store();
    c = &cpu_data(smp_processor_id());
    if (cpu_has(c, X86_FEATURE_MSR_IA32_FEAT_CTL))
    init_ia32_feat_ctl(c);
    microcode_bsp_resume();
//
// This needs to happen after the microcode has been updated upon resume
// because some of the MSRs are "emulated" in microcode.
//
    msr_restore_context(ctxt);
    }
// Needed by apm.c
#[no_mangle]
pub unsafe extern "C" fn restore_processor_state() -> void notrace {
    void notrace restore_processor_state(void)
    {
    __restore_processor_state(&saved_context);
    }

    EXPORT_SYMBOL(restore_processor_state);

#[no_mangle]
unsafe extern "C" fn resume_play_dead() -> void __noreturn {
    static void __noreturn resume_play_dead(void)
    {
    play_dead_common();
    tboot_shutdown(TB_SHUTDOWN_WFS);
    hlt_play_dead();
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_resume_nonboot_cpu_disable() -> c_int {
    int hibernate_resume_nonboot_cpu_disable(void)
    {
    void (*play_dead)(void) = smp_ops.play_dead;
    int ret;
//
// Ensure that MONITOR/MWAIT will not be used in the "play dead" loop
// during hibernate image restoration, because it is likely that the
// monitored address will be actually written to at that time and then
// the "dead" CPU will attempt to execute instructions again, but the
// address in its instruction pointer may not be possible to resolve
// any more at that point (the page tables used by it previously may
// have been overwritten by hibernate image data).
//
// First, make sure that we wake up all the potentially disabled SMT
// threads which have been initially brought up and then put into
// mwait/cpuidle sleep.
// Those will be put to proper (not interfering with hibernation
// resume) sleep afterwards, and the resumed kernel will decide itself
// what to do with them.
//
    ret = cpuhp_smt_enable();
    if (ret)
    return ret;
    smp_ops.play_dead = resume_play_dead;
    ret = freeze_secondary_cpus(0);
    smp_ops.play_dead = play_dead;
    return ret;
    }

//
// When bsp_check() is called in hibernate and suspend, cpu hotplug
// is disabled already. So it's unnecessary to handle race condition between
// cpumask query and cpu hotplug.
//
#[no_mangle]
unsafe extern "C" fn bsp_check() -> c_int {
    static int bsp_check(void)
    {
    if (cpumask_first(cpu_online_mask) != 0) {
    pr_warn("CPU0 is offline.\n");
    return -ENODEV;
    }
    return 0;
    }
    static int bsp_pm_callback(struct notifier_block *nb, unsigned long action,
    void *ptr)
    {
    let mut ret: c_int = 0;
    switch (action) {
    case PM_SUSPEND_PREPARE:
    case PM_HIBERNATION_PREPARE:
    ret = bsp_check();
    break;
    default:
    break;
    }
    return notifier_from_errno(ret);
    }
#[no_mangle]
unsafe extern "C" fn bsp_pm_check_init() -> int __init {
    static int __init bsp_pm_check_init(void)
    {
//
// Set this bsp_pm_callback as lower priority than
// cpu_hotplug_pm_callback. So cpu_hotplug_pm_callback will be called
// earlier to disable cpu hotplug before bsp online check.
//
    pm_notifier(bsp_pm_callback, -INT_MAX);
    return 0;
    }
    core_initcall(bsp_pm_check_init);
#[no_mangle]
unsafe extern "C" fn msr_build_context(msr_id: *const u32, num: c_int) -> c_int {
    static int msr_build_context(const u32 *msr_id, const int num)
    {
    struct saved_msrs *saved_msrs = &saved_context.saved_msrs;
    struct saved_msr *msr_array;
    int total_num;
    int i, j;
    total_num = saved_msrs.num + num;
    msr_array = kmalloc_objs(struct saved_msr, total_num);
    if (!msr_array) {
    pr_err("x86/pm: Can not allocate memory to save/restore MSRs during suspend.\n");
    return -ENOMEM;
    }
    if (saved_msrs.array) {
//
// Multiple callbacks can invoke this function, so copy any
// MSR save requests from previous invocations.
//
    memcpy(msr_array, saved_msrs.array,
    sizeof(struct saved_msr) * saved_msrs.num);
    kfree(saved_msrs.array);
    }
    for (i = saved_msrs.num, j = 0; i < total_num; i++, j++) {
    u64 dummy;
    msr_array[i].info.msr_no	= msr_id[j];
    msr_array[i].valid		= !rdmsrq_safe(msr_id[j], &dummy);
    msr_array[i].info.reg.q		= 0;
    }
    saved_msrs.num   = total_num;
    saved_msrs.array = msr_array;
    return 0;
    }
//
// The following sections are a quirk framework for problematic BIOSen:
// Sometimes MSRs are modified by the BIOSen after suspended to
// RAM, this might cause unexpected behavior after wakeup.
// Thus we save/restore these specified MSRs across suspend/resume
// in order to work around it.
//
// For any further problematic BIOSen/platforms,
// please add your own function similar to msr_initialize_bdw.
//
#[no_mangle]
unsafe extern "C" fn msr_initialize_bdw(d: *const dmi_system_id) -> c_int {
    static int msr_initialize_bdw(const struct dmi_system_id *d)
    {
// Add any extra MSR ids into this array.
    u32 bdw_msr_id[] = { MSR_IA32_THERM_CONTROL };
    pr_info("x86/pm: %s detected, MSR saving is needed during suspending.\n", d.ident);
    return msr_build_context(bdw_msr_id, ARRAY_SIZE(bdw_msr_id));
    }
    static const struct dmi_system_id msr_save_dmi_table[] = {
    {
    .callback = msr_initialize_bdw,
    .ident = "BROADWELL BDX_EP",
    .matches = {
    DMI_MATCH(DMI_PRODUCT_NAME, "GRANTLEY"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "E63448-400"),
    },
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn msr_save_cpuid_features(c: *const x86_cpu_id) -> c_int {
    static int msr_save_cpuid_features(const struct x86_cpu_id *c)
    {
    u32 cpuid_msr_id[] = {
    MSR_AMD64_CPUID_FN_1,
    };
    pr_info("x86/pm: family %#hx cpu detected, MSR saving is needed during suspending.\n",
    c.family);
    return msr_build_context(cpuid_msr_id, ARRAY_SIZE(cpuid_msr_id));
    }
    static const struct x86_cpu_id msr_save_cpu_table[] = {
    X86_MATCH_VENDOR_FAM(AMD, 0x15, &msr_save_cpuid_features),
    X86_MATCH_VENDOR_FAM(AMD, 0x16, &msr_save_cpuid_features),
    {}
    };
    typedef int (*pm_cpu_match_t)(const struct x86_cpu_id *);
#[no_mangle]
unsafe extern "C" fn pm_cpu_check(c: *const x86_cpu_id) -> c_int {
    static int pm_cpu_check(const struct x86_cpu_id *c)
    {
    const struct x86_cpu_id *m;
    let mut ret: c_int = 0;
    m = x86_match_cpu(msr_save_cpu_table);
    if (m) {
    pm_cpu_match_t fn;
    fn = (pm_cpu_match_t)m.driver_data;
    ret = fn(m);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm_save_spec_msr() {
    static void pm_save_spec_msr(void)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_enumeration {
    pub msr_no: u32,
    pub feature: u32,
    } msr_enum[] = {
    { MSR_IA32_SPEC_CTRL,	 X86_FEATURE_MSR_SPEC_CTRL },
    { MSR_IA32_TSX_CTRL,	 X86_FEATURE_MSR_TSX_CTRL },
    { MSR_TSX_FORCE_ABORT,	 X86_FEATURE_TSX_FORCE_ABORT },
    { MSR_IA32_MCU_OPT_CTRL, X86_FEATURE_SRBDS_CTRL },
    { MSR_AMD64_LS_CFG,	 X86_FEATURE_LS_CFG_SSBD },
    { MSR_AMD64_DE_CFG,	 X86_FEATURE_LFENCE_RDTSC },
}

    int i;
    for (i = 0; i < ARRAY_SIZE(msr_enum); i++) {
    if (boot_cpu_has(msr_enum[i].feature))
    msr_build_context(&msr_enum[i].msr_no, 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn pm_check_save_msr() -> c_int {
    static int pm_check_save_msr(void)
    {
    dmi_check_system(msr_save_dmi_table);
    pm_cpu_check(msr_save_cpu_table);
    pm_save_spec_msr();
    return 0;
    }
    device_initcall(pm_check_save_msr);
