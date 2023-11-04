use std::io::Result;

pub fn get_domain(url: &str) -> Result<&str> {
    let protocol_delimeter_index = url.find("://").map(|x| { x + 3 }).or(Some(usize::MIN));
    let user_delimiter_index = url.find("@").map(|x| { x + 1 }).or(Some(usize::MIN));
    let uri_start = protocol_delimeter_index.max(user_delimiter_index)
        .or(Some(0))
        .unwrap();

    let uri = &url[uri_start..];

    let port_delimiter_index = uri.find(":").or(Some(usize::MAX));
    let path_delimiter_index = uri.find("/").or(Some(usize::MAX));
    let domain_end = port_delimiter_index.min(path_delimiter_index)
        .or(Some(uri.len()))
        .unwrap();

    let domain = &uri[..domain_end];
    Ok(domain)
}


#[cfg(test)]
mod tests {
    use super::get_domain;

    #[test]
    fn gets_domain_correctly() {
        assert_eq!(get_domain("git@github.com:user/repo.git").unwrap(), ("github.com"));
        assert_eq!(get_domain("ssh://git@github.com/project/repo.git").unwrap(), "github.com");
    }
}

