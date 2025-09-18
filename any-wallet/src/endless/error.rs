base_infra::gen_impl_code_enum! {
    EdsWltErr {
        InvalidMnemonic = ("WLT001", "Invalid mnemonic phrase"),
        SeedDerive = ("WLT002", "Failed to derive root key from mnemonic"),
        ChildDerive = ("WLT003", "Failed to derive child private key"),
        PrivateKey = ("WLT004", "Failed to convert child key into Endless private key"),
        IndexOverflow = ("WLT005", "Mnemonic derivation index overflow"),
    }
}
