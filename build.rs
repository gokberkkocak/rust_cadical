extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "generate-bindings")]
    generate_bindings();
    build();
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(){
    let bindings = bindgen::Builder::default()
        .header("vendor/ccadical.h")
        .whitelist_type("*CCaDiCaL*")
        .whitelist_function("ccadical_init")
        .whitelist_function("ccadical_assume")
        .whitelist_function("ccadical_release")
        .whitelist_function("ccadical_signature")
        .whitelist_function("ccadical_add")
        .whitelist_function("ccadical_val")
        .whitelist_function("ccadical_solve")
        .whitelist_function("ccadical_simplify")
        .whitelist_function("ccadical_set_option")
        .whitelist_function("ccadical_print_statistics")
        .whitelist_function("ccadical_solver_nodes")
        .whitelist_function("ccadical_nb_learnt")
        .rustfmt_bindings(true)
        // .enable_cxx_namespaces()
        .clang_arg("-Ivendor/")
        // .clang_arg(r"-std=c++11")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cadical_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build() {
    cc::Build::new()
        .include("vendor")
        .cpp(true)
        .file("vendor/analyze.cpp")  
        .file("vendor/arena.cpp")  
        .file("vendor/assume.cpp")  
        .file("vendor/averages.cpp")  
        .file("vendor/backtrack.cpp")  
        .file("vendor/backward.cpp")  
        .file("vendor/bins.cpp")  
        .file("vendor/block.cpp")  
        // .file("vendor/cadical.cpp")  
        .file("vendor/ccadical.cpp")  
        .file("vendor/checker.cpp")  
        .file("vendor/clause.cpp")  
        .file("vendor/collect.cpp")  
        .file("vendor/compact.cpp")  
        .file("vendor/condition.cpp")  
        .file("vendor/config.cpp")  
        .file("vendor/cover.cpp")  
        .file("vendor/decide.cpp")  
        .file("vendor/decompose.cpp")  
        .file("vendor/deduplicate.cpp")  
        .file("vendor/elim.cpp")  
        .file("vendor/ema.cpp")  
        .file("vendor/extend.cpp")  
        .file("vendor/external.cpp")  
        .file("vendor/file.cpp")  
        .file("vendor/flags.cpp")  
        .file("vendor/format.cpp")  
        .file("vendor/gates.cpp")  
        .file("vendor/instantiate.cpp")  
        .file("vendor/internal.cpp")  
        .file("vendor/ipasir.cpp")  
        .file("vendor/limit.cpp")  
        .file("vendor/logging.cpp")  
        .file("vendor/lucky.cpp")  
        .file("vendor/message.cpp")  
        .file("vendor/minimize.cpp")  
        // .file("vendor/mobical.cpp")  
        .file("vendor/occs.cpp")  
        .file("vendor/options.cpp")  
        .file("vendor/parse.cpp")  
        .file("vendor/phases.cpp")  
        .file("vendor/probe.cpp")  
        .file("vendor/profile.cpp")  
        .file("vendor/proof.cpp")  
        .file("vendor/propagate.cpp")  
        .file("vendor/queue.cpp")  
        .file("vendor/random.cpp")  
        .file("vendor/reduce.cpp")  
        .file("vendor/rephase.cpp")  
        .file("vendor/report.cpp")  
        .file("vendor/resources.cpp")  
        .file("vendor/restart.cpp")  
        .file("vendor/restore.cpp")  
        .file("vendor/score.cpp")  
        .file("vendor/signal.cpp")  
        .file("vendor/solution.cpp")  
        .file("vendor/solver.cpp")  
        .file("vendor/stats.cpp")  
        .file("vendor/subsume.cpp")  
        .file("vendor/terminal.cpp")  
        .file("vendor/ternary.cpp")  
        .file("vendor/tracer.cpp")  
        .file("vendor/transred.cpp")  
        .file("vendor/util.cpp")  
        .file("vendor/var.cpp")  
        // .file("vendor/version.cpp")  
        .file("vendor/vivify.cpp")  
        .file("vendor/walk.cpp")  
        .file("vendor/watch.cpp")  
        .flag_if_supported("DNDEBUG")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-w")
        .opt_level(3)
        .compile("cadical");
}