// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io;
use sys_common::backtrace::Frame;

pub use sys_common::gnu::libbacktrace::{foreach_symbol_fileline, resolve_symname};
pub struct BacktraceContext;

#[inline(never)]
pub fn unwind_backtrace(_frames: &mut [Frame])
    -> io::Result<(usize, BacktraceContext)>
{
    Ok((0, BacktraceContext))
}

pub mod gnu {
    use io;
    use fs;
    use libc::c_char;

    pub fn get_executable_filename() -> io::Result<(Vec<c_char>, fs::File)> {
        Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
    }
}
