mod solver;
use solver::Solver;

fn main() {
    // // // dbg!(body.len());
    // // let body = serde_json::to_string_pretty(&value).unwrap();
    // dbg!(solver::encode_payload(&body));
    // // let device = solver::Device::new();
    // // dbg!(&device);

    let initial_payload = solver::initial_payload(None);
    dbg!(initial_payload.unwrap());
    let solver = Solver::new("bed41483-2834-400c-8a9d-0960ac80c8e2", "2.AeI9ZL4rqT1nTmmZigF2l2rzzNe3jXswovuWhfmj25qB6otijpD_oXiEeLP34a22l21TgO6EYzsRjWc8fJ9mchO1AlSr80jv1PRMHTaDnhZWWeIpeJhFB3YWHV20OiyDBEvdbEqSakrFxG-5HR57yIvOemLwMXa4EdBET6e4YIX5h58OAGP4YqwtqTUsCzb_PjwSlubCqaTkfzj3UhDB4NnpyYf-x_dfleCgxuVTNTqH8S1y1kgaFd85-_BaDSwlYP5fnl59X5YAZ6J5cbXFUqUtfOtaLsiH3TFeSrn9CqUcInHxQA5evo5Anst2qJjs1AjZx9pzHtOo4Gan7HIdS1gK3zFfgpzqeIfmdPBLZBTWaLchRyvgNE7zFMjEkM3lP1yWcf4JnNBo-XO7xfOVRmeQhpuoxTQ=");

    dbg!(solver.solve().unwrap());
    // // // println!("{}", );

    // // let pre = "{\"application_manifest\":\"2.AdVcYX_6cIB15rBXSWb1pUb9aa48my0Doxp30jgW63NLyRNKh1eelG9GwMxRC2ups8te4VstDQrvm3zRsnlWCQ0qNDLGNb3viRDYQ_NkyJA0RrXYukm3_fXoe8Ca3-TLKK6oa-Etd1p7_z2yc9t9-bIlpZ5dMX366JK91R7S3g==\"}";
    // let x = base64::decode_config("Y84aQUVt-gajNA5qlH-V0bIF6j6jjpooIDIcr5zAuFLxI_YthIZo1M484GXgAtr8vdSttijEJWrXKxmqWWgMY53NBpWumglAYAAERhCh-p9VEtQ-4u1ya3NGbXE5jdB0otxJP1GVrzjw1EN8LtNrd30pXGA541tl-EJMUsfEEzxoThrFJsulyF3LE0O52iv8qrRGDIzuL-MMPfgbujg5rl5a3Ka3TcCaxuDqREppBK475CeV1OuHv7yH37CF6IqCFqMWHXWrePF8IWlD11vdwwa4SM6QjouZ", base64::URL_SAFE_NO_PAD).unwrap();

    // let y = base64::decode_config("Vu8MsxOkp1z1_VMwlH-V0bIF6j6jjpooIDIcr5zAuFLxI_YthIZo1M484GXgAtr8vdSttijEJWrXKxmqWWgMY53NBpWumglAYAAERhCh-p9VEtQ-4u1ya3NGbXE5jdB0otxJP1GVrzjw1EN8LtNrd30pXGA541tl-EJMUsfEEzxoThrFJsulyF3LE0O52iv8qrRGDIzuL-MMPfgbujg5rl5a3Ka3TcCaxuDqREppBK475CeV1OuHv7yH37CF6IqCFqMWHXWrePF8IWlD11vdwwa4SM6QjouZ", base64::URL_SAFE_NO_PAD).unwrap();
    // dbg!(BigEndian::read_u32(&x[0..4]));
    // dbg!(BigEndian::read_u32(&y[0..4]));
    // dbg!(BigEndian::read_u32(&x[0..4]) / 0x4678d05);
    // dbg!(BigEndian::read_u32(&y[0..4]) / 0x4678d05);
    // let mut buf = [0; 4];
    // BigEndian::write_u32(&mut buf, BigEndian::read_u32(&x[0..4]) / 0x4678d05);
    // println!("{:?}", buf);
    // dbg!((BigEndian::read_u32(&x[4..8]) ^ 201) / 0x4678d05);
    // dbg!((BigEndian::read_u32(&y[4..8]) ^ 201) / 0x4678d05);
    // for i in 0..12 {
    //     x[i] =
    // }

    // let value = base64::encode_config(&x, base64::URL_SAFE_NO_PAD);

    // println!("{:?}", &pre.as_bytes()[..4]);
    // println!("{:?}\n{:?}", &x[12..16], &y[12..16]);
    // dbg!(BigEndian::read_u32(&pre.as_bytes()[..4]));
    // // dbg!();
    // dbg!(BigEndian::read_u32(&y[12..16]));

    // let val = BigEndian::read_u32(&pre.as_bytes()[..4]);
    // let z = BigEndian::read_u32(&x[8..12]);

    // for v in 0..0xFFFFFFFF as u32 {
    //     // dbg!(z ^ v);
    //     if z ^ v / 0x4678d05  == val {
    //         let mut buf = [0; 4];
    //         BigEndian::write_u32(&mut buf, v);
    //         dbg!(buf);
    //         dbg!(v);
    //         println!("{:?}\n{:?}", &x[..12], &y[..12]);
    //         return;
    //     }
    // };

    // let random_number: u32 = rand::random();

    // let x_1 = LittleEndian::read_u64(&x[..]);
    // let x_2 =  BigEndian::read_u32(&x[4..8]);
    // let x_4 =  BigEndian::read_u32(&x[8..]);
    // dbg!(x_1);
    // dbg!(x_2);
    // dbg!(x_4);
    // dbg!(x_1);
    // dbg!(x_2);
    // dbg!(x_3);
    // dbg!(x_4);
    // let x_2 = (x_2 * 0x4678d05 & 0xff00ff00) >> 8 | (x_2 * 0x4678d05 & 0xff00ff) << 8;
    // println!("{}", x_2 >> 0x10 | x_2 << 0x10)

    // solver::encode_payload("23456745345678");
}
