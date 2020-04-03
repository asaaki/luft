default:
	@echo nope

run-cross:
	@cd cross && ./run.sh

prepare-cross:
	@cd cross && ./setup.sh
