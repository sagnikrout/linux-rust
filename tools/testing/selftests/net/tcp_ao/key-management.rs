//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tcp_ao/key-management.c
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
// Author: Dmitry Safonov <dima@arista.com>

    let mut nr_packets: usize = 20;
    let mut msg_len: usize = 100;
    let mut quota: usize = nr_packets * msg_len;
    union tcp_addr wrong_addr;

    let mut test_vrf_ifindex: static int = 200;
    let mut test_vrf_tabid: static uint8_t = 42;
#[no_mangle]
unsafe extern "C" fn setup_vrfs() {
    static void setup_vrfs(void)
    {
    int err;
    if (!kernel_config_has(KCONFIG_NET_VRF))
    return;
    err = add_vrf("ksft-vrf", test_vrf_tabid, test_vrf_ifindex, -1);
    if (err)
    test_error("Failed to add a VRF: %d", err);
    err = link_set_up("ksft-vrf");
    if (err)
    test_error("Failed to bring up a VRF");
    err = ip_route_add_vrf(veth_name, TEST_FAMILY,
    this_ip_addr, this_ip_dest, test_vrf_tabid);
    if (err)
    test_error("Failed to add a route to VRF");
    }
#[no_mangle]
unsafe extern "C" fn prepare_sk(addr: *mut union tcp_addr, sndid: u8, rcvid: u8) -> c_int {
    static int prepare_sk(union tcp_addr *addr, uint8_t sndid, uint8_t rcvid)
    {
    let mut sk: c_int = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if (sk < 0)
    test_error("socket()");
    if (test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest,
    DEFAULT_TEST_PREFIX, 100, 100))
    test_error("test_add_key()");
    if (addr && test_add_key(sk, SECOND_PASSWORD, *addr,
    DEFAULT_TEST_PREFIX, sndid, rcvid))
    test_error("test_add_key()");
    return sk;
    }
#[no_mangle]
unsafe extern "C" fn prepare_lsk(addr: *mut union tcp_addr, sndid: u8, rcvid: u8) -> c_int {
    static int prepare_lsk(union tcp_addr *addr, uint8_t sndid, uint8_t rcvid)
    {
    let mut sk: c_int = prepare_sk(addr, sndid, rcvid);
    if (listen(sk, 10))
    test_error("listen()");
    return sk;
    }
    static int test_del_key(int sk, uint8_t sndid, uint8_t rcvid, int ifindex,
    bool async, int current_key, int rnext_key)
    {
    let mut ao_info: tcp_ao_info_opt = {};
    let mut key: tcp_ao_getsockopt = {};
    let mut del: tcp_ao_del = {};
    sockaddr_af sockaddr;
    int err;
    tcp_addr_to_sockaddr_in(&del.addr, &this_ip_dest, 0);
    del.prefix = DEFAULT_TEST_PREFIX;
    del.sndid = sndid;
    del.rcvid = rcvid;
    if (ifindex) {
    del.keyflags = TCP_AO_KEYF_IFINDEX;
    del.ifindex = ifindex;
    }
    if (current_key >= 0) {
    del.set_current = 1;
    del.current_key = (uint8_t)current_key;
    }
    if (rnext_key >= 0) {
    del.set_rnext = 1;
    del.rnext = (uint8_t)rnext_key;
    }
    err = setsockopt(sk, IPPROTO_TCP, TCP_AO_DEL_KEY, &del, sizeof(del));
    if (err < 0)
    return -errno;
    if (async)
    return 0;
    tcp_addr_to_sockaddr_in(&sockaddr, &this_ip_dest, 0);
    err = test_get_one_ao(sk, &key, &sockaddr, sizeof(sockaddr),
    DEFAULT_TEST_PREFIX, sndid, rcvid,
    del.keyflags, del.ifindex);
    if (!err)
    return -EEXIST;
    if (err != -E2BIG)
    test_error("getsockopt()");
    if (current_key < 0 && rnext_key < 0)
    return 0;
    if (test_get_ao_info(sk, &ao_info))
    test_error("getsockopt(TCP_AO_INFO) failed");
    if (current_key >= 0 && ao_info.current_key != (uint8_t)current_key)
    return -ENOTRECOVERABLE;
    if (rnext_key >= 0 && ao_info.rnext != (uint8_t)rnext_key)
    return -ENOTRECOVERABLE;
    return 0;
    }
    static void try_delete_key(char *tst_name, int sk, uint8_t sndid, uint8_t rcvid,
    int ifindex, bool async, int current_key, int rnext_key,
    fault_t inj)
    {
    int err;
    err = test_del_key(sk, sndid, rcvid, ifindex, async, current_key, rnext_key);
    if ((err == -EBUSY && fault(BUSY)) || (err == -EINVAL && fault(CURRNEXT))) {
    test_ok("%s: key deletion was prevented", tst_name);
    return;
    }
    if (err && fault(FIXME)) {
    test_xfail("%s: failed to delete the key %u:%u %d",
    tst_name, sndid, rcvid, err);
    return;
    }
    if (!err) {
    if (fault(BUSY) || fault(CURRNEXT)) {
    test_fail("%s: the key was deleted %u:%u %d", tst_name,
    sndid, rcvid, err);
    } else {
    test_ok("%s: the key was deleted", tst_name);
    }
    return;
    }
    test_fail("%s: can't delete the key %u:%u %d", tst_name, sndid, rcvid, err);
    }
#[no_mangle]
unsafe extern "C" fn test_set_key(sk: c_int, current_keyid: c_int, rnext_keyid: c_int) -> c_int {
    static int test_set_key(int sk, int current_keyid, int rnext_keyid)
    {
    let mut ao_info: tcp_ao_info_opt = {};
    int err;
    if (current_keyid >= 0) {
    ao_info.set_current = 1;
    ao_info.current_key = (uint8_t)current_keyid;
    }
    if (rnext_keyid >= 0) {
    ao_info.set_rnext = 1;
    ao_info.rnext = (uint8_t)rnext_keyid;
    }
    err = test_set_ao_info(sk, &ao_info);
    if (err)
    return err;
    if (test_get_ao_info(sk, &ao_info))
    test_error("getsockopt(TCP_AO_INFO) failed");
    if (current_keyid >= 0 && ao_info.current_key != (uint8_t)current_keyid)
    return -ENOTRECOVERABLE;
    if (rnext_keyid >= 0 && ao_info.rnext != (uint8_t)rnext_keyid)
    return -ENOTRECOVERABLE;
    return 0;
    }
    static int test_add_current_rnext_key(int sk, const char *key, uint8_t keyflags,
    union tcp_addr in_addr, uint8_t prefix,
    bool set_current, bool set_rnext,
    uint8_t sndid, uint8_t rcvid)
    {
    let mut tmp: tcp_ao_add = {};
    int err;
    err = test_prepare_key(&tmp, DEFAULT_TEST_ALGO, in_addr,
    set_current, set_rnext,
    prefix, 0, sndid, rcvid, 0, keyflags,
    strlen(key), key);
    if (err)
    return err;
    err = setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &tmp, sizeof(tmp));
    if (err < 0)
    return -errno;
    return test_verify_socket_key(sk, &tmp);
    }
    static int __try_add_current_rnext_key(int sk, const char *key, uint8_t keyflags,
    union tcp_addr in_addr, uint8_t prefix,
    bool set_current, bool set_rnext,
    uint8_t sndid, uint8_t rcvid)
    {
    let mut ao_info: tcp_ao_info_opt = {};
    int err;
    err = test_add_current_rnext_key(sk, key, keyflags, in_addr, prefix,
    set_current, set_rnext, sndid, rcvid);
    if (err)
    return err;
    if (test_get_ao_info(sk, &ao_info))
    test_error("getsockopt(TCP_AO_INFO) failed");
    if (set_current && ao_info.current_key != sndid)
    return -ENOTRECOVERABLE;
    if (set_rnext && ao_info.rnext != rcvid)
    return -ENOTRECOVERABLE;
    return 0;
    }
    static void try_add_current_rnext_key(char *tst_name, int sk, const char *key,
    uint8_t keyflags,
    union tcp_addr in_addr, uint8_t prefix,
    bool set_current, bool set_rnext,
    uint8_t sndid, uint8_t rcvid, fault_t inj)
    {
    int err;
    err = __try_add_current_rnext_key(sk, key, keyflags, in_addr, prefix,
    set_current, set_rnext, sndid, rcvid);
    if (!err && !fault(CURRNEXT)) {
    test_ok("%s", tst_name);
    return;
    }
    if (err == -EINVAL && fault(CURRNEXT)) {
    test_ok("%s", tst_name);
    return;
    }
    test_fail("%s", tst_name);
    }
#[no_mangle]
unsafe extern "C" fn check_closed_socket() {
    static void check_closed_socket(void)
    {
    int sk;
    sk = prepare_sk(&this_ip_dest, 200, 200);
    try_delete_key("closed socket, delete a key", sk, 200, 200, 0, 0, -1, -1, 0);
    try_delete_key("closed socket, delete all keys", sk, 100, 100, 0, 0, -1, -1, 0);
    close(sk);
    sk = prepare_sk(&this_ip_dest, 200, 200);
    if (test_set_key(sk, 100, 200))
    test_error("failed to set current/rnext keys");
    try_delete_key("closed socket, delete current key", sk, 100, 100, 0, 0, -1, -1, FAULT_BUSY);
    try_delete_key("closed socket, delete rnext key", sk, 200, 200, 0, 0, -1, -1, FAULT_BUSY);
    close(sk);
    sk = prepare_sk(&this_ip_dest, 200, 200);
    if (test_add_key(sk, "Glory to heros!", this_ip_dest,
    DEFAULT_TEST_PREFIX, 10, 11))
    test_error("test_add_key()");
    if (test_add_key(sk, "Glory to Ukraine!", this_ip_dest,
    DEFAULT_TEST_PREFIX, 12, 13))
    test_error("test_add_key()");
    try_delete_key("closed socket, delete a key + set current/rnext", sk,
    100, 100, 0, 0, 10, 13, 0);
    try_delete_key("closed socket, force-delete current key", sk, 10, 11, 0, 0, 200, -1, 0);
    try_delete_key("closed socket, force-delete rnext key", sk, 12, 13, 0, 0, -1, 200, 0);
    try_delete_key("closed socket, delete current+rnext key", sk,
    200, 200, 0, 0, -1, -1, FAULT_BUSY);
    close(sk);
    sk = prepare_sk(&this_ip_dest, 200, 200);
    if (test_set_key(sk, 100, 200))
    test_error("failed to set current/rnext keys");
    try_add_current_rnext_key("closed socket, add + change current key",
    sk, "Laaaa! Lalala-la-la-lalala...", 0,
    this_ip_dest, DEFAULT_TEST_PREFIX,
    true, false, 10, 20, 0);
    try_add_current_rnext_key("closed socket, add + change rnext key",
    sk, "Laaaa! Lalala-la-la-lalala...", 0,
    this_ip_dest, DEFAULT_TEST_PREFIX,
    false, true, 20, 10, 0);
    close(sk);
    if (!should_skip_test("closed socket, add + delete VRF-scoped key",
    KCONFIG_NET_VRF)) {
    sk = prepare_sk(&this_ip_dest, 200, 200);
    if (test_add_key_vrf(sk, SECOND_PASSWORD, TCP_AO_KEYF_IFINDEX,
    this_ip_dest, DEFAULT_TEST_PREFIX,
    test_vrf_ifindex, 201, 201))
    test_error("test_add_key_vrf()");
    try_delete_key("closed socket, add + delete VRF-scoped key", sk, 201, 201,
    test_vrf_ifindex, 0, -1, -1, 0);
    close(sk);
    }
    }
#[no_mangle]
unsafe extern "C" fn assert_no_current_rnext(tst_msg: *const c_char, sk: c_int) {
    static void assert_no_current_rnext(const char *tst_msg, int sk)
    {
    let mut ao_info: tcp_ao_info_opt = {};
    if (test_get_ao_info(sk, &ao_info))
    test_error("getsockopt(TCP_AO_INFO) failed");
    errno = 0;
    if (ao_info.set_current || ao_info.set_rnext) {
    test_xfail("%s: the socket has current/rnext keys: %d:%d",
    tst_msg,
    (ao_info.set_current) ? ao_info.current_key : -1,
    (ao_info.set_rnext) ? ao_info.rnext : -1);
    } else {
    test_ok("%s: the socket has no current/rnext keys", tst_msg);
    }
    }
#[no_mangle]
unsafe extern "C" fn assert_no_tcp_repair() {
    static void assert_no_tcp_repair(void)
    {
    let mut ao_img: tcp_ao_repair = {};
    let mut len: socklen_t = sizeof(ao_img);
    int sk, err;
    sk = prepare_sk(&this_ip_dest, 200, 200);
    test_enable_repair(sk);
    if (listen(sk, 10))
    test_error("listen()");
    errno = 0;
    err = getsockopt(sk, SOL_TCP, TCP_AO_REPAIR, &ao_img, &len);
    if (err && errno == EPERM)
    test_ok("listen socket, getsockopt(TCP_AO_REPAIR) is restricted");
    else
    test_fail("listen socket, getsockopt(TCP_AO_REPAIR) works");
    errno = 0;
    err = setsockopt(sk, SOL_TCP, TCP_AO_REPAIR, &ao_img, sizeof(ao_img));
    if (err && errno == EPERM)
    test_ok("listen socket, setsockopt(TCP_AO_REPAIR) is restricted");
    else
    test_fail("listen socket, setsockopt(TCP_AO_REPAIR) works");
    close(sk);
    }
#[no_mangle]
unsafe extern "C" fn check_listen_socket() {
    static void check_listen_socket(void)
    {
    int sk, err;
    sk = prepare_lsk(&this_ip_dest, 200, 200);
    try_delete_key("listen socket, delete a key", sk, 200, 200, 0, 0, -1, -1, 0);
    try_delete_key("listen socket, delete all keys", sk, 100, 100, 0, 0, -1, -1, 0);
    close(sk);
    sk = prepare_lsk(&this_ip_dest, 200, 200);
    err = test_set_key(sk, 100, -1);
    if (err == -EINVAL)
    test_ok("listen socket, setting current key not allowed");
    else
    test_fail("listen socket, set current key");
    err = test_set_key(sk, -1, 200);
    if (err == -EINVAL)
    test_ok("listen socket, setting rnext key not allowed");
    else
    test_fail("listen socket, set rnext key");
    close(sk);
    sk = prepare_sk(&this_ip_dest, 200, 200);
    if (test_set_key(sk, 100, 200))
    test_error("failed to set current/rnext keys");
    if (listen(sk, 10))
    test_error("listen()");
    assert_no_current_rnext("listen() after current/rnext keys set", sk);
    try_delete_key("listen socket, delete current key from before listen()", sk,
    100, 100, 0, 0, -1, -1, FAULT_FIXME);
    try_delete_key("listen socket, delete rnext key from before listen()", sk,
    200, 200, 0, 0, -1, -1, FAULT_FIXME);
    close(sk);
    assert_no_tcp_repair();
    sk = prepare_lsk(&this_ip_dest, 200, 200);
    if (test_add_key(sk, "Glory to heros!", this_ip_dest,
    DEFAULT_TEST_PREFIX, 10, 11))
    test_error("test_add_key()");
    if (test_add_key(sk, "Glory to Ukraine!", this_ip_dest,
    DEFAULT_TEST_PREFIX, 12, 13))
    test_error("test_add_key()");
    try_delete_key("listen socket, delete a key + set current/rnext", sk,
    100, 100, 0, 0, 10, 13, FAULT_CURRNEXT);
    try_delete_key("listen socket, force-delete current key", sk,
    10, 11, 0, 0, 200, -1, FAULT_CURRNEXT);
    try_delete_key("listen socket, force-delete rnext key", sk,
    12, 13, 0, 0, -1, 200, FAULT_CURRNEXT);
    try_delete_key("listen socket, delete a key", sk,
    200, 200, 0, 0, -1, -1, 0);
    close(sk);
    sk = prepare_lsk(&this_ip_dest, 200, 200);
    try_add_current_rnext_key("listen socket, add + change current key",
    sk, "Laaaa! Lalala-la-la-lalala...", 0,
    this_ip_dest, DEFAULT_TEST_PREFIX,
    true, false, 10, 20, FAULT_CURRNEXT);
    try_add_current_rnext_key("listen socket, add + change rnext key",
    sk, "Laaaa! Lalala-la-la-lalala...", 0,
    this_ip_dest, DEFAULT_TEST_PREFIX,
    false, true, 20, 10, FAULT_CURRNEXT);
    close(sk);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_key {
    pub password: [c_char; TCP_AO_MAXKEYLEN],
    pub alg: *const c_char,
    pub len: c_uint,
    pub client_keyid: u8,
    pub server_keyid: u8,
    pub maclen: u8,
    uint8_t matches_client		: 1,
    matches_server		: 1,
    matches_vrf		: 1,
    is_current		: 1,
    is_rnext		: 1,
    used_on_server_tx	: 1,
    used_on_client_tx	: 1,
    pub 1: skip_counters_checks :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_collection {
    pub nr_keys: c_uint,
    pub keys: *mut test_key,
}

    static struct key_collection collection;
pub const TEST_MAX_MACLEN: c_int = 16;
    const char *test_algos[] = { "cmac(aes128)", "hmac(sha1)", "hmac(sha256)" };
    const unsigned int test_maclens[] = { 1, 4, 12, 16 };
pub const MACLEN_SHIFT: c_int = 2;
pub const ALGOS_SHIFT: c_int = 4;
#[no_mangle]
unsafe extern "C" fn make_mask(shift: c_uint, prev_shift: c_uint) -> c_uint {
    static unsigned int make_mask(unsigned int shift, unsigned int prev_shift)
    {
    let mut ret: c_uint = BIT(shift) - 1;
    return ret << prev_shift;
    }
#[no_mangle]
unsafe extern "C" fn init_key_in_collection(index: c_uint, randomized: bool) {
    static void init_key_in_collection(unsigned int index, bool randomized)
    {
    struct test_key *key = &collection.keys[index];
    unsigned int algos_index;
// Same for randomized and non-randomized test flows
    key.client_keyid = index;
    key.server_keyid = 127 + index;
    key.matches_client = 1;
    key.matches_server = 1;
    key.matches_vrf = 1;
// not really even random, but good enough for a test
    key.len = rand() % (TCP_AO_MAXKEYLEN - TEST_TCP_AO_MINKEYLEN);
    key.len += TEST_TCP_AO_MINKEYLEN;
    randomize_buffer(key.password, key.len);
    if (randomized) {
    key.maclen = (rand() % TEST_MAX_MACLEN) + 1;
    algos_index = rand();
    } else {
    let mut shift: c_uint = MACLEN_SHIFT;
    key.maclen = test_maclens[index & make_mask(shift, 0)];
    algos_index = index & make_mask(ALGOS_SHIFT, shift);
    }
    key.alg = test_algos[algos_index % ARRAY_SIZE(test_algos)];
    }
#[no_mangle]
unsafe extern "C" fn init_default_key_collection(nr_keys: c_uint, randomized: bool) -> c_int {
    static int init_default_key_collection(unsigned int nr_keys, bool randomized)
    {
    let mut key_sz: usize = sizeof(collection.keys[0]);
    if (!nr_keys) {
    free(collection.keys);
    collection.keys = core::ptr::null_mut();
    return 0;
    }
//
// All keys have uniq sndid/rcvid and sndid != rcvid in order to
// check for any bugs/issues for different keyids, visible to both
// peers. Keyid == 254 is unused.
//
    if (nr_keys > 127)
    test_error("Test requires too many keys, correct the source");
    collection.keys = reallocarray(collection.keys, nr_keys, key_sz);
    if (!collection.keys)
    return -ENOMEM;
    memset(collection.keys, 0, nr_keys * key_sz);
    collection.nr_keys = nr_keys;
    while (nr_keys--)
    init_key_in_collection(nr_keys, randomized);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_key_error(msg: *const c_char, key: *mut test_key) {
    static void test_key_error(const char *msg, struct test_key *key)
    {
    test_error("%s: key: { %s, %u:%u, %u, %u:%u:%u:%u:%u (%u)}",
    msg, key.alg, key.client_keyid, key.server_keyid,
    key.maclen, key.matches_client, key.matches_server,
    key.matches_vrf, key.is_current, key.is_rnext, key.len);
    }
    static int test_add_key_cr(int sk, const char *pwd, unsigned int pwd_len,
    union tcp_addr addr, uint8_t vrf,
    uint8_t sndid, uint8_t rcvid,
    uint8_t maclen, const char *alg,
    bool set_current, bool set_rnext)
    {
    let mut tmp: tcp_ao_add = {};
    let mut keyflags: u8 = 0;
    int err;
    if (!alg)
    alg = DEFAULT_TEST_ALGO;
    if (vrf)
    keyflags |= TCP_AO_KEYF_IFINDEX;
    err = test_prepare_key(&tmp, alg, addr, set_current, set_rnext,
    DEFAULT_TEST_PREFIX, vrf, sndid, rcvid, maclen,
    keyflags, pwd_len, pwd);
    if (err)
    return err;
    err = setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &tmp, sizeof(tmp));
    if (err < 0)
    return -errno;
    return test_verify_socket_key(sk, &tmp);
    }
    static void verify_current_rnext(const char *tst, int sk,
    int current_keyid, int rnext_keyid)
    {
    let mut ao_info: tcp_ao_info_opt = {};
    if (test_get_ao_info(sk, &ao_info))
    test_error("getsockopt(TCP_AO_INFO) failed");
    errno = 0;
    if (current_keyid >= 0) {
    if (!ao_info.set_current)
    test_fail("%s: the socket doesn't have current key", tst);
#[no_mangle]
pub unsafe extern "C" fn if(current_keyid: ao_info.current_key !=) -> else {
    else if (ao_info.current_key != current_keyid)
    test_fail("%s: current key is not the expected one %d != %u",
    tst, current_keyid, ao_info.current_key);
    else
    test_ok("%s: current key %u as expected",
    tst, ao_info.current_key);
    }
    if (rnext_keyid >= 0) {
    if (!ao_info.set_rnext)
    test_fail("%s: the socket doesn't have rnext key", tst);
#[no_mangle]
pub unsafe extern "C" fn if(rnext_keyid: ao_info.rnext !=) -> else {
    else if (ao_info.rnext != rnext_keyid)
    test_fail("%s: rnext key is not the expected one %d != %u",
    tst, rnext_keyid, ao_info.rnext);
    else
    test_ok("%s: rnext key %u as expected", tst, ao_info.rnext);
    }
    }
#[no_mangle]
unsafe extern "C" fn key_collection_socket(server: bool, port: c_uint) -> c_int {
    static int key_collection_socket(bool server, unsigned int port)
    {
    unsigned int i;
    int sk;
    if (server)
    sk = test_listen_socket(this_ip_addr, port, 1);
    else
    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if (sk < 0)
    test_error("socket()");
    for (i = 0; i < collection.nr_keys; i++) {
    struct test_key *key = &collection.keys[i];
    union tcp_addr *addr = &wrong_addr;
    uint8_t sndid, rcvid, vrf;
    let mut set_current: bool = false, set_rnext = false;
    if (key.matches_vrf)
    vrf = 0;
    else
    vrf = test_vrf_ifindex;
    if (server) {
    if (key.matches_client)
    addr = &this_ip_dest;
    sndid = key.server_keyid;
    rcvid = key.client_keyid;
    } else {
    if (key.matches_server)
    addr = &this_ip_dest;
    sndid = key.client_keyid;
    rcvid = key.server_keyid;
    key.used_on_client_tx = set_current = key.is_current;
    key.used_on_server_tx = set_rnext = key.is_rnext;
    }
    if (test_add_key_cr(sk, key.password, key.len,
// addr, vrf, sndid, rcvid, key->maclen,
    key.alg, set_current, set_rnext))
    test_key_error("setsockopt(TCP_AO_ADD_KEY)", key);

    test_print("%s [%u/%u] key: { %s, %u:%u, %u, %u:%u:%u:%u (%u)}",
    server ? "server" : "client", i, collection.nr_keys,
    key.alg, rcvid, sndid, key.maclen,
    key.matches_client, key.matches_server,
    key.is_current, key.is_rnext, key.len);

    }
    return sk;
    }
    static void verify_counters(const char *tst_name, bool is_listen_sk, bool server,
    struct tcp_counters *a, struct tcp_counters *b)
    {
    unsigned int i;
    test_assert_counters_sk(tst_name, a, b, TEST_CNT_GOOD);
    for (i = 0; i < collection.nr_keys; i++) {
    struct test_key *key = &collection.keys[i];
    uint8_t sndid, rcvid;
    bool rx_cnt_expected;
    if (key.skip_counters_checks)
    continue;
    if (server) {
    sndid = key.server_keyid;
    rcvid = key.client_keyid;
    rx_cnt_expected = key.used_on_client_tx;
    } else {
    sndid = key.client_keyid;
    rcvid = key.server_keyid;
    rx_cnt_expected = key.used_on_server_tx;
    }
    test_assert_counters_key(tst_name, &a.ao, &b.ao,
    rx_cnt_expected ? TEST_CNT_KEY_GOOD : 0,
    sndid, rcvid);
    }
    test_tcp_counters_free(a);
    test_tcp_counters_free(b);
    test_ok("%s: passed counters checks", tst_name);
    }
    static struct tcp_ao_getsockopt *lookup_key(struct tcp_ao_getsockopt *buf,
    size_t len, int sndid, int rcvid)
    {
    size_t i;
    for (i = 0; i < len; i++) {
    if (sndid >= 0 && buf[i].sndid != sndid)
    continue;
    if (rcvid >= 0 && buf[i].rcvid != rcvid)
    continue;
    return &buf[i];
    }
    return core::ptr::null_mut();
    }
    static void verify_keys(const char *tst_name, int sk,
    bool is_listen_sk, bool server)
    {
    let mut len: socklen_t = sizeof(struct tcp_ao_getsockopt);
    struct tcp_ao_getsockopt *keys;
    let mut passed_test: bool = true;
    unsigned int i;
    keys = calloc(collection.nr_keys, len);
    if (!keys)
    test_error("calloc()");
    keys.nkeys = collection.nr_keys;
    keys.get_all = 1;
    if (getsockopt(sk, IPPROTO_TCP, TCP_AO_GET_KEYS, keys, &len)) {
    free(keys);
    test_error("getsockopt(TCP_AO_GET_KEYS)");
    }
    for (i = 0; i < collection.nr_keys; i++) {
    struct test_key *key = &collection.keys[i];
    struct tcp_ao_getsockopt *dump_key;
    let mut is_kdf_aes_128_cmac: bool = false;
    let mut is_cmac_aes: bool = false;
    uint8_t sndid, rcvid;
    let mut matches: bool = false;
    if (server) {
    if (key.matches_client)
    matches = true;
    sndid = key.server_keyid;
    rcvid = key.client_keyid;
    } else {
    if (key.matches_server)
    matches = true;
    sndid = key.client_keyid;
    rcvid = key.server_keyid;
    }
    if (!key.matches_vrf)
    matches = false;
// no keys get removed on the original listener socket
    if (is_listen_sk)
    matches = true;
    dump_key = lookup_key(keys, keys.nkeys, sndid, rcvid);
    if (matches != !!dump_key) {
    test_fail("%s: key %u:%u %s%s on the socket",
    tst_name, sndid, rcvid,
    key.matches_vrf ? "" : "[vrf] ",
    matches ? "disappeared" : "yet present");
    passed_test = false;
    goto out;
    }
    if (!dump_key)
    continue;
    if (!strcmp("cmac(aes128)", key.alg)) {
    is_kdf_aes_128_cmac = (key.len != 16);
    is_cmac_aes = true;
    }
    if (is_cmac_aes) {
    if (strcmp(dump_key.alg_name, "cmac(aes)")) {
    test_fail("%s: key %u:%u cmac(aes) has unexpected alg %s",
    tst_name, sndid, rcvid,
    dump_key.alg_name);
    passed_test = false;
    continue;
    }
    } else if (strcmp(dump_key.alg_name, key.alg)) {
    test_fail("%s: key %u:%u has unexpected alg %s != %s",
    tst_name, sndid, rcvid,
    dump_key.alg_name, key.alg);
    passed_test = false;
    continue;
    }
    if (is_kdf_aes_128_cmac) {
    if (dump_key.keylen != 16) {
    test_fail("%s: key %u:%u cmac(aes128) has unexpected len %u",
    tst_name, sndid, rcvid,
    dump_key.keylen);
    continue;
    }
    } else if (dump_key.keylen != key.len) {
    test_fail("%s: key %u:%u changed password len %u != %u",
    tst_name, sndid, rcvid,
    dump_key.keylen, key.len);
    passed_test = false;
    continue;
    }
    if (!is_kdf_aes_128_cmac &&
    memcmp(dump_key.key, key.password, key.len)) {
    test_fail("%s: key %u:%u has different password",
    tst_name, sndid, rcvid);
    passed_test = false;
    continue;
    }
    if (dump_key.maclen != key.maclen) {
    test_fail("%s: key %u:%u changed maclen %u != %u",
    tst_name, sndid, rcvid,
    dump_key.maclen, key.maclen);
    passed_test = false;
    continue;
    }
    }
    if (passed_test)
    test_ok("%s: The socket keys are consistent with the expectations",
    tst_name);
    out:
    free(keys);
    }
    static int start_server(const char *tst_name, unsigned int port, size_t quota,
    struct tcp_counters *begin,
    unsigned int current_index, unsigned int rnext_index)
    {
    struct tcp_counters lsk_c1, lsk_c2;
    ssize_t bytes;
    int sk, lsk;
    synchronize_threads(); /* 1: key collection initialized */
    lsk = key_collection_socket(true, port);
    if (test_get_tcp_counters(lsk, &lsk_c1))
    test_error("test_get_tcp_counters()");
    synchronize_threads(); /* 2: MKTs added => connect() */
    if (test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0))
    test_error("test_wait_fd()");
    sk = accept(lsk, core::ptr::null_mut(), core::ptr::null_mut());
    if (sk < 0)
    test_error("accept()");
    if (test_get_tcp_counters(sk, begin))
    test_error("test_get_tcp_counters()");
    synchronize_threads(); /* 3: accepted => send data */
    if (test_get_tcp_counters(lsk, &lsk_c2))
    test_error("test_get_tcp_counters()");
    verify_keys(tst_name, lsk, true, true);
    close(lsk);
    bytes = test_server_run(sk, quota, TEST_TIMEOUT_SEC);
    if (bytes != quota)
    test_fail("%s: server served: %zd", tst_name, bytes);
    else
    test_ok("%s: server alive", tst_name);
    verify_counters(tst_name, true, true, &lsk_c1, &lsk_c2);
    return sk;
    }
    static void end_server(const char *tst_name, int sk,
    struct tcp_counters *begin)
    {
    struct tcp_counters end;
    if (test_get_tcp_counters(sk, &end))
    test_error("test_get_tcp_counters()");
    verify_keys(tst_name, sk, false, true);
    synchronize_threads(); /* 4: verified => closed */
    close(sk);
    verify_counters(tst_name, false, true, begin, &end);
    synchronize_threads(); /* 5: counters */
    }
    static void try_server_run(const char *tst_name, unsigned int port, size_t quota,
    unsigned int current_index, unsigned int rnext_index)
    {
    struct tcp_counters tmp;
    int sk;
    sk = start_server(tst_name, port, quota, &tmp,
    current_index, rnext_index);
    end_server(tst_name, sk, &tmp);
    }
    static void server_rotations(const char *tst_name, unsigned int port,
    size_t quota, unsigned int rotations,
    unsigned int current_index, unsigned int rnext_index)
    {
    struct tcp_counters tmp;
    unsigned int i;
    int sk;
    sk = start_server(tst_name, port, quota, &tmp,
    current_index, rnext_index);
    for (i = current_index + 1; rotations > 0; i++, rotations--) {
    ssize_t bytes;
    if (i >= collection.nr_keys)
    i = 0;
    bytes = test_server_run(sk, quota, TEST_TIMEOUT_SEC);
    if (bytes != quota) {
    test_fail("%s: server served: %zd", tst_name, bytes);
    return;
    }
    verify_current_rnext(tst_name, sk,
    collection.keys[i].server_keyid, -1);
    synchronize_threads(); /* verify current/rnext */
    }
    end_server(tst_name, sk, &tmp);
    }
    static int run_client(const char *tst_name, unsigned int port,
    unsigned int nr_keys, int current_index, int rnext_index,
    struct tcp_counters *before,
    const size_t msg_sz, const size_t msg_nr)
    {
    int sk;
    synchronize_threads(); /* 1: key collection initialized */
    sk = key_collection_socket(false, port);
    if (current_index >= 0 || rnext_index >= 0) {
    let mut sndid: c_int = -1, rcvid = -1;
    if (current_index >= 0)
    sndid = collection.keys[current_index].client_keyid;
    if (rnext_index >= 0)
    rcvid = collection.keys[rnext_index].server_keyid;
    if (test_set_key(sk, sndid, rcvid))
    test_error("failed to set current/rnext keys");
    }
    if (before && test_get_tcp_counters(sk, before))
    test_error("test_get_tcp_counters()");
    synchronize_threads(); /* 2: MKTs added => connect() */
    if (test_connect_socket(sk, this_ip_dest, port++) <= 0)
    test_error("failed to connect()");
    if (current_index < 0)
    current_index = nr_keys - 1;
    if (rnext_index < 0)
    rnext_index = nr_keys - 1;
    collection.keys[current_index].used_on_client_tx = 1;
    collection.keys[rnext_index].used_on_server_tx = 1;
    synchronize_threads(); /* 3: accepted => send data */
    if (test_client_verify(sk, msg_sz, msg_nr)) {
    test_fail("verify failed");
    close(sk);
    if (before)
    test_tcp_counters_free(before);
    return -1;
    }
    return sk;
    }
    static int start_client(const char *tst_name, unsigned int port,
    unsigned int nr_keys, int current_index, int rnext_index,
    struct tcp_counters *before,
    const size_t msg_sz, const size_t msg_nr)
    {
    if (init_default_key_collection(nr_keys, true))
    test_error("Failed to init the key collection");
    return run_client(tst_name, port, nr_keys, current_index,
    rnext_index, before, msg_sz, msg_nr);
    }
    static void end_client(const char *tst_name, int sk, unsigned int nr_keys,
    int current_index, int rnext_index,
    struct tcp_counters *start)
    {
    struct tcp_counters end;
// Some application may become dependent on this kernel choice
    if (current_index < 0)
    current_index = nr_keys - 1;
    if (rnext_index < 0)
    rnext_index = nr_keys - 1;
    verify_current_rnext(tst_name, sk,
    collection.keys[current_index].client_keyid,
    collection.keys[rnext_index].server_keyid);
    if (start && test_get_tcp_counters(sk, &end))
    test_error("test_get_tcp_counters()");
    verify_keys(tst_name, sk, false, false);
    synchronize_threads(); /* 4: verify => closed */
    close(sk);
    if (start)
    verify_counters(tst_name, false, false, start, &end);
    synchronize_threads(); /* 5: counters */
    }
#[no_mangle]
unsafe extern "C" fn try_unmatched_keys(sk: c_int, rnext_index: *mut c_int, port: c_uint) {
    static void try_unmatched_keys(int sk, int *rnext_index, unsigned int port)
    {
    struct test_key *key;
    let mut i: c_uint = 0;
    int err;
    do {
    key = &collection.keys[i];
    if (!key.matches_server)
    break;
    } while (++i < collection.nr_keys);
    if (key.matches_server)
    test_error("all keys on client match the server");
    err = test_add_key_cr(sk, key.password, key.len, wrong_addr,
    0, key.client_keyid, key.server_keyid,
    key.maclen, key.alg, 0, 0);
    if (!err) {
    test_fail("Added a key with non-matching ip-address for established sk");
    return;
    }
    if (err == -EINVAL)
    test_ok("Can't add a key with non-matching ip-address for established sk");
    else
    test_error("Failed to add a key");
    err = test_add_key_cr(sk, key.password, key.len, this_ip_dest,
    test_vrf_ifindex,
    key.client_keyid, key.server_keyid,
    key.maclen, key.alg, 0, 0);
    if (!err) {
    test_fail("Added a key with non-matching VRF for established sk");
    return;
    }
    if (err == -EINVAL)
    test_ok("Can't add a key with non-matching VRF for established sk");
    else
    test_error("Failed to add a key");
    for (i = 0; i < collection.nr_keys; i++) {
    key = &collection.keys[i];
    if (!key.matches_client)
    break;
    }
    if (key.matches_client)
    test_error("all keys on server match the client");
    if (test_set_key(sk, -1, key.server_keyid))
    test_error("Can't change the current key");
    trace_ao_event_expect(TCP_AO_RNEXT_REQUEST, this_ip_addr, this_ip_dest,
    -1, port, 0, -1, -1, -1, -1, -1,
    -1, key.server_keyid, -1);
    if (test_client_verify(sk, msg_len, nr_packets))
    test_fail("verify failed");
// rnext_index = i;
    }
    static int client_non_matching(const char *tst_name, unsigned int port,
    unsigned int nr_keys,
    int current_index, int rnext_index,
    const size_t msg_sz, const size_t msg_nr)
    {
    unsigned int i;
    if (init_default_key_collection(nr_keys, true))
    test_error("Failed to init the key collection");
    for (i = 0; i < nr_keys; i++) {
// key (0, 0) matches
    collection.keys[i].matches_client = !!((i + 3) % 4);
    collection.keys[i].matches_server = !!((i + 2) % 4);
    if (kernel_config_has(KCONFIG_NET_VRF))
    collection.keys[i].matches_vrf = !!((i + 1) % 4);
    }
    return run_client(tst_name, port, nr_keys, current_index,
    rnext_index, core::ptr::null_mut(), msg_sz, msg_nr);
    }
    static void check_current_back(const char *tst_name, unsigned int port,
    unsigned int nr_keys,
    unsigned int current_index, unsigned int rnext_index,
    unsigned int rotate_to_index)
    {
    struct tcp_counters tmp;
    int sk;
    sk = start_client(tst_name, port, nr_keys, current_index, rnext_index,
    &tmp, msg_len, nr_packets);
    if (sk < 0)
    return;
    if (test_set_key(sk, collection.keys[rotate_to_index].client_keyid, -1))
    test_error("Can't change the current key");
    trace_ao_event_expect(TCP_AO_RNEXT_REQUEST, this_ip_dest, this_ip_addr,
    port, -1, 0, -1, -1, -1, -1, -1,
    collection.keys[rotate_to_index].client_keyid,
    collection.keys[current_index].client_keyid, -1);
    if (test_client_verify(sk, msg_len, nr_packets))
    test_fail("verify failed");
// There is a race here: between setting the current_key with
// setsockopt(TCP_AO_INFO) and starting to send some data - there
// might have been a segment received with the desired
// RNext_key set. In turn that would mean that the first outgoing
// segment will have the desired current_key (flipped back).
// Which is what the user/test wants. As it's racy, skip checking
// the counters, yet check what are the resulting current/rnext
// keys on both sides.
//
    collection.keys[rotate_to_index].skip_counters_checks = 1;
    end_client(tst_name, sk, nr_keys, current_index, rnext_index, &tmp);
    }
    static void roll_over_keys(const char *tst_name, unsigned int port,
    unsigned int nr_keys, unsigned int rotations,
    unsigned int current_index, unsigned int rnext_index)
    {
    struct tcp_counters tmp;
    unsigned int i;
    int sk;
    sk = start_client(tst_name, port, nr_keys, current_index, rnext_index,
    &tmp, msg_len, nr_packets);
    if (sk < 0)
    return;
    for (i = rnext_index + 1; rotations > 0; i++, rotations--) {
    if (i >= collection.nr_keys)
    i = 0;
    trace_ao_event_expect(TCP_AO_RNEXT_REQUEST,
    this_ip_addr, this_ip_dest,
    -1, port, 0, -1, -1, -1, -1, -1,
    i == 0 ? -1 : collection.keys[i - 1].server_keyid,
    collection.keys[i].server_keyid, -1);
    if (test_set_key(sk, -1, collection.keys[i].server_keyid))
    test_error("Can't change the Rnext key");
    if (test_client_verify(sk, msg_len, nr_packets)) {
    test_fail("verify failed");
    close(sk);
    test_tcp_counters_free(&tmp);
    return;
    }
    verify_current_rnext(tst_name, sk, -1,
    collection.keys[i].server_keyid);
    collection.keys[i].used_on_server_tx = 1;
    synchronize_threads(); /* verify current/rnext */
    }
    end_client(tst_name, sk, nr_keys, current_index, rnext_index, &tmp);
    }
    static void try_client_run(const char *tst_name, unsigned int port,
    unsigned int nr_keys, int current_index, int rnext_index)
    {
    struct tcp_counters tmp;
    int sk;
    sk = start_client(tst_name, port, nr_keys, current_index, rnext_index,
    &tmp, msg_len, nr_packets);
    if (sk < 0)
    return;
    end_client(tst_name, sk, nr_keys, current_index, rnext_index, &tmp);
    }
    static void try_client_match(const char *tst_name, unsigned int port,
    unsigned int nr_keys,
    int current_index, int rnext_index)
    {
    int sk;
    sk = client_non_matching(tst_name, port, nr_keys, current_index,
    rnext_index, msg_len, nr_packets);
    if (sk < 0)
    return;
    try_unmatched_keys(sk, &rnext_index, port);
    end_client(tst_name, sk, nr_keys, current_index, rnext_index, core::ptr::null_mut());
    }
    static void *server_fn(void *arg)
    {
    let mut port: c_uint = test_server_port;
    setup_vrfs();
    try_server_run("server: Check current/rnext keys unset before connect()",
    port++, quota, 19, 19);
    try_server_run("server: Check current/rnext keys set before connect()",
    port++, quota, 10, 10);
    try_server_run("server: Check current != rnext keys set before connect()",
    port++, quota, 5, 10);
    try_server_run("server: Check current flapping back on peer's RnextKey request",
    port++, quota * 2, 5, 10);
    server_rotations("server: Rotate over all different keys", port++,
    quota, 20, 0, 0);
    try_server_run("server: Check accept() => established key matching",
    port++, quota * 2, 0, 0);
    synchronize_threads(); /* don't race to exit: client exits */
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn check_established_socket() {
    static void check_established_socket(void)
    {
    let mut port: c_uint = test_server_port;
    try_client_run("client: Check current/rnext keys unset before connect()",
    port++, 20, -1, -1);
    try_client_run("client: Check current/rnext keys set before connect()",
    port++, 20, 10, 10);
    try_client_run("client: Check current != rnext keys set before connect()",
    port++, 20, 10, 5);
    check_current_back("client: Check current flapping back on peer's RnextKey request",
    port++, 20, 10, 5, 2);
    roll_over_keys("client: Rotate over all different keys", port++,
    20, 20, 0, 0);
    try_client_match("client: Check connect() => established key matching",
    port++, 20, 0, 0);
    }
    static void *client_fn(void *arg)
    {
    if (inet_pton(TEST_FAMILY, TEST_WRONG_IP, &wrong_addr) != 1)
    test_error("Can't convert ip address %s", TEST_WRONG_IP);
    setup_vrfs();
    check_closed_socket();
    check_listen_socket();
    check_established_socket();
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    test_init(122, server_fn, client_fn);
    return 0;
    }
