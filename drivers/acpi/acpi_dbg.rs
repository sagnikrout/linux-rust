//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_dbg.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ACPI AML interfacing support
//
// Copyright (C) 2015, Intel Corporation
// Authors: Lv Zheng <lv.zheng@intel.com>
//
// #define DEBUG

    (CIRC_CNT((circ).head, (circ).tail, ACPI_AML_BUF_SIZE))

    (CIRC_CNT_TO_END((circ).head, (circ).tail, ACPI_AML_BUF_SIZE))

    (CIRC_SPACE((circ).head, (circ).tail, ACPI_AML_BUF_SIZE))

    (CIRC_SPACE_TO_END((circ).head, (circ).tail, ACPI_AML_BUF_SIZE))
pub const ACPI_AML_OPENED: c_uint = 0x0001;
pub const ACPI_AML_CLOSED: c_uint = 0x0002;
pub const ACPI_AML_IN_USER: c_uint = 0x0004 /* user space is writing cmd */;
pub const ACPI_AML_IN_KERN: c_uint = 0x0008 /* kernel space is reading cmd */;
pub const ACPI_AML_OUT_USER: c_uint = 0x0010 /* user space is reading log */;
pub const ACPI_AML_OUT_KERN: c_uint = 0x0020 /* kernel space is writing log */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aml_io {
    pub wait: wait_queue_head_t,
    pub flags: c_ulong,
    pub users: c_ulong,
    pub lock: mutex,
    pub thread: *mut task_struct,
    pub __aligned(ACPI_AML_BUF_ALIGN): char out_buf[ACPI_AML_BUF_SIZE],
    pub out_crc: circ_buf,
    pub __aligned(ACPI_AML_BUF_ALIGN): char in_buf[ACPI_AML_BUF_SIZE],
    pub in_crc: circ_buf,
    pub function: acpi_osd_exec_callback,
    pub context: *mut c_void,
    pub usages: c_ulong,
}

    static struct acpi_aml_io acpi_aml_io;
    static bool acpi_aml_initialized;
    static struct file *acpi_aml_active_reader;
    static struct dentry *acpi_aml_dentry;
