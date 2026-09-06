//! Automatically rewritten from C to Rust
//! Source: fs/nfs_common/nfslocalio.c
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
// Copyright (C) 2024 Mike Snitzer <snitzer@hammerspace.com>
// Copyright (C) 2024 NeilBrown <neilb@suse.de>
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("NFS localio protocol bypass support");
    static DEFINE_SPINLOCK(nfs_uuids_lock);
//
// Global list of nfs_uuid_t instances
// that is protected by nfs_uuids_lock.
//
    static LIST_HEAD(nfs_uuids);
//
// Lock ordering:
// 1: nfs_uuid->lock
// 2: nfs_uuids_lock
// 3: nfs_uuid->list_lock (aka nn->local_clients_lock)
//
// May skip locks in select cases, but never hold multiple
// locks out of order.
//
#[no_mangle]
pub unsafe extern "C" fn nfs_uuid_init(nfs_uuid: *mut nfs_uuid_t) {
    void nfs_uuid_init(nfs_uuid_t *nfs_uuid)
    {
    RCU_INIT_POINTER(nfs_uuid.net, core::ptr::null_mut());
    nfs_uuid.dom = core::ptr::null_mut();
    nfs_uuid.list_lock = core::ptr::null_mut();
    INIT_LIST_HEAD(&nfs_uuid.list);
    INIT_LIST_HEAD(&nfs_uuid.files);
    spin_lock_init(&nfs_uuid.lock);
    nfs_uuid.nfs3_localio_probe_count = 0;
    }
    EXPORT_SYMBOL_GPL(nfs_uuid_init);
#[no_mangle]
pub unsafe extern "C" fn nfs_uuid_begin(nfs_uuid: *mut nfs_uuid_t) -> bool {
    bool nfs_uuid_begin(nfs_uuid_t *nfs_uuid)
    {
    spin_lock(&nfs_uuid.lock);
    if (rcu_access_pointer(nfs_uuid.net)) {
// This nfs_uuid is already in use
    spin_unlock(&nfs_uuid.lock);
    return false;
    }
    spin_lock(&nfs_uuids_lock);
    if (!list_empty(&nfs_uuid.list)) {
// This nfs_uuid is already in use
    spin_unlock(&nfs_uuids_lock);
    spin_unlock(&nfs_uuid.lock);
    return false;
    }
    list_add_tail(&nfs_uuid.list, &nfs_uuids);
    spin_unlock(&nfs_uuids_lock);
    uuid_gen(&nfs_uuid.uuid);
    spin_unlock(&nfs_uuid.lock);
    return true;
    }
    EXPORT_SYMBOL_GPL(nfs_uuid_begin);
#[no_mangle]
pub unsafe extern "C" fn nfs_uuid_end(nfs_uuid: *mut nfs_uuid_t) {
    void nfs_uuid_end(nfs_uuid_t *nfs_uuid)
    {
    if (!rcu_access_pointer(nfs_uuid.net)) {
    spin_lock(&nfs_uuid.lock);
    if (!rcu_access_pointer(nfs_uuid.net)) {
// Not local, remove from nfs_uuids
    spin_lock(&nfs_uuids_lock);
    list_del_init(&nfs_uuid.list);
    spin_unlock(&nfs_uuids_lock);
    }
    spin_unlock(&nfs_uuid.lock);
    }
    }
    EXPORT_SYMBOL_GPL(nfs_uuid_end);
#[no_mangle]
unsafe extern "C" fn nfs_uuid_lookup_locked(uuid: *const uuid_t) -> *mut nfs_uuid_t {
    static nfs_uuid_t * nfs_uuid_lookup_locked(const uuid_t *uuid)
    {
    nfs_uuid_t *nfs_uuid;
    list_for_each_entry(nfs_uuid, &nfs_uuids, list)
    if (uuid_equal(&nfs_uuid.uuid, uuid))
    return nfs_uuid;
    return core::ptr::null_mut();
    }
    static struct module *nfsd_mod;
    void nfs_uuid_is_local(const uuid_t *uuid, struct list_head *list,
    spinlock_t *list_lock, struct net *net,
    struct auth_domain *dom, struct module *mod)
    {
    nfs_uuid_t *nfs_uuid;
    spin_lock(&nfs_uuids_lock);
    nfs_uuid = nfs_uuid_lookup_locked(uuid);
    if (!nfs_uuid) {
    spin_unlock(&nfs_uuids_lock);
    return;
    }
//
// We don't hold a ref on the net, but instead put
// ourselves on @list (nn->local_clients) so the net
// pointer can be invalidated.
//
    spin_lock(list_lock); /* list_lock is nn.local_clients_lock */
    list_move(&nfs_uuid.list, list);
    spin_unlock(list_lock);
    spin_unlock(&nfs_uuids_lock);
// Once nfs_uuid is parented to @list, avoid global nfs_uuids_lock
    spin_lock(&nfs_uuid.lock);
    __module_get(mod);
    nfsd_mod = mod;
    nfs_uuid.list_lock = list_lock;
    kref_get(&dom.ref);
    nfs_uuid.dom = dom;
    rcu_assign_pointer(nfs_uuid.net, net);
    spin_unlock(&nfs_uuid.lock);
    }
    EXPORT_SYMBOL_GPL(nfs_uuid_is_local);
#[no_mangle]
pub unsafe extern "C" fn nfs_localio_enable_client(clp: *mut nfs_client) {
    void nfs_localio_enable_client(struct nfs_client *clp)
    {
// nfs_uuid_is_local() does the actual enablement
    trace_nfs_localio_enable_client(clp);
    }
    EXPORT_SYMBOL_GPL(nfs_localio_enable_client);
//
// Cleanup the nfs_uuid_t embedded in an nfs_client.
// This is the long-form of nfs_uuid_init().
//
#[no_mangle]
unsafe extern "C" fn nfs_uuid_put(nfs_uuid: *mut nfs_uuid_t) -> bool {
    static bool nfs_uuid_put(nfs_uuid_t *nfs_uuid)
    {
    struct nfs_file_localio *nfl;
    spin_lock(&nfs_uuid.lock);
    if (unlikely(!rcu_access_pointer(nfs_uuid.net))) {
    spin_unlock(&nfs_uuid.lock);
    return false;
    }
    RCU_INIT_POINTER(nfs_uuid.net, core::ptr::null_mut());
    if (nfs_uuid.dom) {
    auth_domain_put(nfs_uuid.dom);
    nfs_uuid.dom = core::ptr::null_mut();
    }
// Walk list of files and ensure their last references dropped
    while ((nfl = list_first_entry_or_null(&nfs_uuid.files,
    struct nfs_file_localio,
    list)) != core::ptr::null_mut()) {
// If nfs_uuid is already NULL, nfs_close_local_fh is
// closing and we must wait, else we unlink and close.
//
    if (rcu_access_pointer(nfl.nfs_uuid) == core::ptr::null_mut()) {
// nfs_close_local_fh() is doing the
// close and we must wait. until it unlinks
//
    wait_var_event_spinlock(nfs_uuid,
    list_first_entry_or_null(
    &nfs_uuid.files,
    struct nfs_file_localio,
    list) != nfl,
    &nfs_uuid.lock);
    continue;
    }
// Remove nfl from nfs_uuid->files list
    list_del_init(&nfl.list);
    spin_unlock(&nfs_uuid.lock);
    nfs_to_nfsd_file_put_local(&nfl.ro_file);
    nfs_to_nfsd_file_put_local(&nfl.rw_file);
    cond_resched();
    spin_lock(&nfs_uuid.lock);
// Now we can allow racing nfs_close_local_fh() to
// skip the locking.
//
    store_release_wake_up(&nfl.nfs_uuid, RCU_INITIALIZER(core::ptr::null_mut()));
    }
// Remove client from nn->local_clients
    if (nfs_uuid.list_lock) {
    spin_lock(nfs_uuid.list_lock);
    BUG_ON(list_empty(&nfs_uuid.list));
    list_del_init(&nfs_uuid.list);
    spin_unlock(nfs_uuid.list_lock);
    nfs_uuid.list_lock = core::ptr::null_mut();
    }
    module_put(nfsd_mod);
    spin_unlock(&nfs_uuid.lock);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn nfs_localio_disable_client(clp: *mut nfs_client) {
    void nfs_localio_disable_client(struct nfs_client *clp)
    {
    if (nfs_uuid_put(&clp.cl_uuid))
    trace_nfs_localio_disable_client(clp);
    }
    EXPORT_SYMBOL_GPL(nfs_localio_disable_client);
    void nfs_localio_invalidate_clients(struct list_head *nn_local_clients,
    spinlock_t *nn_local_clients_lock)
    {
    LIST_HEAD(local_clients);
    nfs_uuid_t *nfs_uuid, *tmp;
    struct nfs_client *clp;
    spin_lock(nn_local_clients_lock);
    list_splice_init(nn_local_clients, &local_clients);
    spin_unlock(nn_local_clients_lock);
    list_for_each_entry_safe(nfs_uuid, tmp, &local_clients, list) {
    if (WARN_ON(nfs_uuid.list_lock != nn_local_clients_lock))
    break;
    clp = container_of(nfs_uuid, struct nfs_client, cl_uuid);
    nfs_localio_disable_client(clp);
    }
    }
    EXPORT_SYMBOL_GPL(nfs_localio_invalidate_clients);
#[no_mangle]
unsafe extern "C" fn nfs_uuid_add_file(nfs_uuid: *mut nfs_uuid_t, nfl: *mut nfs_file_localio) -> c_int {
    static int nfs_uuid_add_file(nfs_uuid_t *nfs_uuid, struct nfs_file_localio *nfl)
    {
    let mut ret: c_int = 0;
// Add nfl to nfs_uuid->files if it isn't already
    spin_lock(&nfs_uuid.lock);
    if (rcu_access_pointer(nfs_uuid.net) == core::ptr::null_mut()) {
    ret = -ENXIO;
    } else if (list_empty(&nfl.list)) {
    rcu_assign_pointer(nfl.nfs_uuid, nfs_uuid);
    list_add_tail(&nfl.list, &nfs_uuid.files);
    }
    spin_unlock(&nfs_uuid.lock);
    return ret;
    }
//
// Caller is responsible for calling nfsd_net_put and
// nfsd_file_put (via nfs_to_nfsd_file_put_local).
//
    struct nfsd_file *nfs_open_local_fh(nfs_uuid_t *uuid,
    struct rpc_clnt *rpc_clnt, const struct cred *cred,
    const struct nfs_fh *nfs_fh, struct nfs_file_localio *nfl,
    struct nfsd_file __rcu **pnf,
    const fmode_t fmode)
    {
    struct net *net;
    struct nfsd_file *localio;
//
// Not running in nfsd context, so must safely get reference on nfsd_serv.
// But the server may already be shutting down, if so disallow new localio.
// uuid->net is NOT a counted reference, but rcu_read_lock() ensures that
// if uuid->net is not NULL, then calling nfsd_net_try_get() is safe
// and if it succeeds we will have an implied reference to the net.
//
// Otherwise NFS may not have ref on NFSD and therefore cannot safely
// make 'nfs_to' calls.
//
    rcu_read_lock();
    net = rcu_dereference(uuid.net);
    if (!net || !nfs_to.nfsd_net_try_get(net)) {
    rcu_read_unlock();
    return ERR_PTR(-ENXIO);
    }
    rcu_read_unlock();
// We have an implied reference to net thanks to nfsd_net_try_get
    localio = nfs_to.nfsd_open_local_fh(net, uuid.dom, rpc_clnt, cred,
    nfs_fh, pnf, fmode);
    if (!IS_ERR(localio) && nfs_uuid_add_file(uuid, nfl) < 0) {
//
// Delete the cached file when racing with nfs_uuid_put().
// Since nfl->nfs_uuid was never published via
// rcu_assign_pointer(), nfs_close_local_fh() will early-return
// and cannot clean up after us.  Drop the slot's file ref and
// its paired net ref, then drop the caller-owned nfsd_file ref
// (+1) and the entry-time nfsd_net ref carried via nf->nf_net,
// and return -ENXIO so the caller never dereferences the
// now-cleared localio.
//
    struct nfsd_file __rcu *tmp =
    (struct nfsd_file  __rcu *)localio;
    nfs_to_nfsd_file_put_local(pnf);
    nfs_to_nfsd_file_put_local(&tmp);
    localio = ERR_PTR(-ENXIO);
    }
    nfs_to_nfsd_net_put(net);
    return localio;
    }
    EXPORT_SYMBOL_GPL(nfs_open_local_fh);
#[no_mangle]
pub unsafe extern "C" fn nfs_close_local_fh(nfl: *mut nfs_file_localio) {
    void nfs_close_local_fh(struct nfs_file_localio *nfl)
    {
    nfs_uuid_t *nfs_uuid;
    rcu_read_lock();
    nfs_uuid = rcu_dereference(nfl.nfs_uuid);
    if (!nfs_uuid) {
// regular (non-LOCALIO) NFS will hammer this
    rcu_read_unlock();
    return;
    }
    spin_lock(&nfs_uuid.lock);
    if (!rcu_access_pointer(nfl.nfs_uuid)) {
// nfs_uuid_put has finished here
    spin_unlock(&nfs_uuid.lock);
    rcu_read_unlock();
    return;
    }
    if (list_empty(&nfl.list)) {
// nfs_uuid_put() has started closing files, wait for it
// to finished
//
    spin_unlock(&nfs_uuid.lock);
    rcu_read_unlock();
    wait_var_event(&nfl.nfs_uuid,
    rcu_access_pointer(nfl.nfs_uuid) == core::ptr::null_mut());
    return;
    }
// tell nfs_uuid_put() to wait for us
    RCU_INIT_POINTER(nfl.nfs_uuid, core::ptr::null_mut());
    spin_unlock(&nfs_uuid.lock);
    rcu_read_unlock();
    nfs_to_nfsd_file_put_local(&nfl.ro_file);
    nfs_to_nfsd_file_put_local(&nfl.rw_file);
// Remove nfl from nfs_uuid->files list and signal nfs_uuid_put()
// that we are done.  The moment we drop the spinlock the
// nfs_uuid could be freed.
//
    spin_lock(&nfs_uuid.lock);
    list_del_init(&nfl.list);
    wake_up_var_locked(nfs_uuid, &nfs_uuid.lock);
    spin_unlock(&nfs_uuid.lock);
    }
    EXPORT_SYMBOL_GPL(nfs_close_local_fh);
//
// The NFS LOCALIO code needs to call into NFSD using various symbols,
// but cannot be statically linked, because that will make the NFS
// module always depend on the NFSD module.
//
// 'nfs_to' provides NFS access to NFSD functions needed for LOCALIO,
// its lifetime is tightly coupled to the NFSD module and will always
// be available to NFS LOCALIO because any successful client<->server
// LOCALIO handshake results in a reference on the NFSD module (above),
// so NFS implicitly holds a reference to the NFSD module and its
// functions in the 'nfs_to' nfsd_localio_operations cannot disappear.
//
// If the last NFS client using LOCALIO disconnects (and its reference
// on NFSD dropped) then NFSD could be unloaded, resulting in 'nfs_to'
// functions being invalid pointers. But if NFSD isn't loaded then NFS
// will not be able to handshake with NFSD and will have no cause to
// try to call 'nfs_to' function pointers. If/when NFSD is reloaded it
// will reinitialize the 'nfs_to' function pointers and make LOCALIO
// possible.
//
    const struct nfsd_localio_operations *nfs_to;
    EXPORT_SYMBOL_GPL(nfs_to);
