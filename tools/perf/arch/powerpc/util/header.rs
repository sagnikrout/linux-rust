//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/powerpc/util/header.c
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

#[no_mangle]
unsafe extern "C" fn is_compat_mode() -> bool {
    static bool is_compat_mode(void)
    {
    let mut base_platform: c_ulong = getauxval(AT_BASE_PLATFORM);
    let mut platform: c_ulong = getauxval(AT_PLATFORM);
    if (!strcmp((char *)platform, (char *)base_platform))
    return false;
    return true;
    }
    int
    get_cpuid(char *buffer, size_t sz, struct perf_cpu cpu __maybe_unused)
    {
    unsigned long pvr;
    int nb;
    pvr = mfspr(SPRN_PVR);
    nb = scnprintf(buffer, sz, "%lu,%lu$", PVR_VER(pvr), PVR_REV(pvr));
// look for end marker to ensure the entire data fit
    if (strchr(buffer, '$')) {
    buffer[nb-1] = '\0';
    return 0;
    }
    return ENOBUFS;
    }
    char *
    get_cpuid_str(struct perf_cpu cpu __maybe_unused)
    {
    char *bufp;
    unsigned long pvr;
//
// IBM Power System supports compatible mode. That is
// Nth generation platform can support previous generation
// OS in a mode called compatibile mode. For ex. LPAR can be
// booted in a Power9 mode when the system is a Power10.
//
// In the compatible mode, care must be taken when generating
// PVR value. When read, PVR will be of the AT_BASE_PLATFORM
// To support generic events, return 0x00ffffff as pvr when
// booted in compat mode. Based on this pvr value, json will
// pick events from pmu-events/arch/powerpc/compat
//
    if (!is_compat_mode())
    pvr = mfspr(SPRN_PVR);
    else
    pvr = 0x00ffffff;
    if (asprintf(&bufp, "0x%.8lx", pvr) < 0)
    bufp = core::ptr::null_mut();
    return bufp;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_get_runtimeparam(pm: *const pmu_metric) -> c_int {
    int arch_get_runtimeparam(const struct pmu_metric *pm)
    {
    int count;
    char path[PATH_MAX] = "/devices/hv_24x7/interface/";
    strcat(path, pm.aggr_mode == PerChip ? "sockets" : "coresperchip");
    return sysfs__read_int(path, &count) < 0 ? 1 : count;
    }
