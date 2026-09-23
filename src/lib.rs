// Generated from OpenAPI. Do not edit methods by hand.
pub const DEFAULT_BASE_URL: &str = "https://app.selltostate.com";

#[derive(Debug)]
pub struct AuthError { pub body: String }
#[derive(Debug)]
pub struct PaymentError { pub body: String, pub payment_link: Option<String>, pub plan: Option<String>, pub machine_payment: Option<String> }
#[derive(Debug)]
pub struct RateLimitError { pub body: String }
#[derive(Debug)]
pub enum ClientError { Auth(AuthError), Payment(PaymentError), RateLimit(RateLimitError), Other(String) }

pub struct SellToState {
    pub base_url: String,
    pub api_key: Option<String>,
    pub client_id: Option<String>,
}

impl SellToState {
    pub fn new(api_key: Option<String>, client_id: Option<String>, base_url: Option<String>) -> Self {
        let key = api_key.or_else(|| std::env::var("STS_API_KEY").ok());
        let url = base_url
            .or_else(|| std::env::var("STS_API_URL").ok())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        Self { base_url: url.trim_end_matches('/').to_string(), api_key: key, client_id }
    }

    pub fn request(&self, method: &str, path: &str, args: &std::collections::HashMap<String, String>, path_params: &[&str]) -> Result<String, ClientError> {
        let mut url_path = path.to_string();
        for name in path_params {
            let value = args.get(*name).ok_or_else(|| ClientError::Other(format!("missing path param: {name}")))?;
            url_path = url_path.replace(&format!("{{{name}}}"), value);
        }
        let mut url = format!("{}{}", self.base_url, url_path);
        let mut first = true;
        for (key, value) in args {
            if path_params.contains(&key.as_str()) || key == "body" || value.is_empty() { continue; }
            url.push(if first { '?' } else { '&' });
            first = false;
            url.push_str(key);
            url.push('=');
            url.push_str(value);
        }
        let mut cmd = std::process::Command::new("curl");
        cmd.arg("--silent").arg("--show-error").arg("--write-out").arg("
%{http_code}");
        cmd.arg("-X").arg(method).arg(&url);
        cmd.arg("-H").arg("accept: application/json");
        if let Some(key) = &self.api_key {
            cmd.arg("-H").arg(format!("x-api-key: {key}"));
        } else if let Some(id) = &self.client_id {
            cmd.arg("-H").arg(format!("authorization: Bearer client:{id}"));
        }
        if method != "GET" {
            if let Some(body) = args.get("body") {
                cmd.arg("-H").arg("content-type: application/json").arg("--data").arg(body);
            }
        }
        let out = cmd.output().map_err(|e| ClientError::Other(e.to_string()))?;
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        let (body, code) = split_status(&text);
        let status: u16 = code.trim().parse().unwrap_or(0);
        if status == 401 { return Err(ClientError::Auth(AuthError { body })); }
        if status == 402 {
            return Err(ClientError::Payment(PaymentError {
                payment_link: json_string(&body, "payment_link"),
                plan: json_string(&body, "plan"),
                machine_payment: json_string(&body, "machine_payment"),
                body,
            }));
        }
        if status == 429 { return Err(ClientError::RateLimit(RateLimitError { body })); }
        if status >= 400 || !out.status.success() { return Err(ClientError::Other(body)); }
        Ok(body)
    }

    pub fn searchAwards(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/awards", args, &[])
    }

    pub fn exportAwardsCsv(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/awards.csv", args, &[])
    }

    pub fn getPrice(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/price", args, &[])
    }

    pub fn payAutonomously(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("POST", "/api/v1/pay", args, &[])
    }

    pub fn registerClient(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("POST", "/api/oidc/register", args, &[])
    }

    pub fn mcp(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("POST", "/api/mcp", args, &[])
    }

    pub fn protectedResourceMetadata(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/.well-known/oauth-protected-resource", args, &[])
    }

    pub fn authorizationServerMetadata(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/.well-known/oauth-authorization-server", args, &[])
    }

    pub fn whoami(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/whoami", args, &[])
    }

    pub fn search(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/search", args, &[])
    }

    pub fn listBuyers(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/buyers", args, &[])
    }

    pub fn getBuyer(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/buyers/{slug}", args, &["slug"])
    }

    pub fn listSuppliers(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/suppliers", args, &[])
    }

    pub fn getSupplier(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/suppliers/{slug}", args, &["slug"])
    }

    pub fn listSectors(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/sectors", args, &[])
    }

    pub fn getSector(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/sectors/{slug}", args, &["slug"])
    }

    pub fn getTender(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/tenders/{id}", args, &["id"])
    }

    pub fn getLists(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/lists", args, &[])
    }

    pub fn createList(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("POST", "/api/v1/lists", args, &[])
    }

    pub fn getList(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/lists/{id}", args, &["id"])
    }

    pub fn renameList(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("PATCH", "/api/v1/lists/{id}", args, &["id"])
    }

    pub fn deleteList(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("DELETE", "/api/v1/lists/{id}", args, &["id"])
    }

    pub fn addToList(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("POST", "/api/v1/lists/{id}/items", args, &["id"])
    }

    pub fn removeFromList(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("DELETE", "/api/v1/lists/{id}/items", args, &["id"])
    }

    pub fn getSavedSearches(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/saved-searches", args, &[])
    }

    pub fn saveSearch(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("POST", "/api/v1/saved-searches", args, &[])
    }

    pub fn getSavedSearch(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("GET", "/api/v1/saved-searches/{id}", args, &["id"])
    }

    pub fn renameSavedSearch(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("PATCH", "/api/v1/saved-searches/{id}", args, &["id"])
    }

    pub fn deleteSavedSearch(&self, args: &std::collections::HashMap<String, String>) -> Result<String, ClientError> {
        self.request("DELETE", "/api/v1/saved-searches/{id}", args, &["id"])
    }
}

fn split_status(text: &str) -> (String, String) {
    if let Some(i) = text.rfind('
') {
        (text[..i].to_string(), text[i + 1..].to_string())
    } else {
        (text.to_string(), String::new())
    }
}

fn json_string(json: &str, name: &str) -> Option<String> {
    let needle = format!(""{name}":"");
    let at = json.find(&needle)?;
    let start = at + needle.len();
    let end = json[start..].find('"')? + start;
    Some(json[start..end].to_string())
}
