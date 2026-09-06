//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/capability.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// This is <linux/capability.h>
//
// Andrew G. Morgan <morgan@kernel.org>
// Alexander Kjeldaas <astor@guardian.no>
// with help from Aleph1, Roland Buresund and Andrew Main.
//
// See here for the libcap2 library (compliant with Section 25 of
// the withdrawn POSIX 1003.1e Draft 17):
//
// https://www.kernel.org/pub/linux/libs/security/linux-privs/libcap2
//

// User-level do most of the mapping between kernel and user
// Note, cap_t, is defined by POSIX (draft) to be an "opaque" pointer to
pub const _LINUX_CAPABILITY_VERSION_1: c_uint = 0x19980330;
pub const _LINUX_CAPABILITY_U32S_1: c_int = 1;
pub const _LINUX_CAPABILITY_VERSION_2: c_uint = 0x20071026  /* deprecated - use v3 */;
pub const _LINUX_CAPABILITY_U32S_2: c_int = 2;
pub const _LINUX_CAPABILITY_VERSION_3: c_uint = 0x20080522;
pub const _LINUX_CAPABILITY_U32S_3: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}

pub const VFS_CAP_REVISION_MASK: c_uint = 0xFF000000;
pub const VFS_CAP_REVISION_SHIFT: c_int = 24;

pub const VFS_CAP_FLAGS_EFFECTIVE: c_uint = 0x000001;
pub const VFS_CAP_REVISION_1: c_uint = 0x01000000;
pub const VFS_CAP_U32_1: c_int = 1;

pub const VFS_CAP_REVISION_2: c_uint = 0x02000000;
pub const VFS_CAP_U32_2: c_int = 2;

pub const VFS_CAP_REVISION_3: c_uint = 0x03000000;
pub const VFS_CAP_U32_3: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfs_cap_data {
    pub /: *mut *mut __le32 magic_etc; / Little endian,
    pub /: *mut *mut __le32 permitted; / Little endian,
    pub /: *mut *mut __le32 inheritable; / Little endian,
    pub data: [}; VFS_CAP_U32],
}

//
// same as vfs_cap_data but with a rootid at the end
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfs_ns_cap_data {
    pub magic_etc: __le32,
    pub /: *mut *mut __le32 permitted; / Little endian,
    pub /: *mut *mut __le32 inheritable; / Little endian,
    pub data: [}; VFS_CAP_U32],
    pub rootid: __le32,
}

//
// Backwardly compatible definition for source code - trapped in a
// 32-bit world. If you find you need this, please consider using
// libcap to untrap yourself...
//

