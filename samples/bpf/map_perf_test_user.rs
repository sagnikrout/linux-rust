//! Automatically rewritten from C to Rust
//! Source: samples/bpf/map_perf_test_user.c
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
// Copyright (c) 2016 Facebook
//
// Macro flag: #define _GNU_SOURCE

pub const MAX_NR_CPUS: c_int = 1024;
#[no_mangle]
unsafe extern "C" fn time_get_ns() -> __u64 {
    static __u64 time_get_ns(void)
    {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000000000ull + ts.tv_nsec;
    }
    enum test_type {
    HASH_PREALLOC,
    PERCPU_HASH_PREALLOC,
    HASH_KMALLOC,
    PERCPU_HASH_KMALLOC,
    LRU_HASH_PREALLOC,
    NOCOMMON_LRU_HASH_PREALLOC,
    LPM_KMALLOC,
    HASH_LOOKUP,
    ARRAY_LOOKUP,
    INNER_LRU_HASH_PREALLOC,
    LRU_HASH_LOOKUP,
    NR_TESTS,
    };
    const char *test_map_names[NR_TESTS] = {
    [HASH_PREALLOC] = "hash_map",
    [PERCPU_HASH_PREALLOC] = "percpu_hash_map",
    [HASH_KMALLOC] = "hash_map_alloc",
    [PERCPU_HASH_KMALLOC] = "percpu_hash_map_alloc",
    [LRU_HASH_PREALLOC] = "lru_hash_map",
    [NOCOMMON_LRU_HASH_PREALLOC] = "nocommon_lru_hash_map",
    [LPM_KMALLOC] = "lpm_trie_map_alloc",
    [HASH_LOOKUP] = "hash_map",
    [ARRAY_LOOKUP] = "array_map",
    [INNER_LRU_HASH_PREALLOC] = "inner_lru_hash_map",
    [LRU_HASH_LOOKUP] = "lru_hash_lookup_map",
    };
    enum map_idx {
    array_of_lru_hashs_idx,
    hash_map_alloc_idx,
    lru_hash_lookup_idx,
    NR_IDXES,
    };
    static int map_fd[NR_IDXES];
    let mut test_flags: static int = ~0;
    static uint32_t num_map_entries;
    static uint32_t inner_lru_hash_size;
    let mut lru_hash_lookup_test_entries: static int = 32;
    let mut max_cnt: static uint32_t = 10000;
#[no_mangle]
unsafe extern "C" fn check_test_flags(t: enum test_type) -> c_int {
    static int check_test_flags(enum test_type t)
    {
    return test_flags & TEST_BIT(t);
    }
#[no_mangle]
unsafe extern "C" fn test_hash_prealloc(cpu: c_int) {
    static void test_hash_prealloc(int cpu)
    {
    __u64 start_time;
    int i;
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++)
    syscall(__NR_getuid);
    printf("%d:hash_map_perf pre-alloc %lld events per sec\n",
    cpu, max_cnt * 1000000000ll / (time_get_ns() - start_time));
    }
#[no_mangle]
unsafe extern "C" fn pre_test_lru_hash_lookup(tasks: c_int) -> c_int {
    static int pre_test_lru_hash_lookup(int tasks)
    {
    let mut fd: c_int = map_fd[lru_hash_lookup_idx];
    uint32_t key;
    let mut val: c_long = 1;
    int ret;
    if (num_map_entries > lru_hash_lookup_test_entries)
    lru_hash_lookup_test_entries = num_map_entries;
// Populate the lru_hash_map for LRU_HASH_LOOKUP perf test.
//
// It is fine that the user requests for a map with
// num_map_entries < 32 and some of the later lru hash lookup
// may return not found.  For LRU map, we are not interested
// in such small map performance.
//
    for (key = 0; key < lru_hash_lookup_test_entries; key++) {
    ret = bpf_map_update_elem(fd, &key, &val, BPF_NOEXIST);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_test_lru(test: enum test_type, cpu: c_int) {
    static void do_test_lru(enum test_type test, int cpu)
    {
    static int inner_lru_map_fds[MAX_NR_CPUS];
    let mut in6: sockaddr_in6 = { .sin6_family = AF_INET6 };
    const char *test_name;
    __u64 start_time;
    int i, ret;
    if (test == INNER_LRU_HASH_PREALLOC && cpu) {
// If CPU is not 0, create inner_lru hash map and insert the fd
// value into the array_of_lru_hash map. In case of CPU 0,
// 'inner_lru_hash_map' was statically inserted on the map init
//
    let mut outer_fd: c_int = map_fd[array_of_lru_hashs_idx];
    unsigned int mycpu, mynode;
    LIBBPF_OPTS(bpf_map_create_opts, opts,
    .map_flags = BPF_F_NUMA_NODE,
    );
    assert(cpu < MAX_NR_CPUS);
    ret = syscall(__NR_getcpu, &mycpu, &mynode, core::ptr::null_mut());
    assert(!ret);
    opts.numa_node = mynode;
    inner_lru_map_fds[cpu] =
    bpf_map_create(BPF_MAP_TYPE_LRU_HASH,
    test_map_names[INNER_LRU_HASH_PREALLOC],
    sizeof(uint32_t),
    sizeof(long),
    inner_lru_hash_size, &opts);
    if (inner_lru_map_fds[cpu] == -1) {
    printf("cannot create BPF_MAP_TYPE_LRU_HASH %s(%d)\n",
    strerror(errno), errno);
    exit(1);
    }
    ret = bpf_map_update_elem(outer_fd, &cpu,
    &inner_lru_map_fds[cpu],
    BPF_ANY);
    if (ret) {
    printf("cannot update ARRAY_OF_LRU_HASHS with key:%u. %s(%d)\n",
    cpu, strerror(errno), errno);
    exit(1);
    }
    }
    in6.sin6_addr.s6_addr16[0] = 0xdead;
    in6.sin6_addr.s6_addr16[1] = 0xbeef;
    if (test == LRU_HASH_PREALLOC) {
    test_name = "lru_hash_map_perf";
    in6.sin6_addr.s6_addr16[2] = 0;
    } else if (test == NOCOMMON_LRU_HASH_PREALLOC) {
    test_name = "nocommon_lru_hash_map_perf";
    in6.sin6_addr.s6_addr16[2] = 1;
    } else if (test == INNER_LRU_HASH_PREALLOC) {
    test_name = "inner_lru_hash_map_perf";
    in6.sin6_addr.s6_addr16[2] = 2;
    } else if (test == LRU_HASH_LOOKUP) {
    test_name = "lru_hash_lookup_perf";
    in6.sin6_addr.s6_addr16[2] = 3;
    in6.sin6_addr.s6_addr32[3] = 0;
    } else {
    assert(0);
    }
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++) {
    ret = connect(-1, (const struct sockaddr *)&in6, sizeof(in6));
    assert(ret == -1 && errno == EBADF);
    if (in6.sin6_addr.s6_addr32[3] <
    lru_hash_lookup_test_entries - 32)
    in6.sin6_addr.s6_addr32[3] += 32;
    else
    in6.sin6_addr.s6_addr32[3] = 0;
    }
    printf("%d:%s pre-alloc %lld events per sec\n",
    cpu, test_name,
    max_cnt * 1000000000ll / (time_get_ns() - start_time));
    }
#[no_mangle]
unsafe extern "C" fn test_lru_hash_prealloc(cpu: c_int) {
    static void test_lru_hash_prealloc(int cpu)
    {
    do_test_lru(LRU_HASH_PREALLOC, cpu);
    }
#[no_mangle]
unsafe extern "C" fn test_nocommon_lru_hash_prealloc(cpu: c_int) {
    static void test_nocommon_lru_hash_prealloc(int cpu)
    {
    do_test_lru(NOCOMMON_LRU_HASH_PREALLOC, cpu);
    }
#[no_mangle]
unsafe extern "C" fn test_inner_lru_hash_prealloc(cpu: c_int) {
    static void test_inner_lru_hash_prealloc(int cpu)
    {
    do_test_lru(INNER_LRU_HASH_PREALLOC, cpu);
    }
#[no_mangle]
unsafe extern "C" fn test_lru_hash_lookup(cpu: c_int) {
    static void test_lru_hash_lookup(int cpu)
    {
    do_test_lru(LRU_HASH_LOOKUP, cpu);
    }
#[no_mangle]
unsafe extern "C" fn test_percpu_hash_prealloc(cpu: c_int) {
    static void test_percpu_hash_prealloc(int cpu)
    {
    __u64 start_time;
    int i;
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++)
    syscall(__NR_geteuid);
    printf("%d:percpu_hash_map_perf pre-alloc %lld events per sec\n",
    cpu, max_cnt * 1000000000ll / (time_get_ns() - start_time));
    }
#[no_mangle]
unsafe extern "C" fn test_hash_kmalloc(cpu: c_int) {
    static void test_hash_kmalloc(int cpu)
    {
    __u64 start_time;
    int i;
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++)
    syscall(__NR_getgid);
    printf("%d:hash_map_perf kmalloc %lld events per sec\n",
    cpu, max_cnt * 1000000000ll / (time_get_ns() - start_time));
    }
#[no_mangle]
unsafe extern "C" fn test_percpu_hash_kmalloc(cpu: c_int) {
    static void test_percpu_hash_kmalloc(int cpu)
    {
    __u64 start_time;
    int i;
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++)
    syscall(__NR_getegid);
    printf("%d:percpu_hash_map_perf kmalloc %lld events per sec\n",
    cpu, max_cnt * 1000000000ll / (time_get_ns() - start_time));
    }
#[no_mangle]
unsafe extern "C" fn test_lpm_kmalloc(cpu: c_int) {
    static void test_lpm_kmalloc(int cpu)
    {
    __u64 start_time;
    int i;
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++)
    syscall(__NR_gettid);
    printf("%d:lpm_perf kmalloc %lld events per sec\n",
    cpu, max_cnt * 1000000000ll / (time_get_ns() - start_time));
    }
#[no_mangle]
unsafe extern "C" fn test_hash_lookup(cpu: c_int) {
    static void test_hash_lookup(int cpu)
    {
    __u64 start_time;
    int i;
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++)
    syscall(__NR_getpgid, 0);
    printf("%d:hash_lookup %lld lookups per sec\n",
    cpu, max_cnt * 1000000000ll * 64 / (time_get_ns() - start_time));
    }
#[no_mangle]
unsafe extern "C" fn test_array_lookup(cpu: c_int) {
    static void test_array_lookup(int cpu)
    {
    __u64 start_time;
    int i;
    start_time = time_get_ns();
    for (i = 0; i < max_cnt; i++)
    syscall(__NR_getppid, 0);
    printf("%d:array_lookup %lld lookups per sec\n",
    cpu, max_cnt * 1000000000ll * 64 / (time_get_ns() - start_time));
    }
    typedef int (*pre_test_func)(int tasks);
    const pre_test_func pre_test_funcs[] = {
    [LRU_HASH_LOOKUP] = pre_test_lru_hash_lookup,
    };
    typedef void (*test_func)(int cpu);
    const test_func test_funcs[] = {
    [HASH_PREALLOC] = test_hash_prealloc,
    [PERCPU_HASH_PREALLOC] = test_percpu_hash_prealloc,
    [HASH_KMALLOC] = test_hash_kmalloc,
    [PERCPU_HASH_KMALLOC] = test_percpu_hash_kmalloc,
    [LRU_HASH_PREALLOC] = test_lru_hash_prealloc,
    [NOCOMMON_LRU_HASH_PREALLOC] = test_nocommon_lru_hash_prealloc,
    [LPM_KMALLOC] = test_lpm_kmalloc,
    [HASH_LOOKUP] = test_hash_lookup,
    [ARRAY_LOOKUP] = test_array_lookup,
    [INNER_LRU_HASH_PREALLOC] = test_inner_lru_hash_prealloc,
    [LRU_HASH_LOOKUP] = test_lru_hash_lookup,
    };
#[no_mangle]
unsafe extern "C" fn pre_test(tasks: c_int) -> c_int {
    static int pre_test(int tasks)
    {
    int i;
    for (i = 0; i < NR_TESTS; i++) {
    if (pre_test_funcs[i] && check_test_flags(i)) {
    let mut ret: c_int = pre_test_funcs[i](tasks);
    if (ret)
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loop(cpu: c_int) {
    static void loop(int cpu)
    {
    cpu_set_t cpuset;
    int i;
    CPU_ZERO(&cpuset);
    CPU_SET(cpu, &cpuset);
    sched_setaffinity(0, sizeof(cpuset), &cpuset);
    for (i = 0; i < NR_TESTS; i++) {
    if (check_test_flags(i))
    test_funcs[i](cpu);
    }
    }
#[no_mangle]
unsafe extern "C" fn run_perf_test(tasks: c_int) {
    static void run_perf_test(int tasks)
    {
    pid_t pid[tasks];
    int i;
    assert(!pre_test(tasks));
    for (i = 0; i < tasks; i++) {
    pid[i] = fork();
    if (pid[i] == 0) {
    loop(i);
    exit(0);
    } else if (pid[i] == -1) {
    printf("couldn't spawn #%d process\n", i);
    exit(1);
    }
    }
    for (i = 0; i < tasks; i++) {
    int status;
    assert(waitpid(pid[i], &status, 0) == pid[i]);
    assert(status == 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn fill_lpm_trie() {
    static void fill_lpm_trie(void)
    {
    struct bpf_lpm_trie_key_u8 *key;
    let mut value: c_ulong = 0;
    unsigned int i;
    int r;
    key = alloca(sizeof(*key) + 4);
    key.prefixlen = 32;
    for (i = 0; i < 512; ++i) {
    key.prefixlen = rand() % 33;
    key.data[0] = rand() & 0xff;
    key.data[1] = rand() & 0xff;
    key.data[2] = rand() & 0xff;
    key.data[3] = rand() & 0xff;
    r = bpf_map_update_elem(map_fd[hash_map_alloc_idx],
    key, &value, 0);
    assert(!r);
    }
    key.prefixlen = 32;
    key.data[0] = 192;
    key.data[1] = 168;
    key.data[2] = 0;
    key.data[3] = 1;
    value = 128;
    r = bpf_map_update_elem(map_fd[hash_map_alloc_idx], key, &value, 0);
    assert(!r);
    }
#[no_mangle]
unsafe extern "C" fn fixup_map(obj: *mut bpf_object) {
    static void fixup_map(struct bpf_object *obj)
    {
    struct bpf_map *map;
    int i;
    bpf_object__for_each_map(map, obj) {
    const char *name = bpf_map__name(map);
// Only change the max_entries for the enabled test(s)
    for (i = 0; i < NR_TESTS; i++) {
    if (!strcmp(test_map_names[i], name) &&
    (check_test_flags(i))) {
    bpf_map__set_max_entries(map, num_map_entries);
    continue;
    }
    }
    }
    inner_lru_hash_size = num_map_entries;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut nr_cpus: c_int = sysconf(_SC_NPROCESSORS_ONLN);
    struct bpf_link *links[8];
    struct bpf_program *prog;
    struct bpf_object *obj;
    struct bpf_map *map;
    char filename[256];
    let mut i: c_int = 0;
    if (argc > 1)
    test_flags = atoi(argv[1]) ? : test_flags;
    if (argc > 2)
    nr_cpus = atoi(argv[2]) ? : nr_cpus;
    if (argc > 3)
    num_map_entries = atoi(argv[3]);
    if (argc > 4)
    max_cnt = atoi(argv[4]);
    snprintf(filename, sizeof(filename), "%s.bpf.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return 0;
    }
    map = bpf_object__find_map_by_name(obj, "inner_lru_hash_map");
    if (libbpf_get_error(map)) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    inner_lru_hash_size = bpf_map__max_entries(map);
    if (!inner_lru_hash_size) {
    fprintf(stderr, "ERROR: failed to get map attribute\n");
    goto cleanup;
    }
// resize BPF map prior to loading
    if (num_map_entries > 0)
    fixup_map(obj);
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    map_fd[0] = bpf_object__find_map_fd_by_name(obj, "array_of_lru_hashs");
    map_fd[1] = bpf_object__find_map_fd_by_name(obj, "hash_map_alloc");
    map_fd[2] = bpf_object__find_map_fd_by_name(obj, "lru_hash_lookup_map");
    if (map_fd[0] < 0 || map_fd[1] < 0 || map_fd[2] < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    bpf_object__for_each_program(prog, obj) {
    links[i] = bpf_program__attach(prog);
    if (libbpf_get_error(links[i])) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    links[i] = core::ptr::null_mut();
    goto cleanup;
    }
    i++;
    }
    fill_lpm_trie();
    run_perf_test(nr_cpus);
    cleanup:
    for (i--; i >= 0; i--)
    bpf_link__destroy(links[i]);
    bpf_object__close(obj);
    return 0;
    }
