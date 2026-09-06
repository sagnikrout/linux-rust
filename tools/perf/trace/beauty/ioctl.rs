//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/ioctl.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// trace/beauty/ioctl.c
//
// Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

//
// FIXME: to support all arches we have to improve this, for
// now, to build on older systems without things like TIOCGEXCL,
// get it directly from our copy.
//
// Right now only x86 is being supported for beautifying ioctl args
// in 'perf trace', see tools/perf/trace/beauty/Build and builtin-trace.c
//

#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_tty_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_tty_cmd(int nr, int dir, char *bf, size_t size)
    {
    static const char *ioctl_tty_cmd[] = {
    [_IOC_NR(TCGETS)] = "TCGETS", "TCSETS", "TCSETSW", "TCSETSF", "TCGETA", "TCSETA", "TCSETAW",
    "TCSETAF", "TCSBRK", "TCXONC", "TCFLSH", "TIOCEXCL", "TIOCNXCL", "TIOCSCTTY",
    "TIOCGPGRP", "TIOCSPGRP", "TIOCOUTQ", "TIOCSTI", "TIOCGWINSZ", "TIOCSWINSZ",
    "TIOCMGET", "TIOCMBIS", "TIOCMBIC", "TIOCMSET", "TIOCGSOFTCAR", "TIOCSSOFTCAR",
    "FIONREAD", "TIOCLINUX", "TIOCCONS", "TIOCGSERIAL", "TIOCSSERIAL", "TIOCPKT",
    "FIONBIO", "TIOCNOTTY", "TIOCSETD", "TIOCGETD", "TCSBRKP",
    [_IOC_NR(TIOCSBRK)] = "TIOCSBRK", "TIOCCBRK", "TIOCGSID", "TCGETS2", "TCSETS2",
    "TCSETSW2", "TCSETSF2", "TIOCGRS48", "TIOCSRS485", "TIOCGPTN", "TIOCSPTLCK",
    "TIOCGDEV", "TCSETX", "TCSETXF", "TCSETXW", "TIOCSIG", "TIOCVHANGUP", "TIOCGPKT",
    "TIOCGPTLCK", [_IOC_NR(TIOCGEXCL)] = "TIOCGEXCL", "TIOCGPTPEER",
    "TIOCGISO7816", "TIOCSISO7816",
    [_IOC_NR(FIONCLEX)] = "FIONCLEX", "FIOCLEX", "FIOASYNC", "TIOCSERCONFIG",
    "TIOCSERGWILD", "TIOCSERSWILD", "TIOCGLCKTRMIOS", "TIOCSLCKTRMIOS",
    "TIOCSERGSTRUCT", "TIOCSERGETLSR", "TIOCSERGETMULTI", "TIOCSERSETMULTI",
    "TIOCMIWAIT", "TIOCGICOUNT", };
    static DEFINE_STRARRAY(ioctl_tty_cmd, "");
    if (nr < strarray__ioctl_tty_cmd.nr_entries && strarray__ioctl_tty_cmd.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "%s", strarray__ioctl_tty_cmd.entries[nr]);
    return scnprintf(bf, size, "(%#x, %#x, %#x)", 'T', nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_drm_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_drm_cmd(int nr, int dir, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(drm_ioctl_cmds, "");
    if (nr < strarray__drm_ioctl_cmds.nr_entries && strarray__drm_ioctl_cmds.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "DRM_%s", strarray__drm_ioctl_cmds.entries[nr]);
    return scnprintf(bf, size, "(%#x, %#x, %#x)", 'd', nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_sndrv_pcm_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_sndrv_pcm_cmd(int nr, int dir, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(sndrv_pcm_ioctl_cmds, "");
    if (nr < strarray__sndrv_pcm_ioctl_cmds.nr_entries && strarray__sndrv_pcm_ioctl_cmds.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "SNDRV_PCM_%s", strarray__sndrv_pcm_ioctl_cmds.entries[nr]);
    return scnprintf(bf, size, "(%#x, %#x, %#x)", 'A', nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_sndrv_ctl_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_sndrv_ctl_cmd(int nr, int dir, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(sndrv_ctl_ioctl_cmds, "");
    if (nr < strarray__sndrv_ctl_ioctl_cmds.nr_entries && strarray__sndrv_ctl_ioctl_cmds.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "SNDRV_CTL_%s", strarray__sndrv_ctl_ioctl_cmds.entries[nr]);
    return scnprintf(bf, size, "(%#x, %#x, %#x)", 'U', nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_kvm_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_kvm_cmd(int nr, int dir, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(kvm_ioctl_cmds, "");
    if (nr < strarray__kvm_ioctl_cmds.nr_entries && strarray__kvm_ioctl_cmds.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "KVM_%s", strarray__kvm_ioctl_cmds.entries[nr]);
    return scnprintf(bf, size, "(%#x, %#x, %#x)", 0xAE, nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_vhost_virtio_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_vhost_virtio_cmd(int nr, int dir, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(vhost_virtio_ioctl_cmds, "");
    static DEFINE_STRARRAY(vhost_virtio_ioctl_read_cmds, "");
    struct strarray *s = (dir & _IOC_READ) ? &strarray__vhost_virtio_ioctl_read_cmds : &strarray__vhost_virtio_ioctl_cmds;
    if (nr < s.nr_entries && s.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "VHOST_%s", s.entries[nr]);
    return scnprintf(bf, size, "(%#x, %#x, %#x)", 0xAF, nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_perf_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_perf_cmd(int nr, int dir, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(perf_ioctl_cmds, "");
    if (nr < strarray__perf_ioctl_cmds.nr_entries && strarray__perf_ioctl_cmds.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "PERF_%s", strarray__perf_ioctl_cmds.entries[nr]);
    return scnprintf(bf, size, "(%#x, %#x, %#x)", 0xAE, nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_usbdevfs_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t ioctl__scnprintf_usbdevfs_cmd(int nr, int dir, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(usbdevfs_ioctl_cmds, "");
    if (nr < strarray__usbdevfs_ioctl_cmds.nr_entries && strarray__usbdevfs_ioctl_cmds.entries[nr] != core::ptr::null_mut())
    return scnprintf(bf, size, "USBDEVFS_%s", strarray__usbdevfs_ioctl_cmds.entries[nr]);
    return scnprintf(bf, size, "(%c, %#x, %#x)", 'U', nr, dir);
    }
#[no_mangle]
unsafe extern "C" fn ioctl__scnprintf_cmd(cmd: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t ioctl__scnprintf_cmd(unsigned long cmd, char *bf, size_t size, bool show_prefix)
    {
    const char *prefix = "_IOC_";
    int dir	 = _IOC_DIR(cmd),
    type = _IOC_TYPE(cmd),
    nr	 = _IOC_NR(cmd),
    sz	 = _IOC_SIZE(cmd);
    let mut printed: c_int = 0;
    static const struct ioctl_type {
    int	type;
    size_t	(*scnprintf)(int nr, int dir, char *bf, size_t size);
    } ioctl_types[] = { /* Must be ordered by type */
    { .type	= '$', .scnprintf = ioctl__scnprintf_perf_cmd, },
    ['A' - '$'] = { .type	= 'A', .scnprintf = ioctl__scnprintf_sndrv_pcm_cmd, },
    ['T' - '$'] = { .type	= 'T', .scnprintf = ioctl__scnprintf_tty_cmd, },
    ['U' - '$'] = { .type	= 'U', .scnprintf = ioctl__scnprintf_sndrv_ctl_cmd, },
    ['d' - '$'] = { .type	= 'd', .scnprintf = ioctl__scnprintf_drm_cmd, },
    [0xAE - '$'] = { .type	= 0xAE, .scnprintf = ioctl__scnprintf_kvm_cmd, },
    [0xAF - '$'] = { .type	= 0xAF, .scnprintf = ioctl__scnprintf_vhost_virtio_cmd, },
    };
    let mut nr_types: c_int = ARRAY_SIZE(ioctl_types);
    if (type >= ioctl_types[0].type && type <= ioctl_types[nr_types - 1].type) {
    let mut index: c_int = type - ioctl_types[0].type;
    if (ioctl_types[index].scnprintf != core::ptr::null_mut())
    return ioctl_types[index].scnprintf(nr, dir, bf, size);
    }
    printed += scnprintf(bf + printed, size - printed, "%c", '(');
    if (dir == _IOC_NONE) {
    printed += scnprintf(bf + printed, size - printed, "%s%s", show_prefix ? prefix : "", "NONE");
    } else {
    if (dir & _IOC_READ)
    printed += scnprintf(bf + printed, size - printed, "%s%s", show_prefix ? prefix : "", "READ");
    if (dir & _IOC_WRITE) {
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", dir & _IOC_READ ? "|" : "",
    show_prefix ? prefix : "",  "WRITE");
    }
    }
    return printed + scnprintf(bf + printed, size - printed, ", %#x, %#x, %#x)", type, nr, sz);
    }

pub const USB_DEVICE_MAJOR: c_int = 189;

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_ioctl_cmd(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_ioctl_cmd(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut cmd: c_ulong = arg.val;
    let mut fd: c_int = syscall_arg__val(arg, 0);
    struct file *file = thread__files_entry(arg.thread, fd);
    if (file != core::ptr::null_mut()) {
    if (file.dev_maj == USB_DEVICE_MAJOR)
    return ioctl__scnprintf_usbdevfs_cmd(_IOC_NR(cmd), _IOC_DIR(cmd), bf, size);
    }
    return ioctl__scnprintf_cmd(cmd, bf, size, arg.show_string_prefix);
    }
