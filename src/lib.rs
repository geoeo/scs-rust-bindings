#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

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
    fn ScsMatrixStruct() {
        assert_eq!(40,mem::size_of::<ScsMatrix>());
    }
}
