# Producing a rustc panic with a minimal dropshot example

I managed to make a specific version of rustc panic by doing something with dropshot that should've been a compiler error.

Specifically, I was doing something like this in an endpoint (see main.rs for a full example):
```rust
    let server_context = rqctx.context(); // returns a &Context
    server_context.field.test_mut(); // test_mut() requires a &mut
```

This should've been a compiler error (and was on other versions of rust), but instead produced a panic.

I've only found that this panic happens on the nightly-2021-11-24 version, which is what propolis uses.

## Reproducing

```bash
$ rustup default nightly-2021-11-24
$ cargo build
```

The full stack trace is:
```
$ RUST_BACKTRACE=full cargo build
   Compiling dropshot-rustc-panic v0.1.0 (/Users/jordanhendricks/src/play/dropshot-rustc-panic)
thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 2', compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs:450:53
stack backtrace:
   0:        0x1084419c8 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h61a7d3e4c5d884f5
   1:        0x10848a980 - core::fmt::write::h15b38023853d48c8
   2:        0x1084338a0 - std::io::Write::write_fmt::h8d01a6f9467a53cf
   3:        0x1084447cc - std::panicking::default_hook::{{closure}}::h202969861cea2a40
   4:        0x1084443ac - std::panicking::default_hook::h6075a10f4c58af64
   5:        0x100da23c4 - rustc_driver[35b3b9d305ec1a86]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x108444fa0 - std::panicking::rust_panic_with_hook::h0d2ad4d26e65e971
   7:        0x108444ac0 - std::panicking::begin_panic_handler::{{closure}}::h81487f6587d82c75
   8:        0x108441e94 - std::sys_common::backtrace::__rust_end_short_backtrace::hc543c9309bc89e91
   9:        0x108444a28 - _rust_begin_unwind
  10:        0x1084ba170 - core::panicking::panic_fmt::h61e5a76b978edc22
  11:        0x1084ba144 - core::panicking::panic_bounds_check::hbcdb5a8d91c8ce55
  12:        0x1036a41e4 - <rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>::report_mutability_error
  13:        0x1036ab930 - <rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>::access_place
  14:        0x1036aa2b8 - <rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt as rustc_mir_dataflow[2505fbb23d1ca289]::framework::visitor::ResultsVisitor>::visit_statement_before_primary_effect
  15:        0x10362c8c8 - <rustc_mir_dataflow[2505fbb23d1ca289]::framework::direction::Forward as rustc_mir_dataflow[2505fbb23d1ca289]::framework::direction::Direction>::visit_results_in_block::<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::MovePathIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::InitIndex>>, rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_borrowck[6143a6a723f63a46]::dataflow::Borrows>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::EverInitializedPlaces>>, rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>
  16:        0x1035de068 - rustc_mir_dataflow[2505fbb23d1ca289]::framework::visitor::visit_results::<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::MovePathIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::InitIndex>>, rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_borrowck[6143a6a723f63a46]::dataflow::Borrows>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::EverInitializedPlaces>>, core[489ab2277f19021d]::iter::adapters::map::Map<rustc_middle[29c0f47ba1981f7e]::mir::traversal::ReversePostorder, rustc_borrowck[6143a6a723f63a46]::do_mir_borrowck::{closure#2}>, rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>
  17:        0x1036b0a8c - rustc_borrowck[6143a6a723f63a46]::do_mir_borrowck
  18:        0x10361d81c - <rustc_infer[233d2e4639a00b1a]::infer::InferCtxtBuilder>::enter::<rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult, rustc_borrowck[6143a6a723f63a46]::mir_borrowck::{closure#0}>
  19:        0x1036a9a34 - rustc_borrowck[6143a6a723f63a46]::mir_borrowck
  20:        0x10368bf40 - <rustc_borrowck[6143a6a723f63a46]::provide::{closure#0} as core[489ab2277f19021d]::ops::function::FnOnce<(rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId)>>::call_once
  21:        0x103c4c444 - <rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepGraph<rustc_middle[29c0f47ba1981f7e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId, &rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult>
  22:        0x103b50968 - rustc_data_structures[ba5b925ae5d36803]::stack::ensure_sufficient_stack::<(&rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult, rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepNodeIndex), rustc_query_system[2ed05f00384aea15]::query::plumbing::execute_job<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId, &rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult>::{closure#3}>
  23:        0x103a321c4 - rustc_query_system[2ed05f00384aea15]::query::plumbing::try_execute_query::<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_query_system[2ed05f00384aea15]::query::caches::DefaultCache<rustc_span[6eae83f828617b72]::def_id::LocalDefId, &rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult>>
  24:        0x103aa2194 - rustc_query_system[2ed05f00384aea15]::query::plumbing::get_query::<rustc_query_impl[1e8f6af660f906f5]::queries::mir_borrowck, rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt>
  25:        0x10367fe78 - <rustc_borrowck[6143a6a723f63a46]::type_check::TypeChecker>::prove_closure_bounds
  26:        0x10367dca4 - <rustc_borrowck[6143a6a723f63a46]::type_check::TypeChecker>::check_rvalue
  27:        0x103683434 - <rustc_borrowck[6143a6a723f63a46]::type_check::TypeChecker>::typeck_mir
  28:        0x1036777d4 - rustc_borrowck[6143a6a723f63a46]::type_check::type_check
  29:        0x103642268 - rustc_borrowck[6143a6a723f63a46]::nll::compute_regions
  30:        0x1036ae4d8 - rustc_borrowck[6143a6a723f63a46]::do_mir_borrowck
  31:        0x10361d81c - <rustc_infer[233d2e4639a00b1a]::infer::InferCtxtBuilder>::enter::<rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult, rustc_borrowck[6143a6a723f63a46]::mir_borrowck::{closure#0}>
  32:        0x1036a9a34 - rustc_borrowck[6143a6a723f63a46]::mir_borrowck
  33:        0x10368bf40 - <rustc_borrowck[6143a6a723f63a46]::provide::{closure#0} as core[489ab2277f19021d]::ops::function::FnOnce<(rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId)>>::call_once
  34:        0x103c4c444 - <rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepGraph<rustc_middle[29c0f47ba1981f7e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId, &rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult>
  35:        0x103b50968 - rustc_data_structures[ba5b925ae5d36803]::stack::ensure_sufficient_stack::<(&rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult, rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepNodeIndex), rustc_query_system[2ed05f00384aea15]::query::plumbing::execute_job<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId, &rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult>::{closure#3}>
  36:        0x103a321c4 - rustc_query_system[2ed05f00384aea15]::query::plumbing::try_execute_query::<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_query_system[2ed05f00384aea15]::query::caches::DefaultCache<rustc_span[6eae83f828617b72]::def_id::LocalDefId, &rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult>>
  37:        0x103aa2194 - rustc_query_system[2ed05f00384aea15]::query::plumbing::get_query::<rustc_query_impl[1e8f6af660f906f5]::queries::mir_borrowck, rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt>
  38:        0x1031f8cd4 - rustc_typeck[3e022c09ec342e5c]::collect::type_of::type_of
  39:        0x103c5a70c - <rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepGraph<rustc_middle[29c0f47ba1981f7e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, rustc_span[6eae83f828617b72]::def_id::DefId, &rustc_middle[29c0f47ba1981f7e]::ty::TyS>
  40:        0x103b4f688 - rustc_data_structures[ba5b925ae5d36803]::stack::ensure_sufficient_stack::<(&rustc_middle[29c0f47ba1981f7e]::ty::TyS, rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepNodeIndex), rustc_query_system[2ed05f00384aea15]::query::plumbing::execute_job<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_span[6eae83f828617b72]::def_id::DefId, &rustc_middle[29c0f47ba1981f7e]::ty::TyS>::{closure#3}>
  41:        0x103a4af78 - rustc_query_system[2ed05f00384aea15]::query::plumbing::try_execute_query::<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_query_system[2ed05f00384aea15]::query::caches::DefaultCache<rustc_span[6eae83f828617b72]::def_id::DefId, &rustc_middle[29c0f47ba1981f7e]::ty::TyS>>
  42:        0x103ad8bac - rustc_query_system[2ed05f00384aea15]::query::plumbing::get_query::<rustc_query_impl[1e8f6af660f906f5]::queries::type_of, rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt>
  43:        0x1032107ac - rustc_typeck[3e022c09ec342e5c]::check::check::check_item_type
  44:        0x1031c1138 - <rustc_middle[29c0f47ba1981f7e]::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck[3e022c09ec342e5c]::check::CheckItemTypesVisitor>
  45:        0x103214fb4 - rustc_typeck[3e022c09ec342e5c]::check::check::check_mod_item_types
  46:        0x103c4d1fc - <rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepGraph<rustc_middle[29c0f47ba1981f7e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId, ()>
  47:        0x103b549e4 - rustc_data_structures[ba5b925ae5d36803]::stack::ensure_sufficient_stack::<((), rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepNodeIndex), rustc_query_system[2ed05f00384aea15]::query::plumbing::execute_job<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId, ()>::{closure#3}>
  48:        0x103a34038 - rustc_query_system[2ed05f00384aea15]::query::plumbing::try_execute_query::<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_query_system[2ed05f00384aea15]::query::caches::DefaultCache<rustc_span[6eae83f828617b72]::def_id::LocalDefId, ()>>
  49:        0x103ab5c90 - rustc_query_system[2ed05f00384aea15]::query::plumbing::get_query::<rustc_query_impl[1e8f6af660f906f5]::queries::check_mod_item_types, rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt>
  50:        0x1031bf370 - <rustc_middle[29c0f47ba1981f7e]::hir::map::Map>::for_each_module::<rustc_typeck[3e022c09ec342e5c]::check_crate::{closure#6}::{closure#0}>
  51:        0x1031071c4 - <rustc_session[5f6c487c9c714a47]::session::Session>::time::<(), rustc_typeck[3e022c09ec342e5c]::check_crate::{closure#6}>
  52:        0x103106c9c - rustc_typeck[3e022c09ec342e5c]::check_crate
  53:        0x100eb351c - rustc_interface[3e6b3c7def9e9d82]::passes::analysis
  54:        0x103c6d8c0 - <rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepGraph<rustc_middle[29c0f47ba1981f7e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, (), core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>
  55:        0x103b466a0 - rustc_data_structures[ba5b925ae5d36803]::stack::ensure_sufficient_stack::<(core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_query_system[2ed05f00384aea15]::dep_graph::graph::DepNodeIndex), rustc_query_system[2ed05f00384aea15]::query::plumbing::execute_job<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, (), core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#3}>
  56:        0x103a61d10 - rustc_query_system[2ed05f00384aea15]::query::plumbing::try_execute_query::<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_query_system[2ed05f00384aea15]::query::caches::DefaultCache<(), core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>>
  57:        0x103ad8cfc - rustc_query_system[2ed05f00384aea15]::query::plumbing::get_query::<rustc_query_impl[1e8f6af660f906f5]::queries::analysis, rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt>
  58:        0x100e016f4 - <rustc_interface[3e6b3c7def9e9d82]::passes::QueryContext>::enter::<rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>
  59:        0x100de5998 - <rustc_interface[3e6b3c7def9e9d82]::interface::Compiler>::enter::<rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}::{closure#2}, core[489ab2277f19021d]::result::Result<core[489ab2277f19021d]::option::Option<rustc_interface[3e6b3c7def9e9d82]::queries::Linker>, rustc_errors[1a053f9350564121]::ErrorReported>>
  60:        0x100daaf90 - rustc_span[6eae83f828617b72]::with_source_map::<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_interface[3e6b3c7def9e9d82]::interface::create_compiler_and_run<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#1}>
  61:        0x100de6220 - rustc_interface[3e6b3c7def9e9d82]::interface::create_compiler_and_run::<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>
  62:        0x100db6948 - <scoped_tls[34b11748aea080ce]::ScopedKey<rustc_span[6eae83f828617b72]::SessionGlobals>>::set::<rustc_interface[3e6b3c7def9e9d82]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[3e6b3c7def9e9d82]::interface::run_compiler<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#0}::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>
  63:        0x100db49b4 - std[edbc36b44a871839]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3e6b3c7def9e9d82]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[3e6b3c7def9e9d82]::interface::run_compiler<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>
  64:        0x100e069d0 - <<std[edbc36b44a871839]::thread::Builder>::spawn_unchecked<rustc_interface[3e6b3c7def9e9d82]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[3e6b3c7def9e9d82]::interface::run_compiler<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#1} as core[489ab2277f19021d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  65:        0x10844e318 - std::sys::unix::thread::Thread::new::thread_start::h02ceb9b16148ae76
  66:        0x199547878 - _pthread_jit_write_protect_np

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (65c55bf93 2021-11-23) running on aarch64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `<impl at src/main.rs:18:1: 21:3>::from::test_endpoint::{closure#0}`
#1 [mir_borrowck] borrow-checking `<impl at src/main.rs:18:1: 21:3>::from::test_endpoint`
#2 [type_of] computing type of `<impl at src/main.rs:18:1: 21:3>::from::test_endpoint::{opaque#0}`
#3 [check_mod_item_types] checking item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `dropshot-rustc-panic`
```

## Expected behavior

With other Rust versions, the compiler fails with a message I would expect.

For example:

```
$ rustup default stable
$ cargo build          
   Compiling dropshot-rustc-panic v0.1.0 (/Users/jordanhendricks/src/play/dropshot-rustc-panic)
error[E0596]: cannot borrow `server_context.field` as mutable, as it is behind a `&` reference
  --> src/main.rs:29:5
   |
25 |     let server_context = rqctx.context();
   |         -------------- help: consider changing this to be a mutable reference: `&mut ServerContext`
...
29 |     server_context.field.test_mut();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `server_context` is a `&` reference, so the data it refers to cannot be borrowed as mutable

```

