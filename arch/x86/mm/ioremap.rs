//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/ioremap.c
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
// Re-map IO memory to kernel address space so that we can access it.
// This is needed for high PCI addresses that aren't mapped in the
// 640k-1MB IO memory area on PC's
//
// (C) Copyright 1995 1996 Linus Torvalds
//

//
// Descriptor controlling ioremap() behavior.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioremap_desc {
    pub flags: c_uint,
}

//
// Fix up the linear direct mapping of the kernel to avoid cache attribute
// conflicts.
//
    int ioremap_change_attr(unsigned long vaddr, unsigned long size,
    enum page_cache_mode pcm)
    {
    let mut nrpages: c_ulong = size >> PAGE_SHIFT;
    int err;
    switch (pcm) {
    case _PAGE_CACHE_MODE_UC:
    default:
    err = _set_memory_uc(vaddr, nrpages);
    break;
    case _PAGE_CACHE_MODE_WC:
    err = _set_memory_wc(vaddr, nrpages);
    break;
    case _PAGE_CACHE_MODE_WT:
    err = _set_memory_wt(vaddr, nrpages);
    break;
    case _PAGE_CACHE_MODE_WB:
    err = _set_memory_wb(vaddr, nrpages);
    break;
    }
    return err;
    }
// Does the range (or a subset of) contain normal RAM?
#[no_mangle]
unsafe extern "C" fn __ioremap_check_ram(res: *mut resource) -> c_uint {
    static unsigned int __ioremap_check_ram(struct resource *res)
    {
    unsigned long start_pfn, stop_pfn;
    unsigned long pfn;
    if ((res.flags & IORESOURCE_SYSTEM_RAM) != IORESOURCE_SYSTEM_RAM)
    return 0;
    start_pfn = (res.start + PAGE_SIZE - 1) >> PAGE_SHIFT;
    stop_pfn = (res.end + 1) >> PAGE_SHIFT;
    if (stop_pfn > start_pfn) {
    for_each_valid_pfn(pfn, start_pfn, stop_pfn)
    if (!PageReserved(pfn_to_page(pfn)))
    return IORES_MAP_SYSTEM_RAM;
    }
    return 0;
    }
//
// In a SEV guest, NONE and RESERVED should not be mapped encrypted because
// there the whole memory is already encrypted.
//
#[no_mangle]
unsafe extern "C" fn __ioremap_check_encrypted(res: *mut resource) -> c_uint {
    static unsigned int __ioremap_check_encrypted(struct resource *res)
    {
    if (!cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT))
    return 0;
    switch (res.desc) {
    case IORES_DESC_NONE:
    case IORES_DESC_RESERVED:
    break;
    default:
    return IORES_MAP_ENCRYPTED;
    }
    return 0;
    }
//
// The EFI runtime services data area is not covered by walk_mem_res(), but must
// be mapped encrypted when SEV is active.
//
#[no_mangle]
unsafe extern "C" fn __ioremap_check_other(addr: resource_size_t, desc: *mut ioremap_desc) {
    static void __ioremap_check_other(resource_size_t addr, struct ioremap_desc *desc)
    {
    if (!cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT))
    return;
    if (x86_platform.hyper.is_private_mmio(addr)) {
    desc.flags |= IORES_MAP_ENCRYPTED;
    return;
    }
    if (!IS_ENABLED(CONFIG_EFI))
    return;
    if (efi_mem_type(addr) == EFI_RUNTIME_SERVICES_DATA ||
    (efi_mem_type(addr) == EFI_BOOT_SERVICES_DATA &&
    efi_mem_attributes(addr) & EFI_MEMORY_RUNTIME))
    desc.flags |= IORES_MAP_ENCRYPTED;
    }
#[no_mangle]
unsafe extern "C" fn __ioremap_collect_map_flags(res: *mut resource, arg: *mut c_void) -> c_int {
    static int __ioremap_collect_map_flags(struct resource *res, void *arg)
    {
    struct ioremap_desc *desc = arg;
    if (!(desc.flags & IORES_MAP_SYSTEM_RAM))
    desc.flags |= __ioremap_check_ram(res);
    if (!(desc.flags & IORES_MAP_ENCRYPTED))
    desc.flags |= __ioremap_check_encrypted(res);
    return ((desc.flags & (IORES_MAP_SYSTEM_RAM | IORES_MAP_ENCRYPTED)) ==
    (IORES_MAP_SYSTEM_RAM | IORES_MAP_ENCRYPTED));
    }
//
// To avoid multiple resource walks, this function walks resources marked as
// IORESOURCE_MEM and IORESOURCE_BUSY and looking for system RAM and/or a
// resource described not as IORES_DESC_NONE (e.g. IORES_DESC_ACPI_TABLES).
//
// After that, deal with misc other ranges in __ioremap_check_other() which do
// not fall into the above category.
//
    static void __ioremap_check_mem(resource_size_t addr, unsigned long size,
    struct ioremap_desc *desc)
    {
    u64 start, end;
    start = (u64)addr;
    end = start + size - 1;
    memset(desc, 0, sizeof(struct ioremap_desc));
    walk_mem_res(start, end, desc, __ioremap_collect_map_flags);
    __ioremap_check_other(addr, desc);
    }
//
// Remap an arbitrary physical address space into the kernel virtual
// address space. It transparently creates kernel huge I/O mapping when
// the physical address is aligned by a huge page size (1GB or 2MB) and
// the requested size is at least the huge page size.
//
// NOTE: MTRRs can override PAT memory types with a 4KB granularity.
// Therefore, the mapping code falls back to use a smaller page toward 4KB
// when a mapping range is covered by non-WB type of MTRRs.
//
// NOTE! We need to allow non-page-aligned mappings too: we will obviously
// have to convert them into an offset in a page-aligned mapping, but the
// caller shouldn't need to know that small detail.
//
    static void __iomem *
    __ioremap_caller(resource_size_t phys_addr, unsigned long size,
    enum page_cache_mode pcm, void *caller, bool encrypted)
    {
    unsigned long offset, vaddr;
    resource_size_t last_addr;
    let mut unaligned_phys_addr: resource_size_t = phys_addr;
    let mut unaligned_size: c_ulong = size;
    struct ioremap_desc io_desc;
    struct vm_struct *area;
    enum page_cache_mode new_pcm;
    pgprot_t prot;
    int retval;
    void __iomem *ret_addr;
// Don't allow wraparound or zero size
    last_addr = phys_addr + size - 1;
    if (!size || last_addr < phys_addr)
    return core::ptr::null_mut();
    if (!phys_addr_valid(phys_addr)) {
    printk(KERN_WARNING "ioremap: invalid physical address %llx\n",
    (unsigned long long)phys_addr);
    WARN_ON_ONCE(1);
    return core::ptr::null_mut();
    }
    __ioremap_check_mem(phys_addr, size, &io_desc);
//
// Don't allow anybody to remap normal RAM that we're using..
//
    if (io_desc.flags & IORES_MAP_SYSTEM_RAM) {
    WARN_ONCE(1, "ioremap on RAM at %pa - %pa\n",
    &phys_addr, &last_addr);
    return core::ptr::null_mut();
    }
//
// Mappings have to be page-aligned
//
    offset = phys_addr & ~PAGE_MASK;
    phys_addr &= PAGE_MASK;
    size = PAGE_ALIGN(last_addr+1) - phys_addr;
//
// Mask out any bits not part of the actual physical
// address, like memory encryption bits.
//
    phys_addr &= PHYSICAL_PAGE_MASK;
    retval = memtype_reserve(phys_addr, (u64)phys_addr + size,
    pcm, &new_pcm);
    if (retval) {
    printk(KERN_ERR "ioremap memtype_reserve failed %d\n", retval);
    return core::ptr::null_mut();
    }
    if (pcm != new_pcm) {
    if (!is_new_memtype_allowed(phys_addr, size, pcm, new_pcm)) {
    printk(KERN_ERR
    "ioremap error for 0x%llx-0x%llx, requested 0x%x, got 0x%x\n",
    (unsigned long long)phys_addr,
    (unsigned long long)(phys_addr + size),
    pcm, new_pcm);
    goto err_free_memtype;
    }
    pcm = new_pcm;
    }
//
// If the page being mapped is in memory and SEV is active then
// make sure the memory encryption attribute is enabled in the
// resulting mapping.
// In TDX guests, memory is marked private by default. If encryption
// is not requested (using encrypted), explicitly set decrypt
// attribute in all IOREMAPPED memory.
//
    prot = PAGE_KERNEL_IO;
    if ((io_desc.flags & IORES_MAP_ENCRYPTED) || encrypted)
    prot = pgprot_encrypted(prot);
    else
    prot = pgprot_decrypted(prot);
    switch (pcm) {
    case _PAGE_CACHE_MODE_UC:
    default:
    prot = __pgprot(pgprot_val(prot) |
    cachemode2protval(_PAGE_CACHE_MODE_UC));
    break;
    case _PAGE_CACHE_MODE_UC_MINUS:
    prot = __pgprot(pgprot_val(prot) |
    cachemode2protval(_PAGE_CACHE_MODE_UC_MINUS));
    break;
    case _PAGE_CACHE_MODE_WC:
    prot = __pgprot(pgprot_val(prot) |
    cachemode2protval(_PAGE_CACHE_MODE_WC));
    break;
    case _PAGE_CACHE_MODE_WT:
    prot = __pgprot(pgprot_val(prot) |
    cachemode2protval(_PAGE_CACHE_MODE_WT));
    break;
    case _PAGE_CACHE_MODE_WB:
    break;
    }
//
// Ok, go for it..
//
    area = get_vm_area_caller(size, VM_IOREMAP, caller);
    if (!area)
    goto err_free_memtype;
    area.phys_addr = phys_addr;
    vaddr = (unsigned long) area.addr;
    if (memtype_kernel_map_sync(phys_addr, size, pcm))
    goto err_free_area;
    if (ioremap_page_range(vaddr, vaddr + size, phys_addr, prot))
    goto err_free_area;
    ret_addr = (void __iomem *) (vaddr + offset);
    mmiotrace_ioremap(unaligned_phys_addr, unaligned_size, ret_addr);
//
// Check if the request spans more than any BAR in the iomem resource
// tree.
//
    if (iomem_map_sanity_check(unaligned_phys_addr, unaligned_size))
    pr_warn("caller %pS mapping multiple BARs\n", caller);
    return ret_addr;
    err_free_area:
    free_vm_area(area);
    err_free_memtype:
    memtype_free(phys_addr, phys_addr + size);
    return core::ptr::null_mut();
    }
//
// ioremap     -   map bus memory into CPU space
// @phys_addr:    bus address of the memory
// @size:      size of the resource to map
//
// ioremap performs a platform specific sequence of operations to
// make bus memory CPU accessible via the readb/readw/readl/writeb
// writew/writel functions and the other mmio helpers. The returned
// address is not guaranteed to be usable directly as a virtual
// address.
//
// This version of ioremap ensures that the memory is marked uncachable
// on the CPU as well as honouring existing caching rules from things like
// the PCI bus. Note that there are other caches and buffers on many
// busses. In particular driver authors should read up on PCI writes
//
// It's useful if some control registers are in such an area and
// write combining or read caching is not desirable:
//
// Must be freed with iounmap.
//
    void __iomem *ioremap(resource_size_t phys_addr, unsigned long size)
    {
//
// Ideally, this should be:
// pat_enabled() ? _PAGE_CACHE_MODE_UC : _PAGE_CACHE_MODE_UC_MINUS;
//
// Till we fix all X drivers to use ioremap_wc(), we will use
// UC MINUS. Drivers that are certain they need or can already
// be converted over to strong UC can use ioremap_uc().
//
    let mut pcm: enum page_cache_mode = _PAGE_CACHE_MODE_UC_MINUS;
    return __ioremap_caller(phys_addr, size, pcm,
    __builtin_return_address(0), false);
    }
    EXPORT_SYMBOL(ioremap);
//
// ioremap_uc     -   map bus memory into CPU space as strongly uncachable
// @phys_addr:    bus address of the memory
// @size:      size of the resource to map
//
// ioremap_uc performs a platform specific sequence of operations to
// make bus memory CPU accessible via the readb/readw/readl/writeb
// writew/writel functions and the other mmio helpers. The returned
// address is not guaranteed to be usable directly as a virtual
// address.
//
// This version of ioremap ensures that the memory is marked with a strong
// preference as completely uncachable on the CPU when possible. For non-PAT
// systems this ends up setting page-attribute flags PCD=1, PWT=1. For PAT
// systems this will set the PAT entry for the pages as strong UC.  This call
// will honor existing caching rules from things like the PCI bus. Note that
// there are other caches and buffers on many busses. In particular driver
// authors should read up on PCI writes.
//
// It's useful if some control registers are in such an area and
// write combining or read caching is not desirable:
//
// Must be freed with iounmap.
//
    void __iomem *ioremap_uc(resource_size_t phys_addr, unsigned long size)
    {
    let mut pcm: enum page_cache_mode = _PAGE_CACHE_MODE_UC;
    return __ioremap_caller(phys_addr, size, pcm,
    __builtin_return_address(0), false);
    }
    EXPORT_SYMBOL_GPL(ioremap_uc);
//
// ioremap_wc	-	map memory into CPU space write combined
// @phys_addr:	bus address of the memory
// @size:	size of the resource to map
//
// This version of ioremap ensures that the memory is marked write combining.
// Write combining allows faster writes to some hardware devices.
//
// Must be freed with iounmap.
//
    void __iomem *ioremap_wc(resource_size_t phys_addr, unsigned long size)
    {
    return __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WC,
    __builtin_return_address(0), false);
    }
    EXPORT_SYMBOL(ioremap_wc);
//
// ioremap_wt	-	map memory into CPU space write through
// @phys_addr:	bus address of the memory
// @size:	size of the resource to map
//
// This version of ioremap ensures that the memory is marked write through.
// Write through stores data into memory while keeping the cache up-to-date.
//
// Must be freed with iounmap.
//
    void __iomem *ioremap_wt(resource_size_t phys_addr, unsigned long size)
    {
    return __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WT,
    __builtin_return_address(0), false);
    }
    EXPORT_SYMBOL(ioremap_wt);
    void __iomem *ioremap_encrypted(resource_size_t phys_addr, unsigned long size)
    {
    return __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WB,
    __builtin_return_address(0), true);
    }
    EXPORT_SYMBOL(ioremap_encrypted);
    void __iomem *ioremap_cache(resource_size_t phys_addr, unsigned long size)
    {
    return __ioremap_caller(phys_addr, size, _PAGE_CACHE_MODE_WB,
    __builtin_return_address(0), false);
    }
    EXPORT_SYMBOL(ioremap_cache);
    void __iomem *ioremap_prot(resource_size_t phys_addr, unsigned long size,
    pgprot_t prot)
    {
    return __ioremap_caller(phys_addr, size,
    pgprot2cachemode(prot),
    __builtin_return_address(0), false);
    }
    EXPORT_SYMBOL(ioremap_prot);
