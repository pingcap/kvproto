FROM ubuntu:20.04

# If the timezone is not set the apt install will become interactive
ENV TZ=UTC
RUN echo "UTC" >> /etc/timezone

RUN apt-get update --yes \
 && apt-get install --yes tzdata \
 && dpkg-reconfigure --frontend noninteractive tzdata \
 && echo "deps for Rust and C++" \
 && apt-get install --yes cmake clang libclang-dev llvm llvm-dev \
 && echo "deps for building C++" \
 && apt-get install --yes git golang libssl-dev \
 && rm -rf /var/lib/apt/lists/*

# C++ setup
ENV GRPC_INSTALL_PATH=/grpcinstall/v1_26_0
ENV PATH="/usr/lib/protoc/bin:$GRPC_INSTALL_PATH/bin:$PATH"

# C++ setup
RUN mkdir -p $GRPC_INSTALL_PATH \
 && git clone https://github.com/grpc/grpc.git \
 && cd grpc \
 && git checkout v1.26.0 \
 && git submodule update --init \
 && mkdir -p grpcbuild \
 && cd grpcbuild \
 && cmake .. -DgRPC_BUILD_TESTS=OFF -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=$GRPC_INSTALL_PATH \
 && make install \
 && cd .. \
 && rm -rf grpcbuild \
 && mkdir -p grpcbuild \
 && cd grpcbuild \
 && cmake .. -DgRPC_INSTALL=ON -DgRPC_BUILD_TESTS=OFF -DgRPC_PROTOBUF_PROVIDER=package -DgRPC_ZLIB_PROVIDER=package -DgRPC_CARES_PROVIDER=package -DgRPC_SSL_PROVIDER=package -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=$GRPC_INSTALL_PATH && make install \
 && cd ..

# Golang setup
RUN apt-get update --yes \
 && apt-get install --yes curl unzip \
 && rm -rf /var/lib/apt/lists/*

ENV GO_VERSION "1.18.2"
RUN rm -rf /usr/local/go \
 && curl -O https://dl.google.com/go/go$GO_VERSION.linux-amd64.tar.gz \
 && tar -C /usr/local -xzf go$GO_VERSION.linux-amd64.tar.gz \
 && rm go$GO_VERSION.linux-amd64.tar.gz
ENV PATH="/usr/local/go/bin:$PATH"

# Protoc setup
ENV PROTOC_VERSION "3.8.0"
RUN curl -L https://github.com/google/protobuf/releases/download/v$PROTOC_VERSION/protoc-$PROTOC_VERSION-linux-x86_64.zip -o protoc.zip && \
    unzip protoc.zip -d protoc && \
    mv protoc /usr/lib/

## Rust setup
ENV RUST_TEST_THREADS "1"
ENV RUST_BACKTRACE "1"
ENV RUSTFLAGS "-Dwarnings"
ENV CPLUS_INCLUDE_PATH "/usr/lib/gcc/x86_64-linux-gnu/6/include"
RUN apt-get update --yes \
 && apt-get install --yes cargo \
 && rm -rf /var/lib/apt/lists/*
