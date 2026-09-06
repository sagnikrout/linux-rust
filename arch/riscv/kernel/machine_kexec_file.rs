//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/machine_kexec_file.c
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
// kexec_file for riscv, use vmlinux as the dump-capture kernel image.
//
// Copyright (C) 2021 Huawei Technologies Co, Ltd.
//
// Author: Liao Chang (liaochang1@huawei.com)
//

    const struct kexec_file_ops * const kexec_file_loaders[] = {
    &elf_kexec_ops,
    &image_kexec_ops,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int {
    int arch_kimage_file_post_load_cleanup(struct kimage *image)
    {
    kvfree(image.arch.fdt);
    image.arch.fdt = core::ptr::null_mut();
    vfree(image.elf_headers);
    image.elf_headers = core::ptr::null_mut();
    image.elf_headers_sz = 0;
    return kexec_image_post_load_cleanup_default(image);
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
    unsigned int nr_ranges = 2 + crashk_cma_cnt; /* For exclusion of crashkernel region */
    walk_system_ram_res(0, -1, &nr_ranges, get_nr_ram_ranges_callback);
    return nr_ranges;
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
    static char *setup_kdump_cmdline(struct kimage *image, char *cmdline,
    unsigned long cmdline_len)
    {
    int elfcorehdr_strlen;
    char *cmdline_ptr;
    cmdline_ptr = kzalloc(COMMAND_LINE_SIZE, GFP_KERNEL);
    if (!cmdline_ptr)
    return core::ptr::null_mut();
    elfcorehdr_strlen = sprintf(cmdline_ptr, "elfcorehdr=0x%lx ",
    image.elf_load_addr);
    if (elfcorehdr_strlen + cmdline_len > COMMAND_LINE_SIZE) {
    pr_err("Appending elfcorehdr=<addr> exceeds cmdline size\n");
    kfree(cmdline_ptr);
    return core::ptr::null_mut();
    }
    memcpy(cmdline_ptr + elfcorehdr_strlen, cmdline, cmdline_len);
// Ensure it's nul terminated
    cmdline_ptr[COMMAND_LINE_SIZE - 1] = '\0';
    return cmdline_ptr;
    }

pub const RISCV_IMM_BITS: c_int = 12;

    (((x) + (RISCV_IMM_REACH >> 1)) & ~(RISCV_IMM_REACH - 1))

    (RV_X(x, 0, 12) << 20)

    ((RV_X(x, 1, 4) << 8) | (RV_X(x, 5, 6) << 25) | \
    (RV_X(x, 11, 1) << 7) | (RV_X(x, 12, 1) << 31))

    (RV_X(x, 12, 20) << 12)

    ((RV_X(x, 1, 10) << 21) | (RV_X(x, 11, 1) << 20) | \
    (RV_X(x, 12, 8) << 12) | (RV_X(x, 20, 1) << 31))

    ((RV_X(x, 1, 2) << 3) | (RV_X(x, 3, 2) << 10) | (RV_X(x, 5, 1) << 2) | \
    (RV_X(x, 6, 2) << 5) | (RV_X(x, 8, 1) << 12))

    ((RV_X(x, 1, 3) << 3) | (RV_X(x, 4, 1) << 11) | (RV_X(x, 5, 1) << 2) | \
    (RV_X(x, 6, 1) << 7) | (RV_X(x, 7, 1) << 6) | (RV_X(x, 8, 2) << 9) | \
    (RV_X(x, 10, 1) << 8) | (RV_X(x, 11, 1) << 12))

    (ENCODE_UTYPE_IMM(RISCV_CONST_HIGH_PART(x)) | \
    (ENCODE_ITYPE_IMM(RISCV_CONST_LOW_PART(x)) << 32))

    (ENCODE_UTYPE_IMM(x) | (ENCODE_ITYPE_IMM(x) << 32))

    ((~ENCODE_##type##_IMM((uint64_t)(-1))) & (x))
    int arch_kexec_apply_relocations_add(struct purgatory_info *pi,
    Elf_Shdr *section,
    const Elf_Shdr *relsec,
    const Elf_Shdr *symtab)
    {
    const char *strtab, *name, *shstrtab;
    const Elf_Shdr *sechdrs;
    Elf64_Rela *relas;
    int i, r_type;
// String & section header string table
    sechdrs = (void *)pi.ehdr + pi.ehdr.e_shoff;
    strtab = (char *)pi.ehdr + sechdrs[symtab.sh_link].sh_offset;
    shstrtab = (char *)pi.ehdr + sechdrs[pi.ehdr.e_shstrndx].sh_offset;
    relas = (void *)pi.ehdr + relsec.sh_offset;
    for (i = 0; i < relsec.sh_size / sizeof(*relas); i++) {
    const Elf_Sym *sym;	/* symbol to relocate */
    unsigned long addr;	/* final location after relocation */
    unsigned long val;	/* relocated symbol value */
    unsigned long sec_base;	/* relocated symbol value */
    void *loc;		/* tmp location to modify */
    sym = (void *)pi.ehdr + symtab.sh_offset;
    sym += ELF64_R_SYM(relas[i].r_info);
    if (sym.st_name)
    name = strtab + sym.st_name;
    else
    name = shstrtab + sechdrs[sym.st_shndx].sh_name;
    loc = pi.purgatory_buf;
    loc += section.sh_offset;
    loc += relas[i].r_offset;
    if (sym.st_shndx == SHN_ABS)
    sec_base = 0;
#[no_mangle]
pub unsafe extern "C" fn if(pi->ehdr->e_shnum: sym->st_shndx >=) -> else {
    pr_err("Invalid section %d for symbol %s\n",
    sym.st_shndx, name);
    return -ENOEXEC;
    } else
    sec_base = pi.sechdrs[sym.st_shndx].sh_addr;
    val = sym.st_value;
    val += sec_base;
    val += relas[i].r_addend;
    addr = section.sh_addr + relas[i].r_offset;
    r_type = ELF64_R_TYPE(relas[i].r_info);
    switch (r_type) {
    case R_RISCV_BRANCH:
// (u32 *)loc = CLEAN_IMM(BTYPE, *(u32 *)loc) |
    ENCODE_BTYPE_IMM(val - addr);
    break;
    case R_RISCV_JAL:
// (u32 *)loc = CLEAN_IMM(JTYPE, *(u32 *)loc) |
    ENCODE_JTYPE_IMM(val - addr);
    break;
//
// With no R_RISCV_PCREL_LO12_S, R_RISCV_PCREL_LO12_I
// sym is expected to be next to R_RISCV_PCREL_HI20
// in purgatory relsec. Handle it like R_RISCV_CALL
// sym, instead of searching the whole relsec.
//
    case R_RISCV_PCREL_HI20:
    case R_RISCV_CALL_PLT:
    case R_RISCV_CALL:
// (u64 *)loc = CLEAN_IMM(UITYPE, *(u64 *)loc) |
    ENCODE_UJTYPE_IMM(val - addr);
    break;
    case R_RISCV_RVC_BRANCH:
// (u32 *)loc = CLEAN_IMM(CBTYPE, *(u32 *)loc) |
    ENCODE_CBTYPE_IMM(val - addr);
    break;
    case R_RISCV_RVC_JUMP:
// (u32 *)loc = CLEAN_IMM(CJTYPE, *(u32 *)loc) |
    ENCODE_CJTYPE_IMM(val - addr);
    break;
    case R_RISCV_ADD16:
// (u16 *)loc += val;
    break;
    case R_RISCV_SUB16:
// (u16 *)loc -= val;
    break;
    case R_RISCV_ADD32:
// (u32 *)loc += val;
    break;
    case R_RISCV_SUB32:
// (u32 *)loc -= val;
    break;
// It has been applied by R_RISCV_PCREL_HI20 sym
    case R_RISCV_PCREL_LO12_I:
    case R_RISCV_ALIGN:
    case R_RISCV_RELAX:
    break;
    case R_RISCV_64:
// (u64 *)loc = val;
    break;
    default:
    pr_err("Unknown rela relocation: %d\n", r_type);
    return -ENOEXEC;
    }
    }
    return 0;
    }
    int load_extra_segments(struct kimage *image, unsigned long kernel_start,
    unsigned long kernel_len, char *initrd,
    unsigned long initrd_len, char *cmdline,
    unsigned long cmdline_len)
    {
    int ret;
    void *fdt;
    let mut initrd_pbase: c_ulong = 0UL;
    let mut kbuf: kexec_buf = {};
    char *modified_cmdline = core::ptr::null_mut();
    kbuf.image = image;
    kbuf.buf_min = kernel_start + kernel_len;
    kbuf.buf_max = PFN_PHYS(max_low_pfn);

// Add elfcorehdr
    if (image.type == KEXEC_TYPE_CRASH) {
    void *headers;
    unsigned long headers_sz;
    ret = crash_prepare_headers(true, &headers, &headers_sz, core::ptr::null_mut());
    if (ret) {
    pr_err("Preparing elf core header failed\n");
    goto out;
    }
    kbuf.buffer = headers;
    kbuf.bufsz = headers_sz;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = headers_sz;
    kbuf.buf_align = ELF_CORE_HEADER_ALIGN;
    kbuf.top_down = true;
    ret = kexec_add_buffer(&kbuf);
    if (ret) {
    vfree(headers);
    goto out;
    }
    image.elf_headers = headers;
    image.elf_load_addr = kbuf.mem;
    image.elf_headers_sz = headers_sz;
    kexec_dprintk("Loaded elf core header at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    image.elf_load_addr, kbuf.bufsz, kbuf.memsz);
// Setup cmdline for kdump kernel case
    modified_cmdline = setup_kdump_cmdline(image, cmdline,
    cmdline_len);
    if (!modified_cmdline) {
    pr_err("Setting up cmdline for kdump kernel failed\n");
    ret = -EINVAL;
    goto out;
    }
    cmdline = modified_cmdline;
    }

// Add purgatory to the image
    kbuf.top_down = true;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    ret = kexec_load_purgatory(image, &kbuf);
    if (ret) {
    pr_err("Error loading purgatory ret=%d\n", ret);
    goto out;
    }
    kexec_dprintk("Loaded purgatory at 0x%lx\n", kbuf.mem);
    ret = kexec_purgatory_get_set_symbol(image, "riscv_kernel_entry",
    &kernel_start,
    sizeof(kernel_start), 0);
    if (ret)
    pr_err("Error update purgatory ret=%d\n", ret);

// Add the initrd to the image
    if (initrd != core::ptr::null_mut()) {
    kbuf.buffer = initrd;
    kbuf.bufsz = kbuf.memsz = initrd_len;
    kbuf.buf_align = PAGE_SIZE;
    kbuf.top_down = true;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    ret = kexec_add_buffer(&kbuf);
    if (ret)
    goto out;
    initrd_pbase = kbuf.mem;
    kexec_dprintk("Loaded initrd at 0x%lx\n", initrd_pbase);
    }
// Add the DTB to the image
    fdt = of_kexec_alloc_and_setup_fdt(image, initrd_pbase,
    initrd_len, cmdline, 0);
    if (!fdt) {
    pr_err("Error setting up the new device tree.\n");
    ret = -EINVAL;
    goto out;
    }
    fdt_pack(fdt);
    kbuf.buffer = fdt;
    kbuf.bufsz = kbuf.memsz = fdt_totalsize(fdt);
    kbuf.buf_align = PAGE_SIZE;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.top_down = true;
    ret = kexec_add_buffer(&kbuf);
    if (ret) {
    pr_err("Error add DTB kbuf ret=%d\n", ret);
    goto out_free_fdt;
    }
// Cache the fdt buffer address for memory cleanup
    image.arch.fdt = fdt;
    kexec_dprintk("Loaded device tree at 0x%lx\n", kbuf.mem);
    goto out;
    out_free_fdt:
    kvfree(fdt);
    out:
    kfree(modified_cmdline);
    return ret;
    }
