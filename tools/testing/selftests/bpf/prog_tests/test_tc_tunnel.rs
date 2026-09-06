//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_tc_tunnel.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// End-to-end eBPF tunnel test suite
// The file tests BPF network tunnels implementation. For each tunnel
// type, the test validates that:
// - basic communication can first be established between the two veths
// - when adding a BPF-based encapsulation on client egress, it now fails
// to communicate with the server
// - when adding a kernel-based decapsulation on server ingress, client
// can now connect
// - when replacing the kernel-based decapsulation with a BPF-based one,
// the client can still connect
//

pub const TEST_NAME_MAX_LEN: c_int = 64;
pub const PROG_NAME_MAX_LEN: c_int = 64;
pub const TUNNEL_ARGS_MAX_LEN: c_int = 128;
pub const BUFFER_LEN: c_int = 2000;
pub const DEFAULT_TEST_DATA_SIZE: c_int = 100;

pub const TIMEOUT_MS: c_int = 1000;
pub const TEST_PORT: c_int = 8000;
pub const UDP_PORT: c_int = 5555;
pub const MPLS_UDP_PORT: c_int = 6635;
pub const FOU_MPLS_PROTO: c_int = 137;
pub const VXLAN_ID: c_int = 1;
pub const VXLAN_PORT: c_int = 8472;
pub const MPLS_TABLE_ENTRIES_COUNT: c_int = 65536;
    static char tx_buffer[BUFFER_LEN], rx_buffer[BUFFER_LEN];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subtest_cfg {
    pub ebpf_tun_type: *mut c_char,
    pub iproute_tun_type: *mut c_char,
    pub mac_tun_type: *mut c_char,
    pub ipproto: c_int,
    pub dst): *mut *mut *mut void (extra_decap_mod_args_cb)(struct subtest_cfg cfg, char,
    pub tunnel_need_veth_mac: bool,
    pub configure_fou_rx_port: bool,
    pub tmode: *mut c_char,
    pub expect_kern_decap_failure: bool,
    pub configure_mpls: bool,
    pub test_gso: bool,
    pub tunnel_client_addr: *mut c_char,
    pub tunnel_server_addr: *mut c_char,
    pub name: [c_char; TEST_NAME_MAX_LEN],
    pub server_addr: *mut c_char,
    pub client_egress_prog_fd: c_int,
    pub server_ingress_prog_fd: c_int,
    pub extra_decap_mod_args: [c_char; TUNNEL_ARGS_MAX_LEN],
    pub server_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct connection {
    pub client_fd: c_int,
    pub server_fd: c_int,
}

#[no_mangle]
unsafe extern "C" fn build_subtest_name(cfg: *mut subtest_cfg, dst: *mut c_char, size: usize) -> c_int {
    static int build_subtest_name(struct subtest_cfg *cfg, char *dst, size_t size)
    {
    int ret;
    ret = snprintf(dst, size, "%s_%s", cfg.ebpf_tun_type,
    cfg.mac_tun_type);
    return ret < 0 ? ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn set_subtest_progs(cfg: *mut subtest_cfg, skel: *mut test_tc_tunnel) -> c_int {
    static int set_subtest_progs(struct subtest_cfg *cfg, struct test_tc_tunnel *skel)
    {
    char prog_name[PROG_NAME_MAX_LEN];
    struct bpf_program *prog;
    int ret;
    ret = snprintf(prog_name, PROG_NAME_MAX_LEN, "__encap_");
    if (ret < 0)
    return ret;
    ret = build_subtest_name(cfg, prog_name + ret, PROG_NAME_MAX_LEN - ret);
    if (ret < 0)
    return ret;
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!prog)
    return -1;
    cfg.client_egress_prog_fd = bpf_program__fd(prog);
    cfg.server_ingress_prog_fd = bpf_program__fd(skel.progs.decap_f);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_subtest_addresses(cfg: *mut subtest_cfg) {
    static void set_subtest_addresses(struct subtest_cfg *cfg)
    {
    if (cfg.ipproto == 6)
    cfg.server_addr = IP6_ADDR_VETH2;
    else
    cfg.server_addr = IP4_ADDR_VETH2;
// Some specific tunnel types need specific addressing, it then
// has been already set in the configuration table. Otherwise,
// deduce the relevant addressing from the ipproto
//
    if (cfg.tunnel_client_addr && cfg.tunnel_server_addr)
    return;
    if (cfg.ipproto == 6) {
    cfg.tunnel_client_addr = IP6_ADDR_VETH1;
    cfg.tunnel_server_addr = IP6_ADDR_VETH2;
    } else {
    cfg.tunnel_client_addr = IP4_ADDR_VETH1;
    cfg.tunnel_server_addr = IP4_ADDR_VETH2;
    }
    }
#[no_mangle]
unsafe extern "C" fn run_server(cfg: *mut subtest_cfg) -> c_int {
    static int run_server(struct subtest_cfg *cfg)
    {
    let mut family: c_int = cfg.ipproto == 6 ? AF_INET6 : AF_INET;
    struct nstoken *nstoken;
    struct network_helper_opts opts = {
    .timeout_ms = TIMEOUT_MS
    };
    nstoken = open_netns(SERVER_NS);
    if (!ASSERT_OK_PTR(nstoken, "open server ns"))
    return -1;
    cfg.server_fd = start_server_str(family, SOCK_STREAM, cfg.server_addr,
    TEST_PORT, &opts);
    close_netns(nstoken);
    if (!ASSERT_OK_FD(cfg.server_fd, "start server"))
    return -1;
    return 0;
    }
    static int check_server_rx_data(struct subtest_cfg *cfg,
    struct connection *conn, int len)
    {
    int err;
    memset(rx_buffer, 0, BUFFER_LEN);
    err = recv(conn.server_fd, rx_buffer, len, 0);
    if (!ASSERT_EQ(err, len, "check rx data len"))
    return 1;
    if (!ASSERT_MEMEQ(tx_buffer, rx_buffer, len, "check received data"))
    return 1;
    return 0;
    }
    static struct connection *connect_client_to_server(struct subtest_cfg *cfg)
    {
    let mut opts: network_helper_opts = {.timeout_ms = 1000};
    let mut family: c_int = cfg.ipproto == 6 ? AF_INET6 : AF_INET;
    struct connection *conn = core::ptr::null_mut();
    int client_fd, server_fd;
    conn = malloc(sizeof(struct connection));
    if (!conn)
    return conn;
    client_fd = connect_to_addr_str(family, SOCK_STREAM, cfg.server_addr,
    TEST_PORT, &opts);
    if (client_fd < 0) {
    free(conn);
    return core::ptr::null_mut();
    }
    server_fd = accept(cfg.server_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if (server_fd < 0) {
    close(client_fd);
    free(conn);
    return core::ptr::null_mut();
    }
    conn.server_fd = server_fd;
    conn.client_fd = client_fd;
    return conn;
    }
    static void disconnect_client_from_server(struct subtest_cfg *cfg,
    struct connection *conn)
    {
    close(conn.server_fd);
    close(conn.client_fd);
    free(conn);
    }
#[no_mangle]
unsafe extern "C" fn send_and_test_data(cfg: *mut subtest_cfg) -> c_int {
    static int send_and_test_data(struct subtest_cfg *cfg)
    {
    struct connection *conn;
    int err, res = -1;
    conn = connect_client_to_server(cfg);
    if (!ASSERT_OK_PTR(conn, "connect to server"))
    return -1;
    err = send(conn.client_fd, tx_buffer, DEFAULT_TEST_DATA_SIZE, 0);
    if (!ASSERT_EQ(err, DEFAULT_TEST_DATA_SIZE, "send data from client"))
    goto end;
    if (check_server_rx_data(cfg, conn, DEFAULT_TEST_DATA_SIZE))
    goto end;
    if (!cfg.test_gso) {
    res = 0;
    goto end;
    }
    err = send(conn.client_fd, tx_buffer, GSO_TEST_DATA_SIZE, 0);
    if (!ASSERT_EQ(err, GSO_TEST_DATA_SIZE, "send (large) data from client"))
    goto end;
    if (check_server_rx_data(cfg, conn, DEFAULT_TEST_DATA_SIZE))
    goto end;
    res = 0;
    end:
    disconnect_client_from_server(cfg, conn);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn vxlan_decap_mod_args_cb(cfg: *mut subtest_cfg, dst: *mut c_char) {
    static void vxlan_decap_mod_args_cb(struct subtest_cfg *cfg, char *dst)
    {
    snprintf(dst, TUNNEL_ARGS_MAX_LEN, "id %d dstport %d udp6zerocsumrx",
    VXLAN_ID, VXLAN_PORT);
    }
#[no_mangle]
unsafe extern "C" fn udp_decap_mod_args_cb(cfg: *mut subtest_cfg, dst: *mut c_char) {
    static void udp_decap_mod_args_cb(struct subtest_cfg *cfg, char *dst)
    {
    let mut is_mpls: bool = !strcmp(cfg.mac_tun_type, "mpls");
    snprintf(dst, TUNNEL_ARGS_MAX_LEN,
    "encap fou encap-sport auto encap-dport %d",
    is_mpls ? MPLS_UDP_PORT : UDP_PORT);
    }
#[no_mangle]
unsafe extern "C" fn configure_fou_rx_port(cfg: *mut subtest_cfg, add: bool) -> c_int {
    static int configure_fou_rx_port(struct subtest_cfg *cfg, bool add)
    {
    let mut is_mpls: bool = strcmp(cfg.mac_tun_type, "mpls") == 0;
    int fou_proto;
    if (is_mpls)
    fou_proto = FOU_MPLS_PROTO;
    else
    fou_proto = cfg.ipproto == 6 ? 41 : 4;
    SYS(fail, "ip fou %s port %d ipproto %d%s", add ? "add" : "del",
    is_mpls ? MPLS_UDP_PORT : UDP_PORT, fou_proto,
    cfg.ipproto == 6 ? " -6" : "");
    return 0;
    fail:
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn add_fou_rx_port(cfg: *mut subtest_cfg) -> c_int {
    static int add_fou_rx_port(struct subtest_cfg *cfg)
    {
    return configure_fou_rx_port(cfg, true);
    }
#[no_mangle]
unsafe extern "C" fn del_fou_rx_port(cfg: *mut subtest_cfg) -> c_int {
    static int del_fou_rx_port(struct subtest_cfg *cfg)
    {
    return configure_fou_rx_port(cfg, false);
    }
#[no_mangle]
unsafe extern "C" fn update_tunnel_intf_addr(cfg: *mut subtest_cfg) -> c_int {
    static int update_tunnel_intf_addr(struct subtest_cfg *cfg)
    {
    SYS(fail, "ip link set dev testtun0 address " MAC_ADDR_VETH2);
    return 0;
    fail:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn configure_kernel_for_mpls(cfg: *mut subtest_cfg) -> c_int {
    static int configure_kernel_for_mpls(struct subtest_cfg *cfg)
    {
    SYS(fail, "sysctl -qw net.mpls.platform_labels=%d",
    MPLS_TABLE_ENTRIES_COUNT);
    SYS(fail, "ip -f mpls route add 1000 dev lo");
    SYS(fail, "ip link set lo up");
    SYS(fail, "sysctl -qw net.mpls.conf.testtun0.input=1");
    SYS(fail, "sysctl -qw net.ipv4.conf.lo.rp_filter=0");
    return 0;
    fail:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn configure_encapsulation(cfg: *mut subtest_cfg) -> c_int {
    static int configure_encapsulation(struct subtest_cfg *cfg)
    {
    int ret;
    ret = tc_prog_attach("veth1", -1, cfg.client_egress_prog_fd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn configure_kernel_decapsulation(cfg: *mut subtest_cfg) -> c_int {
    static int configure_kernel_decapsulation(struct subtest_cfg *cfg)
    {
    struct nstoken *nstoken = open_netns(SERVER_NS);
    let mut ret: c_int = -1;
    if (!ASSERT_OK_PTR(nstoken, "open server ns"))
    return ret;
    if (cfg.configure_fou_rx_port &&
    !ASSERT_OK(add_fou_rx_port(cfg), "configure FOU RX port"))
    goto fail;
    SYS(fail, "ip link add name testtun0 type %s %s remote %s local %s %s",
    cfg.iproute_tun_type, cfg.tmode ? cfg.tmode : "",
    cfg.tunnel_client_addr, cfg.tunnel_server_addr,
    cfg.extra_decap_mod_args);
    if (cfg.tunnel_need_veth_mac &&
    !ASSERT_OK(update_tunnel_intf_addr(cfg), "update testtun0 mac"))
    goto fail;
    if (cfg.configure_mpls &&
    (!ASSERT_OK(configure_kernel_for_mpls(cfg),
    "configure MPLS decap")))
    goto fail;
    SYS(fail, "sysctl -qw net.ipv4.conf.all.rp_filter=0");
    SYS(fail, "sysctl -qw net.ipv4.conf.testtun0.rp_filter=0");
    SYS(fail, "ip link set dev testtun0 up");
    ret = 0;
    fail:
    close_netns(nstoken);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remove_kernel_decapsulation(cfg: *mut subtest_cfg) {
    static void remove_kernel_decapsulation(struct subtest_cfg *cfg)
    {
    SYS_NOFAIL("ip link del testtun0");
    if (cfg.configure_mpls)
    SYS_NOFAIL("ip -f mpls route del 1000 dev lo");
    if (cfg.configure_fou_rx_port)
    del_fou_rx_port(cfg);
    }
#[no_mangle]
unsafe extern "C" fn configure_ebpf_decapsulation(cfg: *mut subtest_cfg) -> c_int {
    static int configure_ebpf_decapsulation(struct subtest_cfg *cfg)
    {
    struct nstoken *nstoken = open_netns(SERVER_NS);
    let mut ret: c_int = -1;
    if (!ASSERT_OK_PTR(nstoken, "open server ns"))
    return ret;
    if (!cfg.expect_kern_decap_failure)
    SYS(fail, "ip link del testtun0");
    if (!ASSERT_OK(tc_prog_attach("veth2", cfg.server_ingress_prog_fd, -1),
    "attach_program"))
    goto fail;
    ret = 0;
    fail:
    close_netns(nstoken);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn run_test(cfg: *mut subtest_cfg) {
    static void run_test(struct subtest_cfg *cfg)
    {
    struct nstoken *nstoken;
    if (!ASSERT_OK(run_server(cfg), "run server"))
    return;
    nstoken = open_netns(CLIENT_NS);
    if (!ASSERT_OK_PTR(nstoken, "open client ns"))
    goto fail;
// Basic communication must work
    if (!ASSERT_OK(send_and_test_data(cfg), "connect without any encap"))
    goto fail;
// Attach encapsulation program to client
    if (!ASSERT_OK(configure_encapsulation(cfg), "configure encapsulation"))
    goto fail;
// If supported, insert kernel decap module, connection must succeed
    if (!cfg.expect_kern_decap_failure) {
    if (!ASSERT_OK(configure_kernel_decapsulation(cfg),
    "configure kernel decapsulation"))
    goto fail;
    if (!ASSERT_OK(send_and_test_data(cfg),
    "connect with encap prog and kern decap"))
    goto fail;
    }
// Replace kernel decapsulation with BPF decapsulation, test must pass
    if (!ASSERT_OK(configure_ebpf_decapsulation(cfg), "configure ebpf decapsulation"))
    goto fail;
    ASSERT_OK(send_and_test_data(cfg), "connect with encap and decap progs");
    fail:
    close_netns(nstoken);
    close(cfg.server_fd);
    }
#[no_mangle]
unsafe extern "C" fn setup() -> c_int {
    static int setup(void)
    {
    struct nstoken *nstoken_client, *nstoken_server;
    int fd, err;
    fd = open("/dev/urandom", O_RDONLY);
    if (!ASSERT_OK_FD(fd, "open urandom"))
    goto fail;
    err = read(fd, tx_buffer, BUFFER_LEN);
    close(fd);
    if (!ASSERT_EQ(err, BUFFER_LEN, "read random bytes"))
    goto fail;
// Configure the testing network
    if (!ASSERT_OK(make_netns(CLIENT_NS), "create client ns") ||
    !ASSERT_OK(make_netns(SERVER_NS), "create server ns"))
    goto fail;
    nstoken_client = open_netns(CLIENT_NS);
    if (!ASSERT_OK_PTR(nstoken_client, "open client ns"))
    goto fail_delete_ns;
    SYS(fail_close_ns_client, "ip link add %s type veth peer name %s",
    "veth1 mtu 1500 netns " CLIENT_NS " address " MAC_ADDR_VETH1,
    "veth2 mtu 1500 netns " SERVER_NS " address " MAC_ADDR_VETH2);
    SYS(fail_close_ns_client, "ip link set veth1 up");
    nstoken_server = open_netns(SERVER_NS);
    if (!ASSERT_OK_PTR(nstoken_server, "open server ns"))
    goto fail_close_ns_client;
    SYS(fail_close_ns_server, "ip link set veth2 up");
    close_netns(nstoken_server);
    close_netns(nstoken_client);
    return 0;
    fail_close_ns_server:
    close_netns(nstoken_server);
    fail_close_ns_client:
    close_netns(nstoken_client);
    fail_delete_ns:
    SYS_NOFAIL("ip netns del " CLIENT_NS);
    SYS_NOFAIL("ip netns del " SERVER_NS);
    fail:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn subtest_setup(skel: *mut test_tc_tunnel, cfg: *mut subtest_cfg) -> c_int {
    static int subtest_setup(struct test_tc_tunnel *skel, struct subtest_cfg *cfg)
    {
    struct nstoken *nstoken_client, *nstoken_server;
    let mut ret: c_int = -1;
    set_subtest_addresses(cfg);
    if (!ASSERT_OK(set_subtest_progs(cfg, skel),
    "find subtest progs"))
    goto fail;
    if (cfg.extra_decap_mod_args_cb)
    cfg.extra_decap_mod_args_cb(cfg, cfg.extra_decap_mod_args);
    nstoken_client = open_netns(CLIENT_NS);
    if (!ASSERT_OK_PTR(nstoken_client, "open client ns"))
    goto fail;
    SYS(fail_close_client_ns,
    "ip -4 addr add " IP4_ADDR_VETH1 "/24 dev veth1");
    SYS(fail_close_client_ns, "ip -4 route flush table main");
    SYS(fail_close_client_ns,
    "ip -4 route add " IP4_ADDR_VETH2 " mtu 1450 dev veth1");
    SYS(fail_close_client_ns,
    "ip -6 addr add " IP6_ADDR_VETH1 "/64 dev veth1 nodad");
    SYS(fail_close_client_ns, "ip -6 route flush table main");
    SYS(fail_close_client_ns,
    "ip -6 route add " IP6_ADDR_VETH2 " mtu 1430 dev veth1");
    nstoken_server = open_netns(SERVER_NS);
    if (!ASSERT_OK_PTR(nstoken_server, "open server ns"))
    goto fail_close_client_ns;
    SYS(fail_close_server_ns,
    "ip -4 addr add " IP4_ADDR_VETH2 "/24 dev veth2");
    SYS(fail_close_server_ns,
    "ip -6 addr add " IP6_ADDR_VETH2 "/64 dev veth2 nodad");
    ret = 0;
    fail_close_server_ns:
    close_netns(nstoken_server);
    fail_close_client_ns:
    close_netns(nstoken_client);
    fail:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn subtest_cleanup(cfg: *mut subtest_cfg) {
    static void subtest_cleanup(struct subtest_cfg *cfg)
    {
    struct nstoken *nstoken;
    nstoken = open_netns(CLIENT_NS);
    if (ASSERT_OK_PTR(nstoken, "open clien ns")) {
    SYS_NOFAIL("tc qdisc delete dev veth1 parent ffff:fff1");
    SYS_NOFAIL("ip a flush veth1");
    close_netns(nstoken);
    }
    nstoken = open_netns(SERVER_NS);
    if (ASSERT_OK_PTR(nstoken, "open clien ns")) {
    SYS_NOFAIL("tc qdisc delete dev veth2 parent ffff:fff1");
    SYS_NOFAIL("ip a flush veth2");
    if (!cfg.expect_kern_decap_failure)
    remove_kernel_decapsulation(cfg);
    close_netns(nstoken);
    }
    }
#[no_mangle]
unsafe extern "C" fn cleanup() {
    static void cleanup(void)
    {
    remove_netns(CLIENT_NS);
    remove_netns(SERVER_NS);
    }
    static struct subtest_cfg subtests_cfg[] = {
    {
    .ebpf_tun_type = "ipip",
    .mac_tun_type = "none",
    .iproute_tun_type = "ipip",
    .ipproto = 4,
    },
    {
    .ebpf_tun_type = "ipip6",
    .mac_tun_type = "none",
    .iproute_tun_type = "ip6tnl",
    .ipproto = 4,
    .tunnel_client_addr = IP6_ADDR_VETH1,
    .tunnel_server_addr = IP6_ADDR_VETH2,
    },
    {
    .ebpf_tun_type = "ip6tnl",
    .iproute_tun_type = "ip6tnl",
    .mac_tun_type = "none",
    .ipproto = 6,
    },
    {
    .mac_tun_type = "none",
    .ebpf_tun_type = "sit",
    .iproute_tun_type = "sit",
    .ipproto = 6,
    .tunnel_client_addr = IP4_ADDR_VETH1,
    .tunnel_server_addr = IP4_ADDR_VETH2,
    },
    {
    .ebpf_tun_type = "vxlan",
    .mac_tun_type = "eth",
    .iproute_tun_type = "vxlan",
    .ipproto = 4,
    .extra_decap_mod_args_cb = vxlan_decap_mod_args_cb,
    .tunnel_need_veth_mac = true
    },
    {
    .ebpf_tun_type = "ip6vxlan",
    .mac_tun_type = "eth",
    .iproute_tun_type = "vxlan",
    .ipproto = 6,
    .extra_decap_mod_args_cb = vxlan_decap_mod_args_cb,
    .tunnel_need_veth_mac = true
    },
    {
    .ebpf_tun_type = "gre",
    .mac_tun_type = "none",
    .iproute_tun_type = "gre",
    .ipproto = 4,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "gre",
    .mac_tun_type = "eth",
    .iproute_tun_type = "gretap",
    .ipproto = 4,
    .tunnel_need_veth_mac = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "gre",
    .mac_tun_type = "mpls",
    .iproute_tun_type = "gre",
    .ipproto = 4,
    .configure_mpls = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "ip6gre",
    .mac_tun_type = "none",
    .iproute_tun_type = "ip6gre",
    .ipproto = 6,
    .test_gso = true,
    },
    {
    .ebpf_tun_type = "ip6gre",
    .mac_tun_type = "eth",
    .iproute_tun_type = "ip6gretap",
    .ipproto = 6,
    .tunnel_need_veth_mac = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "ip6gre",
    .mac_tun_type = "mpls",
    .iproute_tun_type = "ip6gre",
    .ipproto = 6,
    .configure_mpls = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "udp",
    .mac_tun_type = "none",
    .iproute_tun_type = "ipip",
    .ipproto = 4,
    .extra_decap_mod_args_cb = udp_decap_mod_args_cb,
    .configure_fou_rx_port = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "udp",
    .mac_tun_type = "eth",
    .iproute_tun_type = "ipip",
    .ipproto = 4,
    .extra_decap_mod_args_cb = udp_decap_mod_args_cb,
    .configure_fou_rx_port = true,
    .expect_kern_decap_failure = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "udp",
    .mac_tun_type = "mpls",
    .iproute_tun_type = "ipip",
    .ipproto = 4,
    .extra_decap_mod_args_cb = udp_decap_mod_args_cb,
    .configure_fou_rx_port = true,
    .tmode = "mode any ttl 255",
    .configure_mpls = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "ip6udp",
    .mac_tun_type = "none",
    .iproute_tun_type = "ip6tnl",
    .ipproto = 6,
    .extra_decap_mod_args_cb = udp_decap_mod_args_cb,
    .configure_fou_rx_port = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "ip6udp",
    .mac_tun_type = "eth",
    .iproute_tun_type = "ip6tnl",
    .ipproto = 6,
    .extra_decap_mod_args_cb = udp_decap_mod_args_cb,
    .configure_fou_rx_port = true,
    .expect_kern_decap_failure = true,
    .test_gso = true
    },
    {
    .ebpf_tun_type = "ip6udp",
    .mac_tun_type = "mpls",
    .iproute_tun_type = "ip6tnl",
    .ipproto = 6,
    .extra_decap_mod_args_cb = udp_decap_mod_args_cb,
    .configure_fou_rx_port = true,
    .tmode = "mode any ttl 255",
    .expect_kern_decap_failure = true,
    .test_gso = true
    },
    };
#[no_mangle]
pub unsafe extern "C" fn test_tc_tunnel() {
    void test_tc_tunnel(void)
    {
    struct test_tc_tunnel *skel;
    struct subtest_cfg *cfg;
    int i, ret;
    skel = test_tc_tunnel__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel open and load"))
    return;
    if (!ASSERT_OK(setup(), "global setup"))
    goto out;
    for (i = 0; i < ARRAY_SIZE(subtests_cfg); i++) {
    cfg = &subtests_cfg[i];
    ret = build_subtest_name(cfg, cfg.name, TEST_NAME_MAX_LEN);
    if (ret < 0 || !test__start_subtest(cfg.name))
    continue;
    if (subtest_setup(skel, cfg) == 0)
    run_test(cfg);
    subtest_cleanup(cfg);
    }
    cleanup();
    out:
    test_tc_tunnel__destroy(skel);
    }
