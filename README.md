# Web Display

Get a shell for cross-compiling by running `docker-compose run --rm
build bash`

This will build a docker image with everything necessary to
cross-compile the project.

Then, in the shell inside the docker container run `cargo build
--target=armv7-unknown-linux-gnueabihf`
