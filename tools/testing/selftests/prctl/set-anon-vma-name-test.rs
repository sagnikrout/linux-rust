//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/prctl/set-anon-vma-name-test.c
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
// This test covers the anonymous VMA naming functionality through prctl calls
//

pub const AREA_SIZE: c_int = 1024;

pub const PR_SET_VMA: c_uint = 0x53564d41;
pub const PR_SET_VMA_ANON_NAME: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn rename_vma(addr: c_ulong, size: c_ulong, name: *mut c_char) -> c_int {
    int rename_vma(unsigned long addr, unsigned long size, char *name)
    {
    int res;
    res = prctl(PR_SET_VMA, PR_SET_VMA_ANON_NAME, addr, size, name);
    if (res < 0)
    return -errno;
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn was_renaming_successful(target_name: *mut c_char, ptr: c_ulong) -> c_int {
    int was_renaming_successful(char *target_name, unsigned long ptr)
    {
    FILE *maps_file;
    char line_buf[512], name[128], mode[8];
    unsigned long start_addr, end_addr, offset;
    unsigned int major_id, minor_id, node_id;
    char target_buf[128];
    let mut res: c_int = 0, sscanf_res;
// The entry name in maps will be in format [anon:<target_name>]
    sprintf(target_buf, "[anon:%s]", target_name);
    maps_file = fopen("/proc/self/maps", "r");
    if (!maps_file) {
    printf("## /proc/self/maps file opening error\n");
    return 0;
    }
// Parse the maps file to find the entry we renamed
    while (fgets(line_buf, sizeof(line_buf), maps_file)) {
    sscanf_res = sscanf(line_buf, "%lx-%lx %7s %lx %u:%u %u %s", &start_addr,
    &end_addr, mode, &offset, &major_id,
    &minor_id, &node_id, name);
    if (sscanf_res == EOF) {
    res = 0;
    printf("## EOF while parsing the maps file\n");
    break;
    }
    if (!strcmp(name, target_buf) && start_addr == ptr) {
    res = 1;
    break;
    }
    }
    fclose(maps_file);
    return res;
    }
    FIXTURE(vma) {
    void *ptr_anon, *ptr_not_anon;
    int fd_not_anon;
    };
    FIXTURE_SETUP(vma) {
    char template[] = "./set-anon-vma-test-XXXXXX";
    self.ptr_anon = mmap(core::ptr::null_mut(), AREA_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    ASSERT_NE(self.ptr_anon, MAP_FAILED);
    self.fd_not_anon = mkstemp(template);
    ASSERT_NE(self.fd_not_anon, -1);
    unlink(template);
    ASSERT_EQ(ftruncate(self.fd_not_anon, AREA_SIZE), 0);
    self.ptr_not_anon = mmap(core::ptr::null_mut(), AREA_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE, self.fd_not_anon, 0);
    ASSERT_NE(self.ptr_not_anon, MAP_FAILED);
    close(self.fd_not_anon);
    }
    FIXTURE_TEARDOWN(vma) {
    munmap(self.ptr_anon, AREA_SIZE);
    munmap(self.ptr_not_anon, AREA_SIZE);
    }
    TEST_F(vma, renaming) {
    TH_LOG("Try to rename the VMA with correct parameters");
    EXPECT_GE(rename_vma((unsigned long)self.ptr_anon, AREA_SIZE, GOOD_NAME), 0);
    EXPECT_TRUE(was_renaming_successful(GOOD_NAME, (unsigned long)self.ptr_anon));
    TH_LOG("Try to pass invalid name (with non-printable character \\1) to rename the VMA");
    EXPECT_EQ(rename_vma((unsigned long)self.ptr_anon, AREA_SIZE, BAD_NAME), -EINVAL);
    TH_LOG("Try to rename non-anonymous VMA");
    EXPECT_EQ(rename_vma((unsigned long) self.ptr_not_anon, AREA_SIZE, GOOD_NAME), -EBADF);
    }
    TEST_HARNESS_MAIN
