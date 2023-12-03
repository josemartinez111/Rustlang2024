// FILE: ./lib.rs
// ___________________________________________________________

pub mod utils {
  pub mod error;
  pub mod utilities;
}
// _______________________________________________

pub mod web {
  pub mod handlers {
    pub mod health_check;
  }

  pub mod auth {
    pub mod routes_login;
  }

  pub mod routes;
  pub mod run_server;
}

// ___________________________________________________________

pub mod models {
  pub mod health_check;
}
// ___________________________________________________________
