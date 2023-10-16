use pythnet_sdk::wire::array;

use {
    anyhow::Result,
    axum::{
        extract::State,
        Json,
    },
    serde_qs::axum::QsQuery,
    utoipa::{
        IntoParams,
        ToSchema,
    },
};
use crate::api::RestError;

// TODO: this should probably take path parameters /v1/revelation/<chain_id>/<sequence_number>
/// Reveal the random value for a given sequence number.
///
/// Given a sequence number, retrieve the corresponding random value that this provider has committed to.
/// This endpoint will not return the random value unless someone has requested the sequence number on-chain.
#[utoipa::path(
get,
path = "/v1/revelation",
responses(
(status = 200, description = "Random value successfully retrieved", body = GetRandomValueResponse),
(status = 403, description = "Random value cannot currently be retrieved", body = String)
),
params(
GetRandomValueQueryParams
)
)]
pub async fn revelation(
    State(state): State<crate::api::ApiState>,
    QsQuery(params): QsQuery<GetRandomValueQueryParams>,
) -> Result<Json<GetRandomValueResponse>, RestError> {
    let sequence: u64 = params.sequence.try_into().map_err(|_| RestError::InvalidSequenceNumber)?;

    let r = state.contract.get_request(state.provider_address, sequence).call().await.map_err(|e| RestError::TemporarilyUnavailable)?;

    if r.sequence_number != 0 {
        let value = &state.state.reveal(sequence).map_err(|_| RestError::Unknown)?;
        Ok(Json(GetRandomValueResponse { value: (*value).clone() }))
    } else {
        Err(RestError::NoPendingRequest)
    }
}

#[derive(Debug, serde::Deserialize, IntoParams)]
#[into_params(parameter_in=Query)]
pub struct GetRandomValueQueryParams {
    sequence: u64,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct GetRandomValueResponse {
    // TODO: choose serialization format
    #[serde(with = "array")]
    value:      [u8; 32],
}