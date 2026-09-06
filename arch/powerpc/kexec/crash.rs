//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kexec/crash.c
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
// Architecture specific (PPC64) functions for kexec based crash dumps.
//
// Copyright (C) 2005, IBM Corp.
//
// Created by: Haren Myneni
//

//
// The primary CPU waits a while for all secondary CPUs to enter. This is to
// avoid sending an IPI if the secondary CPUs are entering
// crash_kexec_secondary on their own (eg via a system reset).
//
// The secondary timeout has to be longer than the primary. Both timeouts are
// in milliseconds.
//
pub const PRIMARY_TIMEOUT: c_int = 500;
pub const SECONDARY_TIMEOUT: c_int = 1000;
pub const IPI_TIMEOUT: c_int = 10000;
pub const REAL_MODE_TIMEOUT: c_int = 10000;
    static int time_to_dump;
//
// In case of system reset, secondary CPUs enter crash_kexec_secondary with out
// having to send an IPI explicitly. So, indicate if the crash is via
// system reset to avoid sending another IPI.
//
    static int is_via_system_reset;
//
// crash_wake_offline should be set to 1 by platforms that intend to wake
// up offline cpus prior to jumping to a kdump kernel. Currently powernv
// sets it to 1, since we want to avoid things from happening when an
// offline CPU wakes up due to something like an HMI (malfunction error),
// which propagates to all threads.
//
    int crash_wake_offline;
pub const CRASH_HANDLER_MAX: c_int = 3;
// List of shutdown handles
    static crash_shutdown_t crash_shutdown_handles[CRASH_HANDLER_MAX];
    static DEFINE_SPINLOCK(crash_handlers_lock);
    static unsigned long crash_shutdown_buf[JMP_BUF_LEN];
    let mut crash_shutdown_cpu: static int = -1;
