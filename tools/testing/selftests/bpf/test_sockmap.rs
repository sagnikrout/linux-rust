//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/test_sockmap.c
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
// Copyright (c) 2017-2018 Covalent IO, Inc. http://covalent.io

    int running;
    static void running_handler(int a);
// randomly selected ports for testing on lo
pub const S1_PORT: c_int = 10000;
pub const S2_PORT: c_int = 10001;

pub const EDATAINTEGRITY: c_int = 2001;
// global sockets
    int s1, s2, c1, c2, p1, p2;
    int test_cnt;
    int passed;
    int failed;
    int map_fd[8];
    struct bpf_map *maps[8];
    struct bpf_program *progs[8];
    struct bpf_link *links[8];
    int txmsg_pass;
    int txmsg_redir;
    int txmsg_drop;
    int txmsg_apply;
    int txmsg_cork;
    int txmsg_start;
    int txmsg_end;
    int txmsg_start_push;
    int txmsg_end_push;
    int txmsg_start_pop;
    int txmsg_pop;
    int txmsg_ingress;
    int txmsg_redir_skb;
    int peek_flag;
    int skb_use_parser;
    int txmsg_omit_skb_parser;
    int verify_push_start;
    int verify_push_len;
    int verify_pop_start;
    int verify_pop_len;
    static const struct option long_options[] = {
    {"help",	no_argument,		core::ptr::null_mut(), 'h' },
    {"cgroup",	required_argument,	core::ptr::null_mut(), 'c' },
    {"rate",	required_argument,	core::ptr::null_mut(), 'r' },
    {"verbose",	optional_argument,	core::ptr::null_mut(), 'v' },
    {"iov_count",	required_argument,	core::ptr::null_mut(), 'i' },
    {"length",	required_argument,	core::ptr::null_mut(), 'l' },
    {"test",	required_argument,	core::ptr::null_mut(), 't' },
    {"data_test",   no_argument,		core::ptr::null_mut(), 'd' },
    {"txmsg",		no_argument,	&txmsg_pass,  1  },
    {"txmsg_redir",		no_argument,	&txmsg_redir, 1  },
    {"txmsg_drop",		no_argument,	&txmsg_drop, 1 },
    {"txmsg_apply",	required_argument,	core::ptr::null_mut(), 'a'},
    {"txmsg_cork",	required_argument,	core::ptr::null_mut(), 'k'},
    {"txmsg_start", required_argument,	core::ptr::null_mut(), 's'},
    {"txmsg_end",	required_argument,	core::ptr::null_mut(), 'e'},
    {"txmsg_start_push", required_argument,	core::ptr::null_mut(), 'p'},
    {"txmsg_end_push",   required_argument,	core::ptr::null_mut(), 'q'},
    {"txmsg_start_pop",  required_argument,	core::ptr::null_mut(), 'w'},
    {"txmsg_pop",	     required_argument,	core::ptr::null_mut(), 'x'},
    {"txmsg_ingress", no_argument,		&txmsg_ingress, 1 },
    {"txmsg_redir_skb", no_argument,	&txmsg_redir_skb, 1 },
    {"peek", no_argument,			&peek_flag, 1 },
    {"txmsg_omit_skb_parser", no_argument,      &txmsg_omit_skb_parser, 1},
    {"whitelist", required_argument,	core::ptr::null_mut(), 'n' },
    {"blacklist", required_argument,	core::ptr::null_mut(), 'b' },
    {0, 0, core::ptr::null_mut(), 0 }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_env {
    pub type: *const c_char,
    pub subtest: *const c_char,
    pub prepend: *const c_char,
    pub test_num: c_int,
    pub subtest_num: c_int,
    pub succ_cnt: c_int,
    pub fail_cnt: c_int,
    pub fail_last: c_int,
}

    struct test_env env;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockmap_options {
    pub verbose: c_int,
    pub base: bool,
    pub sendpage: bool,
    pub data_test: bool,
    pub drop_expected: bool,
    pub check_recved_len: bool,
    pub tx_wait_mem: bool,
    pub iov_count: c_int,
    pub iov_length: c_int,
    pub rate: c_int,
    pub map: *mut c_char,
    pub whitelist: *mut c_char,
    pub blacklist: *mut c_char,
    pub prepend: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _test {
    pub title: *mut c_char,
    pub opt): *mut *mut void (tester)(int cg_fd, struct sockmap_options,
}

#[no_mangle]
unsafe extern "C" fn test_start() {
    static void test_start(void)
    {
    env.subtest_num++;
    }
#[no_mangle]
unsafe extern "C" fn test_fail() {
    static void test_fail(void)
    {
    env.fail_cnt++;
    }
#[no_mangle]
unsafe extern "C" fn test_pass() {
    static void test_pass(void)
    {
    env.succ_cnt++;
    }
#[no_mangle]
unsafe extern "C" fn test_reset() {
    static void test_reset(void)
    {
    txmsg_start = txmsg_end = 0;
    txmsg_start_pop = txmsg_pop = 0;
    txmsg_start_push = txmsg_end_push = 0;
    txmsg_pass = txmsg_drop = txmsg_redir = 0;
    txmsg_apply = txmsg_cork = 0;
    txmsg_ingress = txmsg_redir_skb = 0;
    txmsg_omit_skb_parser = 0;
    skb_use_parser = 0;
    }
#[no_mangle]
unsafe extern "C" fn test_start_subtest(t: *const _test, o: *mut sockmap_options) -> c_int {
    static int test_start_subtest(const struct _test *t, struct sockmap_options *o)
    {
    env.type = o.map;
    env.subtest = t.title;
    env.prepend = o.prepend;
    env.test_num++;
    env.subtest_num = 0;
    env.fail_last = env.fail_cnt;
    test_reset();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_end_subtest() {
    static void test_end_subtest(void)
    {
    let mut error: c_int = env.fail_cnt - env.fail_last;
    let mut type: c_int = strcmp(env.type, BPF_SOCKMAP_FILENAME);
    if (!error)
    test_pass();
    fprintf(stdout, "#%2d/%2d %8s:%s:%s:%s\n",
    env.test_num, env.subtest_num,
    !type ? "sockmap" : "sockhash",
    env.prepend ? : "",
    env.subtest, error ? "FAIL" : "OK");
    }
#[no_mangle]
unsafe extern "C" fn test_print_results() {
    static void test_print_results(void)
    {
    fprintf(stdout, "Pass: %d Fail: %d\n",
    env.succ_cnt, env.fail_cnt);
    }
#[no_mangle]
unsafe extern "C" fn usage(argv[]: *mut c_char) {
    static void usage(char *argv[])
    {
    int i;
    printf(" Usage: %s --cgroup <cgroup_path>\n", argv[0]);
    printf(" options:\n");
    for (i = 0; long_options[i].name != 0; i++) {
    printf(" --%-12s", long_options[i].name);
    if (long_options[i].flag != core::ptr::null_mut())
    printf(" flag (internal value:%d)\n",
// long_options[i].flag);
    else
    printf(" -%c\n", long_options[i].val);
    }
    printf("\n");
    }
#[no_mangle]
unsafe extern "C" fn sockmap_init_sockets(verbose: c_int) -> c_int {
    static int sockmap_init_sockets(int verbose)
    {
    int i, err, one = 1;
    struct sockaddr_in addr;
    int *fds[4] = {&s1, &s2, &c1, &c2};
    s1 = s2 = p1 = p2 = c1 = c2 = 0;
// Init sockets
    for (i = 0; i < 4; i++) {
// fds[i] = socket(AF_INET, SOCK_STREAM, 0);
    if (*fds[i] < 0) {
    perror("socket s1 failed()");
    return errno;
    }
    }
// Allow reuse
    for (i = 0; i < 2; i++) {
    err = setsockopt(*fds[i], SOL_SOCKET, SO_REUSEADDR,
    (char *)&one, sizeof(one));
    if (err) {
    perror("setsockopt failed()");
    return errno;
    }
    }
// Non-blocking sockets
    for (i = 0; i < 2; i++) {
    err = ioctl(*fds[i], FIONBIO, (char *)&one);
    if (err < 0) {
    perror("ioctl s1 failed()");
    return errno;
    }
    }
// Bind server sockets
    memset(&addr, 0, sizeof(struct sockaddr_in));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");
    addr.sin_port = htons(S1_PORT);
    err = bind(s1, (struct sockaddr *)&addr, sizeof(addr));
    if (err < 0) {
    perror("bind s1 failed()");
    return errno;
    }
    addr.sin_port = htons(S2_PORT);
    err = bind(s2, (struct sockaddr *)&addr, sizeof(addr));
    if (err < 0) {
    perror("bind s2 failed()");
    return errno;
    }
// Listen server sockets
    addr.sin_port = htons(S1_PORT);
    err = listen(s1, 32);
    if (err < 0) {
    perror("listen s1 failed()");
    return errno;
    }
    addr.sin_port = htons(S2_PORT);
    err = listen(s2, 32);
    if (err < 0) {
    perror("listen s1 failed()");
    return errno;
    }
// Initiate Connect
    addr.sin_port = htons(S1_PORT);
    err = connect(c1, (struct sockaddr *)&addr, sizeof(addr));
    if (err < 0 && errno != EINPROGRESS) {
    perror("connect c1 failed()");
    return errno;
    }
    addr.sin_port = htons(S2_PORT);
    err = connect(c2, (struct sockaddr *)&addr, sizeof(addr));
    if (err < 0 && errno != EINPROGRESS) {
    perror("connect c2 failed()");
    return errno;
    } else if (err < 0) {
    err = 0;
    }
// Accept Connecrtions
    p1 = accept(s1, core::ptr::null_mut(), core::ptr::null_mut());
    if (p1 < 0) {
    perror("accept s1 failed()");
    return errno;
    }
    p2 = accept(s2, core::ptr::null_mut(), core::ptr::null_mut());
    if (p2 < 0) {
    perror("accept s1 failed()");
    return errno;
    }
    if (verbose > 1) {
    printf("connected sockets: c1 <. p1, c2 <. p2\n");
    printf("cgroups binding: c1(%i) <. s1(%i) - - - c2(%i) <. s2(%i)\n",
    c1, s1, c2, s2);
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_stats {
    pub bytes_sent: usize,
    pub bytes_recvd: usize,
    pub start: timespec,
    pub end: timespec,
}

    static int msg_loop_sendpage(int fd, int iov_length, int cnt,
    struct msg_stats *s,
    struct sockmap_options *opt)
    {
    let mut drop: bool = opt.drop_expected;
    let mut k: c_uchar = 0;
    int i, j, fp;
    FILE *file;
    file = tmpfile();
    if (!file) {
    perror("create file for sendpage");
    return 1;
    }
    for (i = 0; i < cnt; i++, k = 0) {
    for (j = 0; j < iov_length; j++, k++)
    fwrite(&k, sizeof(char), 1, file);
    }
    fflush(file);
    fseek(file, 0, SEEK_SET);
    fp = fileno(file);
    clock_gettime(CLOCK_MONOTONIC, &s.start);
    for (i = 0; i < cnt; i++) {
    int sent;
    errno = 0;
    sent = sendfile(fd, fp, core::ptr::null_mut(), iov_length);
    if (!drop && sent < 0) {
    perror("sendpage loop error");
    fclose(file);
    return sent;
    } else if (drop && sent >= 0) {
    printf("sendpage loop error expected: %i errno %i\n",
    sent, errno);
    fclose(file);
    return -EIO;
    }
    if (sent > 0)
    s.bytes_sent += sent;
    }
    clock_gettime(CLOCK_MONOTONIC, &s.end);
    fclose(file);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msg_free_iov(msg: *mut msghdr) {
    static void msg_free_iov(struct msghdr *msg)
    {
    int i;
    for (i = 0; i < msg.msg_iovlen; i++)
    free(msg.msg_iov[i].iov_base);
    free(msg.msg_iov);
    msg.msg_iov = core::ptr::null_mut();
    msg.msg_iovlen = 0;
    }
    static int msg_alloc_iov(struct msghdr *msg,
    int iov_count, int iov_length,
    bool data, bool xmit)
    {
    let mut k: c_uchar = 0;
    struct iovec *iov;
    int i;
    iov = calloc(iov_count, sizeof(struct iovec));
    if (!iov)
    return errno;
    for (i = 0; i < iov_count; i++) {
    unsigned char *d = calloc(iov_length, sizeof(char));
    if (!d) {
    fprintf(stderr, "iov_count %i/%i OOM\n", i, iov_count);
    goto unwind_iov;
    }
    iov[i].iov_base = d;
    iov[i].iov_len = iov_length;
    if (data && xmit) {
    int j;
    for (j = 0; j < iov_length; j++)
    d[j] = k++;
    }
    }
    msg.msg_iov = iov;
    msg.msg_iovlen = iov_count;
    return 0;
    unwind_iov:
    for (i--; i >= 0 ; i--)
    free(iov[i].iov_base);
    free(iov);
    return -ENOMEM;
    }
// In push or pop test, we need to do some calculations for msg_verify_data
#[no_mangle]
unsafe extern "C" fn msg_verify_date_prep() {
    static void msg_verify_date_prep(void)
    {
    let mut push_range_end: c_int = txmsg_start_push + txmsg_end_push - 1;
    let mut pop_range_end: c_int = txmsg_start_pop + txmsg_pop - 1;
    if (txmsg_end_push && txmsg_pop &&
    txmsg_start_push <= pop_range_end && txmsg_start_pop <= push_range_end) {
// The push range and the pop range overlap
    int overlap_len;
    verify_push_start = txmsg_start_push;
    verify_pop_start = txmsg_start_pop;
    if (txmsg_start_push < txmsg_start_pop)
    overlap_len = min(push_range_end - txmsg_start_pop + 1, txmsg_pop);
    else
    overlap_len = min(pop_range_end - txmsg_start_push + 1, txmsg_end_push);
    verify_push_len = max(txmsg_end_push - overlap_len, 0);
    verify_pop_len = max(txmsg_pop - overlap_len, 0);
    } else {
// Otherwise
    verify_push_start = txmsg_start_push;
    verify_pop_start = txmsg_start_pop;
    verify_push_len = txmsg_end_push;
    verify_pop_len = txmsg_pop;
    }
    }
    static int msg_verify_data(struct msghdr *msg, int size, int chunk_sz,
    unsigned char *k_p, int *bytes_cnt_p,
    int *check_cnt_p, int *push_p)
    {
    let mut bytes_cnt: c_int = *bytes_cnt_p, check_cnt = *check_cnt_p, push = *push_p;
    let mut k: c_uchar = *k_p;
    int i, j;
    for (i = 0, j = 0; i < msg.msg_iovlen && size; i++, j = 0) {
    unsigned char *d = msg.msg_iov[i].iov_base;
    for (; j < msg.msg_iov[i].iov_len && size; j++) {
    if (push > 0 &&
    check_cnt == verify_push_start + verify_push_len - push) {
    int skipped;
    revisit_push:
    skipped = push;
    if (j + push >= msg.msg_iov[i].iov_len)
    skipped = msg.msg_iov[i].iov_len - j;
    push -= skipped;
    size -= skipped;
    j += skipped - 1;
    check_cnt += skipped;
    continue;
    }
    if (verify_pop_len > 0 && check_cnt == verify_pop_start) {
    bytes_cnt += verify_pop_len;
    check_cnt += verify_pop_len;
    k += verify_pop_len;
    if (bytes_cnt == chunk_sz) {
    k = 0;
    bytes_cnt = 0;
    check_cnt = 0;
    push = verify_push_len;
    }
    if (push > 0 &&
    check_cnt == verify_push_start + verify_push_len - push)
    goto revisit_push;
    }
    if (d[j] != k++) {
    fprintf(stderr,
    "detected data corruption @iov[%i]:%i %02x != %02x, %02x ?= %02x\n",
    i, j, d[j], k - 1, d[j+1], k);
    return -EDATAINTEGRITY;
    }
    bytes_cnt++;
    check_cnt++;
    if (bytes_cnt == chunk_sz) {
    k = 0;
    bytes_cnt = 0;
    check_cnt = 0;
    push = verify_push_len;
    }
    size--;
    }
    }
// k_p = k;
// bytes_cnt_p = bytes_cnt;
// check_cnt_p = check_cnt;
// push_p = push;
    return 0;
    }
    static int msg_loop(int fd, int iov_count, int iov_length, int cnt,
    struct msg_stats *s, bool tx,
    struct sockmap_options *opt)
    {
    let mut msg: msghdr = {0}, msg_peek = {0};
    int err, i, flags = MSG_NOSIGNAL;
    let mut drop: bool = opt.drop_expected;
    let mut data: bool = opt.data_test;
    let mut iov_alloc_length: c_int = iov_length;
    if (!tx && opt.check_recved_len)
    iov_alloc_length *= 2;
    err = msg_alloc_iov(&msg, iov_count, iov_alloc_length, data, tx);
    if (err)
    goto out_errno;
    if (peek_flag) {
    err = msg_alloc_iov(&msg_peek, iov_count, iov_length, data, tx);
    if (err)
    goto out_errno;
    }
    if (tx) {
    clock_gettime(CLOCK_MONOTONIC, &s.start);
    for (i = 0; i < cnt; i++) {
    int sent;
    errno = 0;
    sent = sendmsg(fd, &msg, flags);
    if (!drop && sent < 0) {
    if (opt.tx_wait_mem && errno == EACCES) {
    errno = 0;
    goto out_errno;
    }
    perror("sendmsg loop error");
    goto out_errno;
    } else if (drop && sent >= 0) {
    fprintf(stderr,
    "sendmsg loop error expected: %i errno %i\n",
    sent, errno);
    errno = -EIO;
    goto out_errno;
    }
    if (sent > 0)
    s.bytes_sent += sent;
    }
    clock_gettime(CLOCK_MONOTONIC, &s.end);
    } else {
    float total_bytes, txmsg_pop_total, txmsg_push_total;
    int slct, recvp = 0, recv, max_fd = fd;
    let mut fd_flags: c_int = O_NONBLOCK;
    struct timeval timeout;
    let mut k: c_uchar = 0;
    let mut bytes_cnt: c_int = 0;
    let mut check_cnt: c_int = 0;
    let mut push: c_int = 0;
    fd_set w;
    fcntl(fd, fd_flags);
// Account for pop bytes noting each iteration of apply will
// call msg_pop_data helper so we need to account for this
// by calculating the number of apply iterations. Note user
// of the tool can create cases where no data is sent by
// manipulating pop/push/pull/etc. For example txmsg_apply 1
// with txmsg_pop 1 will try to apply 1B at a time but each
// iteration will then pop 1B so no data will ever be sent.
// This is really only useful for testing edge cases in code
// paths.
//
    total_bytes = (float)iov_length * (float)cnt;
    if (!opt.sendpage)
    total_bytes *= (float)iov_count;
    if (txmsg_apply) {
    txmsg_push_total = txmsg_end_push * (total_bytes / txmsg_apply);
    txmsg_pop_total = txmsg_pop * (total_bytes / txmsg_apply);
    } else {
    txmsg_push_total = txmsg_end_push * cnt;
    txmsg_pop_total = txmsg_pop * cnt;
    }
    total_bytes += txmsg_push_total;
    total_bytes -= txmsg_pop_total;
    if (data) {
    msg_verify_date_prep();
    push = verify_push_len;
    }
    err = clock_gettime(CLOCK_MONOTONIC, &s.start);
    if (err < 0)
    perror("recv start time");
    while (s.bytes_recvd < total_bytes) {
    if (txmsg_cork) {
    timeout.tv_sec = 0;
    timeout.tv_usec = 300000;
    } else {
    timeout.tv_sec = 3;
    timeout.tv_usec = 0;
    }
// FD sets
    FD_ZERO(&w);
    FD_SET(fd, &w);
    slct = select(max_fd + 1, &w, core::ptr::null_mut(), core::ptr::null_mut(), &timeout);
    if (slct == -1) {
    perror("select()");
    clock_gettime(CLOCK_MONOTONIC, &s.end);
    goto out_errno;
    } else if (!slct) {
    if (opt.verbose)
    fprintf(stderr, "unexpected timeout: recved %zu/%f pop_total %f\n", s.bytes_recvd, total_bytes, txmsg_pop_total);
    errno = -EIO;
    clock_gettime(CLOCK_MONOTONIC, &s.end);
    goto out_errno;
    }
    if (opt.tx_wait_mem) {
    FD_ZERO(&w);
    FD_SET(fd, &w);
    slct = select(max_fd + 1, core::ptr::null_mut(), core::ptr::null_mut(), &w, &timeout);
    errno = 0;
    close(fd);
    goto out_errno;
    }
    errno = 0;
    if (peek_flag) {
    flags |= MSG_PEEK;
    recvp = recvmsg(fd, &msg_peek, flags);
    if (recvp < 0) {
    if (errno != EWOULDBLOCK) {
    clock_gettime(CLOCK_MONOTONIC, &s.end);
    goto out_errno;
    }
    }
    flags = 0;
    }
    recv = recvmsg(fd, &msg, flags);
    if (recv < 0) {
    if (errno != EWOULDBLOCK) {
    clock_gettime(CLOCK_MONOTONIC, &s.end);
    perror("recv failed()");
    goto out_errno;
    }
    }
    if (recv > 0)
    s.bytes_recvd += recv;
    if (opt.check_recved_len && s.bytes_recvd > total_bytes) {
    errno = EMSGSIZE;
    fprintf(stderr, "recv failed(), bytes_recvd:%zd, total_bytes:%f\n",
    s.bytes_recvd, total_bytes);
    goto out_errno;
    }
    if (data) {
    int chunk_sz = opt.sendpage ?
    iov_length :
    iov_length * iov_count;
    errno = msg_verify_data(&msg, recv, chunk_sz, &k, &bytes_cnt,
    &check_cnt, &push);
    if (errno) {
    perror("data verify msg failed");
    goto out_errno;
    }
    if (recvp) {
    errno = msg_verify_data(&msg_peek,
    recvp,
    chunk_sz,
    &k,
    &bytes_cnt,
    &check_cnt,
    &push);
    if (errno) {
    perror("data verify msg_peek failed");
    goto out_errno;
    }
    }
    }
    }
    clock_gettime(CLOCK_MONOTONIC, &s.end);
    }
    msg_free_iov(&msg);
    msg_free_iov(&msg_peek);
    return err;
    out_errno:
    msg_free_iov(&msg);
    msg_free_iov(&msg_peek);
    return errno;
    }
    let mut giga: static float = 1000000000;
#[no_mangle]
pub unsafe extern "C" fn sentBps(s: msg_stats) -> float {
    static inline float sentBps(struct msg_stats s)
    {
    return s.bytes_sent / (s.end.tv_sec - s.start.tv_sec);
    }
#[no_mangle]
pub unsafe extern "C" fn recvdBps(s: msg_stats) -> float {
    static inline float recvdBps(struct msg_stats s)
    {
    return s.bytes_recvd / (s.end.tv_sec - s.start.tv_sec);
    }
#[no_mangle]
unsafe extern "C" fn sendmsg_test(opt: *mut sockmap_options) -> c_int {
    static int sendmsg_test(struct sockmap_options *opt)
    {
    let mut sent_Bps: float = 0, recvd_Bps = 0;
    int rx_fd, txpid, rxpid, err = 0;
    let mut s: msg_stats = {0};
    let mut iov_count: c_int = opt.iov_count;
    let mut iov_buf: c_int = opt.iov_length;
    int rx_status, tx_status;
    let mut cnt: c_int = opt.rate;
    errno = 0;
    if (opt.base)
    rx_fd = p1;
    else
    rx_fd = p2;
    if (opt.tx_wait_mem) {
    struct timeval timeout;
    let mut rxtx_buf_len: c_int = 1024;
    timeout.tv_sec = 3;
    timeout.tv_usec = 0;
    err = setsockopt(c2, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(struct timeval));
    err |= setsockopt(c2, SOL_SOCKET, SO_SNDBUFFORCE, &rxtx_buf_len, sizeof(int));
    err |= setsockopt(p2, SOL_SOCKET, SO_RCVBUFFORCE, &rxtx_buf_len, sizeof(int));
    if (err) {
    perror("setsockopt failed()");
    return errno;
    }
    }
    rxpid = fork();
    if (rxpid == 0) {
    if (opt.drop_expected)
    _exit(0);
    if (!iov_buf) /* zero bytes sent case */
    _exit(0);
    if (opt.sendpage)
    iov_count = 1;
    err = msg_loop(rx_fd, iov_count, iov_buf,
    cnt, &s, false, opt);
    if (opt.verbose > 1)
    fprintf(stderr,
    "msg_loop_rx: iov_count %i iov_buf %i cnt %i err %i\n",
    iov_count, iov_buf, cnt, err);
    if (s.end.tv_sec - s.start.tv_sec) {
    sent_Bps = sentBps(s);
    recvd_Bps = recvdBps(s);
    }
    if (opt.verbose > 1)
    fprintf(stdout,
    "rx_sendmsg: TX: %zuB %fB/s %fGB/s RX: %zuB %fB/s %fGB/s %s\n",
    s.bytes_sent, sent_Bps, sent_Bps/giga,
    s.bytes_recvd, recvd_Bps, recvd_Bps/giga,
    peek_flag ? "(peek_msg)" : "");
    if (err && err != -EDATAINTEGRITY && txmsg_cork)
    err = 0;
    exit(err ? 1 : 0);
    } else if (rxpid == -1) {
    perror("msg_loop_rx");
    return errno;
    }
    if (opt.tx_wait_mem)
    close(c2);
    txpid = fork();
    if (txpid == 0) {
    if (opt.sendpage)
    err = msg_loop_sendpage(c1, iov_buf, cnt, &s, opt);
    else
    err = msg_loop(c1, iov_count, iov_buf,
    cnt, &s, true, opt);
    if (err)
    fprintf(stderr,
    "msg_loop_tx: iov_count %i iov_buf %i cnt %i err %i\n",
    iov_count, iov_buf, cnt, err);
    if (s.end.tv_sec - s.start.tv_sec) {
    sent_Bps = sentBps(s);
    recvd_Bps = recvdBps(s);
    }
    if (opt.verbose > 1)
    fprintf(stdout,
    "tx_sendmsg: TX: %zuB %fB/s %f GB/s RX: %zuB %fB/s %fGB/s\n",
    s.bytes_sent, sent_Bps, sent_Bps/giga,
    s.bytes_recvd, recvd_Bps, recvd_Bps/giga);
    exit(err ? 1 : 0);
    } else if (txpid == -1) {
    perror("msg_loop_tx");
    return errno;
    }
    assert(waitpid(rxpid, &rx_status, 0) == rxpid);
    assert(waitpid(txpid, &tx_status, 0) == txpid);
    if (WIFEXITED(rx_status)) {
    err = WEXITSTATUS(rx_status);
    if (err) {
    fprintf(stderr, "rx thread exited with err %d.\n", err);
    goto out;
    }
    }
    if (WIFEXITED(tx_status)) {
    err = WEXITSTATUS(tx_status);
    if (err)
    fprintf(stderr, "tx thread exited with err %d.\n", err);
    }
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn forever_ping_pong(rate: c_int, opt: *mut sockmap_options) -> c_int {
    static int forever_ping_pong(int rate, struct sockmap_options *opt)
    {
    struct timeval timeout;
    char buf[1024] = {0};
    int sc;
    timeout.tv_sec = 10;
    timeout.tv_usec = 0;
// Ping/Pong data from client to server
    sc = send(c1, buf, sizeof(buf), 0);
    if (sc < 0) {
    perror("send failed()");
    return sc;
    }
    do {
    int s, rc, i, max_fd = p2;
    fd_set w;
// FD sets
    FD_ZERO(&w);
    FD_SET(c1, &w);
    FD_SET(c2, &w);
    FD_SET(p1, &w);
    FD_SET(p2, &w);
    s = select(max_fd + 1, &w, core::ptr::null_mut(), core::ptr::null_mut(), &timeout);
    if (s == -1) {
    perror("select()");
    break;
    } else if (!s) {
    fprintf(stderr, "unexpected timeout\n");
    break;
    }
    for (i = 0; i <= max_fd && s > 0; ++i) {
    if (!FD_ISSET(i, &w))
    continue;
    s--;
    rc = recv(i, buf, sizeof(buf), 0);
    if (rc < 0) {
    if (errno != EWOULDBLOCK) {
    perror("recv failed()");
    return rc;
    }
    }
    if (rc == 0) {
    close(i);
    break;
    }
    sc = send(i, buf, rc, 0);
    if (sc < 0) {
    perror("send failed()");
    return sc;
    }
    }
    if (rate)
    sleep(rate);
    if (opt.verbose) {
    printf(".");
    fflush(stdout);
    }
    } while (running);
    return 0;
    }
    enum {
    SELFTESTS,
    PING_PONG,
    SENDMSG,
    BASE,
    BASE_SENDPAGE,
    SENDPAGE,
    };
#[no_mangle]
unsafe extern "C" fn run_options(options: *mut sockmap_options, cg_fd: c_int, test: c_int) -> c_int {
    static int run_options(struct sockmap_options *options, int cg_fd,  int test)
    {
    int i, key, next_key, err, zero = 0;
    struct bpf_program *tx_prog;
// If base test skip BPF setup
    if (test == BASE || test == BASE_SENDPAGE)
    goto run;
// Attach programs to sockmap
    if (!txmsg_omit_skb_parser) {
    links[0] = bpf_program__attach_sockmap(progs[0], map_fd[0]);
    if (!links[0]) {
    fprintf(stderr,
    "ERROR: bpf_program__attach_sockmap (sockmap %i.%i): (%s)\n",
    bpf_program__fd(progs[0]), map_fd[0], strerror(errno));
    return -1;
    }
    }
    links[1] = bpf_program__attach_sockmap(progs[1], map_fd[0]);
    if (!links[1]) {
    fprintf(stderr, "ERROR: bpf_program__attach_sockmap (sockmap): (%s)\n",
    strerror(errno));
    return -1;
    }
// Attach to cgroups
    err = bpf_prog_attach(bpf_program__fd(progs[2]), cg_fd, BPF_CGROUP_SOCK_OPS, 0);
    if (err) {
    fprintf(stderr, "ERROR: bpf_prog_attach (groups): %d (%s)\n",
    err, strerror(errno));
    return err;
    }
    run:
    err = sockmap_init_sockets(options.verbose);
    if (err) {
    fprintf(stderr, "ERROR: test socket failed: %d\n", err);
    goto out;
    }
// Attach txmsg program to sockmap
    if (txmsg_pass)
    tx_prog = progs[3];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: txmsg_redir) -> else {
    else if (txmsg_redir)
    tx_prog = progs[4];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: txmsg_apply) -> else {
    else if (txmsg_apply)
    tx_prog = progs[5];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: txmsg_cork) -> else {
    else if (txmsg_cork)
    tx_prog = progs[6];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: txmsg_drop) -> else {
    else if (txmsg_drop)
    tx_prog = progs[7];
    else
    tx_prog = core::ptr::null_mut();
    if (tx_prog) {
    int redir_fd;
    links[4] = bpf_program__attach_sockmap(tx_prog, map_fd[1]);
    if (!links[4]) {
    fprintf(stderr,
    "ERROR: bpf_program__attach_sockmap (txmsg): (%s)\n",
    strerror(errno));
    err = -1;
    goto out;
    }
    i = 0;
    err = bpf_map_update_elem(map_fd[1], &i, &c1, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (txmsg):  %d (%s\n",
    err, strerror(errno));
    goto out;
    }
    if (txmsg_redir)
    redir_fd = c2;
    else
    redir_fd = c1;
    err = bpf_map_update_elem(map_fd[2], &i, &redir_fd, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (txmsg):  %d (%s\n",
    err, strerror(errno));
    goto out;
    }
    if (txmsg_apply) {
    err = bpf_map_update_elem(map_fd[3],
    &i, &txmsg_apply, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (apply_bytes):  %d (%s\n",
    err, strerror(errno));
    goto out;
    }
    }
    if (txmsg_cork) {
    err = bpf_map_update_elem(map_fd[4],
    &i, &txmsg_cork, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (cork_bytes):  %d (%s\n",
    err, strerror(errno));
    goto out;
    }
    }
    if (txmsg_start) {
    err = bpf_map_update_elem(map_fd[5],
    &i, &txmsg_start, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (txmsg_start):  %d (%s)\n",
    err, strerror(errno));
    goto out;
    }
    }
    if (txmsg_end) {
    i = 1;
    err = bpf_map_update_elem(map_fd[5],
    &i, &txmsg_end, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (txmsg_end):  %d (%s)\n",
    err, strerror(errno));
    goto out;
    }
    }
    if (txmsg_start_push) {
    i = 2;
    err = bpf_map_update_elem(map_fd[5],
    &i, &txmsg_start_push, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (txmsg_start_push):  %d (%s)\n",
    err, strerror(errno));
    goto out;
    }
    }
    if (txmsg_end_push) {
    i = 3;
    err = bpf_map_update_elem(map_fd[5],
    &i, &txmsg_end_push, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem %i@%i (txmsg_end_push):  %d (%s)\n",
    txmsg_end_push, i, err, strerror(errno));
    goto out;
    }
    }
    if (txmsg_start_pop) {
    i = 4;
    err = bpf_map_update_elem(map_fd[5],
    &i, &txmsg_start_pop, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem %i@%i (txmsg_start_pop):  %d (%s)\n",
    txmsg_start_pop, i, err, strerror(errno));
    goto out;
    }
    } else {
    i = 4;
    bpf_map_update_elem(map_fd[5],
    &i, &txmsg_start_pop, BPF_ANY);
    }
    if (txmsg_pop) {
    i = 5;
    err = bpf_map_update_elem(map_fd[5],
    &i, &txmsg_pop, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem %i@%i (txmsg_pop):  %d (%s)\n",
    txmsg_pop, i, err, strerror(errno));
    goto out;
    }
    } else {
    i = 5;
    bpf_map_update_elem(map_fd[5],
    &i, &txmsg_pop, BPF_ANY);
    }
    if (txmsg_ingress) {
    let mut in: c_int = BPF_F_INGRESS;
    i = 0;
    err = bpf_map_update_elem(map_fd[6], &i, &in, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (txmsg_ingress): %d (%s)\n",
    err, strerror(errno));
    }
    i = 1;
    err = bpf_map_update_elem(map_fd[1], &i, &p1, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (p1 txmsg): %d (%s)\n",
    err, strerror(errno));
    }
    err = bpf_map_update_elem(map_fd[2], &i, &p1, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (p1 redir): %d (%s)\n",
    err, strerror(errno));
    }
    i = 2;
    err = bpf_map_update_elem(map_fd[2], &i, &p2, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (p2 txmsg): %d (%s)\n",
    err, strerror(errno));
    }
    }
    if (txmsg_redir_skb) {
    int skb_fd = (test == SENDMSG || test == SENDPAGE) ?
    p2 : p1;
    let mut ingress: c_int = BPF_F_INGRESS;
    i = 0;
    err = bpf_map_update_elem(map_fd[7],
    &i, &ingress, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (txmsg_ingress): %d (%s)\n",
    err, strerror(errno));
    }
    i = 3;
    err = bpf_map_update_elem(map_fd[0], &i, &skb_fd, BPF_ANY);
    if (err) {
    fprintf(stderr,
    "ERROR: bpf_map_update_elem (c1 sockmap): %d (%s)\n",
    err, strerror(errno));
    }
    }
    }
    if (skb_use_parser) {
    i = 2;
    err = bpf_map_update_elem(map_fd[7], &i, &skb_use_parser, BPF_ANY);
    }
    if (txmsg_drop)
    options.drop_expected = true;
    if (test == PING_PONG)
    err = forever_ping_pong(options.rate, options);
#[no_mangle]
pub unsafe extern "C" fn if(SENDMSG: test ==) -> else {
    options.base = false;
    options.sendpage = false;
    err = sendmsg_test(options);
    } else if (test == SENDPAGE) {
    options.base = false;
    options.sendpage = true;
    err = sendmsg_test(options);
    } else if (test == BASE) {
    options.base = true;
    options.sendpage = false;
    err = sendmsg_test(options);
    } else if (test == BASE_SENDPAGE) {
    options.base = true;
    options.sendpage = true;
    err = sendmsg_test(options);
    } else
    fprintf(stderr, "unknown test\n");
    out:
// Detach and zero all the maps
    bpf_prog_detach2(bpf_program__fd(progs[2]), cg_fd, BPF_CGROUP_SOCK_OPS);
    for (i = 0; i < ARRAY_SIZE(links); i++) {
    if (links[i])
    bpf_link__detach(links[i]);
    }
    for (i = 0; i < ARRAY_SIZE(map_fd); i++) {
    key = next_key = 0;
    bpf_map_update_elem(map_fd[i], &key, &zero, BPF_ANY);
    while (bpf_map_get_next_key(map_fd[i], &key, &next_key) == 0) {
    bpf_map_update_elem(map_fd[i], &key, &zero, BPF_ANY);
    key = next_key;
    }
    }
    close(s1);
    close(s2);
    close(p1);
    close(p2);
    close(c1);
    close(c2);
    return err;
    }
    static char *test_to_str(int test)
    {
    switch (test) {
    case SENDMSG:
    return "sendmsg";
    case SENDPAGE:
    return "sendpage";
    }
    return "unknown";
    }
#[no_mangle]
unsafe extern "C" fn append_str(dst: *mut c_char, src: *const c_char, dst_cap: usize) {
    static void append_str(char *dst, const char *src, size_t dst_cap)
    {
    let mut avail: usize = dst_cap - strlen(dst);
    if (avail <= 1) /* just zero byte could be written */
    return;
    strncat(dst, src, avail - 1); /* strncat() adds + 1 for zero byte */
    }
pub const OPTSTRING: c_int = 60;
#[no_mangle]
unsafe extern "C" fn test_options(options: *mut c_char) {
    static void test_options(char *options)
    {
    char tstr[OPTSTRING];
    memset(options, 0, OPTSTRING);
    if (txmsg_pass)
    append_str(options, "pass,", OPTSTRING);
    if (txmsg_redir)
    append_str(options, "redir,", OPTSTRING);
    if (txmsg_drop)
    append_str(options, "drop,", OPTSTRING);
    if (txmsg_apply) {
    snprintf(tstr, OPTSTRING, "apply %d,", txmsg_apply);
    append_str(options, tstr, OPTSTRING);
    }
    if (txmsg_cork) {
    snprintf(tstr, OPTSTRING, "cork %d,", txmsg_cork);
    append_str(options, tstr, OPTSTRING);
    }
    if (txmsg_start) {
    snprintf(tstr, OPTSTRING, "start %d,", txmsg_start);
    append_str(options, tstr, OPTSTRING);
    }
    if (txmsg_end) {
    snprintf(tstr, OPTSTRING, "end %d,", txmsg_end);
    append_str(options, tstr, OPTSTRING);
    }
    if (txmsg_start_pop) {
    snprintf(tstr, OPTSTRING, "pop (%d,%d),",
    txmsg_start_pop, txmsg_start_pop + txmsg_pop);
    append_str(options, tstr, OPTSTRING);
    }
    if (txmsg_ingress)
    append_str(options, "ingress,", OPTSTRING);
    if (txmsg_redir_skb)
    append_str(options, "redir_skb,", OPTSTRING);
    if (peek_flag)
    append_str(options, "peek,", OPTSTRING);
    }
#[no_mangle]
unsafe extern "C" fn __test_exec(cgrp: c_int, test: c_int, opt: *mut sockmap_options) -> c_int {
    static int __test_exec(int cgrp, int test, struct sockmap_options *opt)
    {
    char *options = calloc(OPTSTRING, sizeof(char));
    int err;
    if (test == SENDPAGE)
    opt.sendpage = true;
    else
    opt.sendpage = false;
    if (txmsg_drop)
    opt.drop_expected = true;
    else
    opt.drop_expected = false;
    test_options(options);
    if (opt.verbose) {
    fprintf(stdout,
    " [TEST %i]: (%i, %i, %i, %s, %s): ",
    test_cnt, opt.rate, opt.iov_count, opt.iov_length,
    test_to_str(test), options);
    fflush(stdout);
    }
    err = run_options(opt, cgrp, test);
    if (opt.verbose)
    fprintf(stdout, " %s\n", !err ? "PASS" : "FAILED");
    test_cnt++;
    !err ? passed++ : failed++;
    free(options);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_exec(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_exec(int cgrp, struct sockmap_options *opt)
    {
    let mut type: c_int = strcmp(opt.map, BPF_SOCKMAP_FILENAME);
    int err;
    if (type == 0) {
    test_start();
    err = __test_exec(cgrp, SENDMSG, opt);
    if (err)
    test_fail();
    } else {
    test_start();
    err = __test_exec(cgrp, SENDPAGE, opt);
    if (err)
    test_fail();
    }
    }
#[no_mangle]
unsafe extern "C" fn test_send_one(opt: *mut sockmap_options, cgrp: c_int) {
    static void test_send_one(struct sockmap_options *opt, int cgrp)
    {
    opt.iov_length = 1;
    opt.iov_count = 1;
    opt.rate = 1;
    test_exec(cgrp, opt);
    opt.iov_length = 1;
    opt.iov_count = 1024;
    opt.rate = 1;
    test_exec(cgrp, opt);
    opt.iov_length = 1024;
    opt.iov_count = 1;
    opt.rate = 1;
    test_exec(cgrp, opt);
    }
#[no_mangle]
unsafe extern "C" fn test_send_many(opt: *mut sockmap_options, cgrp: c_int) {
    static void test_send_many(struct sockmap_options *opt, int cgrp)
    {
    opt.iov_length = 3;
    opt.iov_count = 1;
    opt.rate = 512;
    test_exec(cgrp, opt);
    opt.rate = 100;
    opt.iov_count = 1;
    opt.iov_length = 5;
    test_exec(cgrp, opt);
    }
#[no_mangle]
unsafe extern "C" fn test_send_large(opt: *mut sockmap_options, cgrp: c_int) {
    static void test_send_large(struct sockmap_options *opt, int cgrp)
    {
    opt.iov_length = 8192;
    opt.iov_count = 32;
    opt.rate = 2;
    test_exec(cgrp, opt);
    }
#[no_mangle]
unsafe extern "C" fn test_send(opt: *mut sockmap_options, cgrp: c_int) {
    static void test_send(struct sockmap_options *opt, int cgrp)
    {
    test_send_one(opt, cgrp);
    test_send_many(opt, cgrp);
    test_send_large(opt, cgrp);
    sched_yield();
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_pass(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_pass(int cgrp, struct sockmap_options *opt)
    {
// Test small and large iov_count values with pass/redir/apply/cork
    txmsg_pass = 1;
    test_send(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_redir(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_redir(int cgrp, struct sockmap_options *opt)
    {
    txmsg_redir = 1;
    test_send(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_redir_wait_sndmem(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_redir_wait_sndmem(int cgrp, struct sockmap_options *opt)
    {
    opt.tx_wait_mem = true;
    txmsg_redir = 1;
    test_send_large(opt, cgrp);
    txmsg_redir = 1;
    txmsg_apply = 4097;
    test_send_large(opt, cgrp);
    opt.tx_wait_mem = false;
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_drop(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_drop(int cgrp, struct sockmap_options *opt)
    {
    txmsg_drop = 1;
    test_send(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_ingress_redir(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_ingress_redir(int cgrp, struct sockmap_options *opt)
    {
    txmsg_pass = txmsg_drop = 0;
    txmsg_ingress = txmsg_redir = 1;
    test_send(opt, cgrp);
    }
// Test cork with hung data. This tests poor usage patterns where
// cork can leave data on the ring if user program is buggy and
// doesn't flush them somehow. They do take some time however
// because they wait for a timeout. Test pass, redir and cork with
// apply logic. Use cork size of 4097 with send_large to avoid
// aligning cork size with send size.
//
#[no_mangle]
unsafe extern "C" fn test_txmsg_cork_hangs(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_cork_hangs(int cgrp, struct sockmap_options *opt)
    {
    txmsg_pass = 1;
    txmsg_redir = 0;
    txmsg_cork = 4097;
    txmsg_apply = 4097;
    test_send_large(opt, cgrp);
    txmsg_pass = 0;
    txmsg_redir = 1;
    txmsg_apply = 0;
    txmsg_cork = 4097;
    test_send_large(opt, cgrp);
    txmsg_pass = 0;
    txmsg_redir = 1;
    txmsg_apply = 4097;
    txmsg_cork = 4097;
    test_send_large(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_pull(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_pull(int cgrp, struct sockmap_options *opt)
    {
// Test basic start/end
    txmsg_pass = 1;
    txmsg_start = 1;
    txmsg_end = 2;
    test_send(opt, cgrp);
// Test >4k pull
    txmsg_pass = 1;
    txmsg_start = 4096;
    txmsg_end = 9182;
    test_send_large(opt, cgrp);
// Test pull + redirect
    txmsg_redir = 1;
    txmsg_start = 1;
    txmsg_end = 2;
    test_send(opt, cgrp);
// Test pull + cork
    txmsg_redir = 0;
    txmsg_cork = 512;
    txmsg_start = 1;
    txmsg_end = 2;
    test_send_many(opt, cgrp);
// Test pull + cork + redirect
    txmsg_redir = 1;
    txmsg_cork = 512;
    txmsg_start = 1;
    txmsg_end = 2;
    test_send_many(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_pop(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_pop(int cgrp, struct sockmap_options *opt)
    {
    let mut data: bool = opt.data_test;
// Test basic pop
    txmsg_pass = 1;
    txmsg_start_pop = 1;
    txmsg_pop = 2;
    test_send_many(opt, cgrp);
// Test pop with >4k
    txmsg_pass = 1;
    txmsg_start_pop = 4096;
    txmsg_pop = 4096;
    test_send_large(opt, cgrp);
// Test pop + redirect
    txmsg_redir = 1;
    txmsg_start_pop = 1;
    txmsg_pop = 2;
    test_send_many(opt, cgrp);
// TODO: Test for pop + cork should be different,
// - It makes the layout of the received data difficult
// - It makes it hard to calculate the total_bytes in the recvmsg
// Temporarily skip the data integrity test for this case now.
//
    opt.data_test = false;
// Test pop + cork
    txmsg_redir = 0;
    txmsg_cork = 512;
    txmsg_start_pop = 1;
    txmsg_pop = 2;
    test_send_many(opt, cgrp);
// Test pop + redirect + cork
    txmsg_redir = 1;
    txmsg_cork = 4;
    txmsg_start_pop = 1;
    txmsg_pop = 2;
    test_send_many(opt, cgrp);
    opt.data_test = data;
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_push(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_push(int cgrp, struct sockmap_options *opt)
    {
    let mut data: bool = opt.data_test;
// Test basic push
    txmsg_pass = 1;
    txmsg_start_push = 1;
    txmsg_end_push = 1;
    test_send(opt, cgrp);
// Test push 4kB >4k
    txmsg_pass = 1;
    txmsg_start_push = 4096;
    txmsg_end_push = 4096;
    test_send_large(opt, cgrp);
// Test push + redirect
    txmsg_redir = 1;
    txmsg_start_push = 1;
    txmsg_end_push = 2;
    test_send_many(opt, cgrp);
// TODO: Test for push + cork should be different,
// - It makes the layout of the received data difficult
// - It makes it hard to calculate the total_bytes in the recvmsg
// Temporarily skip the data integrity test for this case now.
//
    opt.data_test = false;
// Test push + cork
    txmsg_redir = 0;
    txmsg_cork = 512;
    txmsg_start_push = 1;
    txmsg_end_push = 2;
    test_send_many(opt, cgrp);
    opt.data_test = data;
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_push_pop(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_push_pop(int cgrp, struct sockmap_options *opt)
    {
// Test push/pop range overlapping
    txmsg_pass = 1;
    txmsg_start_push = 1;
    txmsg_end_push = 10;
    txmsg_start_pop = 5;
    txmsg_pop = 4;
    test_send_large(opt, cgrp);
    txmsg_pass = 1;
    txmsg_start_push = 1;
    txmsg_end_push = 10;
    txmsg_start_pop = 5;
    txmsg_pop = 16;
    test_send_large(opt, cgrp);
    txmsg_pass = 1;
    txmsg_start_push = 5;
    txmsg_end_push = 4;
    txmsg_start_pop = 1;
    txmsg_pop = 10;
    test_send_large(opt, cgrp);
    txmsg_pass = 1;
    txmsg_start_push = 5;
    txmsg_end_push = 16;
    txmsg_start_pop = 1;
    txmsg_pop = 10;
    test_send_large(opt, cgrp);
// Test push/pop range non-overlapping
    txmsg_pass = 1;
    txmsg_start_push = 1;
    txmsg_end_push = 10;
    txmsg_start_pop = 16;
    txmsg_pop = 4;
    test_send_large(opt, cgrp);
    txmsg_pass = 1;
    txmsg_start_push = 16;
    txmsg_end_push = 10;
    txmsg_start_pop = 5;
    txmsg_pop = 4;
    test_send_large(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_apply(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_apply(int cgrp, struct sockmap_options *opt)
    {
    txmsg_pass = 1;
    txmsg_redir = 0;
    txmsg_ingress = 0;
    txmsg_apply = 1;
    txmsg_cork = 0;
    test_send_one(opt, cgrp);
    txmsg_pass = 0;
    txmsg_redir = 1;
    txmsg_ingress = 0;
    txmsg_apply = 1;
    txmsg_cork = 0;
    test_send_one(opt, cgrp);
    txmsg_pass = 0;
    txmsg_redir = 1;
    txmsg_ingress = 1;
    txmsg_apply = 1;
    txmsg_cork = 0;
    test_send_one(opt, cgrp);
    txmsg_pass = 1;
    txmsg_redir = 0;
    txmsg_ingress = 0;
    txmsg_apply = 1024;
    txmsg_cork = 0;
    test_send_large(opt, cgrp);
    txmsg_pass = 0;
    txmsg_redir = 1;
    txmsg_ingress = 0;
    txmsg_apply = 1024;
    txmsg_cork = 0;
    test_send_large(opt, cgrp);
    txmsg_pass = 0;
    txmsg_redir = 1;
    txmsg_ingress = 1;
    txmsg_apply = 1024;
    txmsg_cork = 0;
    test_send_large(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_cork(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_cork(int cgrp, struct sockmap_options *opt)
    {
    txmsg_pass = 1;
    txmsg_redir = 0;
    txmsg_apply = 0;
    txmsg_cork = 1;
    test_send(opt, cgrp);
    txmsg_pass = 1;
    txmsg_redir = 0;
    txmsg_apply = 1;
    txmsg_cork = 1;
    test_send(opt, cgrp);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_ingress_parser(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_ingress_parser(int cgrp, struct sockmap_options *opt)
    {
    txmsg_pass = 1;
    skb_use_parser = 512;
    opt.iov_length = 256;
    opt.iov_count = 1;
    opt.rate = 2;
    test_exec(cgrp, opt);
    }
#[no_mangle]
unsafe extern "C" fn test_txmsg_ingress_parser2(cgrp: c_int, opt: *mut sockmap_options) {
    static void test_txmsg_ingress_parser2(int cgrp, struct sockmap_options *opt)
    {
    skb_use_parser = 10;
    opt.iov_length = 20;
    opt.iov_count = 1;
    opt.rate = 1;
    opt.check_recved_len = true;
    test_exec(cgrp, opt);
    opt.check_recved_len = false;
    }
    char *map_names[] = {
    "sock_map",
    "sock_map_txmsg",
    "sock_map_redir",
    "sock_apply_bytes",
    "sock_cork_bytes",
    "sock_bytes",
    "sock_redir_flags",
    "sock_skb_opts",
    };
#[no_mangle]
unsafe extern "C" fn populate_progs(bpf_file: *mut c_char) -> c_int {
    static int populate_progs(char *bpf_file)
    {
    struct bpf_program *prog;
    struct bpf_object *obj;
    let mut i: c_int = 0;
    long err;
    obj = bpf_object__open(bpf_file);
    err = libbpf_get_error(obj);
    if (err) {
    char err_buf[256];
    libbpf_strerror(err, err_buf, sizeof(err_buf));
    printf("Unable to load eBPF objects in file '%s' : %s\n",
    bpf_file, err_buf);
    return -1;
    }
    i = bpf_object__load(obj);
    i = 0;
    bpf_object__for_each_program(prog, obj) {
    progs[i] = prog;
    i++;
    }
    for (i = 0; i < ARRAY_SIZE(map_fd); i++) {
    maps[i] = bpf_object__find_map_by_name(obj, map_names[i]);
    map_fd[i] = bpf_map__fd(maps[i]);
    if (map_fd[i] < 0) {
    fprintf(stderr, "load_bpf_file: (%i) %s\n",
    map_fd[i], strerror(errno));
    return -1;
    }
    }
    for (i = 0; i < ARRAY_SIZE(links); i++)
    links[i] = core::ptr::null_mut();
    return 0;
    }
    struct _test test[] = {
    {"txmsg test passthrough", test_txmsg_pass},
    {"txmsg test redirect", test_txmsg_redir},
    {"txmsg test redirect wait send mem", test_txmsg_redir_wait_sndmem},
    {"txmsg test drop", test_txmsg_drop},
    {"txmsg test ingress redirect", test_txmsg_ingress_redir},
    {"txmsg test apply", test_txmsg_apply},
    {"txmsg test cork", test_txmsg_cork},
    {"txmsg test hanging corks", test_txmsg_cork_hangs},
    {"txmsg test push_data", test_txmsg_push},
    {"txmsg test pull-data", test_txmsg_pull},
    {"txmsg test pop-data", test_txmsg_pop},
    {"txmsg test push/pop data", test_txmsg_push_pop},
    {"txmsg test ingress parser", test_txmsg_ingress_parser},
    {"txmsg test ingress parser2", test_txmsg_ingress_parser2},
    };
#[no_mangle]
unsafe extern "C" fn check_whitelist(t: *mut _test, opt: *mut sockmap_options) -> c_int {
    static int check_whitelist(struct _test *t, struct sockmap_options *opt)
    {
    char *entry, *ptr;
    if (!opt.whitelist)
    return 0;
    ptr = strdup(opt.whitelist);
    if (!ptr)
    return -ENOMEM;
    entry = strtok(ptr, ",");
    while (entry) {
    if ((opt.prepend && strstr(opt.prepend, entry) != 0) ||
    strstr(opt.map, entry) != 0 ||
    strstr(t.title, entry) != 0) {
    free(ptr);
    return 0;
    }
    entry = strtok(core::ptr::null_mut(), ",");
    }
    free(ptr);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn check_blacklist(t: *mut _test, opt: *mut sockmap_options) -> c_int {
    static int check_blacklist(struct _test *t, struct sockmap_options *opt)
    {
    char *entry, *ptr;
    if (!opt.blacklist)
    return -EINVAL;
    ptr = strdup(opt.blacklist);
    if (!ptr)
    return -ENOMEM;
    entry = strtok(ptr, ",");
    while (entry) {
    if ((opt.prepend && strstr(opt.prepend, entry) != 0) ||
    strstr(opt.map, entry) != 0 ||
    strstr(t.title, entry) != 0) {
    free(ptr);
    return 0;
    }
    entry = strtok(core::ptr::null_mut(), ",");
    }
    free(ptr);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn __test_selftests(cg_fd: c_int, opt: *mut sockmap_options) -> c_int {
    static int __test_selftests(int cg_fd, struct sockmap_options *opt)
    {
    int i, err;
    err = populate_progs(opt.map);
    if (err < 0) {
    fprintf(stderr, "ERROR: (%i) load bpf failed\n", err);
    return err;
    }
// Tests basic commands and APIs
    for (i = 0; i < ARRAY_SIZE(test); i++) {
    let mut t: _test = test[i];
    if (check_whitelist(&t, opt) != 0)
    continue;
    if (check_blacklist(&t, opt) == 0)
    continue;
    test_start_subtest(&t, opt);
    t.tester(cg_fd, opt);
    test_end_subtest();
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_selftests_sockmap(cg_fd: c_int, opt: *mut sockmap_options) {
    static void test_selftests_sockmap(int cg_fd, struct sockmap_options *opt)
    {
    opt.map = BPF_SOCKMAP_FILENAME;
    __test_selftests(cg_fd, opt);
    }
#[no_mangle]
unsafe extern "C" fn test_selftests_sockhash(cg_fd: c_int, opt: *mut sockmap_options) {
    static void test_selftests_sockhash(int cg_fd, struct sockmap_options *opt)
    {
    opt.map = BPF_SOCKHASH_FILENAME;
    __test_selftests(cg_fd, opt);
    }
#[no_mangle]
unsafe extern "C" fn test_selftest(cg_fd: c_int, opt: *mut sockmap_options) -> c_int {
    static int test_selftest(int cg_fd, struct sockmap_options *opt)
    {
    test_selftests_sockmap(cg_fd, opt);
    test_selftests_sockhash(cg_fd, opt);
    test_print_results();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut iov_count: c_int = 1, length = 1024, rate = 1;
    let mut options: sockmap_options = {0};
    int opt, longindex, err, cg_fd = 0;
    char *bpf_file = BPF_SOCKMAP_FILENAME;
    let mut test: c_int = SELFTESTS;
    let mut cg_created: bool = 0;
    while ((opt = getopt_long(argc, argv, ":dhv:c:r:i:l:t:p:q:n:b:",
    long_options, &longindex)) != -1) {
    switch (opt) {
    case 's':
    txmsg_start = atoi(optarg);
    break;
    case 'e':
    txmsg_end = atoi(optarg);
    break;
    case 'p':
    txmsg_start_push = atoi(optarg);
    break;
    case 'q':
    txmsg_end_push = atoi(optarg);
    break;
    case 'w':
    txmsg_start_pop = atoi(optarg);
    break;
    case 'x':
    txmsg_pop = atoi(optarg);
    break;
    case 'a':
    txmsg_apply = atoi(optarg);
    break;
    case 'k':
    txmsg_cork = atoi(optarg);
    break;
    case 'c':
    cg_fd = open(optarg, O_DIRECTORY, O_RDONLY);
    if (cg_fd < 0) {
    fprintf(stderr,
    "ERROR: (%i) open cg path failed: %s\n",
    cg_fd, optarg);
    return cg_fd;
    }
    break;
    case 'r':
    rate = atoi(optarg);
    break;
    case 'v':
    options.verbose = 1;
    if (optarg)
    options.verbose = atoi(optarg);
    break;
    case 'i':
    iov_count = atoi(optarg);
    break;
    case 'l':
    length = atoi(optarg);
    break;
    case 'd':
    options.data_test = true;
    break;
    case 't':
    if (strcmp(optarg, "ping") == 0) {
    test = PING_PONG;
    } else if (strcmp(optarg, "sendmsg") == 0) {
    test = SENDMSG;
    } else if (strcmp(optarg, "base") == 0) {
    test = BASE;
    } else if (strcmp(optarg, "base_sendpage") == 0) {
    test = BASE_SENDPAGE;
    } else if (strcmp(optarg, "sendpage") == 0) {
    test = SENDPAGE;
    } else {
    usage(argv);
    return -1;
    }
    break;
    case 'n':
    options.whitelist = strdup(optarg);
    if (!options.whitelist)
    return -ENOMEM;
    break;
    case 'b':
    options.blacklist = strdup(optarg);
    if (!options.blacklist)
    return -ENOMEM;
    case 0:
    break;
    case 'h':
    default:
    usage(argv);
    return -1;
    }
    }
    if (!cg_fd) {
    cg_fd = cgroup_setup_and_join(CG_PATH);
    if (cg_fd < 0)
    return cg_fd;
    cg_created = 1;
    }
// Use libbpf 1.0 API mode
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    if (test == SELFTESTS) {
    err = test_selftest(cg_fd, &options);
    goto out;
    }
    err = populate_progs(bpf_file);
    if (err) {
    fprintf(stderr, "populate program: (%s) %s\n",
    bpf_file, strerror(errno));
    return 1;
    }
    running = 1;
// catch SIGINT
    signal(SIGINT, running_handler);
    options.iov_count = iov_count;
    options.iov_length = length;
    options.rate = rate;
    err = run_options(&options, cg_fd, test);
    out:
    if (options.whitelist)
    free(options.whitelist);
    if (options.blacklist)
    free(options.blacklist);
    close(cg_fd);
    if (cg_created)
    cleanup_cgroup_environment();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn running_handler(a: c_int) {
    void running_handler(int a)
    {
    running = 0;
    }
