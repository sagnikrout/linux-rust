//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/tc.c
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

    static bool tc_qdisc_print(struct __test_metadata *_metadata,
    struct tc_getqdisc_rsp *q)
    {
    let mut was_fq_codel: bool = false;
    char ifname[IF_NAMESIZE];
    const char *name;
    name = if_indextoname(q._hdr.tcm_ifindex, ifname);
    EXPECT_TRUE((bool)name);
    ksft_print_msg("%16s: ", name ?: "no-name");
    if (q._len.kind) {
    printf("%s  ", q.kind);
    if (q.options._present.fq_codel) {
    struct tc_fq_codel_attrs *fq_codel;
    struct tc_fq_codel_xstats *stats;
    fq_codel = &q.options.fq_codel;
    stats = q.stats2.app.fq_codel;
    EXPECT_EQ(true,
    fq_codel._present.limit &&
    fq_codel._present.target &&
    q.stats2.app._len.fq_codel);
    if (fq_codel._present.limit)
    printf("limit: %dp ", fq_codel.limit);
    if (fq_codel._present.target)
    printf("target: %dms ",
    (fq_codel.target + 500) / 1000);
    if (q.stats2.app._len.fq_codel)
    printf("new_flow_cnt: %d ",
    stats.qdisc_stats.new_flow_count);
    was_fq_codel = true;
    }
    }
    printf("\n");
    return was_fq_codel;
    }
    static const char *vlan_act_name(struct tc_vlan *p)
    {
    switch (p.v_action) {
    case TCA_VLAN_ACT_POP:
    return "pop";
    case TCA_VLAN_ACT_PUSH:
    return "push";
    case TCA_VLAN_ACT_MODIFY:
    return "modify";
    default:
    break;
    }
    return "not supported";
    }
    static const char *gact_act_name(struct tc_gact *p)
    {
    switch (p.action) {
    case TC_ACT_SHOT:
    return "drop";
    case TC_ACT_OK:
    return "ok";
    case TC_ACT_PIPE:
    return "pipe";
    default:
    break;
    }
    return "not supported";
    }
