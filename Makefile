SHELL=/bin/bash
EXERCISES = hello-world option result pattern-matching lifetimes

all: $(EXERCISES)

.SILENT:

.PHONY: $(EXERCISES)
$(EXERCISES):
	echo "Running $@..."
	if diff "$@/expected.stdout" <(cd "$@" && cargo run); \
	then \
		echo "PASS $@"; \
	else \
		echo "FAIL $@"; \
		false; \
	fi
