//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/irq.c
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
// Copyright (C) 2017 - Cambridge Greys Ltd
// Copyright (C) 2011 - 2014 Cisco Systems Inc
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// Epoll support
    let mut epollfd: static int = -1;
pub const MAX_EPOLL_EVENTS: c_int = 64;
    static struct epoll_event epoll_events[MAX_EPOLL_EVENTS];
// Helper to return an Epoll data pointer from an epoll event structure.
// We need to keep this one on the userspace side to keep includes separate
//
    void *os_epoll_get_data_pointer(int index)
    {
    return epoll_events[index].data.ptr;
    }
// Helper to compare events versus the events in the epoll structure.
// Same as above - needs to be on the userspace side
//
#[no_mangle]
pub unsafe extern "C" fn os_epoll_triggered(index: c_int, events: c_int) -> c_int {
    int os_epoll_triggered(int index, int events)
    {
    return epoll_events[index].events & events;
    }
// Helper to set the event mask.
// The event mask is opaque to the kernel side, because it does not have
// access to the right includes/defines for EPOLL constants.
//
#[no_mangle]
pub unsafe extern "C" fn os_event_mask(irq_type: enum um_irq_type) -> c_int {
    int os_event_mask(enum um_irq_type irq_type)
    {
    if (irq_type == IRQ_READ)
    return EPOLLIN | EPOLLPRI | EPOLLERR | EPOLLHUP | EPOLLRDHUP;
    if (irq_type == IRQ_WRITE)
    return EPOLLOUT;
    return 0;
    }
//
// Initial Epoll Setup
//
#[no_mangle]
pub unsafe extern "C" fn os_setup_epoll() -> c_int {
    int os_setup_epoll(void)
    {
    epollfd = epoll_create(MAX_EPOLL_EVENTS);
    return epollfd;
    }
//
// Helper to run the actual epoll_wait
//
#[no_mangle]
pub unsafe extern "C" fn os_waiting_for_events_epoll() -> c_int {
    int os_waiting_for_events_epoll(void)
    {
    int n, err;
    n = epoll_wait(epollfd,
    (struct epoll_event *) &epoll_events, MAX_EPOLL_EVENTS, 0);
    if (n < 0) {
    err = -errno;
    if (errno != EINTR)
    printk(
    UM_KERN_ERR "os_waiting_for_events:"
    " epoll returned %d, error = %s\n", n,
    strerror(errno)
    );
    return err;
    }
    return n;
    }
//
// Helper to add a fd to epoll
//
#[no_mangle]
pub unsafe extern "C" fn os_add_epoll_fd(events: c_int, fd: c_int, data: *mut c_void) -> c_int {
    int os_add_epoll_fd(int events, int fd, void *data)
    {
    struct epoll_event event;
    int result;
    event.data.ptr = data;
    event.events = events | EPOLLET;
    result = epoll_ctl(epollfd, EPOLL_CTL_ADD, fd, &event);
    if ((result) && (errno == EEXIST))
    result = os_mod_epoll_fd(events, fd, data);
    if (result)
    printk("epollctl add err fd %d, %s\n", fd, strerror(errno));
    return result;
    }
//
// Helper to mod the fd event mask and/or data backreference
//
#[no_mangle]
pub unsafe extern "C" fn os_mod_epoll_fd(events: c_int, fd: c_int, data: *mut c_void) -> c_int {
    int os_mod_epoll_fd(int events, int fd, void *data)
    {
    struct epoll_event event;
    int result;
    event.data.ptr = data;
    event.events = events;
    result = epoll_ctl(epollfd, EPOLL_CTL_MOD, fd, &event);
    if (result)
    printk(UM_KERN_ERR
    "epollctl mod err fd %d, %s\n", fd, strerror(errno));
    return result;
    }
//
// Helper to delete the epoll fd
//
#[no_mangle]
pub unsafe extern "C" fn os_del_epoll_fd(fd: c_int) -> c_int {
    int os_del_epoll_fd(int fd)
    {
    struct epoll_event event;
// This is quiet as we use this as IO ON/OFF - so it is often
// invoked on a non-existent fd
//
    return epoll_ctl(epollfd, EPOLL_CTL_DEL, fd, &event);
    }
#[no_mangle]
pub unsafe extern "C" fn os_set_ioignore() {
    void os_set_ioignore(void)
    {
    signal(SIGIO, SIG_IGN);
    }
#[no_mangle]
pub unsafe extern "C" fn os_close_epoll_fd() {
    void os_close_epoll_fd(void)
    {
// Needed so we do not leak an fd when rebooting
    os_close_file(epollfd);
    }
