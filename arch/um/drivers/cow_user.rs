//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/cow_user.c
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
// Copyright (C) 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
//
// _XOPEN_SOURCE is needed for pread, but we define _GNU_SOURCE, which defines
// that.
//

//
// arch/um/Makefile remaps strrchr to kernel_strrchr; call the kernel
// name directly to avoid glibc >= 2.43's C23 strrchr macro.
//
    char *kernel_strrchr(const char *, int);
pub const PATH_LEN_V1: c_int = 256;
// unsigned time_t works until year 2106
    typedef __u32 time32_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cow_header_v1 {
    pub magic: __s32,
    pub version: __s32,
    pub backing_file: [c_char; PATH_LEN_V1],
    pub mtime: time32_t,
    pub size: __u64,
    pub sectorsize: __s32,
    pub __attribute__((packed)): },
//
// Define PATH_LEN_V3 as the usual value of MAXPATHLEN, just hard-code it in
// case other systems have different values for MAXPATHLEN.
//
// The same must hold for V2 - we want file format compatibility, not anything
// else.
//
pub const PATH_LEN_V3: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cow_header_v2 {
    pub magic: __u32,
    pub version: __u32,
    pub backing_file: [c_char; PATH_LEN_V2],
    pub mtime: time32_t,
    pub size: __u64,
    pub sectorsize: __s32,
    pub __attribute__((packed)): },
//
// Changes from V2 -
// PATH_LEN_V3 as described above
// Explicitly specify field bit lengths for systems with different
// lengths for the usual C types.  Not sure whether char or
// time_t should be changed, this can be changed later without
// breaking compatibility
// Add alignment field so that different alignments can be used for the
// bitmap and data
// Add cow_format field to allow for the possibility of different ways
// of specifying the COW blocks.  For now, the only value is 0,
// for the traditional COW bitmap.
// Move the backing_file field to the end of the header.  This allows
// for the possibility of expanding it into the padding required
// by the bitmap alignment.
// The bitmap and data portions of the file will be aligned as specified
// by the alignment field.  This is to allow COW files to be
// put on devices with restrictions on access alignments, such as
// /dev/raw, with a 512 byte alignment restriction.  This also
// allows the data to be more aligned more strictly than on
// sector boundaries.  This is needed for ubd-mmap, which needs
// the data to be page aligned.
// Fixed (finally!) the rounding bug
//
// Until Dec2005, __attribute__((packed)) was left out from the below
// definition, leading on 64-bit systems to 4 bytes of padding after mtime, to
// align size to 8-byte alignment.  This shifted all fields above (no padding
// was present on 32-bit, no other padding was added).
//
// However, this _can be detected_: it means that cow_format (always 0 until
// now) is shifted onto the first 4 bytes of backing_file, where it is otherwise
// impossible to find 4 zeros. -bb
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cow_header_v3 {
    pub magic: __u32,
    pub version: __u32,
    pub mtime: __u32,
    pub size: __u64,
    pub sectorsize: __u32,
    pub alignment: __u32,
    pub cow_format: __u32,
    pub backing_file: [c_char; PATH_LEN_V3],
    pub __attribute__((packed)): },
// This is the broken layout used by some 64-bit binaries.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cow_header_v3_broken {
    pub magic: __u32,
    pub version: __u32,
    pub mtime: __s64,
    pub size: __u64,
    pub sectorsize: __u32,
    pub alignment: __u32,
    pub cow_format: __u32,
    pub backing_file: [c_char; PATH_LEN_V3],
}

// COW format definitions - for now, we have only the usual COW bitmap
pub const COW_BITMAP: c_int = 0;
    union cow_header {
    struct cow_header_v1 v1;
    struct cow_header_v2 v2;
    struct cow_header_v3 v3;
    struct cow_header_v3_broken v3_b;
    };
pub const COW_MAGIC: c_uint = 0x4f4f4f4d  /* MOOO */;
pub const COW_VERSION: c_int = 3;

    void cow_sizes(int version, __u64 size, int sectorsize, int align,
    int bitmap_offset, unsigned long *bitmap_len_out,
    int *data_offset_out)
    {
    if (version < 3) {
// bitmap_len_out = (size + sectorsize - 1) / (8 * sectorsize);
// data_offset_out = bitmap_offset + *bitmap_len_out;
// data_offset_out = (*data_offset_out + sectorsize - 1)
    sectorsize;
// data_offset_out *= sectorsize;
    }
    else {
// bitmap_len_out = DIV_ROUND(size, sectorsize);
// bitmap_len_out = DIV_ROUND(*bitmap_len_out, 8);
// data_offset_out = bitmap_offset + *bitmap_len_out;
// data_offset_out = ROUND_UP(*data_offset_out, align);
    }
    }
