//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/aoe/aoe.h
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


// Copyright (c) 2013 Coraid, Inc.  See COPYING for GPL terms.

pub const AOE_MAJOR: c_int = 152;

// set AOE_PARTITIONS to 1 to use whole-disks only
// default is 16, which is 15 partitions plus the whole disk
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoe_hdr {
    pub dst: [c_uchar; 6],
    pub src: [c_uchar; 6],
    pub type: __be16,
    pub verfl: c_uchar,
    pub err: c_uchar,
    pub major: __be16,
    pub minor: c_uchar,
    pub cmd: c_uchar,
    pub tag: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoe_atahdr {
    pub aflags: c_uchar,
    pub errfeat: c_uchar,
    pub scnt: c_uchar,
    pub cmdstat: c_uchar,
    pub lba0: c_uchar,
    pub lba1: c_uchar,
    pub lba2: c_uchar,
    pub lba3: c_uchar,
    pub lba4: c_uchar,
    pub lba5: c_uchar,
    pub res: [c_uchar; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoe_cfghdr {
    pub bufcnt: __be16,
    pub fwver: __be16,
    pub scnt: c_uchar,
    pub aoeccmd: c_uchar,
    pub cslen: [c_uchar; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoe_req {
    pub nr_bios: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buf {
    pub nframesout: c_ulong,
    pub bio: *mut bio,
    pub iter: bvec_iter,
    pub rq: *mut request,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frame_flags {
    FFL_PROBE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame {
    pub head: list_head,
    pub tag: u32,
    pub /: *mut *mut ktime_t sent; / high-res time packet was sent,
    pub waited: c_ulong,
    pub waited_total: c_ulong,
    pub /: *mut *mut *mut aoetgt t; / parent target I belong to,
    pub /: *mut *mut *mut sk_buff skb; / command skb freed on module exit,
    pub /: *mut *mut *mut sk_buff r_skb; / response skb for async processing,
    pub buf: *mut buf,
    pub iter: bvec_iter,
    pub flags: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoeif {
    pub nd: *mut net_device,
    pub lost: c_ulong,
    pub bcnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoetgt {
    pub addr: [c_uchar; 6],
    pub /: *mut *mut ushort nframes; / cap on frames to use,
    pub /: *mut *mut *mut aoedev d; / parent device I belong to,
    pub /: *mut *mut list_head ffree; / list of free frames,
    pub ifs: [aoeif; NAOEIFS],
    pub /: *mut *mut *mut aoeif ifp; / current aoeif in use,
    pub /: *mut *mut ushort nout; / number of AoE commands outstanding,
    pub /: *mut *mut ushort maxout; / current value for max outstanding,
    pub /: *mut *mut ushort next_cwnd; / incr maxout after decrementing to zero,
    pub /: *mut *mut ushort ssthresh; / slow start threshold,
    pub /: *mut *mut ulong falloc; / number of allocated frames,
    pub /: *mut *mut int taint; / how much we want to avoid this aoetgt,
    pub minbcnt: c_int,
    pub rpkts: int wpkts,,
    pub nout_probes: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoedev {
    pub next: *mut aoedev,
    pub sysminor: c_ulong,
    pub aoemajor: c_ulong,
    pub /: *mut *mut u32 rttavg; / scaled AoE round trip time average,
    pub /: *mut *mut u32 rttdev; / scaled round trip time mean deviation,
    pub aoeminor: u16,
    pub flags: u16,
    pub /: *mut *mut u16 nopen; / (bd_openers isn't available without sleeping),
    pub /: *mut *mut u16 fw_ver; / version of blade's firmware,
    pub /: *mut *mut u16 lasttag; / last tag sent,
    pub useme: u16,
    pub ref: c_ulong,
    pub /: *mut *mut work_work;/ disk create work struct,
    pub gd: *mut gendisk,
    pub debugfs: *mut dentry,
    pub blkq: *mut request_queue,
    pub rq_list: list_head,
    pub tag_set: blk_mq_tag_set,
    pub geo: hd_geometry,
    pub ssize: sector_t,
    pub timer: timer_list,
    pub lock: spinlock_t,
    pub skbpool: sk_buff_head,
    pub /: *mut *mut *mut mempool_t bufpool; / for deadlock-free Buf allocation,
    pub buf: *mut buf,
    pub nxbio: *mut bio,
    pub rq: *mut request,
    pub ip: },
    pub maxbcnt: c_ulong,
    pub /: *mut *mut list_head factive[NFACTIVE]; / hash of active frames,
    pub /: *mut *mut list_head rexmitq; / deferred retransmissions,
    pub targets: *mut aoetgt,
    pub /: *mut *mut ulong ntargets; / number of allocated aoetgt pointers,
    pub /: *mut *mut *mut *mut aoetgt tgt; / target in use when working,
    pub kicked: c_ulong,
    pub ident: [c_char; 512],
}

// kthread tracking
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktstate {
    pub rendez: completion,
    pub task: *mut task_struct,
    pub waitq: *mut wait_queue_head_t,
    pub (int): *mut *mut int (fn),
    pub name: [c_char; 12],
    pub lock: *mut spinlock_t,
    pub id: c_int,
    pub active: c_int,
}

extern "C" {
    pub fn aoeblk_init() -> c_int;
}
extern "C" {
    pub fn aoeblk_exit();
}
extern "C" {
    pub fn aoeblk_gdalloc(: *mut c_void);
}
extern "C" {
    pub fn aoedisk_rm_debugfs(d: *mut aoedev);
}
extern "C" {
    pub fn aoechr_init() -> c_int;
}
extern "C" {
    pub fn aoechr_exit();
}
extern "C" {
    pub fn aoechr_error(: *mut c_char);
}
extern "C" {
    pub fn aoecmd_work(d: *mut aoedev);
}
extern "C" {
    pub fn aoecmd_cfg(aoemajor: c_ushort, aoeminor: c_uchar);
}
extern "C" {
    pub fn aoecmd_cfg_rsp(: *mut sk_buff);
}
extern "C" {
    pub fn aoecmd_sleepwork(: *mut work_struct);
}
extern "C" {
    pub fn aoecmd_wreset(t: *mut aoetgt);
}
extern "C" {
    pub fn aoecmd_cleanslate(: *mut aoedev);
}
extern "C" {
    pub fn aoecmd_exit();
}
extern "C" {
    pub fn aoecmd_init() -> c_int;
}
extern "C" {
    pub fn aoe_freetframe(: *mut frame);
}
extern "C" {
    pub fn aoe_flush_iocq();
}
extern "C" {
    pub fn aoe_flush_iocq_by_index(_arg: c_int);
}
extern "C" {
    pub fn aoe_end_request(: *mut aoedev, : *mut request, _arg: c_int);
}
extern "C" {
    pub fn aoe_ktstart(k: *mut ktstate) -> c_int;
}
extern "C" {
    pub fn aoe_ktstop(k: *mut ktstate);
}
extern "C" {
    pub fn aoedev_init() -> c_int;
}
extern "C" {
    pub fn aoedev_exit();
}
extern "C" {
    pub fn aoedev_downdev(d: *mut aoedev);
}
extern "C" {
    pub fn aoedev_flush(str: *const char __user, size: usize) -> c_int;
}
extern "C" {
    pub fn aoe_failbuf(: *mut aoedev, : *mut buf);
}
extern "C" {
    pub fn aoedev_put(: *mut aoedev);
}
extern "C" {
    pub fn aoenet_init() -> c_int;
}
extern "C" {
    pub fn aoenet_exit();
}
extern "C" {
    pub fn aoenet_xmit(: *mut sk_buff_head);
}
extern "C" {
    pub fn is_aoe_netif(ifp: *mut net_device) -> c_int;
}
extern "C" {
    pub fn set_aoe_iflist(str: *const char __user, size: usize) -> c_int;
}
