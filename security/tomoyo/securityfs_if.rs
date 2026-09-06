//! Automatically rewritten from C to Rust
//! Source: security/tomoyo/securityfs_if.c
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
// security/tomoyo/securityfs_if.c
//
// Copyright (C) 2005-2011  NTT DATA CORPORATION
//

//
// tomoyo_check_task_acl - Check permission for task operation.
//
// @r:   Pointer to "struct tomoyo_request_info".
// @ptr: Pointer to "struct tomoyo_acl_info".
//
// Returns true if granted, false otherwise.
//
    static bool tomoyo_check_task_acl(struct tomoyo_request_info *r,
    const struct tomoyo_acl_info *ptr)
    {
    const struct tomoyo_task_acl *acl = container_of(ptr, typeof(*acl),
    head);
    return !tomoyo_pathcmp(r.param.task.domainname, acl.domainname);
    }
//
// tomoyo_write_self - write() for /sys/kernel/security/tomoyo/self_domain interface.
//
// @file:  Pointer to "struct file".
// @buf:   Domainname to transit to.
// @count: Size of @buf.
// @ppos:  Unused.
//
// Returns @count on success, negative value otherwise.
//
// If domain transition was permitted but the domain transition failed, this
// function returns error rather than terminating current thread with SIGKILL.
//
    static ssize_t tomoyo_write_self(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    char *data;
    int error;
    if (!count || count >= TOMOYO_EXEC_TMPSIZE - 10)
    return -ENOMEM;
    data = memdup_user_nul(buf, count);
    if (IS_ERR(data))
    return PTR_ERR(data);
    tomoyo_normalize_line(data);
    if (tomoyo_correct_domain(data)) {
    let mut idx: c_int = tomoyo_read_lock();
    struct tomoyo_path_info name;
    struct tomoyo_request_info r;
    name.name = data;
    tomoyo_fill_path_info(&name);
// Check "task manual_domain_transition" permission.
    tomoyo_init_request_info(&r, core::ptr::null_mut(), TOMOYO_MAC_FILE_EXECUTE);
    r.param_type = TOMOYO_TYPE_MANUAL_TASK_ACL;
    r.param.task.domainname = &name;
    tomoyo_check_acl(&r, tomoyo_check_task_acl);
    if (!r.granted)
    error = -EPERM;
    else {
    struct tomoyo_domain_info *new_domain =
    tomoyo_assign_domain(data, true);
    if (!new_domain) {
    error = -ENOENT;
    } else {
    struct tomoyo_task *s = tomoyo_task(current);
    struct tomoyo_domain_info *old_domain =
    s.domain_info;
    s.domain_info = new_domain;
    atomic_inc(&new_domain.users);
    atomic_dec(&old_domain.users);
    error = 0;
    }
    }
    tomoyo_read_unlock(idx);
    } else
    error = -EINVAL;
    kfree(data);
    return error ? error : count;
    }
//
// tomoyo_read_self - read() for /sys/kernel/security/tomoyo/self_domain interface.
//
// @file:  Pointer to "struct file".
// @buf:   Domainname which current thread belongs to.
// @count: Size of @buf.
// @ppos:  Bytes read by now.
//
// Returns read size on success, negative value otherwise.
//
    static ssize_t tomoyo_read_self(struct file *file, char __user *buf,
    size_t count, loff_t *ppos)
    {
    const char *domain = tomoyo_domain().domainname.name;
    let mut len: loff_t = strlen(domain);
    let mut pos: loff_t = *ppos;
    if (pos >= len || !count)
    return 0;
    len -= pos;
    if (count < len)
    len = count;
    if (copy_to_user(buf, domain + pos, len))
    return -EFAULT;
// ppos += len;
    return len;
    }
// Operations for /sys/kernel/security/tomoyo/self_domain interface.
    static const struct file_operations tomoyo_self_operations = {
    .write = tomoyo_write_self,
    .read  = tomoyo_read_self,
    };
//
// tomoyo_open - open() for /sys/kernel/security/tomoyo/ interface.
//
// @inode: Pointer to "struct inode".
// @file:  Pointer to "struct file".
//
// Returns 0 on success, negative value otherwise.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_open(inode: *mut inode, file: *mut file) -> c_int {
    static int tomoyo_open(struct inode *inode, struct file *file)
    {
    let mut key: u8 = (uintptr_t) file_inode(file).i_private;
    return tomoyo_open_control(key, file);
    }
