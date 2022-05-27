FROM ubuntu:20.04
ENV DEBIAN_FRONTEND noninteractive

RUN apt update -y
RUN apt install python3 python3-pip python3-opencv -y

COPY ./target/x86_64-unknown-linux-musl/release/no-bitches-bot /app
RUN file /app
RUN chmod +x /app

RUN echo a
COPY ./src /src
COPY ./datasets /datasets
COPY ./requirements.txt /requirements.txt

RUN python3 -m pip install -r /requirements.txt
RUN apt install wget -y
# RUN apt install libssl-dev libssl1.1 wget mlocate -y
# # RUN locate libssl
# # RUN ln -s /usr/local/lib/libssl.so.3 /usr/lib/libssl.so.3
# # RUN ln -s /usr/local/lib/libcrypto.so.3 /usr/lib/libcrypto.so.3
# # RUN ldconfig
# RUN apt-get install -y build-essential
# RUN apt-get install -y zlib1g-dev
# ARG OPENSSL_VERSION=3.0.2
# RUN wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz
# RUN tar xvfz openssl-${OPENSSL_VERSION}.tar.gz
# RUN cd openssl-${OPENSSL_VERSION} && ./config && make && make install
# RUN echo '/usr/local/lib' >> /etc/ld.so.conf
# RUN cat /etc/ld.so.conf
# RUN ldconfig
# RUN echo 'export LD_LIBRARY_PATH=/usr/local/lib' >> ~/.bash_profile && . ~/.bash_profile
# RUN export LD_LIBRARY_PATH=/usr/local/lib
# # RUN find / | grep ssl
# RUN ln -s /openssl-3.0.2/libssl.so.3 /usr/lib/libssl.so.3
# RUN ln -s /openssl-3.0.2/libcrypto.so.3 /usr/lib/libcrypto.so.3
# RUN apt install libc6 libc-bin -y