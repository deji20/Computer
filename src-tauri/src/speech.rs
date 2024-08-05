use ollama_rs::{error::OllamaError, generation::completion::{request::GenerationRequest, GenerationResponse, GenerationResponseStream}, Ollama};
use tokio_stream::{Stream, StreamExt};

pub struct Speech{
    model: String,
    ollama: Ollama
} 

impl Clone for Speech {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
            ollama: self.ollama.clone()
        }
    }
}

impl Default for Speech {
    fn default() -> Self {
        Self {
            model: "phi3".to_string(),
            ollama: Ollama::default()
        }
    }
}

impl Speech {
    pub fn new(model: String) -> Self {
        Self {
            model,
            ollama: Ollama::default()
        }
    }
    pub async fn respond(&self, question: String) -> Result<String, Box<dyn std::error::Error>> {
        let request = GenerationRequest::new(self.model.clone(), question);
        let mut stream = self.ollama
            .generate_stream(request)
            .await?;
        let mut response = String::new();
        while let Some(res) = stream.next().await {
            let responses = res.unwrap();
            for resp in responses {
                //clear console
                // print!("\x1B[2J\x1B[1;1H");
                println!("{}", response);
                response.push_str(&resp.response);
            }
        }
        Ok(response)
    }

    pub async fn respond_stream(&self, question: String) -> Result<GenerationResponseStream, OllamaError> {
        let request = GenerationRequest::new(self.model.clone(), question);
        let stream = self.ollama.generate_stream(request).await;
        stream
    }
}