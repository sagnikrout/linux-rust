//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/util/machine.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extra_kernel_map_info {
    pub cnt: c_int,
    pub max_cnt: c_int,
    pub maps: *mut extra_kernel_map,
    pub get_entry_trampolines: bool,
    pub entry_trampoline: u64,
}

    static int add_extra_kernel_map(struct extra_kernel_map_info *mi, u64 start,
    u64 end, u64 pgoff, const char *name)
    {
    if (mi.cnt >= mi.max_cnt) {
    void *buf;
    size_t sz;
    mi.max_cnt = mi.max_cnt ? mi.max_cnt * 2 : 32;
    sz = sizeof(struct extra_kernel_map) * mi.max_cnt;
    buf = realloc(mi.maps, sz);
    if (!buf)
    return -1;
    mi.maps = buf;
    }
    mi.maps[mi.cnt].start = start;
    mi.maps[mi.cnt].end   = end;
    mi.maps[mi.cnt].pgoff = pgoff;
    strlcpy(mi.maps[mi.cnt].name, name, KMAP_NAME_LEN);
    mi.cnt += 1;
    return 0;
    }
    static int find_extra_kernel_maps(void *arg, const char *name, char type,
    u64 start)
    {
    struct extra_kernel_map_info *mi = arg;
    if (!mi.entry_trampoline && kallsyms2elf_binding(type) == STB_GLOBAL &&
    !strcmp(name, "_entry_trampoline")) {
    mi.entry_trampoline = start;
    return 0;
    }
    if (is_entry_trampoline(name)) {
    let mut end: u64 = start + page_size;
    return add_extra_kernel_map(mi, start, end, 0, name);
    }
    return 0;
    }
    int machine__create_extra_kernel_maps(struct machine *machine,
    struct dso *kernel)
    {
    let mut mi: extra_kernel_map_info = { .cnt = 0, };
    char filename[PATH_MAX];
    int ret;
    int i;
    machine__get_kallsyms_filename(machine, filename, PATH_MAX);
    if (symbol__restricted_filename(filename, "/proc/kallsyms"))
    return 0;
    ret = kallsyms__parse(filename, &mi, find_extra_kernel_maps);
    if (ret)
    goto out_free;
    if (!mi.entry_trampoline)
    goto out_free;
    for (i = 0; i < mi.cnt; i++) {
    struct extra_kernel_map *xm = &mi.maps[i];
    xm.pgoff = mi.entry_trampoline;
    ret = machine__create_extra_kernel_map(machine, kernel, xm);
    if (ret)
    goto out_free;
    }
    machine.trampolines_mapped = mi.cnt;
    out_free:
    free(mi.maps);
    return ret;
    }