//
// POSIX-draft defined capabilities.
//
// In a system with the [_POSIX_CHOWN_RESTRICTED] option defined, this
pub const CAP_CHOWN: c_int = 0;
// Override all DAC access, including ACL execute access if
pub const CAP_DAC_OVERRIDE: c_int = 1;
// Overrides all DAC restrictions regarding read and search on files
pub const CAP_DAC_READ_SEARCH: c_int = 2;
// Overrides all restrictions about allowed operations on files, where
pub const CAP_FOWNER: c_int = 3;
// Overrides the following restrictions that the effective user ID
pub const CAP_FSETID: c_int = 4;
// Overrides the restriction that the real or effective user ID of a
pub const CAP_KILL: c_int = 5;
// Allows setgid(2) manipulation
// Allows setgroups(2)
// Allows forged gids on socket credentials passing.
pub const CAP_SETGID: c_int = 6;
// Allows set*uid(2) manipulation (including fsuid).
// Allows forged pids on socket credentials passing.
pub const CAP_SETUID: c_int = 7;
//
// Linux-specific capabilities
//
// Without VFS support for capabilities:
// Transfer any capability in your permitted set to any pid,
// remove any capability in your permitted set from any pid
// With VFS support for capabilities (neither of above, but)
// Add any capability from current's capability bounding set
// to the current process' inheritable set
// Allow taking bits out of capability bounding set
// Allow modification of the securebits for a process
//
pub const CAP_SETPCAP: c_int = 8;
// Allow modification of S_IMMUTABLE and S_APPEND file attributes
pub const CAP_LINUX_IMMUTABLE: c_int = 9;
// Allows binding to TCP/UDP sockets below 1024
// Allows binding to ATM VCIs below 32
pub const CAP_NET_BIND_SERVICE: c_int = 10;
// Allow broadcasting, listen to multicast
pub const CAP_NET_BROADCAST: c_int = 11;
// Allow interface configuration
// Allow administration of IP firewall, masquerading and accounting
// Allow setting debug option on sockets
// Allow modification of routing tables
// Allow setting arbitrary process / process group ownership on
// Allow binding to any address for transparent proxying (also via NET_RAW)
// Allow setting TOS (type of service)
// Allow setting promiscuous mode
// Allow clearing driver statistics
// Allow multicasting
// Allow read/write of device-specific registers
// Allow activation of ATM control sockets
pub const CAP_NET_ADMIN: c_int = 12;
// Allow use of RAW sockets
// Allow use of PACKET sockets
// Allow binding to any address for transparent proxying (also via NET_ADMIN)
pub const CAP_NET_RAW: c_int = 13;
// Allow locking of shared memory segments
// Allow mlock and mlockall (which doesn't really have anything to do
pub const CAP_IPC_LOCK: c_int = 14;
// Override IPC ownership checks
pub const CAP_IPC_OWNER: c_int = 15;
// Insert and remove kernel modules - modify kernel without limit
pub const CAP_SYS_MODULE: c_int = 16;
// Allow ioperm/iopl access
// Allow sending USB messages to any device via /dev/bus/usb
pub const CAP_SYS_RAWIO: c_int = 17;
// Allow use of chroot()
pub const CAP_SYS_CHROOT: c_int = 18;
// Allow ptrace() of any process
pub const CAP_SYS_PTRACE: c_int = 19;
// Allow configuration of process accounting
pub const CAP_SYS_PACCT: c_int = 20;
// Allow configuration of the secure attention key
// Allow administration of the random device
// Allow examination and configuration of disk quotas
// Allow setting the domainname
// Allow setting the hostname
// Allow mount() and umount(), setting up new smb connection
// Allow some autofs root ioctls
// Allow nfsservctl
// Allow VM86_REQUEST_IRQ
// Allow to read/write pci config on alpha
// Allow irix_prctl on mips (setstacksize)
// Allow flushing all cache on m68k (sys_cacheflush)
// Allow removing semaphores
// Used instead of CAP_CHOWN to "chown" IPC message queues, semaphores
// Allow locking/unlocking of shared memory segment
// Allow turning swap on/off
// Allow forged pids on socket credentials passing
// Allow setting readahead and flushing buffers on block devices
// Allow setting geometry in floppy driver
// Allow turning DMA on/off in xd driver
// Allow administration of md devices (mostly the above, but some
// Allow tuning the ide driver
// Allow access to the nvram device
// Allow administration of apm_bios, serial and bttv (TV) device
// Allow manufacturer commands in isdn CAPI support driver
// Allow reading non-standardized portions of pci configuration space
// Allow DDI debug ioctl on sbpcd driver
// Allow setting up serial ports
// Allow sending raw qic-117 commands
// Allow enabling/disabling tagged queuing on SCSI controllers and sending
// Allow setting encryption key on loopback filesystem
// Allow setting zone reclaim policy
// Allow everything under CAP_BPF and CAP_PERFMON for backward compatibility
// Allow setting hardware protection emergency action
pub const CAP_SYS_ADMIN: c_int = 21;
// Allow use of reboot()
pub const CAP_SYS_BOOT: c_int = 22;
// Allow raising priority and setting priority on other (different
// Allow use of FIFO and round-robin (realtime) scheduling on own
// Allow setting cpu affinity on other processes
// Allow setting realtime ioprio class
// Allow setting ioprio class on other processes
pub const CAP_SYS_NICE: c_int = 23;
// Override resource limits. Set resource limits.
// Override quota limits.
// Override reserved space on ext2 filesystem
// Modify data journaling mode on ext3 filesystem (uses journaling
// NOTE: ext2 honors fsuid when checking for resource overrides, so
// Override size restrictions on IPC message queues
// Allow more than 64hz interrupts from the real-time clock
// Override max number of consoles on console allocation
// Override max number of keymaps
// Control memory reclaim behavior
pub const CAP_SYS_RESOURCE: c_int = 24;
// Allow manipulation of system clock
// Allow irix_stime on mips
// Allow setting the real-time clock
pub const CAP_SYS_TIME: c_int = 25;
// Allow configuration of tty devices
// Allow vhangup() of tty
pub const CAP_SYS_TTY_CONFIG: c_int = 26;
// Allow the privileged aspects of mknod()
pub const CAP_MKNOD: c_int = 27;
// Allow taking of leases on files
pub const CAP_LEASE: c_int = 28;
// Allow writing the audit log via unicast netlink socket
pub const CAP_AUDIT_WRITE: c_int = 29;
// Allow configuration of audit via unicast netlink socket
pub const CAP_AUDIT_CONTROL: c_int = 30;
// Set or remove capabilities on files.
pub const CAP_SETFCAP: c_int = 31;
// Override MAC access.
pub const CAP_MAC_OVERRIDE: c_int = 32;
// Allow MAC configuration or state changes.
pub const CAP_MAC_ADMIN: c_int = 33;
// Allow configuring the kernel's syslog (printk behaviour)
pub const CAP_SYSLOG: c_int = 34;
// Allow triggering something that will wake the system
pub const CAP_WAKE_ALARM: c_int = 35;
// Allow preventing system suspends
pub const CAP_BLOCK_SUSPEND: c_int = 36;
// Allow reading the audit log via multicast netlink socket
pub const CAP_AUDIT_READ: c_int = 37;
//
// Allow system performance and observability privileged operations
// using perf_events, i915_perf and other kernel subsystems
//
pub const CAP_PERFMON: c_int = 38;
//
// CAP_BPF allows the following BPF operations:
// - Creating all types of BPF maps
// - Advanced verifier features
// - Indirect variable access
// - Bounded loops
// - BPF to BPF function calls
// - Scalar precision tracking
// - Larger complexity limits
// - Dead code elimination
// - And potentially other features
// - Loading BPF Type Format (BTF) data
// - Retrieve xlated and JITed code of BPF programs
// - Use bpf_spin_lock() helper
//
// CAP_PERFMON relaxes the verifier checks further:
// - BPF progs can use of pointer-to-integer conversions
// - speculation attack hardening measures are bypassed
// - bpf_probe_read to read arbitrary kernel memory is allowed
// - bpf_trace_printk to print kernel memory is allowed
//
// CAP_SYS_ADMIN is required to use bpf_probe_write_user.
//
// CAP_SYS_ADMIN is required to iterate system wide loaded
// programs, maps, links, BTFs and convert their IDs to file descriptors.
//
// CAP_PERFMON and CAP_BPF are required to load tracing programs.
// CAP_NET_ADMIN and CAP_BPF are required to load networking programs.
//
pub const CAP_BPF: c_int = 39;
// Allow checkpoint/restore related operations
// Allow PID selection during clone3()
// Allow writing to ns_last_pid
pub const CAP_CHECKPOINT_RESTORE: c_int = 40;

//
// Bit location of each capability (used by user-space library and kernel)
//

