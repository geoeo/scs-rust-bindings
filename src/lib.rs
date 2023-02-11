#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::alloc::{alloc, dealloc, Layout};

    #[test]
    fn scs_flags() {
        assert_eq!(SCS_INFEASIBLE_INACCURATE,-7);
        assert_eq!(SCS_UNBOUNDED_INACCURATE,-6);
        assert_eq!(SCS_SIGINT,-5);
        assert_eq!(SCS_FAILED,-4);
        assert_eq!(SCS_INDETERMINATE,-3);
        assert_eq!(SCS_INFEASIBLE,-2);
        assert_eq!(SCS_UNBOUNDED,-1);
        assert_eq!(SCS_UNFINISHED,0);
        assert_eq!(SCS_SOLVED,1);
        assert_eq!(SCS_SOLVED_INACCURATE,2);
    }

    #[test]
    fn scs_matrix_struct() {
        assert_eq!(40,mem::size_of::<ScsMatrix>());
    }

    #[test]
    fn basic_alloc() {
        unsafe {
            let k_layout = Layout::new::<ScsCone>();
            let k = alloc(k_layout);
            let k_ref = k as *mut ScsCone;
            (*k_ref).z = 1i64;
            assert_eq!(1,(*k_ref).z);
            dealloc(k,k_layout);
        }
    }

    /**
     * https://www.cvxgrp.org/scs/examples/c.html#c-example
     */
    #[test]
    fn c_example() {

        /* Set up the problem data */
        /* A and P must be in compressed sparse column format */
        let mut P_x : [f64;3] = [3., -1., 2.]; /* Upper triangular of P only */
        let mut P_i : [i64;3] = [0, 0, 1];
        let mut P_p : [i64;3] = [0, 1, 3];
        let mut A_x : [f64;4] = [-1., 1., 1., 1.];
        let mut A_i : [i64;4] = [0, 1, 0, 2];
        let mut A_p : [i64;3] = [0, 2, 4];
        let mut b : [f64;3] = [-1., 0.3, -0.5];
        let mut c : [f64;2] = [-1., -1.];
        /* data shapes */
        let n : i64 = 2; /* number of variables */
        let m : i64 = 3; /* number of constraints */
        unsafe {

            /* Allocate SCS structs */
            let k_layout = Layout::new::<ScsCone>();
            let k = alloc(k_layout);
            let k_ref = k as *mut ScsCone;
            
            let d_layout = Layout::new::<ScsData>();
            let d = alloc(d_layout);
            let d_ref = d as *mut ScsData;

            let stgs_layout = Layout::new::<ScsSettings>();
            let stgs = alloc(stgs_layout);
            let stgs_ref = stgs as *mut ScsSettings;

            let sol_layout = Layout::new::<ScsSolution>();
            let sol = alloc(sol_layout);
            let sol_ref = sol as *mut ScsSolution;

            let info_layout = Layout::new::<ScsInfo>();
            let info = alloc(info_layout);
            let info_ref = info as *mut ScsInfo;

            let a_matrix_layout = Layout::new::<ScsMatrix>();
            let a_matrix = alloc(a_matrix_layout);
            let a_matrix_ref = a_matrix as *mut ScsMatrix;

            let p_matrix_layout = Layout::new::<ScsMatrix>();
            let p_matrix = alloc(p_matrix_layout);
            let p_matrix_ref = p_matrix as *mut ScsMatrix;

            /* Fill in data struct */
            (*d_ref).m = m;
            (*d_ref).n = n;
            (*d_ref).b = b.as_mut_ptr();
            (*d_ref).c = c.as_mut_ptr();
            (*d_ref).A = a_matrix_ref;
            (*d_ref).P = p_matrix_ref;

            (*a_matrix_ref).x = A_x.as_mut_ptr();
            (*a_matrix_ref).i = A_i.as_mut_ptr();
            (*a_matrix_ref).p = A_p.as_mut_ptr();
            (*a_matrix_ref).m = m;
            (*a_matrix_ref).n = n;

            (*p_matrix_ref).x = P_x.as_mut_ptr();
            (*p_matrix_ref).i = P_i.as_mut_ptr();
            (*p_matrix_ref).p = P_p.as_mut_ptr();
            (*p_matrix_ref).m = n;
            (*p_matrix_ref).n = n;

            /* Cone */
            (*k_ref).z = 1;
            (*k_ref).l = 2;

            scs_set_default_settings(stgs_ref);

            /* Modify tolerances */
            (*stgs_ref).eps_abs = 1e-9;
            (*stgs_ref).eps_rel = 1e-9;

            /* Initialize SCS workspace */
            let scs_work = scs_init(d_ref, k_ref, stgs_ref);

            /* Solve! */
            let exitflag = scs_solve(scs_work, sol_ref, info_ref, 0);

            /*
            * If we wanted to solve many related problems with different
            * b / c vectors we could update the SCS workspace as follows:
            *
            * int success = scs_update(scs_work, new_b, new_c)
            * int new_exitflag = scs_solve(scs_work, sol, info, 1);
            *
            */

            /* Free SCS workspace */
            scs_finish(scs_work);

            /* Verify that SCS solved the problem */
            println!("SCS solved successfully: {}", exitflag == SCS_SOLVED as i64);

            /* Print some info about the solve */
            println!("SCS took {} iters, using the {:?} linear solver.", (*info_ref).iter,
                    (*info_ref).lin_sys_solver);

            /* Print solution x */
            println!("Optimal solution vector x*:");
            for i in 0..n {
                let v =  ((*sol_ref).x).add(i as usize);
                println!("x[{}] = {}", i,*v);
            }

            /* Print dual solution y */
            println!("Optimal dual vector y*:");
            for i in 0..m {
                let v =  ((*sol_ref).y).add(i as usize);
                println!("y[{}] = {}", i, *v);
            }

            dealloc(p_matrix,p_matrix_layout);
            dealloc(a_matrix,a_matrix_layout);
            dealloc(info,info_layout);
            dealloc(sol,sol_layout);
            dealloc(stgs,stgs_layout);
            dealloc(d,d_layout);
            dealloc(k,k_layout);
        }
    }

}
