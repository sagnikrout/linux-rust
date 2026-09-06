//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/efi/memmap.c
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
// Common EFI memory map functions.
//

#[no_mangle]
unsafe extern "C" fn __efi_memmap_alloc_early(size: c_ulong) -> phys_addr_t __init {
    static phys_addr_t __init __efi_memmap_alloc_early(unsigned long size)
    {
    return memblock_phys_alloc(size, SMP_CACHE_BYTES);
    }
#[no_mangle]
unsafe extern "C" fn __efi_memmap_alloc_late(size: c_ulong) -> phys_addr_t __init {
    static phys_addr_t __init __efi_memmap_alloc_late(unsigned long size)
    {
    let mut order: c_uint = get_order(size);
    struct page *p = alloc_pages(GFP_KERNEL, order);
    if (!p)
    return 0;
    return PFN_PHYS(page_to_pfn(p));
    }
    static
#[no_mangle]
pub unsafe extern "C" fn __efi_memmap_free(phys: u64, size: c_ulong, flags: c_ulong) -> void __init {
    void __init __efi_memmap_free(u64 phys, unsigned long size, unsigned long flags)
    {
    if (flags & EFI_MEMMAP_MEMBLOCK) {
    memblock_phys_free(phys, size);
    } else if (flags & EFI_MEMMAP_SLAB) {
    struct page *p = pfn_to_page(PHYS_PFN(phys));
    let mut order: c_uint = get_order(size);
    __free_pages(p, order);
    }
    }
//
// efi_memmap_alloc - Allocate memory for the EFI memory map
// @num_entries: Number of entries in the allocated map.
// @data: efi memmap installation parameters
//
// Depending on whether mm_init() has already been invoked or not,
// either memblock or "normal" page allocation is used.
//
// Returns zero on success, a negative error code on failure.
//
    int __init efi_memmap_alloc(unsigned int num_entries,
    struct efi_memory_map_data *data)
    {
// Expect allocation parameters are zero initialized
    WARN_ON(data.phys_map || data.size);
    data.size = num_entries * efi.memmap.desc_size;
    data.desc_version = efi.memmap.desc_version;
    data.desc_size = efi.memmap.desc_size;
    data.flags &= ~(EFI_MEMMAP_SLAB | EFI_MEMMAP_MEMBLOCK);
    data.flags |= efi.memmap.flags & EFI_MEMMAP_LATE;
    if (slab_is_available()) {
    data.flags |= EFI_MEMMAP_SLAB;
    data.phys_map = __efi_memmap_alloc_late(data.size);
    } else {
    data.flags |= EFI_MEMMAP_MEMBLOCK;
    data.phys_map = __efi_memmap_alloc_early(data.size);
    }
    if (!data.phys_map)
    return -ENOMEM;
    return 0;
    }
//
// efi_memmap_install - Install a new EFI memory map in efi.memmap
// @data: efi memmap installation parameters
//
// Unlike efi_memmap_init_*(), this function does not allow the caller
// to switch from early to late mappings. It simply uses the existing
// mapping function and installs the new memmap.
//
// Returns zero on success, a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn efi_memmap_install(data: *mut efi_memory_map_data) -> int __init {
    int __init efi_memmap_install(struct efi_memory_map_data *data)
    {
    let mut size: c_ulong = efi.memmap.desc_size * efi.memmap.nr_map;
    let mut flags: c_ulong = efi.memmap.flags;
    let mut phys: u64 = efi.memmap.phys_map;
    int ret;
    efi_memmap_unmap();
    if (efi_enabled(EFI_PARAVIRT))
    return 0;
    ret = __efi_memmap_init(data);
    if (ret)
    return ret;
    __efi_memmap_free(phys, size, flags);
    return 0;
    }
//
// efi_memmap_split_count - Count number of additional EFI memmap entries
// @md: EFI memory descriptor to split
// @range: Address range (start, end) to split around
//
// Returns the number of additional EFI memmap entries required to
// accommodate @range.
//
#[no_mangle]
pub unsafe extern "C" fn efi_memmap_split_count(md: *mut efi_memory_desc_t, range: *mut range) -> int __init {
    int __init efi_memmap_split_count(efi_memory_desc_t *md, struct range *range)
    {
    u64 m_start, m_end;
    u64 start, end;
    let mut count: c_int = 0;
    start = md.phys_addr;
    end = start + (md.num_pages << EFI_PAGE_SHIFT) - 1;
// modifying range
    m_start = range.start;
    m_end = range.end;
    if (m_start <= start) {
// split into 2 parts
    if (start < m_end && m_end < end)
    count++;
    }
    if (start < m_start && m_start < end) {
// split into 3 parts
    if (m_end < end)
    count += 2;
// split into 2 parts
    if (end <= m_end)
    count++;
    }
    return count;
    }
//
// efi_memmap_insert - Insert a memory region in an EFI memmap
// @old_memmap: The existing EFI memory map structure
// @buf: Address of buffer to store new map
// @mem: Memory map entry to insert
//
// It is suggested that you call efi_memmap_split_count() first
// to see how large @buf needs to be.
//
    void __init efi_memmap_insert(struct efi_memory_map *old_memmap, void *buf,
    struct efi_mem_range *mem)
    {
    u64 m_start, m_end, m_attr;
    efi_memory_desc_t *md;
    u64 start, end;
    void *old, *new;
// modifying range
    m_start = mem.range.start;
    m_end = mem.range.end;
    m_attr = mem.attribute;
//
// The EFI memory map deals with regions in EFI_PAGE_SIZE
// units. Ensure that the region described by 'mem' is aligned
// correctly.
//
    if (!IS_ALIGNED(m_start, EFI_PAGE_SIZE) ||
    !IS_ALIGNED(m_end + 1, EFI_PAGE_SIZE)) {
    WARN_ON(1);
    return;
    }
    for (old = old_memmap.map, new = buf;
    old < old_memmap.map_end;
    old += old_memmap.desc_size, new += old_memmap.desc_size) {
// copy original EFI memory descriptor
    memcpy(new, old, old_memmap.desc_size);
    md = new;
    start = md.phys_addr;
    end = md.phys_addr + (md.num_pages << EFI_PAGE_SHIFT) - 1;
    if (m_start <= start && end <= m_end)
    md.attribute |= m_attr;
    if (m_start <= start &&
    (start < m_end && m_end < end)) {
// first part
    md.attribute |= m_attr;
    md.num_pages = (m_end - md.phys_addr + 1) >>
    EFI_PAGE_SHIFT;
// latter part
    new += old_memmap.desc_size;
    memcpy(new, old, old_memmap.desc_size);
    md = new;
    md.phys_addr = m_end + 1;
    md.num_pages = (end - md.phys_addr + 1) >>
    EFI_PAGE_SHIFT;
    }
    if ((start < m_start && m_start < end) && m_end < end) {
// first part
    md.num_pages = (m_start - md.phys_addr) >>
    EFI_PAGE_SHIFT;
// middle part
    new += old_memmap.desc_size;
    memcpy(new, old, old_memmap.desc_size);
    md = new;
    md.attribute |= m_attr;
    md.phys_addr = m_start;
    md.num_pages = (m_end - m_start + 1) >>
    EFI_PAGE_SHIFT;
// last part
    new += old_memmap.desc_size;
    memcpy(new, old, old_memmap.desc_size);
    md = new;
    md.phys_addr = m_end + 1;
    md.num_pages = (end - m_end) >>
    EFI_PAGE_SHIFT;
    }
    if ((start < m_start && m_start < end) &&
    (end <= m_end)) {
// first part
    md.num_pages = (m_start - md.phys_addr) >>
    EFI_PAGE_SHIFT;
// latter part
    new += old_memmap.desc_size;
    memcpy(new, old, old_memmap.desc_size);
    md = new;
    md.phys_addr = m_start;
    md.num_pages = (end - md.phys_addr + 1) >>
    EFI_PAGE_SHIFT;
    md.attribute |= m_attr;
    }
    }
    }
