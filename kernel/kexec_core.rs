//! Automatically rewritten from C to Rust
//! Source: kernel/kexec_core.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// kexec.c - kexec system call core code.
// Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
//

pub static mut __kexec_lock: core::sync::atomic::AtomicI32 = ATOMIC_INIT(0);
// Flag to indicate we are going to kexec a new kernel
pub static mut kexec_in_progress: bool = false;
    let mut kexec_file_dbg_print = 0;
//
// When kexec transitions to the new kernel there is a one-to-one
// mapping between physical and virtual addresses.  On processors
// where you can disable the MMU this is trivial, and easy.  For
// others it is still a simple predictable page table to setup.
//
// In that environment kexec copies the new kernel to its final
// resting place.  This means I can only support memory whose
// physical address can fit in an unsigned long.  In particular
// addresses where (pfn << PAGE_SHIFT) > ULONG_MAX cannot be handled.
// If the assembly stub has more restrictive requirements
// KEXEC_SOURCE_MEMORY_LIMIT and KEXEC_DEST_MEMORY_LIMIT can be
// defined more restrictively in <asm/kexec.h>.
//
// The code for the transition from the current kernel to the
// new kernel is placed in the control_code_buffer, whose size
// is given by KEXEC_CONTROL_PAGE_SIZE.  In the best case only a single
// page of memory is necessary, but some architectures require more.
// Because this memory must be identity mapped in the transition from
// virtual to physical addresses it must live in the range
// 0 - TASK_SIZE, as only the user space mappings are arbitrarily
// modifiable.
//
// The assembly stub in the control code buffer is passed a linked list
// of descriptor pages detailing the source pages of the new kernel,
// and the destination addresses of those source pages.  As this data
// structure is not used in the context of the current OS, it must
// be self-contained.
//
// The code has been made to work with highmem pages and will use a
// destination page in its final resting place (if it happens
// to allocate it).  The end product of this is that most of the
// physical address space, and most of RAM can be used.
//
// Future directions include:
// - allocating a page table with the control code buffer identity
// mapped, to simplify machine_kexec and make kexec_on_panic more
// reliable.
//
// KIMAGE_NO_DEST is an impossible destination address..., for
// allocating pages whose destination address we do not care about.
//

    static struct page *kimage_alloc_page(struct kimage *image,
    gfp_t gfp_mask,
    unsigned long dest);
