use yew::prelude::*;
use sc::web3::Web3;

use crate::sc;
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
        //create a context provider. 
<h1>{""}</h1>
<form>
<Web3/>
      
</form>     
        </div>
    }
}