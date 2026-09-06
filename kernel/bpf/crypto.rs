//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/crypto.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2024 Meta, Inc

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_crypto_type_list {
    pub type: *const bpf_crypto_type,
    pub list: list_head,
}

// BPF crypto initialization parameters struct
//
// struct bpf_crypto_params - BPF crypto initialization parameters structure
// @type:	The string of crypto operation type.
// @reserved:	Reserved member, will be reused for more options in future
// Values:
// 0
// @algo:	The string of algorithm to initialize.
// @key:	The cipher key used to init crypto algorithm.
// @key_len:	The length of cipher key.
// @authsize:	The length of authentication tag used by algorithm.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_crypto_params {
    pub type: [c_char; 14],
    pub reserved: [u8; 2],
    pub algo: [c_char; 128],
    pub key: [u8; 256],
    pub key_len: u32,
    pub authsize: u32,
}

pub static mut bpf_crypto_types: usize = 0;
pub static mut bpf_crypto_types_sem: usize = 0;
//
// struct bpf_crypto_ctx - refcounted BPF crypto context structure
// @type:	The pointer to bpf crypto type
// @tfm:	The pointer to instance of crypto API struct.
// @siv_len:    Size of IV and state storage for cipher
// @rcu:	The RCU head used to free the crypto context with RCU safety.
// @usage:	Object reference counter. When the refcount goes to 0, the
// memory is released back to the BPF allocator, which provides
// RCU safety.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_crypto_ctx {
    pub type: *const bpf_crypto_type,
    pub tfm: *mut c_void,
    pub siv_len: u32,
    pub rcu: rcu_head,
    pub usage: refcount_t,
}

#[no_mangle]
pub unsafe extern "C" fn bpf_crypto_register_type(type: *const bpf_crypto_type) -> c_int {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    down_write(&bpf_crypto_types_sem);
    list_for_each_entry(node, &bpf_crypto_types, list) {
    if (!strcmp(node.type.name, type.name)) {
// goto;
    }
    }
    node = kmalloc_obj(*node);
    err = -ENOMEM;
    if (!node) {
// goto;
    }
    node.type = type;
    list_add(&node.list, &bpf_crypto_types);
    err = 0;
// label;
    up_write(&bpf_crypto_types_sem);
    return err;
    }
    EXPORT_SYMBOL_GPL(bpf_crypto_register_type);
#[no_mangle]
pub unsafe extern "C" fn bpf_crypto_unregister_type(type: *const bpf_crypto_type) -> c_int {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    down_write(&bpf_crypto_types_sem);
    list_for_each_entry(node, &bpf_crypto_types, list) {
    if (strcmp(node.type.name, type.name)) {
    continue;
    }
    list_del(&node.list);
    kfree(node);
    err = 0;
    break;
    }
    up_write(&bpf_crypto_types_sem);
    return err;
    }
    EXPORT_SYMBOL_GPL(bpf_crypto_unregister_type);
    static const struct bpf_crypto_type *bpf_crypto_get_type(const char *name)
    {
    let mut type = ERR_PTR(-ENOENT);
pub static mut node: *mut c_void = core::ptr::null_mut();
    down_read(&bpf_crypto_types_sem);
    list_for_each_entry(node, &bpf_crypto_types, list) {
    if (strcmp(node.type.name, name)) {
    continue;
    }
    if (try_module_get(node.type.owner)) {
    type = node.type;
    }
    break;
    }
    up_read(&bpf_crypto_types_sem);
    return type;
    }
    __bpf_kfunc_start_defs();
