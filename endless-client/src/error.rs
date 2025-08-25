base_infra::gen_impl_code_enum! {
    SdkErr {
        InvalidHexPriKey = ("ACCT02", "Invalid hex private key"),
        ParseToEd25519Sk = ("ACCT03", "Failed to parse to ed25519 private key"),

        GetIndexErr = ("SDK000", "Failed to get_index"),
        ParseIdentifier = ("SDK001", "Failed to parse to Identifier"),
        ParseTypeArgs = ("SDK002", "Failed to parse to TypeTag "),
        SubmitTxnErr = ("SDK003", "Failed to submit transaction"),
        SystemTimeErr = ("SDK004", "Failed to get system time"),
        ViewBcsErr = ("SDK005", "Failed to view_bcs"),
    }
}