#[no_mangle]
unsafe extern "C" fn absolutize(to: *mut c_char, size: c_int, from: *mut c_char) -> c_int {
    static int absolutize(char *to, int size, char *from)
    {
    char save_cwd[256], *slash;
    int remaining;
    if (getcwd(save_cwd, sizeof(save_cwd)) == core::ptr::null_mut()) {
    cow_printf("absolutize : unable to get cwd - errno = %d\n",
    errno);
    return -1;
    }
    slash = kernel_strrchr(from, '/');
    if (slash != core::ptr::null_mut()) {
// slash = '\0';
    if (chdir(from)) {
// slash = '/';
    cow_printf("absolutize : Can't cd to '%s' - "
    "errno = %d\n", from, errno);
    return -1;
    }
// slash = '/';
    if (getcwd(to, size) == core::ptr::null_mut()) {
    cow_printf("absolutize : unable to get cwd of '%s' - "
    "errno = %d\n", from, errno);
    return -1;
    }
    remaining = size - strlen(to);
    if (strlen(slash) + 1 > remaining) {
    cow_printf("absolutize : unable to fit '%s' into %d "
    "chars\n", from, size);
    return -1;
    }
    strcat(to, slash);
    }
    else {
    if (strlen(save_cwd) + 1 + strlen(from) + 1 > size) {
    cow_printf("absolutize : unable to fit '%s' into %d "
    "chars\n", from, size);
    return -1;
    }
    strcpy(to, save_cwd);
    strcat(to, "/");
    strcat(to, from);
    }
    if (chdir(save_cwd)) {
    cow_printf("absolutize : Can't cd to '%s' - "
    "errno = %d\n", save_cwd, errno);
    return -1;
    }
    return 0;
    }
    int write_cow_header(char *cow_file, int fd, char *backing_file,
    int sectorsize, int alignment, unsigned long long *size)
    {
    struct cow_header_v3 *header;
    long long modtime;
    int err;
    err = cow_seek_file(fd, 0);
    if (err < 0) {
    cow_printf("write_cow_header - lseek failed, err = %d\n", -err);
    goto out;
    }
    err = -ENOMEM;
    header = cow_malloc(sizeof(*header));
    if (header == core::ptr::null_mut()) {
    cow_printf("write_cow_header - failed to allocate COW V3 "
    "header\n");
    goto out;
    }
    header.magic = htobe32(COW_MAGIC);
    header.version = htobe32(COW_VERSION);
    err = -EINVAL;
    if (strlen(backing_file) > sizeof(header.backing_file) - 1) {
// Below, %zd is for a size_t value
    cow_printf("Backing file name \"%s\" is too long - names are "
    "limited to %zd characters\n", backing_file,
    sizeof(header.backing_file) - 1);
    goto out_free;
    }
    if (absolutize(header.backing_file, sizeof(header.backing_file),
    backing_file))
    goto out_free;
    err = os_file_modtime(header.backing_file, &modtime);
    if (err < 0) {
    cow_printf("write_cow_header - backing file '%s' mtime "
    "request failed, err = %d\n", header.backing_file,
    -err);
    goto out_free;
    }
    err = cow_file_size(header.backing_file, size);
    if (err < 0) {
    cow_printf("write_cow_header - couldn't get size of "
    "backing file '%s', err = %d\n",
    header.backing_file, -err);
    goto out_free;
    }
    header.mtime = htobe32(modtime);
    header.size = htobe64(*size);
    header.sectorsize = htobe32(sectorsize);
    header.alignment = htobe32(alignment);
    header.cow_format = COW_BITMAP;
    err = cow_write_file(fd, header, sizeof(*header));
    if (err != sizeof(*header)) {
    cow_printf("write_cow_header - write of header to "
    "new COW file '%s' failed, err = %d\n", cow_file,
    -err);
    goto out_free;
    }
    err = 0;
    out_free:
    cow_free(header);
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn file_reader(offset: __u64, buf: *mut c_char, len: c_int, arg: *mut c_void) -> c_int {
    int file_reader(__u64 offset, char *buf, int len, void *arg)
    {
    let mut fd: c_int = *((int *) arg);
    return pread(fd, buf, len, offset);
    }
// XXX Need to sanity-check the values read from the header
    int read_cow_header(int (*reader)(__u64, char *, int, void *), void *arg,
    __u32 *version_out, char **backing_file_out,
    long long *mtime_out, unsigned long long *size_out,
    int *sectorsize_out, __u32 *align_out,
    int *bitmap_offset_out)
    {
    union cow_header *header;
    char *file;
    int err, n;
    unsigned long version, magic;
    header = cow_malloc(sizeof(*header));
    if (header == core::ptr::null_mut()) {
    cow_printf("read_cow_header - Failed to allocate header\n");
    return -ENOMEM;
    }
    err = -EINVAL;
    n = (*reader)(0, (char *) header, sizeof(*header), arg);
    if (n < offsetof(typeof(header.v1), backing_file)) {
    cow_printf("read_cow_header - short header\n");
    goto out;
    }
    magic = header.v1.magic;
    if (magic == COW_MAGIC)
    version = header.v1.version;
#[no_mangle]
pub unsafe extern "C" fn if(be32toh(COW_MAGIC): magic ==) -> else {
    else if (magic == be32toh(COW_MAGIC))
    version = be32toh(header.v1.version);
// No error printed because the non-COW case comes through here
    else goto out;
// version_out = version;
    if (version == 1) {
    if (n < sizeof(header.v1)) {
    cow_printf("read_cow_header - failed to read V1 "
    "header\n");
    goto out;
    }
// mtime_out = header->v1.mtime;
// size_out = header->v1.size;
// sectorsize_out = header->v1.sectorsize;
// bitmap_offset_out = sizeof(header->v1);
// align_out = *sectorsize_out;
    file = header.v1.backing_file;
    }
#[no_mangle]
pub unsafe extern "C" fn if(2: version ==) -> else {
    if (n < sizeof(header.v2)) {
    cow_printf("read_cow_header - failed to read V2 "
    "header\n");
    goto out;
    }
// mtime_out = be32toh(header->v2.mtime);
// size_out = be64toh(header->v2.size);
// sectorsize_out = be32toh(header->v2.sectorsize);
// bitmap_offset_out = sizeof(header->v2);
// align_out = *sectorsize_out;
    file = header.v2.backing_file;
    }
// This is very subtle - see above at union cow_header definition
#[no_mangle]
pub unsafe extern "C" fn if(0): *mut *mut *mut version == 3 && (((int)header->v3.backing_file) !=) -> else {
    if (n < sizeof(header.v3)) {
    cow_printf("read_cow_header - failed to read V3 "
    "header\n");
    goto out;
    }
// mtime_out = be32toh(header->v3.mtime);
// size_out = be64toh(header->v3.size);
// sectorsize_out = be32toh(header->v3.sectorsize);
// align_out = be32toh(header->v3.alignment);
    if (*align_out == 0) {
    cow_printf("read_cow_header - invalid COW header, "
    "align == 0\n");
    }
// bitmap_offset_out = ROUND_UP(sizeof(header->v3), *align_out);
    file = header.v3.backing_file;
    }
#[no_mangle]
pub unsafe extern "C" fn if(3: version ==) -> else {
    cow_printf("read_cow_header - broken V3 file with"
    " 64-bit layout - recovering content.\n");
    if (n < sizeof(header.v3_b)) {
    cow_printf("read_cow_header - failed to read V3 "
    "header\n");
    goto out;
    }
//
// this was used until Dec2005 - 64bits are needed to represent
// 2106+. I.e. we can safely do this truncating cast.
//
// Additionally, we must use be32toh() instead of be64toh(), since
// the program used to use the former (tested - I got mtime
// mismatch "0 vs whatever").
//
// Ever heard about bug-to-bug-compatibility ? ;-)
// mtime_out = (time32_t) be32toh(header->v3_b.mtime);
// size_out = be64toh(header->v3_b.size);
// sectorsize_out = be32toh(header->v3_b.sectorsize);
// align_out = be32toh(header->v3_b.alignment);
    if (*align_out == 0) {
    cow_printf("read_cow_header - invalid COW header, "
    "align == 0\n");
    }
// bitmap_offset_out = ROUND_UP(sizeof(header->v3_b), *align_out);
    file = header.v3_b.backing_file;
    }
    else {
    cow_printf("read_cow_header - invalid COW version\n");
    goto out;
    }
    err = -ENOMEM;
// backing_file_out = cow_strdup(file);
    if (*backing_file_out == core::ptr::null_mut()) {
    cow_printf("read_cow_header - failed to allocate backing "
    "file\n");
    goto out;
    }
    err = 0;
    out:
    cow_free(header);
    return err;
    }
    int init_cow_file(int fd, char *cow_file, char *backing_file, int sectorsize,
    int alignment, int *bitmap_offset_out,
    unsigned long *bitmap_len_out, int *data_offset_out)
    {
    unsigned long long size, offset;
    let mut zero: c_char = 0;
    int err;
    err = write_cow_header(cow_file, fd, backing_file, sectorsize,
    alignment, &size);
    if (err)
    goto out;
// bitmap_offset_out = ROUND_UP(sizeof(struct cow_header_v3), alignment);
    cow_sizes(COW_VERSION, size, sectorsize, alignment, *bitmap_offset_out,
    bitmap_len_out, data_offset_out);
    offset = *data_offset_out + size - sizeof(zero);
    err = cow_seek_file(fd, offset);
    if (err < 0) {
    cow_printf("cow bitmap lseek failed : err = %d\n", -err);
    goto out;
    }
//
// does not really matter how much we write it is just to set EOF
// this also sets the entire COW bitmap
// to zero without having to allocate it
//
    err = cow_write_file(fd, &zero, sizeof(zero));
    if (err != sizeof(zero)) {
    cow_printf("Write of bitmap to new COW file '%s' failed, "
    "err = %d\n", cow_file, -err);
    if (err >= 0)
    err = -EINVAL;
    goto out;
    }
    return 0;
    out:
    return err;
    }
