//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/loongarch/util/header.c
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
// Implementation of get_cpuid().
//
// Author: Nikita Shubin <n.shubin@yadro.com>
// Bibo Mao <maobibo@loongson.cn>
// Huacai Chen <chenhuacai@loongson.cn>
//

//
// Output example from /proc/cpuinfo
// CPU Family              : Loongson-64bit
// Model Name              : Loongson-3C5000
// CPU Revision            : 0x10
// FPU Revision            : 0x01
//

    static char *_get_field(const char *line)
    {
    char *line2, *nl;
    line2 = strrchr(line, ' ');
    if (!line2)
    return core::ptr::null_mut();
    line2++;
    nl = strrchr(line, '\n');
    if (!nl)
    return core::ptr::null_mut();
    return strndup(line2, nl - line2);
    }
    static char *_get_cpuid(void)
    {
    unsigned long line_sz;
    char *line, *model, *cpuid;
    FILE *file;
    file = fopen(CPUINFO, "r");
    if (file == core::ptr::null_mut())
    return core::ptr::null_mut();
    line = model = cpuid = core::ptr::null_mut();
    while (getline(&line, &line_sz, file) != -1) {
    if (strncmp(line, CPUINFO_MODEL, strlen(CPUINFO_MODEL)))
    continue;
    model = _get_field(line);
    if (!model)
    goto out_free;
    break;
    }
    if (model && (asprintf(&cpuid, "%s", model) < 0))
    cpuid = core::ptr::null_mut();
    out_free:
    fclose(file);
    free(model);
    return cpuid;
    }
#[no_mangle]
pub unsafe extern "C" fn get_cpuid(buffer: *mut c_char, sz: usize, __maybe_unused: perf_cpu cpu) -> c_int {
    int get_cpuid(char *buffer, size_t sz, struct perf_cpu cpu __maybe_unused)
    {
    let mut ret: c_int = 0;
    char *cpuid = _get_cpuid();
    if (!cpuid)
    return EINVAL;
    if (sz < strlen(cpuid)) {
    ret = ENOBUFS;
    goto out_free;
    }
    scnprintf(buffer, sz, "%s", cpuid);
    out_free:
    free(cpuid);
    return ret;
    }
    char *get_cpuid_str(struct perf_cpu cpu __maybe_unused)
    {
    return _get_cpuid();
    }