#[no_mangle]
unsafe extern "C" fn handle_fault(regs: *mut pt_regs) -> c_int {
    static int handle_fault(struct pt_regs *regs)
    {
    if (crash_shutdown_cpu == smp_processor_id())
    longjmp(crash_shutdown_buf, 1);
    return 0;
    }

    static atomic_t cpus_in_crash;
#[no_mangle]
pub unsafe extern "C" fn crash_ipi_callback(regs: *mut pt_regs) {
    void crash_ipi_callback(struct pt_regs *regs)
    {
    let mut cpus_state_saved: static cpumask_t = CPU_MASK_NONE;
    let mut cpu: c_int = smp_processor_id();
    hard_irq_disable();
    if (!cpumask_test_cpu(cpu, &cpus_state_saved)) {
    crash_save_cpu(regs, cpu);
    cpumask_set_cpu(cpu, &cpus_state_saved);
    }
    atomic_inc(&cpus_in_crash);
    smp_mb__after_atomic();
//
// Starting the kdump boot.
// This barrier is needed to make sure that all CPUs are stopped.
//
    while (!time_to_dump)
    cpu_relax();
    if (ppc_md.kexec_cpu_down)
    ppc_md.kexec_cpu_down(1, 1);

    kexec_smp_wait();

    for (;;);	/* FIXME */

// NOTREACHED
    }
#[no_mangle]
unsafe extern "C" fn crash_kexec_prepare_cpus() {
    static void crash_kexec_prepare_cpus(void)
    {
    unsigned int msecs;
    volatile unsigned int ncpus = num_online_cpus() - 1;/* Excluding the panic cpu */
    let mut tries: volatile int = 0;
    int (*old_handler)(struct pt_regs *regs);
    printk(KERN_EMERG "Sending IPI to other CPUs\n");
    if (crash_wake_offline)
    ncpus = num_present_cpus() - 1;
//
// If we came in via system reset, secondaries enter via crash_kexec_secondary().
// So, wait a while for the secondary CPUs to enter for that case.
// Else, send IPI to all other CPUs.
//
    if (is_via_system_reset)
    mdelay(PRIMARY_TIMEOUT);
    else
    crash_send_ipi(crash_ipi_callback);
    smp_wmb();
    again:
//
// FIXME: Until we will have the way to stop other CPUs reliably,
// the crash CPU will send an IPI and wait for other CPUs to
// respond.
//
    msecs = IPI_TIMEOUT;
    while ((atomic_read(&cpus_in_crash) < ncpus) && (--msecs > 0))
    mdelay(1);
// Would it be better to replace the trap vector here?
    if (atomic_read(&cpus_in_crash) >= ncpus) {
    printk(KERN_EMERG "IPI complete\n");
    return;
    }
    printk(KERN_EMERG "ERROR: %d cpu(s) not responding\n",
    ncpus - atomic_read(&cpus_in_crash));
//
// If we have a panic timeout set then we can't wait indefinitely
// for someone to activate system reset. We also give up on the
// second time through if system reset fail to work.
//
    if ((panic_timeout > 0) || (tries > 0))
    return;
//
// A system reset will cause all CPUs to take an 0x100 exception.
// The primary CPU returns here via setjmp, and the secondary
// CPUs reexecute the crash_kexec_secondary path.
//
    old_handler = __debugger;
    __debugger = handle_fault;
    crash_shutdown_cpu = smp_processor_id();
    if (setjmp(crash_shutdown_buf) == 0) {
    printk(KERN_EMERG "Activate system reset (dumprestart) "
    "to stop other cpu(s)\n");
//
// A system reset will force all CPUs to execute the
// crash code again. We need to reset cpus_in_crash so we
// wait for everyone to do this.
//
    atomic_set(&cpus_in_crash, 0);
    smp_mb();
    while (atomic_read(&cpus_in_crash) < ncpus)
    cpu_relax();
    }
    crash_shutdown_cpu = -1;
    __debugger = old_handler;
    tries++;
    goto again;
    }
//
// This function will be called by secondary cpus.
//
#[no_mangle]
pub unsafe extern "C" fn crash_kexec_secondary(regs: *mut pt_regs) {
    void crash_kexec_secondary(struct pt_regs *regs)
    {
    unsigned long flags;
    let mut msecs: c_int = SECONDARY_TIMEOUT;
    local_irq_save(flags);
// Wait for the primary crash CPU to signal its progress
    while (crashing_cpu < 0) {
    if (--msecs < 0) {
// No response, kdump image may not have been loaded
    local_irq_restore(flags);
    return;
    }
    mdelay(1);
    }
    crash_ipi_callback(regs);
    }

#[no_mangle]
unsafe extern "C" fn crash_kexec_prepare_cpus() {
    static void crash_kexec_prepare_cpus(void)
    {
//
// move the secondaries to us so that we can copy
// the new kernel 0-0x100 safely
//
// do this if kexec in setup.c ?
//

    smp_release_cpus();

// FIXME

    }
#[no_mangle]
pub unsafe extern "C" fn crash_kexec_secondary(regs: *mut pt_regs) {
    void crash_kexec_secondary(struct pt_regs *regs)
    {
    }

// wait for all the CPUs to hit real mode but timeout if they don't come in

#[no_mangle]
pub unsafe extern "C" fn crash_kexec_wait_realmode(cpu: c_int) -> noinstr static void __maybe_unused {
    noinstr static void __maybe_unused crash_kexec_wait_realmode(int cpu)
    {
    unsigned int msecs;
    int i;
    msecs = REAL_MODE_TIMEOUT;
    for (i=0; i < nr_cpu_ids && msecs > 0; i++) {
    if (i == cpu)
    continue;
    while (paca_ptrs[i].kexec_state < KEXEC_STATE_REAL_MODE) {
    barrier();
    if (!cpu_possible(i) || !cpu_online(i) || (msecs <= 0))
    break;
    msecs--;
    mdelay(1);
    }
    }
    mb();
    }

    static inline void crash_kexec_wait_realmode(int cpu) {}

#[no_mangle]
pub unsafe extern "C" fn crash_kexec_prepare() {
    void crash_kexec_prepare(void)
    {
// Avoid hardlocking with irresponsive CPU holding logbuf_lock
    printk_deferred_enter();
//
// This function is only called after the system
// has panicked or is otherwise in a critical state.
// The minimum amount of code to allow a kexec'd kernel
// to run successfully needs to happen here.
//
// In practice this means stopping other cpus in
// an SMP system.
// The kernel is broken so disable interrupts.
//
    hard_irq_disable();
//
// Make a note of crashing cpu. Will be used in machine_kexec
// such that another IPI will not be sent.
//
    crashing_cpu = smp_processor_id();
    crash_kexec_prepare_cpus();
    }
//
// Register a function to be called on shutdown.  Only use this if you
// can't reset your device in the second kernel.
//
#[no_mangle]
pub unsafe extern "C" fn crash_shutdown_register(handler: crash_shutdown_t) -> c_int {
    int crash_shutdown_register(crash_shutdown_t handler)
    {
    unsigned int i, rc;
    spin_lock(&crash_handlers_lock);
    for (i = 0 ; i < CRASH_HANDLER_MAX; i++)
    if (!crash_shutdown_handles[i]) {
// Insert handle at first empty entry
    crash_shutdown_handles[i] = handler;
    rc = 0;
    break;
    }
    if (i == CRASH_HANDLER_MAX) {
    printk(KERN_ERR "Crash shutdown handles full, "
    "not registered.\n");
    rc = 1;
    }
    spin_unlock(&crash_handlers_lock);
    return rc;
    }
    EXPORT_SYMBOL(crash_shutdown_register);
#[no_mangle]
pub unsafe extern "C" fn crash_shutdown_unregister(handler: crash_shutdown_t) -> c_int {
    int crash_shutdown_unregister(crash_shutdown_t handler)
    {
    unsigned int i, rc;
    spin_lock(&crash_handlers_lock);
    for (i = 0 ; i < CRASH_HANDLER_MAX; i++)
    if (crash_shutdown_handles[i] == handler)
    break;
    if (i == CRASH_HANDLER_MAX) {
    printk(KERN_ERR "Crash shutdown handle not found\n");
    rc = 1;
    } else {
// Shift handles down
    for (; i < (CRASH_HANDLER_MAX - 1); i++)
    crash_shutdown_handles[i] =
    crash_shutdown_handles[i+1];
//
// Reset last entry to NULL now that it has been shifted down,
// this will allow new handles to be added here.
//
    crash_shutdown_handles[i] = core::ptr::null_mut();
    rc = 0;
    }
    spin_unlock(&crash_handlers_lock);
    return rc;
    }
    EXPORT_SYMBOL(crash_shutdown_unregister);
#[no_mangle]
pub unsafe extern "C" fn default_machine_crash_shutdown(regs: *mut pt_regs) {
    void default_machine_crash_shutdown(struct pt_regs *regs)
    {
    volatile unsigned int i;
    int (*old_handler)(struct pt_regs *regs);
    if (TRAP(regs) == INTERRUPT_SYSTEM_RESET)
    is_via_system_reset = 1;
    if (IS_ENABLED(CONFIG_SMP))
    crash_smp_send_stop();
    else
    crash_kexec_prepare();
    crash_save_cpu(regs, crashing_cpu);
    time_to_dump = 1;
    crash_kexec_wait_realmode(crashing_cpu);
    machine_kexec_mask_interrupts();
//
// Call registered shutdown routines safely.  Swap out
// __debugger_fault_handler, and replace on exit.
//
    old_handler = __debugger_fault_handler;
    __debugger_fault_handler = handle_fault;
    crash_shutdown_cpu = smp_processor_id();
    for (i = 0; i < CRASH_HANDLER_MAX && crash_shutdown_handles[i]; i++) {
    if (setjmp(crash_shutdown_buf) == 0) {
//
// Insert syncs and delay to ensure
// instructions in the dangerous region don't
// leak away from this protected region.
//
    asm volatile("sync; isync");
// dangerous region
    crash_shutdown_handles[i]();
    asm volatile("sync; isync");
    }
    }
    crash_shutdown_cpu = -1;
    __debugger_fault_handler = old_handler;
    if (ppc_md.kexec_cpu_down)
    ppc_md.kexec_cpu_down(1, 0);
    }

//
// sync_backup_region_phdr - synchronize backup region offset between
// kexec image and ELF core header.
// @image: Kexec image.
// @ehdr: ELF core header.
// @phdr_to_kimage: If true, read the offset from the ELF program header
// and update the kimage backup region. If false, update
// the ELF program header offset from the kimage backup
// region.
//
// Note: During kexec_load, this is called with phdr_to_kimage = true. For
// kexec_file_load and ELF core header recreation during memory hotplug
// events, it is called with phdr_to_kimage = false.
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn sync_backup_region_phdr(image: *mut kimage, ehdr: *mut Elf64_Ehdr, phdr_to_kimage: bool) {
    void sync_backup_region_phdr(struct kimage *image, Elf64_Ehdr *ehdr, bool phdr_to_kimage)
    {
    Elf64_Phdr *phdr;
    unsigned int i;
    phdr = (Elf64_Phdr *)(ehdr + 1);
    for (i = 0; i < ehdr.e_phnum; i++, phdr++) {
    if (phdr.p_paddr == BACKUP_SRC_START) {
    if (phdr_to_kimage)
    image.arch.backup_start = phdr.p_offset;
    else
    phdr.p_offset = image.arch.backup_start;
    kexec_dprintk("Backup region offset updated to 0x%lx\n",
    image.arch.backup_start);
    return;
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_post_load(image: *mut kimage) -> c_int {
    int machine_kexec_post_load(struct kimage *image)
    {
    int i;
    unsigned long mem;
    unsigned char *ptr;
    if (image.type != KEXEC_TYPE_CRASH)
    return 0;
    if (image.file_mode)
    return 0;
    for (i = 0; i < image.nr_segments; i++) {
    mem = image.segment[i].mem;
    ptr = (char *)__va(mem);
    if (ptr && memcmp(ptr, ELFMAG, SELFMAG) == 0)
    sync_backup_region_phdr(image, (Elf64_Ehdr *) ptr, true);
    }
    return 0;
    }

//
// Advertise preferred elfcorehdr size to userspace via
// /sys/kernel/crash_elfcorehdr_size sysfs interface.
//
#[no_mangle]
pub unsafe extern "C" fn arch_crash_get_elfcorehdr_size() -> c_uint {
    unsigned int arch_crash_get_elfcorehdr_size(void)
    {
    unsigned long phdr_cnt;
// A program header for possible CPUs + vmcoreinfo
    phdr_cnt = num_possible_cpus() + 1;
    if (IS_ENABLED(CONFIG_MEMORY_HOTPLUG))
    phdr_cnt += CONFIG_CRASH_MAX_MEMORY_RANGES;
    return sizeof(struct elfhdr) + (phdr_cnt * sizeof(Elf64_Phdr));
    }
//
// update_crash_elfcorehdr() - Recreate the elfcorehdr and replace it with old
// elfcorehdr in the kexec segment array.
// @image: the active struct kimage
// @mn: struct memory_notify data handler
//
#[no_mangle]
unsafe extern "C" fn update_crash_elfcorehdr(image: *mut kimage, mn: *mut memory_notify) {
    static void update_crash_elfcorehdr(struct kimage *image, struct memory_notify *mn)
    {
    int ret;
    struct crash_mem *cmem = core::ptr::null_mut();
    struct kexec_segment *ksegment;
    void *ptr, *mem, *elfbuf = core::ptr::null_mut();
    unsigned long elfsz, memsz, base_addr, size, end;
    ksegment = &image.segment[image.elfcorehdr_index];
    mem = (void *) ksegment.mem;
    memsz = ksegment.memsz;
    ret = get_crash_memory_ranges(&cmem);
    if (ret) {
    pr_err("Failed to get crash mem range\n");
    goto out;
    }
//
// The hot unplugged memory is part of crash memory ranges,
// remove it here.
//
    if (image.hp_action == KEXEC_CRASH_HP_REMOVE_MEMORY) {
    base_addr = PFN_PHYS(mn.start_pfn);
    size = mn.nr_pages * PAGE_SIZE;
    end = base_addr + size - 1;
    ret = arch_crash_exclude_mem_range(&cmem, base_addr, end);
    if (ret) {
    pr_err("Failed to remove hot-unplugged memory from crash memory ranges\n");
    goto out;
    }
    }
    ret = crash_prepare_elf64_headers(cmem, false, &elfbuf, &elfsz);
    if (ret) {
    pr_err("Failed to prepare elf header\n");
    goto out;
    }
//
// It is unlikely that kernel hit this because elfcorehdr kexec
// segment (memsz) is built with addition space to accommodate growing
// number of crash memory ranges while loading the kdump kernel. It is
// Just to avoid any unforeseen case.
//
    if (elfsz > memsz) {
    pr_err("Updated crash elfcorehdr elfsz %lu > memsz %lu", elfsz, memsz);
    goto out;
    }
    sync_backup_region_phdr(image, (Elf64_Ehdr *) elfbuf, false);
    ptr = __va(mem);
    if (ptr) {
// Temporarily invalidate the crash image while it is replaced
    xchg(&kexec_crash_image, core::ptr::null_mut());
// Replace the old elfcorehdr with newly prepared elfcorehdr
    memcpy((void *)ptr, elfbuf, elfsz);
// The crash image is now valid once again
    xchg(&kexec_crash_image, image);
    }
    out:
    kvfree(cmem);
    kvfree(elfbuf);
    }
//
// get_fdt_index - Loop through the kexec segment array and find
// the index of the FDT segment.
// @image: a pointer to kexec_crash_image
//
// Returns the index of FDT segment in the kexec segment array
// if found; otherwise -1.
//
#[no_mangle]
unsafe extern "C" fn get_fdt_index(image: *mut kimage) -> c_int {
    static int get_fdt_index(struct kimage *image)
    {
    void *ptr;
    unsigned long mem;
    int i, fdt_index = -1;
// Find the FDT segment index in kexec segment array.
    for (i = 0; i < image.nr_segments; i++) {
    mem = image.segment[i].mem;
    ptr = __va(mem);
    if (ptr && fdt_magic(ptr) == FDT_MAGIC) {
    fdt_index = i;
    break;
    }
    }
    return fdt_index;
    }
//
// update_crash_fdt - updates the cpus node of the crash FDT.
//
// @image: a pointer to kexec_crash_image
//
#[no_mangle]
unsafe extern "C" fn update_crash_fdt(image: *mut kimage) {
    static void update_crash_fdt(struct kimage *image)
    {
    void *fdt;
    int fdt_index;
    fdt_index = get_fdt_index(image);
    if (fdt_index < 0) {
    pr_err("Unable to locate FDT segment.\n");
    return;
    }
    fdt = __va((void *)image.segment[fdt_index].mem);
// Temporarily invalidate the crash image while it is replaced
    xchg(&kexec_crash_image, core::ptr::null_mut());
// update FDT to reflect changes in CPU resources
    if (update_cpus_node(fdt))
    pr_err("Failed to update crash FDT");
// The crash image is now valid once again
    xchg(&kexec_crash_image, image);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_crash_hotplug_support(image: *mut kimage, kexec_flags: c_ulong) -> c_int {
    int arch_crash_hotplug_support(struct kimage *image, unsigned long kexec_flags)
    {

    if (image.file_mode)
    return 1;

    return kexec_flags & KEXEC_CRASH_HOTPLUG_SUPPORT;
    }
//
// arch_crash_handle_hotplug_event - Handle crash CPU/Memory hotplug events to update the
// necessary kexec segments based on the hotplug event.
// @image: a pointer to kexec_crash_image
// @arg: struct memory_notify handler for memory hotplug case and NULL for CPU hotplug case.
//
// Update the kdump image based on the type of hotplug event, represented by image->hp_action.
// CPU add: Update the FDT segment to include the newly added CPU.
// CPU remove: No action is needed, with the assumption that it's okay to have offline CPUs
// part of the FDT.
// Memory add/remove: No action is taken as this is not yet supported.
//
#[no_mangle]
pub unsafe extern "C" fn arch_crash_handle_hotplug_event(image: *mut kimage, arg: *mut c_void) {
    void arch_crash_handle_hotplug_event(struct kimage *image, void *arg)
    {
    struct memory_notify *mn;
    switch (image.hp_action) {
    case KEXEC_CRASH_HP_REMOVE_CPU:
    return;
    case KEXEC_CRASH_HP_ADD_CPU:
    update_crash_fdt(image);
    break;
    case KEXEC_CRASH_HP_REMOVE_MEMORY:
    case KEXEC_CRASH_HP_ADD_MEMORY:
    mn = (struct memory_notify *)arg;
    update_crash_elfcorehdr(image, mn);
    return;
    default:
    pr_warn_once("Unknown hotplug action\n");
    }
    }
