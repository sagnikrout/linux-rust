//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/debug/i386/dump_psb.c
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
// dump_psb. (c) 2004, Dave Jones, Red Hat Inc.

// Macro flag: #define _GNU_SOURCE

    static long relevant;
    static const int fid_to_mult[32] = {
    110, 115, 120, 125, 50, 55, 60, 65,
    70, 75, 80, 85, 90, 95, 100, 105,
    30, 190, 40, 200, 130, 135, 140, 210,
    150, 225, 160, 165, 170, 180, -1, -1,
    };
    static const int vid_to_voltage[32] = {
    2000, 1950, 1900, 1850, 1800, 1750, 1700, 1650,
    1600, 1550, 1500, 1450, 1400, 1350, 1300, 0,
    1275, 1250, 1225, 1200, 1175, 1150, 1125, 1100,
    1075, 1050, 1024, 1000, 975, 950, 925, 0,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_header {
    pub signature: [c_char; 10],
    pub version: u_char,
    pub flags: u_char,
    pub settlingtime: u_short,
    pub res1: u_char,
    pub numpst: u_char,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pst_header {
    pub cpuid: u_int32_t,
    pub fsb: u_char,
    pub maxfid: u_char,
    pub startvid: u_char,
    pub numpstates: u_char,
    pub __packed: },
    pub fsb: static u_int,
    pub sgtc: static u_int,
    static int
    decode_pst(char *p, int npstates)
    {
    pub i: c_int,
    pub vid: int freq, fid,,
    pub {: for (i = 0; i < npstates; ++i),
    pub p++: *mut fid =,
    pub p++: *mut vid =,
    pub fsb: *mut *mut *mut freq = 100  fid_to_mult[fid],
    printf("   %2d %8dkHz  FID %02x (%2d.%01d)  VID %02x (%4dmV)\n",
    i,
    freq,
    fid, fid_to_mult[fid]/10, fid_to_mult[fid]%10,
    pub vid_to_voltage[vid]): vid,,
    }
    pub 0: return,
    }
    static
#[no_mangle]
pub unsafe extern "C" fn decode_psb(p: *mut c_char, numpst: c_int) {
    void decode_psb(char *p, int numpst)
    {
    pub i: c_int,
    pub psb: *mut psb_header,
    pub pst: *mut pst_header,
    pub p: *mut *mut psb = (struct psb_header),
    if (psb.version != 0x12)
    printf("PSB version: %hhx flags: %hhx settling time %hhuus res1 %hhx num pst %hhu\n",
    psb.version,
    psb.flags,
    psb.settlingtime,
    psb.res1,
    pub 100: *mut *mut sgtc = psb->settlingtime,
    if (sgtc < 10000)
    pub 10000: sgtc =,
    pub psb_header): *mut *mut p = ((char ) psb) + sizeof(struct,
    if (numpst < 0)
    pub psb->numpst: numpst =,
    else
    pub numpst): printf("Overriding number of pst :%d\n",,
    pub {: for (i = 0; i < numpst; i++),
    pub p: *mut *mut pst = (struct pst_header),
    if (relevant != 0) {
    if (relevant!= pst.cpuid)
    pub next_one: goto,
    }
    printf("  PST %d  cpuid %.3x fsb %hhu mfid %hhx svid %hhx numberstates %hhu\n",
    i+1,
    pst.cpuid,
    pst.fsb,
    pst.maxfid,
    pst.startvid,
    pub pst->fsb: fsb =,
    pub pst->numpstates): decode_pst(p + sizeof(struct pst_header),,
    next_one:
    pub 2*pst->numpstates: *mut p += sizeof(struct pst_header) +,
    }
    }
    static struct option info_opts[] = {
    {"numpst", no_argument, core::ptr::null_mut(), 'n'},
}

#[no_mangle]
pub unsafe extern "C" fn print_help() {
    void print_help(void)
    {
    printf ("Usage: dump_psb [options]\n");
    printf ("Options:\n");
    printf ("  -n, --numpst     Set number of PST tables to scan\n");
    printf ("  -r, --relevant   Only display PSTs relevant to cpuid N\n");
    }
    int
    main(int argc, char *argv[])
    {
    int fd;
    let mut numpst: c_int = -1;
    let mut ret: c_int = 0, cont=1;
    char *mem = core::ptr::null_mut();
    char *p;
    do {
    ret = getopt_long(argc, argv, "hr:n:", info_opts, core::ptr::null_mut());
    switch (ret){
    case '?':
    case 'h':
    print_help();
    cont = 0;
    break;
    case 'r':
    relevant = strtol(optarg, core::ptr::null_mut(), 16);
    break;
    case 'n':
    numpst = strtol(optarg, core::ptr::null_mut(), 10);
    break;
    case -1:
    cont = 0;
    break;
    }
    } while(cont);
    fd = open("/dev/mem", O_RDONLY);
    if (fd < 0) {
    printf ("Couldn't open /dev/mem. Are you root?\n");
    exit(1);
    }
    mem = mmap(mem, 0x100000 - 0xc0000, PROT_READ, MAP_SHARED, fd, 0xc0000);
    close(fd);
    for (p = mem; p - mem < LEN; p+=16) {
    if (memcmp(p, "AMDK7PNOW!", 10) == 0) {
    decode_psb(p, numpst);
    break;
    }
    }
    munmap(mem, LEN);
    return 0;
    }
