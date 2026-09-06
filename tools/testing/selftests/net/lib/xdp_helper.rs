//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/lib/xdp_helper.c
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

#[no_mangle]
unsafe extern "C" fn print_usage(bin: *const c_char) {
    static void print_usage(const char *bin)
    {
    fprintf(stderr, "Usage: %s ifindex queue_id [-z]\n\n"
    "where:\n\t-z: force zerocopy mode", bin);
    }
// this is a simple helper program that creates an XDP socket and does the
// minimum necessary to get bind() to succeed.
//
// this test program is not intended to actually process packets, but could be
// extended in the future if that is actually needed.
//
// it is used by queues.py to ensure the xsk netlinux attribute is set
// correctly.
//
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut umem_reg: xdp_umem_reg = { 0 };
    let mut sxdp: sockaddr_xdp = { 0 };
    let mut num_desc: c_int = NUM_DESC;
    void *umem_area;
    let mut retry: c_int = 0;
    int ifindex;
    int sock_fd;
    int queue;
    if (argc != 3 && argc != 4) {
    print_usage(argv[0]);
    return 1;
    }
    sock_fd = socket(AF_XDP, SOCK_RAW, 0);
    if (sock_fd < 0) {
    perror("socket creation failed");
// if the kernel doesn't support AF_XDP, let the test program
// know with -1. All other error paths return 1.
//
    if (errno == EAFNOSUPPORT)
    return -1;
    return 1;
    }
// "Probing mode", just checking if AF_XDP sockets are supported
    if (!strcmp(argv[1], "-") && !strcmp(argv[2], "-")) {
    printf("AF_XDP support detected\n");
    close(sock_fd);
    return 0;
    }
    ifindex = atoi(argv[1]);
    queue = atoi(argv[2]);
    umem_area = mmap(core::ptr::null_mut(), UMEM_SZ, PROT_READ | PROT_WRITE, MAP_PRIVATE |
    MAP_ANONYMOUS, -1, 0);
    if (umem_area == MAP_FAILED) {
    perror("mmap failed");
    return 1;
    }
    umem_reg.addr = (uintptr_t)umem_area;
    umem_reg.len = UMEM_SZ;
    umem_reg.chunk_size = 2048;
    umem_reg.headroom = 0;
    setsockopt(sock_fd, SOL_XDP, XDP_UMEM_REG, &umem_reg,
    sizeof(umem_reg));
    setsockopt(sock_fd, SOL_XDP, XDP_UMEM_FILL_RING, &num_desc,
    sizeof(num_desc));
    setsockopt(sock_fd, SOL_XDP, XDP_UMEM_COMPLETION_RING, &num_desc,
    sizeof(num_desc));
    setsockopt(sock_fd, SOL_XDP, XDP_RX_RING, &num_desc, sizeof(num_desc));
    sxdp.sxdp_family = AF_XDP;
    sxdp.sxdp_ifindex = ifindex;
    sxdp.sxdp_queue_id = queue;
    sxdp.sxdp_flags = 0;
    if (argc > 3) {
    if (!strcmp(argv[3], "-z")) {
    sxdp.sxdp_flags = XDP_ZEROCOPY;
    } else {
    print_usage(argv[0]);
    return 1;
    }
    }
    while (1) {
    if (bind(sock_fd, (struct sockaddr *)&sxdp, sizeof(sxdp)) == 0)
    break;
    if (errno == EBUSY && retry < 3) {
    retry++;
    sleep(1);
    continue;
    } else {
    perror("bind failed");
    munmap(umem_area, UMEM_SZ);
    close(sock_fd);
    return 1;
    }
    }
    ksft_ready();
    ksft_wait();
// parent program will write a byte to stdin when its ready for this
// helper to exit
//
    close(sock_fd);
    return 0;
    }
