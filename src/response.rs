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
    pub issuerStreetAddress: Option<String>,
    pub subjectSerialNumber: Option<String>,
    pub subjectEmailAddress: Option<String>,
    pub expirationDate: Option<String>,
    pub issuerSerialNumber: Option<String>,
    pub issuerOrganizationName: Option<String>,
    pub subjectCommonName: Option<String>,
    pub subjectSurname: Option<String>,
    pub subjectCountry: Option<String>,
    pub subjectGivenName: Option<String>,
    pub issuerProvince: Option<String>,
    pub subjectLocalityName: Option<String>,
    pub issuerStateOrProvinceName: Option<String>,
    pub issuerCommonName: Option<String>,
    pub issueDate: Option<String>,
    pub issuerEmailAddress: Option<String>,
    pub subjectOrganizationUnitName: Option<String>,
    pub subjectOrganizationName: Option<String>,
    pub fingerprint: Option<String>,
    pub issuerLocalityName: Option<String>,
    pub issuerGivenName: Option<String>,
    pub issuerCountry: Option<String>,
    pub subjectStateOrProvinceName: Option<String>,
    pub sha1: Option<String>,
    pub sslVersion: Option<String>,
    pub issuerSurname: Option<String>,
    pub serialNumber: Option<String>,
    pub subjectStreetAddress: Option<String>,
    pub issuerOrganizationUnitName: Option<String>,
    pub subjectProvince: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct SSLCertHistoryResponse {
    pub results: Option<Vec<SSLCertHistoryResult>>,
}

#[derive(RustcDecodable, Debug)]
pub struct SSLCertHistoryResult {
    pub sha1: Option<String>,
    pub ipAddresses: Option<Vec<String>>,
    pub firstSeen: Option<String>,
    pub lastSeen: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct OSINTResponse {
    pub results: Option<Vec<OSINTResult>>,
}

#[derive(RustcDecodable, Debug)]
pub struct OSINTResult {
    pub source: Option<String>,
    pub sourceUrl: Option<String>,
    pub inReport: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(RustcDecodable, Debug)]
pub struct MalwareResponse {
    pub results: Option<Vec<MalwareResult>>,
}

#[derive(RustcDecodable, Debug)]
pub struct MalwareResult {
    pub sample: Option<String>,
    pub source: Option<String>,
    pub sourceUrl: Option<String>,
    pub collectionDate: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct SubdomainsResponse {
    pub queryValue: Option<String>,
    pub subdomains: Option<Vec<String>>,
}

#[derive(RustcDecodable, Debug)]
pub struct AccountResponse {
    pub username: Option<String>,
    pub firstActive: Option<String>,
    pub firstName: Option<String>,
    pub lastName: Option<String>,
    pub lastActive: Option<String>,
    pub organization: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct HostAttributeComponentResponse {
    pub results: Option<Vec<HostAttributeComponentResult>>,
}

#[derive(RustcDecodable, Debug)]
pub struct HostAttributeComponentResult {
    pub category: Option<String>,
    pub hostname: Option<String>,
    pub lastSeen: Option<String>,
    pub firstSeen: Option<String>,
    pub label: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct HostAttributeTrackerResponse {
    pub results: Option<Vec<HostAttributeTrackerResult>>,
}

#[derive(RustcDecodable, Debug)]
pub struct HostAttributeTrackerResult {
    pub attributeValue: Option<String>,
    pub hostname: Option<String>,
    pub lastSeen: Option<String>,
    pub firstSeen: Option<String>,
    pub attributeType: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct PDNSUniqueResponse {
    pub frequency: Option<Vec<(String, i32)>>,
    pub queryType: Option<String>,
    pub total: Option<i32>,
    pub pager: Option<Pager>,
    pub queryValue: Option<String>,
    pub results: Option<Vec<String>>,
}
