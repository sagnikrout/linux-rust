//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tap.c
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

    static const char param_dev_tap_name[] = "xmacvtap0";
    static const char param_dev_dummy_name[] = "xdummy0";
    static unsigned char param_hwaddr_src[] = { 0x00, 0xfe, 0x98, 0x14, 0x22, 0x42 };
    static unsigned char param_hwaddr_dest[] = {
    0x00, 0xfe, 0x98, 0x94, 0xd2, 0x43
    };

pub const PKT_DATA: c_uint = 0xCB;

    static struct rtattr *rtattr_add(struct nlmsghdr *nh, unsigned short type,
    unsigned short len)
    {
    struct rtattr *rta =
    (struct rtattr *)((uint8_t *)nh + RTA_ALIGN(nh.nlmsg_len));
    rta.rta_type = type;
    rta.rta_len = RTA_LENGTH(len);
    nh.nlmsg_len = RTA_ALIGN(nh.nlmsg_len) + RTA_ALIGN(rta.rta_len);
    return rta;
    }
    static struct rtattr *rtattr_begin(struct nlmsghdr *nh, unsigned short type)
    {
    return rtattr_add(nh, type, 0);
    }
#[no_mangle]
unsafe extern "C" fn rtattr_end(nh: *mut nlmsghdr, attr: *mut rtattr) {
    static void rtattr_end(struct nlmsghdr *nh, struct rtattr *attr)
    {
    uint8_t *end = (uint8_t *)nh + nh.nlmsg_len;
    attr.rta_len = end - (uint8_t *)attr;
    }
    static struct rtattr *rtattr_add_str(struct nlmsghdr *nh, unsigned short type,
    const char *s)
    {
    let mut strsz: c_uint = strlen(s) + 1;
    struct rtattr *rta;
    rta = rtattr_add(nh, type, strsz);
    memcpy(RTA_DATA(rta), s, strsz);
    return rta;
    }
    static struct rtattr *rtattr_add_any(struct nlmsghdr *nh, unsigned short type,
    const void *arr, size_t len)
    {
    struct rtattr *rta = rtattr_add(nh, type, len);
    memcpy(RTA_DATA(rta), arr, len);
    return rta;
    }
    static int dev_create(const char *dev, const char *link_type,
    int (*fill_rtattr)(struct nlmsghdr *nh),
    int (*fill_info_data)(struct nlmsghdr *nh))
    {
    struct {
    struct nlmsghdr nh;
    struct ifinfomsg info;
    unsigned char data[MAX_RTNL_PAYLOAD];
    } req;
    struct rtattr *link_info, *info_data;
    int ret, rtnl;
    rtnl = socket(AF_NETLINK, SOCK_DGRAM, NETLINK_ROUTE);
    if (rtnl < 0) {
    fprintf(stderr, "%s: socket %s\n", __func__, strerror(errno));
    return 1;
    }
    memset(&req, 0, sizeof(req));
    req.nh.nlmsg_len = NLMSG_LENGTH(sizeof(req.info));
    req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_CREATE;
    req.nh.nlmsg_type = RTM_NEWLINK;
    req.info.ifi_family = AF_UNSPEC;
    req.info.ifi_type = 1;
    req.info.ifi_index = 0;
    req.info.ifi_flags = IFF_BROADCAST | IFF_UP;
    req.info.ifi_change = 0xffffffff;
    rtattr_add_str(&req.nh, IFLA_IFNAME, dev);
    if (fill_rtattr) {
    ret = fill_rtattr(&req.nh);
    if (ret)
    return ret;
    }
    link_info = rtattr_begin(&req.nh, IFLA_LINKINFO);
    rtattr_add_str(&req.nh, IFLA_INFO_KIND, link_type);
    if (fill_info_data) {
    info_data = rtattr_begin(&req.nh, IFLA_INFO_DATA);
    ret = fill_info_data(&req.nh);
    if (ret)
    return ret;
    rtattr_end(&req.nh, info_data);
    }
    rtattr_end(&req.nh, link_info);
    ret = send(rtnl, &req, req.nh.nlmsg_len, 0);
    if (ret < 0)
    fprintf(stderr, "%s: send %s\n", __func__, strerror(errno));
    ret = (unsigned int)ret != req.nh.nlmsg_len;
    close(rtnl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dev_delete(dev: *const c_char) -> c_int {
    static int dev_delete(const char *dev)
    {
    struct {
    struct nlmsghdr nh;
    struct ifinfomsg info;
    unsigned char data[MAX_RTNL_PAYLOAD];
    } req;
    int ret, rtnl;
    rtnl = socket(AF_NETLINK, SOCK_DGRAM, NETLINK_ROUTE);
    if (rtnl < 0) {
    fprintf(stderr, "%s: socket %s\n", __func__, strerror(errno));
    return 1;
    }
    memset(&req, 0, sizeof(req));
    req.nh.nlmsg_len = NLMSG_LENGTH(sizeof(req.info));
    req.nh.nlmsg_flags = NLM_F_REQUEST;
    req.nh.nlmsg_type = RTM_DELLINK;
    req.info.ifi_family = AF_UNSPEC;
    rtattr_add_str(&req.nh, IFLA_IFNAME, dev);
    ret = send(rtnl, &req, req.nh.nlmsg_len, 0);
    if (ret < 0)
    fprintf(stderr, "%s: send %s\n", __func__, strerror(errno));
    ret = (unsigned int)ret != req.nh.nlmsg_len;
    close(rtnl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn macvtap_fill_rtattr(nh: *mut nlmsghdr) -> c_int {
    static int macvtap_fill_rtattr(struct nlmsghdr *nh)
    {
    int ifindex;
    ifindex = if_nametoindex(param_dev_dummy_name);
    if (ifindex == 0) {
    fprintf(stderr, "%s: ifindex  %s\n", __func__, strerror(errno));
    return -errno;
    }
    rtattr_add_any(nh, IFLA_LINK, &ifindex, sizeof(ifindex));
    rtattr_add_any(nh, IFLA_ADDRESS, param_hwaddr_src, ETH_ALEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn opentap(devname: *const c_char) -> c_int {
    static int opentap(const char *devname)
    {
    int ifindex;
    char buf[256];
    int fd;
    struct ifreq ifr;
    ifindex = if_nametoindex(devname);
    if (ifindex == 0) {
    fprintf(stderr, "%s: ifindex %s\n", __func__, strerror(errno));
    return -errno;
    }
    sprintf(buf, "/dev/tap%d", ifindex);
    fd = open(buf, O_RDWR | O_NONBLOCK);
    if (fd < 0) {
    fprintf(stderr, "%s: open %s\n", __func__, strerror(errno));
    return -errno;
    }
    memset(&ifr, 0, sizeof(ifr));
    strcpy(ifr.ifr_name, devname);
    ifr.ifr_flags = IFF_TAP | IFF_NO_PI | IFF_VNET_HDR | IFF_MULTI_QUEUE;
    if (ioctl(fd, TUNSETIFF, &ifr, sizeof(ifr)) < 0)
    return -errno;
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn build_eth(buf: *mut u8, proto: u16) -> usize {
    size_t build_eth(uint8_t *buf, uint16_t proto)
    {
    struct ethhdr *eth = (struct ethhdr *)buf;
    eth.h_proto = htons(proto);
    memcpy(eth.h_source, param_hwaddr_src, ETH_ALEN);
    memcpy(eth.h_dest, param_hwaddr_dest, ETH_ALEN);
    return ETH_HLEN;
    }
#[no_mangle]
unsafe extern "C" fn add_csum(buf: *const u8, len: c_int) -> u32 {
    static uint32_t add_csum(const uint8_t *buf, int len)
    {
    let mut sum: u32 = 0;
    uint16_t *sbuf = (uint16_t *)buf;
    while (len > 1) {
    sum += *sbuf++;
    len -= 2;
    }
    if (len)
    sum += *(uint8_t *)sbuf;
    return sum;
    }
#[no_mangle]
unsafe extern "C" fn finish_ip_csum(sum: u32) -> u16 {
    static uint16_t finish_ip_csum(uint32_t sum)
    {
    let mut lo: u16 = sum & 0xffff;
    let mut hi: u16 = sum >> 16;
    return ~(lo + hi);
    }
    static uint16_t build_ip_csum(const uint8_t *buf, int len,
    uint32_t sum)
    {
    sum += add_csum(buf, len);
    return finish_ip_csum(sum);
    }
#[no_mangle]
unsafe extern "C" fn build_ipv4_header(buf: *mut u8, payload_len: c_int) -> c_int {
    static int build_ipv4_header(uint8_t *buf, int payload_len)
    {
    struct iphdr *iph = (struct iphdr *)buf;
    iph.ihl = 5;
    iph.version = 4;
    iph.ttl = 8;
    iph.tot_len =
    htons(sizeof(*iph) + sizeof(struct udphdr) + payload_len);
    iph.id = htons(1337);
    iph.protocol = IPPROTO_UDP;
    iph.saddr = htonl((172 << 24) | (17 << 16) | 2);
    iph.daddr = htonl((172 << 24) | (17 << 16) | 1);
    iph.check = build_ip_csum(buf, iph.ihl << 2, 0);
    return iph.ihl << 2;
    }
#[no_mangle]
unsafe extern "C" fn build_udp_packet(buf: *mut u8, payload_len: c_int, csum_off: bool) -> c_int {
    static int build_udp_packet(uint8_t *buf, int payload_len, bool csum_off)
    {
    let mut ip4alen: c_int = sizeof(uint32_t);
    struct udphdr *udph = (struct udphdr *)buf;
    let mut len: c_int = sizeof(*udph) + payload_len;
    let mut sum: u32 = 0;
    udph.source = htons(22);
    udph.dest = htons(58822);
    udph.len = htons(len);
    memset(buf + sizeof(struct udphdr), PKT_DATA, payload_len);
    sum = add_csum(buf - 2 * ip4alen, 2 * ip4alen);
    sum += htons(IPPROTO_UDP) + udph.len;
    if (!csum_off)
    sum += add_csum(buf, len);
    udph.check = finish_ip_csum(sum);
    return sizeof(*udph) + payload_len;
    }
#[no_mangle]
pub unsafe extern "C" fn build_test_packet_valid_udp_gso(buf: *mut u8, payload_len: usize) -> usize {
    size_t build_test_packet_valid_udp_gso(uint8_t *buf, size_t payload_len)
    {
    uint8_t *cur = buf;
    struct virtio_net_hdr *vh = (struct virtio_net_hdr *)buf;
    vh.hdr_len = ETH_HLEN + sizeof(struct iphdr) + sizeof(struct udphdr);
    vh.flags = VIRTIO_NET_HDR_F_NEEDS_CSUM;
    vh.csum_start = ETH_HLEN + sizeof(struct iphdr);
    vh.csum_offset = __builtin_offsetof(struct udphdr, check);
    vh.gso_type = VIRTIO_NET_HDR_GSO_UDP;
    vh.gso_size = ETH_DATA_LEN - sizeof(struct iphdr);
    cur += sizeof(*vh);
    cur += build_eth(cur, ETH_P_IP);
    cur += build_ipv4_header(cur, payload_len);
    cur += build_udp_packet(cur, payload_len, true);
    return cur - buf;
    }
#[no_mangle]
pub unsafe extern "C" fn build_test_packet_valid_udp_csum(buf: *mut u8, payload_len: usize) -> usize {
    size_t build_test_packet_valid_udp_csum(uint8_t *buf, size_t payload_len)
    {
    uint8_t *cur = buf;
    struct virtio_net_hdr *vh = (struct virtio_net_hdr *)buf;
    vh.flags = VIRTIO_NET_HDR_F_DATA_VALID;
    vh.gso_type = VIRTIO_NET_HDR_GSO_NONE;
    cur += sizeof(*vh);
    cur += build_eth(cur, ETH_P_IP);
    cur += build_ipv4_header(cur, payload_len);
    cur += build_udp_packet(cur, payload_len, false);
    return cur - buf;
    }
    size_t build_test_packet_crash_tap_invalid_eth_proto(uint8_t *buf,
    size_t payload_len)
    {
    uint8_t *cur = buf;
    struct virtio_net_hdr *vh = (struct virtio_net_hdr *)buf;
    vh.hdr_len = ETH_HLEN + sizeof(struct iphdr) + sizeof(struct udphdr);
    vh.flags = 0;
    vh.gso_type = VIRTIO_NET_HDR_GSO_UDP;
    vh.gso_size = ETH_DATA_LEN - sizeof(struct iphdr);
    cur += sizeof(*vh);
    cur += build_eth(cur, 0);
    cur += sizeof(struct iphdr) + sizeof(struct udphdr);
    cur += build_ipv4_header(cur, payload_len);
    cur += build_udp_packet(cur, payload_len, true);
    cur += payload_len;
    return cur - buf;
    }
    FIXTURE(tap)
    {
    int fd;
    };
    FIXTURE_SETUP(tap)
    {
    int ret;
    ret = dev_create(param_dev_dummy_name, "dummy", core::ptr::null_mut(), core::ptr::null_mut());
    EXPECT_EQ(ret, 0);
    ret = dev_create(param_dev_tap_name, "macvtap", macvtap_fill_rtattr,
    core::ptr::null_mut());
    EXPECT_EQ(ret, 0);
    self.fd = opentap(param_dev_tap_name);
    ASSERT_GE(self.fd, 0);
    }
    FIXTURE_TEARDOWN(tap)
    {
    int ret;
    if (self.fd != -1)
    close(self.fd);
    ret = dev_delete(param_dev_tap_name);
    EXPECT_EQ(ret, 0);
    ret = dev_delete(param_dev_dummy_name);
    EXPECT_EQ(ret, 0);
    }
    TEST_F(tap, test_packet_valid_udp_gso)
    {
    uint8_t pkt[TEST_PACKET_SZ];
    size_t off;
    int ret;
    memset(pkt, 0, sizeof(pkt));
    off = build_test_packet_valid_udp_gso(pkt, 1021);
    ret = write(self.fd, pkt, off);
    ASSERT_EQ(ret, off);
    }
    TEST_F(tap, test_packet_valid_udp_csum)
    {
    uint8_t pkt[TEST_PACKET_SZ];
    size_t off;
    int ret;
    memset(pkt, 0, sizeof(pkt));
    off = build_test_packet_valid_udp_csum(pkt, 1024);
    ret = write(self.fd, pkt, off);
    ASSERT_EQ(ret, off);
    }
    TEST_F(tap, test_packet_crash_tap_invalid_eth_proto)
    {
    uint8_t pkt[TEST_PACKET_SZ];
    size_t off;
    int ret;
    memset(pkt, 0, sizeof(pkt));
    off = build_test_packet_crash_tap_invalid_eth_proto(pkt, 1024);
    ret = write(self.fd, pkt, off);
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, EINVAL);
    }
    TEST_HARNESS_MAIN
