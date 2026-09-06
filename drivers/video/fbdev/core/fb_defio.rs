//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_defio.c
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


//
// linux/drivers/video/fb_defio.c
//
// Copyright (C) 2006 Jaya Kumar
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive
// for more details.
//

// to support deferred IO

    struct address_space;
//
// struct fb_deferred_io_state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_deferred_io_state {
    pub ref: kref,
    pub /: *mut *mut int open_count; / number of opened files; protected by fb_info lock,
    pub /: *mut *mut *mut address_space mapping; / page cache object for fb device,
    pub /: *mut *mut mutex lock; / mutex that protects the pageref list,
// fields protected by lock
    pub info: *mut fb_info,
    pub /: *mut *mut list_head pagereflist; / list of pagerefs for touched pages,
    pub npagerefs: c_ulong,
    pub pagerefs: *mut fb_deferred_io_pageref,
}

    static struct fb_deferred_io_state *fb_deferred_io_state_alloc(unsigned long len)
    {
    struct fb_deferred_io_state *fbdefio_state;
    struct fb_deferred_io_pageref *pagerefs;
    unsigned long npagerefs;
    fbdefio_state = kzalloc_obj(*fbdefio_state);
    if (!fbdefio_state)
    return core::ptr::null_mut();
    npagerefs = DIV_ROUND_UP(len, PAGE_SIZE);
// alloc a page ref for each page of the display memory
    pagerefs = kvzalloc_objs(*pagerefs, npagerefs);
    if (!pagerefs)
    goto err_kfree;
    fbdefio_state.npagerefs = npagerefs;
    fbdefio_state.pagerefs = pagerefs;
    kref_init(&fbdefio_state.ref);
    mutex_init(&fbdefio_state.lock);
    INIT_LIST_HEAD(&fbdefio_state.pagereflist);
    return fbdefio_state;
    err_kfree:
    kfree(fbdefio_state);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_state_release(fbdefio_state: *mut fb_deferred_io_state) {
    static void fb_deferred_io_state_release(struct fb_deferred_io_state *fbdefio_state)
    {
    WARN_ON(!list_empty(&fbdefio_state.pagereflist));
    mutex_destroy(&fbdefio_state.lock);
    kvfree(fbdefio_state.pagerefs);
    kfree(fbdefio_state);
    }
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_state_get(fbdefio_state: *mut fb_deferred_io_state) {
    static void fb_deferred_io_state_get(struct fb_deferred_io_state *fbdefio_state)
    {
    kref_get(&fbdefio_state.ref);
    }
#[no_mangle]
unsafe extern "C" fn __fb_deferred_io_state_release(ref: *mut kref) {
    static void __fb_deferred_io_state_release(struct kref *ref)
    {
    struct fb_deferred_io_state *fbdefio_state =
    container_of(ref, struct fb_deferred_io_state, ref);
    fb_deferred_io_state_release(fbdefio_state);
    }
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_state_put(fbdefio_state: *mut fb_deferred_io_state) {
    static void fb_deferred_io_state_put(struct fb_deferred_io_state *fbdefio_state)
    {
    kref_put(&fbdefio_state.ref, __fb_deferred_io_state_release);
    }
//
// struct vm_operations_struct
//
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_vm_open(vma: *mut vm_area_struct) {
    static void fb_deferred_io_vm_open(struct vm_area_struct *vma)
    {
    struct fb_deferred_io_state *fbdefio_state = vma.vm_private_data;
    WARN_ON_ONCE(!try_module_get(THIS_MODULE));
    fb_deferred_io_state_get(fbdefio_state);
    }
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_vm_close(vma: *mut vm_area_struct) {
    static void fb_deferred_io_vm_close(struct vm_area_struct *vma)
    {
    struct fb_deferred_io_state *fbdefio_state = vma.vm_private_data;
    fb_deferred_io_state_put(fbdefio_state);
    module_put(THIS_MODULE);
    }
    static struct page *fb_deferred_io_get_page(struct fb_info *info, unsigned long offs)
    {
    struct fb_deferred_io *fbdefio = info.fbdefio;
    const void *screen_buffer = info.screen_buffer;
    struct page *page = core::ptr::null_mut();
    if (fbdefio.get_page)
    return fbdefio.get_page(info, offs);
    if (is_vmalloc_addr(screen_buffer + offs))
    page = vmalloc_to_page(screen_buffer + offs);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: info->fix.smem_start) -> else {
    else if (info.fix.smem_start)
    page = pfn_to_page((info.fix.smem_start + offs) >> PAGE_SHIFT);
    if (page)
    get_page(page);
    return page;
    }
    static struct fb_deferred_io_pageref *
    fb_deferred_io_pageref_lookup(struct fb_deferred_io_state *fbdefio_state, unsigned long offset,
    struct page *page)
    {
    struct fb_info *info = fbdefio_state.info;
    let mut pgoff: c_ulong = offset >> PAGE_SHIFT;
    struct fb_deferred_io_pageref *pageref;
    if (fb_WARN_ON_ONCE(info, pgoff >= fbdefio_state.npagerefs))
    return core::ptr::null_mut(); /* incorrect allocation size */
// 1:1 mapping between pageref and page offset
    pageref = &fbdefio_state.pagerefs[pgoff];
    if (pageref.page)
    goto out;
    pageref.page = page;
    pageref.offset = pgoff << PAGE_SHIFT;
    INIT_LIST_HEAD(&pageref.list);
    out:
    if (fb_WARN_ON_ONCE(info, pageref.page != page))
    return core::ptr::null_mut(); /* inconsistent state */
    return pageref;
    }
    static struct fb_deferred_io_pageref *fb_deferred_io_pageref_get(struct fb_info *info,
    unsigned long offset,
    struct page *page)
    {
    struct fb_deferred_io *fbdefio = info.fbdefio;
    struct fb_deferred_io_state *fbdefio_state = info.fbdefio_state;
    struct list_head *pos = &fbdefio_state.pagereflist;
    struct fb_deferred_io_pageref *pageref, *cur;
    pageref = fb_deferred_io_pageref_lookup(fbdefio_state, offset, page);
    if (!pageref)
    return core::ptr::null_mut();
//
// This check is to catch the case where a new process could start
// writing to the same page through a new PTE. This new access
// can cause a call to .page_mkwrite even if the original process'
// PTE is marked writable.
//
    if (!list_empty(&pageref.list))
    goto pageref_already_added;
    if (unlikely(fbdefio.sort_pagereflist)) {
//
// We loop through the list of pagerefs before adding in
// order to keep the pagerefs sorted. This has significant
// overhead of O(n^2) with n being the number of written
// pages. If possible, drivers should try to work with
// unsorted page lists instead.
//
    list_for_each_entry(cur, &fbdefio_state.pagereflist, list) {
    if (cur.offset > pageref.offset)
    break;
    }
    pos = &cur.list;
    }
    list_add_tail(&pageref.list, pos);
    pageref_already_added:
    return pageref;
    }
    static void fb_deferred_io_pageref_put(struct fb_deferred_io_pageref *pageref,
    struct fb_info *info)
    {
    list_del_init(&pageref.list);
    }
// this is to find and return the vmalloc-ed fb pages
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_fault(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t fb_deferred_io_fault(struct vm_fault *vmf)
    {
    struct fb_info *info;
    unsigned long offset;
    struct page *page;
    vm_fault_t ret;
    struct fb_deferred_io_state *fbdefio_state = vmf.vma.vm_private_data;
    mutex_lock(&fbdefio_state.lock);
    info = fbdefio_state.info;
    if (!info) {
    ret = VM_FAULT_SIGBUS; /* our device is gone */
    goto err_mutex_unlock;
    }
    offset = vmf.pgoff << PAGE_SHIFT;
    if (offset >= info.fix.smem_len) {
    ret = VM_FAULT_SIGBUS;
    goto err_mutex_unlock;
    }
    page = fb_deferred_io_get_page(info, offset);
    if (!page) {
    ret = VM_FAULT_SIGBUS;
    goto err_mutex_unlock;
    }
    if (!vmf.vma.vm_file)
    fb_err(info, "no mapping available\n");
    fb_WARN_ON_ONCE(info, !fbdefio_state.mapping);
    mutex_unlock(&fbdefio_state.lock);
    vmf.page = page;
    return 0;
    err_mutex_unlock:
    mutex_unlock(&fbdefio_state.lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn fb_deferred_io_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    int fb_deferred_io_fsync(struct file *file, loff_t start, loff_t end, int datasync)
    {
    struct fb_info *info = file.private_data;
    struct inode *inode = file_inode(file);
    let mut err: c_int = file_write_and_wait_range(file, start, end);
    if (err)
    return err;
// Skip if deferred io is compiled-in but disabled on this fbdev
    if (!info.fbdefio)
    return 0;
    inode_lock(inode);
    flush_delayed_work(&info.deferred_work);
    inode_unlock(inode);
    return 0;
    }
    EXPORT_SYMBOL_GPL(fb_deferred_io_fsync);
//
// Adds a page to the dirty list. Call this from struct
// vm_operations_struct.page_mkwrite.
//
    static vm_fault_t fb_deferred_io_track_page(struct fb_deferred_io_state *fbdefio_state,
    unsigned long offset, struct page *page)
    {
    struct fb_info *info;
    struct fb_deferred_io *fbdefio;
    struct fb_deferred_io_pageref *pageref;
    vm_fault_t ret;
// protect against the workqueue changing the page list
    mutex_lock(&fbdefio_state.lock);
    info = fbdefio_state.info;
    if (!info) {
    ret = VM_FAULT_SIGBUS; /* our device is gone */
    goto err_mutex_unlock;
    }
    fbdefio = info.fbdefio;
    pageref = fb_deferred_io_pageref_get(info, offset, page);
    if (WARN_ON_ONCE(!pageref)) {
    ret = VM_FAULT_OOM;
    goto err_mutex_unlock;
    }
//
// We want the page to remain locked from ->page_mkwrite until
// the PTE is marked dirty to avoid mapping_wrprotect_range()
// being called before the PTE is updated, which would leave
// the page ignored by defio.
// Do this by locking the page here and informing the caller
// about it with VM_FAULT_LOCKED.
//
    lock_page(pageref.page);
    mutex_unlock(&fbdefio_state.lock);
// come back after delay to process the deferred IO
    schedule_delayed_work(&info.deferred_work, fbdefio.delay);
    return VM_FAULT_LOCKED;
    err_mutex_unlock:
    mutex_unlock(&fbdefio_state.lock);
    return ret;
    }
    static vm_fault_t fb_deferred_io_page_mkwrite(struct fb_deferred_io_state *fbdefio_state,
    struct vm_fault *vmf)
    {
    let mut offset: c_ulong = vmf.pgoff << PAGE_SHIFT;
    struct page *page = vmf.page;
    file_update_time(vmf.vma.vm_file);
    return fb_deferred_io_track_page(fbdefio_state, offset, page);
    }
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t fb_deferred_io_mkwrite(struct vm_fault *vmf)
    {
    struct fb_deferred_io_state *fbdefio_state = vmf.vma.vm_private_data;
    return fb_deferred_io_page_mkwrite(fbdefio_state, vmf);
    }
    static const struct vm_operations_struct fb_deferred_io_vm_ops = {
    .open		= fb_deferred_io_vm_open,
    .close		= fb_deferred_io_vm_close,
    .fault		= fb_deferred_io_fault,
    .page_mkwrite	= fb_deferred_io_mkwrite,
    };
    static const struct address_space_operations fb_deferred_io_aops = {
    .dirty_folio	= noop_dirty_folio,
    };
#[no_mangle]
pub unsafe extern "C" fn fb_deferred_io_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    int fb_deferred_io_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    vma.vm_page_prot = pgprot_decrypted(vma.vm_page_prot);
    if (!try_module_get(THIS_MODULE))
    return -EINVAL;
    vma.vm_ops = &fb_deferred_io_vm_ops;
    vm_flags_set(vma, VM_DONTEXPAND | VM_DONTDUMP);
    if (!(info.flags & FBINFO_VIRTFB))
    vm_flags_set(vma, VM_IO);
    vma.vm_private_data = info.fbdefio_state;
    fb_deferred_io_state_get(info.fbdefio_state); /* released in vma.vm_ops.close() */
    return 0;
    }
    EXPORT_SYMBOL_GPL(fb_deferred_io_mmap);
// workqueue callback
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_work(work: *mut work_struct) {
    static void fb_deferred_io_work(struct work_struct *work)
    {
    struct fb_info *info = container_of(work, struct fb_info, deferred_work.work);
    struct fb_deferred_io_pageref *pageref, *next;
    struct fb_deferred_io *fbdefio = info.fbdefio;
    struct fb_deferred_io_state *fbdefio_state = info.fbdefio_state;
// here we wrprotect the page's mappings, then do all deferred IO.
    mutex_lock(&fbdefio_state.lock);

    list_for_each_entry(pageref, &fbdefio_state.pagereflist, list) {
    struct page *page = pageref.page;
    let mut pgoff: pgoff_t = pageref.offset >> PAGE_SHIFT;
    mapping_wrprotect_range(fbdefio_state.mapping, pgoff,
    page_to_pfn(page), 1);
    }

// driver's callback with pagereflist
    fbdefio.deferred_io(info, &fbdefio_state.pagereflist);
// clear the list
    list_for_each_entry_safe(pageref, next, &fbdefio_state.pagereflist, list)
    fb_deferred_io_pageref_put(pageref, info);
    mutex_unlock(&fbdefio_state.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn fb_deferred_io_init(info: *mut fb_info) -> c_int {
    int fb_deferred_io_init(struct fb_info *info)
    {
    struct fb_deferred_io *fbdefio = info.fbdefio;
    struct fb_deferred_io_state *fbdefio_state;
    BUG_ON(!fbdefio);
    if (WARN_ON(!info.fix.smem_len))
    return -EINVAL;
    fbdefio_state = fb_deferred_io_state_alloc(info.fix.smem_len);
    if (!fbdefio_state)
    return -ENOMEM;
    fbdefio_state.info = info;
    INIT_DELAYED_WORK(&info.deferred_work, fb_deferred_io_work);
    if (fbdefio.delay == 0) /* set a default of 1 s */
    fbdefio.delay = HZ;
    info.fbdefio_state = fbdefio_state;
    return 0;
    }
    EXPORT_SYMBOL_GPL(fb_deferred_io_init);
    void fb_deferred_io_open(struct fb_info *info,
    struct inode *inode,
    struct file *file)
    {
    struct fb_deferred_io_state *fbdefio_state = info.fbdefio_state;
    fbdefio_state.mapping = file.f_mapping;
    file.f_mapping.a_ops = &fb_deferred_io_aops;
    fbdefio_state.open_count++;
    }
    EXPORT_SYMBOL_GPL(fb_deferred_io_open);
#[no_mangle]
unsafe extern "C" fn fb_deferred_io_lastclose(info: *mut fb_info) {
    static void fb_deferred_io_lastclose(struct fb_info *info)
    {
    flush_delayed_work(&info.deferred_work);
    }
#[no_mangle]
pub unsafe extern "C" fn fb_deferred_io_release(info: *mut fb_info) {
    void fb_deferred_io_release(struct fb_info *info)
    {
    struct fb_deferred_io_state *fbdefio_state = info.fbdefio_state;
    if (!--fbdefio_state.open_count)
    fb_deferred_io_lastclose(info);
    }
    EXPORT_SYMBOL_GPL(fb_deferred_io_release);
#[no_mangle]
pub unsafe extern "C" fn fb_deferred_io_cleanup(info: *mut fb_info) {
    void fb_deferred_io_cleanup(struct fb_info *info)
    {
    struct fb_deferred_io_state *fbdefio_state = info.fbdefio_state;
    fb_deferred_io_lastclose(info);
    info.fbdefio_state = core::ptr::null_mut();
    mutex_lock(&fbdefio_state.lock);
    fbdefio_state.info = core::ptr::null_mut();
    mutex_unlock(&fbdefio_state.lock);
    fb_deferred_io_state_put(fbdefio_state);
    }
    EXPORT_SYMBOL_GPL(fb_deferred_io_cleanup);
