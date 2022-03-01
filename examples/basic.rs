fn main() {
    let imeis = [
        "566210190731872",
        "490154203237518",
        "152343379164287",
        "358122562214015",
    ];

    for imei in imeis {
        let valid = imei::valid(imei);

        println!("{imei}: {valid}");
    }
}
