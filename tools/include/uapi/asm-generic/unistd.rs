//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/asm-generic/unistd.h
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
// This file contains the system call numbers, based on the
// layout of the x86-64 architecture, which embeds the
// pointer to the syscall in the table.
//
// As a basic principle, no duplication of functionality
// should be added, e.g. we don't use lseek when llseek
// is present. New architectures should use this file
// and implement the less feature-full calls in user space.
//

pub const __NR_io_setup: c_int = 0;
pub const __NR_io_destroy: c_int = 1;
pub const __NR_io_submit: c_int = 2;
pub const __NR_io_cancel: c_int = 3;

pub const __NR_io_getevents: c_int = 4;

pub const __NR_setxattr: c_int = 5;
pub const __NR_lsetxattr: c_int = 6;
pub const __NR_fsetxattr: c_int = 7;
pub const __NR_getxattr: c_int = 8;
pub const __NR_lgetxattr: c_int = 9;
pub const __NR_fgetxattr: c_int = 10;
pub const __NR_listxattr: c_int = 11;
pub const __NR_llistxattr: c_int = 12;
pub const __NR_flistxattr: c_int = 13;
pub const __NR_removexattr: c_int = 14;
pub const __NR_lremovexattr: c_int = 15;
pub const __NR_fremovexattr: c_int = 16;
pub const __NR_getcwd: c_int = 17;
pub const __NR_lookup_dcookie: c_int = 18;
pub const __NR_eventfd2: c_int = 19;
pub const __NR_epoll_create1: c_int = 20;
pub const __NR_epoll_ctl: c_int = 21;
pub const __NR_epoll_pwait: c_int = 22;
pub const __NR_dup: c_int = 23;
pub const __NR_dup3: c_int = 24;
pub const __NR3264_fcntl: c_int = 25;
pub const __NR_inotify_init1: c_int = 26;
pub const __NR_inotify_add_watch: c_int = 27;
pub const __NR_inotify_rm_watch: c_int = 28;
pub const __NR_ioctl: c_int = 29;
pub const __NR_ioprio_set: c_int = 30;
pub const __NR_ioprio_get: c_int = 31;
pub const __NR_flock: c_int = 32;
pub const __NR_mknodat: c_int = 33;
pub const __NR_mkdirat: c_int = 34;
pub const __NR_unlinkat: c_int = 35;
pub const __NR_symlinkat: c_int = 36;
pub const __NR_linkat: c_int = 37;

// renameat is superseded with flags by renameat2
pub const __NR_renameat: c_int = 38;

pub const __NR_umount2: c_int = 39;
pub const __NR_mount: c_int = 40;
pub const __NR_pivot_root: c_int = 41;
pub const __NR_nfsservctl: c_int = 42;
pub const __NR3264_statfs: c_int = 43;
pub const __NR3264_fstatfs: c_int = 44;
pub const __NR3264_truncate: c_int = 45;
pub const __NR3264_ftruncate: c_int = 46;
pub const __NR_fallocate: c_int = 47;
pub const __NR_faccessat: c_int = 48;
pub const __NR_chdir: c_int = 49;
pub const __NR_fchdir: c_int = 50;
pub const __NR_chroot: c_int = 51;
pub const __NR_fchmod: c_int = 52;
pub const __NR_fchmodat: c_int = 53;
pub const __NR_fchownat: c_int = 54;
pub const __NR_fchown: c_int = 55;
pub const __NR_openat: c_int = 56;
pub const __NR_close: c_int = 57;
pub const __NR_vhangup: c_int = 58;
pub const __NR_pipe2: c_int = 59;
pub const __NR_quotactl: c_int = 60;
pub const __NR_getdents64: c_int = 61;
pub const __NR3264_lseek: c_int = 62;
pub const __NR_read: c_int = 63;
pub const __NR_write: c_int = 64;
pub const __NR_readv: c_int = 65;
pub const __NR_writev: c_int = 66;
pub const __NR_pread64: c_int = 67;
pub const __NR_pwrite64: c_int = 68;
pub const __NR_preadv: c_int = 69;
pub const __NR_pwritev: c_int = 70;
pub const __NR3264_sendfile: c_int = 71;

