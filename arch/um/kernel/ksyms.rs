//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/ksyms.c
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
// Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

    EXPORT_SYMBOL(um_get_signals);
    EXPORT_SYMBOL(um_set_signals);
    EXPORT_SYMBOL(os_stat_fd);
    EXPORT_SYMBOL(os_stat_file);
    EXPORT_SYMBOL(os_access);
    EXPORT_SYMBOL(os_set_exec_close);
    EXPORT_SYMBOL(os_getpid);
    EXPORT_SYMBOL(os_open_file);
    EXPORT_SYMBOL(os_read_file);
    EXPORT_SYMBOL(os_write_file);
    EXPORT_SYMBOL(os_seek_file);
    EXPORT_SYMBOL(os_lock_file);
    EXPORT_SYMBOL(os_ioctl_generic);
    EXPORT_SYMBOL(os_pipe);
    EXPORT_SYMBOL(os_file_type);
    EXPORT_SYMBOL(os_file_mode);
    EXPORT_SYMBOL(os_file_size);
    EXPORT_SYMBOL(os_flush_stdout);
    EXPORT_SYMBOL(os_close_file);
    EXPORT_SYMBOL(os_set_fd_async);
    EXPORT_SYMBOL(os_set_fd_block);
    EXPORT_SYMBOL(helper_wait);
    EXPORT_SYMBOL(os_shutdown_socket);
    EXPORT_SYMBOL(os_create_unix_socket);
    EXPORT_SYMBOL(os_connect_socket);
    EXPORT_SYMBOL(os_accept_connection);
    EXPORT_SYMBOL(os_rcv_fd_msg);
    EXPORT_SYMBOL(run_helper);
    EXPORT_SYMBOL(os_major);
    EXPORT_SYMBOL(os_minor);
    EXPORT_SYMBOL(os_makedev);
    EXPORT_SYMBOL(os_eventfd);
    EXPORT_SYMBOL(os_sendmsg_fds);
    EXPORT_SYMBOL(add_sigio_fd);
    EXPORT_SYMBOL(ignore_sigio_fd);
    EXPORT_SYMBOL(sigio_broken);
    EXPORT_SYMBOL(syscall);
