//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/hack-coff.c
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
// hack-coff.c - hack the header of an xcoff file to fill in
// a few fields needed by the Open Firmware xcoff loader on
// Power Macs but not initialized by objcopy.
//
// Copyright (C) Paul Mackerras 1997.
//

pub const AOUT_MAGIC: c_uint = 0x010b;

    + ((unsigned char *)(x))[1])

    ((unsigned char *)(x))[1] = (v) & 0xff)

    + (((unsigned char *)(x))[1] << 16) \
    + (((unsigned char *)(x))[2] << 8) \
    + ((unsigned char *)(x))[3])
    int
    main(int ac, char **av)
    {
    int fd;
    int i, nsect;
    int aoutsz;
    struct external_filehdr fhdr;
    struct aouthdr aout;
    struct external_scnhdr shdr;
    if (ac != 2) {
    fprintf(stderr, "Usage: hack-coff coff-file\n");
    exit(1);
    }
    if ((fd = open(av[1], 2)) == -1) {
    perror(av[2]);
    exit(1);
    }
    if (read(fd, &fhdr, sizeof(fhdr)) != sizeof(fhdr))
    goto readerr;
    i = get_16be(fhdr.f_magic);
    if (i != U802TOCMAGIC && i != U802WRMAGIC && i != U802ROMAGIC) {
    fprintf(stderr, "%s: not an xcoff file\n", av[1]);
    exit(1);
    }
    aoutsz = get_16be(fhdr.f_opthdr);
    if (read(fd, &aout, aoutsz) != aoutsz)
    goto readerr;
    nsect = get_16be(fhdr.f_nscns);
    for (i = 0; i < nsect; ++i) {
    if (read(fd, &shdr, sizeof(shdr)) != sizeof(shdr))
    goto readerr;
    if (strcmp(shdr.s_name, ".text") == 0) {
    put_16be(aout.o_snentry, i+1);
    put_16be(aout.o_sntext, i+1);
    } else if (strcmp(shdr.s_name, ".data") == 0) {
    put_16be(aout.o_sndata, i+1);
    } else if (strcmp(shdr.s_name, ".bss") == 0) {
    put_16be(aout.o_snbss, i+1);
    }
    }
    put_16be(aout.magic, AOUT_MAGIC);
    if (lseek(fd, (long) sizeof(struct external_filehdr), 0) == -1
    || write(fd, &aout, aoutsz) != aoutsz) {
    fprintf(stderr, "%s: write error\n", av[1]);
    exit(1);
    }
    close(fd);
    exit(0);
    readerr:
    fprintf(stderr, "%s: read error or file too short\n", av[1]);
    exit(1);
    }
