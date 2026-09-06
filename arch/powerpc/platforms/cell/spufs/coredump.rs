//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spufs/coredump.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SPU core dump code
//
// (C) Copyright 2006 IBM Corp.
//
// Author: Dwayne Grant McConnell <decimal@us.ibm.com>
//

#[no_mangle]
unsafe extern "C" fn spufs_ctx_note_size(ctx: *mut spu_context, dfd: c_int) -> c_int {
    static int spufs_ctx_note_size(struct spu_context *ctx, int dfd)
    {
    int i, sz, total = 0;
    char *name;
    char fullname[80];
    for (i = 0; spufs_coredump_read[i].name != core::ptr::null_mut(); i++) {
    name = spufs_coredump_read[i].name;
    sz = spufs_coredump_read[i].size;
    sprintf(fullname, "SPU/%d/%s", dfd, name);
    total += sizeof(struct elf_note);
    total += roundup(strlen(fullname) + 1, 4);
    total += roundup(sz, 4);
    }
    return total;
    }
#[no_mangle]
unsafe extern "C" fn match_context(v: *const c_void, file: *mut file, fd: unsigned) -> c_int {
    static int match_context(const void *v, struct file *file, unsigned fd)
    {
    struct spu_context *ctx;
    if (file.f_op != &spufs_context_fops)
    return 0;
    ctx = SPUFS_I(file_inode(file)).i_ctx;
    if (ctx.flags & SPU_CREATE_NOSCHED)
    return 0;
    return fd + 1;
    }
//
// The additional architecture-specific notes for Cell are various
// context files in the spu context.
//
// This function iterates over all open file descriptors and sees
// if they are a directory in spufs.  In that case we use spufs
// internal functionality to dump them without needing to actually
// open the files.
//
// descriptor table is not shared, so files can't change or go away.
//
    static struct spu_context *coredump_next_context(int *fd)
    {
    struct spu_context *ctx = core::ptr::null_mut();
    struct file *file;
    let mut n: c_int = iterate_fd(current.files, *fd, match_context, core::ptr::null_mut());
    if (!n)
    return core::ptr::null_mut();
// fd = n - 1;
    file = fget_raw(*fd);
    if (file) {
    ctx = SPUFS_I(file_inode(file)).i_ctx;
    get_spu_context(ctx);
    fput(file);
    }
    return ctx;
    }
#[no_mangle]
pub unsafe extern "C" fn spufs_coredump_extra_notes_size() -> c_int {
    int spufs_coredump_extra_notes_size(void)
    {
    struct spu_context *ctx;
    let mut size: c_int = 0, rc, fd;
    fd = 0;
    while ((ctx = coredump_next_context(&fd)) != core::ptr::null_mut()) {
    rc = spu_acquire_saved(ctx);
    if (rc) {
    put_spu_context(ctx);
    break;
    }
    rc = spufs_ctx_note_size(ctx, fd);
    spu_release_saved(ctx);
    if (rc < 0) {
    put_spu_context(ctx);
    break;
    }
    size += rc;
// start searching the next fd next time
    fd++;
    put_spu_context(ctx);
    }
    return size;
    }
    static int spufs_arch_write_note(struct spu_context *ctx, int i,
    struct coredump_params *cprm, int dfd)
    {
    let mut sz: usize = spufs_coredump_read[i].size;
    char fullname[80];
    struct elf_note en;
    int ret;
    sprintf(fullname, "SPU/%d/%s", dfd, spufs_coredump_read[i].name);
    en.n_namesz = strlen(fullname) + 1;
    en.n_descsz = sz;
    en.n_type = NT_SPU;
    if (!dump_emit(cprm, &en, sizeof(en)))
    return -EIO;
    if (!dump_emit(cprm, fullname, en.n_namesz))
    return -EIO;
    if (!dump_align(cprm, 4))
    return -EIO;
    if (spufs_coredump_read[i].dump) {
    ret = spufs_coredump_read[i].dump(ctx, cprm);
    if (ret < 0)
    return ret;
    } else {
    char buf[32];
    ret = snprintf(buf, sizeof(buf), "0x%.16llx",
    spufs_coredump_read[i].get(ctx));
    if (ret >= sizeof(buf))
    return sizeof(buf);
// count trailing the NULL:
    if (!dump_emit(cprm, buf, ret + 1))
    return -EIO;
    }
    dump_skip_to(cprm, roundup(cprm.pos - ret + sz, 4));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn spufs_coredump_extra_notes_write(cprm: *mut coredump_params) -> c_int {
    int spufs_coredump_extra_notes_write(struct coredump_params *cprm)
    {
    struct spu_context *ctx;
    int fd, j, rc;
    fd = 0;
    while ((ctx = coredump_next_context(&fd)) != core::ptr::null_mut()) {
    rc = spu_acquire_saved(ctx);
    if (rc)
    return rc;
    for (j = 0; spufs_coredump_read[j].name != core::ptr::null_mut(); j++) {
    rc = spufs_arch_write_note(ctx, j, cprm, fd);
    if (rc) {
    spu_release_saved(ctx);
    return rc;
    }
    }
    spu_release_saved(ctx);
// start searching the next fd next time
    fd++;
    }
    return 0;
    }
