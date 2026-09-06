//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/test_progs.h
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

pub type __sum16 = __u16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum verbosity {
    VERBOSE_NONE,
    VERBOSE_NORMAL,
    VERBOSE_VERY,
    VERBOSE_SUPER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_filter {
    pub name: *mut c_char,
    pub subtests: *mut c_char,
    pub subtest_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_filter_set {
    pub tests: *mut test_filter,
    pub cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_selector {
    pub whitelist: test_filter_set,
    pub blacklist: test_filter_set,
    pub num_set: *mut bool,
    pub num_set_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subtest_state {
    pub name: *mut c_char,
    pub log_cnt: usize,
    pub log_buf: *mut c_char,
    pub error_cnt: c_int,
    pub skipped: bool,
    pub filtered: bool,
    pub should_tmon: bool,
    pub stdout_saved: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_state {
    pub tested: bool,
    pub force_log: bool,
    pub error_cnt: c_int,
    pub skip_cnt: c_int,
    pub sub_succ_cnt: c_int,
    pub subtest_states: *mut subtest_state,
    pub subtest_num: c_int,
    pub log_cnt: usize,
    pub log_buf: *mut c_char,
    pub stdout_saved: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_env {
    pub test_selector: test_selector,
    pub subtest_selector: test_selector,
    pub tmon_selector: test_selector,
    pub verifier_stats: bool,
    pub debug: bool,
    pub error_summary: bool,
    pub verbosity: verbosity,
    pub jit_enabled: bool,
    pub has_testmod: bool,
    pub get_test_cnt: bool,
    pub list_test_names: bool,
    pub /: *mut *mut *mut prog_test_def test; / current running test,
    pub /: *mut *mut *mut test_state test_state; / current running test state,
    pub /: *mut *mut *mut subtest_state subtest_state; / current running subtest state,
    pub stdout_saved: *mut FILE,
    pub stderr_saved: *mut FILE,
    pub nr_cpus: c_int,
    pub json: *mut FILE,
    pub /: *mut *mut int succ_cnt; / successful tests,
    pub /: *mut *mut int sub_succ_cnt; / successful sub-tests,
    pub /: *mut *mut int fail_cnt; / failed tests,
    pub /: *mut *mut int skip_cnt; / skipped tests,
    pub /: *mut *mut int not_built_cnt; / tests not built,
    pub saved_netns_fd: c_int,
    pub /: *mut *mut int workers; / number of worker process,
    pub /: *mut *mut int worker_id; / id number of current worker, main process is -1,
    pub /: *mut *mut *mut pid_t worker_pids; / array of worker pids,
    pub /: *mut *mut *mut int worker_socks; / array of worker socks,
    pub /: *mut *mut *mut int worker_current_test; / array of current running test for each worker,
    pub main_thread: pthread_t,
    pub secs_till_notify: c_int,
    pub secs_till_kill: c_int,
    pub /: *mut *mut timer_t watchdog; / watch for stalled tests/subtests,
    pub watchdog_state: { WD_NOTIFY, WD_KILL },
}

pub const MAX_LOG_TRUNK_SIZE: c_int = 8192;
pub const MAX_SUBTEST_NAME: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msg_type {
    MSG_DO_TEST = 0,
    MSG_TEST_DONE = 1,
    MSG_TEST_LOG = 2,
    MSG_SUBTEST_DONE = 3,
    MSG_EXIT = 255,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg {
    pub type: msg_type,
    pub num: c_int,
    pub do_test: },
    pub num: c_int,
    pub sub_succ_cnt: c_int,
    pub error_cnt: c_int,
    pub skip_cnt: c_int,
    pub have_log: bool,
    pub subtest_num: c_int,
    pub test_done: },
    pub 1]: char log_buf[MAX_LOG_TRUNK_SIZE +,
    pub is_last: bool,
    pub test_log: },
    pub num: c_int,
    pub 1]: char name[MAX_SUBTEST_NAME +,
    pub error_cnt: c_int,
    pub skipped: bool,
    pub filtered: bool,
    pub have_log: bool,
    pub subtest_done: },
}

extern "C" {
    pub fn test_log();
}
extern "C" {
    pub fn test__start_subtest_with_desc(name: *const c_char, description: *const c_char) -> bool;
}
extern "C" {
    pub fn test__start_subtest(name: *const c_char) -> bool;
}
extern "C" {
    pub fn test__end_subtest();
}
extern "C" {
    pub fn test__skip();
}
extern "C" {
    pub fn test__fail();
}
extern "C" {
    pub fn test__join_cgroup(path: *const c_char) -> c_int;
}
extern "C" {
    pub fn hexdump(prefix: *const c_char, buf: *const c_void, len: usize);
}

extern "C" {
    pub fn start_libbpf_log_capture() -> c_int;
}
extern "C" {
    pub fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn compare_map_keys(map1_fd: c_int, map2_fd: c_int) -> c_int;
}
extern "C" {
    pub fn compare_stack_ips(smap_fd: c_int, amap_fd: c_int, stack_trace_len: c_int) -> c_int;
}
extern "C" {
    pub fn trigger_module_test_read(read_sz: c_int) -> c_int;
}
extern "C" {
    pub fn trigger_module_test_write(write_sz: c_int) -> c_int;
}
extern "C" {
    pub fn write_sysctl(sysctl: *const c_char, value: *const c_char) -> c_int;
}
extern "C" {
    pub fn get_bpf_max_tramp_links_from(btf: *mut btf) -> c_int;
}
extern "C" {
    pub fn get_bpf_max_tramp_links() -> c_int;
}
extern "C" {
    pub fn netns_free(netns: *mut netns_obj);
}

extern "C" {
    pub fn int(obj: *mut *mut pre_execution_cb)(struct bpf_object) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_loader {
    pub log_buf: *mut c_char,
    pub log_buf_sz: usize,
    pub pre_execution_cb: pre_execution_cb,
    pub obj: *mut bpf_object,
}

extern "C" {
    pub fn test_loader_fini(tester: *mut test_loader);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expect_msg {
    pub /: *const *const *const char substr; / substring match,
    pub regex: regex_t,
    pub is_regex: bool,
    pub on_next_line: bool,
    pub negative: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expected_msgs {
    pub patterns: *mut expect_msg,
    pub cnt: usize,
}

extern "C" {
    pub fn free_msgs(msgs: *mut expected_msgs);
}
extern "C" {
    pub fn verify_test_stderr(obj: *mut bpf_object, prog: *mut bpf_program);
}
