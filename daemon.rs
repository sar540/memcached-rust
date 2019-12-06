#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn setsid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
}
pub type __pid_t = libc::c_int;
/*    $Header: /cvsroot/wikipedia/willow/src/bin/willow/daemon.c,v 1.1 2005/05/02 19:15:21 kateturner Exp $    */
/*    $NetBSD: daemon.c,v 1.9 2003/08/07 16:42:46 agc Exp $    */
/*-
 * Copyright (c) 1990, 1993
 *    The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
#[no_mangle]
pub unsafe extern "C" fn daemonize(mut nochdir: libc::c_int,
                                   mut noclose: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    match fork() {
        -1 => { return -(1 as libc::c_int) }
        0 => { }
        _ => { _exit(0 as libc::c_int); }
    }
    if setsid() == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if nochdir == 0 as libc::c_int {
        if chdir(b"/\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int {
            perror(b"chdir\x00" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
    }
    if noclose == 0 as libc::c_int &&
           {
               fd =
                   open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                        0o2 as libc::c_int, 0 as libc::c_int);
               (fd) != -(1 as libc::c_int)
           } {
        if dup2(fd, 0 as libc::c_int) < 0 as libc::c_int {
            perror(b"dup2 stdin\x00" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        if dup2(fd, 1 as libc::c_int) < 0 as libc::c_int {
            perror(b"dup2 stdout\x00" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        if dup2(fd, 2 as libc::c_int) < 0 as libc::c_int {
            perror(b"dup2 stderr\x00" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        if fd > 2 as libc::c_int {
            if close(fd) < 0 as libc::c_int {
                perror(b"close\x00" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int)
            }
        }
    }
    return 0 as libc::c_int;
}
