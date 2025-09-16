use octofer_github::GithubConfig;

pub struct ServerConfig {
    address: std::net::Ipv4Addr,
    port: u16,
}

pub struct Config {
    gh_config: GithubConfig,
    server_config: ServerConfig,
}
