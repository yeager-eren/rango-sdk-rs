#[derive(Debug)]
pub enum SdkErr {
    UreqErr(ureq::Error),
    UrlSerializeErr(serde_urlencoded::ser::Error),
    Err(std::io::Error),
}

impl From<ureq::Error> for SdkErr {
    fn from(value: ureq::Error) -> Self {
        SdkErr::UreqErr(value)
    }
}

impl From<serde_urlencoded::ser::Error> for SdkErr {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        SdkErr::UrlSerializeErr(value)
    }
}

impl From<std::io::Error> for SdkErr {
    fn from(value: std::io::Error) -> Self {
        SdkErr::Err(value)
    }
}
