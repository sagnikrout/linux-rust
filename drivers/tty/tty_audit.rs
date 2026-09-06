//! Automatically rewritten from C to Rust
//! Source: drivers/tty/tty_audit.c
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
// Creating audit events from TTY input.
//
// Copyright (C) 2007 Red Hat, Inc.  All rights reserved.
//
// Authors: Miloslav Trmac <mitr@redhat.com>
//

pub const TTY_AUDIT_BUF_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_audit_buf {
    pub /: *mut *mut mutex mutex; / Protects all data below,
    pub /: *mut *mut dev_t dev; / The TTY which the data is from,
    pub icanon: bool,
    pub valid: usize,
    pub /: *mut *mut *mut u8 data; / Allocated size TTY_AUDIT_BUF_SIZE,
}

    static struct tty_audit_buf *tty_audit_buf_ref(void)
    {
    struct tty_audit_buf *buf;
    buf = current.signal.tty_audit_buf;
    WARN_ON(buf == ERR_PTR(-ESRCH));
    return buf;
    }
    static struct tty_audit_buf *tty_audit_buf_alloc(void)
    {
    struct tty_audit_buf *buf;
    buf = kzalloc_obj(*buf);
    if (!buf)
    goto err;
    buf.data = kmalloc(TTY_AUDIT_BUF_SIZE, GFP_KERNEL);
    if (!buf.data)
    goto err_buf;
    mutex_init(&buf.mutex);
    return buf;
    err_buf:
    kfree(buf);
    err:
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn tty_audit_buf_free(buf: *mut tty_audit_buf) {
    static void tty_audit_buf_free(struct tty_audit_buf *buf)
    {
    WARN_ON(buf.valid != 0);
    kfree(buf.data);
    kfree(buf);
    }
    static void tty_audit_log(const char *description, dev_t dev,
    const u8 *data, size_t size)
    {
    struct audit_buffer *ab;
    let mut pid: pid_t = task_pid_nr(current);
    let mut uid: uid_t = from_kuid(&init_user_ns, task_uid(current));
    let mut loginuid: uid_t = from_kuid(&init_user_ns, audit_get_loginuid(current));
    let mut sessionid: c_uint = audit_get_sessionid(current);
    char name[TASK_COMM_LEN];
    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_TTY);
    if (!ab)
    return;
    audit_log_format(ab, "%s pid=%u uid=%u auid=%u ses=%u major=%d minor=%d comm=",
    description, pid, uid, loginuid, sessionid,
    MAJOR(dev), MINOR(dev));
    get_task_comm(name, current);
    audit_log_untrustedstring(ab, name);
    audit_log_format(ab, " data=");
    audit_log_n_hex(ab, data, size);
    audit_log_end(ab);
    }
//
// tty_audit_buf_push	-	Push buffered data out
//
// Generate an audit message from the contents of @buf, which is owned by
// the current task.  @buf->mutex must be locked.
//
#[no_mangle]
unsafe extern "C" fn tty_audit_buf_push(buf: *mut tty_audit_buf) {
    static void tty_audit_buf_push(struct tty_audit_buf *buf)
    {
    if (buf.valid == 0)
    return;
    if (audit_enabled == AUDIT_OFF) {
    buf.valid = 0;
    return;
    }
    tty_audit_log("tty", buf.dev, buf.data, buf.valid);
    buf.valid = 0;
    }
//
// tty_audit_exit	-	Handle a task exit
//
// Make sure all buffered data is written out and deallocate the buffer.
// Only needs to be called if current->signal->tty_audit_buf != %NULL.
//
// The process is single-threaded at this point; no other threads share
// current->signal.
//
#[no_mangle]
pub unsafe extern "C" fn tty_audit_exit() {
    void tty_audit_exit(void)
    {
    struct tty_audit_buf *buf;
    buf = xchg(&current.signal.tty_audit_buf, ERR_PTR(-ESRCH));
    if (!buf)
    return;
    tty_audit_buf_push(buf);
    tty_audit_buf_free(buf);
    }
//
// tty_audit_fork	-	Copy TTY audit state for a new task
//
// Set up TTY audit state in @sig from current.  @sig needs no locking.
//
#[no_mangle]
pub unsafe extern "C" fn tty_audit_fork(sig: *mut signal_struct) {
    void tty_audit_fork(struct signal_struct *sig)
    {
    sig.audit_tty = current.signal.audit_tty;
    }
//
// tty_audit_tiocsti	-	Log TIOCSTI
//
#[no_mangle]
pub unsafe extern "C" fn tty_audit_tiocsti(tty: *const tty_struct, ch: u8) {
    void tty_audit_tiocsti(const struct tty_struct *tty, u8 ch)
    {
    dev_t dev;
    dev = MKDEV(tty.driver.major, tty.driver.minor_start) + tty.index;
    if (tty_audit_push())
    return;
    if (audit_enabled)
    tty_audit_log("ioctl=TIOCSTI", dev, &ch, 1);
    }
//
// tty_audit_push	-	Flush current's pending audit data
//
// Returns 0 if success, -EPERM if tty audit is disabled
//
#[no_mangle]
pub unsafe extern "C" fn tty_audit_push() -> c_int {
    int tty_audit_push(void)
    {
    struct tty_audit_buf *buf;
    if (~current.signal.audit_tty & AUDIT_TTY_ENABLE)
    return -EPERM;
    buf = tty_audit_buf_ref();
    if (!IS_ERR_OR_NULL(buf)) {
    mutex_lock(&buf.mutex);
    tty_audit_buf_push(buf);
    mutex_unlock(&buf.mutex);
    }
    return 0;
    }
//
// tty_audit_buf_get	-	Get an audit buffer.
//
// Get an audit buffer, allocate it if necessary.  Return %NULL
// if out of memory or ERR_PTR(-ESRCH) if tty_audit_exit() has already
// occurred.  Otherwise, return a new reference to the buffer.
//
    static struct tty_audit_buf *tty_audit_buf_get(void)
    {
    struct tty_audit_buf *buf;
    buf = tty_audit_buf_ref();
    if (buf)
    return buf;
    buf = tty_audit_buf_alloc();
    if (buf == core::ptr::null_mut()) {
    audit_log_lost("out of memory in TTY auditing");
    return core::ptr::null_mut();
    }
// Race to use this buffer, free it if another wins
    if (cmpxchg(&current.signal.tty_audit_buf, core::ptr::null_mut(), buf) != core::ptr::null_mut())
    tty_audit_buf_free(buf);
    return tty_audit_buf_ref();
    }
//
// tty_audit_add_data	-	Add data for TTY auditing.
//
// Audit @data of @size from @tty, if necessary.
//
    void tty_audit_add_data(const struct tty_struct *tty, const void *data,
    size_t size)
    {
    struct tty_audit_buf *buf;
    unsigned int audit_tty;
    let mut icanon: bool = L_ICANON(tty);
    dev_t dev;
    audit_tty = READ_ONCE(current.signal.audit_tty);
    if (~audit_tty & AUDIT_TTY_ENABLE)
    return;
    if (unlikely(size == 0))
    return;
    if (tty.driver.type == TTY_DRIVER_TYPE_PTY
    && tty.driver.subtype == PTY_TYPE_MASTER)
    return;
    if ((~audit_tty & AUDIT_TTY_LOG_PASSWD) && icanon && !L_ECHO(tty))
    return;
    buf = tty_audit_buf_get();
    if (IS_ERR_OR_NULL(buf))
    return;
    mutex_lock(&buf.mutex);
    dev = MKDEV(tty.driver.major, tty.driver.minor_start) + tty.index;
    if (buf.dev != dev || buf.icanon != icanon) {
    tty_audit_buf_push(buf);
    buf.dev = dev;
    buf.icanon = icanon;
    }
    do {
    size_t run;
    run = TTY_AUDIT_BUF_SIZE - buf.valid;
    if (run > size)
    run = size;
    memcpy(buf.data + buf.valid, data, run);
    buf.valid += run;
    data += run;
    size -= run;
    if (buf.valid == TTY_AUDIT_BUF_SIZE)
    tty_audit_buf_push(buf);
    } while (size != 0);
    mutex_unlock(&buf.mutex);
    }
