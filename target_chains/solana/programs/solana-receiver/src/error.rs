use anchor_lang::prelude::*;

#[error_code]
pub enum ReceiverError {
    #[msg("The emitter of the VAA is not Solana or Pythnet.")]
    EmitterChainNotSolanaOrPythnet,
    #[msg("The posted VAA has wrong magic number.")]
    PostedVaaHeaderWrongMagicNumber,
    #[msg("An error occured when deserializing the VAA.")]
    DeserializeVAAFailed,
    #[msg("An error occurred when deserializing the updates.")]
    DeserializeUpdateFailed,
    #[msg("Received an invalid wormhole message")]
    InvalidWormholeMessage,
    #[msg("Received an invalid price update")]
    InvalidPriceUpdate,
    #[msg("Received an invalid accumulator message")]
    InvalidAccumulatorMessage,
    #[msg("Received an invalid accumulator message type")]
    InvalidAccumulatorMessageType,
}
