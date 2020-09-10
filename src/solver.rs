use byteorder::{BigEndian, ByteOrder, LittleEndian};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;

static SCREEN_RESOLUTIONS: &'static [(usize, usize)] = &[(375, 812), (414, 896)];
static APPLICATION_MANIFEST: &'static str = "2.AdVcYX_6cIB15rBXSWb1pUb9aa48my0Doxp30jgW63NLyRNKh1eelG9GwMxRC2ups8te4VstDQrvm3zRsnlWCQ0qNDLGNb3viRDYQ_NkyJA0RrXYukm3_fXoe8Ca3-TLKK6oa-Etd1p7_z2yc9t9-bIlpZ5dMX366JK91R7S3g==";
static COUNTRY_LANGUAGES: &'static [(&str, &str)] = &[
    ("GB", "en"),
    ("US", "en"),
    ("DE", "de"),
    ("FR", "fr"),
    ("ES", "es"),
    ("GR", "el"),
    ("IE", "en"),
    ("IT", "it"),
    ("SE", "sv"),
];

pub struct Solver<'a> {
    session: &'a str,
    question: &'a str,
    device: Device<'a>,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device<'a> {
    bundle_identifier: &'a str,
    bundle_version: &'a str,
    country_code: &'a str,
    debugger: bool,
    device: Screen,
    ip_addresses: HashMap<&'a str, String>,
    is_jail_broken: bool,
    is_simulator: bool,
    language_code: &'a str,
    model: &'a str,
    name: &'a str,
    uuid: String,
    vendor_id: String,
    version: &'a str,
}

#[derive(Serialize, Debug, Default)]
pub struct Screen {
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Screen { width, height }
    }
}

#[derive(Serialize, Debug)]
pub struct Solution<'a> {
    answer: &'a str,
    fingerprint: &'a Device<'a>,
}
#[derive(Serialize, Debug)]
pub struct InitialPayload<'a> {
    application_manifest: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    old_token: Option<&'a str>,
}

#[derive(Serialize, Debug)]
pub struct SolutionPayload<'a> {
    answer: &'a str,
    session: &'a str,
}

impl<'a> Device<'a> {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ip_addresses: HashMap<&'a str, String> = HashMap::new();
        ip_addresses.insert(
            "awdl0/ipv6",
            format!(
                "fe80::{:x}:{:x}:{:x}:{:x}",
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>()
            ),
        );
        ip_addresses.insert(
            "en0/ipv4",
            format!("192.168.{}.{}", rng.gen_range(0, 2), rng.gen_range(1, 255)),
        );
        ip_addresses.insert(
            "en0/ipv6",
            format!(
                "2a00:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>()
            ),
        );
        ip_addresses.insert("lo0/ipv4", "127.0.0.1".to_string()); // static
        ip_addresses.insert("lo0/ipv6", "fe80::1".to_string()); // static
        ip_addresses.insert(
            "utun0/ipv6",
            format!(
                "fe80::{:x}:{:x}:{:x}:{:x}",
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>()
            ),
        );
        ip_addresses.insert(
            "utun1/ipv6",
            format!(
                "fe80::{:x}:{:x}:{:x}:{:x}",
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>()
            ),
        );


        let (width, height) = SCREEN_RESOLUTIONS.choose(&mut rng).unwrap();
        let (country_code, language_code) = COUNTRY_LANGUAGES.choose(&mut rng).unwrap();

        let device_uuid = uuid::Uuid::new_v4().to_string();
        let vendor_id = uuid::Uuid::new_v4().to_string();

