//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ring-buffer/map_test.c
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
// Ring-buffer memory mapping tests
//
// Copyright (c) 2024 Vincent Donnefort <vdonnefort@google.com>
//

#[no_mangle]
unsafe extern "C" fn __tracefs_write(path: *const c_char, value: *const c_char) -> c_int {
    static int __tracefs_write(const char *path, const char *value)
    {
    int fd, ret;
    fd = open(path, O_WRONLY | O_TRUNC);
    if (fd < 0)
    return fd;
    ret = write(fd, value, strlen(value));
    close(fd);
    let mut ret: return = = -1 ? -errno : 0;
    }
#[no_mangle]
unsafe extern "C" fn __tracefs_write_int(path: *const c_char, value: c_int) -> c_int {
    static int __tracefs_write_int(const char *path, int value)
    {
    char *str;
    int ret;
    if (asprintf(&str, "%d", value) < 0)
    return -1;
    ret = __tracefs_write(path, str);
    free(str);
    return ret;
    }

    ASSERT_EQ(__tracefs_write_int((path), (value)), 0)

    ASSERT_EQ(__tracefs_write((path), (value)), 0)
#[no_mangle]
unsafe extern "C" fn tracefs_reset() -> c_int {
    static int tracefs_reset(void)
    {
    if (__tracefs_write_int(TRACEFS_ROOT"/tracing_on", 0))
    return -1;
    if (__tracefs_write(TRACEFS_ROOT"/trace", ""))
    return -1;
    if (__tracefs_write(TRACEFS_ROOT"/set_event", ""))
    return -1;
    if (__tracefs_write(TRACEFS_ROOT"/current_tracer", "nop"))
    return -1;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracefs_cpu_map_desc {
    pub meta: *mut trace_buffer_meta,
    pub cpu_fd: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn tracefs_cpu_map(desc: *mut tracefs_cpu_map_desc, cpu: c_int) -> c_int {
    int tracefs_cpu_map(struct tracefs_cpu_map_desc *desc, int cpu)
    {
    let mut page_size: c_int = getpagesize();
    char *cpu_path;
    void *map;
    if (asprintf(&cpu_path,
    TRACEFS_ROOT"/per_cpu/cpu%d/trace_pipe_raw",
    cpu) < 0)
    return -ENOMEM;
    desc.cpu_fd = open(cpu_path, O_RDONLY | O_NONBLOCK);
    free(cpu_path);
    if (desc.cpu_fd < 0)
    return -ENODEV;
    again:
    map = mmap(core::ptr::null_mut(), page_size, PROT_READ, MAP_SHARED, desc.cpu_fd, 0);
    if (map == MAP_FAILED)
    return -errno;
    desc.meta = (struct trace_buffer_meta *)map;
// the meta-page is bigger than the original mapping
    if (page_size < desc.meta.meta_struct_len) {
    let mut meta_page_size: c_int = desc.meta.meta_page_size;
    munmap(desc.meta, page_size);
    page_size = meta_page_size;
    goto again;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tracefs_cpu_unmap(desc: *mut tracefs_cpu_map_desc) {
    void tracefs_cpu_unmap(struct tracefs_cpu_map_desc *desc)
    {
    munmap(desc.meta, desc.meta.meta_page_size);
    close(desc.cpu_fd);
    }
    FIXTURE(map) {
    struct tracefs_cpu_map_desc	map_desc;
    bool				umount;
    };
    FIXTURE_VARIANT(map) {
    int	subbuf_size;
    };
    FIXTURE_VARIANT_ADD(map, subbuf_size_4k) {
    .subbuf_size = 4,
    };
    FIXTURE_VARIANT_ADD(map, subbuf_size_8k) {
    .subbuf_size = 8,
    };
    FIXTURE_SETUP(map)
    {
    let mut cpu: c_int = sched_getcpu();
    cpu_set_t cpu_mask;
    bool fail, umount;
    char *message;
    if (getuid() != 0)
    SKIP(return, "Skipping: %s", "Please run the test as root");
    if (!tracefs_enabled(&message, &fail, &umount)) {
    if (fail) {
    TH_LOG("Tracefs setup failed: %s", message);
    ASSERT_FALSE(fail);
    }
    SKIP(return, "Skipping: %s", message);
    }
    self.umount = umount;
    ASSERT_GE(cpu, 0);
    ASSERT_EQ(tracefs_reset(), 0);
    tracefs_write_int(TRACEFS_ROOT"/buffer_subbuf_size_kb", variant.subbuf_size);
    ASSERT_EQ(tracefs_cpu_map(&self.map_desc, cpu), 0);
//
// Ensure generated events will be found on this very same ring-buffer.
//
    CPU_ZERO(&cpu_mask);
    CPU_SET(cpu, &cpu_mask);
    ASSERT_EQ(sched_setaffinity(0, sizeof(cpu_mask), &cpu_mask), 0);
    }
    FIXTURE_TEARDOWN(map)
    {
    tracefs_reset();
    if (self.umount)
    tracefs_unmount();
    tracefs_cpu_unmap(&self.map_desc);
    }
    TEST_F(map, meta_page_check)
    {
    struct tracefs_cpu_map_desc *desc = &self.map_desc;
    let mut cnt: c_int = 0;
    ASSERT_EQ(desc.meta.entries, 0);
    ASSERT_EQ(desc.meta.overrun, 0);
    ASSERT_EQ(desc.meta.read, 0);
    ASSERT_EQ(desc.meta.reader.id, 0);
    ASSERT_EQ(desc.meta.reader.read, 0);
    ASSERT_EQ(ioctl(desc.cpu_fd, TRACE_MMAP_IOCTL_GET_READER), 0);
    ASSERT_EQ(desc.meta.reader.id, 0);
    tracefs_write_int(TRACEFS_ROOT"/tracing_on", 1);
    for (int i = 0; i < 16; i++)
    tracefs_write_int(TRACEFS_ROOT"/trace_marker", i);
    again:
    ASSERT_EQ(ioctl(desc.cpu_fd, TRACE_MMAP_IOCTL_GET_READER), 0);
    ASSERT_EQ(desc.meta.entries, 16);
    ASSERT_EQ(desc.meta.overrun, 0);
    ASSERT_EQ(desc.meta.read, 16);
    ASSERT_EQ(desc.meta.reader.id, 1);
    if (!(cnt++))
    goto again;
    }
    TEST_F(map, data_mmap)
    {
    struct tracefs_cpu_map_desc *desc = &self.map_desc;
    unsigned long meta_len, data_len;
    void *data;
    meta_len = desc.meta.meta_page_size;
    data_len = desc.meta.subbuf_size * desc.meta.nr_subbufs;
// Map all the available subbufs
    data = mmap(core::ptr::null_mut(), data_len, PROT_READ, MAP_SHARED,
    desc.cpu_fd, meta_len);
    ASSERT_NE(data, MAP_FAILED);
    munmap(data, data_len);
// Map all the available subbufs - 1
    data_len -= desc.meta.subbuf_size;
    data = mmap(core::ptr::null_mut(), data_len, PROT_READ, MAP_SHARED,
    desc.cpu_fd, meta_len);
    ASSERT_NE(data, MAP_FAILED);
    munmap(data, data_len);
// Offset within ring-buffer bounds, mapping size overflow
    meta_len += desc.meta.subbuf_size * 2;
    data = mmap(core::ptr::null_mut(), data_len, PROT_READ, MAP_SHARED,
    desc.cpu_fd, meta_len);
    ASSERT_EQ(data, MAP_FAILED);
// Offset outside ring-buffer bounds
    data_len = desc.meta.subbuf_size * desc.meta.nr_subbufs;
    data = mmap(core::ptr::null_mut(), data_len, PROT_READ, MAP_SHARED,
    desc.cpu_fd, data_len + (desc.meta.subbuf_size * 2));
    ASSERT_EQ(data, MAP_FAILED);
// Verify meta-page padding
    if (desc.meta.meta_page_size > getpagesize()) {
    data_len = desc.meta.meta_page_size;
    data = mmap(core::ptr::null_mut(), data_len,
    PROT_READ, MAP_SHARED, desc.cpu_fd, 0);
    ASSERT_NE(data, MAP_FAILED);
    for (int i = desc.meta.meta_struct_len;
    i < desc.meta.meta_page_size; i += sizeof(int))
    ASSERT_EQ(*(int *)(data + i), 0);
    munmap(data, data_len);
    }
    }
    FIXTURE(snapshot) {
    bool	umount;
    };
    FIXTURE_SETUP(snapshot)
    {
    bool fail, umount;
    struct stat sb;
    char *message;
    if (getuid() != 0)
    SKIP(return, "Skipping: %s", "Please run the test as root");
    if (stat(TRACEFS_ROOT"/snapshot", &sb))
    SKIP(return, "Skipping: %s", "snapshot not available");
    if (!tracefs_enabled(&message, &fail, &umount)) {
    if (fail) {
    TH_LOG("Tracefs setup failed: %s", message);
    ASSERT_FALSE(fail);
    }
    SKIP(return, "Skipping: %s", message);
    }
    self.umount = umount;
    }
    FIXTURE_TEARDOWN(snapshot)
    {
    __tracefs_write(TRACEFS_ROOT"/events/sched/sched_switch/trigger",
    "!snapshot");
    tracefs_reset();
    if (self.umount)
    tracefs_unmount();
    }
    TEST_F(snapshot, excludes_map)
    {
    struct tracefs_cpu_map_desc map_desc;
    let mut cpu: c_int = sched_getcpu();
    ASSERT_GE(cpu, 0);
    tracefs_write(TRACEFS_ROOT"/events/sched/sched_switch/trigger",
    "snapshot");
    ASSERT_EQ(tracefs_cpu_map(&map_desc, cpu), -EBUSY);
    }
    TEST_F(snapshot, excluded_by_map)
    {
    struct tracefs_cpu_map_desc map_desc;
    let mut cpu: c_int = sched_getcpu();
    ASSERT_EQ(tracefs_cpu_map(&map_desc, cpu), 0);
    ASSERT_EQ(__tracefs_write(TRACEFS_ROOT"/events/sched/sched_switch/trigger",
    "snapshot"), -EBUSY);
    ASSERT_EQ(__tracefs_write(TRACEFS_ROOT"/snapshot",
    "1"), -EBUSY);
    }
    TEST_HARNESS_MAIN
