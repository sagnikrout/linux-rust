//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/umid.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

pub const UMID_LEN: c_int = 64;
// Changed by set_umid, which is run early in boot
    static char umid[UMID_LEN] = { 0 };
// Changed by set_uml_dir and make_uml_dir, which are run early in boot
    static char *uml_dir = UML_DIR;
#[no_mangle]
unsafe extern "C" fn make_uml_dir() -> int __init {
    static int __init make_uml_dir(void)
    {
    char dir[512] = { '\0' };
    int len, err;
    if (*uml_dir == '~') {
    char *home = getenv("HOME");
    err = -ENOENT;
    if (home == core::ptr::null_mut()) {
    printk(UM_KERN_ERR
    "%s: no value in environment for $HOME\n",
    __func__);
    goto err;
    }
    strscpy(dir, home);
    uml_dir++;
    }
    strlcat(dir, uml_dir, sizeof(dir));
    len = strlen(dir);
    if (len > 0 && dir[len - 1] != '/')
    strlcat(dir, "/", sizeof(dir));
    err = -ENOMEM;
    uml_dir = malloc(strlen(dir) + 1);
    if (uml_dir == core::ptr::null_mut()) {
    printk(UM_KERN_ERR "%s : malloc failed, errno = %d\n",
    __func__, errno);
    goto err;
    }
    strcpy(uml_dir, dir);
    if ((mkdir(uml_dir, 0777) < 0) && (errno != EEXIST)) {
    printk(UM_KERN_ERR "Failed to mkdir '%s': %s\n",
    uml_dir, strerror(errno));
    err = -errno;
    goto err_free;
    }
    return 0;
    err_free:
    free(uml_dir);
    err:
    uml_dir = core::ptr::null_mut();
    return err;
    }
//
// Unlinks the files contained in @dir and then removes @dir.
// Doesn't handle directory trees, so it's not like rm -rf, but almost such. We
// ignore ENOENT errors for anything (they happen, strangely enough - possibly
// due to races between multiple dying UML threads).
//
#[no_mangle]
unsafe extern "C" fn remove_files_and_dir(dir: *mut c_char) -> c_int {
    static int remove_files_and_dir(char *dir)
    {
    DIR *directory;
    struct dirent *ent;
    int len;
    char file[256];
    int ret;
    directory = opendir(dir);
    if (directory == core::ptr::null_mut()) {
    if (errno != ENOENT)
    return -errno;
    else
    return 0;
    }
    while ((ent = readdir(directory)) != core::ptr::null_mut()) {
    if (!strcmp(ent.d_name, ".") || !strcmp(ent.d_name, ".."))
    continue;
    len = strlen(dir) + strlen("/") + strlen(ent.d_name) + 1;
    if (len > sizeof(file)) {
    ret = -E2BIG;
    goto out;
    }
    sprintf(file, "%s/%s", dir, ent.d_name);
    if (unlink(file) < 0 && errno != ENOENT) {
    ret = -errno;
    goto out;
    }
    }
    if (rmdir(dir) < 0 && errno != ENOENT) {
    ret = -errno;
    goto out;
    }
    ret = 0;
    out:
    closedir(directory);
    return ret;
    }
//
// This says that there isn't already a user of the specified directory even if
// there are errors during the checking.  This is because if these errors
// happen, the directory is unusable by the pre-existing UML, so we might as
// well take it over.  This could happen either by
// the existing UML somehow corrupting its umid directory
// something other than UML sticking stuff in the directory
// this boot racing with a shutdown of the other UML
// In any of these cases, the directory isn't useful for anything else.
//
// Boolean return: 1 if in use, 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn is_umdir_used(dir: *mut c_char) -> c_int {
    static inline int is_umdir_used(char *dir)
    {
    char pid[sizeof("nnnnnnnnn")], *end, *file;
    int fd, p, n;
    let mut filelen: usize = strlen(dir) + sizeof("/pid") + 1;
    file = malloc(filelen);
    if (!file)
    return -ENOMEM;
    snprintf(file, filelen, "%s/pid", dir);
    fd = open(file, O_RDONLY);
    if (fd < 0) {
    fd = -errno;
    if (fd != -ENOENT) {
    printk(UM_KERN_ERR "is_umdir_used : couldn't open pid "
    "file '%s', err = %d\n", file, -fd);
    }
    goto out;
    }
    n = read(fd, pid, sizeof(pid));
    if (n < 0) {
    printk(UM_KERN_ERR "is_umdir_used : couldn't read pid file "
    "'%s', err = %d\n", file, errno);
    goto out_close;
    } else if (n == 0) {
    printk(UM_KERN_ERR "is_umdir_used : couldn't read pid file "
    "'%s', 0-byte read\n", file);
    goto out_close;
    }
    p = strtoul(pid, &end, 0);
    if (end == pid) {
    printk(UM_KERN_ERR "is_umdir_used : couldn't parse pid file "
    "'%s', errno = %d\n", file, errno);
    goto out_close;
    }
    if ((kill(p, 0) == 0) || (errno != ESRCH)) {
    printk(UM_KERN_ERR "umid \"%s\" is already in use by pid %d\n",
    umid, p);
    return 1;
    }
    out_close:
    close(fd);
    out:
    free(file);
    return 0;
    }
//
// Try to remove the directory @dir unless it's in use.
// Precondition: @dir exists.
// Returns 0 for success, < 0 for failure in removal or if the directory is in
// use.
//
#[no_mangle]
unsafe extern "C" fn umdir_take_if_dead(dir: *mut c_char) -> c_int {
    static int umdir_take_if_dead(char *dir)
    {
    int ret;
    if (is_umdir_used(dir))
    return -EEXIST;
    ret = remove_files_and_dir(dir);
    if (ret) {
    printk(UM_KERN_ERR "is_umdir_used - remove_files_and_dir "
    "failed with err = %d\n", ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn create_pid_file() -> void __init {
    static void __init create_pid_file(void)
    {
    char pid[sizeof("nnnnnnnnn")], *file;
    int fd, n;
    n = strlen(uml_dir) + UMID_LEN + sizeof("/pid");
    file = malloc(n);
    if (!file)
    return;
    if (umid_file_name("pid", file, n))
    goto out;
    fd = open(file, O_RDWR | O_CREAT | O_EXCL, 0644);
    if (fd < 0) {
    printk(UM_KERN_ERR "Open of machine pid file \"%s\" failed: "
    "%s\n", file, strerror(errno));
    goto out;
    }
    snprintf(pid, sizeof(pid), "%d\n", getpid());
    n = write(fd, pid, strlen(pid));
    if (n != strlen(pid))
    printk(UM_KERN_ERR "Write of pid file failed - err = %d\n",
    errno);
    close(fd);
    out:
    free(file);
    }
#[no_mangle]
pub unsafe extern "C" fn set_umid(name: *mut c_char) -> int __init {
    int __init set_umid(char *name)
    {
    if (strlen(name) > UMID_LEN - 1)
    return -E2BIG;
    strscpy(umid, name);
    return 0;
    }
// Changed in make_umid, which is called during early boot
    let mut umid_setup: static int = 0;
#[no_mangle]
unsafe extern "C" fn make_umid() -> int __init {
    static int __init make_umid(void)
    {
    int fd, err;
    char tmp[256];
    if (umid_setup)
    return 0;
    make_uml_dir();
    if (*umid == '\0') {
    strscpy(tmp, uml_dir);
    strlcat(tmp, "XXXXXX", sizeof(tmp));
    fd = mkstemp(tmp);
    if (fd < 0) {
    printk(UM_KERN_ERR "make_umid - mkstemp(%s) failed: "
    "%s\n", tmp, strerror(errno));
    err = -errno;
    goto err;
    }
    close(fd);
    set_umid(&tmp[strlen(uml_dir)]);
//
// There's a nice tiny little race between this unlink and
// the mkdir below.  It'd be nice if there were a mkstemp
// for directories.
//
    if (unlink(tmp)) {
    err = -errno;
    goto err;
    }
    }
    snprintf(tmp, sizeof(tmp), "%s%s", uml_dir, umid);
    err = mkdir(tmp, 0777);
    if (err < 0) {
    err = -errno;
    if (err != -EEXIST)
    goto err;
    if (umdir_take_if_dead(tmp) < 0)
    goto err;
    err = mkdir(tmp, 0777);
    }
    if (err) {
    err = -errno;
    printk(UM_KERN_ERR "Failed to create '%s' - err = %d\n", umid,
    errno);
    goto err;
    }
    umid_setup = 1;
    create_pid_file();
    err = 0;
    err:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn make_umid_init() -> int __init {
    static int __init make_umid_init(void)
    {
    if (!make_umid())
    return 0;
//
// If initializing with the given umid failed, then try again with
// a random one.
//
    printk(UM_KERN_ERR "Failed to initialize umid \"%s\", trying with a "
    "random umid\n", umid);
// umid = '\0';
    make_umid();
    return 0;
    }
    __initcall(make_umid_init);
#[no_mangle]
pub unsafe extern "C" fn umid_file_name(name: *mut c_char, buf: *mut c_char, len: c_int) -> int __init {
    int __init umid_file_name(char *name, char *buf, int len)
    {
    int n, err;
    err = make_umid();
    if (err)
    return err;
    n = snprintf(buf, len, "%s%s/%s", uml_dir, umid, name);
    if (n >= len) {
    printk(UM_KERN_ERR "umid_file_name : buffer too short\n");
    return -E2BIG;
    }
    return 0;
    }
    char *get_umid(void)
    {
    return umid;
    }
#[no_mangle]
unsafe extern "C" fn set_uml_dir(name: *mut c_char, add: *mut c_int) -> int __init {
    static int __init set_uml_dir(char *name, int *add)
    {
// add = 0;
    if (*name == '\0') {
    os_warn("uml_dir can't be an empty string\n");
    return 0;
    }
    if (name[strlen(name) - 1] == '/') {
    uml_dir = name;
    return 0;
    }
    uml_dir = malloc(strlen(name) + 2);
    if (uml_dir == core::ptr::null_mut()) {
    os_warn("Failed to malloc uml_dir - error = %d\n", errno);
//
// Return 0 here because do_initcalls doesn't look at
// the return value.
//
    return 0;
    }
    sprintf(uml_dir, "%s/", name);
    return 0;
    }
    __uml_setup("uml_dir=", set_uml_dir,
    "uml_dir=<directory>\n"
    "    The location to place the pid and umid files.\n\n"
    );
#[no_mangle]
unsafe extern "C" fn remove_umid_dir() {
    static void remove_umid_dir(void)
    {
    char *dir, err;
    dir = malloc(strlen(uml_dir) + UMID_LEN + 1);
    if (!dir)
    return;
    sprintf(dir, "%s%s", uml_dir, umid);
    err = remove_files_and_dir(dir);
    if (err)
    os_warn("%s - remove_files_and_dir failed with err = %d\n",
    __func__, err);
    free(dir);
    }
    __uml_exitcall(remove_umid_dir);
