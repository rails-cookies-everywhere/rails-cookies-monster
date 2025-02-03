# check=skip=SecretsUsedInArgOrEnv

# Dockerfile
################################################################################
# Stage 0: Setup
ARG BASE_IMAGE_TAG=latest
ARG RAILS_PATCH=none

FROM rails-cookies-everywhere:ruby-base-${BASE_IMAGE_TAG} AS build-rails
ARG RAILS_VERSION_TAG

LABEL rails-cookies=everywhere
LABEL rails-cookies-everywhere=rails:v$RAILS_VERSION_TAG

################################################################################
# Stage 1: Build rails
WORKDIR /rails

RUN git checkout v$RAILS_VERSION_TAG
RUN gem build rails.gemspec

################################################################################
# Stage 2: Install rails
FROM build-rails AS install-rails
ARG RAILS_VERSION_TAG
WORKDIR /rails

RUN gem install ./rails-$RAILS_VERSION_TAG.gem

################################################################################
# Stage 3.0: Patch (none)
FROM install-rails AS patch-rails-none
RUN echo "Hello from none"

################################################################################
# Stage 3.1: Patch (7.0.x)
FROM install-rails AS patch-rails-7.0.x
RUN echo "Hello from 7.0.x"
ARG RAILS_VERSION_TAG
WORKDIR /
COPY rails-7.0.x.patch /
RUN sed /rails-7.0.x.patch -i -e "s|7.0.x|$RAILS_VERSION_TAG|g"; true
RUN patch -p1 < ./rails-7.0.x.patch; true

################################################################################
# Stage 4: Create app

FROM patch-rails-${RAILS_PATCH} AS create-app
WORKDIR /app

RUN rails new cookie-monster \
  --quiet \
  --skip-bundle \
  --skip-git \
  --skip-docker \
  --skip-keeps \
  --skip-action-mailer \
  --skip-action-mailbox \
  --skip-action-text \
  --skip-active-record \
  --skip-active-job \
  --skip-active-storage \
  --skip-action-cable \
  --skip-asset-pipeline \
  --skip-javascript \
  --skip-hotwire \
  --skip-jbuilder \
  --skip-test \
  --skip-system-test \
  --skip-bootsnap \
  --skip-dev-gems \
  --skip-thruster \
  --skip-rubocop \
  --skip-brakeman \
  --skip-ci \
  --skip-kamal \
  --skip-solid

################################################################################
# Stage 5: Setup app
FROM create-app AS setup-app
WORKDIR /app/cookie-monster

RUN sed -i Gemfile -e 's|gem "rails", "~> |gem "rails", "|'
RUN sed -i config/environments/production.rb -e 's|config.force_ssl = true|# config.force_ssl = true|'
RUN bundle install
COPY rails_patch/rails_controller.rb /app/cookie-monster/app/controllers/monsters_controller.rb
COPY rails_patch/rails_routes.rb /app/cookie-monster/config/routes.rb

################################################################################
# Stage 6: Run the app
FROM setup-app AS run-app
WORKDIR /app/cookie-monster

ENV RAILS_ENV="production"
ENV SECRET_KEY_BASE="rails-cookies-everywhere"
ENV CANARY_VALUE="correct-horse-battery-staple"

EXPOSE 3000
CMD ["./bin/rails", "server"]
