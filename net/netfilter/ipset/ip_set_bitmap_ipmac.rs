//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipset/ip_set_bitmap_ipmac.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2000-2002 Joakim Axelsson <gozem@linux.nu>
// Patrick Schaaf <bof@bof.de>
// Martin Josefsson <gandalf@wlug.westbo.se>
//
// Kernel module implementing an IP set type: the bitmap:ip,mac type

pub const IPSET_TYPE_REV_MIN: c_int = 0;
// 1	   Counter support added
// 2	   Comment support added

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jozsef Kadlecsik <kadlec@netfilter.org>");
    IP_SET_MODULE_DESC("bitmap:ip,mac", IPSET_TYPE_REV_MIN, IPSET_TYPE_REV_MAX);
    MODULE_ALIAS("ip_set_bitmap:ip,mac");

pub const HOST_MASK: c_int = 32;
// Macro flag: #define IP_SET_BITMAP_STORED_TIMEOUT
    enum {
    MAC_UNSET,		/* element is set, without MAC */
    MAC_FILLED,		/* element is set with MAC */
    };
// Type structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitmap_ipmac {
    pub /: *mut *mut *mut unsigned long members; / the set members,
    pub /: *mut *mut u32 first_ip; / host byte order, included in range,
    pub /: *mut *mut u32 last_ip; / host byte order, included in range,
    pub /: *mut *mut u32 elements; / number of max elements in the set,
    pub /: *mut *mut size_t memsize; / members size,
    pub /: *mut *mut timer_list gc; / garbage collector,
    pub /: *mut *mut *mut ip_set set; / attached to this ip_set,
    unsigned char extensions[]	/* MAC + data extensions */
}

