//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/machine_kexec_file.c
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
// s390 code for kexec_file_load system call
//
// Copyright IBM Corp. 2018
//
// Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
//

    const struct kexec_file_ops * const kexec_file_loaders[] = {
    &s390_kexec_elf_ops,
    &s390_kexec_image_ops,
    core::ptr::null_mut(),
    };

#[no_mangle]
pub unsafe extern "C" fn s390_verify_sig(kernel: *const c_char, kernel_len: c_ulong) -> c_int {
    int s390_verify_sig(const char *kernel, unsigned long kernel_len)
    {
    let mut marker_len: c_ulong = sizeof(MODULE_SIGNATURE_MARKER) - 1;
    struct module_signature *ms;
    unsigned long sig_len;
    int ret;
// Skip signature verification when not secure IPLed.
    if (!ipl_secure_flag)
    return 0;
    if (marker_len > kernel_len)
    return -EKEYREJECTED;
    if (memcmp(kernel + kernel_len - marker_len, MODULE_SIGNATURE_MARKER,
    marker_len))
    return -EKEYREJECTED;
    kernel_len -= marker_len;
    ms = (void *)kernel + kernel_len - sizeof(*ms);
    kernel_len -= sizeof(*ms);
    sig_len = be32_to_cpu(ms.sig_len);
    if (sig_len >= kernel_len)
    return -EKEYREJECTED;
    kernel_len -= sig_len;
    if (ms.id_type != MODULE_SIGNATURE_TYPE_PKCS7)
    return -EKEYREJECTED;
    if (ms.algo != 0 ||
    ms.hash != 0 ||
    ms.signer_len != 0 ||
    ms.key_id_len != 0 ||
    ms.__pad[0] != 0 ||
    ms.__pad[1] != 0 ||
    ms.__pad[2] != 0) {
    return -EBADMSG;
    }
    ret = verify_pkcs7_signature(kernel, kernel_len,
    kernel + kernel_len, sig_len,
    VERIFY_USE_SECONDARY_KEYRING,
    VERIFYING_MODULE_SIGNATURE,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (ret == -ENOKEY && IS_ENABLED(CONFIG_INTEGRITY_PLATFORM_KEYRING))
    ret = verify_pkcs7_signature(kernel, kernel_len,
    kernel + kernel_len, sig_len,
    VERIFY_USE_PLATFORM_KEYRING,
    VERIFYING_MODULE_SIGNATURE,
    core::ptr::null_mut(), core::ptr::null_mut());
    return ret;
    }

    static int kexec_file_update_purgatory(struct kimage *image,
    struct s390_load_data *data)
    {
    u64 entry, type;
    int ret;
    if (image.type == KEXEC_TYPE_CRASH) {
    entry = STARTUP_KDUMP_OFFSET;
    type = KEXEC_TYPE_CRASH;
    } else {
    entry = STARTUP_NORMAL_OFFSET;
    type = KEXEC_TYPE_DEFAULT;
    }
    ret = kexec_purgatory_get_set_symbol(image, "kernel_entry", &entry,
    sizeof(entry), false);
    if (ret)
    return ret;
    ret = kexec_purgatory_get_set_symbol(image, "kernel_type", &type,
    sizeof(type), false);
    if (ret)
    return ret;

    if (image.type == KEXEC_TYPE_CRASH) {
    u64 crash_size;
    ret = kexec_purgatory_get_set_symbol(image, "crash_start",
    &crashk_res.start,
    sizeof(crashk_res.start),
    false);
    if (ret)
    return ret;
    crash_size = crashk_res.end - crashk_res.start + 1;
    ret = kexec_purgatory_get_set_symbol(image, "crash_size",
    &crash_size,
    sizeof(crash_size),
    false);
    }

    return ret;
    }
    static int kexec_file_add_purgatory(struct kimage *image,
    struct s390_load_data *data)
    {
    let mut buf: kexec_buf = {};
    int ret;
    buf.image = image;
    data.memsz = ALIGN(data.memsz, PAGE_SIZE);
    buf.mem = data.memsz;

    if (image.type == KEXEC_TYPE_CRASH)
    buf.mem += crashk_res.start;

    ret = kexec_load_purgatory(image, &buf);
    if (ret)
    return ret;
    data.memsz += buf.memsz;
    return kexec_file_update_purgatory(image, data);
    }
    static int kexec_file_add_initrd(struct kimage *image,
    struct s390_load_data *data)
    {
    let mut buf: kexec_buf = {};
    int ret;
    buf.image = image;
    buf.buffer = image.initrd_buf;
    buf.bufsz = image.initrd_buf_len;
    data.memsz = ALIGN(data.memsz, PAGE_SIZE);
    buf.mem = data.memsz;

    if (image.type == KEXEC_TYPE_CRASH)
    buf.mem += crashk_res.start;

    buf.memsz = buf.bufsz;
    data.parm.initrd_start = data.memsz;
    data.parm.initrd_size = buf.memsz;
    data.memsz += buf.memsz;
    ret = kexec_add_buffer(&buf);
    if (ret)
    return ret;
    return ipl_report_add_component(data.report, &buf, 0, 0);
    }
    static int kexec_file_add_ipl_report(struct kimage *image,
    struct s390_load_data *data)
    {
    __u32 *lc_ipl_parmblock_ptr;
    unsigned int len, ncerts;
    let mut buf: kexec_buf = {};
    unsigned long addr;
    void *ptr, *end;
    int ret;
    buf.image = image;
    data.memsz = ALIGN(data.memsz, PAGE_SIZE);
    buf.mem = data.memsz;
    ptr = __va(ipl_cert_list_addr);
    end = ptr + ipl_cert_list_size;
    ncerts = 0;
    while (ptr < end) {
    ncerts++;
    len = *(unsigned int *)ptr;
    ptr += sizeof(len);
    ptr += len;
    }
    addr = data.memsz + data.report.size;
    addr += ncerts * sizeof(struct ipl_rb_certificate_entry);
    ptr = __va(ipl_cert_list_addr);
    while (ptr < end) {
    len = *(unsigned int *)ptr;
    ptr += sizeof(len);
    ipl_report_add_certificate(data.report, ptr, addr, len);
    addr += len;
    ptr += len;
    }
    ret = -ENOMEM;
    buf.buffer = ipl_report_finish(data.report);
    if (!buf.buffer)
    goto out;
    buf.bufsz = data.report.size;
    buf.memsz = buf.bufsz;
    image.arch.ipl_buf = buf.buffer;
    data.memsz += buf.memsz;
    lc_ipl_parmblock_ptr =
    data.kernel_buf + offsetof(struct lowcore, ipl_parmblock_ptr);
// lc_ipl_parmblock_ptr = (__u32)buf.mem;

    if (image.type == KEXEC_TYPE_CRASH)
    buf.mem += crashk_res.start;

    ret = kexec_add_buffer(&buf);
    out:
    return ret;
    }
    void *kexec_file_add_components(struct kimage *image,
    int (*add_kernel)(struct kimage *image,
    struct s390_load_data *data))
    {
    let mut max_command_line_size: c_ulong = LEGACY_COMMAND_LINE_SIZE;
    let mut data: s390_load_data = {0};
    unsigned long minsize;
    int ret;
    data.report = ipl_report_init(&ipl_block);
    if (IS_ERR(data.report))
    return data.report;
    ret = add_kernel(image, &data);
    if (ret)
    goto out;
    ret = -EINVAL;
    minsize = PARMAREA + offsetof(struct parmarea, command_line);
    if (image.kernel_buf_len < minsize)
    goto out;
    if (data.parm.max_command_line_size)
    max_command_line_size = data.parm.max_command_line_size;
    if (minsize + max_command_line_size < minsize)
    goto out;
    if (image.kernel_buf_len < minsize + max_command_line_size)
    goto out;
    if (image.cmdline_buf_len >= max_command_line_size) {
    pr_err("Kernel command line exceeds supported limit of %lu", max_command_line_size);
    goto out;
    }
    memcpy(data.parm.command_line, image.cmdline_buf,
    image.cmdline_buf_len);

    if (image.type == KEXEC_TYPE_CRASH) {
    data.parm.oldmem_base = crashk_res.start;
    data.parm.oldmem_size = crashk_res.end - crashk_res.start + 1;
    }

    if (image.initrd_buf) {
    ret = kexec_file_add_initrd(image, &data);
    if (ret)
    goto out;
    }
    ret = kexec_file_add_purgatory(image, &data);
    if (ret)
    goto out;
    if (data.kernel_mem == 0) {
    let mut restart_psw: c_ulong = 0x0008000080000000UL;
    restart_psw += image.start;
    memcpy(data.kernel_buf, &restart_psw, sizeof(restart_psw));
    image.start = 0;
    }
    ret = kexec_file_add_ipl_report(image, &data);
    out:
    ipl_report_free(data.report);
    return ERR_PTR(ret);
    }
    int arch_kexec_apply_relocations_add(struct purgatory_info *pi,
    Elf_Shdr *section,
    const Elf_Shdr *relsec,
    const Elf_Shdr *symtab)
    {
    const char *strtab, *name, *shstrtab;
    const Elf_Shdr *sechdrs;
    Elf_Rela *relas;
    int i, r_type;
    int ret;
// String & section header string table
    sechdrs = (void *)pi.ehdr + pi.ehdr.e_shoff;
    strtab = (char *)pi.ehdr + sechdrs[symtab.sh_link].sh_offset;
    shstrtab = (char *)pi.ehdr + sechdrs[pi.ehdr.e_shstrndx].sh_offset;
    relas = (void *)pi.ehdr + relsec.sh_offset;
    for (i = 0; i < relsec.sh_size / sizeof(*relas); i++) {
    const Elf_Sym *sym;	/* symbol to relocate */
    unsigned long addr;	/* final location after relocation */
    unsigned long val;	/* relocated symbol value */
    void *loc;		/* tmp location to modify */
    sym = (void *)pi.ehdr + symtab.sh_offset;
    sym += ELF64_R_SYM(relas[i].r_info);
    if (sym.st_name)
    name = strtab + sym.st_name;
    else
    name = shstrtab + sechdrs[sym.st_shndx].sh_name;
    if (sym.st_shndx == SHN_UNDEF) {
    pr_err("Undefined symbol: %s\n", name);
    return -ENOEXEC;
    }
    if (sym.st_shndx == SHN_COMMON) {
    pr_err("symbol '%s' in common section\n", name);
    return -ENOEXEC;
    }
    if (sym.st_shndx >= pi.ehdr.e_shnum &&
    sym.st_shndx != SHN_ABS) {
    pr_err("Invalid section %d for symbol %s\n",
    sym.st_shndx, name);
    return -ENOEXEC;
    }
    loc = pi.purgatory_buf;
    loc += section.sh_offset;
    loc += relas[i].r_offset;
    val = sym.st_value;
    if (sym.st_shndx != SHN_ABS)
    val += pi.sechdrs[sym.st_shndx].sh_addr;
    val += relas[i].r_addend;
    addr = section.sh_addr + relas[i].r_offset;
    r_type = ELF64_R_TYPE(relas[i].r_info);
    if (r_type == R_390_PLT32DBL)
    r_type = R_390_PC32DBL;
    ret = arch_kexec_do_relocs(r_type, loc, val, addr);
    if (ret) {
    pr_err("Unknown rela relocation: %d\n", r_type);
    return -ENOEXEC;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int {
    int arch_kimage_file_post_load_cleanup(struct kimage *image)
    {
    vfree(image.arch.ipl_buf);
    image.arch.ipl_buf = core::ptr::null_mut();
    return kexec_image_post_load_cleanup_default(image);
    }
