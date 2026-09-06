//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/hibernate.c
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
// :
// Hibernate support specific for ARM64
//
// Derived from work on ARM hibernation support by:
//
// Ubuntu project, hibernation support for mach-dove
// Copyright (C) 2010 Nokia Corporation (Hiroshi Doyu)
// Copyright (C) 2010 Texas Instruments, Inc. (Teerth Reddy et al.)
// Copyright (C) 2006 Rafael J. Wysocki <rjw@sisk.pl>
//

//
// Hibernate core relies on this value being 0 on resume, and marks it
// __nosavedata assuming it will keep the resume kernel's '0' value. This
// doesn't happen with either KASLR.
//
// defined as "__visible int in_suspend __nosavedata" in
// kernel/power/hibernate.c
//
    extern int in_suspend;
// Do we need to reset el2?

// hyp-stub vectors, used to restore el2 during resume from hibernate.
    extern char __hyp_stub_vectors[];
//
// The logical cpu number we should resume on, initialised to a non-cpu
// number.
//
    let mut sleep_cpu: static int = -EINVAL;
//
// Values that may not change over hibernate/resume. We put the build number
// and date in here so that we guarantee not to resume with a different
// kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_hibernate_hdr_invariants {
    pub 1]: char uts_version[__NEW_UTS_LEN +,
}

// These values need to be know across a hibernate/restore.
    static struct arch_hibernate_hdr {
    struct arch_hibernate_hdr_invariants invariants;
// These are needed to find the relocated kernel if built with kaslr
    phys_addr_t	ttbr1_el1;
    void		(*reenter_kernel)(void);
//
// We need to know where the __hyp_stub_vectors are after restore to
// re-configure el2.
//
    phys_addr_t	__hyp_stub_vectors;
    u64		sleep_cpu_mpidr;
    } resume_hdr;
