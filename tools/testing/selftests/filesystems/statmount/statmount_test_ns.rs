//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/statmount/statmount_test_ns.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Macro flag: #define _GNU_SOURCE

pub const NSID_PASS: c_int = 0;
pub const NSID_FAIL: c_int = 1;
pub const NSID_SKIP: c_int = 2;
pub const NSID_ERROR: c_int = 3;
#[no_mangle]
unsafe extern "C" fn handle_result(ret: c_int, testname: *const c_char) {
    static void handle_result(int ret, const char *testname)
    {
    if (ret == NSID_PASS)
    ksft_test_result_pass("%s\n", testname);
#[no_mangle]
pub unsafe extern "C" fn if(NSID_FAIL: ret ==) -> else {
    else if (ret == NSID_FAIL)
    ksft_test_result_fail("%s\n", testname);
#[no_mangle]
pub unsafe extern "C" fn if(NSID_ERROR: ret ==) -> else {
    else if (ret == NSID_ERROR)
    ksft_exit_fail_msg("%s\n", testname);
    else
    ksft_test_result_skip("%s\n", testname);
    }
#[no_mangle]
unsafe extern "C" fn get_mnt_ns_id(mnt_ns: *const c_char, mnt_ns_id: *mut u64) -> c_int {
    static int get_mnt_ns_id(const char *mnt_ns, uint64_t *mnt_ns_id)
    {
    let mut fd: c_int = open(mnt_ns, O_RDONLY);
    if (fd < 0) {
    ksft_print_msg("failed to open for ns %s: %s\n",
    mnt_ns, strerror(errno));
    sleep(60);
    return NSID_ERROR;
    }
    if (ioctl(fd, NS_GET_MNTNS_ID, mnt_ns_id) < 0) {
    ksft_print_msg("failed to get the nsid for ns %s: %s\n",
    mnt_ns, strerror(errno));
    return NSID_ERROR;
    }
    close(fd);
    return NSID_PASS;
    }
#[no_mangle]
unsafe extern "C" fn setup_namespace() -> c_int {
    static int setup_namespace(void)
    {
    if (setup_userns() != 0)
    return NSID_ERROR;
    return NSID_PASS;
    }
#[no_mangle]
unsafe extern "C" fn _test_statmount_mnt_ns_id() -> c_int {
    static int _test_statmount_mnt_ns_id(void)
    {
    struct statmount sm;
    uint64_t mnt_ns_id;
    uint64_t root_id;
    int ret;
    ret = get_mnt_ns_id("/proc/self/ns/mnt", &mnt_ns_id);
    if (ret != NSID_PASS)
    return ret;
    root_id = get_unique_mnt_id("/");
    if (!root_id)
    return NSID_ERROR;
    ret = statmount(root_id, 0, 0, STATMOUNT_MNT_NS_ID, &sm, sizeof(sm), 0);
    if (ret == -1) {
    ksft_print_msg("statmount mnt ns id: %s\n", strerror(errno));
    return NSID_ERROR;
    }
    if (sm.size != sizeof(sm)) {
    ksft_print_msg("unexpected size: %u != %u\n", sm.size,
    (uint32_t)sizeof(sm));
    return NSID_FAIL;
    }
    if (sm.mask != STATMOUNT_MNT_NS_ID) {
    ksft_print_msg("statmount mnt ns id unavailable\n");
    return NSID_SKIP;
    }
    if (sm.mnt_ns_id != mnt_ns_id) {
    ksft_print_msg("unexpected mnt ns ID: 0x%llx != 0x%llx\n",
    (unsigned long long)sm.mnt_ns_id,
    (unsigned long long)mnt_ns_id);
    return NSID_FAIL;
    }
    return NSID_PASS;
    }
#[no_mangle]
unsafe extern "C" fn _test_statmount_mnt_ns_id_by_fd() -> c_int {
    static int _test_statmount_mnt_ns_id_by_fd(void)
    {
    struct statmount sm;
    uint64_t mnt_ns_id;
    int ret, fd, mounted = 1, status = NSID_ERROR;
    char mnt[] = "/statmount.fd.XXXXXX";
    ret = get_mnt_ns_id("/proc/self/ns/mnt", &mnt_ns_id);
    if (ret != NSID_PASS)
    return ret;
    if (!mkdtemp(mnt)) {
    ksft_print_msg("statmount by fd mnt ns id mkdtemp: %s\n", strerror(errno));
    return NSID_ERROR;
    }
    if (mount(mnt, mnt, core::ptr::null_mut(), MS_BIND, 0)) {
    ksft_print_msg("statmount by fd mnt ns id mount: %s\n", strerror(errno));
    status = NSID_ERROR;
    goto err;
    }
    fd = open(mnt, O_PATH);
    if (fd < 0) {
    ksft_print_msg("statmount by fd mnt ns id open: %s\n", strerror(errno));
    goto err;
    }
    ret = statmount(0, 0, fd, STATMOUNT_MNT_NS_ID, &sm, sizeof(sm), STATMOUNT_BY_FD);
    if (ret == -1) {
    ksft_print_msg("statmount mnt ns id statmount: %s\n", strerror(errno));
    status = NSID_ERROR;
    goto out;
    }
    if (sm.size != sizeof(sm)) {
    ksft_print_msg("unexpected size: %u != %u\n", sm.size,
    (uint32_t)sizeof(sm));
    status = NSID_FAIL;
    goto out;
    }
    if (sm.mask != STATMOUNT_MNT_NS_ID) {
    ksft_print_msg("statmount mnt ns id unavailable\n");
    status = NSID_SKIP;
    goto out;
    }
    if (sm.mnt_ns_id != mnt_ns_id) {
    ksft_print_msg("unexpected mnt ns ID: 0x%llx != 0x%llx\n",
    (unsigned long long)sm.mnt_ns_id,
    (unsigned long long)mnt_ns_id);
    status = NSID_FAIL;
    goto out;
    }
    mounted = 0;
    if (umount2(mnt, MNT_DETACH)) {
    ksft_print_msg("statmount by fd mnt ns id umount2: %s\n", strerror(errno));
    goto out;
    }
    ret = statmount(0, 0, fd, STATMOUNT_MNT_NS_ID, &sm, sizeof(sm), STATMOUNT_BY_FD);
    if (ret == -1) {
    ksft_print_msg("statmount mnt ns id statmount: %s\n", strerror(errno));
    status = NSID_ERROR;
    goto out;
    }
    if (sm.size != sizeof(sm)) {
    ksft_print_msg("unexpected size: %u != %u\n", sm.size,
    (uint32_t)sizeof(sm));
    status = NSID_FAIL;
    goto out;
    }
    if (sm.mask == STATMOUNT_MNT_NS_ID) {
    ksft_print_msg("unexpected STATMOUNT_MNT_NS_ID in mask\n");
    status = NSID_FAIL;
    goto out;
    }
    status = NSID_PASS;
    out:
    close(fd);
    if (mounted)
    umount2(mnt, MNT_DETACH);
    err:
    rmdir(mnt);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_mnt_ns_id() {
    static void test_statmount_mnt_ns_id(void)
    {
    pid_t pid;
    int ret;
    pid = fork();
    if (pid < 0)
    ksft_exit_fail_msg("failed to fork: %s\n", strerror(errno));
// We're the original pid, wait for the result.
    if (pid != 0) {
    ret = wait_for_pid(pid);
    handle_result(ret, "test statmount ns id");
    return;
    }
    ret = setup_namespace();
    if (ret != NSID_PASS)
    exit(ret);
    ret = _test_statmount_mnt_ns_id();
    if (ret != NSID_PASS)
    exit(ret);
    ret = _test_statmount_mnt_ns_id_by_fd();
    exit(ret);
    }
#[no_mangle]
unsafe extern "C" fn validate_external_listmount(pid: pid_t, child_nr_mounts: u64) -> c_int {
    static int validate_external_listmount(pid_t pid, uint64_t child_nr_mounts)
    {
    uint64_t list[256];
    uint64_t mnt_ns_id;
    uint64_t nr_mounts;
    char buf[256];
    int ret;
// Get the mount ns id for our child.
    snprintf(buf, sizeof(buf), "/proc/%lu/ns/mnt", (unsigned long)pid);
    ret = get_mnt_ns_id(buf, &mnt_ns_id);
    nr_mounts = listmount(LSMT_ROOT, mnt_ns_id, 0, list, 256, 0);
    if (nr_mounts == (uint64_t)-1) {
    ksft_print_msg("listmount: %s\n", strerror(errno));
    return NSID_ERROR;
    }
    if (nr_mounts != child_nr_mounts) {
    ksft_print_msg("listmount results is %zi != %zi\n", nr_mounts,
    child_nr_mounts);
    return NSID_FAIL;
    }
// Validate that all of our entries match our mnt_ns_id.
    for (int i = 0; i < nr_mounts; i++) {
    struct statmount sm;
    ret = statmount(list[i], mnt_ns_id, 0, STATMOUNT_MNT_NS_ID, &sm,
    sizeof(sm), 0);
    if (ret < 0) {
    ksft_print_msg("statmount mnt ns id: %s\n", strerror(errno));
    return NSID_ERROR;
    }
    if (sm.mask != STATMOUNT_MNT_NS_ID) {
    ksft_print_msg("statmount mnt ns id unavailable\n");
    return NSID_SKIP;
    }
    if (sm.mnt_ns_id != mnt_ns_id) {
    ksft_print_msg("listmount gave us the wrong ns id: 0x%llx != 0x%llx\n",
    (unsigned long long)sm.mnt_ns_id,
    (unsigned long long)mnt_ns_id);
    return NSID_FAIL;
    }
    }
    return NSID_PASS;
    }
#[no_mangle]
unsafe extern "C" fn test_listmount_ns() {
    static void test_listmount_ns(void)
    {
    uint64_t nr_mounts;
    char pval;
    int child_ready_pipe[2];
    int parent_ready_pipe[2];
    pid_t pid;
    int ret, child_ret;
    if (pipe(child_ready_pipe) < 0)
    ksft_exit_fail_msg("failed to create the child pipe: %s\n",
    strerror(errno));
    if (pipe(parent_ready_pipe) < 0)
    ksft_exit_fail_msg("failed to create the parent pipe: %s\n",
    strerror(errno));
    pid = fork();
    if (pid < 0)
    ksft_exit_fail_msg("failed to fork: %s\n", strerror(errno));
    if (pid == 0) {
    char cval;
    uint64_t list[256];
    close(child_ready_pipe[0]);
    close(parent_ready_pipe[1]);
    ret = setup_namespace();
    if (ret != NSID_PASS)
    exit(ret);
    nr_mounts = listmount(LSMT_ROOT, 0, 0, list, 256, 0);
    if (nr_mounts == (uint64_t)-1) {
    ksft_print_msg("listmount: %s\n", strerror(errno));
    exit(NSID_FAIL);
    }
//
// Tell our parent how many mounts we have, and then wait for it
// to tell us we're done.
//
    if (write(child_ready_pipe[1], &nr_mounts, sizeof(nr_mounts)) !=
    sizeof(nr_mounts))
    ret = NSID_ERROR;
    if (read(parent_ready_pipe[0], &cval, sizeof(cval)) != sizeof(cval))
    ret = NSID_ERROR;
    exit(NSID_PASS);
    }
    close(child_ready_pipe[1]);
    close(parent_ready_pipe[0]);
// Wait until the child has created everything.
    if (read(child_ready_pipe[0], &nr_mounts, sizeof(nr_mounts)) !=
    sizeof(nr_mounts))
    ret = NSID_ERROR;
    ret = validate_external_listmount(pid, nr_mounts);
    if (write(parent_ready_pipe[1], &pval, sizeof(pval)) != sizeof(pval))
    ret = NSID_ERROR;
    child_ret = wait_for_pid(pid);
    if (child_ret != NSID_PASS)
    ret = child_ret;
    handle_result(ret, "test listmount ns id");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int ret;
    ksft_print_header();
    ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
    assert(ret == -1);
    if (errno == ENOSYS)
    ksft_exit_skip("statmount() syscall not supported\n");
    ksft_set_plan(2);
    test_statmount_mnt_ns_id();
    test_listmount_ns();
    if (ksft_get_fail_cnt() + ksft_get_error_cnt() > 0)
    ksft_exit_fail();
    else
    ksft_exit_pass();
    }
