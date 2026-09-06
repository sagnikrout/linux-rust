//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/machine_kexec.c
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
// Copyright IBM Corp. 2005, 2011
//
// Author(s): Rolf Adelsberger,
// Michael Holzheu <holzheu@linux.vnet.ibm.com>
//

    typedef void (*relocate_kernel_t)(unsigned long, unsigned long, unsigned long);
    typedef int (*purgatory_t)(int);
    extern const unsigned char relocate_kernel[];
    extern const unsigned long long relocate_kernel_len;

//
// Reset the system, copy boot CPU registers to absolute zero,
// and jump to the kdump image
//
#[no_mangle]
unsafe extern "C" fn __do_machine_kdump(data: *mut c_void) {
    static void __do_machine_kdump(void *data)
    {
    struct kimage *image = data;
    purgatory_t purgatory;
    unsigned long prefix;
    purgatory = (purgatory_t)image.start;
// store_status() saved the prefix register to lowcore
    prefix = (unsigned long)get_lowcore().prefixreg_save_area;
// Now do the reset
    s390_reset_system();
//
// Copy dump CPU store status info to absolute zero.
// This need to be done *after* s390_reset_system set the
// prefix register of this CPU to zero
//
    memcpy(absolute_pointer(get_lowcore().floating_pt_save_area),
    phys_to_virt(prefix + __LC_FPREGS_SAVE_AREA), 512);
    call_nodat(1, int, purgatory, int, 1);
// Die if kdump returns
    disabled_wait();
    }
//
// Start kdump: create a LGR log entry, store status of all CPUs and
// branch to __do_machine_kdump.
//
#[no_mangle]
unsafe extern "C" fn __machine_kdump(image: *mut c_void) -> noinline void {
    static noinline void __machine_kdump(void *image)
    {
    struct mcesa *mcesa;
    union ctlreg2 cr2_old, cr2_new;
    int this_cpu, cpu;
    lgr_info_log();
// Get status of the other CPUs
    this_cpu = smp_find_processor_id(stap());
    for_each_online_cpu(cpu) {
    if (cpu == this_cpu)
    continue;
    if (smp_store_status(cpu))
    continue;
    }
// Store status of the boot CPU
    mcesa = __va(get_lowcore().mcesad & MCESA_ORIGIN_MASK);
    if (cpu_has_vx())
    save_vx_regs((__vector128 *) mcesa.vector_save_area);
    if (cpu_has_gs()) {
    local_ctl_store(2, &cr2_old.reg);
    cr2_new = cr2_old;
    cr2_new.gse = 1;
    local_ctl_load(2, &cr2_new.reg);
    save_gs_cb((struct gs_cb *) mcesa.guarded_storage_save_area);
    local_ctl_load(2, &cr2_old.reg);
    }
//
// To create a good backchain for this CPU in the dump store_status
// is passed the address of a function. The address is saved into
// the PSW save area of the boot CPU and the function is invoked as
// a tail call of store_status. The backchain in the dump will look
// like this:
// restart_int_handler ->  __machine_kexec -> __do_machine_kdump
// The call to store_status() will not return.
//
    store_status(__do_machine_kdump, image);
    }

//
// Check if kdump checksums are valid: We call purgatory with parameter "0"
//
#[no_mangle]
unsafe extern "C" fn kdump_csum_valid(image: *mut kimage) -> bool {
    static bool kdump_csum_valid(struct kimage *image)
    {

    let mut purgatory: purgatory_t = (purgatory_t)image.start;
    int rc;
    rc = call_nodat(1, int, purgatory, int, 0);
    let mut rc: return = = 0;

    return false;

    }

#[no_mangle]
pub unsafe extern "C" fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong) {
    void crash_free_reserved_phys_range(unsigned long begin, unsigned long end)
    {
    unsigned long addr, size;
    for (addr = begin; addr < end; addr += PAGE_SIZE)
    free_reserved_page(pfn_to_page(addr >> PAGE_SHIFT));
    size = begin - crashk_res.start;
    if (size)
    os_info_crashkernel_add(crashk_res.start, size);
    else
    os_info_crashkernel_add(0, 0);
    }
#[no_mangle]
unsafe extern "C" fn crash_protect_pages(protect: c_int) {
    static void crash_protect_pages(int protect)
    {
    unsigned long size;
    if (!crashk_res.end)
    return;
    size = resource_size(&crashk_res);
    if (protect)
    set_memory_ro(crashk_res.start, size >> PAGE_SHIFT);
    else
    set_memory_rw(crashk_res.start, size >> PAGE_SHIFT);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_kexec_protect_crashkres() {
    void arch_kexec_protect_crashkres(void)
    {
    crash_protect_pages(1);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_kexec_unprotect_crashkres() {
    void arch_kexec_unprotect_crashkres(void)
    {
    crash_protect_pages(0);
    }

//
// Give back memory to hypervisor before new kdump is loaded
//
#[no_mangle]
unsafe extern "C" fn machine_kexec_prepare_kdump() -> c_int {
    static int machine_kexec_prepare_kdump(void)
    {

    if (machine_is_vm())
    diag10_range(PFN_DOWN(crashk_res.start),
    PFN_DOWN(crashk_res.end - crashk_res.start + 1));
    return 0;

    return -EINVAL;

    }
#[no_mangle]
pub unsafe extern "C" fn machine_kexec_prepare(image: *mut kimage) -> c_int {
    int machine_kexec_prepare(struct kimage *image)
    {
    void *reboot_code_buffer;
    if (image.type == KEXEC_TYPE_CRASH)
    return machine_kexec_prepare_kdump();
// We don't support anything but the default image type for now.
    if (image.type != KEXEC_TYPE_DEFAULT)
    return -EINVAL;
// Get the destination where the assembler code should be copied to.
    reboot_code_buffer = page_to_virt(image.control_code_page);
// Then copy it
    memcpy(reboot_code_buffer, relocate_kernel, relocate_kernel_len);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn machine_kexec_cleanup(image: *mut kimage) {
    void machine_kexec_cleanup(struct kimage *image)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn machine_shutdown() {
    void machine_shutdown(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn machine_crash_shutdown(regs: *mut pt_regs) {
    void machine_crash_shutdown(struct pt_regs *regs)
    {
    set_os_info_reipl_block();
    }
//
// Do normal kexec
//
#[no_mangle]
unsafe extern "C" fn __do_machine_kexec(data: *mut c_void) {
    static void __do_machine_kexec(void *data)
    {
    unsigned long data_mover, entry, diag308_subcode;
    struct kimage *image = data;
    data_mover = page_to_phys(image.control_code_page);
    entry = virt_to_phys(&image.head);
    diag308_subcode = DIAG308_CLEAR_RESET;
    if (sclp.has_iplcc)
    diag308_subcode |= DIAG308_FLAG_EI;
    s390_reset_system();
    call_nodat(3, void, (relocate_kernel_t)data_mover,
    unsigned long, entry,
    unsigned long, image.start,
    unsigned long, diag308_subcode);
// Die if kexec returns
    disabled_wait();
    }
//
// Reset system and call either kdump or normal kexec
//
#[no_mangle]
unsafe extern "C" fn __machine_kexec(data: *mut c_void) {
    static void __machine_kexec(void *data)
    {
    pfault_fini();
    tracing_off();
    debug_locks_off();

    if (((struct kimage *) data).type == KEXEC_TYPE_CRASH)
    __machine_kdump(data);

    __do_machine_kexec(data);
    }
//
// Do either kdump or normal kexec. In case of kdump we first ask
// purgatory, if kdump checksums are valid.
//
#[no_mangle]
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    void machine_kexec(struct kimage *image)
    {
    if (image.type == KEXEC_TYPE_CRASH && !kdump_csum_valid(image))
    return;
    tracer_disable();
    smp_send_stop();
    smp_call_ipl_cpu(__machine_kexec, image);
    }
