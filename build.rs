use std::io::Result;

fn main() -> Result<()> {
    const PAYMENT_PROTOS: &[&str] =
        &["service-apis/proto/sited_io/payment/v1/stripe.proto"];

    const COMMERCE_PROTOS: &[&str] = &[
        "service-apis/proto/sited_io/commerce/v1/shop.proto",
        "service-apis/proto/sited_io/commerce/v1/offer.proto",
        "service-apis/proto/sited_io/commerce/v1/shipping_rate.proto",
    ];

    const INCLUDES: &[&str] = &["service-apis/proto"];

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path("src/api/FILE_DESCRIPTOR_SET")
        .build_client(false)
        .build_server(true)
        .compile(PAYMENT_PROTOS, INCLUDES)?;

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(false)
        .compile(COMMERCE_PROTOS, INCLUDES)?;

    Ok(())
}
