//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/flow_dissector_classification.c
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
// Macro flag: #define _GNU_SOURCE

pub const CFG_PORT_INNER: c_int = 8000;
pub const CFG_PORT_GUE: c_int = 6080;
pub const SUBTEST_NAME_MAX_LEN: c_int = 32;

pub const MAX_SOURCE_PORTS: c_int = 3;
pub const TEST_PACKETS_COUNT: c_int = 10;
pub const TEST_PACKET_LEN: c_int = 100;

    {							\
    .sin_family = AF_INET,				\
    .sin_port = __constant_htons(port),		\
    .sin_addr.s_addr = __constant_htonl(addr4),	\
    }

    {						\
    .sin6_family = AF_INET6,		\
    .sin6_port = __constant_htons(port),	\
    .sin6_addr = addr6,			\
    }

    INIT_ADDR6(IN6ADDR_LOOPBACK_INIT, CFG_PORT_INNER)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grehdr {
    pub unused: u16,
    pub protocol: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guehdr {
    union {
    struct {

    pub 2: __u8 hlen : 5, control : 1, version :,

    pub 5: __u8 version : 2, control : 1, hlen :,

    pub proto_ctype: __u8,
    pub flags: __be16,
}

    __be32 word;
    };
    };
    static char buf[ETH_DATA_LEN];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_configuration {
    pub name: [c_char; SUBTEST_NAME_MAX_LEN],
    pub (*test_setup)(void): *mut c_int,
    pub (*test_teardown)(void): *mut c_void,
    pub source_ports: [c_int; MAX_SOURCE_PORTS],
    pub cfg_l3_inner: c_int,
    pub in_saddr4: sockaddr_in,
    pub in_daddr4: sockaddr_in,
    pub in_saddr6: sockaddr_in6,
    pub in_daddr6: sockaddr_in6,
    pub cfg_l3_outer: c_int,
    pub out_saddr4: sockaddr_in,
    pub out_daddr4: sockaddr_in,
    pub out_saddr6: sockaddr_in6,
    pub out_daddr6: sockaddr_in6,
    pub cfg_encap_proto: c_int,
    pub cfg_dsfield_inner: u8,
    pub cfg_dsfield_outer: u8,
    pub cfg_l3_extra: c_int,
    pub extra_saddr4: sockaddr_in,
    pub extra_daddr4: sockaddr_in,
    pub extra_saddr6: sockaddr_in6,
    pub extra_daddr6: sockaddr_in6,
}

#[no_mangle]
unsafe extern "C" fn util_gettime() -> c_ulong {
    static unsigned long util_gettime(void)
    {
    struct timeval tv;
    gettimeofday(&tv, core::ptr::null_mut());
    return (tv.tv_sec * 1000) + (tv.tv_usec / 1000);
    }
    static void build_ipv4_header(void *header, uint8_t proto, uint32_t src,
    uint32_t dst, int payload_len, uint8_t tos)
    {
    struct iphdr *iph = header;
    iph.ihl = 5;
    iph.version = 4;
    iph.tos = tos;
    iph.ttl = 8;
    iph.tot_len = htons(sizeof(*iph) + payload_len);
    iph.id = htons(1337);
    iph.protocol = proto;
    iph.saddr = src;
    iph.daddr = dst;
    iph.check = build_ip_csum((void *)iph);
    }
#[no_mangle]
unsafe extern "C" fn ipv6_set_dsfield(ip6h: *mut ipv6hdr, dsfield: u8) {
    static void ipv6_set_dsfield(struct ipv6hdr *ip6h, uint8_t dsfield)
    {
    uint16_t val, *ptr = (uint16_t *)ip6h;
    val = ntohs(*ptr);
    val &= 0xF00F;
    val |= ((uint16_t)dsfield) << 4;
// ptr = htons(val);
    }
    static void build_ipv6_header(void *header, uint8_t proto,
    const struct sockaddr_in6 *src,
    const struct sockaddr_in6 *dst, int payload_len,
    uint8_t dsfield)
    {
    struct ipv6hdr *ip6h = header;
    ip6h.version = 6;
    ip6h.payload_len = htons(payload_len);
    ip6h.nexthdr = proto;
    ip6h.hop_limit = 8;
    ipv6_set_dsfield(ip6h, dsfield);
    memcpy(&ip6h.saddr, &src.sin6_addr, sizeof(ip6h.saddr));
    memcpy(&ip6h.daddr, &dst.sin6_addr, sizeof(ip6h.daddr));
    }
    static void build_udp_header(void *header, int payload_len, uint16_t sport,
    uint16_t dport, int family)
    {
    struct udphdr *udph = header;
    let mut len: c_int = sizeof(*udph) + payload_len;
    udph.source = htons(sport);
    udph.dest = htons(dport);
    udph.len = htons(len);
    udph.check = 0;
    if (family == AF_INET)
    udph.check = build_udp_v4_csum(header - sizeof(struct iphdr),
    udph);
    else
    udph.check = build_udp_v6_csum(header - sizeof(struct ipv6hdr),
    udph);
    }
#[no_mangle]
unsafe extern "C" fn build_gue_header(header: *mut c_void, proto: u8) {
    static void build_gue_header(void *header, uint8_t proto)
    {
    struct guehdr *gueh = header;
    gueh.proto_ctype = proto;
    }
#[no_mangle]
unsafe extern "C" fn build_gre_header(header: *mut c_void, proto: u16) {
    static void build_gre_header(void *header, uint16_t proto)
    {
    struct grehdr *greh = header;
    greh.protocol = htons(proto);
    }
#[no_mangle]
unsafe extern "C" fn l3_length(family: c_int) -> c_int {
    static int l3_length(int family)
    {
    if (family == AF_INET)
    return sizeof(struct iphdr);
    else
    return sizeof(struct ipv6hdr);
    }
#[no_mangle]
unsafe extern "C" fn build_packet(test: *const test_configuration, sport: u16) -> c_int {
    static int build_packet(const struct test_configuration *test, uint16_t sport)
    {
    let mut ol3_len: c_int = 0, ol4_len = 0, il3_len = 0, il4_len = 0;
    let mut el3_len: c_int = 0, packet_len;
    memset(buf, 0, ETH_DATA_LEN);
    if (test.cfg_l3_extra)
    el3_len = l3_length(test.cfg_l3_extra);
// calculate header offsets
    if (test.cfg_encap_proto) {
    ol3_len = l3_length(test.cfg_l3_outer);
    if (test.cfg_encap_proto == IPPROTO_GRE)
    ol4_len = sizeof(struct grehdr);
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_UDP: test->cfg_encap_proto ==) -> else {
    else if (test.cfg_encap_proto == IPPROTO_UDP)
    ol4_len = sizeof(struct udphdr) + sizeof(struct guehdr);
    }
    il3_len = l3_length(test.cfg_l3_inner);
    il4_len = sizeof(struct udphdr);
    packet_len = el3_len + ol3_len + ol4_len + il3_len + il4_len +
    TEST_PACKET_LEN;
    if (!ASSERT_LE(packet_len, sizeof(buf), "check packet size"))
    return -1;
//
// Fill packet from inside out, to calculate correct checksums.
// But create ip before udp headers, as udp uses ip for pseudo-sum.
//
    memset(buf + el3_len + ol3_len + ol4_len + il3_len + il4_len,
    TEST_PACKET_PATTERN, TEST_PACKET_LEN);
// add zero byte for udp csum padding
    buf[el3_len + ol3_len + ol4_len + il3_len + il4_len + TEST_PACKET_LEN] =
    0;
    switch (test.cfg_l3_inner) {
    case PF_INET:
    build_ipv4_header(buf + el3_len + ol3_len + ol4_len,
    IPPROTO_UDP, test.in_saddr4.sin_addr.s_addr,
    test.in_daddr4.sin_addr.s_addr,
    il4_len + TEST_PACKET_LEN,
    test.cfg_dsfield_inner);
    break;
    case PF_INET6:
    build_ipv6_header(buf + el3_len + ol3_len + ol4_len,
    IPPROTO_UDP, &test.in_saddr6,
    &test.in_daddr6, il4_len + TEST_PACKET_LEN,
    test.cfg_dsfield_inner);
    break;
    }
    build_udp_header(buf + el3_len + ol3_len + ol4_len + il3_len,
    TEST_PACKET_LEN, sport, CFG_PORT_INNER,
    test.cfg_l3_inner);
    if (!test.cfg_encap_proto)
    return il3_len + il4_len + TEST_PACKET_LEN;
    switch (test.cfg_l3_outer) {
    case PF_INET:
    build_ipv4_header(buf + el3_len, test.cfg_encap_proto,
    test.out_saddr4.sin_addr.s_addr,
    test.out_daddr4.sin_addr.s_addr,
    ol4_len + il3_len + il4_len + TEST_PACKET_LEN,
    test.cfg_dsfield_outer);
    break;
    case PF_INET6:
    build_ipv6_header(buf + el3_len, test.cfg_encap_proto,
    &test.out_saddr6, &test.out_daddr6,
    ol4_len + il3_len + il4_len + TEST_PACKET_LEN,
    test.cfg_dsfield_outer);
    break;
    }
    switch (test.cfg_encap_proto) {
    case IPPROTO_UDP:
    build_gue_header(buf + el3_len + ol3_len + ol4_len -
    sizeof(struct guehdr),
    test.cfg_l3_inner == PF_INET ? IPPROTO_IPIP :
    IPPROTO_IPV6);
    build_udp_header(buf + el3_len + ol3_len,
    sizeof(struct guehdr) + il3_len + il4_len +
    TEST_PACKET_LEN,
    sport, CFG_PORT_GUE, test.cfg_l3_outer);
    break;
    case IPPROTO_GRE:
    build_gre_header(buf + el3_len + ol3_len,
    test.cfg_l3_inner == PF_INET ? ETH_P_IP :
    ETH_P_IPV6);
    break;
    }
    switch (test.cfg_l3_extra) {
    case PF_INET:
    build_ipv4_header(buf,
    test.cfg_l3_outer == PF_INET ? IPPROTO_IPIP :
    IPPROTO_IPV6,
    test.extra_saddr4.sin_addr.s_addr,
    test.extra_daddr4.sin_addr.s_addr,
    ol3_len + ol4_len + il3_len + il4_len +
    TEST_PACKET_LEN,
    0);
    break;
    case PF_INET6:
    build_ipv6_header(buf,
    test.cfg_l3_outer == PF_INET ? IPPROTO_IPIP :
    IPPROTO_IPV6,
    &test.extra_saddr6, &test.extra_daddr6,
    ol3_len + ol4_len + il3_len + il4_len +
    TEST_PACKET_LEN,
    0);
    break;
    }
    return el3_len + ol3_len + ol4_len + il3_len + il4_len +
    TEST_PACKET_LEN;
    }
// sender transmits encapsulated over RAW or unencap'd over UDP
#[no_mangle]
unsafe extern "C" fn setup_tx(test: *const test_configuration) -> c_int {
    static int setup_tx(const struct test_configuration *test)
    {
    int family, fd, ret;
    if (test.cfg_l3_extra)
    family = test.cfg_l3_extra;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: test->cfg_l3_outer) -> else {
    else if (test.cfg_l3_outer)
    family = test.cfg_l3_outer;
    else
    family = test.cfg_l3_inner;
    fd = socket(family, SOCK_RAW, IPPROTO_RAW);
    if (!ASSERT_OK_FD(fd, "setup tx socket"))
    return fd;
    if (test.cfg_l3_extra) {
    if (test.cfg_l3_extra == PF_INET)
    ret = connect(fd, (void *)&test.extra_daddr4,
    sizeof(test.extra_daddr4));
    else
    ret = connect(fd, (void *)&test.extra_daddr6,
    sizeof(test.extra_daddr6));
    if (!ASSERT_OK(ret, "connect")) {
    close(fd);
    return ret;
    }
    } else if (test.cfg_l3_outer) {
// connect to destination if not encapsulated
    if (test.cfg_l3_outer == PF_INET)
    ret = connect(fd, (void *)&test.out_daddr4,
    sizeof(test.out_daddr4));
    else
    ret = connect(fd, (void *)&test.out_daddr6,
    sizeof(test.out_daddr6));
    if (!ASSERT_OK(ret, "connect")) {
    close(fd);
    return ret;
    }
    } else {
// otherwise using loopback
    if (test.cfg_l3_inner == PF_INET)
    ret = connect(fd, (void *)&test.in_daddr4,
    sizeof(test.in_daddr4));
    else
    ret = connect(fd, (void *)&test.in_daddr6,
    sizeof(test.in_daddr6));
    if (!ASSERT_OK(ret, "connect")) {
    close(fd);
    return ret;
    }
    }
    return fd;
    }
// receiver reads unencapsulated UDP
#[no_mangle]
unsafe extern "C" fn setup_rx(test: *const test_configuration) -> c_int {
    static int setup_rx(const struct test_configuration *test)
    {
    int fd, ret;
    fd = socket(test.cfg_l3_inner, SOCK_DGRAM, 0);
    if (!ASSERT_OK_FD(fd, "socket rx"))
    return fd;
    if (test.cfg_l3_inner == PF_INET)
    ret = bind(fd, (void *)&test.in_daddr4,
    sizeof(test.in_daddr4));
    else
    ret = bind(fd, (void *)&test.in_daddr6,
    sizeof(test.in_daddr6));
    if (!ASSERT_OK(ret, "bind rx")) {
    close(fd);
    return ret;
    }
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn do_tx(fd: c_int, pkt: *const c_char, len: c_int) -> c_int {
    static int do_tx(int fd, const char *pkt, int len)
    {
    int ret;
    ret = write(fd, pkt, len);
    return ret != len;
    }
#[no_mangle]
unsafe extern "C" fn do_poll(fd: c_int, events: c_short, timeout: c_int) -> c_int {
    static int do_poll(int fd, short events, int timeout)
    {
    struct pollfd pfd;
    int ret;
    pfd.fd = fd;
    pfd.events = events;
    ret = poll(&pfd, 1, timeout);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_rx(fd: c_int) -> c_int {
    static int do_rx(int fd)
    {
    char rbuf;
    int ret, num = 0;
    while (1) {
    ret = recv(fd, &rbuf, 1, MSG_DONTWAIT);
    if (ret == -1 && errno == EAGAIN)
    break;
    if (ret < 0)
    return -1;
    if (!ASSERT_EQ(rbuf, TEST_PACKET_PATTERN, "check pkt pattern"))
    return -1;
    num++;
    }
    return num;
    }
    static int run_test(const struct test_configuration *test,
    int source_port_index)
    {
    let mut fdt: c_int = -1, fdr = -1, len, tx = 0, rx = 0, err;
    unsigned long tstop, tcur;
    fdr = setup_rx(test);
    fdt = setup_tx(test);
    if (!ASSERT_OK_FD(fdr, "setup rx") || !ASSERT_OK_FD(fdt, "setup tx")) {
    err = -1;
    goto out_close_sockets;
    }
    len = build_packet(test,
    (uint16_t)test.source_ports[source_port_index]);
    if (!ASSERT_GT(len, 0, "build test packet"))
    return -1;
    tcur = util_gettime();
    tstop = tcur;
    while (tx < TEST_PACKETS_COUNT) {
    if (!ASSERT_OK(do_tx(fdt, buf, len), "do_tx"))
    break;
    tx++;
    err = do_rx(fdr);
    if (!ASSERT_GE(err, 0, "do_rx"))
    break;
    rx += err;
    }
// read straggler packets, if any
    if (rx < tx) {
    tstop = util_gettime() + 100;
    while (rx < tx) {
    tcur = util_gettime();
    if (tcur >= tstop)
    break;
    err = do_poll(fdr, POLLIN, tstop - tcur);
    if (err < 0)
    break;
    err = do_rx(fdr);
    if (err >= 0)
    rx += err;
    }
    }
    out_close_sockets:
    close(fdt);
    close(fdr);
    return rx;
    }
#[no_mangle]
unsafe extern "C" fn attach_and_configure_program(skel: *mut bpf_flow) -> c_int {
    static int attach_and_configure_program(struct bpf_flow *skel)
    {
    struct bpf_map *prog_array = skel.maps.jmp_table;
    int main_prog_fd, sub_prog_fd, map_fd, i, err;
    struct bpf_program *prog;
    char prog_name[32];
    main_prog_fd = bpf_program__fd(skel.progs._dissect);
    if (main_prog_fd < 0)
    return main_prog_fd;
    err = bpf_prog_attach(main_prog_fd, 0, BPF_FLOW_DISSECTOR, 0);
    if (err)
    return err;
    map_fd = bpf_map__fd(prog_array);
    if (map_fd < 0)
    return map_fd;
    for (i = 0; i < bpf_map__max_entries(prog_array); i++) {
    snprintf(prog_name, sizeof(prog_name), "flow_dissector_%d", i);
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!prog)
    return -1;
    sub_prog_fd = bpf_program__fd(prog);
    if (sub_prog_fd < 0)
    return -1;
    err = bpf_map_update_elem(map_fd, &i, &sub_prog_fd, BPF_ANY);
    if (err)
    return -1;
    }
    return main_prog_fd;
    }
#[no_mangle]
unsafe extern "C" fn detach_program(skel: *mut bpf_flow, prog_fd: c_int) {
    static void detach_program(struct bpf_flow *skel, int prog_fd)
    {
    bpf_prog_detach2(prog_fd, 0, BPF_FLOW_DISSECTOR);
    }
#[no_mangle]
unsafe extern "C" fn set_port_drop(pf: c_int, multi_port: bool) -> c_int {
    static int set_port_drop(int pf, bool multi_port)
    {
    char dst_port[16];
    snprintf(dst_port, sizeof(dst_port), "%d", CFG_PORT_INNER);
    SYS(fail, "tc qdisc add dev lo ingress");
    SYS(fail_delete_qdisc, "tc filter add %s %s %s %s %s %s %s %s %s %s %s %s",
    "dev lo",
    "parent FFFF:",
    "protocol", pf == PF_INET6 ? "ipv6" : "ip",
    "pref 1337",
    "flower",
    "ip_proto udp",
    "src_port", multi_port ? "8-10" : "9",
    "dst_port", dst_port,
    "action drop");
    return 0;
    fail_delete_qdisc:
    SYS_NOFAIL("tc qdisc del dev lo ingress");
    fail:
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn remove_filter() {
    static void remove_filter(void)
    {
    SYS_NOFAIL("tc filter del dev lo ingress");
    SYS_NOFAIL("tc qdisc del dev lo ingress");
    }
#[no_mangle]
unsafe extern "C" fn ipv4_setup() -> c_int {
    static int ipv4_setup(void)
    {
    return set_port_drop(PF_INET, false);
    }
#[no_mangle]
unsafe extern "C" fn ipv6_setup() -> c_int {
    static int ipv6_setup(void)
    {
    return set_port_drop(PF_INET6, false);
    }
#[no_mangle]
unsafe extern "C" fn port_range_setup() -> c_int {
    static int port_range_setup(void)
    {
    return set_port_drop(PF_INET, true);
    }
#[no_mangle]
unsafe extern "C" fn set_addresses() -> c_int {
    static int set_addresses(void)
    {
    SYS(out, "ip -4 addr add  %s dev lo", TEST_IPV4);
    SYS(out_remove_ipv4, "ip -6 addr add %s dev lo", TEST_IPV6);
    return 0;
    out_remove_ipv4:
    SYS_NOFAIL("ip -4 addr del %s dev lo", TEST_IPV4);
    out:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn unset_addresses() {
    static void unset_addresses(void)
    {
    SYS_NOFAIL("ip -4 addr del %s dev lo", TEST_IPV4);
    SYS_NOFAIL("ip -6 addr del %s dev lo", TEST_IPV6);
    }
#[no_mangle]
unsafe extern "C" fn ipip_setup() -> c_int {
    static int ipip_setup(void)
    {
    if (!ASSERT_OK(set_addresses(), "configure addresses"))
    return -1;
    if (!ASSERT_OK(set_port_drop(PF_INET, false), "set filter"))
    goto out_unset_addresses;
    SYS(out_remove_filter,
    "ip link add ipip_test type ipip remote %s local %s dev lo",
    TEST_TUNNEL_REMOTE, TEST_TUNNEL_LOCAL);
    SYS(out_clean_netif, "ip link set ipip_test up");
    return 0;
    out_clean_netif:
    SYS_NOFAIL("ip link del ipip_test");
    out_remove_filter:
    remove_filter();
    out_unset_addresses:
    unset_addresses();
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn ipip_shutdown() {
    static void ipip_shutdown(void)
    {
    SYS_NOFAIL("ip link del ipip_test");
    remove_filter();
    unset_addresses();
    }
#[no_mangle]
unsafe extern "C" fn gre_setup() -> c_int {
    static int gre_setup(void)
    {
    if (!ASSERT_OK(set_addresses(), "configure addresses"))
    return -1;
    if (!ASSERT_OK(set_port_drop(PF_INET, false), "set filter"))
    goto out_unset_addresses;
    SYS(out_remove_filter,
    "ip link add gre_test type gre remote %s local %s dev lo",
    TEST_TUNNEL_REMOTE, TEST_TUNNEL_LOCAL);
    SYS(out_clean_netif, "ip link set gre_test up");
    return 0;
    out_clean_netif:
    SYS_NOFAIL("ip link del ipip_test");
    out_remove_filter:
    remove_filter();
    out_unset_addresses:
    unset_addresses();
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn gre_shutdown() {
    static void gre_shutdown(void)
    {
    SYS_NOFAIL("ip link del gre_test");
    remove_filter();
    unset_addresses();
    }
    static const struct test_configuration tests_input[] = {
    {
    .name = "ipv4",
    .test_setup = ipv4_setup,
    .test_teardown = remove_filter,
    .source_ports = { 8, 9, 10 },
    .cfg_l3_inner = PF_INET,
    .in_saddr4 = TEST_IN4_SRC_ADDR_DEFAULT,
    .in_daddr4 = TEST_IN4_DST_ADDR_DEFAULT
    },
    {
    .name = "ipv4_continue_dissect",
    .test_setup = ipv4_setup,
    .test_teardown = remove_filter,
    .source_ports = { 8, 9, 10 },
    .cfg_l3_inner = PF_INET,
    .in_saddr4 = TEST_IN4_SRC_ADDR_DISSECT_CONTINUE,
    .in_daddr4 = TEST_IN4_DST_ADDR_DEFAULT },
    {
    .name = "ipip",
    .test_setup = ipip_setup,
    .test_teardown = ipip_shutdown,
    .source_ports = { 8, 9, 10 },
    .cfg_l3_inner = PF_INET,
    .in_saddr4 = TEST_IN4_SRC_ADDR_IPIP,
    .in_daddr4 = TEST_IN4_DST_ADDR_IPIP,
    .out_saddr4 = TEST_OUT4_SRC_ADDR_DEFAULT,
    .out_daddr4 = TEST_OUT4_DST_ADDR_DEFAULT,
    .cfg_l3_outer = PF_INET,
    .cfg_encap_proto = IPPROTO_IPIP,
    },
    {
    .name = "gre",
    .test_setup = gre_setup,
    .test_teardown = gre_shutdown,
    .source_ports = { 8, 9, 10 },
    .cfg_l3_inner = PF_INET,
    .in_saddr4 = TEST_IN4_SRC_ADDR_IPIP,
    .in_daddr4 = TEST_IN4_DST_ADDR_IPIP,
    .out_saddr4 = TEST_OUT4_SRC_ADDR_DEFAULT,
    .out_daddr4 = TEST_OUT4_DST_ADDR_DEFAULT,
    .cfg_l3_outer = PF_INET,
    .cfg_encap_proto = IPPROTO_GRE,
    },
    {
    .name = "port_range",
    .test_setup = port_range_setup,
    .test_teardown = remove_filter,
    .source_ports = { 7, 9, 11 },
    .cfg_l3_inner = PF_INET,
    .in_saddr4 = TEST_IN4_SRC_ADDR_DEFAULT,
    .in_daddr4 = TEST_IN4_DST_ADDR_DEFAULT },
    {
    .name = "ipv6",
    .test_setup = ipv6_setup,
    .test_teardown = remove_filter,
    .source_ports = { 8, 9, 10 },
    .cfg_l3_inner = PF_INET6,
    .in_saddr6 = TEST_IN6_SRC_ADDR_DEFAULT,
    .in_daddr6 = TEST_IN6_DST_ADDR_DEFAULT
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_ctx {
    pub skel: *mut bpf_flow,
    pub ns: *mut netns_obj,
    pub prog_fd: c_int,
}

#[no_mangle]
unsafe extern "C" fn test_global_init(ctx: *mut test_ctx) -> c_int {
    static int test_global_init(struct test_ctx *ctx)
    {
    int err;
    ctx.skel = bpf_flow__open_and_load();
    if (!ASSERT_OK_PTR(ctx.skel, "open and load flow_dissector"))
    return -1;
    ctx.ns = netns_new("flow_dissector_classification", true);
    if (!ASSERT_OK_PTR(ctx.ns, "switch ns"))
    goto out_destroy_skel;
    err = write_sysctl("/proc/sys/net/ipv4/conf/default/rp_filter", "0");
    err |= write_sysctl("/proc/sys/net/ipv4/conf/all/rp_filter", "0");
    err |= write_sysctl("/proc/sys/net/ipv4/conf/lo/rp_filter", "0");
    if (!ASSERT_OK(err, "configure net tunables"))
    goto out_clean_ns;
    ctx.prog_fd = attach_and_configure_program(ctx.skel);
    if (!ASSERT_OK_FD(ctx.prog_fd, "attach and configure program"))
    goto out_clean_ns;
    return 0;
    out_clean_ns:
    netns_free(ctx.ns);
    out_destroy_skel:
    bpf_flow__destroy(ctx.skel);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn test_global_shutdown(ctx: *mut test_ctx) {
    static void test_global_shutdown(struct test_ctx *ctx)
    {
    detach_program(ctx.skel, ctx.prog_fd);
    netns_free(ctx.ns);
    bpf_flow__destroy(ctx.skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_flow_dissector_classification() {
    void test_flow_dissector_classification(void)
    {
    struct test_ctx ctx;
    const struct test_configuration *test;
    int i;
    if (test_global_init(&ctx))
    return;
    for (i = 0; i < ARRAY_SIZE(tests_input); i++) {
    if (!test__start_subtest(tests_input[i].name))
    continue;
    test = &tests_input[i];
// All tests are expected to have one rx-ok port first,
// then a non-working rx port, and finally a rx-ok port
//
    if (test.test_setup &&
    !ASSERT_OK(test.test_setup(), "init filter"))
    continue;
    ASSERT_EQ(run_test(test, 0), TEST_PACKETS_COUNT,
    "test first port");
    ASSERT_EQ(run_test(test, 1), 0, "test second port");
    ASSERT_EQ(run_test(test, 2), TEST_PACKETS_COUNT,
    "test third port");
    if (test.test_teardown)
    test.test_teardown();
    }
    test_global_shutdown(&ctx);
    }
