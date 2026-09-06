//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/profiler.h
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
// Copyright (c) 2020 Facebook

pub const TASK_COMM_LEN: c_int = 16;
pub const MAX_ANCESTORS: c_int = 4;
pub const MAX_PATH: c_int = 256;
pub const KILL_TARGET_LEN: c_int = 64;
pub const CTL_MAXNAME: c_int = 10;
pub const MAX_ARGS_LEN: c_int = 4096;
pub const MAX_FILENAME_LEN: c_int = 512;
pub const MAX_ENVIRON_LEN: c_int = 8192;
pub const MAX_PATH_DEPTH: c_int = 32;

pub const MAX_CGROUPS_PATH_DEPTH: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_type {
    INVALID_EVENT,
    EXEC_EVENT,
    FORK_EVENT,
    KILL_EVENT,
    SYSCTL_EVENT,
    FILEMOD_EVENT,
    MAX_DATA_TYPE_EVENT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum filemod_type {
    FMOD_OPEN,
    FMOD_LINK,
    FMOD_SYMLINK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ancestors_data_t {
    pub ancestor_pids: [pid_t; MAX_ANCESTORS],
    pub ancestor_exec_ids: [u32; MAX_ANCESTORS],
    pub ancestor_start_times: [u64; MAX_ANCESTORS],
    pub num_ancestors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_metadata_t {
    pub type: data_type,
    pub pid: pid_t,
    pub exec_id: u32,
    pub uid: uid_t,
    pub gid: gid_t,
    pub start_time: u64,
    pub cpu_id: u32,
    pub bpf_stats_num_perf_events: u64,
    pub bpf_stats_start_ktime_ns: u64,
    pub comm_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_data_t {
    pub cgroup_root_inode: ino_t,
    pub cgroup_proc_inode: ino_t,
    pub cgroup_root_mtime: u64,
    pub cgroup_proc_mtime: u64,
    pub cgroup_root_length: u16,
    pub cgroup_proc_length: u16,
    pub cgroup_full_length: u16,
    pub cgroup_full_path_root_pos: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_sysctl_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub ancestors_info: ancestors_data_t,
    pub sysctl_val_length: u8,
    pub sysctl_path_length: u16,
    pub payload: [c_char; MAX_SYSCTL_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_kill_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub ancestors_info: ancestors_data_t,
    pub kill_target_pid: pid_t,
    pub kill_sig: c_int,
    pub kill_count: u32,
    pub last_kill_time: u64,
    pub kill_target_name_length: u8,
    pub kill_target_cgroup_proc_length: u8,
    pub payload: [c_char; MAX_KILL_PAYLOAD_LEN],
    pub payload_length: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_exec_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub parent_pid: pid_t,
    pub parent_exec_id: u32,
    pub parent_uid: uid_t,
    pub parent_start_time: u64,
    pub bin_path_length: u16,
    pub cmdline_length: u16,
    pub environment_length: u16,
    pub payload: [c_char; MAX_EXEC_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_fork_data_t {
    pub meta: var_metadata_t,
    pub parent_pid: pid_t,
    pub parent_exec_id: u32,
    pub parent_start_time: u64,
    pub payload: [c_char; MAX_METADATA_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_filemod_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub fmod_type: filemod_type,
    pub dst_flags: c_uint,
    pub src_device_id: u32,
    pub dst_device_id: u32,
    pub src_inode: ino_t,
    pub dst_inode: ino_t,
    pub src_filepath_length: u16,
    pub dst_filepath_length: u16,
    pub payload: [c_char; MAX_FILEMOD_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct profiler_config_struct {
    pub fetch_cgroups_from_bpf: bool,
    pub cgroup_fs_inode: ino_t,
    pub cgroup_login_session_inode: ino_t,
    pub kill_signals_mask: u64,
    pub inode_filter: ino_t,
    pub stale_info_secs: u32,
    pub use_variable_buffers: bool,
    pub read_environ_from_exec: bool,
    pub enable_cgroup_v1_resolver: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_stats_data {
    pub time_elapsed_ns: u64,
    pub num_executions: u64,
    pub num_perf_events: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_stats_ctx {
    pub start_time_ns: u64,
    pub bpf_func_stats_data_val: *mut *mut bpf_func_stats_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_function_id {
    profiler_bpf_proc_sys_write,
    profiler_bpf_sched_process_exec,
    profiler_bpf_sched_process_exit,
    profiler_bpf_sys_enter_kill,
    profiler_bpf_do_file_open_ret,
    profiler_bpf_sched_process_fork,
    profiler_bpf_vfs_link,
    profiler_bpf_vfs_symlink,
    profiler_bpf_max_function_id
}
