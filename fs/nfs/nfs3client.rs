//! Automatically rewritten from C to Rust
//! Source: fs/nfs/nfs3client.c
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

    let mut nfsacl_rpcstat: static struct rpc_stat = { &nfsacl_program };
    static const struct rpc_version *nfsacl_version[] = {
    [3]			= &nfsacl_version3,
    };
    const struct rpc_program nfsacl_program = {
    .name			= "nfsacl",
    .number			= NFS_ACL_PROGRAM,
    .nrvers			= ARRAY_SIZE(nfsacl_version),
    .version		= nfsacl_version,
    .stats			= &nfsacl_rpcstat,
    };
//
// Initialise an NFSv3 ACL client connection
//
#[no_mangle]
unsafe extern "C" fn nfs_init_server_aclclient(server: *mut nfs_server) {
    static void nfs_init_server_aclclient(struct nfs_server *server)
    {
    if (server.flags & NFS_MOUNT_NOACL)
    goto out_noacl;
    server.client_acl = rpc_bind_new_program(server.client, &nfsacl_program, 3);
    if (IS_ERR(server.client_acl))
    goto out_noacl;
    nfs_sysfs_link_rpc_client(server, server.client_acl, core::ptr::null_mut());
// No errors! Assume that Sun nfsacls are supported
    server.caps |= NFS_CAP_ACLS;
    return;
    out_noacl:
    server.caps &= ~NFS_CAP_ACLS;
    }

#[no_mangle]
pub unsafe extern "C" fn nfs_init_server_aclclient(server: *mut nfs_server) {
    static inline void nfs_init_server_aclclient(struct nfs_server *server)
    {
    server.flags &= ~NFS_MOUNT_NOACL;
    server.caps &= ~NFS_CAP_ACLS;
    }

    struct nfs_server *nfs3_create_server(struct fs_context *fc)
    {
    struct nfs_server *server = nfs_create_server(fc);
// Create a client RPC handle for the NFS v3 ACL management interface
    if (!IS_ERR(server))
    nfs_init_server_aclclient(server);
    return server;
    }
    struct nfs_server *nfs3_clone_server(struct nfs_server *source,
    struct nfs_fh *fh,
    struct nfs_fattr *fattr,
    rpc_authflavor_t flavor)
    {
    struct nfs_server *server = nfs_clone_server(source, fh, fattr, flavor);
    if (!IS_ERR(server) && !IS_ERR(source.client_acl))
    nfs_init_server_aclclient(server);
    return server;
    }
//
// Set up a pNFS Data Server client over NFSv3.
//
// Return any existing nfs_client that matches server address,port,version
// and minorversion.
//
// For a new nfs_client, use a soft mount (default), a low retrans and a
// low timeout interval so that if a connection is lost, we retry through
// the MDS.
//
    struct nfs_client *nfs3_set_ds_client(struct nfs_server *mds_srv,
    const struct sockaddr_storage *ds_addr, int ds_addrlen,
    int ds_proto, unsigned int ds_timeo, unsigned int ds_retrans)
    {
    struct rpc_timeout ds_timeout;
    let mut connect_timeout: c_ulong = ds_timeo * (ds_retrans + 1) * HZ / 10;
    struct nfs_client *mds_clp = mds_srv.nfs_client;
    struct nfs_client_initdata cl_init = {
    .addr = ds_addr,
    .addrlen = ds_addrlen,
    .nodename = mds_clp.cl_rpcclient.cl_nodename,
    .ip_addr = mds_clp.cl_ipaddr,
    .nfs_mod = &nfs_v3,
    .proto = ds_proto,
    .net = mds_clp.cl_net,
    .timeparms = &ds_timeout,
    .cred = mds_srv.cred,
    .xprtsec = {
    .policy = RPC_XPRTSEC_NONE,
    .cert_serial = TLS_NO_CERT,
    .privkey_serial = TLS_NO_PRIVKEY,
    },
    .connect_timeout = connect_timeout,
    .reconnect_timeout = connect_timeout,
    };
    struct nfs_client *clp;
    char buf[INET6_ADDRSTRLEN + 1];
// fake a hostname because lockd wants it
    if (rpc_ntop((struct sockaddr *)ds_addr, buf, sizeof(buf)) <= 0)
    return ERR_PTR(-EINVAL);
    cl_init.hostname = buf;
    switch (ds_proto) {
    case XPRT_TRANSPORT_TCP_TLS:
    if (mds_clp.cl_xprtsec.policy != RPC_XPRTSEC_NONE)
    cl_init.xprtsec = mds_clp.cl_xprtsec;
    else
    ds_proto = XPRT_TRANSPORT_TCP;
    fallthrough;
    case XPRT_TRANSPORT_RDMA:
    case XPRT_TRANSPORT_TCP:
    if (mds_clp.cl_nconnect > 1)
    cl_init.nconnect = mds_clp.cl_nconnect;
    }
    if (mds_srv.flags & NFS_MOUNT_NORESVPORT)
    __set_bit(NFS_CS_NORESVPORT, &cl_init.init_flags);
    if (test_bit(NFS_CS_NETUNREACH_FATAL, &mds_clp.cl_flags))
    __set_bit(NFS_CS_NETUNREACH_FATAL, &cl_init.init_flags);
    __set_bit(NFS_CS_DS, &cl_init.init_flags);
// Use the MDS nfs_client cl_ipaddr.
    nfs_init_timeout_values(&ds_timeout, ds_proto, ds_timeo, ds_retrans);
    clp = nfs_get_client(&cl_init);
    return clp;
    }
    EXPORT_SYMBOL_GPL(nfs3_set_ds_client);
