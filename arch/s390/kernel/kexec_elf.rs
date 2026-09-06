//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/kexec_elf.c
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
// ELF loader for kexec_file_load system call.
//
// Copyright IBM Corp. 2018
//
// Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
//

    static int kexec_file_add_kernel_elf(struct kimage *image,
    struct s390_load_data *data)
    {
    let mut buf: kexec_buf = {};
    const Elf_Ehdr *ehdr;
    const Elf_Phdr *phdr;
    Elf_Addr entry;
    void *kernel;
    int i, ret;
    kernel = image.kernel_buf;
    ehdr = (Elf_Ehdr *)kernel;
    buf.image = image;
    if (image.type == KEXEC_TYPE_CRASH)
    entry = STARTUP_KDUMP_OFFSET;
    else
    entry = ehdr.e_entry;
    phdr = (void *)ehdr + ehdr.e_phoff;
    for (i = 0; i < ehdr.e_phnum; i++, phdr++) {
    if (phdr.p_type != PT_LOAD)
    continue;
    buf.buffer = kernel + phdr.p_offset;
    buf.bufsz = phdr.p_filesz;
    buf.mem = ALIGN(phdr.p_paddr, phdr.p_align);

    if (image.type == KEXEC_TYPE_CRASH)
    buf.mem += crashk_res.start;

    buf.memsz = phdr.p_memsz;
    data.memsz = ALIGN(data.memsz, phdr.p_align) + buf.memsz;
    if (entry - phdr.p_paddr < phdr.p_memsz) {
    data.kernel_buf = buf.buffer;
    data.kernel_mem = buf.mem;
    data.parm = buf.buffer + PARMAREA;
    }
    ipl_report_add_component(data.report, &buf,
    IPL_RB_COMPONENT_FLAG_SIGNED |
    IPL_RB_COMPONENT_FLAG_VERIFIED,
    IPL_RB_CERT_UNKNOWN);
    ret = kexec_add_buffer(&buf);
    if (ret)
    return ret;
    }
    return data.memsz ? 0 : -EINVAL;
    }
    static void *s390_elf_load(struct kimage *image,
    char *kernel, unsigned long kernel_len,
    char *initrd, unsigned long initrd_len,
    char *cmdline, unsigned long cmdline_len)
    {
    const Elf_Ehdr *ehdr;
    const Elf_Phdr *phdr;
    size_t size;
    int i;
// image->fobs->probe already checked for valid ELF magic number.
    ehdr = (Elf_Ehdr *)kernel;
    if (ehdr.e_type != ET_EXEC ||
    ehdr.e_ident[EI_CLASS] != ELFCLASS64 ||
    !elf_check_arch(ehdr))
    return ERR_PTR(-EINVAL);
    if (!ehdr.e_phnum || ehdr.e_phentsize != sizeof(Elf_Phdr))
    return ERR_PTR(-EINVAL);
    size = ehdr.e_ehsize + ehdr.e_phoff;
    size += ehdr.e_phentsize * ehdr.e_phnum;
    if (size > kernel_len)
    return ERR_PTR(-EINVAL);
    phdr = (void *)ehdr + ehdr.e_phoff;
    size = ALIGN(size, phdr.p_align);
    for (i = 0; i < ehdr.e_phnum; i++, phdr++) {
    if (phdr.p_type == PT_INTERP)
    return ERR_PTR(-EINVAL);
    if (phdr.p_offset > kernel_len)
    return ERR_PTR(-EINVAL);
    size += ALIGN(phdr.p_filesz, phdr.p_align);
    }
    if (size > kernel_len)
    return ERR_PTR(-EINVAL);
    return kexec_file_add_components(image, kexec_file_add_kernel_elf);
    }
#[no_mangle]
unsafe extern "C" fn s390_elf_probe(buf: *const c_char, len: c_ulong) -> c_int {
    static int s390_elf_probe(const char *buf, unsigned long len)
    {
    const Elf_Ehdr *ehdr;
    if (len < sizeof(Elf_Ehdr))
    return -ENOEXEC;
    ehdr = (Elf_Ehdr *)buf;
// Only check the ELF magic number here and do proper validity check
// in the loader. Any check here that fails would send the erroneous
// ELF file to the image loader that does not care what it gets.
// (Most likely) causing behavior not intended by the user.
//
    if (memcmp(ehdr.e_ident, ELFMAG, SELFMAG) != 0)
    return -ENOEXEC;
    return 0;
    }
    const struct kexec_file_ops s390_kexec_elf_ops = {
    .probe = s390_elf_probe,
    .load = s390_elf_load,

    .verify_sig = s390_verify_sig,

    };
