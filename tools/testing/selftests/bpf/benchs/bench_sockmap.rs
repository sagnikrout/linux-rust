//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_sockmap.c
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

pub const DATA_REPEAT_SIZE: c_int = 10;
    static const char snd_data[DATA_REPEAT_SIZE] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
// c1 <-> [p1, p2] <-> c2
// RX bench(BPF_SK_SKB_STREAM_VERDICT):
// ARG_FW_RX_PASS:
// send(p2) -> recv(c2) -> bpf skb passthrough -> recv(c2)
// ARG_FW_RX_VERDICT_EGRESS:
// send(c1) -> verdict skb to tx queuec of p2 -> recv(c2)
// ARG_FW_RX_VERDICT_INGRESS:
// send(c1) -> verdict skb to rx queuec of c2 -> recv(c2)
//
// TX bench(BPF_SK_MSG_VERDIC):
// ARG_FW_TX_PASS:
// send(p2) -> bpf msg passthrough -> send(p2) -> recv(c2)
// ARG_FW_TX_VERDICT_INGRESS:
// send(p2) -> verdict msg to rx queue of c2 -> recv(c2)
// ARG_FW_TX_VERDICT_EGRESS:
// send(p1) -> verdict msg to tx queue of p2 -> recv(c2)
//
    enum SOCKMAP_ARG_FLAG {
    ARG_FW_RX_NORMAL = 11000,
    ARG_FW_RX_PASS,
    ARG_FW_RX_VERDICT_EGRESS,
    ARG_FW_RX_VERDICT_INGRESS,
    ARG_FW_TX_NORMAL,
    ARG_FW_TX_PASS,
    ARG_FW_TX_VERDICT_INGRESS,
    ARG_FW_TX_VERDICT_EGRESS,
    ARG_CTL_RX_STRP,
    ARG_CONSUMER_DELAY_TIME,
    ARG_PRODUCER_DURATION,
    };

    ((ctx.mode) == ARG_FW_TX_NORMAL)

    ((ctx.mode) == ARG_FW_TX_VERDICT_INGRESS)

    ((ctx.mode) == ARG_FW_TX_VERDICT_EGRESS)

    ((ctx.mode) == ARG_FW_TX_PASS)

    TXMODE_BPF_PASS() ||			\
    TXMODE_BPF_INGRESS() ||			\
    TXMODE_BPF_EGRESS())

    TXMODE_NORMAL() ||			\
    TXMODE_BPF())

    ((ctx.mode) == ARG_FW_RX_NORMAL)

    ((ctx.mode) == ARG_FW_RX_PASS)

    ((ctx.mode) == ARG_FW_RX_VERDICT_EGRESS)

    ((ctx.mode) == ARG_FW_RX_VERDICT_INGRESS)

    RXMODE_BPF_VERDICT_INGRESS() ||		\
    RXMODE_BPF_VERDICT_EGRESS())

    RXMODE_BPF_PASS() ||			\
    RXMODE_BPF_VERDICT())

    RXMODE_NORMAL() ||			\
    RXMODE_BPF())
    static struct socmap_ctx {
    struct bench_sockmap_prog *skel;
    enum SOCKMAP_ARG_FLAG mode;

    int		fds[5];
    long		send_calls;
    long		read_calls;
    long		prod_send;
    long		user_read;
    int		file_size;
    int		delay_consumer;
    int		prod_run_time;
    int		strp_size;
    } ctx = {
    .prod_send	= 0,
    .user_read	= 0,
    .file_size	= FILE_SIZE,
    .mode		= ARG_FW_RX_VERDICT_EGRESS,
    .fds		= {0},
    .delay_consumer = 0,
    .prod_run_time	= 0,
    .strp_size	= 0,
    };
#[no_mangle]
unsafe extern "C" fn bench_sockmap_prog_destroy() {
    static void bench_sockmap_prog_destroy(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(ctx.fds); i++) {
    if (ctx.fds[i] > 0)
    close(ctx.fds[i]);
    }
    bench_sockmap_prog__destroy(ctx.skel);
    }
    static void init_addr(struct sockaddr_storage *ss,
    socklen_t *len)
    {
    struct sockaddr_in *addr4 = memset(ss, 0, sizeof(*ss));
    addr4.sin_family = AF_INET;
    addr4.sin_port = 0;
    addr4.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
// len = sizeof(*addr4);
    }
