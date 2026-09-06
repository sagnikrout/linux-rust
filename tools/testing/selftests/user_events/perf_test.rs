//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/user_events/perf_test.c
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
// User Events Perf Events Test Program
//
// Copyright (c) 2021 Beau Belgrave <beaub@linux.microsoft.com>
//

    const char *data_file = "/sys/kernel/tracing/user_events_data";
    const char *id_file = "/sys/kernel/tracing/events/user_events/__test_event/id";
    const char *fmt_file = "/sys/kernel/tracing/events/user_events/__test_event/format";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub index: __u32,
    pub field1: __u32,
    pub field2: __u32,
}

    static long perf_event_open(struct perf_event_attr *pe, pid_t pid,
    int cpu, int group_fd, unsigned long flags)
    {
    return syscall(__NR_perf_event_open, pe, pid, cpu, group_fd, flags);
    }
#[no_mangle]
unsafe extern "C" fn get_id() -> c_int {
    static int get_id(void)
    {
    FILE *fp = fopen(id_file, "r");
    int ret, id = 0;
    if (!fp)
    return -1;
    ret = fscanf(fp, "%d", &id);
    fclose(fp);
    if (ret != 1)
    return -1;
    return id;
    }
#[no_mangle]
unsafe extern "C" fn get_offset() -> c_int {
    static int get_offset(void)
    {
    FILE *fp = fopen(fmt_file, "r");
    int ret, c, last = 0, offset = 0;
    if (!fp)
    return -1;
// Read until empty line
    while (true) {
    c = getc(fp);
    if (c == EOF)
    break;
    if (last == '\n' && c == '\n')
    break;
    last = c;
    }
    ret = fscanf(fp, "\tfield:u32 field1;\toffset:%d;", &offset);
    fclose(fp);
    if (ret != 1)
    return -1;
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn clear(check: *mut c_int) -> c_int {
    static int clear(int *check)
    {
    let mut unreg: user_unreg = {0};
    int i, ret;
    unreg.size = sizeof(unreg);
    unreg.disable_bit = 31;
    unreg.disable_addr = (__u64)check;
    let mut fd: c_int = open(data_file, O_RDWR);
    if (fd == -1)
    return -1;
    if (ioctl(fd, DIAG_IOCSUNREG, &unreg) == -1)
    if (errno != ENOENT)
    return -1;
//
// Deleting the event drops its last reference, but the unregister
// above defers that put (and the freeing of the enabler) past an RCU
// grace period. The delete can therefore transiently fail with -EBUSY
// until that reference is dropped. Retry for up to ~10 seconds so the
// event is actually gone before the next test registers the same name.
//
    for (i = 0; i < 10000; ++i) {
    ret = ioctl(fd, DIAG_IOCSDEL, "__test_event");
    if (ret == 0 || errno == ENOENT) {
    ret = 0;
    break;
    }
    if (errno != EBUSY) {
    close(fd);
    return -1;
    }
    usleep(1000);
    }
    close(fd);
    return ret;
    }
    FIXTURE(user) {
    int data_fd;
    int check;
    bool umount;
    };
    FIXTURE_SETUP(user) {
    USER_EVENT_FIXTURE_SETUP(return, self.umount);
    self.data_fd = open(data_file, O_RDWR);
    ASSERT_NE(-1, self.data_fd);
    }
    FIXTURE_TEARDOWN(user) {
    USER_EVENT_FIXTURE_TEARDOWN(self.umount);
    close(self.data_fd);
    if (clear(&self.check) != 0)
    printf("WARNING: Clear didn't work!\n");
    }
    TEST_F(user, perf_write) {
    let mut pe: perf_event_attr = {0};
    let mut reg: user_reg = {0};
    struct event event;
    struct perf_event_mmap_page *perf_page;
    let mut page_size: c_int = sysconf(_SC_PAGESIZE);
    int id, fd, offset;
    __u32 *val;
    reg.size = sizeof(reg);
    reg.name_args = (__u64)"__test_event u32 field1; u32 field2";
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)&self.check;
    reg.enable_size = sizeof(self.check);
// Register should work
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
    ASSERT_EQ(0, self.check);
// Id should be there
    id = get_id();
    ASSERT_NE(-1, id);
    offset = get_offset();
    ASSERT_NE(-1, offset);
    pe.type = PERF_TYPE_TRACEPOINT;
    pe.size = sizeof(pe);
    pe.config = id;
    pe.sample_type = PERF_SAMPLE_RAW;
    pe.sample_period = 1;
    pe.wakeup_events = 1;
// Tracepoint attach should work
    fd = perf_event_open(&pe, 0, -1, -1, 0);
    ASSERT_NE(-1, fd);
    perf_page = mmap(core::ptr::null_mut(), page_size * 2, PROT_READ, MAP_SHARED, fd, 0);
    ASSERT_NE(MAP_FAILED, perf_page);
// Status should be updated
    ASSERT_EQ(1 << reg.enable_bit, self.check);
    event.index = reg.write_index;
    event.field1 = 0xc001;
    event.field2 = 0xc01a;
// Ensure write shows up at correct offset
    ASSERT_NE(-1, write(self.data_fd, &event, sizeof(event)));
    val = (void *)(((char *)perf_page) + perf_page.data_offset);
    ASSERT_EQ(PERF_RECORD_SAMPLE, *val);
// Skip over header and size, move to offset
    val += 3;
    val = (void *)((char *)val) + offset;
// Ensure correct
    ASSERT_EQ(event.field1, *val++);
    ASSERT_EQ(event.field2, *val++);
    munmap(perf_page, page_size * 2);
    close(fd);
// Status should be updated
    ASSERT_EQ(0, self.check);
    }
    TEST_F(user, perf_empty_events) {
    let mut pe: perf_event_attr = {0};
    let mut reg: user_reg = {0};
    struct perf_event_mmap_page *perf_page;
    let mut page_size: c_int = sysconf(_SC_PAGESIZE);
    int id, fd;
    __u32 *val;
    reg.size = sizeof(reg);
    reg.name_args = (__u64)"__test_event";
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)&self.check;
    reg.enable_size = sizeof(self.check);
// Register should work
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
    ASSERT_EQ(0, self.check);
// Id should be there
    id = get_id();
    ASSERT_NE(-1, id);
    pe.type = PERF_TYPE_TRACEPOINT;
    pe.size = sizeof(pe);
    pe.config = id;
    pe.sample_type = PERF_SAMPLE_RAW;
    pe.sample_period = 1;
    pe.wakeup_events = 1;
// Tracepoint attach should work
    fd = perf_event_open(&pe, 0, -1, -1, 0);
    ASSERT_NE(-1, fd);
    perf_page = mmap(core::ptr::null_mut(), page_size * 2, PROT_READ, MAP_SHARED, fd, 0);
    ASSERT_NE(MAP_FAILED, perf_page);
// Status should be updated
    ASSERT_EQ(1 << reg.enable_bit, self.check);
// Ensure write shows up at correct offset
    ASSERT_NE(-1, write(self.data_fd, (void *)&reg.write_index,
    sizeof(reg.write_index)));
    val = (void *)(((char *)perf_page) + perf_page.data_offset);
    ASSERT_EQ(PERF_RECORD_SAMPLE, *val);
    munmap(perf_page, page_size * 2);
    close(fd);
// Status should be updated
    ASSERT_EQ(0, self.check);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    return test_harness_run(argc, argv);
    }
