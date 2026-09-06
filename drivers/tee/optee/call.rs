//! Automatically rewritten from C to Rust
//! Source: drivers/tee/optee/call.c
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
//
// Copyright (c) 2015-2021, 2023 Linaro Limited
//

pub const MAX_ARG_PARAM_COUNT: c_int = 6;
//
// How much memory we allocate for each entry. This doesn't have to be a
// single page, but it makes sense to keep at least keep it as multiples of
// the page size.
//

//
// We need to have a compile time constant to be able to determine the
// maximum needed size of the bit field.
//

//
// Shared memory for argument structs are cached here. The number of
// arguments structs that can fit is determined at runtime depending on the
// needed RPC parameter count reported by secure world
// (optee->rpc_param_count).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_shm_arg_entry {
    pub list_node: list_head,
    pub shm: *mut tee_shm,
    pub MAX_ARG_COUNT_PER_ENTRY): DECLARE_BITMAP(map,,
}

#[no_mangle]
pub unsafe extern "C" fn optee_cq_init(cq: *mut optee_call_queue, thread_count: c_int) {
    void optee_cq_init(struct optee_call_queue *cq, int thread_count)
    {
    mutex_init(&cq.mutex);
    INIT_LIST_HEAD(&cq.waiters);
//
// If cq->total_thread_count is 0 then we're not trying to keep
// track of how many free threads we have, instead we're relying on
// the secure world to tell us when we're out of thread and have to
// wait for another thread to become available.
//
    cq.total_thread_count = thread_count;
    cq.free_thread_count = thread_count;
    }
    void optee_cq_wait_init(struct optee_call_queue *cq,
    struct optee_call_waiter *w, bool sys_thread)
    {
    unsigned int free_thread_threshold;
    let mut need_wait: bool = false;
    memset(w, 0, sizeof(*w));
//
// We're preparing to make a call to secure world. In case we can't
// allocate a thread in secure world we'll end up waiting in
// optee_cq_wait_for_completion().
//
// Normally if there's no contention in secure world the call will
// complete and we can cleanup directly with optee_cq_wait_final().
//
    mutex_lock(&cq.mutex);
//
// We add ourselves to the queue, but we don't wait. This
// guarantees that we don't lose a completion if secure world
// returns busy and another thread just exited and try to complete
// someone.
//
    init_completion(&w.c);
    list_add_tail(&w.list_node, &cq.waiters);
    w.sys_thread = sys_thread;
    if (cq.total_thread_count) {
    if (sys_thread || !cq.sys_thread_req_count)
    free_thread_threshold = 0;
    else
    free_thread_threshold = 1;
    if (cq.free_thread_count > free_thread_threshold)
    cq.free_thread_count--;
    else
    need_wait = true;
    }
    mutex_unlock(&cq.mutex);
    while (need_wait) {
    optee_cq_wait_for_completion(cq, w);
    mutex_lock(&cq.mutex);
    if (sys_thread || !cq.sys_thread_req_count)
    free_thread_threshold = 0;
    else
    free_thread_threshold = 1;
    if (cq.free_thread_count > free_thread_threshold) {
    cq.free_thread_count--;
    need_wait = false;
    }
    mutex_unlock(&cq.mutex);
    }
    }
    void optee_cq_wait_for_completion(struct optee_call_queue *cq,
    struct optee_call_waiter *w)
    {
    wait_for_completion(&w.c);
    mutex_lock(&cq.mutex);
// Move to end of list to get out of the way for other waiters
    list_del(&w.list_node);
    reinit_completion(&w.c);
    list_add_tail(&w.list_node, &cq.waiters);
    mutex_unlock(&cq.mutex);
    }
#[no_mangle]
unsafe extern "C" fn optee_cq_complete_one(cq: *mut optee_call_queue) {
    static void optee_cq_complete_one(struct optee_call_queue *cq)
    {
    struct optee_call_waiter *w;
// Wake a waiting system session if any, prior to a normal session
    list_for_each_entry(w, &cq.waiters, list_node) {
    if (w.sys_thread && !completion_done(&w.c)) {
    complete(&w.c);
    return;
    }
    }
    list_for_each_entry(w, &cq.waiters, list_node) {
    if (!completion_done(&w.c)) {
    complete(&w.c);
    break;
    }
    }
    }
    void optee_cq_wait_final(struct optee_call_queue *cq,
    struct optee_call_waiter *w)
    {
//
// We're done with the call to secure world. The thread in secure
// world that was used for this call is now available for some
// other task to use.
//
    mutex_lock(&cq.mutex);
// Get out of the list
    list_del(&w.list_node);
    cq.free_thread_count++;
// Wake up one eventual waiting task
    optee_cq_complete_one(cq);
//
// If we're completed we've got a completion from another task that
// was just done with its call to secure world. Since yet another
// thread now is available in secure world wake up another eventual
// waiting task.
//
    if (completion_done(&w.c))
    optee_cq_complete_one(cq);
    mutex_unlock(&cq.mutex);
    }
// Count registered system sessions to reserved a system thread or not
#[no_mangle]
unsafe extern "C" fn optee_cq_incr_sys_thread_count(cq: *mut optee_call_queue) -> bool {
    static bool optee_cq_incr_sys_thread_count(struct optee_call_queue *cq)
    {
    if (cq.total_thread_count <= 1)
    return false;
    mutex_lock(&cq.mutex);
    cq.sys_thread_req_count++;
    mutex_unlock(&cq.mutex);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn optee_cq_decr_sys_thread_count(cq: *mut optee_call_queue) {
    static void optee_cq_decr_sys_thread_count(struct optee_call_queue *cq)
    {
    mutex_lock(&cq.mutex);
    cq.sys_thread_req_count--;
// If there's someone waiting, let it resume
    optee_cq_complete_one(cq);
    mutex_unlock(&cq.mutex);
    }
// Requires the filpstate mutex to be held
    static struct optee_session *find_session(struct optee_context_data *ctxdata,
    u32 session_id)
    {
    struct optee_session *sess;
    list_for_each_entry(sess, &ctxdata.sess_list, list_node)
    if (sess.session_id == session_id)
    return sess;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn optee_shm_arg_cache_init(optee: *mut optee, flags: u32) {
    void optee_shm_arg_cache_init(struct optee *optee, u32 flags)
    {
    INIT_LIST_HEAD(&optee.shm_arg_cache.shm_args);
    mutex_init(&optee.shm_arg_cache.mutex);
    optee.shm_arg_cache.flags = flags;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_shm_arg_cache_uninit(optee: *mut optee) {
    void optee_shm_arg_cache_uninit(struct optee *optee)
    {
    struct list_head *head = &optee.shm_arg_cache.shm_args;
    struct optee_shm_arg_entry *entry;
    mutex_destroy(&optee.shm_arg_cache.mutex);
    while (!list_empty(head)) {
    entry = list_first_entry(head, struct optee_shm_arg_entry,
    list_node);
    list_del(&entry.list_node);
    if (find_first_bit(entry.map, MAX_ARG_COUNT_PER_ENTRY) !=
    MAX_ARG_COUNT_PER_ENTRY) {
    pr_err("Freeing non-free entry\n");
    }
    tee_shm_free(entry.shm);
    kfree(entry);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn optee_msg_arg_size(rpc_param_count: usize) -> usize {
    size_t optee_msg_arg_size(size_t rpc_param_count)
    {
    let mut sz: usize = OPTEE_MSG_GET_ARG_SIZE(MAX_ARG_PARAM_COUNT);
    if (rpc_param_count)
    sz += OPTEE_MSG_GET_ARG_SIZE(rpc_param_count);
    return sz;
    }
//
// optee_get_msg_arg() - Provide shared memory for argument struct
// @ctx:	Caller TEE context
// @num_params:	Number of parameter to store
// @entry_ret:	Entry pointer, needed when freeing the buffer
// @shm_ret:	Shared memory buffer
// @offs_ret:	Offset of argument strut in shared memory buffer
//
// @returns a pointer to the argument struct in memory, else an ERR_PTR
//
    struct optee_msg_arg *optee_get_msg_arg(struct tee_context *ctx,
    size_t num_params,
    struct optee_shm_arg_entry **entry_ret,
    struct tee_shm **shm_ret,
    u_int *offs_ret)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    let mut sz: usize = optee_msg_arg_size(optee.rpc_param_count);
    struct optee_shm_arg_entry *entry;
    struct optee_msg_arg *ma;
    size_t args_per_entry;
    u_long bit;
    u_int offs;
    void *res;
    if (num_params > MAX_ARG_PARAM_COUNT)
    return ERR_PTR(-EINVAL);
    if (optee.shm_arg_cache.flags & OPTEE_SHM_ARG_SHARED)
    args_per_entry = SHM_ENTRY_SIZE / sz;
    else
    args_per_entry = 1;
    mutex_lock(&optee.shm_arg_cache.mutex);
    list_for_each_entry(entry, &optee.shm_arg_cache.shm_args, list_node) {
    bit = find_first_zero_bit(entry.map, MAX_ARG_COUNT_PER_ENTRY);
    if (bit < args_per_entry)
    goto have_entry;
    }
//
// No entry was found, let's allocate a new.
//
    entry = kzalloc_obj(*entry);
    if (!entry) {
    res = ERR_PTR(-ENOMEM);
    goto out;
    }
    if (optee.shm_arg_cache.flags & OPTEE_SHM_ARG_ALLOC_PRIV)
    res = tee_shm_alloc_priv_buf(ctx, SHM_ENTRY_SIZE);
    else
    res = tee_shm_alloc_kernel_buf(ctx, SHM_ENTRY_SIZE);
    if (IS_ERR(res)) {
    kfree(entry);
    goto out;
    }
    entry.shm = res;
    list_add(&entry.list_node, &optee.shm_arg_cache.shm_args);
    bit = 0;
    have_entry:
    offs = bit * sz;
    res = tee_shm_get_va(entry.shm, offs);
    if (IS_ERR(res))
    goto out;
    ma = res;
    set_bit(bit, entry.map);
    memset(ma, 0, sz);
    ma.num_params = num_params;
// entry_ret = entry;
// shm_ret = entry->shm;
// offs_ret = offs;
    out:
    mutex_unlock(&optee.shm_arg_cache.mutex);
    return res;
    }
//
// optee_free_msg_arg() - Free previsouly obtained shared memory
// @ctx:	Caller TEE context
// @entry:	Pointer returned when the shared memory was obtained
// @offs:	Offset of shared memory buffer to free
//
// This function frees the shared memory obtained with optee_get_msg_arg().
//
    void optee_free_msg_arg(struct tee_context *ctx,
    struct optee_shm_arg_entry *entry, u_int offs)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    let mut sz: usize = optee_msg_arg_size(optee.rpc_param_count);
    u_long bit;
    if (offs > SHM_ENTRY_SIZE || offs % sz) {
    pr_err("Invalid offs %u\n", offs);
    return;
    }
    bit = offs / sz;
    mutex_lock(&optee.shm_arg_cache.mutex);
    if (!test_bit(bit, entry.map))
    pr_err("Bit pos %lu is already free\n", bit);
    clear_bit(bit, entry.map);
    mutex_unlock(&optee.shm_arg_cache.mutex);
    }
    int optee_open_session(struct tee_context *ctx,
    struct tee_ioctl_open_session_arg *arg,
    struct tee_param *param)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    struct optee_context_data *ctxdata = ctx.data;
    struct optee_shm_arg_entry *entry;
    struct tee_shm *shm;
    struct optee_msg_arg *msg_arg;
    struct optee_session *sess = core::ptr::null_mut();
    uuid_t client_uuid;
    u_int offs;
    int rc;
// +2 for the meta parameters added below
    msg_arg = optee_get_msg_arg(ctx, arg.num_params + 2,
    &entry, &shm, &offs);
    if (IS_ERR(msg_arg))
    return PTR_ERR(msg_arg);
    msg_arg.cmd = OPTEE_MSG_CMD_OPEN_SESSION;
    msg_arg.cancel_id = arg.cancel_id;
//
// Initialize and add the meta parameters needed when opening a
// session.
//
    msg_arg.params[0].attr = OPTEE_MSG_ATTR_TYPE_VALUE_INPUT |
    OPTEE_MSG_ATTR_META;
    msg_arg.params[1].attr = OPTEE_MSG_ATTR_TYPE_VALUE_INPUT |
    OPTEE_MSG_ATTR_META;
    memcpy(&msg_arg.params[0].u.value, arg.uuid, sizeof(arg.uuid));
    msg_arg.params[1].u.value.c = arg.clnt_login;
    rc = tee_session_calc_client_uuid(&client_uuid, arg.clnt_login,
    arg.clnt_uuid);
    if (rc)
    goto out;
    export_uuid(msg_arg.params[1].u.octets, &client_uuid);
    rc = optee.ops.to_msg_param(optee, msg_arg.params + 2,
    arg.num_params, param);
    if (rc)
    goto out;
    sess = kzalloc_obj(*sess);
    if (!sess) {
    rc = -ENOMEM;
    goto out;
    }
    if (optee.ops.do_call_with_arg(ctx, shm, offs,
    sess.use_sys_thread)) {
    msg_arg.ret = TEEC_ERROR_COMMUNICATION;
    msg_arg.ret_origin = TEEC_ORIGIN_COMMS;
    }
    if (msg_arg.ret == TEEC_SUCCESS) {
// A new session has been created, add it to the list.
    sess.session_id = msg_arg.session;
    mutex_lock(&ctxdata.mutex);
    list_add(&sess.list_node, &ctxdata.sess_list);
    mutex_unlock(&ctxdata.mutex);
    } else {
    kfree(sess);
    }
    if (optee.ops.from_msg_param(optee, param, arg.num_params,
    msg_arg.params + 2)) {
    arg.ret = TEEC_ERROR_COMMUNICATION;
    arg.ret_origin = TEEC_ORIGIN_COMMS;
// Close session again to avoid leakage
    optee_close_session(ctx, msg_arg.session);
    } else {
    arg.session = msg_arg.session;
    arg.ret = msg_arg.ret;
    arg.ret_origin = msg_arg.ret_origin;
    }
    out:
    optee_free_msg_arg(ctx, entry, offs);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_system_session(ctx: *mut tee_context, session: u32) -> c_int {
    int optee_system_session(struct tee_context *ctx, u32 session)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    struct optee_context_data *ctxdata = ctx.data;
    struct optee_session *sess;
    let mut rc: c_int = -EINVAL;
    mutex_lock(&ctxdata.mutex);
    sess = find_session(ctxdata, session);
    if (sess && (sess.use_sys_thread ||
    optee_cq_incr_sys_thread_count(&optee.call_queue))) {
    sess.use_sys_thread = true;
    rc = 0;
    }
    mutex_unlock(&ctxdata.mutex);
    return rc;
    }
    int optee_close_session_helper(struct tee_context *ctx, u32 session,
    bool system_thread)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    struct optee_shm_arg_entry *entry;
    struct optee_msg_arg *msg_arg;
    struct tee_shm *shm;
    u_int offs;
    msg_arg = optee_get_msg_arg(ctx, 0, &entry, &shm, &offs);
    if (IS_ERR(msg_arg))
    return PTR_ERR(msg_arg);
    msg_arg.cmd = OPTEE_MSG_CMD_CLOSE_SESSION;
    msg_arg.session = session;
    optee.ops.do_call_with_arg(ctx, shm, offs, system_thread);
    optee_free_msg_arg(ctx, entry, offs);
    if (system_thread)
    optee_cq_decr_sys_thread_count(&optee.call_queue);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_close_session(ctx: *mut tee_context, session: u32) -> c_int {
    int optee_close_session(struct tee_context *ctx, u32 session)
    {
    struct optee_context_data *ctxdata = ctx.data;
    struct optee_session *sess;
    bool system_thread;
// Check that the session is valid and remove it from the list
    mutex_lock(&ctxdata.mutex);
    sess = find_session(ctxdata, session);
    if (sess)
    list_del(&sess.list_node);
    mutex_unlock(&ctxdata.mutex);
    if (!sess)
    return -EINVAL;
    system_thread = sess.use_sys_thread;
    kfree(sess);
    return optee_close_session_helper(ctx, session, system_thread);
    }
    int optee_invoke_func(struct tee_context *ctx, struct tee_ioctl_invoke_arg *arg,
    struct tee_param *param)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    struct optee_context_data *ctxdata = ctx.data;
    struct optee_shm_arg_entry *entry;
    struct optee_msg_arg *msg_arg;
    struct optee_session *sess;
    struct tee_shm *shm;
    bool system_thread;
    u_int offs;
    int rc;
// Check that the session is valid
    mutex_lock(&ctxdata.mutex);
    sess = find_session(ctxdata, arg.session);
    if (sess)
    system_thread = sess.use_sys_thread;
    mutex_unlock(&ctxdata.mutex);
    if (!sess)
    return -EINVAL;
    msg_arg = optee_get_msg_arg(ctx, arg.num_params,
    &entry, &shm, &offs);
    if (IS_ERR(msg_arg))
    return PTR_ERR(msg_arg);
    msg_arg.cmd = OPTEE_MSG_CMD_INVOKE_COMMAND;
    msg_arg.func = arg.func;
    msg_arg.session = arg.session;
    msg_arg.cancel_id = arg.cancel_id;
    rc = optee.ops.to_msg_param(optee, msg_arg.params, arg.num_params,
    param);
    if (rc)
    goto out;
    if (optee.ops.do_call_with_arg(ctx, shm, offs, system_thread)) {
    msg_arg.ret = TEEC_ERROR_COMMUNICATION;
    msg_arg.ret_origin = TEEC_ORIGIN_COMMS;
    }
    if (optee.ops.from_msg_param(optee, param, arg.num_params,
    msg_arg.params)) {
    msg_arg.ret = TEEC_ERROR_COMMUNICATION;
    msg_arg.ret_origin = TEEC_ORIGIN_COMMS;
    }
    arg.ret = msg_arg.ret;
    arg.ret_origin = msg_arg.ret_origin;
    out:
    optee_free_msg_arg(ctx, entry, offs);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_cancel_req(ctx: *mut tee_context, cancel_id: u32, session: u32) -> c_int {
    int optee_cancel_req(struct tee_context *ctx, u32 cancel_id, u32 session)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    struct optee_context_data *ctxdata = ctx.data;
    struct optee_shm_arg_entry *entry;
    struct optee_msg_arg *msg_arg;
    struct optee_session *sess;
    bool system_thread;
    struct tee_shm *shm;
    u_int offs;
// Check that the session is valid
    mutex_lock(&ctxdata.mutex);
    sess = find_session(ctxdata, session);
    if (sess)
    system_thread = sess.use_sys_thread;
    mutex_unlock(&ctxdata.mutex);
    if (!sess)
    return -EINVAL;
    msg_arg = optee_get_msg_arg(ctx, 0, &entry, &shm, &offs);
    if (IS_ERR(msg_arg))
    return PTR_ERR(msg_arg);
    msg_arg.cmd = OPTEE_MSG_CMD_CANCEL;
    msg_arg.session = session;
    msg_arg.cancel_id = cancel_id;
    optee.ops.do_call_with_arg(ctx, shm, offs, system_thread);
    optee_free_msg_arg(ctx, entry, offs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_normal_memory(p: pgprot_t) -> bool {
    static bool is_normal_memory(pgprot_t p)
    {

    return (((pgprot_val(p) & L_PTE_MT_MASK) == L_PTE_MT_WRITEALLOC) ||
    ((pgprot_val(p) & L_PTE_MT_MASK) == L_PTE_MT_WRITEBACK));

    return ((pgprot_val(p) & PTE_ATTRINDX_MASK) == PTE_ATTRINDX(MT_NORMAL)) ||
    ((pgprot_val(p) & PTE_ATTRINDX_MASK) == PTE_ATTRINDX(MT_NORMAL_TAGGED));

    }
    static int __check_mem_type(struct mm_struct *mm, unsigned long start,
    unsigned long end)
    {
    struct vm_area_struct *vma;
    VMA_ITERATOR(vmi, mm, start);
    for_each_vma_range(vmi, vma, end) {
    if (!is_normal_memory(vma.vm_page_prot))
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_check_mem_type(start: c_ulong, num_pages: usize) -> c_int {
    int optee_check_mem_type(unsigned long start, size_t num_pages)
    {
    struct mm_struct *mm = current.mm;
    int rc;
//
// Allow kernel address to register with OP-TEE as kernel
// pages are configured as normal memory only.
//
    if (virt_addr_valid((void *)start) || is_vmalloc_addr((void *)start))
    return 0;
    mmap_read_lock(mm);
    rc = __check_mem_type(mm, start, start + num_pages * PAGE_SIZE);
    mmap_read_unlock(mm);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn simple_call_with_arg(ctx: *mut tee_context, cmd: u32) -> c_int {
    static int simple_call_with_arg(struct tee_context *ctx, u32 cmd)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    struct optee_shm_arg_entry *entry;
    struct optee_msg_arg *msg_arg;
    struct tee_shm *shm;
    u_int offs;
    msg_arg = optee_get_msg_arg(ctx, 0, &entry, &shm, &offs);
    if (IS_ERR(msg_arg))
    return PTR_ERR(msg_arg);
    msg_arg.cmd = cmd;
    optee.ops.do_call_with_arg(ctx, shm, offs, false);
    optee_free_msg_arg(ctx, entry, offs);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_do_bottom_half(ctx: *mut tee_context) -> c_int {
    int optee_do_bottom_half(struct tee_context *ctx)
    {
    return simple_call_with_arg(ctx, OPTEE_MSG_CMD_DO_BOTTOM_HALF);
    }
#[no_mangle]
pub unsafe extern "C" fn optee_stop_async_notif(ctx: *mut tee_context) -> c_int {
    int optee_stop_async_notif(struct tee_context *ctx)
    {
    return simple_call_with_arg(ctx, OPTEE_MSG_CMD_STOP_ASYNC_NOTIF);
    }
