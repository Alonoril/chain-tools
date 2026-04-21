base_infra::gen_impl_code_enum! {
    EdsErr {
        // bcs
        ToBcsBytes = ("BCS000", "Failed to convert to bcs bytes"),

        // acct
        GetAcctSeqNum = ("ACCT01", "Failed to get account sequence number"),
        InvalidHexPriKey = ("ACCT02", "Invalid hex private key"),
        ParseToEd25519Sk = ("ACCT03", "Failed to parse to ed25519 private key"),

        DecodeEventErr = ("EDS_SDK01", "Failed to decode event"),
        InvalidModuleName = ("EDS_SDK02", "Invalid module name"),
        InvalidEventName = ("EDS_SDK03", "Invalid module event name"),

        // client
        InvalidNodeUrl = ("CLT001", "Invalid endless node url"),
        GetVersionErr = ("CLT002", "get_endless_version failed"),
        TokenBalanceOf = ("CLT003", "Get primary_fungible_store::balance failed"),
        EdsBalanceOf = ("CLT004", "Get endless_coin::balance failed"),
        ParseHashValue = ("CLT005", "Failed to parse to HashValue by"),

        GetIndexErr = ("SDK000", "Failed to get_index"),
        ParseIdentifier = ("SDK001", "Failed to parse to Identifier"),
        ParseTypeArgs = ("SDK002", "Failed to parse to TypeTag "),
        SubmitTxnErr = ("SDK003", "Failed to submit transaction"),
        SystemTimeErr = ("SDK004", "Failed to get system time"),
        ViewBcsErr = ("SDK005", "Failed to view_bcs"),
        SimulateTxnErr = ("SDK006", "Failed to simulate transaction"),
        WaitForTxnErr = ("SDK007", "Failed to wait for transaction"),
        GetTxnByHash = ("SDK008", "Failed to get transaction by hash"),
        GetTxnsByVersions = ("SDK009", "Failed to get transactions by versions"),
        GetTxnByVersion = ("SDK010", "Failed to get transaction by version"),
        ParseTxnInfo = ("SDK011", "Failed to parse transaction info"),

    }
}
