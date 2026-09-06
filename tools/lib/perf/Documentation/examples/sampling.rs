//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/Documentation/examples/sampling.c
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


    static int libperf_print(enum libperf_print_level level,
    const char *fmt, va_list ap)
    {
    return vfprintf(stderr, fmt, ap);
    }
    union u64_swap {
    __u64 val64;
    __u32 val32[2];
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct perf_evlist *evlist;
    struct perf_evsel *evsel;
    struct perf_mmap *map;
    struct perf_cpu_map *cpus;
    struct perf_event_attr attr = {
    .type        = PERF_TYPE_HARDWARE,
    .config      = PERF_COUNT_HW_CPU_CYCLES,
    .disabled    = 1,
    .freq        = 1,
    .sample_freq = 10,
    .sample_type = PERF_SAMPLE_IP|PERF_SAMPLE_TID|PERF_SAMPLE_CPU|PERF_SAMPLE_PERIOD,
    };
    let mut err: c_int = -1;
    union perf_event *event;
    libperf_init(libperf_print);
    cpus = perf_cpu_map__new_online_cpus();
    if (!cpus) {
    fprintf(stderr, "failed to create cpus\n");
    return -1;
    }
    evlist = perf_evlist__new();
    if (!evlist) {
    fprintf(stderr, "failed to create evlist\n");
    goto out_cpus;
    }
    evsel = perf_evsel__new(&attr);
    if (!evsel) {
    fprintf(stderr, "failed to create cycles\n");
    goto out_cpus;
    }
    perf_evlist__add(evlist, evsel);
    perf_evlist__set_maps(evlist, cpus, core::ptr::null_mut());
    err = perf_evlist__open(evlist);
    if (err) {
    fprintf(stderr, "failed to open evlist\n");
    goto out_evlist;
    }
    err = perf_evlist__mmap(evlist, 4);
    if (err) {
    fprintf(stderr, "failed to mmap evlist\n");
    goto out_evlist;
    }
    perf_evlist__enable(evlist);
    sleep(3);
    perf_evlist__disable(evlist);
    perf_evlist__for_each_mmap(evlist, map, false) {
    if (perf_mmap__read_init(map) < 0)
    continue;
    while ((event = perf_mmap__read_event(map)) != core::ptr::null_mut()) {
    int cpu, pid, tid;
    __u64 ip, period, *array;
    union u64_swap u;
    array = event.sample.array;
    ip = *array;
    array++;
    u.val64 = *array;
    pid = u.val32[0];
    tid = u.val32[1];
    array++;
    u.val64 = *array;
    cpu = u.val32[0];
    array++;
    period = *array;
    fprintf(stdout, "cpu %3d, pid %6d, tid %6d, ip %20llx, period %20llu\n",
    cpu, pid, tid, ip, period);
    perf_mmap__consume(map);
    }
    perf_mmap__read_done(map);
    }
    out_evlist:
    perf_evlist__delete(evlist);
    out_cpus:
    perf_cpu_map__put(cpus);
    return err;
    }
