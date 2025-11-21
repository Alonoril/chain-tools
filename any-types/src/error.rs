base_infra::gen_impl_code_enum! {
    TypErr {
        // date
        // StrToDate = ("UTC001", "Failed to parse NaiveDateTime from string "),
        // TimestampToDate = ("UTC002", "Failed to parse NaiveDateTime from timestamp"),

        // string / uxx
        ParseU128Err = ("Uint01", "Failed to parse u128 from string"),

        // Uxx / decimal
        ParseDecimalErr = ("DEC001", "Failed to parse decimal from string"),
        U128ToDecErr = ("DEC002", "Failed to parse decimal from u128"),
        U64ToDecErr = ("DEC00", "Failed to parse decimal from u64"),
        EthUnitsError = ("DEC003", "Failed to parse decimal from Ether units"),
        // Uxx to big_decimal
        U128ToBigDec = ("DEC005", "Failed to parse BigDecimal from U128"),
        U256ToBigDec = ("DEC005", "Failed to parse BigDecimal from U256"),
        BigDecToRsU128 = ("DEC006", "Failed to convert BigDecimal to Rust U128"),
        F32ToBigDecErr = ("DEC007", "Failed to parse decimal from f32"),
        F64ToBigDecErr = ("DEC008", "Failed to parse decimal from f64"),
        U256FromStr = ("DEC009", "Failed to parse string to U256"),
        // str to BigDecimal
        BigDecFromStr = ("DEC010", "Failed to parse string to BigDecimal"),

        // bcs
        ToBcsBytes = ("BCS000", "Failed to convert to bcs bytes"),

        // pool type
        IllPoolTypeValue = ("PoolType", "Invalid pool type value"),


        // chain
        InvalidChain = ("CHN001", "Invalid chain"),
        InvalidChainType = ("CHN002", "Invalid chain type"),

        InvalidAddrLen = ("ADDR04", "Invalid address length"),
        AcctAddrParseErr = ("ACAD01", "Failed to parse to account address"),

        // token Type
        IllTokenTypeValue = ("TOKEN1", "Invalid token type value"),

        // solidity type
        EvmParseSolValueErr = ("EvmSolValErr", "Failed to parse evm solidity value by "),
        IllSolTypeValue = ("EvmSolType", "Invalid solidity type"),
        EvmSolStrParseErr = ("EvmSolStr", "Failed to parse evm solidity type to rust string"),
    }
}
