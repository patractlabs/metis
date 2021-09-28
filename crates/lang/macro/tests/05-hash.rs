#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests {
    use impl_serde::serialize as serde_hex;
    use metis_lang as metis;

    #[test]
    fn hash_works() {
        let hashs = metis::hash!(polkadot);

        let hex = serde_hex::to_hex(&hashs, false);
        assert_eq!(
            hex,
            "0x91f5db5efd808f0b0414a29c7a11e977da0d4b09c0ac8b52af9d60fe67e25367"
        );
    }

    #[test]
    fn select_id_works() {
        assert_eq!(
            metis::selector_id!(on_erc721_received),
            [90u8, 119u8, 73u8, 174u8]
        );
        assert_eq!(
            metis::selector_id!(supports_interface),
            [0xe6, 0x11, 0x3a, 0x8a]
        );
    }
}
