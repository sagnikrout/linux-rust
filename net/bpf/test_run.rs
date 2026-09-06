//! Automatically rewritten from C to Rust
//! Source: net/bpf/test_run.c
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
// Copyright (c) 2017 Facebook
//

// Macro flag: #define CREATE_TRACE_POINTS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_timer {
    pub i: u32,
    pub time_spent: u64 time_start,,
}

#[no_mangle]
unsafe extern "C" fn bpf_test_timer_enter(t: *mut bpf_test_timer) {
    static void bpf_test_timer_enter(struct bpf_test_timer *t)
    __acquires(rcu)
    {
    rcu_read_lock_dont_migrate();
    t.time_start = ktime_get_ns();
    }
#[no_mangle]
unsafe extern "C" fn bpf_test_timer_leave(t: *mut bpf_test_timer) {
    static void bpf_test_timer_leave(struct bpf_test_timer *t)
    __releases(rcu)
    {
    t.time_start = 0;
    rcu_read_unlock_migrate();
    }
    static bool bpf_test_timer_continue(struct bpf_test_timer *t, int iterations,
    u32 repeat, int *err, u32 *duration)
    __must_hold(rcu)
    {
    t.i += iterations;
    if (t.i >= repeat) {
// We're done.
    t.time_spent += ktime_get_ns() - t.time_start;
    do_div(t.time_spent, t.i);
// duration = t->time_spent > U32_MAX ? U32_MAX : (u32)t->time_spent;
// err = 0;
    goto reset;
    }
    if (signal_pending(current)) {
// During iteration: we've been cancelled, abort.
// err = -EINTR;
    goto reset;
    }
    if (need_resched()) {
// During iteration: we need to reschedule between runs.
    t.time_spent += ktime_get_ns() - t.time_start;
    bpf_test_timer_leave(t);
    cond_resched();
    bpf_test_timer_enter(t);
    }
// Do another round.
    return true;
    reset:
    t.i = 0;
    return false;
    }
// We put this struct at the head of each page with a context and frame
// initialised when the page is allocated, so we don't have to do this on each
// repetition of the test run.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_page_head {
    pub orig_ctx: xdp_buff,
    pub ctx: xdp_buff,
    union {
// ::data_hard_start starts here
    pub frame): DECLARE_FLEX_ARRAY(struct xdp_frame,,
    pub data): DECLARE_FLEX_ARRAY(u8,,
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_test_data {
    pub orig_ctx: *mut xdp_buff,
    pub rxq: xdp_rxq_info,
    pub dev: *mut net_device,
    pub pp: *mut page_pool,
    pub frames: *mut xdp_frame,
    pub skbs: *mut sk_buff,
    pub mem: xdp_mem_info,
    pub batch_size: u32,
    pub frame_cnt: u32,
}

// tools/testing/selftests/bpf/prog_tests/xdp_do_redirect.c:%MAX_PKT_SIZE
// must be updated accordingly this gets changed, otherwise BPF selftests
// will fail.
//

pub const TEST_XDP_MAX_BATCH: c_int = 256;
#[no_mangle]
unsafe extern "C" fn xdp_test_run_init_page(netmem: netmem_ref, arg: *mut c_void) {
    static void xdp_test_run_init_page(netmem_ref netmem, void *arg)
    {
    struct xdp_page_head *head =
    phys_to_virt(page_to_phys(netmem_to_page(netmem)));
    struct xdp_buff *new_ctx, *orig_ctx;
    let mut headroom: u32 = XDP_PACKET_HEADROOM;
    struct xdp_test_data *xdp = arg;
    size_t frm_len, meta_len;
    struct xdp_frame *frm;
    void *data;
    orig_ctx = xdp.orig_ctx;
    frm_len = orig_ctx.data_end - orig_ctx.data_meta;
    meta_len = orig_ctx.data - orig_ctx.data_meta;
    headroom -= meta_len;
    new_ctx = &head.ctx;
    frm = head.frame;
    data = head.data;
    memcpy(data + headroom, orig_ctx.data_meta, frm_len);
    xdp_init_buff(new_ctx, TEST_XDP_FRAME_SIZE, &xdp.rxq);
    xdp_prepare_buff(new_ctx, data, headroom, frm_len, true);
    new_ctx.data = new_ctx.data_meta + meta_len;
    xdp_update_frame_from_buff(new_ctx, frm);
    frm.mem_type = new_ctx.rxq.mem.type;
    memcpy(&head.orig_ctx, new_ctx, sizeof(head.orig_ctx));
    }
#[no_mangle]
unsafe extern "C" fn xdp_test_run_setup(xdp: *mut xdp_test_data, orig_ctx: *mut xdp_buff) -> c_int {
    static int xdp_test_run_setup(struct xdp_test_data *xdp, struct xdp_buff *orig_ctx)
    {
    struct page_pool *pp;
    let mut err: c_int = -ENOMEM;
    struct page_pool_params pp_params = {
    .order = 0,
    .flags = 0,
    .pool_size = xdp.batch_size,
    .nid = NUMA_NO_NODE,
    .init_callback = xdp_test_run_init_page,
    .init_arg = xdp,
    };
    xdp.frames = kvmalloc_array(xdp.batch_size, sizeof(void *), GFP_KERNEL);
    if (!xdp.frames)
    return -ENOMEM;
    xdp.skbs = kvmalloc_array(xdp.batch_size, sizeof(void *), GFP_KERNEL);
    if (!xdp.skbs)
    goto err_skbs;
    pp = page_pool_create(&pp_params);
    if (IS_ERR(pp)) {
    err = PTR_ERR(pp);
    goto err_pp;
    }
// will copy 'mem.id' into pp->xdp_mem_id
    err = xdp_reg_mem_model(&xdp.mem, MEM_TYPE_PAGE_POOL, pp);
    if (err)
    goto err_mmodel;
    xdp.pp = pp;
// We create a 'fake' RXQ referencing the original dev, but with an
// xdp_mem_info pointing to our page_pool
//
    xdp_rxq_info_reg(&xdp.rxq, orig_ctx.rxq.dev, 0, 0);
    xdp.rxq.mem.type = MEM_TYPE_PAGE_POOL;
    xdp.rxq.mem.id = pp.xdp_mem_id;
    xdp.dev = orig_ctx.rxq.dev;
    xdp.orig_ctx = orig_ctx;
    return 0;
    err_mmodel:
    page_pool_destroy(pp);
    err_pp:
    kvfree(xdp.skbs);
    err_skbs:
    kvfree(xdp.frames);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn xdp_test_run_teardown(xdp: *mut xdp_test_data) {
    static void xdp_test_run_teardown(struct xdp_test_data *xdp)
    {
    xdp_unreg_mem_model(&xdp.mem);
    page_pool_destroy(xdp.pp);
    kfree(xdp.frames);
    kfree(xdp.skbs);
    }
#[no_mangle]
unsafe extern "C" fn frame_was_changed(head: *const xdp_page_head) -> bool {
    static bool frame_was_changed(const struct xdp_page_head *head)
    {
// xdp_scrub_frame() zeroes the data pointer, flags is the last field,
// i.e. has the highest chances to be overwritten. If those two are
// untouched, it's most likely safe to skip the context reset.
//
    return head.frame.data != head.orig_ctx.data ||
    head.frame.flags != head.orig_ctx.flags;
    }
#[no_mangle]
unsafe extern "C" fn ctx_was_changed(head: *mut xdp_page_head) -> bool {
    static bool ctx_was_changed(struct xdp_page_head *head)
    {
    return head.orig_ctx.data != head.ctx.data ||
    head.orig_ctx.data_meta != head.ctx.data_meta ||
    head.orig_ctx.data_end != head.ctx.data_end;
    }
#[no_mangle]
unsafe extern "C" fn reset_ctx(head: *mut xdp_page_head) {
    static void reset_ctx(struct xdp_page_head *head)
    {
    if (likely(!frame_was_changed(head) && !ctx_was_changed(head)))
    return;
    head.ctx.data = head.orig_ctx.data;
    head.ctx.data_meta = head.orig_ctx.data_meta;
    head.ctx.data_end = head.orig_ctx.data_end;
    xdp_update_frame_from_buff(&head.ctx, head.frame);
    head.frame.mem_type = head.orig_ctx.rxq.mem.type;
    }
    static int xdp_recv_frames(struct xdp_frame **frames, int nframes,
    struct sk_buff **skbs,
    struct net_device *dev)
    {
    let mut gfp: gfp_t = __GFP_ZERO | GFP_ATOMIC;
    int i;
    LIST_HEAD(list);
    if (unlikely(!kmem_cache_alloc_bulk(net_hotdata.skbuff_cache, gfp,
    nframes, (void **)skbs))) {
    for (i = 0; i < nframes; i++)
    xdp_return_frame(frames[i]);
    return -ENOMEM;
    }
    for (i = 0; i < nframes; i++) {
    struct xdp_frame *xdpf = frames[i];
    struct sk_buff *skb = skbs[i];
    skb = __xdp_build_skb_from_frame(xdpf, skb, dev);
    if (!skb) {
    xdp_return_frame(xdpf);
    continue;
    }
    list_add_tail(&skb.list, &list);
    }
    netif_receive_skb_list(&list);
    return 0;
    }
    static int xdp_test_run_batch(struct xdp_test_data *xdp, struct bpf_prog *prog,
    u32 repeat)
    {
    struct bpf_net_context __bpf_net_ctx, *bpf_net_ctx;
    let mut err: c_int = 0, act, ret, i, nframes = 0, batch_sz;
    struct xdp_frame **frames = xdp.frames;
    struct bpf_redirect_info *ri;
    struct xdp_page_head *head;
    struct xdp_frame *frm;
    let mut redirect: bool = false;
    struct xdp_buff *ctx;
    struct page *page;
    batch_sz = min_t(u32, repeat, xdp.batch_size);
    local_bh_disable();
    bpf_net_ctx = bpf_net_ctx_set(&__bpf_net_ctx);
    ri = bpf_net_ctx_get_ri();
    xdp_set_return_frame_no_direct();
    for (i = 0; i < batch_sz; i++) {
    page = page_pool_dev_alloc_pages(xdp.pp);
    if (!page) {
    err = -ENOMEM;
    goto out;
    }
    head = phys_to_virt(page_to_phys(page));
    reset_ctx(head);
    ctx = &head.ctx;
    frm = head.frame;
    xdp.frame_cnt++;
    act = bpf_prog_run_xdp(prog, ctx);
// if program changed pkt bounds we need to update the xdp_frame
    if (unlikely(ctx_was_changed(head))) {
    ret = xdp_update_frame_from_buff(ctx, frm);
    if (ret) {
    xdp_return_buff(ctx);
    continue;
    }
    }
    switch (act) {
    case XDP_TX:
// we can't do a real XDP_TX since we're not in the
// driver, so turn it into a REDIRECT back to the same
// index
//
    ri.tgt_index = xdp.dev.ifindex;
    ri.map_id = INT_MAX;
    ri.map_type = BPF_MAP_TYPE_UNSPEC;
    fallthrough;
    case XDP_REDIRECT:
    redirect = true;
    ret = xdp_do_redirect_frame(xdp.dev, ctx, frm, prog);
    if (ret)
    xdp_return_buff(ctx);
    break;
    case XDP_PASS:
    frames[nframes++] = frm;
    break;
    default:
    bpf_warn_invalid_xdp_action(core::ptr::null_mut(), prog, act);
    fallthrough;
    case XDP_DROP:
    xdp_return_buff(ctx);
    break;
    }
    }
    out:
    if (redirect)
    xdp_do_flush();
    if (nframes) {
    ret = xdp_recv_frames(frames, nframes, xdp.skbs, xdp.dev);
    if (ret)
    err = ret;
    }
    xdp_clear_return_frame_no_direct();
    bpf_net_ctx_clear(bpf_net_ctx);
    local_bh_enable();
    return err;
    }
    static int bpf_test_run_xdp_live(struct bpf_prog *prog, struct xdp_buff *ctx,
    u32 repeat, u32 batch_size, u32 *time)
    {
    let mut xdp: xdp_test_data = { .batch_size = batch_size };
    let mut t: bpf_test_timer = {};
    int ret;
    if (!repeat)
    repeat = 1;
    ret = xdp_test_run_setup(&xdp, ctx);
    if (ret)
    return ret;
    bpf_test_timer_enter(&t);
    do {
    xdp.frame_cnt = 0;
    ret = xdp_test_run_batch(&xdp, prog, repeat - t.i);
    if (unlikely(ret < 0))
    break;
    } while (bpf_test_timer_continue(&t, xdp.frame_cnt, repeat, &ret, time));
    bpf_test_timer_leave(&t);
    xdp_test_run_teardown(&xdp);
    return ret;
    }
    static int bpf_test_run(struct bpf_prog *prog, void *ctx, u32 repeat,
    u32 *retval, u32 *time, bool xdp)
    {
    struct bpf_net_context __bpf_net_ctx, *bpf_net_ctx;
    let mut item: bpf_prog_array_item = {.prog = prog};
    struct bpf_run_ctx *old_ctx;
    struct bpf_cg_run_ctx run_ctx;
    let mut t: bpf_test_timer = {};
    enum bpf_cgroup_storage_type stype;
    int ret;
    for_each_cgroup_storage_type(stype) {
    item.cgroup_storage[stype] = bpf_cgroup_storage_alloc(prog, stype);
    if (IS_ERR(item.cgroup_storage[stype])) {
    item.cgroup_storage[stype] = core::ptr::null_mut();
    for_each_cgroup_storage_type(stype)
    bpf_cgroup_storage_free(item.cgroup_storage[stype]);
    return -ENOMEM;
    }
    }
    if (!repeat)
    repeat = 1;
    bpf_test_timer_enter(&t);
    old_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    do {
    run_ctx.prog_item = &item;
    local_bh_disable();
    bpf_net_ctx = bpf_net_ctx_set(&__bpf_net_ctx);
    if (xdp)
// retval = bpf_prog_run_xdp(prog, ctx);
    else
// retval = bpf_prog_run(prog, ctx);
    bpf_net_ctx_clear(bpf_net_ctx);
    local_bh_enable();
    } while (bpf_test_timer_continue(&t, 1, repeat, &ret, time));
    bpf_reset_run_ctx(old_ctx);
    bpf_test_timer_leave(&t);
    for_each_cgroup_storage_type(stype)
    bpf_cgroup_storage_free(item.cgroup_storage[stype]);
    return ret;
    }
    static int bpf_test_finish(const union bpf_attr *kattr,
    union bpf_attr __user *uattr, const void *data,
    struct skb_shared_info *sinfo, u32 size, u32 frag_size,
    u32 retval, u32 duration)
    {
    void __user *data_out = u64_to_user_ptr(kattr.test.data_out);
    let mut err: c_int = -EFAULT;
    let mut copy_size: u32 = size;
// Clamp copy if the user has provided a size hint, but copy the full
// buffer if not to retain old behaviour.
//
    if (kattr.test.data_size_out &&
    copy_size > kattr.test.data_size_out) {
    copy_size = kattr.test.data_size_out;
    err = -ENOSPC;
    }
    if (data_out) {
    let mut head_len: u32 = size - frag_size;
    let mut len: u32 = min(copy_size, head_len);
    if (copy_to_user(data_out, data, len))
    goto out;
    if (sinfo) {
    int i, offset = len;
    u32 data_len;
    for (i = 0; i < sinfo.nr_frags; i++) {
    skb_frag_t *frag = &sinfo.frags[i];
    if (offset >= copy_size) {
    err = -ENOSPC;
    break;
    }
    data_len = min_t(u32, copy_size - offset,
    skb_frag_size(frag));
    if (copy_to_user(data_out + offset,
    skb_frag_address(frag),
    data_len))
    goto out;
    offset += data_len;
    }
    }
    }
    if (copy_to_user(&uattr.test.data_size_out, &size, sizeof(size)))
    goto out;
    if (copy_to_user(&uattr.test.retval, &retval, sizeof(retval)))
    goto out;
    if (copy_to_user(&uattr.test.duration, &duration, sizeof(duration)))
    goto out;
    if (err != -ENOSPC)
    err = 0;
    out:
    trace_bpf_test_finish(&err);
    return err;
    }
// Integer types of various sizes and pointer combinations cover variety of
// architecture dependent calling conventions. 7+ can be supported in the
// future.
//
    __bpf_kfunc_start_defs();
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test1(a: c_int) -> __bpf_kfunc int {
    __bpf_kfunc int bpf_fentry_test1(int a)
    {
    return a + 1;
    }
    EXPORT_SYMBOL_GPL(bpf_fentry_test1);
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test2(a: c_int, b: u64) -> noinline int {
    noinline int bpf_fentry_test2(int a, u64 b)
    {
    return a + b;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test3(a: c_char, b: c_int, c: u64) -> noinline int {
    noinline int bpf_fentry_test3(char a, int b, u64 c)
    {
    return a + b + c;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test4(a: *mut c_void, b: c_char, c: c_int, d: u64) -> noinline int {
    noinline int bpf_fentry_test4(void *a, char b, int c, u64 d)
    {
    return (long)a + b + c + d;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test5(a: u64, b: *mut c_void, c: c_short, d: c_int, e: u64) -> noinline int {
    noinline int bpf_fentry_test5(u64 a, void *b, short c, int d, u64 e)
    {
    return a + (long)b + c + d + e;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test6(a: u64, b: *mut c_void, c: c_short, d: c_int, e: *mut c_void, f: u64) -> noinline int {
    noinline int bpf_fentry_test6(u64 a, void *b, short c, int d, void *e, u64 f)
    {
    return a + (long)b + c + d + (long)e + f;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fentry_test_t {
    pub a: *mut bpf_fentry_test_t,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test7(arg: *mut bpf_fentry_test_t) -> noinline int {
    noinline int bpf_fentry_test7(struct bpf_fentry_test_t *arg)
    {
    asm volatile ("" : "+r"(arg));
    return (long)arg;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test8(arg: *mut bpf_fentry_test_t) -> noinline int {
    noinline int bpf_fentry_test8(struct bpf_fentry_test_t *arg)
    {
    return (long)arg.a;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test9(a: *mut u32) -> __bpf_kfunc u32 {
    __bpf_kfunc u32 bpf_fentry_test9(u32 *a)
    {
    return *a;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test10(a: *const c_void) -> noinline int {
    noinline int bpf_fentry_test10(const void *a)
    {
    return (long)a;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test_sinfo(sinfo: *mut skb_shared_info) -> noinline void {
    noinline void bpf_fentry_test_sinfo(struct skb_shared_info *sinfo)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test_ppvoid(pp: *mut c_void) -> noinline void {
    noinline void bpf_fentry_test_ppvoid(void **pp)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test_pppvoid(ppp: *mut c_void) -> noinline void {
    noinline void bpf_fentry_test_pppvoid(void ***ppp)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_test_ppfile(ppf: *mut file) -> noinline void {
    noinline void bpf_fentry_test_ppfile(struct file **ppf)
    {
    }
    noinline struct file **bpf_fexit_test_ret_ppfile(void)
    {
    return (struct file **)core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_modify_return_test(a: c_int, b: *mut c_int) -> __bpf_kfunc int {
    __bpf_kfunc int bpf_modify_return_test(int a, int *b)
    {
// b += 1;
    return a + *b;
    }
    __bpf_kfunc int bpf_modify_return_test2(int a, int *b, short c, int d,
    void *e, char f, int g)
    {
// b += 1;
    return a + *b + c + d + (long)e + f + g;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_modify_return_test_tp(nonce: c_int) -> __bpf_kfunc int {
    __bpf_kfunc int bpf_modify_return_test_tp(int nonce)
    {
    trace_bpf_trigger_tp(nonce);
    return nonce;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fentry_shadow_test(a: c_int) -> noinline int {
    noinline int bpf_fentry_shadow_test(int a)
    {
    return a + 1;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_member1 {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_member {
    pub m: prog_test_member1,
    pub c: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_ref_kfunc {
    pub a: c_int,
    pub b: c_int,
    pub memb: prog_test_member,
    pub next: *mut prog_test_ref_kfunc,
    pub cnt: refcount_t,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_kfunc_call_test_release(p: *mut prog_test_ref_kfunc) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_kfunc_call_test_release(struct prog_test_ref_kfunc *p)
    {
    refcount_dec(&p.cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_kfunc_call_test_release_dtor(p: *mut c_void) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_kfunc_call_test_release_dtor(void *p)
    {
    bpf_kfunc_call_test_release(p);
    }
    CFI_NOSEAL(bpf_kfunc_call_test_release_dtor);
#[no_mangle]
pub unsafe extern "C" fn bpf_kfunc_call_memb_release(p: *mut prog_test_member) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_kfunc_call_memb_release(struct prog_test_member *p)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_kfunc_call_memb_release_dtor(p: *mut c_void) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_kfunc_call_memb_release_dtor(void *p)
    {
    }
    CFI_NOSEAL(bpf_kfunc_call_memb_release_dtor);
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(bpf_test_modify_return_ids)
    BTF_ID_FLAGS(func, bpf_modify_return_test)
    BTF_ID_FLAGS(func, bpf_modify_return_test2)
    BTF_ID_FLAGS(func, bpf_modify_return_test_tp)
    BTF_ID_FLAGS(func, bpf_fentry_test1, KF_SLEEPABLE)
    BTF_KFUNCS_END(bpf_test_modify_return_ids)
    static const struct btf_kfunc_id_set bpf_test_modify_return_set = {
    .owner = THIS_MODULE,
    .set   = &bpf_test_modify_return_ids,
    };
    BTF_KFUNCS_START(test_sk_check_kfunc_ids)
    BTF_ID_FLAGS(func, bpf_kfunc_call_test_release, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_kfunc_call_memb_release, KF_RELEASE)
    BTF_KFUNCS_END(test_sk_check_kfunc_ids)
    static void *bpf_test_init(const union bpf_attr *kattr, u32 user_size,
    u32 size, u32 headroom, u32 tailroom)
    {
    void __user *data_in = u64_to_user_ptr(kattr.test.data_in);
    void *data;
    if (user_size > PAGE_SIZE - headroom - tailroom)
    return ERR_PTR(-EINVAL);
    size = SKB_DATA_ALIGN(size);
    data = kzalloc(size + headroom + tailroom, GFP_USER);
    if (!data)
    return ERR_PTR(-ENOMEM);
    if (copy_from_user(data + headroom, data_in, user_size)) {
    kfree(data);
    return ERR_PTR(-EFAULT);
    }
    return data;
    }
    int bpf_prog_test_run_tracing(struct bpf_prog *prog,
    const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    let mut arg: bpf_fentry_test_t = {};
    let mut side_effect: u16 = 0, ret = 0;
    let mut b: c_int = 2, err = -EFAULT;
    let mut retval: u32 = 0;
    if (kattr.test.flags || kattr.test.cpu || kattr.test.batch_size)
    return -EINVAL;
    switch (prog.expected_attach_type) {
    case BPF_TRACE_FENTRY:
    case BPF_TRACE_FEXIT:
    case BPF_TRACE_FSESSION:
    case BPF_TRACE_FENTRY_MULTI:
    case BPF_TRACE_FEXIT_MULTI:
    case BPF_TRACE_FSESSION_MULTI:
    if (bpf_fentry_test1(1) != 2 ||
    bpf_fentry_test2(2, 3) != 5 ||
    bpf_fentry_test3(4, 5, 6) != 15 ||
    bpf_fentry_test4((void *)7, 8, 9, 10) != 34 ||
    bpf_fentry_test5(11, (void *)12, 13, 14, 15) != 65 ||
    bpf_fentry_test6(16, (void *)17, 18, 19, (void *)20, 21) != 111 ||
    bpf_fentry_test7((struct bpf_fentry_test_t *)0) != 0 ||
    bpf_fentry_test8(&arg) != 0 ||
    bpf_fentry_test9(&retval) != 0 ||
    bpf_fentry_test10((void *)0) != 0)
    goto out;
    break;
    case BPF_MODIFY_RETURN:
    ret = bpf_modify_return_test(1, &b);
    if (b != 2)
    side_effect++;
    b = 2;
    ret += bpf_modify_return_test2(1, &b, 3, 4, (void *)5, 6, 7);
    if (b != 2)
    side_effect++;
    break;
    default:
    goto out;
    }
    retval = ((u32)side_effect << 16) | ret;
    if (copy_to_user(&uattr.test.retval, &retval, sizeof(retval)))
    goto out;
    err = 0;
    out:
    trace_bpf_test_finish(&err);
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_raw_tp_test_run_info {
    pub prog: *mut bpf_prog,
    pub ctx: *mut c_void,
    pub retval: u32,
}

    static void
    __bpf_prog_test_run_raw_tp(void *data)
    {
    struct bpf_raw_tp_test_run_info *info = data;
    struct srcu_ctr __percpu *scp = core::ptr::null_mut();
    let mut run_ctx: bpf_trace_run_ctx = {};
    struct bpf_run_ctx *old_run_ctx;
    old_run_ctx = bpf_set_run_ctx(&run_ctx.run_ctx);
    if (info.prog.sleepable) {
    scp = rcu_read_lock_tasks_trace();
    migrate_disable();
    } else {
    rcu_read_lock();
    }
    if (unlikely(!bpf_prog_get_recursion_context(info.prog))) {
    bpf_prog_inc_misses_counter(info.prog);
    goto out;
    }
    info.retval = bpf_prog_run(info.prog, info.ctx);
    out:
    bpf_prog_put_recursion_context(info.prog);
    if (info.prog.sleepable) {
    migrate_enable();
    rcu_read_unlock_tasks_trace(scp);
    } else {
    rcu_read_unlock();
    }
    bpf_reset_run_ctx(old_run_ctx);
    }
    int bpf_prog_test_run_raw_tp(struct bpf_prog *prog,
    const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    void __user *ctx_in = u64_to_user_ptr(kattr.test.ctx_in);
    let mut ctx_size_in: __u32 = kattr.test.ctx_size_in;
    struct bpf_raw_tp_test_run_info info;
    let mut cpu: c_int = kattr.test.cpu, err = 0;
    int current_cpu;
// doesn't support data_in/out, ctx_out, duration, or repeat
    if (kattr.test.data_in || kattr.test.data_out ||
    kattr.test.ctx_out || kattr.test.duration ||
    kattr.test.repeat || kattr.test.batch_size)
    return -EINVAL;
    if (ctx_size_in < prog.aux.max_ctx_offset ||
    ctx_size_in > MAX_BPF_FUNC_ARGS * sizeof(u64))
    return -EINVAL;
    if ((kattr.test.flags & BPF_F_TEST_RUN_ON_CPU) == 0 && cpu != 0)
    return -EINVAL;
//
// Sleepable programs cannot run with preemption disabled or in
// hardirq context (smp_call_function_single), reject the flag.
//
    if (prog.sleepable && (kattr.test.flags & BPF_F_TEST_RUN_ON_CPU))
    return -EINVAL;
    if (ctx_size_in) {
    info.ctx = memdup_user(ctx_in, ctx_size_in);
    if (IS_ERR(info.ctx))
    return PTR_ERR(info.ctx);
    } else {
    info.ctx = core::ptr::null_mut();
    }
    info.retval = 0;
    info.prog = prog;
    if (prog.sleepable) {
    __bpf_prog_test_run_raw_tp(&info);
    } else {
    current_cpu = get_cpu();
    if ((kattr.test.flags & BPF_F_TEST_RUN_ON_CPU) == 0 ||
    cpu == current_cpu) {
    __bpf_prog_test_run_raw_tp(&info);
    } else if (cpu >= nr_cpu_ids || !cpu_online(cpu)) {
//
// smp_call_function_single() also checks cpu_online()
// after csd_lock(). However, since cpu is from user
// space, let's do an extra quick check to filter out
// invalid value before smp_call_function_single().
//
    err = -ENXIO;
    } else {
    err = smp_call_function_single(cpu,
    __bpf_prog_test_run_raw_tp,
    &info, 1);
    }
    put_cpu();
    }
    if (!err &&
    copy_to_user(&uattr.test.retval, &info.retval, sizeof(u32)))
    err = -EFAULT;
    kfree(info.ctx);
    return err;
    }
    static void *bpf_ctx_init(const union bpf_attr *kattr, u32 max_size)
    {
    void __user *data_in = u64_to_user_ptr(kattr.test.ctx_in);
    void __user *data_out = u64_to_user_ptr(kattr.test.ctx_out);
    let mut size: u32 = kattr.test.ctx_size_in;
    void *data;
    int err;
    if (!data_in && !data_out)
    return core::ptr::null_mut();
    data = kzalloc(max_size, GFP_USER);
    if (!data)
    return ERR_PTR(-ENOMEM);
    if (data_in) {
    err = bpf_check_uarg_tail_zero(USER_BPFPTR(data_in), max_size, size);
    if (err) {
    kfree(data);
    return ERR_PTR(err);
    }
    size = min_t(u32, max_size, size);
    if (copy_from_user(data, data_in, size)) {
    kfree(data);
    return ERR_PTR(-EFAULT);
    }
    }
    return data;
    }
    static int bpf_ctx_finish(const union bpf_attr *kattr,
    union bpf_attr __user *uattr, const void *data,
    u32 size)
    {
    void __user *data_out = u64_to_user_ptr(kattr.test.ctx_out);
    let mut err: c_int = -EFAULT;
    let mut copy_size: u32 = size;
    if (!data || !data_out)
    return 0;
    if (copy_size > kattr.test.ctx_size_out) {
    copy_size = kattr.test.ctx_size_out;
    err = -ENOSPC;
    }
    if (copy_to_user(data_out, data, copy_size))
    goto out;
    if (copy_to_user(&uattr.test.ctx_size_out, &size, sizeof(size)))
    goto out;
    if (err != -ENOSPC)
    err = 0;
    out:
    return err;
    }
//
// range_is_zero - test whether buffer is initialized
// @buf: buffer to check
// @from: check from this position
// @to: check up until (excluding) this position
//
// This function returns true if the there is a non-zero byte
// in the buf in the range [from,to).
//
#[no_mangle]
pub unsafe extern "C" fn range_is_zero(buf: *mut c_void, from: usize, to: usize) -> bool {
    static inline bool range_is_zero(void *buf, size_t from, size_t to)
    {
    return !memchr_inv((u8 *)buf + from, 0, to - from);
    }
#[no_mangle]
unsafe extern "C" fn convert___skb_to_skb(skb: *mut sk_buff, __skb: *mut __sk_buff) -> c_int {
    static int convert___skb_to_skb(struct sk_buff *skb, struct __sk_buff *__skb)
    {
    struct qdisc_skb_cb *cb = (struct qdisc_skb_cb *)skb.cb;
    if (!__skb)
    return 0;
// make sure the fields we don't use are zeroed
    if (!range_is_zero(__skb, 0, offsetof(struct __sk_buff, mark)))
    return -EINVAL;
// mark is allowed
    if (!range_is_zero(__skb, offsetofend(struct __sk_buff, mark),
    offsetof(struct __sk_buff, priority)))
    return -EINVAL;
// priority is allowed
// ingress_ifindex is allowed
// ifindex is allowed
    if (!range_is_zero(__skb, offsetofend(struct __sk_buff, ifindex),
    offsetof(struct __sk_buff, cb)))
    return -EINVAL;
// cb is allowed
    if (!range_is_zero(__skb, offsetofend(struct __sk_buff, cb),
    offsetof(struct __sk_buff, data_end)))
    return -EINVAL;
// data_end is allowed, but not copied to skb
    if (!range_is_zero(__skb, offsetofend(struct __sk_buff, data_end),
    offsetof(struct __sk_buff, tstamp)))
    return -EINVAL;
// tstamp is allowed
// wire_len is allowed
// gso_segs is allowed
    if (!range_is_zero(__skb, offsetofend(struct __sk_buff, gso_segs),
    offsetof(struct __sk_buff, gso_size)))
    return -EINVAL;
// gso_size is allowed
    if (!range_is_zero(__skb, offsetofend(struct __sk_buff, gso_size),
    offsetof(struct __sk_buff, hwtstamp)))
    return -EINVAL;
// hwtstamp is allowed
    if (!range_is_zero(__skb, offsetofend(struct __sk_buff, hwtstamp),
    sizeof(struct __sk_buff)))
    return -EINVAL;
    skb.mark = __skb.mark;
    skb.priority = __skb.priority;
    skb.skb_iif = __skb.ingress_ifindex;
    skb.tstamp = __skb.tstamp;
    memcpy(&cb.data, __skb.cb, QDISC_CB_PRIV_LEN);
    if (__skb.wire_len == 0) {
    cb.pkt_len = skb.len;
    } else {
    if (__skb.wire_len < skb.len ||
    __skb.wire_len > GSO_LEGACY_MAX_SIZE)
    return -EINVAL;
    cb.pkt_len = __skb.wire_len;
    }
    if (__skb.gso_segs > GSO_MAX_SEGS)
    return -EINVAL;
// Currently GSO type is zero/unset. If this gets extended with
// a small list of accepted GSO types in future, the filter for
// an unset GSO type in bpf_clone_redirect() can be lifted.
//
    skb_shinfo(skb).gso_segs = __skb.gso_segs;
    skb_shinfo(skb).gso_size = __skb.gso_size;
    skb_shinfo(skb).hwtstamps.hwtstamp = __skb.hwtstamp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn convert_skb_to___skb(skb: *mut sk_buff, __skb: *mut __sk_buff) {
    static void convert_skb_to___skb(struct sk_buff *skb, struct __sk_buff *__skb)
    {
    struct qdisc_skb_cb *cb = (struct qdisc_skb_cb *)skb.cb;
    if (!__skb)
    return;
    __skb.mark = skb.mark;
    __skb.priority = skb.priority;
    __skb.ingress_ifindex = skb.skb_iif;
    __skb.ifindex = skb.dev.ifindex;
    __skb.tstamp = skb.tstamp;
    memcpy(__skb.cb, &cb.data, QDISC_CB_PRIV_LEN);
    __skb.wire_len = cb.pkt_len;
    __skb.gso_segs = skb_shinfo(skb).gso_segs;
    __skb.hwtstamp = skb_shinfo(skb).hwtstamps.hwtstamp;
    }
    static struct proto bpf_dummy_proto = {
    .name   = "bpf_dummy",
    .owner  = THIS_MODULE,
    .obj_size = sizeof(struct sock),
    };
    int bpf_prog_test_run_skb(struct bpf_prog *prog, const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    let mut is_l2: bool = false, is_direct_pkt_access = false, is_lwt = false;
    let mut tailroom: u32 = SKB_DATA_ALIGN(sizeof(struct skb_shared_info));
    struct net *net = current.nsproxy.net_ns;
    struct net_device *dev = net.loopback_dev;
    let mut headroom: u32 = NET_SKB_PAD + NET_IP_ALIGN;
    let mut linear_sz: u32 = kattr.test.data_size_in;
    let mut repeat: u32 = kattr.test.repeat;
    struct __sk_buff *ctx = core::ptr::null_mut();
    struct sk_buff *skb = core::ptr::null_mut();
    struct sock *sk = core::ptr::null_mut();
    u32 retval, duration;
    let mut hh_len: c_int = ETH_HLEN;
    void *data = core::ptr::null_mut();
    int ret;
    if ((kattr.test.flags & ~BPF_F_TEST_SKB_CHECKSUM_COMPLETE) ||
    kattr.test.cpu || kattr.test.batch_size)
    return -EINVAL;
    if (kattr.test.data_size_in < ETH_HLEN)
    return -EINVAL;
    switch (prog.type) {
    case BPF_PROG_TYPE_SCHED_CLS:
    case BPF_PROG_TYPE_SCHED_ACT:
    is_direct_pkt_access = true;
    is_l2 = true;
    break;
    case BPF_PROG_TYPE_LWT_IN:
    case BPF_PROG_TYPE_LWT_OUT:
    case BPF_PROG_TYPE_LWT_XMIT:
    is_lwt = true;
    fallthrough;
    case BPF_PROG_TYPE_CGROUP_SKB:
    is_direct_pkt_access = true;
    break;
    default:
    break;
    }
    ctx = bpf_ctx_init(kattr, sizeof(struct __sk_buff));
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    if (ctx) {
    if (ctx.data_end > kattr.test.data_size_in || ctx.data || ctx.data_meta) {
    ret = -EINVAL;
    goto out;
    }
    if (ctx.data_end) {
// Non-linear LWT test_run is unsupported for now.
    if (is_lwt) {
    ret = -EINVAL;
    goto out;
    }
    linear_sz = max(ETH_HLEN, ctx.data_end);
    }
    }
    linear_sz = min_t(u32, linear_sz, PAGE_SIZE - headroom - tailroom);
    data = bpf_test_init(kattr, linear_sz, linear_sz, headroom, tailroom);
    if (IS_ERR(data)) {
    ret = PTR_ERR(data);
    data = core::ptr::null_mut();
    goto out;
    }
    sk = sk_alloc(net, AF_UNSPEC, GFP_USER, &bpf_dummy_proto, 1);
    if (!sk) {
    ret = -ENOMEM;
    goto out;
    }
    sock_init_data(core::ptr::null_mut(), sk);
    skb = slab_build_skb(data);
    if (!skb) {
    ret = -ENOMEM;
    goto out;
    }
    skb.sk = sk;
    data = core::ptr::null_mut(); /* data released via kfree_skb */
    skb_reserve(skb, NET_SKB_PAD + NET_IP_ALIGN);
    __skb_put(skb, linear_sz);
    if (unlikely(kattr.test.data_size_in > linear_sz)) {
    void __user *data_in = u64_to_user_ptr(kattr.test.data_in);
    struct skb_shared_info *sinfo = skb_shinfo(skb);
    let mut copied: u32 = linear_sz;
    while (copied < kattr.test.data_size_in) {
    struct page *page;
    u32 data_len;
    if (sinfo.nr_frags == MAX_SKB_FRAGS) {
    ret = -ENOMEM;
    goto out;
    }
    page = alloc_page(GFP_KERNEL);
    if (!page) {
    ret = -ENOMEM;
    goto out;
    }
    data_len = min_t(u32, kattr.test.data_size_in - copied,
    PAGE_SIZE);
    skb_fill_page_desc(skb, sinfo.nr_frags, page, 0, data_len);
    if (copy_from_user(page_address(page), data_in + copied,
    data_len)) {
    ret = -EFAULT;
    goto out;
    }
    skb.data_len += data_len;
    skb.truesize += PAGE_SIZE;
    skb.len += data_len;
    copied += data_len;
    }
    }
    if (ctx && ctx.ifindex > 1) {
    dev = dev_get_by_index(net, ctx.ifindex);
    if (!dev) {
    ret = -ENODEV;
    goto out;
    }
    }
    skb.protocol = eth_type_trans(skb, dev);
    skb_reset_network_header(skb);
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    if (skb_headlen(skb) < sizeof(struct iphdr)) {
    ret = -EINVAL;
    goto out;
    }
    sk.sk_family = AF_INET;
    sk.sk_rcv_saddr = ip_hdr(skb).saddr;
    sk.sk_daddr = ip_hdr(skb).daddr;
    break;

    case htons(ETH_P_IPV6):
    if (skb_headlen(skb) < sizeof(struct ipv6hdr)) {
    ret = -EINVAL;
    goto out;
    }
    sk.sk_family = AF_INET6;
    sk.sk_v6_rcv_saddr = ipv6_hdr(skb).saddr;
    sk.sk_v6_daddr = ipv6_hdr(skb).daddr;
    break;

    default:
    break;
    }
    if (is_l2)
    __skb_push(skb, hh_len);
    if (is_direct_pkt_access)
    bpf_compute_data_pointers(skb);
    ret = convert___skb_to_skb(skb, ctx);
    if (ret)
    goto out;
    if (kattr.test.flags & BPF_F_TEST_SKB_CHECKSUM_COMPLETE) {
    let mut off: c_int = skb_network_offset(skb);
    let mut len: c_int = skb.len - off;
    skb.csum = skb_checksum(skb, off, len, 0);
    skb.ip_summed = CHECKSUM_COMPLETE;
    }
    if (prog.type == BPF_PROG_TYPE_LWT_XMIT) {
    if (!ipv6_mod_enabled()) {
    pr_warn_once("Please test this program with IPv6 enabled kernel\n");
    ret = -EOPNOTSUPP;
    goto out;
    }

    dst_hold(&net.ipv6.ip6_null_entry.dst);
    skb_dst_set(skb, &net.ipv6.ip6_null_entry.dst);

    }
    ret = bpf_test_run(prog, skb, repeat, &retval, &duration, false);
    if (ret)
    goto out;
    if (!is_l2) {
    if (skb_headroom(skb) < hh_len) {
    let mut nhead: c_int = HH_DATA_ALIGN(hh_len - skb_headroom(skb));
    if (pskb_expand_head(skb, nhead, 0, GFP_USER)) {
    ret = -ENOMEM;
    goto out;
    }
    }
    memset(__skb_push(skb, hh_len), 0, hh_len);
    }
    if (kattr.test.flags & BPF_F_TEST_SKB_CHECKSUM_COMPLETE) {
    let mut off: c_int = skb_network_offset(skb);
    let mut len: c_int = skb.len - off;
    __wsum csum;
    csum = skb_checksum(skb, off, len, 0);
    if (csum_fold(skb.csum) != csum_fold(csum)) {
    ret = -EBADMSG;
    goto out;
    }
    }
    convert_skb_to___skb(skb, ctx);
    if (skb_is_nonlinear(skb))
// bpf program can never convert linear skb to non-linear
    WARN_ON_ONCE(linear_sz == kattr.test.data_size_in);
    ret = bpf_test_finish(kattr, uattr, skb.data, skb_shinfo(skb), skb.len,
    skb.data_len, retval, duration);
    if (!ret)
    ret = bpf_ctx_finish(kattr, uattr, ctx,
    sizeof(struct __sk_buff));
    out:
    if (dev && dev != net.loopback_dev)
    dev_put(dev);
    kfree_skb(skb);
    kfree(data);
    if (sk)
    sk_free(sk);
    kfree(ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xdp_convert_md_to_buff(xdp_md: *mut xdp_md, xdp: *mut xdp_buff) -> c_int {
    static int xdp_convert_md_to_buff(struct xdp_md *xdp_md, struct xdp_buff *xdp)
    {
    unsigned int ingress_ifindex, rx_queue_index;
    struct netdev_rx_queue *rxqueue;
    struct net_device *device;
    if (!xdp_md)
    return 0;
    if (xdp_md.egress_ifindex != 0)
    return -EINVAL;
    ingress_ifindex = xdp_md.ingress_ifindex;
    rx_queue_index = xdp_md.rx_queue_index;
    if (!ingress_ifindex && rx_queue_index)
    return -EINVAL;
    if (ingress_ifindex) {
    device = dev_get_by_index(current.nsproxy.net_ns,
    ingress_ifindex);
    if (!device)
    return -ENODEV;
    if (rx_queue_index >= device.real_num_rx_queues)
    goto free_dev;
    rxqueue = __netif_get_rx_queue(device, rx_queue_index);
    if (!xdp_rxq_info_is_reg(&rxqueue.xdp_rxq))
    goto free_dev;
    xdp.rxq = &rxqueue.xdp_rxq;
// The device is now tracked in the xdp->rxq for later
// dev_put()
//
    }
    xdp.data = xdp.data_meta + xdp_md.data;
    return 0;
    free_dev:
    dev_put(device);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn xdp_convert_buff_to_md(xdp: *mut xdp_buff, xdp_md: *mut xdp_md) {
    static void xdp_convert_buff_to_md(struct xdp_buff *xdp, struct xdp_md *xdp_md)
    {
    if (!xdp_md)
    return;
    xdp_md.data = xdp.data - xdp.data_meta;
    xdp_md.data_end = xdp.data_end - xdp.data_meta;
    if (xdp_md.ingress_ifindex)
    dev_put(xdp.rxq.dev);
    }
    int bpf_prog_test_run_xdp(struct bpf_prog *prog, const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    let mut do_live: bool = (kattr.test.flags & BPF_F_TEST_XDP_LIVE_FRAMES);
    let mut tailroom: u32 = SKB_DATA_ALIGN(sizeof(struct skb_shared_info));
    let mut retval: u32 = 0, meta_sz = 0, duration, max_linear_sz, size;
    let mut linear_sz: u32 = kattr.test.data_size_in;
    let mut batch_size: u32 = kattr.test.batch_size;
    let mut headroom: u32 = XDP_PACKET_HEADROOM;
    let mut repeat: u32 = kattr.test.repeat;
    struct netdev_rx_queue *rxqueue;
    struct skb_shared_info *sinfo;
    let mut xdp: xdp_buff = {};
    int i, ret = -EINVAL;
    struct xdp_md *ctx;
    void *data;
    if (prog.expected_attach_type == BPF_XDP_DEVMAP ||
    prog.expected_attach_type == BPF_XDP_CPUMAP)
    return -EINVAL;
    if (kattr.test.flags & ~BPF_F_TEST_XDP_LIVE_FRAMES)
    return -EINVAL;
    if (bpf_prog_is_dev_bound(prog.aux))
    return -EINVAL;
    if (do_live) {
    if (!batch_size)
    batch_size = NAPI_POLL_WEIGHT;
#[no_mangle]
pub unsafe extern "C" fn if(TEST_XDP_MAX_BATCH: batch_size >) -> else {
    else if (batch_size > TEST_XDP_MAX_BATCH)
    return -E2BIG;
    } else if (batch_size) {
    return -EINVAL;
    }
    ctx = bpf_ctx_init(kattr, sizeof(struct xdp_md));
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    if (ctx) {
// There can't be user provided data before the meta data
    if (ctx.data_meta || ctx.data_end > kattr.test.data_size_in ||
    ctx.data > ctx.data_end ||
    (do_live && (kattr.test.data_out || kattr.test.ctx_out)))
    goto free_ctx;
    meta_sz = ctx.data;
    if (xdp_metalen_invalid(meta_sz) || meta_sz > headroom - sizeof(struct xdp_frame))
    goto free_ctx;
// Meta data is allocated from the headroom
    headroom -= meta_sz;
    linear_sz = ctx.data_end;
    }
// The xdp_page_head structure takes up space in each page, limiting the
// size of the packet data; add the extra size to headroom here to make
// sure it's accounted in the length checks below, but not in the
// metadata size check above.
//
    if (do_live)
    headroom += sizeof(struct xdp_page_head);
    max_linear_sz = PAGE_SIZE - headroom - tailroom;
    linear_sz = min_t(u32, linear_sz, max_linear_sz);
// disallow live data mode for jumbo frames
    if (do_live && kattr.test.data_size_in > linear_sz)
    goto free_ctx;
    if (kattr.test.data_size_in - meta_sz < ETH_HLEN)
    goto free_ctx;
    data = bpf_test_init(kattr, linear_sz, max_linear_sz, headroom, tailroom);
    if (IS_ERR(data)) {
    ret = PTR_ERR(data);
    goto free_ctx;
    }
    rxqueue = __netif_get_rx_queue(current.nsproxy.net_ns.loopback_dev, 0);
    rxqueue.xdp_rxq.frag_size = PAGE_SIZE;
    xdp_init_buff(&xdp, rxqueue.xdp_rxq.frag_size, &rxqueue.xdp_rxq);
    xdp_prepare_buff(&xdp, data, headroom, linear_sz, true);
    sinfo = xdp_get_shared_info_from_buff(&xdp);
    ret = xdp_convert_md_to_buff(ctx, &xdp);
    if (ret)
    goto free_data;
    size = linear_sz;
    if (unlikely(kattr.test.data_size_in > size)) {
    void __user *data_in = u64_to_user_ptr(kattr.test.data_in);
    while (size < kattr.test.data_size_in) {
    struct page *page;
    skb_frag_t *frag;
    u32 data_len;
    if (sinfo.nr_frags == MAX_SKB_FRAGS) {
    ret = -ENOMEM;
    goto out_put_dev;
    }
    page = alloc_page(GFP_KERNEL);
    if (!page) {
    ret = -ENOMEM;
    goto out_put_dev;
    }
    frag = &sinfo.frags[sinfo.nr_frags++];
    data_len = min_t(u32, kattr.test.data_size_in - size,
    PAGE_SIZE);
    skb_frag_fill_page_desc(frag, page, 0, data_len);
    if (copy_from_user(page_address(page), data_in + size,
    data_len)) {
    ret = -EFAULT;
    goto out_put_dev;
    }
    sinfo.xdp_frags_size += data_len;
    size += data_len;
    }
    xdp_buff_set_frags_flag(&xdp);
    }
    if (repeat > 1)
    bpf_prog_change_xdp(core::ptr::null_mut(), prog);
    if (do_live)
    ret = bpf_test_run_xdp_live(prog, &xdp, repeat, batch_size, &duration);
    else
    ret = bpf_test_run(prog, &xdp, repeat, &retval, &duration, true);
    out_put_dev:
// We convert the xdp_buff back to an xdp_md before checking the return
// code so the reference count of any held netdevice will be decremented
// even if the test run failed.
//
    xdp_convert_buff_to_md(&xdp, ctx);
    if (ret)
    goto out;
    size = xdp.data_end - xdp.data_meta + sinfo.xdp_frags_size;
    ret = bpf_test_finish(kattr, uattr, xdp.data_meta, sinfo, size, sinfo.xdp_frags_size,
    retval, duration);
    if (!ret)
    ret = bpf_ctx_finish(kattr, uattr, ctx,
    sizeof(struct xdp_md));
    out:
    if (repeat > 1)
    bpf_prog_change_xdp(prog, core::ptr::null_mut());
    free_data:
    for (i = 0; i < sinfo.nr_frags; i++)
    __free_page(skb_frag_page(&sinfo.frags[i]));
    kfree(data);
    free_ctx:
    kfree(ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn verify_user_bpf_flow_keys(ctx: *mut bpf_flow_keys) -> c_int {
    static int verify_user_bpf_flow_keys(struct bpf_flow_keys *ctx)
    {
// make sure the fields we don't use are zeroed
    if (!range_is_zero(ctx, 0, offsetof(struct bpf_flow_keys, flags)))
    return -EINVAL;
// flags is allowed
    if (!range_is_zero(ctx, offsetofend(struct bpf_flow_keys, flags),
    sizeof(struct bpf_flow_keys)))
    return -EINVAL;
    return 0;
    }
    int bpf_prog_test_run_flow_dissector(struct bpf_prog *prog,
    const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    let mut t: bpf_test_timer = {};
    let mut size: u32 = kattr.test.data_size_in;
    let mut ctx: bpf_flow_dissector = {};
    let mut repeat: u32 = kattr.test.repeat;
    struct bpf_flow_keys *user_ctx;
    struct bpf_flow_keys flow_keys;
    const struct ethhdr *eth;
    let mut flags: c_uint = 0;
    u32 retval, duration;
    void *data;
    int ret;
    if (kattr.test.flags || kattr.test.cpu || kattr.test.batch_size)
    return -EINVAL;
    if (size < ETH_HLEN)
    return -EINVAL;
    data = bpf_test_init(kattr, kattr.test.data_size_in, size, 0, 0);
    if (IS_ERR(data))
    return PTR_ERR(data);
    eth = (struct ethhdr *)data;
    if (!repeat)
    repeat = 1;
    user_ctx = bpf_ctx_init(kattr, sizeof(struct bpf_flow_keys));
    if (IS_ERR(user_ctx)) {
    kfree(data);
    return PTR_ERR(user_ctx);
    }
    if (user_ctx) {
    ret = verify_user_bpf_flow_keys(user_ctx);
    if (ret)
    goto out;
    flags = user_ctx.flags;
    }
    ctx.flow_keys = &flow_keys;
    ctx.data = data;
    ctx.data_end = (__u8 *)data + size;
    bpf_test_timer_enter(&t);
    do {
    retval = bpf_flow_dissect(prog, &ctx, eth.h_proto, ETH_HLEN,
    size, flags);
    } while (bpf_test_timer_continue(&t, 1, repeat, &ret, &duration));
    bpf_test_timer_leave(&t);
    if (ret < 0)
    goto out;
    ret = bpf_test_finish(kattr, uattr, &flow_keys, core::ptr::null_mut(),
    sizeof(flow_keys), 0, retval, duration);
    if (!ret)
    ret = bpf_ctx_finish(kattr, uattr, user_ctx,
    sizeof(struct bpf_flow_keys));
    out:
    kfree(user_ctx);
    kfree(data);
    return ret;
    }
    int bpf_prog_test_run_sk_lookup(struct bpf_prog *prog, const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    let mut t: bpf_test_timer = {};
    struct bpf_prog_array *progs = core::ptr::null_mut();
    let mut ctx: bpf_sk_lookup_kern = {};
    let mut repeat: u32 = kattr.test.repeat;
    struct bpf_sk_lookup *user_ctx;
    u32 retval, duration;
    let mut ret: c_int = -EINVAL;
    if (kattr.test.flags || kattr.test.cpu || kattr.test.batch_size)
    return -EINVAL;
    if (kattr.test.data_in || kattr.test.data_size_in || kattr.test.data_out ||
    kattr.test.data_size_out)
    return -EINVAL;
    if (!repeat)
    repeat = 1;
    user_ctx = bpf_ctx_init(kattr, sizeof(*user_ctx));
    if (IS_ERR(user_ctx))
    return PTR_ERR(user_ctx);
    if (!user_ctx)
    return -EINVAL;
    if (user_ctx.sk)
    goto out;
    if (!range_is_zero(user_ctx, offsetofend(typeof(*user_ctx), local_port), sizeof(*user_ctx)))
    goto out;
    if (user_ctx.local_port > U16_MAX) {
    ret = -ERANGE;
    goto out;
    }
    ctx.family = (u16)user_ctx.family;
    ctx.protocol = (u16)user_ctx.protocol;
    ctx.dport = (u16)user_ctx.local_port;
    ctx.sport = user_ctx.remote_port;
    switch (ctx.family) {
    case AF_INET:
    ctx.v4.daddr = ( __be32)user_ctx.local_ip4;
    ctx.v4.saddr = ( __be32)user_ctx.remote_ip4;
    break;

    case AF_INET6:
    ctx.v6.daddr = (struct in6_addr *)user_ctx.local_ip6;
    ctx.v6.saddr = (struct in6_addr *)user_ctx.remote_ip6;
    break;

    default:
    ret = -EAFNOSUPPORT;
    goto out;
    }
    progs = bpf_prog_array_alloc(1, GFP_KERNEL);
    if (!progs) {
    ret = -ENOMEM;
    goto out;
    }
    progs.items[0].prog = prog;
    bpf_test_timer_enter(&t);
    do {
    ctx.selected_sk = core::ptr::null_mut();
    retval = BPF_PROG_SK_LOOKUP_RUN_ARRAY(progs, ctx, bpf_prog_run);
    } while (bpf_test_timer_continue(&t, 1, repeat, &ret, &duration));
    bpf_test_timer_leave(&t);
    if (ret < 0)
    goto out;
    user_ctx.cookie = 0;
    if (ctx.selected_sk) {
    if (ctx.selected_sk.sk_reuseport && !ctx.no_reuseport) {
    ret = -EOPNOTSUPP;
    goto out;
    }
    user_ctx.cookie = sock_gen_cookie(ctx.selected_sk);
    }
    ret = bpf_test_finish(kattr, uattr, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, retval, duration);
    if (!ret)
    ret = bpf_ctx_finish(kattr, uattr, user_ctx, sizeof(*user_ctx));
    out:
    bpf_prog_array_free(progs);
    kfree(user_ctx);
    return ret;
    }
    int bpf_prog_test_run_syscall(struct bpf_prog *prog,
    const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    void __user *ctx_in = u64_to_user_ptr(kattr.test.ctx_in);
    let mut ctx_size_in: __u32 = kattr.test.ctx_size_in;
    void *ctx = core::ptr::null_mut();
    u32 retval;
    let mut err: c_int = 0;
// doesn't support data_in/out, ctx_out, duration, or repeat or flags
    if (kattr.test.data_in || kattr.test.data_out ||
    kattr.test.ctx_out || kattr.test.duration ||
    kattr.test.repeat || kattr.test.flags ||
    kattr.test.batch_size)
    return -EINVAL;
    if (ctx_size_in < prog.aux.max_ctx_offset ||
    ctx_size_in > U16_MAX)
    return -EINVAL;
    if (ctx_size_in) {
    ctx = memdup_user(ctx_in, ctx_size_in);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    }
    rcu_read_lock_trace();
    retval = bpf_prog_run_pin_on_cpu(prog, ctx);
    rcu_read_unlock_trace();
    if (copy_to_user(&uattr.test.retval, &retval, sizeof(u32))) {
    err = -EFAULT;
    goto out;
    }
    if (ctx_size_in)
    if (copy_to_user(ctx_in, ctx, ctx_size_in))
    err = -EFAULT;
    out:
    kfree(ctx);
    return err;
    }
    static int verify_and_copy_hook_state(struct nf_hook_state *state,
    const struct nf_hook_state *user,
    struct net_device *dev)
    {
    if (user.in || user.out)
    return -EINVAL;
    if (user.net || user.sk || user.okfn)
    return -EINVAL;
    switch (user.pf) {
    case NFPROTO_IPV4:
    case NFPROTO_IPV6:
    switch (state.hook) {
    case NF_INET_PRE_ROUTING:
    state.in = dev;
    break;
    case NF_INET_LOCAL_IN:
    state.in = dev;
    break;
    case NF_INET_FORWARD:
    state.in = dev;
    state.out = dev;
    break;
    case NF_INET_LOCAL_OUT:
    state.out = dev;
    break;
    case NF_INET_POST_ROUTING:
    state.out = dev;
    break;
    }
    break;
    default:
    return -EINVAL;
    }
    state.pf = user.pf;
    state.hook = user.hook;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfproto_eth(nfproto: c_int) -> __be16 {
    static __be16 nfproto_eth(int nfproto)
    {
    switch (nfproto) {
    case NFPROTO_IPV4:
    return htons(ETH_P_IP);
    case NFPROTO_IPV6:
    break;
    }
    return htons(ETH_P_IPV6);
    }
    int bpf_prog_test_run_nf(struct bpf_prog *prog,
    const union bpf_attr *kattr,
    union bpf_attr __user *uattr)
    {
    struct net *net = current.nsproxy.net_ns;
    struct net_device *dev = net.loopback_dev;
    struct nf_hook_state *user_ctx, hook_state = {
    .pf = NFPROTO_IPV4,
    .hook = NF_INET_LOCAL_OUT,
    };
    let mut size: u32 = kattr.test.data_size_in;
    let mut repeat: u32 = kattr.test.repeat;
    struct bpf_nf_ctx ctx = {
    .state = &hook_state,
    };
    struct sk_buff *skb = core::ptr::null_mut();
    u32 retval, duration;
    void *data;
    int ret;
    if (kattr.test.flags || kattr.test.cpu || kattr.test.batch_size)
    return -EINVAL;
    if (size < sizeof(struct iphdr))
    return -EINVAL;
    data = bpf_test_init(kattr, kattr.test.data_size_in, size,
    NET_SKB_PAD + NET_IP_ALIGN,
    SKB_DATA_ALIGN(sizeof(struct skb_shared_info)));
    if (IS_ERR(data))
    return PTR_ERR(data);
    if (!repeat)
    repeat = 1;
    user_ctx = bpf_ctx_init(kattr, sizeof(struct nf_hook_state));
    if (IS_ERR(user_ctx)) {
    kfree(data);
    return PTR_ERR(user_ctx);
    }
    if (user_ctx) {
    ret = verify_and_copy_hook_state(&hook_state, user_ctx, dev);
    if (ret)
    goto out;
    }
    skb = slab_build_skb(data);
    if (!skb) {
    ret = -ENOMEM;
    goto out;
    }
    data = core::ptr::null_mut(); /* data released via kfree_skb */
    skb_reserve(skb, NET_SKB_PAD + NET_IP_ALIGN);
    __skb_put(skb, size);
    ret = -EINVAL;
    if (hook_state.hook != NF_INET_LOCAL_OUT) {
    if (size < ETH_HLEN + sizeof(struct iphdr))
    goto out;
    skb.protocol = eth_type_trans(skb, dev);
    switch (skb.protocol) {
    case htons(ETH_P_IP):
    if (hook_state.pf == NFPROTO_IPV4)
    break;
    goto out;
    case htons(ETH_P_IPV6):
    if (size < ETH_HLEN + sizeof(struct ipv6hdr))
    goto out;
    if (hook_state.pf == NFPROTO_IPV6)
    break;
    goto out;
    default:
    ret = -EPROTO;
    goto out;
    }
    skb_reset_network_header(skb);
    } else {
    skb.protocol = nfproto_eth(hook_state.pf);
    }
    ctx.skb = skb;
    ret = bpf_test_run(prog, &ctx, repeat, &retval, &duration, false);
    if (ret)
    goto out;
    ret = bpf_test_finish(kattr, uattr, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, retval, duration);
    out:
    kfree(user_ctx);
    kfree_skb(skb);
    kfree(data);
    return ret;
    }
    static const struct btf_kfunc_id_set bpf_prog_test_kfunc_set = {
    .owner = THIS_MODULE,
    .set   = &test_sk_check_kfunc_ids,
    };
    BTF_ID_LIST(bpf_prog_test_dtor_kfunc_ids)
    BTF_ID(struct, prog_test_ref_kfunc)
    BTF_ID(func, bpf_kfunc_call_test_release_dtor)
    BTF_ID(struct, prog_test_member)
    BTF_ID(func, bpf_kfunc_call_memb_release_dtor)
#[no_mangle]
unsafe extern "C" fn bpf_prog_test_run_init() -> int __init {
    static int __init bpf_prog_test_run_init(void)
    {
    const struct btf_id_dtor_kfunc bpf_prog_test_dtor_kfunc[] = {
    {
    .btf_id       = bpf_prog_test_dtor_kfunc_ids[0],
    .kfunc_btf_id = bpf_prog_test_dtor_kfunc_ids[1]
    },
    {
    .btf_id	= bpf_prog_test_dtor_kfunc_ids[2],
    .kfunc_btf_id = bpf_prog_test_dtor_kfunc_ids[3],
    },
    };
    int ret;
    ret = register_btf_fmodret_id_set(&bpf_test_modify_return_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &bpf_prog_test_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &bpf_prog_test_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &bpf_prog_test_kfunc_set);
    return ret ?: register_btf_id_dtor_kfuncs(bpf_prog_test_dtor_kfunc,
    ARRAY_SIZE(bpf_prog_test_dtor_kfunc),
    THIS_MODULE);
    }
    late_initcall(bpf_prog_test_run_init);