//
// iounmap - Free a IO remapping
// @addr: virtual address from ioremap_
//
// Caller must ensure there is only one unmapping for the same pointer.
//
#[no_mangle]
pub unsafe extern "C" fn iounmap(addr: *mut volatile void __iomem) {
    void iounmap(volatile void __iomem *addr)
    {
    struct vm_struct *p, *o;
    if (WARN_ON_ONCE(!is_ioremap_addr((void  *)addr)))
    return;
//
// The PCI/ISA range special-casing was removed from __ioremap()
// so this check, in theory, can be removed. However, there are
// cases where iounmap() is called for addresses not obtained via
// ioremap() (vga16fb for example). Add a warning so that these
// cases can be caught and fixed.
//
    if ((void  *)addr >= phys_to_virt(ISA_START_ADDRESS) &&
    (void  *)addr < phys_to_virt(ISA_END_ADDRESS)) {
    WARN(1, "iounmap() called for ISA range not obtained using ioremap()\n");
    return;
    }
    mmiotrace_iounmap(addr);
    addr = (volatile void __iomem *)
    (PAGE_MASK & (unsigned long )addr);
// Use the vm area unlocked, assuming the caller
    ensures there isn't another iounmap for the same address
    in parallel. Reuse of the virtual address is prevented by
    leaving it in the global lists until we're done with it.
    cpa takes care of the direct mappings. */
    p = find_vm_area((void  *)addr);
    if (!p) {
    printk(KERN_ERR "iounmap: bad address %p\n", addr);
    dump_stack();
    return;
    }
    kmsan_iounmap_page_range((unsigned long)addr,
    (unsigned long)addr + get_vm_area_size(p));
    memtype_free(p.phys_addr, p.phys_addr + get_vm_area_size(p));
// Finally remove it
    o = remove_vm_area((void  *)addr);
    BUG_ON(p != o || o == core::ptr::null_mut());
    kfree(p);
    }
    EXPORT_SYMBOL(iounmap);
    void *arch_memremap_wb(phys_addr_t phys_addr, size_t size, unsigned long flags)
    {
    if ((flags & MEMREMAP_DEC) || cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT))
    return (void  *)ioremap_cache(phys_addr, size);
    return (void  *)ioremap_encrypted(phys_addr, size);
    }
