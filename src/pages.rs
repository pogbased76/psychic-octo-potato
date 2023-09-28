use yew_router::prelude::*;

//Create home feeds, settings, contracts, channels, users ca$htags #tags. 
//
//search for an example configuration from an app that uses both vite and trunk.
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/profile")]
    Profile,
    #[at("/:username")]
    Username,
    #[at("/:forum")]
    Forum,
    #[at("/:$")]
    Cashtag,
    #[not_found]
    #[at("/404")]
    NotFound,
}