//
// bpf_crypto_ctx_create() - Create a mutable BPF crypto context.
//
// Allocates a crypto context that can be used, acquired, and released by
// a BPF program. The crypto context returned by this function must either
// be embedded in a map as a kptr, or freed with bpf_crypto_ctx_release().
// As crypto API functions use GFP_KERNEL allocations, this function can
// only be used in sleepable BPF programs.
//
// bpf_crypto_ctx_create() allocates memory for crypto context.
// It may return NULL if no memory is available.
// @params:	pointer to struct bpf_crypto_params which contains all the
// details needed to initialise crypto context.
// @params__sz:	size of steuct bpf_crypto_params usef by bpf program
// @err:	integer to store error code when NULL is returned.
//
    __bpf_kfunc struct bpf_crypto_ctx *
    bpf_crypto_ctx_create(const struct bpf_crypto_params *params, u32 params__sz,
    int *err)
    {
pub static mut type: *mut c_void = core::ptr::null_mut();
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    if (!params || params.reserved[0] || params.reserved[1] ||
    params__sz != sizeof!(bpf_crypto_params)) {
// err = -EINVAL;
    return core::ptr::null_mut();
    }
    type = bpf_crypto_get_type(params.type);
    if (IS_ERR(type)) {
// err = PTR_ERR(type);
    return core::ptr::null_mut();
    }
    if (!type.has_algo(params.algo)) {
// err = -EOPNOTSUPP;
// goto;
    }
    if (!!params.authsize ^ !!type.setauthsize) {
// err = -EOPNOTSUPP;
// goto;
    }
    if (!params.key_len || params.key_len > sizeof!(params.key)) {
// err = -EINVAL;
// goto;
    }
    ctx = kzalloc_obj(*ctx);
    if (!ctx) {
// err = -ENOMEM;
// goto;
    }
    ctx.type = type;
    ctx.tfm = type.alloc_tfm(params.algo);
    if (IS_ERR(ctx.tfm)) {
// err = PTR_ERR(ctx->tfm);
// goto;
    }
    if (params.authsize) {
// err = type->setauthsize(ctx->tfm, params->authsize);
    if (*err) {
// goto;
    }
    }
// err = type->setkey(ctx->tfm, params->key, params->key_len);
    if (*err) {
// goto;
    }
    if (type.get_flags(ctx.tfm) & CRYPTO_TFM_NEED_KEY) {
// err = -EINVAL;
// goto;
    }
    ctx.siv_len = type.ivsize(ctx.tfm) + type.statesize(ctx.tfm);
    refcount_set(&ctx.usage, 1);
    return ctx;
// label;
    type.free_tfm(ctx.tfm);
// label;
    kfree(ctx);
// label;
    module_put!(type.owner);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn crypto_free_cb(head: *mut rcu_head) {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    ctx = container_of!(head, bpf_crypto_ctx, rcu);
    ctx.type.free_tfm(ctx.tfm);
    module_put!(ctx.type.owner);
    kfree(ctx);
    }
//
// bpf_crypto_ctx_acquire() - Acquire a reference to a BPF crypto context.
// @ctx: The BPF crypto context being acquired. The ctx must be a trusted
// pointer.
//
// Acquires a reference to a BPF crypto context. The context returned by this function
// must either be embedded in a map as a kptr, or freed with
// bpf_crypto_ctx_release().
//
    __bpf_kfunc struct bpf_crypto_ctx *
    bpf_crypto_ctx_acquire(bpf_crypto_ctx *ctx)
    {
    if (!refcount_inc_not_zero(&ctx.usage)) {
    return core::ptr::null_mut();
    }
    return ctx;
    }
//
// bpf_crypto_ctx_release() - Release a previously acquired BPF crypto context.
// @ctx: The crypto context being released.
//
// Releases a previously acquired reference to a BPF crypto context. When the final
// reference of the BPF crypto context has been released, its memory
// will be released.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_crypto_ctx_release(ctx: *mut bpf_crypto_ctx) -> __bpf_kfunc void {
    if (refcount_dec_and_test(&ctx.usage)) {
    call_rcu(&ctx.rcu, crypto_free_cb);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_crypto_ctx_release_dtor(ctx: *mut c_void) -> __bpf_kfunc void {
    bpf_crypto_ctx_release(ctx);
    }
    CFI_NOSEAL(bpf_crypto_ctx_release_dtor);
#[no_mangle]
pub unsafe extern "C" fn bpf_crypto_crypt(ctx: *mut bpf_crypto_ctx, src: *mut bpf_dynptr_kern, dst: *mut bpf_dynptr_kern, siv: *mut bpf_dynptr_kern, decrypt: bool) -> c_int {
    u32 src_len, dst_len, siv_len;
pub static mut psrc: *mut c_void = core::ptr::null_mut();
    let mut pdst = core::ptr::null_mut();
    let mut piv = core::ptr::null_mut();
    let mut err = 0;
    if (__bpf_dynptr_is_rdonly(dst)) {
    return -EINVAL;
    }
    siv_len = siv ? __bpf_dynptr_size(siv) : 0;
    src_len = __bpf_dynptr_size(src);
    dst_len = __bpf_dynptr_size(dst);
    if (!src_len || !dst_len || src_len > dst_len) {
    return -EINVAL;
    }
    if (siv_len != ctx.siv_len) {
    return -EINVAL;
    }
    psrc = __bpf_dynptr_data(src, src_len);
    if (!psrc) {
    return -EINVAL;
    }
    pdst = __bpf_dynptr_data_rw(dst, dst_len);
    if (!pdst) {
    return -EINVAL;
    }
    piv = siv_len ? __bpf_dynptr_data_rw(siv, siv_len) : core::ptr::null_mut();
    if (siv_len && !piv) {
    return -EINVAL;
    }
    err = decrypt ? ctx.type.decrypt(ctx.tfm, psrc, pdst, src_len, piv)
    : ctx.type.encrypt(ctx.tfm, psrc, pdst, src_len, piv);
    return err;
    }
//
// bpf_crypto_decrypt() - Decrypt buffer using configured context and IV provided.
// @ctx:		The crypto context being used. The ctx must be a trusted pointer.
// @src:		bpf_dynptr to the encrypted data. Must be a trusted pointer.
// @dst:		bpf_dynptr to the buffer where to store the result. Must be a trusted pointer.
// @siv__nullable:	bpf_dynptr to IV data and state data to be used by decryptor. May be NULL.
//
// Decrypts provided buffer using IV data and the crypto context. Crypto context must be configured.
//
    __bpf_kfunc int bpf_crypto_decrypt(bpf_crypto_ctx *ctx,
    const struct bpf_dynptr *src,
    const struct bpf_dynptr *dst,
    const struct bpf_dynptr *siv__nullable)
    {
    let mut src_kern = src;
    let mut dst_kern = dst;
    let mut siv_kern = siv__nullable;
    return bpf_crypto_crypt(ctx, src_kern, dst_kern, siv_kern, true);
    }
//
// bpf_crypto_encrypt() - Encrypt buffer using configured context and IV provided.
// @ctx:		The crypto context being used. The ctx must be a trusted pointer.
// @src:		bpf_dynptr to the plain data. Must be a trusted pointer.
// @dst:		bpf_dynptr to the buffer where to store the result. Must be a trusted pointer.
// @siv__nullable:	bpf_dynptr to IV data and state data to be used by decryptor. May be NULL.
//
// Encrypts provided buffer using IV data and the crypto context. Crypto context must be configured.
//
    __bpf_kfunc int bpf_crypto_encrypt(bpf_crypto_ctx *ctx,
    const struct bpf_dynptr *src,
    const struct bpf_dynptr *dst,
    const struct bpf_dynptr *siv__nullable)
    {
    let mut src_kern = src;
    let mut dst_kern = dst;
    let mut siv_kern = siv__nullable;
    return bpf_crypto_crypt(ctx, src_kern, dst_kern, siv_kern, false);
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(crypt_init_kfunc_btf_ids)
    BTF_ID_FLAGS(func, bpf_crypto_ctx_create, KF_ACQUIRE | KF_RET_NULL | KF_SLEEPABLE)
    BTF_ID_FLAGS(func, bpf_crypto_ctx_release, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_crypto_ctx_acquire, KF_ACQUIRE | KF_RCU | KF_RET_NULL)
    BTF_KFUNCS_END(crypt_init_kfunc_btf_ids)
pub static mut btf_kfunc_id_set: usize = 0;
    BTF_KFUNCS_START(crypt_kfunc_btf_ids)
    BTF_ID_FLAGS(func, bpf_crypto_decrypt, KF_RCU)
    BTF_ID_FLAGS(func, bpf_crypto_encrypt, KF_RCU)
    BTF_KFUNCS_END(crypt_kfunc_btf_ids)
pub static mut btf_kfunc_id_set: usize = 0;
    BTF_ID_LIST(bpf_crypto_dtor_ids)
    BTF_ID(struct, bpf_crypto_ctx)
    BTF_ID(func, bpf_crypto_ctx_release_dtor)
#[no_mangle]
unsafe extern "C" fn crypto_kfunc_init() -> c_int {
    let mut ret = 0;
pub static mut btf_id_dtor_kfunc: usize = 0;
    ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &crypt_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_ACT, &crypt_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP, &crypt_kfunc_set);
    ret = ret ?: register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL,
    &crypt_init_kfunc_set);
    return  ret ?: register_btf_id_dtor_kfuncs(bpf_crypto_dtors,
    ARRAY_SIZE!(bpf_crypto_dtors),
    THIS_MODULE);
    }
    late_initcall!(crypto_kfunc_init);