# Makefile
# ___________________________________________________________

# Recipe that uses the positional argument
#watch:
#	@echo "Running watch for $(PROJECT)"
#	@cd $(PROJECT) && cargo watch -q -c -w src/ -x run | lolcat

# EXAMPLES: make watch PROJECT=axum-app2024/
watch:
	@cd $(PROJECT) && RUST_LOG=info cargo watch -q -c -w src/ -s \
	"cargo run 2>&1 | awk '/NOW LISTENING ON/ {print_it = 1} print_it'"
# ___________________________________________________________

# EXAMPLES: make test PROJECT=axum-app2024/
test:
	@echo "Running watch for tests in $(PROJECT)" | lolcat
	@cd $(PROJECT) && cargo watch -q -c -w src/ -w tests/ -x "test -q quick_dev -- --nocapture"
# ___________________________________________________________

# EXAMPLES: make examples EXAMPLE=ticket_example
examples:
	@echo "\n\n\nRunning example in example-code project\n\n" | lolcat
	@cd example-code && cargo run --bin $(EXAMPLE) --quiet
# ___________________________________________________________





