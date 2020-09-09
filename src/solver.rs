use byteorder::ByteOrder;
use serde::Serialize;
use std::collections::HashMap;

static APPLICATION_MANIFEST: &'static str = "2.AdVcYX_6cIB15rBXSWb1pUb9aa48my0Doxp30jgW63NLyRNKh1eelG9GwMxRC2ups8te4VstDQrvm3zRsnlWCQ0qNDLGNb3viRDYQ_NkyJA0RrXYukm3_fXoe8Ca3-TLKK6oa-Etd1p7_z2yc9t9-bIlpZ5dMX366JK91R7S3g==";

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
    ip_addresses: HashMap<&'a str, &'a str>,
    is_jail_broken: bool,
    is_simulator: bool,
    language_code: &'a str,
    model: &'a str,
    name: &'a str,
    uuid: &'a str,
    vendor_id: &'a str,
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
pub struct Payload<'a> {
    answer: &'a str,
    session: &'a str,
}

// retval: {"answer":"XEORj25my5rDRRw9WBYlPVgZBiUYDDFUY5GjbVVjWoycdDFN","fingerprint":{"uuid":"54082C20-76B9-45C6-9EDD-768A00EAC0D2","vendorId":"80F766B1-B79F-4640-AAE9-2674BEC1803B","ipAddresses":{"en0\/ipv4":"192.168.1.206","utun1\/ipv6":"fe80::b3f:48be:d51a:f521","lo0\/ipv6":"fe80::1","awdl0\/ipv6":"fe80::c4b2:9cff:fef3:63e7","en0\/ipv6":"2a00:23c5:a80:5401:89be:25d9:abb6:54d4","lo0\/ipv4":"127.0.0.1","utun0\/ipv6":"fe80::9822:a007:42a0:cde1"},"version":"13.5","bundleIdentifier":"com.ashworthandparker.end","languageCode":"en","countryCode":"GB","bundleVersion":"3.6.3","isSimulator":false,"isJailBroken":true,"debugger":false,"device":{"width":375,"height":812},"model":"iPhone","name":""}}

// ipAddresses =     {
//     "awdl0/ipv6" = "fe80::c4b2:9cff:fef3:63e7";
//     "en0/ipv4" = "192.168.1.206";
//     "en0/ipv6" = "2a00:23c5:a80:5401:2855:c97a:4ca5:4ff1";
//     "en1/ipv4" = "169.254.135.18";
//     "en1/ipv6" = "fe80::1c25:fc7c:138a:5ef7";
//     "lo0/ipv4" = "127.0.0.1";
//     "lo0/ipv6" = "fe80::1";
//     "utun0/ipv6" = "fe80::9822:a007:42a0:cde1";
//     "utun1/ipv6" = "fe80::b3f:48be:d51a:f521";
// };

// {
//     bundleIdentifier = "com.ashworthandparker.end";
//     bundleVersion = "3.6.3";
//     countryCode = GB;
//     debugger = 0;
//     device =     {
//         height = 812;
//         width = 375;
//     };
//     ipAddresses =     {
//         "awdl0/ipv6" = "fe80::c4b2:9cff:fef3:63e7";
//         "en0/ipv4" = "192.168.1.206";
//         "en0/ipv6" = "2a00:23c5:a80:5401:2855:c97a:4ca5:4ff1";
//         "en1/ipv6" = "fe80::1c25:fc7c:138a:5ef7";
//         "lo0/ipv4" = "127.0.0.1";
//         "lo0/ipv6" = "fe80::1";
//         "utun0/ipv6" = "fe80::9822:a007:42a0:cde1";
//         "utun1/ipv6" = "fe80::b3f:48be:d51a:f521";
//     };
//     isJailBroken = 1;
//     isSimulator = 0;
//     languageCode = en;
//     model = iPhone;
//     name = "";
//     uuid = "54082C20-76B9-45C6-9EDD-768A00EAC0D2";
//     vendorId = "80F766B1-B79F-4640-AAE9-2674BEC1803B";
//     version = "13.5";
// }

// {
//     bundleIdentifier = "com.ashworthandparker.end";
//     bundleVersion = "3.6.3";
//     countryCode = GB;
//     debugger = 0;
//     device =     {
//         height = 812;
//         width = 375;
//     };
//     ipAddresses =     {
//         "awdl0/ipv6" = "fe80::c4b2:9cff:fef3:63e7";
//         "en0/ipv4" = "192.168.1.206";
//         "en0/ipv6" = "2a00:23c5:a80:5401:2855:c97a:4ca5:4ff1";
//         "en1/ipv4" = "169.254.135.18";
//         "en1/ipv6" = "fe80::1c25:fc7c:138a:5ef7";
//         "lo0/ipv4" = "127.0.0.1";
//         "lo0/ipv6" = "fe80::1";
//         "utun0/ipv6" = "fe80::9822:a007:42a0:cde1";
//         "utun1/ipv6" = "fe80::b3f:48be:d51a:f521";
//     };
//     isJailBroken = 1;
//     isSimulator = 0;
//     languageCode = en;
//     model = iPhone;
//     name = "";
//     uuid = "0E3710AE-6B79-442A-A8E4-62F97C3D90BA";
//     vendorId = "80F766B1-B79F-4640-AAE9-2674BEC1803B";
//     version = "13.5";
// }

impl<'a> Device<'a> {
    pub fn new() -> Self {
        let mut ip_addresses: HashMap<&'a str, &'a str> = HashMap::new();
        ip_addresses.insert("awdl0/ipv6", "fe80::c4b2:9cff:fef3:63e7");
        ip_addresses.insert("en0/ipv4", "192.168.1.206"); // router gen
        ip_addresses.insert("en0/ipv6", "2a00:23c5:a80:5401:2855:c97a:4ca5:4ff1"); // random ipv6 gen
        ip_addresses.insert("en1/ipv4", "169.254.135.18");
        ip_addresses.insert("en1/ipv6", "fe80::1c25:fc7c:138a:5ef7");
        ip_addresses.insert("lo0/ipv4", "127.0.0.1"); // static
        ip_addresses.insert("lo0/ipv6", "fe80::1"); // static
        ip_addresses.insert("utun0/ipv6", "fe80::9822:a007:42a0:cde1");
        ip_addresses.insert("utun1/ipv6", "fe80::b3f:48be:d51a:f521");

        Device {
            bundle_identifier: "com.ashworthandparker.end",
            bundle_version: "3.6.3",
            country_code: "GB",
            debugger: false,
            device: Screen::new(812, 375),
            ip_addresses,
            is_jail_broken: false,
            is_simulator: false,
            language_code: "en",
            model: "iPhone",
            name: "",
            uuid: "0E3710AE-6B79-442A-A8E4-62F97C3D90BA",
            vendor_id: "80F766B1-B79F-4640-AAE9-2674BEC1803B",
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
            self.vendor_id,
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
        let payload = Payload {
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
    byteorder::BigEndian::write_u64(&mut buf, chunk * 0x4678d05);
    payload.extend_from_slice(&buf[4..8]);
    let random_xor = length * 0x4678d05 as u64 ^ chunk as u64;
    let mut buf = [0; 8];
    byteorder::BigEndian::write_u64(&mut buf, random_xor);
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
        let section_value = byteorder::LittleEndian::read_uint(&section, section.len());
        let temp_val =
            section_value & 0xffffffff00000000 | section_value * 0x4678d05 ^ chunk as u64;
        let mut buf = [0; 8];
        byteorder::BigEndian::write_u64(&mut buf, temp_val);
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
