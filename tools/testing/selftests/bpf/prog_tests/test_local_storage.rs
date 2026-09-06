//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_local_storage.c
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
// Copyright (C) 2020 Google LLC.
//

pub const TEST_STORAGE_VALUE: c_uint = 0xbeefdead;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct storage {
    pub inode: *mut c_void,
    pub value: c_uint,
}

// Fork and exec the provided rm binary and return the exit code of the
// forked process and its pid.
//
#[no_mangle]
unsafe extern "C" fn run_self_unlink(skel: *mut local_storage, rm_path: *const c_char) -> c_int {
    static int run_self_unlink(struct local_storage *skel, const char *rm_path)
    {
    int child_pid, child_status, ret;
    int null_fd;
    child_pid = fork();
    if (child_pid == 0) {
    null_fd = open("/dev/null", O_WRONLY);
    dup2(null_fd, STDOUT_FILENO);
    dup2(null_fd, STDERR_FILENO);
    close(null_fd);
    skel.bss.monitored_pid = getpid();
// Use the copied /usr/bin/rm to delete itself
// /tmp/copy_of_rm /tmp/copy_of_rm.
//
    ret = execlp(rm_path, rm_path, rm_path, core::ptr::null_mut());
    if (ret)
    exit(errno);
    } else if (child_pid > 0) {
    waitpid(child_pid, &child_status, 0);
    ASSERT_EQ(skel.data.task_storage_result, 0, "task_storage_result");
    return WEXITSTATUS(child_status);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn check_syscall_operations(map_fd: c_int, obj_fd: c_int) -> bool {
    static bool check_syscall_operations(int map_fd, int obj_fd)
    {
    struct storage val = { .value = TEST_STORAGE_VALUE },
    lookup_val = { .value = 0 };
    int err;
// Looking up an existing element should fail initially
    err = bpf_map_lookup_elem_flags(map_fd, &obj_fd, &lookup_val, 0);
    if (!ASSERT_EQ(err, -ENOENT, "bpf_map_lookup_elem"))
    return false;
// Create a new element
    err = bpf_map_update_elem(map_fd, &obj_fd, &val, BPF_NOEXIST);
    if (!ASSERT_OK(err, "bpf_map_update_elem"))
    return false;
// Lookup the newly created element
    err = bpf_map_lookup_elem_flags(map_fd, &obj_fd, &lookup_val, 0);
    if (!ASSERT_OK(err, "bpf_map_lookup_elem"))
    return false;
// Check the value of the newly created element
    if (!ASSERT_EQ(lookup_val.value, val.value, "bpf_map_lookup_elem"))
    return false;
    err = bpf_map_delete_elem(map_fd, &obj_fd);
    if (!ASSERT_OK(err, "bpf_map_delete_elem()"))
    return false;
// The lookup should fail, now that the element has been deleted
    err = bpf_map_lookup_elem_flags(map_fd, &obj_fd, &lookup_val, 0);
    if (!ASSERT_EQ(err, -ENOENT, "bpf_map_lookup_elem"))
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_local_storage() {
    void test_test_local_storage(void)
    {
    char tmp_dir_path[] = "/tmp/local_storageXXXXXX";
    int err, serv_sk = -1, task_fd = -1, rm_fd = -1;
    struct local_storage *skel = core::ptr::null_mut();
    char tmp_exec_path[64];
    char cmd[256];
    skel = local_storage__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    goto close_prog;
    err = local_storage__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto close_prog;
    task_fd = sys_pidfd_open(getpid(), 0);
    if (!ASSERT_GE(task_fd, 0, "pidfd_open"))
    goto close_prog;
    if (!check_syscall_operations(bpf_map__fd(skel.maps.task_storage_map),
    task_fd))
    goto close_prog;
    if (!ASSERT_OK_PTR(mkdtemp(tmp_dir_path), "mkdtemp"))
    goto close_prog;
    snprintf(tmp_exec_path, sizeof(tmp_exec_path), "%s/copy_of_rm",
    tmp_dir_path);
    snprintf(cmd, sizeof(cmd), "cp /bin/rm %s", tmp_exec_path);
    if (!ASSERT_OK(system(cmd), "system(cp)"))
    goto close_prog_rmdir;
    rm_fd = open(tmp_exec_path, O_RDONLY);
    if (!ASSERT_GE(rm_fd, 0, "open(tmp_exec_path)"))
    goto close_prog_rmdir;
    if (!check_syscall_operations(bpf_map__fd(skel.maps.inode_storage_map),
    rm_fd))
    goto close_prog_rmdir;
// Sets skel->bss->monitored_pid to the pid of the forked child
// forks a child process that executes tmp_exec_path and tries to
// unlink its executable. This operation should be denied by the loaded
// LSM program.
//
    err = run_self_unlink(skel, tmp_exec_path);
    if (!ASSERT_EQ(err, EPERM, "run_self_unlink"))
    goto close_prog_rmdir;
// Set the process being monitored to be the current process
    skel.bss.monitored_pid = getpid();
// Move copy_of_rm to a new location so that it triggers the
// inode_rename LSM hook with a new_dentry that has a NULL inode ptr.
//
    snprintf(cmd, sizeof(cmd), "mv %s/copy_of_rm %s/check_null_ptr",
    tmp_dir_path, tmp_dir_path);
    if (!ASSERT_OK(system(cmd), "system(mv)"))
    goto close_prog_rmdir;
    ASSERT_EQ(skel.data.inode_storage_result, 0, "inode_storage_result");
    serv_sk = start_server(AF_INET6, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (!ASSERT_GE(serv_sk, 0, "start_server"))
    goto close_prog_rmdir;
    ASSERT_EQ(skel.data.sk_storage_result, 0, "sk_storage_result");
    if (!check_syscall_operations(bpf_map__fd(skel.maps.sk_storage_map),
    serv_sk))
    goto close_prog_rmdir;
    close_prog_rmdir:
    snprintf(cmd, sizeof(cmd), "rm -rf %s", tmp_dir_path);
    system(cmd);
    close_prog:
    close(serv_sk);
    close(rm_fd);
    close(task_fd);
    local_storage__destroy(skel);
    }
