extern crate rustc_serialize;

#[derive(RustcDecodable, Debug)]
pub struct Pager {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(RustcDecodable, Debug)]
pub struct PDNSResult {
    pub recordHash: Option<String>,
    pub resolve: Option<String>,
    pub value: Option<String>,
    pub source: Option<Vec<String>>,
    pub lastSeen: Option<String>,
    pub firstSeen: Option<String>,
    pub collected: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct PDNSResponse {
    pub totalRecords: u32,
    pub queryValue: Option<String>,
    pub queryType: Option<String>,
    pub firstSeen: Option<String>,
    pub lastSeen: Option<String>,
    pub results: Option<Vec<PDNSResult>>,
    pub pager: Option<Pager>,
}

#[derive(RustcDecodable, Debug)]
pub struct WhoisResponse {
    pub contactEmail: Option<String>,
    pub domain: Option<String>,
    pub billing: Option<Registrant>,
    pub zone: Option<Registrant>,
    pub nameServers: Option<Vec<String>>,
    pub registered: Option<String>,
    pub lastLoadedAt: Option<String>,
    pub whoisServer: Option<String>,
    pub registryUpdatedAt: Option<String>,
    pub admin: Option<Registrant>,
    pub expiresAt: Option<String>,
    pub registrar: Option<String>,
    pub tech: Option<Registrant>,
    pub registrant: Option<Registrant>,
}

#[derive(RustcDecodable, Debug)]
pub struct Registrant {
    pub city: Option<String>,
    pub name: Option<String>,
    pub country: Option<String>,
    pub telephone: Option<String>,
    pub state: Option<String>,
    pub street: Option<String>,
    pub postalCode: Option<String>,
    pub organization: Option<String>,
    pub email: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct SSLCertResponse {
    pub results: Option<Vec<SSLCertResults>>,
}

#[derive(RustcDecodable, Debug)]
pub struct SSLCertResults {
    pub sha1: Option<String>,
    pub ipAddresses: Option<Vec<String>>,
    pub firstSeen: Option<String>,
    pub lastSeen: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct OSINTResponse {
    pub results: Option<Vec<OSINTResults>>,
}

#[derive(RustcDecodable, Debug)]
pub struct OSINTResults {
    pub source: Option<String>,
    pub sourceUrl: Option<String>,
    pub inReport: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}
