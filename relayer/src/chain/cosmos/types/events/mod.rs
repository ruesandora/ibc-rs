use ibc::events::IbcEvent;
use ibc::Height;
use tendermint::abci::Event as AbciEvent;

pub mod channel;
pub mod client;
pub mod connection;

pub fn from_tx_response_event(height: Height, event: &AbciEvent) -> Option<IbcEvent> {
    // Return the first hit we find
    if let Some(mut client_res) = client::try_from_tx(event) {
        client_res.set_height(height);
        Some(client_res)
    } else if let Some(mut conn_res) = connection::try_from_tx(event) {
        conn_res.set_height(height);
        Some(conn_res)
    } else if let Some(mut chan_res) = channel::try_from_tx(event) {
        chan_res.set_height(height);
        Some(chan_res)
    } else {
        None
    }
}
