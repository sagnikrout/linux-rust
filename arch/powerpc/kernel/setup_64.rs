//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/setup_64.c
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
// Common boot and setup code.
//
// Copyright (C) 2001 PPC64 Team, IBM Corp
//

    int spinning_secondaries;
    u64 ppc64_pft_size;
    struct ppc64_caches ppc64_caches = {
    .l1d = {
    .block_size = 0x40,
    .log_block_size = 6,
    },
    .l1i = {
    .block_size = 0x40,
    .log_block_size = 6
    },
    };
    EXPORT_SYMBOL_GPL(ppc64_caches);

#[no_mangle]
pub unsafe extern "C" fn setup_tlb_core_data() -> void __init {
    void __init setup_tlb_core_data(void)
    {
    int cpu;
    BUILD_BUG_ON(offsetof(struct tlb_core_data, lock) != 0);
    for_each_possible_cpu(cpu) {
    let mut first: c_int = cpu_first_thread_sibling(cpu);
//
// If we boot via kdump on a non-primary thread,
// make sure we point at the thread that actually
// set up this TLB.
//
    if (cpu_first_thread_sibling(boot_cpuid) == first)
    first = boot_cpuid;
    paca_ptrs[cpu].tcd_ptr = &paca_ptrs[first].tcd;
//
// If we have threads, we need either tlbsrx.
// or e6500 tablewalk mode, or else TLB handlers
// will be racy and could produce duplicate entries.
// Should we panic instead?
//
    WARN_ONCE(smt_enabled_at_boot >= 2 &&
    book3e_htw_mode != PPC_HTW_E6500,
    "%s: unsupported MMU configuration\n", __func__);
    }
    }

    static char *smt_enabled_cmdline;
// Look for ibm,smt-enabled OF option
#[no_mangle]
pub unsafe extern "C" fn check_smt_enabled() -> void __init {
    void __init check_smt_enabled(void)
    {
    struct device_node *dn;
    const char *smt_option;
// Default to enabling all threads
    smt_enabled_at_boot = threads_per_core;
// Allow the command line to overrule the OF option
    if (smt_enabled_cmdline) {
    if (!strcmp(smt_enabled_cmdline, "on"))
    smt_enabled_at_boot = threads_per_core;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(smt_enabled_cmdline, _arg: "off")) -> else {
    else if (!strcmp(smt_enabled_cmdline, "off"))
    smt_enabled_at_boot = 0;
    else {
    int smt;
    if (!kstrtoint(smt_enabled_cmdline, 10, &smt))
    smt_enabled_at_boot =
    min(threads_per_core, smt);
    }
    } else {
    dn = of_find_node_by_path("/options");
    if (dn) {
    smt_option = of_get_property(dn, "ibm,smt-enabled",
    core::ptr::null_mut());
    if (smt_option) {
    if (!strcmp(smt_option, "on"))
    smt_enabled_at_boot = threads_per_core;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(smt_option, _arg: "off")) -> else {
    else if (!strcmp(smt_option, "off"))
    smt_enabled_at_boot = 0;
    }
    of_node_put(dn);
    }
    }
    }
// Look for smt-enabled= cmdline option
#[no_mangle]
unsafe extern "C" fn early_smt_enabled(p: *mut c_char) -> int __init {
    static int __init early_smt_enabled(char *p)
    {
    smt_enabled_cmdline = p;
    return 0;
    }
    early_param("smt-enabled", early_smt_enabled);

// Fix up paca fields required for the boot cpu
#[no_mangle]
unsafe extern "C" fn fixup_boot_paca(boot_paca: *mut paca_struct) -> void __init {
    static void __init fixup_boot_paca(struct paca_struct *boot_paca)
    {
// The boot cpu is started
    boot_paca.cpu_start = 1;

//
// Give the early boot machine check stack somewhere to use, use
// half of the init stack. This is a bit hacky but there should not be
// deep stack usage in early init so shouldn't overflow it or overwrite
// things.
//
    boot_paca.mc_emergency_sp = (void *)&init_thread_union +
    (THREAD_SIZE/2);

// Allow percpu accesses to work until we setup percpu data
    boot_paca.data_offset = 0;
// Mark interrupts soft and hard disabled in PACA
    boot_paca.irq_soft_mask = IRQS_DISABLED;
    boot_paca.irq_happened = PACA_IRQ_HARD_DIS;
    WARN_ON(mfmsr() & MSR_EE);
    }
#[no_mangle]
unsafe extern "C" fn configure_exceptions() -> void __init {
    static void __init configure_exceptions(void)
    {
//
// Setup the trampolines from the lowmem exception vectors
// to the kdump kernel when not using a relocatable kernel.
//
    setup_kdump_trampoline();
// Under a PAPR hypervisor, we need hypercalls
    if (firmware_has_feature(FW_FEATURE_SET_MODE)) {
//
// - PR KVM does not support AIL mode interrupts in the host
// while a PR guest is running.
//
// - SCV system call interrupt vectors are only implemented for
// AIL mode interrupts.
//
// - On pseries, AIL mode can only be enabled and disabled
// system-wide so when a PR VM is created on a pseries host,
// all CPUs of the host are set to AIL=0 mode.
//
// - Therefore host CPUs must not execute scv while a PR VM
// exists.
//
// - SCV support can not be disabled dynamically because the
// feature is advertised to host userspace. Disabling the
// facility and emulating it would be possible but is not
// implemented.
//
// - So SCV support is blanket disabled if PR KVM could possibly
// run. That is, PR support compiled in, booting on pseries
// with hash MMU.
//
    if (IS_ENABLED(CONFIG_KVM_BOOK3S_PR_POSSIBLE) && !radix_enabled()) {
    init_task.thread.fscr &= ~FSCR_SCV;
    cur_cpu_spec.cpu_user_features2 &= ~PPC_FEATURE2_SCV;
    }
// Enable AIL if possible
    if (!pseries_enable_reloc_on_exc()) {
    init_task.thread.fscr &= ~FSCR_SCV;
    cur_cpu_spec.cpu_user_features2 &= ~PPC_FEATURE2_SCV;
    }
//
// Tell the hypervisor that we want our exceptions to
// be taken in little endian mode.
//
// We don't call this for big endian as our calling convention
// makes us always enter in BE, and the call may fail under
// some circumstances with kdump.
//

    pseries_little_endian_exceptions();

    } else {
// Set endian mode using OPAL
    if (firmware_has_feature(FW_FEATURE_OPAL))
    opal_configure_cores();
// AIL on native is done in cpu_ready_for_interrupts()
    }
    }
#[no_mangle]
unsafe extern "C" fn cpu_ready_for_interrupts() {
    static void cpu_ready_for_interrupts(void)
    {
//
// Enable AIL if supported, and we are in hypervisor mode. This
// is called once for every processor.
//
// If we are not in hypervisor mode the job is done once for
// the whole partition in configure_exceptions().
//
    if (cpu_has_feature(CPU_FTR_HVMODE)) {
    let mut lpcr: c_ulong = mfspr(SPRN_LPCR);
    let mut new_lpcr: c_ulong = lpcr;
    if (cpu_has_feature(CPU_FTR_ARCH_31)) {
// P10 DD1 does not have HAIL
    if (pvr_version_is(PVR_POWER10) &&
    (mfspr(SPRN_PVR) & 0xf00) == 0x100)
    new_lpcr |= LPCR_AIL_3;
    else
    new_lpcr |= LPCR_HAIL;
    } else if (cpu_has_feature(CPU_FTR_ARCH_207S)) {
    new_lpcr |= LPCR_AIL_3;
    }
    if (new_lpcr != lpcr)
    mtspr(SPRN_LPCR, new_lpcr);
    }
//
// Set HFSCR:TM based on CPU features:
// In the special case of TM no suspend (P9N DD2.1), Linux is
// told TM is off via the dt-ftrs but told to (partially) use
// it via OPAL_REINIT_CPUS_TM_SUSPEND_DISABLED. So HFSCR[TM]
// will be off from dt-ftrs but we need to turn it on for the
// no suspend case.
//
    if (cpu_has_feature(CPU_FTR_HVMODE)) {
    if (cpu_has_feature(CPU_FTR_TM_COMP))
    mtspr(SPRN_HFSCR, mfspr(SPRN_HFSCR) | HFSCR_TM);
    else
    mtspr(SPRN_HFSCR, mfspr(SPRN_HFSCR) & ~HFSCR_TM);
    }
// Set IR and DR in PACA MSR
    get_paca().kernel_msr = MSR_KERNEL;
    }
    let mut spr_default_dscr: c_ulong = 0;
#[no_mangle]
unsafe extern "C" fn record_spr_defaults() -> void __init {
    static void __init record_spr_defaults(void)
    {
    if (early_cpu_has_feature(CPU_FTR_DSCR))
    spr_default_dscr = mfspr(SPRN_DSCR);
    }
//
// Early initialization entry point. This is called by head.S
// with MMU translation disabled. We rely on the "feature" of
// the CPU that ignores the top 2 bits of the address in real
// mode so we can access kernel globals normally provided we
// only toy with things in the RMO region. From here, we do
// some early parsing of the device-tree to setup out MEMBLOCK
// data structures, and allocate & initialize the hash table
// and segment tables so we can start running with translation
// enabled.
//
// It is this function which will call the probe() callback of
// the various platform types and copy the matching one to the
// global ppc_md structure. Your platform can eventually do
// some very early initializations from the probe() routine, but
// this is not recommended, be very careful as, for example, the
// device-tree is not accessible via normal means at this point.
//
#[no_mangle]
pub unsafe extern "C" fn early_setup(dt_ptr: c_ulong) -> void __init {
    void __init early_setup(unsigned long dt_ptr)
    {
    static __initdata struct paca_struct boot_paca;
// -------- printk is _NOT_ safe to use here ! -------
//
// Assume we're on cpu 0 for now.
//
// We need to load a PACA very early for a few reasons.
//
// The stack protector canary is stored in the paca, so as soon as we
// call any stack protected code we need r13 pointing somewhere valid.
//
// If we are using kcov it will call in_task() in its instrumentation,
// which relies on the current task from the PACA.
//
// dt_cpu_ftrs_init() calls into generic OF/fdt code, as well as
// printk(), which can trigger both stack protector and kcov.
//
// percpu variables and spin locks also use the paca.
//
// So set up a temporary paca. It will be replaced below once we know
// what CPU we are on.
//
    initialise_paca(&boot_paca, 0);
    fixup_boot_paca(&boot_paca);
    WARN_ON(local_paca);
    setup_paca(&boot_paca); /* install the paca into registers */
// -------- printk is now safe to use -------
    if (IS_ENABLED(CONFIG_PPC_BOOK3S_64) && (mfmsr() & MSR_HV))
    enable_machine_check();
// Try new device tree based feature discovery ...
    if (!dt_cpu_ftrs_init(__va(dt_ptr)))
// Otherwise use the old style CPU table
    identify_cpu(0, mfspr(SPRN_PVR));
// Enable early debugging if any specified (see udbg.h)
    udbg_early_init();
    udbg_printf(" . %s(), dt_ptr: 0x%lx\n", __func__, dt_ptr);
//
// Do early initialization using the flattened device
// tree, such as retrieving the physical memory map or
// calculating/retrieving the hash table size, discover
// boot_cpuid and boot_cpu_hwid.
//
    early_init_devtree(__va(dt_ptr));
    allocate_paca_ptrs();
    allocate_paca(boot_cpuid);
    set_hard_smp_processor_id(boot_cpuid, boot_cpu_hwid);
    fixup_boot_paca(paca_ptrs[boot_cpuid]);
    setup_paca(paca_ptrs[boot_cpuid]); /* install the paca into registers */
// smp_processor_id() now reports boot_cpuid

    task_thread_info(current).cpu = boot_cpuid; // fix task_cpu(current)

//
// Configure exception handlers. This include setting up trampolines
// if needed, setting exception endian mode, etc...
//
    configure_exceptions();
//
// Configure Kernel Userspace Protection. This needs to happen before
// feature fixups for platforms that implement this using features.
//
    setup_kup();
// Apply all the dynamic patching
    apply_feature_fixups();
    setup_feature_keys();
// Initialize the hash table or TLB handling
    early_init_mmu();
    early_ioremap_setup();
//
// After firmware and early platform setup code has set things up,
// we note the SPR values for configurable control/performance
// registers, and use those as initial defaults.
//
    record_spr_defaults();
//
// At this point, we can let interrupts switch to virtual mode
// (the MMU has been setup), so adjust the MSR in the PACA to
// have IR and DR set and enable AIL if it exists
//
    cpu_ready_for_interrupts();
//
// We enable ftrace here, but since we only support DYNAMIC_FTRACE, it
// will only actually get enabled on the boot cpu much later once
// ftrace itself has been initialized.
//
    this_cpu_enable_ftrace();
    udbg_printf(" <- %s()\n", __func__);

//
// This needs to be done *last* (after the above udbg_printf() even)
//
// Right after we return from this function, we turn on the MMU
// which means the real-mode access trick that btext does will
// no longer work, it needs to switch to using a real MMU
// mapping. This call will ensure that it does
//
    btext_map();

    }

#[no_mangle]
pub unsafe extern "C" fn early_setup_secondary() {
    void early_setup_secondary(void)
    {
// Mark interrupts disabled in PACA
    irq_soft_mask_set(IRQS_DISABLED);
// Initialize the hash table or TLB handling
    early_init_mmu_secondary();
// Perform any KUP setup that is per-cpu
    setup_kup();
//
// At this point, we can let interrupts switch to virtual mode
// (the MMU has been setup), so adjust the MSR in the PACA to
// have IR and DR set.
//
    cpu_ready_for_interrupts();
    }

#[no_mangle]
pub unsafe extern "C" fn panic_smp_self_stop() -> void __noreturn {
    void __noreturn panic_smp_self_stop(void)
    {
    hard_irq_disable();
    spin_begin();
    while (1)
    spin_cpu_relax();
    }

#[no_mangle]
unsafe extern "C" fn use_spinloop() -> bool {
    static bool use_spinloop(void)
    {
    if (IS_ENABLED(CONFIG_PPC_BOOK3S)) {
//
// See comments in head_64.S -- not all platforms insert
// secondaries at __secondary_hold and wait at the spin
// loop.
//
    if (firmware_has_feature(FW_FEATURE_OPAL))
    return false;
    return true;
    }
//
// When book3e boots from kexec, the ePAPR spin table does
// not get used.
//
    return of_property_read_bool(of_chosen, "linux,booted-from-kexec");
    }
#[no_mangle]
pub unsafe extern "C" fn smp_release_cpus() {
    void smp_release_cpus(void)
    {
    unsigned long *ptr;
    int i;
    if (!use_spinloop())
    return;
// All secondary cpus are spinning on a common spinloop, release them
// all now so they can start to spin on their individual paca
// spinloops. For non SMP kernels, the secondary cpus never get out
// of the common spinloop.
//
    ptr  = (unsigned long *)((unsigned long)&__secondary_hold_spinloop
    - PHYSICAL_START);
// ptr = ppc_function_entry(generic_secondary_smp_init);
// And wait a bit for them to catch up
    for (i = 0; i < 100000; i++) {
    mb();
    HMT_low();
    if (spinning_secondaries == 0)
    break;
    udelay(1);
    }
    pr_debug("spinning_secondaries = %d\n", spinning_secondaries);
    }

//
// Initialize some remaining members of the ppc64_caches and systemcfg
// structures
// (at least until we get rid of them completely). This is mostly some
// cache informations about the CPU that will be used by cache flush
// routines and/or provided to userland
//
    static void __init init_cache_info(struct ppc_cache_info *info, u32 size, u32 lsize,
    u32 bsize, u32 sets)
    {
    info.size = size;
    info.sets = sets;
    info.line_size = lsize;
    info.block_size = bsize;
    info.log_block_size = __ilog2(bsize);
    if (bsize)
    info.blocks_per_page = PAGE_SIZE / bsize;
    else
    info.blocks_per_page = 0;
    if (sets == 0)
    info.assoc = 0xffff;
    else
    info.assoc = size / (sets * lsize);
    }
    static bool __init parse_cache_info(struct device_node *np,
    bool icache,
    struct ppc_cache_info *info)
    {
    static const char *ipropnames[] __initdata = {
    "i-cache-size",
    "i-cache-sets",
    "i-cache-block-size",
    "i-cache-line-size",
    };
    static const char *dpropnames[] __initdata = {
    "d-cache-size",
    "d-cache-sets",
    "d-cache-block-size",
    "d-cache-line-size",
    };
    const char **propnames = icache ? ipropnames : dpropnames;
    const __be32 *sizep, *lsizep, *bsizep, *setsp;
    u32 size, lsize, bsize, sets;
    let mut success: bool = true;
    size = 0;
    sets = -1u;
    lsize = bsize = cur_cpu_spec.dcache_bsize;
    sizep = of_get_property(np, propnames[0], core::ptr::null_mut());
    if (sizep != core::ptr::null_mut())
    size = be32_to_cpu(*sizep);
    setsp = of_get_property(np, propnames[1], core::ptr::null_mut());
    if (setsp != core::ptr::null_mut())
    sets = be32_to_cpu(*setsp);
    bsizep = of_get_property(np, propnames[2], core::ptr::null_mut());
    lsizep = of_get_property(np, propnames[3], core::ptr::null_mut());
    if (bsizep == core::ptr::null_mut())
    bsizep = lsizep;
    if (lsizep == core::ptr::null_mut())
    lsizep = bsizep;
    if (lsizep != core::ptr::null_mut())
    lsize = be32_to_cpu(*lsizep);
    if (bsizep != core::ptr::null_mut())
    bsize = be32_to_cpu(*bsizep);
    if (sizep == core::ptr::null_mut() || bsizep == core::ptr::null_mut() || lsizep == core::ptr::null_mut())
    success = false;
//
// OF is weird .. it represents fully associative caches
// as "1 way" which doesn't make much sense and doesn't
// leave room for direct mapped. We'll assume that 0
// in OF means direct mapped for that reason.
//
    if (sets == 1)
    sets = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: sets ==) -> else {
    else if (sets == 0)
    sets = 1;
    init_cache_info(info, size, lsize, bsize, sets);
    return success;
    }
#[no_mangle]
pub unsafe extern "C" fn initialize_cache_info() -> void __init {
    void __init initialize_cache_info(void)
    {
    struct device_node *cpu = core::ptr::null_mut(), *l2, *l3 = core::ptr::null_mut();
    u32 pvr;
//
// All shipping POWER8 machines have a firmware bug that
// puts incorrect information in the device-tree. This will
// be (hopefully) fixed for future chips but for now hard
// code the values if we are running on one of these
//
    pvr = PVR_VER(mfspr(SPRN_PVR));
    if (pvr == PVR_POWER8 || pvr == PVR_POWER8E ||
    pvr == PVR_POWER8NVL) {
// size    lsize   blk  sets
    init_cache_info(&ppc64_caches.l1i, 0x8000,   128,  128, 32);
    init_cache_info(&ppc64_caches.l1d, 0x10000,  128,  128, 64);
    init_cache_info(&ppc64_caches.l2,  0x80000,  128,  0,   512);
    init_cache_info(&ppc64_caches.l3,  0x800000, 128,  0,   8192);
    } else
    cpu = of_find_node_by_type(core::ptr::null_mut(), "cpu");
//
// We're assuming *all* of the CPUs have the same
// d-cache and i-cache sizes... -Peter
//
    if (cpu) {
    if (!parse_cache_info(cpu, false, &ppc64_caches.l1d))
    pr_warn("Argh, can't find dcache properties !\n");
    if (!parse_cache_info(cpu, true, &ppc64_caches.l1i))
    pr_warn("Argh, can't find icache properties !\n");
//
// Try to find the L2 and L3 if any. Assume they are
// unified and use the D-side properties.
//
    l2 = of_find_next_cache_node(cpu);
    of_node_put(cpu);
    if (l2) {
    parse_cache_info(l2, false, &ppc64_caches.l2);
    l3 = of_find_next_cache_node(l2);
    of_node_put(l2);
    }
    if (l3) {
    parse_cache_info(l3, false, &ppc64_caches.l3);
    of_node_put(l3);
    }
    }
// For use by binfmt_elf
    dcache_bsize = ppc64_caches.l1d.block_size;
    icache_bsize = ppc64_caches.l1i.block_size;
    cur_cpu_spec.dcache_bsize = dcache_bsize;
    cur_cpu_spec.icache_bsize = icache_bsize;
    }
//
// This returns the limit below which memory accesses to the linear
// mapping are guarnateed not to cause an architectural exception (e.g.,
// TLB or SLB miss fault).
//
// This is used to allocate PACAs and various interrupt stacks that
// that are accessed early in interrupt handlers that must not cause
// re-entrant interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn ppc64_bolted_size() -> __init u64 {
    __init u64 ppc64_bolted_size(void)
    {

// Freescale BookE bolts the entire linear mapping
    return linear_map_top;

// BookS radix, does not take faults on linear mapping
    if (early_radix_enabled())
    return ULONG_MAX;
// BookS hash, the first segment is bolted
    if (early_mmu_has_feature(MMU_FTR_1T_SEGMENT))
    return 1UL << SID_SHIFT_1T;
    return 1UL << SID_SHIFT;

    }
#[no_mangle]
unsafe extern "C" fn alloc_stack(limit: c_ulong, cpu: c_int) -> *mut void __init {
    static void *__init alloc_stack(unsigned long limit, int cpu)
    {
    void *ptr;
    BUILD_BUG_ON(STACK_INT_FRAME_SIZE % 16);
    ptr = memblock_alloc_try_nid(THREAD_SIZE, THREAD_ALIGN,
    MEMBLOCK_LOW_LIMIT, limit,
    early_cpu_to_node(cpu));
    if (!ptr)
    panic("cannot allocate stacks");
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn irqstack_early_init() -> void __init {
    void __init irqstack_early_init(void)
    {
    let mut limit: u64 = ppc64_bolted_size();
    unsigned int i;
//
// Interrupt stacks must be in the first segment since we
// cannot afford to take SLB misses on them. They are not
// accessed in realmode.
//
    for_each_possible_cpu(i) {
    softirq_ctx[i] = alloc_stack(limit, i);
    hardirq_ctx[i] = alloc_stack(limit, i);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn exc_lvl_early_init() -> void __init {
    void __init exc_lvl_early_init(void)
    {
    unsigned int i;
    for_each_possible_cpu(i) {
    void *sp;
    sp = alloc_stack(ULONG_MAX, i);
    critirq_ctx[i] = sp;
    paca_ptrs[i].crit_kstack = sp + THREAD_SIZE;
    sp = alloc_stack(ULONG_MAX, i);
    dbgirq_ctx[i] = sp;
    paca_ptrs[i].dbg_kstack = sp + THREAD_SIZE;
    sp = alloc_stack(ULONG_MAX, i);
    mcheckirq_ctx[i] = sp;
    paca_ptrs[i].mc_kstack = sp + THREAD_SIZE;
    }
    if (cpu_has_feature(CPU_FTR_DEBUG_LVL_EXC))
    patch_exception(0x040, exc_debug_debug_book3e);
    }

//
// Stack space used when we detect a bad kernel stack pointer, and
// early in SMP boots before relocation is enabled. Exclusive emergency
// stack for machine checks.
//
#[no_mangle]
pub unsafe extern "C" fn emergency_stack_init() -> void __init {
    void __init emergency_stack_init(void)
    {
    u64 limit, mce_limit;
    unsigned int i;
//
// Emergency stacks must be under 256MB, we cannot afford to take
// SLB misses on them. The ABI also requires them to be 128-byte
// aligned.
//
// Since we use these as temporary stacks during secondary CPU
// bringup, machine check, system reset, and HMI, we need to get
// at them in real mode. This means they must also be within the RMO
// region.
//
// The IRQ stacks allocated elsewhere in this file are zeroed and
// initialized in kernel/irq.c. These are initialized here in order
// to have emergency stacks available as early as possible.
//
    limit = mce_limit = min(ppc64_bolted_size(), ppc64_rma_size);
//
// Machine check on pseries calls rtas, but can't use the static
// rtas_args due to a machine check hitting while the lock is held.
// rtas args have to be under 4GB, so the machine check stack is
// limited to 4GB so args can be put on stack.
//
    if (firmware_has_feature(FW_FEATURE_LPAR) && mce_limit > SZ_4G)
    mce_limit = SZ_4G;
    for_each_possible_cpu(i) {
    paca_ptrs[i].emergency_sp = alloc_stack(limit, i) + THREAD_SIZE;

// emergency stack for NMI exception handling.
    paca_ptrs[i].nmi_emergency_sp = alloc_stack(limit, i) + THREAD_SIZE;
// emergency stack for machine check exception handling.
    paca_ptrs[i].mc_emergency_sp = alloc_stack(mce_limit, i) + THREAD_SIZE;

    }
    }

#[no_mangle]
unsafe extern "C" fn pcpu_cpu_distance(from: c_uint, to: c_uint) -> c_int {
    static int pcpu_cpu_distance(unsigned int from, unsigned int to)
    {
    if (early_cpu_to_node(from) == early_cpu_to_node(to))
    return LOCAL_DISTANCE;
    else
    return REMOTE_DISTANCE;
    }
#[no_mangle]
unsafe extern "C" fn pcpu_cpu_to_node(cpu: c_int) -> __init int {
    static __init int pcpu_cpu_to_node(int cpu)
    {
    return early_cpu_to_node(cpu);
    }
    unsigned long __per_cpu_offset[NR_CPUS] __read_mostly;
    EXPORT_SYMBOL(__per_cpu_offset);
    DEFINE_STATIC_KEY_FALSE(__percpu_first_chunk_is_paged);
#[no_mangle]
pub unsafe extern "C" fn setup_per_cpu_areas() -> void __init {
    void __init setup_per_cpu_areas(void)
    {
    let mut dyn_size: usize = PERCPU_MODULE_RESERVE + PERCPU_DYNAMIC_RESERVE;
    size_t atom_size;
    unsigned long delta;
    unsigned int cpu;
    let mut rc: c_int = -EINVAL;
//
// BookE and BookS radix are historical values and should be revisited.
//
    if (IS_ENABLED(CONFIG_PPC_BOOK3E_64)) {
    atom_size = SZ_1M;
    } else if (radix_enabled()) {
    atom_size = PAGE_SIZE;
    } else if (IS_ENABLED(CONFIG_PPC_64S_HASH_MMU)) {
//
// Linear mapping is one of 4K, 1M and 16M.  For 4K, no need
// to group units.  For larger mappings, use 1M atom which
// should be large enough to contain a number of units.
//
    if (mmu_linear_psize == MMU_PAGE_4K)
    atom_size = PAGE_SIZE;
    else
    atom_size = SZ_1M;
    }
    if (pcpu_chosen_fc != PCPU_FC_PAGE) {
    rc = pcpu_embed_first_chunk(0, dyn_size, atom_size, pcpu_cpu_distance,
    pcpu_cpu_to_node);
    if (rc)
    pr_warn("PERCPU: %s allocator failed (%d), "
    "falling back to page size\n",
    pcpu_fc_names[pcpu_chosen_fc], rc);
    }
    if (rc < 0)
    rc = pcpu_page_first_chunk(0, pcpu_cpu_to_node);
    if (rc < 0)
    panic("cannot initialize percpu area (err=%d)", rc);
    static_key_enable(&__percpu_first_chunk_is_paged.key);
    delta = (unsigned long)pcpu_base_addr - (unsigned long)__per_cpu_start;
    for_each_possible_cpu(cpu) {
    __per_cpu_offset[cpu] = delta + pcpu_unit_offsets[cpu];
    paca_ptrs[cpu].data_offset = __per_cpu_offset[cpu];
    }
    }

#[no_mangle]
pub unsafe extern "C" fn memory_block_size_bytes() -> c_ulong {
    unsigned long memory_block_size_bytes(void)
    {
    if (ppc_md.memory_block_size)
    return ppc_md.memory_block_size();
    return MIN_MEMORY_BLOCK_SIZE;
    }

    struct ppc_pci_io ppc_pci_io;
    EXPORT_SYMBOL(ppc_pci_io);

#[no_mangle]
pub unsafe extern "C" fn hw_nmi_get_sample_period(watchdog_thresh: c_int) -> u64 {
    u64 hw_nmi_get_sample_period(int watchdog_thresh)
    {
    return ppc_proc_freq * watchdog_thresh;
    }

//
// The perf based hardlockup detector breaks PMU event based branches, so
// disable it by default. Book3S has a soft-nmi hardlockup detector based
// on the decrementer interrupt, so it does not suffer from this problem.
//
// It is likely to get false positives in KVM guests, so disable it there
// by default too. PowerVM will not stop or arbitrarily oversubscribe
// CPUs, but give a minimum regular allotment even with SPLPAR, so enable
// the detector for non-KVM guests, assume PowerVM.
//
#[no_mangle]
unsafe extern "C" fn disable_hardlockup_detector() -> int __init {
    static int __init disable_hardlockup_detector(void)
    {

    hardlockup_detector_disable();

    if (firmware_has_feature(FW_FEATURE_LPAR)) {
    check_kvm_guest();
    if (is_kvm_guest())
    hardlockup_detector_disable();
    }

    return 0;
    }
    early_initcall(disable_hardlockup_detector);
