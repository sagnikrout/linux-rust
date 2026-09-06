//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/icmp_rfc4884.c
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

    let mut src_port: static unsigned short = 44444;
    let mut dst_port: static unsigned short = 55555;
    let mut min_orig_dgram_len: static int = 128;
    static const int min_payload_len_v4 =
    min_orig_dgram_len - sizeof(struct iphdr) - sizeof(struct udphdr);
    static const int min_payload_len_v6 =
    min_orig_dgram_len - sizeof(struct ipv6hdr) - sizeof(struct udphdr);
    let mut orig_payload_byte: static uint8_t = 0xAA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_inet {
    union {
    pub v6: sockaddr_in6,
    pub v4: sockaddr_in,
    pub sa: sockaddr,
}

    socklen_t len;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_case_info {
    pub domain: c_int,
    pub level: c_int,
    pub opt1: c_int,
    pub opt2: c_int,
    pub proto: c_int,
    int	(*build_func)(uint8_t *buf, ssize_t buflen, bool with_ext,
    int payload_len, bool bad_csum, bool bad_len,
    pub smaller_len): bool,
    pub min_payload: c_int,
}

#[no_mangle]
unsafe extern "C" fn bringup_loopback() -> c_int {
    static int bringup_loopback(void)
    {
    struct ifreq ifr = {
    .ifr_name = "lo"
    };
    int fd;
    fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (fd < 0)
    return -1;
    if (ioctl(fd, SIOCGIFFLAGS, &ifr) < 0)
    goto err;
    ifr.ifr_flags = ifr.ifr_flags | IFF_UP;
    if (ioctl(fd, SIOCSIFFLAGS, &ifr) < 0)
    goto err;
    close(fd);
    return 0;
    err:
    close(fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn csum(buf: *const c_void, len: usize) -> u16 {
    static uint16_t csum(const void *buf, size_t len)
    {
    const uint8_t *data = buf;
    let mut sum: u32 = 0;
    while (len > 1) {
    sum += (data[0] << 8) | data[1];
    data += 2;
    len -= 2;
    }
    if (len == 1)
    sum += data[0] << 8;
    while (sum >> 16)
    sum = (sum & 0xFFFF) + (sum >> 16);
    return ~sum & 0xFFFF;
    }
#[no_mangle]
unsafe extern "C" fn poll_err(fd: c_int) -> c_int {
    static int poll_err(int fd)
    {
    struct pollfd pfd;
    memset(&pfd, 0, sizeof(pfd));
    pfd.fd = fd;
    if (poll(&pfd, 1, 5000) != 1 || pfd.revents != POLLERR)
    return -1;
    return 0;
    }
    static void set_addr(struct sockaddr_inet *addr, int domain,
    unsigned short port)
    {
    memset(addr, 0, sizeof(*addr));
    switch (domain) {
    case AF_INET:
    addr.v4.sin_family = AF_INET;
    addr.v4.sin_port = htons(port);
    addr.v4.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr.len = sizeof(addr.v4);
    break;
    case AF_INET6:
    addr.v6.sin6_family = AF_INET6;
    addr.v6.sin6_port = htons(port);
    addr.v6.sin6_addr = in6addr_loopback;
    addr.len = sizeof(addr.v6);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn bind_and_setsockopt(fd: c_int, info: *const ip_case_info) -> c_int {
    static int bind_and_setsockopt(int fd, const struct ip_case_info *info)
    {
    struct sockaddr_inet addr;
    let mut opt: c_int = 1;
    set_addr(&addr, info.domain, src_port);
    if (setsockopt(fd, info.level, info.opt1, &opt, sizeof(opt)) < 0)
    return -1;
    if (setsockopt(fd, info.level, info.opt2, &opt, sizeof(opt)) < 0)
    return -1;
    return bind(fd, &addr.sa, addr.len);
    }
    static int build_rfc4884_ext(uint8_t *buf, size_t buflen, bool bad_csum,
    bool bad_len, bool smaller_len)
    {
    struct icmp_extobj_hdr *objh;
    struct icmp_ext_hdr *exthdr;
    size_t obj_len, ext_len;
    uint16_t sum;
// Use an object payload of 4 bytes
    obj_len = sizeof(*objh) + sizeof(uint32_t);
    ext_len = sizeof(*exthdr) + obj_len;
    if (ext_len > buflen)
    return -EINVAL;
    exthdr = (struct icmp_ext_hdr *)buf;
    objh = (struct icmp_extobj_hdr *)(buf + sizeof(*exthdr));
    exthdr.version = 2;
// When encoding a bad object length, either encode a length too small
// to fit the object header or too big to fit in the packet.
//
    if (bad_len)
    obj_len = smaller_len ? sizeof(*objh) - 1 : obj_len * 2;
    objh.length = htons(obj_len);
    sum = csum(buf, ext_len);
    exthdr.checksum = htons(bad_csum ? sum - 1 : sum);
    return ext_len;
    }
#[no_mangle]
unsafe extern "C" fn build_orig_dgram_v4(buf: *mut u8, buflen: isize, payload_len: c_int) -> c_int {
    static int build_orig_dgram_v4(uint8_t *buf, ssize_t buflen, int payload_len)
    {
    struct udphdr *udph;
    struct iphdr *iph;
    let mut len: usize = 0;
    len = sizeof(*iph) + sizeof(*udph) + payload_len;
    if (len > buflen)
    return -EINVAL;
    iph = (struct iphdr *)buf;
    udph = (struct udphdr *)(buf + sizeof(*iph));
    iph.version = 4;
    iph.ihl = 5;
    iph.protocol = IPPROTO_UDP;
    iph.saddr = htonl(INADDR_LOOPBACK);
    iph.daddr = htonl(INADDR_LOOPBACK);
    iph.tot_len = htons(len);
    iph.check = htons(csum(iph, sizeof(*iph)));
    udph.source = htons(src_port);
    udph.dest = htons(dst_port);
    udph.len = htons(sizeof(*udph) + payload_len);
    memset(buf + sizeof(*iph) + sizeof(*udph), orig_payload_byte,
    payload_len);
    return len;
    }
#[no_mangle]
unsafe extern "C" fn build_orig_dgram_v6(buf: *mut u8, buflen: isize, payload_len: c_int) -> c_int {
    static int build_orig_dgram_v6(uint8_t *buf, ssize_t buflen, int payload_len)
    {
    struct udphdr *udph;
    struct ipv6hdr *iph;
    let mut len: usize = 0;
    len = sizeof(*iph) + sizeof(*udph) + payload_len;
    if (len > buflen)
    return -EINVAL;
    iph = (struct ipv6hdr *)buf;
    udph = (struct udphdr *)(buf + sizeof(*iph));
    iph.version = 6;
    iph.payload_len = htons(sizeof(*udph) + payload_len);
    iph.nexthdr = IPPROTO_UDP;
    iph.saddr = in6addr_loopback;
    iph.daddr = in6addr_loopback;
    udph.source = htons(src_port);
    udph.dest = htons(dst_port);
    udph.len = htons(sizeof(*udph) + payload_len);
    memset(buf + sizeof(*iph) + sizeof(*udph), orig_payload_byte,
    payload_len);
    return len;
    }
    static int build_icmpv4_pkt(uint8_t *buf, ssize_t buflen, bool with_ext,
    int payload_len, bool bad_csum, bool bad_len,
    bool smaller_len)
    {
    struct icmphdr *icmph;
    int len, ret;
    len = sizeof(*icmph);
    memset(buf, 0, buflen);
    icmph = (struct icmphdr *)buf;
    icmph.type = ICMP_DEST_UNREACH;
    icmph.code = ICMP_PORT_UNREACH;
    icmph.checksum = 0;
    ret = build_orig_dgram_v4(buf + len, buflen - len, payload_len);
    if (ret < 0)
    return ret;
    len += ret;
    icmph.un.reserved[1] = (len - sizeof(*icmph)) / sizeof(uint32_t);
    if (with_ext) {
    ret = build_rfc4884_ext(buf + len, buflen - len,
    bad_csum, bad_len, smaller_len);
    if (ret < 0)
    return ret;
    len += ret;
    }
    icmph.checksum = htons(csum(icmph, len));
    return len;
    }
    static int build_icmpv6_pkt(uint8_t *buf, ssize_t buflen, bool with_ext,
    int payload_len, bool bad_csum, bool bad_len,
    bool smaller_len)
    {
    struct icmp6hdr *icmph;
    int len, ret;
    len = sizeof(*icmph);
    memset(buf, 0, buflen);
    icmph = (struct icmp6hdr *)buf;
    icmph.icmp6_type = ICMPV6_DEST_UNREACH;
    icmph.icmp6_code = ICMPV6_PORT_UNREACH;
    icmph.icmp6_cksum = 0;
    ret = build_orig_dgram_v6(buf + len, buflen - len, payload_len);
    if (ret < 0)
    return ret;
    len += ret;
    icmph.icmp6_datagram_len = (len - sizeof(*icmph)) / sizeof(uint64_t);
    if (with_ext) {
    ret = build_rfc4884_ext(buf + len, buflen - len,
    bad_csum, bad_len, smaller_len);
    if (ret < 0)
    return ret;
    len += ret;
    }
    icmph.icmp6_cksum = htons(csum(icmph, len));
    return len;
    }
    FIXTURE(rfc4884) {};
    FIXTURE_SETUP(rfc4884)
    {
    int ret;
    ret = unshare(CLONE_NEWNET);
    ASSERT_EQ(ret, 0) {
    TH_LOG("unshare(CLONE_NEWNET) failed: %s", strerror(errno));
    }
    ret = bringup_loopback();
    ASSERT_EQ(ret, 0) TH_LOG("Failed to bring up loopback interface");
    }
    FIXTURE_TEARDOWN(rfc4884)
    {
    }
    const struct ip_case_info ipv4_info = {
    .domain		= AF_INET,
    .level		= SOL_IP,
    .opt1		= IP_RECVERR,
    .opt2		= IP_RECVERR_RFC4884,
    .proto		= IPPROTO_ICMP,
    .build_func	= build_icmpv4_pkt,
    .min_payload	= min_payload_len_v4,
    };
    const struct ip_case_info ipv6_info = {
    .domain		= AF_INET6,
    .level		= SOL_IPV6,
    .opt1		= IPV6_RECVERR,
    .opt2		= IPV6_RECVERR_RFC4884,
    .proto		= IPPROTO_ICMPV6,
    .build_func	= build_icmpv6_pkt,
    .min_payload	= min_payload_len_v6,
    };
    FIXTURE_VARIANT(rfc4884) {
// IPv4/v6 related information
    struct ip_case_info	info;
// Whether to append an ICMP extension or not
    bool			with_ext;
// UDP payload length
    int			payload_len;
// Whether to generate a bad checksum in the ICMP extension structure
    bool			bad_csum;
// Whether to generate a bad length in the ICMP object header
    bool			bad_len;
// Whether it is too small to fit the object header or too big to fit
// in the packet
//
    bool			smaller_len;
    };
// Tests that a valid ICMPv4 error message with extension and the original
// datagram is smaller than 128 bytes, generates an error with zero offset,
// and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_ext_small_payload) {
    .info		= ipv4_info,
    .with_ext	= true,
    .payload_len	= 64,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv4 error message with extension and 128 bytes original
// datagram, generates an error with the expected offset, and does not raise the
// SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_ext) {
    .info		= ipv4_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v4,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv4 error message with extension and the original
// datagram is larger than 128 bytes, generates an error with the expected
// offset, and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_ext_large_payload) {
    .info		= ipv4_info,
    .with_ext	= true,
    .payload_len	= 256,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv4 error message without extension and the original
// datagram is smaller than 128 bytes, generates an error with zero offset,
// and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_no_ext_small_payload) {
    .info		= ipv4_info,
    .with_ext	= false,
    .payload_len	= 64,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv4 error message without extension and 128 bytes
// original datagram, generates an error with zero offset, and does not raise
// the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_no_ext_min_payload) {
    .info		= ipv4_info,
    .with_ext	= false,
    .payload_len	= min_payload_len_v4,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv4 error message without extension and the original
// datagram is larger than 128 bytes, generates an error with zero offset,
// and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_no_ext_large_payload) {
    .info		= ipv4_info,
    .with_ext	= false,
    .payload_len	= 256,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that an ICMPv4 error message with extension and an invalid checksum,
// generates an error with the expected offset, and raises the
// SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_invalid_ext_checksum) {
    .info		= ipv4_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v4,
    .bad_csum	= true,
    .bad_len	= false,
    };
// Tests that an ICMPv4 error message with extension and an object length
// smaller than the object header, generates an error with the expected offset,
// and raises the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_invalid_ext_length_small) {
    .info		= ipv4_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v4,
    .bad_csum	= false,
    .bad_len	= true,
    .smaller_len	= true,
    };
// Tests that an ICMPv4 error message with extension and an object length that
// is too big to fit in the packet, generates an error with the expected offset,
// and raises the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv4_invalid_ext_length_large) {
    .info		= ipv4_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v4,
    .bad_csum	= false,
    .bad_len	= true,
    .smaller_len	= false,
    };
// Tests that a valid ICMPv6 error message with extension and the original
// datagram is smaller than 128 bytes, generates an error with zero offset,
// and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_ext_small_payload) {
    .info		= ipv6_info,
    .with_ext	= true,
    .payload_len	= 64,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv6 error message with extension and 128 bytes original
// datagram, generates an error with the expected offset, and does not raise the
// SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_ext) {
    .info		= ipv6_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v6,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv6 error message with extension and the original
// datagram is larger than 128 bytes, generates an error with the expected
// offset, and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_ext_large_payload) {
    .info		= ipv6_info,
    .with_ext	= true,
    .payload_len	= 256,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv6 error message without extension and the original
// datagram is smaller than 128 bytes, generates an error with zero offset,
// and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_no_ext_small_payload) {
    .info		= ipv6_info,
    .with_ext	= false,
    .payload_len	= 64,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv6 error message without extension and 128 bytes
// original datagram, generates an error with zero offset, and does not
// raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_no_ext_min_payload) {
    .info		= ipv6_info,
    .with_ext	= false,
    .payload_len	= min_payload_len_v6,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that a valid ICMPv6 error message without extension and the original
// datagram is larger than 128 bytes, generates an error with zero offset,
// and does not raise the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_no_ext_large_payload) {
    .info		= ipv6_info,
    .with_ext	= false,
    .payload_len	= 256,
    .bad_csum	= false,
    .bad_len	= false,
    };
// Tests that an ICMPv6 error message with extension and an invalid checksum,
// generates an error with the expected offset, and raises the
// SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_invalid_ext_checksum) {
    .info		= ipv6_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v6,
    .bad_csum	= true,
    .bad_len	= false,
    };
// Tests that an ICMPv6 error message with extension and an object length
// smaller than the object header, generates an error with the expected offset,
// and raises the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_invalid_ext_length_small) {
    .info		= ipv6_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v6,
    .bad_csum	= false,
    .bad_len	= true,
    .smaller_len	= true,
    };
// Tests that an ICMPv6 error message with extension and an object length that
// is too big to fit in the packet, generates an error with the expected offset,
// and raises the SO_EE_RFC4884_FLAG_INVALID flag.
//
    FIXTURE_VARIANT_ADD(rfc4884, ipv6_invalid_ext_length_large) {
    .info		= ipv6_info,
    .with_ext	= true,
    .payload_len	= min_payload_len_v6,
    .bad_csum	= false,
    .bad_len	= true,
    .smaller_len	= false,
    };
    static void
    check_rfc4884_offset(struct __test_metadata *_metadata, int sock,
#[no_mangle]
pub unsafe extern "C" fn FIXTURE_VARIANT(v: *mut rfc4884)) -> const {
    const FIXTURE_VARIANT(rfc4884) *v)
    {
    char rxbuf[1024];
    char ctrl[1024];
    struct iovec iov = {
    .iov_base = rxbuf,
    .iov_len = sizeof(rxbuf)
    };
    struct msghdr msg = {
    .msg_iov = &iov,
    .msg_iovlen = 1,
    .msg_control = ctrl,
    .msg_controllen = sizeof(ctrl),
    };
    struct cmsghdr *cmsg;
    int recv;
    ASSERT_EQ(poll_err(sock), 0);
    recv = recvmsg(sock, &msg, MSG_ERRQUEUE);
    ASSERT_GE(recv, 0) TH_LOG("recvmsg(MSG_ERRQUEUE) failed");
    for (cmsg = CMSG_FIRSTHDR(&msg); cmsg; cmsg = CMSG_NXTHDR(&msg, cmsg)) {
    bool is_invalid, expected_invalid;
    struct sock_extended_err *ee;
    int expected_off;
    uint16_t off;
    if (cmsg.cmsg_level != v.info.level ||
    cmsg.cmsg_type != v.info.opt1) {
    TH_LOG("Unrelated cmsgs were encountered in recvmsg()");
    continue;
    }
    ee = (struct sock_extended_err *)CMSG_DATA(cmsg);
    off = ee.ee_rfc4884.len;
    is_invalid = ee.ee_rfc4884.flags & SO_EE_RFC4884_FLAG_INVALID;
    expected_invalid = v.bad_csum || v.bad_len;
    ASSERT_EQ(is_invalid, expected_invalid) {
    TH_LOG("Expected invalidity flag to be %d, but got %d",
    expected_invalid, is_invalid);
    }
    expected_off =
    (v.with_ext && v.payload_len >= v.info.min_payload) ?
    v.payload_len : 0;
    ASSERT_EQ(off, expected_off) {
    TH_LOG("Expected RFC4884 offset %u, got %u",
    expected_off, off);
    }
    break;
    }
    }
    TEST_F(rfc4884, rfc4884)
    {
    const typeof(variant) v = variant;
    struct sockaddr_inet addr;
    uint8_t pkt[1024];
    int dgram, raw;
    int len, sent;
    int err;
    dgram = socket(v.info.domain, SOCK_DGRAM, 0);
    ASSERT_GE(dgram, 0) TH_LOG("Opening datagram socket failed");
    err = bind_and_setsockopt(dgram, &v.info);
    ASSERT_EQ(err, 0) TH_LOG("Bind failed");
    raw = socket(v.info.domain, SOCK_RAW, v.info.proto);
    ASSERT_GE(raw, 0) TH_LOG("Opening raw socket failed");
    len = v.info.build_func(pkt, sizeof(pkt), v.with_ext, v.payload_len,
    v.bad_csum, v.bad_len, v.smaller_len);
    ASSERT_GT(len, 0) TH_LOG("Building packet failed");
    set_addr(&addr, v.info.domain, 0);
    sent = sendto(raw, pkt, len, 0, &addr.sa, addr.len);
    ASSERT_EQ(len, sent) TH_LOG("Sending packet failed");
    check_rfc4884_offset(_metadata, dgram, v);
    close(dgram);
    close(raw);
    }
    TEST_HARNESS_MAIN
