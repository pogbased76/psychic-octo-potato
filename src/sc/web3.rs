use yew::prelude::*;
use yew_ethereum_provider::{
    chain, AccountLabel, ConnectButton, EthereumContextProvider, SwitchNetworkButton,
};

#[function_component]
pub fn Web3() -> Html {
    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton />
                <SwitchNetworkButton chain={chain::ethereum()}/>
                <SwitchNetworkButton chain={chain::avalanche_testnet()}/>
                <AccountLabel />
            </EthereumContextProvider>
        </div>
    }
}

//After the user logs. If it's there first time prompt them for a username.
//than give them option dispaly names & aliases.