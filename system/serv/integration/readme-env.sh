#!/usr/bin/env bash

export PATH=/nix/store/8v97ngkcpfzgghwnnr7fsz33p2x22gy9-gcc-wrapper-14.3.0/bin:/nix/store/fnvsac4yaw2146ig4p54xnnm6b6alkjw-gnumake-4.4.1/bin:/run/wrappers/bin:/home/akama/.nix-profile/bin:/nix/profile/bin:/home/akama/.local/state/nix/profile/bin:/etc/profiles/per-user/akama/bin:/nix/var/nix/profiles/default/bin:/run/current-system/sw/bin

export CC=/nix/store/8v97ngkcpfzgghwnnr7fsz33p2x22gy9-gcc-wrapper-14.3.0/bin/gcc
export AR=/nix/store/8v97ngkcpfzgghwnnr7fsz33p2x22gy9-gcc-wrapper-14.3.0/bin/ar
export RANLIB=/nix/store/8v97ngkcpfzgghwnnr7fsz33p2x22gy9-gcc-wrapper-14.3.0/bin/ranlib
export PKG_CONFIG_FOR_TARGET=pkg-config
export PKG_CONFIG_PATH_FOR_TARGET=/nix/store/cqkz127l4l98xgp7p6y1j2cqvyx0w3jl-openssl-3.6.2-dev/lib/pkgconfig
export OPENSSL_INCLUDE_DIR=/nix/store/cqkz127l4l98xgp7p6y1j2cqvyx0w3jl-openssl-3.6.2-dev/include
export OPENSSL_LIB_DIR=/nix/store/by19gg2pqp2b1gqphdf6bwk2g3hgak79-openssl-3.6.2/lib
export OPENSSL_NO_VENDOR=1
export LD_LIBRARY_PATH=/nix/store/by19gg2pqp2b1gqphdf6bwk2g3hgak79-openssl-3.6.2/lib

export SGX_DCAP_QPL=/nix/store/ag1rm51flifx1vfplvrfz0ib5knhyqhh-sgx-dcap-quote-provider/lib/libdcap_quoteprov.so.1
export SGX_DCAP_QVE=/tmp/psw/ae/data/prebuilt/libsgx_qve.signed.so
export SGX_DCAP_QVL=/nix/store/w5cbxfpq4580l44i3q3xpj0d43xwcdsc-sgx-dcap-quote-verify-2.25.100.3/lib/libsgx_dcap_quoteverify.so

export AESM_SOCKET=/run/aesmd/aesm.socket
export AESM_PROXY=127.0.0.1:5555
export PCCS_URL=https://pccs.phala.network
export NO_PROXY=127.0.0.1,localhost

export STAR_ALLOW_DEBUG_ENCLAVES=1
export STAR_ALLOW_ADVISORY_ENCLAVES=1
export STAR_TRUST_SAME_SIGNER_ENCLAVES=1

export STAR_ENCLAVE_SGXS=/home/akama/star/system/serv/target/x86_64-fortanix-unknown-sgx/release/enclave.sgxs
export STAR_KEY_MANAGER_SGXS=/home/akama/star/system/serv/target/x86_64-fortanix-unknown-sgx/release/key-manager.sgxs
export STAR_KEY_MANAGER_SEALED_KEYS=/home/akama/star/system/serv/integration/key-manager.keys.sealed

export STAR_KEY_MANAGER_ADDR=127.0.0.1:8090
export STAR_KEY_MANAGER_URL=http://127.0.0.1:8090

export STAR_API_ADDR=127.0.0.1:18080
export STAR_PROXY_ADDR=127.0.0.1:18081
export STAR_UPSTREAM_ADDR=127.0.0.1:13000

export TLS_CERT_PATH=
export TLS_KEY_PATH=
