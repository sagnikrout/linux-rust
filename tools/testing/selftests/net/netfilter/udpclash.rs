//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/netfilter/udpclash.c
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
// Usage: ./udpclash <IP> <PORT>
//
// Emit THREAD_COUNT UDP packets sharing the same saddr:daddr pair.
//
// This mimics DNS resolver libraries that emit A and AAAA requests
// in parallel.
//
// This exercises conntrack clash resolution logic added and later
// refined in
//
// 71d8c47fc653 ("netfilter: conntrack: introduce clash resolution on insertion race")
// ed07d9a021df ("netfilter: nf_conntrack: resolve clash for matching conntracks")
// 6a757c07e51f ("netfilter: conntrack: allow insertion of clashing entries")
//

pub const THREAD_COUNT: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_args {
    pub si_remote: *const sockaddr_in,
    pub sockfd: c_int,
}

    let mut wait: static volatile int = 1;
    static void *thread_main(void *varg)
    {
    const struct sockaddr_in *si_remote;
    const struct thread_args *args = varg;
    static const char msg[] = "foo";
    si_remote = args.si_remote;
    while (wait == 1)
    ;
    if (sendto(args.sockfd, msg, strlen(msg), MSG_NOSIGNAL,
    (struct sockaddr *)si_remote, sizeof(*si_remote)) < 0)
    exit(111);
    return varg;
    }
#[no_mangle]
unsafe extern "C" fn run_test(fd: c_int, si_remote: *const sockaddr_in) -> c_int {
    static int run_test(int fd, const struct sockaddr_in *si_remote)
    {
    struct thread_args thread_args = {
    .si_remote = si_remote,
    .sockfd = fd,
    };
    pthread_t *tid = calloc(THREAD_COUNT, sizeof(pthread_t));
    let mut repl_count: c_uint = 0, timeout = 0;
    int i;
    if (!tid) {
    perror("calloc");
    return 1;
    }
    for (i = 0; i < THREAD_COUNT; i++) {
    let mut err: c_int = pthread_create(&tid[i], core::ptr::null_mut(), &thread_main, &thread_args);
    if (err != 0) {
    perror("pthread_create");
    exit(1);
    }
    }
    wait = 0;
    for (i = 0; i < THREAD_COUNT; i++)
    pthread_join(tid[i], core::ptr::null_mut());
    while (repl_count < THREAD_COUNT) {
    struct sockaddr_in si_repl;
    let mut si_repl_len: socklen_t = sizeof(si_repl);
    char repl[512];
    ssize_t ret;
    ret = recvfrom(fd, repl, sizeof(repl), MSG_NOSIGNAL,
    (struct sockaddr *) &si_repl, &si_repl_len);
    if (ret < 0) {
    if (timeout++ > 5000) {
    fputs("timed out while waiting for reply from thread\n", stderr);
    break;
    }
// give reply time to pass though the stack
    usleep(1000);
    continue;
    }
    if (si_repl_len != sizeof(*si_remote)) {
    fprintf(stderr, "warning: reply has unexpected repl_len %d vs %d\n",
    (int)si_repl_len, (int)sizeof(si_repl));
    } else if (si_remote.sin_addr.s_addr != si_repl.sin_addr.s_addr ||
    si_remote.sin_port != si_repl.sin_port) {
    char a[64], b[64];
    inet_ntop(AF_INET, &si_remote.sin_addr, a, sizeof(a));
    inet_ntop(AF_INET, &si_repl.sin_addr, b, sizeof(b));
    fprintf(stderr, "reply from wrong source: want %s:%d got %s:%d\n",
    a, ntohs(si_remote.sin_port), b, ntohs(si_repl.sin_port));
    }
    repl_count++;
    }
    printf("got %d of %d replies\n", repl_count, THREAD_COUNT);
    free(tid);
    let mut repl_count: return = = THREAD_COUNT ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct sockaddr_in si_local = {
    .sin_family = AF_INET,
    };
    struct sockaddr_in si_remote = {
    .sin_family = AF_INET,
    };
    int fd, ret;
    if (argc < 3) {
    fputs("Usage: send_udp <daddr> <dport>\n", stderr);
    return 1;
    }
    si_remote.sin_port = htons(atoi(argv[2]));
    si_remote.sin_addr.s_addr = inet_addr(argv[1]);
    fd = socket(AF_INET, SOCK_DGRAM|SOCK_CLOEXEC|SOCK_NONBLOCK, IPPROTO_UDP);
    if (fd < 0) {
    perror("socket");
    return 1;
    }
    if (bind(fd, (struct sockaddr *)&si_local, sizeof(si_local)) < 0) {
    perror("bind");
    return 1;
    }
    ret = run_test(fd, &si_remote);
    close(fd);
    return ret;
    }
