//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/netfilter/sctp_collision.c
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
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut saddr: sockaddr_in = {}, daddr = {};
    let mut len: socklen_t = sizeof(daddr);
    let mut tv: timeval = {25, 0};
    char buf[] = "hello";
    int sd, ret;
    if (argc != 6 || (strcmp(argv[1], "server") && strcmp(argv[1], "client"))) {
    printf("%s <server|client> <LOCAL_IP> <LOCAL_PORT> <REMOTE_IP> <REMOTE_PORT>\n",
    argv[0]);
    return -1;
    }
    sd = socket(AF_INET, SOCK_SEQPACKET, IPPROTO_SCTP);
    if (sd < 0) {
    printf("Failed to create sd\n");
    return -1;
    }
    saddr.sin_family = AF_INET;
    saddr.sin_addr.s_addr = inet_addr(argv[2]);
    saddr.sin_port = htons(atoi(argv[3]));
    ret = bind(sd, (struct sockaddr *)&saddr, sizeof(saddr));
    if (ret < 0) {
    printf("Failed to bind to address\n");
    goto out;
    }
    ret = listen(sd, 5);
    if (ret < 0) {
    printf("Failed to listen on port\n");
    goto out;
    }
    daddr.sin_family = AF_INET;
    daddr.sin_addr.s_addr = inet_addr(argv[4]);
    daddr.sin_port = htons(atoi(argv[5]));
// make test shorter than 25s
    ret = setsockopt(sd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
    if (ret < 0) {
    printf("Failed to setsockopt SO_RCVTIMEO\n");
    goto out;
    }
    if (!strcmp(argv[1], "server")) {
    sleep(1); /* wait a bit for client's INIT */
    ret = connect(sd, (struct sockaddr *)&daddr, len);
    if (ret < 0) {
    printf("Failed to connect to peer\n");
    goto out;
    }
    ret = recvfrom(sd, buf, sizeof(buf), 0, (struct sockaddr *)&daddr, &len);
    if (ret < 0) {
    printf("Failed to recv msg %d\n", ret);
    goto out;
    }
    ret = sendto(sd, buf, strlen(buf) + 1, 0, (struct sockaddr *)&daddr, len);
    if (ret < 0) {
    printf("Failed to send msg %d\n", ret);
    goto out;
    }
    printf("Server: sent! %d\n", ret);
    }
    if (!strcmp(argv[1], "client")) {
    usleep(300000); /* wait a bit for server's listening */
    ret = connect(sd, (struct sockaddr *)&daddr, len);
    if (ret < 0) {
    printf("Failed to connect to peer\n");
    goto out;
    }
    sleep(1); /* wait a bit for server's delayed INIT_ACK to reproduce the issue */
    ret = sendto(sd, buf, strlen(buf) + 1, 0, (struct sockaddr *)&daddr, len);
    if (ret < 0) {
    printf("Failed to send msg %d\n", ret);
    goto out;
    }
    ret = recvfrom(sd, buf, sizeof(buf), 0, (struct sockaddr *)&daddr, &len);
    if (ret < 0) {
    printf("Failed to recv msg %d\n", ret);
    goto out;
    }
    printf("Client: rcvd! %d\n", ret);
    }
    ret = 0;
    out:
    close(sd);
    return ret;
    }