pub const __NR_pselect6: c_int = 72;
pub const __NR_ppoll: c_int = 73;

pub const __NR_signalfd4: c_int = 74;
pub const __NR_vmsplice: c_int = 75;
pub const __NR_splice: c_int = 76;
pub const __NR_tee: c_int = 77;
pub const __NR_readlinkat: c_int = 78;

pub const __NR3264_fstatat: c_int = 79;
pub const __NR3264_fstat: c_int = 80;

pub const __NR_sync: c_int = 81;
pub const __NR_fsync: c_int = 82;
pub const __NR_fdatasync: c_int = 83;

pub const __NR_sync_file_range2: c_int = 84;

pub const __NR_sync_file_range: c_int = 84;

pub const __NR_timerfd_create: c_int = 85;

pub const __NR_timerfd_settime: c_int = 86;
pub const __NR_timerfd_gettime: c_int = 87;

pub const __NR_utimensat: c_int = 88;

pub const __NR_acct: c_int = 89;
pub const __NR_capget: c_int = 90;
pub const __NR_capset: c_int = 91;
pub const __NR_personality: c_int = 92;
pub const __NR_exit: c_int = 93;
pub const __NR_exit_group: c_int = 94;
pub const __NR_waitid: c_int = 95;
pub const __NR_set_tid_address: c_int = 96;
pub const __NR_unshare: c_int = 97;

pub const __NR_futex: c_int = 98;

pub const __NR_set_robust_list: c_int = 99;
pub const __NR_get_robust_list: c_int = 100;

pub const __NR_nanosleep: c_int = 101;

pub const __NR_getitimer: c_int = 102;
pub const __NR_setitimer: c_int = 103;
pub const __NR_kexec_load: c_int = 104;
pub const __NR_init_module: c_int = 105;
pub const __NR_delete_module: c_int = 106;
pub const __NR_timer_create: c_int = 107;

pub const __NR_timer_gettime: c_int = 108;

pub const __NR_timer_getoverrun: c_int = 109;

pub const __NR_timer_settime: c_int = 110;

pub const __NR_timer_delete: c_int = 111;

pub const __NR_clock_settime: c_int = 112;
pub const __NR_clock_gettime: c_int = 113;
pub const __NR_clock_getres: c_int = 114;
pub const __NR_clock_nanosleep: c_int = 115;

pub const __NR_syslog: c_int = 116;
pub const __NR_ptrace: c_int = 117;
pub const __NR_sched_setparam: c_int = 118;
pub const __NR_sched_setscheduler: c_int = 119;
pub const __NR_sched_getscheduler: c_int = 120;
pub const __NR_sched_getparam: c_int = 121;
pub const __NR_sched_setaffinity: c_int = 122;
pub const __NR_sched_getaffinity: c_int = 123;
pub const __NR_sched_yield: c_int = 124;
pub const __NR_sched_get_priority_max: c_int = 125;
pub const __NR_sched_get_priority_min: c_int = 126;

pub const __NR_sched_rr_get_interval: c_int = 127;

pub const __NR_restart_syscall: c_int = 128;
pub const __NR_kill: c_int = 129;
pub const __NR_tkill: c_int = 130;
pub const __NR_tgkill: c_int = 131;
pub const __NR_sigaltstack: c_int = 132;
pub const __NR_rt_sigsuspend: c_int = 133;
pub const __NR_rt_sigaction: c_int = 134;
pub const __NR_rt_sigprocmask: c_int = 135;
pub const __NR_rt_sigpending: c_int = 136;

pub const __NR_rt_sigtimedwait: c_int = 137;

