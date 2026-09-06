//! Automatically rewritten from C to Rust
//! Source: net/bluetooth/bnep/sock.c
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
//
    BNEP implementation for Linux Bluetooth stack (BlueZ).
    Copyright (C) 2001-2002 Inventel Systemes
    Written 2001-2002 by
    David Libault  <david.libault@inventel.fr>
    Copyright (C) 2002 Maxim Krasnyansky <maxk@qualcomm.com>
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
    OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
    IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
    CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
    WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
    ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
    OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
    ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
    COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
    SOFTWARE IS DISCLAIMED.
//

    static struct bt_sock_list bnep_sk_list = {
    .lock = __RW_LOCK_UNLOCKED(bnep_sk_list.lock)
    };
#[no_mangle]
unsafe extern "C" fn bnep_sock_release(sock: *mut socket) -> c_int {
    static int bnep_sock_release(struct socket *sock)
    {
    struct sock *sk = sock.sk;
    BT_DBG("sock %p sk %p", sock, sk);
    if (!sk)
    return 0;
    bt_sock_unlink(&bnep_sk_list, sk);
    sock_orphan(sk);
    sock_put(sk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_bnep_sock_ioctl(sock: *mut socket, cmd: c_uint, argp: *mut void __user) -> c_int {
    static int do_bnep_sock_ioctl(struct socket *sock, unsigned int cmd, void __user *argp)
    {
    struct bnep_connlist_req cl;
    struct bnep_connadd_req  ca;
    struct bnep_conndel_req  cd;
    struct bnep_conninfo ci;
    struct socket *nsock;
    let mut supp_feat: __u32 = BIT(BNEP_SETUP_RESPONSE);
    int err;
    BT_DBG("cmd %x arg %p", cmd, argp);
    switch (cmd) {
    case BNEPCONNADD:
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
    if (copy_from_user(&ca, argp, sizeof(ca)))
    return -EFAULT;
    nsock = sockfd_lookup(ca.sock, &err);
    if (!nsock)
    return err;
    if (nsock.sk.sk_state != BT_CONNECTED) {
    sockfd_put(nsock);
    return -EBADFD;
    }
    ca.device[sizeof(ca.device)-1] = 0;
    err = bnep_add_connection(&ca, nsock);
    if (!err) {
    if (copy_to_user(argp, &ca, sizeof(ca)))
    err = -EFAULT;
    } else
    sockfd_put(nsock);
    return err;
    case BNEPCONNDEL:
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
    if (copy_from_user(&cd, argp, sizeof(cd)))
    return -EFAULT;
    return bnep_del_connection(&cd);
    case BNEPGETCONNLIST:
    if (copy_from_user(&cl, argp, sizeof(cl)))
    return -EFAULT;
    if (cl.cnum <= 0)
    return -EINVAL;
    err = bnep_get_connlist(&cl);
    if (!err && copy_to_user(argp, &cl, sizeof(cl)))
    return -EFAULT;
    return err;
    case BNEPGETCONNINFO:
    if (copy_from_user(&ci, argp, sizeof(ci)))
    return -EFAULT;
    err = bnep_get_conninfo(&ci);
    if (!err && copy_to_user(argp, &ci, sizeof(ci)))
    return -EFAULT;
    return err;
    case BNEPGETSUPPFEAT:
    if (copy_to_user(argp, &supp_feat, sizeof(supp_feat)))
    return -EFAULT;
    return 0;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bnep_sock_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
    static int bnep_sock_ioctl(struct socket *sock, unsigned int cmd, unsigned long arg)
    {
    return do_bnep_sock_ioctl(sock, cmd, (void __user *)arg);
    }

#[no_mangle]
unsafe extern "C" fn bnep_sock_compat_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
    static int bnep_sock_compat_ioctl(struct socket *sock, unsigned int cmd, unsigned long arg)
    {
    void __user *argp = compat_ptr(arg);
    if (cmd == BNEPGETCONNLIST) {
    struct bnep_connlist_req cl;
    unsigned __user *p = argp;
    u32 uci;
    int err;
    if (get_user(cl.cnum, p) || get_user(uci, p + 1))
    return -EFAULT;
    cl.ci = compat_ptr(uci);
    if (cl.cnum <= 0)
    return -EINVAL;
    err = bnep_get_connlist(&cl);
    if (!err && put_user(cl.cnum, p))
    err = -EFAULT;
    return err;
    }
    return do_bnep_sock_ioctl(sock, cmd, argp);
    }

    static const struct proto_ops bnep_sock_ops = {
    .family		= PF_BLUETOOTH,
    .owner		= THIS_MODULE,
    .release	= bnep_sock_release,
    .ioctl		= bnep_sock_ioctl,

    .compat_ioctl	= bnep_sock_compat_ioctl,

    .bind		= sock_no_bind,
    .getname	= sock_no_getname,
    .sendmsg	= sock_no_sendmsg,
    .recvmsg	= sock_no_recvmsg,
    .listen		= sock_no_listen,
    .shutdown	= sock_no_shutdown,
    .connect	= sock_no_connect,
    .socketpair	= sock_no_socketpair,
    .accept		= sock_no_accept,
    .mmap		= sock_no_mmap
    };
    static struct proto bnep_proto = {
    .name		= "BNEP",
    .owner		= THIS_MODULE,
    .obj_size	= sizeof(struct bt_sock)
    };
    static int bnep_sock_create(struct net *net, struct socket *sock, int protocol,
    int kern)
    {
    struct sock *sk;
    BT_DBG("sock %p", sock);
    if (sock.type != SOCK_RAW)
    return -ESOCKTNOSUPPORT;
    sk = bt_sock_alloc(net, sock, &bnep_proto, protocol, GFP_ATOMIC, kern);
    if (!sk)
    return -ENOMEM;
    sock.ops = &bnep_sock_ops;
    sock.state = SS_UNCONNECTED;
    bt_sock_link(&bnep_sk_list, sk);
    return 0;
    }
    static const struct net_proto_family bnep_sock_family_ops = {
    .family = PF_BLUETOOTH,
    .owner	= THIS_MODULE,
    .create = bnep_sock_create
    };
#[no_mangle]
pub unsafe extern "C" fn bnep_sock_init() -> int __init {
    int __init bnep_sock_init(void)
    {
    int err;
    err = proto_register(&bnep_proto, 0);
    if (err < 0)
    return err;
    err = bt_sock_register(BTPROTO_BNEP, &bnep_sock_family_ops);
    if (err < 0) {
    BT_ERR("Can't register BNEP socket");
    goto error;
    }
    err = bt_procfs_init(&init_net, "bnep", &bnep_sk_list, core::ptr::null_mut());
    if (err < 0) {
    BT_ERR("Failed to create BNEP proc file");
    bt_sock_unregister(BTPROTO_BNEP);
    goto error;
    }
    BT_INFO("BNEP socket layer initialized");
    return 0;
    error:
    proto_unregister(&bnep_proto);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn bnep_sock_cleanup() -> void __exit {
    void __exit bnep_sock_cleanup(void)
    {
    bt_procfs_cleanup(&init_net, "bnep");
    bt_sock_unregister(BTPROTO_BNEP);
    proto_unregister(&bnep_proto);
    }
