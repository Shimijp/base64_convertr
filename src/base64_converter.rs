use crate::errors::Errors;
use crate::utils::{DECODE_TABLE, Bases, Padding, get_padding, is_legal_base64_string};


 pub(crate) struct Base64Converter {
    chars: Vec<u8>,
    base: Bases,
    padding: Padding,
}

impl Base64Converter {
    pub fn new(original: &str,mew_base: Bases ) -> Result<Self, Errors> {
        let chars = original.as_bytes();
        match  is_legal_base64_string(original){
            true => Ok(Base64Converter{
                chars: chars.to_vec(),
                base: mew_base,
                padding: get_padding(original),
            }),
            false => Err(Errors::IllegalBase64String),

        }
    }
    pub fn convert(&self, input: &str) -> String {
        //TODO
        let mut string = String::new();

        match  self.base {

            Bases::Ascii => string = convert_base64_to_ascii(&self.chars, &self.padding),
            _ => string.push_str(""),

        }
        string
    }
}


/// I know this is very wet (not DRY) and I am sorry,
fn convert_base64_to_ascii(bytes: &Vec<u8>, padding: &Padding) -> String {
    if bytes.is_empty()    {
        return "".to_string();
    }
    let (body, tail) = bytes.split_at(bytes.len() - 4);
    let mut body_str = String::with_capacity(bytes.len() * 3 / 4);



    for chunk in body.chunks(4) {
        let c1 = DECODE_TABLE[chunk[0] as usize];
        let c2 = DECODE_TABLE[chunk[1] as usize];
        let c3 = DECODE_TABLE[chunk[2] as usize];
        let c4 = DECODE_TABLE[chunk[3] as usize];
        let b1 = (c1 << 2) | (c2 >> 4);
        let b2 = ((c2 & 0x0F) << 4) | (c3 >> 2);
        let b3 = ((c3 & 0x03) << 6) | c4;
        body_str.push(b1 as char);
        body_str.push(b2 as char);
        body_str.push(b3 as char);

    }
    match padding {

        Padding::Zero =>
            {
                let c1 = DECODE_TABLE[tail[0] as usize];
                let c2 = DECODE_TABLE[tail[1] as usize];
                let c3 = DECODE_TABLE[tail[2] as usize];
                let c4 = DECODE_TABLE[tail[3] as usize];
                let b1 = (c1 << 2) | (c2 >> 4);
                let b2 = ((c2 & 0x0F) << 4) | (c3 >> 2);
                let b3 = ((c3 & 0x03) << 6) | c4;
                body_str.push(b1 as char);
                body_str.push(b2 as char);
                body_str.push(b3 as char);
            }
        Padding::OneByte =>
        {
            let c1 = DECODE_TABLE[tail[0] as usize];
            let c2 = DECODE_TABLE[tail[1] as usize];
            let c3 = DECODE_TABLE[tail[2] as usize];
            let b1 = (c1 << 2) | (c2 >> 4);
            let b2 = ((c2 & 0x0F) << 4) | (c3 >> 2);

            body_str.push(b1 as char);
            body_str.push(b2 as char);
        }
        Padding::TwoBytes =>
        {
            let c1 = DECODE_TABLE[tail[0] as usize];
            let c2 = DECODE_TABLE[tail[1] as usize];
            let b1 = (c1 << 2) | (c2 >> 4);
            body_str.push(b1 as char);
        }
    }

    body_str
}

