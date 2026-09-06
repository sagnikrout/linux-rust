//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_kwork_top.c
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
// bpf_kwork_top.c
//
// Copyright (c) 2022  Huawei Inc,  Yang Jihong <yangjihong1@huawei.com>
//

//
// This should be in sync with "util/kwork_top.bpf.c"
//
pub const MAX_COMMAND_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_data {
    pub timestamp: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_data {
    pub runtime: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_data {
    pub tgid: __u32,
    pub is_kthread: __u32,
    pub comm: [c_char; MAX_COMMAND_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_key {
    pub type: __u32,
    pub pid: __u32,
    pub task_p: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_key {
    pub pid: __u32,
    pub cpu: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwork_class_bpf {
    pub class: *mut kwork_class,
    pub (*load_prepare)(void): *mut c_void,
}

    static struct kwork_top_bpf *skel;
#[no_mangle]
pub unsafe extern "C" fn perf_kwork__top_start() {
    void perf_kwork__top_start(void)
    {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    skel.bss.from_timestamp = (u64)ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec;
    skel.bss.enabled = 1;
    pr_debug("perf kwork top start at: %lld\n", skel.bss.from_timestamp);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_kwork__top_finish() {
    void perf_kwork__top_finish(void)
    {
    struct timespec ts;
    skel.bss.enabled = 0;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    skel.bss.to_timestamp = (u64)ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec;
    pr_debug("perf kwork top finish at: %lld\n", skel.bss.to_timestamp);
    }
#[no_mangle]
unsafe extern "C" fn irq_load_prepare() {
    static void irq_load_prepare(void)
    {
    bpf_program__set_autoload(skel.progs.on_irq_handler_entry, true);
    bpf_program__set_autoload(skel.progs.on_irq_handler_exit, true);
    }
    static struct kwork_class_bpf kwork_irq_bpf = {
    .load_prepare = irq_load_prepare,
    };
#[no_mangle]
unsafe extern "C" fn softirq_load_prepare() {
    static void softirq_load_prepare(void)
    {
    bpf_program__set_autoload(skel.progs.on_softirq_entry, true);
    bpf_program__set_autoload(skel.progs.on_softirq_exit, true);
    }
    static struct kwork_class_bpf kwork_softirq_bpf = {
    .load_prepare = softirq_load_prepare,
    };
#[no_mangle]
unsafe extern "C" fn sched_load_prepare() {
    static void sched_load_prepare(void)
    {
    bpf_program__set_autoload(skel.progs.on_switch, true);
    }
    static struct kwork_class_bpf kwork_sched_bpf = {
    .load_prepare = sched_load_prepare,
    };
    static struct kwork_class_bpf *
    kwork_class_bpf_supported_list[KWORK_CLASS_MAX] = {
    [KWORK_CLASS_IRQ]	= &kwork_irq_bpf,
    [KWORK_CLASS_SOFTIRQ]	= &kwork_softirq_bpf,
    [KWORK_CLASS_SCHED]	= &kwork_sched_bpf,
    };
#[no_mangle]
unsafe extern "C" fn valid_kwork_class_type(type: enum kwork_class_type) -> bool {
    static bool valid_kwork_class_type(enum kwork_class_type type)
    {
    return type >= 0 && type < KWORK_CLASS_MAX;
    }
#[no_mangle]
unsafe extern "C" fn setup_filters(kwork: *mut perf_kwork) -> c_int {
    static int setup_filters(struct perf_kwork *kwork)
    {
    if (kwork.cpu_list) {
    unsigned int idx;
    int nr_cpus, fd;
    struct perf_cpu_map *map;
    struct perf_cpu cpu;
    fd = bpf_map__fd(skel.maps.kwork_top_cpu_filter);
    if (fd < 0) {
    pr_debug("Invalid cpu filter fd\n");
    return -1;
    }
    map = perf_cpu_map__new(kwork.cpu_list);
    if (!map) {
    pr_debug("Invalid cpu_list\n");
    return -1;
    }
    nr_cpus = libbpf_num_possible_cpus();
    perf_cpu_map__for_each_cpu(cpu, idx, map) {
    let mut val: u8 = 1;
    if (cpu.cpu >= nr_cpus) {
    perf_cpu_map__put(map);
    pr_err("Requested cpu %d too large\n", cpu.cpu);
    return -1;
    }
    bpf_map_update_elem(fd, &cpu.cpu, &val, BPF_ANY);
    }
    perf_cpu_map__put(map);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_kwork__top_prepare_bpf(kwork: *mut perf_kwork) -> c_int {
    int perf_kwork__top_prepare_bpf(struct perf_kwork *kwork)
    {
    struct bpf_program *prog;
    struct kwork_class *class;
    struct kwork_class_bpf *class_bpf;
    enum kwork_class_type type;
    skel = kwork_top_bpf__open();
    if (!skel) {
    pr_debug("Failed to open kwork top skeleton\n");
    return -1;
    }
//
// set all progs to non-autoload,
// then set corresponding progs according to config
//
    bpf_object__for_each_program(prog, skel.obj)
    bpf_program__set_autoload(prog, false);
    list_for_each_entry(class, &kwork.class_list, list) {
    type = class.type;
    if (!valid_kwork_class_type(type) ||
    !kwork_class_bpf_supported_list[type]) {
    pr_err("Unsupported bpf trace class %s\n", class.name);
    goto out;
    }
    class_bpf = kwork_class_bpf_supported_list[type];
    class_bpf.class = class;
    if (class_bpf.load_prepare)
    class_bpf.load_prepare();
    }
    if (kwork.cpu_list)
    skel.rodata.has_cpu_filter = 1;
    if (kwork_top_bpf__load(skel)) {
    pr_debug("Failed to load kwork top skeleton\n");
    goto out;
    }
    if (setup_filters(kwork))
    goto out;
    if (kwork_top_bpf__attach(skel)) {
    pr_debug("Failed to attach kwork top skeleton\n");
    goto out;
    }
    return 0;
    out:
    kwork_top_bpf__destroy(skel);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn read_task_info(work: *mut kwork_work) {
    static void read_task_info(struct kwork_work *work)
    {
    int fd;
    struct task_data data;
    struct task_key key = {
    .pid = work.id,
    .cpu = work.cpu,
    };
    fd = bpf_map__fd(skel.maps.kwork_top_tasks);
    if (fd < 0) {
    pr_debug("Invalid top tasks map fd\n");
    return;
    }
    if (!bpf_map_lookup_elem(fd, &key, &data)) {
    work.tgid = data.tgid;
    work.is_kthread = data.is_kthread;
    work.name = strdup(data.comm);
    }
    }
    static int add_work(struct perf_kwork *kwork, struct work_key *key,
    struct work_data *data, int cpu)
    {
    struct kwork_class_bpf *bpf_trace;
    struct kwork_work *work;
    struct kwork_work tmp = {
    .id = key.pid,
    .cpu = cpu,
    .name = core::ptr::null_mut(),
    };
    let mut type: enum kwork_class_type = key.type;
    if (!valid_kwork_class_type(type)) {
    pr_debug("Invalid class type %d to add work\n", type);
    return -1;
    }
    bpf_trace = kwork_class_bpf_supported_list[type];
    tmp.class = bpf_trace.class;
    work = kwork.add_work(kwork, tmp.class, &tmp);
    if (!work)
    return -1;
    work.total_runtime = data.runtime;
    read_task_info(work);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_kwork__top_read_bpf(kwork: *mut perf_kwork) -> c_int {
    int perf_kwork__top_read_bpf(struct perf_kwork *kwork)
    {
    int i, fd, nr_cpus;
    struct work_data *data;
    struct work_key key, prev;
    fd = bpf_map__fd(skel.maps.kwork_top_works);
    if (fd < 0) {
    pr_debug("Invalid top runtime fd\n");
    return -1;
    }
    nr_cpus = libbpf_num_possible_cpus();
    data = calloc(nr_cpus, sizeof(struct work_data));
    if (!data)
    return -1;
    memset(&prev, 0, sizeof(prev));
    while (!bpf_map_get_next_key(fd, &prev, &key)) {
    if ((bpf_map_lookup_elem(fd, &key, data)) != 0) {
    pr_debug("Failed to lookup top elem\n");
    return -1;
    }
    for (i = 0; i < nr_cpus; i++) {
    if (data[i].runtime == 0)
    continue;
    if (add_work(kwork, &key, &data[i], i))
    return -1;
    }
    prev = key;
    }
    free(data);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_kwork__top_cleanup_bpf() {
    void perf_kwork__top_cleanup_bpf(void)
    {
    kwork_top_bpf__destroy(skel);
    }
