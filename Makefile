# Makefile
# ___________________________________________________________

# Recipe that uses the positional argument
#watch:
#	@echo "Running watch for $(PROJECT)"
#	@cd $(PROJECT) && cargo watch -q -c -w src/ -x run | lolcat

# Recipe that uses the loggers
# Recipe that uses the loggers
watch:
	@cd $(PROJECT) && RUST_LOG=info cargo watch -q -c -w src/ -s \
	"cargo run 2>&1 | awk '/NOW LISTENING ON/ {print_it = 1} print_it'"
# ___________________________________________________________

test:
	@echo "Running watch for tests in $(PROJECT)"
	@cd $(PROJECT) && cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
# ___________________________________________________________

# Add package to project with features
#add:
#	@echo "Adding package $(PACKAGE) to project $(PROJECT_NAME) with feature(s) $(FEATURE)"
#	cargo add $(PACKAGE) -p $(PROJECT_NAME) --features "$(FEATURE)"
# ___________________________________________________________





