// use serde::Deserialize;
// use reqwest::Error;

// #[derive(Deserialize, Debug)]
// struct User {
//     login: String,
//     id: u32,
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
//                               owner = "rust-lang-nursery",
//                               repo = "rust-cookbook");
//     println!("{}", request_url);
//     let response = reqwest::get(&request_url).await?;

//     let users: Vec<User> = response.json().await?;
//     println!("{:?}", users);
//     Ok(())
// }




// use reqwest::Result;
// use std::time::Duration;
// use reqwest::ClientBuilder;

// #[tokio::main]
// async fn main() -> Result<()> {
//     let user = "ferris-the-crab";
//     let request_url = format!("https://api.github.com/users/{}", user);
//     println!("{}", request_url);

//     let timeout = Duration::new(5, 0);
//     let client = ClientBuilder::new().timeout(timeout).build()?;
//     let response = client.head(&request_url).send().await?;

//     if response.status().is_success() {
//         println!("{} is a user!", user);
//     } else {
//         println!("{} is not a user!", user);
//     }

//     Ok(())
// }


// use error_chain::error_chain;
// use serde::Deserialize;
// use serde_json::json;
// use std::env;
// use reqwest::Client;

// error_chain! {
//     foreign_links {
//         EnvVar(env::VarError);
//         HttpRequest(reqwest::Error);
//     }
// }

// #[derive(Deserialize, Debug)]
// struct Gist {
//     id: String,
//     html_url: String,
// }

// #[tokio::main]
// async fn main() ->  Result<()> {
//     let gh_user = env::var("GH_USER")?;
//     let gh_pass = env::var("GH_PASS")?;

//     let gist_body = json!({
//         "description": "the description for this gist",
//         "public": true,
//         "files": {
//              "main.rs": {
//              "content": r#"fn main() { println!("hello world!");}"#
//             }
//         }});

//     let request_url = "https://api.github.com/gists";
//     let response = Client::new()
//         .post(request_url)
//         .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
//         .json(&gist_body)
//         .send().await?;

//     let gist: Gist = response.json().await?;
//     println!("Created {:?}", gist);

//     let request_url = format!("{}/{}",request_url, gist.id);
//     let response = Client::new()
//         .delete(&request_url)
//         .basic_auth(gh_user, Some(gh_pass))
//         .send().await?;

//     println!("Gist {} deleted! Status code: {}",gist.id, response.status());
//     Ok(())
// }



use reqwest::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    dependencies: Vec<Dependency>,
    meta: Meta,
}

#[derive(Deserialize)]
struct Dependency {
    crate_id: String,
}

#[derive(Deserialize)]
struct Meta {
    total: u32,
}

struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
        Ok(ReverseDependencies {
               crate_id: crate_id.to_owned(),
               dependencies: vec![].into_iter(),
               client: reqwest::blocking::Client::new(),
               page: 0,
               per_page: 100,
               total: 0,
           })
    }

    fn try_next(&mut self) -> Result<Option<Dependency>> {
        if let Some(dep) = self.dependencies.next() {
            return Ok(Some(dep));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!("https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
                          self.crate_id,
                          self.page,
                          self.per_page);

        let response = self.client.get(&url).send()?.json::<ApiResponse>()?;
        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;
        Ok(self.dependencies.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependency>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn main() -> Result<()> {
    for dep in ReverseDependencies::of("serde")? {
        println!("reverse dependency: {}", dep?.crate_id);
    }
    Ok(())
}
