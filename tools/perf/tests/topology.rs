//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/topology.c
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

pub const DATA_SIZE: c_int = 10;
#[no_mangle]
unsafe extern "C" fn get_temp(path: *mut c_char) -> c_int {
    static int get_temp(char *path)
    {
    int fd;
    strcpy(path, TEMPL);
    fd = mkstemp(path);
    if (fd < 0) {
    perror("mkstemp failed");
    return -1;
    }
    close(fd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn session_write_header(path: *mut c_char) -> c_int {
    static int session_write_header(char *path)
    {
    struct perf_session *session;
    struct perf_data data = {
    .path = path,
    .mode = PERF_DATA_MODE_WRITE,
    };
    let mut target: target = {};
    session = perf_session__new(&data, core::ptr::null_mut());
    TEST_ASSERT_VAL("can't get session", !IS_ERR(session));
    session.evlist = evlist__new_default(&target, /*sample_callchains=*/false);
    TEST_ASSERT_VAL("can't get evlist", session.evlist);
    evlist__set_session(session.evlist, session);
    perf_header__set_feat(&session.header, HEADER_CPU_TOPOLOGY);
    perf_header__set_feat(&session.header, HEADER_NRCPUS);
    perf_header__set_feat(&session.header, HEADER_ARCH);
    session.header.data_size += DATA_SIZE;
    TEST_ASSERT_VAL("failed to write header",
    !perf_session__write_header(session, session.evlist,
    perf_data__fd(&data), true));
    evlist__put(session.evlist);
    perf_session__delete(session);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_cpu_topology(path: *mut c_char, map: *mut perf_cpu_map) -> c_int {
    static int check_cpu_topology(char *path, struct perf_cpu_map *map)
    {
    struct perf_session *session;
    struct perf_data data = {
    .path = path,
    .mode = PERF_DATA_MODE_READ,
    };
    unsigned int i;
    struct aggr_cpu_id id;
    struct perf_cpu cpu;
    struct perf_env *env;
    uint16_t e_machine;
    session = perf_session__new(&data, core::ptr::null_mut());
    TEST_ASSERT_VAL("can't get session", !IS_ERR(session));
    env = perf_session__env(session);
    cpu__setup_cpunode_map();
// On platforms with large numbers of CPUs process_cpu_topology()
// might issue an error while reading the perf.data file section
// HEADER_CPU_TOPOLOGY and the cpu_topology_map pointed to by member
// cpu is a NULL pointer.
// Example: On s390
// CPU 0 is on core_id 0 and physical_package_id 6
// CPU 1 is on core_id 1 and physical_package_id 3
//
// Core_id and physical_package_id are platform and architecture
// dependent and might have higher numbers than the CPU id.
// This actually depends on the configuration.
//
// In this case process_cpu_topology() prints error message:
// "socket_id number is too big. You may need to upgrade the
// perf tool."
//
// This is the reason why this test might be skipped. aarch64 and
// s390 always write this part of the header, even when the above
// condition is true (see do_core_id_test in header.c). So always
// run this test on those platforms.
//
    e_machine = perf_env__e_machine(env, core::ptr::null_mut());
    if (!env.cpu && e_machine != EM_S390 && e_machine != EM_AARCH64)
    return TEST_SKIP;
//
// In powerpc pSeries platform, not all the topology information
// are exposed via sysfs. Due to restriction, detail like
// physical_package_id will be set to -1. Hence skip this
// test if physical_package_id returns -1 for cpu from perf_cpu_map.
//
    if (e_machine == EM_PPC64) {
    if (cpu__get_socket_id(perf_cpu_map__cpu(map, 0)) == -1)
    return TEST_SKIP;
    }
    TEST_ASSERT_VAL("Session header CPU map not set", env.cpu);
    for (i = 0; i < (unsigned int)env.nr_cpus_avail; i++) {
    cpu.cpu = i;
    if (!perf_cpu_map__has(map, cpu))
    continue;
    pr_debug("CPU %d, core %d, socket %d\n", i,
    env.cpu[i].core_id,
    env.cpu[i].socket_id);
    }
// Test that CPU ID contains socket, die, core and CPU
    perf_cpu_map__for_each_cpu(cpu, i, map) {
    id = aggr_cpu_id__cpu(cpu, core::ptr::null_mut());
    TEST_ASSERT_VAL("Cpu map - CPU ID doesn't match",
    cpu.cpu == id.cpu.cpu);
    TEST_ASSERT_VAL("Cpu map - Core ID doesn't match",
    env.cpu[cpu.cpu].core_id == id.core);
    TEST_ASSERT_VAL("Cpu map - Socket ID doesn't match",
    env.cpu[cpu.cpu].socket_id == id.socket);
    TEST_ASSERT_VAL("Cpu map - Die ID doesn't match",
    env.cpu[cpu.cpu].die_id == id.die);
    TEST_ASSERT_VAL("Cpu map - Node ID is set", id.node == -1);
    TEST_ASSERT_VAL("Cpu map - Thread IDX is set", id.thread_idx == -1);
    }
// Test that core ID contains socket, die and core
    perf_cpu_map__for_each_cpu(cpu, i, map) {
    id = aggr_cpu_id__core(cpu, core::ptr::null_mut());
    TEST_ASSERT_VAL("Core map - Core ID doesn't match",
    env.cpu[cpu.cpu].core_id == id.core);
    TEST_ASSERT_VAL("Core map - Socket ID doesn't match",
    env.cpu[cpu.cpu].socket_id == id.socket);
    TEST_ASSERT_VAL("Core map - Die ID doesn't match",
    env.cpu[cpu.cpu].die_id == id.die);
    TEST_ASSERT_VAL("Core map - Node ID is set", id.node == -1);
    TEST_ASSERT_VAL("Core map - Thread IDX is set", id.thread_idx == -1);
    }
// Test that die ID contains socket and die
    perf_cpu_map__for_each_cpu(cpu, i, map) {
    id = aggr_cpu_id__die(cpu, core::ptr::null_mut());
    TEST_ASSERT_VAL("Die map - Socket ID doesn't match",
    env.cpu[cpu.cpu].socket_id == id.socket);
    TEST_ASSERT_VAL("Die map - Die ID doesn't match",
    env.cpu[cpu.cpu].die_id == id.die);
    TEST_ASSERT_VAL("Die map - Node ID is set", id.node == -1);
    TEST_ASSERT_VAL("Die map - Core is set", id.core == -1);
    TEST_ASSERT_VAL("Die map - CPU is set", id.cpu.cpu == -1);
    TEST_ASSERT_VAL("Die map - Thread IDX is set", id.thread_idx == -1);
    }
// Test that socket ID contains only socket
    perf_cpu_map__for_each_cpu(cpu, i, map) {
    id = aggr_cpu_id__socket(cpu, core::ptr::null_mut());
    TEST_ASSERT_VAL("Socket map - Socket ID doesn't match",
    env.cpu[cpu.cpu].socket_id == id.socket);
    TEST_ASSERT_VAL("Socket map - Node ID is set", id.node == -1);
    TEST_ASSERT_VAL("Socket map - Die ID is set", id.die == -1);
    TEST_ASSERT_VAL("Socket map - Core is set", id.core == -1);
    TEST_ASSERT_VAL("Socket map - CPU is set", id.cpu.cpu == -1);
    TEST_ASSERT_VAL("Socket map - Thread IDX is set", id.thread_idx == -1);
    }
// Test that node ID contains only node
    perf_cpu_map__for_each_cpu(cpu, i, map) {
    id = aggr_cpu_id__node(cpu, core::ptr::null_mut());
    TEST_ASSERT_VAL("Node map - Node ID doesn't match",
    cpu__get_node(cpu) == id.node);
    TEST_ASSERT_VAL("Node map - Socket is set", id.socket == -1);
    TEST_ASSERT_VAL("Node map - Die ID is set", id.die == -1);
    TEST_ASSERT_VAL("Node map - Core is set", id.core == -1);
    TEST_ASSERT_VAL("Node map - CPU is set", id.cpu.cpu == -1);
    TEST_ASSERT_VAL("Node map - Thread IDX is set", id.thread_idx == -1);
    }
    perf_session__delete(session);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test__session_topology(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__session_topology(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    char path[PATH_MAX];
    struct perf_cpu_map *map;
    let mut ret: c_int = TEST_FAIL;
    TEST_ASSERT_VAL("can't get templ file", !get_temp(path));
    pr_debug("templ file: %s\n", path);
    if (session_write_header(path))
    goto free_path;
    map = perf_cpu_map__new_online_cpus();
    if (map == core::ptr::null_mut()) {
    pr_debug("failed to get system cpumap\n");
    goto free_path;
    }
    ret = check_cpu_topology(path, map);
    perf_cpu_map__put(map);
    free_path:
    unlink(path);
    return ret;
    }
    DEFINE_SUITE("Session topology", session_topology);