pub const __NR_rt_sigqueueinfo: c_int = 138;
pub const __NR_rt_sigreturn: c_int = 139;
pub const __NR_setpriority: c_int = 140;
pub const __NR_getpriority: c_int = 141;
pub const __NR_reboot: c_int = 142;
pub const __NR_setregid: c_int = 143;
pub const __NR_setgid: c_int = 144;
pub const __NR_setreuid: c_int = 145;
pub const __NR_setuid: c_int = 146;
pub const __NR_setresuid: c_int = 147;
pub const __NR_getresuid: c_int = 148;
pub const __NR_setresgid: c_int = 149;
pub const __NR_getresgid: c_int = 150;
pub const __NR_setfsuid: c_int = 151;
pub const __NR_setfsgid: c_int = 152;
pub const __NR_times: c_int = 153;
pub const __NR_setpgid: c_int = 154;
pub const __NR_getpgid: c_int = 155;
pub const __NR_getsid: c_int = 156;
pub const __NR_setsid: c_int = 157;
pub const __NR_getgroups: c_int = 158;
pub const __NR_setgroups: c_int = 159;
pub const __NR_uname: c_int = 160;
pub const __NR_sethostname: c_int = 161;
pub const __NR_setdomainname: c_int = 162;

// getrlimit and setrlimit are superseded with prlimit64
pub const __NR_getrlimit: c_int = 163;
pub const __NR_setrlimit: c_int = 164;

pub const __NR_getrusage: c_int = 165;
pub const __NR_umask: c_int = 166;
pub const __NR_prctl: c_int = 167;
pub const __NR_getcpu: c_int = 168;

pub const __NR_gettimeofday: c_int = 169;
pub const __NR_settimeofday: c_int = 170;
pub const __NR_adjtimex: c_int = 171;

pub const __NR_getpid: c_int = 172;
pub const __NR_getppid: c_int = 173;
pub const __NR_getuid: c_int = 174;
pub const __NR_geteuid: c_int = 175;
pub const __NR_getgid: c_int = 176;
pub const __NR_getegid: c_int = 177;
pub const __NR_gettid: c_int = 178;
pub const __NR_sysinfo: c_int = 179;
pub const __NR_mq_open: c_int = 180;
pub const __NR_mq_unlink: c_int = 181;

pub const __NR_mq_timedsend: c_int = 182;
pub const __NR_mq_timedreceive: c_int = 183;

pub const __NR_mq_notify: c_int = 184;
pub const __NR_mq_getsetattr: c_int = 185;
pub const __NR_msgget: c_int = 186;
pub const __NR_msgctl: c_int = 187;
pub const __NR_msgrcv: c_int = 188;
pub const __NR_msgsnd: c_int = 189;
pub const __NR_semget: c_int = 190;
pub const __NR_semctl: c_int = 191;

pub const __NR_semtimedop: c_int = 192;

