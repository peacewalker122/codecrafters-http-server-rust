use anyhow::Ok;

#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub directory: Option<String>,
}

pub fn parse(mut args: impl Iterator<Item = String>) -> anyhow::Result<Config> {
    let mut res = Config {
        url: String::new(),
        directory: None,
    };
    while let Some(flag) = args.next() {
        match flag {
            a if a.eq("-p") => {
                // port
                if let Some(val) = args.next() {
                    res.url = format!("0.0.0.0:{val}")
                }
            }
            a if a.eq("--directory") => {
                // directory path
                if let Some(val) = args.next() {
                    res.directory = Some(val)
                }
            }
            _ => {}
        }
    }

    if res.url.len() == 0 {
        res.url = String::from("0.0.0.0:3300")
    }

    Ok(res)
}
