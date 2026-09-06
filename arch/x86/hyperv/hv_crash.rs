//! Automatically rewritten from C to Rust
//! Source: arch/x86/hyperv/hv_crash.c
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
// X86 specific Hyper-V root partition kdump/crash support module
//
// Copyright (C) 2025, Microsoft, Inc.
//
// This module implements hypervisor RAM collection into vmcore for both
// cases of the hypervisor crash and Linux root crash. Hyper-V implements
// a disable hypercall with a 32bit protected mode ABI callback. This
// mechanism must be used to unlock hypervisor RAM. Since the hypervisor RAM
// is already mapped in Linux, it is automatically collected into Linux vmcore,
// and can be examined by the crash command (raw RAM dump) or windbg.
//
// At a high level:
//
// Hypervisor Crash:
// Upon crash, hypervisor goes into an emergency minimal dispatch loop, a
// restrictive mode with very limited hypercall and MSR support. Each cpu
// then injects NMIs into root vcpus. A shared page is used to check
// by Linux in the NMI handler if the hypervisor has crashed. This shared
// page is setup in hv_root_crash_init during boot.
//
// Linux Crash:
// In case of Linux crash, the callback hv_crash_stop_other_cpus will send
// NMIs to all cpus, then proceed to the crash_nmi_callback where it waits
// for all cpus to be in NMI.
//
// NMI Handler (upon quorum):
// Eventually, in both cases, all cpus will end up in the NMI handler.
// Hyper-V requires the disable hypervisor must be done from the BSP. So
// the BSP NMI handler saves current context, does some fixups and makes
// the hypercall to disable the hypervisor, ie, devirtualize. Hypervisor
// at that point will suspend all vcpus (except the BSP), unlock all its
// RAM, and return to Linux at the 32bit mode entry RIP.
//
// Linux 32bit entry trampoline will then restore long mode and call C
// function here to restore context and continue execution to crash kexec.
//

    bool hv_crash_enabled;
    EXPORT_SYMBOL_GPL(hv_crash_enabled);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_crash_ctxt {
    pub rsp: c_ulong,
    pub cr0: c_ulong,
    pub cr2: c_ulong,
    pub cr4: c_ulong,
    pub cr8: c_ulong,
    pub cs: u16,
    pub ss: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub gdt_fill: u16,
    pub gdtr: desc_ptr,
    pub idt_fill: [c_char; 6],
    pub idtr: desc_ptr,
    pub gsbase: u64,
    pub efer: u64,
    pub pat: u64,
}

    static struct hv_crash_ctxt hv_crash_ctxt;
// Shared hypervisor page that contains crash dump area we peek into.
// NB: windbg looks for "hv_cda" symbol so don't change it.
//
    static struct hv_crashdump_area *hv_cda;
    static u32 trampoline_pa, devirt_arg;
    static atomic_t crash_cpus_wait;
    static void *hv_crash_ptpgs[4];
    static bool hv_has_crashed, lx_has_crashed;
