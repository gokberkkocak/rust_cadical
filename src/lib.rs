pub mod bindings {
    // #![allow(non_upper_case_globals)]
    // #![allow(non_camel_case_types)]
    // #![allow(non_snake_case)]
    // #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/cadical_bindings.rs"));
}

use bindings::CCaDiCaL;
use bindings::ccadical_init;
use bindings::ccadical_assume;
use bindings::ccadical_add;
use bindings::ccadical_solve;
use bindings::ccadical_val;
use bindings::ccadical_solver_nodes;
use bindings::ccadical_set_option;
use bindings::ccadical_nb_learnt;

use std::ffi::CString;

pub fn init_cadical_solver() -> *mut CCaDiCaL {
    let s : *mut CCaDiCaL =  unsafe { ccadical_init() };
    s
}

pub fn add_assumptions_to_cadical_solver(s : *mut CCaDiCaL, assumptions : Vec<i32>){
    for i in assumptions{
        unsafe {ccadical_assume(s, i) };
    }
}

pub fn add_clause_to_cadical_solver(s : *mut CCaDiCaL, given : Vec<i32>){
    for i in given{
        unsafe {ccadical_add(s, i) };
    }
    unsafe {ccadical_add(s, 0)};
}

pub fn run_cadical(s : *mut CCaDiCaL) -> i32 {
    // cadical automatically clears assumptions after the solver call.
    let ret = unsafe { ccadical_solve(s) };
    ret
} 


/// Gets a solution from Cadical solver while using the given nb_vars 
/// to allocate a new Rust solution vec to write and return.
pub fn get_cadical_solution(s : *mut CCaDiCaL, nb_vars : usize) -> Vec<i32>{
    let mut model : Vec<i32> = Vec::with_capacity(nb_vars);
    for i in 1..nb_vars+1{
        model.push( unsafe { ccadical_val(s, i as i32)}); 
    }
    model
}

/// Gets a solution from Cadical solver and writes on to the given Vector.
/// No memory allocation is done here unless the given model has a smaller capacity then the given nb_vars. 
pub fn get_cadical_solution_no_malloc(s : *mut CCaDiCaL, model : &mut Vec<i32>, nb_vars : usize){
    model.clear();
    for i in 1..nb_vars+1{
        model.push( unsafe { ccadical_val(s, i as i32)}); 
    }
}

pub fn get_cadical_solver_stats(s : *mut CCaDiCaL) -> i64{
    let nodes : i64 = unsafe { ccadical_solver_nodes(s)};
    nodes
}

pub fn get_cadical_nb_learnt_clauses(s : *mut CCaDiCaL) -> i64{
    let l : i64 = unsafe { ccadical_nb_learnt(s)};
    l
}

pub fn set_cadical_rnd_seed(s : *mut CCaDiCaL, seed : i64) {
    let rnd_text = CString::new("seed").expect("CString::new failed");
    unsafe { ccadical_set_option(s, rnd_text.as_ptr(), seed as i32) };
}