//
// tomoyo_release - close() for /sys/kernel/security/tomoyo/ interface.
//
// @inode: Pointer to "struct inode".
// @file:  Pointer to "struct file".
//
#[no_mangle]
unsafe extern "C" fn tomoyo_release(inode: *mut inode, file: *mut file) -> c_int {
    static int tomoyo_release(struct inode *inode, struct file *file)
    {
    tomoyo_close_control(file.private_data);
    return 0;
    }
//
// tomoyo_poll - poll() for /sys/kernel/security/tomoyo/ interface.
//
// @file: Pointer to "struct file".
// @wait: Pointer to "poll_table". Maybe NULL.
//
// Returns EPOLLIN | EPOLLRDNORM | EPOLLOUT | EPOLLWRNORM if ready to read/write,
// EPOLLOUT | EPOLLWRNORM otherwise.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t tomoyo_poll(struct file *file, poll_table *wait)
    {
    return tomoyo_poll_control(file, wait);
    }
//
// tomoyo_read - read() for /sys/kernel/security/tomoyo/ interface.
//
// @file:  Pointer to "struct file".
// @buf:   Pointer to buffer.
// @count: Size of @buf.
// @ppos:  Unused.
//
// Returns bytes read on success, negative value otherwise.
//
    static ssize_t tomoyo_read(struct file *file, char __user *buf, size_t count,
    loff_t *ppos)
    {
    return tomoyo_read_control(file.private_data, buf, count);
    }
//
// tomoyo_write - write() for /sys/kernel/security/tomoyo/ interface.
//
// @file:  Pointer to "struct file".
// @buf:   Pointer to buffer.
// @count: Size of @buf.
// @ppos:  Unused.
//
// Returns @count on success, negative value otherwise.
//
    static ssize_t tomoyo_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    return tomoyo_write_control(file.private_data, buf, count);
    }
//
// tomoyo_operations is a "struct file_operations" which is used for handling
// /sys/kernel/security/tomoyo/ interface.
//
// Some files under /sys/kernel/security/tomoyo/ directory accept open(O_RDWR).
// See tomoyo_io_buffer for internals.
//
    static const struct file_operations tomoyo_operations = {
    .open    = tomoyo_open,
    .release = tomoyo_release,
    .poll    = tomoyo_poll,
    .read    = tomoyo_read,
    .write   = tomoyo_write,
    .llseek  = noop_llseek,
    };
//
// tomoyo_create_entry - Create interface files under /sys/kernel/security/tomoyo/ directory.
//
// @name:   The name of the interface file.
// @mode:   The permission of the interface file.
// @parent: The parent directory.
// @key:    Type of interface.
//
// Returns nothing.
//
    static void __init tomoyo_create_entry(const char *name, const umode_t mode,
    struct dentry *parent, const u8 key)
    {
    securityfs_create_file(name, mode, parent, (void *) (uintptr_t) key,
    &tomoyo_operations);
    }
//
// tomoyo_interface_init - Initialize /sys/kernel/security/tomoyo/ interface.
//
// Returns 0.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_interface_init() -> int __init {
    int __init tomoyo_interface_init(void)
    {
    struct tomoyo_domain_info *domain;
    struct dentry *tomoyo_dir;
    if (!tomoyo_enabled)
    return 0;
    domain = tomoyo_domain();
// Don't create securityfs entries unless registered.
    if (domain != &tomoyo_kernel_domain)
    return 0;
    tomoyo_dir = securityfs_create_dir("tomoyo", core::ptr::null_mut());
    tomoyo_create_entry("query",            0600, tomoyo_dir,
    TOMOYO_QUERY);
    tomoyo_create_entry("domain_policy",    0600, tomoyo_dir,
    TOMOYO_DOMAINPOLICY);
    tomoyo_create_entry("exception_policy", 0600, tomoyo_dir,
    TOMOYO_EXCEPTIONPOLICY);
    tomoyo_create_entry("audit",            0400, tomoyo_dir,
    TOMOYO_AUDIT);
    tomoyo_create_entry(".process_status",  0600, tomoyo_dir,
    TOMOYO_PROCESS_STATUS);
    tomoyo_create_entry("stat",             0644, tomoyo_dir,
    TOMOYO_STAT);
    tomoyo_create_entry("profile",          0600, tomoyo_dir,
    TOMOYO_PROFILE);
    tomoyo_create_entry("manager",          0600, tomoyo_dir,
    TOMOYO_MANAGER);
    tomoyo_create_entry("version",          0400, tomoyo_dir,
    TOMOYO_VERSION);
    securityfs_create_file("self_domain", 0666, tomoyo_dir, core::ptr::null_mut(),
    &tomoyo_self_operations);
    tomoyo_load_builtin_policy();
    return 0;
    }
