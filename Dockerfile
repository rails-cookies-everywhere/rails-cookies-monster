# Dockerfile
# Stage 1: Build rails
FROM rails-base AS build-rails
ARG RAILS_VERSION_TAG
WORKDIR /rails

RUN git checkout v$RAILS_VERSION_TAG
RUN gem build rails.gemspec

# Stage 2: Install rails
FROM build-rails AS install-rails
ARG RAILS_VERSION_TAG
WORKDIR /rails

RUN gem install ./rails-$RAILS_VERSION_TAG.gem

# Stage 3: Create app
FROM install-rails AS create-app
WORKDIR /app

RUN rails new cookie-monster

# Stage 4: Setup app
FROM create-app AS setup-app
WORKDIR /app/cookie-monster

RUN bundle install
COPY rails_patch/rails_controller.rb /app/cookie-monster/app/controllers/monsters_controller.rb
COPY rails_patch/rails_routes.rb /app/cookie-monster/config/routes.rb

# Stage 5: Run the app
FROM setup-app AS run-app
WORKDIR /app/cookie-monster

ENV RAILS_ENV="production"
ENV SECRET_KEY_BASE="rails-cookies-everywhere"
ENV CANARY_VALUE="correct-horse-battery-staple"

EXPOSE 3000
CMD ["./bin/rails", "server"]
