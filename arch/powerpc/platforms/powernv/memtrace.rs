//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/memtrace.c
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
// Copyright (C) IBM Corporation, 2014, 2017
// Anton Blanchard, Rashmica Gupta.
//

// This enables us to keep track of the memory removed from each node.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memtrace_entry {
    pub mem: *mut c_void,
    pub start: u64,
    pub size: u64,
    pub nid: u32,
    pub dir: *mut dentry,
    pub name: [c_char; 16],
}

    static DEFINE_MUTEX(memtrace_mutex);
    static u64 memtrace_size;
    static struct memtrace_entry *memtrace_array;
    static unsigned int memtrace_array_nr;
    static ssize_t memtrace_read(struct file *filp, char __user *ubuf,
    size_t count, loff_t *ppos)
    {
    struct memtrace_entry *ent = filp.private_data;
    return simple_read_from_buffer(ubuf, count, ppos, ent.mem, ent.size);
    }
#[no_mangle]
unsafe extern "C" fn memtrace_mmap(filp: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int memtrace_mmap(struct file *filp, struct vm_area_struct *vma)
    {
    struct memtrace_entry *ent = filp.private_data;
    let mut ent_nrpages: c_ulong = ent.size >> PAGE_SHIFT;
    let mut vma_nrpages: c_ulong = vma_pages(vma);
// The requested page offset should be within object's page count
    if (vma.vm_pgoff >= ent_nrpages)
    return -EINVAL;
// The requested mapping range should remain within the bounds
    if (vma_nrpages > ent_nrpages - vma.vm_pgoff)
    return -EINVAL;
    vma.vm_page_prot = pgprot_noncached(vma.vm_page_prot);
    return remap_pfn_range(vma, vma.vm_start, PHYS_PFN(ent.start) + vma.vm_pgoff,
    vma.vm_end - vma.vm_start, vma.vm_page_prot);
    }
    static const struct file_operations memtrace_fops = {
    .llseek = default_llseek,
    .read	= memtrace_read,
    .open	= simple_open,
    .mmap   = memtrace_mmap,
    };

//
// flush_dcache_range_chunked(): Write any modified data cache blocks out to
// memory and invalidate them, in chunks of up to FLUSH_CHUNK_SIZE
// Does not invalidate the corresponding instruction cache blocks.
//
// @start: the start address
// @stop: the stop address (exclusive)
// @chunk: the max size of the chunks
//
    static void flush_dcache_range_chunked(unsigned long start, unsigned long stop,
    unsigned long chunk)
    {
    unsigned long i;
    for (i = start; i < stop; i += chunk) {
    flush_dcache_range(i, min(stop, i + chunk));
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn memtrace_alloc_node(nid: u32, size: u64) -> u64 {
    static u64 memtrace_alloc_node(u32 nid, u64 size)
    {
    let mut nr_pages: c_ulong = PHYS_PFN(size);
    unsigned long pfn, start_pfn;
    struct page *page;
//
// Trace memory needs to be aligned to the size, which is guaranteed
// by alloc_contig_pages().
//
    page = alloc_contig_pages(nr_pages, GFP_KERNEL | __GFP_THISNODE |
    __GFP_NOWARN | __GFP_ZERO, nid, core::ptr::null_mut());
    if (!page)
    return 0;
    start_pfn = page_to_pfn(page);
//
// Before we go ahead and use this range as cache inhibited range
// flush the cache.
//
    flush_dcache_range_chunked((unsigned long)pfn_to_kaddr(start_pfn),
    (unsigned long)pfn_to_kaddr(start_pfn + nr_pages),
    FLUSH_CHUNK_SIZE);
//
// Set pages PageOffline(), to indicate that nobody (e.g., hibernation,
// dumping, ...) should be touching these pages.
//
    for (pfn = start_pfn; pfn < start_pfn + nr_pages; pfn++)
    __SetPageOffline(pfn_to_page(pfn));
    arch_remove_linear_mapping(PFN_PHYS(start_pfn), size);
    return PFN_PHYS(start_pfn);
    }
#[no_mangle]
unsafe extern "C" fn memtrace_init_regions_runtime(size: u64) -> c_int {
    static int memtrace_init_regions_runtime(u64 size)
    {
    u32 nid;
    u64 m;
    memtrace_array = kzalloc_objs(struct memtrace_entry, num_online_nodes());
    if (!memtrace_array) {
    pr_err("Failed to allocate memtrace_array\n");
    return -EINVAL;
    }
    for_each_online_node(nid) {
    m = memtrace_alloc_node(nid, size);
//
// A node might not have any local memory, so warn but
// continue on.
//
    if (!m) {
    pr_err("Failed to allocate trace memory on node %d\n", nid);
    continue;
    }
    pr_info("Allocated trace memory on node %d at 0x%016llx\n", nid, m);
    memtrace_array[memtrace_array_nr].start = m;
    memtrace_array[memtrace_array_nr].size = size;
    memtrace_array[memtrace_array_nr].nid = nid;
    memtrace_array_nr++;
    }
    return 0;
    }
    static struct dentry *memtrace_debugfs_dir;
#[no_mangle]
unsafe extern "C" fn memtrace_init_debugfs() -> c_int {
    static int memtrace_init_debugfs(void)
    {
    let mut ret: c_int = 0;
    int i;
    for (i = 0; i < memtrace_array_nr; i++) {
    struct dentry *dir;
    struct memtrace_entry *ent = &memtrace_array[i];
    ent.mem = ioremap(ent.start, ent.size);
// Warn but continue on
    if (!ent.mem) {
    pr_err("Failed to map trace memory at 0x%llx\n",
    ent.start);
    ret = -1;
    continue;
    }
    snprintf(ent.name, 16, "%08x", ent.nid);
    dir = debugfs_create_dir(ent.name, memtrace_debugfs_dir);
    ent.dir = dir;
    debugfs_create_file_unsafe("trace", 0600, dir, ent, &memtrace_fops);
    debugfs_create_x64("start", 0400, dir, &ent.start);
    debugfs_create_x64("size", 0400, dir, &ent.size);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn memtrace_free(nid: c_int, start: u64, size: u64) -> c_int {
    static int memtrace_free(int nid, u64 start, u64 size)
    {
    let mut params: mhp_params = { .pgprot = PAGE_KERNEL };
    let mut nr_pages: c_ulong = PHYS_PFN(size);
    let mut start_pfn: c_ulong = PHYS_PFN(start);
    unsigned long pfn;
    int ret;
    ret = arch_create_linear_mapping(nid, start, size, &params);
    if (ret)
    return ret;
    for (pfn = start_pfn; pfn < start_pfn + nr_pages; pfn++)
    __ClearPageOffline(pfn_to_page(pfn));
    free_contig_range(start_pfn, nr_pages);
    return 0;
    }
//
// Iterate through the chunks of memory we allocated and attempt to expose
// them back to the kernel.
//
#[no_mangle]
unsafe extern "C" fn memtrace_free_regions() -> c_int {
    static int memtrace_free_regions(void)
    {
    int i, ret = 0;
    struct memtrace_entry *ent;
    for (i = memtrace_array_nr - 1; i >= 0; i--) {
    ent = &memtrace_array[i];
// We have freed this chunk previously
    if (ent.nid == NUMA_NO_NODE)
    continue;
// Remove from io mappings
    if (ent.mem) {
    iounmap(ent.mem);
    ent.mem = 0;
    }
    if (memtrace_free(ent.nid, ent.start, ent.size)) {
    pr_err("Failed to free trace memory on node %d\n",
    ent.nid);
    ret += 1;
    continue;
    }
//
// Memory was freed successfully so clean up references to it
// so on reentry we can tell that this chunk was freed.
//
    debugfs_remove_recursive(ent.dir);
    pr_info("Freed trace memory back on node %d\n", ent.nid);
    ent.size = ent.start = ent.nid = NUMA_NO_NODE;
    }
    if (ret)
    return ret;
// If all chunks of memory were freed successfully, reset globals
    kfree(memtrace_array);
    memtrace_array = core::ptr::null_mut();
    memtrace_size = 0;
    memtrace_array_nr = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn memtrace_enable_set(data: *mut c_void, val: u64) -> c_int {
    static int memtrace_enable_set(void *data, u64 val)
    {
    let mut rc: c_int = -EAGAIN;
    u64 bytes;
//
// Don't attempt to do anything if size isn't aligned to a memory
// block or equal to zero.
//
    bytes = memory_block_size_bytes();
    if (val & (bytes - 1)) {
    pr_err("Value must be aligned with 0x%llx\n", bytes);
    return -EINVAL;
    }
    mutex_lock(&memtrace_mutex);
// Free all previously allocated memory.
    if (memtrace_size && memtrace_free_regions())
    goto out_unlock;
    if (!val) {
    rc = 0;
    goto out_unlock;
    }
// Allocate memory.
    if (memtrace_init_regions_runtime(val))
    goto out_unlock;
    if (memtrace_init_debugfs())
    goto out_unlock;
    memtrace_size = val;
    rc = 0;
    out_unlock:
    mutex_unlock(&memtrace_mutex);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn memtrace_enable_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int memtrace_enable_get(void *data, u64 *val)
    {
// val = memtrace_size;
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(memtrace_init_fops, memtrace_enable_get,
    memtrace_enable_set, "0x%016llx\n");
#[no_mangle]
unsafe extern "C" fn memtrace_init() -> c_int {
    static int memtrace_init(void)
    {
    memtrace_debugfs_dir = debugfs_create_dir("memtrace",
    arch_debugfs_dir);
    debugfs_create_file("enable", 0600, memtrace_debugfs_dir,
    core::ptr::null_mut(), &memtrace_init_fops);
    return 0;
    }
    machine_device_initcall(powernv, memtrace_init);
