//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/fdtparams.c
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

    enum {
    SYSTAB,
    MMBASE,
    MMSIZE,
    DCSIZE,
    DCVERS,
    PARAMCOUNT
    };
    static __initconst const char name[][22] = {
    [SYSTAB] = "System Table         ",
    [MMBASE] = "MemMap Address       ",
    [MMSIZE] = "MemMap Size          ",
    [DCSIZE] = "MemMap Desc. Size    ",
    [DCVERS] = "MemMap Desc. Version ",
    };
    static __initconst const struct {
    const char	path[17];
    u8		paravirt;
    const char	params[PARAMCOUNT][26];
    } dt_params[] = {
    {

    .path = "/hypervisor/uefi",
    .paravirt = 1,
    .params = {
    [SYSTAB] = "xen,uefi-system-table",
    [MMBASE] = "xen,uefi-mmap-start",
    [MMSIZE] = "xen,uefi-mmap-size",
    [DCSIZE] = "xen,uefi-mmap-desc-size",
    [DCVERS] = "xen,uefi-mmap-desc-ver",
    }
    }, {

    .path = "/chosen",
    .params = {	//  <-----------26----------.
    [SYSTAB] = "linux,uefi-system-table",
    [MMBASE] = "linux,uefi-mmap-start",
    [MMSIZE] = "linux,uefi-mmap-size",
    [DCSIZE] = "linux,uefi-mmap-desc-size",
    [DCVERS] = "linux,uefi-mmap-desc-ver",
    }
    }
    };
    static int __init efi_get_fdt_prop(const void *fdt, int node, const char *pname,
    const char *rname, void *var, int size)
    {
    const void *prop;
    int len;
    u64 val;
    prop = fdt_getprop(fdt, node, pname, &len);
    if (!prop)
    return 1;
    val = (len == 4) ? (u64)be32_to_cpup(prop) : get_unaligned_be64(prop);
    if (size == 8)
// (u64 *)var = val;
    else
// (u32 *)var = (val < U32_MAX) ? val : U32_MAX; // saturate
    if (efi_enabled(EFI_DBG))
    pr_info("  %s: 0x%0*llx\n", rname, size * 2, val);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_get_fdt_params(mm: *mut efi_memory_map_data) -> u64 __init {
    u64 __init efi_get_fdt_params(struct efi_memory_map_data *mm)
    {
    const void *fdt = initial_boot_params;
    unsigned long systab;
    int i, j, node;
    struct {
    void	*var;
    int	size;
    } target[] = {
    [SYSTAB] = { &systab,		sizeof(systab) },
    [MMBASE] = { &mm.phys_map,	sizeof(mm.phys_map) },
    [MMSIZE] = { &mm.size,		sizeof(mm.size) },
    [DCSIZE] = { &mm.desc_size,	sizeof(mm.desc_size) },
    [DCVERS] = { &mm.desc_version,	sizeof(mm.desc_version) },
    };
    BUILD_BUG_ON(ARRAY_SIZE(target) != ARRAY_SIZE(name));
    BUILD_BUG_ON(ARRAY_SIZE(target) != ARRAY_SIZE(dt_params[0].params));
    if (!fdt)
    return 0;
    for (i = 0; i < ARRAY_SIZE(dt_params); i++) {
    node = fdt_path_offset(fdt, dt_params[i].path);
    if (node < 0)
    continue;
    if (efi_enabled(EFI_DBG))
    pr_info("Getting UEFI parameters from %s in DT:\n",
    dt_params[i].path);
    for (j = 0; j < ARRAY_SIZE(target); j++) {
    const char *pname = dt_params[i].params[j];
    if (!efi_get_fdt_prop(fdt, node, pname, name[j],
    target[j].var, target[j].size))
    continue;
    if (!j)
    goto notfound;
    pr_err("Can't find property '%s' in DT!\n", pname);
    return 0;
    }
    if (dt_params[i].paravirt)
    set_bit(EFI_PARAVIRT, &efi.flags);
    return systab;
    }
    notfound:
    pr_info("UEFI not found.\n");
    return 0;
    }
