//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_trace_augment.c
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


    static struct augmented_raw_syscalls_bpf *skel;
    static struct evsel *bpf_output;
#[no_mangle]
pub unsafe extern "C" fn augmented_syscalls__prepare() -> c_int {
    int augmented_syscalls__prepare(void)
    {
    struct bpf_program *prog;
    char buf[128];
    int err;
    skel = augmented_raw_syscalls_bpf__open();
    if (!skel) {
    pr_debug("Failed to open augmented syscalls BPF skeleton\n");
    return -errno;
    }
//
// Disable attaching the BPF programs except for sys_enter and
// sys_exit that tail call into this as necessary.
//
    bpf_object__for_each_program(prog, skel.obj) {
    if (prog != skel.progs.sys_enter && prog != skel.progs.sys_exit)
    bpf_program__set_autoattach(prog, /*autoattach=*/false);
    }
    err = augmented_raw_syscalls_bpf__load(skel);
    if (err < 0) {
    libbpf_strerror(err, buf, sizeof(buf));
    pr_debug("Failed to load augmented syscalls BPF skeleton: %s\n", buf);
    return err;
    }
    augmented_raw_syscalls_bpf__attach(skel);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn augmented_syscalls__create_bpf_output(evlist: *mut evlist) -> c_int {
    int augmented_syscalls__create_bpf_output(struct evlist *evlist)
    {
    let mut err: c_int = parse_event(evlist, "bpf-output/no-inherit=1,name=__augmented_syscalls__/");
    if (err) {
    pr_err("ERROR: Setup BPF output event failed: %d\n", err);
    return err;
    }
    bpf_output = evlist__last(evlist);
    assert(evsel__name_is(bpf_output, "__augmented_syscalls__"));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn augmented_syscalls__setup_bpf_output() {
    void augmented_syscalls__setup_bpf_output(void)
    {
    struct perf_cpu cpu;
    unsigned int i;
    if (bpf_output == core::ptr::null_mut())
    return;
//
// Set up the __augmented_syscalls__ BPF map to hold for each
// CPU the bpf-output event's file descriptor.
//
    perf_cpu_map__for_each_cpu(cpu, i, bpf_output.core.cpus) {
    let mut mycpu: c_int = cpu.cpu;
    bpf_map__update_elem(skel.maps.__augmented_syscalls__,
    &mycpu, sizeof(mycpu),
    xyarray__entry(bpf_output.core.fd,
    mycpu, 0),
    sizeof(__u32), BPF_ANY);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn augmented_syscalls__set_filter_pids(nr: c_uint, pids: *mut pid_t) -> c_int {
    int augmented_syscalls__set_filter_pids(unsigned int nr, pid_t *pids)
    {
    let mut value: bool = true;
    let mut err: c_int = 0;
    if (skel == core::ptr::null_mut())
    return 0;
    for (size_t i = 0; i < nr; ++i) {
    err = bpf_map__update_elem(skel.maps.pids_filtered, &pids[i],
    sizeof(*pids), &value, sizeof(value),
    BPF_ANY);
    if (err)
    break;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn augmented_syscalls__get_map_fds(enter_fd: *mut c_int, exit_fd: *mut c_int, beauty_fd: *mut c_int) -> c_int {
    int augmented_syscalls__get_map_fds(int *enter_fd, int *exit_fd, int *beauty_fd)
    {
    if (skel == core::ptr::null_mut())
    return -1;
// enter_fd = bpf_map__fd(skel->maps.syscalls_sys_enter);
// exit_fd  = bpf_map__fd(skel->maps.syscalls_sys_exit);
// beauty_fd = bpf_map__fd(skel->maps.beauty_map_enter);
    if (*enter_fd < 0 || *exit_fd < 0 || *beauty_fd < 0) {
    pr_err("Error: failed to get syscall or beauty map fd\n");
    return -1;
    }
    return 0;
    }
    struct bpf_program *augmented_syscalls__unaugmented(void)
    {
    return skel.progs.syscall_unaugmented;
    }
    struct bpf_program *augmented_syscalls__find_by_title(const char *name)
    {
    struct bpf_program *pos;
    const char *sec_name;
    if (skel.obj == core::ptr::null_mut())
    return core::ptr::null_mut();
    bpf_object__for_each_program(pos, skel.obj) {
    sec_name = bpf_program__section_name(pos);
    if (sec_name && !strcmp(sec_name, name))
    return pos;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn augmented_syscalls__cleanup() {
    void augmented_syscalls__cleanup(void)
    {
    augmented_raw_syscalls_bpf__destroy(skel);
    }