pub const __NR_semop: c_int = 193;
pub const __NR_shmget: c_int = 194;
pub const __NR_shmctl: c_int = 195;
pub const __NR_shmat: c_int = 196;
pub const __NR_shmdt: c_int = 197;
pub const __NR_socket: c_int = 198;
pub const __NR_socketpair: c_int = 199;
pub const __NR_bind: c_int = 200;
pub const __NR_listen: c_int = 201;
pub const __NR_accept: c_int = 202;
pub const __NR_connect: c_int = 203;
pub const __NR_getsockname: c_int = 204;
pub const __NR_getpeername: c_int = 205;
pub const __NR_sendto: c_int = 206;
pub const __NR_recvfrom: c_int = 207;
pub const __NR_setsockopt: c_int = 208;
pub const __NR_getsockopt: c_int = 209;
pub const __NR_shutdown: c_int = 210;
pub const __NR_sendmsg: c_int = 211;
pub const __NR_recvmsg: c_int = 212;
pub const __NR_readahead: c_int = 213;
pub const __NR_brk: c_int = 214;
pub const __NR_munmap: c_int = 215;
pub const __NR_mremap: c_int = 216;
pub const __NR_add_key: c_int = 217;
pub const __NR_request_key: c_int = 218;
pub const __NR_keyctl: c_int = 219;
pub const __NR_clone: c_int = 220;
pub const __NR_execve: c_int = 221;
pub const __NR3264_mmap: c_int = 222;
pub const __NR3264_fadvise64: c_int = 223;
// CONFIG_MMU only
pub const __NR_swapon: c_int = 224;
pub const __NR_swapoff: c_int = 225;
pub const __NR_mprotect: c_int = 226;
pub const __NR_msync: c_int = 227;
pub const __NR_mlock: c_int = 228;
pub const __NR_munlock: c_int = 229;
pub const __NR_mlockall: c_int = 230;
pub const __NR_munlockall: c_int = 231;
pub const __NR_mincore: c_int = 232;
pub const __NR_madvise: c_int = 233;
pub const __NR_remap_file_pages: c_int = 234;
pub const __NR_mbind: c_int = 235;
pub const __NR_get_mempolicy: c_int = 236;
pub const __NR_set_mempolicy: c_int = 237;
pub const __NR_migrate_pages: c_int = 238;
pub const __NR_move_pages: c_int = 239;

pub const __NR_rt_tgsigqueueinfo: c_int = 240;
pub const __NR_perf_event_open: c_int = 241;
pub const __NR_accept4: c_int = 242;

pub const __NR_recvmmsg: c_int = 243;

//
// Architectures may provide up to 16 syscalls of their own
// starting with this value.
//
pub const __NR_arch_specific_syscall: c_int = 244;

pub const __NR_wait4: c_int = 260;

pub const __NR_prlimit64: c_int = 261;
pub const __NR_fanotify_init: c_int = 262;
pub const __NR_fanotify_mark: c_int = 263;
pub const __NR_name_to_handle_at: c_int = 264;
pub const __NR_open_by_handle_at: c_int = 265;

pub const __NR_clock_adjtime: c_int = 266;

pub const __NR_syncfs: c_int = 267;
pub const __NR_setns: c_int = 268;
pub const __NR_sendmmsg: c_int = 269;
pub const __NR_process_vm_readv: c_int = 270;
pub const __NR_process_vm_writev: c_int = 271;
pub const __NR_kcmp: c_int = 272;
pub const __NR_finit_module: c_int = 273;
pub const __NR_sched_setattr: c_int = 274;
pub const __NR_sched_getattr: c_int = 275;
pub const __NR_renameat2: c_int = 276;
pub const __NR_seccomp: c_int = 277;
pub const __NR_getrandom: c_int = 278;
pub const __NR_memfd_create: c_int = 279;
pub const __NR_bpf: c_int = 280;
pub const __NR_execveat: c_int = 281;
pub const __NR_userfaultfd: c_int = 282;
pub const __NR_membarrier: c_int = 283;
pub const __NR_mlock2: c_int = 284;
pub const __NR_copy_file_range: c_int = 285;
pub const __NR_preadv2: c_int = 286;
pub const __NR_pwritev2: c_int = 287;
pub const __NR_pkey_mprotect: c_int = 288;
pub const __NR_pkey_alloc: c_int = 289;
pub const __NR_pkey_free: c_int = 290;
pub const __NR_statx: c_int = 291;

pub const __NR_io_pgetevents: c_int = 292;

pub const __NR_rseq: c_int = 293;
pub const __NR_kexec_file_load: c_int = 294;
// 295 through 402 are unassigned to sync up with generic numbers, don't use