//
// Convert a physical pointer to a virtual kernel pointer for /dev/mem
// access
//
    void *xlate_dev_mem_ptr(phys_addr_t phys)
    {
    let mut start: c_ulong = phys &  PAGE_MASK;
    let mut offset: c_ulong = phys & ~PAGE_MASK;
    void *vaddr;
// memremap() maps if RAM, otherwise falls back to ioremap()
    vaddr = memremap(start, PAGE_SIZE, MEMREMAP_WB);
// Only add the offset on success and return NULL if memremap() failed
    if (vaddr)
    vaddr += offset;
    return vaddr;
    }
#[no_mangle]
pub unsafe extern "C" fn unxlate_dev_mem_ptr(phys: phys_addr_t, addr: *mut c_void) {
    void unxlate_dev_mem_ptr(phys_addr_t phys, void *addr)
    {
    memunmap((void *)((unsigned long)addr & PAGE_MASK));
    }

//
// Examine the physical address to determine if it is an area of memory
// that should be mapped decrypted.  If the memory is not part of the
// kernel usable area it was accessed and created decrypted, so these
// areas should be mapped decrypted. And since the encryption key can
// change across reboots, persistent memory should also be mapped
// decrypted.
//
// If SEV is active, that implies that BIOS/UEFI also ran encrypted so
// only persistent memory should be mapped decrypted.
//
    static bool memremap_should_map_decrypted(resource_size_t phys_addr,
    unsigned long size)
    {
    int is_pmem;
//
// Check if the address is part of a persistent memory region.
// This check covers areas added by E820, EFI and ACPI.
//
    is_pmem = region_intersects(phys_addr, size, IORESOURCE_MEM,
    IORES_DESC_PERSISTENT_MEMORY);
    if (is_pmem != REGION_DISJOINT)
    return true;
//
// Check if the non-volatile attribute is set for an EFI
// reserved area.
//
    if (efi_enabled(EFI_BOOT)) {
    switch (efi_mem_type(phys_addr)) {
    case EFI_RESERVED_TYPE:
    if (efi_mem_attributes(phys_addr) & EFI_MEMORY_NV)
    return true;
    break;
    default:
    break;
    }
    }
// Check if the address is outside kernel usable area
    switch (e820__get_entry_type(phys_addr, phys_addr + size - 1)) {
    case E820_TYPE_RESERVED:
    case E820_TYPE_ACPI:
    case E820_TYPE_NVS:
    case E820_TYPE_UNUSABLE:
// For SEV, these areas are encrypted
    if (cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT))
    break;
    fallthrough;
    case E820_TYPE_PRAM:
    return true;
    default:
    break;
    }
    return false;
    }
