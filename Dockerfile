FROM yinguobing/opencv:4.5.4-devel-rust1.62-ubuntu20.04 as build

WORKDIR /build/stream-capture
ADD . /build/stream-capture/

RUN /root/.cargo/bin/cargo build --release

FROM yinguobing/opencv:4.5.4-runtime-ubuntu20.04

LABEL Name=STREAM-CAPTURE Version=0.0.1

WORKDIR /app

COPY --from=build /build/stream-capture/target/release/stream-capture /app/
RUN ldconfig

CMD ["/app/stream-capture"]
