//! Automatically rewritten from C to Rust
//! Source: fs/proc/vmcore.c
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
// fs/proc/vmcore.c Interface for accessing the crash
// dump from the system's previous life.
// Heavily borrowed from fs/proc/kcore.c
// Created by: Hariprasad Nellitheertha (hari@in.ibm.com)
// Copyright (C) IBM Corporation, 2004. All rights reserved
//

// List representing chunks of contiguous memory areas and their offsets in
// vmcore file.
//
    static LIST_HEAD(vmcore_list);
// Stores the pointer to the buffer containing kernel elf core headers.
    static char *elfcorebuf;
    static size_t elfcorebuf_sz;
    static size_t elfcorebuf_sz_orig;
    static char *elfnotes_buf;
    static size_t elfnotes_sz;
// Size of all notes minus the device dump notes
    static size_t elfnotes_orig_sz;
// Total size of vmcore file.
    static u64 vmcore_size;
    static struct proc_dir_entry *proc_vmcore;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcoredd_node {
    pub /: *mut *mut list_head list; / List of dumps,
    pub /: *mut *mut *mut void buf; / Buffer containing device's dump,
    pub /: *mut *mut unsigned int size; / Size of the buffer,
}

// Device Dump list and mutex to synchronize access to list
    static LIST_HEAD(vmcoredd_list);
    static bool vmcoredd_disabled;
    core_param(novmcoredd, vmcoredd_disabled, bool, 0);

// Device Dump Size
    static size_t vmcoredd_orig_sz;
    static DEFINE_MUTEX(vmcore_mutex);
    DEFINE_STATIC_SRCU(vmcore_cb_srcu);
// List of registered vmcore callbacks.
    static LIST_HEAD(vmcore_cb_list);
// Whether the vmcore has been opened once.
    static bool vmcore_opened;
// Whether the vmcore is currently open.
    static unsigned int vmcore_open;
    static void vmcore_process_device_ram(struct vmcore_cb *cb);
