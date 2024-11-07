FROM rust:1.63-buster

COPY ./maps			        /filler/maps
COPY ./robots		        /filler/robots
COPY ./game_engine	        /filler/game_engine
COPY ./solution             /filler/solution

WORKDIR /filler/solution

RUN cargo build --release

WORKDIR /filler

ENTRYPOINT /bin/bash
