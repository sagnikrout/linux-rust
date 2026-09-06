//! Automatically rewritten from C to Rust
//! Source: lib/vdso/datastore.c
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

    static u8 vdso_initdata[VDSO_NR_PAGES * PAGE_SIZE] __aligned(PAGE_SIZE) __initdata = {};

    struct vdso_time_data *vdso_k_time_data __ro_after_init =
    (void *)&vdso_initdata[VDSO_TIME_PAGE_OFFSET * PAGE_SIZE];
    static_assert(sizeof(struct vdso_time_data) <= PAGE_SIZE);

    struct vdso_rng_data *vdso_k_rng_data __ro_after_init =
    (void *)&vdso_initdata[VDSO_RNG_PAGE_OFFSET * PAGE_SIZE];
    static_assert(sizeof(struct vdso_rng_data) <= PAGE_SIZE);

    struct vdso_arch_data *vdso_k_arch_data __ro_after_init =
    (void *)&vdso_initdata[VDSO_ARCH_PAGES_START * PAGE_SIZE];

    static struct page *vdso_data_pages __ro_after_init;
#[no_mangle]
pub unsafe extern "C" fn vdso_setup_data_pages() -> void __init {
    void __init vdso_setup_data_pages(void)
    {
    let mut order: c_uint = get_order(VDSO_NR_PAGES * PAGE_SIZE);
//
// Allocate the data pages dynamically. SPARC does not support mapping
// static pages to be mapped into userspace.
// It is also a requirement for mlockall() support.
//
// Do not use folios. In time namespaces the pages are mapped in a different order
// to userspace, which is not handled by the folio optimizations in finish_fault().
//
    vdso_data_pages = alloc_pages(GFP_KERNEL, order);
    if (!vdso_data_pages)
    panic("Unable to allocate VDSO storage pages");
// The pages are mapped one-by-one into userspace and each one needs to be refcounted.
    split_page(vdso_data_pages, order);
// Move the data already written by other subsystems to the new pages
    memcpy(page_address(vdso_data_pages), vdso_initdata, VDSO_NR_PAGES * PAGE_SIZE);
    if (IS_ENABLED(CONFIG_GENERIC_GETTIMEOFDAY))
    vdso_k_time_data = page_address(vdso_data_pages + VDSO_TIME_PAGE_OFFSET);
    if (IS_ENABLED(CONFIG_VDSO_GETRANDOM))
    vdso_k_rng_data = page_address(vdso_data_pages + VDSO_RNG_PAGE_OFFSET);
    if (IS_ENABLED(CONFIG_ARCH_HAS_VDSO_ARCH_DATA))
    vdso_k_arch_data = page_address(vdso_data_pages + VDSO_ARCH_PAGES_START);
    }
    static vm_fault_t vvar_fault(const struct vm_special_mapping *sm,
    struct vm_area_struct *vma, struct vm_fault *vmf)
    {
    struct page *page, *timens_page;
    if (unlikely(vmf.flags & FAULT_FLAG_REMOTE))
    return VM_FAULT_SIGBUS;
    page = vdso_data_pages + vmf.pgoff;
    timens_page = find_timens_vvar_page(vma);
    switch (vmf.pgoff) {
    case VDSO_TIME_PAGE_OFFSET:
    if (!IS_ENABLED(CONFIG_GENERIC_GETTIMEOFDAY) || !timens_page)
    break;
//
// Fault in VVAR page too, since it will be accessed
// to get clock data anyway.
//
    unsigned long addr;
    vm_fault_t err;
    addr = vmf.address + VDSO_TIMENS_PAGE_OFFSET * PAGE_SIZE;
    err = vmf_insert_page(vma, addr, page);
    if (unlikely(err & VM_FAULT_ERROR))
    return err;
    page = timens_page;
    break;
    case VDSO_TIMENS_PAGE_OFFSET:
//
// If a task belongs to a time namespace then a namespace
// specific VVAR is mapped with the VVAR_DATA_PAGE_OFFSET and
// the real VVAR page is mapped with the VVAR_TIMENS_PAGE_OFFSET
// offset.
// See also the comment near timens_setup_vdso_data().
//
    if (!IS_ENABLED(CONFIG_TIME_NS) || !timens_page)
    break;
    page = vdso_data_pages + VDSO_TIME_PAGE_OFFSET;
    break;
    case VDSO_RNG_PAGE_OFFSET:
    case VDSO_ARCH_PAGES_START ... VDSO_ARCH_PAGES_END:
    break;
    default:
    return VM_FAULT_SIGBUS;
    }
    get_page(page);
    vmf.page = page;
    return 0;
    }
    const struct vm_special_mapping vdso_vvar_mapping = {
    .name	= "[vvar]",
    .fault	= vvar_fault,
    };
    struct vm_area_struct *vdso_install_vvar_mapping(struct mm_struct *mm, unsigned long addr)
    {
    return _install_special_mapping(mm, addr, VDSO_NR_PAGES * PAGE_SIZE,
    VM_READ | VM_MAYREAD | VM_DONTDUMP |
    VM_MIXEDMAP | VM_SEALED_SYSMAP,
    &vdso_vvar_mapping);
    }