#[no_mangle]
pub unsafe extern "C" fn register_vmcore_cb(cb: *mut vmcore_cb) {
    void register_vmcore_cb(struct vmcore_cb *cb)
    {
    INIT_LIST_HEAD(&cb.next);
    mutex_lock(&vmcore_mutex);
    list_add_tail(&cb.next, &vmcore_cb_list);
//
// Registering a vmcore callback after the vmcore was opened is
// very unusual (e.g., manual driver loading).
//
    if (vmcore_opened)
    pr_warn_once("Unexpected vmcore callback registration\n");
    if (!vmcore_open && cb.get_device_ram)
    vmcore_process_device_ram(cb);
    mutex_unlock(&vmcore_mutex);
    }
    EXPORT_SYMBOL_GPL(register_vmcore_cb);
#[no_mangle]
pub unsafe extern "C" fn unregister_vmcore_cb(cb: *mut vmcore_cb) {
    void unregister_vmcore_cb(struct vmcore_cb *cb)
    {
    mutex_lock(&vmcore_mutex);
    list_del_rcu(&cb.next);
//
// Unregistering a vmcore callback after the vmcore was opened is
// very unusual (e.g., forced driver removal), but we cannot stop
// unregistering.
//
    if (vmcore_opened)
    pr_warn_once("Unexpected vmcore callback unregistration\n");
    mutex_unlock(&vmcore_mutex);
    synchronize_srcu(&vmcore_cb_srcu);
    }
    EXPORT_SYMBOL_GPL(unregister_vmcore_cb);
#[no_mangle]
unsafe extern "C" fn pfn_is_ram(pfn: c_ulong) -> bool {
    static bool pfn_is_ram(unsigned long pfn)
    {
    struct vmcore_cb *cb;
    let mut ret: bool = true;
    list_for_each_entry_srcu(cb, &vmcore_cb_list, next,
    srcu_read_lock_held(&vmcore_cb_srcu)) {
    if (unlikely(!cb.pfn_is_ram))
    continue;
    ret = cb.pfn_is_ram(cb, pfn);
    if (!ret)
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn open_vmcore(inode: *mut inode, file: *mut file) -> c_int {
    static int open_vmcore(struct inode *inode, struct file *file)
    {
    mutex_lock(&vmcore_mutex);
    vmcore_opened = true;
    if (vmcore_open + 1 == 0) {
    mutex_unlock(&vmcore_mutex);
    return -EBUSY;
    }
    vmcore_open++;
    mutex_unlock(&vmcore_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn release_vmcore(inode: *mut inode, file: *mut file) -> c_int {
    static int release_vmcore(struct inode *inode, struct file *file)
    {
    mutex_lock(&vmcore_mutex);
    vmcore_open--;
    mutex_unlock(&vmcore_mutex);
    return 0;
    }
// Reads a page from the oldmem device from given offset.
    ssize_t read_from_oldmem(struct iov_iter *iter, size_t count,
    u64 *ppos, bool encrypted)
    {
    unsigned long pfn, offset;
    ssize_t nr_bytes;
    let mut read: isize = 0, tmp;
    int idx;
    if (!count)
    return 0;
    offset = (unsigned long)(*ppos % PAGE_SIZE);
    pfn = (unsigned long)(*ppos / PAGE_SIZE);
    idx = srcu_read_lock(&vmcore_cb_srcu);
    do {
    if (count > (PAGE_SIZE - offset))
    nr_bytes = PAGE_SIZE - offset;
    else
    nr_bytes = count;
// If pfn is not ram, return zeros for sparse dump files
    if (!pfn_is_ram(pfn)) {
    tmp = iov_iter_zero(nr_bytes, iter);
    } else {
    if (encrypted)
    tmp = copy_oldmem_page_encrypted(iter, pfn,
    nr_bytes,
    offset);
    else
    tmp = copy_oldmem_page(iter, pfn, nr_bytes,
    offset);
    }
    if (tmp < nr_bytes) {
    srcu_read_unlock(&vmcore_cb_srcu, idx);
    return -EFAULT;
    }
// ppos += nr_bytes;
    count -= nr_bytes;
    read += nr_bytes;
    ++pfn;
    offset = 0;
    } while (count);
    srcu_read_unlock(&vmcore_cb_srcu, idx);
    return read;
    }
//
// Architectures may override this function to allocate ELF header in 2nd kernel
//
#[no_mangle]
pub unsafe extern "C" fn elfcorehdr_alloc(addr: *mut c_ulonglong, size: *mut c_ulonglong) -> int __weak {
    int __weak elfcorehdr_alloc(unsigned long long *addr, unsigned long long *size)
    {
    return 0;
    }
//
// Architectures may override this function to free header
//
#[no_mangle]
pub unsafe extern "C" fn elfcorehdr_free(addr: c_ulonglong) -> void __weak {
    void __weak elfcorehdr_free(unsigned long long addr)
    {}
//
// Architectures may override this function to read from ELF header
//
#[no_mangle]
pub unsafe extern "C" fn elfcorehdr_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> ssize_t __weak {
    ssize_t __weak elfcorehdr_read(char *buf, size_t count, u64 *ppos)
    {
    let mut kvec: kvec = { .iov_base = buf, .iov_len = count };
    struct iov_iter iter;
    iov_iter_kvec(&iter, ITER_DEST, &kvec, 1, count);
    return read_from_oldmem(&iter, count, ppos, false);
    }
//
// Architectures may override this function to read from notes sections
//
#[no_mangle]
pub unsafe extern "C" fn elfcorehdr_read_notes(buf: *mut c_char, count: usize, ppos: *mut u64) -> ssize_t __weak {
    ssize_t __weak elfcorehdr_read_notes(char *buf, size_t count, u64 *ppos)
    {
    let mut kvec: kvec = { .iov_base = buf, .iov_len = count };
    struct iov_iter iter;
    iov_iter_kvec(&iter, ITER_DEST, &kvec, 1, count);
    return read_from_oldmem(&iter, count, ppos,
    cc_platform_has(CC_ATTR_MEM_ENCRYPT));
    }
//
// Architectures may override this function to map oldmem
//
    int __weak remap_oldmem_pfn_range(struct vm_area_struct *vma,
    unsigned long from, unsigned long pfn,
    unsigned long size, pgprot_t prot)
    {
    prot = pgprot_encrypted(prot);
    return remap_pfn_range(vma, from, pfn, size, prot);
    }
//
// Architectures which support memory encryption override this.
//
    ssize_t __weak copy_oldmem_page_encrypted(struct iov_iter *iter,
    unsigned long pfn, size_t csize, unsigned long offset)
    {
    return copy_oldmem_page(iter, pfn, csize, offset);
    }

#[no_mangle]
unsafe extern "C" fn vmcoredd_copy_dumps(iter: *mut iov_iter, start: u64, size: usize) -> c_int {
    static int vmcoredd_copy_dumps(struct iov_iter *iter, u64 start, size_t size)
    {
    struct vmcoredd_node *dump;
    let mut offset: u64 = 0;
    size_t tsz;
    char *buf;
    list_for_each_entry(dump, &vmcoredd_list, list) {
    if (start < offset + dump.size) {
    tsz = min(offset + (u64)dump.size - start, (u64)size);
    buf = dump.buf + start - offset;
    if (copy_to_iter(buf, tsz, iter) < tsz)
    return -EFAULT;
    size -= tsz;
    start += tsz;
// Leave now if buffer filled already
    if (!size)
    return 0;
    }
    offset += dump.size;
    }
    return 0;
    }

    static int vmcoredd_mmap_dumps(struct vm_area_struct *vma, unsigned long dst,
    u64 start, size_t size)
    {
    struct vmcoredd_node *dump;
    let mut offset: u64 = 0;
    size_t tsz;
    char *buf;
    list_for_each_entry(dump, &vmcoredd_list, list) {
    if (start < offset + dump.size) {
    tsz = min(offset + (u64)dump.size - start, (u64)size);
    buf = dump.buf + start - offset;
    if (remap_vmalloc_range_partial(vma, dst, buf, 0,
    tsz))
    return -EFAULT;
    size -= tsz;
    start += tsz;
    dst += tsz;
// Leave now if buffer filled already
    if (!size)
    return 0;
    }
    offset += dump.size;
    }
    return 0;
    }

// Read from the ELF header and then the crash dump. On error, negative value is
// returned otherwise number of bytes read are returned.
//
#[no_mangle]
unsafe extern "C" fn __read_vmcore(iter: *mut iov_iter, fpos: *mut loff_t) -> isize {
    static ssize_t __read_vmcore(struct iov_iter *iter, loff_t *fpos)
    {
    struct vmcore_range *m = core::ptr::null_mut();
    let mut acc: isize = 0, tmp;
    size_t tsz;
    u64 start;
    if (!iov_iter_count(iter) || *fpos >= vmcore_size)
    return 0;
    iov_iter_truncate(iter, vmcore_size - *fpos);
// Read ELF core header
    if (*fpos < elfcorebuf_sz) {
    tsz = min(elfcorebuf_sz - (size_t)*fpos, iov_iter_count(iter));
    if (copy_to_iter(elfcorebuf + *fpos, tsz, iter) < tsz)
    return -EFAULT;
// fpos += tsz;
    acc += tsz;
// leave now if filled buffer already
    if (!iov_iter_count(iter))
    return acc;
    }
// Read ELF note segment
    if (*fpos < elfcorebuf_sz + elfnotes_sz) {
    void *kaddr;
// We add device dumps before other elf notes because the
// other elf notes may not fill the elf notes buffer
// completely and we will end up with zero-filled data
// between the elf notes and the device dumps. Tools will
// then try to decode this zero-filled data as valid notes
// and we don't want that. Hence, adding device dumps before
// the other elf notes ensure that zero-filled data can be
// avoided.
//

// Read device dumps
    if (*fpos < elfcorebuf_sz + vmcoredd_orig_sz) {
    tsz = min(elfcorebuf_sz + vmcoredd_orig_sz -
    (size_t)*fpos, iov_iter_count(iter));
    start = *fpos - elfcorebuf_sz;
    if (vmcoredd_copy_dumps(iter, start, tsz))
    return -EFAULT;
// fpos += tsz;
    acc += tsz;
// leave now if filled buffer already
    if (!iov_iter_count(iter))
    return acc;
    }

// Read remaining elf notes
    tsz = min(elfcorebuf_sz + elfnotes_sz - (size_t)*fpos,
    iov_iter_count(iter));
    kaddr = elfnotes_buf + *fpos - elfcorebuf_sz - vmcoredd_orig_sz;
    if (copy_to_iter(kaddr, tsz, iter) < tsz)
    return -EFAULT;
// fpos += tsz;
    acc += tsz;
// leave now if filled buffer already
    if (!iov_iter_count(iter))
    return acc;
    cond_resched();
    }
    list_for_each_entry(m, &vmcore_list, list) {
    if (*fpos < m.offset + m.size) {
    tsz = (size_t)min_t(unsigned long long,
    m.offset + m.size - *fpos,
    iov_iter_count(iter));
    start = m.paddr + *fpos - m.offset;
    tmp = read_from_oldmem(iter, tsz, &start,
    cc_platform_has(CC_ATTR_MEM_ENCRYPT));
    if (tmp < 0)
    return tmp;
// fpos += tsz;
    acc += tsz;
// leave now if filled buffer already
    if (!iov_iter_count(iter))
    return acc;
    }
    cond_resched();
    }
    return acc;
    }
#[no_mangle]
unsafe extern "C" fn read_vmcore(iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    static ssize_t read_vmcore(struct kiocb *iocb, struct iov_iter *iter)
    {
    return __read_vmcore(iter, &iocb.ki_pos);
    }
//
// vmcore_alloc_buf - allocate buffer in vmalloc memory
// @size: size of buffer
//
// If CONFIG_MMU is defined, use vmalloc_user() to allow users to mmap
// the buffer to user-space by means of remap_vmalloc_range().
//
// If CONFIG_MMU is not defined, use vzalloc() since mmap_vmcore() is
// disabled and there's no need to allow users to mmap the buffer.
//
    static inline char *vmcore_alloc_buf(size_t size)
    {

    return vmalloc_user(size);

    return vzalloc(size);

    }
//
// Disable mmap_vmcore() if CONFIG_MMU is not defined. MMU is
// essential for mmap_vmcore() in order to map physically
// non-contiguous objects (ELF header, ELF note segment and memory
// regions in the 1st kernel pointed to by PT_LOAD entries) into
// virtually contiguous user-space in ELF layout.
//

//
// The vmcore fault handler uses the page cache and fills data using the
// standard __read_vmcore() function.
//
// On s390 the fault handler is used for memory regions that can't be mapped
// directly with remap_pfn_range().
//
#[no_mangle]
unsafe extern "C" fn mmap_vmcore_fault(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t mmap_vmcore_fault(struct vm_fault *vmf)
    {

    struct address_space *mapping = vmf.vma.vm_file.f_mapping;
    let mut index: pgoff_t = vmf.pgoff;
    struct iov_iter iter;
    struct kvec kvec;
    struct page *page;
    loff_t offset;
    int rc;
    page = find_or_create_page(mapping, index, GFP_KERNEL);
    if (!page)
    return VM_FAULT_OOM;
    if (!PageUptodate(page)) {
    offset = (loff_t) index << PAGE_SHIFT;
    kvec.iov_base = page_address(page);
    kvec.iov_len = PAGE_SIZE;
    iov_iter_kvec(&iter, ITER_DEST, &kvec, 1, PAGE_SIZE);
    rc = __read_vmcore(&iter, &offset);
    if (rc < 0) {
    unlock_page(page);
    put_page(page);
    return vmf_error(rc);
    }
    SetPageUptodate(page);
    }
    unlock_page(page);
    vmf.page = page;
    return 0;

    return VM_FAULT_SIGBUS;

    }
    static const struct vm_operations_struct vmcore_mmap_ops = {
    .fault = mmap_vmcore_fault,
    };
//
// remap_oldmem_pfn_checked - do remap_oldmem_pfn_range replacing all pages
// reported as not being ram with the zero page.
//
// @vma: vm_area_struct describing requested mapping
// @from: start remapping from
// @pfn: page frame number to start remapping to
// @size: remapping size
// @prot: protection bits
//
// Returns zero on success, -EAGAIN on failure.
//
    static int remap_oldmem_pfn_checked(struct vm_area_struct *vma,
    unsigned long from, unsigned long pfn,
    unsigned long size, pgprot_t prot)
    {
    unsigned long map_size;
    unsigned long pos_start, pos_end, pos;
    let mut zeropage_pfn: c_ulong = zero_pfn(0);
    let mut len: usize = 0;
    pos_start = pfn;
    pos_end = pfn + (size >> PAGE_SHIFT);
    for (pos = pos_start; pos < pos_end; ++pos) {
    if (!pfn_is_ram(pos)) {
//
// We hit a page which is not ram. Remap the continuous
// region between pos_start and pos-1 and replace
// the non-ram page at pos with the zero page.
//
    if (pos > pos_start) {
// Remap continuous region
    map_size = (pos - pos_start) << PAGE_SHIFT;
    if (remap_oldmem_pfn_range(vma, from + len,
    pos_start, map_size,
    prot))
    goto fail;
    len += map_size;
    }
// Remap the zero page
    if (remap_oldmem_pfn_range(vma, from + len,
    zeropage_pfn,
    PAGE_SIZE, prot))
    goto fail;
    len += PAGE_SIZE;
    pos_start = pos + 1;
    }
    }
    if (pos > pos_start) {
// Remap the rest
    map_size = (pos - pos_start) << PAGE_SHIFT;
    if (remap_oldmem_pfn_range(vma, from + len, pos_start,
    map_size, prot))
    goto fail;
    }
    return 0;
    fail:
    do_munmap(vma.vm_mm, from, len, core::ptr::null_mut());
    return -EAGAIN;
    }
    static int vmcore_remap_oldmem_pfn(struct vm_area_struct *vma,
    unsigned long from, unsigned long pfn,
    unsigned long size, pgprot_t prot)
    {
    int ret, idx;
//
// Check if a callback was registered to avoid looping over all
// pages without a reason.
//
    idx = srcu_read_lock(&vmcore_cb_srcu);
    if (!list_empty(&vmcore_cb_list))
    ret = remap_oldmem_pfn_checked(vma, from, pfn, size, prot);
    else
    ret = remap_oldmem_pfn_range(vma, from, pfn, size, prot);
    srcu_read_unlock(&vmcore_cb_srcu, idx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mmap_vmcore(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int mmap_vmcore(struct file *file, struct vm_area_struct *vma)
    {
    let mut size: usize = vma.vm_end - vma.vm_start;
    u64 start, end, len, tsz;
    struct vmcore_range *m;
    start = (u64)vma.vm_pgoff << PAGE_SHIFT;
    end = start + size;
    if (size > vmcore_size || end > vmcore_size)
    return -EINVAL;
    if (vma.vm_flags & (VM_WRITE | VM_EXEC))
    return -EPERM;
    vm_flags_mod(vma, VM_MIXEDMAP, VM_MAYWRITE | VM_MAYEXEC);
    vma.vm_ops = &vmcore_mmap_ops;
    len = 0;
    if (start < elfcorebuf_sz) {
    u64 pfn;
    tsz = min(elfcorebuf_sz - (size_t)start, size);
    pfn = __pa(elfcorebuf + start) >> PAGE_SHIFT;
    if (remap_pfn_range(vma, vma.vm_start, pfn, tsz,
    vma.vm_page_prot))
    return -EAGAIN;
    size -= tsz;
    start += tsz;
    len += tsz;
    if (size == 0)
    return 0;
    }
    if (start < elfcorebuf_sz + elfnotes_sz) {
    void *kaddr;
// We add device dumps before other elf notes because the
// other elf notes may not fill the elf notes buffer
// completely and we will end up with zero-filled data
// between the elf notes and the device dumps. Tools will
// then try to decode this zero-filled data as valid notes
// and we don't want that. Hence, adding device dumps before
// the other elf notes ensure that zero-filled data can be
// avoided. This also ensures that the device dumps and
// other elf notes can be properly mmaped at page aligned
// address.
//

// Read device dumps
    if (start < elfcorebuf_sz + vmcoredd_orig_sz) {
    u64 start_off;
    tsz = min(elfcorebuf_sz + vmcoredd_orig_sz -
    (size_t)start, size);
    start_off = start - elfcorebuf_sz;
    if (vmcoredd_mmap_dumps(vma, vma.vm_start + len,
    start_off, tsz))
    goto fail;
    size -= tsz;
    start += tsz;
    len += tsz;
// leave now if filled buffer already
    if (!size)
    return 0;
    }

// Read remaining elf notes
    tsz = min(elfcorebuf_sz + elfnotes_sz - (size_t)start, size);
    kaddr = elfnotes_buf + start - elfcorebuf_sz - vmcoredd_orig_sz;
    if (remap_vmalloc_range_partial(vma, vma.vm_start + len,
    kaddr, 0, tsz))
    goto fail;
    size -= tsz;
    start += tsz;
    len += tsz;
    if (size == 0)
    return 0;
    }
    list_for_each_entry(m, &vmcore_list, list) {
    if (start < m.offset + m.size) {
    let mut paddr: u64 = 0;
    tsz = (size_t)min_t(unsigned long long,
    m.offset + m.size - start, size);
    paddr = m.paddr + start - m.offset;
    if (vmcore_remap_oldmem_pfn(vma, vma.vm_start + len,
    paddr >> PAGE_SHIFT, tsz,
    vma.vm_page_prot))
    goto fail;
    size -= tsz;
    start += tsz;
    len += tsz;
    if (size == 0)
    return 0;
    }
    }
    return 0;
    fail:
    do_munmap(vma.vm_mm, vma.vm_start, len, core::ptr::null_mut());
    return -EAGAIN;
    }

#[no_mangle]
unsafe extern "C" fn mmap_vmcore(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int mmap_vmcore(struct file *file, struct vm_area_struct *vma)
    {
    return -ENOSYS;
    }

    static const struct proc_ops vmcore_proc_ops = {
    .proc_open	= open_vmcore,
    .proc_release	= release_vmcore,
    .proc_read_iter	= read_vmcore,
    .proc_lseek	= default_llseek,
    .proc_mmap	= mmap_vmcore,
    };
    static u64 get_vmcore_size(size_t elfsz, size_t elfnotesegsz,
    struct list_head *vc_list)
    {
    struct vmcore_range *m;
    u64 size;
    size = elfsz + elfnotesegsz;
    list_for_each_entry(m, vc_list, list) {
    size += m.size;
    }
    return size;
    }
//
// update_note_header_size_elf64 - update p_memsz member of each PT_NOTE entry
//
// @ehdr_ptr: ELF header
//
// This function updates p_memsz member of each PT_NOTE entry in the
// program header table pointed to by @ehdr_ptr to real size of ELF
// note segment.
//
#[no_mangle]
unsafe extern "C" fn update_note_header_size_elf64(ehdr_ptr: *const Elf64_Ehdr) -> int __init {
    static int __init update_note_header_size_elf64(const Elf64_Ehdr *ehdr_ptr)
    {
    int i, rc=0;
    Elf64_Phdr *phdr_ptr;
    Elf64_Nhdr *nhdr_ptr;
    phdr_ptr = (Elf64_Phdr *)(ehdr_ptr + 1);
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    void *notes_section;
    u64 offset, max_sz, sz, real_sz = 0;
    if (phdr_ptr.p_type != PT_NOTE)
    continue;
    max_sz = phdr_ptr.p_memsz;
    offset = phdr_ptr.p_offset;
    notes_section = kmalloc(max_sz, GFP_KERNEL);
    if (!notes_section)
    return -ENOMEM;
    rc = elfcorehdr_read_notes(notes_section, max_sz, &offset);
    if (rc < 0) {
    kfree(notes_section);
    return rc;
    }
    nhdr_ptr = notes_section;
    while (nhdr_ptr.n_namesz != 0) {
    sz = sizeof(Elf64_Nhdr) +
    (((u64)nhdr_ptr.n_namesz + 3) & ~3) +
    (((u64)nhdr_ptr.n_descsz + 3) & ~3);
    if ((real_sz + sz) > max_sz) {
    pr_warn("Warning: Exceeded p_memsz, dropping PT_NOTE entry n_namesz=0x%x, n_descsz=0x%x\n",
    nhdr_ptr.n_namesz, nhdr_ptr.n_descsz);
    break;
    }
    real_sz += sz;
    nhdr_ptr = (Elf64_Nhdr*)((char*)nhdr_ptr + sz);
    }
    kfree(notes_section);
    phdr_ptr.p_memsz = real_sz;
    if (real_sz == 0) {
    pr_warn("Warning: Zero PT_NOTE entries found\n");
    }
    }
    return 0;
    }
//
// get_note_number_and_size_elf64 - get the number of PT_NOTE program
// headers and sum of real size of their ELF note segment headers and
// data.
//
// @ehdr_ptr: ELF header
// @nr_ptnote: buffer for the number of PT_NOTE program headers
// @sz_ptnote: buffer for size of unique PT_NOTE program header
//
// This function is used to merge multiple PT_NOTE program headers
// into a unique single one. The resulting unique entry will have
// @sz_ptnote in its phdr->p_mem.
//
// It is assumed that program headers with PT_NOTE type pointed to by
// @ehdr_ptr has already been updated by update_note_header_size_elf64
// and each of PT_NOTE program headers has actual ELF note segment
// size in its p_memsz member.
//
    static int __init get_note_number_and_size_elf64(const Elf64_Ehdr *ehdr_ptr,
    int *nr_ptnote, u64 *sz_ptnote)
    {
    int i;
    Elf64_Phdr *phdr_ptr;
// nr_ptnote = *sz_ptnote = 0;
    phdr_ptr = (Elf64_Phdr *)(ehdr_ptr + 1);
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    if (phdr_ptr.p_type != PT_NOTE)
    continue;
// nr_ptnote += 1;
// sz_ptnote += phdr_ptr->p_memsz;
    }
    return 0;
    }
//
// copy_notes_elf64 - copy ELF note segments in a given buffer
//
// @ehdr_ptr: ELF header
// @notes_buf: buffer into which ELF note segments are copied
//
// This function is used to copy ELF note segment in the 1st kernel
// into the buffer @notes_buf in the 2nd kernel. It is assumed that
// size of the buffer @notes_buf is equal to or larger than sum of the
// real ELF note segment headers and data.
//
// It is assumed that program headers with PT_NOTE type pointed to by
// @ehdr_ptr has already been updated by update_note_header_size_elf64
// and each of PT_NOTE program headers has actual ELF note segment
// size in its p_memsz member.
//
#[no_mangle]
unsafe extern "C" fn copy_notes_elf64(ehdr_ptr: *const Elf64_Ehdr, notes_buf: *mut c_char) -> int __init {
    static int __init copy_notes_elf64(const Elf64_Ehdr *ehdr_ptr, char *notes_buf)
    {
    int i, rc=0;
    Elf64_Phdr *phdr_ptr;
    phdr_ptr = (Elf64_Phdr*)(ehdr_ptr + 1);
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    u64 offset;
    if (phdr_ptr.p_type != PT_NOTE)
    continue;
    offset = phdr_ptr.p_offset;
    rc = elfcorehdr_read_notes(notes_buf, phdr_ptr.p_memsz,
    &offset);
    if (rc < 0)
    return rc;
    notes_buf += phdr_ptr.p_memsz;
    }
    return 0;
    }
// Merges all the PT_NOTE headers into one.
    static int __init merge_note_headers_elf64(char *elfptr, size_t *elfsz,
    char **notes_buf, size_t *notes_sz)
    {
    int i, nr_ptnote=0, rc=0;
    char *tmp;
    Elf64_Ehdr *ehdr_ptr;
    Elf64_Phdr phdr;
    let mut phdr_sz: u64 = 0, note_off;
    ehdr_ptr = (Elf64_Ehdr *)elfptr;
    rc = update_note_header_size_elf64(ehdr_ptr);
    if (rc < 0)
    return rc;
    rc = get_note_number_and_size_elf64(ehdr_ptr, &nr_ptnote, &phdr_sz);
    if (rc < 0)
    return rc;
// notes_sz = roundup(phdr_sz, PAGE_SIZE);
// notes_buf = vmcore_alloc_buf(*notes_sz);
    if (!*notes_buf)
    return -ENOMEM;
    rc = copy_notes_elf64(ehdr_ptr, *notes_buf);
    if (rc < 0)
    return rc;
// Prepare merged PT_NOTE program header.
    phdr.p_type    = PT_NOTE;
    phdr.p_flags   = 0;
    note_off = sizeof(Elf64_Ehdr) +
    (ehdr_ptr.e_phnum - nr_ptnote +1) * sizeof(Elf64_Phdr);
    phdr.p_offset  = roundup(note_off, PAGE_SIZE);
    phdr.p_vaddr   = phdr.p_paddr = 0;
    phdr.p_filesz  = phdr.p_memsz = phdr_sz;
    phdr.p_align   = 4;
// Add merged PT_NOTE program header
    tmp = elfptr + sizeof(Elf64_Ehdr);
    memcpy(tmp, &phdr, sizeof(phdr));
    tmp += sizeof(phdr);
// Remove unwanted PT_NOTE program headers.
    i = (nr_ptnote - 1) * sizeof(Elf64_Phdr);
// elfsz = *elfsz - i;
    memmove(tmp, tmp+i, ((*elfsz)-sizeof(Elf64_Ehdr)-sizeof(Elf64_Phdr)));
    memset(elfptr + *elfsz, 0, i);
// elfsz = roundup(*elfsz, PAGE_SIZE);
// Modify e_phnum to reflect merged headers.
    ehdr_ptr.e_phnum = ehdr_ptr.e_phnum - nr_ptnote + 1;
// Store the size of all notes.  We need this to update the note
// header when the device dumps will be added.
//
    elfnotes_orig_sz = phdr.p_memsz;
    return 0;
    }
//
// update_note_header_size_elf32 - update p_memsz member of each PT_NOTE entry
//
// @ehdr_ptr: ELF header
//
// This function updates p_memsz member of each PT_NOTE entry in the
// program header table pointed to by @ehdr_ptr to real size of ELF
// note segment.
//
#[no_mangle]
unsafe extern "C" fn update_note_header_size_elf32(ehdr_ptr: *const Elf32_Ehdr) -> int __init {
    static int __init update_note_header_size_elf32(const Elf32_Ehdr *ehdr_ptr)
    {
    int i, rc=0;
    Elf32_Phdr *phdr_ptr;
    Elf32_Nhdr *nhdr_ptr;
    phdr_ptr = (Elf32_Phdr *)(ehdr_ptr + 1);
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    void *notes_section;
    u64 offset, max_sz, sz, real_sz = 0;
    if (phdr_ptr.p_type != PT_NOTE)
    continue;
    max_sz = phdr_ptr.p_memsz;
    offset = phdr_ptr.p_offset;
    notes_section = kmalloc(max_sz, GFP_KERNEL);
    if (!notes_section)
    return -ENOMEM;
    rc = elfcorehdr_read_notes(notes_section, max_sz, &offset);
    if (rc < 0) {
    kfree(notes_section);
    return rc;
    }
    nhdr_ptr = notes_section;
    while (nhdr_ptr.n_namesz != 0) {
    sz = sizeof(Elf32_Nhdr) +
    (((u64)nhdr_ptr.n_namesz + 3) & ~3) +
    (((u64)nhdr_ptr.n_descsz + 3) & ~3);
    if ((real_sz + sz) > max_sz) {
    pr_warn("Warning: Exceeded p_memsz, dropping PT_NOTE entry n_namesz=0x%x, n_descsz=0x%x\n",
    nhdr_ptr.n_namesz, nhdr_ptr.n_descsz);
    break;
    }
    real_sz += sz;
    nhdr_ptr = (Elf32_Nhdr*)((char*)nhdr_ptr + sz);
    }
    kfree(notes_section);
    phdr_ptr.p_memsz = real_sz;
    if (real_sz == 0) {
    pr_warn("Warning: Zero PT_NOTE entries found\n");
    }
    }
    return 0;
    }
//
// get_note_number_and_size_elf32 - get the number of PT_NOTE program
// headers and sum of real size of their ELF note segment headers and
// data.
//
// @ehdr_ptr: ELF header
// @nr_ptnote: buffer for the number of PT_NOTE program headers
// @sz_ptnote: buffer for size of unique PT_NOTE program header
//
// This function is used to merge multiple PT_NOTE program headers
// into a unique single one. The resulting unique entry will have
// @sz_ptnote in its phdr->p_mem.
//
// It is assumed that program headers with PT_NOTE type pointed to by
// @ehdr_ptr has already been updated by update_note_header_size_elf32
// and each of PT_NOTE program headers has actual ELF note segment
// size in its p_memsz member.
//
    static int __init get_note_number_and_size_elf32(const Elf32_Ehdr *ehdr_ptr,
    int *nr_ptnote, u64 *sz_ptnote)
    {
    int i;
    Elf32_Phdr *phdr_ptr;
// nr_ptnote = *sz_ptnote = 0;
    phdr_ptr = (Elf32_Phdr *)(ehdr_ptr + 1);
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    if (phdr_ptr.p_type != PT_NOTE)
    continue;
// nr_ptnote += 1;
// sz_ptnote += phdr_ptr->p_memsz;
    }
    return 0;
    }
//
// copy_notes_elf32 - copy ELF note segments in a given buffer
//
// @ehdr_ptr: ELF header
// @notes_buf: buffer into which ELF note segments are copied
//
// This function is used to copy ELF note segment in the 1st kernel
// into the buffer @notes_buf in the 2nd kernel. It is assumed that
// size of the buffer @notes_buf is equal to or larger than sum of the
// real ELF note segment headers and data.
//
// It is assumed that program headers with PT_NOTE type pointed to by
// @ehdr_ptr has already been updated by update_note_header_size_elf32
// and each of PT_NOTE program headers has actual ELF note segment
// size in its p_memsz member.
//
#[no_mangle]
unsafe extern "C" fn copy_notes_elf32(ehdr_ptr: *const Elf32_Ehdr, notes_buf: *mut c_char) -> int __init {
    static int __init copy_notes_elf32(const Elf32_Ehdr *ehdr_ptr, char *notes_buf)
    {
    int i, rc=0;
    Elf32_Phdr *phdr_ptr;
    phdr_ptr = (Elf32_Phdr*)(ehdr_ptr + 1);
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    u64 offset;
    if (phdr_ptr.p_type != PT_NOTE)
    continue;
    offset = phdr_ptr.p_offset;
    rc = elfcorehdr_read_notes(notes_buf, phdr_ptr.p_memsz,
    &offset);
    if (rc < 0)
    return rc;
    notes_buf += phdr_ptr.p_memsz;
    }
    return 0;
    }
// Merges all the PT_NOTE headers into one.
    static int __init merge_note_headers_elf32(char *elfptr, size_t *elfsz,
    char **notes_buf, size_t *notes_sz)
    {
    int i, nr_ptnote=0, rc=0;
    char *tmp;
    Elf32_Ehdr *ehdr_ptr;
    Elf32_Phdr phdr;
    let mut phdr_sz: u64 = 0, note_off;
    ehdr_ptr = (Elf32_Ehdr *)elfptr;
    rc = update_note_header_size_elf32(ehdr_ptr);
    if (rc < 0)
    return rc;
    rc = get_note_number_and_size_elf32(ehdr_ptr, &nr_ptnote, &phdr_sz);
    if (rc < 0)
    return rc;
// notes_sz = roundup(phdr_sz, PAGE_SIZE);
// notes_buf = vmcore_alloc_buf(*notes_sz);
    if (!*notes_buf)
    return -ENOMEM;
    rc = copy_notes_elf32(ehdr_ptr, *notes_buf);
    if (rc < 0)
    return rc;
// Prepare merged PT_NOTE program header.
    phdr.p_type    = PT_NOTE;
    phdr.p_flags   = 0;
    note_off = sizeof(Elf32_Ehdr) +
    (ehdr_ptr.e_phnum - nr_ptnote +1) * sizeof(Elf32_Phdr);
    phdr.p_offset  = roundup(note_off, PAGE_SIZE);
    phdr.p_vaddr   = phdr.p_paddr = 0;
    phdr.p_filesz  = phdr.p_memsz = phdr_sz;
    phdr.p_align   = 4;
// Add merged PT_NOTE program header
    tmp = elfptr + sizeof(Elf32_Ehdr);
    memcpy(tmp, &phdr, sizeof(phdr));
    tmp += sizeof(phdr);
// Remove unwanted PT_NOTE program headers.
    i = (nr_ptnote - 1) * sizeof(Elf32_Phdr);
// elfsz = *elfsz - i;
    memmove(tmp, tmp+i, ((*elfsz)-sizeof(Elf32_Ehdr)-sizeof(Elf32_Phdr)));
    memset(elfptr + *elfsz, 0, i);
// elfsz = roundup(*elfsz, PAGE_SIZE);
// Modify e_phnum to reflect merged headers.
    ehdr_ptr.e_phnum = ehdr_ptr.e_phnum - nr_ptnote + 1;
// Store the size of all notes.  We need this to update the note
// header when the device dumps will be added.
//
    elfnotes_orig_sz = phdr.p_memsz;
    return 0;
    }
// Add memory chunks represented by program headers to vmcore list. Also update
// the new offset fields of exported program headers.
    static int __init process_ptload_program_headers_elf64(char *elfptr,
    size_t elfsz,
    size_t elfnotes_sz,
    struct list_head *vc_list)
    {
    int i;
    Elf64_Ehdr *ehdr_ptr;
    Elf64_Phdr *phdr_ptr;
    loff_t vmcore_off;
    ehdr_ptr = (Elf64_Ehdr *)elfptr;
    phdr_ptr = (Elf64_Phdr*)(elfptr + sizeof(Elf64_Ehdr)); /* PT_NOTE hdr */
// Skip ELF header, program headers and ELF note segment.
    vmcore_off = elfsz + elfnotes_sz;
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    u64 paddr, start, end, size;
    if (phdr_ptr.p_type != PT_LOAD)
    continue;
    paddr = phdr_ptr.p_offset;
    start = rounddown(paddr, PAGE_SIZE);
    end = roundup(paddr + phdr_ptr.p_memsz, PAGE_SIZE);
    size = end - start;
    if (vmcore_alloc_add_range(vc_list, start, size))
    return -ENOMEM;
// Update the program header offset.
    phdr_ptr.p_offset = vmcore_off + (paddr - start);
    vmcore_off = vmcore_off + size;
    }
    return 0;
    }
    static int __init process_ptload_program_headers_elf32(char *elfptr,
    size_t elfsz,
    size_t elfnotes_sz,
    struct list_head *vc_list)
    {
    int i;
    Elf32_Ehdr *ehdr_ptr;
    Elf32_Phdr *phdr_ptr;
    loff_t vmcore_off;
    ehdr_ptr = (Elf32_Ehdr *)elfptr;
    phdr_ptr = (Elf32_Phdr*)(elfptr + sizeof(Elf32_Ehdr)); /* PT_NOTE hdr */
// Skip ELF header, program headers and ELF note segment.
    vmcore_off = elfsz + elfnotes_sz;
    for (i = 0; i < ehdr_ptr.e_phnum; i++, phdr_ptr++) {
    u64 paddr, start, end, size;
    if (phdr_ptr.p_type != PT_LOAD)
    continue;
    paddr = phdr_ptr.p_offset;
    start = rounddown(paddr, PAGE_SIZE);
    end = roundup(paddr + phdr_ptr.p_memsz, PAGE_SIZE);
    size = end - start;
    if (vmcore_alloc_add_range(vc_list, start, size))
    return -ENOMEM;
// Update the program header offset
    phdr_ptr.p_offset = vmcore_off + (paddr - start);
    vmcore_off = vmcore_off + size;
    }
    return 0;
    }
// Sets offset fields of vmcore elements.
    static void set_vmcore_list_offsets(size_t elfsz, size_t elfnotes_sz,
    struct list_head *vc_list)
    {
    struct vmcore_range *m;
    loff_t vmcore_off;
// Skip ELF header, program headers and ELF note segment.
    vmcore_off = elfsz + elfnotes_sz;
    list_for_each_entry(m, vc_list, list) {
    m.offset = vmcore_off;
    vmcore_off += m.size;
    }
    }
#[no_mangle]
unsafe extern "C" fn free_elfcorebuf() {
    static void free_elfcorebuf(void)
    {
    free_pages((unsigned long)elfcorebuf, get_order(elfcorebuf_sz_orig));
    elfcorebuf = core::ptr::null_mut();
    vfree(elfnotes_buf);
    elfnotes_buf = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn parse_crash_elf64_headers() -> int __init {
    static int __init parse_crash_elf64_headers(void)
    {
    let mut rc: c_int = 0;
    Elf64_Ehdr ehdr;
    u64 addr;
    addr = elfcorehdr_addr;
// Read ELF header
    rc = elfcorehdr_read((char *)&ehdr, sizeof(Elf64_Ehdr), &addr);
    if (rc < 0)
    return rc;
// Do some basic Verification.
    if (memcmp(ehdr.e_ident, ELFMAG, SELFMAG) != 0 ||
    (ehdr.e_type != ET_CORE) ||
    !vmcore_elf64_check_arch(&ehdr) ||
    ehdr.e_ident[EI_CLASS] != ELFCLASS64 ||
    ehdr.e_ident[EI_VERSION] != EV_CURRENT ||
    ehdr.e_version != EV_CURRENT ||
    ehdr.e_ehsize != sizeof(Elf64_Ehdr) ||
    ehdr.e_phentsize != sizeof(Elf64_Phdr) ||
    ehdr.e_phnum == 0) {
    pr_warn("Warning: Core image elf header is not sane\n");
    return -EINVAL;
    }
// Read in all elf headers.
    elfcorebuf_sz_orig = sizeof(Elf64_Ehdr) +
    ehdr.e_phnum * sizeof(Elf64_Phdr);
    elfcorebuf_sz = elfcorebuf_sz_orig;
    elfcorebuf = (void *)__get_free_pages(GFP_KERNEL | __GFP_ZERO,
    get_order(elfcorebuf_sz_orig));
    if (!elfcorebuf)
    return -ENOMEM;
    addr = elfcorehdr_addr;
    rc = elfcorehdr_read(elfcorebuf, elfcorebuf_sz_orig, &addr);
    if (rc < 0)
    goto fail;
// Merge all PT_NOTE headers into one.
    rc = merge_note_headers_elf64(elfcorebuf, &elfcorebuf_sz,
    &elfnotes_buf, &elfnotes_sz);
    if (rc)
    goto fail;
    rc = process_ptload_program_headers_elf64(elfcorebuf, elfcorebuf_sz,
    elfnotes_sz, &vmcore_list);
    if (rc)
    goto fail;
    set_vmcore_list_offsets(elfcorebuf_sz, elfnotes_sz, &vmcore_list);
    return 0;
    fail:
    free_elfcorebuf();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn parse_crash_elf32_headers() -> int __init {
    static int __init parse_crash_elf32_headers(void)
    {
    let mut rc: c_int = 0;
    Elf32_Ehdr ehdr;
    u64 addr;
    addr = elfcorehdr_addr;
// Read ELF header
    rc = elfcorehdr_read((char *)&ehdr, sizeof(Elf32_Ehdr), &addr);
    if (rc < 0)
    return rc;
// Do some basic Verification.
    if (memcmp(ehdr.e_ident, ELFMAG, SELFMAG) != 0 ||
    (ehdr.e_type != ET_CORE) ||
    !vmcore_elf32_check_arch(&ehdr) ||
    ehdr.e_ident[EI_CLASS] != ELFCLASS32||
    ehdr.e_ident[EI_VERSION] != EV_CURRENT ||
    ehdr.e_version != EV_CURRENT ||
    ehdr.e_ehsize != sizeof(Elf32_Ehdr) ||
    ehdr.e_phentsize != sizeof(Elf32_Phdr) ||
    ehdr.e_phnum == 0) {
    pr_warn("Warning: Core image elf header is not sane\n");
    return -EINVAL;
    }
// Read in all elf headers.
    elfcorebuf_sz_orig = sizeof(Elf32_Ehdr) + ehdr.e_phnum * sizeof(Elf32_Phdr);
    elfcorebuf_sz = elfcorebuf_sz_orig;
    elfcorebuf = (void *)__get_free_pages(GFP_KERNEL | __GFP_ZERO,
    get_order(elfcorebuf_sz_orig));
    if (!elfcorebuf)
    return -ENOMEM;
    addr = elfcorehdr_addr;
    rc = elfcorehdr_read(elfcorebuf, elfcorebuf_sz_orig, &addr);
    if (rc < 0)
    goto fail;
// Merge all PT_NOTE headers into one.
    rc = merge_note_headers_elf32(elfcorebuf, &elfcorebuf_sz,
    &elfnotes_buf, &elfnotes_sz);
    if (rc)
    goto fail;
    rc = process_ptload_program_headers_elf32(elfcorebuf, elfcorebuf_sz,
    elfnotes_sz, &vmcore_list);
    if (rc)
    goto fail;
    set_vmcore_list_offsets(elfcorebuf_sz, elfnotes_sz, &vmcore_list);
    return 0;
    fail:
    free_elfcorebuf();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn parse_crash_elf_headers() -> int __init {
    static int __init parse_crash_elf_headers(void)
    {
    unsigned char e_ident[EI_NIDENT];
    u64 addr;
    let mut rc: c_int = 0;
    addr = elfcorehdr_addr;
    rc = elfcorehdr_read(e_ident, EI_NIDENT, &addr);
    if (rc < 0)
    return rc;
    if (memcmp(e_ident, ELFMAG, SELFMAG) != 0) {
    pr_warn("Warning: Core image elf header not found\n");
    return -EINVAL;
    }
    if (e_ident[EI_CLASS] == ELFCLASS64) {
    rc = parse_crash_elf64_headers();
    if (rc)
    return rc;
    } else if (e_ident[EI_CLASS] == ELFCLASS32) {
    rc = parse_crash_elf32_headers();
    if (rc)
    return rc;
    } else {
    pr_warn("Warning: Core image elf header is not sane\n");
    return -EINVAL;
    }
// Determine vmcore size.
    vmcore_size = get_vmcore_size(elfcorebuf_sz, elfnotes_sz,
    &vmcore_list);
    return 0;
    }

//
// vmcoredd_write_header - Write vmcore device dump header at the
// beginning of the dump's buffer.
// @buf: Output buffer where the note is written
// @data: Dump info
// @size: Size of the dump
//
// Fills beginning of the dump's buffer with vmcore device dump header.
//
    static void vmcoredd_write_header(void *buf, struct vmcoredd_data *data,
    u32 size)
    {
    struct vmcoredd_header *vdd_hdr = (struct vmcoredd_header *)buf;
    vdd_hdr.n_namesz = sizeof(vdd_hdr.name);
    vdd_hdr.n_descsz = size + sizeof(vdd_hdr.dump_name);
    vdd_hdr.n_type = NT_VMCOREDD;
    strscpy_pad(vdd_hdr.name, VMCOREDD_NOTE_NAME);
    strscpy_pad(vdd_hdr.dump_name, data.dump_name);
    }
//
// vmcoredd_update_program_headers - Update all ELF program headers
// @elfptr: Pointer to elf header
// @elfnotesz: Size of elf notes aligned to page size
// @vmcoreddsz: Size of device dumps to be added to elf note header
//
// Determine type of ELF header (Elf64 or Elf32) and update the elf note size.
// Also update the offsets of all the program headers after the elf note header.
//
    static void vmcoredd_update_program_headers(char *elfptr, size_t elfnotesz,
    size_t vmcoreddsz)
    {
    unsigned char *e_ident = (unsigned char *)elfptr;
    u64 start, end, size;
    loff_t vmcore_off;
    u32 i;
    vmcore_off = elfcorebuf_sz + elfnotesz;
    if (e_ident[EI_CLASS] == ELFCLASS64) {
    Elf64_Ehdr *ehdr = (Elf64_Ehdr *)elfptr;
    Elf64_Phdr *phdr = (Elf64_Phdr *)(elfptr + sizeof(Elf64_Ehdr));
// Update all program headers
    for (i = 0; i < ehdr.e_phnum; i++, phdr++) {
    if (phdr.p_type == PT_NOTE) {
// Update note size
    phdr.p_memsz = elfnotes_orig_sz + vmcoreddsz;
    phdr.p_filesz = phdr.p_memsz;
    continue;
    }
    start = rounddown(phdr.p_offset, PAGE_SIZE);
    end = roundup(phdr.p_offset + phdr.p_memsz,
    PAGE_SIZE);
    size = end - start;
    phdr.p_offset = vmcore_off + (phdr.p_offset - start);
    vmcore_off += size;
    }
    } else {
    Elf32_Ehdr *ehdr = (Elf32_Ehdr *)elfptr;
    Elf32_Phdr *phdr = (Elf32_Phdr *)(elfptr + sizeof(Elf32_Ehdr));
// Update all program headers
    for (i = 0; i < ehdr.e_phnum; i++, phdr++) {
    if (phdr.p_type == PT_NOTE) {
// Update note size
    phdr.p_memsz = elfnotes_orig_sz + vmcoreddsz;
    phdr.p_filesz = phdr.p_memsz;
    continue;
    }
    start = rounddown(phdr.p_offset, PAGE_SIZE);
    end = roundup(phdr.p_offset + phdr.p_memsz,
    PAGE_SIZE);
    size = end - start;
    phdr.p_offset = vmcore_off + (phdr.p_offset - start);
    vmcore_off += size;
    }
    }
    }
//
// vmcoredd_update_size - Update the total size of the device dumps and update
// ELF header
// @dump_size: Size of the current device dump to be added to total size
//
// Update the total size of all the device dumps and update the ELF program
// headers. Calculate the new offsets for the vmcore list and update the
// total vmcore size.
//
#[no_mangle]
unsafe extern "C" fn vmcoredd_update_size(dump_size: usize) {
    static void vmcoredd_update_size(size_t dump_size)
    {
    vmcoredd_orig_sz += dump_size;
    elfnotes_sz = roundup(elfnotes_orig_sz, PAGE_SIZE) + vmcoredd_orig_sz;
    vmcoredd_update_program_headers(elfcorebuf, elfnotes_sz,
    vmcoredd_orig_sz);
// Update vmcore list offsets
    set_vmcore_list_offsets(elfcorebuf_sz, elfnotes_sz, &vmcore_list);
    vmcore_size = get_vmcore_size(elfcorebuf_sz, elfnotes_sz,
    &vmcore_list);
    proc_vmcore.size = vmcore_size;
    }
//
// vmcore_add_device_dump - Add a buffer containing device dump to vmcore
// @data: dump info.
//
// Allocate a buffer and invoke the calling driver's dump collect routine.
// Write ELF note at the beginning of the buffer to indicate vmcore device
// dump and add the dump to global list.
//
#[no_mangle]
pub unsafe extern "C" fn vmcore_add_device_dump(data: *mut vmcoredd_data) -> c_int {
    int vmcore_add_device_dump(struct vmcoredd_data *data)
    {
    struct vmcoredd_node *dump;
    void *buf = core::ptr::null_mut();
    size_t data_size;
    int ret;
    if (vmcoredd_disabled) {
    pr_err_once("Device dump is disabled\n");
    return -EINVAL;
    }
    if (!data || !strlen(data.dump_name) ||
    !data.vmcoredd_callback || !data.size)
    return -EINVAL;
    dump = vzalloc(sizeof(*dump));
    if (!dump)
    return -ENOMEM;
// Keep size of the buffer page aligned so that it can be mmaped
    data_size = roundup(sizeof(struct vmcoredd_header) + data.size,
    PAGE_SIZE);
// Allocate buffer for driver's to write their dumps
    buf = vmcore_alloc_buf(data_size);
    if (!buf) {
    ret = -ENOMEM;
    goto out_err;
    }
    vmcoredd_write_header(buf, data, data_size -
    sizeof(struct vmcoredd_header));
// Invoke the driver's dump collection routing
    ret = data.vmcoredd_callback(data, buf +
    sizeof(struct vmcoredd_header));
    if (ret)
    goto out_err;
    dump.buf = buf;
    dump.size = data_size;
// Add the dump to driver sysfs list and update the elfcore hdr
    scoped_guard(mutex, &vmcore_mutex) {
    if (vmcore_opened)
    pr_warn_once("Unexpected adding of device dump\n");
    if (vmcore_open) {
    ret = -EBUSY;
    goto out_err;
    }
    list_add_tail(&dump.list, &vmcoredd_list);
    vmcoredd_update_size(data_size);
    }
    return 0;
    out_err:
    vfree(buf);
    vfree(dump);
    return ret;
    }
    EXPORT_SYMBOL(vmcore_add_device_dump);

#[no_mangle]
unsafe extern "C" fn vmcore_realloc_elfcore_buffer_elf64(new_size: usize) -> c_int {
    static int vmcore_realloc_elfcore_buffer_elf64(size_t new_size)
    {
    char *elfcorebuf_new;
    if (WARN_ON_ONCE(new_size < elfcorebuf_sz))
    return -EINVAL;
    if (get_order(elfcorebuf_sz_orig) == get_order(new_size)) {
    elfcorebuf_sz_orig = new_size;
    return 0;
    }
    elfcorebuf_new = (void *)__get_free_pages(GFP_KERNEL | __GFP_ZERO,
    get_order(new_size));
    if (!elfcorebuf_new)
    return -ENOMEM;
    memcpy(elfcorebuf_new, elfcorebuf, elfcorebuf_sz);
    free_pages((unsigned long)elfcorebuf, get_order(elfcorebuf_sz_orig));
    elfcorebuf = elfcorebuf_new;
    elfcorebuf_sz_orig = new_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmcore_reset_offsets_elf64() {
    static void vmcore_reset_offsets_elf64(void)
    {
    Elf64_Phdr *phdr_start = (Elf64_Phdr *)(elfcorebuf + sizeof(Elf64_Ehdr));
    let mut vmcore_off: loff_t = elfcorebuf_sz + elfnotes_sz;
    Elf64_Ehdr *ehdr = (Elf64_Ehdr *)elfcorebuf;
    Elf64_Phdr *phdr;
    int i;
    for (i = 0, phdr = phdr_start; i < ehdr.e_phnum; i++, phdr++) {
    u64 start, end;
//
// After merge_note_headers_elf64() we should only have a single
// PT_NOTE entry that starts immediately after elfcorebuf_sz.
//
    if (phdr.p_type == PT_NOTE) {
    phdr.p_offset = elfcorebuf_sz;
    continue;
    }
    start = rounddown(phdr.p_offset, PAGE_SIZE);
    end = roundup(phdr.p_offset + phdr.p_memsz, PAGE_SIZE);
    phdr.p_offset = vmcore_off + (phdr.p_offset - start);
    vmcore_off = vmcore_off + end - start;
    }
    set_vmcore_list_offsets(elfcorebuf_sz, elfnotes_sz, &vmcore_list);
    }
#[no_mangle]
unsafe extern "C" fn vmcore_add_device_ram_elf64(list: *mut list_head, count: usize) -> c_int {
    static int vmcore_add_device_ram_elf64(struct list_head *list, size_t count)
    {
    Elf64_Phdr *phdr_start = (Elf64_Phdr *)(elfcorebuf + sizeof(Elf64_Ehdr));
    Elf64_Ehdr *ehdr = (Elf64_Ehdr *)elfcorebuf;
    struct vmcore_range *cur;
    Elf64_Phdr *phdr;
    size_t new_size;
    int rc;
    if ((Elf32_Half)(ehdr.e_phnum + count) != ehdr.e_phnum + count) {
    pr_err("too many device ram ranges\n");
    return -ENOSPC;
    }
// elfcorebuf_sz must always cover full pages.
    new_size = sizeof(Elf64_Ehdr) +
    (ehdr.e_phnum + count) * sizeof(Elf64_Phdr);
    new_size = roundup(new_size, PAGE_SIZE);
//
// Make sure we have sufficient space to include the new PT_LOAD
// entries.
//
    rc = vmcore_realloc_elfcore_buffer_elf64(new_size);
    if (rc) {
    pr_err("resizing elfcore failed\n");
    return rc;
    }
// Modify our used elfcore buffer size to cover the new entries.
    elfcorebuf_sz = new_size;
// Fill the added PT_LOAD entries.
    phdr = phdr_start + ehdr.e_phnum;
    list_for_each_entry(cur, list, list) {
    WARN_ON_ONCE(!IS_ALIGNED(cur.paddr | cur.size, PAGE_SIZE));
    elfcorehdr_fill_device_ram_ptload_elf64(phdr, cur.paddr, cur.size);
// p_offset will be adjusted later.
    phdr++;
    ehdr.e_phnum++;
    }
    list_splice_tail(list, &vmcore_list);
// We changed elfcorebuf_sz and added new entries; reset all offsets.
    vmcore_reset_offsets_elf64();
// Finally, recalculate the total vmcore size.
    vmcore_size = get_vmcore_size(elfcorebuf_sz, elfnotes_sz,
    &vmcore_list);
    proc_vmcore.size = vmcore_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmcore_process_device_ram(cb: *mut vmcore_cb) {
    static void vmcore_process_device_ram(struct vmcore_cb *cb)
    {
    unsigned char *e_ident = (unsigned char *)elfcorebuf;
    struct vmcore_range *first, *m;
    LIST_HEAD(list);
    int count;
// We only support Elf64 dumps for now.
    if (WARN_ON_ONCE(e_ident[EI_CLASS] != ELFCLASS64)) {
    pr_err("device ram ranges only support Elf64\n");
    return;
    }
    if (cb.get_device_ram(cb, &list)) {
    pr_err("obtaining device ram ranges failed\n");
    return;
    }
    count = list_count_nodes(&list);
    if (!count)
    return;
//
// For some reason these ranges are already know? Might happen
// with unusual register->unregister->register sequences; we'll simply
// sanity check using the first range.
//
    first = list_first_entry(&list, struct vmcore_range, list);
    list_for_each_entry(m, &vmcore_list, list) {
    let mut m_end: c_ulonglong = m.paddr + m.size;
    let mut first_end: c_ulonglong = first.paddr + first.size;
    if (first.paddr < m_end && m.paddr < first_end)
    goto out_free;
    }
// If adding the mem nodes succeeds, they must not be freed.
    if (!vmcore_add_device_ram_elf64(&list, count))
    return;
    out_free:
    vmcore_free_ranges(&list);
    }

#[no_mangle]
unsafe extern "C" fn vmcore_process_device_ram(cb: *mut vmcore_cb) {
    static void vmcore_process_device_ram(struct vmcore_cb *cb)
    {
    }

// Free all dumps in vmcore device dump list
#[no_mangle]
unsafe extern "C" fn vmcore_free_device_dumps() {
    static void vmcore_free_device_dumps(void)
    {

    mutex_lock(&vmcore_mutex);
    while (!list_empty(&vmcoredd_list)) {
    struct vmcoredd_node *dump;
    dump = list_first_entry(&vmcoredd_list, struct vmcoredd_node,
    list);
    list_del(&dump.list);
    vfree(dump.buf);
    vfree(dump);
    }
    mutex_unlock(&vmcore_mutex);

    }
// Init function for vmcore module.
#[no_mangle]
unsafe extern "C" fn vmcore_init() -> int __init {
    static int __init vmcore_init(void)
    {
    let mut rc: c_int = 0;
// Allow architectures to allocate ELF header in 2nd kernel
    rc = elfcorehdr_alloc(&elfcorehdr_addr, &elfcorehdr_size);
    if (rc)
    return rc;
//
// If elfcorehdr= has been passed in cmdline or created in 2nd kernel,
// then capture the dump.
//
    if (!(is_vmcore_usable()))
    return rc;
    rc = parse_crash_elf_headers();
    if (rc) {
    elfcorehdr_free(elfcorehdr_addr);
    pr_warn("not initialized\n");
    return rc;
    }
    elfcorehdr_free(elfcorehdr_addr);
    elfcorehdr_addr = ELFCORE_ADDR_ERR;
    proc_vmcore = proc_create("vmcore", S_IRUSR, core::ptr::null_mut(), &vmcore_proc_ops);
    if (proc_vmcore)
    proc_vmcore.size = vmcore_size;
    return 0;
    }
    fs_initcall(vmcore_init);
// Cleanup function for vmcore module.
#[no_mangle]
pub unsafe extern "C" fn vmcore_cleanup() {
    void vmcore_cleanup(void)
    {
    if (proc_vmcore) {
    proc_remove(proc_vmcore);
    proc_vmcore = core::ptr::null_mut();
    }
    vmcore_free_ranges(&vmcore_list);
    free_elfcorebuf();
// clear vmcore device dump list
    vmcore_free_device_dumps();
    }
