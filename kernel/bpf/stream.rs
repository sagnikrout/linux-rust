//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/stream.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn bpf_stream_elem_init(elem: *mut bpf_stream_elem, len: c_int) {
    static void bpf_stream_elem_init(struct bpf_stream_elem *elem, int len)
    {
    init_llist_node(&elem.node);
    elem.total_len = len;
    elem.consumed_len = 0;
    }
    static struct bpf_stream_elem *bpf_stream_elem_alloc(int len)
    {
    let mut max_len: c_int = ARRAY_SIZE((struct bpf_bprintf_buffers){}.buf);
    struct bpf_stream_elem *elem;
    size_t alloc_size;
//
// Length denotes the amount of data to be written as part of stream element,
// thus includes '\0' byte. We're capped by how much bpf_bprintf_buffers can
// accomodate, therefore deny allocations that won't fit into them.
//
    if (len < 0 || len > max_len)
    return core::ptr::null_mut();
    alloc_size = offsetof(struct bpf_stream_elem, str[len]);
    elem = kmalloc_nolock(alloc_size, __GFP_ZERO, -1);
    if (!elem)
    return core::ptr::null_mut();
    bpf_stream_elem_init(elem, len);
    return elem;
    }
#[no_mangle]
unsafe extern "C" fn __bpf_stream_push_str(log: *mut llist_head, str: *const c_char, len: c_int) -> c_int {
    static int __bpf_stream_push_str(struct llist_head *log, const char *str, int len)
    {
    struct bpf_stream_elem *elem = core::ptr::null_mut();
//
// Allocate a bpf_prog_stream_elem and push it to the bpf_prog_stream
// log, elements will be popped at once and reversed to print the log.
//
    elem = bpf_stream_elem_alloc(len);
    if (!elem)
    return -ENOMEM;
    memcpy(elem.str, str, len);
    llist_add(&elem.node, log);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_consume_capacity(stream: *mut bpf_stream, len: c_int) -> c_int {
    static int bpf_stream_consume_capacity(struct bpf_stream *stream, int len)
    {
    if (atomic_read(&stream.capacity) >= BPF_STREAM_MAX_CAPACITY)
    return -ENOSPC;
    if (atomic_add_return(len, &stream.capacity) >= BPF_STREAM_MAX_CAPACITY) {
    atomic_sub(len, &stream.capacity);
    return -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_release_capacity(stream: *mut bpf_stream, elem: *mut bpf_stream_elem) {
    static void bpf_stream_release_capacity(struct bpf_stream *stream, struct bpf_stream_elem *elem)
    {
    let mut len: c_int = elem.total_len;
    atomic_sub(len, &stream.capacity);
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_push_str(stream: *mut bpf_stream, str: *const c_char, len: c_int) -> c_int {
    static int bpf_stream_push_str(struct bpf_stream *stream, const char *str, int len)
    {
    let mut ret: c_int = bpf_stream_consume_capacity(stream, len);
    return ret ?: __bpf_stream_push_str(&stream.log, str, len);
    }
    static struct bpf_stream *bpf_stream_get(enum bpf_stream_id stream_id, struct bpf_prog_aux *aux)
    {
    if (stream_id != BPF_STDOUT && stream_id != BPF_STDERR)
    return core::ptr::null_mut();
    return &aux.stream[stream_id - 1];
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_free_elem(elem: *mut bpf_stream_elem) {
    static void bpf_stream_free_elem(struct bpf_stream_elem *elem)
    {
    kfree_nolock(elem);
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_free_list(list: *mut llist_node) {
    static void bpf_stream_free_list(struct llist_node *list)
    {
    struct bpf_stream_elem *elem, *tmp;
    llist_for_each_entry_safe(elem, tmp, list, node)
    bpf_stream_free_elem(elem);
    }
    static struct llist_node *bpf_stream_backlog_peek(struct bpf_stream *stream)
    {
    return stream.backlog_head;
    }
    static struct llist_node *bpf_stream_backlog_pop(struct bpf_stream *stream)
    {
    struct llist_node *node;
    node = stream.backlog_head;
    if (stream.backlog_head == stream.backlog_tail)
    stream.backlog_head = stream.backlog_tail = core::ptr::null_mut();
    else
    stream.backlog_head = node.next;
    return node;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_backlog_fill(stream: *mut bpf_stream) {
    static void bpf_stream_backlog_fill(struct bpf_stream *stream)
    {
    struct llist_node *head, *tail;
    if (llist_empty(&stream.log))
    return;
    tail = llist_del_all(&stream.log);
    if (!tail)
    return;
    head = llist_reverse_order(tail);
    if (!stream.backlog_head) {
    stream.backlog_head = head;
    stream.backlog_tail = tail;
    } else {
    stream.backlog_tail.next = head;
    stream.backlog_tail = tail;
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_consume_elem(elem: *mut bpf_stream_elem, len: *mut c_int) -> bool {
    static bool bpf_stream_consume_elem(struct bpf_stream_elem *elem, int *len)
    {
    let mut rem: c_int = elem.total_len - elem.consumed_len;
    let mut used: c_int = min(rem, *len);
    elem.consumed_len += used;
// len -= used;
    return elem.consumed_len == elem.total_len;
    }
#[no_mangle]
unsafe extern "C" fn bpf_stream_read(stream: *mut bpf_stream, buf: *mut void __user, len: c_int) -> c_int {
    static int bpf_stream_read(struct bpf_stream *stream, void __user *buf, int len)
    {
    let mut rem_len: c_int = len, cons_len, ret = 0;
    struct bpf_stream_elem *elem = core::ptr::null_mut();
    struct llist_node *node;
    mutex_lock(&stream.lock);
    while (rem_len) {
    let mut pos: c_int = len - rem_len;
    bool cont;
    node = bpf_stream_backlog_peek(stream);
    if (!node) {
    bpf_stream_backlog_fill(stream);
    node = bpf_stream_backlog_peek(stream);
    }
    if (!node)
    break;
    elem = container_of(node, typeof(*elem), node);
    cons_len = elem.consumed_len;
    cont = bpf_stream_consume_elem(elem, &rem_len) == false;
    ret = copy_to_user(buf + pos, elem.str + cons_len,
    elem.consumed_len - cons_len);
// Restore in case of error.
    if (ret) {
    ret = -EFAULT;
    elem.consumed_len = cons_len;
    break;
    }
    if (cont)
    continue;
    bpf_stream_backlog_pop(stream);
    bpf_stream_release_capacity(stream, elem);
    bpf_stream_free_elem(elem);
    }
    mutex_unlock(&stream.lock);
    return ret ? ret : len - rem_len;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_stream_read(prog: *mut bpf_prog, stream_id: enum bpf_stream_id, buf: *mut void __user, len: c_int) -> c_int {
    int bpf_prog_stream_read(struct bpf_prog *prog, enum bpf_stream_id stream_id, void __user *buf, int len)
    {
    struct bpf_stream *stream;
    stream = bpf_stream_get(stream_id, prog.aux);
    if (!stream)
    return -ENOENT;
    return bpf_stream_read(stream, buf, len);
    }
    __bpf_kfunc_start_defs();
//
// Avoid using enum bpf_stream_id so that kfunc users don't have to pull in the
// enum in headers.
//
    __bpf_kfunc int bpf_stream_vprintk(int stream_id, const char *fmt__str, const void *args,
    u32 len__sz, struct bpf_prog_aux *aux)
    {
    struct bpf_bprintf_data data = {
    .get_bin_args	= true,
    .get_buf	= true,
    };
    let mut fmt_size: u32 = strlen(fmt__str) + 1;
    struct bpf_stream *stream;
    let mut data_len: u32 = len__sz;
    int ret, num_args;
    stream = bpf_stream_get(stream_id, aux);
    if (!stream)
    return -ENOENT;
    if (data_len & 7 || data_len > MAX_BPRINTF_VARARGS * 8 ||
    (data_len && !args))
    return -EINVAL;
    num_args = data_len / 8;
    ret = bpf_bprintf_prepare(fmt__str, fmt_size, args, num_args, &data);
    if (ret < 0)
    return ret;
    ret = bstr_printf(data.buf, MAX_BPRINTF_BUF, fmt__str, data.bin_args);
// Exclude NULL byte during push.
    ret = bpf_stream_push_str(stream, data.buf, ret);
    bpf_bprintf_cleanup(&data);
    return ret;
    }
// Directly trigger a stack dump from the program.
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_print_stack(stream_id: c_int, aux: *mut bpf_prog_aux) -> __bpf_kfunc int {
    __bpf_kfunc int bpf_stream_print_stack(int stream_id, struct bpf_prog_aux *aux)
    {
    struct bpf_stream_stage ss;
    struct bpf_prog *prog;
// Make sure the stream ID is valid.
    if (!bpf_stream_get(stream_id, aux))
    return -ENOENT;
    prog = aux.main_prog_aux.prog;
    bpf_stream_stage(ss, prog, stream_id, ({
    bpf_stream_dump_stack(ss);
    }));
    return 0;
    }
    __bpf_kfunc_end_defs();
// Added kfunc to common_btf_ids
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_stream_init(prog: *mut bpf_prog) {
    void bpf_prog_stream_init(struct bpf_prog *prog)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(prog.aux.stream); i++) {
    atomic_set(&prog.aux.stream[i].capacity, 0);
    init_llist_head(&prog.aux.stream[i].log);
    mutex_init(&prog.aux.stream[i].lock);
    prog.aux.stream[i].backlog_head = core::ptr::null_mut();
    prog.aux.stream[i].backlog_tail = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_stream_free(prog: *mut bpf_prog) {
    void bpf_prog_stream_free(struct bpf_prog *prog)
    {
    struct llist_node *list;
    int i;
    for (i = 0; i < ARRAY_SIZE(prog.aux.stream); i++) {
    list = llist_del_all(&prog.aux.stream[i].log);
    bpf_stream_free_list(list);
    bpf_stream_free_list(prog.aux.stream[i].backlog_head);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_init(ss: *mut bpf_stream_stage) {
    void bpf_stream_stage_init(struct bpf_stream_stage *ss)
    {
    init_llist_head(&ss.log);
    ss.len = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_free(ss: *mut bpf_stream_stage) {
    void bpf_stream_stage_free(struct bpf_stream_stage *ss)
    {
    struct llist_node *node;
    node = llist_del_all(&ss.log);
    bpf_stream_free_list(node);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_printk(ss: *mut bpf_stream_stage, fmt: *const c_char, ...) -> c_int {
    int bpf_stream_stage_printk(struct bpf_stream_stage *ss, const char *fmt, ...)
    {
    struct bpf_bprintf_buffers *buf;
    va_list args;
    int ret;
    if (bpf_try_get_buffers(&buf))
    return -EBUSY;
    va_start(args, fmt);
    ret = vsnprintf(buf.buf, ARRAY_SIZE(buf.buf), fmt, args);
    va_end(args);
    ss.len += ret;
// Exclude NULL byte during push.
    ret = __bpf_stream_push_str(&ss.log, buf.buf, ret);
    bpf_put_buffers();
    return ret;
    }
    int bpf_stream_stage_commit(struct bpf_stream_stage *ss, struct bpf_prog *prog,
    enum bpf_stream_id stream_id)
    {
    struct llist_node *list, *head, *tail;
    struct bpf_stream *stream;
    int ret;
    stream = bpf_stream_get(stream_id, prog.aux);
    if (!stream)
    return -EINVAL;
    ret = bpf_stream_consume_capacity(stream, ss.len);
    if (ret)
    return ret;
    list = llist_del_all(&ss.log);
    head = tail = list;
    if (!list)
    return 0;
    while (llist_next(list)) {
    tail = llist_next(list);
    list = tail;
    }
    llist_add_batch(head, tail, &stream.log);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dump_stack_ctx {
    pub ss: *mut bpf_stream_stage,
    pub err: c_int,
}

#[no_mangle]
unsafe extern "C" fn dump_stack_cb(cookie: *mut c_void, ip: u64, sp: u64, bp: u64) -> bool {
    static bool dump_stack_cb(void *cookie, u64 ip, u64 sp, u64 bp)
    {
    struct dump_stack_ctx *ctxp = cookie;
    const char *file = "", *line = "";
    struct bpf_prog *prog;
    int num, ret;
    rcu_read_lock();
    prog = bpf_prog_ksym_find(ip);
    rcu_read_unlock();
    if (prog) {
    ret = bpf_prog_get_file_line(prog, ip, &file, &line, &num);
    if (ret < 0)
    goto end;
    ctxp.err = bpf_stream_stage_printk(ctxp.ss, "%pS\n  %s @ %s:%d\n",
    (void *)(long)ip, line, file, num);
    return !ctxp.err;
    }
    end:
    ctxp.err = bpf_stream_stage_printk(ctxp.ss, "%pS\n", (void *)(long)ip);
    return !ctxp.err;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_stream_stage_dump_stack(ss: *mut bpf_stream_stage) -> c_int {
    int bpf_stream_stage_dump_stack(struct bpf_stream_stage *ss)
    {
    let mut ctx: dump_stack_ctx = { .ss = ss };
    int ret;
    ret = bpf_stream_stage_printk(ss, "CPU: %d UID: %d PID: %d Comm: %s\n",
    raw_smp_processor_id(), __kuid_val(current_real_cred().euid),
    current.pid, current.comm);
    if (ret)
    return ret;
    ret = bpf_stream_stage_printk(ss, "Call trace:\n");
    if (ret)
    return ret;
    arch_bpf_stack_walk(dump_stack_cb, &ctx);
    if (ctx.err)
    return ctx.err;
    return bpf_stream_stage_printk(ss, "\n");
    }
