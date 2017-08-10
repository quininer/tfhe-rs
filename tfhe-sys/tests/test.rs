extern crate tfhe_sys;

use tfhe_sys::*;


#[allow(non_upper_case_globals)]
#[test]
fn test_tfhe() {
    const N: i32 = 1024;
    const k: i32 = 1;
    const n: i32 = 500;
    const l_bk: i32 = 3;
    const Bgbit_bk: i32 = 10;
    const ks_t: i32 = 15;
    const ks_basebit: i32 = 1;
    const alpha_in: f64 = 5e-4;
    const alpha_bk: f64 = 9e-9;

    unsafe {
        let params_in = new_LweParams(n, alpha_in, 1. / 16.);
        let params_accum = new_TLweParams(N, k, alpha_bk, 1. / 16.);
        let params_bk = new_TGswParams(l_bk, Bgbit_bk, params_accum);

        let key = new_LweKey(params_in);
        lweKeyGen(key);

        let key_bk = new_TGswKey(params_bk);
        tGswKeyGen(key_bk);

        let bk = new_LweBootstrappingKey(ks_t, ks_basebit, params_in, params_bk);
        tfhe_createLweBootstrappingKey(bk, key, key_bk);

        let test = new_LweSample(params_in);
        let test_out = new_LweSample(params_in);

        let mu = modSwitchToTorus32(1, 4);

        let mu_in = modSwitchToTorus32(-1, 4);
        lweSymEncrypt(test, mu_in, alpha_in, key);
        println!("in_message: {}", mu_in);

        println!("starting bootstrapping...");

        tfhe_bootstrap(test_out, bk, mu, test);

        let mu_out = lweSymDecrypt(test_out, key, 4);
        let phase_out = lwePhase(test_out, key);
        println!("end_variance: {}", (*test_out).current_variance);
        println!("end_phase: {}", phase_out);
        println!("end_message: {}", mu_out);

        assert_eq!(mu_in, mu_out);

        delete_LweSample(test_out);
        delete_LweSample(test);
        delete_LweBootstrappingKey(bk);
        delete_TGswKey(key_bk);
        delete_LweKey(key);
        delete_TGswParams(params_bk);
        delete_TLweParams(params_accum);
        delete_LweParams(params_in);
    }
}