//
// Examine the physical address to determine if it is EFI data. Check
// it against the boot params structure and EFI tables and memory types.
//
#[no_mangle]
unsafe extern "C" fn memremap_is_efi_data(phys_addr: resource_size_t) -> bool {
    static bool memremap_is_efi_data(resource_size_t phys_addr)
    {
    u64 paddr;
// Check if the address is part of EFI boot/runtime data
    if (!efi_enabled(EFI_BOOT))
    return false;
    paddr = boot_params.efi_info.efi_memmap_hi;
    paddr <<= 32;
    paddr |= boot_params.efi_info.efi_memmap;
    if (phys_addr == paddr)
    return true;
    paddr = boot_params.efi_info.efi_systab_hi;
    paddr <<= 32;
    paddr |= boot_params.efi_info.efi_systab;
    if (phys_addr == paddr)
    return true;
    if (efi_is_table_address(phys_addr))
    return true;
    switch (efi_mem_type(phys_addr)) {
    case EFI_BOOT_SERVICES_DATA:
    case EFI_RUNTIME_SERVICES_DATA:
    return true;
    default:
    break;
    }
    return false;
    }
//
// Examine the physical address to determine if it is boot data by checking
// it against the boot params setup_data chain.
//
#[no_mangle]
unsafe extern "C" fn __memremap_is_setup_data(phys_addr: resource_size_t, early: bool) -> bool __ref {
    static bool __ref __memremap_is_setup_data(resource_size_t phys_addr, bool early)
    {
    let mut setup_data_sz: c_uint = sizeof(struct setup_data);
    struct setup_indirect *indirect;
    struct setup_data *data;
    u64 paddr, paddr_next;
    paddr = boot_params.hdr.setup_data;
    while (paddr) {
    unsigned int len, size;
    if (phys_addr == paddr)
    return true;
    if (early)
    data = early_memremap_decrypted(paddr, setup_data_sz);
    else
    data = memremap(paddr, setup_data_sz, MEMREMAP_WB | MEMREMAP_DEC);
    if (!data) {
    pr_warn("failed to remap setup_data entry\n");
    return false;
    }
    size = setup_data_sz;
    paddr_next = data.next;
    len = data.len;
    if ((phys_addr > paddr) &&
    (phys_addr < (paddr + setup_data_sz + len))) {
    if (early)
    early_memunmap(data, setup_data_sz);
    else
    memunmap(data);
    return true;
    }
    if (data.type == SETUP_INDIRECT) {
    size += len;
    if (early) {
    early_memunmap(data, setup_data_sz);
    data = early_memremap_decrypted(paddr, size);
    } else {
    memunmap(data);
    data = memremap(paddr, size, MEMREMAP_WB | MEMREMAP_DEC);
    }
    if (!data) {
    pr_warn("failed to remap indirect setup_data\n");
    return false;
    }
    indirect = (struct setup_indirect *)data.data;
    if (indirect.type != SETUP_INDIRECT) {
    paddr = indirect.addr;
    len = indirect.len;
    }
    }
    if (early)
    early_memunmap(data, size);
    else
    memunmap(data);
    if ((phys_addr > paddr) && (phys_addr < (paddr + len)))
    return true;
    paddr = paddr_next;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn memremap_is_setup_data(phys_addr: resource_size_t) -> bool {
    static bool memremap_is_setup_data(resource_size_t phys_addr)
    {
    return __memremap_is_setup_data(phys_addr, false);
    }
#[no_mangle]
unsafe extern "C" fn early_memremap_is_setup_data(phys_addr: resource_size_t) -> bool __init {
    static bool __init early_memremap_is_setup_data(resource_size_t phys_addr)
    {
    return __memremap_is_setup_data(phys_addr, true);
    }
//
// Architecture function to determine if RAM remap is allowed. By default, a
// RAM remap will map the data as encrypted. Determine if a RAM remap should
// not be done so that the data will be mapped decrypted.
//
    bool arch_memremap_can_ram_remap(resource_size_t phys_addr, unsigned long size,
    unsigned long flags)
    {
    if (!cc_platform_has(CC_ATTR_MEM_ENCRYPT))
    return true;
    if (flags & MEMREMAP_ENC)
    return true;
    if (flags & MEMREMAP_DEC)
    return false;
    if (cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT)) {
    if (memremap_is_setup_data(phys_addr) ||
    memremap_is_efi_data(phys_addr))
    return false;
    }
    return !memremap_should_map_decrypted(phys_addr, size);
    }
//
// Architecture override of __weak function to adjust the protection attributes
// used when remapping memory. By default, early_memremap() will map the data
// as encrypted. Determine if an encrypted mapping should not be done and set
// the appropriate protection attributes.
//
    pgprot_t __init early_memremap_pgprot_adjust(resource_size_t phys_addr,
    unsigned long size,
    pgprot_t prot)
    {
    bool encrypted_prot;
    if (!cc_platform_has(CC_ATTR_MEM_ENCRYPT))
    return prot;
    encrypted_prot = true;
    if (cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT)) {
    if (early_memremap_is_setup_data(phys_addr) ||
    memremap_is_efi_data(phys_addr))
    encrypted_prot = false;
    }
    if (encrypted_prot && memremap_should_map_decrypted(phys_addr, size))
    encrypted_prot = false;
    return encrypted_prot ? pgprot_encrypted(prot)
    : pgprot_decrypted(prot);
    }
#[no_mangle]
pub unsafe extern "C" fn phys_mem_access_encrypted(phys_addr: c_ulong, size: c_ulong) -> bool {
    bool phys_mem_access_encrypted(unsigned long phys_addr, unsigned long size)
    {
    return arch_memremap_can_ram_remap(phys_addr, size, 0);
    }
// Remap memory with encryption
    void __init *early_memremap_encrypted(resource_size_t phys_addr,
    unsigned long size)
    {
    return early_memremap_prot(phys_addr, size, __PAGE_KERNEL_ENC);
    }
//
// Remap memory with encryption and write-protected - cannot be called
// before pat_init() is called
//
    void __init *early_memremap_encrypted_wp(resource_size_t phys_addr,
    unsigned long size)
    {
    if (!x86_has_pat_wp())
    return core::ptr::null_mut();
    return early_memremap_prot(phys_addr, size, __PAGE_KERNEL_ENC_WP);
    }
// Remap memory without encryption
    void __init *early_memremap_decrypted(resource_size_t phys_addr,
    unsigned long size)
    {
    return early_memremap_prot(phys_addr, size, __PAGE_KERNEL_NOENC);
    }
//
// Remap memory without encryption and write-protected - cannot be called
// before pat_init() is called
//
    void __init *early_memremap_decrypted_wp(resource_size_t phys_addr,
    unsigned long size)
    {
    if (!x86_has_pat_wp())
    return core::ptr::null_mut();
    return early_memremap_prot(phys_addr, size, __PAGE_KERNEL_NOENC_WP);
    }

    static pte_t bm_pte[PAGE_SIZE/sizeof(pte_t)] __page_aligned_bss;
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_pmd(addr: c_ulong) -> *mut pmd_t  __init {
    static inline pmd_t * __init early_ioremap_pmd(unsigned long addr)
    {
// Don't assume we're using swapper_pg_dir at this point
    pgd_t *base = __va(read_cr3_pa());
    pgd_t *pgd = &base[pgd_index(addr)];
    p4d_t *p4d = p4d_offset(pgd, addr);
    pud_t *pud = pud_offset(p4d, addr);
    pmd_t *pmd = pmd_offset(pud, addr);
    return pmd;
    }
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_pte(addr: c_ulong) -> *mut pte_t  __init {
    static inline pte_t * __init early_ioremap_pte(unsigned long addr)
    {
    return &bm_pte[pte_index(addr)];
    }
#[no_mangle]
pub unsafe extern "C" fn is_early_ioremap_ptep(ptep: *mut pte_t) -> bool __init {
    bool __init is_early_ioremap_ptep(pte_t *ptep)
    {
    return ptep >= &bm_pte[0] && ptep < &bm_pte[PAGE_SIZE/sizeof(pte_t)];
    }
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_init() -> void __init {
    void __init early_ioremap_init(void)
    {
    pmd_t *pmd;

    BUILD_BUG_ON((fix_to_virt(0) + PAGE_SIZE) & ((1 << PMD_SHIFT) - 1));

    WARN_ON((fix_to_virt(0) + PAGE_SIZE) & ((1 << PMD_SHIFT) - 1));

    early_ioremap_setup();
    pmd = early_ioremap_pmd(fix_to_virt(FIX_BTMAP_BEGIN));
    memset(bm_pte, 0, sizeof(bm_pte));
    pmd_populate_kernel(&init_mm, pmd, bm_pte);
//
// The boot-ioremap range spans multiple pmds, for which
// we are not prepared:
//

    BUILD_BUG_ON((__fix_to_virt(FIX_BTMAP_BEGIN) >> PMD_SHIFT)
    != (__fix_to_virt(FIX_BTMAP_END) >> PMD_SHIFT));

    if (pmd != early_ioremap_pmd(fix_to_virt(FIX_BTMAP_END))) {
    WARN_ON(1);
    printk(KERN_WARNING "pmd %p != %p\n",
    pmd, early_ioremap_pmd(fix_to_virt(FIX_BTMAP_END)));
    printk(KERN_WARNING "fix_to_virt(FIX_BTMAP_BEGIN): %08lx\n",
    fix_to_virt(FIX_BTMAP_BEGIN));
    printk(KERN_WARNING "fix_to_virt(FIX_BTMAP_END):   %08lx\n",
    fix_to_virt(FIX_BTMAP_END));
    printk(KERN_WARNING "FIX_BTMAP_END:       %d\n", FIX_BTMAP_END);
    printk(KERN_WARNING "FIX_BTMAP_BEGIN:     %d\n",
    FIX_BTMAP_BEGIN);
    }
    }
    void __init __early_set_fixmap(enum fixed_addresses idx,
    phys_addr_t phys, pgprot_t flags)
    {
    let mut addr: c_ulong = __fix_to_virt(idx);
    pte_t *pte;
    if (idx >= __end_of_fixed_addresses) {
    BUG();
    return;
    }
    pte = early_ioremap_pte(addr);
// Sanitize 'prot' against any unsupported bits:
    pgprot_val(flags) &= __supported_pte_mask;
    if (pgprot_val(flags))
    set_pte(pte, pfn_pte(phys >> PAGE_SHIFT, flags));
    else
    pte_clear(&init_mm, addr, pte);
    flush_tlb_one_kernel(addr);
    }
