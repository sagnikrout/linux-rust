//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/numa.c
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
// numa.c
//
// numa: Simulate NUMA-sensitive workload and measure their NUMA performance
//

//
// Regular printout to the terminal, suppressed if -q is specified:
//

//
// Debug printf:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_data {
    pub curr_cpu: c_int,
    pub bind_cpumask: *mut cpu_set_t,
    pub bind_node: c_int,
    pub process_data: *mut u8,
    pub process_nr: c_int,
    pub thread_nr: c_int,
    pub task_nr: c_int,
    pub loops_done: c_uint,
    pub val: u64,
    pub runtime_ns: u64,
    pub system_time_ns: u64,
    pub user_time_ns: u64,
    pub speed_gbs: double,
    pub process_lock: *mut mutex,
}

// Parameters set by options:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct params {
// Startup synchronization:
    pub serialize_startup: bool,
// Task hierarchy:
    pub nr_proc: c_int,
    pub nr_threads: c_int,
// Working set sizes:
    pub mb_global_str: *const c_char,
    pub mb_proc_str: *const c_char,
    pub mb_proc_locked_str: *const c_char,
    pub mb_thread_str: *const c_char,
    pub mb_global: double,
    pub mb_proc: double,
    pub mb_proc_locked: double,
    pub mb_thread: double,
// Access patterns to the working set:
    pub data_reads: bool,
    pub data_writes: bool,
    pub data_backwards: bool,
    pub data_zero_memset: bool,
    pub data_rand_walk: bool,
    pub nr_loops: u32,
    pub nr_secs: u32,
    pub sleep_usecs: u32,
// Working set initialization:
    pub init_zero: bool,
    pub init_random: bool,
    pub init_cpu0: bool,
// Misc options:
    pub show_details: c_int,
    pub run_all: c_int,
    pub thp: c_int,
    pub bytes_global: c_long,
    pub bytes_process: c_long,
    pub bytes_process_locked: c_long,
    pub bytes_thread: c_long,
    pub nr_tasks: c_int,
    pub show_convergence: bool,
    pub measure_convergence: bool,
    pub perturb_secs: c_int,
    pub nr_cpus: c_int,
    pub nr_nodes: c_int,
// Affinity options -C and -N:
    pub cpu_list_str: *mut c_char,
    pub node_list_str: *mut c_char,
}

// Global, read-writable area, accessible to all processes and threads:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct global_info {
    pub data: *mut u8,
    pub startup_mutex: mutex,
    pub startup_cond: cond,
    pub nr_tasks_started: c_int,
    pub start_work_mutex: mutex,
    pub start_work_cond: cond,
    pub nr_tasks_working: c_int,
    pub start_work: bool,
    pub stop_work_mutex: mutex,
    pub bytes_done: u64,
    pub threads: *mut thread_data,
// Convergence latency measurement:
    pub all_converged: bool,
    pub stop_work: bool,
    pub print_once: c_int,
    pub p: params,
}

    static struct global_info	*g = core::ptr::null_mut();
    static int parse_cpus_opt(const struct option *opt, const char *arg, int unset);
    static int parse_nodes_opt(const struct option *opt, const char *arg, int unset);
    static struct params p0;
    static const struct option options[] = {
    OPT_INTEGER('p', "nr_proc"	, &p0.nr_proc,		"number of processes"),
    OPT_INTEGER('t', "nr_threads"	, &p0.nr_threads,	"number of threads per process"),
    OPT_STRING('G', "mb_global"	, &p0.mb_global_str,	"MB", "global  memory (MBs)"),
    OPT_STRING('P', "mb_proc"	, &p0.mb_proc_str,	"MB", "process memory (MBs)"),
    OPT_STRING('L', "mb_proc_locked", &p0.mb_proc_locked_str,"MB", "process serialized/locked memory access (MBs), <= process_memory"),
    OPT_STRING('T', "mb_thread"	, &p0.mb_thread_str,	"MB", "thread  memory (MBs)"),
    OPT_UINTEGER('l', "nr_loops"	, &p0.nr_loops,		"max number of loops to run (default: unlimited)"),
    OPT_UINTEGER('s', "nr_secs"	, &p0.nr_secs,		"max number of seconds to run (default: 5 secs)"),
    OPT_UINTEGER('u', "usleep"	, &p0.sleep_usecs,	"usecs to sleep per loop iteration"),
    OPT_BOOLEAN('R', "data_reads"	, &p0.data_reads,	"access the data via reads (can be mixed with -W)"),
    OPT_BOOLEAN('W', "data_writes"	, &p0.data_writes,	"access the data via writes (can be mixed with -R)"),
    OPT_BOOLEAN('B', "data_backwards", &p0.data_backwards,	"access the data backwards as well"),
    OPT_BOOLEAN('Z', "data_zero_memset", &p0.data_zero_memset,"access the data via glibc bzero only"),
    OPT_BOOLEAN('r', "data_rand_walk", &p0.data_rand_walk,	"access the data with random (32bit LFSR) walk"),
    OPT_BOOLEAN('z', "init_zero"	, &p0.init_zero,	"bzero the initial allocations"),
    OPT_BOOLEAN('I', "init_random"	, &p0.init_random,	"randomize the contents of the initial allocations"),
    OPT_BOOLEAN('0', "init_cpu0"	, &p0.init_cpu0,	"do the initial allocations on CPU#0"),
    OPT_INTEGER('x', "perturb_secs", &p0.perturb_secs,	"perturb thread 0/0 every X secs, to test convergence stability"),
    OPT_INCR   ('d', "show_details"	, &p0.show_details,	"Show details"),
    OPT_INCR   ('a', "all"		, &p0.run_all,		"Run all tests in the suite"),
    OPT_INTEGER('H', "thp"		, &p0.thp,		"MADV_NOHUGEPAGE < 0 < MADV_HUGEPAGE"),
    OPT_BOOLEAN('c', "show_convergence", &p0.show_convergence, "show convergence details, "
    "convergence is reached when each process (all its threads) is running on a single NUMA node."),
    OPT_BOOLEAN('m', "measure_convergence",	&p0.measure_convergence, "measure convergence latency"),
    OPT_BOOLEAN('q', "quiet"	, &quiet,
    "quiet mode (do not show any warnings or messages)"),
    OPT_BOOLEAN('S', "serialize-startup", &p0.serialize_startup,"serialize thread startup"),
// Special option string parsing callbacks:
    OPT_CALLBACK('C', "cpus", core::ptr::null_mut(), "cpu[,cpu2,...cpuN]",
    "bind the first N tasks to these specific cpus (the rest is unbound)",
    parse_cpus_opt),
    OPT_CALLBACK('M', "memnodes", core::ptr::null_mut(), "node[,node2,...nodeN]",
    "bind the first N tasks to these specific memory nodes (the rest is unbound)",
    parse_nodes_opt),
    OPT_END()
    };
    static const char * const bench_numa_usage[] = {
    "perf bench numa <options>",
    core::ptr::null_mut()
    };
    static const char * const numa_usage[] = {
    "perf bench numa mem [<options>]",
    core::ptr::null_mut()
    };
