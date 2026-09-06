//! Automatically rewritten from C to Rust
//! Source: samples/fanotify/fs-monitor.c
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
// Copyright 2021, Collabora Ltd.
//
// Macro flag: #define _GNU_SOURCE

pub const FAN_FS_ERROR: c_uint = 0x00008000;
pub const FAN_EVENT_INFO_TYPE_ERROR: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_info_error {
    pub hdr: fanotify_event_info_header,
    pub error: __s32,
    pub error_count: __u32,
}

pub const FILEID_INO32_GEN: c_int = 1;

pub const FILEID_INVALID: c_uint = 0xff;

#[no_mangle]
unsafe extern "C" fn print_fh(fh: *mut file_handle) {
    static void print_fh(struct file_handle *fh)
    {
    int i;
    uint32_t *h = (uint32_t *) fh.f_handle;
    printf("\tfh: ");
    for (i = 0; i < fh.handle_bytes; i++)
    printf("%hhx", fh.f_handle[i]);
    printf("\n");
    printf("\tdecoded fh: ");
    if (fh.handle_type == FILEID_INO32_GEN)
    printf("inode=%u gen=%u\n", h[0], h[1]);
#[no_mangle]
pub unsafe extern "C" fn if(!fh->handle_bytes: fh->handle_type == FILEID_INVALID &&) -> else {
    else if (fh.handle_type == FILEID_INVALID && !fh.handle_bytes)
    printf("Type %d (Superblock error)\n", fh.handle_type);
    else
    printf("Type %d (Unknown)\n", fh.handle_type);
    }
#[no_mangle]
unsafe extern "C" fn handle_notifications(buffer: *mut c_char, len: c_int) {
    static void handle_notifications(char *buffer, int len)
    {
    struct fanotify_event_metadata *event =
    (struct fanotify_event_metadata *) buffer;
    struct fanotify_event_info_header *info;
    struct fanotify_event_info_error *err;
    struct fanotify_event_info_fid *fid;
    int off;
    for (; FAN_EVENT_OK(event, len); event = FAN_EVENT_NEXT(event, len)) {
    if (event.mask != FAN_FS_ERROR) {
    printf("unexpected FAN MARK: %llx\n",
    (unsigned long long)event.mask);
    goto next_event;
    }
    if (event.fd != FAN_NOFD) {
    printf("Unexpected fd (!= FAN_NOFD)\n");
    goto next_event;
    }
    printf("FAN_FS_ERROR (len=%d)\n", event.event_len);
    for (off = sizeof(*event) ; off < event.event_len;
    off += info.len) {
    info = (struct fanotify_event_info_header *)
    ((char *) event + off);
    switch (info.info_type) {
    case FAN_EVENT_INFO_TYPE_ERROR:
    err = (struct fanotify_event_info_error *) info;
    printf("\tGeneric Error Record: len=%d\n",
    err.hdr.len);
    printf("\terror: %d\n", err.error);
    printf("\terror_count: %d\n", err.error_count);
    break;
    case FAN_EVENT_INFO_TYPE_FID:
    fid = (struct fanotify_event_info_fid *) info;
    printf("\tfsid: %x%x\n",

    fid.fsid.val[0], fid.fsid.val[1]);

    fid.fsid.__val[0], fid.fsid.__val[1]);

    print_fh((struct file_handle *) &fid.handle);
    break;
    default:
    printf("\tUnknown info type=%d len=%d:\n",
    info.info_type, info.len);
    }
    }
    next_event:
    printf("---\n\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int fd;
    char buffer[BUFSIZ];
    if (argc < 2) {
    printf("Missing path argument\n");
    return 1;
    }
    fd = fanotify_init(FAN_CLASS_NOTIF|FAN_REPORT_FID, O_RDONLY);
    if (fd < 0)
    errx(1, "fanotify_init");
    if (fanotify_mark(fd, FAN_MARK_ADD|FAN_MARK_FILESYSTEM,
    FAN_FS_ERROR, AT_FDCWD, argv[1])) {
    errx(1, "fanotify_mark");
    }
    while (1) {
    let mut n: c_int = read(fd, buffer, BUFSIZ);
    if (n < 0)
    errx(1, "read");
    handle_notifications(buffer, n);
    }
    return 0;
    }
