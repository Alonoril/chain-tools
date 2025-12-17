base_infra::gen_impl_code_enum! {
    EdsErr {
        // bcs
        ToBcsBytes = ("BCS000", "Failed to convert to bcs bytes"),

        // acct
        GetAcctSeqNum = ("ACCT01", "Failed to get account sequence number"),
        InvalidHexPriKey = ("ACCT02", "Invalid hex private key"),
        ParseToEd25519Sk = ("ACCT03", "Failed to parse to ed25519 private key"),

        // client
        InvalidNodeUrl = ("CLT001", "Invalid endless node url"),
        GetVersionErr = ("CLT002", "get_endless_version failed"),
        TokenBalanceOf = ("CLT003", "Get primary_fungible_store::balance failed"),
        EdsBalanceOf = ("CLT004", "Get endless_coin::balance failed"),

        GetIndexErr = ("SDK000", "Failed to get_index"),
        ParseIdentifier = ("SDK001", "Failed to parse to Identifier"),
        ParseTypeArgs = ("SDK002", "Failed to parse to TypeTag "),
        SubmitTxnErr = ("SDK003", "Failed to submit transaction"),
        SystemTimeErr = ("SDK004", "Failed to get system time"),
        ViewBcsErr = ("SDK005", "Failed to view_bcs"),
        SimulateTxnErr = ("SDK006", "Failed to simulate transaction"),
        WaitForTxnErr = ("SDK007", "Failed to wait for transaction"),

    }
}
