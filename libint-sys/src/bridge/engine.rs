#[cxx::bridge]
mod ffi {
    enum Operator {
        overlap,
        kinetic,
        nuclear,
        erf_nuclear,
        erfc_nuclear,
        erfx_nuclear,
        q_gau,
        emultipole1,
        emultipole2,
        emultipole3,
        sphemultipole,
        opVop,
        op_q_gau_op,
        delta,
        coulomb,
        r12_m1 = coulomb,
        cgtg,
        cgtg_x_coulomb,
        delcgtg2,
        r12,
        r12_1 = r12,
        erf_coulomb,
        erfc_coulomb,
        erfx_coulomb,
        stg,
        stg_x_coulomb,
        yukawa = stg_x_coulomb,
        invalid = -1,
        first_1body_oper = overlap,
        last_1body_oper = op_q_gau_op,
        first_2body_oper = delta,
        last_2body_oper = stg_x_coulomb,
        first_oper = first_1body_oper,
        last_oper = last_2body_oper,
    }

    extern "C++" {
        include!("");

        type Engine;
        type Operator;
    }
}
