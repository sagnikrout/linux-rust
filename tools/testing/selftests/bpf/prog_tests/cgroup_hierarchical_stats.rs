//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_hierarchical_stats.c
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
// This test makes sure BPF stats collection using rstat works correctly.
// The test uses 3 BPF progs:
// (a) counter: This BPF prog is invoked every time we attach a process to a
// cgroup and locklessly increments a percpu counter.
// The program then calls cgroup_rstat_updated() to inform rstat
// of an update on the (cpu, cgroup) pair.
//
// (b) flusher: This BPF prog is invoked when an rstat flush is ongoing, it
// aggregates all percpu counters to a total counter, and also
// propagates the changes to the ancestor cgroups.
//
// (c) dumper: This BPF prog is a cgroup_iter. It is used to output the total
// counter of a cgroup through reading a file in userspace.
//
// The test sets up a cgroup hierarchy, and the above programs. It spawns a few
// processes in the leaf cgroups and makes sure all the counters are aggregated
// correctly.
//
// Copyright 2022 Google LLC.
//

pub const PAGE_SIZE: c_int = 4096;

pub const PROCESSES_PER_CGROUP: c_int = 3;

pub const CG_ROOT_ID: c_int = 1;

    static struct {
    const char *path, *name;
    unsigned long long id;
    int fd;
    } cgroups[] = {
    CGROUP_PATH("/", "test"),
    CGROUP_PATH("/test", "child1"),
    CGROUP_PATH("/test", "child2"),
    CGROUP_PATH("/test/child1", "child1_1"),
    CGROUP_PATH("/test/child1", "child1_2"),
    CGROUP_PATH("/test/child2", "child2_1"),
    CGROUP_PATH("/test/child2", "child2_2"),
    };

pub const N_NON_LEAF_CGROUPS: c_int = 3;
    static int root_cgroup_fd;
    static bool mounted_bpffs;
