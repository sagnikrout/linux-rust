//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_pid.c
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
// trace_find_filtered_pid - check if a pid exists in a filtered_pid list
// @filtered_pids: The list of pids to check
// @search_pid: The PID to find in @filtered_pids
//
// Returns true if @search_pid is found in @filtered_pids, and false otherwise.
//
    bool
    trace_find_filtered_pid(struct trace_pid_list *filtered_pids, pid_t search_pid)
    {
    return trace_pid_list_is_set(filtered_pids, search_pid);
    }
//
// trace_ignore_this_task - should a task be ignored for tracing
// @filtered_pids: The list of pids to check
// @filtered_no_pids: The list of pids not to be traced
// @task: The task that should be ignored if not filtered
//
// Checks if @task should be traced or not from @filtered_pids.
// Returns true if @task should *NOT* be traced.
// Returns false if @task should be traced.
//
    bool
    trace_ignore_this_task(struct trace_pid_list *filtered_pids,
    struct trace_pid_list *filtered_no_pids,
    struct task_struct *task)
    {
//
// If filtered_no_pids is not empty, and the task's pid is listed
// in filtered_no_pids, then return true.
// Otherwise, if filtered_pids is empty, that means we can
// trace all tasks. If it has content, then only trace pids
// within filtered_pids.
//
    return (filtered_pids &&
    !trace_find_filtered_pid(filtered_pids, task.pid)) ||
    (filtered_no_pids &&
    trace_find_filtered_pid(filtered_no_pids, task.pid));
    }
//
// trace_filter_add_remove_task - Add or remove a task from a pid_list
// @pid_list: The list to modify
// @self: The current task for fork or NULL for exit
// @task: The task to add or remove
//
// If adding a task, if @self is defined, the task is only added if @self
// is also included in @pid_list. This happens on fork and tasks should
// only be added when the parent is listed. If @self is NULL, then the
// @task pid will be removed from the list, which would happen on exit
// of a task.
//
    void trace_filter_add_remove_task(struct trace_pid_list *pid_list,
    struct task_struct *self,
    struct task_struct *task)
    {
    if (!pid_list)
    return;
// For forks, we only add if the forking task is listed
    if (self) {
    if (!trace_find_filtered_pid(pid_list, self.pid))
    return;
    }
// "self" is set for forks, and NULL for exits
    if (self)
    trace_pid_list_set(pid_list, task.pid);
    else
    trace_pid_list_clear(pid_list, task.pid);
    }
//
// trace_pid_next - Used for seq_file to get to the next pid of a pid_list
// @pid_list: The pid list to show
// @v: The last pid that was shown (+1 the actual pid to let zero be displayed)
// @pos: The position of the file
//
// This is used by the seq_file "next" operation to iterate the pids
// listed in a trace_pid_list structure.
//
// Returns the pid+1 as we want to display pid of zero, but NULL would
// stop the iteration.
//
    void *trace_pid_next(struct trace_pid_list *pid_list, void *v, loff_t *pos)
    {
    let mut pid: c_long = (unsigned long)v;
    unsigned int next;
    (*pos)++;
// pid already is +1 of the actual previous bit
    if (trace_pid_list_next(pid_list, pid, &next) < 0)
    return core::ptr::null_mut();
    pid = next;
// Return pid + 1 to allow zero to be represented
    return (void *)(pid + 1);
    }
//
// trace_pid_start - Used for seq_file to start reading pid lists
// @pid_list: The pid list to show
// @pos: The position of the file
//
// This is used by seq_file "start" operation to start the iteration
// of listing pids.
//
// Returns the pid+1 as we want to display pid of zero, but NULL would
// stop the iteration.
//
    void *trace_pid_start(struct trace_pid_list *pid_list, loff_t *pos)
    {
    unsigned long pid;
    unsigned int first;
    let mut l: loff_t = 0;
    if (trace_pid_list_first(pid_list, &first) < 0)
    return core::ptr::null_mut();
    pid = first;
// Return pid + 1 so that zero can be the exit value
    for (pid++; pid && l < *pos;
    pid = (unsigned long)trace_pid_next(pid_list, (void *)pid, &l))
    ;
    return (void *)pid;
    }
//
// trace_pid_show - show the current pid in seq_file processing
// @m: The seq_file structure to write into
// @v: A void pointer of the pid (+1) value to display
//
// Can be directly used by seq_file operations to display the current
// pid value.
//
#[no_mangle]
pub unsafe extern "C" fn trace_pid_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    int trace_pid_show(struct seq_file *m, void *v)
    {
    let mut pid: c_ulong = (unsigned long)v - 1;
    seq_printf(m, "%lu\n", pid);
    return 0;
    }
// 128 should be much more than enough
pub const PID_BUF_SIZE: c_int = 127;
    int trace_pid_write(struct trace_pid_list *filtered_pids,
    struct trace_pid_list **new_pid_list,
    const char __user *ubuf, size_t cnt)
    {
    struct trace_pid_list *pid_list;
    struct trace_parser parser;
    unsigned long val;
    let mut nr_pids: c_int = 0;
    let mut read: isize = 0;
    ssize_t ret;
    loff_t pos;
    pid_t pid;
    if (trace_parser_get_init(&parser, PID_BUF_SIZE + 1))
    return -ENOMEM;
//
// Always recreate a new array. The write is an all or nothing
// operation. Always create a new array when adding new pids by
// the user. If the operation fails, then the current list is
// not modified.
//
    pid_list = trace_pid_list_alloc();
    if (!pid_list) {
    trace_parser_put(&parser);
    return -ENOMEM;
    }
    if (filtered_pids) {
// copy the current bits to the new max
    ret = trace_pid_list_first(filtered_pids, &pid);
    while (!ret) {
    ret = trace_pid_list_set(pid_list, pid);
    if (ret < 0)
    goto out;
    ret = trace_pid_list_next(filtered_pids, pid + 1, &pid);
    nr_pids++;
    }
    }
    ret = 0;
    while (cnt > 0) {
    pos = 0;
    ret = trace_get_user(&parser, ubuf, cnt, &pos);
    if (ret < 0)
    break;
    read += ret;
    ubuf += ret;
    cnt -= ret;
    if (!trace_parser_loaded(&parser))
    break;
    ret = -EINVAL;
    if (kstrtoul(parser.buffer, 0, &val))
    break;
    pid = (pid_t)val;
    if (trace_pid_list_set(pid_list, pid) < 0) {
    ret = -1;
    break;
    }
    nr_pids++;
    trace_parser_clear(&parser);
    ret = 0;
    }
    out:
    trace_parser_put(&parser);
    if (ret < 0) {
    trace_pid_list_free(pid_list);
    return ret;
    }
    if (!nr_pids) {
// Cleared the list of pids
    trace_pid_list_free(pid_list);
    pid_list = core::ptr::null_mut();
    }
// new_pid_list = pid_list;
    return read;
    }