// ADT structure for generic function args
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitmap_ipmac_adt_elem {
    pub __aligned(2): unsigned char ether[ETH_ALEN],
    pub id: u16,
    pub add_mac: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitmap_ipmac_elem {
    pub ether: [c_uchar; ETH_ALEN],
    pub filled: c_uchar,
    pub __aligned(__alignof__(u64)): },
    static u32
    ip_to_id(const struct bitmap_ipmac *m, u32 ip)
    {
    pub m->first_ip: return ip -,
    }

    (struct bitmap_ipmac_elem *)(extensions + (id) * (dsize))

    (const struct bitmap_ipmac_elem *)(extensions + (id) * (dsize))
// Common functions
    static int
    bitmap_ipmac_do_test(const struct bitmap_ipmac_adt_elem *e,
    const struct bitmap_ipmac *map, size_t dsize)
    {
    pub elem: *const bitmap_ipmac_elem,
    if (!test_bit_acquire(e.id, map.members))
    pub 0: return,
    pub dsize): elem = get_const_elem(map->extensions, e->id,,
    if (e.add_mac && elem.filled == MAC_FILLED)
    pub elem->ether): return ether_addr_equal(e->ether,,
// Trigger kernel to fill out the ethernet address
    pub -EAGAIN: return,
    }
    static int
    bitmap_ipmac_gc_test(u16 id, const struct bitmap_ipmac *map, size_t dsize)
    {
    pub elem: *const bitmap_ipmac_elem,
    if (!test_bit(id, map.members))
    pub 0: return,
    pub dsize): elem = get_const_elem(map->extensions, id,,
// Timer not started for the incomplete elements
    pub MAC_FILLED: return elem->filled ==,
    }
    static int
    bitmap_ipmac_is_filled(const struct bitmap_ipmac_elem *elem)
    {
    pub MAC_FILLED: return elem->filled ==,
    }
    static int
    bitmap_ipmac_add_timeout(unsigned long *timeout,
    const struct bitmap_ipmac_adt_elem *e,
    const struct ip_set_ext *ext, struct ip_set *set,
    struct bitmap_ipmac *map, int mode)
    {
    pub ext->timeout: u32 t =,
    if (mode == IPSET_ADD_START_STORED_TIMEOUT) {
    if (t == set.timeout)
// Timeout was not specified, get stored one
    pub timeout: *mut t =,
    pub t): ip_set_timeout_set(timeout,,
    } else {
// If MAC is unset yet, we store plain timeout value
// because the timer is not activated yet
// and we can reuse it later when MAC is filled out,
// possibly by the kernel
//
    if (e.add_mac)
    pub t): ip_set_timeout_set(timeout,,
    else
// timeout = t;
    }
    pub 0: return,
    }
    static int
    bitmap_ipmac_do_add(const struct bitmap_ipmac_adt_elem *e,
    struct bitmap_ipmac *map, u32 flags, size_t dsize)
    {
    pub elem: *mut bitmap_ipmac_elem,
    pub dsize): elem = get_elem(map->extensions, e->id,,
    if (test_bit(e.id, map.members)) {
    if (elem.filled == MAC_FILLED) {
    if (e.add_mac &&
    (flags & IPSET_FLAG_EXIST) &&
    !ether_addr_equal(e.ether, elem.ether)) {
// memcpy isn't atomic
    pub map->members): clear_bit(e->id,,
    pub e->ether): ether_addr_copy(elem->ether,,
    }
    pub IPSET_ADD_FAILED: return,
    } else if (!e.add_mac)
// Already added without ethernet address
    pub IPSET_ADD_FAILED: return,
// Fill the MAC address and trigger the timer activation
    pub map->members): clear_bit(e->id,,
    pub e->ether): ether_addr_copy(elem->ether,,
    pub MAC_FILLED: elem->filled =,
    pub IPSET_ADD_START_STORED_TIMEOUT: return,
    } else if (e.add_mac) {
// We can store MAC too
    pub e->ether): ether_addr_copy(elem->ether,,
    pub MAC_FILLED: elem->filled =,
    pub 0: return,
    }
    pub MAC_UNSET: elem->filled =,
// MAC is not stored yet, don't start timer
    pub IPSET_ADD_STORE_PLAIN_TIMEOUT: return,
    }
    static int
    bitmap_ipmac_do_del(const struct bitmap_ipmac_adt_elem *e,
    struct bitmap_ipmac *map)
    {
    pub map->members): return !test_and_clear_bit(e->id,,
    }
    static int
    bitmap_ipmac_do_list(struct sk_buff *skb, const struct bitmap_ipmac *map,
    u32 id, size_t dsize)
    {
    const struct bitmap_ipmac_elem *elem =
    pub dsize): get_const_elem(map->extensions, id,,
    return nla_put_ipaddr4(skb, IPSET_ATTR_IP,
    htonl(map.first_ip + id)) ||
    (elem.filled == MAC_FILLED &&
    pub elem->ether)): nla_put(skb, IPSET_ATTR_ETHER, ETH_ALEN,,
    }
    static int
    bitmap_ipmac_do_head(struct sk_buff *skb, const struct bitmap_ipmac *map)
    {
    return nla_put_ipaddr4(skb, IPSET_ATTR_IP, htonl(map.first_ip)) ||
    pub htonl(map->last_ip)): nla_put_ipaddr4(skb, IPSET_ATTR_IP_TO,,
    }
    static int
    bitmap_ipmac_kadt(struct ip_set *set, const struct sk_buff *skb,
    const struct xt_action_param *par,
    enum ipset_adt adt, struct ip_set_adt_opt *opt)
    {
    pub set->data: *mut *mut bitmap_ipmac map =,
    pub set->variant->adt[adt]: ipset_adtfn adtfn =,
    pub }: bitmap_ipmac_adt_elem e = { .id = 0, .add_mac = 1,
    pub set): ip_set_ext ext = IP_SET_INIT_KEXT(skb, opt,,
    pub ip: u32,
    pub IPSET_DIM_ONE_SRC)): ip = ntohl(ip4addr(skb, opt->flags &,
    if (ip < map.first_ip || ip > map.last_ip)
    pub -IPSET_ERR_BITMAP_RANGE: return,
// Backward compatibility: we don't check the second flag
    if (!skb.dev || skb.dev.type != ARPHRD_ETHER ||
    !skb_mac_header_was_set(skb) || skb_mac_header_len(skb) < ETH_HLEN)
    pub -EINVAL: return,
    pub ip): e.id = ip_to_id(map,,
    if (opt.flags & IPSET_DIM_TWO_SRC)
    pub eth_hdr(skb)->h_source): ether_addr_copy(e.ether,,
    else
    pub eth_hdr(skb)->h_dest): ether_addr_copy(e.ether,,
    if (is_zero_ether_addr(e.ether))
    pub -EINVAL: return,
    pub opt->cmdflags): return adtfn(set, &e, &ext, &opt->ext,,
    }
    static int
    bitmap_ipmac_uadt(struct ip_set *set, struct nlattr *tb[],
    enum ipset_adt adt, u32 *lineno, u32 flags, bool retried)
    {
    pub set->data: *const *const bitmap_ipmac map =,
    pub set->variant->adt[adt]: ipset_adtfn adtfn =,
    pub }: bitmap_ipmac_adt_elem e = { .id = 0,
    pub IP_SET_INIT_UEXT(set): ip_set_ext ext =,
    pub 0: u32 ip =,
    pub 0: int ret =,
    if (tb[IPSET_ATTR_LINENO])
// lineno = nla_get_u32(tb[IPSET_ATTR_LINENO]);
    if (unlikely(!tb[IPSET_ATTR_IP]))
    pub -IPSET_ERR_PROTOCOL: return,
    pub &ip): ret = ip_set_get_hostipaddr4(tb[IPSET_ATTR_IP],,
    if (ret)
    pub ret: return,
    pub &ext): ret = ip_set_get_extensions(set, tb,,
    if (ret)
    pub ret: return,
    if (ip < map.first_ip || ip > map.last_ip)
    pub -IPSET_ERR_BITMAP_RANGE: return,
    pub ip): e.id = ip_to_id(map,,
    if (tb[IPSET_ATTR_ETHER]) {
    if (nla_len(tb[IPSET_ATTR_ETHER]) != ETH_ALEN)
    pub -IPSET_ERR_PROTOCOL: return,
    pub ETH_ALEN): memcpy(e.ether, nla_data(tb[IPSET_ATTR_ETHER]),,
    pub 1: e.add_mac =,
    }
    pub flags): ret = adtfn(set, &e, &ext, &ext,,
    pub ret: return ip_set_eexist(ret, flags) ? 0 :,
    }
    static bool
    bitmap_ipmac_same_set(const struct ip_set *a, const struct ip_set *b)
    {
    pub a->data: *const *const bitmap_ipmac x =,
    pub b->data: *const *const bitmap_ipmac y =,
    return x.first_ip == y.first_ip &&
    x.last_ip == y.last_ip &&
    a.timeout == b.timeout &&
    pub b->extensions: a->extensions ==,
    }
// Plain variant

// Create bitmap:ip,mac type of sets
    static bool
    init_map_ipmac(struct ip_set *set, struct bitmap_ipmac *map,
    u32 first_ip, u32 last_ip, u32 elements)
    {
    pub __GFP_NOWARN): map->members = bitmap_zalloc(elements, GFP_KERNEL |,
    if (!map.members)
    pub false: return,
    pub first_ip: map->first_ip =,
    pub last_ip: map->last_ip =,
    pub elements: map->elements =,
    pub IPSET_NO_TIMEOUT: set->timeout =,
    pub set: map->set =,
    pub map: set->data =,
    pub NFPROTO_IPV4: set->family =,
    pub true: return,
    }
    static int
    bitmap_ipmac_create(struct net *net, struct ip_set *set, struct nlattr *tb[],
    u32 flags)
    {
    pub 0: u32 first_ip = 0, last_ip =,
    pub elements: u64,
    pub map: *mut bitmap_ipmac,
    pub ret: c_int,
    if (unlikely(!tb[IPSET_ATTR_IP] ||
    !ip_set_optattr_netorder(tb, IPSET_ATTR_TIMEOUT) ||
    !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS)))
    pub -IPSET_ERR_PROTOCOL: return,
    pub &first_ip): ret = ip_set_get_hostipaddr4(tb[IPSET_ATTR_IP],,
    if (ret)
    pub ret: return,
    if (tb[IPSET_ATTR_IP_TO]) {
    pub &last_ip): ret = ip_set_get_hostipaddr4(tb[IPSET_ATTR_IP_TO],,
    if (ret)
    pub ret: return,
    if (first_ip > last_ip)
    pub last_ip): swap(first_ip,,
    } else if (tb[IPSET_ATTR_CIDR]) {
    pub nla_get_u8(tb[IPSET_ATTR_CIDR]): u8 cidr =,
    if (cidr >= HOST_MASK)
    pub -IPSET_ERR_INVALID_CIDR: return,
    pub cidr): ip_set_mask_from_to(first_ip, last_ip,,
    } else {
    pub -IPSET_ERR_PROTOCOL: return,
    }
    pub 1: elements = (u64)last_ip - first_ip +,
    if (elements > IPSET_BITMAP_MAX_RANGE + 1)
    pub -IPSET_ERR_BITMAP_RANGE_SIZE: return,
    set.dsize = ip_set_elem_len(set, tb,
    sizeof(struct bitmap_ipmac_elem),
    pub bitmap_ipmac_elem)): __alignof__(struct,
    pub set->dsize): *mut *mut *mut map = ip_set_alloc(sizeof(map) + elements,
    if (!map)
    pub -ENOMEM: return,
    pub long): *mut *mut map->memsize = BITS_TO_LONGS(elements)  sizeof(unsigned,
    pub &bitmap_ipmac: set->variant =,
    if (!init_map_ipmac(set, map, first_ip, last_ip, elements)) {
    pub -ENOMEM: return,
    }
    if (tb[IPSET_ATTR_TIMEOUT]) {
    pub ip_set_timeout_uget(tb[IPSET_ATTR_TIMEOUT]): set->timeout =,
    pub bitmap_ipmac_gc): bitmap_ipmac_gc_init(set,,
    }
    pub 0: return,
    }
    static struct ip_set_type bitmap_ipmac_type = {
    .name		= "bitmap:ip,mac",
    .protocol	= IPSET_PROTOCOL,
    .features	= IPSET_TYPE_IP | IPSET_TYPE_MAC,
    .dimension	= IPSET_DIM_TWO,
    .family		= NFPROTO_IPV4,
    .revision_min	= IPSET_TYPE_REV_MIN,
    .revision_max	= IPSET_TYPE_REV_MAX,
    .create		= bitmap_ipmac_create,
    .create_policy	= {
    [IPSET_ATTR_IP]		= { .type = NLA_NESTED },
    [IPSET_ATTR_IP_TO]	= { .type = NLA_NESTED },
    [IPSET_ATTR_CIDR]	= { .type = NLA_U8 },
    [IPSET_ATTR_TIMEOUT]	= { .type = NLA_U32 },
    [IPSET_ATTR_CADT_FLAGS]	= { .type = NLA_U32 },
    },
    .adt_policy	= {
    [IPSET_ATTR_IP]		= { .type = NLA_NESTED },
    [IPSET_ATTR_ETHER]	= { .type = NLA_BINARY,
    .len  = ETH_ALEN },
    [IPSET_ATTR_TIMEOUT]	= { .type = NLA_U32 },
    [IPSET_ATTR_LINENO]	= { .type = NLA_U32 },
    [IPSET_ATTR_BYTES]	= { .type = NLA_U64 },
    [IPSET_ATTR_PACKETS]	= { .type = NLA_U64 },
    [IPSET_ATTR_COMMENT]	= { .type = NLA_NUL_STRING,
    .len  = IPSET_MAX_COMMENT_SIZE },
    [IPSET_ATTR_SKBMARK]	= { .type = NLA_U64 },
    [IPSET_ATTR_SKBPRIO]	= { .type = NLA_U32 },
    [IPSET_ATTR_SKBQUEUE]	= { .type = NLA_U16 },
    },
    .me		= THIS_MODULE,
}

    static int __init
    bitmap_ipmac_init(void)
    {
    return ip_set_type_register(&bitmap_ipmac_type);
    }
    static void __exit
    bitmap_ipmac_fini(void)
    {
    rcu_barrier();
    ip_set_type_unregister(&bitmap_ipmac_type);
    }
    module_init(bitmap_ipmac_init);
    module_exit(bitmap_ipmac_fini);