#[no_mangle]
pub unsafe extern "C" fn arch_hdr_invariants(i: *mut arch_hibernate_hdr_invariants) {
    static inline void arch_hdr_invariants(struct arch_hibernate_hdr_invariants *i)
    {
    memset(i, 0, sizeof(*i));
    memcpy(i.uts_version, init_utsname().version, sizeof(i.uts_version));
    }
#[no_mangle]
pub unsafe extern "C" fn pfn_is_nosave(pfn: c_ulong) -> c_int {
    int pfn_is_nosave(unsigned long pfn)
    {
    let mut nosave_begin_pfn: c_ulong = sym_to_pfn(&__nosave_begin);
    let mut nosave_end_pfn: c_ulong = sym_to_pfn(&__nosave_end - 1);
    return ((pfn >= nosave_begin_pfn) && (pfn <= nosave_end_pfn)) ||
    crash_is_nosave(pfn);
    }
#[no_mangle]
pub unsafe extern "C" fn save_processor_state() -> void notrace {
    void notrace save_processor_state(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn restore_processor_state() -> void notrace {
    void notrace restore_processor_state(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn arch_hibernation_header_save(addr: *mut c_void, max_size: c_uint) -> c_int {
    int arch_hibernation_header_save(void *addr, unsigned int max_size)
    {
    struct arch_hibernate_hdr *hdr = addr;
    if (max_size < sizeof(*hdr))
    return -EOVERFLOW;
    arch_hdr_invariants(&hdr.invariants);
    hdr.ttbr1_el1		= __pa_symbol(swapper_pg_dir);
    hdr.reenter_kernel	= _cpu_resume;
// We can't use __hyp_get_vectors() because kvm may still be loaded
    if (el2_reset_needed())
    hdr.__hyp_stub_vectors = __pa_symbol(__hyp_stub_vectors);
    else
    hdr.__hyp_stub_vectors = 0;
// Save the mpidr of the cpu we called cpu_suspend() on...
    if (sleep_cpu < 0) {
    pr_err("Failing to hibernate on an unknown CPU.\n");
    return -ENODEV;
    }
    hdr.sleep_cpu_mpidr = cpu_logical_map(sleep_cpu);
    pr_info("Hibernating on CPU %d [mpidr:0x%llx]\n", sleep_cpu,
    hdr.sleep_cpu_mpidr);
    return 0;
    }
    EXPORT_SYMBOL(arch_hibernation_header_save);
#[no_mangle]
pub unsafe extern "C" fn arch_hibernation_header_restore(addr: *mut c_void) -> c_int {
    int arch_hibernation_header_restore(void *addr)
    {
    int ret;
    struct arch_hibernate_hdr_invariants invariants;
    struct arch_hibernate_hdr *hdr = addr;
    arch_hdr_invariants(&invariants);
    if (memcmp(&hdr.invariants, &invariants, sizeof(invariants))) {
    pr_crit("Hibernate image not generated by this kernel!\n");
    return -EINVAL;
    }
    sleep_cpu = get_logical_index(hdr.sleep_cpu_mpidr);
    pr_info("Hibernated on CPU %d [mpidr:0x%llx]\n", sleep_cpu,
    hdr.sleep_cpu_mpidr);
    if (sleep_cpu < 0) {
    pr_crit("Hibernated on a CPU not known to this kernel!\n");
    sleep_cpu = -EINVAL;
    return -EINVAL;
    }
    ret = bringup_hibernate_cpu(sleep_cpu);
    if (ret) {
    sleep_cpu = -EINVAL;
    return ret;
    }
    resume_hdr = *hdr;
    return 0;
    }
    EXPORT_SYMBOL(arch_hibernation_header_restore);
    static void *hibernate_page_alloc(void *arg)
    {
    return (void *)get_safe_page(( gfp_t)(unsigned long)arg);
    }
//
// Copies length bytes, starting at src_start into an new page,
// perform cache maintenance, then maps it at the specified address low
// address as executable.
//
// This is used by hibernate to copy the code it needs to execute when
// overwriting the kernel text. This function generates a new set of page
// tables, which it loads into ttbr0.
//
// Length is provided as we probably only want 4K of data, even on a 64K
// page system.
//
    static int create_safe_exec_page(void *src_start, size_t length,
    phys_addr_t *phys_dst_addr)
    {
    struct trans_pgd_info trans_info = {
    .trans_alloc_page	= hibernate_page_alloc,
    .trans_alloc_arg	= ( void *)GFP_ATOMIC,
    };
    void *page = (void *)get_safe_page(GFP_ATOMIC);
    phys_addr_t trans_ttbr0;
    unsigned long t0sz;
    int rc;
    if (!page)
    return -ENOMEM;
    memcpy(page, src_start, length);
    caches_clean_inval_pou((unsigned long)page, (unsigned long)page + length);
    rc = trans_pgd_idmap_page(&trans_info, &trans_ttbr0, &t0sz, page);
    if (rc)
    return rc;
    cpu_install_ttbr0(trans_ttbr0, t0sz);
// phys_dst_addr = virt_to_phys(page);
    return 0;
    }

    static DEFINE_XARRAY(mte_pages);
#[no_mangle]
unsafe extern "C" fn save_tags(page: *mut page, pfn: c_ulong) -> c_int {
    static int save_tags(struct page *page, unsigned long pfn)
    {
    void *tag_storage, *ret;
    tag_storage = mte_allocate_tag_storage();
    if (!tag_storage)
    return -ENOMEM;
    mte_save_page_tags(page_address(page), tag_storage);
    ret = xa_store(&mte_pages, pfn, tag_storage, GFP_KERNEL);
    if (WARN(xa_is_err(ret), "Failed to store MTE tags")) {
    mte_free_tag_storage(tag_storage);
    return xa_err(ret);
    } else if (WARN(ret, "swsusp: %s: Duplicate entry", __func__)) {
    mte_free_tag_storage(ret);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn swsusp_mte_free_storage() {
    static void swsusp_mte_free_storage(void)
    {
    XA_STATE(xa_state, &mte_pages, 0);
    void *tags;
    xa_lock(&mte_pages);
    xas_for_each(&xa_state, tags, ULONG_MAX) {
    mte_free_tag_storage(tags);
    }
    xa_unlock(&mte_pages);
    xa_destroy(&mte_pages);
    }
#[no_mangle]
unsafe extern "C" fn swsusp_mte_save_tags() -> c_int {
    static int swsusp_mte_save_tags(void)
    {
    struct zone *zone;
    unsigned long pfn, max_zone_pfn;
    let mut ret: c_int = 0;
    let mut n: c_int = 0;
    if (!system_supports_mte())
    return 0;
    for_each_populated_zone(zone) {
    max_zone_pfn = zone_end_pfn(zone);
    for (pfn = zone.zone_start_pfn; pfn < max_zone_pfn; pfn++) {
    struct page *page = pfn_to_online_page(pfn);
    struct folio *folio;
    if (!page)
    continue;
    folio = page_folio(page);
    if (folio_test_hugetlb(folio) &&
    !folio_test_hugetlb_mte_tagged(folio))
    continue;
    if (!page_mte_tagged(page))
    continue;
    ret = save_tags(page, pfn);
    if (ret) {
    swsusp_mte_free_storage();
    goto out;
    }
    n++;
    }
    }
    pr_info("Saved %d MTE pages\n", n);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn swsusp_mte_restore_tags() {
    static void swsusp_mte_restore_tags(void)
    {
    XA_STATE(xa_state, &mte_pages, 0);
    let mut n: c_int = 0;
    void *tags;
    xa_lock(&mte_pages);
    xas_for_each(&xa_state, tags, ULONG_MAX) {
    let mut pfn: c_ulong = xa_state.xa_index;
    struct page *page = pfn_to_online_page(pfn);
    mte_restore_page_tags(page_address(page), tags);
    mte_free_tag_storage(tags);
    n++;
    }
    xa_unlock(&mte_pages);
    pr_info("Restored %d MTE pages\n", n);
    xa_destroy(&mte_pages);
    }

#[no_mangle]
unsafe extern "C" fn swsusp_mte_save_tags() -> c_int {
    static int swsusp_mte_save_tags(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn swsusp_mte_restore_tags() {
    static void swsusp_mte_restore_tags(void)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn swsusp_arch_suspend() -> c_int {
    int swsusp_arch_suspend(void)
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    struct sleep_stack_data state;
    if (cpus_are_stuck_in_kernel()) {
    pr_err("Can't hibernate: no mechanism to offline secondary CPUs.\n");
    return -EBUSY;
    }
    flags = local_daif_save();
    if (__cpu_suspend_enter(&state)) {
// make the crash dump kernel image visible/saveable
    crash_prepare_suspend();
    ret = swsusp_mte_save_tags();
    if (ret) {
    local_daif_restore(flags);
    return ret;
    }
    sleep_cpu = smp_processor_id();
    ret = swsusp_save();
    } else {
// Clean kernel core startup/idle code to PoC
    dcache_clean_inval_poc((unsigned long)__mmuoff_data_start,
    (unsigned long)__mmuoff_data_end);
    dcache_clean_inval_poc((unsigned long)__idmap_text_start,
    (unsigned long)__idmap_text_end);
// Clean kvm setup code to PoC?
    if (el2_reset_needed()) {
    dcache_clean_inval_poc(
    (unsigned long)__hyp_idmap_text_start,
    (unsigned long)__hyp_idmap_text_end);
    dcache_clean_inval_poc((unsigned long)__hyp_text_start,
    (unsigned long)__hyp_text_end);
    }
    swsusp_mte_restore_tags();
// make the crash dump kernel image protected again
    crash_post_resume();
//
// Tell the hibernation core that we've just restored
// the memory
//
    in_suspend = 0;
    sleep_cpu = -EINVAL;
    __cpu_suspend_exit();
//
// Just in case the boot kernel did turn the SSBD
// mitigation off behind our back, let's set the state
// to what we expect it to be.
//
    spectre_v4_enable_mitigation(core::ptr::null_mut());
    }
    local_daif_restore(flags);
    return ret;
    }
//
// Setup then Resume from the hibernate image using swsusp_arch_suspend_exit().
//
// Memory allocated by get_safe_page() will be dealt with by the hibernate code,
// we don't need to free it here.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_arch_resume() -> int __nocfi {
    int __nocfi swsusp_arch_resume(void)
    {
    int rc;
    void *zero_page;
    size_t exit_size;
    pgd_t *tmp_pg_dir;
    phys_addr_t el2_vectors;
    void __noreturn (*hibernate_exit)(phys_addr_t, phys_addr_t, void *,
    void *, phys_addr_t, phys_addr_t);
    struct trans_pgd_info trans_info = {
    .trans_alloc_page	= hibernate_page_alloc,
    .trans_alloc_arg	= ( void *)GFP_ATOMIC,
    };
//
// Restoring the memory image will overwrite the ttbr1 page tables.
// Create a second copy of just the linear map, and use this when
// restoring.
//
    rc = trans_pgd_create_copy(&trans_info, &tmp_pg_dir, PAGE_OFFSET,
    PAGE_END);
    if (rc)
    return rc;
//
// We need a zero page that is zero before & after resume in order
// to break before make on the ttbr1 page tables.
//
    zero_page = (void *)get_safe_page(GFP_ATOMIC);
    if (!zero_page) {
    pr_err("Failed to allocate zero page.\n");
    return -ENOMEM;
    }
    if (el2_reset_needed()) {
    rc = trans_pgd_copy_el2_vectors(&trans_info, &el2_vectors);
    if (rc) {
    pr_err("Failed to setup el2 vectors\n");
    return rc;
    }
    }
    exit_size = __hibernate_exit_text_end - __hibernate_exit_text_start;
//
// Copy swsusp_arch_suspend_exit() to a safe page. This will generate
// a new set of ttbr0 page tables and load them.
//
    rc = create_safe_exec_page(__hibernate_exit_text_start, exit_size,
    (phys_addr_t *)&hibernate_exit);
    if (rc) {
    pr_err("Failed to create safe executable page for hibernate_exit code.\n");
    return rc;
    }
//
// KASLR will cause the el2 vectors to be in a different location in
// the resumed kernel. Load hibernate's temporary copy into el2.
//
// We can skip this step if we booted at EL1, or are running with VHE.
//
    if (el2_reset_needed())
    __hyp_set_vectors(el2_vectors);
//
// It is necessary to mask all DAIF exceptions here as:
//
// - The copy of swsusp_arch_suspend_exit() in the hibernation
// text cannot handle taking any exceptions.
//
// - The suspended kernel masked all DAIF exceptions in
// swsusp_arch_resume(), and expects to be re-entered in the
// same state : with all DAIF exceptions masked.
//
    local_daif_save();
    hibernate_exit(virt_to_phys(tmp_pg_dir), resume_hdr.ttbr1_el1,
    resume_hdr.reenter_kernel, restore_pblist,
    resume_hdr.__hyp_stub_vectors, virt_to_phys(zero_page));
    unreachable();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_resume_nonboot_cpu_disable() -> c_int {
    int hibernate_resume_nonboot_cpu_disable(void)
    {
    if (sleep_cpu < 0) {
    pr_err("Failing to resume from hibernate on an unknown CPU.\n");
    return -ENODEV;
    }
    return freeze_secondary_cpus(sleep_cpu);
    }
