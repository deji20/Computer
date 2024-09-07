use tauri::async_runtime::Sender;
use tokio_stream::StreamExt;
use crate::speech::Speech;
use crate::abilities::Ability;

pub struct Brain{
    speech: Speech,
    thoughts: Option<Sender<String>>,
}

impl Brain {
    pub fn new(thoughts: Option<Sender<String>>) -> Self { 
        Self {
            speech: Speech::default(),
            thoughts,
        }
    }

    pub async fn decision(&self, command: &str) -> Result<Ability, String> 
        {
        // format the below text to rust string
        let prompt = format!("
Let's Roleplay, You are a helpful AI assistant program made to determine which function 
it should call to handle a task input by the user. 
You have the following functions available to you:

{abilities}

your response should only ever be in the form of the following JSON objects:
{{
    \"ability\": string, 
    \"arguments\": {{ 
        \"arg1\": value, 
        \"arg2\": value, 
        ...
    }}
}}

ability should be the name of the function to call, and arguments should be a string with the arguments you would pass to the function.
Ignore any arguments which you can not infer from the prompt.
do not include any other text in your response.

output your response given the following prompt:
prompt:{prompt}
output: ", 
        abilities = Ability::list_string().join("\n"),
        prompt = command);
        if let Ok(mut stream) = self.speech.respond_stream(prompt).await {
            let mut response = String::new();
            while let Some(res) = stream.next().await {
                let responses = res.unwrap();
                for resp in responses {
                    //clear console
                    // print!("\x1B[2J\x1B[1;1H");
                    print!("{}", resp.response);
                    if let Some(thought) = self.thoughts.clone(){
                        thought.send(resp.response.clone()).await.unwrap();
                    } 
                    response.push_str(&resp.response);
                }
            }
            //regex, get everything between brackets including the brackets: \{.*?\}
            //split the response into the ability and the arguments
            if let Ok(choice) = serde_json::from_str::<Choice>(&response) {
                println!("
                    ability: {}
                    args: {}
                ", 
                format!("{}", choice.ability), 
                format!("{}", match &choice.arguments { Some(args) => args.to_string(), None => "None".to_string()})); 
                Ok(Ability::from_string(&choice.ability, choice.arguments))
            } else{
                Err(format!("Couldn't Parse output: {}", response))
            }
        }else{
            Err("Error".to_string())
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Choice {
    ability: String,
    arguments: Option<serde_json::Value>,
}