#[no_mangle]
pub unsafe extern "C" fn __acpi_aml_running() -> bool {
    static inline bool __acpi_aml_running(void)
    {
    return acpi_aml_io.thread ? true : false;
    }
#[no_mangle]
pub unsafe extern "C" fn __acpi_aml_access_ok(flag: c_ulong) -> bool {
    static inline bool __acpi_aml_access_ok(unsigned long flag)
    {
//
// The debugger interface is in opened state (OPENED && !CLOSED),
// then it is allowed to access the debugger buffers from either
// user space or the kernel space.
// In addition, for the kernel space, only the debugger thread
// (thread ID matched) is allowed to access.
//
    if (!(acpi_aml_io.flags & ACPI_AML_OPENED) ||
    (acpi_aml_io.flags & ACPI_AML_CLOSED) ||
    !__acpi_aml_running())
    return false;
    if ((flag & ACPI_AML_KERN) &&
    current != acpi_aml_io.thread)
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __acpi_aml_readable(circ: *mut circ_buf, flag: c_ulong) -> bool {
    static inline bool __acpi_aml_readable(struct circ_buf *circ, unsigned long flag)
    {
//
// Another read is not in progress and there is data in buffer
// available for read.
//
    if (!(acpi_aml_io.flags & flag) && circ_count(circ))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __acpi_aml_writable(circ: *mut circ_buf, flag: c_ulong) -> bool {
    static inline bool __acpi_aml_writable(struct circ_buf *circ, unsigned long flag)
    {
//
// Another write is not in progress and there is buffer space
// available for write.
//
    if (!(acpi_aml_io.flags & flag) && circ_space(circ))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __acpi_aml_busy() -> bool {
    static inline bool __acpi_aml_busy(void)
    {
    if (acpi_aml_io.flags & ACPI_AML_BUSY)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __acpi_aml_used() -> bool {
    static inline bool __acpi_aml_used(void)
    {
    return acpi_aml_io.usages ? true : false;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_aml_running() -> bool {
    static inline bool acpi_aml_running(void)
    {
    bool ret;
    mutex_lock(&acpi_aml_io.lock);
    ret = __acpi_aml_running();
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_busy() -> bool {
    static bool acpi_aml_busy(void)
    {
    bool ret;
    mutex_lock(&acpi_aml_io.lock);
    ret = __acpi_aml_busy();
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_used() -> bool {
    static bool acpi_aml_used(void)
    {
    bool ret;
//
// The usage count is prepared to avoid race conditions between the
// starts and the stops of the debugger thread.
//
    mutex_lock(&acpi_aml_io.lock);
    ret = __acpi_aml_used();
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_kern_readable() -> bool {
    static bool acpi_aml_kern_readable(void)
    {
    bool ret;
    mutex_lock(&acpi_aml_io.lock);
    ret = !__acpi_aml_access_ok(ACPI_AML_IN_KERN) ||
    __acpi_aml_readable(&acpi_aml_io.in_crc, ACPI_AML_IN_KERN);
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_kern_writable() -> bool {
    static bool acpi_aml_kern_writable(void)
    {
    bool ret;
    mutex_lock(&acpi_aml_io.lock);
    ret = !__acpi_aml_access_ok(ACPI_AML_OUT_KERN) ||
    __acpi_aml_writable(&acpi_aml_io.out_crc, ACPI_AML_OUT_KERN);
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_user_readable() -> bool {
    static bool acpi_aml_user_readable(void)
    {
    bool ret;
    mutex_lock(&acpi_aml_io.lock);
    ret = !__acpi_aml_access_ok(ACPI_AML_OUT_USER) ||
    __acpi_aml_readable(&acpi_aml_io.out_crc, ACPI_AML_OUT_USER);
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_user_writable() -> bool {
    static bool acpi_aml_user_writable(void)
    {
    bool ret;
    mutex_lock(&acpi_aml_io.lock);
    ret = !__acpi_aml_access_ok(ACPI_AML_IN_USER) ||
    __acpi_aml_writable(&acpi_aml_io.in_crc, ACPI_AML_IN_USER);
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_lock_write(circ: *mut circ_buf, flag: c_ulong) -> c_int {
    static int acpi_aml_lock_write(struct circ_buf *circ, unsigned long flag)
    {
    let mut ret: c_int = 0;
    mutex_lock(&acpi_aml_io.lock);
    if (!__acpi_aml_access_ok(flag)) {
    ret = -EFAULT;
    goto out;
    }
    if (!__acpi_aml_writable(circ, flag)) {
    ret = -EAGAIN;
    goto out;
    }
    acpi_aml_io.flags |= flag;
    out:
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_lock_read(circ: *mut circ_buf, flag: c_ulong) -> c_int {
    static int acpi_aml_lock_read(struct circ_buf *circ, unsigned long flag)
    {
    let mut ret: c_int = 0;
    mutex_lock(&acpi_aml_io.lock);
    if (!__acpi_aml_access_ok(flag)) {
    ret = -EFAULT;
    goto out;
    }
    if (!__acpi_aml_readable(circ, flag)) {
    ret = -EAGAIN;
    goto out;
    }
    acpi_aml_io.flags |= flag;
    out:
    mutex_unlock(&acpi_aml_io.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_unlock_fifo(flag: c_ulong, wakeup: bool) {
    static void acpi_aml_unlock_fifo(unsigned long flag, bool wakeup)
    {
    mutex_lock(&acpi_aml_io.lock);
    acpi_aml_io.flags &= ~flag;
    if (wakeup)
    wake_up_interruptible(&acpi_aml_io.wait);
    mutex_unlock(&acpi_aml_io.lock);
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_write_kern(buf: *const c_char, len: c_int) -> c_int {
    static int acpi_aml_write_kern(const char *buf, int len)
    {
    int ret;
    struct circ_buf *crc = &acpi_aml_io.out_crc;
    int n;
    char *p;
    ret = acpi_aml_lock_write(crc, ACPI_AML_OUT_KERN);
    if (ret < 0)
    return ret;
// sync tail before inserting logs
    smp_mb();
    p = &crc.buf[crc.head];
    n = min(len, circ_space_to_end(crc));
    memcpy(p, buf, n);
// sync head after inserting logs
    smp_wmb();
    crc.head = (crc.head + n) & (ACPI_AML_BUF_SIZE - 1);
    acpi_aml_unlock_fifo(ACPI_AML_OUT_KERN, true);
    return n;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_readb_kern() -> c_int {
    static int acpi_aml_readb_kern(void)
    {
    int ret;
    struct circ_buf *crc = &acpi_aml_io.in_crc;
    char *p;
    ret = acpi_aml_lock_read(crc, ACPI_AML_IN_KERN);
    if (ret < 0)
    return ret;
// sync head before removing cmds
    smp_rmb();
    p = &crc.buf[crc.tail];
    ret = (int)*p;
// sync tail before inserting cmds
    smp_mb();
    crc.tail = (crc.tail + 1) & (ACPI_AML_BUF_SIZE - 1);
    acpi_aml_unlock_fifo(ACPI_AML_IN_KERN, true);
    return ret;
    }
//
// acpi_aml_write_log() - Capture debugger output
// @msg: the debugger output
//
// This function should be used to implement acpi_os_printf() to filter out
// the debugger output and store the output into the debugger interface
// buffer. Return the size of stored logs or errno.
//
#[no_mangle]
unsafe extern "C" fn acpi_aml_write_log(msg: *const c_char) -> isize {
    static ssize_t acpi_aml_write_log(const char *msg)
    {
    let mut ret: c_int = 0;
    let mut count: c_int = 0, size = 0;
    if (!acpi_aml_initialized)
    return -ENODEV;
    if (msg)
    count = strlen(msg);
    while (count > 0) {
    again:
    ret = acpi_aml_write_kern(msg + size, count);
    if (ret == -EAGAIN) {
    ret = wait_event_interruptible(acpi_aml_io.wait,
    acpi_aml_kern_writable());
//
// We need to retry when the condition
// becomes true.
//
    if (ret == 0)
    goto again;
    break;
    }
    if (ret < 0)
    break;
    size += ret;
    count -= ret;
    }
    return size > 0 ? size : ret;
    }
//
// acpi_aml_read_cmd() - Capture debugger input
// @msg: the debugger input
// @size: the size of the debugger input
//
// This function should be used to implement acpi_os_get_line() to capture
// the debugger input commands and store the input commands into the
// debugger interface buffer. Return the size of stored commands or errno.
//
#[no_mangle]
unsafe extern "C" fn acpi_aml_read_cmd(msg: *mut c_char, count: usize) -> isize {
    static ssize_t acpi_aml_read_cmd(char *msg, size_t count)
    {
    let mut ret: c_int = 0;
    let mut size: c_int = 0;
//
// This is ensured by the running fact of the debugger thread
// unless a bug is introduced.
//
    BUG_ON(!acpi_aml_initialized);
    while (count > 0) {
    again:
//
// Check each input byte to find the end of the command.
//
    ret = acpi_aml_readb_kern();
    if (ret == -EAGAIN) {
    ret = wait_event_interruptible(acpi_aml_io.wait,
    acpi_aml_kern_readable());
//
// We need to retry when the condition becomes
// true.
//
    if (ret == 0)
    goto again;
    }
    if (ret < 0)
    break;
// (msg + size) = (char)ret;
    size++;
    count--;
    if (ret == '\n') {
//
// acpi_os_get_line() requires a zero terminated command
// string.
//
// (msg + size - 1) = '\0';
    break;
    }
    }
    return size > 0 ? size : ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_thread(unused: *mut c_void) -> c_int {
    static int acpi_aml_thread(void *unused)
    {
    let mut function: acpi_osd_exec_callback = core::ptr::null_mut();
    void *context;
    mutex_lock(&acpi_aml_io.lock);
    if (acpi_aml_io.function) {
    acpi_aml_io.usages++;
    function = acpi_aml_io.function;
    context = acpi_aml_io.context;
    }
    mutex_unlock(&acpi_aml_io.lock);
    if (function)
    function(context);
    mutex_lock(&acpi_aml_io.lock);
    acpi_aml_io.usages--;
    if (!__acpi_aml_used()) {
    acpi_aml_io.thread = core::ptr::null_mut();
    wake_up(&acpi_aml_io.wait);
    }
    mutex_unlock(&acpi_aml_io.lock);
    return 0;
    }
//
// acpi_aml_create_thread() - Create AML debugger thread
// @function: the debugger thread callback
// @context: the context to be passed to the debugger thread
//
// This function should be used to implement acpi_os_execute() which is
// used by the ACPICA debugger to create the debugger thread.
//
#[no_mangle]
unsafe extern "C" fn acpi_aml_create_thread(function: acpi_osd_exec_callback, context: *mut c_void) -> c_int {
    static int acpi_aml_create_thread(acpi_osd_exec_callback function, void *context)
    {
    struct task_struct *t;
    mutex_lock(&acpi_aml_io.lock);
    acpi_aml_io.function = function;
    acpi_aml_io.context = context;
    mutex_unlock(&acpi_aml_io.lock);
    t = kthread_create(acpi_aml_thread, core::ptr::null_mut(), "aml");
    if (IS_ERR(t)) {
    pr_err("Failed to create AML debugger thread.\n");
    return PTR_ERR(t);
    }
    mutex_lock(&acpi_aml_io.lock);
    acpi_aml_io.thread = t;
    acpi_set_debugger_thread_id((acpi_thread_id)(unsigned long)t);
    wake_up_process(t);
    mutex_unlock(&acpi_aml_io.lock);
    return 0;
    }
    static int acpi_aml_wait_command_ready(bool single_step,
    char *buffer, size_t length)
    {
    acpi_status status;
    if (single_step)
    acpi_os_printf("\n%1c ", ACPI_DEBUGGER_EXECUTE_PROMPT);
    else
    acpi_os_printf("\n%1c ", ACPI_DEBUGGER_COMMAND_PROMPT);
    status = acpi_os_get_line(buffer, length, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_notify_command_complete() -> c_int {
    static int acpi_aml_notify_command_complete(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_open(inode: *mut inode, file: *mut file) -> c_int {
    static int acpi_aml_open(struct inode *inode, struct file *file)
    {
    let mut ret: c_int = 0;
    acpi_status status;
    mutex_lock(&acpi_aml_io.lock);
//
// The debugger interface is being closed, no new user is allowed
// during this period.
//
    if (acpi_aml_io.flags & ACPI_AML_CLOSED) {
    ret = -EBUSY;
    goto err_lock;
    }
    if ((file.f_flags & O_ACCMODE) != O_WRONLY) {
//
// Only one reader is allowed to initiate the debugger
// thread.
//
    if (acpi_aml_active_reader) {
    ret = -EBUSY;
    goto err_lock;
    } else {
    pr_debug("Opening debugger reader.\n");
    acpi_aml_active_reader = file;
    }
    } else {
//
// No writer is allowed unless the debugger thread is
// ready.
//
    if (!(acpi_aml_io.flags & ACPI_AML_OPENED)) {
    ret = -ENODEV;
    goto err_lock;
    }
    }
    if (acpi_aml_active_reader == file) {
    pr_debug("Opening debugger interface.\n");
    mutex_unlock(&acpi_aml_io.lock);
    pr_debug("Initializing debugger thread.\n");
    status = acpi_initialize_debugger();
    if (ACPI_FAILURE(status)) {
    pr_err("Failed to initialize debugger.\n");
    ret = -EINVAL;
    goto err_exit;
    }
    pr_debug("Debugger thread initialized.\n");
    mutex_lock(&acpi_aml_io.lock);
    acpi_aml_io.flags |= ACPI_AML_OPENED;
    acpi_aml_io.out_crc.head = acpi_aml_io.out_crc.tail = 0;
    acpi_aml_io.in_crc.head = acpi_aml_io.in_crc.tail = 0;
    pr_debug("Debugger interface opened.\n");
    }
    acpi_aml_io.users++;
    err_lock:
    if (ret < 0) {
    if (acpi_aml_active_reader == file)
    acpi_aml_active_reader = core::ptr::null_mut();
    }
    mutex_unlock(&acpi_aml_io.lock);
    err_exit:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_release(inode: *mut inode, file: *mut file) -> c_int {
    static int acpi_aml_release(struct inode *inode, struct file *file)
    {
    mutex_lock(&acpi_aml_io.lock);
    acpi_aml_io.users--;
    if (file == acpi_aml_active_reader) {
    pr_debug("Closing debugger reader.\n");
    acpi_aml_active_reader = core::ptr::null_mut();
    pr_debug("Closing debugger interface.\n");
    acpi_aml_io.flags |= ACPI_AML_CLOSED;
//
// Wake up all user space/kernel space blocked
// readers/writers.
//
    wake_up_interruptible(&acpi_aml_io.wait);
    mutex_unlock(&acpi_aml_io.lock);
//
// Wait all user space/kernel space readers/writers to
// stop so that ACPICA command loop of the debugger thread
// should fail all its command line reads after this point.
//
    wait_event(acpi_aml_io.wait, !acpi_aml_busy());
//
// Then we try to terminate the debugger thread if it is
// not terminated.
//
    pr_debug("Terminating debugger thread.\n");
    acpi_terminate_debugger();
    wait_event(acpi_aml_io.wait, !acpi_aml_used());
    pr_debug("Debugger thread terminated.\n");
    mutex_lock(&acpi_aml_io.lock);
    acpi_aml_io.flags &= ~ACPI_AML_OPENED;
    }
    if (acpi_aml_io.users == 0) {
    pr_debug("Debugger interface closed.\n");
    acpi_aml_io.flags &= ~ACPI_AML_CLOSED;
    }
    mutex_unlock(&acpi_aml_io.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_read_user(buf: *mut char __user, len: usize) -> isize {
    static ssize_t acpi_aml_read_user(char __user *buf, size_t len)
    {
    struct circ_buf *crc = &acpi_aml_io.out_crc;
    ssize_t ret;
    size_t n;
    char *p;
    ret = acpi_aml_lock_read(crc, ACPI_AML_OUT_USER);
    if (ret < 0)
    return ret;
// sync head before removing logs
    smp_rmb();
    p = &crc.buf[crc.tail];
    n = min_t(size_t, len, circ_count_to_end(crc));
    if (copy_to_user(buf, p, n)) {
    ret = -EFAULT;
    goto out;
    }
// sync tail after removing logs
    smp_mb();
    crc.tail = (crc.tail + n) & (ACPI_AML_BUF_SIZE - 1);
    ret = n;
    out:
    acpi_aml_unlock_fifo(ACPI_AML_OUT_USER, ret >= 0);
    return ret;
    }
    static ssize_t acpi_aml_read(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    let mut ret: isize = 0;
    let mut size: isize = 0;
    if (!count)
    return 0;
    if (!access_ok(buf, count))
    return -EFAULT;
    while (count > 0) {
    again:
    ret = acpi_aml_read_user(buf + size, count);
    if (ret == -EAGAIN) {
    if (file.f_flags & O_NONBLOCK)
    break;
    else {
    ret = wait_event_interruptible(acpi_aml_io.wait,
    acpi_aml_user_readable());
//
// We need to retry when the condition
// becomes true.
//
    if (ret == 0)
    goto again;
    }
    }
    if (ret < 0) {
    if (!acpi_aml_running())
    ret = 0;
    break;
    }
    if (ret) {
    size += ret;
    count -= ret;
// ppos += ret;
    break;
    }
    }
    return size > 0 ? size : ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_write_user(buf: *const char __user, len: usize) -> isize {
    static ssize_t acpi_aml_write_user(const char __user *buf, size_t len)
    {
    struct circ_buf *crc = &acpi_aml_io.in_crc;
    ssize_t ret;
    size_t n;
    char *p;
    ret = acpi_aml_lock_write(crc, ACPI_AML_IN_USER);
    if (ret < 0)
    return ret;
// sync tail before inserting cmds
    smp_mb();
    p = &crc.buf[crc.head];
    n = min_t(size_t, len, circ_space_to_end(crc));
    if (copy_from_user(p, buf, n)) {
    ret = -EFAULT;
    goto out;
    }
// sync head after inserting cmds
    smp_wmb();
    crc.head = (crc.head + n) & (ACPI_AML_BUF_SIZE - 1);
    ret = n;
    out:
    acpi_aml_unlock_fifo(ACPI_AML_IN_USER, ret >= 0);
    return ret;
    }
    static ssize_t acpi_aml_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    let mut ret: isize = 0;
    let mut size: isize = 0;
    if (!count)
    return 0;
    if (!access_ok(buf, count))
    return -EFAULT;
    while (count > 0) {
    again:
    ret = acpi_aml_write_user(buf + size, count);
    if (ret == -EAGAIN) {
    if (file.f_flags & O_NONBLOCK)
    break;
    else {
    ret = wait_event_interruptible(acpi_aml_io.wait,
    acpi_aml_user_writable());
//
// We need to retry when the condition
// becomes true.
//
    if (ret == 0)
    goto again;
    }
    }
    if (ret < 0) {
    if (!acpi_aml_running())
    ret = 0;
    break;
    }
    if (ret) {
    size += ret;
    count -= ret;
// ppos += ret;
    }
    }
    return size > 0 ? size : ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t acpi_aml_poll(struct file *file, poll_table *wait)
    {
    let mut masks: __poll_t = 0;
    poll_wait(file, &acpi_aml_io.wait, wait);
    if (acpi_aml_user_readable())
    masks |= EPOLLIN | EPOLLRDNORM;
    if (acpi_aml_user_writable())
    masks |= EPOLLOUT | EPOLLWRNORM;
    return masks;
    }
    static const struct file_operations acpi_aml_operations = {
    .read		= acpi_aml_read,
    .write		= acpi_aml_write,
    .poll		= acpi_aml_poll,
    .open		= acpi_aml_open,
    .release	= acpi_aml_release,
    .llseek		= generic_file_llseek,
    };
    static const struct acpi_debugger_ops acpi_aml_debugger = {
    .create_thread		 = acpi_aml_create_thread,
    .read_cmd		 = acpi_aml_read_cmd,
    .write_log		 = acpi_aml_write_log,
    .wait_command_ready	 = acpi_aml_wait_command_ready,
    .notify_command_complete = acpi_aml_notify_command_complete,
    };
#[no_mangle]
unsafe extern "C" fn acpi_aml_init() -> int __init {
    static int __init acpi_aml_init(void)
    {
    int ret;
    if (acpi_disabled)
    return -ENODEV;
// Initialize AML IO interface
    mutex_init(&acpi_aml_io.lock);
    init_waitqueue_head(&acpi_aml_io.wait);
    acpi_aml_io.out_crc.buf = acpi_aml_io.out_buf;
    acpi_aml_io.in_crc.buf = acpi_aml_io.in_buf;
    acpi_aml_dentry = debugfs_create_file("acpidbg",
    S_IFREG | S_IRUGO | S_IWUSR,
    acpi_debugfs_dir, core::ptr::null_mut(),
    &acpi_aml_operations);
    ret = acpi_register_debugger(THIS_MODULE, &acpi_aml_debugger);
    if (ret) {
    debugfs_remove(acpi_aml_dentry);
    acpi_aml_dentry = core::ptr::null_mut();
    return ret;
    }
    acpi_aml_initialized = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_aml_exit() -> void __exit {
    static void __exit acpi_aml_exit(void)
    {
    if (acpi_aml_initialized) {
    acpi_unregister_debugger(&acpi_aml_debugger);
    debugfs_remove(acpi_aml_dentry);
    acpi_aml_dentry = core::ptr::null_mut();
    acpi_aml_initialized = false;
    }
    }
    module_init(acpi_aml_init);
    module_exit(acpi_aml_exit);
    MODULE_AUTHOR("Lv Zheng");
    MODULE_DESCRIPTION("ACPI debugger userspace IO driver");
    MODULE_LICENSE("GPL");
