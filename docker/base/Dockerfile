# Dockerfile.base
ARG BASE_IMAGE_TAG=latest
FROM ruby:${BASE_IMAGE_TAG}

LABEL rails-cookies=everywhere
LABEL rails-cookies-everywhere=ruby-base:${BASE_IMAGE_TAG}

# Install dependencies
RUN apt-get update -qq && apt-get install -y nodejs git

# Clone the Rails repository
RUN git clone https://github.com/rails/rails.git /rails

# Set the working directory
WORKDIR /rails
