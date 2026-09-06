//! Automatically rewritten from C to Rust
//! Source: fs/filesystems.c
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
// linux/fs/filesystems.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// table of configured filesystems
//

//
// Read-mostly filesystem drivers list.
//
// Readers walk under rcu_read_lock(); writers take file_systems_lock
// and publish via _rcu hlist primitives.  unregister_filesystem()
// synchronize_rcu()s after unlock so the embedded file_system_type
// can't go away under a reader.  To keep using a filesystem after
// the RCU section ends, take a module reference via try_module_get().
//
    static HLIST_HEAD(file_systems);
    static DEFINE_SPINLOCK(file_systems_lock);

//
// Cache a stringified version of the filesystem list.
//
// The fs list gets queried a lot by userspace because of libselinux, including
// rather surprising programs (would you guess *sed* is on the list?). In order
// to reduce the overhead we cache the resulting string, which normally hangs
// around below 512 bytes in size.
//
// As the list almost never changes, its creation is not particularly optimized
// to keep things simple.
//
// We sort it out on read in order to not introduce a failure point for fs
// registration (in principle we may be unable to alloc memory for the list).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_systems_string {
    pub rcu: rcu_head,
    pub gen: c_ulong,
    pub len: usize,
    pub string: [c_char; ],
}

    static unsigned long file_systems_gen;
    static struct file_systems_string __read_mostly __rcu *file_systems_string;
    static void invalidate_filesystems_string(void);

    static inline void invalidate_filesystems_string(void) { }

// WARNING: This can be used only if we _already_ own a reference
    struct file_system_type *get_filesystem(struct file_system_type *fs)
    {
    __module_get(fs.owner);
    return fs;
    }
