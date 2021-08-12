# syntax=docker/dockerfile:1
FROM debian:buster-slim AS build

ENV DEBIAN_FRONTEND noninteractive
ARG UID
ARG GID
ARG USER=docker

# Install dependences and build tools
RUN apt-get update
RUN apt-get -y install cargo devscripts build-essential lintian bash-completion alien

# Add current user to docker image (prevent output files to be owned by root)
RUN addgroup --gid ${GID} ${USER}
RUN adduser --disabled-password --gecos '' --uid ${UID} --gid ${GID} ${USER}

# Create the directory where de packages will live  and change ownership to $USER
RUN mkdir -p /home/${USER}/dpkg
RUN chown -R ${USER}:${USER} /home/${USER}

# Run containers as USER
USER ${USER}

# Change to working directory (pointing at root in local fs)
WORKDIR /home/${USER}/dpkg/src
