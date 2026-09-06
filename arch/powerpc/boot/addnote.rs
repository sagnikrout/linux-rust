//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/addnote.c
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
// Program to hack in a PT_NOTE program header entry in an ELF file.
// This is needed for OF on RS/6000s to load an image correctly.
// Note that OF needs a program header entry for the note, not an
// ELF section.
//
// Copyright 2000 Paul Mackerras.
//
// Adapted for 64 bit little endian images by Andrew Tauferner.
//
// Usage: addnote zImage
//

// CHRP note section
    static const char arch[] = "PowerPC";
pub const N_DESCR: c_int = 6;
    unsigned int descr[N_DESCR] = {
    0xffffffff,		/* real-mode = true */
    0x02000000,		/* real-base, i.e. where we expect OF to be */
    0xffffffff,		/* real-size */
    0xffffffff,		/* virt-base */
    0xffffffff,		/* virt-size */
    0x4000,			/* load-base */
    };
// RPA note section
    static const char rpaname[] = "IBM,RPA-Client-Config";
//
// Note: setting ignore_my_client_config *should* mean that OF ignores
// all the other fields, but there is a firmware bug which means that
// it looks at the splpar field at least.  So these values need to be
// reasonable.
//
pub const N_RPA_DESCR: c_int = 8;
    unsigned int rpanote[N_RPA_DESCR] = {
    0,			/* lparaffinity */
    64,			/* min_rmo_size */
    0,			/* min_rmo_percent */
    40,			/* max_pft_size */
    1,			/* splpar */
    -1,			/* min_load */
    0,			/* new_mem_def */
    1,			/* ignore_my_client_config */
    };

    unsigned char buf[1024];
pub const ELFDATA2LSB: c_int = 1;
pub const ELFDATA2MSB: c_int = 2;
    let mut e_data: static int = ELFDATA2MSB;
pub const ELFCLASS32: c_int = 1;
pub const ELFCLASS64: c_int = 2;
    let mut e_class: static int = ELFCLASS32;

    ((unsigned long long)GET_32BE((off)+4ULL)))

    buf[(off) + 1] = (v) & 0xff)

    PUT_32BE((off) + 4, (unsigned long long)(v))))

    (((unsigned long long)GET_32LE((off)+4ULL)) << 32ULL))

    buf[(off) + 1] = ((v) >> 8) & 0xff)

    PUT_32LE((off) + 4, (unsigned long long)(v) >> 32L))

    PUT_16LE(off, v))

    PUT_32LE(off, v))

    PUT_64LE(off, v))
// Structure of an ELF file

pub const EI_CLASS: c_int = 4;
pub const EI_DATA: c_int = 5;

    unsigned char elf_magic[4] = { 0x7f, 'E', 'L', 'F' };
    int
    main(int ac, char **av)
    {
    int fd, n, i;
    unsigned long ph, ps, np;
    long nnote, nnote2, ns;
    if (ac != 2) {
    fprintf(stderr, "Usage: %s elf-file\n", av[0]);
    exit(1);
    }
    fd = open(av[1], O_RDWR);
    if (fd < 0) {
    perror(av[1]);
    exit(1);
    }
    nnote = 12 + ROUNDUP(strlen(arch) + 1) + sizeof(descr);
    nnote2 = 12 + ROUNDUP(strlen(rpaname) + 1) + sizeof(rpanote);
    n = read(fd, buf, sizeof(buf));
    if (n < 0) {
    perror("read");
    exit(1);
    }
    if (memcmp(&buf[E_IDENT+EI_MAGIC], elf_magic, 4) != 0)
    goto notelf;
    e_class = buf[E_IDENT+EI_CLASS];
    if (e_class != ELFCLASS32 && e_class != ELFCLASS64)
    goto notelf;
    e_data = buf[E_IDENT+EI_DATA];
    if (e_data != ELFDATA2MSB && e_data != ELFDATA2LSB)
    goto notelf;
    if (n < E_HSIZE)
    goto notelf;
    ph = (e_class == ELFCLASS32 ? GET_32(E_PHOFF) : GET_64(E_PHOFF));
    ps = GET_16(E_PHENTSIZE);
    np = GET_16(E_PHNUM);
    if (ph < E_HSIZE || ps < PH_HSIZE || np < 1)
    goto notelf;
    if (ph + (np + 2) * ps + nnote + nnote2 > n)
    goto nospace;
    for (i = 0; i < np; ++i) {
    if (GET_32(ph + PH_TYPE) == PT_NOTE) {
    fprintf(stderr, "%s already has a note entry\n",
    av[1]);
    exit(0);
    }
    ph += ps;
    }
// XXX check that the area we want to use is all zeroes
    for (i = 0; i < 2 * ps + nnote + nnote2; ++i)
    if (buf[ph + i] != 0)
    goto nospace;
// fill in the program header entry
    ns = ph + 2 * ps;
    PUT_32(ph + PH_TYPE, PT_NOTE);
    if (e_class == ELFCLASS32)
    PUT_32(ph + PH_OFFSET, ns);
    else
    PUT_64(ph + PH_OFFSET, ns);
    if (e_class == ELFCLASS32)
    PUT_32(ph + PH_FILESZ, nnote);
    else
    PUT_64(ph + PH_FILESZ, nnote);
// fill in the note area we point to
// XXX we should probably make this a proper section
    PUT_32(ns, strlen(arch) + 1);
    PUT_32(ns + 4, N_DESCR * 4);
    PUT_32(ns + 8, 0x1275);
    strcpy((char *) &buf[ns + 12], arch);
    ns += 12 + strlen(arch) + 1;
    for (i = 0; i < N_DESCR; ++i, ns += 4)
    PUT_32BE(ns, descr[i]);
// fill in the second program header entry and the RPA note area
    ph += ps;
    PUT_32(ph + PH_TYPE, PT_NOTE);
    if (e_class == ELFCLASS32)
    PUT_32(ph + PH_OFFSET, ns);
    else
    PUT_64(ph + PH_OFFSET, ns);
    if (e_class == ELFCLASS32)
    PUT_32(ph + PH_FILESZ, nnote);
    else
    PUT_64(ph + PH_FILESZ, nnote2);
// fill in the note area we point to
    PUT_32(ns, strlen(rpaname) + 1);
    PUT_32(ns + 4, sizeof(rpanote));
    PUT_32(ns + 8, 0x12759999);
    strcpy((char *) &buf[ns + 12], rpaname);
    ns += 12 + ROUNDUP(strlen(rpaname) + 1);
    for (i = 0; i < N_RPA_DESCR; ++i, ns += 4)
    PUT_32BE(ns, rpanote[i]);
// Update the number of program headers
    PUT_16(E_PHNUM, np + 2);
// write back
    i = lseek(fd, (long) 0, SEEK_SET);
    if (i < 0) {
    perror("lseek");
    exit(1);
    }
    i = write(fd, buf, n);
    if (i < 0) {
    perror("write");
    exit(1);
    }
    if (i < n) {
    fprintf(stderr, "%s: write truncated\n", av[1]);
    exit(1);
    }
    exit(0);
    notelf:
    fprintf(stderr, "%s does not appear to be an ELF file\n", av[1]);
    exit(1);
    nospace:
    fprintf(stderr, "sorry, I can't find space in %s to put the note\n",
    av[1]);
    exit(1);
    }
