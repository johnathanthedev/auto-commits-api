# Daily Commits API

An API for Daily Commits Frontend.

## About this project

The purpose of this project is to have an ongoing service that will log commits daily to a designated repo.

## Requirements

- Docker

## How to run locally

You will need a .env file in the root of this project. Look at .env.example for reference.

Clone this repo and run `docker-compose up`.

## Migrations

Run migrations running `make db-migrate` in container.