#[no_mangle]
pub unsafe extern "C" fn sanity_check_segment_list(image: *mut kimage) -> c_int {
    let mut i = 0;
pub static mut nr_segments: c_ulong = image.nr_segments;
pub static mut total_pages: c_ulong = 0;
pub static mut nr_pages: c_ulong = totalram_pages();
//
// Verify we have good destination addresses.  The caller is
// responsible for making certain we don't attempt to load
// the new image into invalid or reserved areas of RAM.  This
// just verifies it is an address we can use.
//
// Since the kernel does everything in page size chunks ensure
// the destination addresses are page aligned.  Too many
// special cases crop of when we don't do this.  The most
// insidious is getting overlapping destination addresses
// simply because addresses are changed to page size
// granularity.
//
    for (i = 0; i < nr_segments; i++) {
    unsigned long mstart, mend;
    mstart = image.segment[i].mem;
    mend   = mstart + image.segment[i].memsz;
    if (mstart > mend) {
    return -EADDRNOTAVAIL;
    }
    if ((mstart & ~PAGE_MASK) || (mend & ~PAGE_MASK)) {
    return -EADDRNOTAVAIL;
    }
    if (mend >= KEXEC_DESTINATION_MEMORY_LIMIT) {
    return -EADDRNOTAVAIL;
    }
    }
// Verify our destination addresses do not overlap.
// If we alloed overlapping destination addresses
// through very weird things can happen with no
// easy explanation as one segment stops on another.
//
    for (i = 0; i < nr_segments; i++) {
    unsigned long mstart, mend;
    let mut j = 0;
    mstart = image.segment[i].mem;
    mend   = mstart + image.segment[i].memsz;
    for (j = 0; j < i; j++) {
    unsigned long pstart, pend;
    pstart = image.segment[j].mem;
    pend   = pstart + image.segment[j].memsz;
// Do the segments overlap ?
    if ((mend > pstart) && (mstart < pend)) {
    return -EINVAL;
    }
    }
    }
// Ensure our buffer sizes are strictly less than
// our memory sizes.  This should always be the case,
// and it is easier to check up front than to be surprised
// later on.
//
    for (i = 0; i < nr_segments; i++) {
    if (image.segment[i].bufsz > image.segment[i].memsz) {
    return -EINVAL;
    }
    }
//
// Verify that no more than half of memory will be consumed. If the
// request from userspace is too large, a large amount of time will be
// wasted allocating pages, which can cause a soft lockup.
//
    for (i = 0; i < nr_segments; i++) {
    if (PAGE_COUNT(image.segment[i].memsz) > nr_pages / 2) {
    return -EINVAL;
    }
    total_pages += PAGE_COUNT(image.segment[i].memsz);
    }
    if (total_pages > nr_pages / 2) {
    return -EINVAL;
    }

//
// Verify we have good destination addresses.  Normally
// the caller is responsible for making certain we don't
// attempt to load the new image into invalid or reserved
// areas of RAM.  But crash kernels are preloaded into a
// reserved area of ram.  We must ensure the addresses
// are in the reserved area otherwise preloading the
// kernel could corrupt things.
//
    if (image.type == KEXEC_TYPE_CRASH) {
    for (i = 0; i < nr_segments; i++) {
    unsigned long mstart, mend;
    mstart = image.segment[i].mem;
    mend = mstart + image.segment[i].memsz - 1;
// Ensure we are within the crash kernel limits
    if ((mstart < phys_to_boot_phys(crashk_res.start)) ||
    (mend > phys_to_boot_phys(crashk_res.end)))
    return -EADDRNOTAVAIL;
    }
    }

//
// The destination addresses are searched from system RAM rather than
// being allocated from the buddy allocator, so they are not guaranteed
// to be accepted by the current kernel.  Accept the destination
// addresses before kexec swaps their content with the segments' source
// pages to avoid accessing memory before it is accepted.
//
    for (i = 0; i < nr_segments; i++)
    accept_memory(image.segment[i].mem, image.segment[i].memsz);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn do_kimage_alloc_init() {
    let mut image = core::ptr::null_mut();
// Allocate a controlling structure
    image = kzalloc_obj(*image);
    if (!image) {
    return core::ptr::null_mut();
    }
    image.entry = &image.head;
    image.last_entry = &image.head;
    image.control_page = ~0; /* By default this does not apply */
    image.type = KEXEC_TYPE_DEFAULT;
// Initialize the list of control pages
// INIT_LIST_HEAD;
// Initialize the list of destination pages
// INIT_LIST_HEAD;
// Initialize the list of unusable pages
// INIT_LIST_HEAD;

    image.hp_action = KEXEC_CRASH_HP_NONE;
    image.elfcorehdr_index = -1;
    image.elfcorehdr_updated = false;

    return image;
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_is_destination_range() {
    let mut i = 0;
    for (i = 0; i < image.nr_segments; i++) {
    unsigned long mstart, mend;
    mstart = image.segment[i].mem;
    mend = mstart + image.segment[i].memsz - 1;
    if ((end >= mstart) && (start <= mend)) {
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_alloc_pages() {
    let mut pages = core::ptr::null_mut();
    if (fatal_signal_pending(current)) {
    return core::ptr::null_mut();
    }
    pages = alloc_pages(gfp_mask & ~__GFP_ZERO, order);
    if (pages) {
    unsigned int count, i;
    pages.mapping = core::ptr::null_mut();
    set_page_private(pages, order);
    count = 1 << order;
    for (i = 0; i < count; i++)
    SetPageReserved(pages + i);
    arch_kexec_post_alloc_pages(page_address(pages), count,
    gfp_mask);
    if (gfp_mask & __GFP_ZERO) {
    for (i = 0; i < count; i++)
    }
    clear_highpage(pages + i);
    }
    return pages;
    }
#[no_mangle]
unsafe extern "C" fn kimage_free_pages(page: *mut page) {
    unsigned int order, count, i;
    order = page_private(page);
    count = 1 << order;
    arch_kexec_pre_free_pages(page_address(page), count);
    for (i = 0; i < count; i++)
    ClearPageReserved(page + i);
    __free_pages(page, order);
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_free_page_list(list: *mut list_head) {
    struct page *page, *next;
    list_for_each_entry_safe(page, next, list, lru) {
    list_del(&page.lru);
    kimage_free_pages(page);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_alloc_normal_control_pages() {
// Control pages are special, they are the intermediaries
// that are needed while we copy the rest of the pages
// to their final resting place.  As such they must
// not conflict with either the destination addresses
// or memory the kernel is already using.
//
// The only case where we really need more than one of
// these are for architectures where we cannot disable
// the MMU and must instead generate an identity mapped
// page table for all of the memory.
//
// At worst this runs in O(N) of the image size.
//
    let mut extra_pages;
    let mut pages = core::ptr::null_mut();
    let mut count = 0;
    count = 1 << order;
// INIT_LIST_HEAD;
// Loop while I can allocate a page and the page allocated
// is a destination page.
//
    do {
    unsigned long pfn, epfn, addr, eaddr;
    pages = kimage_alloc_pages(KEXEC_CONTROL_MEMORY_GFP, order);
    if (!pages) {
    break;
    }
    pfn   = page_to_boot_pfn(pages);
    epfn  = pfn + count;
    addr  = pfn << PAGE_SHIFT;
    eaddr = (epfn << PAGE_SHIFT) - 1;
    if ((epfn >= (KEXEC_CONTROL_MEMORY_LIMIT >> PAGE_SHIFT)) ||
    kimage_is_destination_range(image, addr, eaddr)) {
    list_add(&pages.lru, &extra_pages);
    pages = core::ptr::null_mut();
    }
    } while (!pages);
    if (pages) {
// Remember the allocated page...
    list_add(&pages.lru, &image.control_pages);
// Because the page is already in it's destination
// location we will never allocate another page at
// that address.  Therefore kimage_alloc_pages
// will not return it (again) and we don't need
// to give it an entry in image->segment[].
//
    }
// Deal with the destination pages I have inadvertently allocated.
//
// Ideally I would convert multi-page allocations into single
// page allocations, and add everything to image->dest_pages.
//
// For now it is simpler to just free the pages.
//
    kimage_free_page_list(&extra_pages);
    return pages;
    }

#[no_mangle]
pub unsafe extern "C" fn kimage_alloc_crash_control_pages() {
// Control pages are special, they are the intermediaries
// that are needed while we copy the rest of the pages
// to their final resting place.  As such they must
// not conflict with either the destination addresses
// or memory the kernel is already using.
//
// Control pages are also the only pags we must allocate
// when loading a crash kernel.  All of the other pages
// are specified by the segments and we just memcpy
// into them directly.
//
// The only case where we really need more than one of
// these are for architectures where we cannot disable
// the MMU and must instead generate an identity mapped
// page table for all of the memory.
//
// Given the low demand this implements a very simple
// allocator that finds the first hole of the appropriate
// size in the reserved memory region, and allocates all
// of the memory up to and including the hole.
//
    unsigned long hole_start, hole_end, size;
    let mut pages = core::ptr::null_mut();
    pages = core::ptr::null_mut();
    size = (1 << order) << PAGE_SHIFT;
    hole_start = ALIGN(image.control_page, size);
    hole_end   = hole_start + size - 1;
    while (hole_end <= crashk_res.end) {
    let mut i = 0;
    cond_resched();
    if (hole_end > KEXEC_CRASH_CONTROL_MEMORY_LIMIT) {
    break;
    }
// See if I overlap any of the segments
    for (i = 0; i < image.nr_segments; i++) {
    unsigned long mstart, mend;
    mstart = image.segment[i].mem;
    mend   = mstart + image.segment[i].memsz - 1;
    if ((hole_end >= mstart) && (hole_start <= mend)) {
// Advance the hole to the end of the segment
    hole_start = ALIGN(mend, size);
    hole_end   = hole_start + size - 1;
    break;
    }
    }
// If I don't overlap any segments I have found my hole!
    if (i == image.nr_segments) {
    pages = pfn_to_page(hole_start >> PAGE_SHIFT);
    image.control_page = hole_end + 1;
    break;
    }
    }
// Ensure that these pages are decrypted if SME is enabled.
    if (pages) {
    arch_kexec_post_alloc_pages(page_address(pages), 1 << order, 0);
    }
    return pages;
    }

#[no_mangle]
pub unsafe extern "C" fn kimage_alloc_control_pages() {
    struct page *pages = core::ptr::null_mut();
    match (image.type) {
    KEXEC_TYPE_DEFAULT => {
    pages = kimage_alloc_normal_control_pages(image, order);
    break;

    KEXEC_TYPE_CRASH => {
    pages = kimage_alloc_crash_control_pages(image, order);
    break;

    }
    return pages;
    }
#[no_mangle]
unsafe extern "C" fn kimage_add_entry(image: *mut kimage, entry: kimage_entry_t) -> c_int {
    if (*image.entry != 0) {
    image.entry++;
    }
    if (image.entry == image.last_entry) {
    let mut ind_page = core::ptr::null_mut();
    let mut page = core::ptr::null_mut();
    page = kimage_alloc_page(image, GFP_KERNEL, KIMAGE_NO_DEST);
    if (!page) {
    return -ENOMEM;
    }
    ind_page = page_address(page);
// image->entry = virt_to_boot_phys(ind_page) | IND_INDIRECTION;
    image.entry = ind_page;
    image.last_entry = ind_page +
    ((PAGE_SIZE/sizeof(kimage_entry_t)) - 1);
    }
// image->entry = entry;
    image.entry++;
// image->entry = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_set_destination() {
    destination &= PAGE_MASK;
    return kimage_add_entry(image, destination | IND_DESTINATION);
    }
#[no_mangle]
unsafe extern "C" fn kimage_add_page(image: *mut kimage, page: c_ulong) -> c_int {
    page &= PAGE_MASK;
    return kimage_add_entry(image, page | IND_SOURCE);
    }
#[no_mangle]
unsafe extern "C" fn kimage_free_extra_pages(image: *mut kimage) {
// Walk through and free any extra destination pages I may have
    kimage_free_page_list(&image.dest_pages);
// Walk through and free any unusable pages I have cached
    kimage_free_page_list(&image.unusable_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_terminate(image: *mut kimage) {
    if (*image.entry != 0) {
    image.entry++;
    }
// image->entry = IND_DONE;
    }

    for (ptr = &image.head; (entry = *ptr) && !(entry & IND_DONE); 
    ptr = (entry & IND_INDIRECTION) ? 
    boot_phys_to_virt((entry & PAGE_MASK)) : ptr + 1)
#[no_mangle]
unsafe extern "C" fn kimage_free_entry(entry: kimage_entry_t) {
    let mut page = core::ptr::null_mut();
    page = boot_pfn_to_page(entry >> PAGE_SHIFT);
    kimage_free_pages(page);
    }
#[no_mangle]
unsafe extern "C" fn kimage_free_cma(image: *mut kimage) {
    let mut i = 0;
    for (i = 0; i < image.nr_segments; i++) {
    struct page *cma = image.segment_cma[i];
pub static mut nr_pages: u32 = image.segment[i].memsz >> PAGE_SHIFT;
    if (!cma) {
    continue;
    }
    arch_kexec_pre_free_pages(page_address(cma), nr_pages);
    dma_release_from_contiguous(core::ptr::null_mut(), cma, nr_pages);
    image.segment_cma[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_free(image: *mut kimage) {
    kimage_entry_t *ptr, entry;
pub static mut ind: kimage_entry_t = 0;
    if (!image) {
    return;
    }

    if (image.vmcoreinfo_data_copy) {
    crash_update_vmcoreinfo_safecopy(core::ptr::null_mut());
    vunmap(image.vmcoreinfo_data_copy);
    }

    kimage_free_extra_pages(image);
    for_each_kimage_entry(image, ptr, entry) {
    if (entry & IND_INDIRECTION) {
// Free the previous indirection page
    if (ind & IND_INDIRECTION) {
    kimage_free_entry(ind);
    }
// Save this indirection page until we are
// done with it.
//
    ind = entry;
    } else if (entry & IND_SOURCE)
    kimage_free_entry(entry);
    }
// Free the final indirection page
    if (ind & IND_INDIRECTION) {
    kimage_free_entry(ind);
    }
// Handle any machine specific cleanup
    machine_kexec_cleanup(image);
// Free the kexec control pages...
    kimage_free_page_list(&image.control_pages);
// Free CMA allocations
    kimage_free_cma(image);
//
// Free up any temporary buffers allocated. This might hit if
// error occurred much later after buffer allocation.
//
    if (image.file_mode) {
    kimage_file_post_load_cleanup(image);
    }
    kfree(image);
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_dst_used() {
    kimage_entry_t *ptr, entry;
pub static mut destination: c_ulong = 0;
    for_each_kimage_entry(image, ptr, entry) {
    if (entry & IND_DESTINATION) {
    destination = entry & PAGE_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn if(IND_SOURCE: entry &) -> else {
    if (page == destination) {
    return ptr;
    }
    destination += PAGE_SIZE;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_alloc_page() {
//
// Here we implement safeguards to ensure that a source page
// is not copied to its destination page before the data on
// the destination page is no longer useful.
//
// To do this we maintain the invariant that a source page is
// either its own destination page, or it is not a
// destination page at all.
//
// That is slightly stronger than required, but the proof
// that no problems will not occur is trivial, and the
// implementation is simply to verify.
//
// When allocating all pages normally this algorithm will run
// in O(N) time, but in the worst case it will run in O(N^2)
// time.   If the runtime is a problem the data structures can
// be fixed.
//
    let mut page = core::ptr::null_mut();
    let mut addr = 0;
//
// Walk through the list of destination pages, and see if I
// have a match.
//
    list_for_each_entry(page, &image.dest_pages, lru) {
    addr = page_to_boot_pfn(page) << PAGE_SHIFT;
    if (addr == destination) {
    list_del(&page.lru);
    return page;
    }
    }
    page = core::ptr::null_mut();
    while (1) {
    let mut old = core::ptr::null_mut();
// Allocate a page, if we run out of memory give up
    page = kimage_alloc_pages(gfp_mask, 0);
    if (!page) {
    return core::ptr::null_mut();
    }
// If the page cannot be used file it away
    if (page_to_boot_pfn(page) >
    (KEXEC_SOURCE_MEMORY_LIMIT >> PAGE_SHIFT)) {
    list_add(&page.lru, &image.unusable_pages);
    continue;
    }
    addr = page_to_boot_pfn(page) << PAGE_SHIFT;
// If it is the destination page we want use it
    if (addr == destination) {
    break;
    }
// If the page is not a destination page use it
    if (!kimage_is_destination_range(image, addr,
    addr + PAGE_SIZE - 1))
    break;
//
// I know that the page is someones destination page.
// See if there is already a source page for this
// destination page.  And if so swap the source pages.
//
    old = kimage_dst_used(image, addr);
    if (old) {
// If so move it
    let mut old_addr = 0;
    let mut old_page = core::ptr::null_mut();
    old_addr = *old & PAGE_MASK;
    old_page = boot_pfn_to_page(old_addr >> PAGE_SHIFT);
    copy_highpage(page, old_page);
// old = addr | (*old & ~PAGE_MASK);
// The old page I have found cannot be a
// destination page, so return it if it's
// gfp_flags honor the ones passed in.
//
    if (!(gfp_mask & __GFP_HIGHMEM) &&
    PageHighMem(old_page)) {
    kimage_free_pages(old_page);
    continue;
    }
    page = old_page;
    break;
    }
// Place the page on the destination list, to be used later
    list_add(&page.lru, &image.dest_pages);
    }
    return page;
    }
#[no_mangle]
unsafe extern "C" fn kimage_load_cma_segment(image: *mut kimage, idx: c_int) -> c_int {
    struct kexec_segment *segment = &image.segment[idx];
    struct page *cma = image.segment_cma[idx];
    char *ptr = page_address(cma);
    size_t ubytes, mbytes;
pub static mut result: c_int = 0;
    unsigned char __user *buf = core::ptr::null_mut();
    unsigned char *kbuf = core::ptr::null_mut();
    if (image.file_mode) {
    kbuf = segment.kbuf;
    }
    else {
    buf = segment.buf;
    }
    ubytes = segment.bufsz;
    mbytes = segment.memsz;
// Then copy from source buffer to the CMA one
    while (mbytes) {
    size_t uchunk, mchunk;
    mchunk = min_t(size_t, mbytes, PAGE_SIZE);
    uchunk = min(ubytes, mchunk);
    if (uchunk) {
// For file based kexec, source pages are in kernel memory
    if (image.file_mode) {
    memcpy(ptr, kbuf, uchunk);
    }
    else {
    result = copy_from_user(ptr, buf, uchunk);
    }
    ubytes -= uchunk;
    if (image.file_mode) {
    kbuf += uchunk;
    }
    else {
    buf += uchunk;
    }
    }
    if (result) {
    result = -EFAULT;
    goto out;
    }
    ptr    += mchunk;
    mbytes -= mchunk;
    cond_resched();
    }
// Clear any remainder
    memset(ptr, 0, mbytes);
    out:
    return result;
    }
#[no_mangle]
unsafe extern "C" fn kimage_load_normal_segment(image: *mut kimage, idx: c_int) -> c_int {
    struct kexec_segment *segment = &image.segment[idx];
    let mut maddr = 0;
    size_t ubytes, mbytes;
    let mut result = 0;
    unsigned char __user *buf = core::ptr::null_mut();
    unsigned char *kbuf = core::ptr::null_mut();
    if (image.file_mode) {
    kbuf = segment.kbuf;
    }
    else {
    buf = segment.buf;
    }
    ubytes = segment.bufsz;
    mbytes = segment.memsz;
    maddr = segment.mem;
    if (image.segment_cma[idx]) {
    return kimage_load_cma_segment(image, idx);
    }
    result = kimage_set_destination(image, maddr);
    if (result < 0) {
    goto out;
    }
    while (mbytes) {
    let mut page = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
    size_t uchunk, mchunk;
    page = kimage_alloc_page(image, GFP_HIGHUSER, maddr);
    if (!page) {
    result  = -ENOMEM;
    goto out;
    }
    result = kimage_add_page(image, page_to_boot_pfn(page)
    << PAGE_SHIFT);
    if (result < 0) {
    goto out;
    }
    ptr = kmap_local_page(page);
// Start with a clear page
    clear_page(ptr);
    mchunk = min_t(size_t, mbytes, PAGE_SIZE);
    uchunk = min(ubytes, mchunk);
    if (uchunk) {
// For file based kexec, source pages are in kernel memory
    if (image.file_mode) {
    memcpy(ptr, kbuf, uchunk);
    }
    else {
    result = copy_from_user(ptr, buf, uchunk);
    }
    ubytes -= uchunk;
    if (image.file_mode) {
    kbuf += uchunk;
    }
    else {
    buf += uchunk;
    }
    }
    kunmap_local(ptr);
    if (result) {
    result = -EFAULT;
    goto out;
    }
    maddr  += mchunk;
    mbytes -= mchunk;
    cond_resched();
    }
    out:
    return result;
    }

#[no_mangle]
unsafe extern "C" fn kimage_load_crash_segment(image: *mut kimage, idx: c_int) -> c_int {
// For crash dumps kernels we simply copy the data from
// user space to it's destination.
// We do things a page at a time for the sake of kmap.
//
    struct kexec_segment *segment = &image.segment[idx];
    let mut maddr = 0;
    size_t ubytes, mbytes;
    let mut result = 0;
    unsigned char __user *buf = core::ptr::null_mut();
    unsigned char *kbuf = core::ptr::null_mut();
    result = 0;
    if (image.file_mode) {
    kbuf = segment.kbuf;
    }
    else {
    buf = segment.buf;
    }
    ubytes = segment.bufsz;
    mbytes = segment.memsz;
    maddr = segment.mem;
    while (mbytes) {
    let mut page = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
    size_t uchunk, mchunk;
    page = boot_pfn_to_page(maddr >> PAGE_SHIFT);
    if (!page) {
    result  = -ENOMEM;
    goto out;
    }
    arch_kexec_post_alloc_pages(page_address(page), 1, 0);
    ptr = kmap_local_page(page);
    mchunk = min_t(size_t, mbytes, PAGE_SIZE);
    uchunk = min(ubytes, mchunk);
    if (mchunk > uchunk) {
// Zero the trailing part of the page
    memset(ptr + uchunk, 0, mchunk - uchunk);
    }
    if (uchunk) {
// For file based kexec, source pages are in kernel memory
    if (image.file_mode) {
    memcpy(ptr, kbuf, uchunk);
    }
    else {
    result = copy_from_user(ptr, buf, uchunk);
    }
    ubytes -= uchunk;
    if (image.file_mode) {
    kbuf += uchunk;
    }
    else {
    buf += uchunk;
    }
    }
    kexec_flush_icache_page(page);
    kunmap_local(ptr);
    arch_kexec_pre_free_pages(page_address(page), 1);
    if (result) {
    result = -EFAULT;
    goto out;
    }
    maddr  += mchunk;
    mbytes -= mchunk;
    cond_resched();
    }
    out:
    return result;
    }

#[no_mangle]
pub unsafe extern "C" fn kimage_load_segment(image: *mut kimage, idx: c_int) -> c_int {
pub static mut result: c_int = -ENOMEM;
    match (image.type) {
    KEXEC_TYPE_DEFAULT => {
    result = kimage_load_normal_segment(image, idx);
    break;

    KEXEC_TYPE_CRASH => {
    result = kimage_load_crash_segment(image, idx);
    break;

    }
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_map_segment() {
    unsigned long addr, size, eaddr;
    unsigned long src_page_addr, dest_page_addr = 0;
    kimage_entry_t *ptr, entry;
    let mut src_pages = core::ptr::null_mut();
    let mut npages = 0;
    let mut cma = core::ptr::null_mut();
    void *vaddr = core::ptr::null_mut();
    let mut i = 0;
    cma = image.segment_cma[idx];
    if (cma) {
    return page_address(cma);
    }
    addr = image.segment[idx].mem;
    size = image.segment[idx].memsz;
    eaddr = addr + size;
//
// Collect the source pages and map them in a contiguous VA range.
//
    npages = PFN_UP(eaddr) - PFN_DOWN(addr);
    src_pages = kmalloc_objs(*src_pages, npages);
    if (!src_pages) {
    pr_err("Could not allocate ima pages array.\n");
    return core::ptr::null_mut();
    }
    i = 0;
    for_each_kimage_entry(image, ptr, entry) {
    if (entry & IND_DESTINATION) {
    dest_page_addr = entry & PAGE_MASK;
    } else if (entry & IND_SOURCE) {
    if (dest_page_addr >= addr && dest_page_addr < eaddr) {
    src_page_addr = entry & PAGE_MASK;
    src_pages[i++] =
    virt_to_page(__va(src_page_addr));
    if (i == npages) {
    break;
    }
    dest_page_addr += PAGE_SIZE;
    }
    }
    }
// Sanity check.
// WARN_ON;
    vaddr = vmap(src_pages, npages, VM_MAP, PAGE_KERNEL);
    kfree(src_pages);
    if (!vaddr) {
    pr_err("Could not map ima buffer.\n");
    }
    return vaddr;
    }
#[no_mangle]
pub unsafe extern "C" fn kimage_unmap_segment(segment_buffer: *mut c_void) {
    if (is_vmalloc_addr(segment_buffer)) {
    vunmap(segment_buffer);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_load_limit {
// Mutex protects the limit count.
    pub mutex: mutex,
    pub limit: c_int,
}

pub static mut kexec_load_limit: usize = 0;
pub static mut kexec_load_limit: usize = 0;
    let mut kexec_image = core::ptr::null_mut();
    let mut kexec_crash_image = core::ptr::null_mut();
    static int kexec_load_disabled;

#[no_mangle]
pub unsafe extern "C" fn kexec_limit_handler() {
    struct kexec_load_limit *limit = table.data;
    let mut val = 0;
    struct ctl_table tmp = {
    .data = &val,
    .maxlen = sizeof(val),
    .mode = table.mode,
    };
    let mut ret = 0;
    if (write) {
    ret = proc_dointvec(&tmp, write, buffer, lenp, ppos);
    if (ret) {
    return ret;
    }
    if (val < 0) {
    return -EINVAL;
    }
    mutex_lock(&limit.mutex);
    if (limit.limit != -1 && val >= limit.limit) {
    ret = -EINVAL;
    }
    else {
    limit.limit = val;
    }
    mutex_unlock(&limit.mutex);
    return ret;
    }
    mutex_lock(&limit.mutex);
    val = limit.limit;
    mutex_unlock(&limit.mutex);
    return proc_dointvec(&tmp, write, buffer, lenp, ppos);
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn kexec_core_sysctl_init() -> c_int {
    register_sysctl_init("kernel", kexec_core_sysctls);
    return 0;
    }
// late_initcall;

#[no_mangle]
pub unsafe extern "C" fn kexec_load_permitted(kexec_image_type: c_int) -> bool {
    let mut limit = core::ptr::null_mut();
//
// Only the superuser can use the kexec syscall and if it has not
// been disabled.
//
    if (!capable(CAP_SYS_BOOT) || kexec_load_disabled) {
    return false;
    }
// Check limit counter and decrease it.
    limit = (kexec_image_type == KEXEC_TYPE_CRASH) ?
    &load_limit_panic : &load_limit_reboot;
    mutex_lock(&limit.mutex);
    if (!limit.limit) {
    mutex_unlock(&limit.mutex);
    return false;
    }
    if (limit.limit != -1) {
    limit.limit--;
    }
    mutex_unlock(&limit.mutex);
    return true;
    }
//
// Move into place and start executing a preloaded standalone
// executable.  If nothing was preloaded return an error.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_kexec() -> c_int {
pub static mut error: c_int = 0;
    if (!kexec_trylock()) {
    return -EBUSY;
    }
    if (!kexec_image) {
    error = -EINVAL;
    goto Unlock;
    }
    if (!kexec_image.preserve_context) {
    error = liveupdate_reboot();
    if (error) {
    goto Unlock;
    }
    }

    if (kexec_image.preserve_context) {
//
// This flow is analogous to hibernation flows that occur
// before creating an image and before jumping from the
// restore kernel to the image one, so it uses the same
// device callbacks as those two flows.
//
    pm_prepare_console();
    error = freeze_processes();
    if (error) {
    error = -EBUSY;
    goto Restore_console;
    }
    console_suspend_all();
    error = dpm_suspend_start(PMSG_FREEZE);
    if (error) {
    goto Resume_devices;
    }
//
// dpm_suspend_end() must be called after dpm_suspend_start()
// to complete the transition, like in the hibernation flows
// mentioned above.
//
    error = dpm_suspend_end(PMSG_FREEZE);
    if (error) {
    goto Resume_devices;
    }
    error = suspend_disable_secondary_cpus();
    if (error) {
    goto Enable_cpus;
    }
    local_irq_disable();
    error = syscore_suspend();
    if (error) {
    goto Enable_irqs;
    }
    } else {

    {
    }
    kexec_in_progress = true;
    kernel_restart_prepare("kexec reboot");
    migrate_to_reboot_cpu();
    syscore_shutdown();
//
// migrate_to_reboot_cpu() disables CPU hotplug assuming that
// no further code needs to use CPU hotplug (which is true in
// the reboot case). However, the kexec path depends on using
// CPU hotplug again; so re-enable it here.
//
    cpu_hotplug_enable();
    pr_notice("Starting new kernel\n");
    machine_shutdown();
    }
    kmsg_dump(KMSG_DUMP_SHUTDOWN);
    machine_kexec(kexec_image);

    if (kexec_image.preserve_context) {
//
// This flow is analogous to hibernation flows that occur after
// creating an image and after the image kernel has got control
// back, and in case the devices have been reset or otherwise
// manipulated in the meantime, it uses the device callbacks
// used by the latter.
//
    syscore_resume();
    Enable_irqs:
    local_irq_enable();
    Enable_cpus:
    suspend_enable_secondary_cpus();
    dpm_resume_start(PMSG_RESTORE);
    Resume_devices:
    dpm_resume_end(PMSG_RESTORE);
    console_resume_all();
    thaw_processes();
    Restore_console:
    pm_restore_console();
    }

    Unlock:
    kexec_unlock();
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn loaded_show() {
    return sysfs_emit(buf, "%d\n", !!kexec_image);
    }
pub static mut loaded_attr: kobj_attribute = __ATTR_RO(loaded);

#[no_mangle]
pub unsafe extern "C" fn crash_loaded_show() {
    return sysfs_emit(buf, "%d\n", kexec_crash_loaded());
    }
pub static mut crash_loaded_attr: kobj_attribute = __ATTR_RO(crash_loaded);

#[no_mangle]
pub unsafe extern "C" fn crash_cma_ranges_show() {
pub static mut len: isize = 0;
    let mut i = 0;
    for (i = 0; i < crashk_cma_cnt; ++i) {
    len += sysfs_emit_at(buf, len, "%08llx-%08llx\n",
    crashk_cma_ranges[i].start,
    crashk_cma_ranges[i].end);
    }
    return len;
    }
pub static mut crash_cma_ranges_attr: kobj_attribute = __ATTR_RO(crash_cma_ranges);

#[no_mangle]
pub unsafe extern "C" fn crash_size_show() {
pub static mut size: isize = crash_get_memory_size();
    if (size < 0) {
    return size;
    }
    return sysfs_emit(buf, "%zd\n", size);
    }
#[no_mangle]
pub unsafe extern "C" fn crash_size_store() {
    let mut cnt = 0;
    let mut ret = 0;
    if (kstrtoul(buf, 0, &cnt)) {
    return -EINVAL;
    }
    ret = crash_shrink_memory(cnt);
    return ret < 0 ? ret : count;
    }
pub static mut crash_size_attr: kobj_attribute = __ATTR_RW(crash_size);

#[no_mangle]
pub unsafe extern "C" fn crash_elfcorehdr_size_show() {
pub static mut sz: c_uint = crash_get_elfcorehdr_size();
    return sysfs_emit(buf, "%u\n", sz);
    }
pub static mut crash_elfcorehdr_size_attr: kobj_attribute = __ATTR_RO(crash_elfcorehdr_size);

    static struct attribute *kexec_attrs[] = {
    &loaded_attr.attr,

    &crash_loaded_attr.attr,
    &crash_size_attr.attr,

    &crash_cma_ranges_attr.attr,

    &crash_elfcorehdr_size_attr.attr,

    core::ptr::null_mut()
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_link_entry {
    pub target: *const c_char,
    pub name: *const c_char,
}

pub static mut kexec_link_entry: usize = 0;
    static struct kobject *kexec_kobj;
// ATTRIBUTE_GROUPS;
#[no_mangle]
unsafe extern "C" fn init_kexec_sysctl() -> c_int {
    let mut error = 0;
    let mut i = 0;
    kexec_kobj = kobject_create_and_add("kexec", kernel_kobj);
    if (!kexec_kobj) {
    pr_err("failed to create kexec kobject\n");
    return -ENOMEM;
    }
    error = sysfs_create_groups(kexec_kobj, kexec_groups);
    if (error) {
    goto kset_exit;
    }
    for (i = 0; i < ARRAY_SIZE(kexec_links); i++) {
    error = compat_only_sysfs_link_entry_to_kobj(kernel_kobj, kexec_kobj,
    kexec_links[i].target,
    kexec_links[i].name);
    if (error) {
    pr_err("Unable to create %s symlink (%d)", kexec_links[i].name, error);
    }
    }
    return 0;
    kset_exit:
    kobject_put(kexec_kobj);
    return error;
    }
// subsys_initcall;
}
}
}
}