#[no_mangle]
unsafe extern "C" fn set_non_block(fd: c_int, blocking: bool) -> bool {
    static bool set_non_block(int fd, bool blocking)
    {
    let mut flags: c_int = fcntl(fd, F_GETFL, 0);
    if (flags == -1)
    return false;
    flags = blocking ? (flags | O_NONBLOCK) : (flags & ~O_NONBLOCK);
    return (fcntl(fd, F_SETFL, flags) == 0);
    }
#[no_mangle]
unsafe extern "C" fn create_pair(c: *mut c_int, p: *mut c_int, type: c_int) -> c_int {
    static int create_pair(int *c, int *p, int type)
    {
    struct sockaddr_storage addr;
    int err, cfd, pfd;
    let mut addr_len: socklen_t = sizeof(struct sockaddr_storage);
    err = getsockname(ctx.sfd, (struct sockaddr *)&addr, &addr_len);
    if (err) {
    fprintf(stderr, "getsockname error %d\n", errno);
    return err;
    }
    cfd = socket(AF_INET, type, 0);
    if (cfd < 0) {
    fprintf(stderr, "socket error %d\n", errno);
    return err;
    }
    err = connect(cfd, (struct sockaddr *)&addr, addr_len);
    if (err && errno != EINPROGRESS) {
    fprintf(stderr, "connect error %d\n", errno);
    return err;
    }
    pfd = accept(ctx.sfd, core::ptr::null_mut(), core::ptr::null_mut());
    if (pfd < 0) {
    fprintf(stderr, "accept error %d\n", errno);
    return err;
    }
// c = cfd;
// p = pfd;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn create_sockets() -> c_int {
    static int create_sockets(void)
    {
    struct sockaddr_storage addr;
    int err, one = 1;
    socklen_t addr_len;
    init_addr(&addr, &addr_len);
    ctx.sfd = socket(AF_INET, SOCK_STREAM, 0);
    if (ctx.sfd < 0) {
    fprintf(stderr, "socket error:%d\n", errno);
    return ctx.sfd;
    }
    err = setsockopt(ctx.sfd, SOL_SOCKET, SO_REUSEPORT, &one, sizeof(one));
    if (err) {
    fprintf(stderr, "setsockopt error:%d\n", errno);
    return err;
    }
    err = bind(ctx.sfd, (struct sockaddr *)&addr, addr_len);
    if (err) {
    fprintf(stderr, "bind error:%d\n", errno);
    return err;
    }
    err = listen(ctx.sfd, SOMAXCONN);
    if (err) {
    fprintf(stderr, "listen error:%d\n", errno);
    return err;
    }
    err = create_pair(&ctx.c1, &ctx.p1, SOCK_STREAM);
    if (err) {
    fprintf(stderr, "create_pair 1 error\n");
    return err;
    }
    err = create_pair(&ctx.c2, &ctx.p2, SOCK_STREAM);
    if (err) {
    fprintf(stderr, "create_pair 2 error\n");
    return err;
    }
    printf("create socket fd c1:%d p1:%d c2:%d p2:%d\n",
    ctx.c1, ctx.p1, ctx.c2, ctx.p2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn validate() {
    static void validate(void)
    {
    if (env.consumer_cnt != 2 || env.producer_cnt != 1 ||
    !env.affinity)
    goto err;
    return;
    err:
    fprintf(stderr, "argument '-c 2 -p 1 -a' is necessary");
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn setup_rx_sockmap() -> c_int {
    static int setup_rx_sockmap(void)
    {
    int verdict, pass, parser, map;
    let mut zero: c_int = 0, one = 1;
    int err;
    parser = bpf_program__fd(ctx.skel.progs.prog_skb_parser);
    verdict = bpf_program__fd(ctx.skel.progs.prog_skb_verdict);
    pass = bpf_program__fd(ctx.skel.progs.prog_skb_pass);
    map = bpf_map__fd(ctx.skel.maps.sock_map_rx);
    if (ctx.strp_size != 0) {
    ctx.skel.bss.pkt_size = ctx.strp_size;
    err = bpf_prog_attach(parser, map, BPF_SK_SKB_STREAM_PARSER, 0);
    if (err)
    return err;
    }
    if (RXMODE_BPF_VERDICT())
    err = bpf_prog_attach(verdict, map, BPF_SK_SKB_STREAM_VERDICT, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: RXMODE_BPF_PASS()) -> else {
    else if (RXMODE_BPF_PASS())
    err = bpf_prog_attach(pass, map, BPF_SK_SKB_STREAM_VERDICT, 0);
    if (err)
    return err;
    if (RXMODE_BPF_PASS())
    return bpf_map_update_elem(map, &zero, &ctx.c2, BPF_NOEXIST);
    err = bpf_map_update_elem(map, &zero, &ctx.p1, BPF_NOEXIST);
    if (err < 0)
    return err;
    if (RXMODE_BPF_VERDICT_INGRESS()) {
    ctx.skel.bss.verdict_dir = BPF_F_INGRESS;
    err = bpf_map_update_elem(map, &one, &ctx.c2, BPF_NOEXIST);
    } else {
    err = bpf_map_update_elem(map, &one, &ctx.p2, BPF_NOEXIST);
    }
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_tx_sockmap() -> c_int {
    static int setup_tx_sockmap(void)
    {
    let mut zero: c_int = 0, one = 1;
    int prog, map;
    int err;
    map = bpf_map__fd(ctx.skel.maps.sock_map_tx);
    prog = TXMODE_BPF_PASS() ?
    bpf_program__fd(ctx.skel.progs.prog_skmsg_pass) :
    bpf_program__fd(ctx.skel.progs.prog_skmsg_verdict);
    err = bpf_prog_attach(prog, map, BPF_SK_MSG_VERDICT, 0);
    if (err)
    return err;
    if (TXMODE_BPF_EGRESS()) {
    err = bpf_map_update_elem(map, &zero, &ctx.p1, BPF_NOEXIST);
    err |= bpf_map_update_elem(map, &one, &ctx.p2, BPF_NOEXIST);
    } else {
    ctx.skel.bss.verdict_dir = BPF_F_INGRESS;
    err = bpf_map_update_elem(map, &zero, &ctx.p2, BPF_NOEXIST);
    err |= bpf_map_update_elem(map, &one, &ctx.c2, BPF_NOEXIST);
    }
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup() {
    static void setup(void)
    {
    int err;
    ctx.skel = bench_sockmap_prog__open_and_load();
    if (!ctx.skel) {
    fprintf(stderr, "error loading skel\n");
    exit(1);
    }
    if (create_sockets()) {
    fprintf(stderr, "create_net_mode error\n");
    goto err;
    }
    if (RXMODE_BPF()) {
    err = setup_rx_sockmap();
    if (err) {
    fprintf(stderr, "setup_rx_sockmap error:%d\n", err);
    goto err;
    }
    } else if (TXMODE_BPF()) {
    err = setup_tx_sockmap();
    if (err) {
    fprintf(stderr, "setup_tx_sockmap error:%d\n", err);
    goto err;
    }
    } else {
    fprintf(stderr, "unknown sockmap bench mode: %d\n", ctx.mode);
    goto err;
    }
    return;
    err:
    bench_sockmap_prog_destroy();
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn measure(res: *mut bench_res) {
    static void measure(struct bench_res *res)
    {
    res.drops = atomic_swap(&ctx.prod_send, 0);
    res.hits = atomic_swap(&ctx.skel.bss.process_byte, 0);
    res.false_hits = atomic_swap(&ctx.user_read, 0);
    res.important_hits = atomic_swap(&ctx.send_calls, 0);
    res.important_hits |= atomic_swap(&ctx.read_calls, 0) << 32;
    }
#[no_mangle]
unsafe extern "C" fn verify_data(check_pos: *mut c_int, buf: *mut c_char, rcv: c_int) {
    static void verify_data(int *check_pos, char *buf, int rcv)
    {
    for (int i = 0 ; i < rcv; i++) {
    if (buf[i] != snd_data[(*check_pos) % DATA_REPEAT_SIZE]) {
    fprintf(stderr, "verify data fail");
    exit(1);
    }
    (*check_pos)++;
    if (*check_pos >= FILE_SIZE)
// check_pos = 0;
    }
    }
    static void *consumer(void *input)
    {
    int rcv, sent;
    let mut check_pos: c_int = 0;
    let mut tid: c_int = (long)input;
    let mut recv_buf_size: c_int = FILE_SIZE;
    char *buf = malloc(recv_buf_size);
    let mut delay_read: c_int = ctx.delay_consumer;
    if (!buf) {
    fprintf(stderr, "fail to init read buffer");
    return core::ptr::null_mut();
    }
    while (true) {
    if (tid == 1) {
// consumer 1 is unused for tx test and stream verdict test
    if (RXMODE_BPF() || TXMODE())
    return core::ptr::null_mut();
// it's only for RX_NORMAL which service as reserve-proxy mode
    rcv = read(ctx.p1, buf, recv_buf_size);
    if (rcv < 0) {
    fprintf(stderr, "fail to read p1");
    return core::ptr::null_mut();
    }
    sent = send(ctx.p2, buf, recv_buf_size, 0);
    if (sent < 0) {
    fprintf(stderr, "fail to send p2");
    return core::ptr::null_mut();
    }
    } else {
    if (delay_read != 0) {
    if (delay_read < 0)
    return core::ptr::null_mut();
    sleep(delay_read);
    delay_read = 0;
    }
// read real endpoint by consumer 0
    atomic_inc(&ctx.read_calls);
    rcv = read(ctx.c2, buf, recv_buf_size);
    if (rcv < 0 && errno != EAGAIN) {
    fprintf(stderr, "%s fail to read c2 %d\n", __func__, errno);
    return core::ptr::null_mut();
    }
    verify_data(&check_pos, buf, rcv);
    atomic_add(&ctx.user_read, rcv);
    }
    }
    return core::ptr::null_mut();
    }
    static void *producer(void *input)
    {
    let mut off: c_int = 0, fp, need_sent, sent;
    let mut file_size: c_int = ctx.file_size;
    struct timespec ts1, ts2;
    int target;
    FILE *file;
    file = tmpfile();
    if (!file) {
    fprintf(stderr, "create file for sendfile");
    return core::ptr::null_mut();
    }
// we need simple verify
    for (int i = 0; i < file_size; i++) {
    if (fwrite(&snd_data[off], sizeof(char), 1, file) != 1) {
    fprintf(stderr, "init tmpfile error");
    return core::ptr::null_mut();
    }
    if (++off >= sizeof(snd_data))
    off = 0;
    }
    fflush(file);
    fseek(file, 0, SEEK_SET);
    fp = fileno(file);
    need_sent = file_size;
    clock_gettime(CLOCK_MONOTONIC, &ts1);
    if (RXMODE_BPF_VERDICT())
    target = ctx.c1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: TXMODE_BPF_EGRESS()) -> else {
    else if (TXMODE_BPF_EGRESS())
    target = ctx.p1;
    else
    target = ctx.p2;
    set_non_block(target, true);
    while (true) {
    if (ctx.prod_run_time) {
    clock_gettime(CLOCK_MONOTONIC, &ts2);
    if (ts2.tv_sec - ts1.tv_sec > ctx.prod_run_time)
    return core::ptr::null_mut();
    }
    errno = 0;
    atomic_inc(&ctx.send_calls);
    sent = sendfile(target, fp, core::ptr::null_mut(), need_sent);
    if (sent < 0) {
    if (errno != EAGAIN && errno != ENOMEM && errno != ENOBUFS) {
    fprintf(stderr, "sendfile return %d, errorno %d:%s\n",
    sent, errno, strerror(errno));
    return core::ptr::null_mut();
    }
    continue;
    } else if (sent < need_sent) {
    need_sent -= sent;
    atomic_add(&ctx.prod_send, sent);
    continue;
    }
    atomic_add(&ctx.prod_send, need_sent);
    need_sent = file_size;
    lseek(fp, 0, SEEK_SET);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long) {
    static void report_progress(int iter, struct bench_res *res, long delta_ns)
    {
    double speed_mbs, prod_mbs, bpf_mbs, send_hz, read_hz;
    prod_mbs = res.drops / 1000000.0 / (delta_ns / 1000000000.0);
    speed_mbs = res.false_hits / 1000000.0 / (delta_ns / 1000000000.0);
    bpf_mbs = res.hits / 1000000.0 / (delta_ns / 1000000000.0);
    send_hz = (res.important_hits & 0xFFFFFFFF) / (delta_ns / 1000000000.0);
    read_hz = (res.important_hits >> 32) / (delta_ns / 1000000000.0);
    printf("Iter %3d (%7.3lfus): ",
    iter, (delta_ns - 1000000000) / 1000.0);
    printf("Send Speed %8.3lf MB/s (%8.3lf calls/s), BPF Speed %8.3lf MB/s, "
    "Rcv Speed %8.3lf MB/s (%8.3lf calls/s)\n",
    prod_mbs, send_hz, bpf_mbs, speed_mbs, read_hz);
    }
#[no_mangle]
unsafe extern "C" fn report_final(res[]: bench_res, res_cnt: c_int) {
    static void report_final(struct bench_res res[], int res_cnt)
    {
    let mut verdict_mbs_mean: double = 0.0;
    let mut verdict_total: c_long = 0;
    int i;
    for (i = 0; i < res_cnt; i++) {
    verdict_mbs_mean += res[i].hits / 1000000.0 / (0.0 + res_cnt);
    verdict_total += res[i].hits / 1000000.0;
    }
    printf("Summary: total trans %8.3lu MB \u00B1 %5.3lf MB/s\n",
    verdict_total, verdict_mbs_mean);
    }
    static const struct argp_option opts[] = {
    { "rx-normal", ARG_FW_RX_NORMAL, core::ptr::null_mut(), 0,
    "simple reserve-proxy mode, no bfp enabled"},
    { "rx-pass", ARG_FW_RX_PASS, core::ptr::null_mut(), 0,
    "run bpf prog but no redir applied"},
    { "rx-strp", ARG_CTL_RX_STRP, "Byte", 0,
    "enable strparser and set the encapsulation size"},
    { "rx-verdict-egress", ARG_FW_RX_VERDICT_EGRESS, core::ptr::null_mut(), 0,
    "forward data with bpf(stream verdict)"},
    { "rx-verdict-ingress", ARG_FW_RX_VERDICT_INGRESS, core::ptr::null_mut(), 0,
    "forward data with bpf(stream verdict)"},
    { "tx-normal", ARG_FW_TX_NORMAL, core::ptr::null_mut(), 0,
    "simple c-s mode, no bfp enabled"},
    { "tx-pass", ARG_FW_TX_PASS, core::ptr::null_mut(), 0,
    "run bpf prog but no redir applied"},
    { "tx-verdict-ingress", ARG_FW_TX_VERDICT_INGRESS, core::ptr::null_mut(), 0,
    "forward msg to ingress queue of another socket"},
    { "tx-verdict-egress", ARG_FW_TX_VERDICT_EGRESS, core::ptr::null_mut(), 0,
    "forward msg to egress queue of another socket"},
    { "delay-consumer", ARG_CONSUMER_DELAY_TIME, "SEC", 0,
    "delay consumer start"},
    { "producer-duration", ARG_PRODUCER_DURATION, "SEC", 0,
    "producer duration"},
    {},
    };
#[no_mangle]
unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    static error_t parse_arg(int key, char *arg, struct argp_state *state)
    {
    switch (key) {
    case ARG_FW_RX_NORMAL...ARG_FW_TX_VERDICT_EGRESS:
    ctx.mode = key;
    break;
    case ARG_CONSUMER_DELAY_TIME:
    ctx.delay_consumer = strtol(arg, core::ptr::null_mut(), 10);
    break;
    case ARG_PRODUCER_DURATION:
    ctx.prod_run_time = strtol(arg, core::ptr::null_mut(), 10);
    break;
    case ARG_CTL_RX_STRP:
    ctx.strp_size = strtol(arg, core::ptr::null_mut(), 10);
    break;
    default:
    return ARGP_ERR_UNKNOWN;
    }
    return 0;
    }
// exported into benchmark runner
    const struct argp bench_sockmap_argp = {
    .options	= opts,
    .parser		= parse_arg,
    };
// Benchmark performance of creating bpf local storage
    const struct bench bench_sockmap = {
    .name			= "sockmap",
    .argp			= &bench_sockmap_argp,
    .validate		= validate,
    .setup			= setup,
    .producer_thread	= producer,
    .consumer_thread	= consumer,
    .measure		= measure,
    .report_progress	= report_progress,
    .report_final		= report_final,
    };
