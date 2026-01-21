mkdir ../thirdparty
cd ../thirdparty

git clone https://github.com/apache/teaclave-sgx-sdk
cd teaclave-sgx-sdk/tools/docker

docker build -t teaclave . -f Dockerfile.ubuntu20.04