#[no_mangle]
pub unsafe extern "C" fn put_filesystem(fs: *mut file_system_type) {
    void put_filesystem(struct file_system_type *fs)
    {
    module_put(fs.owner);
    }
    static struct file_system_type *find_filesystem(const char *name, unsigned len)
    {
    struct file_system_type *fs;
    hlist_for_each_entry_rcu(fs, &file_systems, list,
    lockdep_is_held(&file_systems_lock))
    if (strncmp(fs.name, name, len) == 0 && !fs.name[len])
    return fs;
    return core::ptr::null_mut();
    }
//
// register_filesystem - register a new filesystem
// @fs: the file system structure
//
// Adds the file system passed to the list of file systems the kernel
// is aware of for mount and other syscalls. Returns 0 on success,
// or a negative errno code on an error.
//
// The &struct file_system_type that is passed is linked into the kernel
// structures and must not be freed until the file system has been
// unregistered.
//
#[no_mangle]
pub unsafe extern "C" fn register_filesystem(fs: *mut file_system_type) -> c_int {
    int register_filesystem(struct file_system_type *fs)
    {
    if (fs.parameters &&
    !fs_validate_description(fs.name, fs.parameters))
    return -EINVAL;
    BUG_ON(strchr(fs.name, '.'));
    if (!hlist_unhashed_lockless(&fs.list))
    return -EBUSY;
    guard(spinlock)(&file_systems_lock);
    if (find_filesystem(fs.name, strlen(fs.name)))
    return -EBUSY;
    hlist_add_tail_rcu(&fs.list, &file_systems);
    invalidate_filesystems_string();
    return 0;
    }
    EXPORT_SYMBOL(register_filesystem);
//
// unregister_filesystem - unregister a file system
// @fs: filesystem to unregister
//
// Remove a file system that was previously successfully registered
// with the kernel. An error is returned if the file system is not found.
// Zero is returned on a success.
//
// Once this function has returned the &struct file_system_type structure
// may be freed or reused.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_filesystem(fs: *mut file_system_type) -> c_int {
    int unregister_filesystem(struct file_system_type *fs)
    {
    scoped_guard(spinlock, &file_systems_lock) {
    if (hlist_unhashed(&fs.list))
    return -EINVAL;
    hlist_del_init_rcu(&fs.list);
    invalidate_filesystems_string();
    }
    synchronize_rcu();
    return 0;
    }
    EXPORT_SYMBOL(unregister_filesystem);

#[no_mangle]
unsafe extern "C" fn fs_index(__name: *const char __user) -> c_int {
    static int fs_index(const char __user *__name)
    {
    struct file_system_type *p;
    char *name __free(kfree) = strndup_user(__name, PATH_MAX);
    let mut index: c_int = 0;
    if (IS_ERR(name))
    return PTR_ERR(name);
    guard(rcu)();
    hlist_for_each_entry_rcu(p, &file_systems, list) {
    if (strcmp(p.name, name) == 0)
    return index;
    index++;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn fs_name(index: c_uint, buf: *mut char __user) -> c_int {
    static int fs_name(unsigned int index, char __user *buf)
    {
    struct file_system_type *p, *found = core::ptr::null_mut();
    int len, res;
    scoped_guard(rcu) {
    hlist_for_each_entry_rcu(p, &file_systems, list) {
    if (index--)
    continue;
    if (try_module_get(p.owner))
    found = p;
    break;
    }
    }
    if (!found)
    return -EINVAL;
// OK, we got the reference, so we can safely block
    len = strlen(found.name) + 1;
    res = copy_to_user(buf, found.name, len) ? -EFAULT : 0;
    put_filesystem(found);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn fs_maxindex() -> c_int {
    static int fs_maxindex(void)
    {
    struct file_system_type *p;
    let mut index: c_int = 0;
    guard(rcu)();
    hlist_for_each_entry_rcu(p, &file_systems, list)
    index++;
    return index;
    }
//
// Whee.. Weird sysv syscall.
//
    SYSCALL_DEFINE3(sysfs, int, option, unsigned long, arg1, unsigned long, arg2)
    {
    let mut retval: c_int = -EINVAL;
    switch (option) {
    case 1:
    retval = fs_index((const char __user *) arg1);
    break;
    case 2:
    retval = fs_name(arg1, (char __user *) arg2);
    break;
    case 3:
    retval = fs_maxindex();
    break;
    }
    return retval;
    }

#[no_mangle]
pub unsafe extern "C" fn list_bdev_fs_names(buf: *mut c_char, size: usize) -> int __init {
    int __init list_bdev_fs_names(char *buf, size_t size)
    {
    struct file_system_type *p;
    size_t len;
    let mut count: c_int = 0;
    guard(rcu)();
    hlist_for_each_entry_rcu(p, &file_systems, list) {
    if (!(p.fs_flags & FS_REQUIRES_DEV))
    continue;
    len = strlen(p.name) + 1;
    if (len > size) {
    pr_warn("%s: truncating file system list\n", __func__);
    break;
    }
    memcpy(buf, p.name, len);
    buf += len;
    size -= len;
    count++;
    }
    return count;
    }

#[no_mangle]
unsafe extern "C" fn invalidate_filesystems_string() {
    static void invalidate_filesystems_string(void)
    {
    struct file_systems_string *old;
    lockdep_assert_held_write(&file_systems_lock);
    file_systems_gen++;
    old = rcu_replace_pointer(file_systems_string, core::ptr::null_mut(),
    lockdep_is_held(&file_systems_lock));
    if (old)
    kfree_rcu(old, rcu);
    }
#[no_mangle]
unsafe extern "C" fn regen_filesystems_string() -> __cold noinline int {
    static __cold noinline int regen_filesystems_string(void)
    {
    struct file_system_type *p;
    struct file_systems_string *old, *new;
    size_t newlen, usedlen;
    unsigned long gen;
    retry:
    newlen = 0;
// pre-calc space for each fs
    spin_lock(&file_systems_lock);
    gen = file_systems_gen;
    hlist_for_each_entry_rcu(p, &file_systems, list) {
    if (!(p.fs_flags & FS_REQUIRES_DEV))
    newlen += strlen("nodev");
    newlen += strlen("\t") + strlen(p.name) + strlen("\n");
    }
    spin_unlock(&file_systems_lock);
    new = kmalloc(offsetof(struct file_systems_string, string) + newlen + 1,
    GFP_KERNEL);
    if (!new)
    return -ENOMEM;
    new.gen = gen;
    new.len = newlen;
    new.string[newlen] = '\0';
    spin_lock(&file_systems_lock);
    old = file_systems_string;
//
// Did someone beat us to it?
//
    if (old && old.gen == file_systems_gen) {
    spin_unlock(&file_systems_lock);
    kfree(new);
    return 0;
    }
//
// Did the list change in the meantime?
//
    if (gen != file_systems_gen) {
    spin_unlock(&file_systems_lock);
    kfree(new);
    goto retry;
    }
//
// Populate the string.
//
// We know we have just enough space because we calculated the right
// size the previous time we had the lock and confirmed the list has
// not changed after reacquiring it.
//
    usedlen = 0;
    hlist_for_each_entry_rcu(p, &file_systems, list) {
    usedlen += sprintf(&new.string[usedlen], "%s\t%s\n",
    (p.fs_flags & FS_REQUIRES_DEV) ? "" : "nodev",
    p.name);
    }
    if (WARN_ON_ONCE(new.len != strlen(new.string))) {
//
// Should never happen of course, keep this in case someone changes string
// generation above and messes it up.
//
    spin_unlock(&file_systems_lock);
    kfree(new);
    return -EINVAL;
    }
    rcu_assign_pointer(file_systems_string, new);
    spin_unlock(&file_systems_lock);
    if (old)
    kfree_rcu(old, rcu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn filesystems_proc_show_fallback(m: *mut seq_file, v: *mut c_void) -> __cold noinline int {
    static __cold noinline int filesystems_proc_show_fallback(struct seq_file *m, void *v)
    {
    struct file_system_type *p;
    guard(rcu)();
    hlist_for_each_entry_rcu(p, &file_systems, list) {
    seq_printf(m, "%s\t%s\n",
    (p.fs_flags & FS_REQUIRES_DEV) ? "" : "nodev",
    p.name);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn filesystems_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int filesystems_proc_show(struct seq_file *m, void *v)
    {
    struct file_systems_string *fss;
    for (;;) {
    scoped_guard(rcu) {
    fss = rcu_dereference(file_systems_string);
    if (likely(fss)) {
    seq_write(m, fss.string, fss.len);
    return 0;
    }
    }
    let mut err: c_int = regen_filesystems_string();
    if (unlikely(err))
    return filesystems_proc_show_fallback(m, v);
    }
    }
#[no_mangle]
unsafe extern "C" fn proc_filesystems_init() -> int __init {
    static int __init proc_filesystems_init(void)
    {
    struct proc_dir_entry *pde;
    pde = proc_create_single("filesystems", 0, core::ptr::null_mut(), filesystems_proc_show);
    if (!pde)
    return -ENOMEM;
    proc_make_permanent(pde);
    return 0;
    }
    module_init(proc_filesystems_init);

    static struct file_system_type *__get_fs_type(const char *name, int len)
    {
    struct file_system_type *fs;
    guard(rcu)();
    fs = find_filesystem(name, len);
    if (fs && !try_module_get(fs.owner))
    fs = core::ptr::null_mut();
    return fs;
    }
    struct file_system_type *get_fs_type(const char *name)
    {
    struct file_system_type *fs;
    const char *dot = strchr(name, '.');
    let mut len: c_int = dot ? dot - name : strlen(name);
    fs = __get_fs_type(name, len);
    if (!fs && (request_module("fs-%.*s", len, name) == 0)) {
    fs = __get_fs_type(name, len);
    if (!fs)
    pr_warn_once("request_module fs-%.*s succeeded, but still no fs?\n",
    len, name);
    }
    if (dot && fs && !(fs.fs_flags & FS_HAS_SUBTYPE)) {
    put_filesystem(fs);
    fs = core::ptr::null_mut();
    }
    return fs;
    }
    EXPORT_SYMBOL(get_fs_type);
