//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/dmabuf_iter.c
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
// Copyright (c) 2025 Google

    let mut udmabuf: static int = -1;
    static const char udmabuf_test_buffer_name[DMA_BUF_NAME_LEN] = "udmabuf_test_buffer_for_iter";
    static size_t udmabuf_test_buffer_size;
    let mut sysheap_dmabuf: static int = -1;
    static const char sysheap_test_buffer_name[DMA_BUF_NAME_LEN] = "sysheap_test_buffer_for_iter";
    static size_t sysheap_test_buffer_size;
#[no_mangle]
unsafe extern "C" fn create_udmabuf() -> c_int {
    static int create_udmabuf(void)
    {
    struct udmabuf_create create;
    int dev_udmabuf, memfd, local_udmabuf;
    udmabuf_test_buffer_size = 10 * getpagesize();
    if (!ASSERT_LE(sizeof(udmabuf_test_buffer_name), DMA_BUF_NAME_LEN, "NAMETOOLONG"))
    return -1;
    memfd = memfd_create("memfd_test", MFD_ALLOW_SEALING);
    if (!ASSERT_OK_FD(memfd, "memfd_create"))
    return -1;
    if (!ASSERT_OK(ftruncate(memfd, udmabuf_test_buffer_size), "ftruncate"))
    goto close_memfd;
    if (!ASSERT_OK(fcntl(memfd, F_ADD_SEALS, F_SEAL_SHRINK), "seal"))
    goto close_memfd;
    dev_udmabuf = open("/dev/udmabuf", O_RDONLY);
    if (!ASSERT_OK_FD(dev_udmabuf, "open udmabuf"))
    goto close_memfd;
    memset(&create, 0, sizeof(create));
    create.memfd = memfd;
    create.flags = UDMABUF_FLAGS_CLOEXEC;
    create.offset = 0;
    create.size = udmabuf_test_buffer_size;
    local_udmabuf = ioctl(dev_udmabuf, UDMABUF_CREATE, &create);
    close(dev_udmabuf);
    if (!ASSERT_OK_FD(local_udmabuf, "udmabuf_create"))
    goto close_memfd;
    if (!ASSERT_OK(ioctl(local_udmabuf, DMA_BUF_SET_NAME_B, udmabuf_test_buffer_name), "name"))
    goto close_udmabuf;
    return local_udmabuf;
    close_udmabuf:
    close(local_udmabuf);
    close_memfd:
    close(memfd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn create_sys_heap_dmabuf(bytes: usize) -> c_int {
    static int create_sys_heap_dmabuf(size_t bytes)
    {
    struct dma_heap_allocation_data data = {
    .len = bytes,
    .fd = 0,
    .fd_flags = O_RDWR | O_CLOEXEC,
    .heap_flags = 0,
    };
    int heap_fd, ret;
    if (!ASSERT_LE(sizeof(sysheap_test_buffer_name), DMA_BUF_NAME_LEN, "NAMETOOLONG"))
    return -1;
    heap_fd = open("/dev/dma_heap/system", O_RDONLY);
    if (!ASSERT_OK_FD(heap_fd, "open dma heap"))
    return -1;
    ret = ioctl(heap_fd, DMA_HEAP_IOCTL_ALLOC, &data);
    close(heap_fd);
    if (!ASSERT_OK(ret, "syheap alloc"))
    return -1;
    if (!ASSERT_OK(ioctl(data.fd, DMA_BUF_SET_NAME_B, sysheap_test_buffer_name), "name"))
    goto close_sysheap_dmabuf;
    return data.fd;
    close_sysheap_dmabuf:
    close(data.fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn create_test_buffers() -> c_int {
    static int create_test_buffers(void)
    {
    udmabuf = create_udmabuf();
    sysheap_test_buffer_size = 20 * getpagesize();
    sysheap_dmabuf = create_sys_heap_dmabuf(sysheap_test_buffer_size);
    if (udmabuf < 0 || sysheap_dmabuf < 0)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn destroy_test_buffers() {
    static void destroy_test_buffers(void)
    {
    close(udmabuf);
    udmabuf = -1;
    close(sysheap_dmabuf);
    sysheap_dmabuf = -1;
    }
    enum Fields { INODE, SIZE, NAME, EXPORTER, FIELD_COUNT };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DmabufInfo {
    pub inode: c_ulong,
    pub size: c_ulong,
    pub name: [c_char; DMA_BUF_NAME_LEN],
    pub exporter: [c_char; 32],
}

    static bool check_dmabuf_info(const struct DmabufInfo *bufinfo,
    unsigned long size,
    const char *name, const char *exporter)
    {
    return size == bufinfo.size &&
    !strcmp(name, bufinfo.name) &&
    !strcmp(exporter, bufinfo.exporter);
    }
#[no_mangle]
unsafe extern "C" fn subtest_dmabuf_iter_check_no_infinite_reads(skel: *mut dmabuf_iter) {
    static void subtest_dmabuf_iter_check_no_infinite_reads(struct dmabuf_iter *skel)
    {
    int iter_fd;
    char buf[256];
    iter_fd = bpf_iter_create(bpf_link__fd(skel.links.dmabuf_collector));
    if (!ASSERT_OK_FD(iter_fd, "iter_create"))
    return;
    while (read(iter_fd, buf, sizeof(buf)) > 0)
    ; /* Read out all contents */
// Next reads should return 0
    ASSERT_EQ(read(iter_fd, buf, sizeof(buf)), 0, "read");
    close(iter_fd);
    }
#[no_mangle]
unsafe extern "C" fn subtest_dmabuf_iter_check_default_iter(skel: *mut dmabuf_iter) {
    static void subtest_dmabuf_iter_check_default_iter(struct dmabuf_iter *skel)
    {
    let mut found_test_sysheap_dmabuf: bool = false;
    let mut found_test_udmabuf: bool = false;
    struct DmabufInfo bufinfo;
    let mut linesize: usize = 0;
    char *line = core::ptr::null_mut();
    FILE *iter_file;
    int iter_fd, f = INODE;
    iter_fd = bpf_iter_create(bpf_link__fd(skel.links.dmabuf_collector));
    if (!ASSERT_OK_FD(iter_fd, "iter_create"))
    return;
    iter_file = fdopen(iter_fd, "r");
    if (!ASSERT_OK_PTR(iter_file, "fdopen"))
    goto close_iter_fd;
    while (getline(&line, &linesize, iter_file) != -1) {
    if (f % FIELD_COUNT == INODE) {
    ASSERT_EQ(sscanf(line, "%ld", &bufinfo.inode), 1,
    "read inode");
    } else if (f % FIELD_COUNT == SIZE) {
    ASSERT_EQ(sscanf(line, "%ld", &bufinfo.size), 1,
    "read size");
    } else if (f % FIELD_COUNT == NAME) {
    ASSERT_EQ(sscanf(line, "%s", bufinfo.name), 1,
    "read name");
    } else if (f % FIELD_COUNT == EXPORTER) {
    ASSERT_EQ(sscanf(line, "%31s", bufinfo.exporter), 1,
    "read exporter");
    if (check_dmabuf_info(&bufinfo,
    sysheap_test_buffer_size,
    sysheap_test_buffer_name,
    "system"))
    found_test_sysheap_dmabuf = true;
    else if (check_dmabuf_info(&bufinfo,
    udmabuf_test_buffer_size,
    udmabuf_test_buffer_name,
    "udmabuf"))
    found_test_udmabuf = true;
    }
    ++f;
    }
    ASSERT_EQ(f % FIELD_COUNT, INODE, "number of fields");
    ASSERT_TRUE(found_test_sysheap_dmabuf, "found_test_sysheap_dmabuf");
    ASSERT_TRUE(found_test_udmabuf, "found_test_udmabuf");
    free(line);
    fclose(iter_file);
    close_iter_fd:
    close(iter_fd);
    }
#[no_mangle]
unsafe extern "C" fn subtest_dmabuf_iter_check_lots_of_buffers(skel: *mut dmabuf_iter) {
    static void subtest_dmabuf_iter_check_lots_of_buffers(struct dmabuf_iter *skel)
    {
    int iter_fd;
    char buf[1024];
    let mut total_bytes_read: usize = 0;
    ssize_t bytes_read;
    iter_fd = bpf_iter_create(bpf_link__fd(skel.links.dmabuf_collector));
    if (!ASSERT_OK_FD(iter_fd, "iter_create"))
    return;
    while ((bytes_read = read(iter_fd, buf, sizeof(buf))) > 0)
    total_bytes_read += bytes_read;
    ASSERT_GT(total_bytes_read, 4096, "total_bytes_read");
    close(iter_fd);
    }
#[no_mangle]
unsafe extern "C" fn subtest_dmabuf_iter_check_open_coded(skel: *mut dmabuf_iter, map_fd: c_int) {
    static void subtest_dmabuf_iter_check_open_coded(struct dmabuf_iter *skel, int map_fd)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    char key[DMA_BUF_NAME_LEN];
    int err, fd;
    bool found;
// No need to attach it, just run it directly
    fd = bpf_program__fd(skel.progs.iter_dmabuf_for_each);
    err = bpf_prog_test_run_opts(fd, &topts);
    if (!ASSERT_OK(err, "test_run_opts err"))
    return;
    if (!ASSERT_OK(topts.retval, "test_run_opts retval"))
    return;
    if (!ASSERT_OK(bpf_map_get_next_key(map_fd, core::ptr::null_mut(), key), "get next key"))
    return;
    do {
    ASSERT_OK(bpf_map_lookup_elem(map_fd, key, &found), "lookup");
    ASSERT_TRUE(found, "found test buffer");
    } while (bpf_map_get_next_key(map_fd, key, key));
    }
#[no_mangle]
pub unsafe extern "C" fn test_dmabuf_iter() {
    void test_dmabuf_iter(void)
    {
    struct dmabuf_iter *skel = core::ptr::null_mut();
    int map_fd;
    let mut f: bool = false;
    skel = dmabuf_iter__open_and_load();
    if (!ASSERT_OK_PTR(skel, "dmabuf_iter__open_and_load"))
    return;
    map_fd = bpf_map__fd(skel.maps.testbuf_hash);
    if (!ASSERT_OK_FD(map_fd, "map_fd"))
    goto destroy_skel;
    if (!ASSERT_OK(bpf_map_update_elem(map_fd, udmabuf_test_buffer_name, &f, BPF_ANY),
    "insert udmabuf"))
    goto destroy_skel;
    if (!ASSERT_OK(bpf_map_update_elem(map_fd, sysheap_test_buffer_name, &f, BPF_ANY),
    "insert sysheap buffer"))
    goto destroy_skel;
    if (!ASSERT_OK(create_test_buffers(), "create_test_buffers"))
    goto destroy;
    if (!ASSERT_OK(dmabuf_iter__attach(skel), "skel_attach"))
    goto destroy;
    if (test__start_subtest("no_infinite_reads"))
    subtest_dmabuf_iter_check_no_infinite_reads(skel);
    if (test__start_subtest("default_iter"))
    subtest_dmabuf_iter_check_default_iter(skel);
    if (test__start_subtest("lots_of_buffers")) {
    let mut NUM_BUFS: usize = 100;
    int buffers[NUM_BUFS];
    int i;
    for (i = 0; i < NUM_BUFS; ++i) {
    buffers[i] = create_sys_heap_dmabuf(getpagesize());
    if (!ASSERT_OK_FD(buffers[i], "dmabuf_fd"))
    goto cleanup_bufs;
    }
    subtest_dmabuf_iter_check_lots_of_buffers(skel);
    cleanup_bufs:
    for (--i; i >= 0; --i)
    close(buffers[i]);
    }
    if (test__start_subtest("open_coded"))
    subtest_dmabuf_iter_check_open_coded(skel, map_fd);
    destroy:
    destroy_test_buffers();
    destroy_skel:
    dmabuf_iter__destroy(skel);
    }
