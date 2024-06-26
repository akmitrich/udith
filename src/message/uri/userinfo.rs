use nom::{
    bytes::complete::{tag, take_while},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct UserInfo {
    pub user: String,
    pub password: Option<String>,
}

impl UserInfo {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        // userinfo = ( user / telephone-subscriber ) [ ":" password ] "@"
        map(
            tuple((parse_user, parse_password, tag(b"@"))),
            |(user, password, _)| Self { user, password },
        )(src)
    }
}

impl ToString for UserInfo {
    fn to_string(&self) -> String {
        let pass = if let Some(ref pass) = self.password {
            format!(":{}", pass)
        } else {
            String::new()
        };
        format!("{}{}", self.user, pass)
    }
}

fn parse_user(src: &[u8]) -> IResult<&[u8], String> {
    let (rest, user_bytes) = take_while(|x: u8| !b":@".contains(&x) && x.is_ascii_graphic())(src)?;
    if user_bytes.is_empty() {
        Err(nom::Err::Error(nom::error::make_error(
            src,
            nom::error::ErrorKind::Fail,
        )))
    } else {
        Ok((rest, std::str::from_utf8(user_bytes).unwrap().to_owned()))
    }
}

fn parse_password(src: &[u8]) -> IResult<&[u8], Option<String>> {
    let Ok((rest, _)) = tag::<_, _, ()>(b":")(src) else {
        return Ok((src, None));
    };
    let (rest, password_bytes) =
        take_while(|x: u8| !b"@".contains(&x) && x.is_ascii_graphic())(rest)?;
    Ok((
        rest,
        std::str::from_utf8(password_bytes)
            .map(ToOwned::to_owned)
            .ok(),
    ))
}