// reads file at 'path' to 'buf', returns 0 on success.
#[no_mangle]
unsafe extern "C" fn read_from_file(path: *const c_char, buf: *mut c_char, size: usize) -> c_int {
    static int read_from_file(const char *path, char *buf, size_t size)
    {
    int fd, len;
    fd = open(path, O_RDONLY);
    if (fd < 0)
    return fd;
    len = read(fd, buf, size);
    close(fd);
    if (len < 0)
    return len;
    buf[len] = 0;
    return 0;
    }
// mounts bpffs and mkdir for reading stats, returns 0 on success.
#[no_mangle]
unsafe extern "C" fn setup_bpffs() -> c_int {
    static int setup_bpffs(void)
    {
    int err;
// Mount bpffs
    err = mount("bpf", BPFFS_ROOT, "bpf", 0, core::ptr::null_mut());
    mounted_bpffs = !err;
    if (ASSERT_FALSE(err && errno != EBUSY, "mount"))
    return err;
// Create a directory to contain stat files in bpffs
    err = mkdir(BPFFS_ATTACH_COUNTERS, 0755);
    if (!ASSERT_OK(err, "mkdir"))
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_bpffs() {
    static void cleanup_bpffs(void)
    {
// Remove created directory in bpffs
    ASSERT_OK(rmdir(BPFFS_ATTACH_COUNTERS), "rmdir "BPFFS_ATTACH_COUNTERS);
// Unmount bpffs, if it wasn't already mounted when we started
    if (mounted_bpffs)
    return;
    ASSERT_OK(umount(BPFFS_ROOT), "unmount bpffs");
    }
// sets up cgroups, returns 0 on success.
#[no_mangle]
unsafe extern "C" fn setup_cgroups() -> c_int {
    static int setup_cgroups(void)
    {
    int i, fd, err;
    err = setup_cgroup_environment();
    if (!ASSERT_OK(err, "setup_cgroup_environment"))
    return err;
    root_cgroup_fd = get_root_cgroup();
    if (!ASSERT_GE(root_cgroup_fd, 0, "get_root_cgroup"))
    return root_cgroup_fd;
    for (i = 0; i < N_CGROUPS; i++) {
    fd = create_and_get_cgroup(cgroups[i].path);
    if (!ASSERT_GE(fd, 0, "create_and_get_cgroup"))
    return fd;
    cgroups[i].fd = fd;
    cgroups[i].id = get_cgroup_id(cgroups[i].path);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_cgroups() {
    static void cleanup_cgroups(void)
    {
    close(root_cgroup_fd);
    for (int i = 0; i < N_CGROUPS; i++)
    close(cgroups[i].fd);
    cleanup_cgroup_environment();
    }
// Sets up cgroup hiearchary, returns 0 on success.
#[no_mangle]
unsafe extern "C" fn setup_hierarchy() -> c_int {
    static int setup_hierarchy(void)
    {
    return setup_bpffs() || setup_cgroups();
    }
#[no_mangle]
unsafe extern "C" fn destroy_hierarchy() {
    static void destroy_hierarchy(void)
    {
    cleanup_cgroups();
    cleanup_bpffs();
    }
#[no_mangle]
unsafe extern "C" fn attach_processes() -> c_int {
    static int attach_processes(void)
    {
    int i, j, status;
// In every leaf cgroup, attach 3 processes
    for (i = N_NON_LEAF_CGROUPS; i < N_CGROUPS; i++) {
    for (j = 0; j < PROCESSES_PER_CGROUP; j++) {
    pid_t pid;
// Create child and attach to cgroup
    pid = fork();
    if (pid == 0) {
    if (join_parent_cgroup(cgroups[i].path))
    exit(EACCES);
    exit(0);
    }
// Cleanup child
    waitpid(pid, &status, 0);
    if (!ASSERT_TRUE(WIFEXITED(status), "child process exited"))
    return 1;
    if (!ASSERT_EQ(WEXITSTATUS(status), 0,
    "child process exit code"))
    return 1;
    }
    }
    return 0;
    }
    static unsigned long long
    get_attach_counter(unsigned long long cgroup_id, const char *file_name)
    {
    let mut attach_counter: c_ulonglong = 0, id = 0;
    static char buf[128], path[128];
// For every cgroup, read the file generated by cgroup_iter
    snprintf(path, 128, "%s%s", BPFFS_ATTACH_COUNTERS, file_name);
    if (!ASSERT_OK(read_from_file(path, buf, 128), "read cgroup_iter"))
    return 0;
// Check the output file formatting
    ASSERT_EQ(sscanf(buf, "cg_id: %llu, attach_counter: %llu\n",
    &id, &attach_counter), 2, "output format");
// Check that the cgroup_id is displayed correctly
    ASSERT_EQ(id, cgroup_id, "cgroup_id");
// Check that the counter is non-zero
    ASSERT_GT(attach_counter, 0, "attach counter non-zero");
    return attach_counter;
    }
#[no_mangle]
unsafe extern "C" fn check_attach_counters() {
    static void check_attach_counters(void)
    {
    unsigned long long attach_counters[N_CGROUPS], root_attach_counter;
    int i;
    for (i = 0; i < N_CGROUPS; i++)
    attach_counters[i] = get_attach_counter(cgroups[i].id,
    cgroups[i].name);
// Read stats for root too
    root_attach_counter = get_attach_counter(CG_ROOT_ID, CG_ROOT_NAME);
// Check that all leafs cgroups have an attach counter of 3
    for (i = N_NON_LEAF_CGROUPS; i < N_CGROUPS; i++)
    ASSERT_EQ(attach_counters[i], PROCESSES_PER_CGROUP,
    "leaf cgroup attach counter");
// Check that child1 == child1_1 + child1_2
    ASSERT_EQ(attach_counters[1], attach_counters[3] + attach_counters[4],
    "child1_counter");
// Check that child2 == child2_1 + child2_2
    ASSERT_EQ(attach_counters[2], attach_counters[5] + attach_counters[6],
    "child2_counter");
// Check that test == child1 + child2
    ASSERT_EQ(attach_counters[0], attach_counters[1] + attach_counters[2],
    "test_counter");
// Check that root >= test
    ASSERT_GE(root_attach_counter, attach_counters[1], "root_counter");
    }
// Creates iter link and pins in bpffs, returns 0 on success, -errno on failure.
//
    static int setup_cgroup_iter(struct cgroup_hierarchical_stats *obj,
    int cgroup_fd, const char *file_name)
    {
    DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    let mut linfo: union bpf_iter_link_info = {};
    struct bpf_link *link;
    static char path[128];
    int err;
//
// Create an iter link, parameterized by cgroup_fd. We only want to
// traverse one cgroup, so set the traversal order to "self".
//
    linfo.cgroup.cgroup_fd = cgroup_fd;
    linfo.cgroup.order = BPF_CGROUP_ITER_SELF_ONLY;
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    link = bpf_program__attach_iter(obj.progs.dumper, &opts);
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    return -EFAULT;
// Pin the link to a bpffs file
    snprintf(path, 128, "%s%s", BPFFS_ATTACH_COUNTERS, file_name);
    err = bpf_link__pin(link, path);
    ASSERT_OK(err, "pin cgroup_iter");
// Remove the link, leaving only the ref held by the pinned file
    bpf_link__destroy(link);
    return err;
    }
// Sets up programs for collecting stats, returns 0 on success.
#[no_mangle]
unsafe extern "C" fn setup_progs(skel: *mut cgroup_hierarchical_stats) -> c_int {
    static int setup_progs(struct cgroup_hierarchical_stats **skel)
    {
    int i, err;
// skel = cgroup_hierarchical_stats__open_and_load();
    if (!ASSERT_OK_PTR(*skel, "open_and_load"))
    return 1;
// Attach cgroup_iter program that will dump the stats to cgroups
    for (i = 0; i < N_CGROUPS; i++) {
    err = setup_cgroup_iter(*skel, cgroups[i].fd, cgroups[i].name);
    if (!ASSERT_OK(err, "setup_cgroup_iter"))
    return err;
    }
// Also dump stats for root
    err = setup_cgroup_iter(*skel, root_cgroup_fd, CG_ROOT_NAME);
    if (!ASSERT_OK(err, "setup_cgroup_iter"))
    return err;
    bpf_program__set_autoattach((*skel).progs.dumper, false);
    err = cgroup_hierarchical_stats__attach(*skel);
    if (!ASSERT_OK(err, "attach"))
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn destroy_progs(skel: *mut cgroup_hierarchical_stats) {
    static void destroy_progs(struct cgroup_hierarchical_stats *skel)
    {
    static char path[128];
    int i;
    for (i = 0; i < N_CGROUPS; i++) {
// Delete files in bpffs that cgroup_iters are pinned in
    snprintf(path, 128, "%s%s", BPFFS_ATTACH_COUNTERS,
    cgroups[i].name);
    ASSERT_OK(remove(path), "remove cgroup_iter pin");
    }
// Delete root file in bpffs
    snprintf(path, 128, "%s%s", BPFFS_ATTACH_COUNTERS, CG_ROOT_NAME);
    ASSERT_OK(remove(path), "remove cgroup_iter root pin");
    cgroup_hierarchical_stats__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_hierarchical_stats() {
    void test_cgroup_hierarchical_stats(void)
    {
    struct cgroup_hierarchical_stats *skel = core::ptr::null_mut();
    if (setup_hierarchy())
    goto hierarchy_cleanup;
    if (setup_progs(&skel))
    goto cleanup;
    if (attach_processes())
    goto cleanup;
    check_attach_counters();
    cleanup:
    destroy_progs(skel);
    hierarchy_cleanup:
    destroy_hierarchy();
    }
