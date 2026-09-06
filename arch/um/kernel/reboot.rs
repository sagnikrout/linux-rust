//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/reboot.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

    void (*pm_power_off)(void);
    EXPORT_SYMBOL(pm_power_off);
#[no_mangle]
unsafe extern "C" fn kill_off_processes() {
    static void kill_off_processes(void)
    {
    struct task_struct *p;
    int pid;
    read_lock(&tasklist_lock);
    for_each_process(p) {
    struct task_struct *t;
    t = find_lock_task_mm(p);
    if (!t)
    continue;
    pid = t.mm.context.id.pid;
    task_unlock(t);
    os_kill_ptraced_process(pid, 1);
    }
    read_unlock(&tasklist_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn uml_cleanup() {
    void uml_cleanup(void)
    {
    kmalloc_ok = 0;
    do_uml_exitcalls();
    kill_off_processes();
    }
#[no_mangle]
pub unsafe extern "C" fn machine_restart(__unused: *mut *mut c_char) {
    void machine_restart(char * __unused)
    {
    uml_cleanup();
    reboot_skas();
    }
#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    void machine_power_off(void)
    {
    uml_cleanup();
    halt_skas();
    }
#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    void machine_halt(void)
    {
    machine_power_off();
    }
#[no_mangle]
unsafe extern "C" fn sys_power_off_handler(data: *mut sys_off_data) -> c_int {
    static int sys_power_off_handler(struct sys_off_data *data)
    {
    machine_power_off();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn register_power_off() -> c_int {
    static int register_power_off(void)
    {
    register_sys_off_handler(SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    sys_power_off_handler, core::ptr::null_mut());
    return 0;
    }
    __initcall(register_power_off);
