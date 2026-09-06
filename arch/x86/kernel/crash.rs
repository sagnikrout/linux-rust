//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/crash.c
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
// Architecture specific (i386/x86_64) functions for kexec based crash dumps.
//
// Created by: Hariprasad Nellitheertha (hari@in.ibm.com)
//
// Copyright (C) IBM Corporation, 2004. All rights reserved.
// Copyright (C) Red Hat Inc., 2014. All rights reserved.
// Authors:
// Vivek Goyal <vgoyal@redhat.com>
//

// Used while preparing memory map entries for second kernel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crash_memmap_data {
    pub params: *mut boot_params,
// Type of memory
    pub type: c_uint,
}

#[no_mangle]
unsafe extern "C" fn kdump_nmi_callback(cpu: c_int, regs: *mut pt_regs) {
    static void kdump_nmi_callback(int cpu, struct pt_regs *regs)
    {
    crash_save_cpu(regs, cpu);
//
// Disable Intel PT to stop its logging
//
    cpu_emergency_stop_pt();
    kdump_sev_callback();
    disable_local_APIC();
    }
#[no_mangle]
pub unsafe extern "C" fn kdump_nmi_shootdown_cpus() {
    void kdump_nmi_shootdown_cpus(void)
    {
    nmi_shootdown_cpus(kdump_nmi_callback);
    disable_local_APIC();
    }
// Override the weak function in kernel/panic.c
#[no_mangle]
pub unsafe extern "C" fn crash_smp_send_stop() {
    void crash_smp_send_stop(void)
    {
    static int cpus_stopped;
    if (cpus_stopped)
    return;
    if (smp_ops.crash_stop_other_cpus)
    smp_ops.crash_stop_other_cpus();
    else
    smp_send_stop();
    cpus_stopped = 1;
    }

#[no_mangle]
pub unsafe extern "C" fn crash_smp_send_stop() {
    void crash_smp_send_stop(void)
    {
// There are no cpus to shootdown
    }

#[no_mangle]
pub unsafe extern "C" fn native_machine_crash_shutdown(regs: *mut pt_regs) {
    void native_machine_crash_shutdown(struct pt_regs *regs)
    {
// This function is only called after the system
// has panicked or is otherwise in a critical state.
// The minimum amount of code to allow a kexec'd kernel
// to run successfully needs to happen here.
//
// In practice this means shooting down the other cpus in
// an SMP system.
//
// The kernel is broken so disable interrupts
    local_irq_disable();
    crash_smp_send_stop();
    tdx_sys_disable();
    x86_virt_emergency_disable_virtualization_cpu();
//
// Disable Intel PT to stop its logging
//
    cpu_emergency_stop_pt();

// Prevent crash_kexec() from deadlocking on ioapic_lock.
    ioapic_zap_locks();
    clear_IO_APIC();

    lapic_shutdown();
    restore_boot_irq_mode();

    hpet_disable();

//
// Non-crash kexec calls enc_kexec_begin() while scheduling is still
// active. This allows the callback to wait until all in-flight
// shared<->private conversions are complete. In a crash scenario,
// enc_kexec_begin() gets called after all but one CPU have been shut
// down and interrupts have been disabled. This allows the callback to
// detect a race with the conversion and report it.
//
    x86_platform.guest.enc_kexec_begin();
    x86_platform.guest.enc_kexec_finish();
    crash_save_cpu(regs, smp_processor_id());
    }

#[no_mangle]
unsafe extern "C" fn get_nr_ram_ranges_callback(res: *mut resource, arg: *mut c_void) -> c_int {
    static int get_nr_ram_ranges_callback(struct resource *res, void *arg)
    {
    unsigned int *nr_ranges = arg;
    (*nr_ranges)++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_get_system_nr_ranges() -> c_uint {
    unsigned int arch_get_system_nr_ranges(void)
    {
//
// Exclusion of crash region, crashk_low_res and/or crashk_cma_ranges
// may cause range splits. So add extra slots here.
//
// Exclusion of low 1M may not cause another range split, because the
// range of exclude is [0, 1M] and the condition for splitting a new
// region is that the start, end parameters are both in a certain
// existing region in cmem and cannot be equal to existing region's
// start or end. Obviously, the start of [0, 1M] cannot meet this
// condition.
//
// But in order to lest the low 1M could be changed in the future,
// (e.g. [start, 1M]), add a extra slot.
//
    let mut nr_ranges: c_uint = 3 + crashk_cma_cnt;
    walk_system_ram_res(0, -1, &nr_ranges, get_nr_ram_ranges_callback);
    return nr_ranges;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_crash_exclude_ranges(cmem: *mut crash_mem) -> c_int {
    int arch_crash_exclude_ranges(struct crash_mem *cmem)
    {
// Exclude the low 1M because it is always reserved
    return crash_exclude_mem_range(cmem, 0, SZ_1M - 1);
    }
#[no_mangle]
unsafe extern "C" fn prepare_elf64_ram_headers_callback(res: *mut resource, arg: *mut c_void) -> c_int {
    static int prepare_elf64_ram_headers_callback(struct resource *res, void *arg)
    {
    struct crash_mem *cmem = arg;
    cmem.ranges[cmem.nr_ranges].start = res.start;
    cmem.ranges[cmem.nr_ranges].end = res.end;
    cmem.nr_ranges++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_crash_populate_cmem(cmem: *mut crash_mem) -> c_int {
    int arch_crash_populate_cmem(struct crash_mem *cmem)
    {
    return walk_system_ram_res(0, -1, cmem, prepare_elf64_ram_headers_callback);
    }

#[no_mangle]
unsafe extern "C" fn add_e820_entry(params: *mut boot_params, entry: *mut e820_entry) -> c_int {
    static int add_e820_entry(struct boot_params *params, struct e820_entry *entry)
    {
    unsigned int nr_e820_entries;
    nr_e820_entries = params.e820_entries;
    if (nr_e820_entries >= E820_MAX_ENTRIES_ZEROPAGE)
    return 1;
    memcpy(&params.e820_table[nr_e820_entries], entry, sizeof(struct e820_entry));
    params.e820_entries++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn memmap_entry_callback(res: *mut resource, arg: *mut c_void) -> c_int {
    static int memmap_entry_callback(struct resource *res, void *arg)
    {
    struct crash_memmap_data *cmd = arg;
    struct boot_params *params = cmd.params;
    struct e820_entry ei;
    ei.addr = res.start;
    ei.size = resource_size(res);
    ei.type = cmd.type;
    add_e820_entry(params, &ei);
    return 0;
    }
    static int memmap_exclude_ranges(struct kimage *image, struct crash_mem *cmem,
    unsigned long long mstart,
    unsigned long long mend)
    {
    unsigned long start, end;
    int ret;
    cmem.ranges[0].start = mstart;
    cmem.ranges[0].end = mend;
    cmem.nr_ranges = 1;
// Exclude elf header region
    start = image.elf_load_addr;
    end = start + image.elf_headers_sz - 1;
    ret = crash_exclude_mem_range(cmem, start, end);
    if (ret)
    return ret;
// Exclude dm crypt keys region
    if (image.dm_crypt_keys_addr) {
    start = image.dm_crypt_keys_addr;
    end = start + image.dm_crypt_keys_sz - 1;
    return crash_exclude_mem_range(cmem, start, end);
    }
    return ret;
    }
// Prepare memory map for crash dump kernel
#[no_mangle]
pub unsafe extern "C" fn crash_setup_memmap_entries(image: *mut kimage, params: *mut boot_params) -> c_int {
    int crash_setup_memmap_entries(struct kimage *image, struct boot_params *params)
    {
    let mut nr_ranges: c_uint = 0;
    int i, ret = 0;
    unsigned long flags;
    struct e820_entry ei;
    struct crash_memmap_data cmd;
    struct crash_mem *cmem;
//
// In the current x86 architecture code, the elfheader is always
// allocated at crashk_res.start. But it depends on the allocation
// position of elfheader in crashk_res. To avoid potential out of
// bounds in future, add an extra slot.
//
// And using random kexec_buf for passing dm crypt keys may cause a
// range split too, add another extra slot here.
//
    nr_ranges = 3;
    cmem = vzalloc(struct_size(cmem, ranges, nr_ranges));
    if (!cmem)
    return -ENOMEM;
    cmem.max_nr_ranges = nr_ranges;
    memset(&cmd, 0, sizeof(struct crash_memmap_data));
    cmd.params = params;
// Add the low 1M
    cmd.type = E820_TYPE_RAM;
    flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    walk_iomem_res_desc(IORES_DESC_NONE, flags, 0, (1<<20)-1, &cmd,
    memmap_entry_callback);
// Add ACPI tables
    cmd.type = E820_TYPE_ACPI;
    flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    walk_iomem_res_desc(IORES_DESC_ACPI_TABLES, flags, 0, -1, &cmd,
    memmap_entry_callback);
// Add ACPI Non-volatile Storage
    cmd.type = E820_TYPE_NVS;
    walk_iomem_res_desc(IORES_DESC_ACPI_NV_STORAGE, flags, 0, -1, &cmd,
    memmap_entry_callback);
// Add e820 reserved ranges
    cmd.type = E820_TYPE_RESERVED;
    flags = IORESOURCE_MEM;
    walk_iomem_res_desc(IORES_DESC_RESERVED, flags, 0, -1, &cmd,
    memmap_entry_callback);
// Add crashk_low_res region
    if (crashk_low_res.end) {
    ei.addr = crashk_low_res.start;
    ei.size = resource_size(&crashk_low_res);
    ei.type = E820_TYPE_RAM;
    add_e820_entry(params, &ei);
    }
// Exclude some ranges from crashk_res and add rest to memmap
    ret = memmap_exclude_ranges(image, cmem, crashk_res.start, crashk_res.end);
    if (ret)
    goto out;
    for (i = 0; i < cmem.nr_ranges; i++) {
    ei.size = cmem.ranges[i].end - cmem.ranges[i].start + 1;
// If entry is less than a page, skip it
    if (ei.size < PAGE_SIZE)
    continue;
    ei.addr = cmem.ranges[i].start;
    ei.type = E820_TYPE_RAM;
    add_e820_entry(params, &ei);
    }
    for (i = 0; i < crashk_cma_cnt; ++i) {
    ei.addr = crashk_cma_ranges[i].start;
    ei.size = crashk_cma_ranges[i].end -
    crashk_cma_ranges[i].start + 1;
    ei.type = E820_TYPE_RAM;
    add_e820_entry(params, &ei);
    }
    out:
    vfree(cmem);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn crash_load_segments(image: *mut kimage) -> c_int {
    int crash_load_segments(struct kimage *image)
    {
    int ret;
    let mut pnum: c_ulong = 0;
    struct kexec_buf kbuf = { .image = image, .buf_min = 0,
    .buf_max = ULONG_MAX, .top_down = false };
// Prepare elf headers and add a segment
    ret = crash_prepare_headers(IS_ENABLED(CONFIG_X86_64), &kbuf.buffer,
    &kbuf.bufsz, &pnum);
    if (ret)
    return ret;
    image.elf_headers	= kbuf.buffer;
    image.elf_headers_sz	= kbuf.bufsz;
    kbuf.memsz		= kbuf.bufsz;

//
// The elfcorehdr segment size accounts for VMCOREINFO, kernel_map,
// maximum CPUs and maximum memory ranges.
//
    if (IS_ENABLED(CONFIG_MEMORY_HOTPLUG))
    pnum = 2 + CONFIG_NR_CPUS_DEFAULT + CONFIG_CRASH_MAX_MEMORY_RANGES;
    else
    pnum += 2 + CONFIG_NR_CPUS_DEFAULT;
    if (pnum < (unsigned long)PN_XNUM) {
    kbuf.memsz = pnum * sizeof(Elf64_Phdr);
    kbuf.memsz += sizeof(Elf64_Ehdr);
    image.elfcorehdr_index = image.nr_segments;
// Mark as usable to crash kernel, else crash kernel fails on boot
    image.elf_headers_sz = kbuf.memsz;
    } else {
    pr_err("number of Phdrs %lu exceeds max\n", pnum);
    }

    kbuf.buf_align = ELF_CORE_HEADER_ALIGN;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    ret = kexec_add_buffer(&kbuf);
    if (ret)
    return ret;
    image.elf_load_addr = kbuf.mem;
    kexec_dprintk("Loaded ELF headers at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    image.elf_load_addr, kbuf.bufsz, kbuf.memsz);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn arch_crash_hotplug_support(image: *mut kimage, kexec_flags: c_ulong) -> c_int {
    int arch_crash_hotplug_support(struct kimage *image, unsigned long kexec_flags)
    {

    if (image.file_mode)
    return 1;

//
// Initially, crash hotplug support for kexec_load was added
// with the KEXEC_UPDATE_ELFCOREHDR flag. Later, this
// functionality was expanded to accommodate multiple kexec
// segment updates, leading to the introduction of the
// KEXEC_CRASH_HOTPLUG_SUPPORT kexec flag bit. Consequently,
// when the kexec tool sends either of these flags, it indicates
// that the required kexec segment (elfcorehdr) is excluded from
// the SHA calculation.
//
    return (kexec_flags & KEXEC_UPDATE_ELFCOREHDR ||
    kexec_flags & KEXEC_CRASH_HOTPLUG_SUPPORT);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_crash_get_elfcorehdr_size() -> c_uint {
    unsigned int arch_crash_get_elfcorehdr_size(void)
    {
    unsigned int sz;
// kernel_map, VMCOREINFO and maximum CPUs
    sz = 2 + CONFIG_NR_CPUS_DEFAULT;
    if (IS_ENABLED(CONFIG_MEMORY_HOTPLUG))
    sz += CONFIG_CRASH_MAX_MEMORY_RANGES;
    sz *= sizeof(Elf64_Phdr);
    return sz;
    }
//
// arch_crash_handle_hotplug_event() - Handle hotplug elfcorehdr changes
// @image: a pointer to kexec_crash_image
// @arg: struct memory_notify handler for memory hotplug case and
// NULL for CPU hotplug case.
//
// Prepare the new elfcorehdr and replace the existing elfcorehdr.
//
#[no_mangle]
pub unsafe extern "C" fn arch_crash_handle_hotplug_event(image: *mut kimage, arg: *mut c_void) {
    void arch_crash_handle_hotplug_event(struct kimage *image, void *arg)
    {
    void *elfbuf = core::ptr::null_mut(), *old_elfcorehdr;
    unsigned long mem, memsz;
    let mut elfsz: c_ulong = 0;
//
// As crash_prepare_elf64_headers() has already described all
// possible CPUs, there is no need to update the elfcorehdr
// for additional CPU changes.
//
    if ((image.file_mode || image.elfcorehdr_updated) &&
    ((image.hp_action == KEXEC_CRASH_HP_ADD_CPU) ||
    (image.hp_action == KEXEC_CRASH_HP_REMOVE_CPU)))
    return;
//
// Create the new elfcorehdr reflecting the changes to CPU and/or
// memory resources.
//
    if (crash_prepare_headers(IS_ENABLED(CONFIG_X86_64), &elfbuf, &elfsz, core::ptr::null_mut())) {
    pr_err("unable to create new elfcorehdr");
    goto out;
    }
//
// Obtain address and size of the elfcorehdr segment, and
// check it against the new elfcorehdr buffer.
//
    mem = image.segment[image.elfcorehdr_index].mem;
    memsz = image.segment[image.elfcorehdr_index].memsz;
    if (elfsz > memsz) {
    pr_err("update elfcorehdr elfsz %lu > memsz %lu",
    elfsz, memsz);
    goto out;
    }
//
// Copy new elfcorehdr over the old elfcorehdr at destination.
//
    old_elfcorehdr = kmap_local_page(pfn_to_page(mem >> PAGE_SHIFT));
    if (!old_elfcorehdr) {
    pr_err("mapping elfcorehdr segment failed\n");
    goto out;
    }
//
// Temporarily invalidate the crash image while the
// elfcorehdr is updated.
//
    xchg(&kexec_crash_image, core::ptr::null_mut());
    memcpy_flushcache(old_elfcorehdr, elfbuf, elfsz);
    xchg(&kexec_crash_image, image);
    kunmap_local(old_elfcorehdr);
    pr_debug("updated elfcorehdr\n");
    out:
    vfree(elfbuf);
    }
