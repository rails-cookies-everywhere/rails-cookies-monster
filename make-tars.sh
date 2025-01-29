#!/bin/sh

tar -cvf ruby-base.tar      -C docker/base  Dockerfile
tar -cvf rails-versions.tar -C docker/rails Dockerfile rails_patch