#[no_mangle]
unsafe extern "C" fn print_vlan(vlan: *mut tc_act_vlan_attrs) {
    static void print_vlan(struct tc_act_vlan_attrs *vlan)
    {
    printf("%s ", vlan_act_name(vlan.parms));
    if (vlan._present.push_vlan_id)
    printf("id %u ", vlan.push_vlan_id);
    if (vlan._present.push_vlan_protocol)
    printf("protocol %#x ", ntohs(vlan.push_vlan_protocol));
    if (vlan._present.push_vlan_priority)
    printf("priority %u ", vlan.push_vlan_priority);
    }
#[no_mangle]
unsafe extern "C" fn print_gact(gact: *mut tc_act_gact_attrs) {
    static void print_gact(struct tc_act_gact_attrs *gact)
    {
    struct tc_gact *p = gact.parms;
    printf("%s ", gact_act_name(p));
    }
#[no_mangle]
unsafe extern "C" fn flower_print(flower: *mut tc_flower_attrs, kind: *const c_char) {
    static void flower_print(struct tc_flower_attrs *flower, const char *kind)
    {
    struct tc_act_attrs *a;
    unsigned int i;
    ksft_print_msg("%s:\n", kind);
    if (flower._present.key_vlan_id)
    ksft_print_msg("  vlan_id: %u\n", flower.key_vlan_id);
    if (flower._present.key_vlan_prio)
    ksft_print_msg("  vlan_prio: %u\n", flower.key_vlan_prio);
    if (flower._present.key_num_of_vlans)
    ksft_print_msg("  num_of_vlans: %u\n",
    flower.key_num_of_vlans);
    for (i = 0; i < flower._count.act; i++) {
    a = &flower.act[i];
    ksft_print_msg("action order: %i %s ", i + 1, a.kind);
    if (a.options._present.vlan)
    print_vlan(&a.options.vlan);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: a->options._present.gact) -> else {
    else if (a.options._present.gact)
    print_gact(&a.options.gact);
    printf("\n");
    }
    }
    static void tc_filter_print(struct __test_metadata *_metadata,
    struct tc_gettfilter_rsp *f)
    {
    struct tc_options_msg *opt = &f.options;
    if (opt._present.flower) {
    EXPECT_TRUE((bool)f._len.kind);
    flower_print(&opt.flower, f.kind);
    } else if (f._len.kind) {
    ksft_print_msg("%s pref %u proto: %#x\n", f.kind,
    (f._hdr.tcm_info >> 16),
    ntohs(TC_H_MIN(f._hdr.tcm_info)));
    }
    }
#[no_mangle]
unsafe extern "C" fn tc_clsact_add(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    static int tc_clsact_add(struct ynl_sock *ys, int ifi)
    {
    struct tc_newqdisc_req *req;
    int ret;
    req = tc_newqdisc_req_alloc();
    if (!req)
    return -1;
    memset(req, 0, sizeof(*req));
    req._hdr.tcm_ifindex = ifi;
    req._hdr.tcm_parent = TC_H_CLSACT;
    req._hdr.tcm_handle = TC_HANDLE;
    tc_newqdisc_req_set_nlflags(req,
    NLM_F_REQUEST | NLM_F_EXCL | NLM_F_CREATE);
    tc_newqdisc_req_set_kind(req, "clsact");
    ret = tc_newqdisc(ys, req);
    tc_newqdisc_req_free(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc_clsact_del(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    static int tc_clsact_del(struct ynl_sock *ys, int ifi)
    {
    struct tc_delqdisc_req *req;
    int ret;
    req = tc_delqdisc_req_alloc();
    if (!req)
    return -1;
    memset(req, 0, sizeof(*req));
    req._hdr.tcm_ifindex = ifi;
    req._hdr.tcm_parent = TC_H_CLSACT;
    req._hdr.tcm_handle = TC_HANDLE;
    tc_delqdisc_req_set_nlflags(req, NLM_F_REQUEST);
    ret = tc_delqdisc(ys, req);
    tc_delqdisc_req_free(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc_filter_add(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    static int tc_filter_add(struct ynl_sock *ys, int ifi)
    {
    struct tc_newtfilter_req *req;
    struct tc_act_attrs *acts;
    struct tc_vlan p = {
    .action = TC_ACT_PIPE,
    .v_action = TCA_VLAN_ACT_PUSH
    };
    int ret;
    req = tc_newtfilter_req_alloc();
    if (!req)
    return -1;
    memset(req, 0, sizeof(*req));
    acts = tc_act_attrs_alloc(3);
    if (!acts) {
    tc_newtfilter_req_free(req);
    return -1;
    }
    memset(acts, 0, sizeof(*acts) * 3);
    req._hdr.tcm_ifindex = ifi;
    req._hdr.tcm_parent = TC_H_MAKE(TC_H_CLSACT, TC_H_MIN_INGRESS);
    req._hdr.tcm_info = TC_H_MAKE(1 << 16, htons(ETH_P_8021Q));
    req.chain = 0;
    tc_newtfilter_req_set_nlflags(req, NLM_F_REQUEST | NLM_F_EXCL | NLM_F_CREATE);
    tc_newtfilter_req_set_kind(req, "flower");
    tc_newtfilter_req_set_options_flower_key_vlan_id(req, 100);
    tc_newtfilter_req_set_options_flower_key_vlan_prio(req, 5);
    tc_newtfilter_req_set_options_flower_key_num_of_vlans(req, 3);
    __tc_newtfilter_req_set_options_flower_act(req, acts, 3);
// Skip action at index 0 because in TC, the action array
// index starts at 1, with each index defining the action's
// order. In contrast, in YNL indexed arrays start at index 0.
//
    tc_act_attrs_set_kind(&acts[1], "vlan");
    tc_act_attrs_set_options_vlan_parms(&acts[1], &p, sizeof(p));
    tc_act_attrs_set_options_vlan_push_vlan_id(&acts[1], 200);
    tc_act_attrs_set_kind(&acts[2], "vlan");
    tc_act_attrs_set_options_vlan_parms(&acts[2], &p, sizeof(p));
    tc_act_attrs_set_options_vlan_push_vlan_id(&acts[2], 300);
    tc_newtfilter_req_set_options_flower_flags(req, 0);
    tc_newtfilter_req_set_options_flower_key_eth_type(req, htons(0x8100));
    ret = tc_newtfilter(ys, req);
    tc_newtfilter_req_free(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc_filter_del(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    static int tc_filter_del(struct ynl_sock *ys, int ifi)
    {
    struct tc_deltfilter_req *req;
    int ret;
    req = tc_deltfilter_req_alloc();
    if (!req)
    return -1;
    memset(req, 0, sizeof(*req));
    req._hdr.tcm_ifindex = ifi;
    req._hdr.tcm_parent = TC_H_MAKE(TC_H_CLSACT, TC_H_MIN_INGRESS);
    req._hdr.tcm_info = TC_H_MAKE(1 << 16, htons(ETH_P_8021Q));
    tc_deltfilter_req_set_nlflags(req, NLM_F_REQUEST);
    ret = tc_deltfilter(ys, req);
    tc_deltfilter_req_free(req);
    return ret;
    }
    FIXTURE(tc)
    {
    struct ynl_sock *ys;
    int ifindex;
    };
    FIXTURE_SETUP(tc)
    {
    struct ynl_error yerr;
    int ret;
    ret = unshare(CLONE_NEWNET);
    ASSERT_EQ(0, ret);
    self.ifindex = 1; /* loopback */
    self.ys = ynl_sock_create(&ynl_tc_family, &yerr);
    ASSERT_NE(core::ptr::null_mut(), self.ys) {
    TH_LOG("failed to create tc socket: %s", yerr.msg);
    }
    }
    FIXTURE_TEARDOWN(tc)
    {
    ynl_sock_destroy(self.ys);
    }
    TEST_F(tc, qdisc)
    {
    struct tc_getqdisc_req_dump *dreq;
    struct tc_newqdisc_req *add_req;
    struct tc_delqdisc_req *del_req;
    struct tc_getqdisc_list *rsp;
    let mut found: bool = false;
    int ret;
    add_req = tc_newqdisc_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), add_req);
    memset(add_req, 0, sizeof(*add_req));
    add_req._hdr.tcm_ifindex = self.ifindex;
    add_req._hdr.tcm_parent = TC_H_ROOT;
    tc_newqdisc_req_set_nlflags(add_req,
    NLM_F_REQUEST | NLM_F_CREATE);
    tc_newqdisc_req_set_kind(add_req, "fq_codel");
    ret = tc_newqdisc(self.ys, add_req);
    tc_newqdisc_req_free(add_req);
    ASSERT_EQ(0, ret) {
    TH_LOG("qdisc add failed: %s", self.ys.err.msg);
    }
    dreq = tc_getqdisc_req_dump_alloc();
    ASSERT_NE(core::ptr::null_mut(), dreq);
    rsp = tc_getqdisc_dump(self.ys, dreq);
    tc_getqdisc_req_dump_free(dreq);
    ASSERT_NE(core::ptr::null_mut(), rsp) {
    TH_LOG("dump failed: %s", self.ys.err.msg);
    }
    ASSERT_FALSE(ynl_dump_empty(rsp));
    ynl_dump_foreach(rsp, qdisc) {
    found |= tc_qdisc_print(_metadata, qdisc);
    }
    tc_getqdisc_list_free(rsp);
    EXPECT_TRUE(found);
    del_req = tc_delqdisc_req_alloc();
    ASSERT_NE(core::ptr::null_mut(), del_req);
    memset(del_req, 0, sizeof(*del_req));
    del_req._hdr.tcm_ifindex = self.ifindex;
    del_req._hdr.tcm_parent = TC_H_ROOT;
    tc_delqdisc_req_set_nlflags(del_req, NLM_F_REQUEST);
    ret = tc_delqdisc(self.ys, del_req);
    tc_delqdisc_req_free(del_req);
    EXPECT_EQ(0, ret) {
    TH_LOG("qdisc del failed: %s", self.ys.err.msg);
    }
    }
    TEST_F(tc, flower)
    {
    struct tc_gettfilter_req_dump *dreq;
    struct tc_gettfilter_list *rsp;
    let mut found: bool = false;
    int ret;
    ret = tc_clsact_add(self.ys, self.ifindex);
    if (ret)
    SKIP(return, "clsact not supported: %s", self.ys.err.msg);
    ret = tc_filter_add(self.ys, self.ifindex);
    ASSERT_EQ(0, ret) {
    TH_LOG("filter add failed: %s", self.ys.err.msg);
    }
    dreq = tc_gettfilter_req_dump_alloc();
    ASSERT_NE(core::ptr::null_mut(), dreq);
    memset(dreq, 0, sizeof(*dreq));
    dreq._hdr.tcm_ifindex = self.ifindex;
    dreq._hdr.tcm_parent = TC_H_MAKE(TC_H_CLSACT, TC_H_MIN_INGRESS);
    dreq._present.chain = 1;
    dreq.chain = 0;
    rsp = tc_gettfilter_dump(self.ys, dreq);
    tc_gettfilter_req_dump_free(dreq);
    ASSERT_NE(core::ptr::null_mut(), rsp) {
    TH_LOG("filter dump failed: %s", self.ys.err.msg);
    }
    ynl_dump_foreach(rsp, flt) {
    tc_filter_print(_metadata, flt);
    if (flt.options._present.flower) {
    EXPECT_EQ(100, flt.options.flower.key_vlan_id);
    EXPECT_EQ(5, flt.options.flower.key_vlan_prio);
    found = true;
    }
    }
    tc_gettfilter_list_free(rsp);
    EXPECT_TRUE(found);
    ret = tc_filter_del(self.ys, self.ifindex);
    EXPECT_EQ(0, ret) {
    TH_LOG("filter del failed: %s", self.ys.err.msg);
    }
    ret = tc_clsact_del(self.ys, self.ifindex);
    EXPECT_EQ(0, ret) {
    TH_LOG("clsact del failed: %s", self.ys.err.msg);
    }
    }
    TEST_HARNESS_MAIN
