#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod groth16 {
    use ink_prelude::vec::Vec;
    // use curve::{
    //     curve::{Bls12_377, Bls12_381, Bn254, BW6_761},
    //     groth16,
    // };

    /// Groth16 exports
    #[ink(storage)]
    pub struct Groth16;

    impl Groth16 {
        #[ink(constructor)]
        pub fn default() -> Self {
            Groth16 {}
        }

        #[ink(message)]
        pub fn bls12_377_verify(
            &self,
            vk_gamma_abc: Vec<u8>,
            vk: Vec<u8>,
            proof: Vec<u8>,
            public_inputs: Vec<Vec<u8>>,
        ) -> Option<bool> {
            // groth16::verify::<Bls12_377>(&vk_gamma_abc, &vk, &proof, &public_inputs).ok()
            Some(true)
        }
    }
}
