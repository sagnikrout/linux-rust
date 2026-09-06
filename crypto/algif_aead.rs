//! Automatically rewritten from C to Rust
//! Source: crypto/algif_aead.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// algif_aead: User-space interface for AEAD algorithms
//
// Copyright (C) 2014, Stephan Mueller <smueller@chronox.de>
//
// This file provides the user-space API for AEAD ciphers.
//
// The following concept of the memory management is used:
//
// The kernel maintains two SGLs, the TX SGL and the RX SGL. The TX SGL is
// filled by user space with the data submitted via sendmsg.  Filling up the TX
// SGL does not cause a crypto operation -- the data will only be tracked by the
// kernel. Upon receipt of one recvmsg call, the caller must provide a buffer
// which is tracked with the RX SGL.
//
// During the processing of the recvmsg operation, the cipher request is
// allocated and prepared. As part of the recvmsg operation, the processed
// TX buffers are extracted from the TX SGL into a separate SGL.
//
// After the completion of the crypto operation, the RX SGL and the cipher
// request is released. The extracted TX SGL parts are released together with
// the RX SGL release.
//

    static const struct af_alg_allowlist_entry aead_allowlist[] = {
    { "ccm(aes)" }, /* bluez */
    {},
    };
#[no_mangle]
pub unsafe extern "C" fn aead_sufficient_data(sk: *mut sock) -> bool {
    static inline bool aead_sufficient_data(struct sock *sk)
    {
    struct alg_sock *ask = alg_sk(sk);
    struct sock *psk = ask.parent;
    struct alg_sock *pask = alg_sk(psk);
    struct af_alg_ctx *ctx = ask.private;
    struct crypto_aead *tfm = pask.private;
    let mut as: c_uint = crypto_aead_authsize(tfm);
//
// The minimum amount of memory needed for an AEAD cipher is
// the AAD and in case of decryption the tag.
//
    return ctx.used >= ctx.aead_assoclen + (ctx.enc ? 0 : as);
    }
#[no_mangle]
unsafe extern "C" fn aead_sendmsg(sock: *mut socket, msg: *mut msghdr, size: usize) -> c_int {
    static int aead_sendmsg(struct socket *sock, struct msghdr *msg, size_t size)
    {
    struct sock *sk = sock.sk;
    struct alg_sock *ask = alg_sk(sk);
    struct sock *psk = ask.parent;
    struct alg_sock *pask = alg_sk(psk);
    struct crypto_aead *tfm = pask.private;
    let mut ivsize: c_uint = crypto_aead_ivsize(tfm);
    return af_alg_sendmsg(sock, msg, size, ivsize);
    }
    static int _aead_recvmsg(struct socket *sock, struct msghdr *msg,
    size_t ignored, int flags)
    {
    struct sock *sk = sock.sk;
    struct alg_sock *ask = alg_sk(sk);
    struct sock *psk = ask.parent;
    struct alg_sock *pask = alg_sk(psk);
    struct af_alg_ctx *ctx = ask.private;
    struct crypto_aead *tfm = pask.private;
    let mut as: c_uint = crypto_aead_authsize(tfm);
    let mut ivsize: c_uint = crypto_aead_ivsize(tfm);
    struct af_alg_async_req *areq;
    struct scatterlist *rsgl_src, *tsgl_src = core::ptr::null_mut();
    void *iv;
    let mut err: c_int = 0;
    size_t used = 0;		/* [in]  TX bufs to be en/decrypted */
    size_t outlen = 0;		/* [out] RX bufs produced by kernel */
    size_t usedpages = 0;		/* [in]  RX bufs to be used from user */
    size_t processed = 0;		/* [in]  TX bufs to be consumed */
    if (!ctx.init || ctx.more) {
    err = af_alg_wait_for_data(sk, flags, 0);
    if (err)
    return err;
    }
//
// Data length provided by caller via sendmsg that has not yet been
// processed.
//
    used = ctx.used;
//
// Make sure sufficient data is present -- note, the same check is also
// present in sendmsg. The checks in sendmsg shall provide an
// information to the data sender that something is wrong, but they are
// irrelevant to maintain the kernel integrity.  We need this check
// here too in case user space decides to not honor the error message
// in sendmsg and still call recvmsg. This check here protects the
// kernel integrity.
//
    if (!aead_sufficient_data(sk))
    return -EINVAL;
//
// Calculate the minimum output buffer size holding the result of the
// cipher operation. When encrypting data, the receiving buffer is
// larger by the tag length compared to the input buffer as the
// encryption operation generates the tag. For decryption, the input
// buffer provides the tag which is consumed resulting in only the
// plaintext without a buffer for the tag returned to the caller.
//
    if (ctx.enc)
    outlen = used + as;
    else
    outlen = used - as;
//
// The cipher operation input data is reduced by the associated data
// length as this data is processed separately later on.
//
    used -= ctx.aead_assoclen;
// Allocate cipher request for current operation.
    areq = af_alg_alloc_areq(sk, sizeof(struct af_alg_async_req) +
    crypto_aead_reqsize(tfm) + ivsize);
    if (IS_ERR(areq))
    return PTR_ERR(areq);
    iv = (u8 *)aead_request_ctx(&areq.cra_u.aead_req) +
    crypto_aead_reqsize(tfm);
    memcpy(iv, ctx.iv, ivsize);
// convert iovecs of output buffers into RX SGL
    err = af_alg_get_rsgl(sk, msg, flags, areq, outlen, &usedpages);
    if (err)
    goto free;
//
// Ensure output buffer is sufficiently large. If the caller provides
// less buffer space, only use the relative required input size. This
// allows AIO operation where the caller sent all data to be processed
// and the AIO operation performs the operation on the different chunks
// of the input data.
//
    if (usedpages < outlen) {
    let mut less: usize = outlen - usedpages;
    if (used < less + (ctx.enc ? 0 : as)) {
    err = -EINVAL;
    goto free;
    }
    used -= less;
    outlen -= less;
    }
//
// Create a per request TX SGL for this request which tracks the
// SG entries from the global TX SGL.
//
    processed = used + ctx.aead_assoclen;
    areq.tsgl_entries = af_alg_count_tsgl(sk, processed);
    if (!areq.tsgl_entries)
    areq.tsgl_entries = 1;
    areq.tsgl = sock_kmalloc(sk, array_size(sizeof(*areq.tsgl),
    areq.tsgl_entries),
    GFP_KERNEL);
    if (!areq.tsgl) {
    err = -ENOMEM;
    goto free;
    }
    sg_init_table(areq.tsgl, areq.tsgl_entries);
    af_alg_pull_tsgl(sk, processed, areq.tsgl);
    tsgl_src = areq.tsgl;
//
// Copy of AAD from source to destination
//
// The AAD is copied to the destination buffer without change. Even
// when user space uses an in-place cipher operation, the kernel
// will copy the data as it does not see whether such in-place operation
// is initiated.
//
// Use the RX SGL as source (and destination) for crypto op.
    rsgl_src = areq.first_rsgl.sgl.sgt.sgl;
    memcpy_sglist(rsgl_src, tsgl_src, ctx.aead_assoclen);
// Initialize the crypto operation
    aead_request_set_crypt(&areq.cra_u.aead_req, tsgl_src,
    areq.first_rsgl.sgl.sgt.sgl, used, iv);
    aead_request_set_ad(&areq.cra_u.aead_req, ctx.aead_assoclen);
    aead_request_set_tfm(&areq.cra_u.aead_req, tfm);
    aead_request_set_callback(&areq.cra_u.aead_req,
    CRYPTO_TFM_REQ_MAY_SLEEP |
    CRYPTO_TFM_REQ_MAY_BACKLOG,
    crypto_req_done, &ctx.wait);
    err = crypto_wait_req(ctx.enc ?
    crypto_aead_encrypt(&areq.cra_u.aead_req) :
    crypto_aead_decrypt(&areq.cra_u.aead_req),
    &ctx.wait);
    free:
    af_alg_free_resources(areq);
    return err ? err : outlen;
    }
    static int aead_recvmsg(struct socket *sock, struct msghdr *msg,
    size_t ignored, int flags)
    {
    struct sock *sk = sock.sk;
    let mut ret: c_int = 0;
    lock_sock(sk);
    while (msg_data_left(msg)) {
    let mut err: c_int = _aead_recvmsg(sock, msg, ignored, flags);
//
// This error covers -EIOCBQUEUED which implies that we can
// only handle one AIO request. If the caller wants to have
// multiple AIO requests in parallel, he must make multiple
// separate AIO calls.
//
// Also return the error if no data has been processed so far.
//
    if (err <= 0) {
    if (err == -EIOCBQUEUED || err == -EBADMSG || !ret)
    ret = err;
    goto out;
    }
    ret += err;
    }
    out:
    af_alg_wmem_wakeup(sk);
    release_sock(sk);
    return ret;
    }
    static struct proto_ops algif_aead_ops = {
    .family		=	PF_ALG,
    .connect	=	sock_no_connect,
    .socketpair	=	sock_no_socketpair,
    .getname	=	sock_no_getname,
    .ioctl		=	sock_no_ioctl,
    .listen		=	sock_no_listen,
    .shutdown	=	sock_no_shutdown,
    .mmap		=	sock_no_mmap,
    .bind		=	sock_no_bind,
    .accept		=	sock_no_accept,
    .release	=	af_alg_release,
    .sendmsg	=	aead_sendmsg,
    .recvmsg	=	aead_recvmsg,
    .poll		=	af_alg_poll,
    };
#[no_mangle]
unsafe extern "C" fn aead_check_key(sock: *mut socket) -> c_int {
    static int aead_check_key(struct socket *sock)
    {
    let mut err: c_int = 0;
    struct sock *psk;
    struct alg_sock *pask;
    struct crypto_aead *tfm;
    struct sock *sk = sock.sk;
    struct alg_sock *ask = alg_sk(sk);
    lock_sock(sk);
    if (!atomic_read(&ask.nokey_refcnt))
    goto unlock_child;
    psk = ask.parent;
    pask = alg_sk(ask.parent);
    tfm = pask.private;
    err = -ENOKEY;
    lock_sock_nested(psk, SINGLE_DEPTH_NESTING);
    if (crypto_aead_get_flags(tfm) & CRYPTO_TFM_NEED_KEY)
    goto unlock;
    atomic_dec(&pask.nokey_refcnt);
    atomic_set(&ask.nokey_refcnt, 0);
    err = 0;
    unlock:
    release_sock(psk);
    unlock_child:
    release_sock(sk);
    return err;
    }
    static int aead_sendmsg_nokey(struct socket *sock, struct msghdr *msg,
    size_t size)
    {
    int err;
    err = aead_check_key(sock);
    if (err)
    return err;
    return aead_sendmsg(sock, msg, size);
    }
    static int aead_recvmsg_nokey(struct socket *sock, struct msghdr *msg,
    size_t ignored, int flags)
    {
    int err;
    err = aead_check_key(sock);
    if (err)
    return err;
    return aead_recvmsg(sock, msg, ignored, flags);
    }
    static struct proto_ops algif_aead_ops_nokey = {
    .family		=	PF_ALG,
    .connect	=	sock_no_connect,
    .socketpair	=	sock_no_socketpair,
    .getname	=	sock_no_getname,
    .ioctl		=	sock_no_ioctl,
    .listen		=	sock_no_listen,
    .shutdown	=	sock_no_shutdown,
    .mmap		=	sock_no_mmap,
    .bind		=	sock_no_bind,
    .accept		=	sock_no_accept,
    .release	=	af_alg_release,
    .sendmsg	=	aead_sendmsg_nokey,
    .recvmsg	=	aead_recvmsg_nokey,
    .poll		=	af_alg_poll,
    };
    static void *aead_bind(const char *name)
    {
    int err;
    err = af_alg_check_restriction(name, aead_allowlist);
    if (err)
    return ERR_PTR(err);
    return crypto_alloc_aead(name, 0, AF_ALG_CRYPTOAPI_MASK);
    }
#[no_mangle]
unsafe extern "C" fn aead_release(private: *mut c_void) {
    static void aead_release(void *private)
    {
    crypto_free_aead(private);
    }
#[no_mangle]
unsafe extern "C" fn aead_setauthsize(private: *mut c_void, authsize: c_uint) -> c_int {
    static int aead_setauthsize(void *private, unsigned int authsize)
    {
    return crypto_aead_setauthsize(private, authsize);
    }
#[no_mangle]
unsafe extern "C" fn aead_setkey(private: *mut c_void, key: *const u8, keylen: c_uint) -> c_int {
    static int aead_setkey(void *private, const u8 *key, unsigned int keylen)
    {
    return crypto_aead_setkey(private, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn aead_sock_destruct(sk: *mut sock) {
    static void aead_sock_destruct(struct sock *sk)
    {
    struct alg_sock *ask = alg_sk(sk);
    struct af_alg_ctx *ctx = ask.private;
    struct sock *psk = ask.parent;
    struct alg_sock *pask = alg_sk(psk);
    struct crypto_aead *tfm = pask.private;
    let mut ivlen: c_uint = crypto_aead_ivsize(tfm);
    af_alg_pull_tsgl(sk, ctx.used, core::ptr::null_mut());
    sock_kzfree_s(sk, ctx.iv, ivlen);
    sock_kfree_s(sk, ctx, ctx.len);
    af_alg_release_parent(sk);
    }
#[no_mangle]
unsafe extern "C" fn aead_accept_parent_nokey(private: *mut c_void, sk: *mut sock) -> c_int {
    static int aead_accept_parent_nokey(void *private, struct sock *sk)
    {
    struct af_alg_ctx *ctx;
    struct alg_sock *ask = alg_sk(sk);
    struct crypto_aead *tfm = private;
    let mut len: c_uint = sizeof(*ctx);
    let mut ivlen: c_uint = crypto_aead_ivsize(tfm);
    ctx = sock_kmalloc(sk, len, GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    memset(ctx, 0, len);
    ctx.iv = sock_kmalloc(sk, ivlen, GFP_KERNEL);
    if (!ctx.iv) {
    sock_kfree_s(sk, ctx, len);
    return -ENOMEM;
    }
    memset(ctx.iv, 0, ivlen);
    INIT_LIST_HEAD(&ctx.tsgl_list);
    ctx.len = len;
    crypto_init_wait(&ctx.wait);
    ask.private = ctx;
    sk.sk_destruct = aead_sock_destruct;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aead_accept_parent(private: *mut c_void, sk: *mut sock) -> c_int {
    static int aead_accept_parent(void *private, struct sock *sk)
    {
    struct crypto_aead *tfm = private;
    if (crypto_aead_get_flags(tfm) & CRYPTO_TFM_NEED_KEY)
    return -ENOKEY;
    return aead_accept_parent_nokey(private, sk);
    }
    static const struct af_alg_type algif_type_aead = {
    .bind		=	aead_bind,
    .release	=	aead_release,
    .setkey		=	aead_setkey,
    .setauthsize	=	aead_setauthsize,
    .accept		=	aead_accept_parent,
    .accept_nokey	=	aead_accept_parent_nokey,
    .ops		=	&algif_aead_ops,
    .ops_nokey	=	&algif_aead_ops_nokey,
    .name		=	"aead",
    .owner		=	THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn algif_aead_init() -> int __init {
    static int __init algif_aead_init(void)
    {
    return af_alg_register_type(&algif_type_aead);
    }
#[no_mangle]
unsafe extern "C" fn algif_aead_exit() -> void __exit {
    static void __exit algif_aead_exit(void)
    {
    let mut err: c_int = af_alg_unregister_type(&algif_type_aead);
    BUG_ON(err);
    }
    module_init(algif_aead_init);
    module_exit(algif_aead_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Stephan Mueller <smueller@chronox.de>");
    MODULE_DESCRIPTION("AEAD kernel crypto API user space interface");
