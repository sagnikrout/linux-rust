//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm64/util/header.c
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


pub const MIDR_SIZE: c_int = 19;

#[no_mangle]
unsafe extern "C" fn _get_cpuid(buf: *mut c_char, sz: usize, cpu: perf_cpu) -> c_int {
    static int _get_cpuid(char *buf, size_t sz, struct perf_cpu cpu)
    {
    char path[PATH_MAX];
    FILE *file;
    const char *sysfs = sysfs__mountpoint();
    assert(cpu.cpu != -1);
    if (!sysfs || sz < MIDR_SIZE)
    return EINVAL;
    scnprintf(path, PATH_MAX, "%s/devices/system/cpu/cpu%d" MIDR, sysfs, cpu.cpu);
    file = fopen(path, "r");
    if (!file) {
    pr_debug("fopen failed for file %s\n", path);
    return EINVAL;
    }
    if (!fgets(buf, MIDR_SIZE, file)) {
    pr_debug("Failed to read file %s\n", path);
    fclose(file);
    return EINVAL;
    }
    fclose(file);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_cpuid(buf: *mut c_char, sz: usize, cpu: perf_cpu) -> c_int {
    int get_cpuid(char *buf, size_t sz, struct perf_cpu cpu)
    {
    struct perf_cpu_map *cpus;
    unsigned int idx;
    if (cpu.cpu != -1)
    return _get_cpuid(buf, sz, cpu);
    cpus = perf_cpu_map__new_online_cpus();
    if (!cpus)
    return EINVAL;
    perf_cpu_map__for_each_cpu(cpu, idx, cpus) {
    let mut ret: c_int = _get_cpuid(buf, sz, cpu);
    if (ret == 0)
    return 0;
    }
    return EINVAL;
    }
    char *get_cpuid_str(struct perf_cpu cpu)
    {
    char *buf = malloc(MIDR_SIZE);
    int res;
    if (!buf)
    return core::ptr::null_mut();
// read midr from list of cpus mapped to this pmu
    res = get_cpuid(buf, MIDR_SIZE, cpu);
    if (res) {
    pr_err("failed to get cpuid string for CPU %d\n", cpu.cpu);
    free(buf);
    buf = core::ptr::null_mut();
    }
    return buf;
    }
//
// Return 0 if idstr is a higher or equal to version of the same part as
// mapcpuid. Therefore, if mapcpuid has 0 for revision and variant then any
// version of idstr will match as long as it's the same CPU type.
//
// Return 1 if the CPU type is different or the version of idstr is lower.
//
#[no_mangle]
pub unsafe extern "C" fn strcmp_cpuid_str(mapcpuid: *const c_char, idstr: *const c_char) -> c_int {
    int strcmp_cpuid_str(const char *mapcpuid, const char *idstr)
    {
    let mut map_id: u64 = strtoull(mapcpuid, core::ptr::null_mut(), 16);
    let mut map_id_variant: c_char = FIELD_GET(MIDR_VARIANT_MASK, map_id);
    let mut map_id_revision: c_char = FIELD_GET(MIDR_REVISION_MASK, map_id);
    let mut id: u64 = strtoull(idstr, core::ptr::null_mut(), 16);
    let mut id_variant: c_char = FIELD_GET(MIDR_VARIANT_MASK, id);
    let mut id_revision: c_char = FIELD_GET(MIDR_REVISION_MASK, id);
    let mut id_fields: u64 = ~(MIDR_VARIANT_MASK | MIDR_REVISION_MASK);
// Compare without version first
    if ((map_id & id_fields) != (id & id_fields))
    return 1;
//
// ID matches, now compare version.
//
// Arm revisions (like r0p0) are compared here like two digit semver
// values eg. 1.3 < 2.0 < 2.1 < 2.2.
//
// r = high value = 'Variant' field in MIDR
// p = low value  = 'Revision' field in MIDR
//
    if (id_variant > map_id_variant)
    return 0;
    if (id_variant == map_id_variant && id_revision >= map_id_revision)
    return 0;
//
// variant is less than mapfile variant or variants are the same but
// the revision doesn't match. Return no match.
//
    return 1;
    }
