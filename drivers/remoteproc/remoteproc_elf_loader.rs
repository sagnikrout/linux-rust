//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/remoteproc_elf_loader.c
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
// Remote Processor Framework ELF loader
//
// Copyright (C) 2011 Texas Instruments, Inc.
// Copyright (C) 2011 Google, Inc.
//
// Ohad Ben-Cohen <ohad@wizery.com>
// Brian Swetland <swetland@google.com>
// Mark Grosen <mgrosen@ti.com>
// Fernando Guzman Lugo <fernando.lugo@ti.com>
// Suman Anna <s-anna@ti.com>
// Robert Tivy <rtivy@ti.com>
// Armando Uribe De Leon <x0095078@ti.com>
// Sjur Brændeland <sjur.brandeland@stericsson.com>
//

//
// rproc_elf_sanity_check() - Sanity Check for ELF32/ELF64 firmware image
// @rproc: the remote processor handle
// @fw: the ELF firmware image
//
// Make sure this fw image is sane (ie a correct ELF32/ELF64 file).
//
// Return: 0 on success and -EINVAL upon any failure
//
#[no_mangle]
pub unsafe extern "C" fn rproc_elf_sanity_check(rproc: *mut rproc, fw: *const firmware) -> c_int {
    int rproc_elf_sanity_check(struct rproc *rproc, const struct firmware *fw)
    {
    const char *name = rproc.firmware;
    struct device *dev = &rproc.dev;
//
// ELF files are beginning with the same structure. Thus, to simplify
// header parsing, we can use the elf32_hdr one for both elf64 and
// elf32.
//
    struct elf32_hdr *ehdr;
    u32 elf_shdr_get_size;
    u64 phoff, shoff;
    char class;
    u16 phnum;
    if (!fw) {
    dev_err(dev, "failed to load %s\n", name);
    return -EINVAL;
    }
    if (fw.size < sizeof(struct elf32_hdr)) {
    dev_err(dev, "Image is too small\n");
    return -EINVAL;
    }
    ehdr = (struct elf32_hdr *)fw.data;
    if (memcmp(ehdr.e_ident, ELFMAG, SELFMAG)) {
    dev_err(dev, "Image is corrupted (bad magic)\n");
    return -EINVAL;
    }
    class = ehdr.e_ident[EI_CLASS];
    if (class != ELFCLASS32 && class != ELFCLASS64) {
    dev_err(dev, "Unsupported class: %d\n", class);
    return -EINVAL;
    }
    if (class == ELFCLASS64 && fw.size < sizeof(struct elf64_hdr)) {
    dev_err(dev, "elf64 header is too small\n");
    return -EINVAL;
    }
// We assume the firmware has the same endianness as the host

    if (ehdr.e_ident[EI_DATA] != ELFDATA2LSB) {

    if (ehdr.e_ident[EI_DATA] != ELFDATA2MSB) {

    dev_err(dev, "Unsupported firmware endianness\n");
    return -EINVAL;
    }
    phoff = elf_hdr_get_e_phoff(class, fw.data);
    shoff = elf_hdr_get_e_shoff(class, fw.data);
    phnum =  elf_hdr_get_e_phnum(class, fw.data);
    elf_shdr_get_size = elf_size_of_shdr(class);
    if (fw.size < shoff + elf_shdr_get_size) {
    dev_err(dev, "Image is too small\n");
    return -EINVAL;
    }
    if (phnum == 0) {
    dev_err(dev, "No loadable segments\n");
    return -EINVAL;
    }
    if (phoff > fw.size) {
    dev_err(dev, "Firmware size is too small\n");
    return -EINVAL;
    }
    dev_dbg(dev, "Firmware is an elf%d file\n",
    class == ELFCLASS32 ? 32 : 64);
    return 0;
    }
    EXPORT_SYMBOL(rproc_elf_sanity_check);
//
// rproc_elf_get_boot_addr() - Get rproc's boot address.
// @rproc: the remote processor handle
// @fw: the ELF firmware image
//
// Note that the boot address is not a configurable property of all remote
// processors. Some will always boot at a specific hard-coded address.
//
// Return: entry point address of the ELF image
//
#[no_mangle]
pub unsafe extern "C" fn rproc_elf_get_boot_addr(rproc: *mut rproc, fw: *const firmware) -> u64 {
    u64 rproc_elf_get_boot_addr(struct rproc *rproc, const struct firmware *fw)
    {
    return elf_hdr_get_e_entry(fw_elf_get_class(fw), fw.data);
    }
    EXPORT_SYMBOL(rproc_elf_get_boot_addr);
//
// rproc_elf_load_segments() - load firmware segments to memory
// @rproc: remote processor which will be booted using these fw segments
// @fw: the ELF firmware image
//
// This function loads the firmware segments to memory, where the remote
// processor expects them.
//
// Some remote processors will expect their code and data to be placed
// in specific device addresses, and can't have them dynamically assigned.
//
// We currently support only those kind of remote processors, and expect
// the program header's paddr member to contain those addresses. We then go
// through the physically contiguous "carveout" memory regions which we
// allocated (and mapped) earlier on behalf of the remote processor,
// and "translate" device address to kernel addresses, so we can copy the
// segments where they are expected.
//
// Currently we only support remote processors that required carveout
// allocations and got them mapped onto their iommus. Some processors
// might be different: they might not have iommus, and would prefer to
// directly allocate memory for every segment/resource. This is not yet
// supported, though.
//
// Return: 0 on success and an appropriate error code otherwise
//
#[no_mangle]
pub unsafe extern "C" fn rproc_elf_load_segments(rproc: *mut rproc, fw: *const firmware) -> c_int {
    int rproc_elf_load_segments(struct rproc *rproc, const struct firmware *fw)
    {
    struct device *dev = &rproc.dev;
    const void *ehdr, *phdr;
    int i, ret = 0;
    u16 phnum;
    const u8 *elf_data = fw.data;
    let mut class: u8 = fw_elf_get_class(fw);
    let mut elf_phdr_get_size: u32 = elf_size_of_phdr(class);
    ehdr = elf_data;
    phnum = elf_hdr_get_e_phnum(class, ehdr);
    phdr = elf_data + elf_hdr_get_e_phoff(class, ehdr);
// go through the available ELF segments
    for (i = 0; i < phnum; i++, phdr += elf_phdr_get_size) {
    let mut da: u64 = elf_phdr_get_p_paddr(class, phdr);
    let mut memsz: u64 = elf_phdr_get_p_memsz(class, phdr);
    let mut filesz: u64 = elf_phdr_get_p_filesz(class, phdr);
    let mut offset: u64 = elf_phdr_get_p_offset(class, phdr);
    let mut type: u32 = elf_phdr_get_p_type(class, phdr);
    let mut is_iomem: bool = false;
    void *ptr;
    if (type != PT_LOAD || !memsz)
    continue;
    dev_dbg(dev, "phdr: type %d da 0x%llx memsz 0x%llx filesz 0x%llx\n",
    type, da, memsz, filesz);
    if (filesz > memsz) {
    dev_err(dev, "bad phdr filesz 0x%llx memsz 0x%llx\n",
    filesz, memsz);
    ret = -EINVAL;
    break;
    }
    if (offset + filesz > fw.size) {
    dev_err(dev, "truncated fw: need 0x%llx avail 0x%zx\n",
    offset + filesz, fw.size);
    ret = -EINVAL;
    break;
    }
    if (!rproc_u64_fit_in_size_t(memsz)) {
    dev_err(dev, "size (%llx) does not fit in size_t type\n",
    memsz);
    ret = -EOVERFLOW;
    break;
    }
// grab the kernel address for this device address
    ptr = rproc_da_to_va(rproc, da, memsz, &is_iomem);
    if (!ptr) {
    dev_err(dev, "bad phdr da 0x%llx mem 0x%llx\n", da,
    memsz);
    ret = -EINVAL;
    break;
    }
// put the segment where the remote processor expects it
    if (filesz) {
    if (is_iomem)
    memcpy_toio((void __iomem *)ptr, elf_data + offset, filesz);
    else
    memcpy(ptr, elf_data + offset, filesz);
    }
//
// Zero out remaining memory for this segment.
//
// This isn't strictly required since dma_alloc_coherent already
// did this for us. albeit harmless, we may consider removing
// this.
//
    if (memsz > filesz) {
    if (is_iomem)
    memset_io((void __iomem *)(ptr + filesz), 0, memsz - filesz);
    else
    memset(ptr + filesz, 0, memsz - filesz);
    }
    }
    return ret;
    }
    EXPORT_SYMBOL(rproc_elf_load_segments);
    static const void *
    find_table(struct device *dev, const struct firmware *fw)
    {
    const void *shdr, *name_table_shdr;
    int i;
    const char *name_table;
    struct resource_table *table = core::ptr::null_mut();
    const u8 *elf_data = (void *)fw.data;
    let mut class: u8 = fw_elf_get_class(fw);
    let mut fw_size: usize = fw.size;
    const void *ehdr = elf_data;
    let mut shnum: u16 = elf_hdr_get_e_shnum(class, ehdr);
    let mut elf_shdr_get_size: u32 = elf_size_of_shdr(class);
    let mut shstrndx: u16 = elf_hdr_get_e_shstrndx(class, ehdr);
// look for the resource table and handle it
// First, get the section header according to the elf class
    shdr = elf_data + elf_hdr_get_e_shoff(class, ehdr);
// Compute name table section header entry in shdr array
    name_table_shdr = shdr + (shstrndx * elf_shdr_get_size);
// Finally, compute the name table section address in elf
    name_table = elf_data + elf_shdr_get_sh_offset(class, name_table_shdr);
    for (i = 0; i < shnum; i++, shdr += elf_shdr_get_size) {
    let mut size: u64 = elf_shdr_get_sh_size(class, shdr);
    let mut offset: u64 = elf_shdr_get_sh_offset(class, shdr);
    let mut name: u32 = elf_shdr_get_sh_name(class, shdr);
    if (strcmp(name_table + name, ".resource_table"))
    continue;
    table = (struct resource_table *)(elf_data + offset);
// make sure we have the entire table
    if (offset + size > fw_size || offset + size < size) {
    dev_err(dev, "resource table truncated\n");
    return core::ptr::null_mut();
    }
// make sure table has at least the header
    if (sizeof(struct resource_table) > size) {
    dev_err(dev, "header-less resource table\n");
    return core::ptr::null_mut();
    }
// we don't support any version beyond the first
    if (table.ver != 1) {
    dev_err(dev, "unsupported fw ver: %d\n", table.ver);
    return core::ptr::null_mut();
    }
// make sure reserved bytes are zeroes
    if (table.reserved[0] || table.reserved[1]) {
    dev_err(dev, "non zero reserved bytes\n");
    return core::ptr::null_mut();
    }
// make sure the offsets array isn't truncated
    if (struct_size(table, offset, table.num) > size) {
    dev_err(dev, "resource table incomplete\n");
    return core::ptr::null_mut();
    }
    return shdr;
    }
    return core::ptr::null_mut();
    }
//
// rproc_elf_load_rsc_table() - load the resource table
// @rproc: the rproc handle
// @fw: the ELF firmware image
//
// This function finds the resource table inside the remote processor's
// firmware, load it into the @cached_table and update @table_ptr.
//
// Return: 0 on success, negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn rproc_elf_load_rsc_table(rproc: *mut rproc, fw: *const firmware) -> c_int {
    int rproc_elf_load_rsc_table(struct rproc *rproc, const struct firmware *fw)
    {
    const void *shdr;
    struct device *dev = &rproc.dev;
    struct resource_table *table = core::ptr::null_mut();
    const u8 *elf_data = fw.data;
    size_t tablesz;
    let mut class: u8 = fw_elf_get_class(fw);
    u64 sh_offset;
    shdr = find_table(dev, fw);
    if (!shdr)
    return -EINVAL;
    sh_offset = elf_shdr_get_sh_offset(class, shdr);
    table = (struct resource_table *)(elf_data + sh_offset);
    tablesz = elf_shdr_get_sh_size(class, shdr);
//
// Create a copy of the resource table. When a virtio device starts
// and calls vring_new_virtqueue() the address of the allocated vring
// will be stored in the cached_table. Before the device is started,
// cached_table will be copied into device memory.
//
    rproc.cached_table = kmemdup(table, tablesz, GFP_KERNEL);
    if (!rproc.cached_table)
    return -ENOMEM;
    rproc.table_ptr = rproc.cached_table;
    rproc.table_sz = tablesz;
    return 0;
    }
    EXPORT_SYMBOL(rproc_elf_load_rsc_table);
//
// rproc_elf_find_loaded_rsc_table() - find the loaded resource table
// @rproc: the rproc handle
// @fw: the ELF firmware image
//
// This function finds the location of the loaded resource table. Don't
// call this function if the table wasn't loaded yet - it's a bug if you do.
//
// Return: pointer to the resource table if it is found or NULL otherwise.
// If the table wasn't loaded yet the result is unspecified.
//
    struct resource_table *rproc_elf_find_loaded_rsc_table(struct rproc *rproc,
    const struct firmware *fw)
    {
    const void *shdr;
    u64 sh_addr, sh_size;
    let mut class: u8 = fw_elf_get_class(fw);
    struct device *dev = &rproc.dev;
    shdr = find_table(&rproc.dev, fw);
    if (!shdr)
    return core::ptr::null_mut();
    sh_addr = elf_shdr_get_sh_addr(class, shdr);
    sh_size = elf_shdr_get_sh_size(class, shdr);
    if (!rproc_u64_fit_in_size_t(sh_size)) {
    dev_err(dev, "size (%llx) does not fit in size_t type\n",
    sh_size);
    return core::ptr::null_mut();
    }
    return rproc_da_to_va(rproc, sh_addr, sh_size, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(rproc_elf_find_loaded_rsc_table);
