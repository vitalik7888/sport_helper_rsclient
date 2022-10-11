use crate::{rest::{self, RestClientError}, config::{KeyMap, Config}};
use sport_core_db::{
    entity,
    model::{self, Table},
};
use std::{rc::Rc, cell::{RefCell, Ref}};

use log::{info, error};
use tokio::runtime::Runtime;

pub type Result<T> = std::result::Result<T, RestClientError>;

pub struct Controller {
    db: Rc<RefCell<model::Db>>,
    exercises: ControllerExercises,
    person: ControllerPerson,
    key_map: KeyMap,
    cfg: Config,
}

impl Controller {
    pub fn new(host: &str, cfg: Config, key_map: KeyMap) -> Self { 
        let rt = Rc::new(tokio::runtime::Builder::new_current_thread()
                         .enable_all().build().expect("Can`t create client runtime"));
        let db = Rc::new(RefCell::new(model::Db::default()));

        Self { 
            db: db.clone(),
            exercises: ControllerExercises::new(host, rt.clone(), db.clone()),
            person: ControllerPerson::new(host, rt, db),
            cfg,
            key_map,
        } 
    }

    pub fn cfg(&self) -> &Config {
        &self.cfg
    }

    pub fn key_map(&self) -> &KeyMap {
        &self.key_map
    }

    pub fn db(&self) -> Ref<model::Db> {
        Ref::map(self.db.borrow(), |db| db)
    }

    pub fn exercises(&self) -> &ControllerExercises {
        &self.exercises
    }

    pub fn person(&self) -> &ControllerPerson {
        &self.person
    }
}

pub struct ControllerExercises {
    rt: Rc<Runtime>,
    client: rest::Client,
    db: Rc<RefCell<model::Db>>,
}

impl ControllerExercises {
    pub fn new(host: &str, rt: Rc<Runtime>, db: Rc<RefCell<model::Db>>) -> Self {
        Self { 
            rt,
            client: rest::Client::default(&host),
            db,
        }
    }

    /// Get exercise from server
    pub fn load_one(&self, id: entity::ID) {
        match self.rt.block_on(self.client.exercise().get_one(id)) {
            Ok(data) => {
                info!("GET one exercise: {:#?}", data);
                self.db.borrow_mut().exercises_mut().insert(data);
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    /// Get exercises from server
    pub fn load_all(&self) -> Result<()> {
        match self.rt.block_on(self.client.exercise().get_all()) {
            Ok(data) => {
                info!("GET exercises: {:#?}", data);
                data.into_iter()
                    .for_each(|e| {self.db.borrow_mut().exercises_mut().insert(e);});
                Ok(())
            }
            Err(err) => {
                self.db.borrow_mut().exercises_mut().clear(); // clear previous
                error!("{}", err);
                Err(err)
            }
        }
    }

    /// Insert exercise on server
    pub fn insert(&self, data: &entity::Exercise) -> Result<()> {
        match self.rt.block_on(self.client.exercise().insert(&data)) {
            Ok(inserted) => {
                info!("Inserted exercise: {:#?}", inserted);
                self.db.borrow_mut().exercises_mut().insert(inserted);
                Ok(())
            }
            Err(err) => {
                error!("{}", err);
                Err(err)
            }
        }
    }

    /// Update exercise on server
    pub fn update(&self, data: entity::Exercise) -> Result<()> {
        match self.rt.block_on(self.client.exercise().update(&data, data.id)) {
            Ok(updated) => {
                info!("Updated exercise: {:#?}", updated);
                self.db.borrow_mut().exercises_mut().update(data);
                Ok(())
            }
            Err(err) => {
                error!("{}", err);
                Err(err)
            }
        }
    }

    /// Removes exercise from server
    pub fn remove_exercise(&self, id: entity::ID) -> Result<()> {
        match self.rt.block_on(self.client.exercise().remove(id)) {
            Ok(data) => {
                info!("Removed exercise: {:#?}", data);
                self.db.borrow_mut().exercises_mut().remove(id);
                Ok(())
            }
            Err(err) => {
                error!("{}", err);
                Err(err)
            }
        }
    }
}

pub struct ControllerPerson {
    rt: Rc<Runtime>,
    client: rest::Client,
    db: Rc<RefCell<model::Db>>,
}

impl ControllerPerson {
    pub fn new(host: &str, rt: Rc<Runtime>, db: Rc<RefCell<model::Db>>) -> Self {
        Self { 
            rt,
            client: rest::Client::default(&host),
            db,
        }
    }

    /// Get account from server
    pub fn load_one(&self, id: entity::ID) {
        match self.rt.block_on(self.client.person().get_one(id)) {
            Ok(data) => {
                info!("GET one person: {:#?}", data);
                self.db.borrow_mut().persons_mut().insert(data);
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    /// Insert person on server
    pub fn insert(&self, data: &entity::Person) -> Result<()> {
        match self.rt.block_on(self.client.person().insert(&data)) {
            Ok(inserted) => {
                info!("Inserted person: {:#?}", inserted);
                self.db.borrow_mut().persons_mut().insert(inserted);
                Ok(())
            }
            Err(err) => {
                error!("{}", err);
                Err(err)
            }
        }
    }

    /// Update person on server
    pub fn update(&self, data: entity::Person) -> Result<()> {
        match self.rt.block_on(self.client.person().update(&data, data.id)) {
            Ok(updated) => {
                info!("Updated person: {:#?}", updated);
                self.db.borrow_mut().persons_mut().update(data);
                Ok(())
            }
            Err(err) => {
                error!("{}", err);
                Err(err)
            }
        }
    }

    /// Removes person from server
    pub fn remove_person(&self, id: entity::ID) -> Result<()> {
        match self.rt.block_on(self.client.person().remove(id)) {
            Ok(data) => {
                info!("Removed person: {:#?}", data);
                self.db.borrow_mut().persons_mut().remove(id);
                Ok(())
            }
            Err(err) => {
                error!("{}", err);
                Err(err)
            }
        }
    }
}
