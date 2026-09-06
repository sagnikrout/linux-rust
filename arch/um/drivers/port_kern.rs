//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/port_kern.c
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
// Copyright (C) 2001 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_list {
    pub list: list_head,
    pub wait_count: core::sync::atomic::AtomicI32,
    pub has_connection: c_int,
    pub done: completion,
    pub port: c_int,
    pub fd: c_int,
    pub lock: spinlock_t,
    pub pending: list_head,
    pub connections: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_dev {
    pub port: *mut port_list,
    pub helper_pid: c_int,
    pub telnetd_pid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct connection {
    pub list: list_head,
    pub fd: c_int,
    pub helper_pid: c_int,
    pub socket: [c_int; 2],
    pub telnetd_pid: c_int,
    pub port: *mut port_list,
}

#[no_mangle]
unsafe extern "C" fn pipe_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pipe_interrupt(int irq, void *data)
    {
    struct connection *conn = data;
    let mut n_fds: c_int = 1, fd = -1;
    ssize_t ret;
    ret = os_rcv_fd_msg(conn.socket[0], &fd, n_fds, &conn.helper_pid,
    sizeof(conn.helper_pid));
    if (ret != sizeof(conn.helper_pid)) {
    if (ret == -EAGAIN)
    return IRQ_NONE;
    printk(KERN_ERR "pipe_interrupt : os_rcv_fd_msg returned %zd\n",
    ret);
    os_close_file(conn.fd);
    }
    list_del(&conn.list);
    conn.fd = fd;
    list_add(&conn.list, &conn.port.connections);
    complete(&conn.port.done);
    return IRQ_HANDLED;
    }

    "****\n" \
    "There are currently no UML consoles waiting for port connections.\n" \
    "Either disconnect from one to make it available or activate some more\n" \
    "by enabling more consoles in the UML /etc/inittab.\n" \
    "****\n"
#[no_mangle]
unsafe extern "C" fn port_accept(port: *mut port_list) -> c_int {
    static int port_accept(struct port_list *port)
    {
    struct connection *conn;
    int fd, socket[2], pid;
    fd = port_connection(port.fd, socket, &pid);
    if (fd < 0) {
    if (fd != -EAGAIN)
    printk(KERN_ERR "port_accept : port_connection "
    "returned %d\n", -fd);
    goto out;
    }
    conn = kmalloc_obj(*conn, GFP_ATOMIC);
    if (conn == core::ptr::null_mut()) {
    printk(KERN_ERR "port_accept : failed to allocate "
    "connection\n");
    goto out_close;
    }
// conn = ((struct connection)
    { .list 	= LIST_HEAD_INIT(conn.list),
    .fd 		= fd,
    .socket  	= { socket[0], socket[1] },
    .telnetd_pid 	= pid,
    .port 	= port });
    if (um_request_irq(TELNETD_IRQ, socket[0], IRQ_READ, pipe_interrupt,
    IRQF_SHARED, "telnetd", conn) < 0) {
    printk(KERN_ERR "port_accept : failed to get IRQ for "
    "telnetd\n");
    goto out_free;
    }
    if (atomic_read(&port.wait_count) == 0) {
    os_write_file(fd, NO_WAITER_MSG, sizeof(NO_WAITER_MSG));
    printk(KERN_ERR "No one waiting for port\n");
    }
    list_add(&conn.list, &port.pending);
    return 1;
    out_free:
    kfree(conn);
    out_close:
    os_close_file(fd);
    os_kill_process(pid, 1);
    out:
    return 0;
    }
    static DEFINE_MUTEX(ports_mutex);
    static LIST_HEAD(ports);
#[no_mangle]
unsafe extern "C" fn port_work_proc(unused: *mut work_struct) {
    static void port_work_proc(struct work_struct *unused)
    {
    struct port_list *port;
    struct list_head *ele;
    unsigned long flags;
    local_irq_save(flags);
    list_for_each(ele, &ports) {
    port = list_entry(ele, struct port_list, list);
    if (!port.has_connection)
    continue;
    while (port_accept(port))
    ;
    port.has_connection = 0;
    }
    local_irq_restore(flags);
    }
    static DECLARE_WORK(port_work, port_work_proc);
#[no_mangle]
unsafe extern "C" fn port_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t port_interrupt(int irq, void *data)
    {
    struct port_list *port = data;
    port.has_connection = 1;
    schedule_work(&port_work);
    return IRQ_HANDLED;
    }
    void *port_data(int port_num)
    {
    struct list_head *ele;
    struct port_list *port;
    struct port_dev *dev = core::ptr::null_mut();
    int fd;
    mutex_lock(&ports_mutex);
    list_for_each(ele, &ports) {
    port = list_entry(ele, struct port_list, list);
    if (port.port == port_num)
    goto found;
    }
    port = kmalloc_obj(struct port_list);
    if (port == core::ptr::null_mut()) {
    printk(KERN_ERR "Allocation of port list failed\n");
    goto out;
    }
    fd = port_listen_fd(port_num);
    if (fd < 0) {
    printk(KERN_ERR "binding to port %d failed, errno = %d\n",
    port_num, -fd);
    goto out_free;
    }
    if (um_request_irq(ACCEPT_IRQ, fd, IRQ_READ, port_interrupt,
    IRQF_SHARED, "port", port) < 0) {
    printk(KERN_ERR "Failed to get IRQ for port %d\n", port_num);
    goto out_close;
    }
// port = ((struct port_list)
    { .list 	 	= LIST_HEAD_INIT(port.list),
    .wait_count		= ATOMIC_INIT(0),
    .has_connection 	= 0,
    .port 	 	= port_num,
    .fd  			= fd,
    .pending 		= LIST_HEAD_INIT(port.pending),
    .connections 		= LIST_HEAD_INIT(port.connections) });
    spin_lock_init(&port.lock);
    init_completion(&port.done);
    list_add(&port.list, &ports);
    found:
    dev = kmalloc_obj(struct port_dev);
    if (dev == core::ptr::null_mut()) {
    printk(KERN_ERR "Allocation of port device entry failed\n");
    goto out;
    }
// dev = ((struct port_dev) { .port  		= port,
    .helper_pid  	= -1,
    .telnetd_pid  	= -1 });
    goto out;
    out_close:
    os_close_file(fd);
    out_free:
    kfree(port);
    out:
    mutex_unlock(&ports_mutex);
    return dev;
    }