        Device {
            bundle_identifier: "com.ashworthandparker.end",
            bundle_version: "3.6.3",
            country_code,
            debugger: false,
            device: Screen::new(*width, *height),
            ip_addresses,
            is_jail_broken: false,
            is_simulator: false,
            language_code,
            model: "iPhone",
            name: "",
            uuid: device_uuid,
            vendor_id: vendor_id,
            version: "13.5",
        }
    }

    pub fn xor_value(&self) -> usize {
        let mut xor = 0x1EAF;

        for property in &[
            self.bundle_identifier,
            self.bundle_version,
            self.country_code,
            self.language_code,
            self.model,
            self.name,
            &self.vendor_id,
            self.version,
        ] {
            xor += calculate_sum(property)
        }

        xor += &self.device.height + &self.device.width;
        xor += self.debugger as usize + self.is_jail_broken as usize + self.is_simulator as usize;

        for (key, value) in &self.ip_addresses {
            xor += calculate_sum(key);
            xor += calculate_sum(value);
        }

        xor
    }
}

impl<'a> Solver<'a> {
    pub fn new(question: &'a str, session: &'a str) -> Self {
        Solver {
            session,
            question,
            device: Device::new(),
        }
    }

    pub fn solve(&self) -> Result<String, serde_json::Error> {
        let mut answer = self.question.as_bytes().to_vec();
        let length = answer.len();
        let xor_value = self.device.xor_value();
        for i in (0..length).rev() {
            let mut counter = i + 1;
            let mut value = xor_value ^ answer[i] as usize;
            while value != 0 {
                let u_2: usize = (counter & 0xffffffff) / length;
                let idx = (counter & 0xffffffff) - u_2 * length;
                answer[idx] += value as u8;
                value = value >> 2;
                counter += 1;
            }
        }

        let answer = base64::encode_config(answer, base64::URL_SAFE_NO_PAD);

        let solution = Solution {
            answer: &answer,
            fingerprint: &self.device,
        };

        let serialised = serde_json::to_string(&solution)?;
        let answer = base64::encode_config(&serialised, base64::STANDARD);
        let payload = SolutionPayload {
            answer: &answer,
            session: self.session,
        };

        Ok(encode_payload(&serde_json::to_string(&payload)?))
    }
}

fn calculate_sum(string: &str) -> usize {
    0x1F715
        + string
            .as_bytes()
            .iter()
            .fold(0, |acc, byte| acc + *byte as usize)
}

pub fn encode_payload(raw_value: &str) -> String {
    let raw_payload = raw_value.as_bytes();
    let length: u64 = raw_payload.len() as u64;
    let mut payload: Vec<u8> = vec![];
    let mut chunk: u64 = rand::random::<u32>().into();
    let mut buf = [0; 8];
    BigEndian::write_u64(&mut buf, chunk * 0x4678d05);
    payload.extend_from_slice(&buf[4..8]);
    let random_xor = length * 0x4678d05 as u64 ^ chunk as u64;
    let mut buf = [0; 8];
    BigEndian::write_u64(&mut buf, random_xor);
    payload.extend_from_slice(&buf[4..8]);

    let mut uvar1 = 0;
    let mut lvar7 = 0;
    let mut uvar8 = 4;
    let mut uvar10 = 0;

    while uvar1 < length {
        let mut uvar2 = uvar8;
        if length <= uvar8 {
            uvar2 = length;
        }
        uvar1 = uvar10 + 4;
        let section = &raw_payload[uvar10 as usize..(uvar10 + uvar2 + lvar7) as usize];
        let section_value = LittleEndian::read_uint(&section, section.len());
        let temp_val =
            section_value & 0xffffffff00000000 | section_value * 0x4678d05 ^ chunk as u64;
        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, temp_val);
        payload.extend_from_slice(&buf[4..8]);
        uvar8 += 4;
        lvar7 -= 4;
        uvar10 = uvar1;
        chunk = section_value;
    }

    format!(
        "2.{}",
        base64::encode_config(&payload, base64::URL_SAFE_NO_PAD)
    )
}

pub fn initial_payload(old_token: Option<&str>) -> Result<String, serde_json::Error> {
    let payload = InitialPayload {
        application_manifest: APPLICATION_MANIFEST,
        old_token,
    };

    let raw_value = serde_json::to_string(&payload)?;

    Ok(encode_payload(&raw_value))
}
