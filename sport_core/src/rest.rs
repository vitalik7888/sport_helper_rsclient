use async_trait::async_trait;
use log::{info, warn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::{Debug, Display};
use thiserror::Error;

use sport_core_db::entity::{Exercise, Person, ID};

pub type Result<T> = std::result::Result<T, RestClientError>;
pub type PCL = Box<dyn RestEntityClient<Person, Person>>;
pub type ECL = Box<dyn RestEntityClient<Exercise, Exercise>>;

pub struct Client {
    person: PCL,
    exercise: ECL,
}

impl Client {
    pub fn new(pcl: PCL, ecl: ECL) -> Self {
        Self {
            person: pcl,
            exercise: ecl,
        }
    }

    pub fn default(host: &str) -> Self {
        Self {
            person: Box::new(PersonRestClient::new(host)),
            exercise: Box::new(ExerciseRestClient::new(host)),
        }
    }

    pub fn person(&self) -> &PCL {
        &self.person
    }

    pub fn exercise(&self) -> &ECL {
        &self.exercise
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerMessage {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}

impl Display for ServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type: {}, message: {}", self.error_type, self.message)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerError {
    #[serde(rename = "type")]
    error_type: String,
    error: String,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type: {}, error: {}", self.error_type, self.error)
    }
}

#[derive(Error, Debug)]
pub enum RestClientError {
    #[error("parsing error: `{0}`")]
    Parse(#[from] serde_json::Error),
    #[error("request error: `{0}`")]
    Request(#[from] reqwest::Error),
    #[error("server message: `{0}`")]
    ServerMessage(ServerMessage), // TODO check status
    #[error("server error: `{0}`")]
    ServerError(ServerError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerOperationStatus {
    status: String,
}

#[async_trait]
pub trait RestEntityClient<T, B>: Sync + Send
where
T: DeserializeOwned,
B: Serialize + Sync,
{
    async fn get_one(&self, id: ID) -> Result<T> {
        let res = self
            .request_client()
            .get(&format!("{}/{}", self.path(), id))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                info!("Request GET one: Ok");
                Ok(res.json().await?)
            }
            _ => {
                warn!("Request GET one: server message");
                Err(RestClientError::ServerError(res.json().await?))
            }
        }
    }

    async fn get_all(&self) -> Result<Vec<T>> {
        let res = self
            .request_client()
            .get(self.path())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                info!("Request GET all: Ok");
                Ok(res.json().await?)
            }
            _ => {
                warn!("Request GET all: server error");
                Ok(res.json().await?)
            }
        }
    }

    async fn insert(&self, body: &B) -> Result<T> {
        let res = self
            .request_client()
            .post(self.path())
            .body(serde_json::to_string(body)?)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                info!("Request POST insert: Ok");
                Ok(res.json().await?)
            }
            _ => {
                warn!("Request POST insert: server error");
                Ok(res.json().await?)
            }
        }
    }

    async fn update(&self, body: &B, id: ID) -> Result<ServerOperationStatus> {
        // FIXME B without id
        let res = self
            .request_client()
            .put(format!("{}/{}", self.path(), id))
            .body(serde_json::to_string(body)?)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                info!("Request PUT update: Ok");
                Ok(res.json().await?)
            }
            _ => {
                warn!("Request PUT update: server error");
                Ok(res.json().await?)
            }
        }
    }

    async fn remove(&self, id: ID) -> Result<ServerOperationStatus> {
        let res = self
            .request_client()
            .delete(&format!("{}/{}", self.path(), id))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                info!("Request DELETE one: Ok");
                Ok(res.json().await?)
            }
            _ => {
                warn!("Request DELETE one: server error");
                Ok(res.json().await?)
            }
        }
    }

    fn path(&self) -> &str;
    fn request_client(&self) -> &reqwest::Client;
}

struct ExerciseRestClient {
    req_client: reqwest::Client,
    path: String,
}

impl ExerciseRestClient {
    pub fn new(host: &str) -> Self {
        Self {
            req_client: reqwest::Client::new(),
            path: format!("{}/exercises", host),
        }
    }
}

#[async_trait]
impl RestEntityClient<Exercise, Exercise> for ExerciseRestClient {
    fn path(&self) -> &str {
        &self.path
    }

    fn request_client(&self) -> &reqwest::Client {
        &self.req_client
    }
}

struct PersonRestClient {
    req_client: reqwest::Client,
    path: String,
}

impl PersonRestClient {
    pub fn new(host: &str) -> Self {
        Self {
            req_client: reqwest::Client::new(),
            path: format!("{}/persons", host),
        }
    }
}

#[async_trait]
impl RestEntityClient<Person, Person> for PersonRestClient {
    fn path(&self) -> &str {
        &self.path
    }

    fn request_client(&self) -> &reqwest::Client {
        &self.req_client
    }
}
