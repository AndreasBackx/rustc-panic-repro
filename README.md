```
➜ cargo run
   Compiling rustc-panic-repro v0.1.0 (/home/andreasbackx/dev/rustc-panic-repro)
error[E0425]: cannot find value `timezone` in this scope
 --> src/main.rs:7:5
  |
7 |             * **Timezone**: {timezone}
  |     ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `ignores_code_freeze` in this scope
 --> src/main.rs:8:4
  |
8 |             * **Ignores code freeze**: {ignores_code_freeze}
  |    ^^^^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `allowed_push_time` in this scope
 --> src/main.rs:8:51
  |
8 |               * **Ignores code freeze**: {ignores_code_freeze}
  |  ___________________________________________________^
9 | |             * **Allowed push time**: {allowed_push_time}
  | |______^ not found in this scope

error[E0425]: cannot find value `is_pushing_on_holidays` in this scope
  --> src/main.rs:9:36
   |
9  |               * **Allowed push time**: {allowed_push_time}
   |  ____________________________________^
10 | |             * **Pushes on holidays**: {is_pushing_on_holidays}
   | |_ not found in this scope

thread 'rustc' panicked at compiler/rustc_span/src/lib.rs:2027:17:
assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32
stack backtrace:
   0:     0x7fd1bc1ce6ea - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::h304520fd6a30aa07
   1:     0x7fd1bca19525 - core::fmt::write::hf5713710ce10ff22
   2:     0x7fd1bd834d91 - std::io::Write::write_fmt::hda708db57927dacf
   3:     0x7fd1bc1d0dbb - std::panicking::default_hook::{{closure}}::he1ad87607d0c11c5
   4:     0x7fd1bc1d0a2e - std::panicking::default_hook::h81c8cd2e7c59ee33
   5:     0x7fd1bb39a5d7 - std[5204e9590b4985ef]::panicking::update_hook::<alloc[fd15fd9026f491e1]::boxed::Box<rustc_driver_impl[c41f2638408ed175]::install_ice_hook::{closure#0}>>::{closure#0}
   6:     0x7fd1bc1d16d7 - std::panicking::rust_panic_with_hook::had2118629c312a4a
   7:     0x7fd1bc1d1363 - std::panicking::begin_panic_handler::{{closure}}::h7fa5985d111bafa2
   8:     0x7fd1bc1ceb99 - std::sys::backtrace::__rust_end_short_backtrace::h704d151dbefa09c5
   9:     0x7fd1bc1d1064 - rust_begin_unwind
  10:     0x7fd1b9858413 - core::panicking::panic_fmt::h3eea515d05f7a35e
  11:     0x7fd1b9b3c0ec - core::panicking::panic::h102d65dbfa674afe
  12:     0x7fd1ba08896c - <rustc_span[233999951ced9cd1]::source_map::SourceMap>::lookup_char_pos
  13:     0x7fd1bdb060c7 - <rustc_errors[98c49d204a493357]::json::DiagnosticSpan>::from_span_full::<core[d89802b8f5f07590]::iter::sources::from_fn::FromFn<<rustc_span[233999951ced9cd1]::span_encoding::Span>::macro_backtrace::{closure#0}>>
  14:     0x7fd1bdb05d67 - <rustc_errors[98c49d204a493357]::json::DiagnosticSpan>::from_span_label
  15:     0x7fd1bdb05c03 - <rustc_errors[98c49d204a493357]::json::DiagnosticSpan>::from_multispan
  16:     0x7fd1bdae4a16 - <rustc_errors[98c49d204a493357]::json::Diagnostic>::from_errors_diagnostic
  17:     0x7fd1bdae47dd - <rustc_errors[98c49d204a493357]::json::JsonEmitter as rustc_errors[98c49d204a493357]::emitter::Emitter>::emit_diagnostic
  18:     0x7fd1bdae7e01 - <rustc_errors[98c49d204a493357]::DiagCtxtInner>::emit_diagnostic::{closure#3}
  19:     0x7fd1bdaeb51d - rustc_interface[706ab71263ce060a]::callbacks::track_diagnostic::<core[d89802b8f5f07590]::option::Option<rustc_span[233999951ced9cd1]::ErrorGuaranteed>>
  20:     0x7fd1bdae998c - <rustc_errors[98c49d204a493357]::DiagCtxtInner>::emit_diagnostic
  21:     0x7fd1bdaed517 - <rustc_errors[98c49d204a493357]::DiagCtxtHandle>::emit_diagnostic
  22:     0x7fd1bdaed411 - <rustc_span[233999951ced9cd1]::ErrorGuaranteed as rustc_errors[98c49d204a493357]::diagnostic::EmissionGuarantee>::emit_producing_guarantee
  23:     0x7fd1bd71ab11 - <rustc_resolve[5d4f28296941c69b]::Resolver>::resolve_crate::{closure#0}
  24:     0x7fd1bd711cc0 - <rustc_resolve[5d4f28296941c69b]::Resolver>::resolve_crate
  25:     0x7fd1bcf4a775 - rustc_interface[706ab71263ce060a]::passes::resolver_for_lowering_raw
  26:     0x7fd1bcf49973 - rustc_query_impl[2ecbb548ea5419f8]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[2ecbb548ea5419f8]::query_impl::resolver_for_lowering_raw::dynamic_query::{closure#2}::{closure#0}, rustc_middle[c83967c7761a8780]::query::erase::Erased<[u8; 16usize]>>
  27:     0x7fd1bcf49961 - <rustc_query_impl[2ecbb548ea5419f8]::query_impl::resolver_for_lowering_raw::dynamic_query::{closure#2} as core[d89802b8f5f07590]::ops::function::FnOnce<(rustc_middle[c83967c7761a8780]::ty::context::TyCtxt, ())>>::call_once
  28:     0x7fd1bd9f4755 - rustc_query_system[842c6bba149f2c70]::query::plumbing::try_execute_query::<rustc_query_impl[2ecbb548ea5419f8]::DynamicConfig<rustc_query_system[842c6bba149f2c70]::query::caches::SingleCache<rustc_middle[c83967c7761a8780]::query::erase::Erased<[u8; 16usize]>>, false, false, false>, rustc_query_impl[2ecbb548ea5419f8]::plumbing::QueryCtxt, true>
  29:     0x7fd1bd9f428a - rustc_query_impl[2ecbb548ea5419f8]::query_impl::resolver_for_lowering_raw::get_query_incr::__rust_end_short_backtrace
  30:     0x7fd1bd785e6b - rustc_interface[706ab71263ce060a]::interface::run_compiler::<core[d89802b8f5f07590]::result::Result<(), rustc_span[233999951ced9cd1]::ErrorGuaranteed>, rustc_driver_impl[c41f2638408ed175]::run_compiler::{closure#0}>::{closure#1}
  31:     0x7fd1bd83ed16 - std[5204e9590b4985ef]::sys::backtrace::__rust_begin_short_backtrace::<rustc_interface[706ab71263ce060a]::util::run_in_thread_with_globals<rustc_interface[706ab71263ce060a]::interface::run_compiler<core[d89802b8f5f07590]::result::Result<(), rustc_span[233999951ced9cd1]::ErrorGuaranteed>, rustc_driver_impl[c41f2638408ed175]::run_compiler::{closure#0}>::{closure#1}, core[d89802b8f5f07590]::result::Result<(), rustc_span[233999951ced9cd1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[d89802b8f5f07590]::result::Result<(), rustc_span[233999951ced9cd1]::ErrorGuaranteed>>
  32:     0x7fd1bd8779b0 - <<std[5204e9590b4985ef]::thread::Builder>::spawn_unchecked_<rustc_interface[706ab71263ce060a]::util::run_in_thread_with_globals<rustc_interface[706ab71263ce060a]::interface::run_compiler<core[d89802b8f5f07590]::result::Result<(), rustc_span[233999951ced9cd1]::ErrorGuaranteed>, rustc_driver_impl[c41f2638408ed175]::run_compiler::{closure#0}>::{closure#1}, core[d89802b8f5f07590]::result::Result<(), rustc_span[233999951ced9cd1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[d89802b8f5f07590]::result::Result<(), rustc_span[233999951ced9cd1]::ErrorGuaranteed>>::{closure#1} as core[d89802b8f5f07590]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7fd1bd877d2b - std::sys::pal::unix::thread::Thread::new::thread_start::hcdbd1049068002f4
  34:     0x7fd1bedda6d7 - start_thread
  35:     0x7fd1bee5e60c - __clone3
  36:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.82.0 (f6e511eec 2024-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolver_for_lowering_raw] getting the resolver for lowering
end of query stack
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc-panic-repro` (bin "rustc-panic-repro") due to 4 previous errors
```
