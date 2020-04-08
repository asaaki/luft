default:
	@echo nope

luft: release
.PHONY: luft

release:
	@cargo fmt
	@cargo build --release

test-run:
	@echo "You should setup and run an influxdb instance."
	@target/release/luft

run-cross:
	@cd cross && ./run.sh

prepare-cross:
	@cd cross && ./setup.sh

# dockerized cross stage - also not working better, not even on a Mac
doo:
	docker build . -t cross-stage -f cross/Dockerfile
	docker run --rm cross-stage
	docker run --rm -ti -v $(PWD):/stage:delegated -w /stage cross-stage /bin/bash

influxdb:
	docker-compose up -d

stop-influxdb:
	docker-compose down

init-influxdb:
	docker-compose run influxdb influx -host influxdb -execute 'CREATE DATABASE test_co2'

# last 60 seconds after make call
# poor man's "high resolution" fix
TS = $(shell echo $$((`date +%s` - 60))000000000 )

check-influxdb:
	docker-compose run influxdb influx -host influxdb -database test_co2 -execute \
		'SELECT co2,temperature,location FROM measurements2 WHERE time > $(TS)'
