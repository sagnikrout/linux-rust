//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/perf_events/mmap.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Macro flag: #define _GNU_SOURCE

pub const RB_SIZE: c_uint = 0x3000;
pub const AUX_SIZE: c_uint = 0x10000;
pub const AUX_OFFS: c_uint = 0x4000;
pub const HOLE_SIZE: c_uint = 0x1000;
// Reserve space for rb, aux with space for shrink-beyond-vma testing.

pub const MAP_BASE: c_int = 1;
pub const MAP_AUX: c_int = 2;

    FIXTURE(perf_mmap)
    {
    int		fd;
    void		*ptr;
    void		*region;
    };
    FIXTURE_VARIANT(perf_mmap)
    {
    bool		aux;
    unsigned long	ptr_size;
    };
    FIXTURE_VARIANT_ADD(perf_mmap, rb)
    {
    .aux = false,
    .ptr_size = RB_SIZE,
    };
    FIXTURE_VARIANT_ADD(perf_mmap, aux)
    {
    .aux = true,
    .ptr_size = AUX_SIZE,
    };
#[no_mangle]
unsafe extern "C" fn read_event_type(dent: *mut dirent, type: *mut __u32) -> bool {
    static bool read_event_type(struct dirent *dent, __u32 *type)
    {
    char typefn[512];
    FILE *fp;
    int res;
    snprintf(typefn, sizeof(typefn), "%s/%s/type", EVENT_SRC_DIR, dent.d_name);
    fp = fopen(typefn, "r");
    if (!fp)
    return false;
    res = fscanf(fp, "%u", type);
    fclose(fp);
    return res > 0;
    }
    FIXTURE_SETUP(perf_mmap)
    {
    struct perf_event_attr attr = {
    .size		= sizeof(attr),
    .disabled	= 1,
    .exclude_kernel	= 1,
    .exclude_hv	= 1,
    };
    let mut attr_ok: perf_event_attr = {};
    let mut eacces: c_uint = 0, map = 0;
    struct perf_event_mmap_page *rb;
    struct dirent *dent;
    void *aux, *region;
    DIR *dir;
    self.ptr = core::ptr::null_mut();
    dir = opendir(EVENT_SRC_DIR);
    if (!dir)
    SKIP(return, "perf not available.");
    region = mmap(core::ptr::null_mut(), REGION_SIZE, PROT_NONE, MAP_ANON | MAP_PRIVATE, -1, 0);
    ASSERT_NE(region, MAP_FAILED);
    self.region = region;
// Try to find a suitable event on this system
    while ((dent = readdir(dir))) {
    int fd;
    if (!read_event_type(dent, &attr.type))
    continue;
    fd = syscall(SYS_perf_event_open, &attr, 0, -1, -1, 0);
    if (fd < 0) {
    if (errno == EACCES)
    eacces++;
    continue;
    }
// Check whether the event supports mmap()
    rb = mmap(region, RB_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_FIXED, fd, 0);
    if (rb == MAP_FAILED) {
    close(fd);
    continue;
    }
    if (!map) {
// Save the event in case that no AUX capable event is found
    attr_ok = attr;
    map = MAP_BASE;
    }
    if (!variant.aux)
    continue;
    rb.aux_offset = AUX_OFFS;
    rb.aux_size = AUX_SIZE;
// Check whether it supports a AUX buffer
    aux = mmap(region + REGION_AUX_OFFS, AUX_SIZE, PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_FIXED, fd, AUX_OFFS);
    if (aux == MAP_FAILED) {
    munmap(rb, RB_SIZE);
    close(fd);
    continue;
    }
    attr_ok = attr;
    map = MAP_AUX;
    munmap(aux, AUX_SIZE);
    munmap(rb, RB_SIZE);
    close(fd);
    break;
    }
    closedir(dir);
    if (!map) {
    if (!eacces)
    SKIP(return, "No mappable perf event found.");
    else
    SKIP(return, "No permissions for perf_event_open()");
    }
    self.fd = syscall(SYS_perf_event_open, &attr_ok, 0, -1, -1, 0);
    ASSERT_NE(self.fd, -1);
    rb = mmap(region, RB_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_FIXED, self.fd, 0);
    ASSERT_NE(rb, MAP_FAILED);
    if (!variant.aux) {
    self.ptr = rb;
    return;
    }
    if (map != MAP_AUX)
    SKIP(return, "No AUX event found.");
    rb.aux_offset = AUX_OFFS;
    rb.aux_size = AUX_SIZE;
    aux = mmap(region + REGION_AUX_OFFS, AUX_SIZE, PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_FIXED, self.fd, AUX_OFFS);
    ASSERT_NE(aux, MAP_FAILED);
    self.ptr = aux;
    }
    FIXTURE_TEARDOWN(perf_mmap)
    {
    ASSERT_EQ(munmap(self.region, REGION_SIZE), 0);
    if (self.fd != -1)
    ASSERT_EQ(close(self.fd), 0);
    }
    TEST_F(perf_mmap, remap)
    {
    void *tmp, *ptr = self.ptr;
    let mut size: c_ulong = variant.ptr_size;
// Test the invalid remaps
    ASSERT_EQ(mremap(ptr, size, HOLE_SIZE, MREMAP_MAYMOVE), MAP_FAILED);
    ASSERT_EQ(mremap(ptr + HOLE_SIZE, size, HOLE_SIZE, MREMAP_MAYMOVE), MAP_FAILED);
    ASSERT_EQ(mremap(ptr + size - HOLE_SIZE, HOLE_SIZE, size, MREMAP_MAYMOVE), MAP_FAILED);
// Shrink the end of the mapping such that we only unmap past end of the VMA,
// which should succeed and poke a hole into the PROT_NONE region
    ASSERT_NE(mremap(ptr + size - HOLE_SIZE, size, HOLE_SIZE, MREMAP_MAYMOVE), MAP_FAILED);
// Remap the whole buffer to a new address
    tmp = mmap(core::ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    ASSERT_NE(tmp, MAP_FAILED);
// Try splitting offset 1 hole size into VMA, this should fail
    ASSERT_EQ(mremap(ptr + HOLE_SIZE, size - HOLE_SIZE, size - HOLE_SIZE,
    MREMAP_MAYMOVE | MREMAP_FIXED, tmp), MAP_FAILED);
// Remapping the whole thing should succeed fine
    ptr = mremap(ptr, size, size, MREMAP_MAYMOVE | MREMAP_FIXED, tmp);
    ASSERT_EQ(ptr, tmp);
    ASSERT_EQ(munmap(tmp, size), 0);
    }
    TEST_F(perf_mmap, unmap)
    {
    let mut size: c_ulong = variant.ptr_size;
// Try to poke holes into the mappings
    ASSERT_NE(munmap(self.ptr, HOLE_SIZE), 0);
    ASSERT_NE(munmap(self.ptr + HOLE_SIZE, HOLE_SIZE), 0);
    ASSERT_NE(munmap(self.ptr + size - HOLE_SIZE, HOLE_SIZE), 0);
    }
    TEST_F(perf_mmap, map)
    {
    let mut size: c_ulong = variant.ptr_size;
// Try to poke holes into the mappings by mapping anonymous memory over it
    ASSERT_EQ(mmap(self.ptr, HOLE_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANON | MAP_FIXED, -1, 0), MAP_FAILED);
    ASSERT_EQ(mmap(self.ptr + HOLE_SIZE, HOLE_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANON | MAP_FIXED, -1, 0), MAP_FAILED);
    ASSERT_EQ(mmap(self.ptr + size - HOLE_SIZE, HOLE_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANON | MAP_FIXED, -1, 0), MAP_FAILED);
    }
    TEST_HARNESS_MAIN