pub const __NR_clock_gettime64: c_int = 403;
pub const __NR_clock_settime64: c_int = 404;
pub const __NR_clock_adjtime64: c_int = 405;
pub const __NR_clock_getres_time64: c_int = 406;
pub const __NR_clock_nanosleep_time64: c_int = 407;
pub const __NR_timer_gettime64: c_int = 408;
pub const __NR_timer_settime64: c_int = 409;
pub const __NR_timerfd_gettime64: c_int = 410;
pub const __NR_timerfd_settime64: c_int = 411;
pub const __NR_utimensat_time64: c_int = 412;
pub const __NR_pselect6_time64: c_int = 413;
pub const __NR_ppoll_time64: c_int = 414;
pub const __NR_io_pgetevents_time64: c_int = 416;
pub const __NR_recvmmsg_time64: c_int = 417;
pub const __NR_mq_timedsend_time64: c_int = 418;
pub const __NR_mq_timedreceive_time64: c_int = 419;
pub const __NR_semtimedop_time64: c_int = 420;
pub const __NR_rt_sigtimedwait_time64: c_int = 421;
pub const __NR_futex_time64: c_int = 422;
pub const __NR_sched_rr_get_interval_time64: c_int = 423;

pub const __NR_pidfd_send_signal: c_int = 424;
pub const __NR_io_uring_setup: c_int = 425;
pub const __NR_io_uring_enter: c_int = 426;
pub const __NR_io_uring_register: c_int = 427;
pub const __NR_open_tree: c_int = 428;
pub const __NR_move_mount: c_int = 429;
pub const __NR_fsopen: c_int = 430;
pub const __NR_fsconfig: c_int = 431;
pub const __NR_fsmount: c_int = 432;
pub const __NR_fspick: c_int = 433;
pub const __NR_pidfd_open: c_int = 434;
pub const __NR_clone3: c_int = 435;
pub const __NR_close_range: c_int = 436;
pub const __NR_openat2: c_int = 437;
pub const __NR_pidfd_getfd: c_int = 438;
pub const __NR_faccessat2: c_int = 439;
pub const __NR_process_madvise: c_int = 440;
pub const __NR_epoll_pwait2: c_int = 441;
pub const __NR_mount_setattr: c_int = 442;
pub const __NR_quotactl_fd: c_int = 443;
pub const __NR_landlock_create_ruleset: c_int = 444;
pub const __NR_landlock_add_rule: c_int = 445;
pub const __NR_landlock_restrict_self: c_int = 446;

pub const __NR_memfd_secret: c_int = 447;

pub const __NR_process_mrelease: c_int = 448;
pub const __NR_futex_waitv: c_int = 449;
pub const __NR_set_mempolicy_home_node: c_int = 450;
pub const __NR_cachestat: c_int = 451;
pub const __NR_fchmodat2: c_int = 452;
pub const __NR_map_shadow_stack: c_int = 453;
pub const __NR_futex_wake: c_int = 454;
pub const __NR_futex_wait: c_int = 455;
pub const __NR_futex_requeue: c_int = 456;
pub const __NR_statmount: c_int = 457;
pub const __NR_listmount: c_int = 458;
pub const __NR_lsm_get_self_attr: c_int = 459;
pub const __NR_lsm_set_self_attr: c_int = 460;
pub const __NR_lsm_list_modules: c_int = 461;
pub const __NR_mseal: c_int = 462;
pub const __NR_setxattrat: c_int = 463;
pub const __NR_getxattrat: c_int = 464;
pub const __NR_listxattrat: c_int = 465;
pub const __NR_removexattrat: c_int = 466;
pub const __NR_open_tree_attr: c_int = 467;
// fs/inode.c
pub const __NR_file_getattr: c_int = 468;
pub const __NR_file_setattr: c_int = 469;
pub const __NR_listns: c_int = 470;
pub const __NR_rseq_slice_yield: c_int = 471;
// fs/open.c
pub const __NR_fchroot: c_int = 472;

pub const __NR_syscalls: c_int = 473;
//
// 32 bit systems traditionally used different
// syscalls for off_t and loff_t arguments, while
// 64 bit systems only need the off_t version.
// For new 32 bit platforms, there is no need to
// implement the old 32 bit off_t syscalls, so
// they take different names.
// Here we map the numbers so that both versions
// use the same syscall table layout.
//

