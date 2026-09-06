//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/atmdev.h
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
// atmdev.h - ATM device driver declarations and various related items

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_atm_iobuf {
    pub length: c_int,
    pub buffer: compat_uptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_atm_aal_stats {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_atm_dev_stats {
    pub aal0: k_atm_aal_stats,
    pub aal34: k_atm_aal_stats,
    pub aal5: k_atm_aal_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_vcc {
// struct sock has to be the first member of atm_vcc
    pub sk: sock,
    pub /: *mut *mut *mut unsigned long flags; / VCC flags (ATM_VF_),
    pub /: *mut *mut short vpi; / VPI and VCI (types must be equal,
// with sockaddr)
    pub vci: c_int,
    pub /: *mut *mut unsigned long aal_options; / AAL layer options,
    pub /: *mut *mut unsigned long atm_options; / ATM layer options,
    pub /: *mut *mut *mut atm_dev dev; / device back pointer,
    pub /: *mut *mut atm_qos qos; / QOS,
    pub /: *mut *mut *mut *mut void (release_cb)(struct atm_vcc vcc); / release_sock callback,
    pub skb): *mut *mut *mut void (push)(struct atm_vcc vcc,struct sk_buff,
    pub /: *mut *mut *mut *mut *mut void (pop)(struct atm_vcc vcc,struct sk_buff skb); / optional,
    pub skb): *mut *mut *mut int (send)(struct atm_vcc vcc,struct sk_buff,
    pub /: *mut *mut *mut void dev_data; / per-device data,
    pub /: *mut *mut *mut void proto_data; / per-protocol data,
    pub /: *mut *mut *mut k_atm_aal_stats stats; / pointer to AAL stats group,
    pub /: *mut *mut *mut module owner; / owner of ->push function,
    pub /: *mut *mut *mut void user_back; / user backlink - not touched by the,
// native ATM stack, used by sch_atm
}

extern "C" {
    pub fn atm_sk(_arg: sock->sk) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_dev {
    pub /: *const *const *const atmdev_ops ops; / device operations; NULL if unused,
    pub /: *const *const *const char type; / device type name,
    pub /: *mut *mut int number; / device index,
    pub /: *mut *mut *mut void dev_data; / per-device data,
    pub /: *mut *mut *mut void phy_data; / private PHY data,
    pub /: *mut *mut *mut unsigned long flags; / device flags (ATM_DF_),
    pub /: *mut *mut unsigned char esi[ESI_LEN]; / ESI ("MAC" addr),
    pub /: *mut *mut atm_cirange ci_range; / VPI/VCI range,
    pub /: *mut *mut k_atm_dev_stats stats; / statistics,
    pub /: *mut *mut *mut char signal; / signal status (ATM_PHY_SIG_),
    pub /: *mut *mut int link_rate; / link rate (default: OC3),
    pub /: *mut *mut refcount_t refcnt; / reference count,

    pub /: *mut *mut *mut proc_dir_entry proc_entry; / proc entry,
    pub /: *mut *mut *mut char proc_name; / proc entry name,

    pub /: *mut *mut device class_dev; / sysfs device,
    pub /: *mut *mut list_head dev_list; / linkage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmdev_ops {
    pub dev): *mut *mut void (dev_close)(struct atm_dev,
    pub vcc): *mut *mut int (open)(struct atm_vcc,
    pub vcc): *mut *mut void (close)(struct atm_vcc,
    pub arg): *mut *mut *mut int (ioctl)(struct atm_dev dev,unsigned int cmd,void __user,

    pub arg): *mut void __user,

    pub skb): *mut *mut *mut int (send)(struct atm_vcc vcc,struct sk_buff,
    pub page): *mut *mut *mut *mut int (proc_read)(struct atm_dev dev,loff_t pos,char,
    pub owner: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_skb_data {
    pub /: *mut *mut *mut atm_vcc vcc; / ATM VCC,
    pub /: *mut *mut unsigned long atm_options; / ATM layer options,
    pub /: *mut *mut unsigned int acct_truesize; / truesize accounted to vcc,
    pub __packed: },
pub const VCC_HTABLE_SIZE: c_int = 32;
    pub vcc_hash: [extern struct hlist_head; VCC_HTABLE_SIZE],
    pub vcc_sklist_lock: extern rwlock_t,

    pub flags): *mut c_ulong,
    pub number): *mut *mut atm_dev atm_dev_lookup(int,
    pub dev): *mut void atm_dev_deregister(struct atm_dev,
// atm_dev_signal_change
//
// Propagate lower layer signal change in atm_dev->signal to netdevice.
// The event will be sent via a notifier call chain.
//
    pub signal): *mut *mut void atm_dev_signal_change(struct atm_dev dev, char,
    pub sk): *mut void vcc_insert_socket(struct sock,
    pub dev): *mut void atm_dev_release_vccs(struct atm_dev,
//
// Because ATM skbs may not belong to a sock (and we don't
// necessarily want to), skb->truesize may be adjusted,
// escaping the hack in pskb_expand_head() which avoids
// doing so for some cases. So stash the value of truesize
// at the time we accounted it, and atm_pop_raw() can use
// that value later, in case it changes.
//
    pub &sk_atm(vcc)->sk_wmem_alloc): refcount_add(skb->truesize,,
    pub skb->truesize: ATM_SKB(skb)->acct_truesize =,
    pub vcc->atm_options: ATM_SKB(skb)->atm_options =,
    pub &sk_atm(vcc)->sk_rmem_alloc): atomic_add(truesize,,
    pub &sk_atm(vcc)->sk_rmem_alloc): atomic_sub(truesize,,
    pub &dev->flags)): BUG_ON(!test_bit(ATM_DF_REMOVED,,
    pub truesize): *mut *mut int atm_charge(struct atm_vcc vcc,int,
    pub reply): *mut *mut void vcc_release_async(struct atm_vcc vcc, int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_ioctl {
    pub owner: *mut module,
// A module reference is kept if appropriate over this call.
// Return -ENOIOCTLCMD if you don't handle it.
    pub arg): *mut *mut *mut int (ioctl)(struct socket , unsigned int cmd, unsigned long,
    pub list: list_head,
}

//
// register_atm_ioctl - register handler for ioctl operations
// @ioctl: ioctl handler to register
//
// Special (non-device) handlers of ioctl's should
// register here. If you're a normal device, you should
// set .ioctl in your atmdev_ops instead.
//
extern "C" {
    pub fn register_atm_ioctl(ioctl: *mut atm_ioctl);
}
//
// deregister_atm_ioctl - remove the ioctl handler
// @ioctl: ioctl handler to deregister
//
extern "C" {
    pub fn deregister_atm_ioctl(ioctl: *mut atm_ioctl);
}
// register_atmdevice_notifier - register atm_dev notify events
//
// Clients like br2684 will register notify events
// Currently we notify of signal found/lost
//
extern "C" {
    pub fn register_atmdevice_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_atmdevice_notifier(nb: *mut notifier_block);
}
