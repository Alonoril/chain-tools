use base_infra::map_err;
use base_infra::result::AppResult;
use endless_sdk::rest_client::endless_api_types::{IdentifierWrapper, MoveStructTag, MoveType};
use chain_types::endless::AccountAddress;
use crate::error::EdsErr;

pub fn build_move_struct_tag(
    addr: AccountAddress,
    module: &str,
    name: &str,
    gt_params: Vec<MoveType>,
) -> AppResult<MoveStructTag> {
    let module = module
        .parse::<IdentifierWrapper>()
        .map_err(map_err!(&EdsErr::InvalidModuleName))?;
    let name = name
        .parse::<IdentifierWrapper>()
        .map_err(map_err!(&EdsErr::InvalidEventName))?;
    Ok(MoveStructTag::new(addr.into(), module, name, gt_params))
}