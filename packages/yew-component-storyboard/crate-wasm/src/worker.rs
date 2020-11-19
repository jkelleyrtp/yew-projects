use serde_derive::{Deserialize, Serialize};
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetDataFromServer,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    DataFetched,
}

pub enum Msg {
    Updating,
}
pub struct Worker {
    link: AgentLink<Worker>,
}

impl Agent for Worker {
    // In Yew 14.1 Private workers are not actually fully implemented
    // Expect this to get fixed, but you will have to use my fork in Cargo for now
    type Reach = Public<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Worker { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::GetDataFromServer => {
                self.link.respond(who, Response::DataFetched);
            }
        }
    }

    // This needs to be whatever webpack is generating from the workerconfig
    // const worker_config = {
    //     ...
    //     output: {
    //     path: dist,
    //     filename: "worker.js"    <------ this file
    //     }
    // }
    // fn name_of_resource() -> &'static str {
    //     "worker.js"
    // }
}
