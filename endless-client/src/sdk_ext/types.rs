use crate::error::EdsErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use endless_sdk::helper_client::Overrides;
use endless_sdk::move_types::account_address::AccountAddress;
use endless_sdk::move_types::identifier::Identifier;
use endless_sdk::move_types::language_storage::{ModuleId, TypeTag};
use endless_sdk::rest_client::endless_api_types::ViewFunction;
use endless_sdk::types::LocalAccount;
use endless_sdk::types::chain_id::ChainId;
use endless_sdk::types::transaction::EntryFunction;
use moka::future::Cache;
use std::sync::OnceLock;

static CHAIN_ID_CACHE: OnceLock<Cache<(), ChainId>> = OnceLock::new();
pub(crate) struct ChainIdCache;
impl ChainIdCache {
    fn cache(&self) -> &'static Cache<(), ChainId> {
        CHAIN_ID_CACHE.get_or_init(|| Cache::new(1))
    }

    pub async fn get(&self) -> Option<ChainId> {
        self.cache().get(&()).await
    }

    pub async fn set(&self, chain_id: ChainId) {
        self.cache().insert((), chain_id).await;
    }
}

#[derive(Clone)]
pub struct EntryFnArgs<'a> {
    pub module_address: AccountAddress,
    pub signer: &'a LocalAccount,
    pub entry_fn: EntryFunction,
    pub overrides: Option<Overrides>,
    pub fn_name: String,
}

impl<'a> EntryFnArgs<'a> {
    pub fn new(
        signer: &'a LocalAccount,
        module_address: AccountAddress,
        module_name: &'a str,
        function_name: &'a str,
        args: Vec<Vec<u8>>,
        type_args: Vec<&'a str>,
    ) -> AppResult<Self> {
        let fn_name = format!("{module_name}::{function_name}");
        let module_name = Identifier::new(module_name).map_err(map_err!(
            &EdsErr::ParseIdentifier,
            format!("from module {module_name}")
        ))?;

        let fun = Identifier::new(function_name).map_err(map_err!(
            &EdsErr::ParseIdentifier,
            format!("from function {function_name}")
        ))?;

        let m_id = ModuleId::new(module_address, module_name);
        let ty_args = parse_type_tags(type_args)?;
        let entry_fn = EntryFunction::new(m_id, fun, ty_args, args);

        Ok(Self {
            module_address,
            signer,
            entry_fn,
            overrides: None,
            fn_name,
        })
    }

    pub fn with_overrides(self, overrides: Option<Overrides>) -> Self {
        Self { overrides, ..self }
    }
}

#[derive(Clone)]
pub struct ViewFnArgs {
    pub module_address: AccountAddress,
    pub view_fn: ViewFunction,
}

impl ViewFnArgs {
    pub fn new(
        module_address: AccountAddress,
        module_name: &str,
        function_name: &str,
        args: Vec<Vec<u8>>,
        type_args: Vec<&str>,
    ) -> AppResult<Self> {
        let module_name = Identifier::new(module_name).map_err(map_err!(
            &EdsErr::ParseIdentifier,
            format!("from module {module_name}")
        ))?;

        let fun = Identifier::new(function_name).map_err(map_err!(
            &EdsErr::ParseIdentifier,
            format!("from function {function_name}")
        ))?;

        let view_fn = ViewFunction {
            module: ModuleId::new(module_address, module_name),
            function: fun,
            ty_args: parse_type_tags(type_args)?,
            args,
        };

        Ok(Self {
            module_address,
            view_fn,
        })
    }
}

fn parse_type_tag(s: &str) -> AppResult<TypeTag> {
    s.parse::<TypeTag>()
        .map_err(map_err!(&EdsErr::ParseTypeArgs, s.to_string()))
}

fn parse_type_tags(s: Vec<&str>) -> AppResult<Vec<TypeTag>> {
    s.into_iter().map(|s| parse_type_tag(s)).collect()
}