#[no_mangle]
unsafe extern "C" fn hv_panic_timeout_reboot() -> void __noreturn {
    static void __noreturn hv_panic_timeout_reboot(void)
    {
pub const PANIC_TIMER_STEP: c_int = 100;
    if (panic_timeout > 0) {
    int i;
    for (i = 0; i < panic_timeout * 1000; i += PANIC_TIMER_STEP)
    mdelay(PANIC_TIMER_STEP);
    }
    if (panic_timeout)
    native_wrmsrq(HV_X64_MSR_RESET, 1);    /* get hyp to reboot */
    for (;;)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn hv_crash_restore_tss() {
    static void hv_crash_restore_tss(void)
    {
    load_TR_desc();
    }
#[no_mangle]
unsafe extern "C" fn hv_crash_clear_kernpt() {
    static void hv_crash_clear_kernpt(void)
    {
    pgd_t *pgd;
    p4d_t *p4d;
// Clear entry so it's not confusing to someone looking at the core
    pgd = pgd_offset_k(trampoline_pa);
    p4d = p4d_offset(pgd, trampoline_pa);
    native_p4d_clear(p4d);
    }
#[no_mangle]
unsafe extern "C" fn hv_crash_handle() -> void __noreturn {
    static void __noreturn hv_crash_handle(void)
    {
    hv_crash_restore_tss();
    hv_crash_clear_kernpt();
// we are now fully in devirtualized normal kernel mode
    __crash_kexec(core::ptr::null_mut());
    hv_panic_timeout_reboot();
    }
//
// __naked functions do not permit function calls, not even to __always_inline
// functions that only contain asm() blocks themselves. So use a macro instead.
//

#[no_mangle]
pub unsafe extern "C" fn volatile("c"(msr): "wrmsr" ::, _arg: "a"((u32)val), "memory": "d"((u32)(val >> 32)) :) -> asm {
    asm volatile("wrmsr" :: "c"(msr), "a"((u32)val), "d"((u32)(val >> 32)) : "memory")
//
// This is the C entry point from the asm glue code after the disable hypercall.
// We enter here in IA32-e long mode, ie, full 64bit mode running on kernel
// page tables with our below 4G page identity mapped, but using a temporary
// GDT. ds/fs/gs/es are null. ss is not usable. bp is null. stack is not
// available. We restore kernel GDT, and rest of the context, and continue
// to kexec.
//
#[no_mangle]
unsafe extern "C" fn hv_crash_c_entry() -> void __naked {
    static void __naked hv_crash_c_entry(void)
    {
// first thing, restore kernel gdt
    asm volatile("lgdt %0" : : "m" (hv_crash_ctxt.gdtr));
    asm volatile("movw %0, %%ss\n\t"
    "movq %1, %%rsp"
    :: "m"(hv_crash_ctxt.ss), "m"(hv_crash_ctxt.rsp));
    asm volatile("movw %0, %%ds" : : "m"(hv_crash_ctxt.ds));
    asm volatile("movw %0, %%es" : : "m"(hv_crash_ctxt.es));
    asm volatile("movw %0, %%fs" : : "m"(hv_crash_ctxt.fs));
    asm volatile("movw %0, %%gs" : : "m"(hv_crash_ctxt.gs));
    hv_wrmsr(MSR_IA32_CR_PAT, hv_crash_ctxt.pat);
    asm volatile("movq %0, %%cr0" : : "r"(hv_crash_ctxt.cr0));
    asm volatile("movq %0, %%cr8" : : "r"(hv_crash_ctxt.cr8));
    asm volatile("movq %0, %%cr4" : : "r"(hv_crash_ctxt.cr4));
    asm volatile("movq %0, %%cr2" : : "r"(hv_crash_ctxt.cr2));
    asm volatile("lidt %0" : : "m" (hv_crash_ctxt.idtr));
    hv_wrmsr(MSR_GS_BASE, hv_crash_ctxt.gsbase);
    hv_wrmsr(MSR_EFER, hv_crash_ctxt.efer);
// restore the original kernel CS now via far return
    asm volatile("pushq %q0\n\t"
    "pushq %q1\n\t"
    "lretq"
    :: "r"(hv_crash_ctxt.cs), "r"(hv_crash_handle));
    }
// Tell objtool we are using lretq long jump in the above function intentionally
    STACK_FRAME_NON_STANDARD(hv_crash_c_entry);
#[no_mangle]
unsafe extern "C" fn hv_mark_tss_not_busy() {
    static void hv_mark_tss_not_busy(void)
    {
    struct desc_struct *desc = get_current_gdt_rw();
    tss_desc tss;
    memcpy(&tss, &desc[GDT_ENTRY_TSS], sizeof(tss_desc));
    tss.type = 0x9;        /* available 64-bit TSS. 0xB is busy TSS */
    write_gdt_entry(desc, GDT_ENTRY_TSS, &tss, DESC_TSS);
    }
// Save essential context
#[no_mangle]
unsafe extern "C" fn hv_hvcrash_ctxt_save() {
    static void hv_hvcrash_ctxt_save(void)
    {
    struct hv_crash_ctxt *ctxt = &hv_crash_ctxt;
    ctxt.rsp = current_stack_pointer;
    ctxt.cr0 = native_read_cr0();
    ctxt.cr4 = native_read_cr4();
    asm volatile("movq %%cr2, %0" : "=r"(ctxt.cr2));
    asm volatile("movq %%cr8, %0" : "=r"(ctxt.cr8));
    asm volatile("movw %%cs, %0" : "=m"(ctxt.cs));
    asm volatile("movw %%ss, %0" : "=m"(ctxt.ss));
    asm volatile("movw %%ds, %0" : "=m"(ctxt.ds));
    asm volatile("movw %%es, %0" : "=m"(ctxt.es));
    asm volatile("movw %%fs, %0" : "=m"(ctxt.fs));
    asm volatile("movw %%gs, %0" : "=m"(ctxt.gs));
    native_store_gdt(&ctxt.gdtr);
    store_idt(&ctxt.idtr);
    ctxt.gsbase = __rdmsr(MSR_GS_BASE);
    ctxt.efer = __rdmsr(MSR_EFER);
    ctxt.pat = __rdmsr(MSR_IA32_CR_PAT);
    }
// Add trampoline page to the kernel pagetable for transition to kernel PT
#[no_mangle]
unsafe extern "C" fn hv_crash_fixup_kernpt() {
    static void hv_crash_fixup_kernpt(void)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pgd = pgd_offset_k(trampoline_pa);
    p4d = p4d_offset(pgd, trampoline_pa);
// trampoline_pa is below 4G, so no pre-existing entry to clobber
    p4d_populate(&init_mm, p4d, (pud_t *)hv_crash_ptpgs[1]);
    p4d.p4d = p4d.p4d & ~(_PAGE_NX);    /* enable execute */
    }
//
// Notify the hyp that Linux has crashed. This will cause the hyp to quiesce
// and suspend all guest VPs.
//
#[no_mangle]
unsafe extern "C" fn hv_notify_prepare_hyp() {
    static void hv_notify_prepare_hyp(void)
    {
    u64 status;
    struct hv_input_notify_partition_event *input;
    struct hv_partition_event_root_crashdump_input *cda;
    input = *this_cpu_ptr(hyperv_pcpu_input_arg);
    cda = &input.input.crashdump_input;
    memset(input, 0, sizeof(*input));
    input.event = HV_PARTITION_EVENT_ROOT_CRASHDUMP;
    cda.crashdump_action = HV_CRASHDUMP_ENTRY;
    status = hv_do_hypercall(HVCALL_NOTIFY_PARTITION_EVENT, input, core::ptr::null_mut());
    if (!hv_result_success(status))
    return;
    cda.crashdump_action = HV_CRASHDUMP_SUSPEND_ALL_VPS;
    hv_do_hypercall(HVCALL_NOTIFY_PARTITION_EVENT, input, core::ptr::null_mut());
    }
//
// Common function for all cpus before devirtualization.
//
// Hypervisor crash: all cpus get here in NMI context.
// Linux crash: the panicing cpu gets here at base level, all others in NMI
// context. Note, panicing cpu may not be the BSP.
//
// The function is not inlined so it will show on the stack. It is named so
// because the crash cmd looks for certain well known function names on the
// stack before looking into the cpu saved note in the elf section, and
// that work is currently incomplete.
//
// Notes:
// Hypervisor crash:
// - the hypervisor is in a very restrictive mode at this point and any
// vmexit it cannot handle would result in reboot. So, no mumbo jumbo,
// just get to kexec as quickly as possible.
//
// Devirtualization is supported from the BSP only at present.
//
#[no_mangle]
unsafe extern "C" fn crash_nmi_callback(regs: *mut pt_regs) -> noinline __noclone void {
    static noinline __noclone void crash_nmi_callback(struct pt_regs *regs)
    {
    struct hv_input_disable_hyp_ex *input;
    let mut msecs: c_int = 1000, ccpu = smp_processor_id();
    if (ccpu == 0) {
// crash_save_cpu() will be done in the kexec path
    cpu_emergency_stop_pt();	/* disable performance trace */
    atomic_inc(&crash_cpus_wait);
    } else {
    crash_save_cpu(regs, ccpu);
    cpu_emergency_stop_pt();	/* disable performance trace */
    atomic_inc(&crash_cpus_wait);
    for (;;)
    cpu_relax();
    }
    while (atomic_read(&crash_cpus_wait) < num_online_cpus() && msecs--)
    mdelay(1);
    stop_nmi();
    if (!hv_has_crashed)
    hv_notify_prepare_hyp();
    if (crashing_cpu == -1)
    crashing_cpu = ccpu;		/* crash cmd uses this */
    hv_hvcrash_ctxt_save();
    hv_mark_tss_not_busy();
    hv_crash_fixup_kernpt();
    input = *this_cpu_ptr(hyperv_pcpu_input_arg);
    memset(input, 0, sizeof(*input));
    input.rip = trampoline_pa;
    input.arg = devirt_arg;
    (void)hv_do_hypercall(HVCALL_DISABLE_HYP_EX, input, core::ptr::null_mut());
    hv_panic_timeout_reboot();
    }
    static DEFINE_SPINLOCK(hv_crash_reboot_lk);
//
// Generic NMI callback handler: could be called without any crash also.
// hv crash: hypervisor injects NMI's into all cpus
// lx crash: panicing cpu sends NMI to all but self via crash_stop_other_cpus
//
#[no_mangle]
unsafe extern "C" fn hv_crash_nmi_local(cmd: c_uint, regs: *mut pt_regs) -> c_int {
    static int hv_crash_nmi_local(unsigned int cmd, struct pt_regs *regs)
    {
    if (!hv_has_crashed && hv_cda && hv_cda.cda_valid)
    hv_has_crashed = true;
    if (!hv_has_crashed && !lx_has_crashed)
    return NMI_DONE;	/* ignore the NMI */
    if (hv_has_crashed && !kexec_crash_loaded()) {
    if (spin_trylock(&hv_crash_reboot_lk))
    hv_panic_timeout_reboot();
    else
    for (;;)
    cpu_relax();
    }
    crash_nmi_callback(regs);
    return NMI_DONE;
    }
//
// hv_crash_stop_other_cpus() == smp_ops.crash_stop_other_cpus
//
// On normal Linux panic, this is called twice: first from panic and then again
// from native_machine_crash_shutdown.
//
// In case of hyperv, 3 ways to get here:
// 1. hv crash (only BSP will get here):
// BSP : NMI callback -> DisableHv -> hv_crash_asm32 -> hv_crash_c_entry
// -> __crash_kexec -> native_machine_crash_shutdown
// -> crash_smp_send_stop -> smp_ops.crash_stop_other_cpus
// Linux panic:
// 2. panic cpu x: panic() -> crash_smp_send_stop
// -> smp_ops.crash_stop_other_cpus
// 3. BSP: native_machine_crash_shutdown -> crash_smp_send_stop
//
// NB: noclone and non standard stack because of call to crash_setup_regs().
//
#[no_mangle]
unsafe extern "C" fn hv_crash_stop_other_cpus() -> void __noclone {
    static void __noclone hv_crash_stop_other_cpus(void)
    {
    static bool crash_stop_done;
    struct pt_regs lregs;
    let mut ccpu: c_int = smp_processor_id();
    if (hv_has_crashed)
    return;		/* all cpus already in NMI handler path */
    if (!kexec_crash_loaded()) {
    hv_notify_prepare_hyp();
    hv_panic_timeout_reboot();	/* no return */
    }
// If the hv crashes also, we could come here again before cpus_stopped
// is set in crash_smp_send_stop(). So use our own check.
//
    if (crash_stop_done)
    return;
    crash_stop_done = true;
// Linux has crashed: hv is healthy, we can IPI safely
    lx_has_crashed = true;
    wmb();			/* NMI handlers look at lx_has_crashed */
    apic.send_IPI_allbutself(NMI_VECTOR);
    if (crashing_cpu == -1)
    crashing_cpu = ccpu;		/* crash cmd uses this */
// crash_setup_regs() happens in kexec also, but for the kexec cpu which
// is the BSP. We could be here on non-BSP cpu, collect regs if so.
//
    if (ccpu)
    crash_setup_regs(&lregs, core::ptr::null_mut());
    crash_nmi_callback(&lregs);
    }
    STACK_FRAME_NON_STANDARD(hv_crash_stop_other_cpus);
// This GDT is accessed in IA32-e compat mode which uses 32bits addresses
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_gdtreg_32 {
    pub fill: u16,
    pub limit: u16,
    pub address: u32,
    pub __packed: },
// We need a CS with L bit to goto IA32-e long mode from 32bit compat mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_crash_tramp_gdt {
    pub /: *mut *mut u64 null; / index 0, selector 0, null selector,
    pub /: *mut *mut u64 cs64; / index 1, selector 8, cs64 selector,
    pub __packed: },
// No stack, so jump via far ptr in memory to load the 64bit CS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_cs_jmptgt {
    pub address: u32,
    pub csval: u16,
    pub fill: u16,
    pub __packed: },
// Linux use only, hypervisor doesn't look at this struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_crash_tramp_data {
    pub tramp32_cr3: u64,
    pub kernel_cr3: u64,
    pub gdtr32: hv_gdtreg_32,
    pub tramp_gdt: hv_crash_tramp_gdt,
    pub cs_jmptgt: hv_cs_jmptgt,
    pub c_entry_addr: u64,
    pub __packed: },
//
// Setup a temporary gdt to allow the asm code to switch to the long mode.
// Since the asm code is relocated/copied to a below 4G page, it cannot use rip
// relative addressing, hence we must use trampoline_pa here. Also, save other
// info like jmp and C entry targets for same reasons.
//
// Returns: 0 on success, -1 on error
//
#[no_mangle]
unsafe extern "C" fn hv_crash_setup_trampdata(trampoline_va: u64) -> c_int {
    static int hv_crash_setup_trampdata(u64 trampoline_va)
    {
    pub offs: int size,,
    pub dest: *mut c_void,
    pub tramp: *mut hv_crash_tramp_data,
// These must match exactly the ones in the corresponding asm file
    pub 0): BUILD_BUG_ON(offsetof(struct hv_crash_tramp_data, tramp32_cr3) !=,
    pub 8): BUILD_BUG_ON(offsetof(struct hv_crash_tramp_data, kernel_cr3) !=,
    pub 18): BUILD_BUG_ON(offsetof(struct hv_crash_tramp_data, gdtr32.limit) !=,
    BUILD_BUG_ON(offsetof(struct hv_crash_tramp_data,
    pub 40): cs_jmptgt.address) !=,
    pub 48): BUILD_BUG_ON(offsetof(struct hv_crash_tramp_data, c_entry_addr) !=,
// hv_crash_asm_end is beyond last byte by 1
    pub &hv_crash_asm32: size = &hv_crash_asm_end -,
    if (size + sizeof(struct hv_crash_tramp_data) > PAGE_SIZE) {
    pub __func__): pr_err("%s: trampoline page overflow\n",,
    pub -1: return,
    }
    pub )trampoline_va: *mut dest = (void,
    pub size): memcpy(dest, &hv_crash_asm32,,
    pub size: dest +=,
    pub 16): *mut *mut dest = (void )round_up((ulong)dest,,
    pub )dest: *mut tramp = (struct hv_crash_tramp_data,
// see MAX_ASID_AVAILABLE in tlb.c: "PCID 0 is reserved for use by
// non-PCID-aware users". Build cr3 with pcid 0
//
    pub __sme_pa(hv_crash_ptpgs[0]): tramp->tramp32_cr3 =,
// Note, when restoring X86_CR4_PCIDE, cr3[11:0] must be zero
    pub __sme_pa(init_mm.pgd): tramp->kernel_cr3 =,
    pub hv_crash_tramp_gdt): tramp->gdtr32.limit = sizeof(struct,
    tramp.gdtr32.address = trampoline_pa +
    pub trampoline_va: (ulong)&tramp->tramp_gdt -,
// base:0 limit:0xfffff type:b dpl:0 P:1 L:1 D:0 avl:0 G:1
    pub 0x00af9a000000ffff: tramp->tramp_gdt.cs64 =,
    pub 0x8: tramp->cs_jmptgt.csval =,
    pub (ulong)&hv_crash_asm32: offs = (ulong)&hv_crash_asm64 -,
    pub offs: tramp->cs_jmptgt.address = trampoline_pa +,
    pub (u64)&hv_crash_c_entry: tramp->c_entry_addr =,
    pub trampoline_va: devirt_arg = trampoline_pa + (ulong)dest -,
    pub 0: return,
    }
//
// Build 32bit trampoline page table for transition from protected mode
// non-paging to long-mode paging. This transition needs pagetables below 4G.
//
#[no_mangle]
unsafe extern "C" fn hv_crash_build_tramp_pt() {
    static void hv_crash_build_tramp_pt(void)
    {
    pub p4d: *mut p4d_t,
    pub pud: *mut pud_t,
    pub pmd: *mut pmd_t,
    pub pte: *mut pte_t,
    pub trampoline_pa: u64 pa, addr =,
    pub sizeof(p4d): *mut *mut p4d = hv_crash_ptpgs[0] + pgd_index(addr),
    pub virt_to_phys(hv_crash_ptpgs[1]): pa =,
    pub pa)): set_p4d(p4d, __p4d(_PAGE_TABLE |,
    pub /: *mut *mut p4d->p4d &= ~(_PAGE_NX); / enable execute,
    pub sizeof(pud): *mut *mut pud = hv_crash_ptpgs[1] + pud_index(addr),
    pub virt_to_phys(hv_crash_ptpgs[2]): pa =,
    pub pa)): set_pud(pud, __pud(_PAGE_TABLE |,
    pub sizeof(pmd): *mut *mut pmd = hv_crash_ptpgs[2] + pmd_index(addr),
    pub virt_to_phys(hv_crash_ptpgs[3]): pa =,
    pub pa)): set_pmd(pmd, __pmd(_PAGE_TABLE |,
    pub sizeof(pte): *mut *mut pte = hv_crash_ptpgs[3] + pte_index(addr),
    pub PAGE_KERNEL_EXEC)): set_pte(pte, pfn_pte(addr >> PAGE_SHIFT,,
    }
//
// Setup trampoline for devirtualization:
// - a page below 4G, ie 32bit addr containing asm glue code that hyp jmps to
// in protected mode.
// - 4 pages for a temporary page table that asm code uses to turn paging on
// - a temporary gdt to use in the compat mode.
//
// Returns: 0 on success
//
#[no_mangle]
unsafe extern "C" fn hv_crash_trampoline_setup() -> c_int {
    static int hv_crash_trampoline_setup(void)
    {
    pub order: int i, rc,,
    pub page: *mut page,
    pub trampoline_va: u64,
    pub __GFP_ZERO: gfp_t flags32 = GFP_KERNEL | GFP_DMA32 |,
// page for 32bit trampoline assembly code + hv_crash_tramp_data
    pub alloc_page(flags32): page =,
    if (page == core::ptr::null_mut()) {
    pub __func__): pr_err("%s: failed to alloc asm stub page\n",,
    pub -1: return,
    }
    pub (u64)page_to_virt(page): trampoline_va =,
    pub (u32)page_to_phys(page): trampoline_pa =,
    pub /: *mut *mut order = 2; / alloc 2^2 pages,
    pub order): page = alloc_pages(flags32,,
    if (page == core::ptr::null_mut()) {
    pub __func__): pr_err("%s: failed to alloc pt pages\n",,
    pub -1: return,
    }
    pub page++): for (i = 0; i < 4; i++,,
    pub page_to_virt(page): hv_crash_ptpgs[i] =,
    pub hv_crash_setup_trampdata(trampoline_va): rc =,
    if (rc)
    pub errout: goto,
    pub 0: return,
    errout:
    pub order): free_pages((ulong)hv_crash_ptpgs[0],,
    pub rc: return,
    }
// Setup for kdump kexec to collect hypervisor RAM when running as root
#[no_mangle]
pub unsafe extern "C" fn hv_root_crash_init() {
    void hv_root_crash_init(void)
    {
    pub rc: c_int,
    pub input: *mut hv_input_get_system_property,
    pub output: *mut hv_output_get_system_property,
    pub flags: c_ulong,
    pub status: u64,
    pub cda_info: union hv_pfn_range,
    if (pgtable_l5_enabled()) {
    pub PTs\n"): pr_err("Hyper-V: crash dump not yet supported on 5level,
    }
    rc = register_nmi_handler(NMI_LOCAL, hv_crash_nmi_local, NMI_FLAG_FIRST,
    if (rc) {
    pub handler\n"): pr_err("Hyper-V: failed to register crash nmi,
    }
    pub this_cpu_ptr(hyperv_pcpu_input_arg): *mut input =,
    pub this_cpu_ptr(hyperv_pcpu_output_arg): *mut output =,
    pub sizeof(*input)): *mut memset(input, 0,,
    pub HV_SYSTEM_PROPERTY_CRASHDUMPAREA: input->property_id =,
    pub output): status = hv_do_hypercall(HVCALL_GET_SYSTEM_PROPERTY, input,,
    pub output->hv_cda_info.as_uint64: cda_info.as_uint64 =,
    if (!hv_result_success(status)) {
    pr_err("Hyper-V: %s: property:%d %s\n", __func__,
    pub hv_result_to_string(status)): input->property_id,,
    pub err_out: goto,
    }
    if (cda_info.base_pfn == 0) {
    pub 0\n"): pr_err("Hyper-V: hypervisor crash dump area pfn is,
    pub err_out: goto,
    }
    pub HV_HYP_PAGE_SHIFT): hv_cda = phys_to_virt(cda_info.base_pfn <<,
    pub hv_crash_trampoline_setup(): rc =,
    if (rc)
    pub err_out: goto,

    pub hv_crash_stop_other_cpus: smp_ops.crash_stop_other_cpus =,

    pub true: crash_kexec_post_notifiers =,
    pub true: hv_crash_enabled =,
    pub enabled\n"): pr_info("Hyper-V: both linux and hypervisor kdump support,
    err_out:
    pub "hv_crash_nmi"): unregister_nmi_handler(NMI_LOCAL,,
    pub enabled\n"): pr_err("Hyper-V: only linux root kdump support,
    }
