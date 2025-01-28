#!/bin/sh

tar -cvf rails-base.tar     -C docker/base  Dockerfile
tar -cvf rails-versions.tar -C docker/rails Dockerfile rails_patch
