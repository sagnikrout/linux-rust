//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/module.c
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
// Kernel module help for powerpc.
    Copyright (C) 2001, 2003 Rusty Russell IBM Corporation.
    Copyright (C) 2008 Freescale Semiconductor, Inc.
//

    static const Elf_Shdr *find_section(const Elf_Ehdr *hdr,
    const Elf_Shdr *sechdrs,
    const char *name)
    {
    char *secstrings;
    unsigned int i;
    secstrings = (char *)hdr + sechdrs[hdr.e_shstrndx].sh_offset;
    for (i = 1; i < hdr.e_shnum; i++)
    if (strcmp(secstrings+sechdrs[i].sh_name, name) == 0)
    return &sechdrs[i];
    return core::ptr::null_mut();
    }
    int module_finalize(const Elf_Ehdr *hdr,
    const Elf_Shdr *sechdrs, struct module *me)
    {
    const Elf_Shdr *sect;
    int rc;
    rc = module_finalize_ftrace(me, sechdrs);
    if (rc)
    return rc;
// Apply feature fixups
    sect = find_section(hdr, sechdrs, "__ftr_fixup");
    if (sect != core::ptr::null_mut())
    do_feature_fixups(cur_cpu_spec.cpu_features,
    (void *)sect.sh_addr,
    (void *)sect.sh_addr + sect.sh_size);
    sect = find_section(hdr, sechdrs, "__mmu_ftr_fixup");
    if (sect != core::ptr::null_mut())
    do_feature_fixups(cur_cpu_spec.mmu_features,
    (void *)sect.sh_addr,
    (void *)sect.sh_addr + sect.sh_size);

    sect = find_section(hdr, sechdrs, "__fw_ftr_fixup");
    if (sect != core::ptr::null_mut())
    do_feature_fixups(powerpc_firmware_features,
    (void *)sect.sh_addr,
    (void *)sect.sh_addr + sect.sh_size);

    sect = find_section(hdr, sechdrs, ".opd");
    if (sect != core::ptr::null_mut()) {
    me.arch.start_opd = sect.sh_addr;
    me.arch.end_opd = sect.sh_addr + sect.sh_size;
    }

    sect = find_section(hdr, sechdrs, "__spec_barrier_fixup");
    if (sect != core::ptr::null_mut())
    do_barrier_nospec_fixups_range(barrier_nospec_enabled,
    (void *)sect.sh_addr,
    (void *)sect.sh_addr + sect.sh_size);

    sect = find_section(hdr, sechdrs, "__lwsync_fixup");
    if (sect != core::ptr::null_mut())
    do_lwsync_fixups(cur_cpu_spec.cpu_features,
    (void *)sect.sh_addr,
    (void *)sect.sh_addr + sect.sh_size);
    return 0;
    }