#[no_mangle]
pub unsafe extern "C" fn port_wait(data: *mut c_void) -> c_int {
    int port_wait(void *data)
    {
    struct port_dev *dev = data;
    struct connection *conn;
    struct port_list *port = dev.port;
    int fd;
    atomic_inc(&port.wait_count);
    while (1) {
    fd = -ERESTARTSYS;
    if (wait_for_completion_interruptible(&port.done))
    goto out;
    spin_lock(&port.lock);
    conn = list_entry(port.connections.next, struct connection,
    list);
    list_del(&conn.list);
    spin_unlock(&port.lock);
    os_shutdown_socket(conn.socket[0], 1, 1);
    os_close_file(conn.socket[0]);
    os_shutdown_socket(conn.socket[1], 1, 1);
    os_close_file(conn.socket[1]);
// This is done here because freeing an IRQ can't be done
// within the IRQ handler.  So, pipe_interrupt always ups
// the semaphore regardless of whether it got a successful
// connection.  Then we loop here throwing out failed
// connections until a good one is found.
//
    um_free_irq(TELNETD_IRQ, conn);
    if (conn.fd >= 0)
    break;
    os_close_file(conn.fd);
    kfree(conn);
    }
    fd = conn.fd;
    dev.helper_pid = conn.helper_pid;
    dev.telnetd_pid = conn.telnetd_pid;
    kfree(conn);
    out:
    atomic_dec(&port.wait_count);
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn port_remove_dev(d: *mut c_void) {
    void port_remove_dev(void *d)
    {
    struct port_dev *dev = d;
    if (dev.helper_pid != -1)
    os_kill_process(dev.helper_pid, 0);
    if (dev.telnetd_pid != -1)
    os_kill_process(dev.telnetd_pid, 1);
    dev.helper_pid = -1;
    dev.telnetd_pid = -1;
    }
#[no_mangle]
pub unsafe extern "C" fn port_kern_free(d: *mut c_void) {
    void port_kern_free(void *d)
    {
    struct port_dev *dev = d;
    port_remove_dev(dev);
    kfree(dev);
    }
#[no_mangle]
unsafe extern "C" fn free_port() {
    static void free_port(void)
    {
    struct list_head *ele;
    struct port_list *port;
    list_for_each(ele, &ports) {
    port = list_entry(ele, struct port_list, list);
    free_irq_by_fd(port.fd);
    os_close_file(port.fd);
    }
    }
    __uml_exitcall(free_port);