//
// To get number of numa nodes present.
//
#[no_mangle]
unsafe extern "C" fn nr_numa_nodes() -> c_int {
    static int nr_numa_nodes(void)
    {
    int i, nr_nodes = 0;
    for (i = 0; i < g.p.nr_nodes; i++) {
    if (numa_bitmask_isbitset(numa_nodes_ptr, i))
    nr_nodes++;
    }
    return nr_nodes;
    }
//
// To check if given numa node is present.
//
#[no_mangle]
unsafe extern "C" fn is_node_present(node: c_int) -> c_int {
    static int is_node_present(int node)
    {
    return numa_bitmask_isbitset(numa_nodes_ptr, node);
    }
//
// To check given numa node has cpus.
//
#[no_mangle]
unsafe extern "C" fn node_has_cpus(node: c_int) -> bool {
    static bool node_has_cpus(int node)
    {
    struct bitmask *cpumask = numa_allocate_cpumask();
    bool ret = false; /* fall back to nocpus */
    int cpu;
    BUG_ON(!cpumask);
    if (!numa_node_to_cpus(node, cpumask)) {
    for (cpu = 0; cpu < (int)cpumask.size; cpu++) {
    if (numa_bitmask_isbitset(cpumask, cpu)) {
    ret = true;
    break;
    }
    }
    }
    numa_free_cpumask(cpumask);
    return ret;
    }
    static cpu_set_t *bind_to_cpu(int target_cpu)
    {
    let mut nrcpus: c_int = numa_num_possible_cpus();
    cpu_set_t *orig_mask, *mask;
    size_t size;
    orig_mask = CPU_ALLOC(nrcpus);
    BUG_ON(!orig_mask);
    size = CPU_ALLOC_SIZE(nrcpus);
    CPU_ZERO_S(size, orig_mask);
    if (sched_getaffinity(0, size, orig_mask))
    goto err_out;
    mask = CPU_ALLOC(nrcpus);
    if (!mask)
    goto err_out;
    CPU_ZERO_S(size, mask);
    if (target_cpu == -1) {
    int cpu;
    for (cpu = 0; cpu < g.p.nr_cpus; cpu++)
    CPU_SET_S(cpu, size, mask);
    } else {
    if (target_cpu < 0 || target_cpu >= g.p.nr_cpus)
    goto err;
    CPU_SET_S(target_cpu, size, mask);
    }
    if (sched_setaffinity(0, size, mask))
    goto err;
    return orig_mask;
    err:
    CPU_FREE(mask);
    err_out:
    CPU_FREE(orig_mask);
// BUG_ON due to failure in allocation of orig_mask/mask
    BUG_ON(-1);
    return core::ptr::null_mut();
    }
    static cpu_set_t *bind_to_node(int target_node)
    {
    let mut nrcpus: c_int = numa_num_possible_cpus();
    size_t size;
    cpu_set_t *orig_mask, *mask;
    int cpu;
    orig_mask = CPU_ALLOC(nrcpus);
    BUG_ON(!orig_mask);
    size = CPU_ALLOC_SIZE(nrcpus);
    CPU_ZERO_S(size, orig_mask);
    if (sched_getaffinity(0, size, orig_mask))
    goto err_out;
    mask = CPU_ALLOC(nrcpus);
    if (!mask)
    goto err_out;
    CPU_ZERO_S(size, mask);
    if (target_node == NUMA_NO_NODE) {
    for (cpu = 0; cpu < g.p.nr_cpus; cpu++)
    CPU_SET_S(cpu, size, mask);
    } else {
    struct bitmask *cpumask = numa_allocate_cpumask();
    if (!cpumask)
    goto err;
    if (!numa_node_to_cpus(target_node, cpumask)) {
    for (cpu = 0; cpu < (int)cpumask.size; cpu++) {
    if (numa_bitmask_isbitset(cpumask, cpu))
    CPU_SET_S(cpu, size, mask);
    }
    }
    numa_free_cpumask(cpumask);
    }
    if (sched_setaffinity(0, size, mask))
    goto err;
    return orig_mask;
    err:
    CPU_FREE(mask);
    err_out:
    CPU_FREE(orig_mask);
// BUG_ON due to failure in allocation of orig_mask/mask
    BUG_ON(-1);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn bind_to_cpumask(mask: *mut cpu_set_t) {
    static void bind_to_cpumask(cpu_set_t *mask)
    {
    int ret;
    let mut size: usize = CPU_ALLOC_SIZE(numa_num_possible_cpus());
    ret = sched_setaffinity(0, size, mask);
    if (ret) {
    CPU_FREE(mask);
    BUG_ON(ret);
    }
    }
#[no_mangle]
unsafe extern "C" fn mempol_restore() {
    static void mempol_restore(void)
    {
    int ret;
    ret = set_mempolicy(MPOL_DEFAULT, core::ptr::null_mut(), g.p.nr_nodes-1);
    BUG_ON(ret);
    }
#[no_mangle]
unsafe extern "C" fn bind_to_memnode(node: c_int) {
    static void bind_to_memnode(int node)
    {
    struct bitmask *node_mask;
    int ret;
    if (node == NUMA_NO_NODE)
    return;
    node_mask = numa_allocate_nodemask();
    BUG_ON(!node_mask);
    numa_bitmask_clearall(node_mask);
    numa_bitmask_setbit(node_mask, node);
    ret = set_mempolicy(MPOL_BIND, node_mask.maskp, node_mask.size + 1);
    dprintf("binding to node %d, mask: %016lx => %d\n", node, *node_mask.maskp, ret);
    numa_bitmask_free(node_mask);
    BUG_ON(ret);
    }

    do {							\
    char name[20];					\
    \
    snprintf(name, 20, fmt);			\
    prctl(PR_SET_NAME, name);			\
    } while (0)
    static u8 *alloc_data(ssize_t bytes0, int map_flags,
    int init_zero, int init_cpu0, int thp, int init_random)
    {
    cpu_set_t *orig_mask = core::ptr::null_mut();
    ssize_t bytes;
    u8 *buf;
    int ret;
    if (!bytes0)
    return core::ptr::null_mut();
// Allocate and initialize all memory on CPU#0:
    if (init_cpu0) {
    let mut node: c_int = numa_node_of_cpu(0);
    orig_mask = bind_to_node(node);
    bind_to_memnode(node);
    }
    bytes = bytes0 + HPSIZE;
    buf = (void *)mmap(0, bytes, PROT_READ|PROT_WRITE, MAP_ANON|map_flags, -1, 0);
    BUG_ON(buf == (void *)-1);
    if (map_flags == MAP_PRIVATE) {
    if (thp > 0) {
    ret = madvise(buf, bytes, MADV_HUGEPAGE);
    if (ret && !g.print_once) {
    g.print_once = 1;
    printf("WARNING: Could not enable THP - do: 'echo madvise > /sys/kernel/mm/transparent_hugepage/enabled'\n");
    }
    }
    if (thp < 0) {
    ret = madvise(buf, bytes, MADV_NOHUGEPAGE);
    if (ret && !g.print_once) {
    g.print_once = 1;
    printf("WARNING: Could not disable THP: run a CONFIG_TRANSPARENT_HUGEPAGE kernel?\n");
    }
    }
    }
    if (init_zero) {
    bzero(buf, bytes);
    } else {
// Initialize random contents, different in each word:
    if (init_random) {
    u64 *wbuf = (void *)buf;
    let mut off: c_long = rand();
    long i;
    for (i = 0; i < bytes/8; i++)
    wbuf[i] = i + off;
    }
    }
// Align to 2MB boundary:
    buf = (void *)(((unsigned long)buf + HPSIZE-1) & ~(HPSIZE-1));
// Restore affinity:
    if (init_cpu0) {
    bind_to_cpumask(orig_mask);
    CPU_FREE(orig_mask);
    mempol_restore();
    }
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn free_data(data: *mut c_void, bytes: isize) {
    static void free_data(void *data, ssize_t bytes)
    {
    int ret;
    if (!data)
    return;
    ret = munmap(data, bytes);
    BUG_ON(ret);
    }
//
// Create a shared memory buffer that can be shared between processes, zeroed:
//
#[no_mangle]
unsafe extern "C" fn zalloc_shared_data(bytes: isize) -> *mut c_void {
    static void * zalloc_shared_data(ssize_t bytes)
    {
    return alloc_data(bytes, MAP_SHARED, 1, g.p.init_cpu0,  g.p.thp, g.p.init_random);
    }
//
// Create a shared memory buffer that can be shared between processes:
//
#[no_mangle]
unsafe extern "C" fn setup_shared_data(bytes: isize) -> *mut c_void {
    static void * setup_shared_data(ssize_t bytes)
    {
    return alloc_data(bytes, MAP_SHARED, 0, g.p.init_cpu0,  g.p.thp, g.p.init_random);
    }
//
// Allocate process-local memory - this will either be shared between
// threads of this process, or only be accessed by this thread:
//
#[no_mangle]
unsafe extern "C" fn setup_private_data(bytes: isize) -> *mut c_void {
    static void * setup_private_data(ssize_t bytes)
    {
    return alloc_data(bytes, MAP_PRIVATE, 0, g.p.init_cpu0,  g.p.thp, g.p.init_random);
    }
#[no_mangle]
unsafe extern "C" fn parse_cpu_list(arg: *const c_char) -> c_int {
    static int parse_cpu_list(const char *arg)
    {
    p0.cpu_list_str = strdup(arg);
    dprintf("got CPU list: {%s}\n", p0.cpu_list_str);
    return 0;
    }
//
// Check whether a CPU is online
//
// Returns:
// 1 -> if CPU is online
// 0 -> if CPU is offline
// -1 -> error case
//
#[no_mangle]
unsafe extern "C" fn is_cpu_online(cpu: c_uint) -> c_int {
    static int is_cpu_online(unsigned int cpu)
    {
    char *str;
    size_t strlen;
    char buf[256];
    let mut status: c_int = -1;
    struct stat statbuf;
    snprintf(buf, sizeof(buf),
    "/sys/devices/system/cpu/cpu%d", cpu);
    if (stat(buf, &statbuf) != 0)
    return 0;
//
// Check if /sys/devices/system/cpu/cpux/online file
// exists. Some cases cpu0 won't have online file since
// it is not expected to be turned off generally.
// In kernels without CONFIG_HOTPLUG_CPU, this
// file won't exist
//
    snprintf(buf, sizeof(buf),
    "/sys/devices/system/cpu/cpu%d/online", cpu);
    if (stat(buf, &statbuf) != 0)
    return 1;
//
// Read online file using sysfs__read_str.
// If read or open fails, return -1.
// If read succeeds, return value from file
// which gets stored in "str"
//
    snprintf(buf, sizeof(buf),
    "devices/system/cpu/cpu%d/online", cpu);
    if (sysfs__read_str(buf, &str, &strlen) < 0)
    return status;
    status = atoi(str);
    free(str);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn parse_setup_cpu_list() -> c_int {
    static int parse_setup_cpu_list(void)
    {
    struct thread_data *td;
    char *str0, *str;
    int t;
    if (!g.p.cpu_list_str)
    return 0;
    dprintf("g.p.nr_tasks: %d\n", g.p.nr_tasks);
    str0 = str = strdup(g.p.cpu_list_str);
    t = 0;
    BUG_ON(!str);
    tprintf("# binding tasks to CPUs:\n");
    tprintf("#  ");
    while (true) {
    int bind_cpu, bind_cpu_0, bind_cpu_1;
    char *tok, *tok_end, *tok_step, *tok_len, *tok_mul;
    int bind_len;
    int step;
    int mul;
    tok = strsep(&str, ",");
    if (!tok)
    break;
    tok_end = strstr(tok, "-");
    dprintf("\ntoken: {%s}, end: {%s}\n", tok, tok_end);
    if (!tok_end) {
// Single CPU specified:
    bind_cpu_0 = bind_cpu_1 = atol(tok);
    } else {
// CPU range specified (for example: "5-11"):
    bind_cpu_0 = atol(tok);
    bind_cpu_1 = atol(tok_end + 1);
    }
    step = 1;
    tok_step = strstr(tok, "#");
    if (tok_step) {
    step = atol(tok_step + 1);
    BUG_ON(step <= 0 || step >= g.p.nr_cpus);
    }
//
// Mask length.
// Eg: "--cpus 8_4-16#4" means: '--cpus 8_4,12_4,16_4',
// where the _4 means the next 4 CPUs are allowed.
//
    bind_len = 1;
    tok_len = strstr(tok, "_");
    if (tok_len) {
    bind_len = atol(tok_len + 1);
    BUG_ON(bind_len <= 0 || bind_len > g.p.nr_cpus);
    }
// Multiplicator shortcut, "0x8" is a shortcut for: "0,0,0,0,0,0,0,0"
    mul = 1;
    tok_mul = strstr(tok, "x");
    if (tok_mul) {
    mul = atol(tok_mul + 1);
    BUG_ON(mul <= 0);
    }
    dprintf("CPUs: %d_%d-%d#%dx%d\n", bind_cpu_0, bind_len, bind_cpu_1, step, mul);
    if (bind_cpu_0 >= g.p.nr_cpus || bind_cpu_1 >= g.p.nr_cpus) {
    printf("\nTest not applicable, system has only %d CPUs.\n", g.p.nr_cpus);
    return -1;
    }
    if (is_cpu_online(bind_cpu_0) != 1 || is_cpu_online(bind_cpu_1) != 1) {
    printf("\nTest not applicable, bind_cpu_0 or bind_cpu_1 is offline\n");
    return -1;
    }
    BUG_ON(bind_cpu_0 < 0 || bind_cpu_1 < 0);
    BUG_ON(bind_cpu_0 > bind_cpu_1);
    for (bind_cpu = bind_cpu_0; bind_cpu <= bind_cpu_1; bind_cpu += step) {
    let mut size: usize = CPU_ALLOC_SIZE(g.p.nr_cpus);
    int i;
    for (i = 0; i < mul; i++) {
    int cpu;
    if (t >= g.p.nr_tasks) {
    printf("\n# NOTE: ignoring bind CPUs starting at CPU#%d\n #", bind_cpu);
    goto out;
    }
    td = g.threads + t;
    if (t)
    tprintf(",");
    if (bind_len > 1) {
    tprintf("%2d/%d", bind_cpu, bind_len);
    } else {
    tprintf("%2d", bind_cpu);
    }
    td.bind_cpumask = CPU_ALLOC(g.p.nr_cpus);
    BUG_ON(!td.bind_cpumask);
    CPU_ZERO_S(size, td.bind_cpumask);
    for (cpu = bind_cpu; cpu < bind_cpu+bind_len; cpu++) {
    if (cpu < 0 || cpu >= g.p.nr_cpus) {
    CPU_FREE(td.bind_cpumask);
    BUG_ON(-1);
    }
    CPU_SET_S(cpu, size, td.bind_cpumask);
    }
    t++;
    }
    }
    }
    out:
    tprintf("\n");
    if (t < g.p.nr_tasks)
    printf("# NOTE: %d tasks bound, %d tasks unbound\n", t, g.p.nr_tasks - t);
    free(str0);
    return 0;
    }
    static int parse_cpus_opt(const struct option *opt __maybe_unused,
    const char *arg, int unset __maybe_unused)
    {
    if (!arg)
    return -1;
    return parse_cpu_list(arg);
    }
#[no_mangle]
unsafe extern "C" fn parse_node_list(arg: *const c_char) -> c_int {
    static int parse_node_list(const char *arg)
    {
    p0.node_list_str = strdup(arg);
    dprintf("got NODE list: {%s}\n", p0.node_list_str);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parse_setup_node_list() -> c_int {
    static int parse_setup_node_list(void)
    {
    struct thread_data *td;
    char *str0, *str;
    int t;
    if (!g.p.node_list_str)
    return 0;
    dprintf("g.p.nr_tasks: %d\n", g.p.nr_tasks);
    str0 = str = strdup(g.p.node_list_str);
    t = 0;
    BUG_ON(!str);
    tprintf("# binding tasks to NODEs:\n");
    tprintf("# ");
    while (true) {
    int bind_node, bind_node_0, bind_node_1;
    char *tok, *tok_end, *tok_step, *tok_mul;
    int step;
    int mul;
    tok = strsep(&str, ",");
    if (!tok)
    break;
    tok_end = strstr(tok, "-");
    dprintf("\ntoken: {%s}, end: {%s}\n", tok, tok_end);
    if (!tok_end) {
// Single NODE specified:
    bind_node_0 = bind_node_1 = atol(tok);
    } else {
// NODE range specified (for example: "5-11"):
    bind_node_0 = atol(tok);
    bind_node_1 = atol(tok_end + 1);
    }
    step = 1;
    tok_step = strstr(tok, "#");
    if (tok_step) {
    step = atol(tok_step + 1);
    BUG_ON(step <= 0 || step >= g.p.nr_nodes);
    }
// Multiplicator shortcut, "0x8" is a shortcut for: "0,0,0,0,0,0,0,0"
    mul = 1;
    tok_mul = strstr(tok, "x");
    if (tok_mul) {
    mul = atol(tok_mul + 1);
    BUG_ON(mul <= 0);
    }
    dprintf("NODEs: %d-%d #%d\n", bind_node_0, bind_node_1, step);
    if (bind_node_0 >= g.p.nr_nodes || bind_node_1 >= g.p.nr_nodes) {
    printf("\nTest not applicable, system has only %d nodes.\n", g.p.nr_nodes);
    return -1;
    }
    BUG_ON(bind_node_0 < 0 || bind_node_1 < 0);
    BUG_ON(bind_node_0 > bind_node_1);
    for (bind_node = bind_node_0; bind_node <= bind_node_1; bind_node += step) {
    int i;
    for (i = 0; i < mul; i++) {
    if (t >= g.p.nr_tasks || !node_has_cpus(bind_node)) {
    printf("\n# NOTE: ignoring bind NODEs starting at NODE#%d\n", bind_node);
    goto out;
    }
    td = g.threads + t;
    if (!t)
    tprintf(" %2d", bind_node);
    else
    tprintf(",%2d", bind_node);
    td.bind_node = bind_node;
    t++;
    }
    }
    }
    out:
    tprintf("\n");
    if (t < g.p.nr_tasks)
    printf("# NOTE: %d tasks mem-bound, %d tasks unbound\n", t, g.p.nr_tasks - t);
    free(str0);
    return 0;
    }
    static int parse_nodes_opt(const struct option *opt __maybe_unused,
    const char *arg, int unset __maybe_unused)
    {
    if (!arg)
    return -1;
    return parse_node_list(arg);
    }
#[no_mangle]
pub unsafe extern "C" fn lfsr_32(lfsr: u32) -> u32 {
    static inline uint32_t lfsr_32(uint32_t lfsr)
    {
    let mut taps: u32 = BIT(1) | BIT(5) | BIT(6) | BIT(31);
    return (lfsr>>1) ^ ((0x0u - (lfsr & 0x1u)) & taps);
    }
//
// Make sure there's real data dependency to RAM (when read
// accesses are enabled), so the compiler, the CPU and the
// kernel (KSM, zero page, etc.) cannot optimize away RAM
// accesses:
//
#[no_mangle]
pub unsafe extern "C" fn access_data(data: *mut u64, val: u64) -> u64 {
    static inline u64 access_data(u64 *data, u64 val)
    {
    if (g.p.data_reads)
    val += *data;
    if (g.p.data_writes)
// data = val + 1;
    return val;
    }
//
// The worker process does two types of work, a forwards going
// loop and a backwards going loop.
//
// We do this so that on multiprocessor systems we do not create
// a 'train' of processing, with highly synchronized processes,
// skewing the whole benchmark.
//
#[no_mangle]
unsafe extern "C" fn do_work(__data: *mut u8, bytes: c_long, nr: c_int, nr_max: c_int, loop: c_int, val: u64) -> u64 {
    static u64 do_work(u8 *__data, long bytes, int nr, int nr_max, int loop, u64 val)
    {
    let mut words: c_long = bytes/sizeof(u64);
    u64 *data = (void *)__data;
    long chunk_0, chunk_1;
    u64 *d0, *d, *d1;
    long off;
    long i;
    BUG_ON(!data && words);
    BUG_ON(data && !words);
    if (!data)
    return val;
// Very simple memset() work variant:
    if (g.p.data_zero_memset && !g.p.data_rand_walk) {
    bzero(data, bytes);
    return val;
    }
// Spread out by PID/TID nr and by loop nr:
    chunk_0 = words/nr_max;
    chunk_1 = words/g.p.nr_loops;
    off = nr*chunk_0 + loop*chunk_1;
    while (off >= words)
    off -= words;
    if (g.p.data_rand_walk) {
    let mut lfsr: u32 = nr + loop + val;
    long j;
    for (i = 0; i < words/1024; i++) {
    long start, end;
    lfsr = lfsr_32(lfsr);
    start = lfsr % words;
    end = min(start + 1024, words-1);
    if (g.p.data_zero_memset) {
    bzero(data + start, (end-start) * sizeof(u64));
    } else {
    for (j = start; j < end; j++)
    val = access_data(data + j, val);
    }
    }
    } else if (!g.p.data_backwards || (nr + loop) & 1) {
// Process data forwards:
    d0 = data + off;
    d  = data + off + 1;
    d1 = data + words;
    for (;;) {
    if (unlikely(d >= d1))
    d = data;
    if (unlikely(d == d0))
    break;
    val = access_data(d, val);
    d++;
    }
    } else {
// Process data backwards:
    d0 = data + off;
    d  = data + off - 1;
    d1 = data + words;
    for (;;) {
    if (unlikely(d < data))
    d = data + words-1;
    if (unlikely(d == d0))
    break;
    val = access_data(d, val);
    d--;
    }
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn update_curr_cpu(task_nr: c_int, bytes_worked: c_ulong) {
    static void update_curr_cpu(int task_nr, unsigned long bytes_worked)
    {
    unsigned int cpu;
    cpu = sched_getcpu();
    g.threads[task_nr].curr_cpu = cpu;
    prctl(0, bytes_worked);
    }
//
// Count the number of nodes a process's threads
// are spread out on.
//
// A count of 1 means that the process is compressed
// to a single node. A count of g->p.nr_nodes means it's
// spread out on the whole system.
//
#[no_mangle]
unsafe extern "C" fn count_process_nodes(process_nr: c_int) -> c_int {
    static int count_process_nodes(int process_nr)
    {
    char *node_present;
    int nodes;
    int n, t;
    node_present = calloc(g.p.nr_nodes, sizeof(char));
    BUG_ON(!node_present);
    for (t = 0; t < g.p.nr_threads; t++) {
    struct thread_data *td;
    int task_nr;
    int node;
    task_nr = process_nr*g.p.nr_threads + t;
    td = g.threads + task_nr;
    node = numa_node_of_cpu(td.curr_cpu);
    if (node < 0) /* curr_cpu was likely still -1 */ {
    free(node_present);
    return 0;
    }
    node_present[node] = 1;
    }
    nodes = 0;
    for (n = 0; n < g.p.nr_nodes; n++)
    nodes += node_present[n];
    free(node_present);
    return nodes;
    }
//
// Count the number of distinct process-threads a node contains.
//
// A count of 1 means that the node contains only a single
// process. If all nodes on the system contain at most one
// process then we are well-converged.
//
#[no_mangle]
unsafe extern "C" fn count_node_processes(node: c_int) -> c_int {
    static int count_node_processes(int node)
    {
    let mut processes: c_int = 0;
    int t, p;
    for (p = 0; p < g.p.nr_proc; p++) {
    for (t = 0; t < g.p.nr_threads; t++) {
    struct thread_data *td;
    int task_nr;
    int n;
    task_nr = p*g.p.nr_threads + t;
    td = g.threads + task_nr;
    n = numa_node_of_cpu(td.curr_cpu);
    if (n == node) {
    processes++;
    break;
    }
    }
    }
    return processes;
    }
#[no_mangle]
unsafe extern "C" fn calc_convergence_compression(strong: *mut c_int) {
    static void calc_convergence_compression(int *strong)
    {
    unsigned int nodes_min, nodes_max;
    int p;
    nodes_min = -1;
    nodes_max =  0;
    for (p = 0; p < g.p.nr_proc; p++) {
    let mut nodes: c_uint = count_process_nodes(p);
    if (!nodes) {
// strong = 0;
    return;
    }
    nodes_min = min(nodes, nodes_min);
    nodes_max = max(nodes, nodes_max);
    }
// Strong convergence: all threads compress on a single node:
    if (nodes_min == 1 && nodes_max == 1) {
// strong = 1;
    } else {
// strong = 0;
    tprintf(" {%d-%d}", nodes_min, nodes_max);
    }
    }
#[no_mangle]
unsafe extern "C" fn calc_convergence(runtime_ns_max: double, convergence: *mut double) {
    static void calc_convergence(double runtime_ns_max, double *convergence)
    {
    unsigned int loops_done_min, loops_done_max;
    int process_groups;
    int *nodes;
    int distance;
    int nr_min;
    int nr_max;
    int strong;
    int sum;
    int nr;
    int node;
    int cpu;
    int t;
    if (!g.p.show_convergence && !g.p.measure_convergence)
    return;
    nodes = calloc(g.p.nr_nodes, sizeof(int));
    BUG_ON(!nodes);
    loops_done_min = -1;
    loops_done_max = 0;
    for (t = 0; t < g.p.nr_tasks; t++) {
    struct thread_data *td = g.threads + t;
    unsigned int loops_done;
    cpu = td.curr_cpu;
// Not all threads have written it yet:
    if (cpu < 0)
    continue;
    node = numa_node_of_cpu(cpu);
    nodes[node]++;
    loops_done = td.loops_done;
    loops_done_min = min(loops_done, loops_done_min);
    loops_done_max = max(loops_done, loops_done_max);
    }
    nr_max = 0;
    nr_min = g.p.nr_tasks;
    sum = 0;
    for (node = 0; node < g.p.nr_nodes; node++) {
    if (!is_node_present(node))
    continue;
    nr = nodes[node];
    nr_min = min(nr, nr_min);
    nr_max = max(nr, nr_max);
    sum += nr;
    }
    BUG_ON(nr_min > nr_max);
    BUG_ON(sum > g.p.nr_tasks);
    if (0 && (sum < g.p.nr_tasks)) {
    free(nodes);
    return;
    }
//
// Count the number of distinct process groups present
// on nodes - when we are converged this will decrease
// to g->p.nr_proc:
//
    process_groups = 0;
    for (node = 0; node < g.p.nr_nodes; node++) {
    int processes;
    if (!is_node_present(node))
    continue;
    processes = count_node_processes(node);
    nr = nodes[node];
    tprintf(" %2d/%-2d", nr, processes);
    process_groups += processes;
    }
    distance = nr_max - nr_min;
    tprintf(" [%2d/%-2d]", distance, process_groups);
    tprintf(" l:%3d-%-3d (%3d)",
    loops_done_min, loops_done_max, loops_done_max-loops_done_min);
    if (loops_done_min && loops_done_max) {
    let mut skew: double = 1.0 - (double)loops_done_min/loops_done_max;
    tprintf(" [%4.1f%%]", skew * 100.0);
    }
    calc_convergence_compression(&strong);
    if (strong && process_groups == g.p.nr_proc) {
    if (!*convergence) {
// convergence = runtime_ns_max;
    tprintf(" (%6.1fs converged)\n", *convergence / NSEC_PER_SEC);
    if (g.p.measure_convergence) {
    g.all_converged = true;
    g.stop_work = true;
    }
    }
    } else {
    if (*convergence) {
    tprintf(" (%6.1fs de-converged)", runtime_ns_max / NSEC_PER_SEC);
// convergence = 0;
    }
    tprintf("\n");
    }
    free(nodes);
    }
#[no_mangle]
unsafe extern "C" fn show_summary(runtime_ns_max: double, l: c_int, convergence: *mut double) {
    static void show_summary(double runtime_ns_max, int l, double *convergence)
    {
    tprintf("\r #  %5.1f%%  [%.1f mins]",
    (double)(l+1)/g.p.nr_loops*100.0, runtime_ns_max / NSEC_PER_SEC / 60.0);
    calc_convergence(runtime_ns_max, convergence);
    if (g.p.show_details >= 0)
    fflush(stdout);
    }
    static void *worker_thread(void *__tdata)
    {
    struct thread_data *td = __tdata;
    struct timeval start0, start, stop, diff;
    let mut process_nr: c_int = td.process_nr;
    let mut thread_nr: c_int = td.thread_nr;
    unsigned long last_perturbance;
    let mut task_nr: c_int = td.task_nr;
    let mut details: c_int = g.p.show_details;
    int first_task, last_task;
    let mut convergence: double = 0;
    let mut val: u64 = td.val;
    double runtime_ns_max;
    u8 *global_data;
    u8 *process_data;
    u8 *thread_data;
    u64 bytes_done, secs;
    long work_done;
    u32 l;
    struct rusage rusage;
    bind_to_cpumask(td.bind_cpumask);
    bind_to_memnode(td.bind_node);
    set_taskname("thread %d/%d", process_nr, thread_nr);
    global_data = g.data;
    process_data = td.process_data;
    thread_data = setup_private_data(g.p.bytes_thread);
    bytes_done = 0;
    last_task = 0;
    if (process_nr == g.p.nr_proc-1 && thread_nr == g.p.nr_threads-1)
    last_task = 1;
    first_task = 0;
    if (process_nr == 0 && thread_nr == 0)
    first_task = 1;
    if (details >= 2) {
    printf("#  thread %2d / %2d global mem: %p, process mem: %p, thread mem: %p\n",
    process_nr, thread_nr, global_data, process_data, thread_data);
    }
    if (g.p.serialize_startup) {
    mutex_lock(&g.startup_mutex);
    g.nr_tasks_started++;
// The last thread wakes the main process.
    if (g.nr_tasks_started == g.p.nr_tasks)
    cond_signal(&g.startup_cond);
    mutex_unlock(&g.startup_mutex);
// Here we will wait for the main process to start us all at once:
    mutex_lock(&g.start_work_mutex);
    g.start_work = false;
    g.nr_tasks_working++;
    while (!g.start_work)
    cond_wait(&g.start_work_cond, &g.start_work_mutex);
    mutex_unlock(&g.start_work_mutex);
    }
    gettimeofday(&start0, core::ptr::null_mut());
    start = stop = start0;
    last_perturbance = start.tv_sec;
    for (l = 0; l < g.p.nr_loops; l++) {
    start = stop;
    if (g.stop_work)
    break;
    val += do_work(global_data,  g.p.bytes_global,  process_nr, g.p.nr_proc,	l, val);
    val += do_work(process_data, g.p.bytes_process, thread_nr,  g.p.nr_threads,	l, val);
    val += do_work(thread_data,  g.p.bytes_thread,  0,          1,		l, val);
    if (g.p.sleep_usecs) {
    mutex_lock(td.process_lock);
    usleep(g.p.sleep_usecs);
    mutex_unlock(td.process_lock);
    }
//
// Amount of work to be done under a process-global lock:
//
    if (g.p.bytes_process_locked) {
    mutex_lock(td.process_lock);
    val += do_work(process_data, g.p.bytes_process_locked, thread_nr,  g.p.nr_threads,	l, val);
    mutex_unlock(td.process_lock);
    }
    work_done = g.p.bytes_global + g.p.bytes_process +
    g.p.bytes_process_locked + g.p.bytes_thread;
    update_curr_cpu(task_nr, work_done);
    bytes_done += work_done;
    if (details < 0 && !g.p.perturb_secs && !g.p.measure_convergence && !g.p.nr_secs)
    continue;
    td.loops_done = l;
    gettimeofday(&stop, core::ptr::null_mut());
// Check whether our max runtime timed out:
    if (g.p.nr_secs) {
    timersub(&stop, &start0, &diff);
    if ((u32)diff.tv_sec >= g.p.nr_secs) {
    g.stop_work = true;
    break;
    }
    }
// Update the summary at most once per second:
    if (start.tv_sec == stop.tv_sec)
    continue;
//
// Perturb the first task's equilibrium every g->p.perturb_secs seconds,
// by migrating to CPU#0:
//
    if (first_task && g.p.perturb_secs && (int)(stop.tv_sec - last_perturbance) >= g.p.perturb_secs) {
    cpu_set_t *orig_mask;
    int target_cpu;
    int this_cpu;
    last_perturbance = stop.tv_sec;
//
// Depending on where we are running, move into
// the other half of the system, to create some
// real disturbance:
//
    this_cpu = g.threads[task_nr].curr_cpu;
    if (this_cpu < g.p.nr_cpus/2)
    target_cpu = g.p.nr_cpus-1;
    else
    target_cpu = 0;
    orig_mask = bind_to_cpu(target_cpu);
// Here we are running on the target CPU already
    if (details >= 1)
    printf(" (injecting perturbalance, moved to CPU#%d)\n", target_cpu);
    bind_to_cpumask(orig_mask);
    CPU_FREE(orig_mask);
    }
    if (details >= 3) {
    timersub(&stop, &start, &diff);
    runtime_ns_max = diff.tv_sec * NSEC_PER_SEC;
    runtime_ns_max += diff.tv_usec * NSEC_PER_USEC;
    if (details >= 0) {
    printf(" #%2d / %2d: %14.2lf nsecs/op [val: %016"PRIx64"]\n",
    process_nr, thread_nr, runtime_ns_max / bytes_done, val);
    }
    fflush(stdout);
    }
    if (!last_task)
    continue;
    timersub(&stop, &start0, &diff);
    runtime_ns_max = diff.tv_sec * NSEC_PER_SEC;
    runtime_ns_max += diff.tv_usec * NSEC_PER_USEC;
    show_summary(runtime_ns_max, l, &convergence);
    }
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start0, &diff);
    td.runtime_ns = diff.tv_sec * NSEC_PER_SEC;
    td.runtime_ns += diff.tv_usec * NSEC_PER_USEC;
    secs = td.runtime_ns / NSEC_PER_SEC;
    td.speed_gbs = secs ? bytes_done / secs / 1e9 : 0;
    getrusage(RUSAGE_THREAD, &rusage);
    td.system_time_ns = rusage.ru_stime.tv_sec * NSEC_PER_SEC;
    td.system_time_ns += rusage.ru_stime.tv_usec * NSEC_PER_USEC;
    td.user_time_ns = rusage.ru_utime.tv_sec * NSEC_PER_SEC;
    td.user_time_ns += rusage.ru_utime.tv_usec * NSEC_PER_USEC;
    free_data(thread_data, g.p.bytes_thread);
    mutex_lock(&g.stop_work_mutex);
    g.bytes_done += bytes_done;
    mutex_unlock(&g.stop_work_mutex);
    return core::ptr::null_mut();
    }
//
// A worker process starts a couple of threads:
//
#[no_mangle]
unsafe extern "C" fn worker_process(process_nr: c_int) {
    static void worker_process(int process_nr)
    {
    struct mutex process_lock;
    struct thread_data *td;
    pthread_t *pthreads;
    u8 *process_data;
    int task_nr;
    int ret;
    int t;
    mutex_init(&process_lock);
    set_taskname("process %d", process_nr);
//
// Pick up the memory policy and the CPU binding of our first thread,
// so that we initialize memory accordingly:
//
    task_nr = process_nr*g.p.nr_threads;
    td = g.threads + task_nr;
    bind_to_memnode(td.bind_node);
    bind_to_cpumask(td.bind_cpumask);
    pthreads = calloc(g.p.nr_threads, sizeof(pthread_t));
    process_data = setup_private_data(g.p.bytes_process);
    if (g.p.show_details >= 3) {
    printf(" # process %2d global mem: %p, process mem: %p\n",
    process_nr, g.data, process_data);
    }
    for (t = 0; t < g.p.nr_threads; t++) {
    task_nr = process_nr*g.p.nr_threads + t;
    td = g.threads + task_nr;
    td.process_data = process_data;
    td.process_nr   = process_nr;
    td.thread_nr    = t;
    td.task_nr	 = task_nr;
    td.val          = rand();
    td.curr_cpu	 = -1;
    td.process_lock = &process_lock;
    ret = pthread_create(pthreads + t, core::ptr::null_mut(), worker_thread, td);
    BUG_ON(ret);
    }
    for (t = 0; t < g.p.nr_threads; t++) {
    ret = pthread_join(pthreads[t], core::ptr::null_mut());
    BUG_ON(ret);
    }
    free_data(process_data, g.p.bytes_process);
    free(pthreads);
    }
#[no_mangle]
unsafe extern "C" fn print_summary() {
    static void print_summary(void)
    {
    if (g.p.show_details < 0)
    return;
    printf("\n ###\n");
    printf(" # %d %s will execute (on %d nodes, %d CPUs):\n",
    g.p.nr_tasks, g.p.nr_tasks == 1 ? "task" : "tasks", nr_numa_nodes(), g.p.nr_cpus);
    printf(" #      %5dx %5ldMB global  shared mem operations\n",
    g.p.nr_loops, g.p.bytes_global/1024/1024);
    printf(" #      %5dx %5ldMB process shared mem operations\n",
    g.p.nr_loops, g.p.bytes_process/1024/1024);
    printf(" #      %5dx %5ldMB thread  local  mem operations\n",
    g.p.nr_loops, g.p.bytes_thread/1024/1024);
    printf(" ###\n");
    printf("\n ###\n"); fflush(stdout);
    }
#[no_mangle]
unsafe extern "C" fn init_thread_data() {
    static void init_thread_data(void)
    {
    let mut size: isize = sizeof(*g.threads)*g.p.nr_tasks;
    int t;
    g.threads = zalloc_shared_data(size);
    for (t = 0; t < g.p.nr_tasks; t++) {
    struct thread_data *td = g.threads + t;
    let mut cpuset_size: usize = CPU_ALLOC_SIZE(g.p.nr_cpus);
    int cpu;
// Allow all nodes by default:
    td.bind_node = NUMA_NO_NODE;
// Allow all CPUs by default:
    td.bind_cpumask = CPU_ALLOC(g.p.nr_cpus);
    BUG_ON(!td.bind_cpumask);
    CPU_ZERO_S(cpuset_size, td.bind_cpumask);
    for (cpu = 0; cpu < g.p.nr_cpus; cpu++)
    CPU_SET_S(cpu, cpuset_size, td.bind_cpumask);
    }
    }
#[no_mangle]
unsafe extern "C" fn deinit_thread_data() {
    static void deinit_thread_data(void)
    {
    let mut size: isize = sizeof(*g.threads)*g.p.nr_tasks;
    int t;
// Free the bind_cpumask allocated for thread_data
    for (t = 0; t < g.p.nr_tasks; t++) {
    struct thread_data *td = g.threads + t;
    CPU_FREE(td.bind_cpumask);
    }
    free_data(g.threads, size);
    }
#[no_mangle]
unsafe extern "C" fn init() -> c_int {
    static int init(void)
    {
    g = (void *)alloc_data(sizeof(*g), MAP_SHARED, 1, 0, 0 /* THP */, 0);
// Copy over options:
    g.p = p0;
    g.p.nr_cpus = numa_num_configured_cpus();
    g.p.nr_nodes = numa_max_node() + 1;
// char array in count_process_nodes():
    BUG_ON(g.p.nr_nodes < 0);
    if (quiet && !g.p.show_details)
    g.p.show_details = -1;
// Some memory should be specified:
    if (!g.p.mb_global_str && !g.p.mb_proc_str && !g.p.mb_thread_str)
    return -1;
    if (g.p.mb_global_str) {
    g.p.mb_global = atof(g.p.mb_global_str);
    BUG_ON(g.p.mb_global < 0);
    }
    if (g.p.mb_proc_str) {
    g.p.mb_proc = atof(g.p.mb_proc_str);
    BUG_ON(g.p.mb_proc < 0);
    }
    if (g.p.mb_proc_locked_str) {
    g.p.mb_proc_locked = atof(g.p.mb_proc_locked_str);
    BUG_ON(g.p.mb_proc_locked < 0);
    BUG_ON(g.p.mb_proc_locked > g.p.mb_proc);
    }
    if (g.p.mb_thread_str) {
    g.p.mb_thread = atof(g.p.mb_thread_str);
    BUG_ON(g.p.mb_thread < 0);
    }
    BUG_ON(g.p.nr_threads <= 0);
    BUG_ON(g.p.nr_proc <= 0);
    g.p.nr_tasks = g.p.nr_proc*g.p.nr_threads;
    g.p.bytes_global		= g.p.mb_global	*1024L*1024L;
    g.p.bytes_process		= g.p.mb_proc		*1024L*1024L;
    g.p.bytes_process_locked	= g.p.mb_proc_locked	*1024L*1024L;
    g.p.bytes_thread		= g.p.mb_thread	*1024L*1024L;
    g.data = setup_shared_data(g.p.bytes_global);
// Startup serialization:
    mutex_init_pshared(&g.start_work_mutex);
    cond_init_pshared(&g.start_work_cond);
    mutex_init_pshared(&g.startup_mutex);
    cond_init_pshared(&g.startup_cond);
    mutex_init_pshared(&g.stop_work_mutex);
    init_thread_data();
    tprintf("#\n");
    if (parse_setup_cpu_list() || parse_setup_node_list())
    return -1;
    tprintf("#\n");
    print_summary();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn deinit() {
    static void deinit(void)
    {
    free_data(g.data, g.p.bytes_global);
    g.data = core::ptr::null_mut();
    deinit_thread_data();
    free_data(g, sizeof(*g));
    g = core::ptr::null_mut();
    }
//
// Print a short or long result, depending on the verbosity setting:
//
    static void print_res(const char *name, double val,
    const char *txt_unit, const char *txt_short, const char *txt_long)
    {
    if (!name)
    name = "main,";
    if (!quiet)
    printf(" %-30s %15.3f, %-15s %s\n", name, val, txt_unit, txt_short);
    else
    printf(" %14.3f %s\n", val, txt_long);
    }
#[no_mangle]
unsafe extern "C" fn __bench_numa(name: *const c_char) -> c_int {
    static int __bench_numa(const char *name)
    {
    struct timeval start, stop, diff;
    u64 runtime_ns_min, runtime_ns_sum;
    pid_t *pids, pid, wpid;
    double delta_runtime;
    double runtime_avg;
    double runtime_sec_max;
    double runtime_sec_min;
    int wait_stat;
    double bytes;
    int i, t, p;
    if (init())
    return -1;
    pids = calloc(g.p.nr_proc, sizeof(*pids));
    pid = -1;
    if (g.p.serialize_startup) {
    tprintf(" #\n");
    tprintf(" # Startup synchronization: ..."); fflush(stdout);
    }
    gettimeofday(&start, core::ptr::null_mut());
    for (i = 0; i < g.p.nr_proc; i++) {
    pid = fork();
    dprintf(" # process %2d: PID %d\n", i, pid);
    BUG_ON(pid < 0);
    if (!pid) {
// Child process:
    worker_process(i);
    exit(0);
    }
    pids[i] = pid;
    }
    if (g.p.serialize_startup) {
    let mut threads_ready: bool = false;
    double startup_sec;
//
// Wait for all the threads to start up. The last thread will
// signal this process.
//
    mutex_lock(&g.startup_mutex);
    while (g.nr_tasks_started != g.p.nr_tasks)
    cond_wait(&g.startup_cond, &g.startup_mutex);
    mutex_unlock(&g.startup_mutex);
// Wait for all threads to be at the start_work_cond.
    while (!threads_ready) {
    mutex_lock(&g.start_work_mutex);
    threads_ready = (g.nr_tasks_working == g.p.nr_tasks);
    mutex_unlock(&g.start_work_mutex);
    if (!threads_ready)
    usleep(1);
    }
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start, &diff);
    startup_sec = diff.tv_sec * NSEC_PER_SEC;
    startup_sec += diff.tv_usec * NSEC_PER_USEC;
    startup_sec /= NSEC_PER_SEC;
    tprintf(" threads initialized in %.6f seconds.\n", startup_sec);
    tprintf(" #\n");
    start = stop;
// Start all threads running.
    mutex_lock(&g.start_work_mutex);
    g.start_work = true;
    mutex_unlock(&g.start_work_mutex);
    cond_broadcast(&g.start_work_cond);
    } else {
    gettimeofday(&start, core::ptr::null_mut());
    }
// Parent process:
    for (i = 0; i < g.p.nr_proc; i++) {
    wpid = waitpid(pids[i], &wait_stat, 0);
    BUG_ON(wpid < 0);
    BUG_ON(!WIFEXITED(wait_stat));
    }
    runtime_ns_sum = 0;
    runtime_ns_min = -1LL;
    for (t = 0; t < g.p.nr_tasks; t++) {
    let mut thread_runtime_ns: u64 = g.threads[t].runtime_ns;
    runtime_ns_sum += thread_runtime_ns;
    runtime_ns_min = min(thread_runtime_ns, runtime_ns_min);
    }
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start, &diff);
    BUG_ON(bench_format != BENCH_FORMAT_DEFAULT);
    tprintf("\n ###\n");
    tprintf("\n");
    runtime_sec_max = diff.tv_sec * NSEC_PER_SEC;
    runtime_sec_max += diff.tv_usec * NSEC_PER_USEC;
    runtime_sec_max /= NSEC_PER_SEC;
    runtime_sec_min = runtime_ns_min / NSEC_PER_SEC;
    bytes = g.bytes_done;
    runtime_avg = (double)runtime_ns_sum / g.p.nr_tasks / NSEC_PER_SEC;
    if (g.p.measure_convergence) {
    print_res(name, runtime_sec_max,
    "secs,", "NUMA-convergence-latency", "secs latency to NUMA-converge");
    }
    print_res(name, runtime_sec_max,
    "secs,", "runtime-max/thread",	"secs slowest (max) thread-runtime");
    print_res(name, runtime_sec_min,
    "secs,", "runtime-min/thread",	"secs fastest (min) thread-runtime");
    print_res(name, runtime_avg,
    "secs,", "runtime-avg/thread",	"secs average thread-runtime");
    delta_runtime = (runtime_sec_max - runtime_sec_min)/2.0;
    print_res(name, delta_runtime / runtime_sec_max * 100.0,
    "%,", "spread-runtime/thread",	"% difference between max/avg runtime");
    print_res(name, bytes / g.p.nr_tasks / 1e9,
    "GB,", "data/thread",		"GB data processed, per thread");
    print_res(name, bytes / 1e9,
    "GB,", "data-total",		"GB data processed, total");
    print_res(name, runtime_sec_max * NSEC_PER_SEC / (bytes / g.p.nr_tasks),
    "nsecs,", "runtime/byte/thread","nsecs/byte/thread runtime");
    print_res(name, bytes / g.p.nr_tasks / 1e9 / runtime_sec_max,
    "GB/sec,", "thread-speed",	"GB/sec/thread speed");
    print_res(name, bytes / runtime_sec_max / 1e9,
    "GB/sec,", "total-speed",	"GB/sec total speed");
    if (g.p.show_details >= 2) {
    char tname[14 + 2 * 11 + 1];
    struct thread_data *td;
    for (p = 0; p < g.p.nr_proc; p++) {
    for (t = 0; t < g.p.nr_threads; t++) {
    memset(tname, 0, sizeof(tname));
    td = g.threads + p*g.p.nr_threads + t;
    snprintf(tname, sizeof(tname), "process%d:thread%d", p, t);
    print_res(tname, td.speed_gbs,
    "GB/sec",	"thread-speed", "GB/sec/thread speed");
    print_res(tname, td.system_time_ns / NSEC_PER_SEC,
    "secs",	"thread-system-time", "system CPU time/thread");
    print_res(tname, td.user_time_ns / NSEC_PER_SEC,
    "secs",	"thread-user-time", "user CPU time/thread");
    }
    }
    }
    free(pids);
    deinit();
    return 0;
    }
pub const MAX_ARGS: c_int = 50;
#[no_mangle]
unsafe extern "C" fn command_size(argv: *const c_char) -> c_int {
    static int command_size(const char **argv)
    {
    let mut size: c_int = 0;
    while (*argv) {
    size++;
    argv++;
    }
    BUG_ON(size >= MAX_ARGS);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn init_params(p: *mut params, name: *const c_char, argc: c_int, argv: *const c_char) {
    static void init_params(struct params *p, const char *name, int argc, const char **argv)
    {
    int i;
    printf("\n # Running %s \"perf bench numa", name);
    for (i = 0; i < argc; i++)
    printf(" %s", argv[i]);
    printf("\"\n");
    memset(p, 0, sizeof(*p));
// Initialize nonzero defaults:
    p.serialize_startup		= 1;
    p.data_reads			= true;
    p.data_writes			= true;
    p.data_backwards		= true;
    p.data_rand_walk		= true;
    p.nr_loops			= -1;
    p.init_random			= true;
    p.mb_global_str		= "1";
    p.nr_proc			= 1;
    p.nr_threads			= 1;
    p.nr_secs			= 5;
    p.run_all			= argc == 1;
    }
#[no_mangle]
unsafe extern "C" fn run_bench_numa(name: *const c_char, argv: *const c_char) -> c_int {
    static int run_bench_numa(const char *name, const char **argv)
    {
    let mut argc: c_int = command_size(argv);
    init_params(&p0, name, argc, argv);
    argc = parse_options(argc, argv, options, bench_numa_usage, 0);
    if (argc)
    goto err;
    if (__bench_numa(name))
    goto err;
    return 0;
    err:
    return -1;
    }

//
// The built-in test-suite executed by "perf bench numa -a".
//
// (A minimum of 4 nodes and 16 GB of RAM is recommended.)
//
    static const char *tests[][MAX_ARGS] = {
// Basic single-stream NUMA bandwidth measurements:
    { "RAM-bw-local,",     "mem",  "-p",  "1",  "-t",  "1", "-P", "1024",
    "-C" ,   "0", "-M",   "0", OPT_BW_RAM },
    { "RAM-bw-local-NOTHP,",
    "mem",  "-p",  "1",  "-t",  "1", "-P", "1024",
    "-C" ,   "0", "-M",   "0", OPT_BW_RAM_NOTHP },
    { "RAM-bw-remote,",    "mem",  "-p",  "1",  "-t",  "1", "-P", "1024",
    "-C" ,   "0", "-M",   "1", OPT_BW_RAM },
// 2-stream NUMA bandwidth measurements:
    { "RAM-bw-local-2x,",  "mem",  "-p",  "2",  "-t",  "1", "-P", "1024",
    "-C", "0,2", "-M", "0x2", OPT_BW_RAM },
    { "RAM-bw-remote-2x,", "mem",  "-p",  "2",  "-t",  "1", "-P", "1024",
    "-C", "0,2", "-M", "1x2", OPT_BW_RAM },
// Cross-stream NUMA bandwidth measurement:
    { "RAM-bw-cross,",     "mem",  "-p",  "2",  "-t",  "1", "-P", "1024",
    "-C", "0,8", "-M", "1,0", OPT_BW_RAM },
// Convergence latency measurements:
    { " 1x3-convergence,", "mem",  "-p",  "1", "-t",  "3", "-P",  "512", OPT_CONV },
    { " 1x4-convergence,", "mem",  "-p",  "1", "-t",  "4", "-P",  "512", OPT_CONV },
    { " 1x6-convergence,", "mem",  "-p",  "1", "-t",  "6", "-P", "1020", OPT_CONV },
    { " 2x3-convergence,", "mem",  "-p",  "2", "-t",  "3", "-P", "1020", OPT_CONV },
    { " 3x3-convergence,", "mem",  "-p",  "3", "-t",  "3", "-P", "1020", OPT_CONV },
    { " 4x4-convergence,", "mem",  "-p",  "4", "-t",  "4", "-P",  "512", OPT_CONV },
    { " 4x4-convergence-NOTHP,",
    "mem",  "-p",  "4", "-t",  "4", "-P",  "512", OPT_CONV_NOTHP },
    { " 4x6-convergence,", "mem",  "-p",  "4", "-t",  "6", "-P", "1020", OPT_CONV },
    { " 4x8-convergence,", "mem",  "-p",  "4", "-t",  "8", "-P",  "512", OPT_CONV },
    { " 8x4-convergence,", "mem",  "-p",  "8", "-t",  "4", "-P",  "512", OPT_CONV },
    { " 8x4-convergence-NOTHP,",
    "mem",  "-p",  "8", "-t",  "4", "-P",  "512", OPT_CONV_NOTHP },
    { " 3x1-convergence,", "mem",  "-p",  "3", "-t",  "1", "-P",  "512", OPT_CONV },
    { " 4x1-convergence,", "mem",  "-p",  "4", "-t",  "1", "-P",  "512", OPT_CONV },
    { " 8x1-convergence,", "mem",  "-p",  "8", "-t",  "1", "-P",  "512", OPT_CONV },
    { "16x1-convergence,", "mem",  "-p", "16", "-t",  "1", "-P",  "256", OPT_CONV },
    { "32x1-convergence,", "mem",  "-p", "32", "-t",  "1", "-P",  "128", OPT_CONV },
// Various NUMA process/thread layout bandwidth measurements:
    { " 2x1-bw-process,",  "mem",  "-p",  "2", "-t",  "1", "-P", "1024", OPT_BW },
    { " 3x1-bw-process,",  "mem",  "-p",  "3", "-t",  "1", "-P", "1024", OPT_BW },
    { " 4x1-bw-process,",  "mem",  "-p",  "4", "-t",  "1", "-P", "1024", OPT_BW },
    { " 8x1-bw-process,",  "mem",  "-p",  "8", "-t",  "1", "-P", " 512", OPT_BW },
    { " 8x1-bw-process-NOTHP,",
    "mem",  "-p",  "8", "-t",  "1", "-P", " 512", OPT_BW_NOTHP },
    { "16x1-bw-process,",  "mem",  "-p", "16", "-t",  "1", "-P",  "256", OPT_BW },
    { " 1x4-bw-thread,",   "mem",  "-p",  "1", "-t",  "4", "-T",  "256", OPT_BW },
    { " 1x8-bw-thread,",   "mem",  "-p",  "1", "-t",  "8", "-T",  "256", OPT_BW },
    { "1x16-bw-thread,",   "mem",  "-p",  "1", "-t", "16", "-T",  "128", OPT_BW },
    { "1x32-bw-thread,",   "mem",  "-p",  "1", "-t", "32", "-T",   "64", OPT_BW },
    { " 2x3-bw-process,",  "mem",  "-p",  "2", "-t",  "3", "-P",  "512", OPT_BW },
    { " 4x4-bw-process,",  "mem",  "-p",  "4", "-t",  "4", "-P",  "512", OPT_BW },
    { " 4x6-bw-process,",  "mem",  "-p",  "4", "-t",  "6", "-P",  "512", OPT_BW },
    { " 4x8-bw-process,",  "mem",  "-p",  "4", "-t",  "8", "-P",  "512", OPT_BW },
    { " 4x8-bw-process-NOTHP,",
    "mem",  "-p",  "4", "-t",  "8", "-P",  "512", OPT_BW_NOTHP },
    { " 3x3-bw-process,",  "mem",  "-p",  "3", "-t",  "3", "-P",  "512", OPT_BW },
    { " 5x5-bw-process,",  "mem",  "-p",  "5", "-t",  "5", "-P",  "512", OPT_BW },
    { "2x16-bw-process,",  "mem",  "-p",  "2", "-t", "16", "-P",  "512", OPT_BW },
    { "1x32-bw-process,",  "mem",  "-p",  "1", "-t", "32", "-P", "2048", OPT_BW },
    { "numa02-bw,",        "mem",  "-p",  "1", "-t", "32", "-T",   "32", OPT_BW },
    { "numa02-bw-NOTHP,",  "mem",  "-p",  "1", "-t", "32", "-T",   "32", OPT_BW_NOTHP },
    { "numa01-bw-thread,", "mem",  "-p",  "2", "-t", "16", "-T",  "192", OPT_BW },
    { "numa01-bw-thread-NOTHP,",
    "mem",  "-p",  "2", "-t", "16", "-T",  "192", OPT_BW_NOTHP },
    };
#[no_mangle]
unsafe extern "C" fn bench_all() -> c_int {
    static int bench_all(void)
    {
    let mut nr: c_int = ARRAY_SIZE(tests);
    int ret;
    int i;
    ret = system("echo ' #'; echo ' # Running test on: '$(uname -a); echo ' #'");
    BUG_ON(ret < 0);
    for (i = 0; i < nr; i++) {
    run_bench_numa(tests[i][0], tests[i] + 1);
    }
    printf("\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_numa(argc: c_int, argv: *const c_char) -> c_int {
    int bench_numa(int argc, const char **argv)
    {
    init_params(&p0, "main,", argc, argv);
    argc = parse_options(argc, argv, options, bench_numa_usage, 0);
    if (argc)
    goto err;
    if (p0.run_all)
    return bench_all();
    if (__bench_numa(core::ptr::null_mut()))
    goto err;
    return 0;
    err:
    usage_with_options(numa_usage, options);
    return -1;
    }
